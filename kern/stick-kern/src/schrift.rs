//! Schrift mit den Original-Alphabeten der BERNINA artista V4 (ESB, `Analyse_Teil3/09_esb_alphabete.md`).
//!
//! Ein Alphabet besteht aus Zeichen; jedes Zeichen hat eine Grundlinie und Objekte wie im Design des Originals –
//! meist Satinsäulen (EmbroideryPatch) aus einem geschlossenen Umriss mit **Winkellinien**. Das Lesen der Datei steckt
//! in `stick-formate::esb`; hier stehen das Modell, die Umwandlung der Säulen in die zwei Seiten des Rechenkerns und
//! der Satz eines Texts (Höhe, Breite, Buchstabenabstand, Position).

use crate::design::{Objekt, ObjektArt, SatinOptionen, Unterlage};
use crate::geometrie::{Linie, Punkt, p};
use std::collections::BTreeMap;

/// Einheit der Alphabete: 1 mm = 180 (Analyse/code/06)
pub const EINHEIT: f64 = 180.0;
/// Referenzhöhe: Die Zeichensätze im Maschinenspeicher sind alle genau im Maßstab 0,5 gestickt (auch Zurich mit
/// 61,5 mm hohem „H“) → für alle Alphabete gleiche Referenz, Großbuchstaben ≈ 50 mm (`Analyse_Teil3/09` §4,
/// mittel-hoch; im Code noch nicht bestätigt).
pub const REFERENZ_HOEHE_MM: f64 = 50.0;
/// Wortabstand (Leerzeichen) in Teilen der Buchstabenhöhe: Vorlage NORMAL.AMT, Schriftvorgaben 0x3e82 →
/// **0x3e85 = 0,25**; Lettering-Dialog „Word Spacing … % of Height“ (RESENU DLG Lettering); gleiches Feld in jedem
/// Schriftobjekt der Musterdesigns (Datensatz 0x7c, 0,25 oder 0,5). Mittel (Zuordnung über Reihenfolge im Dialog).
pub const WORTABSTAND_ANTEIL: f64 = 0.25;
/// Zeilenabstand (Lücke zwischen zwei Zeilen) in Teilen der Höhe: Vorlage **0x3f2d = 0,6**, Dialog „Line Spacing … %
/// of Height“; in den Musterdesigns 0,6 oder 0,5 am Ende des Datensatzes 0x7c. Mittel (ob Lücke oder Abstand der
/// Grundlinien: als Lücke gedeutet, weil 60 % als Grundlinienabstand Zeilen überlappen ließe).
pub const ZEILENABSTAND_ANTEIL: f64 = 0.6;
/// Standardhöhe einer neuen Schrift: Vorlage **0x3e88 = 1800 → 10 mm**.
pub const STANDARD_HOEHE_MM: f64 = 10.0;
/// Stil „Schrift“ (NORMAL.AMT, `Analyse_Teil3/05`, `Analyse/code/10`): Auto Spacing an mit Faktor 0,9 (0x2736;
/// in den Schrift-Musterdesigns 0,9 bzw. 0,87), Kurzstiche an (0x273a), Auto-Unterlage an mit vorgewählter Art
/// Zickzack (0x27a7) – Handbuch PDF-S. 186/379: Schrift hat Auto-Unterlage, bei kleiner Schrift abschalten.
pub const SCHRIFT_FAKTOR: f64 = 0.9;
pub const SCHRIFT_UNTERLAGE: &str = "zickzack";
/// Laufweg im Buchstaben, wenn das Alphabet keinen nennt (Drifter: Lauf 0x271c 540 = 3,0 mm)
pub const SCHRIFT_LAUFWEG: f64 = 3.0;

/// Bézier-Knoten in Alphabet-Einheiten (1/180 mm), Anfasser optional.
#[derive(Clone, Debug, PartialEq)]
pub struct Knoten {
    pub ein: Option<(f64, f64)>,
    pub punkt: (f64, f64),
    pub aus: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Umriss {
    pub geschlossen: bool,
    pub knoten: Vec<Knoten>,
}

/// Winkellinie einer Satinsäule: von Segment `a` beim Anteil `x` zu Segment `c` beim Anteil `y` des Umrisses
/// (IMPORT.DLL 0x10018970).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Winkellinie {
    pub a: u32,
    pub x: f64,
    pub c: u32,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stichart {
    /// Satin: fester Abstand in mm (0x15, z. B. 72 = 0,40 mm); hängt der Effekt Auto Spacing (0x21) am Objekt, gilt
    /// stattdessen die Tabelle × Faktor (`Analyse_Teil3/05` §1: vorhanden = aktiv)
    Satin {
        abstand: Option<f64>,
        auto_spacing: bool,
    },
    Tatami,
    /// Laufstich mit Stichlänge in mm
    Lauf {
        laenge: Option<f64>,
    },
    Unbekannt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZeichenObjekt {
    /// Klasse im Original (0x06 EmbroideryPatch, 0x05 EmbroideryCurve, 0x0c InputCCurve …)
    pub klasse: u32,
    pub umrisse: Vec<Umriss>,
    pub winkel: Vec<Winkellinie>,
    pub stichart: Stichart,
    /// Unterlagen des Objekts im Alphabet: mittellauf, edge_walk, zickzack, doppelzickzack
    pub unterlagen: Vec<String>,
    /// Stichlänge des Laufwegs (Lauf 0x271c in der Füllung), mm
    pub laufweg: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Zeichen {
    pub zeichen: char,
    /// Grundlinie x1, y1, x2, y2 (Alphabet-Einheiten); Vorschub = x2 − x1
    pub grundlinie: [f64; 4],
    pub objekte: Vec<ZeichenObjekt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alphabet {
    pub name: String,
    /// Buchstabenabstand in Teilen der Höhe (Kopf, float 2: Drifter 0,10) – „automatically as a percentage of letter
    /// height“ (Handbuch PDF-S. 168)
    pub abstand_anteil: f64,
    pub zeichen: BTreeMap<char, Zeichen>,
}

// ---------------------------------------------------------------- Geometrie der Umrisse

fn segmente(u: &Umriss) -> usize {
    if u.geschlossen {
        u.knoten.len()
    } else {
        u.knoten.len().saturating_sub(1)
    }
}

/// Punkt auf Segment `s` beim Anteil `t` (kubische Bézierkurve; fehlender Anfasser = Ankerpunkt).
pub fn segment_punkt(u: &Umriss, s: usize, t: f64) -> (f64, f64) {
    let n = u.knoten.len();
    let s = s.min(segmente(u).saturating_sub(1));
    let a = &u.knoten[s];
    let b = &u.knoten[(s + 1) % n];
    let (p0, p3) = (a.punkt, b.punkt);
    let p1 = a.aus.unwrap_or(p0);
    let p2 = b.ein.unwrap_or(p3);
    let k = [
        (1.0 - t).powi(3),
        3.0 * (1.0 - t).powi(2) * t,
        3.0 * (1.0 - t) * t * t,
        t.powi(3),
    ];
    (
        k[0] * p0.0 + k[1] * p1.0 + k[2] * p2.0 + k[3] * p3.0,
        k[0] * p0.1 + k[1] * p1.1 + k[2] * p2.1 + k[3] * p3.1,
    )
}

fn punkt_bei_parameter(u: &Umriss, pos: f64) -> (f64, f64) {
    let n = segmente(u);
    let mut q = pos;
    if u.geschlossen {
        q = q.rem_euclid(n as f64);
    }
    let s = (q.floor() as usize).min(n.saturating_sub(1));
    segment_punkt(u, s, (q - s as f64).clamp(0.0, 1.0))
}

/// Punkte des Umrisses von Parameter `von` nach `bis` (Parameter = Segment + Anteil) in Richtung ±1.
fn weg(u: &Umriss, von: f64, bis: f64, richtung: f64) -> Vec<(f64, f64)> {
    let n = segmente(u) as f64;
    let d = if u.geschlossen {
        if richtung > 0.0 {
            (bis - von).rem_euclid(n)
        } else {
            (von - bis).rem_euclid(n)
        }
    } else {
        (bis - von) * richtung
    };
    let schritte = ((d.abs() * 16.0).ceil() as usize).max(1);
    (0..=schritte)
        .map(|i| punkt_bei_parameter(u, von + richtung * d * i as f64 / schritte as f64))
        .collect()
}

fn kumuliert(l: &[(f64, f64)]) -> Vec<f64> {
    let mut k = vec![0.0];
    for w in l.windows(2) {
        let last = *k.last().unwrap();
        k.push(last + ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt());
    }
    k
}

fn bei(l: &[(f64, f64)], kum: &[f64], s: f64) -> (f64, f64) {
    if s <= 0.0 {
        return l[0];
    }
    for i in 1..l.len() {
        if kum[i] >= s {
            let t = if kum[i] > kum[i - 1] {
                (s - kum[i - 1]) / (kum[i] - kum[i - 1])
            } else {
                0.0
            };
            return (
                l[i - 1].0 + (l[i].0 - l[i - 1].0) * t,
                l[i - 1].1 + (l[i].1 - l[i - 1].1) * t,
            );
        }
    }
    *l.last().unwrap()
}

/// Punktfolge in Alphabet-Einheiten
pub type Zug = Vec<(f64, f64)>;

/// Satinsäule mit Winkellinien → Seiten A und B (Alphabet-Einheiten) mit Punkt-zu-Punkt-Zuordnung.
///
/// Seite A läuft über den Umriss von der ersten zur letzten Winkellinie (Richtung der a-Werte), Seite B in
/// Gegenrichtung. Zwischen zwei Winkellinien werden beide Teilwege mit gleich vielen Punkten nach relativer
/// Bogenlänge abgetastet (1 Punkt je mm), sodass die Winkellinien selbst exakt zugeordnet bleiben.
/// Grundlage `Analyse_Teil3/09` §2 (hoch); die Abtastung ist eine **eigene Festlegung**.
pub fn saeule_seiten(u: &Umriss, w: &[Winkellinie]) -> Option<(Zug, Zug)> {
    if w.len() < 2 || u.knoten.len() < 2 {
        return None;
    }
    let n = segmente(u) as f64;
    let pa: Vec<f64> = w.iter().map(|l| l.a as f64 + l.x).collect();
    let pb: Vec<f64> = w.iter().map(|l| l.c as f64 + l.y).collect();
    let d = if u.geschlossen {
        (pa[1] - pa[0]).rem_euclid(n)
    } else {
        pa[1] - pa[0]
    };
    let ra = if u.geschlossen {
        if d <= n / 2.0 { 1.0 } else { -1.0 }
    } else if d >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for k in 0..w.len() - 1 {
        let wa = weg(u, pa[k], pa[k + 1], ra);
        let wb = weg(u, pb[k], pb[k + 1], -ra);
        let (ka, kb) = (kumuliert(&wa), kumuliert(&wb));
        let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
        let teile = ((la.max(lb) / EINHEIT).ceil() as usize).max(1);
        let letzte = k == w.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(bei(&wa, &ka, t * la));
            sb.push(bei(&wb, &kb, t * lb));
        }
    }
    Some((sa, sb))
}

/// Satinsäule aus **zwei Schienen** (InputCCurve mit EmbroideryPatch, z. B. Knopflöcher in `ButtonHoles.ESB`):
/// Winkellinie k verbindet Schiene 1 bei Segment `a` + `x` mit Schiene 2 bei Segment `c` + `y`. Zwischen zwei
/// Winkellinien werden beide Teilwege mit gleich vielen Punkten abgetastet (1 Punkt je mm, wie `saeule_seiten`).
/// Ohne Winkellinien: beide Schienen ganz (Zuordnung nach relativer Bogenlänge beim Sticken).
pub fn schienen_seiten(r1: &Umriss, r2: &Umriss, w: &[Winkellinie]) -> Option<(Zug, Zug)> {
    if r1.knoten.len() < 2 || r2.knoten.len() < 2 {
        return None;
    }
    if w.len() < 2 {
        return Some((umriss_punkte(r1), umriss_punkte(r2)));
    }
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for k in 0..w.len() - 1 {
        let (a0, a1) = (w[k].a as f64 + w[k].x, w[k + 1].a as f64 + w[k + 1].x);
        let (b0, b1) = (w[k].c as f64 + w[k].y, w[k + 1].c as f64 + w[k + 1].y);
        let wa = weg(r1, a0, a1, if a1 >= a0 { 1.0 } else { -1.0 });
        let wb = weg(r2, b0, b1, if b1 >= b0 { 1.0 } else { -1.0 });
        let (ka, kb) = (kumuliert(&wa), kumuliert(&wb));
        let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
        let teile = ((la.max(lb) / EINHEIT).ceil() as usize).max(1);
        let letzte = k == w.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(bei(&wa, &ka, t * la));
            sb.push(bei(&wb, &kb, t * lb));
        }
    }
    Some((sa, sb))
}

/// Umriss als Punktliste (Alphabet-Einheiten), je Segment 12 Teilstücke.
pub fn umriss_punkte(u: &Umriss) -> Vec<(f64, f64)> {
    let n = segmente(u);
    let mut out = Vec::new();
    for s in 0..n {
        for i in 0..12 {
            out.push(segment_punkt(u, s, i as f64 / 12.0));
        }
    }
    if let Some(l) = u.knoten.get(if u.geschlossen { 0 } else { u.knoten.len() - 1 }) {
        out.push(l.punkt);
    }
    out
}

/// Punkte eines Laufs mit Stich auf jedem Knoten (Motive, `Analyse_Teil3/07` §1: das Original sticht Eckpunkte genau
/// an): gerade Segmente (ohne Anfasser) nur mit ihren Endpunkten, Bézier-Segmente mit 12 Zwischenpunkten.
pub fn lauf_punkte(u: &Umriss) -> (Vec<(f64, f64)>, Vec<bool>) {
    let n = segmente(u);
    let k = u.knoten.len();
    let (mut pts, mut ecken) = (Vec::new(), Vec::new());
    for s in 0..n {
        let (a, b) = (&u.knoten[s], &u.knoten[(s + 1) % k]);
        pts.push(a.punkt);
        ecken.push(true);
        if a.aus.is_some() || b.ein.is_some() {
            for i in 1..12 {
                pts.push(segment_punkt(u, s, i as f64 / 12.0));
                ecken.push(false);
            }
        }
    }
    if let Some(l) = u.knoten.get(if u.geschlossen { 0 } else { k.saturating_sub(1) }) {
        pts.push(l.punkt);
        ecken.push(true);
    }
    (pts, ecken)
}

// ---------------------------------------------------------------- Textsatz

/// Vorgaben für einen Text (wie Object Properties – Lettering im Original).
#[derive(Clone, Debug, PartialEq)]
pub struct TextVorgabe {
    pub text: String,
    /// Buchstabenhöhe in mm (Original 1–200 mm, Handbuch S. 165)
    pub hoehe: f64,
    /// Breite in % der Höhe (100 = Original)
    pub breite_prozent: f64,
    /// Buchstabenabstand in mm; None = Anteil des Alphabets × Höhe
    pub abstand: Option<f64>,
    /// Bezugspunkt auf der Grundlinie (mm)
    pub position: Punkt,
    /// 0 = Text beginnt bei `position`, 0,5 = mittig, 1 = endet dort (wie Ausrichtung links/Mitte/rechts)
    pub ausrichtung: f64,
    /// Unterlage: None = wie Stil „Schrift“ (Zickzack, bzw. die Unterlage des Alphabets), Some(leer) = keine
    pub unterlage: Option<Vec<String>>,
    pub farbe: String,
    /// Wortabstand in Teilen der Höhe (None = Vorlage 0,25)
    pub wortabstand: Option<f64>,
    /// Zeilenabstand (Lücke) in Teilen der Höhe (None = Vorlage 0,6)
    pub zeilenabstand: Option<f64>,
    /// Grundlinie: None = gerade waagrecht; sonst `Grundlinie` (Handbuch Kap. 19)
    pub grundlinie: Option<Grundlinie>,
    /// Kursiv: Neigung in Grad, −45 … 45, positiv nach rechts (Handbuch Kap. 18 „Italic“, Vorlage 0x3e8a = 0)
    pub kursiv: f64,
}

/// Radius einer neuen Kreis-Grundlinie: Vorlage 0x3e8b/0x3e8c = 9000 → 50 mm (Zuordnung mittel)
pub const KREIS_RADIUS_MM: f64 = 50.0;
/// Buchstabenabstand der Lettering-Vorlage (NORMAL.AMT 0x3e84 = 0,1 der Höhe); hier für Computer-Schriften senkrecht
pub const BUCHSTABENABSTAND_ANTEIL: f64 = 0.1;
/// Stützpunkte eines Ovals/Bogens als Grundlinie – **eigene Festlegung** (Genauigkeit der Bogenlänge)
const BOGEN_PUNKTE: usize = 720;

/// Grundlinie (Handbuch Kap. 19 „Applying lettering baselines“; Dialog „Layout“, RESENU DLG 1039: Baseline, Length,
/// Line Angle, Radius X/Y, Justif. Angle, Tilt, Arc Angle, Position Above/Through/Below).
/// Arten: `kreis` (Circle CW/CCW, auch Oval und Bogen), `gerade` (Straight mit Winkel/Länge), `senkrecht` (Vertical),
/// `kurve` (Any Shape), `mehrfach` (Multiple: oben – Mitte – unten). Der Text wird zuerst gerade gesetzt, dann wird
/// jeder Buchstabe starr auf die Grundlinie gelegt (Bogenlänge = Lage auf der geraden Zeile).
#[derive(Clone, Debug, PartialEq)]
pub struct Grundlinie {
    pub art: String,
    /// Radius X in mm (Kreis/Oval)
    pub radius: f64,
    /// Radius Y in mm; None = Kreis
    pub radius_y: Option<f64>,
    /// Kreis: Justif. Angle = Lage des Ankers in Grad (Standard 90 oben, 270 unten); gerade: Line Angle
    pub winkel: Option<f64>,
    /// Tilt: Drehung des Ovals in Grad
    pub neigung: f64,
    /// Arc Angle: nur ein Bogen dieses Winkels (Grad) um den Anker – der Text wird eingepasst
    pub bogen: Option<f64>,
    /// Length (gerade): feste Länge in mm – der Text wird eingepasst
    pub laenge: Option<f64>,
    /// „oben“ (Circle CW, im Uhrzeiger) oder „unten“ (Circle CCW, gegen den Uhrzeiger)
    pub lage: Option<String>,
    /// Position zur Grundlinie: „ueber“ (Above), „durch“ (Through), „unter“ (Below). Standard laut Handbuch: über der
    /// CW-, unter der CCW-Grundlinie („letters are positioned above the Circle CW and below Circle CCW baseline“)
    pub stand: Option<String>,
    /// Punkte der eigenen Kurve in mm (Any Shape)
    pub punkte: Vec<Punkt>,
    /// Ausrichtung „Full“: Buchstaben gleichmäßig über die ganze Länge verteilen
    pub blocksatz: bool,
}

impl Default for Grundlinie {
    fn default() -> Self {
        Grundlinie {
            art: "kreis".into(),
            radius: KREIS_RADIUS_MM,
            radius_y: None,
            winkel: None,
            neigung: 0.0,
            bogen: None,
            laenge: None,
            lage: None,
            stand: None,
            punkte: Vec::new(),
            blocksatz: false,
        }
    }
}

/// Grundlinie als Polylinie mit Bogenlänge (Oval, Bogen, gerade mit Länge, eigene Kurve).
struct Pfad {
    pts: Vec<Punkt>,
    kum: Vec<f64>,
}

impl Pfad {
    fn neu(roh: Vec<Punkt>) -> Pfad {
        let mut pts: Vec<Punkt> = Vec::new();
        for q in roh {
            if pts.last().is_none_or(|l| (l.x - q.x).hypot(l.y - q.y) > 1e-9) {
                pts.push(q);
            }
        }
        let mut kum = vec![0.0];
        for w in pts.windows(2) {
            kum.push(kum[kum.len() - 1] + (w[1].x - w[0].x).hypot(w[1].y - w[0].y));
        }
        Pfad { pts, kum }
    }

    fn laenge(&self) -> f64 {
        self.kum[self.kum.len() - 1]
    }

    /// Punkt und Richtung (rad) bei Bogenlänge `s`; vor dem Anfang und nach dem Ende geradlinig verlängert.
    fn bei(&self, s: f64) -> (Punkt, f64) {
        let n = self.pts.len();
        let i = self.kum[..n - 1].iter().rposition(|&k| k <= s).unwrap_or(0);
        let (a, b) = (self.pts[i], self.pts[i + 1]);
        let t = (s - self.kum[i]) / (self.kum[i + 1] - self.kum[i]);
        (
            p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t),
            (b.y - a.y).atan2(b.x - a.x),
        )
    }
}

/// Waagrechte Ausdehnung eines Buchstabens (Formpunkte).
fn x_bereich(o: &Objekt) -> Option<(f64, f64)> {
    let mut pts = Vec::new();
    crate::transformation::formpunkte(&o.art, &mut pts);
    if pts.is_empty() {
        return None;
    }
    Some(
        pts.iter()
            .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x))),
    )
}

/// Buchstaben starr abbilden: Punkt (xc, yb) der geraden Zeile → `ziel`, um `drehung` (rad) gedreht.
fn starr(o: &Objekt, xc: f64, yb: f64, ziel: Punkt, drehung: f64, winkel_standard: f64) -> Objekt {
    let (sn, cs) = drehung.sin_cos();
    let m = crate::transformation::Matrix {
        a: cs,
        b: -sn,
        c: sn,
        d: cs,
        e: ziel.x - (cs * xc - sn * yb),
        f: ziel.y - (sn * xc + cs * yb),
    };
    crate::transformation::abbilden(o, &m, winkel_standard)
}

impl Grundlinie {
    pub fn pruefen(&self, text: &str) -> Result<(), String> {
        if !matches!(
            self.art.as_str(),
            "kreis" | "gerade" | "senkrecht" | "kurve" | "mehrfach"
        ) {
            return Err(format!(
                "Grundlinie „{}“ unbekannt (kreis, gerade, senkrecht, kurve, mehrfach)",
                self.art
            ));
        }
        if self.radius <= 0.0 || self.radius_y.is_some_and(|r| r <= 0.0) {
            return Err("Radius der Grundlinie muss größer als 0 sein".into());
        }
        if self.bogen.is_some_and(|b| b <= 0.0 || b > 360.0) {
            return Err("Bogenwinkel der Grundlinie: größer als 0, höchstens 360°".into());
        }
        if self.laenge.is_some_and(|l| l <= 0.0) {
            return Err("Länge der Grundlinie muss größer als 0 sein".into());
        }
        if let Some(x) = self.lage.as_deref().filter(|x| !matches!(*x, "oben" | "unten")) {
            return Err(format!("Lage „{x}“ unbekannt (oben, unten)"));
        }
        if let Some(x) = self
            .stand
            .as_deref()
            .filter(|x| !matches!(*x, "ueber" | "durch" | "unter"))
        {
            return Err(format!("Stand „{x}“ unbekannt (ueber, durch, unter)"));
        }
        if self.art == "kurve" && Pfad::neu(self.punkte.clone()).laenge() <= 0.0 {
            return Err("Grundlinie kurve braucht mindestens zwei verschiedene Punkte".into());
        }
        if self.art == "mehrfach" && text.split('\n').count() > 3 {
            return Err("Grundlinie mehrfach: höchstens drei Zeilen (oben, Mitte, unten)".into());
        }
        if self.blocksatz
            && !(self.art == "kurve"
                || (self.art == "gerade" && self.laenge.is_some())
                || (self.art == "kreis" && self.bogen.is_some()))
        {
            return Err(
                "Ausrichtung „block“ braucht eine Grundlinie mit fester Länge (kurve, gerade mit laenge, kreis mit bogen)"
                    .into(),
            );
        }
        Ok(())
    }

    fn unten(&self) -> bool {
        self.lage.as_deref() == Some("unten")
    }

    /// Welcher Punkt der Buchstabenhöhe auf der Grundlinie liegt (0 = Fuß, h/2 = Mitte, h = Kopf).
    fn stand_versatz(&self, hoehe: f64) -> f64 {
        match self.stand.as_deref() {
            Some("ueber") => 0.0,
            Some("durch") => hoehe / 2.0,
            Some("unter") => hoehe,
            _ if self.unten() => hoehe,
            _ => 0.0,
        }
    }

    fn anker_winkel(&self) -> f64 {
        self.winkel
            .unwrap_or(if self.unten() { 270.0 } else { 90.0 })
            .to_radians()
    }

    /// Kreis: Buchstaben einer gerade gesetzten Zeile auf den Kreis legen. `zeile` = Vorgabe der Zeile (Grundlinie y,
    /// Ankerpunkt x), `mitte` = Kreismitte (Position des Texts). Folgezeilen liegen innen (oben) bzw. außen (unten).
    pub fn anwenden(
        &self,
        objekte: Vec<Objekt>,
        zeile: &TextVorgabe,
        mitte: Punkt,
        winkel_standard: f64,
    ) -> Vec<Objekt> {
        let unten = self.unten();
        let versatz = zeile.position.y - mitte.y; // 0 für die erste Zeile, negativ für Folgezeilen
        let r = if unten {
            self.radius - versatz
        } else {
            self.radius + versatz
        }
        .max(1.0);
        let w0 = self.anker_winkel();
        let d = self.stand_versatz(zeile.hoehe);
        objekte
            .into_iter()
            .map(|o| {
                let Some((x0, x1)) = x_bereich(&o) else {
                    return o;
                };
                let xc = (x0 + x1) / 2.0;
                let s = xc - zeile.position.x;
                let (theta, drehung) = if unten {
                    (w0 + s / r, w0 + s / r + std::f64::consts::FRAC_PI_2)
                } else {
                    (w0 - s / r, w0 - s / r - std::f64::consts::FRAC_PI_2)
                };
                let ziel = p(mitte.x + r * theta.cos(), mitte.y + r * theta.sin());
                starr(&o, xc, zeile.position.y + d, ziel, drehung, winkel_standard)
            })
            .collect()
    }

    /// Pfad der Grundlinie für eine Zeile, Bogenlänge des Ankers und ob der Text eingepasst wird.
    /// None = keine Pfad-Grundlinie (exakter Kreis, senkrecht, gerade ohne Länge, mehrfach).
    fn pfad(&self, zeile: &TextVorgabe, mitte: Punkt) -> Option<(Pfad, f64, bool)> {
        let versatz = zeile.position.y - mitte.y;
        match self.art.as_str() {
            "kurve" => {
                let pf = Pfad::neu(self.punkte.iter().map(|q| p(q.x, q.y + versatz)).collect());
                let anker = zeile.ausrichtung * pf.laenge();
                Some((pf, anker, true))
            }
            "gerade" => {
                let l = self.laenge?;
                let (sn, cs) = self.winkel.unwrap_or(0.0).to_radians().sin_cos();
                let a = zeile.ausrichtung * l;
                // Folgezeilen senkrecht zur Linie versetzt
                let start = p(mitte.x - a * cs - versatz * sn, mitte.y - a * sn + versatz * cs);
                Some((Pfad::neu(vec![start, p(start.x + l * cs, start.y + l * sn)]), a, true))
            }
            "kreis"
                if self.radius_y.is_some_and(|r| r != self.radius) || self.neigung != 0.0 || self.bogen.is_some() =>
            {
                let unten = self.unten();
                let plus = if unten { -versatz } else { versatz };
                let rx = (self.radius + plus).max(1.0);
                let ry = (self.radius_y.unwrap_or(self.radius) + plus).max(1.0);
                let w0 = self.anker_winkel();
                let spanne = self.bogen.unwrap_or(360.0).to_radians();
                let (sn, cs) = self.neigung.to_radians().sin_cos();
                let n = BOGEN_PUNKTE;
                let pts = (0..=n)
                    .map(|k| {
                        let u = (k as f64 / n as f64 - 0.5) * spanne;
                        let t = if unten { w0 + u } else { w0 - u };
                        let (x, y) = (rx * t.cos(), ry * t.sin());
                        p(mitte.x + cs * x - sn * y, mitte.y + sn * x + cs * y)
                    })
                    .collect();
                let pf = Pfad::neu(pts);
                let anker = if self.bogen.is_some() {
                    zeile.ausrichtung * pf.laenge()
                } else {
                    pf.kum[n / 2]
                };
                Some((pf, anker, self.bogen.is_some()))
            }
            _ => None,
        }
    }

    /// Buchstaben entlang eines Pfads legen. Mit `einpassen` füllt der Text genau die Pfadlänge, wenn Blocksatz gewählt
    /// ist oder der Text länger als der Pfad ist (Handbuch Kap. 19: „attempts to fit all the letters without altering the
    /// baseline length … the letter spacing decreases“); die Lücken zwischen den Buchstaben werden dafür gleichmäßig
    /// gestreckt oder gestaucht (**eigene Festlegung** der Rechenweise).
    fn pfad_setzen(
        &self,
        objekte: Vec<Objekt>,
        zeile: &TextVorgabe,
        (pfad, anker, einpassen): (Pfad, f64, bool),
        winkel_standard: f64,
    ) -> Vec<Objekt> {
        let bereiche: Vec<Option<(f64, f64)>> = objekte.iter().map(x_bereich).collect();
        let b: Vec<(f64, f64)> = bereiche.iter().flatten().copied().collect();
        let l = pfad.laenge();
        let faktor = match (b.first(), b.last()) {
            (Some(e), Some(z)) if einpassen && b.len() > 1 => {
                let breite = z.1 - e.0;
                let summe_w: f64 = b.iter().map(|q| q.1 - q.0).sum();
                let summe_g = breite - summe_w;
                ((self.blocksatz || breite > l) && summe_g.abs() > 1e-9).then(|| (l - summe_w) / summe_g)
            }
            _ => None,
        };
        let d = self.stand_versatz(zeile.hoehe);
        let mut lauf = 0.0;
        let mut vorher: Option<f64> = None;
        objekte
            .into_iter()
            .zip(bereiche)
            .map(|(o, ber)| {
                let Some((x0, x1)) = ber else {
                    return o;
                };
                let xc = (x0 + x1) / 2.0;
                let s = match faktor {
                    Some(f) => {
                        if let Some(v) = vorher {
                            lauf += (x0 - v) * f;
                        }
                        let s = lauf + (x1 - x0) / 2.0;
                        lauf += x1 - x0;
                        vorher = Some(x1);
                        s
                    }
                    None => anker + xc - zeile.position.x,
                };
                let (ziel, richtung) = pfad.bei(s);
                starr(&o, xc, zeile.position.y + d, ziel, richtung, winkel_standard)
            })
            .collect()
    }
}

/// Senkrechte Grundlinie (Handbuch Kap. 19 „Creating vertical baselines“): Buchstaben untereinander, waagrecht auf
/// der Linie mittig („letters … are centered along vertical lines“), Buchstabenabstand senkrecht; die Ausrichtung
/// gilt entlang der Linie (links = beginnt oben bei `position`). Folgezeilen stehen links daneben („New lines are
/// placed … from right to left“), Abstand wie waagrecht Höhe × (1 + Zeilenabstand) – **eigene Festlegung**.
fn senkrecht_setzen(
    objekte: Vec<Objekt>,
    idx: &[usize],
    zeile: &TextVorgabe,
    spalte: Punkt,
    abstand: f64,
    winkel_standard: f64,
) -> Vec<Objekt> {
    let zeichen: Vec<char> = zeile.text.chars().collect();
    let wort = zeile.wortabstand.unwrap_or(WORTABSTAND_ANTEIL) * zeile.hoehe;
    let mut lage = vec![0.0; zeichen.len()];
    let (mut t, mut ende) = (0.0, 0.0);
    for (k, &c) in zeichen.iter().enumerate() {
        lage[k] = t;
        match c {
            '^' => {}
            ' ' => t += wort,
            _ => {
                t += zeile.hoehe;
                ende = t;
                t += abstand;
            }
        }
    }
    let oben = spalte.y + zeile.ausrichtung * ende;
    objekte
        .into_iter()
        .zip(idx)
        .map(|(o, &k)| {
            let Some((x0, x1)) = x_bereich(&o) else {
                return o;
            };
            let m = crate::transformation::Matrix {
                e: spalte.x - (x0 + x1) / 2.0,
                f: oben - lage[k] - zeile.hoehe - zeile.position.y,
                ..crate::transformation::Matrix::EINS
            };
            crate::transformation::abbilden(&o, &m, winkel_standard)
        })
        .collect()
}

/// Zusätze eines Texts: Farben für `^` und einzelne Buchstaben (Handbuch Kap. 18/19).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextZusatz {
    /// Farben nach dem 1., 2. … `^` im Text (davor gilt `farbe` des Texts) – im Original „the next color in the palette“
    pub farben: Vec<String>,
    pub einzeln: Vec<Buchstabe>,
    /// Stichart der Schrift (None = wie die Buchstaben des Alphabets, meist Satin)
    pub stichart: Option<SchriftStichart>,
    /// Elastic Lettering (None = ohne)
    pub huelle: Option<Huelle>,
}

/// Teilstücke für die Verformung durch die Hülle in mm – **eigene Festlegung**
const HUELLE_SEGMENT: f64 = 1.0;

/// Elastic Lettering (Handbuch Kap. 21 „Creating special effects with Elastic Lettering“, PDF-S. 189–190): eine Hülle
/// um den ganzen Text, deren Ober- und Unterkante an Griffen nach oben oder unten gezogen wird – **gerade** (Straight:
/// Griffe an den Ecken, „slant or pennant lettering“) oder **gebogen** (Curved: dazu ein Griff in der Mitte, „bulges
/// and arches“). Wirkt mit jeder Grundlinie. Jeder Punkt behält x und seinen Anteil zwischen Unter- und Oberkante.
/// Kantenform linear bzw. Parabel durch die drei Griffe: **eigene Festlegung** (Original nicht gemessen).
#[derive(Clone, Debug, PartialEq)]
pub struct Huelle {
    pub gebogen: bool,
    /// Verschiebung der Oberkante in mm (+ nach oben) links, Mitte, rechts
    pub oben: [f64; 3],
    /// Verschiebung der Unterkante links, Mitte, rechts
    pub unten: [f64; 3],
}

impl Huelle {
    fn kante(&self, w: &[f64; 3], u: f64) -> f64 {
        if self.gebogen {
            w[0] * 2.0 * (u - 0.5) * (u - 1.0) - w[1] * 4.0 * u * (u - 1.0) + w[2] * 2.0 * u * (u - 0.5)
        } else {
            w[0] + (w[2] - w[0]) * u
        }
    }

    /// Alle Objekte des Texts in der Hülle verformen (Bezug: umschließendes Rechteck des ganzen Texts).
    pub fn anwenden(&self, objekte: Vec<Objekt>) -> Vec<Objekt> {
        let mut pts = Vec::new();
        objekte
            .iter()
            .for_each(|o| crate::transformation::formpunkte(&o.art, &mut pts));
        let (x0, x1, y0, y1) = pts.iter().fold((f64::MAX, f64::MIN, f64::MAX, f64::MIN), |r, q| {
            (r.0.min(q.x), r.1.max(q.x), r.2.min(q.y), r.3.max(q.y))
        });
        if pts.is_empty() || x1 - x0 < 1e-9 || y1 - y0 < 1e-9 {
            return objekte;
        }
        let f = |q: Punkt| {
            let u = (q.x - x0) / (x1 - x0);
            let v = (q.y - y0) / (y1 - y0);
            let (ob, un) = (self.kante(&self.oben, u), self.kante(&self.unten, u));
            p(q.x, q.y + un + v * (ob - un))
        };
        objekte
            .iter()
            .map(|o| crate::transformation::verformen(o, &f, HUELLE_SEGMENT))
            .collect()
    }
}

/// Stichart „auto“ einer Schrift: wird ein Satinstich länger als dies (mm), stickt der ganze Text Step. **Eigene
/// Erweiterung** – das Original stickt Schrift als Satin und wechselt nicht selbst (Handbuch Kap. 20: Step „particularly
/// suitable for filling large, irregular lettering shapes“, von Hand). Grenze wie EasyStitch: Flächen bis 8,0 mm Breite
/// Satin, breitere Tatami (NORMAL.AMT Tabelle 0x3f54, `Analyse/code/10` §C, hoch); Handbuch S. 118: „Do not use Satin
/// fill for areas where the stitch length exceeds 7mm“.
pub const AUTO_STEP_AB_MM: f64 = 8.0;

/// Längster Satinstich (Abstand der gepaarten Punkte beider Seiten jeder Satinsäule, ohne Zugausgleich) in mm; Säulen,
/// die schon Step sind, zählen nicht.
pub fn laengster_satinstich(objekte: &[Objekt]) -> f64 {
    fn rek(a: &ObjektArt, m: &mut f64) {
        match a {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                optionen,
                ..
            } if !optionen.step => {
                if let Ok((x, y)) = crate::sticharten::satin::paare_bilden(seite_a, seite_b) {
                    for (p, q) in x.iter().zip(&y) {
                        *m = m.max((p.x - q.x).hypot(p.y - q.y));
                    }
                }
            }
            ObjektArt::Gruppe { teile, .. } => {
                for t in teile {
                    rek(&t.art, m);
                }
            }
            _ => {}
        }
    }
    let mut m = 0.0;
    for o in objekte {
        rek(&o.art, &mut m);
    }
    m
}

/// Satinsäulen auf Step stellen (wie Stichart „step“), z. B. nach der Entscheidung von „auto“.
pub fn auf_step(o: Objekt) -> Objekt {
    stichart_anwenden(o, &SchriftStichart::Step)
}

/// Computer-Schrift nach der Entscheidung von „auto“ auf Step: jeder Buchstabe als **eine** Tatami-Fläche mit Löchern
/// (Vereinigung seiner Säulen = Umriss des Zeichens), Stichwinkel des Stils, Step-Unterlage, wenn die Säulen eine
/// Unterlage hatten. **Eigene Festlegung** (21.11.2026): Step-Reihen in den Teilstücken der automatischen Zerlegung
/// (`saeulen.rs`) liegen an Spitzen und Abzweigungen kreuz und quer (Google Sans „A“ bei 25 mm); die Alphabete des
/// Originals bleiben bei Step je Säule.
pub fn auf_tatami(o: Objekt) -> Objekt {
    let Objekt { farbe, name, art } = o;
    let ObjektArt::Gruppe { teile, umrisse, laufweg } = art else {
        return Objekt { farbe, name, art };
    };
    let unterlage = teile.iter().any(|t| {
        matches!(&t.art, ObjektArt::Satin { unterlage, .. } if *unterlage != Unterlage::Arten(Vec::new()))
    });
    let (teile, umrisse) = flaechen_fuellen(teile, umrisse, &farbe, &name, &|umriss, loecher| ObjektArt::Tatami {
        umriss,
        loecher,
        winkel: None,
        unterlage: Some(unterlage),
        aussparen: Default::default(),
        welle: None,
    });
    Objekt { farbe, name, art: ObjektArt::Gruppe { teile, umrisse, laufweg } }
}

/// Säulen und Flächen eines Buchstabens zu Flächen vereinigen und mit `art` füllen; andere Teile (Läufe) bleiben dahinter.
fn flaechen_fuellen(
    teile: Vec<Objekt>,
    umrisse: Vec<Linie>,
    farbe: &str,
    name: &str,
    art: &dyn Fn(Linie, Vec<Linie>) -> ObjektArt,
) -> (Vec<Objekt>, Vec<Linie>) {
    let mut flaechen = Vec::new();
    let (mut neu, mut neu_umrisse) = (Vec::new(), Vec::new());
    for (i, t) in teile.into_iter().enumerate() {
        match &t.art {
            ObjektArt::Satin { .. } => {
                if let Some(u) = umrisse.get(i).filter(|u| u.len() >= 3) {
                    flaechen.push(crate::aussparen::ring_kontur(u));
                } else {
                    flaechen.extend(crate::aussparen::bedeckt(&t.art, 0.0));
                }
            }
            ObjektArt::Tatami { .. } | ObjektArt::Musterfuellung { .. } => {
                flaechen.extend(crate::aussparen::bedeckt(&t.art, 0.0));
            }
            _ => {
                neu_umrisse.push(umrisse.get(i).cloned().unwrap_or_default());
                neu.push(t);
            }
        }
    }
    let mut teile: Vec<Objekt> = Vec::new();
    let mut ringe: Vec<Linie> = Vec::new();
    for (umriss, loecher) in crate::aussparen::vereinigen(&flaechen) {
        ringe.push(umriss.clone());
        teile.push(Objekt { farbe: farbe.to_string(), name: name.to_string(), art: art(umriss, loecher) });
    }
    teile.extend(neu);
    ringe.extend(neu_umrisse);
    (teile, ringe)
}

/// Stichart einer Schrift (Handbuch Kap. 20 „Applying different stitch types to lettering objects“: Standard Satin,
/// auch Step oder Fancy).
#[derive(Clone, Debug, PartialEq)]
pub enum SchriftStichart {
    /// automatisch (`AUTO_STEP_AB_MM`): entschieden für den ganzen Text nach Größe und Transformation
    /// (`Design::schrift_auto`); beim Setzen der Buchstaben bleibt Satin
    Auto,
    /// Step: die Satinsäulen werden mit Step-Reihen gefüllt, die den Stichwinkeln folgen (`step_reihen`)
    Step,
    /// Fancy: Füllmuster (`muster.rs`) über die ganze Fläche jedes Buchstabens; `winkel` = Drehung des Musters
    /// („Adjust the angle of the stitches by entering a rotation angle“). Fläche = Vereinigung der Säulen –
    /// **eigene Festlegung** (nicht gemessen).
    Muster {
        muster: String,
        winkel: Option<f64>,
        groesse: f64,
    },
}

/// Stichart auf einen Buchstaben (Gruppe) anwenden.
fn stichart_anwenden(o: Objekt, st: &SchriftStichart) -> Objekt {
    let Objekt { farbe, name, art } = o;
    let ObjektArt::Gruppe {
        teile,
        umrisse,
        laufweg,
    } = art
    else {
        return Objekt { farbe, name, art };
    };
    let (teile, umrisse) = match st {
        SchriftStichart::Auto => (teile, umrisse),
        SchriftStichart::Step => (
            teile
                .into_iter()
                .map(|mut t| {
                    if let ObjektArt::Satin { optionen, .. } = &mut t.art {
                        optionen.step = true;
                    }
                    t
                })
                .collect(),
            umrisse,
        ),
        SchriftStichart::Muster {
            muster,
            winkel,
            groesse,
        } => flaechen_fuellen(teile, umrisse, &farbe, &name, &|umriss, loecher| ObjektArt::Musterfuellung {
            umriss,
            loecher,
            muster: muster.clone(),
            winkel: *winkel,
            groesse: *groesse,
            unterlage: None,
            aussparen: Default::default(),
            welle: None,
        }),
    };
    Objekt {
        farbe,
        name,
        art: ObjektArt::Gruppe {
            teile,
            umrisse,
            laufweg,
        },
    }
}

/// Ein einzelner Buchstabe (Reshape Object: verschieben/umformen; „Recoloring letters“).
#[derive(Clone, Debug, PartialEq)]
pub struct Buchstabe {
    /// 1 = erster Buchstabe des Texts (Leerzeichen, Zeilenumbrüche und `^` zählen nicht)
    pub nr: usize,
    pub farbe: Option<String>,
    /// um die Mitte des Buchstabens, nach dem Legen auf die Grundlinie
    pub transform: crate::transformation::Transformation,
    /// neue Winkellinien (Strecken in mm, Lage wie der fertig gesetzte Text vor `transform` des Objekts)
    pub winkellinien: Vec<[Punkt; 2]>,
}

/// Schnitt der Strecke `l` mit der Linie `zug`: Bogenlänge auf `zug` beim ersten Schnittpunkt.
fn schnitt_bogen(zug: &[Punkt], l: &[Punkt; 2]) -> Option<f64> {
    let mut s0 = 0.0;
    for w in zug.windows(2) {
        let (a, b, c, d) = (w[0], w[1], l[0], l[1]);
        let r = (b.x - a.x, b.y - a.y);
        let q = (d.x - c.x, d.y - c.y);
        let nenner = r.0 * q.1 - r.1 * q.0;
        let len = r.0.hypot(r.1);
        if nenner.abs() > 1e-12 {
            let t = ((c.x - a.x) * q.1 - (c.y - a.y) * q.0) / nenner;
            let u = ((c.x - a.x) * r.1 - (c.y - a.y) * r.0) / nenner;
            if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                return Some(s0 + t * len);
            }
        }
        s0 += len;
    }
    None
}

/// Satinsäule mit neuen Winkellinien: Anfang und Ende bleiben, dazwischen bestimmen nur die Linien, die beide Seiten
/// schneiden, die Stichrichtung; zwischen zwei Linien werden beide Seiten nach relativer Bogenlänge zugeordnet (wie
/// `saeule_seiten`, 1 Punkt je mm). None, wenn keine Linie beide Seiten trifft oder sich Linien kreuzen.
fn saeule_mit_linien(a: &Linie, b: &Linie, linien: &[[Punkt; 2]]) -> Option<(Linie, Linie)> {
    use crate::geometrie::{kumuliert, punkt_bei};
    if a.len() < 2 || b.len() < 2 {
        return None;
    }
    let (ka, kb) = (kumuliert(a), kumuliert(b));
    let (la, lb) = (ka[ka.len() - 1], kb[kb.len() - 1]);
    let mut fest: Vec<(f64, f64)> = linien
        .iter()
        .filter_map(|l| Some((schnitt_bogen(a, l)?, schnitt_bogen(b, l)?)))
        .collect();
    if fest.is_empty() {
        return None;
    }
    fest.push((0.0, 0.0));
    fest.push((la, lb));
    fest.sort_by(|x, y| x.0.total_cmp(&y.0));
    // „Make sure that the stitch angles do not intersect each other“ (Handbuch)
    if fest.windows(2).any(|w| w[1].1 < w[0].1) {
        return None;
    }
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for (k, w) in fest.windows(2).enumerate() {
        let teile = ((w[1].0 - w[0].0).max(w[1].1 - w[0].1).ceil() as usize).max(1);
        let letzte = k == fest.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(punkt_bei(a, w[0].0 + t * (w[1].0 - w[0].0), &ka));
            sb.push(punkt_bei(b, w[0].1 + t * (w[1].1 - w[0].1), &kb));
        }
    }
    Some((sa, sb))
}

/// Stichwinkel eines Buchstabens neu setzen (Handbuch Kap. 20 „Adjusting stitch angles of individual letters“:
/// „Digitize stitch angles so that they intersect two sides of the object … Stitches are regenerated with the new
/// angles“). Satinsäulen: `saeule_mit_linien` – die inneren Winkellinien des Alphabets entfallen, Anfang und Ende
/// bleiben (Zuordnung eigene Festlegung). Füllungen (Step/Muster) nehmen die Richtung der ersten Linie, die ihren
/// Umriss schneidet.
fn winkel_setzen(o: &mut Objekt, linien: &[[Punkt; 2]]) {
    let ObjektArt::Gruppe { teile, .. } = &mut o.art else {
        return;
    };
    for t in teile {
        match &mut t.art {
            ObjektArt::Satin { seite_a, seite_b, .. } => {
                if let Some((a, b)) = saeule_mit_linien(seite_a, seite_b, linien) {
                    (*seite_a, *seite_b) = (a, b);
                }
            }
            ObjektArt::Tatami { umriss, winkel, .. } | ObjektArt::Musterfuellung { umriss, winkel, .. } => {
                let mut ring = umriss.clone();
                ring.push(umriss[0]);
                if let Some(l) = linien.iter().find(|l| schnitt_bogen(&ring, l).is_some()) {
                    *winkel = Some((l[1].y - l[0].y).atan2(l[1].x - l[0].x).to_degrees());
                }
            }
            _ => {}
        }
    }
}

/// Anzahl der Buchstaben eines Texts (ohne Leerraum und `^`).
pub fn buchstaben_zahl(text: &str) -> usize {
    text.chars().filter(|&c| !c.is_whitespace() && c != '^').count()
}

fn faerben(o: &mut Objekt, farbe: &str) {
    o.farbe = farbe.to_string();
    if let ObjektArt::Gruppe { teile, .. } = &mut o.art {
        for t in teile {
            t.farbe = farbe.to_string();
        }
    }
}

/// Ganzen Text setzen: Zeilen, Kursiv, Grundlinie, Farbwechsel `^` und einzelne Buchstaben.
/// `satz` setzt eine Zeile gerade und liefert je Buchstabe ein Objekt samt Zeichenindex in der Zeile;
/// `abstand` = Buchstabenabstand in mm (für die senkrechte Grundlinie).
pub fn text_setzen(
    v: &TextVorgabe,
    abstand: f64,
    zusatz: Option<&TextZusatz>,
    winkel_standard: f64,
    satz: impl Fn(&TextVorgabe) -> (Vec<Objekt>, Vec<usize>, Vec<char>),
) -> (Vec<Objekt>, Vec<char>) {
    let (o, f, _) = text_setzen_nr(v, abstand, zusatz, winkel_standard, satz);
    (o, f)
}

/// Wie `text_setzen`, dazu je Objekt die Nummer seines Buchstabens (1 = erster, ohne Leerraum und `^`, wie
/// `buchstaben[].nr`) – für „einzelne Buchstaben“ in der Oberfläche (W9).
pub fn text_setzen_nr(
    v: &TextVorgabe,
    abstand: f64,
    zusatz: Option<&TextZusatz>,
    winkel_standard: f64,
    satz: impl Fn(&TextVorgabe) -> (Vec<Objekt>, Vec<usize>, Vec<char>),
) -> (Vec<Objekt>, Vec<char>, Vec<usize>) {
    let mut nummern = Vec::new();
    let mitte = v.position;
    let schritt = v.hoehe * (1.0 + v.zeilenabstand.unwrap_or(ZEILENABSTAND_ANTEIL));
    let g = v.grundlinie.as_ref();
    let mut out = Vec::new();
    let mut fehlend = Vec::new();
    let (mut nr, mut farbe) = (0usize, 0usize);
    for (i, z) in v.zeilen().into_iter().enumerate() {
        // mehrfach (Multiple): Zeile 1 im Uhrzeiger oben, Zeile 2 waagrecht mittig durch die Kreismitte, Zeile 3 unten
        let z = match g.map(|g| g.art.as_str()) {
            Some("mehrfach") if i == 1 => TextVorgabe {
                position: p(mitte.x, mitte.y - v.hoehe / 2.0),
                ausrichtung: 0.5,
                ..z
            },
            Some("mehrfach") => TextVorgabe { position: mitte, ..z },
            _ => z,
        };
        let (obj, idx, f) = satz(&z);
        fehlend.extend(f);
        let obj = z.neigen(obj, winkel_standard);
        let obj = match g {
            None => obj,
            Some(g) => match (g.art.as_str(), i) {
                ("senkrecht", _) => senkrecht_setzen(
                    obj,
                    &idx,
                    &z,
                    p(mitte.x - i as f64 * schritt, mitte.y),
                    abstand,
                    winkel_standard,
                ),
                ("mehrfach", 0 | 2) => Grundlinie {
                    lage: Some(if i == 0 { "oben" } else { "unten" }.into()),
                    winkel: None,
                    stand: None,
                    ..g.clone()
                }
                .anwenden(obj, &z, mitte, winkel_standard),
                ("mehrfach", _) => obj,
                _ => match g.pfad(&z, mitte) {
                    Some(pf) => g.pfad_setzen(obj, &z, pf, winkel_standard),
                    None if g.art == "kreis" => g.anwenden(obj, &z, mitte, winkel_standard),
                    None if g.art == "gerade" => {
                        let (sn, cs) = g.winkel.unwrap_or(0.0).to_radians().sin_cos();
                        let m = crate::transformation::Matrix {
                            a: cs,
                            b: -sn,
                            c: sn,
                            d: cs,
                            e: mitte.x - (cs * mitte.x - sn * mitte.y),
                            f: mitte.y - (sn * mitte.x + cs * mitte.y),
                        };
                        obj.iter()
                            .map(|o| crate::transformation::abbilden(o, &m, winkel_standard))
                            .collect()
                    }
                    None => obj,
                },
            },
        };
        // Buchstabennummer und Farbe je Zeichen der Zeile
        let zeichen: Vec<char> = z.text.chars().collect();
        let mut je = vec![(0usize, 0usize); zeichen.len()];
        for (k, &c) in zeichen.iter().enumerate() {
            if c == '^' {
                farbe += 1;
            } else if !c.is_whitespace() {
                nr += 1;
            }
            je[k] = (nr, farbe);
        }
        for (mut o, &k) in obj.into_iter().zip(&idx) {
            let (n, fa) = je[k];
            if let Some(zs) = zusatz {
                if fa > 0
                    && let Some(f) = zs.farben.get(fa - 1).or(zs.farben.last())
                {
                    faerben(&mut o, f);
                }
                for b in zs.einzeln.iter().filter(|b| b.nr == n) {
                    if let Some(f) = &b.farbe {
                        faerben(&mut o, f);
                    }
                    if !b.transform.ist_leer()
                        && let Ok(m) = b
                            .transform
                            .matrix(crate::transformation::mitte(std::slice::from_ref(&o)))
                    {
                        o = crate::transformation::abbilden(&o, &m, winkel_standard);
                    }
                }
                if let Some(st) = &zs.stichart {
                    o = stichart_anwenden(o, st);
                }
                for b in zs.einzeln.iter().filter(|b| b.nr == n && !b.winkellinien.is_empty()) {
                    winkel_setzen(&mut o, &b.winkellinien);
                }
            }
            out.push(o);
            nummern.push(n);
        }
    }
    if let Some(h) = zusatz.and_then(|z| z.huelle.as_ref()) {
        out = h.anwenden(out);
    }
    (out, fehlend, nummern)
}

impl TextVorgabe {
    /// Text in Zeilen (Zeilenumbruch `\n`): jede Zeile eine eigene Vorgabe, Grundlinie je Zeile um
    /// Höhe × (1 + Zeilenabstand) nach unten; Ausrichtung gilt je Zeile.
    pub fn zeilen(&self) -> Vec<TextVorgabe> {
        let schritt = self.hoehe * (1.0 + self.zeilenabstand.unwrap_or(ZEILENABSTAND_ANTEIL));
        self.text
            .split('\n')
            .enumerate()
            .map(|(i, t)| TextVorgabe {
                text: t.trim_end_matches('\r').to_string(),
                position: p(self.position.x, self.position.y - i as f64 * schritt),
                ..self.clone()
            })
            .collect()
    }

    /// Kursiv: Buchstaben der (gerade gesetzten) Zeile um ihre Grundlinie neigen.
    pub fn neigen(&self, objekte: Vec<Objekt>, winkel_standard: f64) -> Vec<Objekt> {
        if self.kursiv == 0.0 {
            return objekte;
        }
        let k = self.kursiv.to_radians().tan();
        let m = crate::transformation::Matrix {
            a: 1.0,
            b: k,
            c: 0.0,
            d: 1.0,
            e: -k * self.position.y,
            f: 0.0,
        };
        objekte
            .iter()
            .map(|o| crate::transformation::abbilden(o, &m, winkel_standard))
            .collect()
    }

    fn wort(&self) -> f64 {
        self.wortabstand.unwrap_or(WORTABSTAND_ANTEIL) * self.hoehe * self.breite_prozent / 100.0
    }
}

/// Unterlage einer Säule: Vorgabe des Texts; sonst die Unterlage aus dem Alphabet; sonst die des Stils „Schrift“.
fn unterlage_fuer(o: &ZeichenObjekt, v: &TextVorgabe) -> Unterlage {
    match &v.unterlage {
        Some(l) => Unterlage::Arten(l.clone()),
        None if !o.unterlagen.is_empty() => Unterlage::Arten(o.unterlagen.clone()),
        None => Unterlage::Arten(vec![SCHRIFT_UNTERLAGE.into()]),
    }
}

fn satin_optionen(abstand: Option<f64>, auto_spacing: bool, faktor: f64) -> SatinOptionen {
    SatinOptionen {
        abstand: if auto_spacing { None } else { abstand },
        faktor: Some(faktor),
        kurzstiche: Some(true),
        step: false,
        zickzack: false,
        zugausgleich: None,
    }
}

/// Objekte eines Zeichens (Alphabet oder Motiv) über die Abbildung `abb` (Alphabet-Einheiten → mm) in Teile des
/// Rechenkerns umsetzen: Teile, Umrisse der Teile (für Laufwege, leer bei Läufen) und Stichlänge des Laufwegs.
pub(crate) fn zeichen_teile(
    z: &Zeichen,
    abb: &dyn Fn((f64, f64)) -> Punkt,
    unterlage: &dyn Fn(&ZeichenObjekt) -> Unterlage,
    faktor: f64,
    farbe: &str,
    name: &str,
    ecken_an_knoten: bool,
) -> (Vec<Objekt>, Vec<Linie>, f64) {
    let abb_linie = |l: &[(f64, f64)]| -> Linie { l.iter().map(|&q| abb(q)).collect() };
    let mut teile: Vec<Objekt> = Vec::new();
    let mut umrisse: Vec<Linie> = Vec::new();
    let mut laufweg = SCHRIFT_LAUFWEG;
    for o in &z.objekte {
        if let Some(l) = o.laufweg {
            laufweg = l;
        }
        let art = match (&o.stichart, o.klasse) {
            (Stichart::Satin { abstand, auto_spacing }, 0x06) => {
                let Some(u) = o.umrisse.first() else { continue };
                let Some((a, b)) = saeule_seiten(u, &o.winkel) else {
                    continue;
                };
                umrisse.push(abb_linie(&umriss_punkte(u)));
                ObjektArt::Satin {
                    seite_a: abb_linie(&a),
                    seite_b: abb_linie(&b),
                    unterlage: unterlage(o),
                    optionen: satin_optionen(*abstand, *auto_spacing, faktor),
                }
            }
            (Stichart::Satin { abstand, auto_spacing }, 0x0c) if o.umrisse.len() >= 2 => {
                let (a, b) = (umriss_punkte(&o.umrisse[0]), umriss_punkte(&o.umrisse[1]));
                umrisse.push(abb_linie(&a.iter().chain(b.iter().rev()).copied().collect::<Vec<_>>()));
                ObjektArt::Satin {
                    seite_a: abb_linie(&a),
                    seite_b: abb_linie(&b),
                    unterlage: unterlage(o),
                    optionen: satin_optionen(*abstand, *auto_spacing, faktor),
                }
            }
            (Stichart::Tatami, _) => {
                let Some(u) = o.umrisse.first() else { continue };
                umrisse.push(abb_linie(&umriss_punkte(u)));
                ObjektArt::Tatami {
                    umriss: abb_linie(&umriss_punkte(u)),
                    loecher: o.umrisse[1..].iter().map(|l| abb_linie(&umriss_punkte(l))).collect(),
                    winkel: None,
                    unterlage: Some(!o.unterlagen.is_empty()),
                    aussparen: Default::default(),
                    welle: None,
                }
            }
            (Stichart::Lauf { laenge }, _) => {
                let Some(u) = o.umrisse.first() else { continue };
                umrisse.push(Vec::new());
                let (roh, ecken) = if ecken_an_knoten {
                    lauf_punkte(u)
                } else {
                    let r = umriss_punkte(u);
                    let n = r.len();
                    (r, vec![false; n])
                };
                ObjektArt::Lauf {
                    punkte: abb_linie(&roh),
                    ecken,
                    laenge: *laenge,
                    dreifach: false,
                    geschlossen: u.geschlossen,
                }
            }
            _ => continue,
        };
        teile.push(Objekt {
            farbe: farbe.to_string(),
            name: name.to_string(),
            art,
        });
    }
    (teile, umrisse, laufweg)
}

/// Text in Stickobjekte des Rechenkerns umsetzen. Liefert die Objekte und die Zeichen ohne Entsprechung im Alphabet.
pub fn text_objekte(alpha: &Alphabet, v: &TextVorgabe) -> (Vec<Objekt>, Vec<char>) {
    let (o, _, f) = text_objekte_idx(alpha, v);
    (o, f)
}

/// Wie `text_objekte`, dazu je Objekt (Buchstabe) der Zeichenindex im Text. `^` ist ein Farbwechsel (Handbuch
/// Kap. 18: „insert a color change between two letters by keying a caret (^)“) und belegt keinen Platz.
pub fn text_objekte_idx(alpha: &Alphabet, v: &TextVorgabe) -> (Vec<Objekt>, Vec<usize>, Vec<char>) {
    let s = v.hoehe / REFERENZ_HOEHE_MM / EINHEIT; // Alphabet-Einheit → mm
    let sx = s * v.breite_prozent / 100.0;
    let abstand = v.abstand.unwrap_or(alpha.abstand_anteil * v.hoehe);
    let vorschub = |ch: char| -> Option<f64> {
        if ch == ' ' {
            return Some(v.wort());
        }
        if ch == '^' {
            return None;
        }
        alpha
            .zeichen
            .get(&ch)
            .map(|z| (z.grundlinie[2] - z.grundlinie[0]) * sx + abstand * v.breite_prozent / 100.0)
    };
    let breite: f64 = v.text.chars().filter_map(vorschub).sum::<f64>()
        - if v.text.chars().rfind(|&c| c != '^').is_some_and(|c| c != ' ') {
            abstand * v.breite_prozent / 100.0
        } else {
            0.0
        };
    let mut x = v.position.x - v.ausrichtung * breite.max(0.0);
    let mut objekte = Vec::new();
    let mut idx = Vec::new();
    let mut fehlend = Vec::new();
    for (k, ch) in v.text.chars().enumerate() {
        if ch == ' ' {
            x += v.wort();
            continue;
        }
        if ch == '^' {
            continue;
        }
        let Some(z) = alpha.zeichen.get(&ch) else {
            fehlend.push(ch);
            continue;
        };
        let [gx, gy, gx2, _] = z.grundlinie;
        let abb = |q: (f64, f64)| p(x + (q.0 - gx) * sx, v.position.y + (q.1 - gy) * s);
        let (teile, umrisse, laufweg) = zeichen_teile(
            z,
            &abb,
            &|o| unterlage_fuer(o, v),
            SCHRIFT_FAKTOR,
            &v.farbe,
            &format!("{} {}", alpha.name, ch),
            false,
        );
        if !teile.is_empty() {
            objekte.push(Objekt {
                farbe: v.farbe.clone(),
                name: format!("{} {}", alpha.name, ch),
                art: ObjektArt::Gruppe {
                    teile,
                    umrisse,
                    laufweg,
                },
            });
            idx.push(k);
        }
        x += (gx2 - gx) * sx + abstand * v.breite_prozent / 100.0;
    }
    (objekte, idx, fehlend)
}

// ---------------------------------------------------------------- empfohlene Höhen

/// Empfohlene Buchstabenhöhen (min, max in mm) der Original-Alphabete: Handbuch „Alphabet Samples“ (PDF-S. 380 ff.,
/// `Analyse/daten/alphabets.md`, Sicherheit hoch). Schlüssel: Name klein ohne Sonderzeichen.
const EMPFOHLENE_HOEHEN: [(&str, f64, f64); 49] = [
    ("alice", 8.0, 50.0),
    ("anniversary", 13.0, 75.0),
    ("bamboo", 8.0, 40.0),
    ("blackboard", 8.0, 50.0),
    ("bordercaps", 13.0, 38.0),
    ("castle", 10.0, 40.0),
    ("chicago", 6.0, 75.0),
    ("childsplay", 13.0, 30.0),
    ("christopher", 8.0, 30.0),
    ("craft", 10.0, 50.0),
    ("creativecross", 30.0, 100.0),
    ("cursive", 7.0, 60.0),
    ("diamond2", 18.0, 100.0),
    ("diamond3", 18.0, 100.0),
    ("drifter", 6.0, 40.0),
    ("easyappliqu", 30.0, 100.0),
    ("empress", 13.0, 60.0),
    ("fortress", 6.0, 40.0),
    ("greek2", 8.0, 50.0),
    ("greekionic", 10.0, 50.0),
    ("guinevere", 13.0, 50.0),
    ("hebrew2", 8.0, 40.0),
    ("hebrewdavid", 7.0, 40.0),
    ("hiragana", 13.0, 50.0),
    ("island", 8.0, 50.0),
    ("jikharev", 8.0, 50.0),
    ("katakana", 13.0, 50.0),
    ("ketchikan", 8.0, 30.0),
    ("kingcharles", 8.0, 50.0),
    ("lisa", 13.0, 75.0),
    ("london", 8.0, 50.0),
    ("medley", 7.0, 40.0),
    ("neon", 18.0, 75.0),
    ("nightowl", 6.0, 50.0),
    ("oldeenglish", 13.0, 50.0),
    ("prestige", 5.0, 50.0),
    ("quiltingtext", 4.0, 6.0),
    ("romansmall", 4.0, 6.0),
    ("roundedblock", 8.0, 45.0),
    ("russiantextbook", 8.0, 45.0),
    ("scanner", 7.0, 40.0),
    ("secretgarden", 38.0, 90.0),
    ("swissblock", 6.0, 60.0),
    ("tabloid", 30.0, 75.0),
    ("technicalblock", 8.0, 50.0),
    ("tiffany", 7.0, 35.0),
    ("varsityapplique", 30.0, 75.0),
    ("varsityblock", 6.0, 35.0),
    ("victoria", 10.0, 50.0),
];

/// Empfohlene Höhe (min, max) in mm für ein Alphabet (Name wie im Design-Feld `alphabet`) – Handbuch.
pub fn empfohlene_hoehe(name: &str) -> Option<(f64, f64)> {
    let k: String = name
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();
    let finde = |s: &str| {
        EMPFOHLENE_HOEHEN
            .iter()
            .find(|(n, _, _)| *n == s)
            .map(|&(_, a, b)| (a, b))
    };
    finde(&k)
        .or_else(|| k.strip_suffix("caps").and_then(finde))
        .or_else(|| finde(&format!("{k}caps")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn knoten(x: f64, y: f64) -> Knoten {
        Knoten {
            ein: None,
            punkt: (x, y),
            aus: None,
        }
    }

    /// Rechteckige Säule 0…10 mm × 0…2 mm wie im Alphabet: Umriss 4 Knoten, Winkellinien 0↔3 und 1↔2.
    fn saeule() -> (Umriss, Vec<Winkellinie>) {
        let e = EINHEIT;
        let u = Umriss {
            geschlossen: true,
            knoten: vec![
                knoten(0.0, 0.0),
                knoten(10.0 * e, 0.0),
                knoten(10.0 * e, 2.0 * e),
                knoten(0.0, 2.0 * e),
            ],
        };
        // Segment 0 (Knoten 0→1) bei 0 ↔ Segment 2 (Knoten 2→3) bei 1 = Knoten 3; Knoten 1 ↔ Knoten 2
        let w = vec![
            Winkellinie {
                a: 0,
                x: 0.0,
                c: 2,
                y: 1.0,
            },
            Winkellinie {
                a: 1,
                x: 0.0,
                c: 2,
                y: 0.0,
            },
        ];
        (u, w)
    }

    #[test]
    fn seiten_aus_winkellinien() {
        let (u, w) = saeule();
        let (a, b) = saeule_seiten(&u, &w).unwrap();
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), 11); // 10 mm → 10 Teile + Endpunkt
        assert!((a[0].0).abs() < 1e-9 && (a[0].1).abs() < 1e-9);
        assert!((b[0].1 - 2.0 * EINHEIT).abs() < 1e-9 && b[0].0.abs() < 1e-9);
        assert!((a[10].0 - 10.0 * EINHEIT).abs() < 1e-6);
        assert!((b[10].0 - 10.0 * EINHEIT).abs() < 1e-6 && (b[10].1 - 2.0 * EINHEIT).abs() < 1e-6);
    }

    #[test]
    fn text_setzen_massstab_und_vorschub() {
        let (u, w) = saeule();
        let z = Zeichen {
            zeichen: 'I',
            grundlinie: [0.0, 0.0, 10.0 * EINHEIT, 0.0],
            objekte: vec![ZeichenObjekt {
                klasse: 0x06,
                umrisse: vec![u],
                winkel: w,
                stichart: Stichart::Satin {
                    abstand: Some(0.4),
                    auto_spacing: true,
                },
                unterlagen: vec![],
                laufweg: None,
            }],
        };
        let alpha = Alphabet {
            name: "Test".into(),
            abstand_anteil: 0.1,
            zeichen: [('I', z)].into_iter().collect(),
        };
        let v = TextVorgabe {
            text: "II?".into(),
            hoehe: 25.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(-10.0, 0.0),
            ausrichtung: 0.0,
            farbe: "#000000".into(),
            unterlage: None,
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
            kursiv: 0.0,
        };
        let (obj, fehlend) = text_objekte(&alpha, &v);
        assert_eq!(fehlend, vec!['?']);
        assert_eq!(obj.len(), 2);
        // Stil Schrift: Auto Spacing × 0,9, Kurzstiche, Zickzack-Unterlage
        let (_, u, o) = saeule_aus(&obj[0]);
        assert_eq!(u, Unterlage::Arten(vec!["zickzack".into()]));
        assert_eq!(
            o,
            SatinOptionen {
                abstand: None,
                faktor: Some(0.9),
                kurzstiche: Some(true),
                step: false,
                zickzack: false,
                zugausgleich: None,
            }
        );
        // Maßstab 25/50 = 0,5: Säule 5 mm lang; zweites I beginnt bei −10 + 5 + 2,5 (10 % von 25 mm)
        let (seite_a, _, _) = saeule_aus(&obj[1]);
        assert!((seite_a[0].x - (-2.5)).abs() < 1e-9, "{}", seite_a[0].x);
        assert!((seite_a.last().unwrap().x - 2.5).abs() < 1e-6);
        // mittig: Breite 5 + 2,5 + 5 = 12,5 → Beginn bei −6,25
        let (obj, _) = text_objekte(
            &alpha,
            &TextVorgabe {
                text: "II".into(),
                position: p(0.0, 0.0),
                ausrichtung: 0.5,
                ..v
            },
        );
        let (seite_a, _, _) = saeule_aus(&obj[0]);
        assert!((seite_a[0].x + 6.25).abs() < 1e-9, "{}", seite_a[0].x);
    }

    /// erste Säule eines Buchstabens (Gruppe)
    fn saeule_aus(o: &Objekt) -> (Linie, Unterlage, SatinOptionen) {
        let ObjektArt::Gruppe { teile, .. } = &o.art else {
            panic!("Buchstabe = Gruppe")
        };
        let ObjektArt::Satin {
            seite_a,
            unterlage,
            optionen,
            ..
        } = &teile[0].art
        else {
            panic!("Säule")
        };
        (seite_a.clone(), unterlage.clone(), optionen.clone())
    }

    /// Zeilen: Grundlinie je Zeile um Höhe × (1 + 0,6) tiefer (Vorlage 0x3f2d), Wortabstand 0,25 × Höhe (0x3e85)
    #[test]
    fn zeilen_und_wortabstand() {
        let v = TextVorgabe {
            text: "Hallo\r\nWelt\nzwei".into(),
            hoehe: 10.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(5.0, 20.0),
            ausrichtung: 0.5,
            unterlage: None,
            farbe: "#000000".into(),
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
            kursiv: 0.0,
        };
        let z = v.zeilen();
        assert_eq!(
            z.iter().map(|x| x.text.as_str()).collect::<Vec<_>>(),
            ["Hallo", "Welt", "zwei"]
        );
        assert_eq!(z[1].position, p(5.0, 4.0));
        assert!((z[2].position.y - (20.0 - 32.0)).abs() < 1e-9);
        assert!((v.wort() - 2.5).abs() < 1e-9);
        let eng = TextVorgabe {
            zeilenabstand: Some(0.0),
            wortabstand: Some(0.5),
            ..v
        };
        assert_eq!(eng.zeilen()[1].position, p(5.0, 10.0));
        assert!((eng.wort() - 5.0).abs() < 1e-9);
    }

    /// Kursiv 45°: ein Punkt 5 mm über der Grundlinie wandert 5 mm nach rechts, die Grundlinie bleibt
    #[test]
    fn kursiv_um_die_grundlinie() {
        let o = Objekt {
            farbe: "#000000".into(),
            name: "I".into(),
            art: ObjektArt::Lauf {
                punkte: vec![p(0.0, 2.0), p(0.0, 7.0)],
                ecken: vec![true, true],
                laenge: None,
                dreifach: false,
                geschlossen: false,
            },
        };
        let v = TextVorgabe {
            text: "I".into(),
            hoehe: 5.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(0.0, 2.0),
            ausrichtung: 0.0,
            unterlage: None,
            farbe: "#000000".into(),
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
            kursiv: 45.0,
        };
        let r = v.neigen(vec![o], 45.0);
        let ObjektArt::Lauf { punkte, .. } = &r[0].art else {
            panic!()
        };
        assert!(
            punkte[0].x.abs() < 1e-9 && (punkte[1].x - 5.0).abs() < 1e-9,
            "{punkte:?}"
        );
    }

    /// Ersatz für ein Alphabet: jeder Buchstabe ein Rechteck 5 mm × Höhe, Vorschub 6 mm (auch Leerzeichen)
    fn kaesten(z: &TextVorgabe) -> (Vec<Objekt>, Vec<usize>, Vec<char>) {
        let n = z.text.chars().filter(|&c| c != '^').count() as f64;
        let x0 = z.position.x - z.ausrichtung * (n * 6.0 - 1.0);
        let (y, h) = (z.position.y, z.hoehe);
        let (mut o, mut idx, mut i) = (Vec::new(), Vec::new(), 0.0);
        for (k, c) in z.text.chars().enumerate() {
            if c == '^' {
                continue;
            }
            let x = x0 + i * 6.0;
            i += 1.0;
            if c == ' ' {
                continue;
            }
            let r = vec![p(x, y), p(x + 5.0, y), p(x + 5.0, y + h), p(x, y + h)];
            let teil = Objekt {
                farbe: z.farbe.clone(),
                name: c.to_string(),
                art: ObjektArt::Tatami {
                    umriss: r.clone(),
                    loecher: vec![],
                    winkel: None,
                    unterlage: None,
                    aussparen: Default::default(),
                    welle: None,
                },
            };
            o.push(Objekt {
                farbe: z.farbe.clone(),
                name: c.to_string(),
                art: ObjektArt::Gruppe {
                    teile: vec![teil],
                    umrisse: vec![r],
                    laufweg: 3.0,
                },
            });
            idx.push(k);
        }
        (o, idx, vec![])
    }

    fn satz_vorgabe(text: &str, g: Option<Grundlinie>) -> TextVorgabe {
        TextVorgabe {
            text: text.into(),
            hoehe: 5.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(0.0, 0.0),
            ausrichtung: 0.0,
            unterlage: None,
            farbe: "#000000".into(),
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: g,
            kursiv: 0.0,
        }
    }

    /// [x0, y0, x1, y1]
    fn huelle(o: &Objekt) -> [f64; 4] {
        let mut pts = Vec::new();
        crate::transformation::formpunkte(&o.art, &mut pts);
        pts.iter().fold([f64::MAX, f64::MAX, f64::MIN, f64::MIN], |r, q| {
            [r[0].min(q.x), r[1].min(q.y), r[2].max(q.x), r[3].max(q.y)]
        })
    }

    fn gleich(a: [f64; 4], b: [f64; 4], tol: f64) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < tol)
    }

    /// Senkrecht: Buchstaben untereinander, mittig auf der Linie; die zweite Zeile steht links daneben
    #[test]
    fn senkrechte_grundlinie() {
        let g = Grundlinie {
            art: "senkrecht".into(),
            ..Default::default()
        };
        let (o, _) = text_setzen(&satz_vorgabe("AB\nC", Some(g)), 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[0]), [-2.5, -5.0, 2.5, 0.0], 1e-9),
            "{:?}",
            huelle(&o[0])
        );
        assert!(
            gleich(huelle(&o[1]), [-2.5, -11.0, 2.5, -6.0], 1e-9),
            "{:?}",
            huelle(&o[1])
        );
        // Zeilenschritt 5 × (1 + 0,6) = 8 mm nach links
        assert!(
            gleich(huelle(&o[2]), [-10.5, -5.0, -5.5, 0.0], 1e-9),
            "{:?}",
            huelle(&o[2])
        );
    }

    /// `^` wechselt zur nächsten Farbe und belegt keinen Platz; einzelne Buchstaben färben und verschieben
    #[test]
    fn farbwechsel_und_einzelne_buchstaben() {
        let zs = TextZusatz {
            farben: vec!["#ff0000".into()],
            stichart: None,
            huelle: None,
            einzeln: vec![Buchstabe {
                nr: 3,
                farbe: Some("#0000ff".into()),
                transform: crate::transformation::Transformation {
                    verschieben: Some(p(0.0, 2.0)),
                    ..Default::default()
                },
                winkellinien: vec![],
            }],
        };
        let (o, _) = text_setzen(&satz_vorgabe("A^B C", None), 1.0, Some(&zs), 45.0, kaesten);
        assert_eq!(o.len(), 3);
        assert_eq!(o[0].farbe, "#000000");
        assert_eq!(o[1].farbe, "#ff0000");
        let ObjektArt::Gruppe { teile, .. } = &o[1].art else {
            panic!()
        };
        assert_eq!(teile[0].farbe, "#ff0000");
        assert_eq!(o[2].farbe, "#0000ff");
        assert!(gleich(huelle(&o[1]), [6.0, 0.0, 11.0, 5.0], 1e-9));
        assert!(
            gleich(huelle(&o[2]), [18.0, 2.0, 23.0, 7.0], 1e-9),
            "{:?}",
            huelle(&o[2])
        );
    }

    /// Eigene Kurve (Any Shape), Blocksatz, zu kurze Grundlinie, gerade mit Winkel
    #[test]
    fn eigene_kurve_und_blocksatz() {
        let kurve = |punkte: Vec<Punkt>, blocksatz: bool| Grundlinie {
            art: "kurve".into(),
            punkte,
            blocksatz,
            ..Default::default()
        };
        // Linie nach oben: Fuß auf der Linie, Buchstaben um 90° gedreht
        let g = kurve(vec![p(0.0, 0.0), p(0.0, 100.0)], false);
        let (o, _) = text_setzen(&satz_vorgabe("AB", Some(g)), 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[0]), [-5.0, 0.0, 0.0, 5.0], 1e-9),
            "{:?}",
            huelle(&o[0])
        );
        assert!(gleich(huelle(&o[1]), [-5.0, 6.0, 0.0, 11.0], 1e-9));
        // Blocksatz auf 50 mm: der zweite Buchstabe endet am Ende der Linie
        let g = kurve(vec![p(0.0, 0.0), p(50.0, 0.0)], true);
        let (o, _) = text_setzen(&satz_vorgabe("AB", Some(g)), 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[1]), [45.0, 0.0, 50.0, 5.0], 1e-9),
            "{:?}",
            huelle(&o[1])
        );
        // Text länger als die Linie (8 mm): die Abstände schrumpfen, Buchstaben dürfen sich überlappen
        let g = kurve(vec![p(0.0, 0.0), p(8.0, 0.0)], false);
        let (o, _) = text_setzen(&satz_vorgabe("AB", Some(g)), 1.0, None, 45.0, kaesten);
        assert!(gleich(huelle(&o[1]), [3.0, 0.0, 8.0, 5.0], 1e-9), "{:?}", huelle(&o[1]));
        // gerade, Line Angle 90° ohne Länge: um die Position gedreht
        let g = Grundlinie {
            art: "gerade".into(),
            winkel: Some(90.0),
            ..Default::default()
        };
        let (o, _) = text_setzen(&satz_vorgabe("A", Some(g)), 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[0]), [-5.0, 0.0, 0.0, 5.0], 1e-9),
            "{:?}",
            huelle(&o[0])
        );
        assert!(
            Grundlinie {
                blocksatz: true,
                ..Default::default()
            }
            .pruefen("a")
            .is_err()
        );
    }

    /// Oval (Radius X/Y) und mehrfach (oben – Mitte – unten)
    #[test]
    fn oval_und_mehrfach() {
        let g = Grundlinie {
            radius: 40.0,
            radius_y: Some(20.0),
            ..Default::default()
        };
        let mut v = satz_vorgabe("A", Some(g));
        v.ausrichtung = 0.5;
        let (o, _) = text_setzen(&v, 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[0]), [-2.5, 20.0, 2.5, 25.0], 0.05),
            "{:?}",
            huelle(&o[0])
        );
        let g = Grundlinie {
            art: "mehrfach".into(),
            radius: 20.0,
            ..Default::default()
        };
        let mut v = satz_vorgabe("A\nB\nC", Some(g.clone()));
        v.ausrichtung = 0.5;
        let (o, _) = text_setzen(&v, 1.0, None, 45.0, kaesten);
        assert!(
            gleich(huelle(&o[0]), [-2.5, 20.0, 2.5, 25.0], 1e-9),
            "{:?}",
            huelle(&o[0])
        );
        assert!(
            gleich(huelle(&o[1]), [-2.5, -2.5, 2.5, 2.5], 1e-9),
            "{:?}",
            huelle(&o[1])
        );
        assert!(
            gleich(huelle(&o[2]), [-2.5, -25.0, 2.5, -20.0], 1e-9),
            "{:?}",
            huelle(&o[2])
        );
        assert!(g.pruefen("a\nb\nc\nd").is_err());
    }

    /// Stichart der Schrift: Step setzt die Säulen auf Step; Muster vereinigt die Säulen zu einer Musterfüllung
    #[test]
    fn stichart_der_schrift() {
        let saeule = |a: Linie, b: Linie| Objekt {
            farbe: "#000000".into(),
            name: "T".into(),
            art: ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(vec![]),
                optionen: satin_optionen(None, true, SCHRIFT_FAKTOR),
            },
        };
        let rechteck = |x0: f64, y0: f64, x1: f64, y1: f64| vec![p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)];
        // „T“: Stamm 2 × 10 mm, Balken 6 × 2 mm oben, überlappen sich 2 × 2 mm
        let t = Objekt {
            farbe: "#000000".into(),
            name: "T".into(),
            art: ObjektArt::Gruppe {
                teile: vec![
                    saeule(vec![p(0.0, 0.0), p(0.0, 10.0)], vec![p(2.0, 0.0), p(2.0, 10.0)]),
                    saeule(vec![p(-2.0, 8.0), p(4.0, 8.0)], vec![p(-2.0, 10.0), p(4.0, 10.0)]),
                ],
                umrisse: vec![rechteck(0.0, 0.0, 2.0, 10.0), rechteck(-2.0, 8.0, 4.0, 10.0)],
                laufweg: 3.0,
            },
        };
        let ObjektArt::Gruppe { teile, .. } = stichart_anwenden(t.clone(), &SchriftStichart::Step).art else {
            panic!()
        };
        assert!(
            teile
                .iter()
                .all(|t| matches!(&t.art, ObjektArt::Satin { optionen, .. } if optionen.step))
        );
        let m = SchriftStichart::Muster {
            muster: "001 Diamond (90)".into(),
            winkel: Some(30.0),
            groesse: 100.0,
        };
        let ObjektArt::Gruppe { teile, umrisse, .. } = stichart_anwenden(t, &m).art else {
            panic!()
        };
        assert_eq!(teile.len(), 1);
        assert_eq!(umrisse.len(), 1);
        let ObjektArt::Musterfuellung { umriss, winkel, .. } = &teile[0].art else {
            panic!()
        };
        assert_eq!(*winkel, Some(30.0));
        let f = crate::geometrie::flaeche_vorzeichen(umriss).abs();
        assert!((f - 28.0).abs() < 1e-6, "{f}");
    }

    /// Elastic Lettering: gerade (Wimpel) und gebogen (Bogen) – Kanten folgen den Griffen
    #[test]
    fn elastische_huelle() {
        let (o, _) = text_setzen(&satz_vorgabe("ABCDE", None), 1.0, None, 45.0, kaesten);
        // Text 0 … 29 mm breit, 0 … 5 mm hoch; Wimpel: rechts Oberkante 2 mm herunter, Unterkante 2 mm hinauf
        let h = Huelle {
            gebogen: false,
            oben: [0.0, -1.0, -2.0],
            unten: [0.0, 1.0, 2.0],
        };
        let r = h.anwenden(o.clone());
        assert!(gleich(huelle(&r[0]), [0.0, 0.0, 5.0, 5.0], 0.4), "{:?}", huelle(&r[0]));
        // letzter Buchstabe (x 24 … 29): Oberkante bei 29 auf 3, Unterkante auf 2
        assert!(
            gleich(huelle(&r[4]), [24.0, 1.65, 29.0, 3.35], 0.01),
            "{:?}",
            huelle(&r[4])
        );
        // gebogen: Mitte der Oberkante 4 mm hoch, Unterkante bleibt – mittlerer Buchstabe wird höher
        let b = Huelle {
            gebogen: true,
            oben: [0.0, 4.0, 0.0],
            unten: [0.0; 3],
        };
        let r = b.anwenden(o);
        let m = huelle(&r[2]);
        assert!(m[1].abs() < 1e-9 && m[3] > 8.9 && m[3] <= 9.0 + 1e-9, "{m:?}");
        // Kanten sind geteilt und gebogen: das Rechteck hat jetzt mehr als 4 Punkte
        let ObjektArt::Gruppe { teile, .. } = &r[2].art else {
            panic!()
        };
        let ObjektArt::Tatami { umriss, .. } = &teile[0].art else {
            panic!()
        };
        assert!(umriss.len() > 4);
    }

    /// Stichwinkel je Buchstabe: eine schräge Winkellinie in einer senkrechten Säule wird zum Stichpaar
    #[test]
    fn winkellinien_je_buchstabe() {
        let a = vec![p(0.0, 0.0), p(0.0, 10.0)];
        let b = vec![p(2.0, 0.0), p(2.0, 10.0)];
        let (na, nb) = saeule_mit_linien(&a, &b, &[[p(-1.0, 3.0), p(3.0, 7.0)]]).unwrap();
        assert_eq!(na.len(), nb.len());
        assert!(
            na.iter()
                .zip(&nb)
                .any(|(x, y)| (x.y - 4.0).abs() < 1e-9 && (y.y - 6.0).abs() < 1e-9),
            "{na:?} {nb:?}"
        );
        // Anfang und Ende bleiben
        assert_eq!((na[0], nb[0]), (a[0], b[0]));
        assert_eq!((*na.last().unwrap(), *nb.last().unwrap()), (a[1], b[1]));
        // Linie trifft nur eine Seite: keine Änderung
        assert!(saeule_mit_linien(&a, &b, &[[p(-1.0, 3.0), p(1.0, 3.0)]]).is_none());
        // Füllung: Winkel der Linie
        let (mut o, _) = text_setzen(&satz_vorgabe("A", None), 1.0, None, 45.0, kaesten);
        winkel_setzen(&mut o[0], &[[p(-1.0, -1.0), p(6.0, 6.0)]]);
        let ObjektArt::Gruppe { teile, .. } = &o[0].art else {
            panic!()
        };
        let ObjektArt::Tatami { winkel, .. } = &teile[0].art else {
            panic!()
        };
        assert!((winkel.unwrap() - 45.0).abs() < 1e-9);
    }
}
