//! Bild → Umrisse (Roadmap Phase 4 Punkt 8): ein Bild (PNG, JPG, BMP, GIF) in Farbflächen zerlegen und als
//! Stickobjekte ausgeben – wie „Auto Digitizer“ im Original (Handbuch PDF-S. 207: erkennt Formen im Bild, wählt
//! Sticharten, Reihenfolge nach „closest join“).
//!
//! Ablauf (**eigene Festlegungen**, das Verfahren des Originals ist nicht analysiert):
//! 1. Bild auf höchstens 600 Punkte Kantenlänge verkleinern, Farben auf `farben` reduzieren (k-Means, Startwerte
//!    „am weitesten entfernt zuerst“),
//!    durchsichtige Punkte sind Hintergrund,
//! 2. Hintergrund = häufigste Farbe am Bildrand (abschaltbar), entfällt,
//! 3. Rauschen glätten (Mehrheitsfilter 3 × 3),
//! 4. je Farbe die Umrisse mit Löchern verfolgen (Suzuki–Abe, `imageproc`), vereinfachen (Douglas–Peucker),
//! 5. Flächen unter `min_flaeche` entfallen; je Fläche ein Tatami-Objekt (EasyStitch des Originals: Flächen über 12 mm
//!    Tatami – schmale Flächen als Satin folgen später), sortiert nach Farbe (größte Gesamtfläche zuerst), innerhalb
//!    einer Farbe nach kürzestem Weg.

pub mod easystitch;
pub mod svg;

use image::{Rgba, RgbaImage};
use imageproc::contours::{BorderType, find_contours};
use serde_json::{Value, json};
use stick_kern::geometrie::{Linie, Punkt, abstand_punkt_strecke, p};

/// Einstellungen der Umwandlung.
#[derive(Clone, Debug)]
pub struct BildOptionen {
    /// Breite des Ergebnisses in mm (die Höhe folgt dem Seitenverhältnis)
    pub breite_mm: f64,
    /// Zahl der Farben nach der Reduktion (ohne Hintergrund)
    pub farben: usize,
    /// kleinste Fläche in mm²
    pub min_flaeche: f64,
    /// Toleranz der Vereinfachung in mm
    pub glaettung: f64,
    /// Hintergrund (häufigste Randfarbe) weglassen
    pub ohne_hintergrund: bool,
}

impl Default for BildOptionen {
    fn default() -> Self {
        BildOptionen {
            breite_mm: 80.0,
            farben: 6,
            min_flaeche: 4.0,
            glaettung: 0.2,
            ohne_hintergrund: true,
        }
    }
}

/// Eine Farbfläche des Bildes (mm, Ursprung = Bildmitte, +y nach oben).
#[derive(Clone, Debug)]
pub struct Flaeche {
    pub farbe: [u8; 3],
    pub umriss: Linie,
    pub loecher: Vec<Linie>,
    pub flaeche: f64,
}

fn flaeche(r: &[Punkt]) -> f64 {
    (0..r.len())
        .map(|i| {
            let (a, b) = (r[i], r[(i + 1) % r.len()]);
            a.x * b.y - b.x * a.y
        })
        .sum::<f64>()
        .abs()
        / 2.0
}

/// Douglas–Peucker für geschlossene Ringe.
fn vereinfachen(ring: &[Punkt], tol: f64) -> Linie {
    fn dp(pts: &[Punkt], tol: f64, out: &mut Vec<Punkt>) {
        let (a, b) = (pts[0], pts[pts.len() - 1]);
        let (mut idx, mut dmax) = (0, 0.0);
        for (i, &q) in pts.iter().enumerate().take(pts.len() - 1).skip(1) {
            let d = abstand_punkt_strecke(q, a, b);
            if d > dmax {
                (idx, dmax) = (i, d);
            }
        }
        if dmax > tol {
            dp(&pts[..=idx], tol, out);
            out.pop();
            dp(&pts[idx..], tol, out);
        } else {
            out.push(a);
            out.push(b);
        }
    }
    if ring.len() < 4 {
        return ring.to_vec();
    }
    // am weitesten entfernten Punkt teilen, damit der Ring nicht an einer zufälligen Stelle zerfällt
    let fern = (1..ring.len())
        .max_by(|&i, &j| {
            let d = |k: usize| (ring[k].x - ring[0].x).hypot(ring[k].y - ring[0].y);
            d(i).total_cmp(&d(j))
        })
        .unwrap();
    let mut a: Vec<Punkt> = Vec::new();
    dp(&ring[..=fern], tol, &mut a);
    let mut zweiter: Vec<Punkt> = ring[fern..].to_vec();
    zweiter.push(ring[0]);
    let mut b: Vec<Punkt> = Vec::new();
    dp(&zweiter, tol, &mut b);
    a.pop();
    a.extend(b);
    a.pop();
    a
}

/// Farben reduzieren (k-Means auf den nicht durchsichtigen Punkten; Startwerte: jeweils die am weitesten entfernte Farbe).
fn farben_reduzieren(bild: &RgbaImage, k: usize) -> (Vec<[u8; 3]>, Vec<Option<usize>>) {
    let pix: Vec<Option<[f64; 3]>> = bild
        .pixels()
        .map(|Rgba([r, g, b, a])| (*a >= 128).then(|| [f64::from(*r), f64::from(*g), f64::from(*b)]))
        .collect();
    let mut farbig: Vec<[f64; 3]> = pix.iter().flatten().copied().collect();
    if farbig.is_empty() {
        return (Vec::new(), vec![None; pix.len()]);
    }
    farbig.sort_by(|a, b| (a[0] + a[1] + a[2]).total_cmp(&(b[0] + b[1] + b[2])));
    let k = k.clamp(1, 32).min(farbig.len());
    let mut zentren: Vec<[f64; 3]> = Vec::new();
    let naechstes = |z: &[[f64; 3]], q: &[f64; 3]| -> usize {
        (0..z.len())
            .min_by(|&i, &j| {
                let d = |c: &[f64; 3]| (0..3).map(|t| (c[t] - q[t]).powi(2)).sum::<f64>();
                d(&z[i]).total_cmp(&d(&z[j]))
            })
            .unwrap()
    };
    // Stichprobe für die Iterationen (schnell), Zuordnung danach für alle
    let schritt = (farbig.len() / 20000).max(1);
    let probe: Vec<[f64; 3]> = farbig.iter().step_by(schritt).copied().collect();
    // Startwerte: Farbe in der Mitte der Helligkeit, dann jeweils die von allen Startwerten am weitesten entfernte
    zentren.push(probe[probe.len() / 2]);
    while zentren.len() < k {
        let fern = probe
            .iter()
            .max_by(|a, b| {
                let d = |q: &[f64; 3]| {
                    zentren
                        .iter()
                        .map(|c| (0..3).map(|t| (c[t] - q[t]).powi(2)).sum::<f64>())
                        .fold(f64::MAX, f64::min)
                };
                d(a).total_cmp(&d(b))
            })
            .copied()
            .unwrap();
        zentren.push(fern);
    }
    for _ in 0..12 {
        let mut summe = vec![[0.0f64; 4]; k];
        for q in &probe {
            let i = naechstes(&zentren, q);
            for t in 0..3 {
                summe[i][t] += q[t];
            }
            summe[i][3] += 1.0;
        }
        for i in 0..k {
            if summe[i][3] > 0.0 {
                zentren[i] = [
                    summe[i][0] / summe[i][3],
                    summe[i][1] / summe[i][3],
                    summe[i][2] / summe[i][3],
                ];
            }
        }
    }
    let zuordnung = pix.iter().map(|q| q.map(|c| naechstes(&zentren, &c))).collect();
    let farben = zentren
        .iter()
        .map(|c| [c[0].round() as u8, c[1].round() as u8, c[2].round() as u8])
        .collect();
    (farben, zuordnung)
}

/// Bilddatei laden (PNG, JPG, BMP, GIF).
pub fn bild_laden(pfad: &std::path::Path) -> Result<RgbaImage, String> {
    Ok(image::open(pfad)
        .map_err(|e| format!("{}: {e}", pfad.display()))?
        .to_rgba8())
}

/// Bild in Farbflächen zerlegen.
pub fn vektorisieren(bild: &RgbaImage, o: &BildOptionen) -> Vec<Flaeche> {
    let mut bild = bild.clone();
    let max_seite = bild.width().max(bild.height());
    if max_seite > 600 {
        let s = 600.0 / f64::from(max_seite);
        bild = image::imageops::resize(
            &bild,
            ((f64::from(bild.width()) * s).round() as u32).max(1),
            ((f64::from(bild.height()) * s).round() as u32).max(1),
            image::imageops::FilterType::Triangle,
        );
    }
    let (w, h) = (bild.width() as usize, bild.height() as usize);
    if w == 0 || h == 0 {
        return Vec::new();
    }
    let (farben, mut z) = farben_reduzieren(&bild, o.farben + usize::from(o.ohne_hintergrund));
    // Mehrheitsfilter 3 × 3 (zweimal)
    for _ in 0..2 {
        let alt = z.clone();
        for y in 0..h {
            for x in 0..w {
                let mut zaehl: Vec<(Option<usize>, u8)> = Vec::new();
                for dy in -1i64..=1 {
                    for dx in -1i64..=1 {
                        let (xx, yy) = (x as i64 + dx, y as i64 + dy);
                        if xx < 0 || yy < 0 || xx >= w as i64 || yy >= h as i64 {
                            continue;
                        }
                        let v = alt[yy as usize * w + xx as usize];
                        match zaehl.iter_mut().find(|(c, _)| *c == v) {
                            Some(e) => e.1 += 1,
                            None => zaehl.push((v, 1)),
                        }
                    }
                }
                if let Some(&(v, n)) = zaehl.iter().max_by_key(|e| e.1)
                    && n >= 5
                {
                    z[y * w + x] = v;
                }
            }
        }
    }
    // Hintergrund: häufigste Farbe am Rand
    let hintergrund = o.ohne_hintergrund.then(|| {
        let mut zaehl = vec![0usize; farben.len()];
        for x in 0..w {
            for y in [0, h - 1] {
                if let Some(c) = z[y * w + x] {
                    zaehl[c] += 1;
                }
            }
        }
        for y in 0..h {
            for x in [0, w - 1] {
                if let Some(c) = z[y * w + x] {
                    zaehl[c] += 1;
                }
            }
        }
        (0..farben.len()).max_by_key(|&c| zaehl[c])
    });
    let mm = o.breite_mm / w as f64;
    let zu_mm = |x: f64, y: f64| p((x - w as f64 / 2.0) * mm, (h as f64 / 2.0 - y) * mm);
    let mut out = Vec::new();
    for (c, farbe) in farben.iter().enumerate() {
        if hintergrund.flatten() == Some(c) {
            continue;
        }
        // Maske mit einem Punkt Rand (Konturen am Bildrand schließen)
        let mut maske = image::GrayImage::new(w as u32 + 2, h as u32 + 2);
        for y in 0..h {
            for x in 0..w {
                if z[y * w + x] == Some(c) {
                    maske.put_pixel(x as u32 + 1, y as u32 + 1, image::Luma([255]));
                }
            }
        }
        let konturen = find_contours::<i32>(&maske);
        let ring = |k: usize| -> Linie {
            let pts: Linie = konturen[k]
                .points
                .iter()
                .map(|q| zu_mm(f64::from(q.x) - 1.0 + 0.5, f64::from(q.y) - 1.0 + 0.5))
                .collect();
            vereinfachen(&pts, o.glaettung)
        };
        for (k, kt) in konturen.iter().enumerate() {
            if kt.border_type != BorderType::Outer {
                continue;
            }
            let umriss = ring(k);
            let a = flaeche(&umriss);
            if umriss.len() < 3 || a < o.min_flaeche {
                continue;
            }
            let loecher: Vec<Linie> = konturen
                .iter()
                .enumerate()
                .filter(|(_, l)| l.border_type == BorderType::Hole && l.parent == Some(k))
                .map(|(i, _)| ring(i))
                .filter(|l| l.len() >= 3 && flaeche(l) >= o.min_flaeche)
                .collect();
            let netto = a - loecher.iter().map(|l| flaeche(l)).sum::<f64>();
            out.push(Flaeche {
                farbe: *farbe,
                umriss,
                loecher,
                flaeche: netto,
            });
        }
    }
    out
}

/// Flächen ordnen: Farben nach Gesamtfläche (größte zuerst), innerhalb einer Farbe nächster Anfang (Mittelpunkte).
pub fn ordnen(mut f: Vec<Flaeche>) -> Vec<Flaeche> {
    let mut farben: Vec<([u8; 3], f64)> = Vec::new();
    for x in &f {
        match farben.iter_mut().find(|(c, _)| *c == x.farbe) {
            Some(e) => e.1 += x.flaeche,
            None => farben.push((x.farbe, x.flaeche)),
        }
    }
    farben.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mitte = |x: &Flaeche| {
        let n = x.umriss.len() as f64;
        p(
            x.umriss.iter().map(|q| q.x).sum::<f64>() / n,
            x.umriss.iter().map(|q| q.y).sum::<f64>() / n,
        )
    };
    let mut out = Vec::new();
    let mut pos = p(0.0, 0.0);
    for (c, _) in farben {
        let mut rest: Vec<Flaeche> = Vec::new();
        f.retain(|x| {
            if x.farbe == c {
                rest.push(x.clone());
                false
            } else {
                true
            }
        });
        while !rest.is_empty() {
            let i = (0..rest.len())
                .min_by(|&a, &b| {
                    let d = |k: usize| (mitte(&rest[k]).x - pos.x).hypot(mitte(&rest[k]).y - pos.y);
                    d(a).total_cmp(&d(b))
                })
                .unwrap();
            let x = rest.remove(i);
            pos = mitte(&x);
            out.push(x);
        }
    }
    out
}

/// Design-JSON (Tatami-Flächen) aus den Flächen.
pub fn design_json(flaechen: &[Flaeche], name: &str, rahmen: &str) -> Value {
    let pkt = |l: &Linie| -> Value {
        l.iter()
            .map(|q| json!([(q.x * 100.0).round() / 100.0, (q.y * 100.0).round() / 100.0]))
            .collect()
    };
    let objekte: Vec<Value> = flaechen
        .iter()
        .enumerate()
        .map(|(i, f)| {
            json!({
                "art": "tatami",
                "name": format!("Fläche {}", i + 1),
                "farbe": format!("#{:02x}{:02x}{:02x}", f.farbe[0], f.farbe[1], f.farbe[2]),
                "umriss": pkt(&f.umriss),
                "loecher": f.loecher.iter().map(pkt).collect::<Vec<_>>(),
            })
        })
        .collect();
    json!({"name": name, "rahmen": rahmen, "objekte": objekte})
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Weißer Hintergrund, roter Ring (Loch in der Mitte) und blaues Quadrat
    fn testbild() -> RgbaImage {
        let mut b = RgbaImage::from_pixel(200, 100, Rgba([255, 255, 255, 255]));
        for y in 0..100u32 {
            for x in 0..200u32 {
                let (dx, dy) = (f64::from(x) - 50.0, f64::from(y) - 50.0);
                let r = dx.hypot(dy);
                if (15.0..40.0).contains(&r) {
                    b.put_pixel(x, y, Rgba([200, 30, 30, 255]));
                }
                if (120..180).contains(&x) && (20..80).contains(&y) {
                    b.put_pixel(x, y, Rgba([30, 60, 200, 255]));
                }
            }
        }
        b
    }

    #[test]
    fn ring_und_quadrat() {
        let o = BildOptionen {
            breite_mm: 100.0,
            farben: 2,
            ..Default::default()
        };
        let f = ordnen(vektorisieren(&testbild(), &o));
        assert_eq!(
            f.len(),
            2,
            "{:?}",
            f.iter().map(|x| (x.farbe, x.flaeche)).collect::<Vec<_>>()
        );
        let ring = f.iter().find(|x| x.farbe[0] > 150).unwrap();
        assert_eq!(ring.loecher.len(), 1, "Ring mit Loch");
        // Ring 0,5 mm je Bildpunkt: Fläche π (40² − 15²) / 4 ≈ 1080 mm²
        assert!((ring.flaeche - 1080.0).abs() < 80.0, "{}", ring.flaeche);
        let quadrat = f.iter().find(|x| x.farbe[2] > 150).unwrap();
        assert!((quadrat.flaeche - 900.0).abs() < 40.0, "{}", quadrat.flaeche);
        assert!(quadrat.umriss.len() <= 8, "vereinfacht: {}", quadrat.umriss.len());
        // als Design berechenbar
        let d = stick_kern::Design::aus_value(&design_json(&f, "Test", "artista 130 x 100")).unwrap();
        assert!(d.berechnen().unwrap().stiche.len() > 1000);
    }
}
