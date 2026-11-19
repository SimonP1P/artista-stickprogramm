//! Laufstich (Geradstich) und Dreifachstich entlang einer Linie.

use crate::einstellungen::LaufEinstellung;
use crate::geometrie::{Linie, Punkt, abstand, abstand_punkt_strecke, knick_grad, kumuliert, ohne_doppelte, punkt_bei};

fn ecken(pts: &[Punkt], ecke_grad: f64) -> Vec<usize> {
    let mut idx = vec![0];
    for i in 1..pts.len() - 1 {
        if knick_grad(pts[i - 1], pts[i], pts[i + 1]) >= ecke_grad {
            idx.push(i);
        }
    }
    idx.push(pts.len() - 1);
    idx
}

fn sehnen_ok(teil: &[Punkt], kum: &[f64], s_punkte: &[f64], gap: f64) -> bool {
    for w in s_punkte.windows(2) {
        let (s0, s1) = (w[0], w[1]);
        let (p0, p1) = (punkt_bei(teil, s0, kum), punkt_bei(teil, s1, kum));
        for (q, &s) in teil.iter().zip(kum) {
            if s0 < s && s < s1 && abstand_punkt_strecke(*q, p0, p1) > gap {
                return false;
            }
        }
    }
    true
}

/// Einstichpunkte eines Laufstichs entlang der Linie (erster und letzter Punkt inklusive).
///
/// Wie „feste Länge“ im Original: An Ecken wird eingestochen, dazwischen wird die Strecke gleichmäßig in
/// Stiche ≤ Länge geteilt. Bei `variabel` werden Stiche an Kurven so lange verkürzt, bis die Sehne höchstens
/// `sehnenabstand` von der Linie abweicht (nicht unter `min_laenge`). Ecken hier nach Knickwinkel
/// ≥ `ecke_grad` (eigene Festlegung für berechnete Linien wie Unterlagen); Formen aus dem Design über
/// [`laufstich_mit_ecken`].
pub fn laufstich(linie: &[Punkt], e: &LaufEinstellung, laenge: Option<f64>) -> Linie {
    laufstich_mit_ecken(linie, None, e, laenge)
}

/// Laufstich mit vorgegebenen Eckpunkten (`ecken[i]` gehört zu `linie[i]`): Wie im Original wird an **jedem
/// Eckpunkt** eingestochen, unabhängig vom Winkel, an Kurvenpunkten nicht (an den Musterdesigns gemessen:
/// Eckknoten 99–100 % mit Einstich in allen Winkelbereichen, Kurvenknoten 6 %; `Analyse_Teil3/07_lauf_ecken.md`).
pub fn laufstich_mit_ecken(
    linie: &[Punkt],
    ecken_vorgabe: Option<&[bool]>,
    e: &LaufEinstellung,
    laenge: Option<f64>,
) -> Linie {
    let l = laenge.filter(|&x| x > 0.0).unwrap_or(e.laenge); // wie Pythons `laenge or e.laenge`
    let (pts, flags) = match ecken_vorgabe {
        Some(f) if f.len() == linie.len() => {
            // doppelte Punkte entfernen, Eck-Markierung dabei behalten
            let (mut pts, mut flags): (Linie, Vec<bool>) = (Vec::new(), Vec::new());
            for (&q, &ecke) in linie.iter().zip(f) {
                if pts.last().is_none_or(|&last| abstand(last, q) > 1e-9) {
                    pts.push(q);
                    flags.push(ecke);
                } else if let Some(last) = flags.last_mut() {
                    *last |= ecke;
                }
            }
            (pts, Some(flags))
        }
        _ => (ohne_doppelte(linie), None),
    };
    if pts.len() < 2 {
        return pts;
    }
    let mut out = vec![pts[0]];
    let ecken = match flags {
        Some(f) => {
            let mut idx: Vec<usize> = (0..pts.len())
                .filter(|&i| i == 0 || i == pts.len() - 1 || f[i])
                .collect();
            idx.dedup();
            idx
        }
        None => ecken(&pts, e.ecke_grad),
    };
    for w in ecken.windows(2) {
        let teil = &pts[w[0]..=w[1]];
        let kum = kumuliert(teil);
        let gesamt = *kum.last().unwrap();
        if gesamt < 1e-9 {
            continue;
        }
        let mut n = ((gesamt / l - 1e-9).ceil() as usize).max(1);
        let s_punkte = loop {
            let s_punkte: Vec<f64> = (0..=n).map(|k| gesamt * k as f64 / n as f64).collect();
            if !e.variabel
                || gesamt / ((n + 1) as f64) < e.min_laenge
                || sehnen_ok(teil, &kum, &s_punkte, e.sehnenabstand)
            {
                break s_punkte;
            }
            n += 1;
        };
        out.extend(s_punkte[1..].iter().map(|&s| punkt_bei(teil, s, &kum)));
    }
    out
}

/// Dreifachstich: jeder Stich vor, zurück, vor (Stichpunkte eines Laufstichs als Eingabe).
pub fn dreifach(punkte: &[Punkt]) -> Linie {
    if punkte.len() < 2 {
        return punkte.to_vec();
    }
    let mut out = vec![punkte[0]];
    for w in punkte.windows(2) {
        out.extend([w[1], w[0], w[1]]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::{abstand, bogen, p};

    #[test]
    fn gerade_gleichmaessig() {
        let e = LaufEinstellung::default();
        let pts = laufstich(&[p(0.0, 0.0), p(10.0, 0.0)], &e, None);
        assert_eq!(pts.len(), 5);
        assert!(pts.windows(2).all(|w| (abstand(w[0], w[1]) - 2.5).abs() < 1e-9));
        let pts = laufstich(&[p(0.0, 0.0), p(11.0, 0.0)], &e, None);
        assert_eq!(pts.len(), 6);
        assert!(pts.windows(2).all(|w| (abstand(w[0], w[1]) - 2.2).abs() < 1e-9));
    }

    #[test]
    fn sticht_an_ecken_ein() {
        let pts = laufstich(
            &[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 7.0)],
            &LaufEinstellung::default(),
            None,
        );
        assert!(pts.iter().any(|q| (q.x - 10.0).abs() < 1e-9 && q.y.abs() < 1e-9));
        assert_eq!(pts[0], p(0.0, 0.0));
        assert_eq!(*pts.last().unwrap(), p(10.0, 7.0));
        assert!(pts.windows(2).all(|w| abstand(w[0], w[1]) <= 2.5 + 1e-9));
    }

    #[test]
    fn eckpunkt_auch_bei_flachem_knick() {
        // 5° Knick bei x = 6: als Eckpunkt markiert wird dort eingestochen, als Kurvenpunkt nicht
        let linie = [p(0.0, 0.0), p(6.0, 0.0), p(14.0, 8.0 * 5f64.to_radians().tan())];
        let e = LaufEinstellung::default();
        let eck = laufstich_mit_ecken(&linie, Some(&[true, true, true]), &e, None);
        assert!(eck.iter().any(|q| abstand(*q, linie[1]) < 1e-9));
        let kurve = laufstich_mit_ecken(&linie, Some(&[true, false, true]), &e, None);
        assert!(kurve.iter().all(|q| abstand(*q, linie[1]) > 0.1));
    }

    #[test]
    fn variabel_haelt_sehnenabstand() {
        let kurve = bogen(p(0.0, 0.0), 3.0, 0.0, 180.0, 0.001);
        let e = LaufEinstellung {
            variabel: true,
            ..Default::default()
        };
        let pts = laufstich(&kurve, &e, None);
        for w in pts.windows(2) {
            let (a, b) = (w[0], w[1]);
            let wm = (a.y.atan2(a.x) + b.y.atan2(b.x)) / 2.0;
            let mitte = p(3.0 * wm.cos(), 3.0 * wm.sin());
            assert!(abstand_punkt_strecke(mitte, a, b) <= 0.09 + 1e-3);
        }
        assert!(pts.windows(2).map(|w| abstand(w[0], w[1])).fold(0.0, f64::max) < 2.5);
    }

    #[test]
    fn dreifach_vor_zurueck_vor() {
        let pts = laufstich(&[p(0.0, 0.0), p(5.0, 0.0)], &LaufEinstellung::default(), None);
        let d = dreifach(&pts);
        assert_eq!(d.len(), 1 + 3 * (pts.len() - 1));
        assert_eq!(&d[..4], &[pts[0], pts[1], pts[0], pts[1]]);
    }
}
