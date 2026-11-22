//! Stippling (Stipple Fill, Craft-Stitch-Werkzeugleiste): eine Fläche mit einer mäandernden Laufstichlinie füllen,
//! die sich nicht kreuzt. Einstellungen wie im Dialog „Stipple“ (BERESENU DLG 1450): **Stichlänge** und
//! **Schlaufenabstand** (Loop Spacing).
//!
//! Das Original rechnet über die Mittelachse der Fläche (`StippleFillInterface::StippleFill_MedialAxisSequence`,
//! MOD1 0x101a4970/0x101a4c30, `CreateStippleFillPolygons` 0x101a4590) – nicht nachgebaut. **Eigene Festlegung**
//! (`Analyse_Teil3/24_craft_stitches.md` §4):
//! 1. Raster mit Zellen von 2 × Schlaufenabstand; Zellen, deren Mitte mindestens einen Schlaufenabstand vom Rand
//!    entfernt in der Fläche liegt, zählen.
//! 2. Je zusammenhängendem Teil ein zufälliger Spannbaum (Park–Miller mit festem Startwert → bei jeder Berechnung
//!    gleich).
//! 3. Die Linie läuft im halben Zellabstand um den Baum herum (Viertelzellen-Kreis, „Spanning Tree Coverage“): ein
//!    geschlossener Weg ohne Kreuzung, benachbarte Bahnen liegen einen Schlaufenabstand auseinander.
//! 4. Ecken leicht verschoben und zweimal abgerundet (Chaikin), dann Laufstich mit der Stichlänge.
//!
//! Vorgaben: Stichlänge 2,0 mm, Schlaufenabstand 3,0 mm (jeweils erstes Beispielbild im Handbuch S. 243; weitere
//! Beispiele 1,0 mm bzw. 1,5 mm). Die Werte des Originals liegen im Dokument (`DesignDocument::
//! GetCurrentStippleFillValues` +0x138), nicht im Stil – Sicherheit niedrig. Handbuch: „Stippling is a method of
//! creating a fill made of run stitches which meander around within a border“ (S. 242).

use crate::einstellungen::LaufEinstellung;
use crate::geometrie::{Linie, Punkt, abstand, im_polygon, p, rand_abstand};
use crate::sticharten::lauf::laufstich_mit_ecken;
use std::collections::{BTreeMap, BTreeSet};

/// Vorgabe Stichlänge (Handbuch-Beispiel)
pub const LAENGE: f64 = 2.0;
/// Vorgabe Schlaufenabstand (Handbuch-Beispiel)
pub const ABSTAND: f64 = 3.0;

/// Stippling-Linien (je zusammenhängendem Teil der Fläche eine geschlossene Linie) als Laufstiche. `start` = letzter
/// Stich davor: der erste Teil beginnt am nächsten Punkt.
pub fn stippling(
    umriss: &[Punkt],
    loecher: &[Linie],
    laenge: f64,
    schlaufe: f64,
    e: &LaufEinstellung,
    start: Option<Punkt>,
) -> Vec<Linie> {
    let mut wege = Vec::new();
    let mut von = start;
    for weg in stippling_wege(umriss, loecher, schlaufe, von) {
        let ecken: Vec<bool> = (0..weg.len()).map(|i| i == 0 || i + 1 == weg.len()).collect();
        let lauf = laufstich_mit_ecken(&weg, Some(&ecken), e, Some(laenge));
        von = lauf.last().copied().or(von);
        wege.push(lauf);
    }
    wege
}

/// Park–Miller „minimal standard“ (wie Rand-Effekt und Kurzstiche).
struct Zufall(u64);
impl Zufall {
    fn naechste(&mut self, n: usize) -> usize {
        self.0 = self.0 * 16807 % 2_147_483_647;
        (self.0 % n as u64) as usize
    }
    fn wert(&mut self) -> f64 {
        self.0 = self.0 * 16807 % 2_147_483_647;
        self.0 as f64 / 2_147_483_647.0
    }
}

/// Geometrie der Stippling-Linien (vor dem Laufstich), geschlossen (letzter Punkt = erster).
pub fn stippling_wege(umriss: &[Punkt], loecher: &[Linie], schlaufe: f64, start: Option<Punkt>) -> Vec<Linie> {
    let s = schlaufe.max(0.3);
    let c = 2.0 * s;
    let mut ringe = vec![umriss.to_vec()];
    ringe.extend(loecher.iter().cloned());
    let Some((x0, y0, x1, y1)) = crate::geometrie::begrenzung(umriss.iter().copied()) else {
        return Vec::new();
    };
    let (nx, ny) = (((x1 - x0) / c).ceil() as i64 + 1, ((y1 - y0) / c).ceil() as i64 + 1);
    // Raster zentriert über die Fläche
    let ox = (x0 + x1) / 2.0 - nx as f64 * c / 2.0;
    let oy = (y0 + y1) / 2.0 - ny as f64 * c / 2.0;
    let mitte = |i: i64, j: i64| p(ox + (i as f64 + 0.5) * c, oy + (j as f64 + 0.5) * c);
    let mut zellen = BTreeSet::new();
    for i in 0..nx {
        for j in 0..ny {
            let m = mitte(i, j);
            if im_polygon(m, &ringe) && rand_abstand(m, &ringe) >= s * 0.999 {
                zellen.insert((i, j));
            }
        }
    }
    // zusammenhängende Teile
    let mut teile: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut gesehen = BTreeSet::new();
    for &z in &zellen {
        if !gesehen.insert(z) {
            continue;
        }
        let mut teil = vec![z];
        let mut k = 0;
        while k < teil.len() {
            let (i, j) = teil[k];
            for n in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if zellen.contains(&n) && gesehen.insert(n) {
                    teil.push(n);
                }
            }
            k += 1;
        }
        teile.push(teil);
    }
    let mut zufall = Zufall(20_031_997);
    let mut wege: Vec<Linie> = teile
        .iter()
        .map(|teil| teil_weg(teil, &zellen, s, &mitte, &mut zufall))
        .filter(|w| w.len() > 2)
        .collect();
    // Reihenfolge: jeweils der nächste Teil; Beginn am Punkt nächst dem vorigen Ende
    let mut out = Vec::new();
    let mut von = start.or_else(|| umriss.first().copied());
    while !wege.is_empty() {
        let (k, idx) = wege
            .iter()
            .enumerate()
            .map(|(k, w)| {
                let idx = von.map_or(0, |v| {
                    (0..w.len() - 1)
                        .min_by(|&a, &b| abstand(w[a], v).total_cmp(&abstand(w[b], v)))
                        .unwrap_or(0)
                });
                (k, idx)
            })
            .min_by(|a, b| {
                let d = |(k, i): &(usize, usize)| von.map_or(0.0, |v| abstand(wege[*k][*i], v));
                d(a).total_cmp(&d(b))
            })
            .unwrap();
        let w = wege.remove(k);
        let ring = &w[..w.len() - 1];
        let mut neu: Linie = ring[idx..].iter().chain(&ring[..idx]).copied().collect();
        neu.push(neu[0]);
        von = neu.last().copied();
        out.push(neu);
    }
    out
}

/// Spannbaum über die Zellen eines Teils und Weg um den Baum (Viertelzellen-Kreis), abgerundet.
fn teil_weg(
    teil: &[(i64, i64)],
    zellen: &BTreeSet<(i64, i64)>,
    s: f64,
    mitte: &dyn Fn(i64, i64) -> Punkt,
    zufall: &mut Zufall,
) -> Linie {
    // zufälliger Spannbaum (Prim mit zufällig gewählter Kante aus dem Rand)
    let mut drin = BTreeSet::new();
    let mut baum: BTreeSet<((i64, i64), (i64, i64))> = BTreeSet::new();
    let mut rand: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let nachbarn = |(i, j): (i64, i64)| [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
    drin.insert(teil[0]);
    for n in nachbarn(teil[0]) {
        if zellen.contains(&n) {
            rand.push((teil[0], n));
        }
    }
    while !rand.is_empty() {
        let k = zufall.naechste(rand.len());
        let (a, b) = rand.swap_remove(k);
        if drin.contains(&b) {
            continue;
        }
        drin.insert(b);
        baum.insert((a.min(b), a.max(b)));
        for n in nachbarn(b) {
            if zellen.contains(&n) && !drin.contains(&n) {
                rand.push((b, n));
            }
        }
    }
    // Viertelzellen (2i + a, 2j + b): je Zelle ein Quadrat, an Baumkanten geöffnet und verbunden
    type V = (i64, i64);
    let mut kanten: BTreeSet<(V, V)> = BTreeSet::new();
    let kante = |a: V, b: V| (a.min(b), a.max(b));
    for &(i, j) in teil {
        let (u, v) = (2 * i, 2 * j);
        for k in [
            kante((u, v), (u + 1, v)),
            kante((u + 1, v), (u + 1, v + 1)),
            kante((u, v + 1), (u + 1, v + 1)),
            kante((u, v), (u, v + 1)),
        ] {
            kanten.insert(k);
        }
    }
    for &(a, b) in &baum {
        let (u, v) = (2 * a.0, 2 * a.1);
        if b.0 == a.0 + 1 {
            kanten.remove(&kante((u + 1, v), (u + 1, v + 1)));
            kanten.remove(&kante((u + 2, v), (u + 2, v + 1)));
            kanten.insert(kante((u + 1, v), (u + 2, v)));
            kanten.insert(kante((u + 1, v + 1), (u + 2, v + 1)));
        } else {
            kanten.remove(&kante((u, v + 1), (u + 1, v + 1)));
            kanten.remove(&kante((u, v + 2), (u + 1, v + 2)));
            kanten.insert(kante((u, v + 1), (u, v + 2)));
            kanten.insert(kante((u + 1, v + 1), (u + 1, v + 2)));
        }
    }
    let mut nb: BTreeMap<V, Vec<V>> = BTreeMap::new();
    for &(a, b) in &kanten {
        nb.entry(a).or_default().push(b);
        nb.entry(b).or_default().push(a);
    }
    let Some(&anfang) = nb.keys().next() else {
        return Vec::new();
    };
    let mut kreis = vec![anfang];
    let (mut vor, mut jetzt) = (anfang, nb[&anfang][0]);
    while jetzt != anfang && kreis.len() <= nb.len() {
        kreis.push(jetzt);
        let n = &nb[&jetzt];
        let weiter = if n[0] == vor { n[1] } else { n[0] };
        vor = jetzt;
        jetzt = weiter;
    }
    // Viertelzelle → Punkt (Mitte der Viertelzelle), leicht verschoben
    let viertel = |(a, b): V| {
        let m = mitte(a.div_euclid(2), b.div_euclid(2));
        p(
            m.x + if a.rem_euclid(2) == 0 { -s / 2.0 } else { s / 2.0 },
            m.y + if b.rem_euclid(2) == 0 { -s / 2.0 } else { s / 2.0 },
        )
    };
    let mut pts: Linie = kreis
        .iter()
        .map(|&v| {
            let q = viertel(v);
            p(q.x + (zufall.wert() - 0.5) * 0.3 * s, q.y + (zufall.wert() - 0.5) * 0.3 * s)
        })
        .collect();
    for _ in 0..2 {
        pts = chaikin(&pts);
    }
    pts.push(pts[0]);
    pts
}

/// Ecken eines geschlossenen Linienzugs abrunden (je Kante zwei Punkte bei 1/4 und 3/4).
fn chaikin(ring: &[Punkt]) -> Linie {
    let n = ring.len();
    let mut out = Vec::with_capacity(2 * n);
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        out.push(p(0.75 * a.x + 0.25 * b.x, 0.75 * a.y + 0.25 * b.y));
        out.push(p(0.25 * a.x + 0.75 * b.x, 0.25 * a.y + 0.75 * b.y));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::rechteck;

    /// Schnitt zweier Strecken (ohne gemeinsame Endpunkte)
    fn schneiden(a: Punkt, b: Punkt, c: Punkt, d: Punkt) -> bool {
        let k = |p: Punkt, q: Punkt, r: Punkt| (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
        let (d1, d2, d3, d4) = (k(c, d, a), k(c, d, b), k(a, b, c), k(a, b, d));
        d1 * d2 < -1e-12 && d3 * d4 < -1e-12
    }

    #[test]
    fn stippling_fuellt_ohne_kreuzung() {
        let r = rechteck(p(0.0, 0.0), 30.0, 20.0);
        let wege = stippling_wege(&r, &[], 1.0, None);
        assert_eq!(wege.len(), 1);
        let w = &wege[0];
        assert_eq!(w.first(), w.last());
        // in der Fläche, mit Abstand zum Rand
        let ringe = vec![r.clone()];
        assert!(w.iter().all(|&q| im_polygon(q, &ringe)));
        // keine Kreuzung
        let n = w.len() - 1;
        for i in 0..n {
            for j in i + 2..n {
                if i == 0 && j == n - 1 {
                    continue;
                }
                assert!(!schneiden(w[i], w[i + 1], w[j], w[j + 1]), "Kreuzung bei {i}/{j}");
            }
        }
        // dichte Füllung: Linie ungefähr Fläche / Schlaufenabstand lang
        let l = crate::geometrie::laenge(w);
        assert!(l > 0.6 * 600.0 / 1.0 && l < 1.2 * 600.0, "Länge {l}");
        // gleich bei jeder Berechnung
        assert_eq!(wege, stippling_wege(&r, &[], 1.0, None));
    }

    #[test]
    fn stippling_mit_loch_und_teilen() {
        let r = rechteck(p(0.0, 0.0), 30.0, 30.0);
        let loch = rechteck(p(0.0, 0.0), 10.0, 10.0);
        let wege = stippling_wege(&r, std::slice::from_ref(&loch), 1.0, Some(p(-20.0, -20.0)));
        assert!(!wege.is_empty());
        let ringe = vec![r.clone(), loch.clone()];
        assert!(wege.iter().flatten().all(|&q| im_polygon(q, &ringe)));
        // Laufstich mit Stichlänge
        let st = stippling(&r, &[loch], 2.0, 1.0, &LaufEinstellung::default(), None);
        let max = st.iter().flat_map(|w| w.windows(2)).map(|w| abstand(w[0], w[1])).fold(0.0, f64::max);
        assert!(max <= 2.0 + 1e-9, "Stich {max}");
    }
}
