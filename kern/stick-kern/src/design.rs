//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::einstellungen::{Einstellungen, SatinEinstellung, fuer_stoff};
use crate::geometrie::{
    Linie, Punkt, abstand, abstand_punkt_strecke, bezier, bogen, im_polygon, kreis, p, rechteck, zwischen,
};
use crate::nachbearbeitung::{bernina_regeln, wege_verbinden};
use crate::rahmen::{Rahmen, rahmen};
use crate::schrift::{Alphabet, TextVorgabe, text_objekte};
use crate::sticharten::lauf::{dreifach, laufstich_mit_ecken};
use crate::sticharten::satin::{mittellinie, paare_bilden, satin, seiten_aus_mittellinie};
use crate::sticharten::tatami::tatami;
use crate::sticharten::unterlagen::{auto_satin_unterlage, flaechen_unterlage, satin_unterlage};
use crate::stiche::StichFolge;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

// ---------------------------------------------------------------- JSON-Eingabe

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PunktAngabe {
    Liste(Vec<Punkt>),
    Form(Form),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Form {
    Kreis {
        mitte: Punkt,
        radius: f64,
        #[serde(default)]
        start: f64,
    },
    Rechteck {
        mitte: Punkt,
        breite: f64,
        hoehe: f64,
    },
    Bogen {
        mitte: Punkt,
        radius: f64,
        von: f64,
        bis: f64,
    },
    Bezier([Punkt; 4]),
    Verbinden(Vec<PunktAngabe>),
}

impl PunktAngabe {
    pub fn punkte(&self) -> Linie {
        match self {
            PunktAngabe::Liste(l) => l.clone(),
            PunktAngabe::Form(Form::Kreis { mitte, radius, start }) => kreis(*mitte, *radius, 0.02, *start),
            PunktAngabe::Form(Form::Rechteck { mitte, breite, hoehe }) => rechteck(*mitte, *breite, *hoehe),
            PunktAngabe::Form(Form::Bogen {
                mitte,
                radius,
                von,
                bis,
            }) => bogen(*mitte, *radius, *von, *bis, 0.02),
            PunktAngabe::Form(Form::Bezier(q)) => bezier(q[0], q[1], q[2], q[3], 32),
            PunktAngabe::Form(Form::Verbinden(teile)) => {
                let mut out: Linie = Vec::new();
                for t in teile {
                    let pts = t.punkte();
                    let start = usize::from(matches!((out.last(), pts.first()), (Some(l), Some(f)) if l == f));
                    out.extend_from_slice(&pts[start..]);
                }
                out
            }
        }
    }

    /// Punkte samt Markierung „Eckpunkt“ (wie beim Digitalisieren im Original: Eckpunkt oder Kurvenpunkt).
    /// Punktlisten und Rechteckecken sind Eckpunkte; Kreis, Bogen und Bézier liefern Kurvenpunkte, nur ihre Enden
    /// sind Ecken. Beim Verbinden zählt die Nahtstelle als Ecke (eigene Festlegung).
    pub fn punkte_mit_ecken(&self) -> (Linie, Vec<bool>) {
        match self {
            PunktAngabe::Liste(_) | PunktAngabe::Form(Form::Rechteck { .. }) => {
                let pts = self.punkte();
                let n = pts.len();
                (pts, vec![true; n])
            }
            PunktAngabe::Form(Form::Verbinden(teile)) => {
                let (mut out, mut ecken): (Linie, Vec<bool>) = (Vec::new(), Vec::new());
                for t in teile {
                    let (pts, e) = t.punkte_mit_ecken();
                    let start = usize::from(matches!((out.last(), pts.first()), (Some(l), Some(f)) if l == f));
                    if let (1, Some(letzte)) = (start, ecken.last_mut()) {
                        *letzte = true;
                    }
                    out.extend_from_slice(&pts[start..]);
                    ecken.extend_from_slice(&e[start..]);
                }
                (out, ecken)
            }
            _ => {
                let pts = self.punkte();
                let n = pts.len();
                (pts, (0..n).map(|i| i == 0 || i + 1 == n).collect())
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UnterlageJson {
    Text(String),
    Liste(Vec<String>),
}

fn schwarz() -> String {
    "#000000".into()
}
fn auto() -> UnterlageJson {
    UnterlageJson::Text("auto".into())
}

#[derive(Debug, Deserialize)]
struct LaufJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    punkte: PunktAngabe,
    laenge: Option<f64>,
    dreifach: Option<bool>,
    #[serde(default)]
    geschlossen: bool,
}

#[derive(Debug, Deserialize)]
struct SatinJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    seite_a: PunktAngabe,
    seite_b: PunktAngabe,
    #[serde(default = "auto")]
    unterlage: UnterlageJson,
    /// fester Satinabstand in mm statt Auto Spacing (wie im Alphabet: 0x15)
    abstand: Option<f64>,
    /// Auto-Spacing-Faktor des Objekts (Stil: 0x2736; Schrift 0,9)
    faktor: Option<f64>,
    /// Kurzstiche an Innenkurven (Effekt 0x273a; Software Defaults: aus)
    kurzstiche: Option<bool>,
}

fn hundert() -> f64 {
    100.0
}
fn mitte() -> String {
    "mitte".into()
}
fn text_hoehe() -> f64 {
    20.0
}

/// Text mit einem Original-Alphabet (`schrift.rs`). Die Alphabete lädt der Aufrufer (`Design::alphabete`).
#[derive(Debug, Deserialize)]
struct TextJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    text: String,
    alphabet: String,
    /// Buchstabenhöhe in mm (Original 1–200 mm) – Vorgabe 20 mm ist eine eigene Festlegung
    #[serde(default = "text_hoehe")]
    hoehe: f64,
    /// Breite in % der Höhe
    #[serde(default = "hundert")]
    breite: f64,
    /// Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe)
    abstand: Option<f64>,
    /// Bezugspunkt auf der Grundlinie
    #[serde(default)]
    position: Option<Punkt>,
    /// links | mitte | rechts
    #[serde(default = "mitte")]
    ausrichtung: String,
    /// Unterlage: auto (wie Stil Schrift: Zickzack) | keine | mittellauf | edge_walk | zickzack | doppelzickzack
    #[serde(default = "auto")]
    unterlage: UnterlageJson,
}

#[derive(Debug, Deserialize)]
struct SatinLinieJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    punkte: PunktAngabe,
    breite: Option<f64>,
    #[serde(default = "auto")]
    unterlage: UnterlageJson,
}

#[derive(Debug, Deserialize)]
struct TatamiJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    umriss: PunktAngabe,
    #[serde(default)]
    loecher: Vec<PunktAngabe>,
    winkel: Option<f64>,
    /// ohne Angabe: Auto Underlay des Stils (`auto_an`)
    unterlage: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "art", rename_all = "lowercase")]
enum ObjektJson {
    Lauf(LaufJson),
    Dreifach(LaufJson),
    Satin(SatinJson),
    Satinlinie(SatinLinieJson),
    Tatami(TatamiJson),
    Text(TextJson),
}

fn design_name() -> String {
    "Design".into()
}
fn standard_rahmen() -> String {
    "artista 130 x 100".into()
}
fn standard_stoff() -> String {
    "0 Software Defaults".into()
}

#[derive(Debug, Deserialize)]
struct DesignJson {
    #[serde(default = "design_name")]
    name: String,
    #[serde(default = "standard_rahmen")]
    rahmen: String,
    #[serde(default = "standard_stoff")]
    stoff: String,
    einstellungen: Option<Value>,
    #[serde(default)]
    objekte: Vec<ObjektJson>,
}

// ---------------------------------------------------------------- Modell

#[derive(Clone, Debug, PartialEq)]
pub enum Unterlage {
    /// wie „Auto Underlay“ des Originals: aus oder die im Stil vorgewählte Art (`UnterlageEinstellung::auto_an`)
    Auto,
    /// Liste aus mittellauf, edge_walk, zickzack, doppelzickzack (leer = keine)
    Arten(Vec<String>),
}

impl From<UnterlageJson> for Unterlage {
    fn from(u: UnterlageJson) -> Self {
        match u {
            UnterlageJson::Text(t) if t == "auto" => Unterlage::Auto,
            UnterlageJson::Text(t) if t == "keine" => Unterlage::Arten(vec![]),
            UnterlageJson::Text(t) => Unterlage::Arten(t.split('+').map(|s| s.trim().to_string()).collect()),
            UnterlageJson::Liste(l) => Unterlage::Arten(l),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjektArt {
    /// Laufstich oder Dreifachstich entlang einer Linie
    Lauf {
        punkte: Linie,
        /// je Punkt: Eckpunkt (Einstich wie im Original) oder Kurvenpunkt
        ecken: Vec<bool>,
        laenge: Option<f64>,
        dreifach: bool,
        geschlossen: bool,
    },
    /// Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B)
    Satin {
        seite_a: Linie,
        seite_b: Linie,
        unterlage: Unterlage,
        optionen: SatinOptionen,
    },
    /// Satinbahn konstanter Breite entlang einer Mittellinie
    SatinLinie {
        punkte: Linie,
        breite: f64,
        unterlage: Unterlage,
    },
    /// Tatami-Füllung einer Fläche mit optionalen Löchern
    Tatami {
        umriss: Linie,
        loecher: Vec<Linie>,
        winkel: Option<f64>,
        /// None = Auto Underlay des Stils; mit Unterlage: Tatami-Unterlage „Step“ (Art für Flächen: eigene
        /// Festlegung, die vorgewählte Art des Stils ist Zickzack)
        unterlage: Option<bool>,
    },
    /// Text mit einem Original-Alphabet; wird vor dem Berechnen in Satin-/Lauf-Objekte aufgelöst
    Text { alphabet: String, vorgabe: TextVorgabe },
    /// Zusammengehörige Teile (ein Buchstabe): Reihenfolge nach Nähe, verbunden mit Laufweg statt Sprung, wenn der
    /// Weg unter noch zu stickenden Teilen liegt (`Analyse_Teil3/10` §4)
    Gruppe {
        teile: Vec<Objekt>,
        /// Umrisse der Teile (gleiche Reihenfolge) – dort darf der Laufweg liegen
        umrisse: Vec<Linie>,
        /// Stichlänge des Laufwegs in mm (Alphabet: Lauf 0x271c in der Füllung, 3,0 mm)
        laufweg: f64,
    },
}

/// Einstellungen einer Satinsäule, die vom Stil abweichen (wie die Effekte am Objekt im Original).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SatinOptionen {
    /// fester Abstand in mm (None = Einstellung des Stoffs/Stils, Auto Spacing)
    pub abstand: Option<f64>,
    /// Auto-Spacing-Faktor (None = Stil)
    pub faktor: Option<f64>,
    /// Kurzstiche (None = Stil)
    pub kurzstiche: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Objekt {
    pub farbe: String,
    pub name: String,
    pub art: ObjektArt,
}

fn satin_wege(
    a: &[Punkt],
    b: &[Punkt],
    unterlage: &Unterlage,
    o: &SatinOptionen,
    e: &Einstellungen,
) -> Result<Vec<Linie>, String> {
    let (unter, deck) = satin_getrennt(a, b, unterlage, o, e)?;
    if unter.is_empty() {
        Ok(vec![deck])
    } else {
        // in einem Zug: Unterlage endet am Säulenanfang
        Ok(vec![unter.into_iter().chain(deck).collect()])
    }
}

/// Satinsäule als (Unterlage, Decklage); die Unterlage endet am Säulenanfang (`Analyse_Teil3/07` §3), leer ohne
/// Unterlage.
fn satin_getrennt(
    a: &[Punkt],
    b: &[Punkt],
    unterlage: &Unterlage,
    o: &SatinOptionen,
    e: &Einstellungen,
) -> Result<(Linie, Linie), String> {
    let (a, b) = paare_bilden(a, b)?;
    let mut satin_e: SatinEinstellung = e.satin.clone();
    if let Some(x) = o.abstand {
        satin_e.auto_spacing = false;
        satin_e.abstand_fest = x;
    }
    if let Some(f) = o.faktor {
        satin_e.faktor = f;
    }
    if let Some(k) = o.kurzstiche {
        satin_e.kurzstiche.an = k;
    }
    let arten = match unterlage {
        Unterlage::Auto => auto_satin_unterlage(&e.unterlage),
        Unterlage::Arten(l) => l.clone(),
    };
    let unter = if arten.is_empty() {
        Vec::new()
    } else {
        satin_unterlage(&a, &b, &arten, &e.unterlage)?
    };
    let deck = satin(&a, &b, &satin_e)?;
    Ok((unter, deck))
}

/// Schneiden sich die Strecken a1–a2 und b1–b2?
fn strecken_schneiden(a1: Punkt, a2: Punkt, b1: Punkt, b2: Punkt) -> bool {
    let kreuz = |o: Punkt, a: Punkt, b: Punkt| (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
    let (d1, d2) = (kreuz(b1, b2, a1), kreuz(b1, b2, a2));
    let (d3, d4) = (kreuz(a1, a2, b1), kreuz(a1, a2, b2));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

/// Berühren sich zwei Umrisse (überlappen, kreuzen sich oder liegen näher als 0,15 mm)?
fn umrisse_beruehren(a: &Linie, b: &Linie) -> bool {
    let ring = |l: &Linie| -> Vec<(Punkt, Punkt)> { (0..l.len()).map(|i| (l[i], l[(i + 1) % l.len()])).collect() };
    let (ka, kb) = (ring(a), ring(b));
    a.iter().any(|&q| im_polygon(q, std::slice::from_ref(b)))
        || b.iter().any(|&q| im_polygon(q, std::slice::from_ref(a)))
        || ka
            .iter()
            .any(|&(a1, a2)| kb.iter().any(|&(b1, b2)| strecken_schneiden(a1, a2, b1, b2)))
        || a.iter()
            .any(|&q| kb.iter().any(|&(b1, b2)| abstand_punkt_strecke(q, b1, b2) < 0.15))
        || b.iter()
            .any(|&q| ka.iter().any(|&(a1, a2)| abstand_punkt_strecke(q, a1, a2) < 0.15))
}

/// Teile einer Gruppe (Buchstabe) sticken. Zwischen zwei Teilen ein **Laufweg** (Stichlänge `laufweg`), wenn er
/// verdeckt ist, d. h. unter Teilen liegt, die danach noch gestickt werden – sonst ein Sprung:
/// 1. gerade zum Anfang des nächsten Teils, oder
/// 2. auf kurzem Weg zur Mittellinie des nächsten Teils und auf ihr zurück zu seinem Anfang (liegt unter ihm).
///
/// Die Reihenfolge der Teile wird so gewählt, dass möglichst wenige Sprünge nötig sind, dann möglichst kurze Wege
/// (bis 7 Teile alle Reihenfolgen, sonst „nächster Anfang“).
/// Grundlage: Maschinen-Zeichensatz Drifter „A“ – erst der Querbalken, dann Laufweg im linken Bein (etwa auf seiner
/// Mitte) unter dem späteren Bogen (`Analyse_Teil3/10` §4). Wegführung und Bewertung sind **eigene Festlegungen**.
///
/// **Unterlagen zuerst** (gemessen an den Schrift-Musterdesigns des Originals, `Analyse_Teil3/11`): Teile, deren
/// Umrisse sich berühren, bilden eine Gruppe; je Gruppe stickt das Original **erst alle Unterlagen, in umgekehrter
/// Reihenfolge der Säulen**, dann die Säulen in ihrer Reihenfolge (1 Unterlage: 85 von 85 Gruppen vor den Säulen;
/// mehrere: 43 von 62 genau so, der Rest Messgrenzen). Getrennte Teile (Punkt des „i“) kommen nacheinander.
fn gruppe_wege(
    teile: &[Objekt],
    umrisse: &[Linie],
    laufweg: f64,
    e: &Einstellungen,
    start: Option<Punkt>,
) -> Result<Vec<Linie>, String> {
    // je Teil: Decklage (Wege), Umriss, Mittellinie (bei Satin); die Unterlage einer Satinsäule getrennt
    let mut stuecke: Vec<(Vec<Linie>, Option<&Linie>, Option<Linie>)> = Vec::new();
    let mut unterlagen: Vec<Linie> = Vec::new();
    for (i, t) in teile.iter().enumerate() {
        let (unter, w) = match &t.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => {
                let (u, d) = satin_getrennt(seite_a, seite_b, unterlage, optionen, e)?;
                (u, vec![d])
            }
            _ => (Vec::new(), t.wege(e, None)?),
        };
        let w: Vec<Linie> = w.into_iter().filter(|w| !w.is_empty()).collect();
        if w.is_empty() {
            continue;
        }
        unterlagen.push(unter);
        let mitte = match &t.art {
            ObjektArt::Satin { seite_a, seite_b, .. } => {
                paare_bilden(seite_a, seite_b).ok().map(|(a, b)| mittellinie(&a, &b))
            }
            _ => None,
        };
        stuecke.push((w, umrisse.get(i).filter(|u| u.len() >= 3), mitte));
    }
    let n = stuecke.len().min(63);
    if n == 0 {
        return Ok(Vec::new());
    }
    let anf: Vec<Punkt> = stuecke.iter().map(|(w, _, _)| w[0][0]).collect();
    let end: Vec<Punkt> = stuecke
        .iter()
        .map(|(w, _, _)| *w.last().and_then(|l| l.last()).unwrap())
        .collect();
    let bedeckt = |q: Punkt| -> u64 {
        stuecke[..n]
            .iter()
            .enumerate()
            .filter(|(_, (_, u, _))| u.is_some_and(|r| im_polygon(q, std::slice::from_ref(r))))
            .fold(0u64, |m, (k, _)| m | (1 << k))
    };
    // Prüfpunkte alle 0,5 mm auf einer Strecke; die Enden (je 0,6 mm) nicht, weil der Zugausgleich Anfang und Ende
    // der Säulen über den Umriss schiebt
    let masken = |a: Punkt, b: Punkt| -> Vec<u64> {
        let d = abstand(a, b);
        let pruef = ((d / 0.5).ceil() as usize).max(1);
        (1..pruef)
            .map(|k| k as f64 / pruef as f64)
            .filter(|t| t * d >= 0.6 && (1.0 - t) * d >= 0.6)
            .map(|t| bedeckt(zwischen(a, b, t)))
            .collect()
    };
    // je Verbindung i → j: gerade (Masken, Länge) und über die Mittellinie von j (Masken des Zugangs, Weg)
    struct Verbindung {
        gerade: Vec<u64>,
        gerade_laenge: f64,
        ueber_mitte: Option<(Vec<u64>, Linie)>,
    }
    let mut verb: Vec<Vec<Option<Verbindung>>> = (0..n).map(|_| (0..n).map(|_| None).collect()).collect();
    for i in 0..n {
        for j in (0..n).filter(|&j| j != i) {
            let ueber_mitte = stuecke[j].2.as_ref().filter(|m| m.len() >= 2).map(|m| {
                let k = (0..m.len())
                    .min_by(|&x, &y| abstand(end[i], m[x]).total_cmp(&abstand(end[i], m[y])))
                    .unwrap();
                let mut weg: Linie = vec![end[i]];
                weg.extend(m[..=k].iter().rev().copied());
                weg.push(anf[j]);
                (masken(end[i], m[k]), weg)
            });
            verb[i][j] = Some(Verbindung {
                gerade: masken(end[i], anf[j]),
                gerade_laenge: abstand(end[i], anf[j]),
                ueber_mitte,
            });
        }
    }
    let weglaenge = |w: &Linie| w.windows(2).map(|s| abstand(s[0], s[1])).sum::<f64>();
    // Laufweg i → j, wenn verdeckt (`rest` = noch ausstehende Teile einschließlich j): Punkte von end[i] bis anf[j]
    let laufweg_fuer = |i: usize, j: usize, rest: u64| -> Option<Linie> {
        let v = verb[i][j].as_ref()?;
        if v.gerade.iter().all(|&m| m & rest != 0) {
            return Some(vec![end[i], anf[j]]);
        }
        let (zugang, weg) = v.ueber_mitte.as_ref()?;
        zugang.iter().all(|&m| m & rest != 0).then(|| weg.clone())
    };
    let alle: u64 = (1u64 << n) - 1;
    let anfahrt = |j: usize| start.map_or(0.0, |s| abstand(s, anf[j]));

    // beste Reihenfolge: (Sprünge, Weglänge) möglichst klein
    let mut reihenfolge: Vec<usize> = Vec::new();
    if n <= 7 {
        let mut beste: (usize, f64) = (usize::MAX, f64::INFINITY);
        let mut stapel: Vec<(Vec<usize>, u64, usize, f64)> =
            (0..n).map(|j| (vec![j], alle & !(1 << j), 0, anfahrt(j))).collect();
        while let Some((weg, rest, sprunge, laenge)) = stapel.pop() {
            if sprunge > beste.0 || (sprunge == beste.0 && laenge >= beste.1) {
                continue;
            }
            if rest == 0 {
                beste = (sprunge, laenge);
                reihenfolge = weg;
                continue;
            }
            let i = *weg.last().unwrap();
            for j in (0..n).filter(|&j| rest & (1 << j) != 0) {
                let (s, l) = match laufweg_fuer(i, j, rest) {
                    Some(w) => (sprunge, laenge + weglaenge(&w)),
                    None => (
                        sprunge + 1,
                        laenge + verb[i][j].as_ref().map_or(0.0, |v| v.gerade_laenge),
                    ),
                };
                let mut w = weg.clone();
                w.push(j);
                stapel.push((w, rest & !(1 << j), s, l));
            }
        }
    } else {
        let mut rest = alle;
        let mut pos = start.unwrap_or(anf[0]);
        while rest != 0 {
            let j = (0..n)
                .filter(|&j| rest & (1 << j) != 0)
                .min_by(|&x, &y| abstand(pos, anf[x]).total_cmp(&abstand(pos, anf[y])))
                .unwrap();
            reihenfolge.push(j);
            rest &= !(1 << j);
            pos = end[j];
        }
    }

    // berührende Teile bilden Gruppen (Union-Find); Gruppen in der Reihenfolge ihres ersten Teils
    let mut eltern: Vec<usize> = (0..n).collect();
    fn wurzel(e: &mut [usize], mut k: usize) -> usize {
        while e[k] != k {
            e[k] = e[e[k]];
            k = e[k];
        }
        k
    }
    for i in 0..n {
        for j in i + 1..n {
            if let (Some(a), Some(b)) = (stuecke[i].1, stuecke[j].1)
                && umrisse_beruehren(a, b)
            {
                let (wi, wj) = (wurzel(&mut eltern, i), wurzel(&mut eltern, j));
                eltern[wj] = wi;
            }
        }
    }
    let mut gruppen: Vec<Vec<usize>> = Vec::new();
    let mut gruppe_von: Vec<Option<usize>> = vec![None; n];
    for &j in &reihenfolge {
        let w = wurzel(&mut eltern, j);
        let g = match gruppe_von[w] {
            Some(g) => g,
            None => {
                gruppen.push(Vec::new());
                gruppe_von[w] = Some(gruppen.len() - 1);
                gruppen.len() - 1
            }
        };
        gruppen[g].push(j);
    }

    // Laufweg von `von` nach `nach` (Ziel ist Teil `ziel`), wenn verdeckt von noch ausstehenden Teilen `rest`:
    // gerade, oder über die Mittellinie des Ziels
    let laufweg_zu = |von: Punkt, nach: Punkt, ziel: usize, rest: u64| -> Option<Linie> {
        if masken(von, nach).iter().all(|&m| m & rest != 0) {
            return Some(vec![von, nach]);
        }
        let m = stuecke[ziel].2.as_ref().filter(|m| m.len() >= 2)?;
        let naechster = |q: Punkt| {
            (0..m.len())
                .min_by(|&x, &y| abstand(q, m[x]).total_cmp(&abstand(q, m[y])))
                .unwrap()
        };
        let (k, t) = (naechster(von), naechster(nach));
        if !masken(von, m[k]).iter().all(|&mk| mk & rest != 0) {
            return None;
        }
        let mut weg: Linie = vec![von];
        if k <= t {
            weg.extend(m[k..=t].iter().copied());
        } else {
            weg.extend(m[t..=k].iter().rev().copied());
        }
        weg.push(nach);
        Some(weg)
    };
    // Stück anhängen: mit Laufweg an den letzten Zug, sonst als neuer Zug (Sprung)
    let mut out: Vec<Linie> = Vec::new();
    let anhaengen = |out: &mut Vec<Linie>, stueck: Linie, ziel: usize, rest: u64| {
        let Some(letzter) = out.last_mut() else {
            out.push(stueck);
            return;
        };
        let von = *letzter.last().unwrap();
        if abstand(von, stueck[0]) < 1e-6 {
            letzter.extend(stueck.into_iter().skip(1));
            return;
        }
        match laufweg_zu(von, stueck[0], ziel, rest) {
            Some(weg) => {
                // Laufstiche höchstens `laufweg` lang entlang des Wegs, Eckpunkte des Wegs als Einstiche
                for s in weg.windows(2) {
                    let schritte = ((abstand(s[0], s[1]) / laufweg.max(0.5)).ceil() as usize).max(1);
                    letzter.extend((1..=schritte).map(|k| zwischen(s[0], s[1], k as f64 / schritte as f64)));
                }
                letzter.pop(); // der Anfang des Stücks kommt mit dem Stück selbst
                letzter.extend(stueck);
            }
            None => out.push(stueck),
        }
    };
    let mut rest = alle;
    for g in &gruppen {
        // erst alle Unterlagen der Gruppe, rückwärts – die letzte endet am Anfang der ersten Säule
        for &j in g.iter().rev() {
            if !unterlagen[j].is_empty() {
                anhaengen(&mut out, unterlagen[j].clone(), j, rest);
            }
        }
        // dann die Decklagen in ihrer Reihenfolge
        for &j in g {
            let mut wege = stuecke[j].0.clone().into_iter();
            if let Some(erster) = wege.next() {
                anhaengen(&mut out, erster, j, rest);
            }
            out.extend(wege);
            rest &= !(1 << j);
        }
    }
    // weitere Teile über 63 (kommt bei Buchstaben nicht vor): ohne Laufweg anhängen
    for (k, (w, _, _)) in stuecke.iter().enumerate().skip(n) {
        if !unterlagen[k].is_empty() {
            out.push(unterlagen[k].clone());
        }
        out.extend(w.iter().cloned());
    }
    Ok(out)
}

impl Objekt {
    pub fn wege(&self, e: &Einstellungen, start: Option<Punkt>) -> Result<Vec<Linie>, String> {
        match &self.art {
            ObjektArt::Lauf {
                punkte,
                ecken,
                laenge,
                dreifach: drei,
                geschlossen,
            } => {
                let (mut pts, mut ecken) = (punkte.clone(), ecken.clone());
                if ecken.len() != pts.len() {
                    ecken = vec![true; pts.len()];
                }
                if *geschlossen && !pts.is_empty() && abstand(pts[0], *pts.last().unwrap()) > 1e-9 {
                    pts.push(pts[0]);
                    ecken.push(true);
                }
                let lauf = laufstich_mit_ecken(&pts, Some(&ecken), &e.lauf, *laenge);
                Ok(vec![if *drei { dreifach(&lauf) } else { lauf }])
            }
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => satin_wege(seite_a, seite_b, unterlage, optionen, e),
            ObjektArt::SatinLinie {
                punkte,
                breite,
                unterlage,
            } => {
                let (a, b) = seiten_aus_mittellinie(punkte, *breite);
                satin_wege(&a, &b, unterlage, &SatinOptionen::default(), e)
            }
            ObjektArt::Gruppe {
                teile,
                umrisse,
                laufweg,
            } => gruppe_wege(teile, umrisse, *laufweg, e, start),
            ObjektArt::Text { alphabet, .. } => Err(format!("Text mit Alphabet „{alphabet}“ wurde nicht aufgelöst")),
            ObjektArt::Tatami {
                umriss,
                loecher,
                winkel,
                unterlage,
            } => {
                let w = winkel.unwrap_or(e.tatami.winkel);
                let mut wege = Vec::new();
                let mut start = start;
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    wege.extend(flaechen_unterlage(umriss, loecher, w, &e.unterlage, start));
                    start = wege.last().and_then(|l: &Linie| l.last().copied());
                }
                wege.extend(tatami(umriss, loecher, &e.tatami, Some(w), start));
                Ok(wege)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Design {
    pub name: String,
    pub rahmen: Rahmen,
    pub einstellungen: Einstellungen,
    pub objekte: Vec<Objekt>,
    /// geladene Alphabete (Name wie im Alphabet-Kopf, z. B. „Drifter“); füllt der Aufrufer, siehe `alphabet_namen`
    pub alphabete: BTreeMap<String, Alphabet>,
}

impl Design {
    pub fn aus_json(text: &str) -> Result<Design, String> {
        let v: Value = serde_json::from_str(text).map_err(|e| format!("JSON ungültig: {e}"))?;
        Design::aus_value(&v)
    }

    pub fn aus_value(v: &Value) -> Result<Design, String> {
        let d: DesignJson = serde_json::from_value(v.clone()).map_err(|e| format!("Design ungültig: {e}"))?;
        let mut e = fuer_stoff(&d.stoff)?;
        if let Some(werte) = &d.einstellungen {
            e = e.ueberschreiben(werte)?;
        }
        let objekte = d
            .objekte
            .into_iter()
            .map(|o| match o {
                ObjektJson::Lauf(l) => lauf_objekt(l, false),
                ObjektJson::Dreifach(l) => lauf_objekt(l, true),
                ObjektJson::Satin(s) => Objekt {
                    farbe: s.farbe,
                    name: s.name,
                    art: ObjektArt::Satin {
                        seite_a: s.seite_a.punkte(),
                        seite_b: s.seite_b.punkte(),
                        unterlage: s.unterlage.into(),
                        optionen: SatinOptionen {
                            abstand: s.abstand,
                            faktor: s.faktor,
                            kurzstiche: s.kurzstiche,
                        },
                    },
                },
                ObjektJson::Text(t) => Objekt {
                    name: if t.name.is_empty() { t.text.clone() } else { t.name },
                    art: ObjektArt::Text {
                        alphabet: t.alphabet,
                        vorgabe: TextVorgabe {
                            text: t.text,
                            hoehe: t.hoehe,
                            breite_prozent: t.breite,
                            abstand: t.abstand,
                            position: t.position.unwrap_or(p(0.0, 0.0)),
                            ausrichtung: match t.ausrichtung.as_str() {
                                "links" => 0.0,
                                "rechts" => 1.0,
                                _ => 0.5,
                            },
                            farbe: t.farbe.clone(),
                            unterlage: match Unterlage::from(t.unterlage) {
                                Unterlage::Auto => None,
                                Unterlage::Arten(l) => Some(l),
                            },
                        },
                    },
                    farbe: t.farbe,
                },
                ObjektJson::Satinlinie(s) => Objekt {
                    farbe: s.farbe,
                    name: s.name,
                    art: ObjektArt::SatinLinie {
                        punkte: s.punkte.punkte(),
                        breite: s.breite.unwrap_or(2.0),
                        unterlage: s.unterlage.into(),
                    },
                },
                ObjektJson::Tatami(t) => Objekt {
                    farbe: t.farbe,
                    name: t.name,
                    art: ObjektArt::Tatami {
                        umriss: t.umriss.punkte(),
                        loecher: t.loecher.iter().map(|l| l.punkte()).collect(),
                        winkel: t.winkel,
                        unterlage: t.unterlage,
                    },
                },
            })
            .collect();
        Ok(Design {
            name: d.name,
            rahmen: rahmen(&d.rahmen)?,
            einstellungen: e,
            objekte,
            alphabete: BTreeMap::new(),
        })
    }

    /// Namen der Alphabete, die die Texte des Designs brauchen.
    pub fn alphabet_namen(&self) -> Vec<String> {
        let mut n: Vec<String> = self
            .objekte
            .iter()
            .filter_map(|o| match &o.art {
                ObjektArt::Text { alphabet, .. } => Some(alphabet.clone()),
                _ => None,
            })
            .collect();
        n.sort();
        n.dedup();
        n
    }

    /// Objekte mit aufgelösten Texten (Buchstaben als Satin-/Lauf-Objekte). Fehlende Zeichen werden übersprungen
    /// und im zweiten Wert gemeldet.
    pub fn objekte_aufgeloest(&self) -> Result<(Vec<Objekt>, Vec<char>), String> {
        let mut out = Vec::new();
        let mut fehlend = Vec::new();
        for o in &self.objekte {
            match &o.art {
                ObjektArt::Text { alphabet, vorgabe } => {
                    let a = self
                        .alphabete
                        .get(alphabet)
                        .or_else(|| self.alphabete.values().find(|a| a.name.eq_ignore_ascii_case(alphabet)))
                        .ok_or_else(|| format!("Alphabet „{alphabet}“ nicht geladen"))?;
                    let (obj, f) = text_objekte(a, vorgabe);
                    out.extend(obj);
                    fehlend.extend(f);
                }
                _ => out.push(o.clone()),
            }
        }
        Ok((out, fehlend))
    }

    /// Stiche berechnen: Objekte -> Wege -> verbinden -> Bernina-Regeln (Mindeststich, Auto Jump).
    pub fn berechnen(&self) -> Result<StichFolge, String> {
        let mut teile: Vec<(String, Vec<Linie>)> = Vec::new();
        let mut pos = p(0.0, 0.0);
        let (objekte, _) = self.objekte_aufgeloest()?;
        for obj in &objekte {
            let wege: Vec<Linie> = obj
                .wege(&self.einstellungen, Some(pos))?
                .into_iter()
                .filter(|w| !w.is_empty())
                .collect();
            if let Some(letzter) = wege.last().and_then(|w| w.last()) {
                pos = *letzter;
                teile.push((obj.farbe.clone(), wege));
            }
        }
        let folge = wege_verbinden(&teile, &self.einstellungen.verbinder, p(0.0, 0.0));
        Ok(bernina_regeln(&folge))
    }

    /// Einstiche außerhalb des Stickfelds.
    pub fn ausserhalb(&self, folge: &StichFolge) -> Vec<Punkt> {
        folge
            .stiche
            .iter()
            .map(|s| s.p())
            .filter(|&q| !self.rahmen.enthaelt(q))
            .collect()
    }
}

fn lauf_objekt(l: LaufJson, art_dreifach: bool) -> Objekt {
    let (punkte, ecken) = l.punkte.punkte_mit_ecken();
    Objekt {
        farbe: l.farbe,
        name: l.name,
        art: ObjektArt::Lauf {
            punkte,
            ecken,
            laenge: l.laenge,
            dreifach: l.dreifach.unwrap_or(art_dreifach),
            geschlossen: l.geschlossen,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::einstellungen::fuer_stoff;

    fn saeule(x0: f64, x1: f64, y0: f64, y1: f64, waagrecht: bool) -> (Objekt, Linie) {
        // Seiten A/B und Umriss eines Rechtecks; waagrecht: von x0 nach x1, sonst von y0 nach y1
        let (a, b) = if waagrecht {
            (vec![p(x0, y1), p(x1, y1)], vec![p(x0, y0), p(x1, y0)])
        } else {
            (vec![p(x0, y0), p(x0, y1)], vec![p(x1, y0), p(x1, y1)])
        };
        let umriss = vec![
            p(x0.min(x1), y0),
            p(x0.max(x1), y0),
            p(x0.max(x1), y1),
            p(x0.min(x1), y1),
        ];
        let o = Objekt {
            farbe: "#000000".into(),
            name: String::new(),
            art: ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(vec![]),
                optionen: SatinOptionen::default(),
            },
        };
        (o, umriss)
    }

    /// wie Drifter „A“: Querbalken zuerst (endet über dem linken Bein), dann Laufweg im Bein unter der Säule
    #[test]
    fn gruppe_laufweg_statt_sprung() {
        let (bein, u_bein) = saeule(4.0, 0.0, 0.0, 10.0, false);
        let (balken, u_balken) = saeule(10.0, 1.0, 4.0, 6.0, true);
        let e = fuer_stoff("0 Software Defaults").unwrap();
        // Reihenfolge in der Eingabe: Bein zuerst – die Gruppe stickt trotzdem den Balken zuerst
        let wege = gruppe_wege(&[bein, balken], &[u_bein, u_balken], 3.0, &e, Some(p(0.0, 0.0))).unwrap();
        assert_eq!(wege.len(), 1, "ein Zug ohne Sprung");
        assert!(wege[0][0].x > 9.0, "beginnt am Balken");
    }

    fn mit_unterlage(mut o: Objekt) -> Objekt {
        if let ObjektArt::Satin { unterlage, .. } = &mut o.art {
            *unterlage = Unterlage::Arten(vec!["zickzack".into()]);
        }
        o
    }

    fn stelle(wege: &[Linie], q: Punkt) -> usize {
        let alle: Vec<Punkt> = wege.iter().flatten().copied().collect();
        alle.iter()
            .position(|&x| abstand(x, q) < 1e-9)
            .unwrap_or_else(|| panic!("Punkt {q:?} nicht in den Wegen"))
    }

    /// wie das Original (Schrift-Musterdesigns, `Analyse_Teil3/11`): berührende Säulen – erst alle Unterlagen
    /// rückwärts, dann die Säulen; getrennte Teile nacheinander
    #[test]
    fn gruppe_unterlagen_zuerst() {
        let e = fuer_stoff("0 Software Defaults").unwrap();
        let (bein, u_bein) = saeule(4.0, 0.0, 0.0, 10.0, false);
        let (balken, u_balken) = saeule(10.0, 1.0, 4.0, 6.0, true);
        let (bein, balken) = (mit_unterlage(bein), mit_unterlage(balken));
        let teile = [bein.clone(), balken.clone()];
        let wege = gruppe_wege(&teile, &[u_bein.clone(), u_balken.clone()], 3.0, &e, Some(p(0.0, 0.0))).unwrap();
        let getrennt = |o: &Objekt| match &o.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => satin_getrennt(seite_a, seite_b, unterlage, optionen, &e).unwrap(),
            _ => unreachable!(),
        };
        let (u1, d1) = getrennt(&balken); // Balken zuerst (wie ohne Unterlage)
        let (u2, d2) = getrennt(&bein);
        assert!(!u1.is_empty() && !u2.is_empty());
        let (iu2, iu1, id1, id2) = (
            stelle(&wege, u2[0]),
            stelle(&wege, u1[0]),
            stelle(&wege, d1[0]),
            stelle(&wege, d2[1]),
        );
        assert!(
            iu2 < iu1 && iu1 < id1 && id1 < id2,
            "Folge U2 {iu2}, U1 {iu1}, S1 {id1}, S2 {id2}"
        );
        assert_eq!(wege.len(), 1, "Laufwege unter den Säulen, kein Sprung");

        // getrennte Teile (Punkt und Stamm des „i“): je Teil Unterlage, dann Säule
        let (stamm, u_stamm) = saeule(0.0, 1.0, 0.0, 6.0, false);
        let (punkt, u_punkt) = saeule(0.0, 1.0, 8.0, 9.5, false);
        let (stamm, punkt) = (mit_unterlage(stamm), mit_unterlage(punkt));
        let wege = gruppe_wege(
            &[stamm.clone(), punkt.clone()],
            &[u_stamm, u_punkt],
            3.0,
            &e,
            Some(p(0.5, 0.0)),
        )
        .unwrap();
        let (us, ds) = getrennt(&stamm);
        let (up, dp) = getrennt(&punkt);
        let (a, b) = (
            stelle(&wege, us[0]).min(stelle(&wege, up[0])),
            stelle(&wege, ds[1]).min(stelle(&wege, dp[1])),
        );
        assert!(a < b);
        // die erste Säule folgt direkt auf ihre eigene Unterlage, nicht auf die des anderen Teils
        let erst_stamm = stelle(&wege, us[0]) < stelle(&wege, up[0]);
        let (u_erst, d_erst, u_zweit) = if erst_stamm { (&us, &ds, &up) } else { (&up, &dp, &us) };
        assert!(stelle(&wege, u_erst[0]) < stelle(&wege, d_erst[1]));
        assert!(
            stelle(&wege, d_erst[1]) < stelle(&wege, u_zweit[0]),
            "getrennte Teile nacheinander"
        );
    }
}
