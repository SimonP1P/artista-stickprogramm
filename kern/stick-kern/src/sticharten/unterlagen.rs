//! Unterlagen (Underlay) für Satinsäulen und Flächen.
//!
//! Werte aus der Analyse (`Analyse/code/10` §D, mittel). Jede Satin-Unterlage beginnt und endet am Anfang
//! der Säule, damit der Deckstich danach an der Startkante beginnt (eigene Festlegung).

use super::lauf::laufstich;
use super::satin::{mittellinie, paare_bilden};
use super::tatami::tatami;
use crate::einstellungen::{LaufEinstellung, LaufGrenzen, TatamiEinstellung, UnterlageEinstellung};
use crate::geometrie::{Linie, Punkt, abstand, richtung, versatz_ring, verschieben, zwischen};

pub const SATIN_UNTERLAGEN: [&str; 5] = ["keine", "mittellauf", "edge_walk", "zickzack", "doppelzickzack"];

/// Unterlage „auto“ wie das Werkzeug „Auto Underlay“ des Originals: aus, oder die im Stil vorgewählte Art.
pub fn auto_satin_unterlage(e: &UnterlageEinstellung) -> Vec<String> {
    if e.auto_an { e.auto_art.clone() } else { Vec::new() }
}

/// Seiten um `rand` nach innen rücken; zu schmale Stellen fallen auf die Mitte zusammen.
fn eingerueckt(a: &[Punkt], b: &[Punkt], rand: f64) -> (Linie, Linie) {
    let (mut ai, mut bi) = (Vec::with_capacity(a.len()), Vec::with_capacity(b.len()));
    for (&q, &r) in a.iter().zip(b) {
        if abstand(q, r) <= 2.0 * rand + 0.3 {
            let m = zwischen(q, r, 0.5);
            ai.push(m);
            bi.push(m);
        } else {
            ai.push(verschieben(q, richtung(q, r), rand));
            bi.push(verschieben(r, richtung(r, q), rand));
        }
    }
    (ai, bi)
}

/// Lauf einer Unterlage: im Original „variabel“ (0x271e = 1) mit Mindeststich und Sehnenabstand je Art.
fn lauf(linie: &[Punkt], laenge: f64, g: LaufGrenzen) -> Linie {
    laufstich(
        linie,
        &LaufEinstellung {
            laenge,
            min_laenge: g.min_laenge,
            variabel: true,
            sehnenabstand: g.sehnenabstand,
            ecke_grad: 30.0,
        },
        None,
    )
}

/// Zickzack zwischen a und b: Einstich abwechselnd auf a/b alle laenge/2 Weg (Mittel beider Seiten).
fn zickzack(a: &[Punkt], b: &[Punkt], laenge: f64, phase: f64) -> Linie {
    let halb = laenge / 2.0;
    let mut out = vec![a[0]];
    let mut seite_b = true;
    let mut gelaufen = halb - phase * halb;
    for i in 0..a.len() - 1 {
        let seg = (abstand(a[i], a[i + 1]) + abstand(b[i], b[i + 1])) / 2.0;
        if seg < 1e-9 {
            continue;
        }
        let mut t = 0.0;
        loop {
            let noetig = (halb - gelaufen).max(0.0);
            if noetig > (1.0 - t) * seg + 1e-9 {
                gelaufen += (1.0 - t) * seg;
                break;
            }
            t += noetig / seg;
            gelaufen = 0.0;
            out.push(if seite_b {
                zwischen(b[i], b[i + 1], t)
            } else {
                zwischen(a[i], a[i + 1], t)
            });
            seite_b = !seite_b;
        }
    }
    out.push(if seite_b {
        *b.last().unwrap()
    } else {
        *a.last().unwrap()
    });
    out
}

/// Unterlage-Einstiche für eine Satinsäule; Start und Ende am Säulenanfang (Mitte bzw. Seite A).
pub fn satin_unterlage(
    seite_a: &[Punkt],
    seite_b: &[Punkt],
    arten: &[String],
    e: &UnterlageEinstellung,
) -> Result<Linie, String> {
    let (a, b) = paare_bilden(seite_a, seite_b)?;
    let (ai, bi) = eingerueckt(&a, &b, e.rand);
    let mitte = mittellinie(&a, &b);
    let mut out: Linie = Vec::new();
    fn anhaengen(out: &mut Linie, pts: Linie) {
        let start = usize::from(matches!((out.last(), pts.first()), (Some(&l), Some(&f)) if abstand(l, f) < 1e-9));
        out.extend_from_slice(&pts[start..]);
    }
    let umgekehrt = |l: &[Punkt]| -> Linie { l.iter().rev().copied().collect() };

    for art in arten {
        match art.as_str() {
            "keine" => {}
            "mittellauf" => {
                let hin = lauf(&mitte, e.mittellauf_laenge, e.mittellauf_lauf);
                anhaengen(&mut out, hin.clone());
                anhaengen(&mut out, umgekehrt(&hin));
            }
            "edge_walk" => {
                anhaengen(&mut out, lauf(&ai, e.edge_walk_laenge, e.edge_walk_lauf));
                anhaengen(&mut out, lauf(&umgekehrt(&bi), e.edge_walk_laenge, e.edge_walk_lauf));
                anhaengen(&mut out, vec![ai[0]]);
            }
            "zickzack" | "doppelzickzack" => {
                let laenge = if art == "zickzack" {
                    e.zickzack_laenge
                } else {
                    e.doppelzickzack_laenge
                };
                anhaengen(&mut out, zickzack(&ai, &bi, laenge, 0.0));
                if art == "zickzack" {
                    // „+ Lauf 3,0 mm“ zurück zum Anfang
                    anhaengen(&mut out, lauf(&umgekehrt(&mitte), e.rueck_lauf_laenge, e.zickzack_lauf));
                } else {
                    anhaengen(&mut out, zickzack(&umgekehrt(&bi), &umgekehrt(&ai), laenge, 0.5));
                }
            }
            andere => {
                return Err(format!(
                    "Unbekannte Satin-Unterlage '{andere}'. Möglich: {}",
                    SATIN_UNTERLAGEN.join(", ")
                ));
            }
        }
    }
    Ok(out)
}

/// Tatami-Unterlage: Randlauf (eingerückt) + Tatami 3,0/4,0 mm quer zur Deckstichrichtung.
pub fn flaechen_unterlage(
    umriss: &[Punkt],
    loecher: &[Linie],
    deck_winkel: f64,
    e: &UnterlageEinstellung,
    start: Option<Punkt>,
) -> Vec<Linie> {
    let rand_umriss = versatz_ring(umriss, e.rand, 3.0);
    let rand_loecher: Vec<Linie> = loecher.iter().map(|l| versatz_ring(l, -e.rand, 3.0)).collect();
    let mut ring: Linie = rand_umriss.clone();
    ring.push(rand_umriss[0]);
    if let Some(s) = start {
        let mut i = 0;
        for j in 1..rand_umriss.len() {
            if abstand(rand_umriss[j], s) < abstand(rand_umriss[i], s) {
                i = j;
            }
        }
        ring = rand_umriss[i..].iter().chain(&rand_umriss[..=i]).copied().collect();
    }
    let mut wege = vec![lauf(&ring, e.tatami_randlauf, e.tatami_randlauf_lauf)];
    let te = TatamiEinstellung {
        abstand: e.tatami_abstand,
        laenge: e.tatami_laenge,
        min_rand: 0.4,
        versatz: 0.0,
        winkel: deck_winkel + e.tatami_winkel_versatz,
        zugausgleich: e.zugausgleich,
    };
    let start_fuellung = wege[0].last().copied();
    wege.extend(tatami(&rand_umriss, &rand_loecher, &te, None, start_fuellung));
    wege
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::{p, rechteck};
    use crate::sticharten::satin::seiten_aus_mittellinie;

    #[test]
    fn flaechen_unterlage_innerhalb_und_quer() {
        let wege = flaechen_unterlage(
            &rechteck(p(0.0, 0.0), 30.0, 20.0),
            &[],
            45.0,
            &UnterlageEinstellung::default(),
            None,
        );
        for q in wege.iter().flatten() {
            assert!(q.x.abs() <= 15.0 + 1e-6 && q.y.abs() <= 10.0 + 1e-6);
        }
        let winkel: Vec<i64> = wege[1..]
            .iter()
            .flat_map(|w| w.windows(2))
            .filter(|s| abstand(s[0], s[1]) > 2.0)
            .map(|s| {
                (s[1].y - s[0].y)
                    .atan2(s[1].x - s[0].x)
                    .to_degrees()
                    .rem_euclid(180.0)
                    .round() as i64
            })
            .collect();
        let haeufigste = winkel
            .iter()
            .max_by_key(|w| winkel.iter().filter(|v| v == w).count())
            .unwrap();
        assert_eq!(*haeufigste, 135);
    }

    #[test]
    fn satin_unterlagen_bleiben_innen() {
        let (a, b) = seiten_aus_mittellinie(&[p(0.0, 0.0), p(30.0, 0.0)], 6.0);
        for arten in [
            vec!["mittellauf"],
            vec!["edge_walk"],
            vec!["zickzack"],
            vec!["doppelzickzack"],
            vec!["edge_walk", "zickzack"],
        ] {
            let arten: Vec<String> = arten.iter().map(|s| s.to_string()).collect();
            let pts = satin_unterlage(&a, &b, &arten, &UnterlageEinstellung::default()).unwrap();
            assert!(!pts.is_empty());
            for q in &pts {
                assert!((-0.01..=30.01).contains(&q.x) && q.y.abs() <= 3.0 - 0.2 + 1e-6);
            }
            assert!(pts.last().unwrap().x <= 3.0);
            assert!(pts.windows(2).all(|w| abstand(w[0], w[1]) <= 7.0));
        }
    }

    #[test]
    fn auto_unterlage() {
        // Software Defaults: Auto Underlay aus; eingeschaltet die vorgewählte Art Zickzack
        let mut e = UnterlageEinstellung::default();
        assert!(auto_satin_unterlage(&e).is_empty());
        e.auto_an = true;
        assert_eq!(auto_satin_unterlage(&e), ["zickzack"]);
        assert!(
            satin_unterlage(
                &[p(0.0, 1.0), p(5.0, 1.0)],
                &[p(0.0, -1.0), p(5.0, -1.0)],
                &["quer".into()],
                &UnterlageEinstellung::default()
            )
            .is_err()
        );
    }
}
