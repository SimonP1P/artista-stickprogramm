//! Geometrie-Hilfen. Punkte in mm, +y nach oben.
//!
//! Übertragung von `stickprogramm/src/stickprogramm/geometrie.py` (gleiche Rechenreihenfolge,
//! damit Python-Prototyp und Rust-Kern dieselben Stiche liefern).

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "[f64; 2]", into = "[f64; 2]")]
pub struct Punkt {
    pub x: f64,
    pub y: f64,
}

impl Punkt {
    pub const fn new(x: f64, y: f64) -> Self {
        Punkt { x, y }
    }
}

impl From<[f64; 2]> for Punkt {
    fn from(a: [f64; 2]) -> Self {
        Punkt::new(a[0], a[1])
    }
}

impl From<Punkt> for [f64; 2] {
    fn from(p: Punkt) -> Self {
        [p.x, p.y]
    }
}

pub type Linie = Vec<Punkt>;

pub fn p(x: f64, y: f64) -> Punkt {
    Punkt::new(x, y)
}

pub fn abstand(a: Punkt, b: Punkt) -> f64 {
    (b.x - a.x).hypot(b.y - a.y)
}

pub fn zwischen(a: Punkt, b: Punkt, t: f64) -> Punkt {
    p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
}

/// Einheitsvektor von a nach b ((0,0) bei gleichen Punkten).
pub fn richtung(a: Punkt, b: Punkt) -> Punkt {
    let d = abstand(a, b);
    if d == 0.0 {
        return p(0.0, 0.0);
    }
    p((b.x - a.x) / d, (b.y - a.y) / d)
}

pub fn verschieben(q: Punkt, r: Punkt, d: f64) -> Punkt {
    p(q.x + r.x * d, q.y + r.y * d)
}

pub fn drehen(q: Punkt, winkel_grad: f64, um: Punkt) -> Punkt {
    let w = winkel_grad.to_radians();
    let (s, c) = (w.sin(), w.cos());
    let (x, y) = (q.x - um.x, q.y - um.y);
    p(um.x + x * c - y * s, um.y + x * s + y * c)
}

pub fn laenge(linie: &[Punkt]) -> f64 {
    linie.windows(2).map(|w| abstand(w[0], w[1])).sum()
}

/// Bogenlänge bis zu jedem Punkt.
pub fn kumuliert(linie: &[Punkt]) -> Vec<f64> {
    let mut s = vec![0.0];
    for w in linie.windows(2) {
        let letzte = *s.last().unwrap();
        s.push(letzte + abstand(w[0], w[1]));
    }
    s
}

/// Punkt bei Bogenlänge s (auf [0, Länge] begrenzt).
pub fn punkt_bei(linie: &[Punkt], s: f64, kum: &[f64]) -> Punkt {
    if s <= 0.0 {
        return linie[0];
    }
    if s >= *kum.last().unwrap() {
        return *linie.last().unwrap();
    }
    let (mut lo, mut hi) = (0usize, kum.len() - 1);
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if kum[mid] <= s {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let seg = kum[hi] - kum[lo];
    let t = if seg == 0.0 { 0.0 } else { (s - kum[lo]) / seg };
    zwischen(linie[lo], linie[hi], t)
}

pub fn abstand_punkt_strecke(q: Punkt, a: Punkt, b: Punkt) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let l2 = dx * dx + dy * dy;
    if l2 == 0.0 {
        return abstand(q, a);
    }
    let t = (((q.x - a.x) * dx + (q.y - a.y) * dy) / l2).clamp(0.0, 1.0);
    abstand(q, p(a.x + t * dx, a.y + t * dy))
}

/// Richtungsänderung in b (0° = geradeaus, 180° = Umkehr).
pub fn knick_grad(a: Punkt, b: Punkt, c: Punkt) -> f64 {
    let (r1, r2) = (richtung(a, b), richtung(b, c));
    if (r1.x == 0.0 && r1.y == 0.0) || (r2.x == 0.0 && r2.y == 0.0) {
        return 0.0;
    }
    let cosw = (r1.x * r2.x + r1.y * r2.y).clamp(-1.0, 1.0);
    cosw.acos().to_degrees()
}

pub fn ohne_doppelte(linie: &[Punkt]) -> Linie {
    let mut out: Linie = Vec::with_capacity(linie.len());
    for &q in linie {
        if out.last().is_none_or(|&l| abstand(l, q) > 1e-9) {
            out.push(q);
        }
    }
    out
}

/// Offene Linie um d parallel verschieben (d > 0 = nach links in Laufrichtung), Gehrungsecken.
/// Ergibt genau so viele Punkte wie die Eingabe (Punktpaare bleiben einander zugeordnet).
pub fn versatz_linie(linie: &[Punkt], d: f64, max_gehrung: f64) -> Linie {
    let n = linie.len();
    if n < 2 {
        return linie.to_vec();
    }
    let normalen: Vec<Punkt> = linie
        .windows(2)
        .map(|w| {
            let r = richtung(w[0], w[1]);
            p(-r.y, r.x)
        })
        .collect();
    let mut out = Vec::with_capacity(n);
    for (i, &q) in linie.iter().enumerate() {
        let (nrm, f) = if i == 0 {
            (normalen[0], 1.0)
        } else if i == n - 1 {
            (*normalen.last().unwrap(), 1.0)
        } else {
            let (n1, n2) = (normalen[i - 1], normalen[i]);
            let m = p(n1.x + n2.x, n1.y + n2.y);
            let ml = m.x.hypot(m.y);
            if ml < 1e-9 {
                (n1, 1.0)
            } else {
                let nrm = p(m.x / ml, m.y / ml);
                let cosh = nrm.x * n1.x + nrm.y * n1.y;
                (nrm, (1.0 / cosh.max(1e-9)).min(max_gehrung))
            }
        };
        out.push(p(q.x + nrm.x * d * f, q.y + nrm.y * d * f));
    }
    out
}

// ---------------------------------------------------------------- Polygone

/// Fläche mit Vorzeichen (> 0 = gegen den Uhrzeigersinn).
pub fn flaeche_vorzeichen(poly: &[Punkt]) -> f64 {
    let n = poly.len();
    let mut s = 0.0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        s += a.x * b.y - b.x * a.y;
    }
    s / 2.0
}

pub fn geschlossen_ohne_endpunkt(poly: &[Punkt]) -> Linie {
    let mut q = ohne_doppelte(poly);
    if q.len() > 1 && abstand(q[0], *q.last().unwrap()) < 1e-9 {
        q.pop();
    }
    q
}

pub fn gegen_uhrzeiger(poly: &[Punkt]) -> Linie {
    let mut q = geschlossen_ohne_endpunkt(poly);
    if flaeche_vorzeichen(&q) < 0.0 {
        q.reverse();
    }
    q
}

/// Gerade-Ungerade-Regel über alle Ringe (Umriss + Löcher).
pub fn im_polygon(q: Punkt, ringe: &[Linie]) -> bool {
    let mut innen = false;
    for ring in ringe {
        let n = ring.len();
        for i in 0..n {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            if (a.y > q.y) != (b.y > q.y) {
                let xs = a.x + (q.y - a.y) * (b.x - a.x) / (b.y - a.y);
                if xs > q.x {
                    innen = !innen;
                }
            }
        }
    }
    innen
}

/// Kleinster Abstand von q zu einer Kante der Ringe.
pub fn rand_abstand(q: Punkt, ringe: &[Linie]) -> f64 {
    let mut best = f64::INFINITY;
    for ring in ringe {
        let n = ring.len();
        for i in 0..n {
            best = best.min(abstand_punkt_strecke(q, ring[i], ring[(i + 1) % n]));
        }
    }
    best
}

/// Geschlossenen Ring um d nach innen versetzen (d > 0), Umlaufsinn beliebig.
/// Einfaches Gehrungsverfahren für kleine Abstände (Unterlagenrand, Randlauf).
pub fn versatz_ring(ring: &[Punkt], d: f64, max_gehrung: f64) -> Linie {
    let q = geschlossen_ohne_endpunkt(ring);
    let vorzeichen = if flaeche_vorzeichen(&q) >= 0.0 { 1.0 } else { -1.0 };
    let n = q.len();
    let normalen: Vec<Punkt> = (0..n)
        .map(|i| {
            let r = richtung(q[i], q[(i + 1) % n]);
            p(-r.y * vorzeichen, r.x * vorzeichen)
        })
        .collect();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let n1 = normalen[(i + n - 1) % n];
        let n2 = normalen[i];
        let m = p(n1.x + n2.x, n1.y + n2.y);
        let ml = m.x.hypot(m.y);
        if ml < 1e-9 {
            out.push(p(q[i].x + n2.x * d, q[i].y + n2.y * d));
            continue;
        }
        let nrm = p(m.x / ml, m.y / ml);
        let cosh = nrm.x * n1.x + nrm.y * n1.y;
        let f = (1.0 / cosh.max(1e-9)).min(max_gehrung);
        out.push(p(q[i].x + nrm.x * d * f, q[i].y + nrm.y * d * f));
    }
    out
}

/// (x_min, y_min, x_max, y_max)
pub fn begrenzung(punkte: impl IntoIterator<Item = Punkt>) -> Option<(f64, f64, f64, f64)> {
    let mut it = punkte.into_iter();
    let erster = it.next()?;
    let mut b = (erster.x, erster.y, erster.x, erster.y);
    for q in it {
        b = (b.0.min(q.x), b.1.min(q.y), b.2.max(q.x), b.3.max(q.y));
    }
    Some(b)
}

// ---------------------------------------------------------------- Grundformen

/// Geschlossener Kreis als Ring (ohne doppelten Endpunkt), gegen den Uhrzeigersinn.
pub fn kreis(mitte: Punkt, radius: f64, sehnenfehler: f64, start_grad: f64) -> Linie {
    let arg = (1.0 - sehnenfehler / radius.max(1e-9)).max(-1.0);
    let n = ((PI / arg.acos()).ceil() as usize).max(12);
    let s0 = start_grad.to_radians();
    (0..n)
        .map(|i| {
            let w = s0 + 2.0 * PI * i as f64 / n as f64;
            p(mitte.x + radius * w.cos(), mitte.y + radius * w.sin())
        })
        .collect()
}

/// Offener Kreisbogen von von_grad nach bis_grad (mathematisch positiv, wenn bis > von).
pub fn bogen(mitte: Punkt, radius: f64, von_grad: f64, bis_grad: f64, sehnenfehler: f64) -> Linie {
    let spanne = (bis_grad - von_grad).to_radians();
    let schritt = 2.0 * (1.0 - sehnenfehler / radius.max(1e-9)).max(-1.0).acos();
    let n = ((spanne.abs() / schritt).ceil() as usize).max(2);
    let w0 = von_grad.to_radians();
    (0..=n)
        .map(|i| {
            let w = w0 + spanne * i as f64 / n as f64;
            p(mitte.x + radius * w.cos(), mitte.y + radius * w.sin())
        })
        .collect()
}

pub fn rechteck(mitte: Punkt, breite: f64, hoehe: f64) -> Linie {
    let (b, h) = (breite / 2.0, hoehe / 2.0);
    vec![
        p(mitte.x - b, mitte.y - h),
        p(mitte.x + b, mitte.y - h),
        p(mitte.x + b, mitte.y + h),
        p(mitte.x - b, mitte.y + h),
    ]
}

/// Kubische Bézierkurve als Linie mit n+1 Punkten.
pub fn bezier(p0: Punkt, p1: Punkt, p2: Punkt, p3: Punkt, n: usize) -> Linie {
    (0..=n)
        .map(|i| {
            let t = i as f64 / n as f64;
            // powf wie Pythons `**` (C-pow), damit Prototyp und Kern bitgleich rechnen
            let a = (1.0 - t).powf(3.0);
            let b = 3.0 * t * (1.0 - t).powf(2.0);
            let c = 3.0 * t * t * (1.0 - t);
            let d = t.powf(3.0);
            p(
                a * p0.x + b * p1.x + c * p2.x + d * p3.x,
                a * p0.y + b * p1.y + c * p2.y + d * p3.y,
            )
        })
        .collect()
}
