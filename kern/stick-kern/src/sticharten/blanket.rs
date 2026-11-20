//! Blanket (Langettenstich) – Kontur-Stichart **0x2727** des Originals (Dialog „Outline Stitch“: Stichart Satin 0x2721
//! oder Blanket 0x2727, DIALOG.DLL 0x1000ecca; Felder „Stitch Spacing“ und „Blanket Width“).
//!
//! Werte (`Analyse_Teil3/12_applikation.md` §6):
//! - Vorlage NORMAL.AMT, Füllung 0x2748#4 = 0x2727: **0x2728 = 360 → Abstand 2,0 mm**, **0x2729 = 540 → Breite 3,0 mm**
//!   (Zuordnung wie beim Zickzack 0x2724: 0x2725 Abstand / 0x2726 Breite; mittel).
//! - Form (Handbuch PDF-S. 70–71): „Kamm“ – die Nadel sticht auf beiden Seiten der Säule ein und dann wieder auf der
//!   ersten Seite, der Faden liegt als Rücken an der Seite. Standard „Center“ (um die Linie), wahlweise „Offset“
//!   (Rücken auf der Linie). Je Zahn: Rücken → Spitze → zurück zum Rücken, dann um den Abstand weiter (Folge der Einstiche:
//!   **eigene Festlegung** nach dieser Beschreibung, im Original nicht gemessen – kein Musterdesign enthält Blanket).

use crate::geometrie::{Linie, Punkt, geschlossen_ohne_endpunkt, p};

/// Vorgabe der Vorlage: Abstand der Zähne (0x2728)
pub const ABSTAND: f64 = 2.0;
/// Vorgabe der Vorlage: Breite (0x2729)
pub const BREITE: f64 = 3.0;

/// Punkt und Richtung bei Bogenlänge `s` auf dem Linienzug.
fn bei(pts: &[Punkt], s: f64) -> (Punkt, Punkt) {
    let mut rest = s;
    for w in pts.windows(2) {
        let (dx, dy) = (w[1].x - w[0].x, w[1].y - w[0].y);
        let l = dx.hypot(dy);
        if l < 1e-12 {
            continue;
        }
        if rest <= l + 1e-9 {
            let t = (rest / l).min(1.0);
            return (p(w[0].x + dx * t, w[0].y + dy * t), p(dx / l, dy / l));
        }
        rest -= l;
    }
    let n = pts.len();
    let (a, b) = (pts[n - 2], pts[n - 1]);
    let l = (b.x - a.x).hypot(b.y - a.y).max(1e-12);
    (b, p((b.x - a.x) / l, (b.y - a.y) / l))
}

/// Blanket entlang einer Linie. Zähne zeigen nach **links** der Laufrichtung (bei einem Ring gegen den Uhrzeiger nach
/// innen); `mittig` = „Center“ (Rücken um die halbe Breite nach rechts), sonst „Offset“ (Rücken auf der Linie).
pub fn blanket(punkte: &[Punkt], geschlossen: bool, abstand: f64, breite: f64, mittig: bool) -> Linie {
    let mut pts: Linie = if geschlossen {
        geschlossen_ohne_endpunkt(punkte)
    } else {
        punkte.to_vec()
    };
    if geschlossen && pts.len() >= 2 {
        pts.push(pts[0]);
    }
    pts.dedup_by(|a, b| (a.x - b.x).hypot(a.y - b.y) < 1e-9);
    if pts.len() < 2 || abstand <= 0.0 {
        return pts;
    }
    let laenge: f64 = pts.windows(2).map(|w| (w[1].x - w[0].x).hypot(w[1].y - w[0].y)).sum();
    let n = ((laenge / abstand).round() as usize).max(1);
    let zaehne = if geschlossen { n } else { n + 1 };
    let ruecken_versatz = if mittig { breite / 2.0 } else { 0.0 };
    let mut out = Vec::with_capacity(zaehne * 3 + 1);
    for k in 0..zaehne {
        let (q, d) = bei(&pts, laenge * k as f64 / n as f64);
        let links = p(-d.y, d.x);
        let r = p(q.x - links.x * ruecken_versatz, q.y - links.y * ruecken_versatz);
        let s = p(r.x + links.x * breite, r.y + links.y * breite);
        out.extend([r, s, r]);
    }
    if geschlossen {
        out.push(out[0]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::abstand;

    #[test]
    fn kamm_auf_gerader_linie() {
        let l = blanket(&[p(0.0, 0.0), p(10.0, 0.0)], false, ABSTAND, BREITE, true);
        // 6 Zähne (0, 2, … 10 mm), je Rücken – Spitze – Rücken
        assert_eq!(l.len(), 18);
        assert_eq!(l[0], p(0.0, -1.5));
        assert_eq!(l[1], p(0.0, 1.5));
        assert_eq!(l[2], p(0.0, -1.5));
        assert!((abstand(l[2], l[3]) - 2.0).abs() < 1e-9);
        // Offset: Rücken auf der Linie
        let o = blanket(&[p(0.0, 0.0), p(10.0, 0.0)], false, ABSTAND, BREITE, false);
        assert_eq!((o[0], o[1]), (p(0.0, 0.0), p(0.0, 3.0)));
    }

    #[test]
    fn ring_zaehne_nach_innen() {
        // Quadrat gegen den Uhrzeiger, 40 mm Umfang → 20 Zähne, Spitzen innen
        let q = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let l = blanket(&q, true, 2.0, 2.5, false);
        assert_eq!(l.len(), 20 * 3 + 1);
        assert!(
            l.iter()
                .skip(1)
                .step_by(3)
                .all(|s| s.x > -1e-9 && s.x < 10.0 + 1e-9 && s.y > -1e-9 && s.y < 10.0 + 1e-9)
        );
        assert_eq!(l.first(), l.last());
    }
}
