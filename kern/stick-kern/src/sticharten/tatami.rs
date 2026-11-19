//! Tatami-/Step-Füllung einer Fläche (Umriss mit optionalen Löchern).
//!
//! 1. Fläche so drehen, dass die Stichreihen waagerecht liegen (Stichwinkel).
//! 2. Reihen im Abstand `reihenabstand()` (halber Abstand der Hinreihen, wie im Original) mit allen Kanten
//!    schneiden -> Abschnitte je Reihe (Gerade-Ungerade-Regel).
//! 3. Abschnitte zu zusammenhängenden, einfachen Bereichen verbinden (je Reihe genau ein Abschnitt).
//! 4. Einstiche je Reihe auf einem festen Raster der Stichlänge; das Raster verschiebt sich je Reihe um
//!    `versatz` × Stichlänge. Rasterpunkte näher als `min_rand` am Abschnittsende entfallen. Zugausgleich
//!    verlängert jede Reihe an beiden Enden.
//! 5. Bereiche in der Reihenfolge des geringsten Weges; dazwischen ein Laufweg am Rand wie im Original
//!    (`Analyse_Teil3/08_laufwege_flaechen.md`), nur wenn kein gemeinsamer Rand erreichbar ist ein Sprung.

use super::lauf::laufstich;
use crate::einstellungen::{LaufEinstellung, TatamiEinstellung};
use crate::geometrie::{Linie, Punkt, abstand, drehen, gegen_uhrzeiger, geschlossen_ohne_endpunkt, p, zwischen};
use std::collections::HashMap;

/// Lotfußpunkt von `q` auf einem geschlossenen Ring: (Abstand, Bogenlänge ab Ringanfang).
fn auf_ring(q: Punkt, ring: &[Punkt]) -> (f64, f64) {
    let (mut best, mut s_best, mut s) = (f64::INFINITY, 0.0, 0.0);
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        let l = abstand(a, b);
        let t = if l < 1e-12 {
            0.0
        } else {
            (((q.x - a.x) * (b.x - a.x) + (q.y - a.y) * (b.y - a.y)) / (l * l)).clamp(0.0, 1.0)
        };
        let d = abstand(q, zwischen(a, b, t));
        if d < best {
            (best, s_best) = (d, s + t * l);
        }
        s += l;
    }
    (best, s_best)
}

/// Punkte des Rings von Bogenlänge `s0` bis `s1` in Laufrichtung des Rings (über den Anfang hinweg).
fn ring_stueck(ring: &[Punkt], s0: f64, s1: f64) -> Linie {
    let n = ring.len();
    let umfang: f64 = (0..n).map(|i| abstand(ring[i], ring[(i + 1) % n])).sum();
    let ziel = if s1 >= s0 { s1 - s0 } else { s1 + umfang - s0 };
    let (mut out, mut s, mut gelaufen) = (Vec::new(), 0.0, 0.0);
    // Ring zweimal durchlaufen, damit der Übergang über den Anfang abgedeckt ist
    for k in 0..2 * n {
        let (a, b) = (ring[k % n], ring[(k + 1) % n]);
        let l = abstand(a, b);
        let (von, bis) = (s, s + l);
        if bis >= s0 && l > 1e-12 {
            if out.is_empty() {
                out.push(zwischen(a, b, ((s0 - von) / l).clamp(0.0, 1.0)));
            }
            let rest = ziel - gelaufen;
            let ab = (s0.max(von)) - von;
            if l - ab >= rest {
                out.push(zwischen(a, b, (ab + rest) / l));
                return out;
            }
            gelaufen += l - ab;
            out.push(b);
        }
        s = bis;
    }
    out
}

/// Laufweg vom Ende eines Teilbereichs zum Anfang des nächsten am gemeinsamen Rand entlang (kürzere Richtung).
/// `None`, wenn beide Punkte nicht nah an demselben Rand liegen (dann bleibt es beim Sprung).
fn laufweg(von: Punkt, nach: Punkt, ringe: &[Linie], e: &TatamiEinstellung) -> Option<Linie> {
    let toleranz = 1.0_f64.max(3.0 * e.zugausgleich);
    let mut best: Option<(f64, Linie)> = None;
    for ring in ringe {
        let ((d0, s0), (d1, s1)) = (auf_ring(von, ring), auf_ring(nach, ring));
        if d0 > toleranz || d1 > toleranz {
            continue;
        }
        let vor = ring_stueck(ring, s0, s1);
        let mut zurueck = ring_stueck(ring, s1, s0);
        zurueck.reverse();
        for teil in [vor, zurueck] {
            let mut weg = vec![von];
            weg.extend(teil);
            weg.push(nach);
            let l: f64 = weg.windows(2).map(|w| abstand(w[0], w[1])).sum();
            if best.as_ref().is_none_or(|(bl, _)| l < *bl) {
                best = Some((l, weg));
            }
        }
    }
    let (_, weg) = best?;
    let lauf = LaufEinstellung {
        laenge: e.laufweg_laenge,
        min_laenge: e.laufweg.min_laenge,
        variabel: true,
        sehnenabstand: e.laufweg.sehnenabstand,
        ecke_grad: 30.0,
    };
    Some(laufstich(&weg, &lauf, None))
}

#[derive(Default)]
struct Bereich {
    reihen: Vec<(i64, f64, f64)>, // (Reihennummer, x0, x1)
}

fn schnitte(ringe: &[Linie], y: f64) -> Vec<f64> {
    let mut xs = Vec::new();
    for ring in ringe {
        let n = ring.len();
        for i in 0..n {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            if (a.y > y) != (b.y > y) {
                xs.push(a.x + (y - a.y) * (b.x - a.x) / (b.y - a.y));
            }
        }
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs
}

/// Bereiche bilden; Reihenfolge wie im Prototyp: geschlossene Bereiche in Abschlussreihenfolge, dann offene.
fn bereiche(reihen: &[(i64, Vec<(f64, f64)>)]) -> Vec<Bereich> {
    let mut alle: Vec<Bereich> = Vec::new();
    let mut fertig: Vec<usize> = Vec::new();
    let mut offen: Vec<usize> = Vec::new();
    for (k, abschnitte) in reihen {
        let ueberlapp: Vec<Vec<usize>> = abschnitte
            .iter()
            .map(|&(x0, x1)| {
                offen
                    .iter()
                    .copied()
                    .filter(|&b| {
                        let &(kb, bx0, bx1) = alle[b].reihen.last().unwrap();
                        kb == k - 1 && bx0 < x1 && x0 < bx1
                    })
                    .collect()
            })
            .collect();
        let mut zaehler: HashMap<usize, usize> = HashMap::new();
        for liste in &ueberlapp {
            for &b in liste {
                *zaehler.entry(b).or_insert(0) += 1;
            }
        }
        let mut neu_offen = Vec::new();
        for (&(x0, x1), liste) in abschnitte.iter().zip(&ueberlapp) {
            let b = if liste.len() == 1 && zaehler[&liste[0]] == 1 {
                liste[0]
            } else {
                alle.push(Bereich::default());
                alle.len() - 1
            };
            alle[b].reihen.push((*k, x0, x1));
            neu_offen.push(b);
        }
        for &b in &offen {
            if !neu_offen.contains(&b) {
                fertig.push(b);
            }
        }
        offen = neu_offen;
    }
    let reihenfolge: Vec<usize> = fertig.into_iter().chain(offen).collect();
    let mut nach_index: Vec<Option<Bereich>> = alle.into_iter().map(Some).collect();
    reihenfolge.into_iter().map(|i| nach_index[i].take().unwrap()).collect()
}

fn reihe(k: i64, x0: f64, x1: f64, y: f64, e: &TatamiEinstellung, vorwaerts: bool) -> Linie {
    let l = e.laenge;
    let phase = ((k as f64 * e.versatz) % 1.0) * l;
    let mut m = ((x0 + e.min_rand - phase) / l).ceil() as i64;
    let mut xs = vec![x0];
    loop {
        let x = phase + m as f64 * l;
        if x > x1 - e.min_rand {
            break;
        }
        xs.push(x);
        m += 1;
    }
    xs.push(x1);
    if !vorwaerts {
        xs.reverse();
    }
    xs.into_iter().map(|x| p(x, y)).collect()
}

fn bereich_sticken(b: &Bereich, y0: f64, e: &TatamiEinstellung, von_oben: bool, links_start: bool) -> Linie {
    let reihen: Vec<_> = if von_oben {
        b.reihen.iter().rev().copied().collect()
    } else {
        b.reihen.clone()
    };
    let mut out: Linie = Vec::new();
    let mut vorwaerts = links_start;
    for (k, x0, x1) in reihen {
        let y = y0 + k as f64 * e.reihenabstand();
        let pts = reihe(k, x0, x1, y, e, vorwaerts);
        if let Some(&a) = out.last() {
            // Übergang zur nächsten Reihe: ggf. in Stiche ≤ Stichlänge teilen
            let z = pts[0];
            let n = (abstand(a, z) / e.laenge - 1e-9).ceil() as i64;
            for j in 1..n {
                out.push(p(
                    a.x + (z.x - a.x) * j as f64 / n as f64,
                    a.y + (z.y - a.y) * j as f64 / n as f64,
                ));
            }
        }
        out.extend(pts);
        vorwaerts = !vorwaerts;
    }
    out
}

/// Füllstiche als Liste von Wegen (je zusammenhängender Bereich ein Weg, in Stickreihenfolge).
pub fn tatami(
    umriss: &[Punkt],
    loecher: &[Linie],
    e: &TatamiEinstellung,
    winkel: Option<f64>,
    start: Option<Punkt>,
) -> Vec<Linie> {
    let w = winkel.unwrap_or(e.winkel);
    let null = p(0.0, 0.0);
    let mut ringe = vec![gegen_uhrzeiger(umriss)];
    ringe.extend(loecher.iter().map(|l| geschlossen_ohne_endpunkt(l)));
    let gedreht: Vec<Linie> = ringe
        .iter()
        .map(|r| r.iter().map(|&q| drehen(q, -w, null)).collect())
        .collect();
    if gedreht[0].len() < 3 {
        return Vec::new();
    }
    let y_min = gedreht[0].iter().map(|q| q.y).fold(f64::INFINITY, f64::min);
    let y_max = gedreht[0].iter().map(|q| q.y).fold(f64::NEG_INFINITY, f64::max);
    let ra = e.reihenabstand();
    let y0 = y_min + ra / 2.0;
    let mut reihen = Vec::new();
    let mut k: i64 = 0;
    while y0 + k as f64 * ra < y_max {
        let y = y0 + k as f64 * ra;
        let xs = schnitte(&gedreht, y);
        let mut abschnitte = Vec::new();
        let mut i = 0;
        while i + 1 < xs.len() {
            let (x0, x1) = (xs[i] - e.zugausgleich, xs[i + 1] + e.zugausgleich);
            if x1 - x0 >= 0.3 {
                abschnitte.push((x0, x1));
            }
            i += 2;
        }
        if !abschnitte.is_empty() {
            reihen.push((k, abschnitte));
        }
        k += 1;
    }
    let mut offen = bereiche(&reihen);

    let mut pos = start.map(|s| drehen(s, -w, null));
    let mut wege = Vec::new();
    while !offen.is_empty() {
        let mut best: Option<(f64, usize, bool, bool)> = None;
        for (bi, b) in offen.iter().enumerate() {
            for von_oben in [false, true] {
                let &(kz, x0, x1) = if von_oben {
                    b.reihen.last().unwrap()
                } else {
                    &b.reihen[0]
                };
                let y = y0 + kz as f64 * ra;
                for links in [true, false] {
                    let q = if links { p(x0, y) } else { p(x1, y) };
                    let d = pos.map_or(0.0, |s| abstand(s, q));
                    if best.is_none_or(|bst| d < bst.0) {
                        best = Some((d, bi, von_oben, links));
                    }
                }
            }
        }
        let (_, bi, von_oben, links) = best.unwrap();
        let b = offen.remove(bi);
        let weg = bereich_sticken(&b, y0, e, von_oben, links);
        // Teilbereiche mit einem Laufweg am Rand verbinden statt mit einem Sprung (wie im Original)
        let verbunden = match (wege.last_mut(), pos, weg.first()) {
            (Some(letzter), Some(von), Some(&nach)) => match laufweg(von, nach, &gedreht, e) {
                Some(lauf) => {
                    let letzter: &mut Linie = letzter;
                    letzter.extend(lauf[1..].iter().chain(&weg[1..]));
                    true
                }
                None => false,
            },
            _ => false,
        };
        pos = weg.last().copied();
        if !verbunden {
            wege.push(weg);
        }
    }
    wege.into_iter()
        .map(|weg| weg.into_iter().map(|q| drehen(q, w, null)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::{abstand_punkt_strecke, im_polygon, kreis, rand_abstand, rechteck};

    fn stiche(wege: &[Linie]) -> Vec<(Punkt, Punkt)> {
        wege.iter().flat_map(|w| w.windows(2).map(|s| (s[0], s[1]))).collect()
    }

    #[test]
    fn rechteck_reihen_und_grenzen() {
        let e = TatamiEinstellung {
            winkel: 0.0,
            ..Default::default()
        };
        let wege = tatami(&rechteck(p(0.0, 0.0), 20.0, 10.0), &[], &e, None, None);
        assert_eq!(wege.len(), 1);
        let mut ys: Vec<i64> = wege[0].iter().map(|q| (q.y * 1e6).round() as i64).collect();
        ys.sort();
        ys.dedup();
        assert_eq!(ys.len(), 50); // 10 mm hoch, Nachbarreihen 0,20 mm (Abstand 0,40 mm zwischen Hinreihen)
        for q in &wege[0] {
            assert!(q.x.abs() <= 10.2 + 1e-9 && q.y.abs() <= 5.0);
        }
        for (a, b) in stiche(&wege) {
            assert!(abstand(a, b) <= e.laenge + e.min_rand + 1e-9);
        }
    }

    #[test]
    fn loch_bleibt_frei_und_flaeche_bedeckt() {
        let e = TatamiEinstellung::default();
        let aussen = kreis(p(0.0, 0.0), 15.0, 0.02, 0.0);
        let loch = kreis(p(0.0, 0.0), 6.0, 0.02, 0.0);
        let wege = tatami(&aussen, &[loch], &e, None, None);
        let st = stiche(&wege);
        for &(a, b) in &st {
            for t in [0.0, 0.5, 1.0] {
                let q = p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t);
                let r = q.x.hypot(q.y);
                assert!(r >= 6.0 - e.zugausgleich - 0.05 && r <= 15.0 + e.zugausgleich + 0.05);
            }
        }
        // Abdeckung: Punkte im Ring liegen nah an einem Stich (feste Pseudozufallsfolge)
        let mut z: u64 = 12345;
        for _ in 0..200 {
            z = z.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = 7.0 + 7.0 * ((z >> 11) as f64 / (1u64 << 53) as f64);
            z = z.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let w = std::f64::consts::TAU * ((z >> 11) as f64 / (1u64 << 53) as f64);
            let q = p(r * w.cos(), r * w.sin());
            let d = st
                .iter()
                .map(|&(a, b)| abstand_punkt_strecke(q, a, b))
                .fold(f64::INFINITY, f64::min);
            assert!(d <= e.abstand);
        }
    }

    #[test]
    fn reihen_beim_halben_abstand() {
        // Abstand 0,40 mm zwischen zwei Hinreihen: Nachbarreihen 0,20 mm auseinander
        let e = TatamiEinstellung {
            winkel: 0.0,
            zugausgleich: 0.0,
            ..Default::default()
        };
        let wege = tatami(&rechteck(p(0.0, 0.0), 20.0, 10.0), &[], &e, None, None);
        let mut ys: Vec<f64> = wege.iter().flatten().map(|q| (q.y * 1000.0).round() / 1000.0).collect();
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ys.dedup();
        assert!(ys.windows(2).all(|w| (w[1] - w[0] - 0.20).abs() < 1e-6));
    }

    #[test]
    fn winkel() {
        let e = TatamiEinstellung {
            winkel: 30.0,
            ..Default::default()
        };
        let wege = tatami(&rechteck(p(0.0, 0.0), 30.0, 30.0), &[], &e, None, None);
        for (a, b) in stiche(&wege).into_iter().filter(|(a, b)| abstand(*a, *b) > 2.0) {
            let w = (b.y - a.y).atan2(b.x - a.x).to_degrees().rem_euclid(180.0);
            assert!((w - 30.0).abs() < 0.01);
        }
    }

    #[test]
    fn u_form_mehrere_bereiche() {
        let u = vec![
            p(-10.0, -10.0),
            p(10.0, -10.0),
            p(10.0, 10.0),
            p(5.0, 10.0),
            p(5.0, -5.0),
            p(-5.0, -5.0),
            p(-5.0, 10.0),
            p(-10.0, 10.0),
        ];
        let e = TatamiEinstellung {
            winkel: 0.0,
            ..Default::default()
        };
        let wege = tatami(&u, &[], &e, None, None);
        // mehrere Teilbereiche, aber wie im Original mit Laufwegen am Rand verbunden: ein Weg, kein Sprung
        assert_eq!(wege.len(), 1);
        assert!(
            stiche(&wege)
                .iter()
                .all(|&(a, b)| abstand(a, b) <= e.laenge + e.min_rand + 1e-9)
        );
        let ringe = vec![u.clone()];
        for (a, b) in stiche(&wege) {
            let m = p((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
            assert!(im_polygon(m, &ringe) || rand_abstand(m, &ringe) <= 0.25);
        }
    }
}
