//! **PhotoSnap** (Handbuch S. 119–120; `Analyse_Teil3/25_automatik.md` §3): ein Foto als Reihen von Zickzackstichen,
//! deren Abstand der Helligkeit folgt – „rows of Satin stitches with varied spacing settings. The effect resembles
//! the output of a line printer.“
//!
//! Aus dem Original (MOD1 `PhotoStitchObject::GenerateComponents` 0x10120aa0, `PhotoValues` 0x10122d50/0x101230d0,
//! NORMAL.AMT DataItem 0x3ee4):
//! - **Einfarbig** (Standard, 0x3ee9 = 1): Bild in Graustufen (`ConvertToGrayScale`), eine Lage. **Farbe:** Bild in
//!   vier Lagen zerlegt (`DibSplit::Split_CMYK`): Cyan, Magenta, Gelb, Schwarz.
//! - **Hintergrund** hell/dunkel (0x3ee8, Standard hell): „Light: applies the Max Width value to the lightest part“.
//! - **Winkel** der Reihen (0x3eeb, Vorgabe 0 im Konstruktor; das Handbuch-Bild zeigt 45°). Je Farblage kommt bei
//!   Stichart 0x2828 im Modus 1 ein Winkel dazu: **0°, 45°, 90°, 135°** (0x10120890, Konstanten π/4, π/2, 3π/4); im
//!   Modus 2 bleibt der Winkel und die Lagen sind quer um **0 / 0,4 / 0,8 / 1,2 mm** versetzt (0x101209e0: Punkte
//!   (0, 72/144/216)).
//! - Stichart 0x2828 (Stil: 0x282a = 144 = 0,8 mm, 0x282f = 1800 = 10 mm – als kleinster/größter Abstand gedeutet,
//!   Zuordnung niedrig).
//!
//! **Eigene Festlegungen:** Reihenbreite grob/mittel/fein = 3,0 / 2,0 / 1,0 mm (2,0 mm = NORMAL.AMT 0x3ee5 = 360,
//! Zuordnung niedrig); Abstand = kleinster Abstand / Deckung, begrenzt auf den größten; Deckung = Mittel aus drei
//! Proben quer zur Reihe; Reihen abwechselnd hin und zurück; Farblagen in der Reihenfolge C, M, Y, K.

use crate::geometrie::{Linie, Punkt, p};

/// Reihenbreite je Auflösung (mm): grob, mittel, fein
pub const REIHE_GROB: f64 = 3.0;
pub const REIHE_MITTEL: f64 = 2.0;
pub const REIHE_FEIN: f64 = 1.0;
/// kleinster Abstand (volle Deckung), Stil 0x2828 Feld 0x282a = 144
pub const ABSTAND_MIN: f64 = 0.8;
/// größter Abstand (hellste Stelle), Stil 0x2828 Feld 0x282f = 1800
pub const ABSTAND_MAX: f64 = 10.0;
/// Versatz der Farblagen quer zur Reihe im Modus „versetzt“ (MOD1 0x101209e0: 72 Einheiten je Lage)
pub const LAGEN_VERSATZ: f64 = 0.4;
/// Zusatzwinkel der Farblagen im Modus „gekreuzt“ (MOD1 0x10120890)
pub const LAGEN_WINKEL: [f64; 4] = [0.0, 45.0, 90.0, 135.0];

/// Eine Farblage: Deckung je Bildpunkt (0 = kein Faden, 255 = voll; für hellen Stoff), zeilenweise von oben.
#[derive(Clone, Debug, PartialEq)]
pub struct PhotoLage {
    pub farbe: String,
    pub werte: Vec<u8>,
}

/// Vorgaben eines PhotoSnap-Objekts (mm; Bildmitte = Ursprung, +y nach oben).
#[derive(Clone, Debug, PartialEq)]
pub struct PhotoVorgabe {
    pub breite_px: usize,
    pub hoehe_px: usize,
    /// Größe eines Bildpunkts in mm
    pub mm_px: f64,
    pub lagen: Vec<PhotoLage>,
    /// Grundwinkel der Reihen in Grad
    pub winkel: f64,
    /// Breite einer Reihe in mm
    pub reihe: f64,
    pub dunkel: bool,
    /// Farblagen versetzt statt gekreuzt
    pub versetzt: bool,
    pub abstand_min: f64,
    pub abstand_max: f64,
}

impl PhotoVorgabe {
    /// Deckung 0 … 1 an einem Punkt (mm) der Lage `l`; außerhalb 0
    fn deckung(&self, l: usize, q: Punkt) -> f64 {
        let x = (q.x / self.mm_px + self.breite_px as f64 / 2.0).floor();
        let y = (self.hoehe_px as f64 / 2.0 - q.y / self.mm_px).floor();
        if x < 0.0 || y < 0.0 || x >= self.breite_px as f64 || y >= self.hoehe_px as f64 {
            return 0.0;
        }
        let v = f64::from(self.lagen[l].werte[y as usize * self.breite_px + x as usize]) / 255.0;
        if self.dunkel { 1.0 - v } else { v }
    }

    /// Breite und Höhe in mm
    pub fn groesse(&self) -> (f64, f64) {
        (self.breite_px as f64 * self.mm_px, self.hoehe_px as f64 * self.mm_px)
    }
}

/// Bereich [lo, hi] des Parameters t, für den a + t·d im Rechteck |x| ≤ w/2, |y| ≤ h/2 liegt (Liang–Barsky).
fn im_rechteck(a: Punkt, d: Punkt, w: f64, h: f64) -> Option<(f64, f64)> {
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for (pa, pd, g) in [(a.x, d.x, w / 2.0), (a.y, d.y, h / 2.0)] {
        if pd.abs() < 1e-12 {
            if pa.abs() > g {
                return None;
            }
            continue;
        }
        let (t1, t2) = ((-g - pa) / pd, (g - pa) / pd);
        lo = lo.max(t1.min(t2));
        hi = hi.min(t1.max(t2));
    }
    (lo < hi).then_some((lo, hi))
}

/// Stichpunkte einer Lage: Reihen quer abwechselnd oben/unten, Abstand nach der Deckung.
pub fn lage_stiche(v: &PhotoVorgabe, l: usize) -> Linie {
    let (w, h) = v.groesse();
    let zusatz = if v.versetzt { 0.0 } else { LAGEN_WINKEL[l % 4] };
    let a = (v.winkel + zusatz).to_radians();
    let d = p(a.cos(), a.sin());
    let n = p(-a.sin(), a.cos());
    let quer = if v.versetzt { LAGEN_VERSATZ * (l % 4) as f64 } else { 0.0 };
    let r = v.reihe.max(0.3);
    // Bereich quer zu den Reihen: Projektion der Ecken auf n
    let ecken = [p(-w / 2.0, -h / 2.0), p(w / 2.0, -h / 2.0), p(w / 2.0, h / 2.0), p(-w / 2.0, h / 2.0)];
    let vmin = ecken.iter().map(|e| e.x * n.x + e.y * n.y).fold(f64::INFINITY, f64::min);
    let vmax = ecken.iter().map(|e| e.x * n.x + e.y * n.y).fold(f64::NEG_INFINITY, f64::max);
    let start = vmin + quer.rem_euclid(r) - r;
    let zahl = ((vmax - start) / r).ceil() as usize;
    let at = |u: f64, s: f64| p(u * d.x + s * n.x, u * d.y + s * n.y);
    let mut out: Linie = Vec::new();
    let mut rueck = false;
    for k in 0..zahl {
        let (s0, s1) = (start + k as f64 * r, start + (k + 1) as f64 * r);
        // beide Ränder der Reihe müssen im Bild liegen
        let (Some(a0), Some(a1)) = (im_rechteck(at(0.0, s0), d, w, h), im_rechteck(at(0.0, s1), d, w, h)) else {
            continue;
        };
        let (lo, hi) = (a0.0.max(a1.0), a0.1.min(a1.1));
        if hi - lo < v.abstand_min {
            continue;
        }
        let sm = (s0 + s1) / 2.0;
        let abstand = |u: f64| {
            let c = (v.deckung(l, at(u, sm)) + v.deckung(l, at(u, s0 + r / 4.0)) + v.deckung(l, at(u, s1 - r / 4.0)))
                / 3.0;
            if c <= v.abstand_min / v.abstand_max { v.abstand_max } else { (v.abstand_min / c).min(v.abstand_max) }
        };
        let mut reihe: Linie = Vec::new();
        let mut u = lo;
        let mut oben = true;
        loop {
            reihe.push(at(u, if oben { s1 } else { s0 }));
            oben = !oben;
            if u >= hi {
                break;
            }
            // „Abstand“ = gleiche Seite (Bericht 04): je Einstich der halbe Abstand
            u = (u + abstand(u) / 2.0).min(hi);
        }
        if rueck {
            reihe.reverse();
        }
        rueck = !rueck;
        out.extend(reihe);
    }
    out
}

/// Base64 (Standardalphabet, mit oder ohne „=“) entschlüsseln.
pub fn base64_lesen(t: &str) -> Result<Vec<u8>, String> {
    let wert = |c: u8| -> Result<u32, String> {
        Ok(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return Err(format!("Base64: unerwartetes Zeichen „{}“", c as char)),
        } as u32)
    };
    let z: Vec<u8> = t.bytes().filter(|c| !c.is_ascii_whitespace() && *c != b'=').collect();
    let mut out = Vec::with_capacity(z.len() * 3 / 4);
    for ch in z.chunks(4) {
        let mut n = 0u32;
        for (i, &c) in ch.iter().enumerate() {
            n |= wert(c)? << (18 - 6 * i);
        }
        let bytes = n.to_be_bytes();
        out.extend_from_slice(&bytes[1..ch.len().max(1)]);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vorgabe(werte: Vec<u8>, w: usize, h: usize) -> PhotoVorgabe {
        PhotoVorgabe {
            breite_px: w,
            hoehe_px: h,
            mm_px: 0.5,
            lagen: vec![PhotoLage { farbe: "#000000".into(), werte }],
            winkel: 0.0,
            reihe: REIHE_MITTEL,
            dunkel: false,
            versetzt: false,
            abstand_min: ABSTAND_MIN,
            abstand_max: ABSTAND_MAX,
        }
    }

    /// Links schwarz, rechts weiß: links dicht (0,8 mm), rechts weit (10 mm); alles im Bild
    #[test]
    fn dicht_wo_dunkel() {
        let (w, h) = (80, 20); // 40 × 10 mm
        let werte: Vec<u8> = (0..w * h).map(|i| if i % w < w / 2 { 255 } else { 0 }).collect();
        let v = vorgabe(werte, w, h);
        let s = lage_stiche(&v, 0);
        assert!(s.iter().all(|q| q.x.abs() <= 20.0 + 1e-6 && q.y.abs() <= 5.0 + 1e-6));
        let links = s.iter().filter(|q| q.x < -1.0).count();
        let rechts = s.iter().filter(|q| q.x > 1.0).count();
        assert!(links > 8 * rechts, "{links} {rechts}");
        // 5 Reihen à 2 mm
        let ys: std::collections::BTreeSet<i64> = s.iter().map(|q| (q.y * 10.0).round() as i64).collect();
        assert_eq!(ys.len(), 6, "{ys:?}");
        // dunkler Stoff kehrt um
        let mut d = v.clone();
        d.dunkel = true;
        let s2 = lage_stiche(&d, 0);
        assert!(s2.iter().filter(|q| q.x > 1.0).count() > 8 * s2.iter().filter(|q| q.x < -1.0).count());
    }

    #[test]
    fn base64() {
        assert_eq!(base64_lesen("TWFu").unwrap(), b"Man");
        assert_eq!(base64_lesen("TWE=").unwrap(), b"Ma");
        assert_eq!(base64_lesen("TQ==").unwrap(), b"M");
        assert!(base64_lesen("T$").is_err());
    }
}
