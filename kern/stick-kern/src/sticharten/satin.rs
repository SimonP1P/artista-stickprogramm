//! Satinsäule zwischen zwei Seitenlinien (wie „Satin Column A“ im Original).
//!
//! Die Säule wird durch zwei Seiten A und B beschrieben, beide in Stickrichtung. Punkt i von A gehört zu
//! Punkt i von B (Stichrichtungs-Linie). Haben A und B verschieden viele Punkte, werden sie über die
//! relative Bogenlänge einander zugeordnet.

use crate::einstellungen::SatinEinstellung;
use crate::geometrie::{
    Linie, Punkt, abstand, kumuliert, ohne_doppelte, punkt_bei, richtung, versatz_linie, verschieben, zwischen,
};

/// Wie Pythons `round(t, 9)` für die Zuordnung der Punkte.
fn runden9(t: f64) -> f64 {
    (t * 1e9).round_ties_even() / 1e9
}

pub fn paare_bilden(seite_a: &[Punkt], seite_b: &[Punkt]) -> Result<(Linie, Linie), String> {
    let (a, b) = (ohne_doppelte(seite_a), ohne_doppelte(seite_b));
    if a.len() < 2 || b.len() < 2 {
        return Err("Satinsäule: jede Seite braucht mindestens 2 Punkte".into());
    }
    if a.len() == b.len() {
        return Ok((a, b));
    }
    let (ka, kb) = (kumuliert(&a), kumuliert(&b));
    let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
    let mut ts: Vec<f64> = ka
        .iter()
        .map(|s| runden9(s / la))
        .chain(kb.iter().map(|s| runden9(s / lb)))
        .collect();
    ts.sort_by(|x, y| x.partial_cmp(y).unwrap());
    ts.dedup();
    Ok((
        ts.iter().map(|&t| punkt_bei(&a, t * la, &ka)).collect(),
        ts.iter().map(|&t| punkt_bei(&b, t * lb, &kb)).collect(),
    ))
}

/// Seiten einer Satinbahn konstanter Breite um eine Mittellinie (A links, B rechts).
pub fn seiten_aus_mittellinie(mitte: &[Punkt], breite: f64) -> (Linie, Linie) {
    let m = ohne_doppelte(mitte);
    (
        versatz_linie(&m, breite / 2.0, 4.0),
        versatz_linie(&m, -breite / 2.0, 4.0),
    )
}

pub fn mittellinie(a: &[Punkt], b: &[Punkt]) -> Linie {
    a.iter().zip(b).map(|(&q, &r)| zwischen(q, r, 0.5)).collect()
}

/// Einstichpunkte der Satinsäule, abwechselnd auf Seite A und B, beginnend bei A[0].
///
/// Abstand je Stich aus Auto Spacing (Breite an der Stelle) bzw. fest; Fortschritt gemessen als Mittel der
/// Wege auf beiden Seiten (eigene Festlegung, mittel). Zugausgleich verlängert jeden Stich an beiden Enden.
pub fn satin(seite_a: &[Punkt], seite_b: &[Punkt], e: &SatinEinstellung) -> Result<Linie, String> {
    let (a, b) = paare_bilden(seite_a, seite_b)?;
    let zg = e.zugausgleich;
    let einstich = |pa: Punkt, pb: Punkt, auf_a: bool| -> Punkt {
        if auf_a {
            verschieben(pa, richtung(pb, pa), zg)
        } else {
            verschieben(pb, richtung(pa, pb), zg)
        }
    };

    let mut out: Linie = vec![einstich(a[0], b[0], true)];
    let mut seite = false; // nächster Einstich auf B
    let mut gelaufen = 0.0; // Weg seit dem letzten Einstich (über Segmentgrenzen hinweg)
    for i in 0..a.len() - 1 {
        let seg = (abstand(a[i], a[i + 1]) + abstand(b[i], b[i + 1])) / 2.0;
        if seg < 1e-9 {
            continue;
        }
        let mut t = 0.0;
        loop {
            let (pa, pb) = (zwischen(a[i], a[i + 1], t), zwischen(b[i], b[i + 1], t));
            let noetig = (e.abstand(abstand(pa, pb)) - gelaufen).max(0.0);
            let frei = (1.0 - t) * seg;
            if noetig > frei + 1e-9 {
                gelaufen += frei;
                break;
            }
            t += noetig / seg;
            gelaufen = 0.0;
            let (pa, pb) = (zwischen(a[i], a[i + 1], t), zwischen(b[i], b[i + 1], t));
            out.push(einstich(pa, pb, seite));
            seite = !seite;
        }
    }
    // Ende sauber abschließen: letzter Stich auf die Endkante
    let ende = einstich(*a.last().unwrap(), *b.last().unwrap(), seite);
    if abstand(*out.last().unwrap(), ende) > 1e-6 {
        out.push(ende);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::einstellungen::fuer_stoff;
    use crate::geometrie::{bogen, p};

    fn gerade(breite: f64) -> (Linie, Linie) {
        (
            vec![p(0.0, breite / 2.0), p(20.0, breite / 2.0)],
            vec![p(0.0, -breite / 2.0), p(20.0, -breite / 2.0)],
        )
    }

    fn dx(pts: &[Punkt]) -> Vec<f64> {
        pts[..pts.len() - 1].windows(2).map(|w| w[1].x - w[0].x).collect()
    }

    #[test]
    fn abstand_und_seitenwechsel() {
        let (a, b) = gerade(4.0);
        let pts = satin(&a, &b, &SatinEinstellung::default()).unwrap();
        for (i, q) in pts[..pts.len() - 1].iter().enumerate() {
            let soll = if i % 2 == 0 { 2.2 } else { -2.2 };
            assert!((q.y - soll).abs() < 1e-9);
        }
        assert!(dx(&pts).iter().all(|d| (d - 0.40).abs() < 1e-9));
        assert!((pts.len() as f64 - (20.0 / 0.40 + 2.0)).abs() <= 1.0);
        assert!((pts.last().unwrap().x - 20.0).abs() < 1e-9);
    }

    #[test]
    fn schmale_bahn_lockerer() {
        let (a, b) = gerade(0.5);
        let pts = satin(&a, &b, &SatinEinstellung::default()).unwrap();
        assert!(dx(&pts).iter().all(|d| (d - 0.51).abs() < 1e-9));
    }

    #[test]
    fn fester_abstand_und_stoff() {
        let (a, b) = gerade(2.0);
        let e = SatinEinstellung {
            auto_spacing: false,
            abstand_fest: 0.40,
            ..Default::default()
        };
        assert!(dx(&satin(&a, &b, &e).unwrap()).iter().all(|d| (d - 0.40).abs() < 1e-9));
        let e2 = fuer_stoff("Knit Medium Weight").unwrap().satin;
        assert!(
            dx(&satin(&a, &b, &e2).unwrap())
                .iter()
                .all(|d| (d - 0.46 * 1.10).abs() < 1e-9)
        );
    }

    #[test]
    fn bogen_bleibt_in_der_bahn() {
        let mitte = bogen(p(0.0, 0.0), 20.0, 0.0, 180.0, 0.02);
        let (a, b) = seiten_aus_mittellinie(&mitte, 3.0);
        for q in satin(&a, &b, &SatinEinstellung::default()).unwrap() {
            let r = q.x.hypot(q.y);
            assert!((20.0 - 1.7 - 0.03..=20.0 + 1.7 + 0.03).contains(&r)); // Bogen als Sehnenzug
        }
    }

    #[test]
    fn paare_verschiedene_punktzahl() {
        let a = vec![p(0.0, 1.0), p(5.0, 1.0), p(10.0, 1.0)];
        let b = vec![p(0.0, -1.0), p(10.0, -1.0)];
        let (pa, pb) = paare_bilden(&a, &b).unwrap();
        assert_eq!((pa.len(), pb.len()), (3, 3));
        assert!((pb[1].x - 5.0).abs() < 1e-9 && (pb[1].y + 1.0).abs() < 1e-9);
    }
}
