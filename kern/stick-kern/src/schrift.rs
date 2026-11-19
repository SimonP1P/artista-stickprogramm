//! Schrift mit den Original-Alphabeten der BERNINA artista V4 (ESB, `Analyse_Teil3/09_esb_alphabete.md`).
//!
//! Ein Alphabet besteht aus Zeichen; jedes Zeichen hat eine Grundlinie und Objekte wie im Design des Originals –
//! meist Satinsäulen (EmbroideryPatch) aus einem geschlossenen Umriss mit **Winkellinien**. Das Lesen der Datei steckt
//! in `stick-formate::esb`; hier stehen das Modell, die Umwandlung der Säulen in die zwei Seiten des Rechenkerns und
//! der Satz eines Texts (Höhe, Breite, Buchstabenabstand, Position).

use crate::design::{Objekt, ObjektArt, Unterlage};
use crate::geometrie::{Linie, Punkt, p};
use std::collections::BTreeMap;

/// Einheit der Alphabete: 1 mm = 180 (Analyse/code/06)
pub const EINHEIT: f64 = 180.0;
/// Referenzhöhe: Die Zeichensätze im Maschinenspeicher sind alle genau im Maßstab 0,5 gestickt (auch Zurich mit
/// 61,5 mm hohem „H“) → für alle Alphabete gleiche Referenz, Großbuchstaben ≈ 50 mm (`Analyse_Teil3/09` §4,
/// mittel-hoch; im Code noch nicht bestätigt).
pub const REFERENZ_HOEHE_MM: f64 = 50.0;
/// Breite des Leerzeichens in Teilen der Buchstabenhöhe – **eigene Festlegung** (die Alphabete haben kein
/// Leerzeichen; Wert des Originals noch offen, `Analyse_Teil3/09` §5).
pub const LEERZEICHEN_ANTEIL: f64 = 0.3;

/// Bézier-Knoten in Alphabet-Einheiten (1/180 mm), Anfasser optional.
#[derive(Clone, Debug, PartialEq)]
pub struct Knoten {
    pub ein: Option<(f64, f64)>,
    pub punkt: (f64, f64),
    pub aus: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Umriss {
    pub geschlossen: bool,
    pub knoten: Vec<Knoten>,
}

/// Winkellinie einer Satinsäule: von Segment `a` beim Anteil `x` zu Segment `c` beim Anteil `y` des Umrisses
/// (IMPORT.DLL 0x10018970).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Winkellinie {
    pub a: u32,
    pub x: f64,
    pub c: u32,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stichart {
    /// Satin mit festem Abstand in mm (0x15 im Alphabet, z. B. 72 = 0,40 mm)
    Satin {
        abstand: Option<f64>,
    },
    Tatami,
    /// Laufstich mit Stichlänge in mm
    Lauf {
        laenge: Option<f64>,
    },
    Unbekannt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZeichenObjekt {
    /// Klasse im Original (0x06 EmbroideryPatch, 0x05 EmbroideryCurve, 0x0c InputCCurve …)
    pub klasse: u32,
    pub umrisse: Vec<Umriss>,
    pub winkel: Vec<Winkellinie>,
    pub stichart: Stichart,
    /// Unterlagen des Objekts im Alphabet: mittellauf, edge_walk, zickzack, doppelzickzack
    pub unterlagen: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Zeichen {
    pub zeichen: char,
    /// Grundlinie x1, y1, x2, y2 (Alphabet-Einheiten); Vorschub = x2 − x1
    pub grundlinie: [f64; 4],
    pub objekte: Vec<ZeichenObjekt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alphabet {
    pub name: String,
    /// Buchstabenabstand in Teilen der Höhe (Kopf, float 2: Drifter 0,10) – „automatically as a percentage of letter
    /// height“ (Handbuch PDF-S. 168)
    pub abstand_anteil: f64,
    pub zeichen: BTreeMap<char, Zeichen>,
}

// ---------------------------------------------------------------- Geometrie der Umrisse

fn segmente(u: &Umriss) -> usize {
    if u.geschlossen {
        u.knoten.len()
    } else {
        u.knoten.len().saturating_sub(1)
    }
}

/// Punkt auf Segment `s` beim Anteil `t` (kubische Bézierkurve; fehlender Anfasser = Ankerpunkt).
pub fn segment_punkt(u: &Umriss, s: usize, t: f64) -> (f64, f64) {
    let n = u.knoten.len();
    let s = s.min(segmente(u).saturating_sub(1));
    let a = &u.knoten[s];
    let b = &u.knoten[(s + 1) % n];
    let (p0, p3) = (a.punkt, b.punkt);
    let p1 = a.aus.unwrap_or(p0);
    let p2 = b.ein.unwrap_or(p3);
    let k = [
        (1.0 - t).powi(3),
        3.0 * (1.0 - t).powi(2) * t,
        3.0 * (1.0 - t) * t * t,
        t.powi(3),
    ];
    (
        k[0] * p0.0 + k[1] * p1.0 + k[2] * p2.0 + k[3] * p3.0,
        k[0] * p0.1 + k[1] * p1.1 + k[2] * p2.1 + k[3] * p3.1,
    )
}

fn punkt_bei_parameter(u: &Umriss, pos: f64) -> (f64, f64) {
    let n = segmente(u);
    let mut q = pos;
    if u.geschlossen {
        q = q.rem_euclid(n as f64);
    }
    let s = (q.floor() as usize).min(n.saturating_sub(1));
    segment_punkt(u, s, (q - s as f64).clamp(0.0, 1.0))
}

/// Punkte des Umrisses von Parameter `von` nach `bis` (Parameter = Segment + Anteil) in Richtung ±1.
fn weg(u: &Umriss, von: f64, bis: f64, richtung: f64) -> Vec<(f64, f64)> {
    let n = segmente(u) as f64;
    let d = if u.geschlossen {
        if richtung > 0.0 {
            (bis - von).rem_euclid(n)
        } else {
            (von - bis).rem_euclid(n)
        }
    } else {
        (bis - von) * richtung
    };
    let schritte = ((d.abs() * 16.0).ceil() as usize).max(1);
    (0..=schritte)
        .map(|i| punkt_bei_parameter(u, von + richtung * d * i as f64 / schritte as f64))
        .collect()
}

fn kumuliert(l: &[(f64, f64)]) -> Vec<f64> {
    let mut k = vec![0.0];
    for w in l.windows(2) {
        let last = *k.last().unwrap();
        k.push(last + ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt());
    }
    k
}

fn bei(l: &[(f64, f64)], kum: &[f64], s: f64) -> (f64, f64) {
    if s <= 0.0 {
        return l[0];
    }
    for i in 1..l.len() {
        if kum[i] >= s {
            let t = if kum[i] > kum[i - 1] {
                (s - kum[i - 1]) / (kum[i] - kum[i - 1])
            } else {
                0.0
            };
            return (
                l[i - 1].0 + (l[i].0 - l[i - 1].0) * t,
                l[i - 1].1 + (l[i].1 - l[i - 1].1) * t,
            );
        }
    }
    *l.last().unwrap()
}

/// Punktfolge in Alphabet-Einheiten
pub type Zug = Vec<(f64, f64)>;

/// Satinsäule mit Winkellinien → Seiten A und B (Alphabet-Einheiten) mit Punkt-zu-Punkt-Zuordnung.
///
/// Seite A läuft über den Umriss von der ersten zur letzten Winkellinie (Richtung der a-Werte), Seite B in
/// Gegenrichtung. Zwischen zwei Winkellinien werden beide Teilwege mit gleich vielen Punkten nach relativer
/// Bogenlänge abgetastet (1 Punkt je mm), sodass die Winkellinien selbst exakt zugeordnet bleiben.
/// Grundlage `Analyse_Teil3/09` §2 (hoch); die Abtastung ist eine **eigene Festlegung**.
pub fn saeule_seiten(u: &Umriss, w: &[Winkellinie]) -> Option<(Zug, Zug)> {
    if w.len() < 2 || u.knoten.len() < 2 {
        return None;
    }
    let n = segmente(u) as f64;
    let pa: Vec<f64> = w.iter().map(|l| l.a as f64 + l.x).collect();
    let pb: Vec<f64> = w.iter().map(|l| l.c as f64 + l.y).collect();
    let d = if u.geschlossen {
        (pa[1] - pa[0]).rem_euclid(n)
    } else {
        pa[1] - pa[0]
    };
    let ra = if u.geschlossen {
        if d <= n / 2.0 { 1.0 } else { -1.0 }
    } else if d >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for k in 0..w.len() - 1 {
        let wa = weg(u, pa[k], pa[k + 1], ra);
        let wb = weg(u, pb[k], pb[k + 1], -ra);
        let (ka, kb) = (kumuliert(&wa), kumuliert(&wb));
        let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
        let teile = ((la.max(lb) / EINHEIT).ceil() as usize).max(1);
        let letzte = k == w.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(bei(&wa, &ka, t * la));
            sb.push(bei(&wb, &kb, t * lb));
        }
    }
    Some((sa, sb))
}

/// Umriss als Punktliste (Alphabet-Einheiten), je Segment 12 Teilstücke.
pub fn umriss_punkte(u: &Umriss) -> Vec<(f64, f64)> {
    let n = segmente(u);
    let mut out = Vec::new();
    for s in 0..n {
        for i in 0..12 {
            out.push(segment_punkt(u, s, i as f64 / 12.0));
        }
    }
    if let Some(l) = u.knoten.get(if u.geschlossen { 0 } else { u.knoten.len() - 1 }) {
        out.push(l.punkt);
    }
    out
}

// ---------------------------------------------------------------- Textsatz

/// Vorgaben für einen Text (wie Object Properties – Lettering im Original).
#[derive(Clone, Debug, PartialEq)]
pub struct TextVorgabe {
    pub text: String,
    /// Buchstabenhöhe in mm (Original 1–200 mm, Handbuch S. 165)
    pub hoehe: f64,
    /// Breite in % der Höhe (100 = Original)
    pub breite_prozent: f64,
    /// Buchstabenabstand in mm; None = Anteil des Alphabets × Höhe
    pub abstand: Option<f64>,
    /// Bezugspunkt auf der Grundlinie (mm)
    pub position: Punkt,
    /// 0 = Text beginnt bei `position`, 0,5 = mittig, 1 = endet dort (wie Ausrichtung links/Mitte/rechts)
    pub ausrichtung: f64,
    pub farbe: String,
}

fn unterlage_aus(o: &ZeichenObjekt) -> Unterlage {
    Unterlage::Arten(o.unterlagen.clone())
}

/// Text in Stickobjekte des Rechenkerns umsetzen. Liefert die Objekte und die Zeichen ohne Entsprechung im Alphabet.
pub fn text_objekte(alpha: &Alphabet, v: &TextVorgabe) -> (Vec<Objekt>, Vec<char>) {
    let s = v.hoehe / REFERENZ_HOEHE_MM / EINHEIT; // Alphabet-Einheit → mm
    let sx = s * v.breite_prozent / 100.0;
    let abstand = v.abstand.unwrap_or(alpha.abstand_anteil * v.hoehe);
    let vorschub = |ch: char| -> Option<f64> {
        if ch == ' ' {
            return Some(LEERZEICHEN_ANTEIL * v.hoehe * v.breite_prozent / 100.0);
        }
        alpha
            .zeichen
            .get(&ch)
            .map(|z| (z.grundlinie[2] - z.grundlinie[0]) * sx + abstand * v.breite_prozent / 100.0)
    };
    let breite: f64 = v.text.chars().filter_map(vorschub).sum::<f64>()
        - if v.text.chars().last().is_some_and(|c| c != ' ') {
            abstand * v.breite_prozent / 100.0
        } else {
            0.0
        };
    let mut x = v.position.x - v.ausrichtung * breite.max(0.0);
    let mut objekte = Vec::new();
    let mut fehlend = Vec::new();
    for ch in v.text.chars() {
        if ch == ' ' {
            x += LEERZEICHEN_ANTEIL * v.hoehe * v.breite_prozent / 100.0;
            continue;
        }
        let Some(z) = alpha.zeichen.get(&ch) else {
            fehlend.push(ch);
            continue;
        };
        let [gx, gy, gx2, _] = z.grundlinie;
        let abb = |q: (f64, f64)| p(x + (q.0 - gx) * sx, v.position.y + (q.1 - gy) * s);
        let abb_linie = |l: &[(f64, f64)]| -> Linie { l.iter().map(|&q| abb(q)).collect() };
        for o in &z.objekte {
            let art = match (&o.stichart, o.klasse) {
                (Stichart::Satin { abstand }, 0x06) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    let Some((a, b)) = saeule_seiten(u, &o.winkel) else {
                        continue;
                    };
                    ObjektArt::Satin {
                        seite_a: abb_linie(&a),
                        seite_b: abb_linie(&b),
                        unterlage: unterlage_aus(o),
                        abstand: *abstand,
                    }
                }
                (Stichart::Satin { abstand }, 0x0c) if o.umrisse.len() >= 2 => ObjektArt::Satin {
                    seite_a: abb_linie(&umriss_punkte(&o.umrisse[0])),
                    seite_b: abb_linie(&umriss_punkte(&o.umrisse[1])),
                    unterlage: unterlage_aus(o),
                    abstand: *abstand,
                },
                (Stichart::Tatami, _) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    ObjektArt::Tatami {
                        umriss: abb_linie(&umriss_punkte(u)),
                        loecher: o.umrisse[1..].iter().map(|l| abb_linie(&umriss_punkte(l))).collect(),
                        winkel: None,
                        unterlage: Some(!o.unterlagen.is_empty()),
                    }
                }
                (Stichart::Lauf { laenge }, _) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    let pts = abb_linie(&umriss_punkte(u));
                    let n = pts.len();
                    ObjektArt::Lauf {
                        punkte: pts,
                        ecken: vec![false; n],
                        laenge: *laenge,
                        dreifach: false,
                        geschlossen: u.geschlossen,
                    }
                }
                _ => continue,
            };
            objekte.push(Objekt {
                farbe: v.farbe.clone(),
                name: format!("{} {}", alpha.name, ch),
                art,
            });
        }
        x += (gx2 - gx) * sx + abstand * v.breite_prozent / 100.0;
    }
    (objekte, fehlend)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn knoten(x: f64, y: f64) -> Knoten {
        Knoten {
            ein: None,
            punkt: (x, y),
            aus: None,
        }
    }

    /// Rechteckige Säule 0…10 mm × 0…2 mm wie im Alphabet: Umriss 4 Knoten, Winkellinien 0↔3 und 1↔2.
    fn saeule() -> (Umriss, Vec<Winkellinie>) {
        let e = EINHEIT;
        let u = Umriss {
            geschlossen: true,
            knoten: vec![
                knoten(0.0, 0.0),
                knoten(10.0 * e, 0.0),
                knoten(10.0 * e, 2.0 * e),
                knoten(0.0, 2.0 * e),
            ],
        };
        // Segment 0 (Knoten 0→1) bei 0 ↔ Segment 2 (Knoten 2→3) bei 1 = Knoten 3; Knoten 1 ↔ Knoten 2
        let w = vec![
            Winkellinie {
                a: 0,
                x: 0.0,
                c: 2,
                y: 1.0,
            },
            Winkellinie {
                a: 1,
                x: 0.0,
                c: 2,
                y: 0.0,
            },
        ];
        (u, w)
    }

    #[test]
    fn seiten_aus_winkellinien() {
        let (u, w) = saeule();
        let (a, b) = saeule_seiten(&u, &w).unwrap();
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), 11); // 10 mm → 10 Teile + Endpunkt
        assert!((a[0].0).abs() < 1e-9 && (a[0].1).abs() < 1e-9);
        assert!((b[0].1 - 2.0 * EINHEIT).abs() < 1e-9 && b[0].0.abs() < 1e-9);
        assert!((a[10].0 - 10.0 * EINHEIT).abs() < 1e-6);
        assert!((b[10].0 - 10.0 * EINHEIT).abs() < 1e-6 && (b[10].1 - 2.0 * EINHEIT).abs() < 1e-6);
    }

    #[test]
    fn text_setzen_massstab_und_vorschub() {
        let (u, w) = saeule();
        let z = Zeichen {
            zeichen: 'I',
            grundlinie: [0.0, 0.0, 10.0 * EINHEIT, 0.0],
            objekte: vec![ZeichenObjekt {
                klasse: 0x06,
                umrisse: vec![u],
                winkel: w,
                stichart: Stichart::Satin { abstand: Some(0.4) },
                unterlagen: vec![],
            }],
        };
        let alpha = Alphabet {
            name: "Test".into(),
            abstand_anteil: 0.1,
            zeichen: [('I', z)].into_iter().collect(),
        };
        let v = TextVorgabe {
            text: "II?".into(),
            hoehe: 25.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(-10.0, 0.0),
            ausrichtung: 0.0,
            farbe: "#000000".into(),
        };
        let (obj, fehlend) = text_objekte(&alpha, &v);
        assert_eq!(fehlend, vec!['?']);
        assert_eq!(obj.len(), 2);
        // Maßstab 25/50 = 0,5: Säule 5 mm lang; zweites I beginnt bei −10 + 5 + 2,5 (10 % von 25 mm)
        let ObjektArt::Satin { seite_a, .. } = &obj[1].art else {
            panic!()
        };
        assert!((seite_a[0].x - (-2.5)).abs() < 1e-9, "{}", seite_a[0].x);
        assert!((seite_a.last().unwrap().x - 2.5).abs() < 1e-6);
        // mittig: Breite 5 + 2,5 + 5 = 12,5 → Beginn bei −6,25
        let (obj, _) = text_objekte(
            &alpha,
            &TextVorgabe {
                text: "II".into(),
                position: p(0.0, 0.0),
                ausrichtung: 0.5,
                ..v
            },
        );
        let ObjektArt::Satin { seite_a, .. } = &obj[0].art else {
            panic!()
        };
        assert!((seite_a[0].x + 6.25).abs() < 1e-9, "{}", seite_a[0].x);
    }
}
