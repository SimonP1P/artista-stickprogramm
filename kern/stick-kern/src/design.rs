//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::einstellungen::{Einstellungen, SatinEinstellung, fuer_stoff};
use crate::geometrie::{Linie, Punkt, abstand, bezier, bogen, kreis, p, rechteck};
use crate::nachbearbeitung::{bernina_regeln, wege_verbinden};
use crate::rahmen::{Rahmen, rahmen};
use crate::schrift::{Alphabet, TextVorgabe, text_objekte};
use crate::sticharten::lauf::{dreifach, laufstich_mit_ecken};
use crate::sticharten::satin::{paare_bilden, satin, seiten_aus_mittellinie};
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
        /// fester Abstand in mm (None = Einstellung des Stoffs/Stils, Auto Spacing)
        abstand: Option<f64>,
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
    abstand: Option<f64>,
    e: &Einstellungen,
) -> Result<Vec<Linie>, String> {
    let (a, b) = paare_bilden(a, b)?;
    let satin_e = match abstand {
        Some(x) => SatinEinstellung {
            auto_spacing: false,
            abstand_fest: x,
            ..e.satin.clone()
        },
        None => e.satin.clone(),
    };
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
                abstand,
            } => satin_wege(seite_a, seite_b, unterlage, *abstand, e),
            ObjektArt::SatinLinie {
                punkte,
                breite,
                unterlage,
            } => {
                let (a, b) = seiten_aus_mittellinie(punkte, *breite);
                satin_wege(&a, &b, unterlage, None, e)
            }
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
                        abstand: s.abstand,
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
