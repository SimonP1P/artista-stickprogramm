//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::einstellungen::{Einstellungen, SatinEinstellung, fuer_stoff};
use crate::geometrie::{Linie, Punkt, abstand, bezier, bogen, im_polygon, kreis, p, rechteck, zwischen};
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
    if unter.is_empty() {
        Ok(vec![deck])
    } else {
        // in einem Zug: Unterlage endet am Säulenanfang
        Ok(vec![unter.into_iter().chain(deck).collect()])
    }
}

/// Teile einer Gruppe (Buchstabe) sticken. Zwischen zwei Teilen ein **Laufweg** (Stichlänge `laufweg`), wenn er
/// verdeckt ist, d. h. unter Teilen liegt, die danach noch gestickt werden – sonst ein Sprung:
/// 1. gerade zum Anfang des nächsten Teils, oder
/// 2. auf kurzem Weg zur Mittellinie des nächsten Teils und auf ihr zurück zu seinem Anfang (liegt unter ihm).
///
/// Die Reihenfolge der Teile wird so gewählt, dass möglichst wenige Sprünge nötig sind, dann möglichst kurze Wege
/// (bis 7 Teile alle Reihenfolgen, sonst „nächster Anfang“).
/// Grundlage: Maschinen-Zeichensatz Drifter „A“ – erst der Querbalken, dann Laufweg im linken Bein (etwa auf seiner
/// Mitte) unter dem späteren Bogen (`Analyse_Teil3/10` §4). Wegführung und Bewertung sind **eigene Festlegungen**; das
/// Original stickt außerdem zuerst alle Unterlagen eines Buchstabens (offen).
fn gruppe_wege(
    teile: &[Objekt],
    umrisse: &[Linie],
    laufweg: f64,
    e: &Einstellungen,
    start: Option<Punkt>,
) -> Result<Vec<Linie>, String> {
    // je Teil: Wege, Umriss, Mittellinie (bei Satin)
    let mut stuecke: Vec<(Vec<Linie>, Option<&Linie>, Option<Linie>)> = Vec::new();
    for (i, t) in teile.iter().enumerate() {
        let w: Vec<Linie> = t.wege(e, None)?.into_iter().filter(|w| !w.is_empty()).collect();
        if w.is_empty() {
            continue;
        }
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

    let mut out: Vec<Linie> = Vec::new();
    let mut rest = alle;
    let mut vorher: Option<usize> = None;
    for &j in &reihenfolge {
        let mut wege = stuecke[j].0.clone();
        if let Some(i) = vorher
            && let Some(weg) = laufweg_fuer(i, j, rest)
            && let Some(letzter) = out.last_mut()
        {
            // Laufstiche höchstens `laufweg` lang entlang des Wegs, Eckpunkte des Wegs als Einstiche
            for s in weg.windows(2) {
                let schritte = ((abstand(s[0], s[1]) / laufweg.max(0.5)).ceil() as usize).max(1);
                letzter.extend((1..=schritte).map(|k| zwischen(s[0], s[1], k as f64 / schritte as f64)));
            }
            letzter.pop(); // anf[j] kommt mit dem Teil selbst
            letzter.extend(wege.remove(0));
        }
        out.extend(wege);
        rest &= !(1 << j);
        vorher = Some(j);
    }
    // weitere Teile über 63 (kommt bei Buchstaben nicht vor): ohne Laufweg anhängen
    for (w, _, _) in stuecke.iter().skip(n) {
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
}
