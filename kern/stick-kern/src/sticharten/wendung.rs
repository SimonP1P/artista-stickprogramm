//! Fläche mit mehreren Stichwinkeln („Turning“, Handbuch Kap. 16 „Setting multiple stitch angles“: Winkellinien, die
//! zwei Seiten des Objekts schneiden, „to create a turning fill effect“).
//!
//! Im Original wird daraus ein EmbroideryPatch mit Winkellinien – dasselbe Objekt wie die Säulen der Alphabete
//! (`Analyse_Teil3/03`, `/11` §4). Gemessen an 415 solchen Objekten mit Step-Füllung in den Musterdesigns: 83 % der
//! Step-Stiche liegen innerhalb 10° zur nächsten Winkellinie, 93 % innerhalb 20° (`Analyse_Teil3/06` §6) – die
//! Stiche laufen also **längs der Winkellinien** wie beim Satin, geteilt in Step-Stiche (`satin::step_reihen`).
//!
//! Hier: die Winkellinien teilen den Umriss in zwei Seiten A und B (wie `schrift::saeule_seiten`); die Kappen vor der
//! ersten und nach der letzten Linie laufen in eine Spitze am weitesten Punkt der Kappe aus (**eigene Festlegung**).

use crate::geometrie::{Linie, Punkt, abstand, abstand_punkt_strecke, geschlossen_ohne_endpunkt, kumuliert, punkt_bei};

/// Schnitt der Strecke `l` mit dem Ring: Bogenlängen auf dem Ring und Lage auf der Strecke.
fn schnitte(ring: &[Punkt], kum: &[f64], l: &[Punkt; 2]) -> Vec<(f64, f64)> {
    let n = ring.len();
    let mut out = Vec::new();
    for i in 0..n {
        let (a, b, c, d) = (ring[i], ring[(i + 1) % n], l[0], l[1]);
        let r = (b.x - a.x, b.y - a.y);
        let q = (d.x - c.x, d.y - c.y);
        let nenner = r.0 * q.1 - r.1 * q.0;
        if nenner.abs() < 1e-12 {
            continue;
        }
        let t = ((c.x - a.x) * q.1 - (c.y - a.y) * q.0) / nenner;
        let u = ((c.x - a.x) * r.1 - (c.y - a.y) * r.0) / nenner;
        if (0.0..1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            out.push((kum[i] + t * (kum[i + 1] - kum[i]), u));
        }
    }
    out
}

/// Punkte des Rings von Bogenlänge `von` bis `bis` (vorwärts oder rückwärts, über den Anfang hinweg).
fn bogen(ring: &[Punkt], kum: &[f64], von: f64, bis: f64, vorwaerts: bool) -> Linie {
    let lr = kum[kum.len() - 1];
    let n = ring.len();
    let geschl: Linie = ring.iter().copied().chain(std::iter::once(ring[0])).collect();
    let bei = |s: f64| punkt_bei(&geschl, s.rem_euclid(lr), kum);
    let d = if vorwaerts {
        (bis - von).rem_euclid(lr)
    } else {
        (von - bis).rem_euclid(lr)
    };
    let mut out = vec![bei(von)];
    // Eckpunkte des Rings auf dem Weg
    let mut eck: Vec<(f64, Punkt)> = (0..n)
        .filter_map(|i| {
            let w = if vorwaerts {
                (kum[i] - von).rem_euclid(lr)
            } else {
                (von - kum[i]).rem_euclid(lr)
            };
            (w > 1e-9 && w < d - 1e-9).then_some((w, ring[i]))
        })
        .collect();
    eck.sort_by(|a, b| a.0.total_cmp(&b.0));
    out.extend(eck.into_iter().map(|(_, q)| q));
    out.push(bei(bis));
    out
}

/// Seiten A und B einer Fläche mit Winkellinien; Punkt i von A gehört zu Punkt i von B (1 Punkt je mm zwischen
/// zwei festen Linien, nach relativer Bogenlänge). Fehler, wenn eine Linie den Umriss nicht zweimal schneidet oder die
/// Linien sich kreuzen.
pub fn wende_seiten(umriss: &[Punkt], linien: &[[Punkt; 2]]) -> Result<(Linie, Linie), String> {
    let ring = geschlossen_ohne_endpunkt(umriss);
    if ring.len() < 3 || linien.is_empty() {
        return Err("Wendefüllung braucht einen Umriss und mindestens eine Winkellinie".into());
    }
    let geschl: Linie = ring.iter().copied().chain(std::iter::once(ring[0])).collect();
    let kum = kumuliert(&geschl);
    // je Linie die äußersten Schnittpunkte
    let mut enden: Vec<(f64, usize)> = Vec::new();
    for (i, l) in linien.iter().enumerate() {
        let mut s = schnitte(&ring, &kum, l);
        if s.len() < 2 {
            return Err(format!(
                "Winkellinie {} schneidet den Umriss nicht an zwei Seiten",
                i + 1
            ));
        }
        s.sort_by(|a, b| a.1.total_cmp(&b.1));
        enden.push((s[0].0, i));
        enden.push((s[s.len() - 1].0, i));
    }
    enden.sort_by(|a, b| a.0.total_cmp(&b.0));
    // Folge der Linien rund um den Umriss muss eine Leiter sein: x0 … x(m−1) x(m−1) … x0 (zyklisch)
    let m = linien.len();
    let ids: Vec<usize> = enden.iter().map(|e| e.1).collect();
    let dreh = (0..2 * m)
        .find(|&r| (0..m).all(|j| ids[(r + j) % (2 * m)] == ids[(r + 2 * m - 1 - j) % (2 * m)]))
        .ok_or("Winkellinien müssen die Fläche nacheinander queren, ohne sich zu kreuzen")?;
    let pa: Vec<f64> = (0..m).map(|j| enden[(dreh + j) % (2 * m)].0).collect();
    let pb: Vec<f64> = (0..m).map(|j| enden[(dreh + 2 * m - 1 - j) % (2 * m)].0).collect();
    let geschl_bei = |s: f64| punkt_bei(&geschl, s.rem_euclid(kum[kum.len() - 1]), &kum);

    // Kappe zwischen `von` (vorwärts) und `bis`: Spitze am weitesten Punkt von der Linie, wenn > 0,3 mm
    let spitze = |von: f64, bis: f64| -> Option<f64> {
        let (p, q) = (geschl_bei(von), geschl_bei(bis));
        let lr = kum[kum.len() - 1];
        let d = (bis - von).rem_euclid(lr);
        let schritte = ((d / 0.2).ceil() as usize).max(2);
        let (mut best, mut bs) = (0.0, von);
        for k in 1..schritte {
            let s = von + d * k as f64 / schritte as f64;
            let a = abstand_punkt_strecke(geschl_bei(s), p, q);
            if a > best {
                (best, bs) = (a, s);
            }
        }
        (best > 0.3).then_some(bs)
    };
    let anfang = spitze(pb[0], pa[0]);
    let ende = spitze(pa[m - 1], pb[m - 1]);

    // feste Paare (Bogenlänge auf A, auf B) und die Seiten als Linien
    let mut a: Linie = Vec::new();
    let mut b: Linie = Vec::new();
    let mut fest: Vec<(f64, f64)> = Vec::new();
    let anhaengen = |linie: &mut Linie, teil: Linie| {
        let start = if linie
            .last()
            .is_some_and(|l| teil.first().is_some_and(|t| abstand(*l, *t) < 1e-9))
        {
            1
        } else {
            0
        };
        linie.extend(teil.into_iter().skip(start));
    };
    let laenge = |l: &Linie| kumuliert(l).last().copied().unwrap_or(0.0);
    if let Some(s0) = anfang {
        a.push(geschl_bei(s0));
        b.push(geschl_bei(s0));
        fest.push((0.0, 0.0));
        anhaengen(&mut a, bogen(&ring, &kum, s0, pa[0], true));
        anhaengen(&mut b, bogen(&ring, &kum, s0, pb[0], false));
    } else {
        a.push(geschl_bei(pa[0]));
        b.push(geschl_bei(pb[0]));
    }
    fest.push((laenge(&a), laenge(&b)));
    for k in 1..m {
        anhaengen(&mut a, bogen(&ring, &kum, pa[k - 1], pa[k], true));
        anhaengen(&mut b, bogen(&ring, &kum, pb[k - 1], pb[k], false));
        fest.push((laenge(&a), laenge(&b)));
    }
    if let Some(s1) = ende {
        anhaengen(&mut a, bogen(&ring, &kum, pa[m - 1], s1, true));
        anhaengen(&mut b, bogen(&ring, &kum, pb[m - 1], s1, false));
        fest.push((laenge(&a), laenge(&b)));
    }
    if fest.len() < 2 {
        return Err("Wendefüllung: zu kurz".into());
    }
    let (ka, kb) = (kumuliert(&a), kumuliert(&b));
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for (k, w) in fest.windows(2).enumerate() {
        let teile = ((w[1].0 - w[0].0).max(w[1].1 - w[0].1).ceil() as usize).max(1);
        let letzte = k == fest.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(punkt_bei(&a, w[0].0 + t * (w[1].0 - w[0].0), &ka));
            sb.push(punkt_bei(&b, w[0].1 + t * (w[1].1 - w[0].1), &kb));
        }
    }
    Ok((sa, sb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::p;

    /// Rechteck 20 × 10 mit zwei senkrechten Linien: Seiten oben/unten, Spitzen an den Ecken; mit einer schrägen
    /// Linie dreht sich die Stichrichtung
    #[test]
    fn rechteck_mit_winkellinien() {
        let r = vec![p(0.0, 0.0), p(20.0, 0.0), p(20.0, 10.0), p(0.0, 10.0)];
        let (a, b) = wende_seiten(&r, &[[p(5.0, -1.0), p(5.0, 11.0)], [p(15.0, -1.0), p(15.0, 11.0)]]).unwrap();
        assert_eq!(a.len(), b.len());
        // ein Paar genau auf der Linie x = 5
        assert!(
            a.iter()
                .zip(&b)
                .any(|(x, y)| (x.x - 5.0).abs() < 1e-9 && (y.x - 5.0).abs() < 1e-9)
        );
        // die Seiten liegen auf dem Umriss: eine oben, eine unten (im Mittelteil)
        let mitte: Vec<_> = a.iter().zip(&b).filter(|(x, _)| (x.x - 10.0).abs() < 0.6).collect();
        assert!(mitte.iter().all(|(x, y)| (x.y - y.y).abs() > 9.9), "{mitte:?}");
        // schräge Linie: Paar von (10, 0) nach (12, 10) bzw. umgekehrt
        let (a, b) = wende_seiten(&r, &[[p(9.8, -1.0), p(12.2, 11.0)]]).unwrap();
        assert!(a.iter().zip(&b).any(|(x, y)| abstand(*x, *y) > 10.1));
        // Linie trifft den Umriss nur einmal
        assert!(wende_seiten(&r, &[[p(5.0, 5.0), p(5.0, 11.0)]]).is_err());
        // gekreuzte Linien
        assert!(wende_seiten(&r, &[[p(5.0, -1.0), p(15.0, 11.0)], [p(15.0, -1.0), p(5.0, 11.0)]]).is_err());
    }
}
