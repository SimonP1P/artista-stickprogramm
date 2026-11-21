//! Stickbarkeit einer Schrift: wie dünn und wie dick ihre Striche im Verhältnis zur Buchstabenhöhe sind, daraus
//! „stickbar ab … mm“ (Wunsch Nutzer 21.11.2026). **Eigene Erweiterung** – das Original misst das nicht; es nennt je
//! Alphabet nur eine empfohlene Höhe (`alphabets.md`). Die Grenze für den dünnsten Strich (`MIN_STRICH_MM`) ist an
//! genau diesen Empfehlungen kalibriert (`Analyse_Teil3/21_schriften.md`).
//!
//! Verfahren: Buchstaben einzeln rastern (Großbuchstabenhöhe = `PX_HOEHE` Pixel), euklidische Distanztransformation,
//! **örtliche Strichstärke** je Pixel = Durchmesser des größten einbeschriebenen Kreises, der das Pixel enthält (local
//! thickness). Ausgewertet nach **Strichlänge** (Gewicht 1/Stärke, weil Fläche = Stärke × Länge): `duenn` = 10 %-Wert,
//! `dick` = 90 %-Wert. Ecken und Enden dicker Striche liegen im Kreis des Strichs und zählen deshalb nicht als dünn.

/// Eine Fläche aus geschlossenen Ringen; `gerade_ungerade`: Löcher nach Gerade-ungerade-Regel (Alphabete), sonst
/// Nonzero (TrueType: überlappende Konturen variabler Schriften werden vereinigt).
#[derive(Clone, Debug, Default)]
pub struct Flaeche {
    pub ringe: Vec<Vec<(f64, f64)>>,
    pub gerade_ungerade: bool,
}

/// Strichstärken in Teilen der Bezugshöhe (Großbuchstaben).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strichstaerken {
    /// dünne Striche: 10 % der Strichlänge sind dünner
    pub duenn: f64,
    /// dicke Striche: 10 % der Strichlänge sind dicker
    pub dick: f64,
}

/// Dünnster Strich (mm), den das Original bei der kleinsten empfohlenen Höhe seiner Alphabete noch stickt: Median über
/// die 47 Alphabete mit Höhenangabe von `duenn × min_hoehe` = 0,78 mm (mittlere Hälfte 0,59–0,92 mm), gerundet (Messung `stick stickbarkeit --kalibrieren`,
/// `Analyse_Teil3/21_schriften.md` §3). Abgeleitet, Sicherheit mittel.
pub const MIN_STRICH_MM: f64 = 0.8;

/// Probezeichen für die Messung (Groß- und Kleinbuchstaben mit geraden, runden und schrägen Strichen)
pub const PROBE: &str = "AHMOBEGRSaegmnost";

/// Rasterhöhe der Großbuchstaben in Pixeln
const PX_HOEHE: f64 = 300.0;

/// Kleinste Höhe (mm), ab der die Schrift gut stickbar ist: dünne Striche mindestens `MIN_STRICH_MM`; auf 1 mm
/// aufgerundet.
pub fn stickbar_ab(s: &Strichstaerken) -> f64 {
    if s.duenn <= 0.0 {
        return f64::INFINITY;
    }
    (MIN_STRICH_MM / s.duenn).ceil()
}

/// Strichstärken einer Schrift: je Zeichen eine Liste von Flächen (werden vereinigt), `bezug` = Höhe der
/// Großbuchstaben in denselben Einheiten. `None`, wenn nichts gefüllt ist.
pub fn strichstaerken(zeichen: &[Vec<Flaeche>], bezug: f64) -> Option<Strichstaerken> {
    if bezug <= 0.0 {
        return None;
    }
    let s = PX_HOEHE / bezug;
    let mut werte: Vec<(f32, f32)> = Vec::new();
    for z in zeichen {
        werte.extend(zeichen_staerken(z, s));
    }
    if werte.is_empty() {
        return None;
    }
    werte.sort_by(|a, b| a.0.total_cmp(&b.0));
    let summe: f64 = werte.iter().map(|w| f64::from(w.1)).sum();
    let anteil = |q: f64| -> f64 {
        let ziel = summe * q;
        let mut acc = 0.0;
        for &(t, w) in &werte {
            acc += f64::from(w);
            if acc >= ziel {
                return f64::from(t);
            }
        }
        f64::from(werte.last().map_or(0.0, |w| w.0))
    };
    Some(Strichstaerken {
        duenn: anteil(0.1) / PX_HOEHE,
        dick: anteil(0.9) / PX_HOEHE,
    })
}

/// (Stärke in Pixeln, Gewicht) je gefülltem Pixel eines Zeichens.
fn zeichen_staerken(flaechen: &[Flaeche], s: f64) -> Vec<(f32, f32)> {
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for f in flaechen {
        for r in &f.ringe {
            for &(x, y) in r {
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
        }
    }
    if x0 > x1 || y0 > y1 {
        return Vec::new();
    }
    let rand = 2.0;
    let w = ((x1 - x0) * s + 2.0 * rand).ceil() as usize + 1;
    let h = ((y1 - y0) * s + 2.0 * rand).ceil() as usize + 1;
    if w * h > 4_000_000 {
        return Vec::new();
    }
    let px = |x: f64| (x - x0) * s + rand;
    let py = |y: f64| (y - y0) * s + rand;
    let mut innen = vec![false; w * h];
    for f in flaechen {
        let ringe: Vec<Vec<(f64, f64)>> = f
            .ringe
            .iter()
            .map(|r| r.iter().map(|&(x, y)| (px(x), py(y))).collect())
            .collect();
        fuellen(&ringe, f.gerade_ungerade, w, h, &mut innen);
    }
    let d = distanz(&innen, w, h);
    // Mittelpunkte größter Kreise: Pixel, deren Kreis in keinem Kreis eines Nachbarn liegt (r_n < r + Abstand;
    // Toleranz 0,15 Pixel für schräge Kanten im Raster) – die Mittellinien der Striche
    let mut staerke = vec![0f32; w * h];
    let at = |x: isize, y: isize| -> f32 {
        if x < 0 || y < 0 || x as usize >= w || y as usize >= h {
            0.0
        } else {
            d[y as usize * w + x as usize]
        }
    };
    let mut kaemme: Vec<(usize, usize, f32)> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let r = d[y * w + x];
            if r <= 0.0 {
                continue;
            }
            let (xi, yi) = (x as isize, y as isize);
            let enthalten = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
                .iter()
                .any(|&(dx, dy): &(isize, isize)| {
                    let abstand = if dx != 0 && dy != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
                    at(xi + dx, yi + dy) >= r + abstand - 0.15
                });
            if !enthalten {
                kaemme.push((x, y, r));
            }
        }
    }
    // große Kreise zuerst: kleinere ändern dort nichts mehr
    kaemme.sort_by(|a, b| b.2.total_cmp(&a.2));
    for &(cx, cy, r) in &kaemme {
        let t = (2.0 * r - 0.5).max(1.0);
        let ri = r.ceil() as isize;
        let r2 = r * r;
        for dy in -ri..=ri {
            let y = cy as isize + dy;
            if y < 0 || y as usize >= h {
                continue;
            }
            for dx in -ri..=ri {
                let x = cx as isize + dx;
                if x < 0 || x as usize >= w || ((dx * dx + dy * dy) as f32) >= r2 {
                    continue;
                }
                let i = y as usize * w + x as usize;
                if staerke[i] < t {
                    staerke[i] = t;
                }
            }
        }
    }
    let mut out = Vec::new();
    for i in 0..w * h {
        if innen[i] {
            let t = if staerke[i] > 0.0 {
                staerke[i]
            } else {
                (2.0 * d[i] - 0.5).max(1.0)
            };
            out.push((t, 1.0 / t));
        }
    }
    out
}

/// Ringe füllen (Pixelmitten), Nonzero oder Gerade-ungerade; ODER in `innen`.
fn fuellen(ringe: &[Vec<(f64, f64)>], gerade_ungerade: bool, w: usize, h: usize, innen: &mut [bool]) {
    let mut kanten: Vec<((f64, f64), (f64, f64))> = Vec::new();
    for r in ringe {
        if r.len() < 3 {
            continue;
        }
        for i in 0..r.len() {
            let a = r[i];
            let b = r[(i + 1) % r.len()];
            if a.1 != b.1 {
                kanten.push((a, b));
            }
        }
    }
    let mut schnitte: Vec<(f64, i32)> = Vec::new();
    for y in 0..h {
        let yc = y as f64 + 0.5;
        schnitte.clear();
        for &(a, b) in &kanten {
            let (lo, hi, dir) = if a.1 < b.1 { (a, b, 1) } else { (b, a, -1) };
            if yc >= lo.1 && yc < hi.1 {
                let t = (yc - lo.1) / (hi.1 - lo.1);
                schnitte.push((lo.0 + t * (hi.0 - lo.0), dir));
            }
        }
        schnitte.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut wind = 0i32;
        for k in 0..schnitte.len() {
            wind += if gerade_ungerade { 1 } else { schnitte[k].1 };
            let drin = if gerade_ungerade { wind % 2 != 0 } else { wind != 0 };
            if drin && k + 1 < schnitte.len() {
                let xa = (schnitte[k].0 - 0.5).ceil().max(0.0) as usize;
                let xb = ((schnitte[k + 1].0 - 0.5).ceil().max(0.0) as usize).min(w);
                for x in xa..xb {
                    innen[y * w + x] = true;
                }
            }
        }
    }
}

/// Abstand jedes gefüllten Pixels zum nächsten leeren (Pixelmitten, euklidisch; Felzenszwalb–Huttenlocher).
fn distanz(innen: &[bool], w: usize, h: usize) -> Vec<f32> {
    const UNENDLICH: f64 = 1e20;
    let mut f: Vec<f64> = innen.iter().map(|&b| if b { UNENDLICH } else { 0.0 }).collect();
    let mut zeile = vec![0.0; w.max(h)];
    let mut erg = vec![0.0; w.max(h)];
    for x in 0..w {
        for y in 0..h {
            zeile[y] = f[y * w + x];
        }
        dt1(&zeile[..h], &mut erg[..h]);
        for y in 0..h {
            f[y * w + x] = erg[y];
        }
    }
    for y in 0..h {
        zeile[..w].copy_from_slice(&f[y * w..(y + 1) * w]);
        dt1(&zeile[..w], &mut erg[..w]);
        f[y * w..(y + 1) * w].copy_from_slice(&erg[..w]);
    }
    f.into_iter().map(|q| q.sqrt() as f32).collect()
}

/// Eindimensionale quadrierte Distanztransformation (untere Hülle der Parabeln).
fn dt1(f: &[f64], d: &mut [f64]) {
    let n = f.len();
    if n == 0 {
        return;
    }
    let schnitt = |q: usize, p: usize| {
        ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2.0 * (q as f64 - p as f64))
    };
    let mut v = vec![0usize; n];
    let mut z = vec![0f64; n + 1];
    let mut k = 0usize;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let mut s = schnitt(q, v[k]);
        // z[0] = −∞ → endet spätestens bei k = 0
        while s <= z[k] {
            k -= 1;
            s = schnitt(q, v[k]);
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, dq) in d.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        *dq = (q as f64 - p as f64).powi(2) + f[p];
    }
}

/// Flächen eines Original-Alphabets: je Zeichen die geschlossenen Umrisse jedes Objekts (Satinsäulen, Flächen mit
/// Löchern); offene Linien (Läufe) zählen nicht. Bezug: Referenzhöhe 50 mm in Alphabet-Einheiten.
pub fn alphabet_flaechen(a: &crate::schrift::Alphabet, probe: &str) -> (Vec<Vec<Flaeche>>, f64) {
    let mut out = Vec::new();
    for c in probe.chars() {
        let Some(z) = a.zeichen.get(&c) else { continue };
        let mut fl = Vec::new();
        for o in &z.objekte {
            let ringe: Vec<Vec<(f64, f64)>> = o
                .umrisse
                .iter()
                .filter(|u| u.geschlossen)
                .map(crate::schrift::umriss_punkte)
                .filter(|r| r.len() >= 3)
                .collect();
            if !ringe.is_empty() {
                fl.push(Flaeche {
                    ringe,
                    gerade_ungerade: true,
                });
            }
        }
        if !fl.is_empty() {
            out.push(fl);
        }
    }
    (out, crate::schrift::REFERENZ_HOEHE_MM * crate::schrift::EINHEIT)
}

/// Flächen einer Computer-Schrift: je Zeichen alle Ringe als eine Fläche (Nonzero). Bezug: Versalhöhe.
pub fn ttf_flaechen(s: &crate::schrift_ttf::TtfSchrift, probe: &str) -> (Vec<Vec<Flaeche>>, f64) {
    let out = probe
        .chars()
        .filter_map(|c| s.zeichen.get(&c))
        .filter(|z| !z.ringe.is_empty())
        .map(|z| {
            vec![Flaeche {
                ringe: z.ringe.clone(),
                gerade_ungerade: false,
            }]
        })
        .collect();
    (out, s.versal_hoehe)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rechteck(x: f64, y: f64, b: f64, h: f64) -> Vec<(f64, f64)> {
        vec![(x, y), (x + b, y), (x + b, y + h), (x, y + h)]
    }

    #[test]
    fn balken_gleichmaessig() {
        // Balken 30 breit, 300 hoch, Bezug 300 → Stärke 0,1
        let z = vec![vec![Flaeche {
            ringe: vec![rechteck(0.0, 0.0, 30.0, 300.0)],
            gerade_ungerade: false,
        }]];
        let s = strichstaerken(&z, 300.0).unwrap();
        assert!((s.duenn - 0.1).abs() < 0.01, "{s:?}");
        assert!((s.dick - 0.1).abs() < 0.01, "{s:?}");
    }

    #[test]
    fn haarstrich_wird_erkannt() {
        // „H“ mit dicken Stämmen (40) und dünnem Querbalken (6), Querbalken lang → dünn ≈ 0,02, dick ≈ 0,13
        let z = vec![vec![Flaeche {
            ringe: vec![
                rechteck(0.0, 0.0, 40.0, 300.0),
                rechteck(240.0, 0.0, 40.0, 300.0),
                rechteck(40.0, 147.0, 200.0, 6.0),
            ],
            gerade_ungerade: false,
        }]];
        let s = strichstaerken(&z, 300.0).unwrap();
        assert!(s.duenn < 0.03, "{s:?}");
        assert!((s.dick - 0.133).abs() < 0.015, "{s:?}");
        assert!(stickbar_ab(&s) > 30.0);
    }

    #[test]
    fn ueberlappung_nonzero_und_loch() {
        // zwei überlappende Balken gleicher Richtung (Nonzero) = ein dicker Balken; Ring mit Loch (gerade-ungerade)
        let z = vec![vec![Flaeche {
            ringe: vec![rechteck(0.0, 0.0, 20.0, 300.0), rechteck(10.0, 0.0, 20.0, 300.0)],
            gerade_ungerade: false,
        }]];
        let s = strichstaerken(&z, 300.0).unwrap();
        assert!((s.duenn - 0.1).abs() < 0.01, "{s:?}");
        let ring = vec![vec![Flaeche {
            ringe: vec![rechteck(0.0, 0.0, 300.0, 300.0), rechteck(30.0, 30.0, 240.0, 240.0)],
            gerade_ungerade: true,
        }]];
        let s = strichstaerken(&ring, 300.0).unwrap();
        // Rahmen 30 breit: in den Ecken passt ein Kreis mit Durchmesser 30·√2·2/(1+√2) ≈ 35 → dick bis 0,117
        assert!((s.duenn - 0.1).abs() < 0.01 && s.dick < 0.125, "{s:?}");
    }
}
