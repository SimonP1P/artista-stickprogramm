//! Bild → Umrisse (Roadmap Phase 4 Punkt 8): ein Bild (PNG, JPG, BMP, GIF) in Farbflächen zerlegen und als
//! Stickobjekte ausgeben – wie „Auto Digitizer“ im Original (Handbuch PDF-S. 118–120: erkennt Formen im Bild, wählt
//! Sticharten, Reihenfolge nach „closest join“; Farben weglassen „Omitted Colors“, „white background omitted“).
//!
//! Ablauf (Schritte nach dem Handbuch, Kap. 12 „Image clean up“: Farben reduzieren, Rauschen entfernen, unnötige
//! Details weglassen, Hintergrund entfernen – **die Verfahren selbst sind eigene Festlegungen**, die des Originals sind
//! nicht analysiert):
//! 1. Bild auf höchstens 600 Punkte Kantenlänge verkleinern; durchsichtige Punkte sind Hintergrund,
//! 2. **vereinfachen** (Stufe 0–5): kantenerhaltend glätten (Kuwahara-Filter) – Fotorauschen und feine Details
//!    verschwinden, Kanten bleiben scharf,
//! 3. Farben reduzieren: k-Means im **Lab-Farbraum** (Abstände wie das Auge sie sieht); auf Wunsch jede Farbe auf das
//!    nächste Garn einer Karte ziehen (CIEDE2000 wie `garnkarte`), gleiche Garne fallen zusammen,
//! 4. Rauschen glätten (Mehrheitsfilter 3 × 3), dann **kleine Flächen der Nachbarfarbe zuschlagen** (statt sie
//!    wegzuwerfen – so bleiben keine Löcher),
//! 5. Farben weglassen: ohne Angabe der Vorschlag „weißer Hintergrund“ (häufigste Randfarbe, wenn sie fast weiß ist –
//!    wie „white background omitted“ im Original), sonst die gewählten Farben,
//! 6. je Farbe die Umrisse mit Löchern verfolgen (Suzuki–Abe, `imageproc`), vereinfachen (Douglas–Peucker),
//! 7. Stichart je Fläche: „step“ (Standard, alles Tatami) oder „auto“ wie EasyStitch (Dicke ≤ 8 mm Satinfläche,
//!    darüber Tatami); Werte der Stile Narrow/Wide Objects: Tatami-Abstand 0,45 mm, Zugausgleich 0,40 mm (Handbuch
//!    S. 116 „Change the pull compensation to 0.4mm when using Auto Digitizer“); **Unterlage an** (ebenda: „Always
//!    remember to apply underlays“); sortiert nach Farbe (größte Gesamtfläche zuerst), innerhalb einer Farbe nach
//!    kürzestem Weg.

pub mod easystitch;
pub mod monogramm;
pub mod svg;

use image::{Rgba, RgbaImage};
use imageproc::contours::{BorderType, find_contours};
use serde_json::{Value, json};
use stick_kern::geometrie::{Linie, Punkt, abstand_punkt_strecke, p};

/// Stichart der Flächen (Auto Digitizer: „Fills: Auto / Step“). Standard hier **Step** – eigene Festlegung nach dem
/// Vergleich an einem Foto (21.11.2026): Satin auf den unregelmäßigen Flecken eines Fotos franst an den Rändern aus und
/// rechnet rund zehnmal länger; für Logos und Zeichnungen mit klaren Formen ist „auto“ die bessere Wahl.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fuellung {
    /// nach der Dicke wie EasyStitch: bis 8 mm Satinfläche, darüber Tatami
    Auto,
    /// alles Tatami (Step)
    #[default]
    Step,
}

/// Einstellungen der Umwandlung.
#[derive(Clone, Debug)]
pub struct BildOptionen {
    /// Breite des Ergebnisses in mm (die Höhe folgt dem Seitenverhältnis)
    pub breite_mm: f64,
    /// Zahl der Farben nach der Reduktion (alle Farben des Bildes, auch die weggelassenen)
    pub farben: usize,
    /// kleinste Fläche in mm² – kleinere gehen in der Nachbarfarbe auf
    pub min_flaeche: f64,
    /// Toleranz der Vereinfachung der Umrisse in mm
    pub glaettung: f64,
    /// Bild vereinfachen 0–5 (0 = aus): kantenerhaltendes Glätten vor der Farbreduktion
    pub vereinfachen: u32,
    /// Farben, die nicht gestickt werden (None = Vorschlag: weißer Hintergrund)
    pub weglassen: Option<Vec<[u8; 3]>>,
    /// Garnfarben: jede Farbe wird auf die nächste dieser Farben gezogen (None = Farben des Bildes)
    pub palette: Option<Vec<[u8; 3]>>,
    /// Stichart der Flächen
    pub fuellung: Fuellung,
}

impl Default for BildOptionen {
    fn default() -> Self {
        BildOptionen {
            breite_mm: 80.0,
            farben: 8,
            min_flaeche: 4.0,
            glaettung: 0.2,
            vereinfachen: 2,
            weglassen: None,
            palette: None,
            fuellung: Fuellung::Auto,
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

/// sRGB eines Punkts → Lab
fn lab_px(c: &Rgba<u8>) -> [f64; 3] {
    stick_kern::farbe::lab([c[0], c[1], c[2]])
}

fn abstand2(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    (0..3).map(|t| (a[t] - b[t]).powi(2)).sum()
}

/// Kantenerhaltendes Glätten (Kuwahara): je Punkt das Mittel des ruhigsten der vier Quadrate mit Kantenlänge r + 1
/// (Unruhe = Varianz der Helligkeit). Summentabellen machen es unabhängig von r schnell. Durchsichtige Punkte zählen
/// nicht mit.
fn kuwahara(b: &RgbaImage, r: u32) -> RgbaImage {
    let (w, h) = (b.width() as usize, b.height() as usize);
    let (sw, sh) = (w + 1, h + 1);
    // Summentabellen: R, G, B, Helligkeit, Helligkeit², Anzahl
    let mut t = vec![[0f64; 6]; sw * sh];
    for y in 0..h {
        let mut zeile = [0f64; 6];
        for x in 0..w {
            let c = b.get_pixel(x as u32, y as u32);
            if c[3] >= 128 {
                let (r_, g_, b_) = (f64::from(c[0]), f64::from(c[1]), f64::from(c[2]));
                let l = 0.299 * r_ + 0.587 * g_ + 0.114 * b_;
                for (k, v) in [r_, g_, b_, l, l * l, 1.0].into_iter().enumerate() {
                    zeile[k] += v;
                }
            }
            let oben = t[y * sw + x + 1];
            for k in 0..6 {
                t[(y + 1) * sw + x + 1][k] = oben[k] + zeile[k];
            }
        }
    }
    let summe = |x0: usize, y0: usize, x1: usize, y1: usize| -> [f64; 6] {
        // Rechteck [x0, x1] × [y0, y1] einschließlich
        let mut s = [0f64; 6];
        for k in 0..6 {
            s[k] = t[(y1 + 1) * sw + x1 + 1][k] - t[y0 * sw + x1 + 1][k] - t[(y1 + 1) * sw + x0][k] + t[y0 * sw + x0][k];
        }
        s
    };
    let r = r as usize;
    let mut out = b.clone();
    for y in 0..h {
        for x in 0..w {
            let c = b.get_pixel(x as u32, y as u32);
            if c[3] < 128 {
                continue;
            }
            let (xa, xb) = (x.saturating_sub(r), (x + r).min(w - 1));
            let (ya, yb) = (y.saturating_sub(r), (y + r).min(h - 1));
            let mut best: Option<(f64, [f64; 6])> = None;
            for (x0, x1, y0, y1) in [(xa, x, ya, y), (x, xb, ya, y), (xa, x, y, yb), (x, xb, y, yb)] {
                let s = summe(x0, y0, x1, y1);
                if s[5] < 1.0 {
                    continue;
                }
                let m = s[3] / s[5];
                let var = s[4] / s[5] - m * m;
                if best.as_ref().is_none_or(|(v, _)| var < *v) {
                    best = Some((var, s));
                }
            }
            if let Some((_, s)) = best {
                let n = s[5];
                out.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([(s[0] / n).round() as u8, (s[1] / n).round() as u8, (s[2] / n).round() as u8, c[3]]),
                );
            }
        }
    }
    out
}

/// Farben reduzieren: k-Means im Lab-Raum auf den nicht durchsichtigen Punkten. Startwerte: die Farbe mittlerer
/// Helligkeit, dann jeweils die von allen bisherigen am weitesten entfernte (so bekommt auch eine kleine, auffällige
/// Farbe wie ein Licht ihre eigene). Ergebnisfarbe = Mittel der Bildpunkte (sRGB) jeder Gruppe.
fn farben_reduzieren(bild: &RgbaImage, k: usize) -> (Vec<[u8; 3]>, Vec<Option<usize>>) {
    let pix: Vec<Option<[f64; 3]>> = bild.pixels().map(|c| (c[3] >= 128).then(|| lab_px(c))).collect();
    let mut farbig: Vec<[f64; 3]> = pix.iter().flatten().copied().collect();
    if farbig.is_empty() {
        return (Vec::new(), vec![None; pix.len()]);
    }
    farbig.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let k = k.clamp(1, 32).min(farbig.len());
    let naechstes = |z: &[[f64; 3]], q: &[f64; 3]| -> usize {
        (0..z.len()).min_by(|&i, &j| abstand2(&z[i], q).total_cmp(&abstand2(&z[j], q))).unwrap()
    };
    // Stichprobe für die Iterationen (schnell), Zuordnung danach für alle
    let schritt = (farbig.len() / 20000).max(1);
    let probe: Vec<[f64; 3]> = farbig.iter().step_by(schritt).copied().collect();
    let mut zentren: Vec<[f64; 3]> = vec![probe[probe.len() / 2]];
    let mut naechste: Vec<f64> = probe.iter().map(|q| abstand2(&zentren[0], q)).collect();
    while zentren.len() < k {
        let (i, d) = naechste.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).unwrap();
        if *d < 1e-9 {
            break;
        }
        let neu = probe[i];
        zentren.push(neu);
        for (n, q) in naechste.iter_mut().zip(&probe) {
            *n = n.min(abstand2(&neu, q));
        }
    }
    let k = zentren.len();
    for _ in 0..15 {
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
                zentren[i] = [summe[i][0] / summe[i][3], summe[i][1] / summe[i][3], summe[i][2] / summe[i][3]];
            }
        }
    }
    let zuordnung: Vec<Option<usize>> = pix.iter().map(|q| q.map(|c| naechstes(&zentren, &c))).collect();
    // Farbe = Mittel der sRGB-Werte der Gruppe
    let mut summe = vec![[0u64; 4]; k];
    for (c, z) in bild.pixels().zip(&zuordnung) {
        if let Some(i) = z {
            for t in 0..3 {
                summe[*i][t] += u64::from(c[t]);
            }
            summe[*i][3] += 1;
        }
    }
    let farben = summe
        .iter()
        .map(|s| {
            let n = s[3].max(1) as f64;
            [0, 1, 2].map(|t| (s[t] as f64 / n).round() as u8)
        })
        .collect();
    (farben, zuordnung)
}

/// Bilddatei laden (PNG, JPG, BMP, GIF).
pub fn bild_laden(pfad: &std::path::Path) -> Result<RgbaImage, String> {
    Ok(image::open(pfad)
        .map_err(|e| format!("{}: {e}", pfad.display()))?
        .to_rgba8())
}

/// Vorbereitetes Bild: reduzierte Farben und die Farbe je Bildpunkt (Grundlage für Vorschau und Umrisse).
#[derive(Clone, Debug)]
pub struct Vorbereitet {
    pub breite: usize,
    pub hoehe: usize,
    /// Größe eines Bildpunkts in mm
    pub mm_px: f64,
    /// Farben (nach Häufigkeit, häufigste zuerst; keine doppelt)
    pub farben: Vec<[u8; 3]>,
    /// Bildpunkte je Farbe
    pub punkte: Vec<usize>,
    /// Farbe je Bildpunkt (zeilenweise; None = durchsichtig)
    pub zuordnung: Vec<Option<usize>>,
    /// Vorschlag zum Weglassen: weißer Hintergrund (Nummern in `farben`)
    pub vorschlag: Vec<usize>,
}

impl Vorbereitet {
    /// Farben, die weggelassen werden: Vorschlag oder die gewählten (gleiche Farbe, ±3 je Kanal)
    pub fn weggelassen(&self, o: &BildOptionen) -> Vec<usize> {
        match &o.weglassen {
            None => self.vorschlag.clone(),
            Some(w) => (0..self.farben.len())
                .filter(|&i| w.iter().any(|c| (0..3).all(|t| self.farben[i][t].abs_diff(c[t]) <= 3)))
                .collect(),
        }
    }

    /// Bild der reduzierten Farben (durchsichtig, wo das Bild durchsichtig ist).
    pub fn bild(&self) -> RgbaImage {
        let mut b = RgbaImage::new(self.breite as u32, self.hoehe as u32);
        for (i, z) in self.zuordnung.iter().enumerate() {
            if let Some(c) = z {
                let f = self.farben[*c];
                b.put_pixel((i % self.breite) as u32, (i / self.breite) as u32, Rgba([f[0], f[1], f[2], 255]));
            }
        }
        b
    }

    /// Vorschau als PNG.
    pub fn png(&self) -> Vec<u8> {
        let mut out = std::io::Cursor::new(Vec::new());
        self.bild().write_to(&mut out, image::ImageFormat::Png).expect("PNG schreiben");
        out.into_inner()
    }
}

/// Mehrheitsfilter 3 × 3: ein Punkt nimmt die Farbe an, die mindestens 5 der 9 Punkte haben.
fn mehrheit(z: &mut [Option<usize>], w: usize, h: usize) {
    let alt = z.to_vec();
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

/// Zusammenhängende Flächen (4er-Nachbarschaft) kleiner als `min_px` Punkte bekommen die Farbe, an die sie am
/// längsten grenzen (große Nachbarn zuerst) – so bleiben keine Löcher, wo früher kleine Flächen wegfielen. Mehrere
/// Durchgänge (eine kleine Fläche, die nur an andere kleine grenzt, wird im nächsten Durchgang mitgenommen).
fn kleine_zuschlagen(z: &mut [Option<usize>], w: usize, h: usize, min_px: usize) {
    if min_px <= 1 {
        return;
    }
    for _ in 0..6 {
        // Flächen nummerieren
        let mut nr = vec![u32::MAX; w * h];
        let mut groesse: Vec<usize> = Vec::new();
        let mut stapel: Vec<usize> = Vec::new();
        for s in 0..w * h {
            if nr[s] != u32::MAX || z[s].is_none() {
                continue;
            }
            let n = groesse.len() as u32;
            let c = z[s];
            nr[s] = n;
            stapel.push(s);
            let mut g = 0;
            while let Some(i) = stapel.pop() {
                g += 1;
                let (x, y) = (i % w, i / w);
                let mut nb = |j: usize| {
                    if nr[j] == u32::MAX && z[j] == c {
                        nr[j] = n;
                        stapel.push(j);
                    }
                };
                if x > 0 {
                    nb(i - 1);
                }
                if x + 1 < w {
                    nb(i + 1);
                }
                if y > 0 {
                    nb(i - w);
                }
                if y + 1 < h {
                    nb(i + w);
                }
            }
            groesse.push(g);
        }
        // Grenzlänge kleiner Flächen zu jeder Nachbarfarbe, getrennt nach großen und kleinen Nachbarn
        type Zaehler = Vec<(usize, usize)>;
        let mut grenze: std::collections::HashMap<u32, (Zaehler, Zaehler)> = std::collections::HashMap::new();
        let mut zaehlen = |a: usize, b: usize| {
            let (na, nb) = (nr[a], nr[b]);
            if na == nb || na == u32::MAX || groesse[na as usize] >= min_px {
                return;
            }
            if let Some(cb) = z[b] {
                let e = grenze.entry(na).or_default();
                let v = if nb != u32::MAX && groesse[nb as usize] >= min_px { &mut e.0 } else { &mut e.1 };
                match v.iter_mut().find(|(c, _)| *c == cb) {
                    Some(x) => x.1 += 1,
                    None => v.push((cb, 1)),
                }
            }
        };
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if x + 1 < w {
                    zaehlen(i, i + 1);
                    zaehlen(i + 1, i);
                }
                if y + 1 < h {
                    zaehlen(i, i + w);
                    zaehlen(i + w, i);
                }
            }
        }
        if grenze.is_empty() {
            return;
        }
        // Ziel: die Farbe mit der längsten Grenze zu einer großen Fläche; grenzt die kleine nur an kleine, deren
        // häufigste Farbe (kleine Nester lösen sich so über die Durchgänge auf)
        let neu: std::collections::HashMap<u32, usize> = grenze
            .iter()
            .map(|(n, (gross, klein))| {
                let v = if gross.is_empty() { klein } else { gross };
                (*n, v.iter().max_by_key(|x| (x.1, std::cmp::Reverse(x.0))).unwrap().0)
            })
            .collect();
        for i in 0..w * h {
            if let Some(c) = neu.get(&nr[i]) {
                z[i] = Some(*c);
            }
        }
    }
}

/// Bild vorbereiten: verkleinern, vereinfachen, Farben reduzieren (ggf. auf Garnfarben), glätten, kleine Flächen
/// zuschlagen, Vorschlag zum Weglassen bestimmen.
pub fn vorbereiten(bild: &RgbaImage, o: &BildOptionen) -> Vorbereitet {
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
    let mm_px = if w > 0 { o.breite_mm / w as f64 } else { 1.0 };
    let leer = Vorbereitet {
        breite: w,
        hoehe: h,
        mm_px,
        farben: Vec::new(),
        punkte: Vec::new(),
        zuordnung: vec![None; w * h],
        vorschlag: Vec::new(),
    };
    if w == 0 || h == 0 {
        return leer;
    }
    // vereinfachen: Stufe = Radius in Punkten, ab Stufe 3 zweimal
    let stufe = o.vereinfachen.min(5);
    if stufe > 0 {
        bild = kuwahara(&bild, stufe);
        if stufe >= 3 {
            bild = kuwahara(&bild, stufe - 1);
        }
    }
    let (mut farben, mut z) = farben_reduzieren(&bild, o.farben);
    // auf Garnfarben ziehen; Farben, die dasselbe Garn bekommen, fallen zusammen
    if let Some(pal) = o.palette.as_ref().filter(|p| !p.is_empty()) {
        let pal_lab: Vec<[f64; 3]> = pal.iter().map(|c| stick_kern::farbe::lab(*c)).collect();
        let garn: Vec<[u8; 3]> = farben
            .iter()
            .map(|c| {
                let l = stick_kern::farbe::lab(*c);
                let i = (0..pal.len())
                    .min_by(|&a, &b| {
                        stick_kern::farbe::de2000(l, pal_lab[a]).total_cmp(&stick_kern::farbe::de2000(l, pal_lab[b]))
                    })
                    .unwrap();
                pal[i]
            })
            .collect();
        let mut neu: Vec<[u8; 3]> = Vec::new();
        let abb: Vec<usize> = garn
            .iter()
            .map(|g| match neu.iter().position(|x| x == g) {
                Some(i) => i,
                None => {
                    neu.push(*g);
                    neu.len() - 1
                }
            })
            .collect();
        for x in z.iter_mut().flatten() {
            *x = abb[*x];
        }
        farben = neu;
    }
    for _ in 0..2 {
        mehrheit(&mut z, w, h);
    }
    let min_px = (o.min_flaeche / (mm_px * mm_px)).round().max(0.0) as usize;
    kleine_zuschlagen(&mut z, w, h, min_px);
    // nach Häufigkeit ordnen, ungenutzte Farben entfallen
    let mut punkte = vec![0usize; farben.len()];
    for c in z.iter().flatten() {
        punkte[*c] += 1;
    }
    let mut folge: Vec<usize> = (0..farben.len()).filter(|&i| punkte[i] > 0).collect();
    folge.sort_by_key(|&i| std::cmp::Reverse(punkte[i]));
    let mut neu_nr = vec![usize::MAX; farben.len()];
    for (n, &i) in folge.iter().enumerate() {
        neu_nr[i] = n;
    }
    for x in z.iter_mut().flatten() {
        *x = neu_nr[*x];
    }
    let farben: Vec<[u8; 3]> = folge.iter().map(|&i| farben[i]).collect();
    let punkte: Vec<usize> = folge.iter().map(|&i| punkte[i]).collect();
    // Vorschlag: häufigste Randfarbe, wenn sie fast weiß ist („white background omitted“)
    let mut rand = vec![0usize; farben.len()];
    for x in 0..w {
        for y in [0, h - 1] {
            if let Some(c) = z[y * w + x] {
                rand[c] += 1;
            }
        }
    }
    for y in 0..h {
        for x in [0, w - 1] {
            if let Some(c) = z[y * w + x] {
                rand[c] += 1;
            }
        }
    }
    let vorschlag = (0..farben.len())
        .max_by_key(|&c| rand[c])
        .filter(|&c| rand[c] > 0 && ist_weiss(farben[c]))
        .into_iter()
        .collect();
    Vorbereitet { farben, punkte, zuordnung: z, vorschlag, ..leer }
}

/// „fast weiß“: Helligkeit L* ≥ 90 und kaum Buntheit (C* ≤ 10) – eigene Festlegung
pub fn ist_weiss(c: [u8; 3]) -> bool {
    let [l, a, b] = stick_kern::farbe::lab(c);
    l >= 90.0 && a.hypot(b) <= 10.0
}

/// Flächen (Umriss + Löcher) einer Maske verfolgen: `drin(x, y)` = Bildpunkt gehört dazu.
fn flaechen_aus_maske(v: &Vorbereitet, o: &BildOptionen, farbe: [u8; 3], drin: &dyn Fn(usize, usize) -> bool) -> Vec<Flaeche> {
    let (w, h) = (v.breite, v.hoehe);
    let mm = v.mm_px;
    let zu_mm = |x: f64, y: f64| p((x - w as f64 / 2.0) * mm, (h as f64 / 2.0 - y) * mm);
    // Maske mit einem Punkt Rand (Konturen am Bildrand schließen)
    let mut maske = image::GrayImage::new(w as u32 + 2, h as u32 + 2);
    for y in 0..h {
        for x in 0..w {
            if drin(x, y) {
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
    // Reste unter 1 mm² (EasyStitch: kleinere Objekte entfallen) – größere sind schon zugeschlagen
    let min = easystitch::MIN_FLAECHE_MM2;
    let mut out = Vec::new();
    for (k, kt) in konturen.iter().enumerate() {
        if kt.border_type != BorderType::Outer {
            continue;
        }
        let umriss = ring(k);
        let a = flaeche(&umriss);
        if umriss.len() < 3 || a < min {
            continue;
        }
        let loecher: Vec<Linie> = konturen
            .iter()
            .enumerate()
            .filter(|(_, l)| l.border_type == BorderType::Hole && l.parent == Some(k))
            .map(|(i, _)| ring(i))
            .filter(|l| l.len() >= 3 && flaeche(l) >= min)
            .collect();
        let netto = a - loecher.iter().map(|l| flaeche(l)).sum::<f64>();
        out.push(Flaeche { farbe, umriss, loecher, flaeche: netto });
    }
    out
}

/// Umrisse der nicht weggelassenen Farben.
pub fn umrisse(v: &Vorbereitet, o: &BildOptionen) -> Vec<Flaeche> {
    let (w, z) = (v.breite, &v.zuordnung);
    let weg = v.weggelassen(o);
    let mut out = Vec::new();
    for (c, farbe) in v.farben.iter().enumerate() {
        if weg.contains(&c) {
            continue;
        }
        out.extend(flaechen_aus_maske(v, o, *farbe, &|x, y| z[y * w + x] == Some(c)));
    }
    out
}

/// **Zauberstab** („Magic Wand“, Handbuch S. 114–115: „click in the area … the shape is filled“): die zusammenhängende
/// Fläche gleicher Farbe am Bildpunkt (x, y) – Nachbarn über die Kanten (4er-Nachbarschaft), Löcher = Inseln anderer
/// Farben. `None` = durchsichtiger Punkt oder außerhalb. Verfahren eigene Festlegung (Code des Originals nicht gelesen).
pub fn bereich_bei(v: &Vorbereitet, o: &BildOptionen, x: usize, y: usize) -> Option<Flaeche> {
    let (w, h, z) = (v.breite, v.hoehe, &v.zuordnung);
    if x >= w || y >= h {
        return None;
    }
    let c = z[y * w + x]?;
    let mut drin = vec![false; w * h];
    let mut stapel = vec![(x, y)];
    drin[y * w + x] = true;
    while let Some((a, b)) = stapel.pop() {
        let nachbarn = [
            (a.wrapping_sub(1), b),
            (a + 1, b),
            (a, b.wrapping_sub(1)),
            (a, b + 1),
        ];
        for (nx, ny) in nachbarn {
            if nx < w && ny < h && !drin[ny * w + nx] && z[ny * w + nx] == Some(c) {
                drin[ny * w + nx] = true;
                stapel.push((nx, ny));
            }
        }
    }
    flaechen_aus_maske(v, o, v.farben[c], &|a, b| drin[b * w + a])
        .into_iter()
        .max_by(|a, b| a.flaeche.total_cmp(&b.flaeche))
}

/// Raster für PhotoSnap (`stick_kern::photosnap`): Bild auf `breite_mm` bei `mm_px` je Punkt verkleinern und je Punkt
/// die Deckung 0 … 255 berechnen – einfarbig: 255 − Helligkeit (Graustufen wie `ConvertToGrayScale`, Gewichte
/// 0,299/0,587/0,114), farbig: Cyan, Magenta, Gelb, Schwarz (`Split_CMYK`; Formel K = 1 − max(R, G, B), C = (1 − R − K)
/// / (1 − K) … – übliche Umrechnung, eigene Festlegung). Durchsichtige Punkte bekommen keinen Faden.
/// Ergebnis: (Breite, Höhe in Punkten, Lagen).
pub fn photo_raster(bild: &RgbaImage, breite_mm: f64, mm_px: f64, farbig: bool) -> (usize, usize, Vec<Vec<u8>>) {
    let w = ((breite_mm / mm_px).round() as u32).clamp(4, 2000);
    let h = ((f64::from(w) * f64::from(bild.height()) / f64::from(bild.width().max(1))).round() as u32).clamp(4, 2000);
    let klein = image::imageops::resize(bild, w, h, image::imageops::FilterType::Triangle);
    let n = (w * h) as usize;
    let mut lagen = vec![vec![0u8; n]; if farbig { 4 } else { 1 }];
    for (i, px) in klein.pixels().enumerate() {
        let [r, g, b, a] = px.0.map(|x| f64::from(x) / 255.0);
        let werte: Vec<f64> = if farbig {
            let k = 1.0 - r.max(g).max(b);
            if k >= 1.0 - 1e-9 {
                vec![0.0, 0.0, 0.0, 1.0]
            } else {
                vec![(1.0 - r - k) / (1.0 - k), (1.0 - g - k) / (1.0 - k), (1.0 - b - k) / (1.0 - k), k]
            }
        } else {
            vec![1.0 - (0.299 * r + 0.587 * g + 0.114 * b)]
        };
        for (l, v) in werte.iter().enumerate() {
            lagen[l][i] = (v * a * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
    (w as usize, h as usize, lagen)
}

/// Bild in Farbflächen zerlegen (`vorbereiten` + `umrisse`).
pub fn vektorisieren(bild: &RgbaImage, o: &BildOptionen) -> Vec<Flaeche> {
    umrisse(&vorbereiten(bild, o), o)
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
        // nächster Nachbar ab dem Ende der vorigen Farbe, dann 2-opt (Wege kürzen, solange sich zwei Strecken durch
        // Umdrehen des Stücks dazwischen verkürzen lassen) – eigene Festlegung; das Original ordnet „based on closest
        // join“ (Handbuch S. 116), Verfahren nicht gelesen
        let mut folge: Vec<Flaeche> = Vec::with_capacity(rest.len());
        let mut p0 = pos;
        while !rest.is_empty() {
            let i = (0..rest.len())
                .min_by(|&a, &b| {
                    let d = |k: usize| (mitte(&rest[k]).x - p0.x).hypot(mitte(&rest[k]).y - p0.y);
                    d(a).total_cmp(&d(b))
                })
                .unwrap();
            let x = rest.remove(i);
            p0 = mitte(&x);
            folge.push(x);
        }
        let mut m: Vec<Punkt> = std::iter::once(pos).chain(folge.iter().map(mitte)).collect();
        zwei_opt(&mut m, &mut folge);
        if let Some(l) = folge.last() {
            pos = mitte(l);
        }
        out.extend(folge);
    }
    out
}

/// 2-opt für einen offenen Weg mit festem Anfang `m[0]`: dreht Stücke um, solange der Weg kürzer wird. `m[k + 1]` ist
/// die Mitte von `f[k]`.
fn zwei_opt(m: &mut [Punkt], f: &mut [Flaeche]) {
    let d = |a: Punkt, b: Punkt| (a.x - b.x).hypot(a.y - b.y);
    let n = m.len();
    if n < 4 {
        return;
    }
    for _ in 0..50 {
        let mut besser = false;
        for i in 0..n - 2 {
            for j in i + 2..n {
                // Kanten (i, i+1) und (j, j+1) – am offenen Ende fehlt (j, j+1)
                let alt = d(m[i], m[i + 1]) + if j + 1 < n { d(m[j], m[j + 1]) } else { 0.0 };
                let neu = d(m[i], m[j]) + if j + 1 < n { d(m[i + 1], m[j + 1]) } else { 0.0 };
                if neu + 1e-9 < alt {
                    m[i + 1..=j].reverse();
                    f[i..j].reverse();
                    besser = true;
                }
            }
        }
        if !besser {
            break;
        }
    }
}

/// Design-JSON aus den Flächen. Stichart nach `fuellung`: „auto“ wie EasyStitch (Dicke ≤ 8 mm → Satinfläche,
/// darüber Tatami), „step“ alles Tatami; Werte der EasyStitch-Stile (Tatami-Abstand 0,45 mm, Zugausgleich 0,40 mm),
/// Unterlage an (Handbuch S. 116); Closest Join an (ebenda: „stitching sequence based on closest join“).
/// Zweites Ergebnis: Zahl der Objekte je Stichart.
pub fn design_json_mit(
    flaechen: &[Flaeche],
    name: &str,
    rahmen: &str,
    fuellung: Fuellung,
) -> (Value, std::collections::BTreeMap<String, usize>) {
    use easystitch::SATIN_BIS_MM;
    let mut klassen = std::collections::BTreeMap::new();
    let objekte: Vec<Value> = flaechen
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let satin = fuellung == Fuellung::Auto
                && stick_kern::saeulen::dicke(&f.umriss, &f.loecher).is_ok_and(|d| d <= SATIN_BIS_MM);
            *klassen.entry(if satin { "satinflaeche" } else { "tatami" }.to_string()).or_insert(0) += 1;
            flaeche_objekt(f, i + 1, satin)
        })
        .collect();
    // Auto Digitizer: „determines the stitching sequence based on closest join“ (Handbuch S. 116)
    (json!({"name": name, "rahmen": rahmen, "kuerzeste_verbindung": true, "objekte": objekte}), klassen)
}

/// Objekt-JSON einer Fläche wie beim Auto Digitizer: Satinfläche (`satin`) oder Step, Werte der EasyStitch-Stile,
/// Unterlage an. `nr` zählt im Namen.
pub fn flaeche_objekt(f: &Flaeche, nr: usize, satin: bool) -> Value {
    use easystitch::{TATAMI_ABSTAND_MM, ZUGAUSGLEICH_MM, punkte_json};
    let farbe = format!("#{:02x}{:02x}{:02x}", f.farbe[0], f.farbe[1], f.farbe[2]);
    let loecher: Vec<Value> = f.loecher.iter().map(|l| punkte_json(l)).collect();
    if satin {
        json!({"art": "satinflaeche", "name": format!("Satin {nr}"), "farbe": farbe,
               "umriss": punkte_json(&f.umriss), "loecher": loecher,
               "unterlage": "auto", "einstellungen": {"satin": {"zugausgleich": ZUGAUSGLEICH_MM}}})
    } else {
        json!({"art": "tatami", "name": format!("Fläche {nr}"), "farbe": farbe,
               "umriss": punkte_json(&f.umriss), "loecher": loecher, "unterlage": true,
               "einstellungen": {"tatami": {"abstand": TATAMI_ABSTAND_MM, "zugausgleich": ZUGAUSGLEICH_MM}}})
    }
}

/// Design-JSON mit der Standard-Stichart (Step).
pub fn design_json(flaechen: &[Flaeche], name: &str, rahmen: &str) -> Value {
    design_json_mit(flaechen, name, rahmen, Fuellung::default()).0
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

    /// Zauberstab: Klick auf den Ring liefert ihn mit Loch, Klick ins Quadrat nur das Quadrat, Klick auf durchsichtig nichts
    #[test]
    fn zauberstab_bereich() {
        let o = BildOptionen {
            breite_mm: 100.0,
            farben: 3,
            ..Default::default()
        };
        let v = vorbereiten(&testbild(), &o);
        let ring = bereich_bei(&v, &o, 75 * v.breite / 200, 50 * v.hoehe / 100).unwrap();
        assert_eq!(ring.loecher.len(), 1);
        assert!(ring.farbe[0] > 150, "{:?}", ring.farbe);
        let q = bereich_bei(&v, &o, 150 * v.breite / 200, 50 * v.hoehe / 100).unwrap();
        assert!(q.loecher.is_empty() && q.farbe[2] > 150);
        // 60 × 60 Punkte bei 0,5 mm je Punkt = 30 × 30 mm
        assert!((q.flaeche - 900.0).abs() < 60.0, "{}", q.flaeche);
        assert!(bereich_bei(&v, &o, 10_000, 0).is_none());
    }

    #[test]
    fn ring_und_quadrat() {
        let o = BildOptionen {
            breite_mm: 100.0,
            farben: 3,
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

    /// Rote Fläche mit verstreuten blauen Sprenkeln (2 × 2 Punkte) und weißem Rand
    fn sprenkel() -> RgbaImage {
        let mut b = RgbaImage::from_pixel(200, 100, Rgba([255, 255, 255, 255]));
        for y in 10..90u32 {
            for x in 10..190u32 {
                b.put_pixel(x, y, Rgba([200, 30, 30, 255]));
            }
        }
        for k in 0..30u32 {
            let (x, y) = (20 + (k * 37) % 160, 20 + (k * 23) % 60);
            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                b.put_pixel(x + dx, y + dy, Rgba([30, 60, 200, 255]));
            }
        }
        b
    }

    /// Kleine Flächen gehen in der Nachbarfarbe auf: keine Löcher, keine Sprenkel-Objekte
    #[test]
    fn kleine_flaechen_ohne_loecher() {
        let o = BildOptionen { breite_mm: 100.0, farben: 3, vereinfachen: 0, ..Default::default() };
        let v = vorbereiten(&sprenkel(), &o);
        assert_eq!(v.farben.len(), 2, "Blau ist zugeschlagen: {:?}", v.farben);
        let f = umrisse(&v, &o);
        assert_eq!(f.len(), 1, "nur die rote Fläche (Weiß weggelassen)");
        assert!(f[0].loecher.is_empty(), "keine Löcher");
    }

    /// Weiß mitsticken (leere Liste) und Weglassen nach Farbe
    #[test]
    fn weglassen_waehlbar() {
        let alle = BildOptionen { breite_mm: 100.0, farben: 3, weglassen: Some(Vec::new()), ..Default::default() };
        let v = vorbereiten(&testbild(), &alle);
        assert_eq!(v.vorschlag.len(), 1, "Vorschlag: weißer Hintergrund");
        assert!(ist_weiss(v.farben[v.vorschlag[0]]));
        // Weiß außen und Weiß im Ring, Rot, Blau
        assert_eq!(umrisse(&v, &alle).len(), 4, "Weiß wird mitgestickt");
        let rot = *v.farben.iter().find(|c| c[0] > 150 && c[1] < 100).unwrap();
        let ohne_rot = BildOptionen { weglassen: Some(vec![rot]), ..alle.clone() };
        let f = umrisse(&v, &ohne_rot);
        assert!(f.iter().all(|x| x.farbe != rot) && f.len() == 3);
    }

    /// Garnfarben: zwei ähnliche Rottöne bekommen dasselbe Garn und werden eine Farbe
    #[test]
    fn palette_fasst_zusammen() {
        let mut b = testbild();
        for y in 0..100u32 {
            for x in 0..60u32 {
                if b.get_pixel(x, y)[0] == 200 && x < 50 {
                    b.put_pixel(x, y, Rgba([215, 40, 35, 255]));
                }
            }
        }
        let o = BildOptionen {
            breite_mm: 100.0,
            farben: 4,
            vereinfachen: 0,
            palette: Some(vec![[255, 255, 255], [210, 30, 30], [30, 60, 200], [0, 0, 0]]),
            ..Default::default()
        };
        let v = vorbereiten(&b, &o);
        assert_eq!(v.farben.len(), 3, "{:?}", v.farben);
        assert!(v.farben.contains(&[210, 30, 30]) && v.farben.contains(&[30, 60, 200]));
    }

    /// Vereinfachen glättet Rauschen, Kanten bleiben
    #[test]
    fn kuwahara_glaettet() {
        let mut b = testbild();
        for (i, px) in b.pixels_mut().enumerate() {
            let r = ((i * 7919) % 41) as i32 - 20;
            for t in 0..3 {
                px[t] = (i32::from(px[t]) + r).clamp(0, 255) as u8;
            }
        }
        let g = kuwahara(&b, 2);
        // Mitte des Quadrats fast einfarbig, Kante bei x = 120 bleibt scharf
        let streu = |img: &RgbaImage| {
            let v: Vec<f64> = (130..170).map(|x| f64::from(img.get_pixel(x, 50)[2])).collect();
            let m = v.iter().sum::<f64>() / v.len() as f64;
            v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len() as f64
        };
        assert!(streu(&g) < streu(&b) / 4.0, "{} vs {}", streu(&g), streu(&b));
        assert!(g.get_pixel(118, 50)[0] > 200 && g.get_pixel(122, 50)[2] > 150, "Kante bleibt");
    }
}
