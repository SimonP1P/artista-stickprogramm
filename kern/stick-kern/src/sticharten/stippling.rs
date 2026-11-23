//! Stippling (Stipple Fill, Craft-Stitch-Werkzeugleiste): eine Fläche mit einer mäandernden Laufstichlinie füllen,
//! die sich nicht kreuzt. Einstellungen wie im Dialog „Stipple“ (BERESENU DLG 1450): **Stichlänge** und
//! **Schlaufenabstand** (Loop Spacing).
//!
//! **Nachbau des Originals** (`Analyse_Teil3/24_craft_stitches.md` §4): `StippleFillInterface::
//! CreateStippleFillPolygons` (MOD1 0x101a4590) mit dem Erzeuger 0x101a0100/0x101a2a70:
//! 1. **Wabengitter** (0x101a0800): Punkte im Abstand Schlaufenabstand s, entlang y im Raster s/2 (Anzahl ≡ 2 mod 3),
//!    entlang x im Raster s·√3/2 (Anzahl ungerade), genau über die Begrenzung gestreckt. Behalten wird (Spalte c,
//!    Zeile r), wenn c gerade und r mod 6 ∈ {0, 4} oder c ungerade und r mod 6 ∈ {1, 3}, und der Punkt in der Fläche
//!    liegt (0x101a04e0).
//! 2. **Kanten** zwischen Punkten mit Abstand² < 1,1 · s², wenn die Strecke in der Fläche bleibt (0x101a05a0,
//!    rekursive Halbierung mit 18 Einheiten = 0,1 mm Spiel).
//! 3. **Zufälliger Spannbaum**: Kantengewicht 0,9 … 1,0 aus `rand()` (Funktor 0x101a10c0), kleinster Spannbaum.
//! 4. **Rundgang** je Teil ab dem Punkt, der dem Einstich am nächsten liegt (0x101a1a40): Kinder im Uhrzeigersinn ab
//!    der Richtung zum Elternpunkt (an der Wurzel ab +x), Rückweg über jeden Punkt; doppelte Punkte fallen weg
//!    (0x101a1e20).
//! 5. **Schlaufenpunkte** (0x101a1f40) mit A = 0,4·(Pᵢ − Pᵢ₋₁), B = 0,4·(Pᵢ₊₁ − Pᵢ): Linkskurve → P + B − A,
//!    Rechtskurve → P − B − A, P − B + A, P + A, Sackgasse → halbes Sechseck P ± A/2 + N, P + A, P ± A/2 − N mit
//!    N = √3/2 · A um 90° gedreht. Die Linie liegt links vom Baum und umläuft ihn im Uhrzeigersinn.
//! 6. **Zufall** (0x101a2340): jeder Punkt um (r₁·a + r₂·b) um 90° gedreht verschoben, r₁, r₂ gleichverteilt in
//!    ±0,125 (a, b = Nachbarstrecken).
//! 7. **Glätten** (0x101a2570): je Punkt eine Bézierkurve von Streckenmitte zu Streckenmitte mit Stützpunkten bei 0,8
//!    und 0,2; an den Nahtstellen werden die Tangenten gemittelt. Als Linienzug mit 0,1 mm Toleranz.
//! 8. **Verbinden** (0x101a3250): Schleifen, die sich näher als 1,125 · s kommen, werden an der engsten Stelle mit
//!    einer Lücke von 0,125 · s zu einer Schleife verbunden.
//!
//! Werte im Dokument (`DesignDocument::GetCurrentStippleFillValues` +0x138, 6 Zahlen): Stichlänge 360 = 2,0 mm,
//! Schlaufenabstand 540 = 3,0 mm, Randabstand 0, Zufall 0,125, Kurve 1, Randgewicht 0.
//!
//! **Eigene Festlegungen:** Randabstand 0,5 · s statt 0 (sonst ragen die Schlaufen bis 0,4 · s über den Rand; wie
//! das Original den Wert setzt, ist nicht belegt – die Stippling-Funktionen werden in den Programmdateien nicht
//! aufgerufen). Zufallszahlen wie `rand()` von MSVC mit Startwert 1 je Objekt (bei jeder Berechnung gleich). Weit
//! getrennte Schleifen werden wie bisher nacheinander gestickt (Verbinder), Laufstich mit der Stichlänge.

use crate::einstellungen::LaufEinstellung;
use crate::geometrie::{Linie, Punkt, abstand, abstand_punkt_strecke, begrenzung, im_polygon, p, rand_abstand};
use crate::sticharten::lauf::laufstich_mit_ecken;
use std::collections::HashMap;

/// Vorgabe Stichlänge (Dokument 360 Einheiten)
pub const LAENGE: f64 = 2.0;
/// Vorgabe Schlaufenabstand (Dokument 540 Einheiten)
pub const ABSTAND: f64 = 3.0;
/// Zufall der Schlaufenpunkte (Dokument 0,125, Bereich 0 … 0,5)
const ZUFALL: f64 = 0.125;
/// Randabstand der Gitterpunkte in Schlaufenabständen (eigene Festlegung; Dokument 0)
const RAND: f64 = 0.5;
/// Toleranz beim Umwandeln der Kurven in Linienzüge (18 Einheiten)
const TOLERANZ: f64 = 0.1;
/// Schleifen verbinden bis zu diesem Abstand (× Schlaufenabstand)
const VERBINDEN: f64 = 1.125;
/// Lücke zwischen den beiden Verbindungsstrecken (× Schlaufenabstand)
const LUECKE: f64 = 0.125;
/// Spiel der Kantenprüfung (18 Einheiten)
const SPIEL: f64 = 0.1;

/// Stippling-Linien (je getrennter Schleife eine geschlossene Linie) als Laufstiche. `start` = letzter Stich davor:
/// der Baum wächst vom nächsten Gitterpunkt aus, die erste Schleife beginnt am nächsten Punkt.
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

/// `rand()` der MSVC-Laufzeit (0 … 32767).
struct Zufall(u32);
impl Zufall {
    fn naechste(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(214_013).wrapping_add(2_531_011);
        f64::from((self.0 >> 16) & 0x7fff)
    }
    /// gleichverteilt in [lo, hi] wie 0x101a2540
    fn zwischen(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.naechste() * (hi - lo) / 32767.0
    }
}

/// Geometrie der Stippling-Linien (vor dem Laufstich), geschlossen (letzter Punkt = erster).
pub fn stippling_wege(umriss: &[Punkt], loecher: &[Linie], schlaufe: f64, start: Option<Punkt>) -> Vec<Linie> {
    let s = schlaufe.max(0.3);
    let rand = RAND * s;
    let mut ringe = vec![umriss.to_vec()];
    ringe.extend(loecher.iter().cloned());
    let Some(anfang) = start.or_else(|| umriss.first().copied()) else {
        return Vec::new();
    };
    let punkte = gitter(&ringe, s, rand);
    let kanten = kanten(&punkte, &ringe, s, rand);
    let mut zufall = Zufall(1);
    let baum = spannbaum(punkte.len(), &kanten, &mut zufall);
    let pos: Vec<Punkt> = punkte.iter().map(|g| g.0).collect();
    let mut schleifen: Vec<Linie> = Vec::new();
    for teil in teile(&baum) {
        if teil.len() < 2 {
            continue;
        }
        let wurzel = *teil
            .iter()
            .min_by(|&&a, &&b| abstand(pos[a], anfang).total_cmp(&abstand(pos[b], anfang)))
            .unwrap();
        let runde = rundgang(&pos, &baum, wurzel);
        let mut q = schlaufenpunkte(&runde);
        verschieben(&mut q, &mut zufall);
        let linie = glaetten(&q);
        if linie.len() > 2 {
            schleifen.push(linie);
        }
    }
    verbinden(&mut schleifen, VERBINDEN * s, LUECKE * s);
    // Reihenfolge: jeweils die nächste Schleife; Beginn am Punkt nächst dem vorigen Ende
    let mut out = Vec::new();
    let mut von = anfang;
    while !schleifen.is_empty() {
        let (k, idx, _) = schleifen
            .iter()
            .enumerate()
            .map(|(k, w)| {
                let (i, d) = w
                    .iter()
                    .enumerate()
                    .map(|(i, &q)| (i, abstand(q, von)))
                    .min_by(|a, b| a.1.total_cmp(&b.1))
                    .unwrap();
                (k, i, d)
            })
            .min_by(|a, b| a.2.total_cmp(&b.2))
            .unwrap();
        let w = schleifen.remove(k);
        let mut neu: Linie = w[idx..].iter().chain(&w[..idx]).copied().collect();
        neu.push(neu[0]);
        von = neu[0];
        out.push(neu);
    }
    out
}

/// Wabengitter (0x101a0800): Punkt und (Spalte, Zeile).
fn gitter(ringe: &[Linie], s: f64, rand: f64) -> Vec<(Punkt, (i64, i64))> {
    let Some((x0, y0, x1, y1)) = begrenzung(ringe[0].iter().copied()) else {
        return Vec::new();
    };
    if x1 - x0 < 2.0 * rand || y1 - y0 < 2.0 * rand {
        return Vec::new();
    }
    let r = 1.0625 * rand;
    let (x0, y0, x1, y1) = (x0 + r, y0 + r, x1 - r, y1 - r);
    let (w, h) = (x1 - x0, y1 - y0);
    // entlang y: Raster s/2, Anzahl ≡ 2 (mod 3); entlang x: Raster s·√3/2, Anzahl ungerade
    let mut ny = (h / (s / 2.0)).ceil().max(0.0) as i64 + 1;
    while ny % 3 != 2 {
        ny += 1;
    }
    let mut nx = (w / (s * 3f64.sqrt() / 2.0)).ceil().max(0.0) as i64 + 1;
    if nx % 2 == 0 {
        nx += 1;
    }
    let dy = h / (ny - 1) as f64;
    let dx = if nx > 1 { w / (nx - 1) as f64 } else { 0.0 };
    let mut out = Vec::new();
    for zeile in 0..ny {
        for spalte in 0..nx {
            let behalten = if spalte % 2 == 0 {
                zeile % 6 == 0 || zeile % 6 == 4
            } else {
                zeile % 6 == 1 || zeile % 6 == 3
            };
            if !behalten {
                continue;
            }
            let q = p(x0 + spalte as f64 * dx, y0 + zeile as f64 * dy);
            if innen(q, ringe, rand) {
                out.push((q, (spalte, zeile)));
            }
        }
    }
    out
}

/// Punkt in der Fläche und mindestens `rand` vom Rand (0x101a04e0)
fn innen(q: Punkt, ringe: &[Linie], rand: f64) -> bool {
    im_polygon(q, ringe) && rand_abstand(q, ringe) >= rand
}

/// Strecke bleibt in der Fläche (0x101a05a0): beide Enden innen; reicht der Randabstand beider Enden nicht für die
/// Länge, wird halbiert und jede Hälfte geprüft.
fn strecke_innen(a: Punkt, b: Punkt, ringe: &[Linie], rand: f64, tiefe: u32) -> bool {
    if !im_polygon(a, ringe) || !im_polygon(b, ringe) {
        return false;
    }
    let (da, db) = (rand_abstand(a, ringe), rand_abstand(b, ringe));
    if da < rand || db < rand {
        return false;
    }
    if da + db - 2.0 * rand + SPIEL > abstand(a, b) {
        return true;
    }
    if tiefe > 24 {
        return false;
    }
    let m = p((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
    strecke_innen(a, m, ringe, rand, tiefe + 1) && strecke_innen(m, b, ringe, rand, tiefe + 1)
}

/// Kanten des Wabengitters: Abstand² < 1,1 · s² und Strecke in der Fläche.
fn kanten(punkte: &[(Punkt, (i64, i64))], ringe: &[Linie], s: f64, rand: f64) -> Vec<(usize, usize)> {
    let index: HashMap<(i64, i64), usize> = punkte.iter().enumerate().map(|(i, g)| (g.1, i)).collect();
    let mut out = Vec::new();
    for (i, &(a, (c, r))) in punkte.iter().enumerate() {
        for (dc, dr) in [(0, 1), (0, 2), (1, -2), (1, -1), (1, 0), (1, 1), (1, 2)] {
            let Some(&j) = index.get(&(c + dc, r + dr)) else {
                continue;
            };
            let b = punkte[j].0;
            let d2 = (b.x - a.x).powi(2) + (b.y - a.y).powi(2);
            if d2 < 1.1 * s * s && strecke_innen(a, b, ringe, rand, 0) {
                out.push((i, j));
            }
        }
    }
    out
}

/// Kleinster Spannbaum mit zufälligen Gewichten 0,9 … 1,0 (Funktor 0x101a10c0) als Nachbarlisten.
fn spannbaum(n: usize, kanten: &[(usize, usize)], zufall: &mut Zufall) -> Vec<Vec<usize>> {
    let gewicht: Vec<f64> = kanten.iter().map(|_| 0.9 + 0.1 * zufall.naechste() / 32767.0).collect();
    let mut folge: Vec<usize> = (0..kanten.len()).collect();
    folge.sort_by(|&a, &b| gewicht[a].total_cmp(&gewicht[b]).then(a.cmp(&b)));
    let mut eltern: Vec<usize> = (0..n).collect();
    fn wurzel(e: &mut [usize], mut x: usize) -> usize {
        while e[x] != x {
            e[x] = e[e[x]];
            x = e[x];
        }
        x
    }
    let mut baum = vec![Vec::new(); n];
    for k in folge {
        let (a, b) = kanten[k];
        let (ra, rb) = (wurzel(&mut eltern, a), wurzel(&mut eltern, b));
        if ra != rb {
            eltern[ra] = rb;
            baum[a].push(b);
            baum[b].push(a);
        }
    }
    baum
}

/// Zusammenhängende Teile des Baums (Reihenfolge nach erstem Punkt).
fn teile(baum: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut gesehen = vec![false; baum.len()];
    let mut out = Vec::new();
    for v in 0..baum.len() {
        if gesehen[v] {
            continue;
        }
        gesehen[v] = true;
        let mut teil = vec![v];
        let mut k = 0;
        while k < teil.len() {
            for &w in &baum[teil[k]] {
                if !gesehen[w] {
                    gesehen[w] = true;
                    teil.push(w);
                }
            }
            k += 1;
        }
        out.push(teil);
    }
    out
}

/// Rundgang um den Baum (0x101a1a40 + 0x101a1e20): Punkte, doppelte hintereinander entfernt.
fn rundgang(pos: &[Punkt], baum: &[Vec<usize>], wurzel: usize) -> Linie {
    // Kinder im Uhrzeigersinn ab der Richtung zum Elternpunkt (Wurzel: ab +x)
    let kinder = |v: usize, eltern: Option<usize>| -> Vec<usize> {
        let a = match eltern {
            Some(e) => p(pos[e].x - pos[v].x, pos[e].y - pos[v].y),
            None => p(1.0, 0.0),
        };
        let mut l: Vec<(usize, f64, f64)> = baum[v]
            .iter()
            .filter(|&&w| Some(w) != eltern)
            .map(|&w| {
                let d = p(pos[w].x - pos[v].x, pos[w].y - pos[v].y);
                let (u, s) = (d.x * a.x + d.y * a.y, d.x * a.y - d.y * a.x);
                let n = u.hypot(s);
                if n < 1e-12 { (w, 1.0, 0.0) } else { (w, u / n, s / n) }
            })
            .collect();
        // erst s ≥ 0 (u absteigend), dann s < 0 (u aufsteigend) – Vergleich 0x101a3e10
        l.sort_by(|x, y| match (x.2 >= 0.0, y.2 >= 0.0) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (true, true) => y.1.total_cmp(&x.1),
            (false, false) => x.1.total_cmp(&y.1),
        });
        l.into_iter().map(|x| x.0).collect()
    };
    let mut liste = vec![wurzel];
    let mut stapel = vec![(wurzel, kinder(wurzel, None), 0usize)];
    while let Some((v, ks, k)) = stapel.last_mut() {
        if *k < ks.len() {
            let c = ks[*k];
            *k += 1;
            let v = *v;
            liste.push(c);
            stapel.push((c, kinder(c, Some(v)), 0));
        } else {
            stapel.pop();
            if let Some((e, _, _)) = stapel.last() {
                liste.push(*e);
            }
        }
    }
    // führende Einträge gleich dem letzten überspringen, doppelte hintereinander nur einmal
    let letzter = *liste.last().unwrap();
    let mut i = liste.iter().position(|&x| x != letzter).unwrap_or(liste.len());
    let mut out = Vec::new();
    while i < liste.len() {
        out.push(pos[liste[i]]);
        let v = liste[i];
        while i < liste.len() && liste[i] == v {
            i += 1;
        }
    }
    out
}

/// Schlaufenpunkte um den Rundgang (0x101a1f40).
fn schlaufenpunkte(runde: &[Punkt]) -> Linie {
    let n = runde.len();
    let mut out = Vec::with_capacity(3 * n);
    let h = 3f64.sqrt() / 2.0;
    for i in 0..n {
        let (v, q, w) = (runde[(i + n - 1) % n], runde[i], runde[(i + 1) % n]);
        let a = p(0.4 * (q.x - v.x), 0.4 * (q.y - v.y));
        let b = p(0.4 * (w.x - q.x), 0.4 * (w.y - q.y));
        let kreuz = a.x * b.y - a.y * b.x;
        let grenze = 0.125 * (a.x * a.x + a.y * a.y);
        if kreuz > grenze {
            // Linkskurve: innere Ecke
            out.push(p(q.x + b.x - a.x, q.y + b.y - a.y));
        } else if kreuz < -grenze {
            // Rechtskurve: äußere Ecke
            out.push(p(q.x - b.x - a.x, q.y - b.y - a.y));
            out.push(p(q.x - b.x + a.x, q.y - b.y + a.y));
            out.push(p(q.x + a.x, q.y + a.y));
        } else {
            // Sackgasse: halbes Sechseck um den Punkt
            let nn = p(-h * a.y, h * a.x);
            out.push(p(q.x + nn.x - a.x / 2.0, q.y + nn.y - a.y / 2.0));
            out.push(p(q.x + nn.x + a.x / 2.0, q.y + nn.y + a.y / 2.0));
            out.push(p(q.x + a.x, q.y + a.y));
            out.push(p(q.x + a.x / 2.0 - nn.x, q.y + a.y / 2.0 - nn.y));
            out.push(p(q.x - a.x / 2.0 - nn.x, q.y - a.y / 2.0 - nn.y));
        }
    }
    out
}

/// Zufälliger Versatz der Schlaufenpunkte (0x101a2340), der Reihe nach am schon verschobenen Zug.
fn verschieben(q: &mut [Punkt], zufall: &mut Zufall) {
    let n = q.len();
    for i in 0..n {
        let r1 = zufall.zwischen(-ZUFALL, ZUFALL);
        let r2 = zufall.zwischen(-ZUFALL, ZUFALL);
        let (v, m, w) = (q[(i + n - 1) % n], q[i], q[(i + 1) % n]);
        let a = p(m.x - v.x, m.y - v.y);
        let b = p(w.x - m.x, w.y - m.y);
        q[i] = p(m.x - (r1 * a.y + r2 * b.y), m.y + (r1 * a.x + r2 * b.x));
    }
}

/// Glatte geschlossene Kurve durch die Schlaufenpunkte (0x101a2570) als Linienzug (ohne doppelten Endpunkt).
fn glaetten(q: &[Punkt]) -> Linie {
    let n = q.len();
    if n < 3 {
        return q.to_vec();
    }
    let lerp = |a: Punkt, b: Punkt, t: f64| p(a.x + t * (b.x - a.x), a.y + t * (b.y - a.y));
    // je Punkt: Mitte davor → Mitte danach, Stützpunkte bei 0,8 und 0,2
    let mut bez: Vec<[Punkt; 4]> = (0..n)
        .map(|i| {
            let (v, m, w) = (q[(i + n - 1) % n], q[i], q[(i + 1) % n]);
            [lerp(v, m, 0.5), lerp(v, m, 0.8), lerp(m, w, 0.2), lerp(m, w, 0.5)]
        })
        .collect();
    // Nahtstellen: Tangente gemittelt
    for i in 0..n {
        let j = (i + 1) % n;
        let e = bez[i][3];
        let (c1, c2) = (bez[i][2], bez[j][1]);
        let t = p(((e.x - c1.x) + (c2.x - e.x)) / 2.0, ((e.y - c1.y) + (c2.y - e.y)) / 2.0);
        bez[i][2] = p(e.x - t.x, e.y - t.y);
        bez[j][1] = p(e.x + t.x, e.y + t.y);
    }
    let mut out = Vec::new();
    for b in &bez {
        out.push(b[0]);
        flach(b, &mut out, 0);
    }
    out
}

/// Bézierkurve unterteilen, bis sie höchstens `TOLERANZ` von der Sehne abweicht (Endpunkt nicht ausgegeben).
fn flach(b: &[Punkt; 4], out: &mut Linie, tiefe: u32) {
    let d = abstand_punkt_strecke(b[1], b[0], b[3]).max(abstand_punkt_strecke(b[2], b[0], b[3]));
    if d <= TOLERANZ || tiefe >= 12 {
        return;
    }
    let m = |a: Punkt, c: Punkt| p((a.x + c.x) / 2.0, (a.y + c.y) / 2.0);
    let (p01, p12, p23) = (m(b[0], b[1]), m(b[1], b[2]), m(b[2], b[3]));
    let (p012, p123) = (m(p01, p12), m(p12, p23));
    let mitte = m(p012, p123);
    flach(&[b[0], p01, p012, mitte], out, tiefe + 1);
    out.push(mitte);
    flach(&[mitte, p123, p23, b[3]], out, tiefe + 1);
}

/// Stelle auf einer geschlossenen Schleife: Strecke i (von Punkt i nach i+1) und Anteil t.
#[derive(Clone, Copy)]
struct Stelle {
    i: usize,
    t: f64,
}

fn punkt_bei(w: &[Punkt], s: Stelle) -> Punkt {
    let (a, b) = (w[s.i], w[(s.i + 1) % w.len()]);
    p(a.x + s.t * (b.x - a.x), a.y + s.t * (b.y - a.y))
}

/// Stelle um `d` (mm) vor (d > 0) oder zurück (d < 0) entlang der Schleife.
fn bewegen(w: &[Punkt], mut s: Stelle, d: f64) -> Stelle {
    let n = w.len();
    let len = |i: usize| abstand(w[i], w[(i + 1) % n]);
    let mut rest = d.abs();
    for _ in 0..=n {
        let l = len(s.i);
        if d >= 0.0 {
            let frei = (1.0 - s.t) * l;
            if rest <= frei {
                return Stelle { i: s.i, t: if l > 1e-12 { s.t + rest / l } else { s.t } };
            }
            rest -= frei;
            s = Stelle { i: (s.i + 1) % n, t: 0.0 };
        } else {
            let frei = s.t * l;
            if rest <= frei {
                return Stelle { i: s.i, t: if l > 1e-12 { s.t - rest / l } else { s.t } };
            }
            rest -= frei;
            s = Stelle { i: (s.i + n - 1) % n, t: 1.0 };
        }
    }
    s
}

/// Abschnitt der Schleife vorwärts von `von` bis `bis` (über das Ende hinweg).
fn abschnitt(w: &[Punkt], von: Stelle, bis: Stelle, out: &mut Linie) {
    let n = w.len();
    out.push(punkt_bei(w, von));
    let mut i = von.i;
    // gleiche Strecke und bis hinter von: nur der Rest dieser Strecke
    if !(bis.i == von.i && bis.t >= von.t) {
        loop {
            i = (i + 1) % n;
            out.push(w[i]);
            if i == bis.i {
                break;
            }
        }
    }
    out.push(punkt_bei(w, bis));
}

/// Engste Stelle zweier Schleifen: (Abstand, Stelle auf a, Stelle auf b). Rasterzellen der Größe `max`, weiter
/// entfernte Paare zählen nicht.
fn engste(a: &[Punkt], b: &[Punkt], max: f64) -> Option<(f64, Stelle, Stelle)> {
    let zelle = |q: Punkt| ((q.x / max).floor() as i64, (q.y / max).floor() as i64);
    let gitter = |w: &[Punkt]| {
        let mut g: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for i in 0..w.len() {
            let (s, e) = (w[i], w[(i + 1) % w.len()]);
            let (c0, c1) = (zelle(s), zelle(e));
            for x in c0.0.min(c1.0)..=c0.0.max(c1.0) {
                for y in c0.1.min(c1.1)..=c0.1.max(c1.1) {
                    g.entry((x, y)).or_default().push(i);
                }
            }
        }
        g
    };
    let (ga, gb) = (gitter(a), gitter(b));
    let mut best: Option<(f64, Stelle, Stelle)> = None;
    // Punkt von x gegen Strecken von y
    let mut pruefen = |x: &[Punkt], y: &[Punkt], gy: &HashMap<(i64, i64), Vec<usize>>, x_ist_a: bool| {
        for (i, &q) in x.iter().enumerate() {
            let c = zelle(q);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let Some(l) = gy.get(&(c.0 + dx, c.1 + dy)) else { continue };
                    for &k in l {
                        let (s, e) = (y[k], y[(k + 1) % y.len()]);
                        let (vx, vy) = (e.x - s.x, e.y - s.y);
                        let ll = vx * vx + vy * vy;
                        let t = if ll < 1e-18 { 0.0 } else { (((q.x - s.x) * vx + (q.y - s.y) * vy) / ll).clamp(0.0, 1.0) };
                        let d = abstand(q, p(s.x + t * vx, s.y + t * vy));
                        if d <= max && best.is_none_or(|b| d < b.0) {
                            let (sx, sy) = (Stelle { i, t: 0.0 }, Stelle { i: k, t });
                            best = Some(if x_ist_a { (d, sx, sy) } else { (d, sy, sx) });
                        }
                    }
                }
            }
        }
    };
    pruefen(a, b, &gb, true);
    pruefen(b, a, &ga, false);
    best
}

/// Schleifen verbinden, die sich näher als `max` kommen (0x101a3250): an der engsten Stelle beide Schleifen um
/// `luecke` öffnen und über Kreuz verbinden.
fn verbinden(schleifen: &mut Vec<Linie>, max: f64, luecke: f64) {
    loop {
        let mut best: Option<(f64, usize, usize, Stelle, Stelle)> = None;
        for i in 0..schleifen.len() {
            for j in i + 1..schleifen.len() {
                if let Some((d, si, sj)) = engste(&schleifen[i], &schleifen[j], max)
                    && best.is_none_or(|b| d < b.0)
                {
                    best = Some((d, i, j, si, sj));
                }
            }
        }
        let Some((_, i, j, si, sj)) = best else { return };
        let (a, b) = (&schleifen[i], &schleifen[j]);
        let mut neu = Vec::new();
        abschnitt(a, bewegen(a, si, luecke / 2.0), bewegen(a, si, -luecke / 2.0), &mut neu);
        abschnitt(b, bewegen(b, sj, luecke / 2.0), bewegen(b, sj, -luecke / 2.0), &mut neu);
        neu.dedup_by(|x, y| abstand(*x, *y) < 1e-9);
        schleifen[i] = neu;
        schleifen.remove(j);
    }
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

    fn kreuzungen(w: &[Punkt]) -> usize {
        let n = w.len() - 1;
        let mut z = 0;
        for i in 0..n {
            for j in i + 2..n {
                if i == 0 && j == n - 1 {
                    continue;
                }
                if schneiden(w[i], w[i + 1], w[j], w[j + 1]) {
                    z += 1;
                }
            }
        }
        z
    }

    #[test]
    fn wabengitter_wie_das_original() {
        // 30 × 20 mm, s = 3: Raster y s/2 → 15 + 1 = 16 → 17 (≡ 2 mod 3); x s·√3/2 → 12 + 1 = 13 (ungerade)
        let r = rechteck(p(0.0, 0.0), 30.0, 20.0);
        let g = gitter(std::slice::from_ref(&r), 3.0, 0.0);
        // Randpunkte liegen genau auf dem Umriss und zählen nicht; Raster 30/12 und 20/16
        let a = g.iter().find(|x| x.1 == (1, 1)).unwrap().0;
        let b = g.iter().find(|x| x.1 == (2, 6)).unwrap().0;
        assert!((b.x - a.x - 2.5).abs() < 1e-9 && (b.y - a.y - 5.0 * 1.25).abs() < 1e-9, "{a:?} {b:?}");
        // Nachbarn im Abstand ≈ s, jeder Punkt höchstens 3 Nachbarn
        let k = kanten(&g, std::slice::from_ref(&r), 3.0, 0.0);
        let mut grad = vec![0; g.len()];
        for &(a, b) in &k {
            let d = abstand(g[a].0, g[b].0);
            assert!(d > 2.0 && d < 3.0 * 1.1f64.sqrt(), "Kante {d}");
            grad[a] += 1;
            grad[b] += 1;
        }
        assert!(grad.iter().all(|&x| x <= 3));
    }

    #[test]
    fn rundgang_im_uhrzeigersinn() {
        // Stern: Wurzel 0 in der Mitte, Kinder rechts (1), oben (2), links (3)
        let pos = vec![p(0.0, 0.0), p(1.0, 0.0), p(0.0, 1.0), p(-1.0, 0.0)];
        let baum = vec![vec![2, 1, 3], vec![0], vec![0], vec![0]];
        let r = rundgang(&pos, &baum, 0);
        // ab +x im Uhrzeigersinn: rechts, dann links (über unten), dann oben
        assert_eq!(r, vec![p(1.0, 0.0), p(0.0, 0.0), p(-1.0, 0.0), p(0.0, 0.0), p(0.0, 1.0), p(0.0, 0.0)]);
        // die Schleife liegt links vom Weg: Uhrzeigersinn → negative Fläche
        let q = schlaufenpunkte(&r);
        let fl: f64 = (0..q.len()).map(|i| q[i].x * q[(i + 1) % q.len()].y - q[(i + 1) % q.len()].x * q[i].y).sum();
        assert!(fl < 0.0, "Fläche {fl}");
    }

    #[test]
    fn stippling_fuellt_ohne_kreuzung() {
        let r = rechteck(p(0.0, 0.0), 30.0, 20.0);
        let wege = stippling_wege(&r, &[], 1.0, None);
        assert_eq!(wege.len(), 1);
        let w = &wege[0];
        assert_eq!(w.first(), w.last());
        // in der Fläche
        let ringe = vec![r.clone()];
        assert!(w.iter().all(|&q| im_polygon(q, &ringe)));
        assert_eq!(kreuzungen(w), 0);
        // dichte Füllung: Linie etwa 1,5 × Fläche / Schlaufenabstand lang (Wabe: 2 Seiten je Kante)
        let l = crate::geometrie::laenge(w);
        assert!(l > 0.9 * 600.0 && l < 2.0 * 600.0, "Länge {l}");
        // gleich bei jeder Berechnung
        assert_eq!(wege, stippling_wege(&r, &[], 1.0, None));
    }

    #[test]
    fn stippling_mit_loch_und_teilen() {
        let r = rechteck(p(0.0, 0.0), 30.0, 30.0);
        let loch = rechteck(p(0.0, 0.0), 10.0, 10.0);
        let wege = stippling_wege(&r, std::slice::from_ref(&loch), 1.0, Some(p(-20.0, -20.0)));
        assert_eq!(wege.len(), 1);
        let ringe = vec![r.clone(), loch.clone()];
        assert!(wege.iter().flatten().all(|&q| im_polygon(q, &ringe)));
        assert_eq!(kreuzungen(&wege[0]), 0);
        // zwei Flächen, verbunden durch einen schmalen Steg: Schleifen werden verbunden oder nacheinander gestickt
        let zwei = vec![
            p(-20.0, -5.0),
            p(-2.0, -5.0),
            p(-2.0, -0.2),
            p(2.0, -0.2),
            p(2.0, -5.0),
            p(20.0, -5.0),
            p(20.0, 5.0),
            p(-20.0, 5.0),
        ];
        let w2 = stippling_wege(&zwei, &[], 1.0, None);
        assert!(!w2.is_empty());
        assert!(w2.iter().flatten().all(|&q| im_polygon(q, std::slice::from_ref(&zwei))));
        // Laufstich mit Stichlänge
        let st = stippling(&r, &[loch], 2.0, 1.0, &LaufEinstellung::default(), None);
        let max = st.iter().flat_map(|w| w.windows(2)).map(|w| abstand(w[0], w[1])).fold(0.0, f64::max);
        assert!(max <= 2.0 + 1e-9, "Stich {max}");
    }

    #[test]
    fn schleifen_verbinden() {
        // zwei Kreise gleichen Umlaufs (Uhrzeigersinn), 1 mm auseinander → eine Schleife ohne Kreuzung
        let kreis = |cx: f64| -> Linie {
            (0..64).map(|k| {
                let a = -(k as f64) * std::f64::consts::TAU / 64.0;
                p(cx + 5.0 * a.cos(), 5.0 * a.sin())
            }).collect()
        };
        let mut l = vec![kreis(0.0), kreis(11.0)];
        verbinden(&mut l, 1.125 * 3.0, 0.125 * 3.0);
        assert_eq!(l.len(), 1);
        let mut w = l[0].clone();
        w.push(w[0]);
        assert_eq!(kreuzungen(&w), 0);
    }
}
