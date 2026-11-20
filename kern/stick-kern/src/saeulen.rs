//! Satin-Stichrichtung automatisch – Baustein von ROADMAP 4b.17 (SVG-Import) und 4b.11 (Bild → Umrisse mit Satin).
//! Befund: `Analyse_Teil3/17_auto_satin.md`.
//!
//! Aufgabe: eine beliebige schmale Fläche (Umriss mit Löchern) in **Satinsäulen** zerlegen, je Säule die Stichrichtung
//! bestimmen (Seiten A/B wie bei den Säulen der Alphabete). Bei den Alphabeten liefert das Original die Winkellinien mit;
//! für gezeichnete Formen berechnet es sie selbst – in EZSTITCH.DLL (Turning-Umwandlung `StAnglesWithOverlaps::
//! FoundPatches` 0x1005f720 → `AutoFontDecomposition::Generate` 0x100200a0; Auto Digitizer `DecomposedHomeProduct`):
//!
//! 1. **Rand abtasten** (`ShapeSample`): Abstand der Randpunkte höchstens 1 mm („low precision“ 180), verfeinert auf
//!    0,1 × örtliche Breite („proportional precision“ 0,1), nicht unter 0,05 mm („high precision“ 9) – Werte aus dem
//!    Aufruf in `Generate`.
//! 2. **Triangulieren** (Delaunay mit dem Rand als Zwangskanten, Quad-Edge-Struktur `QuadTree`) und die
//!    **bedeutenden Dreiecke** finden (`SignificantTriangles`: Abzweigungen, Ecken, Serifen).
//! 3. **Schnitte**: an Abzweigungen sternförmig (`AddStarShapedCuts`), an Knicken (Break Angle **87°**, MOD1
//!    0x101f5bc0), Ecken spitzer als **120°** (Corner Detection Angle) bekommen eine Stichlinie.
//! 4. Je Teil eine Säule aus Umriss und **Stichlinien** (EmbroideryPatch: Stichlinien = Paare von Randstellen,
//!    IMPORT.DLL 0x10018970), gemessen an 54 automatisch erzeugten Säulen der Musterdesigns (`art_autosatin.py`):
//!    benachbarte Linien im Median 1,85 mm = 4,2 × Breite auseinander, 15,6° Winkeländerung – Linien nach Krümmung.
//!
//! Die einzelnen Schritte im Code des Originals sind sehr umfangreich; übernommen sind Aufbau und Werte, die
//! Einzelheiten sind **eigene Festlegung** (markiert): Dreiecke mit drei inneren Kanten = Abzweigung (Schnitt über den
//! Schwerpunkt), ein inneres = Ende (Spitze), zwei = Säulenstück; Stichlinien = innere Kanten, ausgewählt nach
//! Douglas-Peucker auf der Mittellinie (Toleranz = Anteil der Breite, an den 54 Säulen eingestellt).

use crate::geometrie::{Linie, Punkt, abstand, flaeche_vorzeichen, geschlossen_ohne_endpunkt, kumuliert, p};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
use std::collections::HashMap;

/// Werte der Zerlegung (Original-Fundstellen in den Feldern).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaeulenWerte {
    /// größter Abstand der Randpunkte (mm) – ShapeSample „low precision“ 180 (1 mm)
    pub abtast_grob: f64,
    /// kleinster Abstand (mm) – „high precision“ 9 (0,05 mm)
    pub abtast_fein: f64,
    /// Abstand als Anteil der örtlichen Breite – „proportional precision“ 0,1
    pub abtast_anteil: f64,
    /// Säule teilen, wo sie enger knickt (Innenwinkel in Grad); 0 = nie. Der Auto Digitizer teilt nicht (gemessen:
    /// Säulen laufen auch durch Kehren von 150°), die TrueType-Umwandlung bei 87° (Break Angle, `KNICKWINKEL_TURNING`)
    pub knickwinkel: f64,
    /// Randecken mit Innenwinkel unter diesem Wert bekommen eine Stichlinie – Corner Detection Angle 120°
    pub eckwinkel: f64,
    /// Auswahl der Stichlinien: Abweichung der Mittellinie als Anteil der Breite (eigene Festlegung; an den 54
    /// automatisch erzeugten Säulen eingestellt: 0,3 ergibt 4,4 × Breite Linienabstand, Original 4,2)
    pub linien_toleranz: f64,
}

impl Default for SaeulenWerte {
    fn default() -> Self {
        SaeulenWerte {
            abtast_grob: 1.0,
            abtast_fein: 0.05,
            abtast_anteil: 0.1,
            knickwinkel: 0.0,
            eckwinkel: 120.0,
            linien_toleranz: 0.3,
        }
    }
}

/// Break Angle der Turning-Umwandlung (TrueType/AutoFont): TurningConversionValues +8 = 87° (MOD1 0x101f5bc0).
pub const KNICKWINKEL_TURNING: f64 = 87.0;

/// Teile, deren Fläche unter diesem Anteil der ganzen Form liegt (und unter `SPLITTER_MM2`), entfallen – Splitter aus
/// sich selbst berührenden Umrissen (**eigene Festlegung**).
const SPLITTER_ANTEIL: f64 = 0.02;
const SPLITTER_MM2: f64 = 0.5;

/// Eine Satinsäule: Seiten A und B (Punkt i von A gehört zu Punkt i von B), die gewählten Stichlinien und der
/// Umriss des Teils (für Laufwege und die Anzeige).
#[derive(Clone, Debug, PartialEq)]
pub struct Saeule {
    pub seite_a: Linie,
    pub seite_b: Linie,
    pub linien: Vec<[Punkt; 2]>,
    pub umriss: Linie,
}

/// Dreiecksnetz der Fläche: Punkte, Dreiecke (innen), Randkanten, Ringe als Punktnummern.
struct Netz {
    pkt: Vec<Punkt>,
    drei: Vec<[usize; 3]>,
    /// je Dreieck und Kante k (Ecken k, k+1): Randkante?
    rand: Vec<[bool; 3]>,
    /// Ringe als Folge von Punktnummern
    ringe: Vec<Vec<usize>>,
    /// je Punkt: (Ring, Stelle)
    lage: HashMap<usize, (usize, usize)>,
    /// je Ring: Bogenlänge bis zu jeder Stelle und Gesamtlänge
    bogen: Vec<(Vec<f64>, f64)>,
}

impl Netz {
    /// Nachbarpunkte auf dem Ring (vorher, nachher)
    fn nachbarn(&self, v: usize) -> Option<(usize, usize)> {
        let &(r, i) = self.lage.get(&v)?;
        let ring = &self.ringe[r];
        let n = ring.len();
        Some((ring[(i + n - 1) % n], ring[(i + 1) % n]))
    }

    /// Innenwinkel der Form am Randpunkt v (Grad): Ring 0 ist der Umriss, alle anderen sind Löcher.
    fn innenwinkel(&self, v: usize) -> Option<f64> {
        let (vor, nach) = self.nachbarn(v)?;
        let &(r, _) = self.lage.get(&v)?;
        let ring: Linie = self.ringe[r].iter().map(|&i| self.pkt[i]).collect();
        Some(innenwinkel_an(
            self.pkt[vor],
            self.pkt[v],
            self.pkt[nach],
            flaeche_vorzeichen(&ring) > 0.0,
            r == 0,
        ))
    }

    /// Kürzerer Weg auf dem Rand zwischen zwei Punkten desselben Rings (sonst `None`).
    fn randabstand(&self, u: usize, v: usize) -> Option<f64> {
        let (&(ru, iu), &(rv, iv)) = (self.lage.get(&u)?, self.lage.get(&v)?);
        if ru != rv {
            return None;
        }
        let (kum, l) = &self.bogen[ru];
        let d = (kum[iu] - kum[iv]).abs();
        Some(d.min(l - d))
    }

    /// Weg auf dem Rand von u nach v (beide eingeschlossen), der kürzere der beiden Richtungen; `None`, wenn die
    /// Punkte nicht auf demselben Ring liegen.
    fn randweg(&self, u: usize, v: usize) -> Option<Vec<usize>> {
        let ((la, a), (lb, b)) = self.randwege(u, v)?;
        Some(if la <= lb { a } else { b })
    }

    /// Kappe: Randweg von u nach v, der keinen der Punkte `meiden` enthält (der Säulenkörper liegt auf der anderen
    /// Seite); sind beide Wege frei, der kürzere, bei `zweiter` der längere (Säule aus nur einer Sehne: beide Hälften).
    fn kappenweg(&self, u: usize, v: usize, meiden: &[usize], zweiter: bool) -> Option<Vec<usize>> {
        let ((la, a), (lb, b)) = self.randwege(u, v)?;
        let frei = |w: &Vec<usize>| !w[1..w.len() - 1].iter().any(|x| meiden.contains(x));
        Some(match (frei(&a), frei(&b)) {
            (true, false) => a,
            (false, true) => b,
            _ => {
                if (la <= lb) != zweiter {
                    a
                } else {
                    b
                }
            }
        })
    }

    /// Beide Randwege von u nach v (vorwärts, rückwärts) mit Länge.
    #[allow(clippy::type_complexity)]
    fn randwege(&self, u: usize, v: usize) -> Option<((f64, Vec<usize>), (f64, Vec<usize>))> {
        let (&(ru, iu), &(rv, iv)) = (self.lage.get(&u)?, self.lage.get(&v)?);
        if ru != rv {
            return None;
        }
        let r = &self.ringe[ru];
        let n = r.len();
        let weg = |vor: bool| -> (f64, Vec<usize>) {
            let mut w = vec![u];
            let mut l = 0.0;
            let mut i = iu;
            while i != iv {
                let j = if vor { (i + 1) % n } else { (i + n - 1) % n };
                l += abstand(self.pkt[r[i]], self.pkt[r[j]]);
                w.push(r[j]);
                i = j;
            }
            (l, w)
        };
        Some((weg(true), weg(false)))
    }
}

fn kante(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

/// Ringe ohne Endpunkt, ohne doppelte Punkte, mindestens 3 Punkte.
fn ringe_vorbereiten(umriss: &[Punkt], loecher: &[Linie]) -> Vec<Linie> {
    std::iter::once(umriss)
        .chain(loecher.iter().map(|l| l.as_slice()))
        .map(geschlossen_ohne_endpunkt)
        .filter(|r| r.len() >= 3 && flaeche_vorzeichen(r).abs() > 1e-9)
        .collect()
}

fn triangulieren(ringe: &[Linie]) -> Result<Netz, String> {
    let mut cdt: ConstrainedDelaunayTriangulation<Point2<f64>> = ConstrainedDelaunayTriangulation::new();
    let mut ring_idx: Vec<Vec<usize>> = Vec::new();
    for r in ringe {
        let mut h = Vec::with_capacity(r.len());
        for q in r {
            h.push(
                cdt.insert(Point2::new(q.x, q.y))
                    .map_err(|e| format!("Triangulation: {e:?}"))?,
            );
        }
        for i in 0..h.len() {
            let (a, b) = (h[i], h[(i + 1) % h.len()]);
            if a != b {
                // kreuzt sich der Rand selbst, entfällt die Kante (kein Abbruch)
                let _ = cdt.try_add_constraint(a, b);
            }
        }
        let mut idx: Vec<usize> = h.iter().map(|x| x.index()).collect();
        idx.dedup();
        ring_idx.push(idx);
    }
    let pkt: Vec<Punkt> = cdt.vertices().map(|v| p(v.position().x, v.position().y)).collect();
    // innen/außen: Flutung über die Dreiecke, jede Randkante wechselt die Seite (robust auch für hauchdünne Dreiecke,
    // deren Schwerpunkt auf dem Rand liegt)
    let mut innen: HashMap<usize, bool> = HashMap::new();
    let mut stapel: Vec<(spade::handles::FixedFaceHandle<spade::handles::InnerTag>, bool)> = Vec::new();
    for e in cdt.convex_hull() {
        for x in [e, e.rev()] {
            if let Some(f) = x.face().as_inner() {
                let z = cdt.is_constraint_edge(x.as_undirected().fix());
                stapel.push((f.fix(), z));
            }
        }
    }
    while let Some((f, z)) = stapel.pop() {
        if innen.contains_key(&f.index()) {
            continue;
        }
        innen.insert(f.index(), z);
        for e in cdt.face(f).adjacent_edges() {
            if let Some(g) = e.rev().face().as_inner()
                && !innen.contains_key(&g.fix().index())
            {
                let wechsel = cdt.is_constraint_edge(e.as_undirected().fix());
                stapel.push((g.fix(), z ^ wechsel));
            }
        }
    }
    let mut drei = Vec::new();
    let mut rand = Vec::new();
    for f in cdt.inner_faces() {
        if !innen.get(&f.fix().index()).copied().unwrap_or(false) {
            continue;
        }
        let v = f.vertices();
        let ix = [v[0].fix().index(), v[1].fix().index(), v[2].fix().index()];
        let mut r = [false; 3];
        for e in f.adjacent_edges() {
            let (a, b) = (e.from().fix().index(), e.to().fix().index());
            for k in 0..3 {
                if kante(ix[k], ix[(k + 1) % 3]) == kante(a, b) {
                    r[k] = cdt.is_constraint_edge(e.as_undirected().fix());
                }
            }
        }
        drei.push(ix);
        rand.push(r);
    }
    // hauchdünne Dreiecke zwischen fast geraden Randpunkten entfernen; ihre lange Kante wird Rand
    loop {
        let duenn = (0..drei.len()).find(|&t| {
            let d: [usize; 3] = drei[t];
            let (a, b, c) = (pkt[d[0]], pkt[d[1]], pkt[d[2]]);
            let flaeche = ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)).abs() / 2.0;
            let lang = abstand(a, b).max(abstand(b, c)).max(abstand(c, a));
            rand[t].iter().filter(|x| **x).count() >= 2 && 2.0 * flaeche / lang.max(1e-12) < 1e-6
        });
        let Some(t) = duenn else { break };
        let d = drei[t];
        if let Some(k) = (0..3).find(|&k| !rand[t][k]) {
            let e = kante(d[k], d[(k + 1) % 3]);
            for (u, du) in drei.iter().enumerate() {
                if u == t {
                    continue;
                }
                for kk in 0..3 {
                    if kante(du[kk], du[(kk + 1) % 3]) == e {
                        rand[u][kk] = true;
                    }
                }
            }
        }
        drei.swap_remove(t);
        rand.swap_remove(t);
    }
    let mut lage = HashMap::new();
    for (ri, r) in ring_idx.iter().enumerate() {
        for (i, &v) in r.iter().enumerate() {
            lage.entry(v).or_insert((ri, i));
        }
    }
    let bogen = ring_idx
        .iter()
        .map(|r| {
            let mut l: Linie = r.iter().map(|&i| pkt[i]).collect();
            l.push(l[0]);
            let k = kumuliert(&l);
            let g = *k.last().unwrap();
            (k, g)
        })
        .collect();
    Ok(Netz {
        pkt,
        drei,
        rand,
        ringe: ring_idx,
        lage,
        bogen,
    })
}

/// Rand abtasten: höchstens `max` je Strecke (Eckpunkte bleiben).
fn abtasten(ring: &[Punkt], max: &dyn Fn(usize) -> f64) -> Linie {
    let n = ring.len();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        out.push(a);
        let teile = (abstand(a, b) / max(i)).ceil().max(1.0) as usize;
        for k in 1..teile {
            let t = k as f64 / teile as f64;
            out.push(p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t));
        }
    }
    out
}

/// Örtliche Breite je Netzpunkt: kürzeste **Quersehne** an diesem Punkt – innere Kante, deren Endpunkte auf dem Rand
/// weit auseinander liegen (Randweg > 1,5 × Sehne; kurze Sehnen über einen fast geraden Rand zählen nicht).
fn breiten(n: &Netz) -> Vec<f64> {
    let mut w = vec![f64::MAX; n.pkt.len()];
    for (d, r) in n.drei.iter().zip(&n.rand) {
        for k in 0..3 {
            if r[k] {
                continue;
            }
            let (a, b) = (d[k], d[(k + 1) % 3]);
            let l = abstand(n.pkt[a], n.pkt[b]);
            if n.randabstand(a, b).is_some_and(|x| x < 1.5 * l) {
                continue;
            }
            // Abstand zu den Randstrecken am Gegenpunkt (senkrechte Breite statt schräger Sehne)
            let quer = |x: usize, y: usize| -> f64 {
                n.nachbarn(y).map_or(l, |(vor, nach)| {
                    crate::geometrie::abstand_punkt_strecke(n.pkt[x], n.pkt[vor], n.pkt[y])
                        .min(crate::geometrie::abstand_punkt_strecke(n.pkt[x], n.pkt[y], n.pkt[nach]))
                })
            };
            w[a] = w[a].min(quer(a, b));
            w[b] = w[b].min(quer(b, a));
        }
    }
    w
}

/// Abtasten wie ShapeSample: erst grob, dann je Randstrecke auf Anteil × Breite verfeinern (höchstens 4 Durchgänge).
fn abgetastet(ringe: &[Linie], o: &SaeulenWerte) -> Result<(Vec<Linie>, Netz), String> {
    let mut r: Vec<Linie> = ringe.iter().map(|x| abtasten(x, &|_| o.abtast_grob)).collect();
    for _ in 0..4 {
        let n = triangulieren(&r)?;
        let w = breiten(&n);
        let lage: HashMap<(u64, u64), usize> = n
            .pkt
            .iter()
            .enumerate()
            .map(|(i, q)| ((q.x.to_bits(), q.y.to_bits()), i))
            .collect();
        let breite = |q: Punkt| {
            lage.get(&(q.x.to_bits(), q.y.to_bits()))
                .map(|&i| w[i])
                .unwrap_or(f64::MAX)
        };
        let mut geaendert = false;
        let neu: Vec<Linie> = r
            .iter()
            .map(|ring| {
                let ziel = |i: usize| {
                    let b = breite(ring[i]).min(breite(ring[(i + 1) % ring.len()]));
                    (o.abtast_anteil * b).clamp(o.abtast_fein, o.abtast_grob)
                };
                let x = abtasten(ring, &|i| ziel(i) * 1.5);
                if x.len() > ring.len() {
                    geaendert = true;
                    abtasten(ring, &ziel)
                } else {
                    x
                }
            })
            .collect();
        if !geaendert {
            return Ok((r, n));
        }
        r = neu;
    }
    let n = triangulieren(&r)?;
    Ok((r, n))
}

/// Ende einer Kette.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Ende {
    /// Ende der Form (Enddreieck): Kappe folgt dem Rand
    Rand,
    /// Abzweigung: Sternschnitt zum Schwerpunkt des Dreiecks (Dreiecksnummer)
    Stern(usize),
    /// Ring oder Teilung am Knick: keine Kappe
    Keins,
}

/// Kette von Dreiecken zwischen zwei Enden: die inneren Kanten in Reihenfolge (Stichlinien-Kandidaten).
struct Kette {
    sehnen: Vec<(usize, usize)>,
    anfang: Ende,
    ende: Ende,
    /// Dreiecke der Kette samt Enddreiecken (für das Abschneiden kurzer Zweige)
    dreiecke: Vec<usize>,
}

fn ketten(n: &Netz, weg: &std::collections::HashSet<usize>) -> Vec<Kette> {
    // Kante -> Dreiecke (ohne entfernte)
    let mut an: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (t, d) in n.drei.iter().enumerate() {
        if weg.contains(&t) {
            continue;
        }
        for k in 0..3 {
            if !n.rand[t][k] {
                an.entry(kante(d[k], d[(k + 1) % 3])).or_default().push(t);
            }
        }
    }
    let innen = |t: usize| -> Vec<(usize, usize)> {
        let d = n.drei[t];
        (0..3)
            .map(|k| kante(d[k], d[(k + 1) % 3]))
            .filter(|e| an.get(e).is_some_and(|v| v.len() == 2))
            .collect()
    };
    let grad: Vec<usize> = (0..n.drei.len())
        .map(|t| if weg.contains(&t) { 9 } else { innen(t).len() })
        .collect();
    let ende_von = |t: usize| -> Ende { if grad[t] == 1 { Ende::Rand } else { Ende::Stern(t) } };
    let gegenueber = |t: usize, e: (usize, usize)| an[&e].iter().copied().find(|&x| x != t).unwrap();
    let mut besucht = vec![false; n.drei.len()];
    let mut genommen: std::collections::HashSet<(usize, (usize, usize))> = Default::default();
    let mut out = Vec::new();
    for t0 in 0..n.drei.len() {
        if !matches!(grad[t0], 1 | 3) {
            continue;
        }
        for e0 in innen(t0) {
            if genommen.contains(&(t0, e0)) {
                continue;
            }
            let mut sehnen = vec![e0];
            let (mut t, mut e) = (gegenueber(t0, e0), e0);
            let mut dreiecke = vec![t0];
            while grad[t] == 2 {
                besucht[t] = true;
                dreiecke.push(t);
                let weiter = innen(t).into_iter().find(|&x| x != e).unwrap();
                sehnen.push(weiter);
                t = gegenueber(t, weiter);
                e = weiter;
            }
            genommen.insert((t0, e0));
            genommen.insert((t, e));
            dreiecke.push(t);
            out.push(Kette {
                sehnen,
                anfang: ende_von(t0),
                ende: ende_von(t),
                dreiecke,
            });
        }
    }
    // übrige Säulenstücke bilden Ringe (z. B. „O“): an der schmalsten Kante öffnen
    for t0 in 0..n.drei.len() {
        if grad[t0] != 2 || besucht[t0] {
            continue;
        }
        let e0 = innen(t0)[0];
        let mut sehnen = vec![e0];
        let (mut t, mut e) = (gegenueber(t0, e0), e0);
        besucht[t0] = true;
        while t != t0 {
            besucht[t] = true;
            let weiter = innen(t).into_iter().find(|&x| x != e).unwrap();
            sehnen.push(weiter);
            t = gegenueber(t, weiter);
            e = weiter;
        }
        // sehnen: e0 … (die letzte führt zurück in t0); an der kürzesten öffnen, Anfang = Ende
        let len = |x: &(usize, usize)| abstand(n.pkt[x.0], n.pkt[x.1]);
        let m = sehnen.len();
        let k = (0..m)
            .min_by(|&a, &b| len(&sehnen[a]).total_cmp(&len(&sehnen[b])))
            .unwrap_or(0);
        let mut r: Vec<(usize, usize)> = (0..m).map(|i| sehnen[(k + i) % m]).collect();
        r.push(r[0]);
        out.push(Kette {
            sehnen: r,
            anfang: Ende::Keins,
            ende: Ende::Keins,
            dreiecke: Vec::new(),
        });
    }
    out
}

/// Sehnen gleich ausrichten: (a, b) mit a auf Seite A; aufeinanderfolgende Sehnen teilen einen Punkt.
fn ausrichten(sehnen: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::with_capacity(sehnen.len());
    for &(x, y) in sehnen {
        let s = match out.last() {
            None => (x, y),
            Some(&(a, b)) => {
                if x == a || y == b {
                    (x, y)
                } else if y == a || x == b {
                    (y, x)
                } else {
                    (x, y)
                }
            }
        };
        out.push(s);
    }
    out
}

/// Innenwinkel der Form an b (Grad) zwischen den Nachbarn a und c; `ccw` = Umlaufsinn des Rings, `umriss` = äußerer
/// Ring (bei Löchern liegt die Form außerhalb des Rings).
fn innenwinkel_an(a: Punkt, b: Punkt, c: Punkt, ccw: bool, umriss: bool) -> f64 {
    let (u, v) = (p(a.x - b.x, a.y - b.y), p(c.x - b.x, c.y - b.y));
    let w = (u.x * v.y - u.y * v.x).atan2(u.x * v.x + u.y * v.y).to_degrees();
    // Innenwinkel des Rings: bei Umlauf gegen den Uhrzeiger der Winkel von v nach u
    let ring = if ccw {
        (-w).rem_euclid(360.0)
    } else {
        w.rem_euclid(360.0)
    };
    if umriss { ring } else { 360.0 - ring }
}

/// Randecken der Form (Innenwinkel unter `eckwinkel`) für die Eckerkennung.
fn eckpunkte(ringe: &[Linie], eckwinkel: f64) -> Vec<Punkt> {
    let mut out = Vec::new();
    for (ri, r) in ringe.iter().enumerate() {
        let n = r.len();
        let ccw = flaeche_vorzeichen(r) > 0.0;
        for i in 0..n {
            if innenwinkel_an(r[(i + n - 1) % n], r[i], r[(i + 1) % n], ccw, ri == 0) < eckwinkel {
                out.push(r[i]);
            }
        }
    }
    out
}

/// Douglas-Peucker auf der Mittellinie zwischen den Stellen `von` und `bis` (beide gewählt).
fn auswahl(m: &[Punkt], von: usize, bis: usize, tol: f64, out: &mut Vec<usize>) {
    if bis <= von + 1 {
        return;
    }
    let (mut best, mut bi) = (0.0, von);
    for i in von + 1..bis {
        let d = crate::geometrie::abstand_punkt_strecke(m[i], m[von], m[bis]);
        if d > best {
            (best, bi) = (d, i);
        }
    }
    if best > tol {
        auswahl(m, von, bi, tol, out);
        out.push(bi);
        auswahl(m, bi, bis, tol, out);
    }
}

/// Richtungsänderung der Mittellinie an der gewählten Stelle `sel[j]` (Grad): zu den nächsten gewählten Stellen, die
/// mindestens `fenster` mm entfernt liegen (die vereinfachte Mittellinie folgt geraden Armen bis an den Knick).
fn knick(m: &[Punkt], kum: &[f64], sel: &[usize], j: usize, fenster: f64) -> f64 {
    let i = sel[j];
    let vor = sel[..j]
        .iter()
        .rev()
        .copied()
        .find(|&x| kum[i] - kum[x] >= fenster)
        .unwrap_or(sel[0]);
    let nach = sel[j + 1..]
        .iter()
        .copied()
        .find(|&x| kum[x] - kum[i] >= fenster)
        .unwrap_or(*sel.last().unwrap());
    if vor == i || nach == i {
        return 0.0;
    }
    let (u, v) = (
        p(m[i].x - m[vor].x, m[i].y - m[vor].y),
        p(m[nach].x - m[i].x, m[nach].y - m[i].y),
    );
    (u.x * v.y - u.y * v.x).atan2(u.x * v.x + u.y * v.y).to_degrees().abs()
}

/// Seite zwischen zwei Stellen: die Endpunkte der Sehnen; liegen zwei nicht nebeneinander auf dem Rand (dazwischen
/// ein abgeschnittener Zweig), folgt die Seite dem Rand.
fn seite(n: &Netz, idx: impl Iterator<Item = usize>) -> Linie {
    let mut v: Vec<usize> = Vec::new();
    for i in idx {
        if v.last() == Some(&i) {
            continue;
        }
        if let Some(&u) = v.last()
            && let Some(w) = n.randweg(u, i)
        {
            v.extend(w.into_iter().skip(1));
            continue;
        }
        v.push(i);
    }
    v.into_iter().map(|i| n.pkt[i]).collect()
}

/// Kappe am Ende einer Säule: Randweg `c` von a nach b um das Ende herum. Der Teil, der am weitesten von der Linie a–b
/// entfernt liegt (bis `tol` darunter), wird der letzte Stich – eine Spitze bei spitzem Ende, die Kante bei flachem
/// (**eigene Festlegung**). Liefert die Wege a → Ende und b → Ende.
fn kappe(c: &[Punkt], tol: f64) -> (Linie, Linie) {
    let (a, b) = (c[0], *c.last().unwrap());
    let d: Vec<f64> = c
        .iter()
        .map(|q| crate::geometrie::abstand_punkt_strecke(*q, a, b))
        .collect();
    let dmax = d.iter().copied().fold(0.0, f64::max);
    let im = d.iter().position(|&x| x >= dmax - 1e-12).unwrap_or(0);
    let mut i0 = im;
    while i0 > 0 && d[i0 - 1] >= dmax - tol {
        i0 -= 1;
    }
    let mut i1 = im;
    while i1 + 1 < c.len() && d[i1 + 1] >= dmax - tol {
        i1 += 1;
    }
    let wa: Linie = c[..=i0].to_vec();
    let mut wb: Linie = c[i1..].to_vec();
    wb.reverse();
    (wa, wb)
}

/// Kurze Seitenzweige (Kette von einer Abzweigung zu einem Ende) abschneiden – im Original `TriangleIsOnDefect`/
/// `OnSerif`/`OnCorner` (`MaxDefectSize`, Corner Detection Angle 120°; die genauen Regeln sind nicht bestimmt, hier
/// **eigene Festlegung**): kürzer als die Breite w an der Abzweigung (z. B. die „Ohren“ an einem flachen Ende), oder
/// Ende in einer spitzen Ecke (Innenwinkel θ < Eckwinkel) und kürzer als 1,2 × w / (2·sin(θ/2)) – so lang ist der Zweig,
/// den eine Ecke zweier Striche der Breite w allein erzeugt (z. B. die Spitze unten am „V“).
fn abschneiden(n: &Netz, eckwinkel: f64) -> std::collections::HashSet<usize> {
    let mut weg = std::collections::HashSet::new();
    for _ in 0..64 {
        let mut neu = false;
        for k in ketten(n, &weg) {
            let (stern, rand_am_ende) = match (k.anfang, k.ende) {
                (Ende::Stern(t), Ende::Rand) => (t, true),
                (Ende::Rand, Ende::Stern(t)) => (t, false),
                _ => continue,
            };
            let _ = stern;
            let s = &k.sehnen;
            let breite = |e: (usize, usize)| abstand(n.pkt[e.0], n.pkt[e.1]);
            let mitte = |e: (usize, usize)| p((n.pkt[e.0].x + n.pkt[e.1].x) / 2.0, (n.pkt[e.0].y + n.pkt[e.1].y) / 2.0);
            let (an_stern, am_rand, drei_rand) = if rand_am_ende {
                (s[0], s[s.len() - 1], *k.dreiecke.last().unwrap())
            } else {
                (s[s.len() - 1], s[0], k.dreiecke[0])
            };
            let mut laenge: f64 = s.windows(2).map(|w| abstand(mitte(w[0]), mitte(w[1]))).sum();
            let d = n.drei[drei_rand];
            let spitze = d.iter().copied().find(|&v| v != am_rand.0 && v != am_rand.1).unwrap();
            laenge += abstand(mitte(am_rand), n.pkt[spitze]);
            let w = breite(an_stern);
            let grenze = match n.innenwinkel(spitze) {
                Some(t) if t < eckwinkel => (1.2 * w / (2.0 * (t.to_radians() / 2.0).sin())).max(w),
                _ => w,
            };
            if laenge < grenze {
                // Zweig ohne das Abzweig-Dreieck entfernen
                for &t in &k.dreiecke {
                    if Ende::Stern(t) != k.anfang && Ende::Stern(t) != k.ende {
                        weg.insert(t);
                    }
                }
                neu = true;
                break;
            }
        }
        if !neu {
            break;
        }
    }
    weg
}

/// Zwischen festen Paaren die Seiten nach relativer Bogenlänge teilen (wie `wendung::wende_seiten`): je Randpunkt
/// und mindestens alle 1 mm ein Paar.
fn paare(abschnitte: &[(Linie, Linie)]) -> (Linie, Linie) {
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for (k, (a, b)) in abschnitte.iter().enumerate() {
        let (ka, kb) = (kumuliert(a), kumuliert(b));
        let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
        let teile = ((la.max(lb)).ceil() as usize).max(a.len().max(b.len()) - 1).max(1);
        let letzte = k == abschnitte.len() - 1;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(crate::geometrie::punkt_bei(a, t * la, &ka));
            sb.push(crate::geometrie::punkt_bei(b, t * lb, &kb));
        }
    }
    (sa, sb)
}

/// Fläche in Satinsäulen zerlegen (siehe Modulbeschreibung). Liefert die Säulen in der Reihenfolge der Zerlegung
/// (die Stickreihenfolge legt der Aufrufer fest, z. B. als Gruppe nach Nähe).
pub fn zerlegen(umriss: &[Punkt], loecher: &[Linie], o: &SaeulenWerte) -> Result<Vec<Saeule>, String> {
    let ringe = ringe_vorbereiten(umriss, loecher);
    if ringe.is_empty() {
        return Err("Fläche ohne Umriss".into());
    }
    let (_, netz) = abgetastet(&ringe, o)?;
    let ecken = eckpunkte(&ringe, o.eckwinkel);
    let ist_ecke = |q: Punkt| ecken.iter().any(|e| abstand(*e, q) < 1e-9);
    let schwerpunkt = |t: usize| {
        let d = netz.drei[t];
        p(
            (netz.pkt[d[0]].x + netz.pkt[d[1]].x + netz.pkt[d[2]].x) / 3.0,
            (netz.pkt[d[0]].y + netz.pkt[d[1]].y + netz.pkt[d[2]].y) / 3.0,
        )
    };
    let weg = abschneiden(&netz, o.eckwinkel);
    let mut out = Vec::new();
    for k in ketten(&netz, &weg) {
        let s = ausrichten(&k.sehnen);
        let pa = |i: usize| netz.pkt[s[i].0];
        let pb = |i: usize| netz.pkt[s[i].1];
        let m: Linie = (0..s.len())
            .map(|i| p((pa(i).x + pb(i).x) / 2.0, (pa(i).y + pb(i).y) / 2.0))
            .collect();
        let breite: Vec<f64> = (0..s.len()).map(|i| abstand(pa(i), pb(i))).collect();
        let mut sortiert = breite.clone();
        sortiert.sort_by(f64::total_cmp);
        let b_med = sortiert[sortiert.len() / 2].max(o.abtast_fein);
        // an einem Ende der Form beginnt die Säule erst bei der ersten Sehne mit voller Breite (≥ 0,75 × Breite); die
        // kurzen Sehnen davor liegen in der Endzone und gehören zur Kappe (**eigene Festlegung**)
        let kum_m = kumuliert(&m);
        let voll = |i: usize| {
            if breite[i] < 0.75 * b_med {
                return false;
            }
            // quer zur Mittellinie (über zwei Breiten): mindestens 70°
            let vor = (i..m.len())
                .find(|&j| kum_m[j] - kum_m[i] >= 2.0 * b_med)
                .unwrap_or(m.len() - 1);
            let rueck = (0..=i).rev().find(|&j| kum_m[i] - kum_m[j] >= 2.0 * b_med).unwrap_or(0);
            let (u, v) = (m[vor], m[rueck]);
            let achse = p(u.x - v.x, u.y - v.y);
            let sehne = p(pb(i).x - pa(i).x, pb(i).y - pa(i).y);
            let n = (achse.x.hypot(achse.y) * sehne.x.hypot(sehne.y)).max(1e-12);
            ((achse.x * sehne.x + achse.y * sehne.y).abs() / n) <= 70f64.to_radians().cos()
        };
        let mut von = if k.anfang == Ende::Rand {
            (0..s.len()).find(|&i| voll(i)).unwrap_or(0)
        } else {
            0
        };
        let mut bis = if k.ende == Ende::Rand {
            (0..s.len()).rev().find(|&i| voll(i)).unwrap_or(s.len() - 1)
        } else {
            s.len() - 1
        };
        if von > bis {
            let l = (0..s.len())
                .max_by(|&x, &y| breite[x].total_cmp(&breite[y]))
                .unwrap_or(0);
            (von, bis) = (l, l);
        }
        // feste Stellen: Anfang, Ende, Randecken (bei einem Fächer um eine Ecke die mittlere Sehne)
        let mut fest = vec![von, bis];
        let mut i = von;
        while i <= bis {
            let ecke = [s[i].0, s[i].1].into_iter().find(|&v| ist_ecke(netz.pkt[v]));
            if let Some(v) = ecke {
                let mut j = i;
                while j < bis && (s[j + 1].0 == v || s[j + 1].1 == v) {
                    j += 1;
                }
                fest.push((i + j) / 2);
                i = j + 1;
            } else {
                i += 1;
            }
        }
        fest.sort_unstable();
        fest.dedup();
        let mut gewaehlt = Vec::new();
        for w in fest.windows(2) {
            gewaehlt.push(w[0]);
            auswahl(&m, w[0], w[1], o.linien_toleranz * b_med, &mut gewaehlt);
        }
        gewaehlt.push(*fest.last().unwrap());
        gewaehlt.dedup();
        if gewaehlt.is_empty() {
            continue;
        }
        // Knicke: Säule dort teilen (Innenwinkel unter dem Knickwinkel)
        let kum = kumuliert(&m);
        let fenster = b_med;
        let mut teile: Vec<Vec<usize>> = vec![Vec::new()];
        let mut letzte_teilung = f64::NEG_INFINITY;
        for (j, &g) in gewaehlt.iter().enumerate() {
            teile.last_mut().unwrap().push(g);
            let innen = j > 0 && j + 1 < gewaehlt.len() && kum[g] - letzte_teilung >= fenster;
            if innen && 180.0 - knick(&m, &kum, &gewaehlt, j, fenster) < o.knickwinkel {
                teile.push(vec![g]);
                letzte_teilung = kum[g];
            }
        }
        let n_teile = teile.len();
        for (ti, sel) in teile.into_iter().enumerate() {
            let anfang = if ti == 0 { k.anfang } else { Ende::Keins };
            let ende = if ti + 1 == n_teile { k.ende } else { Ende::Keins };
            let tol = 0.2 * b_med;
            let mut abschnitte: Vec<(Linie, Linie)> = Vec::new();
            let f = sel[0];
            match anfang {
                Ende::Stern(t) => {
                    let q = schwerpunkt(t);
                    abschnitte.push((vec![q, pa(f)], vec![q, pb(f)]));
                }
                Ende::Rand => {
                    let koerper: Vec<usize> = (f + 1..=*sel.last().unwrap()).flat_map(|i| [s[i].0, s[i].1]).collect();
                    if let Some(c) = netz.kappenweg(s[f].0, s[f].1, &koerper, false) {
                        let c: Linie = c.into_iter().map(|i| netz.pkt[i]).collect();
                        let (mut wa, mut wb) = kappe(&c, tol);
                        wa.reverse();
                        wb.reverse();
                        abschnitte.push((wa, wb));
                    }
                }
                Ende::Keins => {}
            }
            for w in sel.windows(2) {
                let a = seite(&netz, (w[0]..=w[1]).map(|i| s[i].0));
                let b = seite(&netz, (w[0]..=w[1]).map(|i| s[i].1));
                abschnitte.push((a, b));
            }
            let l = *sel.last().unwrap();
            match ende {
                Ende::Stern(t) => {
                    let q = schwerpunkt(t);
                    abschnitte.push((vec![pa(l), q], vec![pb(l), q]));
                }
                Ende::Rand => {
                    let koerper: Vec<usize> = (sel[0]..l).flat_map(|i| [s[i].0, s[i].1]).collect();
                    let zweiter = sel.len() == 1 && anfang == Ende::Rand;
                    if let Some(c) = netz.kappenweg(s[l].0, s[l].1, &koerper, zweiter) {
                        let c: Linie = c.into_iter().map(|i| netz.pkt[i]).collect();
                        abschnitte.push(kappe(&c, tol));
                    }
                }
                Ende::Keins => {}
            }
            if abschnitte.is_empty() {
                continue;
            }
            let (seite_a, seite_b) = paare(&abschnitte);
            let mut umriss = seite_a.clone();
            umriss.extend(seite_b.iter().rev());
            out.push(Saeule {
                linien: sel.iter().map(|&i| [pa(i), pb(i)]).collect(),
                seite_a,
                seite_b,
                umriss,
            });
        }
    }
    // Splitter weglassen
    let ganz: f64 = ringe
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if i == 0 {
                flaeche_vorzeichen(r).abs()
            } else {
                -flaeche_vorzeichen(r).abs()
            }
        })
        .sum();
    out.retain(|x| {
        let f = flaeche_vorzeichen(&x.umriss).abs();
        f >= SPLITTER_MM2 || f >= SPLITTER_ANTEIL * ganz
    });
    Ok(out)
}

/// Größte Breite der Fläche (mm): längste Sehne zwischen zwei Randseiten im Inneren der Säulenstücke – für die Wahl
/// der Stichart wie EasyStitch (Fläche ≤ 8 mm Satin, `Analyse/code/10` §C).
pub fn groesste_breite(umriss: &[Punkt], loecher: &[Linie]) -> Result<f64, String> {
    let ringe = ringe_vorbereiten(umriss, loecher);
    if ringe.is_empty() {
        return Ok(0.0);
    }
    let n = triangulieren(&ringe.iter().map(|r| abtasten(r, &|_| 1.0)).collect::<Vec<_>>())?;
    Ok(breiten(&n).into_iter().filter(|w| *w < f64::MAX).fold(0.0, f64::max))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rechteck(x0: f64, y0: f64, x1: f64, y1: f64) -> Linie {
        vec![p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)]
    }

    /// gerader Balken 30 × 3 mm: eine Säule, Stiche quer (Stichlinien senkrecht zur Länge)
    #[test]
    fn balken_eine_saeule_quer() {
        let s = zerlegen(&rechteck(0.0, 0.0, 30.0, 3.0), &[], &SaeulenWerte::default()).unwrap();
        assert_eq!(s.len(), 1, "{s:#?}");
        let sa = &s[0];
        assert_eq!(sa.seite_a.len(), sa.seite_b.len());
        // in der Mitte: Paare senkrecht (x gleich), 3 mm auseinander
        let mitte: Vec<_> = sa
            .seite_a
            .iter()
            .zip(&sa.seite_b)
            .filter(|(a, _)| (a.x - 15.0).abs() < 5.0)
            .collect();
        assert!(!mitte.is_empty());
        for (a, b) in mitte {
            assert!((a.x - b.x).abs() < 0.3, "{a:?} {b:?}");
            assert!((abstand(*a, *b) - 3.0).abs() < 0.1);
        }
    }

    /// „Y“ aus drei Balken: drei Säulen (Sternschnitt an der Abzweigung)
    #[test]
    fn abzweigung_drei_saeulen() {
        let y = vec![
            p(-1.5, -20.0),
            p(1.5, -20.0),
            p(1.5, 0.0),
            p(13.0, 12.0),
            p(11.0, 14.0),
            p(0.0, 3.0),
            p(-11.0, 14.0),
            p(-13.0, 12.0),
            p(-1.5, 0.0),
        ];
        let s = zerlegen(&y, &[], &SaeulenWerte::default()).unwrap();
        assert_eq!(s.len(), 3, "{:?}", s.iter().map(|x| x.linien.len()).collect::<Vec<_>>());
    }

    /// Ring (Kreis mit Loch): eine geschlossene Säule, Stiche radial
    #[test]
    fn ring_eine_saeule() {
        let kreis = |r: f64| crate::geometrie::kreis(p(0.0, 0.0), r, 0.02, 0.0);
        let mut loch = kreis(8.0);
        loch.reverse();
        let s = zerlegen(&kreis(10.0), &[loch], &SaeulenWerte::default()).unwrap();
        assert_eq!(s.len(), 1);
        for (a, b) in s[0].seite_a.iter().zip(&s[0].seite_b) {
            // radial: beide Punkte auf derselben Geraden durch die Mitte
            let w = (a.x * b.y - a.y * b.x).abs() / (abstand(*a, p(0.0, 0.0)) * abstand(*b, p(0.0, 0.0)));
            assert!(w < 0.15, "nicht radial: {a:?} {b:?}");
        }
    }

    /// „L“: Knick 90° liegt über 87° – keine Teilung; „V“ mit 40°: zwei Säulen bei der Turning-Umwandlung
    #[test]
    fn knick_teilt_spitze_winkel() {
        let l = vec![
            p(0.0, 0.0),
            p(20.0, 0.0),
            p(20.0, 3.0),
            p(3.0, 3.0),
            p(3.0, 20.0),
            p(0.0, 20.0),
        ];
        let s = zerlegen(&l, &[], &SaeulenWerte::default()).unwrap();
        assert_eq!(s.len(), 1);
        // V: zwei Balken (3 mm breit) unter 40° zueinander, unten in einer Spitze verbunden
        let w = 20f64.to_radians();
        let (c, sn) = (w.cos(), w.sin());
        let v = vec![
            p(0.0, 0.0),
            p(20.0 * sn, 20.0 * c),
            p(20.0 * sn - 3.0 * c, 20.0 * c + 3.0 * sn),
            p(0.0, 3.0 / sn),
            p(-20.0 * sn + 3.0 * c, 20.0 * c + 3.0 * sn),
            p(-20.0 * sn, 20.0 * c),
        ];
        let turning = SaeulenWerte {
            knickwinkel: KNICKWINKEL_TURNING,
            ..SaeulenWerte::default()
        };
        let s = zerlegen(&v, &[], &turning).unwrap();
        assert_eq!(s.len(), 2, "{:?}", s.iter().map(|x| x.linien.len()).collect::<Vec<_>>());
        // wie der Auto Digitizer: eine Säule durch die Kehre
        assert_eq!(zerlegen(&v, &[], &SaeulenWerte::default()).unwrap().len(), 1);
    }

    #[test]
    fn breite_eines_balkens() {
        let b = groesste_breite(&rechteck(0.0, 0.0, 30.0, 3.0), &[]).unwrap();
        assert!((b - 3.0).abs() < 0.05, "{b}");
    }
}
