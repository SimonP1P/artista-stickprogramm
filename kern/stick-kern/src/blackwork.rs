//! Blackwork Run (Handbuch S. 131–132, Craft-Stitch-Werkzeugleiste): mehrere sich überlappende Linien werden zu
//! **einem** automatisch geordneten Objekt mit einem Ein- und einem Ausstich – „without having to think about the most
//! efficient stitching sequence and joins“; „objects resequenced, connectors minimized“. Alle Teile bekommen die
//! Stichart, Farbe und Effekte des ersten Objekts; die Teile bleiben einzeln formbar.
//!
//! Verfahren (eigene Festlegung – der Code des Originals ist nicht gelesen; `Analyse_Teil3/24_craft_stitches.md` §5):
//! 1. Die Linien werden an allen Schnitt- und Berührpunkten geteilt (Enden, die bis 0,5 mm an eine andere Linie
//!    heranreichen, werden auf sie gezogen) → Netz aus Knoten und Kanten.
//! 2. Je zusammenhängendem Netz ein Weg über alle Kanten (Euler-Weg). Anfang = Knoten nächst dem vorigen Stich bzw.
//!    Einstich (bei Knoten mit ungerader Kantenzahl der nächste davon). Die übrigen ungeraden Knoten werden paarweise
//!    über den kürzesten Weg im Netz verbunden (diese Kanten werden doppelt gestickt – wie beim Doppellauf der
//!    Blackwork-Handarbeit); als Ende bleibt der Knoten offen, bei dem die Paare zusammen am kürzesten sind.
//! 3. Der Weg beginnt am Ende nächst dem vorigen Stich bzw. Einstich; getrennte Netze folgen nach Nähe.
//!
//! An Knoten (Schnittpunkten) wird eingestochen; sonst gelten die Eckpunkte der Linien.

use crate::geometrie::{Linie, Punkt, abstand, p};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Enden, die so nah an eine andere Linie reichen, werden auf sie gezogen (mm)
pub const FANG: f64 = 0.5;
/// Punkte, die so nah beieinander liegen, sind ein Knoten (mm)
const GLEICH: f64 = 0.05;

struct Kante {
    a: usize,
    b: usize,
    /// Linie von a nach b mit Eckmarken
    linie: Linie,
    ecken: Vec<bool>,
    laenge: f64,
}

/// Schnitt zweier Strecken: Parameter (t, u) auf beiden, wenn sie sich schneiden oder berühren.
fn schnitt(a: Punkt, b: Punkt, c: Punkt, d: Punkt) -> Option<(f64, f64)> {
    let (r, s) = (p(b.x - a.x, b.y - a.y), p(d.x - c.x, d.y - c.y));
    let nenner = r.x * s.y - r.y * s.x;
    if nenner.abs() < 1e-12 {
        return None;
    }
    let q = p(c.x - a.x, c.y - a.y);
    let t = (q.x * s.y - q.y * s.x) / nenner;
    let u = (q.x * r.y - q.y * r.x) / nenner;
    let eps = 1e-7;
    ((-eps..=1.0 + eps).contains(&t) && (-eps..=1.0 + eps).contains(&u)).then_some((t.clamp(0.0, 1.0), u.clamp(0.0, 1.0)))
}

/// Nächster Punkt auf der Strecke a–b und sein Parameter.
fn fusspunkt(q: Punkt, a: Punkt, b: Punkt) -> (Punkt, f64) {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let l2 = dx * dx + dy * dy;
    if l2 < 1e-18 {
        return (a, 0.0);
    }
    let t = (((q.x - a.x) * dx + (q.y - a.y) * dy) / l2).clamp(0.0, 1.0);
    (p(a.x + dx * t, a.y + dy * t), t)
}

#[derive(PartialEq)]
struct Offen(f64, usize);
impl Eq for Offen {}
impl Ord for Offen {
    fn cmp(&self, o: &Self) -> Ordering {
        o.0.total_cmp(&self.0).then(o.1.cmp(&self.1))
    }
}
impl PartialOrd for Offen {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

/// Kürzeste Wege von `von` (Dijkstra): Abstand und Kante zum Vorgänger je Knoten.
fn dijkstra(von: usize, kanten: &[Kante], adj: &[Vec<usize>]) -> (Vec<f64>, Vec<Option<usize>>) {
    let n = adj.len();
    let (mut dist, mut vor) = (vec![f64::INFINITY; n], vec![None; n]);
    dist[von] = 0.0;
    let mut heap = BinaryHeap::from([Offen(0.0, von)]);
    while let Some(Offen(d, v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &k in &adj[v] {
            let w = if kanten[k].a == v { kanten[k].b } else { kanten[k].a };
            let nd = d + kanten[k].laenge;
            if nd < dist[w] - 1e-12 {
                dist[w] = nd;
                vor[w] = Some(k);
                heap.push(Offen(nd, w));
            }
        }
    }
    (dist, vor)
}

/// Euler-Weg ab `start` (Hierholzer): Folge von (Kante, vorwärts).
fn euler(start: usize, kanten: &[Kante], adj: &[Vec<usize>]) -> Vec<(usize, bool)> {
    let mut benutzt = vec![false; kanten.len()];
    let mut zeiger = vec![0; adj.len()];
    let mut stapel: Vec<(usize, Option<(usize, bool)>)> = vec![(start, None)];
    let mut weg = Vec::new();
    while let Some(&(v, _)) = stapel.last() {
        let mut naechste = None;
        while zeiger[v] < adj[v].len() {
            let k = adj[v][zeiger[v]];
            zeiger[v] += 1;
            if !benutzt[k] {
                naechste = Some(k);
                break;
            }
        }
        match naechste {
            Some(k) => {
                benutzt[k] = true;
                let vorwaerts = kanten[k].a == v;
                let w = if vorwaerts { kanten[k].b } else { kanten[k].a };
                stapel.push((w, Some((k, vorwaerts))));
            }
            None => {
                let (_, ueber) = stapel.pop().unwrap();
                if let Some(x) = ueber {
                    weg.push(x);
                }
            }
        }
    }
    weg.reverse();
    weg
}

/// Paare ungerader Knoten gierig nach kürzestem Weg; Summe der Wege.
fn paaren(knoten: &[usize], dist: &dyn Fn(usize, usize) -> f64) -> (f64, Vec<(usize, usize)>) {
    let mut paare: Vec<(f64, usize, usize)> = Vec::new();
    for a in 0..knoten.len() {
        for b in a + 1..knoten.len() {
            paare.push((dist(knoten[a], knoten[b]), knoten[a], knoten[b]));
        }
    }
    paare.sort_by(|x, y| x.0.total_cmp(&y.0));
    let mut frei: std::collections::BTreeSet<usize> = knoten.iter().copied().collect();
    let (mut summe, mut out) = (0.0, Vec::new());
    for (d, a, b) in paare {
        if frei.contains(&a) && frei.contains(&b) {
            frei.remove(&a);
            frei.remove(&b);
            summe += d;
            out.push((a, b));
        }
    }
    (summe, out)
}

/// Anfang eines Netzes wählen und die Kanten doppeln, die für einen Weg über alle Kanten nötig sind: Anfang = Knoten
/// nächst `von` (bei ungeraden Knoten: der nächste ungerade); das Ende ist der ungerade Knoten, bei dem die übrigen
/// Paare zusammen den kürzesten Weg haben.
fn anfang_und_doppeln(
    netz: &[usize],
    von: Punkt,
    knoten: &[Punkt],
    kanten: &mut Vec<Kante>,
    adj: &mut [Vec<usize>],
) -> usize {
    let naechst = |l: &[usize]| -> usize {
        *l.iter()
            .min_by(|&&a, &&b| abstand(knoten[a], von).total_cmp(&abstand(knoten[b], von)))
            .unwrap()
    };
    let ungerade: Vec<usize> = netz.iter().copied().filter(|&v| adj[v].len() % 2 == 1).collect();
    if ungerade.len() < 2 {
        return naechst(netz);
    }
    let s0 = naechst(&ungerade);
    let wege: std::collections::BTreeMap<usize, (Vec<f64>, Vec<Option<usize>>)> =
        ungerade.iter().map(|&v| (v, dijkstra(v, kanten, adj))).collect();
    let dist = |a: usize, b: usize| wege[&a].0[b];
    let mut best: Option<(f64, Vec<(usize, usize)>)> = None;
    for &e in &ungerade {
        if e == s0 {
            continue;
        }
        let rest: Vec<usize> = ungerade.iter().copied().filter(|&v| v != s0 && v != e).collect();
        let (summe, paare) = paaren(&rest, &dist);
        if best.as_ref().is_none_or(|(b, _)| summe < *b - 1e-9) {
            best = Some((summe, paare));
        }
    }
    for (a, b) in best.map(|x| x.1).unwrap_or_default() {
        let mut v = b;
        while v != a {
            let Some(k) = wege[&a].1[v] else { break };
            let x = &kanten[k];
            let kopie = Kante {
                a: x.a,
                b: x.b,
                linie: x.linie.clone(),
                ecken: x.ecken.clone(),
                laenge: x.laenge,
            };
            v = if x.a == v { x.b } else { x.a };
            kanten.push(kopie);
            let neu = kanten.len() - 1;
            adj[kanten[neu].a].push(neu);
            adj[kanten[neu].b].push(neu);
        }
    }
    s0
}

/// Linien (mit Eckmarken) zu möglichst wenigen durchgehenden Zügen verbinden. `start` = Punkt, an dem begonnen werden
/// soll (vorige Nadelposition oder Einstich). Ergebnis: je Netz ein Zug mit Eckmarken.
pub fn blackwork_wege(linien: &[(Linie, Vec<bool>)], start: Option<Punkt>) -> Vec<(Linie, Vec<bool>)> {
    // 1. Linien bereinigen
    let mut ls: Vec<(Linie, Vec<bool>)> = Vec::new();
    for (l, e) in linien {
        let mut q: Linie = Vec::new();
        let mut f: Vec<bool> = Vec::new();
        for (i, &x) in l.iter().enumerate() {
            let ecke = e.get(i).copied().unwrap_or(true);
            if q.last().is_none_or(|&y| abstand(x, y) > 1e-9) {
                q.push(x);
                f.push(ecke);
            }
        }
        if q.len() >= 2 {
            ls.push((q, f));
        }
    }
    if ls.is_empty() {
        return Vec::new();
    }
    // 2. Enden auf nahe Linien ziehen
    for i in 0..ls.len() {
        for ende in [0, ls[i].0.len() - 1] {
            let q = ls[i].0[ende];
            let mut best: Option<(f64, Punkt)> = None;
            for (j, (l, _)) in ls.iter().enumerate() {
                for k in 0..l.len() - 1 {
                    // eigene Nachbarstrecken des Endes nicht
                    if j == i && (k == ende || k + 1 == ende || (ende > 0 && k + 2 == ende) || (ende == 0 && k == 1)) {
                        continue;
                    }
                    let (f, _) = fusspunkt(q, l[k], l[k + 1]);
                    let d = abstand(q, f);
                    if d <= FANG && best.is_none_or(|(bd, _)| d < bd) {
                        best = Some((d, f));
                    }
                }
            }
            if let Some((d, f)) = best
                && d > 1e-9
            {
                ls[i].0[ende] = f;
            }
        }
    }
    // 3. Teilpunkte je Linie: (Strecke, t, Punkt)
    let mut teil: Vec<Vec<(usize, f64, Punkt)>> = ls
        .iter()
        .map(|(l, _)| vec![(0, 0.0, l[0]), (l.len() - 2, 1.0, l[l.len() - 1])])
        .collect();
    let strecken: Vec<(usize, usize)> = ls
        .iter()
        .enumerate()
        .flat_map(|(i, (l, _))| (0..l.len() - 1).map(move |k| (i, k)))
        .collect();
    for x in 0..strecken.len() {
        for y in x + 1..strecken.len() {
            let ((i, k), (j, m)) = (strecken[x], strecken[y]);
            if i == j && m == k + 1 {
                continue; // Nachbarstrecken derselben Linie
            }
            let (a, b) = (ls[i].0[k], ls[i].0[k + 1]);
            let (c, d) = (ls[j].0[m], ls[j].0[m + 1]);
            if let Some((t, u)) = schnitt(a, b, c, d) {
                let q = p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t);
                teil[i].push((k, t, q));
                teil[j].push((m, u, q));
            }
        }
    }
    // 4. Knoten und Kanten
    let mut knoten: Vec<Punkt> = Vec::new();
    let knoten_id = |q: Punkt, knoten: &mut Vec<Punkt>| -> usize {
        if let Some(i) = knoten.iter().position(|&x| abstand(x, q) < GLEICH) {
            i
        } else {
            knoten.push(q);
            knoten.len() - 1
        }
    };
    let mut kanten: Vec<Kante> = Vec::new();
    for (i, (l, e)) in ls.iter().enumerate() {
        let mut tp = teil[i].clone();
        tp.sort_by(|x, y| (x.0 as f64 + x.1).total_cmp(&(y.0 as f64 + y.1)));
        let mut vor: Option<(usize, f64, Punkt, usize)> = None;
        for &(k, t, q) in &tp {
            let id = knoten_id(q, &mut knoten);
            if let Some((k0, t0, q0, id0)) = vor {
                // Linie zwischen den Teilpunkten mit den Eckpunkten dazwischen
                let mut linie = vec![q0];
                let mut ecken = vec![true];
                let von = if t0 >= 1.0 - 1e-9 { k0 + 2 } else { k0 + 1 };
                let bis = if t <= 1e-9 { k } else { k + 1 };
                for v in von..bis {
                    if v < l.len() {
                        linie.push(l[v]);
                        ecken.push(e[v]);
                    }
                }
                linie.push(q);
                ecken.push(true);
                let laenge = crate::geometrie::laenge(&linie);
                if laenge > 1e-6 {
                    kanten.push(Kante {
                        a: id0,
                        b: id,
                        linie,
                        ecken,
                        laenge,
                    });
                }
            }
            vor = Some((k, t, q, id));
        }
    }
    let n = knoten.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (k, x) in kanten.iter().enumerate() {
        adj[x.a].push(k);
        adj[x.b].push(k);
    }
    // 5. Netze (zusammenhängende Knoten)
    let mut netz = vec![usize::MAX; n];
    let mut netze: Vec<Vec<usize>> = Vec::new();
    for s in 0..n {
        if netz[s] != usize::MAX || adj[s].is_empty() {
            continue;
        }
        let id = netze.len();
        let mut liste = vec![s];
        netz[s] = id;
        let mut i = 0;
        while i < liste.len() {
            let v = liste[i];
            for &k in &adj[v] {
                let w = if kanten[k].a == v { kanten[k].b } else { kanten[k].a };
                if netz[w] == usize::MAX {
                    netz[w] = id;
                    liste.push(w);
                }
            }
            i += 1;
        }
        netze.push(liste);
    }
    // 6. Netze nach Nähe; je Netz Anfang wählen, ungerade Knoten paarweise verbinden (Kanten doppeln), Euler-Weg
    let mut out = Vec::new();
    let mut von = start.unwrap_or(ls[0].0[0]);
    let mut offen: Vec<usize> = (0..netze.len()).collect();
    while !offen.is_empty() {
        let naechster = |z: usize| -> usize {
            *netze[z]
                .iter()
                .min_by(|&&a, &&b| abstand(knoten[a], von).total_cmp(&abstand(knoten[b], von)))
                .unwrap()
        };
        let (pos, z) = offen
            .iter()
            .enumerate()
            .map(|(pos, &z)| (pos, z))
            .min_by(|x, y| abstand(knoten[naechster(x.1)], von).total_cmp(&abstand(knoten[naechster(y.1)], von)))
            .unwrap();
        offen.remove(pos);
        let s = anfang_und_doppeln(&netze[z], von, &knoten, &mut kanten, &mut adj);
        let mut linie: Linie = vec![knoten[s]];
        let mut ecken: Vec<bool> = vec![true];
        for (k, vorwaerts) in euler(s, &kanten, &adj) {
            let x = &kanten[k];
            let (l, e): (Vec<Punkt>, Vec<bool>) = if vorwaerts {
                (x.linie.clone(), x.ecken.clone())
            } else {
                (x.linie.iter().rev().copied().collect(), x.ecken.iter().rev().copied().collect())
            };
            linie.extend(&l[1..]);
            ecken.extend(&e[1..]);
        }
        von = *linie.last().unwrap();
        if linie.len() >= 2 {
            out.push((linie, ecken));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(pts: &[[f64; 2]]) -> (Linie, Vec<bool>) {
        (pts.iter().map(|&q| q.into()).collect(), vec![true; pts.len()])
    }

    #[test]
    fn kreuz_ein_zug_mit_doppelung() {
        // zwei sich kreuzende Linien: 4 Enden ungerade → ein Paar wird verbunden, ein Zug
        let w = blackwork_wege(&[l(&[[-10.0, 0.0], [10.0, 0.0]]), l(&[[0.0, -10.0], [0.0, 10.0]])], None);
        assert_eq!(w.len(), 1);
        let gesamt = crate::geometrie::laenge(&w[0].0);
        // 40 mm Linien + zwei Arme (je 10 mm) doppelt – kürzer geht ein Kreuz nicht in einem Zug
        assert!((gesamt - 60.0).abs() < 1e-6, "Länge {gesamt}");
        // Anfang an einem Ende nächst dem Start der ersten Linie
        assert!(abstand(w[0].0[0], p(-10.0, 0.0)) < 1e-9);
    }

    #[test]
    fn dreieck_ohne_doppelung_und_getrennte_netze() {
        let dreieck = [
            l(&[[0.0, 0.0], [10.0, 0.0]]),
            l(&[[10.0, 0.0], [5.0, 8.0]]),
            l(&[[5.0, 8.0], [0.0, 0.0]]),
            l(&[[30.0, 0.0], [40.0, 0.0]]),
        ];
        let w = blackwork_wege(&dreieck, Some(p(-1.0, 0.0)));
        assert_eq!(w.len(), 2);
        let l0 = crate::geometrie::laenge(&w[0].0);
        assert!((l0 - (10.0 + 2.0 * 89f64.sqrt())).abs() < 1e-6, "Dreieck {l0}");
        assert!(abstand(w[0].0[0], p(0.0, 0.0)) < 1e-9);
        assert!(abstand(*w[0].0.last().unwrap(), p(0.0, 0.0)) < 1e-9);
    }

    #[test]
    fn nahes_ende_wird_angezogen() {
        // T-Stoß mit 0,3 mm Lücke: wird verbunden, ein Zug
        let w = blackwork_wege(&[l(&[[-10.0, 0.0], [10.0, 0.0]]), l(&[[0.0, 0.3], [0.0, 10.0]])], None);
        assert_eq!(w.len(), 1);
    }
}
