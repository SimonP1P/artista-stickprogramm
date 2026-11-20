//! Satinsäule zwischen zwei Seitenlinien (wie „Satin Column A“ im Original).
//!
//! Die Säule wird durch zwei Seiten A und B beschrieben, beide in Stickrichtung. Punkt i von A gehört zu
//! Punkt i von B (Stichrichtungs-Linie). Haben A und B verschieden viele Punkte, werden sie über die
//! relative Bogenlänge einander zugeordnet.

use crate::einstellungen::{SatinEinstellung, TatamiEinstellung};
use crate::geometrie::{
    Linie, Punkt, abstand, abstand_punkt_strecke, kumuliert, ohne_doppelte, punkt_bei, richtung, versatz_linie,
    verschieben, zwischen,
};

/// Step (Tatami) in einer Säule – Schrift mit Stichart Step (Handbuch Kap. 20 „Adjusting Step stitch settings for
/// lettering objects“: Abstand, Länge, Step-Muster). `reihen` = Einstiche einer Satinsäule (abwechselnd Seite A, B),
/// berechnet mit dem Step-Abstand (AMT 0x2716, Weg zwischen zwei Hinreihen – jede Reihe rückt um den halben Abstand
/// vor wie beim Satin); jede Reihe wird in Stiche der Step-Länge (0x2717) geteilt, das Raster je Reihe um den
/// Reihenversatz (0x2719) verschoben, Mindeststich am Rand 0x2718 – wie `tatami`. Die Reihen folgen damit den
/// Stichwinkeln der Buchstaben (Handbuch: „Stitches are regenerated with the new angles“). Raster von Seite A aus
/// gemessen: **eigene Festlegung** (keine Schrift mit Step in den Musterdesigns, `Analyse_Teil3/15`).
pub fn step_reihen(reihen: &[Punkt], e: &TatamiEinstellung) -> Linie {
    let mut out: Linie = reihen.iter().take(1).copied().collect();
    let l = e.laenge;
    for (k, w) in reihen.windows(2).enumerate() {
        // gerade Reihen von A nach B, ungerade von B nach A; das Raster zählt immer von A aus
        let (a, b) = if k % 2 == 0 { (w[0], w[1]) } else { (w[1], w[0]) };
        let d = abstand(a, b);
        let phase = ((k as f64 * e.versatz) % 1.0) * l;
        let mut xs = Vec::new();
        let mut m = ((e.min_rand - phase) / l).ceil() as i64;
        loop {
            let x = phase + m as f64 * l;
            if x > d - e.min_rand {
                break;
            }
            xs.push(x);
            m += 1;
        }
        if k % 2 == 1 {
            xs.reverse();
        }
        out.extend(xs.into_iter().map(|x| zwischen(a, b, x / d)));
        out.push(w[1]);
    }
    out
}

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
/// Abstand aus Auto Spacing (Breite an der Stelle) bzw. fest. Wie im Original ist der Abstand der Weg zwischen
/// zwei Einstichen **auf derselben Seite** (Handbuch PDF-S. 68; an 184 000 Satinstichen der Musterdesigns gemessen,
/// `Analyse_Teil3/04_messungen_satin.md`), jeder Einstich rückt also um den halben Abstand vor. Gemessen wird der
/// Fortschritt auf der **längeren Seite** (Außenseite im Bogen): dort entspricht der Weg in den Musterdesigns dem
/// Abstand in allen Krümmungen (Median 1,00–1,08), auf der Mitte nur 0,64–0,99 (`Analyse_Teil3/04` §4).
/// Zugausgleich verlängert jeden Stich an beiden Enden.
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
        let seg = abstand(a[i], a[i + 1]).max(abstand(b[i], b[i + 1]));
        if seg < 1e-9 {
            continue;
        }
        let mut t = 0.0;
        loop {
            let (pa, pb) = (zwischen(a[i], a[i + 1], t), zwischen(b[i], b[i + 1], t));
            let noetig = (e.abstand(abstand(pa, pb)) / 2.0 - gelaufen).max(0.0);
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
    // Kurzstiche nur an den regelmäßigen Einstichen, vor dem Abschluss an der Endkante
    kurzstiche(&mut out, e);
    // Ende sauber abschließen: letzter Stich auf die Endkante
    let ende = einstich(*a.last().unwrap(), *b.last().unwrap(), seite);
    if abstand(*out.last().unwrap(), ende) > 1e-6 {
        out.push(ende);
    }
    Ok(out)
}

/// Zufallszahlen für die Kurzstiche. Das Original nimmt geometry.dll `ACMRandomNumber` mit festem Startwert 0x66ec8f
/// (MOD1 0x10186ef0) – das Verfahren selbst ist **Annahme**: Park–Miller „minimal standard“ (Communications of the
/// ACM 1988, der Name legt es nahe; geometry.dll noch nicht gelesen).
struct AcmZufall(u64);

impl AcmZufall {
    fn naechste(&mut self) -> f64 {
        self.0 = self.0 * 16807 % 2_147_483_647;
        self.0 as f64 / 2_147_483_647.0
    }
}

/// Kurzstiche an Innenkurven wie MOD1 `ShortStitchesEffect` (0x10186ed0 mit 0x10186ad0 und 0x10186a60):
/// Die Einstiche liegen paarweise vor (A, B, A, B …). Für jede Seite nacheinander: Kommen zwei Nachbarstiche auf
/// dieser Seite näher als `schwelle` × Abstand zusammen (kleinster Abstand Einstich ↔ Nachbarstich, 0x10186db0), wird
/// eine Gruppe aus bis zu `max_gruppe` Stichen gebildet, und jeder Stich der Gruppe wird auf Faktor × Länge gekürzt
/// (Einstich rückt zur Gegenseite; Faktor aus der Dreieckstabelle, mit Zufall ±0,04).
/// Sicherheit: Ablauf, Schwelle, Tabelle und Zufallswerte hoch (Code); die Gruppenbildung ist **vereinfacht**
/// (Original 0x10186ad0 mit Toleranzen 0,9/1,1/1,33): Gruppe = folgende Stiche, die näher als 0,9 × Abstand am
/// ersten Stich der Gruppe liegen.
pub fn kurzstiche(pts: &mut [Punkt], e: &SatinEinstellung) {
    let k = &e.kurzstiche;
    let n_paare = pts.len() / 2;
    if !k.an || k.schwelle <= 0.0 || n_paare < 3 {
        return;
    }
    let mut zufall = AcmZufall(0x66ec8f);
    for s in 0..2 {
        let o = 1 - s;
        let p = |pts: &[Punkt], i: usize, seite: usize| pts[2 * i + seite];
        let dicht = |pts: &[Punkt], i: usize, j: usize| -> f64 {
            abstand_punkt_strecke(p(pts, j, s), p(pts, i, s), p(pts, i, o)).min(abstand_punkt_strecke(
                p(pts, i, s),
                p(pts, j, s),
                p(pts, j, o),
            ))
        };
        let mut i = 1;
        while i < n_paare {
            let sp = e.abstand(abstand(p(pts, i, s), p(pts, i, o)));
            if dicht(pts, i - 1, i) >= k.schwelle * sp {
                i += 1;
                continue;
            }
            let mut n = 1;
            while n < k.max_gruppe && i + n < n_paare && dicht(pts, i, i + n) < 0.9 * sp {
                n += 1;
            }
            for j in 0..n {
                let mut f = k.faktor(n, j);
                if k.zufall {
                    f += [-0.04, 0.0, 0.04][((zufall.naechste() * 3.0) as usize).min(2)];
                }
                let (ps, po) = (p(pts, i + j, s), p(pts, i + j, o));
                pts[2 * (i + j) + s] = Punkt::new(po.x + f * (ps.x - po.x), po.y + f * (ps.y - po.y));
            }
            i += n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::einstellungen::{KurzstichEinstellung, fuer_stoff};
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
        // 0,40 mm zwischen zwei Einstichen derselben Seite, also 0,20 mm Vorschub je Einstich
        assert!(dx(&pts).iter().all(|d| (d - 0.20).abs() < 1e-9));
        assert!((pts.len() as f64 - (20.0 / 0.20 + 2.0)).abs() <= 1.0);
        assert!((pts.last().unwrap().x - 20.0).abs() < 1e-9);
    }

    #[test]
    fn schmale_bahn_lockerer() {
        let (a, b) = gerade(0.5);
        let pts = satin(&a, &b, &SatinEinstellung::default()).unwrap();
        assert!(dx(&pts).iter().all(|d| (d - 0.51 / 2.0).abs() < 1e-9));
    }

    #[test]
    fn fester_abstand_und_stoff() {
        let (a, b) = gerade(2.0);
        let e = SatinEinstellung {
            auto_spacing: false,
            abstand_fest: 0.40,
            ..Default::default()
        };
        assert!(dx(&satin(&a, &b, &e).unwrap()).iter().all(|d| (d - 0.20).abs() < 1e-9));
        let e2 = fuer_stoff("Knit Medium Weight").unwrap().satin;
        assert!(
            dx(&satin(&a, &b, &e2).unwrap())
                .iter()
                .all(|d| (d - 0.46 * 1.10 / 2.0).abs() < 1e-9)
        );
    }

    #[test]
    fn kurzstiche_nur_an_der_innenkurve() {
        // enger Bogen: Mittellinie r = 3 mm, Säule 4 mm breit → innen r = 1, außen r = 5
        let mitte = bogen(p(0.0, 0.0), 3.0, 0.0, 180.0, 0.01);
        let (a, b) = seiten_aus_mittellinie(&mitte, 4.0);
        let mut e = SatinEinstellung {
            auto_spacing: false,
            abstand_fest: 0.40,
            zugausgleich: 0.0,
            ..Default::default()
        };
        let ohne = satin(&a, &b, &e).unwrap();
        e.kurzstiche.an = true;
        let mit = satin(&a, &b, &e).unwrap();
        assert_eq!(ohne.len(), mit.len());
        let geaendert: Vec<usize> = (0..ohne.len()).filter(|&i| abstand(ohne[i], mit[i]) > 1e-9).collect();
        assert!(geaendert.len() > 5, "{}", geaendert.len());
        for &i in &geaendert {
            let (r0, r1) = (ohne[i].x.hypot(ohne[i].y), mit[i].x.hypot(mit[i].y));
            assert!(r0 < 3.0 && r1 > r0, "nur innen, und nach außen gerückt: {r0} → {r1}");
            // Faktor 0,61 … 0,94 der Stichlänge (Tabelle ± 0,04): innen r 1 → höchstens 5 − 0,61 × 4
            assert!(r1 <= 5.0 - 0.6 * 4.0 + 1e-6);
        }
        // gerade Säule: nichts zu kürzen
        let (ga, gb) = gerade(3.0);
        let aus = SatinEinstellung {
            kurzstiche: KurzstichEinstellung::default(),
            ..e.clone()
        };
        assert_eq!(satin(&ga, &gb, &e).unwrap(), satin(&ga, &gb, &aus).unwrap());
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
    fn abstand_auf_der_aussenseite() {
        // Bogen Radius 10 mm, Säule 4 mm breit: außen r = 12, innen r = 8
        let mitte = bogen(p(0.0, 0.0), 10.0, 0.0, 90.0, 0.01);
        let (a, b) = seiten_aus_mittellinie(&mitte, 4.0);
        let e = SatinEinstellung {
            auto_spacing: false,
            abstand_fest: 0.40,
            zugausgleich: 0.0,
            ..Default::default()
        };
        let pts = satin(&a, &b, &e).unwrap();
        // mittlerer Weg zwischen Einstichen derselben Seite (Seite 0 = A innen, Seite 1 = B außen)
        let seite = |s: usize| {
            let w: Vec<f64> = pts[s..pts.len() - 1]
                .iter()
                .step_by(2)
                .zip(pts[s + 2..pts.len() - 1].iter().step_by(2))
                .map(|(q, r)| abstand(*q, *r))
                .collect();
            w.iter().sum::<f64>() / w.len() as f64
        };
        let (innen, aussen) = (seite(0), seite(1));
        assert!((aussen - 0.40).abs() < 0.01, "Abstand außen {aussen}");
        assert!((innen - 0.40 * 8.0 / 12.0).abs() < 0.01, "Abstand innen {innen}");
    }

    #[test]
    fn paare_verschiedene_punktzahl() {
        let a = vec![p(0.0, 1.0), p(5.0, 1.0), p(10.0, 1.0)];
        let b = vec![p(0.0, -1.0), p(10.0, -1.0)];
        let (pa, pb) = paare_bilden(&a, &b).unwrap();
        assert_eq!((pa.len(), pb.len()), (3, 3));
        assert!((pb[1].x - 5.0).abs() < 1e-9 && (pb[1].y + 1.0).abs() < 1e-9);
    }

    /// Step in der Säule: Reihen in Stiche ≤ Step-Länge (am Rand + Mindeststich) geteilt, Raster je Reihe um ¼ Länge versetzt (von A gemessen)
    #[test]
    fn step_in_der_saeule() {
        let e = crate::einstellungen::TatamiEinstellung::default();
        let reihen: Linie = (0..6)
            .map(|i| p(if i % 2 == 0 { 0.0 } else { 10.0 }, i as f64 * 0.2))
            .collect();
        let st = step_reihen(&reihen, &e);
        for w in st.windows(2) {
            assert!(abstand(w[0], w[1]) <= e.laenge + e.min_rand);
        }
        let xs: Vec<f64> = st.iter().take(11).map(|q| q.x).collect();
        let soll = [0.0, 4.0, 8.0, 10.0, 9.0, 5.0, 1.0, 0.0, 2.0, 6.0, 10.0];
        assert!(xs.iter().zip(soll).all(|(a, b)| (a - b).abs() < 0.01), "{xs:?}");
    }
}
