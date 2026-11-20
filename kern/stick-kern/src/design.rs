//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::applikation::{ApplikationVorgabe, applikation_objekte};
use crate::aussparen::{Aussparen, Teil, bedeckt, flaeche_minus};
use crate::einstellungen::{Einstellungen, SatinEinstellung, fuer_stoff};
use crate::geometrie::{
    Linie, Punkt, abstand, abstand_punkt_strecke, bezier, bogen, im_polygon, kreis, p, rechteck, zwischen,
};
use crate::nachbearbeitung::{bernina_regeln, wege_verbinden};
use crate::rahmen::{Rahmen, rahmen};
use crate::schrift::{Alphabet, Buchstabe, TextVorgabe, TextZusatz, buchstaben_zahl, text_objekte_idx, text_setzen};
use crate::schrift_ttf::{TtfSchrift, TtfStichart, ttf_text_objekte_idx};
use crate::sticharten::blanket::blanket;
use crate::sticharten::lauf::{dreifach, laufstich_mit_ecken};
use crate::sticharten::satin::{mittellinie, paare_bilden, satin, seiten_aus_mittellinie};
use crate::sticharten::tatami::{tatami, tatami_mit_teilung};
use crate::sticharten::unterlagen::{auto_satin_unterlage, flaechen_unterlage, satin_unterlage};
use crate::stiche::StichFolge;
use crate::transformation::{Transformation, abbilden, mitte as formmitte};
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
    /// Ellipse („Circle/Oval“ des Originals) mit Breite und Höhe in mm
    Oval {
        mitte: Punkt,
        breite: f64,
        hoehe: f64,
        #[serde(default)]
        start: f64,
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
            PunktAngabe::Form(Form::Oval {
                mitte,
                breite,
                hoehe,
                start,
            }) => {
                // Kreis mit dem größeren Halbmesser, in der anderen Richtung gestaucht (Sehnenfehler bleibt ≤ 0,02 mm)
                let r = breite.max(*hoehe) / 2.0;
                kreis(p(0.0, 0.0), r, 0.02, *start)
                    .into_iter()
                    .map(|q| p(mitte.x + q.x * breite / 2.0 / r, mitte.y + q.y * hoehe / 2.0 / r))
                    .collect()
            }
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
    crate::schrift::STANDARD_HOEHE_MM
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
    /// Wortabstand in % der Höhe (Vorlage 25)
    wortabstand: Option<f64>,
    /// Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt ein Zeilenumbruch im Text
    zeilenabstand: Option<f64>,
    /// Grundlinie (kreis, gerade, senkrecht, kurve, mehrfach; beim Kreis ist `position` die Kreismitte)
    grundlinie: Option<GrundlinieJson>,
    /// Kursiv in Grad (−45 … 45)
    #[serde(default)]
    kursiv: f64,
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

/// Text mit einer Computer-Schrift (TrueType/OpenType, `schrift_ttf.rs`). Die Schriften lädt der Aufrufer
/// (`Design::ttf_schriften`, `stick-formate::ttf`).
#[derive(Debug, Deserialize)]
struct TtfJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    text: String,
    /// Name der Schrift („Arial“, „Arial Bold“) oder Pfad einer .ttf/.otf-Datei
    schrift: String,
    /// Höhe der Großbuchstaben in mm
    #[serde(default = "text_hoehe")]
    hoehe: f64,
    #[serde(default = "hundert")]
    breite: f64,
    /// zusätzlicher Buchstabenabstand in mm (ohne Angabe 0)
    abstand: Option<f64>,
    /// Wortabstand in % der Höhe (Vorlage 25)
    wortabstand: Option<f64>,
    /// Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text
    zeilenabstand: Option<f64>,
    /// Grundlinie (kreis, gerade, senkrecht, kurve, mehrfach; beim Kreis ist `position` die Kreismitte)
    grundlinie: Option<GrundlinieJson>,
    /// Kursiv in Grad (−45 … 45)
    #[serde(default)]
    kursiv: f64,
    #[serde(default)]
    position: Option<Punkt>,
    #[serde(default = "mitte")]
    ausrichtung: String,
    /// tatami (Standard) | lauf | dreifach
    #[serde(default = "tatami_art")]
    stichart: String,
    winkel: Option<f64>,
    /// Tatami-Unterlage (ohne Angabe: Auto Underlay des Stils)
    unterlage: Option<bool>,
    /// Stichlänge bei Lauf/Dreifach
    laenge: Option<f64>,
}

/// Grundlinie eines Texts (`schrift::Grundlinie`; Dialog „Layout“ des Originals).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GrundlinieJson {
    /// kreis | gerade | senkrecht | kurve | mehrfach
    art: String,
    /// Radius X in mm (Vorlage 50)
    radius: Option<f64>,
    /// Radius Y in mm (Oval)
    radius_y: Option<f64>,
    /// Kreis: Justif. Angle (Grad); gerade: Line Angle
    winkel: Option<f64>,
    /// Tilt des Ovals in Grad
    #[serde(default)]
    neigung: f64,
    /// Arc Angle in Grad (nur ein Bogen)
    bogen: Option<f64>,
    /// Length der geraden Grundlinie in mm
    laenge: Option<f64>,
    /// oben (Circle CW) | unten (Circle CCW)
    lage: Option<String>,
    /// ueber | durch | unter (Position Above/Through/Below)
    stand: Option<String>,
    /// Punkte oder Form der eigenen Kurve
    punkte: Option<PunktAngabe>,
}

impl GrundlinieJson {
    fn grundlinie(self, blocksatz: bool) -> crate::schrift::Grundlinie {
        crate::schrift::Grundlinie {
            art: self.art,
            radius: self.radius.unwrap_or(crate::schrift::KREIS_RADIUS_MM),
            radius_y: self.radius_y,
            winkel: self.winkel,
            neigung: self.neigung,
            bogen: self.bogen,
            laenge: self.laenge,
            lage: self.lage,
            stand: self.stand,
            punkte: self.punkte.map(|q| q.punkte()).unwrap_or_default(),
            blocksatz,
        }
    }
}

/// Einzelner Buchstabe eines Texts (`"buchstaben"`).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuchstabeJson {
    nr: usize,
    farbe: Option<String>,
    transform: Option<Value>,
}

/// Füllmuster (`muster.rs`): Tatami-Reihen mit Einstichen auf den Musterlinien.
#[derive(Debug, Deserialize)]
struct MusterJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    umriss: PunktAngabe,
    #[serde(default)]
    loecher: Vec<PunktAngabe>,
    /// Name des Musters („001 Diamond (90)“) oder seine Nummer („001“)
    muster: String,
    /// Stichwinkel (ohne Angabe: Winkel aus dem Musternamen, sonst der Füllung)
    winkel: Option<f64>,
    /// Größe des Musters in % (Standard 100)
    #[serde(default = "hundert")]
    groesse: f64,
    unterlage: Option<bool>,
    #[serde(default)]
    aussparen: Vec<String>,
    #[serde(default)]
    ueberlappung: f64,
}

/// Applikation (`applikation.rs`): Platzierung, Heften, Abdecken mit Stopps.
#[derive(Debug, Deserialize)]
struct ApplikationJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    umriss: PunktAngabe,
    #[serde(default)]
    loecher: Vec<PunktAngabe>,
    /// Breite des Abdeck-Satins in mm (Standard 2,5)
    breite: Option<f64>,
    /// fester Satinabstand des Abdeckstichs
    abstand: Option<f64>,
    heft_breite: Option<f64>,
    heft_abstand: Option<f64>,
    platz_laenge: Option<f64>,
    /// „satin“ (Standard) oder „blanket“ (ohne Heftstich, Handbuch S. 247)
    abdeckung: Option<String>,
    /// „vorgeschnitten“ (Pre-cut, Standard) oder „zuschneiden“ (Trim in Place, mit Schnittlinie)
    stil: Option<String>,
}

fn tatami_art() -> String {
    "tatami".into()
}

/// Blanket entlang einer Linie (`sticharten/blanket.rs`).
#[derive(Debug, Deserialize)]
struct BlanketJson {
    #[serde(default = "schwarz")]
    farbe: String,
    #[serde(default)]
    name: String,
    punkte: PunktAngabe,
    #[serde(default)]
    geschlossen: bool,
    /// Abstand der Zähne in mm (Vorlage 2,0)
    abstand: Option<f64>,
    /// Breite in mm (Vorlage 3,0)
    breite: Option<f64>,
    /// „mitte“ (Center, Standard) oder „seite“ (Offset)
    versatz: Option<String>,
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
    /// Namen der Objekte, deren Fläche ausgespart wird (Knockout)
    #[serde(default)]
    aussparen: Vec<String>,
    /// Überlappung der Aussparung in mm (Standard 0 = exakt wie das Original)
    #[serde(default)]
    ueberlappung: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "art", rename_all = "lowercase")]
enum ObjektJson {
    Lauf(LaufJson),
    Dreifach(LaufJson),
    Satin(SatinJson),
    Satinlinie(SatinLinieJson),
    Blanket(BlanketJson),
    Tatami(TatamiJson),
    Text(TextJson),
    Ttf(TtfJson),
    Applikation(ApplikationJson),
    Muster(MusterJson),
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
    /// Designende: „mitte“/„rahmenmitte“ (Standard, Sprung zurück zum Ursprung), „letzter_stich“, „designmitte“,
    /// „unten_mitte“ oder „start“ (zurück zum Start) – Dialog „Start and End“ des Originals
    #[serde(default)]
    ende: Option<String>,
    /// Designstart: „rahmenmitte“ (Standard), „designmitte“, „unten_mitte“ oder „erster_stich“
    #[serde(default)]
    start: Option<String>,
}

/// Start- bzw. Endpunkt des Designs (Dialog „Start and End“, Menü Arrange; Handbuch PDF-S. 204–205). Das Original
/// setzt bei „Auto Start/End“ die **Mitte des Designs** (umschließendes Rechteck): 142 von 143 Musterdesigns mit
/// AutoStartEndObject (Klasse 0x71) beginnen und enden dort (`Analyse_Teil3/10` §6). Standard hier bleibt die
/// Rahmenmitte (Ursprung), bis am Gerät geprüft ist, wie die artista 185 ein Design mit anderem Start platziert.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Bezugspunkt {
    /// Ursprung = Rahmenmitte
    #[default]
    Rahmenmitte,
    /// Mitte des umschließenden Rechtecks der Stiche („Auto Start/End at: center of design“)
    Designmitte,
    /// Mitte der Unterkante („bottom middle“)
    UntenMitte,
    /// kein Verbinder: erster bzw. letzter Stich („First Stitch“/„Last Stitch“)
    Stich,
    /// nur Ende: zurück zum Startpunkt („return to start“)
    Start,
}

impl Bezugspunkt {
    fn aus_text(t: Option<&str>, ende: bool) -> Result<Bezugspunkt, String> {
        Ok(match t {
            None | Some("rahmenmitte") | Some("mitte") => Bezugspunkt::Rahmenmitte,
            Some("designmitte") => Bezugspunkt::Designmitte,
            Some("unten_mitte") => Bezugspunkt::UntenMitte,
            Some("letzter_stich") if ende => Bezugspunkt::Stich,
            Some("erster_stich") if !ende => Bezugspunkt::Stich,
            Some("start") if ende => Bezugspunkt::Start,
            Some(x) => return Err(format!("{} „{x}“ unbekannt", if ende { "Ende" } else { "Start" })),
        })
    }
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
    /// Blanket (Langettenstich, 0x2727) entlang einer Linie
    Blanket {
        punkte: Linie,
        geschlossen: bool,
        abstand: f64,
        breite: f64,
        /// „Center“ (Standard) oder „Offset“ (Rücken auf der Linie, Zähne nach links)
        mittig: bool,
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
        /// Flächen anderer Objekte aussparen (Knockout, `aussparen.rs`); wird vor dem Berechnen aufgelöst
        aussparen: Aussparen,
    },
    /// Text mit einem Original-Alphabet; wird vor dem Berechnen in Satin-/Lauf-Objekte aufgelöst
    Text { alphabet: String, vorgabe: TextVorgabe },
    /// Fläche mit Füllmuster (Program Split): Tatami-Reihen, Einstiche auf den gekachelten Musterlinien
    Musterfuellung {
        umriss: Linie,
        loecher: Vec<Linie>,
        muster: String,
        winkel: Option<f64>,
        groesse: f64,
        unterlage: Option<bool>,
        aussparen: Aussparen,
    },
    /// Applikation (Platzierung, Heften, Abdecken mit Stopps); wird vor dem Berechnen aufgelöst
    Applikation(ApplikationVorgabe),
    /// Text mit einer Computer-Schrift (TrueType/OpenType); wird vor dem Berechnen aufgelöst
    TtfText {
        schrift: String,
        vorgabe: TextVorgabe,
        stichart: TtfStichart,
    },
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
            ObjektArt::Blanket {
                punkte,
                geschlossen,
                abstand,
                breite,
                mittig,
            } => Ok(vec![blanket(punkte, *geschlossen, *abstand, *breite, *mittig)]),
            ObjektArt::Text { alphabet, .. } => Err(format!("Text mit Alphabet „{alphabet}“ wurde nicht aufgelöst")),
            ObjektArt::TtfText { schrift, .. } => Err(format!("Text mit Schrift „{schrift}“ wurde nicht aufgelöst")),
            ObjektArt::Applikation(_) => Err("Applikation wurde nicht aufgelöst".into()),
            ObjektArt::Tatami {
                umriss,
                loecher,
                winkel,
                unterlage,
                ..
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
            ObjektArt::Musterfuellung {
                umriss,
                loecher,
                muster,
                winkel,
                groesse,
                unterlage,
                ..
            } => {
                let m = crate::muster::finden(muster).ok_or_else(|| format!("Füllmuster „{muster}“ gibt es nicht"))?;
                let w = winkel.or(m.winkel).unwrap_or(e.tatami.winkel);
                let mut wege = Vec::new();
                let mut start = start;
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    wege.extend(flaechen_unterlage(umriss, loecher, w, &e.unterlage, start));
                    start = wege.last().and_then(|l: &Linie| l.last().copied());
                }
                let t = m.teilung(*groesse);
                wege.extend(tatami_mit_teilung(umriss, loecher, &e.tatami, Some(w), start, Some(&t)));
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
    /// Designende wie „Start und Ende“ im Original (Handbuch PDF-S. 205: Auto End – Mitte): nach dem letzten Stich ein
    /// Sprung zurück zum Ursprung (Rahmenmitte). Belegt an den Designs der Maschine: 34 von 51 enden genau auf 0,0
    /// (die Zeichensätze, deren Ursprung am Zeilenanfang liegt, auf der Grundlinie). Standard an.
    /// Start- und Endpunkt des Designs (Dialog „Start and End“)
    pub start: Bezugspunkt,
    pub ende: Bezugspunkt,
    /// geladene Computer-Schriften (Name wie im Design); füllt der Aufrufer, siehe `ttf_bedarf`
    pub ttf_schriften: BTreeMap<String, TtfSchrift>,
    /// Transformation je Objekt (Nummer in `objekte`), JSON `"transform"` am Objekt (`transformation.rs`)
    pub transformationen: BTreeMap<usize, Transformation>,
    /// Transformation des ganzen Designs (um seine Mitte), JSON `"transform"` oben im Design
    pub transformation: Transformation,
    /// Farben für `^` und einzelne Buchstaben je Text (Nummer in `objekte`), JSON `"farben"`/`"buchstaben"`
    pub buchstaben: BTreeMap<usize, TextZusatz>,
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
        for o in &d.objekte {
            let block = match o {
                ObjektJson::Text(t) => t.ausrichtung == "block" && t.grundlinie.is_none(),
                ObjektJson::Ttf(t) => t.ausrichtung == "block" && t.grundlinie.is_none(),
                _ => false,
            };
            if block {
                return Err(
                    "Ausrichtung „block“ braucht eine Grundlinie mit fester Länge (kurve, gerade mit laenge, \
                     kreis mit bogen)"
                        .into(),
                );
            }
            if let ObjektJson::Applikation(a) = o {
                if let Some(x) = a.abdeckung.as_deref().filter(|x| !matches!(*x, "satin" | "blanket")) {
                    return Err(format!("Abdeckung „{x}“ unbekannt (satin, blanket)"));
                }
                if let Some(x) = a
                    .stil
                    .as_deref()
                    .filter(|x| !matches!(*x, "vorgeschnitten" | "zuschneiden"))
                {
                    return Err(format!(
                        "Applikationsstil „{x}“ unbekannt (vorgeschnitten, zuschneiden)"
                    ));
                }
            }
        }
        let objekte: Vec<Objekt> = d
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
                                "links" | "block" => 0.0,
                                "rechts" => 1.0,
                                _ => 0.5,
                            },
                            farbe: t.farbe.clone(),
                            wortabstand: t.wortabstand.map(|w| w / 100.0),
                            zeilenabstand: t.zeilenabstand.map(|z| z / 100.0),
                            grundlinie: t.grundlinie.map(|g| g.grundlinie(t.ausrichtung == "block")),
                            kursiv: t.kursiv,
                            unterlage: match Unterlage::from(t.unterlage) {
                                Unterlage::Auto => None,
                                Unterlage::Arten(l) => Some(l),
                            },
                        },
                    },
                    farbe: t.farbe,
                },
                ObjektJson::Muster(m) => Objekt {
                    farbe: m.farbe,
                    name: m.name,
                    art: ObjektArt::Musterfuellung {
                        umriss: m.umriss.punkte(),
                        loecher: m.loecher.iter().map(|l| l.punkte()).collect(),
                        muster: m.muster,
                        winkel: m.winkel,
                        groesse: m.groesse,
                        unterlage: m.unterlage,
                        aussparen: Aussparen {
                            namen: m.aussparen,
                            ueberlappung: m.ueberlappung,
                        },
                    },
                },
                ObjektJson::Applikation(a) => {
                    let st = ApplikationVorgabe::default();
                    Objekt {
                        name: a.name.clone(),
                        art: ObjektArt::Applikation(ApplikationVorgabe {
                            umriss: a.umriss.punkte(),
                            loecher: a.loecher.iter().map(PunktAngabe::punkte).collect(),
                            farbe: a.farbe.clone(),
                            breite: a.breite.unwrap_or(st.breite),
                            abstand: a.abstand,
                            heft_breite: a.heft_breite.unwrap_or(st.heft_breite),
                            heft_abstand: a.heft_abstand.unwrap_or(st.heft_abstand),
                            platz_laenge: a.platz_laenge.unwrap_or(st.platz_laenge),
                            blanket: a.abdeckung.as_deref() == Some("blanket"),
                            zuschneiden: a.stil.as_deref() == Some("zuschneiden"),
                        }),
                        farbe: a.farbe,
                    }
                }
                ObjektJson::Ttf(t) => Objekt {
                    name: if t.name.is_empty() { t.text.clone() } else { t.name },
                    art: ObjektArt::TtfText {
                        schrift: t.schrift,
                        vorgabe: TextVorgabe {
                            text: t.text,
                            hoehe: t.hoehe,
                            breite_prozent: t.breite,
                            abstand: t.abstand,
                            position: t.position.unwrap_or(p(0.0, 0.0)),
                            ausrichtung: match t.ausrichtung.as_str() {
                                "links" | "block" => 0.0,
                                "rechts" => 1.0,
                                _ => 0.5,
                            },
                            farbe: t.farbe.clone(),
                            wortabstand: t.wortabstand.map(|w| w / 100.0),
                            zeilenabstand: t.zeilenabstand.map(|z| z / 100.0),
                            grundlinie: t.grundlinie.map(|g| g.grundlinie(t.ausrichtung == "block")),
                            kursiv: t.kursiv,
                            unterlage: None,
                        },
                        stichart: match t.stichart.as_str() {
                            "lauf" => TtfStichart::Lauf {
                                laenge: t.laenge,
                                dreifach: false,
                            },
                            "dreifach" => TtfStichart::Lauf {
                                laenge: t.laenge,
                                dreifach: true,
                            },
                            _ => TtfStichart::Tatami {
                                winkel: t.winkel,
                                unterlage: t.unterlage,
                            },
                        },
                    },
                    farbe: t.farbe,
                },
                ObjektJson::Blanket(b) => Objekt {
                    farbe: b.farbe,
                    name: b.name,
                    art: ObjektArt::Blanket {
                        punkte: b.punkte.punkte(),
                        geschlossen: b.geschlossen,
                        abstand: b.abstand.unwrap_or(crate::sticharten::blanket::ABSTAND),
                        breite: b.breite.unwrap_or(crate::sticharten::blanket::BREITE),
                        mittig: b.versatz.as_deref() != Some("seite"),
                    },
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
                        aussparen: Aussparen {
                            namen: t.aussparen,
                            ueberlappung: t.ueberlappung,
                        },
                    },
                },
            })
            .collect();
        for o in &objekte {
            if let ObjektArt::Text { vorgabe, .. } | ObjektArt::TtfText { vorgabe, .. } = &o.art
                && let Some(g) = &vorgabe.grundlinie
            {
                g.pruefen(&vorgabe.text)?;
            }
            if let ObjektArt::Text { vorgabe, .. } | ObjektArt::TtfText { vorgabe, .. } = &o.art
                && vorgabe.kursiv.abs() > 45.0
            {
                return Err(format!("Kursiv {}° – höchstens 45° (Handbuch)", vorgabe.kursiv));
            }
        }
        let transform_lesen = |t: &Value, wo: &str| -> Result<Transformation, String> {
            let t: Transformation =
                serde_json::from_value(t.clone()).map_err(|e| format!("{wo}: transform ungültig: {e}"))?;
            t.matrix(p(0.0, 0.0)).map_err(|e| format!("{wo}: {e}"))?;
            Ok(t)
        };
        let mut transformationen = BTreeMap::new();
        if let Some(liste) = v.get("objekte").and_then(Value::as_array) {
            for (i, o) in liste.iter().enumerate() {
                if let Some(t) = o.get("transform") {
                    let t = transform_lesen(t, &format!("Objekt {}", i + 1))?;
                    if !t.ist_leer() {
                        transformationen.insert(i, t);
                    }
                }
            }
        }
        let transformation = match v.get("transform") {
            Some(t) => transform_lesen(t, "Design")?,
            None => Transformation::default(),
        };
        // Farben für `^` und einzelne Buchstaben je Text
        let mut buchstaben = BTreeMap::new();
        let liste = v.get("objekte").and_then(Value::as_array);
        for (i, o) in objekte.iter().enumerate() {
            let (ObjektArt::Text { vorgabe, .. } | ObjektArt::TtfText { vorgabe, .. }) = &o.art else {
                continue;
            };
            let roh = liste.and_then(|l| l.get(i));
            let wo = format!("Objekt {}", i + 1);
            let farben: Vec<String> = match roh.and_then(|r| r.get("farben")) {
                Some(f) => serde_json::from_value(f.clone()).map_err(|e| format!("{wo}: farben ungültig: {e}"))?,
                None => Vec::new(),
            };
            let wechsel = vorgabe.text.matches('^').count();
            if wechsel > farben.len() {
                return Err(format!(
                    "{wo}: {wechsel} Farbwechsel (^) im Text, aber {} Farben in „farben“",
                    farben.len()
                ));
            }
            let n = buchstaben_zahl(&vorgabe.text);
            let mut einzeln = Vec::new();
            if let Some(e) = roh.and_then(|r| r.get("buchstaben")) {
                let l: Vec<BuchstabeJson> =
                    serde_json::from_value(e.clone()).map_err(|e| format!("{wo}: buchstaben ungültig: {e}"))?;
                for b in l {
                    if b.nr == 0 || b.nr > n {
                        return Err(format!("{wo}: Buchstabe {} gibt es nicht (1 … {n})", b.nr));
                    }
                    let transform = match &b.transform {
                        Some(t) => transform_lesen(t, &format!("{wo}, Buchstabe {}", b.nr))?,
                        None => Transformation::default(),
                    };
                    einzeln.push(Buchstabe {
                        nr: b.nr,
                        farbe: b.farbe,
                        transform,
                    });
                }
            }
            if !farben.is_empty() || !einzeln.is_empty() {
                buchstaben.insert(i, TextZusatz { farben, einzeln });
            }
        }
        Ok(Design {
            transformationen,
            transformation,
            buchstaben,
            name: d.name,
            rahmen: rahmen(&d.rahmen)?,
            einstellungen: e,
            objekte,
            alphabete: BTreeMap::new(),
            start: Bezugspunkt::aus_text(d.start.as_deref(), false)?,
            ende: Bezugspunkt::aus_text(d.ende.as_deref(), true)?,
            ttf_schriften: BTreeMap::new(),
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

    /// Computer-Schriften, die das Design braucht: Name → benutzte Zeichen (zum Laden durch den Aufrufer).
    pub fn ttf_bedarf(&self) -> BTreeMap<String, std::collections::BTreeSet<char>> {
        let mut m: BTreeMap<String, std::collections::BTreeSet<char>> = BTreeMap::new();
        for o in &self.objekte {
            if let ObjektArt::TtfText { schrift, vorgabe, .. } = &o.art {
                m.entry(schrift.clone()).or_default().extend(vorgabe.text.chars());
            }
        }
        m
    }

    /// Objekte mit aufgelösten Texten (Buchstaben als Satin-/Lauf-Objekte). Fehlende Zeichen werden übersprungen
    /// und im zweiten Wert gemeldet.
    pub fn objekte_aufgeloest(&self) -> Result<(Vec<Objekt>, Vec<char>), String> {
        let mut out = Vec::new();
        let mut fehlend = Vec::new();
        // je aufgelöstem Objekt: Nummer des Objekts im Design (für das Aussparen nach Namen)
        let mut herkunft: Vec<usize> = Vec::new();
        for (nr, o) in self.objekte.iter().enumerate() {
            match &o.art {
                ObjektArt::Text { alphabet, vorgabe } => {
                    let a = self
                        .alphabete
                        .get(alphabet)
                        .or_else(|| self.alphabete.values().find(|a| a.name.eq_ignore_ascii_case(alphabet)))
                        .ok_or_else(|| format!("Alphabet „{alphabet}“ nicht geladen"))?;
                    let abstand = vorgabe.abstand.unwrap_or(a.abstand_anteil * vorgabe.hoehe);
                    let (obj, f) = text_setzen(
                        vorgabe,
                        abstand,
                        self.buchstaben.get(&nr),
                        self.einstellungen.tatami.winkel,
                        |z| text_objekte_idx(a, z),
                    );
                    out.extend(obj);
                    fehlend.extend(f);
                }
                ObjektArt::Applikation(v) => {
                    out.extend(applikation_objekte(
                        v,
                        if o.name.is_empty() { "Applikation" } else { &o.name },
                    ));
                }
                ObjektArt::TtfText {
                    schrift,
                    vorgabe,
                    stichart,
                } => {
                    let s = self
                        .ttf_schriften
                        .get(schrift)
                        .ok_or_else(|| format!("Schrift „{schrift}“ nicht geladen"))?;
                    let abstand = vorgabe
                        .abstand
                        .unwrap_or(crate::schrift::BUCHSTABENABSTAND_ANTEIL * vorgabe.hoehe);
                    let (obj, f) = text_setzen(
                        vorgabe,
                        abstand,
                        self.buchstaben.get(&nr),
                        self.einstellungen.tatami.winkel,
                        |z| ttf_text_objekte_idx(s, z, stichart),
                    );
                    out.extend(obj);
                    fehlend.extend(f);
                }
                _ => out.push(o.clone()),
            }
            herkunft.resize(out.len(), nr);
        }
        let out = self.transformieren(out, &herkunft)?;
        Ok((self.aussparen_aufloesen(out, &herkunft)?, fehlend))
    }

    /// Transformationen anwenden: erst je Objekt (um die Mitte seiner aufgelösten Teile), dann das ganze Design.
    fn transformieren(&self, mut out: Vec<Objekt>, herkunft: &[usize]) -> Result<Vec<Objekt>, String> {
        let ws = self.einstellungen.tatami.winkel;
        for (nr, t) in &self.transformationen {
            let idx: Vec<usize> = (0..out.len()).filter(|&j| herkunft[j] == *nr).collect();
            let teil: Vec<Objekt> = idx.iter().map(|&j| out[j].clone()).collect();
            let m = t.matrix(formmitte(&teil))?;
            for &j in &idx {
                out[j] = abbilden(&out[j], &m, ws);
            }
        }
        if !self.transformation.ist_leer() {
            let m = self.transformation.matrix(formmitte(&out))?;
            out = out.iter().map(|o| abbilden(o, &m, ws)).collect();
        }
        Ok(out)
    }

    /// Aussparungen (Knockout) auflösen: Füllflächen mit `aussparen` verlieren die Flächen der genannten Objekte;
    /// zerfällt eine Fläche dabei, folgen die Teile als eigene Objekte gleicher Farbe (größter zuerst).
    fn aussparen_aufloesen(&self, objekte: Vec<Objekt>, herkunft: &[usize]) -> Result<Vec<Objekt>, String> {
        let mut neu: Vec<Option<Vec<Teil>>> = Vec::with_capacity(objekte.len());
        for (i, o) in objekte.iter().enumerate() {
            let (umriss, loecher, a) = match &o.art {
                ObjektArt::Tatami {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                }
                | ObjektArt::Musterfuellung {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                } if !aussparen.namen.is_empty() => (umriss, loecher, aussparen),
                _ => {
                    neu.push(None);
                    continue;
                }
            };
            let mut schneider = Vec::new();
            for name in &a.namen {
                let quellen: Vec<usize> = (0..self.objekte.len())
                    .filter(|&k| self.objekte[k].name == *name)
                    .collect();
                if quellen.is_empty() {
                    return Err(format!("Objekt „{name}“ zum Aussparen gibt es nicht"));
                }
                for (j, x) in objekte.iter().enumerate() {
                    if j != i && quellen.contains(&herkunft[j]) {
                        schneider.extend(bedeckt(&x.art, a.ueberlappung));
                    }
                }
            }
            neu.push(Some(flaeche_minus(umriss, loecher, &schneider)));
        }
        let mut out = Vec::with_capacity(objekte.len());
        for (o, n) in objekte.into_iter().zip(neu) {
            let Some(teile) = n else {
                out.push(o);
                continue;
            };
            for (u, l) in teile {
                let mut t = o.clone();
                if let ObjektArt::Tatami {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                }
                | ObjektArt::Musterfuellung {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                } = &mut t.art
                {
                    (*umriss, *loecher, *aussparen) = (u, l, Aussparen::default());
                }
                out.push(t);
            }
        }
        Ok(out)
    }

    /// Stiche berechnen: Objekte -> Wege -> verbinden -> Bernina-Regeln (Mindeststich, Auto Jump).
    pub fn berechnen(&self) -> Result<StichFolge, String> {
        let (objekte, _) = self.objekte_aufgeloest()?;
        let teile_ab = |start: Punkt| -> Result<Vec<(String, Vec<Linie>)>, String> {
            let mut teile = Vec::new();
            let mut pos = start;
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
            Ok(teile)
        };
        let null = p(0.0, 0.0);
        let mut teile = teile_ab(null)?;
        // Mitte bzw. Unterkante des Designs aus den Stichen (umschließendes Rechteck)
        let punkte = || teile.iter().flat_map(|(_, w)| w.iter().flatten());
        let (x0, x1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let (y0, y1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
        let (mitte, unten) = if x0 <= x1 {
            (p((x0 + x1) / 2.0, (y0 + y1) / 2.0), p((x0 + x1) / 2.0, y0))
        } else {
            (null, null)
        };
        let erster = teile
            .first()
            .and_then(|(_, w)| w.first())
            .and_then(|l| l.first())
            .copied();
        let start = match self.start {
            Bezugspunkt::Designmitte => mitte,
            Bezugspunkt::UntenMitte => unten,
            Bezugspunkt::Stich => erster.unwrap_or(null),
            Bezugspunkt::Rahmenmitte | Bezugspunkt::Start => null,
        };
        if start != null {
            teile = teile_ab(start)?;
        }
        let folge = wege_verbinden(&teile, &self.einstellungen.verbinder, start);
        let mut folge = bernina_regeln(&folge);
        let ende = match self.ende {
            Bezugspunkt::Rahmenmitte => Some(null),
            Bezugspunkt::Designmitte => Some(mitte),
            Bezugspunkt::UntenMitte => Some(unten),
            Bezugspunkt::Start => Some(start),
            Bezugspunkt::Stich => None,
        };
        if let Some(z) = ende
            && let Some(letzter) = folge.stiche.last()
            && abstand(letzter.p(), z) > 1e-9
        {
            folge.sprung(z);
        }
        Ok(folge)
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

    /// „Start and End“: Auto Start/End in der Designmitte wie die Musterdesigns mit AutoStartEndObject
    #[test]
    fn start_und_ende_designmitte() {
        let json = r##"{"rahmen": "artista 130 x 100", "start": "designmitte", "ende": "designmitte", "objekte": [
            {"art": "tatami", "umriss": {"rechteck": {"mitte": [20, 10], "breite": 10, "hoehe": 6}}}
        ]}"##;
        let d = Design::aus_json(json).unwrap();
        let f = d.berechnen().unwrap();
        let st: Vec<Punkt> = f
            .stiche
            .iter()
            .filter(|s| s.art == crate::Art::Stich)
            .map(|s| s.p())
            .collect();
        let (x0, x1) = st
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let (y0, y1) = st
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
        let ende = f.stiche.last().unwrap().p();
        assert!(
            abstand(ende, p((x0 + x1) / 2.0, (y0 + y1) / 2.0)) < 0.3,
            "Ende {ende:?}"
        );
        // letzter Stich: kein Sprung am Ende
        let l = Design::aus_json(&json.replace(r#""ende": "designmitte""#, r#""ende": "letzter_stich""#)).unwrap();
        assert_eq!(l.berechnen().unwrap().stiche.last().unwrap().art, crate::Art::Stich);
        assert!(Design::aus_json(&json.replace("designmitte\", \"ende", "irgendwo\", \"ende")).is_err());
    }
}
