//! Handstiche der Craft-Stitch-Werkzeugleiste: **Rückstich** (Backstitch, Kontur-Stichart 0x2850) und **Stielstich**
//! (Stemstitch, 0x2855). Beide Verfahren sind aus MOD1.DLL nachgebaut (`Analyse_Teil3/24_craft_stitches.md` §2–3).
//!
//! **Rückstich** (`BackStitchFill::AddRunPoint`, MOD1 0x1001d960 – Sicherheit hoch):
//! - Die Linie wird erst als Laufstich mit der Stichlänge geteilt (`DoRun` 0x1001d8e0 → Laufgenerator mit Ecken).
//! - Je Laufpunkt P (Vorgänger Q, D = P − Q, o = Fadenstärke · Normale von D) und Rückpunkt M:
//!   - 3 Striche: P + o, M − o, P. Liegt Q weiter als 1,33 · Stärke von der Strecke (P + o, M − o) weg (Ecke), rückt
//!     M vorher auf die Mitte zwischen M und Q.
//!   - 5 Striche: P + o, M + o/2, P − o, M − o/2, P.
//!   - Danach M = P − f · D mit f = Überlappung (am ersten Stich f / (f + 1)); am Ende M = P.
//! - Werte aus dem Stil `<0 Software Defaults>` 0x2850: Länge 0x2851 = 540 (3,0 mm), Stärke 0x2852 = 36 (0,2 mm),
//!   Überlappung 0x2853 = 0,5, Striche 0x2854 = 3. Dialog „Backstitch“ (BERESENU DLG 1448).
//!
//! **Stielstich** (`StemStitchFill`, MOD1 0x1019d330/0x1019d4d0/0x1019d9e0/0x1019d6a0 – Sicherheit mittel):
//! - Laufpunkte C im Schritt L = Abstand / sin(Winkel).
//! - Je Punkt ein schräger Stich über die Linienbreite: s = h · n + (h / tan a) · u (h = halbe Linienstärke, u =
//!   Richtung, n = Normale, a = Winkel); Stiche C + s → C − s.
//! - In engen Kurven (Umkreisradius r < 4 · Breite, über drei Punkte gemittelt) wird der Winkel steiler:
//!   a' = a + (90° − a) · min(1, Breite / r − 0,25).
//! - „Triple“: C + s, C − s + o, C + s − o, C − s mit o = Fadenstärke · Normale von s (drei Fäden nebeneinander).
//! - Werte aus dem Stil 0x2855: Linienstärke 0x2857 = 360 (2,0 mm), Abstand 0x2859 = 270 (1,5 mm), Winkel 0x285a =
//!   0,8727 rad (50°), Art 0x2858 = 3 (= dreifach; 2 = einfach), Fadenstärke 0x2856 = 36 (0,2 mm). Zuordnung Linienstärke
//!   ↔ Abstand über die Verwendung im Code (Breite im Generator, Abstand / sin a als Schrittweite) – mittel.

use crate::einstellungen::LaufEinstellung;
use crate::geometrie::{Linie, Punkt, abstand_punkt_strecke, p};
use crate::sticharten::lauf::laufstich_mit_ecken;

/// Rückstich: Stichlänge 3,0 mm (0x2851 = 540)
pub const RUECK_LAENGE: f64 = 3.0;
/// Rückstich: Fadenstärke 0,2 mm (0x2852 = 36) – Abstand der Hin- und Rückstiche
pub const RUECK_STAERKE: f64 = 0.2;
/// Rückstich: Überlappung 50 % (0x2853 = 0,5)
pub const RUECK_UEBERLAPPUNG: f64 = 0.5;
/// Rückstich: 3 Striche (0x2854)
pub const RUECK_STRICHE: u8 = 3;
/// Stielstich: Linienstärke 2,0 mm (0x2857 = 360)
pub const STIEL_BREITE: f64 = 2.0;
/// Stielstich: Abstand 1,5 mm (0x2859 = 270)
pub const STIEL_ABSTAND: f64 = 1.5;
/// Stielstich: Winkel 50° (0x285a = 0,8727 rad)
pub const STIEL_WINKEL: f64 = 50.0;
/// Stielstich: dreifach (0x2858 = 3)
pub const STIEL_DREIFACH: bool = true;
/// Stielstich: Fadenstärke 0,2 mm (0x2856 = 36)
pub const STIEL_STAERKE: f64 = 0.2;

/// Ein Handstich mit seinen Werten (alle in mm bzw. Grad).
#[derive(Clone, Debug, PartialEq)]
pub enum Handstich {
    Rueckstich {
        laenge: f64,
        staerke: f64,
        ueberlappung: f64,
        striche: u8,
    },
    Stielstich {
        breite: f64,
        abstand: f64,
        winkel: f64,
        dreifach: bool,
        staerke: f64,
    },
}

impl Handstich {
    /// Stiche entlang einer Linie (`ecken[i]` = Einstich an Punkt i wie beim Laufstich).
    pub fn stiche(&self, linie: &[Punkt], ecken: Option<&[bool]>, e: &LaufEinstellung) -> Linie {
        match *self {
            Handstich::Rueckstich {
                laenge,
                staerke,
                ueberlappung,
                striche,
            } => rueckstich(linie, ecken, e, laenge, staerke, ueberlappung, striche),
            Handstich::Stielstich {
                breite,
                abstand,
                winkel,
                dreifach,
                staerke,
            } => stielstich(linie, ecken, e, breite, abstand, winkel, dreifach, staerke),
        }
    }

    /// Breite des Stichs quer zur Linie (für Begrenzung und Aussparen).
    pub fn breite(&self) -> f64 {
        match *self {
            Handstich::Rueckstich { staerke, striche, .. } => {
                if striche == 3 || striche == 5 {
                    2.0 * staerke
                } else {
                    0.0
                }
            }
            Handstich::Stielstich { breite, .. } => breite,
        }
    }
}

fn plus(a: Punkt, b: Punkt) -> Punkt {
    p(a.x + b.x, a.y + b.y)
}
fn minus(a: Punkt, b: Punkt) -> Punkt {
    p(a.x - b.x, a.y - b.y)
}
fn mal(a: Punkt, f: f64) -> Punkt {
    p(a.x * f, a.y * f)
}

/// Rückstich (Backstitch) entlang einer Linie.
pub fn rueckstich(
    linie: &[Punkt],
    ecken: Option<&[bool]>,
    e: &LaufEinstellung,
    laenge: f64,
    staerke: f64,
    ueberlappung: f64,
    striche: u8,
) -> Linie {
    let lauf = laufstich_mit_ecken(linie, ecken, e, Some(laenge));
    rueckstich_aus_lauf(&lauf, staerke, ueberlappung, striche)
}

/// `BackStitchFill::AddRunPoint` für alle Punkte eines Laufstichs.
fn rueckstich_aus_lauf(lauf: &[Punkt], t: f64, f: f64, striche: u8) -> Linie {
    let Some(&erster) = lauf.first() else {
        return Vec::new();
    };
    let mut out = vec![erster];
    let mut m = erster;
    let n = lauf.len();
    for i in 1..n {
        let (pp, q) = (lauf[i], lauf[i - 1]);
        let d = minus(pp, q);
        let l = d.x.hypot(d.y);
        if l < 1e-9 {
            continue;
        }
        let o = p(-d.y / l * t, d.x / l * t);
        match striche {
            3 => {
                let a = plus(pp, o);
                if abstand_punkt_strecke(q, a, minus(m, o)) > 1.33 * t {
                    m = mal(plus(m, q), 0.5);
                }
                out.push(a);
                out.push(minus(m, o));
            }
            5 => {
                out.push(plus(pp, o));
                out.push(plus(m, mal(o, 0.5)));
                out.push(minus(pp, o));
                out.push(minus(m, mal(o, 0.5)));
            }
            _ => {}
        }
        let fi = if i == 1 { f / (f + 1.0) } else { f };
        m = if i + 1 == n { pp } else { minus(pp, mal(d, fi)) };
        out.push(pp);
    }
    out
}

/// Stielstich (Stemstitch) entlang einer Linie.
#[allow(clippy::too_many_arguments)]
pub fn stielstich(
    linie: &[Punkt],
    ecken: Option<&[bool]>,
    e: &LaufEinstellung,
    breite: f64,
    abstand: f64,
    winkel: f64,
    dreifach: bool,
    staerke: f64,
) -> Linie {
    let a = winkel.clamp(5.0, 90.0).to_radians();
    let schritt = abstand.max(0.1) / a.sin();
    let lauf = laufstich_mit_ecken(linie, ecken, e, Some(schritt));
    stiel_aus_lauf(&lauf, breite, a, dreifach, staerke)
}

/// Quadrat des Umkreisradius durch drei Punkte (`GeoCircle::GetRadiusSquared`); auf einer Geraden sehr groß.
fn umkreis_quadrat(a: Punkt, b: Punkt, c: Punkt) -> f64 {
    let (ab, bc, ca) = (minus(b, a), minus(c, b), minus(a, c));
    let kreuz = ab.x * bc.y - ab.y * bc.x;
    if kreuz.abs() < 1e-12 {
        return 1e12;
    }
    let (l1, l2, l3) = (ab.x.hypot(ab.y), bc.x.hypot(bc.y), ca.x.hypot(ca.y));
    let r = l1 * l2 * l3 / (2.0 * kreuz.abs());
    r * r
}

/// Generator 0x1019d9e0 mit Ausgabe 0x1019d6a0.
fn stiel_aus_lauf(pts: &[Punkt], w: f64, a: f64, dreifach: bool, t: f64) -> Linie {
    let n = pts.len();
    if n < 2 {
        return pts.to_vec();
    }
    let h = w / 2.0;
    let mut r2 = vec![1e12; n];
    for i in 1..n - 1 {
        r2[i] = umkreis_quadrat(pts[i - 1], pts[i], pts[i + 1]);
    }
    let glatt: Vec<f64> = (0..n)
        .map(|i| {
            if i == 0 || i + 1 == n {
                r2[i]
            } else {
                (r2[i - 1] + r2[i] + r2[i + 1]) / 3.0
            }
        })
        .collect();
    let mut out = vec![pts[0]];
    for i in 1..n {
        let d = minus(pts[i], pts[i - 1]);
        let l = d.x.hypot(d.y);
        if l < 1e-9 {
            continue;
        }
        let u = mal(d, 1.0 / l);
        let q = p(-u.y * h, u.x * h);
        let mut ta = a.tan();
        if 16.0 * w * w > glatt[i] {
            let k = (w / glatt[i].sqrt() - 0.25).min(1.0);
            if k > 0.0 {
                ta = (a + (std::f64::consts::FRAC_PI_2 - a) * k).tan();
            }
        }
        let s = plus(q, mal(u, h / ta.max(1e-6)));
        let c = pts[i];
        out.push(plus(c, s));
        if dreifach {
            let ls = s.x.hypot(s.y);
            let us = if ls < 1e-12 { p(1.0, 0.0) } else { mal(s, 1.0 / ls) };
            let o = p(-us.y * t, us.x * t);
            out.push(plus(minus(c, s), o));
            out.push(minus(plus(c, s), o));
        }
        out.push(minus(c, s));
    }
    out.push(pts[n - 1]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lauf_e() -> LaufEinstellung {
        LaufEinstellung::default()
    }

    #[test]
    fn rueckstich_drei_striche_gerade() {
        let l = vec![p(0.0, 0.0), p(12.0, 0.0)];
        let s = rueckstich(&l, None, &lauf_e(), 3.0, 0.2, 0.5, 3);
        // 4 Laufstiche à 3 mm → je Stich P + o, M − o, P
        assert_eq!(s.len(), 1 + 4 * 3);
        // erster Stich: zurück bis zum Anfang, dann M = 3 − 1/3 · 3 = 2
        assert!((s[1].x - 3.0).abs() < 1e-9 && (s[1].y - 0.2).abs() < 1e-9);
        assert!((s[2].x - 0.0).abs() < 1e-9 && (s[2].y + 0.2).abs() < 1e-9);
        assert!((s[5].x - 2.0).abs() < 1e-9, "Rückpunkt nach dem ersten Stich: {:?}", s[5]);
        // danach 50 % zurück: 6 − 1,5 = 4,5
        assert!((s[8].x - 4.5).abs() < 1e-9, "{:?}", s[8]);
        assert_eq!(*s.last().unwrap(), p(12.0, 0.0));
    }

    #[test]
    fn rueckstich_fuenf_striche() {
        let l = vec![p(0.0, 0.0), p(6.0, 0.0)];
        let s = rueckstich(&l, None, &lauf_e(), 3.0, 0.2, 0.5, 5);
        assert_eq!(s.len(), 1 + 2 * 5);
        assert!((s[2].y - 0.1).abs() < 1e-9 && (s[3].y + 0.2).abs() < 1e-9);
    }

    #[test]
    fn stielstich_schraeg_und_dreifach() {
        let l = vec![p(0.0, 0.0), p(20.0, 0.0)];
        let s = stielstich(&l, None, &lauf_e(), 2.0, 1.5, 50.0, false, 0.2);
        // Stich C + s → C − s: quer über 2 mm, schräg mit 50° zur Linie
        let (a, b) = (s[1], s[2]);
        assert!((a.y - 1.0).abs() < 1e-9 && (b.y + 1.0).abs() < 1e-9);
        let w = (a.y - b.y).atan2(a.x - b.x).to_degrees();
        assert!((w - 50.0).abs() < 1e-6, "Winkel {w}");
        // Schritt = Abstand / sin 50°
        let schritt = s[3].x - s[1].x;
        let n = (20.0 / (1.5 / 50f64.to_radians().sin()) - 1e-9).ceil();
        assert!((schritt - 20.0 / n).abs() < 1e-9);
        let d = stielstich(&l, None, &lauf_e(), 2.0, 1.5, 50.0, true, 0.2);
        assert_eq!(d.len() - 2, 2 * (s.len() - 2));
    }

    #[test]
    fn stielstich_enge_kurve_steiler() {
        // Kreis mit 3 mm Radius: r < 4 · Breite → Winkel steiler als 50°
        let kreis = crate::geometrie::kreis(p(0.0, 0.0), 3.0, 0.01, 0.0);
        let ecken = vec![false; kreis.len()];
        let s = stielstich(&kreis, Some(&ecken), &lauf_e(), 2.0, 1.5, 50.0, false, 0.2);
        let (a, b) = (s[5], s[6]);
        let quer = (a.x.hypot(a.y) - b.x.hypot(b.y)).abs();
        assert!(quer > 1.5, "Stich muss quer über die Linie gehen: {quer}");
    }
}
