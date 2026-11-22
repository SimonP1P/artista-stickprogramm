//! **Kreuzstich** (Handbuch Kap. 34–37, Programmteil „BERNINA artista Cross Stitch“ = `BLOT.EXE`; Nachtauftrag N9,
//! `Analyse_Teil3/29_kreuzstich.md`).
//!
//! Original (Oberfläche `BLOT.EXE`/`XSLang.DLL`, gelesen mit `dfm_lesen.py` – hoch):
//! - Kreuze liegen auf einem Raster aus Quadraten („matrix of squares or pixels“); die Größe folgt der Stoffdichte
//!   („Fabric Count“: Kreuze je Zoll oder mm je Anzahl Kreuze);
//! - Füllarten (Liste `FillStyle`): Full Cross, 3/4 Cross, 1/2 Cross, 1/4 Cross, Mini Cross, Upright Cross, Double
//!   Cross, Elongated Vertical Cross, Elongated Horizontal Cross, French Knot;
//! - Umriss (Liste `OutlineStyle`): Single Line oder Current Cross Type; Füllen und Umriss je an/aus;
//! - Fäden (Liste `Threads`): 2, 4, 6 = jedes Kreuz einmal, zweimal, dreimal gestickt;
//! - Teilkreuze: „Select the quadrants to be filled“ (Viertel), Halbkreuz „/“ oder „\“.
//!
//! **Stichfolge – eigene Festlegung** (die ARX-Beispiele der CD sind gepackt, das Verfahren von `SQMovementToStitches`
//! ist nicht gelesen): Reihen von unten nach oben, je Reihe zusammenhängende Läufe von links nach rechts. Volle Kreuze
//! im Zickzack hin („/“, „\“ abwechselnd) und zurück (die jeweils anderen Schenkel) – so liegt jeder Stich auf einer
//! Diagonale, nur am rechten Ende des Laufs ein Kantenstich. Übergang zur nächsten Reihe auf der Rasterlinie, wenn
//! sie an gestickte Kreuze grenzt, sonst neuer Zug (Sprung). Teilkreuze Zelle für Zelle, Übergänge auf Rasterlinien
//! wo möglich. Französische Knoten: je Knoten ein Zug aus sechs Stichen à 0,5 mm um die Zellmitte.

use crate::geometrie::{Linie, Punkt, im_polygon, p};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

/// Rasterweite in mm: 10 Kreuze je Zoll (Handbuch-Beispiel „10 stitches per inch“; eigene Festlegung als Standard)
pub const RASTER: f64 = 25.4 / 10.0;
/// Grenzen der Rasterweite (eigene Festlegung: Diagonale ≤ 12,7 mm, Teilkreuz ≥ 0,3 mm)
pub const RASTER_MIN: f64 = 1.0;
pub const RASTER_MAX: f64 = 8.0;
/// Länge der Stiche eines französischen Knotens (eigene Festlegung, über dem Mindeststich 0,3 mm)
pub const KNOTEN: f64 = 0.5;

/// Art des Kreuzes (Reihenfolge wie die Liste „FillStyle“ des Originals)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Art {
    Voll,
    Dreiviertel,
    Halb,
    Viertel,
    Mini,
    Aufrecht,
    Doppelt,
    Laengs,
    Quer,
    Knoten,
}

/// Ecke bzw. Viertel einer Zelle (für Viertel-, Dreiviertel-, Mini-Kreuz; längliche Kreuze nehmen die Seite)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecke {
    UntenLinks,
    UntenRechts,
    ObenLinks,
    ObenRechts,
}

impl Ecke {
    /// Ecke in Zellkoordinaten (0/1)
    fn xy(self) -> (f64, f64) {
        match self {
            Ecke::UntenLinks => (0.0, 0.0),
            Ecke::UntenRechts => (1.0, 0.0),
            Ecke::ObenLinks => (0.0, 1.0),
            Ecke::ObenRechts => (1.0, 1.0),
        }
    }
}

/// Umriss (Liste „OutlineStyle“ des Originals)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Umrandung {
    Keine,
    /// „Single Line“: Laufstich auf den Rasterlinien um die Kreuze
    Linie,
    /// „Current Cross Type“: Kreuze auf den Zellen, durch die der Umriss läuft
    Kreuz,
}

/// Werte eines Kreuzstich-Objekts.
#[derive(Clone, Debug, PartialEq)]
pub struct Werte {
    pub raster: f64,
    pub art: Art,
    pub ecke: Ecke,
    /// Halbkreuz: steigend „/“ (sonst fallend „\“)
    pub steigend: bool,
    /// 1–3 (Fäden 2/4/6)
    pub durchgaenge: usize,
    pub fuellen: bool,
    pub umrandung: Umrandung,
}

impl Default for Werte {
    fn default() -> Self {
        Werte {
            raster: RASTER,
            art: Art::Voll,
            ecke: Ecke::UntenLinks,
            steigend: true,
            durchgaenge: 1,
            fuellen: true,
            umrandung: Umrandung::Keine,
        }
    }
}

pub type Zelle = (i32, i32);

/// Zellen, deren Mitte in der Fläche liegt (Gerade-Ungerade-Regel mit Löchern; eigene Festlegung ≈ halb bedeckt).
pub fn zellen_der_flaeche(umriss: &Linie, loecher: &[Linie], r: f64) -> BTreeSet<Zelle> {
    let mut out = BTreeSet::new();
    if umriss.len() < 3 {
        return out;
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for q in umriss {
        x0 = x0.min(q.x);
        y0 = y0.min(q.y);
        x1 = x1.max(q.x);
        y1 = y1.max(q.y);
    }
    let mut ringe = vec![umriss.clone()];
    ringe.extend(loecher.iter().cloned());
    for j in (y0 / r).floor() as i32..=(y1 / r).ceil() as i32 {
        for i in (x0 / r).floor() as i32..=(x1 / r).ceil() as i32 {
            if im_polygon(p((f64::from(i) + 0.5) * r, (f64::from(j) + 0.5) * r), &ringe) {
                out.insert((i, j));
            }
        }
    }
    out
}

/// Zellen, durch die eine Linie läuft (abgetastet alle r/8).
pub fn zellen_der_linie(l: &Linie, geschlossen: bool, r: f64) -> BTreeSet<Zelle> {
    let mut out = BTreeSet::new();
    let n = l.len();
    let kanten = if geschlossen { n } else { n.saturating_sub(1) };
    for k in 0..kanten {
        let (a, b) = (l[k], l[(k + 1) % n]);
        let schritte = ((a.x - b.x).hypot(a.y - b.y) / (r / 8.0)).ceil().max(1.0) as usize;
        for s in 0..=schritte {
            let t = s as f64 / schritte as f64;
            let (x, y) = (a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t);
            out.insert(((x / r).floor() as i32, (y / r).floor() as i32));
        }
    }
    out
}

/// Läufe einer Reihe: zusammenhängende Spalten [a, b].
fn laeufe(zellen: &BTreeSet<Zelle>) -> BTreeMap<i32, Vec<(i32, i32)>> {
    let mut reihen: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for &(i, j) in zellen {
        reihen.entry(j).or_default().push(i);
    }
    reihen
        .into_iter()
        .map(|(j, mut xs)| {
            xs.sort_unstable();
            let mut l: Vec<(i32, i32)> = Vec::new();
            for x in xs {
                match l.last_mut() {
                    Some((_, b)) if *b + 1 == x => *b = x,
                    _ => l.push((x, x)),
                }
            }
            (j, l)
        })
        .collect()
}

/// Rasterknoten (Spalte, Reihe) in mm.
fn knoten(r: f64, x: i32, y: i32) -> Punkt {
    p(f64::from(x) * r, f64::from(y) * r)
}

/// Punkt in Rastereinheiten (1 = eine Kantenlänge; ganze Zahlen = Rasterknoten)
type Q = (f64, f64);

fn q(x: i32, y: i32) -> Q {
    (f64::from(x), f64::from(y))
}

fn gleich(a: Q, b: Q) -> bool {
    (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9
}

fn ganz(v: f64) -> Option<i32> {
    let r = v.round();
    ((v - r).abs() < 1e-9).then_some(r as i32)
}

/// Volle Kreuze eines Laufs [a, b] in Reihe j: Zickzack hin („/“ und „\\“ abwechselnd), zurück die anderen
/// Schenkel, `d`-mal (weitere Durchgänge über die linke Kante); Anfang unten links, Ende oben links.
fn lauf_voll(a: i32, b: i32, j: i32, d: usize) -> Vec<Q> {
    let n = b - a + 1;
    let mut out = Vec::new();
    for _ in 0..d.max(1) {
        for t in 0..=n {
            out.push(q(a + t, j + t % 2));
        }
        for t in (0..=n).rev() {
            out.push(q(a + t, j + 1 - t % 2));
        }
    }
    out.dedup_by(|x, y| gleich(*x, *y));
    out
}

/// Aufrechte Kreuze eines Laufs: links Mitte → je Zelle Mitte, oben, unten, Mitte → rechts Mitte. Der senkrechte
/// Strich liegt doppelt (ein Laufstich muss aus der Sackgasse zurück); Anfang links, Ende rechts.
fn lauf_aufrecht(a: i32, b: i32, j: i32) -> Vec<Q> {
    let (y, m) = (f64::from(j), f64::from(j) + 0.5);
    let mut out = vec![(f64::from(a), m)];
    for i in a..=b {
        let c = f64::from(i) + 0.5;
        out.extend([(c, m), (c, y + 1.0), (c, y), (c, m), (f64::from(i) + 1.0, m)]);
    }
    out
}

/// Muster einer Teilkreuz-Zelle in Zellkoordinaten (0 … 1); Anfang und Ende liegen auf dem Zellrand, damit der
/// Übergang auf Rasterlinien laufen kann (eigene Festlegung).
fn zellmuster(w: &Werte) -> Vec<Q> {
    let (ex, ey) = w.ecke.xy();
    let m = (0.5, 0.5);
    match w.art {
        // von der Ecke zur Mitte und zurück
        Art::Viertel => vec![(ex, ey), m, (ex, ey)],
        // Viertel zur Mitte, dann der lange Schenkel auf der anderen Diagonale (Handbuch: „a quarter stitch followed
        // by a half stitch“)
        Art::Dreiviertel => {
            let a = (1.0 - ey, ex);
            vec![(ex, ey), m, a, (1.0 - a.0, 1.0 - a.1)]
        }
        // kleines X im Viertel der Ecke: Diagonale über die Kantenmitten, an der Zellkante zur Ecke, zur Mitte und zurück
        Art::Mini => vec![(0.5, ey), (ex, 0.5), (ex, ey), m, (ex, ey)],
        // Kreuz in der linken/rechten Hälfte; Übergang oben auf der Rasterlinie
        Art::Laengs => {
            let x = ex * 0.5;
            vec![(x, 0.0), (x + 0.5, 1.0), (x, 1.0), (x + 0.5, 0.0)]
        }
        // Kreuz in der unteren/oberen Hälfte; Übergang rechts auf der Rasterlinie
        Art::Quer => {
            let y = ey * 0.5;
            vec![(0.0, y), (1.0, y + 0.5), (1.0, y), (0.0, y + 0.5)]
        }
        _ => vec![(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)],
    }
}

/// Muster `d`-mal: hin, zurück, hin …
fn wiederholt<T: Copy>(m: &[T], d: usize) -> Vec<T> {
    let mut out = m.to_vec();
    for w in 1..d.max(1) {
        if w % 2 == 1 {
            out.extend(m.iter().rev().skip(1));
        } else {
            out.extend(m.iter().skip(1));
        }
    }
    out
}

/// Darf ein Übergang auf der Rasterkante a–b (Nachbarknoten) liegen? Nur, wenn sie an ein gesticktes Kreuz grenzt.
fn kante_ok(z: &BTreeSet<Zelle>, a: Zelle, b: Zelle) -> bool {
    if a.1 == b.1 {
        let x = a.0.min(b.0);
        z.contains(&(x, a.1)) || z.contains(&(x, a.1 - 1))
    } else {
        let y = a.1.min(b.1);
        z.contains(&(a.0, y)) || z.contains(&(a.0 - 1, y))
    }
}

/// Knoten, über die ein Punkt auf einer Rasterlinie erreichbar ist (leer: Punkt nicht auf einer erlaubten Linie).
fn zugaenge(a: Q, z: &BTreeSet<Zelle>) -> Vec<Zelle> {
    match (ganz(a.0), ganz(a.1)) {
        (Some(x), Some(y)) => vec![(x, y)],
        (Some(x), None) => {
            let u = (x, a.1.floor() as i32);
            let o = (x, u.1 + 1);
            if kante_ok(z, u, o) { vec![u, o] } else { Vec::new() }
        }
        (None, Some(y)) => {
            let l = (a.0.floor() as i32, y);
            let r = (l.0 + 1, y);
            if kante_ok(z, l, r) { vec![l, r] } else { Vec::new() }
        }
        _ => Vec::new(),
    }
}

/// Größte Entfernung (Rastereinheiten, waagrecht + senkrecht), die auf Rasterlinien überbrückt wird; weiter: Sprung
const WEG_MAX: f64 = 16.0;

/// Übergang von a nach b auf Rasterlinien, die an gestickte Kreuze grenzen (kürzester Weg, wenig Umweg); Ergebnis
/// ohne a, mit b. `None` = kein solcher Weg → neuer Zug (Sprung). Eigene Festlegung: so liegen Verbindungen unter bzw.
/// neben den Kreuzen in denselben Löchern wie die Kreuze.
fn weg(a: Q, b: Q, z: &BTreeSet<Zelle>) -> Option<Vec<Q>> {
    if gleich(a, b) {
        return Some(Vec::new());
    }
    let manh = (a.0 - b.0).abs() + (a.1 - b.1).abs();
    if manh > WEG_MAX {
        return None;
    }
    let (za, zb) = (zugaenge(a, z), zugaenge(b, z));
    if za.is_empty() || zb.is_empty() {
        return None;
    }
    // beide auf derselben Rasterkante
    if za.len() == 2 && zb.len() == 2 && za.iter().all(|n| zb.contains(n)) {
        return Some(vec![b]);
    }
    let grenze = (manh + 8.0) as usize;
    let mut vor: HashMap<Zelle, Option<Zelle>> = HashMap::new();
    let mut schlange = VecDeque::new();
    for &n in &za {
        vor.insert(n, None);
        schlange.push_back((n, 0usize));
    }
    let mut gefunden = None;
    while let Some((n, t)) = schlange.pop_front() {
        if zb.contains(&n) {
            gefunden = Some(n);
            break;
        }
        if t >= grenze {
            continue;
        }
        for m in [(n.0 + 1, n.1), (n.0 - 1, n.1), (n.0, n.1 + 1), (n.0, n.1 - 1)] {
            if !vor.contains_key(&m) && kante_ok(z, n, m) {
                vor.insert(m, Some(n));
                schlange.push_back((m, t + 1));
            }
        }
    }
    let mut n = gefunden?;
    let mut pfad = vec![q(n.0, n.1)];
    while let Some(Some(v)) = vor.get(&n) {
        pfad.push(q(v.0, v.1));
        n = *v;
    }
    pfad.reverse();
    pfad.push(b);
    pfad.dedup_by(|x, y| gleich(*x, *y));
    if pfad.first().is_some_and(|&x| gleich(x, a)) {
        pfad.remove(0);
    }
    Some(pfad)
}

/// Kreuze der Zellen als Züge (in Stickreihenfolge, mm).
fn fuellung(zellen: &BTreeSet<Zelle>, w: &Werte) -> Vec<Linie> {
    let r = w.raster;
    let d = w.durchgaenge.max(1);
    let reihen = laeufe(zellen);
    // Teile (umkehrbar) und ob ein Teil ein eigener Zug ist (Knoten)
    let mut teile: Vec<(Vec<Q>, bool)> = Vec::new();
    // Schlangenlinie: jede zweite Reihe von rechts nach links
    let schlange = |k: usize, l: &Vec<(i32, i32)>| -> Vec<(i32, i32)> {
        if k.is_multiple_of(2) { l.clone() } else { l.iter().rev().copied().collect() }
    };
    match w.art {
        Art::Knoten => {
            let k = KNOTEN / r;
            for &(i, j) in zellen {
                let c = (f64::from(i) + 0.5, f64::from(j) + 0.5);
                let stern = [c, (c.0 + k, c.1), c, (c.0, c.1 + k), c, (c.0 - k, c.1), c];
                teile.push((wiederholt(&stern, d), true));
            }
        }
        Art::Voll => {
            for (j, l) in &reihen {
                for &(a, b) in l {
                    teile.push((lauf_voll(a, b, *j, d), false));
                }
            }
        }
        Art::Aufrecht | Art::Doppelt => {
            for (k, (j, l)) in reihen.iter().enumerate() {
                for (a, b) in schlange(k, l) {
                    let mut t = Vec::new();
                    if w.art == Art::Doppelt {
                        // erst die Kreuze (Ende oben links), an der linken Kante zur Mitte, dann das aufrechte Kreuz
                        t = lauf_voll(a, b, *j, 1);
                    }
                    t.extend(lauf_aufrecht(a, b, *j));
                    teile.push((wiederholt(&t, d), false));
                }
            }
        }
        Art::Halb => {
            // Halbkreuze liegen auf Diagonalen hintereinander: je Diagonale eine Kette (Stich für Stich von Knoten zu
            // Knoten), Nachbarketten abwechselnd hinauf und hinunter. „\\“ über Spiegelung y → −y−1.
            let spiegel = !w.steigend;
            let mut ketten: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
            for &(i, j) in zellen {
                let j = if spiegel { -j - 1 } else { j };
                ketten.entry(i - j).or_default().push(j);
            }
            for (dd, mut js) in ketten {
                js.sort_unstable();
                let mut stuecke: Vec<(i32, i32)> = Vec::new();
                for j in js {
                    match stuecke.last_mut() {
                        Some((_, e)) if *e + 1 == j => *e = j,
                        _ => stuecke.push((j, j)),
                    }
                }
                for (j0, j1) in stuecke {
                    let pfad: Vec<Q> = (j0..=j1 + 1)
                        .map(|j| (f64::from(j + dd), if spiegel { -f64::from(j) } else { f64::from(j) }))
                        .collect();
                    teile.push((wiederholt(&pfad, d), false));
                }
            }
        }
        _ => {
            let m = zellmuster(w);
            for (k, (j, l)) in reihen.iter().enumerate() {
                for (a, b) in schlange(k, l) {
                    let zellen: Vec<i32> = if k.is_multiple_of(2) { (a..=b).collect() } else { (a..=b).rev().collect() };
                    for i in zellen {
                        let zm: Vec<Q> = m.iter().map(|&(u, v)| (f64::from(i) + u, f64::from(*j) + v)).collect();
                        teile.push((wiederholt(&zm, d), false));
                    }
                }
            }
        }
    }
    // verbinden: jedes Teil in der Richtung, deren Anfang näher liegt; Übergang auf Rasterlinien oder neuer Zug
    let mut zuege: Vec<Vec<Q>> = Vec::new();
    let mut zug: Vec<Q> = Vec::new();
    for (mut t, eigen) in teile {
        if t.is_empty() {
            continue;
        }
        if eigen {
            if !zug.is_empty() {
                zuege.push(std::mem::take(&mut zug));
            }
            zuege.push(t);
            continue;
        }
        let Some(&ende) = zug.last() else {
            zug = t;
            continue;
        };
        let dist = |x: Q| (x.0 - ende.0).hypot(x.1 - ende.1);
        if dist(t[t.len() - 1]) + 1e-9 < dist(t[0]) {
            t.reverse();
        }
        match weg(ende, t[0], zellen) {
            Some(w) => {
                zug.extend(w);
                zug.extend(t.into_iter().skip(1));
            }
            None => {
                zuege.push(std::mem::take(&mut zug));
                zug = t;
            }
        }
    }
    if !zug.is_empty() {
        zuege.push(zug);
    }
    zuege.into_iter().map(|z| z.into_iter().map(|(x, y)| p(x * r, y * r)).collect()).collect()
}

/// „Single Line“: Rasterkanten am Rand der Zellen als geschlossene Linien (gegen den Uhrzeigersinn um die Kreuze).
pub fn randlinien(zellen: &BTreeSet<Zelle>, r: f64) -> Vec<Linie> {
    let mut kanten: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for &(i, j) in zellen {
        let mut k = |a: (i32, i32), b: (i32, i32)| kanten.entry(a).or_default().push(b);
        if !zellen.contains(&(i, j - 1)) {
            k((i, j), (i + 1, j));
        }
        if !zellen.contains(&(i + 1, j)) {
            k((i + 1, j), (i + 1, j + 1));
        }
        if !zellen.contains(&(i, j + 1)) {
            k((i + 1, j + 1), (i, j + 1));
        }
        if !zellen.contains(&(i - 1, j)) {
            k((i, j + 1), (i, j));
        }
    }
    let mut out = Vec::new();
    let mut starts: Vec<(i32, i32)> = kanten.keys().copied().collect();
    starts.sort_unstable_by_key(|&(x, y)| (y, x));
    for s in starts {
        while kanten.get(&s).is_some_and(|v| !v.is_empty()) {
            let mut ring = vec![s];
            let mut q = s;
            while let Some(n) = kanten.get_mut(&q).and_then(|v| v.pop()) {
                ring.push(n);
                q = n;
                if q == s {
                    break;
                }
            }
            if ring.len() > 1 {
                out.push(ring.into_iter().map(|(x, y)| knoten(r, x, y)).collect());
            }
        }
    }
    out
}

/// Alle Züge eines Kreuzstich-Objekts: Füllung, danach der Umriss.
pub fn kreuzstich(umriss: &Linie, loecher: &[Linie], zellen_dazu: &[Zelle], w: &Werte) -> Vec<Linie> {
    let r = w.raster;
    let mut flaeche = zellen_der_flaeche(umriss, loecher, r);
    flaeche.extend(zellen_dazu.iter().copied());
    let mut zellen = if w.fuellen { flaeche.clone() } else { BTreeSet::new() };
    if w.umrandung == Umrandung::Kreuz && umriss.len() >= 2 {
        zellen.extend(zellen_der_linie(umriss, true, r));
        for l in loecher {
            zellen.extend(zellen_der_linie(l, true, r));
        }
    }
    let mut zuege = fuellung(&zellen, w);
    if w.umrandung == Umrandung::Linie {
        for ring in randlinien(&flaeche, r) {
            zuege.push(wiederholt(&ring, w.durchgaenge));
        }
    }
    zuege.retain(|z| z.len() >= 2);
    zuege
}

/// Zellen für die Anzeige (Fläche ∪ dazu ∪ Umriss-Kreuze).
pub fn alle_zellen(umriss: &Linie, loecher: &[Linie], zellen_dazu: &[Zelle], w: &Werte) -> BTreeSet<Zelle> {
    let mut z = zellen_der_flaeche(umriss, loecher, w.raster);
    z.extend(zellen_dazu.iter().copied());
    if w.umrandung == Umrandung::Kreuz && umriss.len() >= 2 {
        z.extend(zellen_der_linie(umriss, true, w.raster));
    }
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quadrat(n: f64) -> Linie {
        vec![p(0.0, 0.0), p(n, 0.0), p(n, n), p(0.0, n)]
    }

    #[test]
    fn volle_kreuze_jede_diagonale_einmal() {
        let w = Werte { raster: 2.0, ..Werte::default() };
        // 3 × 2 Zellen
        let z = kreuzstich(&vec![p(0.0, 0.0), p(6.0, 0.0), p(6.0, 4.0), p(0.0, 4.0)], &[], &[], &w);
        assert_eq!(z.len(), 1, "eine Fläche ohne Lücken = ein Zug");
        // jede Zelle hat beide Diagonalen genau einmal
        let mut diag: HashMap<(i32, i32, bool), usize> = HashMap::new();
        for s in z[0].windows(2) {
            let (dx, dy) = (s[1].x - s[0].x, s[1].y - s[0].y);
            if dx.abs() > 1e-9 && dy.abs() > 1e-9 {
                let i = (s[0].x.min(s[1].x) / 2.0).round() as i32;
                let j = (s[0].y.min(s[1].y) / 2.0).round() as i32;
                *diag.entry((i, j, dx * dy > 0.0)).or_default() += 1;
            }
        }
        assert_eq!(diag.len(), 12);
        assert!(diag.values().all(|&n| n == 1), "{diag:?}");
        // alle Stiche auf Rasterlinien oder Diagonalen, keiner länger als die Diagonale
        assert!(z[0].windows(2).all(|s| (s[1].x - s[0].x).hypot(s[1].y - s[0].y) <= 2.0 * 2f64.sqrt() + 1e-9));
    }

    #[test]
    fn faeden_und_teilkreuze() {
        let q = quadrat(4.0);
        let einfach = kreuzstich(&q, &[], &[], &Werte { raster: 2.0, ..Werte::default() });
        let doppelt = kreuzstich(&q, &[], &[], &Werte { raster: 2.0, durchgaenge: 2, ..Werte::default() });
        let n = |z: &Vec<Linie>| z.iter().map(|l| l.len() - 1).sum::<usize>();
        assert!(n(&doppelt) >= 2 * n(&einfach));
        for art in [Art::Dreiviertel, Art::Halb, Art::Viertel, Art::Mini, Art::Aufrecht, Art::Doppelt, Art::Laengs, Art::Quer] {
            let z = kreuzstich(&q, &[], &[], &Werte { raster: 2.0, art, ..Werte::default() });
            assert!(!z.is_empty(), "{art:?}");
            let max = z.iter().flat_map(|l| l.windows(2)).map(|s| (s[1].x - s[0].x).hypot(s[1].y - s[0].y)).fold(0.0, f64::max);
            assert!(max <= 2.0 * 2f64.sqrt() + 1e-9, "{art:?}: {max}");
        }
        let knoten = kreuzstich(&q, &[], &[], &Werte { raster: 2.0, art: Art::Knoten, ..Werte::default() });
        assert_eq!(knoten.len(), 4, "ein Zug je Knoten");
    }

    #[test]
    fn umriss_linie_und_zellen() {
        let w = Werte { raster: 2.0, umrandung: Umrandung::Linie, fuellen: false, ..Werte::default() };
        let z = kreuzstich(&quadrat(4.0), &[], &[(5, 5)], &w);
        // zwei Ringe: 2×2-Block und die einzelne Zelle
        assert_eq!(z.len(), 2);
        assert_eq!(z.iter().map(|l| l.len() - 1).sum::<usize>(), 8 + 4);
        // Loch: Zellen in der Mitte fehlen
        let ring = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let loch = vec![p(4.0, 4.0), p(6.0, 4.0), p(6.0, 6.0), p(4.0, 6.0)];
        let zellen = zellen_der_flaeche(&ring, &[loch], 2.0);
        assert_eq!(zellen.len(), 24);
        assert!(!zellen.contains(&(2, 2)));
    }
}
