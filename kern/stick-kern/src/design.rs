//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::applikation::{ApplikationVorgabe, applikation_objekte};
use crate::aussparen::{Aussparen, Teil, bedeckt, flaeche_minus};
use crate::einstellungen::{Einstellungen, SatinEinstellung, TatamiEinstellung, fuer_stoff};
use crate::geometrie::{
    Linie, Punkt, abstand, abstand_punkt_strecke, bezier, bogen, im_polygon, kreis, p, rechteck, zwischen,
};
use crate::nachbearbeitung::{bernina_regeln, wege_verbinden_mit};
use crate::rahmen::{Rahmen, rahmen};
use crate::schrift::{
    Alphabet, Buchstabe, SchriftStichart, TextVorgabe, TextZusatz, buchstaben_zahl, text_objekte_idx, text_setzen,
};
use crate::schrift_ttf::{TtfSchrift, TtfStichart, ttf_text_objekte_idx};
use crate::sticharten::blanket::blanket;
use crate::sticharten::lauf::{dreifach, laufstich_mit_ecken};
use crate::sticharten::satin::{mittellinie, paare_bilden, satin, satinlinie_seiten};
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

/// Elastic Lettering im JSON: `{"art": "gerade", "oben": [links, rechts], "unten": [links, rechts]}` oder
/// `{"art": "gebogen", "oben": mitte | [links, mitte, rechts], "unten": …}` (mm, + nach oben).
fn huelle_lesen(h: &Value) -> Result<crate::schrift::Huelle, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct HuelleJson {
        art: String,
        #[serde(default)]
        oben: Value,
        #[serde(default)]
        unten: Value,
    }
    let j: HuelleJson = serde_json::from_value(h.clone()).map_err(|e| e.to_string())?;
    let gebogen = match j.art.as_str() {
        "gerade" => false,
        "gebogen" => true,
        x => return Err(format!("Art „{x}“ unbekannt (gerade, gebogen)")),
    };
    let griffe = |w: &Value| -> Result<[f64; 3], String> {
        let l: Vec<f64> = match w {
            Value::Null => Vec::new(),
            Value::Number(n) => vec![n.as_f64().unwrap_or(0.0)],
            _ => serde_json::from_value(w.clone()).map_err(|e| e.to_string())?,
        };
        match (gebogen, l.as_slice()) {
            (_, []) => Ok([0.0; 3]),
            (false, [a, b]) => Ok([*a, (a + b) / 2.0, *b]),
            (true, [m]) => Ok([0.0, *m, 0.0]),
            (true, [a, m, b]) => Ok([*a, *m, *b]),
            _ => Err(if gebogen {
                "gebogen: oben/unten = Mitte oder [links, Mitte, rechts]".into()
            } else {
                "gerade: oben/unten = [links, rechts]".into()
            }),
        }
    };
    Ok(crate::schrift::Huelle {
        gebogen,
        oben: griffe(&j.oben)?,
        unten: griffe(&j.unten)?,
    })
}

/// Einzelner Buchstabe eines Texts (`"buchstaben"`).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuchstabeJson {
    nr: usize,
    farbe: Option<String>,
    transform: Option<Value>,
    /// neue Stichwinkel: Strecken [[x1, y1], [x2, y2]] quer über die Säulen
    #[serde(default)]
    winkellinien: Vec<[Punkt; 2]>,
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
    /// mitte (Center, Standard) | seite (Offset: Linie = Kante, Säule links davon)
    versatz: Option<String>,
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
    /// mehrere Stichwinkel: Strecken [[x1, y1], [x2, y2]], die den Umriss je zweimal schneiden (Turning)
    #[serde(default)]
    winkellinien: Vec<[Punkt; 2]>,
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

/// Standardhöhe eines Monogramms: kleinste empfohlene Höhe der Alphabete Diamond2/Diamond3 (18 mm,
/// `Analyse/daten/alphabets.md`) – die Lettering-Vorlage (10 mm) liegt darunter.
pub const MONOGRAMM_HOEHE_MM: f64 = 18.0;

/// Monogramm (Handbuch Kap. 21 „Creating monograms“): Initialen mit den Monogramm-Alphabeten **Diamond2** (2 Buchstaben)
/// bzw. **Diamond3** (3). Belegung der Zeichen aus den Alphabeten gelesen (`Analyse_Teil3/09` §7): `A`–`Z` = linker
/// Buchstabe, `a`–`z` = rechter, `!` … `:` = mittlerer (nur Diamond3, 18 statt 16 mm hoch), `]` = Raute als Rahmen,
/// `[` = Zierwinkel – beide reichen von ihrem Platz nach links und liegen mittig um die Buchstaben, wenn sie direkt
/// nach ihnen kommen. Das Objekt wird hier in ein `text`-Objekt umgeschrieben (alle Schriftoptionen gelten).
fn monogramme_aufloesen(v: &Value) -> Result<Value, String> {
    let mut v = v.clone();
    let Some(liste) = v.get_mut("objekte").and_then(Value::as_array_mut) else {
        return Ok(v);
    };
    for (i, o) in liste.iter_mut().enumerate() {
        if o.get("art").and_then(Value::as_str) != Some("monogramm") {
            continue;
        }
        let wo = format!("Objekt {}", i + 1);
        let Some(m) = o.as_object_mut() else {
            continue;
        };
        let ini: Vec<char> = m
            .remove("initialen")
            .and_then(|x| x.as_str().map(str::to_uppercase))
            .ok_or_else(|| format!("{wo}: monogramm braucht „initialen“ (2 oder 3 Buchstaben)"))?
            .chars()
            .collect();
        if !(2..=3).contains(&ini.len()) || !ini.iter().all(char::is_ascii_uppercase) {
            return Err(format!("{wo}: initialen: 2 oder 3 Buchstaben A–Z (Diamond2/Diamond3)"));
        }
        let mut text = String::new();
        text.push(ini[0]);
        if ini.len() == 3 {
            text.push((b'!' + (ini[1] as u8 - b'A')) as char);
        }
        text.push(ini[ini.len() - 1].to_ascii_lowercase());
        let rahmen = match m.remove("rahmen").as_ref().and_then(Value::as_str) {
            None | Some("keine") => None,
            Some("raute") => Some(']'),
            Some("zier") => Some('['),
            Some(x) => return Err(format!("{wo}: Rahmen „{x}“ unbekannt (raute, zier, keine)")),
        };
        if let Some(f) = m.remove("rahmen_farbe") {
            if rahmen.is_none() {
                return Err(format!("{wo}: rahmen_farbe ohne rahmen"));
            }
            text.push('^');
            let farben = m.entry("farben").or_insert_with(|| Value::Array(Vec::new()));
            farben
                .as_array_mut()
                .ok_or_else(|| format!("{wo}: farben muss eine Liste sein"))?
                .push(f);
        }
        if let Some(z) = rahmen {
            text.push(z);
        }
        let alphabet = if ini.len() == 3 { "Diamond3" } else { "Diamond2" };
        m.insert("art".into(), Value::from("text"));
        m.insert("text".into(), Value::from(text));
        m.insert("alphabet".into(), Value::from(alphabet));
        m.entry("hoehe").or_insert_with(|| Value::from(MONOGRAMM_HOEHE_MM));
    }
    Ok(v)
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
        /// „Center“ (Standard) oder „Offset“ (Linie = Kante, Säule links davon)
        mittig: bool,
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
    /// Fläche mit mehreren Stichwinkeln (Turning, `sticharten::wendung`): Step-Stiche längs der Winkellinien
    Wendefuellung {
        umriss: Linie,
        winkellinien: Vec<[Punkt; 2]>,
        unterlage: Option<bool>,
    },
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
    /// Step statt Satin in der Säule (Schrift mit Stichart Step, `sticharten::satin::step_reihen`)
    pub step: bool,
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

/// Aufgelöste Objekte, fehlende Zeichen, Herkunft (Nummer des Objekts im Design) je Objekt
type Aufgeloest = (Vec<Objekt>, Vec<char>, Vec<usize>);

/// Double Run: dieselben Stiche rückwärts anhängen (beginnen, wo das Objekt endet – kein Verbindungsstich).
fn doppellauf(wege: &mut Vec<Linie>) {
    let rueck: Vec<Linie> = wege.iter().rev().map(|w| w.iter().rev().copied().collect()).collect();
    for (i, r) in rueck.into_iter().enumerate() {
        if i == 0
            && let Some(letzte) = wege.last_mut()
        {
            letzte.extend(r.into_iter().skip(1));
        } else {
            wege.push(r);
        }
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
    let deck = if o.step {
        // Step-Reihen: Abstand, Zugausgleich der Füllung, ohne Kurzstiche
        let mut se = satin_e.clone();
        se.auto_spacing = false;
        se.abstand_fest = e.tatami.abstand;
        se.zugausgleich = e.tatami.zugausgleich;
        se.kurzstiche.an = false;
        crate::sticharten::satin::step_reihen(&satin(&a, &b, &se)?, &e.tatami)
    } else {
        satin(&a, &b, &satin_e)?
    };
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
    // je Teil die möglichen Richtungen: wie im Alphabet und – bei Satinsäulen – rückwärts. Das Original stickt Säulen
    // in beiden Richtungen und wählt Reihenfolge und Richtung selbst („Closest Join“; gemessen an 4558 Satinstücken
    // der Schrift-Musterdesigns: 62 % vorwärts, 38 % rückwärts, nur 150 von 938 Buchstaben mit ≥ 2 Säulen in
    // Alphabet-Reihenfolge, `Analyse_Teil3/11` §5). Je Richtung: Unterlage, Decklage (Wege), Mittellinie.
    struct Richtung {
        unter: Linie,
        wege: Vec<Linie>,
        mitte: Option<Linie>,
    }
    let mut stuecke: Vec<(Vec<Richtung>, Option<&Linie>)> = Vec::new();
    for (i, t) in teile.iter().enumerate() {
        let mut richt = Vec::new();
        match &t.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => {
                for rueck in [false, true] {
                    let (a, b): (Linie, Linie) = if rueck {
                        (
                            seite_a.iter().rev().copied().collect(),
                            seite_b.iter().rev().copied().collect(),
                        )
                    } else {
                        (seite_a.clone(), seite_b.clone())
                    };
                    let (unter, d) = satin_getrennt(&a, &b, unterlage, optionen, e)?;
                    if d.is_empty() {
                        continue;
                    }
                    let mitte = paare_bilden(&a, &b).ok().map(|(x, y)| mittellinie(&x, &y));
                    richt.push(Richtung {
                        unter,
                        wege: vec![d],
                        mitte,
                    });
                }
            }
            _ => {
                let w: Vec<Linie> = t.wege(e, None)?.into_iter().filter(|w| !w.is_empty()).collect();
                if !w.is_empty() {
                    richt.push(Richtung {
                        unter: Vec::new(),
                        wege: w,
                        mitte: None,
                    });
                }
            }
        }
        if !richt.is_empty() {
            stuecke.push((richt, umrisse.get(i).filter(|u| u.len() >= 3)));
        }
    }
    let n = stuecke.len().min(63);
    if n == 0 {
        return Ok(Vec::new());
    }
    // Knoten = (Teil, Richtung)
    let knoten: Vec<(usize, usize)> = (0..n)
        .flat_map(|j| (0..stuecke[j].0.len()).map(move |d| (j, d)))
        .collect();
    let r = |v: usize| &stuecke[knoten[v].0].0[knoten[v].1];
    let teil = |v: usize| knoten[v].0;
    let m = knoten.len();
    let anf: Vec<Punkt> = (0..m).map(|v| r(v).wege[0][0]).collect();
    let end: Vec<Punkt> = (0..m)
        .map(|v| *r(v).wege.last().and_then(|l| l.last()).unwrap())
        .collect();
    let bedeckt = |q: Punkt| -> u64 {
        stuecke[..n]
            .iter()
            .enumerate()
            .filter(|(_, (_, u))| u.is_some_and(|r| im_polygon(q, std::slice::from_ref(r))))
            .fold(0u64, |mk, (k, _)| mk | (1 << k))
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
    // je Verbindung v → w: gerade (Masken, Länge) und über die Mittellinie von w (Masken des Zugangs, Weg)
    struct Verbindung {
        gerade: Vec<u64>,
        gerade_laenge: f64,
        ueber_mitte: Option<(Vec<u64>, Linie)>,
    }
    let mut verb: Vec<Vec<Option<Verbindung>>> = (0..m).map(|_| (0..m).map(|_| None).collect()).collect();
    for v in 0..m {
        for w in (0..m).filter(|&w| teil(w) != teil(v)) {
            let ueber_mitte = r(w).mitte.as_ref().filter(|mt| mt.len() >= 2).map(|mt| {
                let k = (0..mt.len())
                    .min_by(|&x, &y| abstand(end[v], mt[x]).total_cmp(&abstand(end[v], mt[y])))
                    .unwrap();
                let mut weg: Linie = vec![end[v]];
                weg.extend(mt[..=k].iter().rev().copied());
                weg.push(anf[w]);
                (masken(end[v], mt[k]), weg)
            });
            verb[v][w] = Some(Verbindung {
                gerade: masken(end[v], anf[w]),
                gerade_laenge: abstand(end[v], anf[w]),
                ueber_mitte,
            });
        }
    }
    let weglaenge = |w: &Linie| w.windows(2).map(|s| abstand(s[0], s[1])).sum::<f64>();
    // Laufweg v → w, wenn verdeckt (`rest` = noch ausstehende Teile einschließlich w): Punkte von end[v] bis anf[w]
    let laufweg_fuer = |v: usize, w: usize, rest: u64| -> Option<Linie> {
        let vb = verb[v][w].as_ref()?;
        if vb.gerade.iter().all(|&mk| mk & rest != 0) {
            return Some(vec![end[v], anf[w]]);
        }
        let (zugang, weg) = vb.ueber_mitte.as_ref()?;
        zugang.iter().all(|&mk| mk & rest != 0).then(|| weg.clone())
    };
    let alle: u64 = (1u64 << n) - 1;
    let anfahrt = |w: usize| start.map_or(0.0, |s| abstand(s, anf[w]));
    // nächster Knoten (Rückfall bei vielen Teilen)
    let gierig = || -> Vec<usize> {
        let mut folge = Vec::new();
        let mut rest = alle;
        let mut pos = start.unwrap_or(anf[0]);
        while rest != 0 {
            let w = (0..m)
                .filter(|&w| rest & (1 << teil(w)) != 0)
                .min_by(|&x, &y| abstand(pos, anf[x]).total_cmp(&abstand(pos, anf[y])))
                .unwrap();
            folge.push(w);
            rest &= !(1 << teil(w));
            pos = end[w];
        }
        folge
    };

    // beste Reihenfolge und Richtung: (Sprünge, Weglänge) möglichst klein – eigene Festlegung der Bewertung
    let mut reihenfolge: Vec<usize> = Vec::new();
    if n <= 7 {
        let mut beste: (usize, f64) = (usize::MAX, f64::INFINITY);
        let mut stapel: Vec<(Vec<usize>, u64, usize, f64)> = (0..m)
            .map(|w| (vec![w], alle & !(1 << teil(w)), 0, anfahrt(w)))
            .collect();
        stapel.sort_by(|x, y| y.3.total_cmp(&x.3));
        let mut schritte = 0usize;
        while let Some((weg, rest, sprunge, laenge)) = stapel.pop() {
            schritte += 1;
            if schritte > 200_000 {
                break;
            }
            if sprunge > beste.0 || (sprunge == beste.0 && laenge >= beste.1) {
                continue;
            }
            if rest == 0 {
                beste = (sprunge, laenge);
                reihenfolge = weg;
                continue;
            }
            let v = *weg.last().unwrap();
            let mut neu: Vec<(Vec<usize>, u64, usize, f64)> = (0..m)
                .filter(|&w| rest & (1 << teil(w)) != 0)
                .map(|w| {
                    let (s, l) = match laufweg_fuer(v, w, rest) {
                        Some(lw) => (sprunge, laenge + weglaenge(&lw)),
                        None => (
                            sprunge + 1,
                            laenge + verb[v][w].as_ref().map_or(0.0, |vb| vb.gerade_laenge),
                        ),
                    };
                    let mut wg = weg.clone();
                    wg.push(w);
                    (wg, rest & !(1 << teil(w)), s, l)
                })
                .collect();
            // die günstigsten zuletzt auf den Stapel, damit sie zuerst geprüft werden
            neu.sort_by(|x, y| (y.2, y.3).partial_cmp(&(x.2, x.3)).unwrap_or(std::cmp::Ordering::Equal));
            stapel.extend(neu);
        }
    }
    if reihenfolge.len() != n {
        reihenfolge = gierig();
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
    for &v in &reihenfolge {
        let w = wurzel(&mut eltern, teil(v));
        let g = match gruppe_von[w] {
            Some(g) => g,
            None => {
                gruppen.push(Vec::new());
                gruppe_von[w] = Some(gruppen.len() - 1);
                gruppen.len() - 1
            }
        };
        gruppen[g].push(v);
    }

    // Laufweg von `von` nach `nach` (Ziel ist Knoten `ziel`), wenn verdeckt von noch ausstehenden Teilen `rest`:
    // gerade, oder über die Mittellinie des Ziels
    let laufweg_zu = |von: Punkt, nach: Punkt, ziel: usize, rest: u64| -> Option<Linie> {
        if masken(von, nach).iter().all(|&mk| mk & rest != 0) {
            return Some(vec![von, nach]);
        }
        let mt = r(ziel).mitte.as_ref().filter(|mt| mt.len() >= 2)?;
        let naechster = |q: Punkt| {
            (0..mt.len())
                .min_by(|&x, &y| abstand(q, mt[x]).total_cmp(&abstand(q, mt[y])))
                .unwrap()
        };
        let (k, t) = (naechster(von), naechster(nach));
        if !masken(von, mt[k]).iter().all(|&mk| mk & rest != 0) {
            return None;
        }
        let mut weg: Linie = vec![von];
        if k <= t {
            weg.extend(mt[k..=t].iter().copied());
        } else {
            weg.extend(mt[t..=k].iter().rev().copied());
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
        for &v in g.iter().rev() {
            if !r(v).unter.is_empty() {
                anhaengen(&mut out, r(v).unter.clone(), v, rest);
            }
        }
        // dann die Decklagen in ihrer Reihenfolge
        for &v in g {
            let mut wege = r(v).wege.clone().into_iter();
            if let Some(erster) = wege.next() {
                anhaengen(&mut out, erster, v, rest);
            }
            out.extend(wege);
            rest &= !(1 << teil(v));
        }
    }
    // weitere Teile über 63 (kommt bei Buchstaben nicht vor): ohne Laufweg anhängen
    for (richt, _) in stuecke.iter().skip(n) {
        if !richt[0].unter.is_empty() {
            out.push(richt[0].unter.clone());
        }
        out.extend(richt[0].wege.iter().cloned());
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
                mittig,
            } => {
                let (a, b) = satinlinie_seiten(punkte, *breite, *mittig);
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
            ObjektArt::Wendefuellung {
                umriss,
                winkellinien,
                unterlage,
            } => {
                let (a, b) = crate::sticharten::wendung::wende_seiten(umriss, winkellinien)?;
                let l = &winkellinien[0];
                let w = (l[1].y - l[0].y).atan2(l[1].x - l[0].x).to_degrees();
                let mut wege = Vec::new();
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    // Unterlage wie bei Tatami, Deckrichtung = erste Winkellinie (eigene Festlegung)
                    wege.extend(flaechen_unterlage(umriss, &[], w, &e.unterlage, start));
                }
                // Step-Reihen längs der Winkellinien: Abstand, Zugausgleich, Länge, Versatz der Step-Einstellung
                let mut se = e.satin.clone();
                se.auto_spacing = false;
                se.abstand_fest = e.tatami.abstand;
                se.zugausgleich = e.tatami.zugausgleich;
                se.kurzstiche.an = false;
                wege.push(crate::sticharten::satin::step_reihen(&satin(&a, &b, &se)?, &e.tatami));
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
    /// Double Run (Handbuch Kap. 23 „Reinforcing outlines“): diese Objekte (Nummer in `objekte`) werden nach dem
    /// Sticken mit umgekehrter Stichfolge noch einmal gestickt, gleiche Farbe, ohne Verbindungsstiche. JSON
    /// `"doppellauf": true` am Objekt.
    pub doppellauf: std::collections::BTreeSet<usize>,
    /// Auto Jump je Objekt (Nummer in `objekte`), JSON `"auto_jump"`; ohne Angabe: Standard des Stils
    pub auto_jump: BTreeMap<usize, AutoJump>,
    /// Step-Muster 1–20 je Objekt (Nummer in `objekte`), JSON `"step_muster"` – für Tatami-Flächen und Step in Schrift
    pub step_muster: BTreeMap<usize, usize>,
}

/// Auto Jump eines Objekts (Effects-Dialog, Handbuch Kap. 22 „Preserving long stitches with Auto Jump“): lange Stiche
/// werden zu Sprüngen (kein Einstich in der Mitte), sonst in kürzere Stiche geteilt. Standard an mit 12,7 / 7,0 mm
/// (AMT 0x2711/0x2712/0x2758), im Stil Schrift **aus** (`Analyse/code/10` Tabelle Stile). Kleiner Höchststich (z. B.
/// 6 mm) ergibt nach Handbuch eine Säule „with more loft“.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoJump {
    pub an: bool,
    /// Höchststich in mm (darüber Sprünge bzw. Teilung)
    pub max: f64,
    /// Länge der Sprünge in mm
    pub sprung: f64,
}

impl Default for AutoJump {
    fn default() -> Self {
        AutoJump {
            an: true,
            max: crate::einheiten::MAX_STICH_MM,
            sprung: crate::einheiten::AUTO_JUMP_TEILLAENGE_MM,
        }
    }
}

/// Auto Jump aus: Stiche über dem Höchststich der Maschine in gleich lange echte Stiche teilen (Handbuch: „it is
/// broken into smaller stitches“; gleich lange Stücke: eigene Festlegung).
fn lange_stiche_einstechen(w: &mut Linie, max: f64) {
    let mut out = Vec::with_capacity(w.len());
    for (i, &q) in w.iter().enumerate() {
        if i > 0 {
            let a = w[i - 1];
            let d = abstand(a, q);
            if d > max + 1e-9 {
                let n = (d / max - 1e-9).ceil() as usize;
                out.extend((1..n).map(|j| zwischen(a, q, j as f64 / n as f64)));
            }
        }
        out.push(q);
    }
    *w = out;
}

impl Design {
    pub fn aus_json(text: &str) -> Result<Design, String> {
        let v: Value = serde_json::from_str(text).map_err(|e| format!("JSON ungültig: {e}"))?;
        Design::aus_value(&v)
    }

    pub fn aus_value(v: &Value) -> Result<Design, String> {
        let v = &monogramme_aufloesen(v)?;
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
            if let ObjektJson::Tatami(t) = o
                && !t.winkellinien.is_empty()
                && (!t.loecher.is_empty() || !t.aussparen.is_empty())
            {
                return Err("Fläche mit Winkellinien: ohne Löcher und Aussparen (noch nicht möglich)".into());
            }
            if let ObjektJson::Satinlinie(s) = o
                && let Some(x) = s.versatz.as_deref().filter(|x| !matches!(*x, "mitte" | "seite"))
            {
                return Err(format!("Versatz „{x}“ unbekannt (mitte, seite)"));
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
                            step: false,
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
                        mittig: s.versatz.as_deref() != Some("seite"),
                    },
                },
                ObjektJson::Tatami(t) if !t.winkellinien.is_empty() => Objekt {
                    farbe: t.farbe,
                    name: t.name,
                    art: ObjektArt::Wendefuellung {
                        umriss: t.umriss.punkte(),
                        winkellinien: t.winkellinien,
                        unterlage: t.unterlage,
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
                        winkellinien: b.winkellinien,
                    });
                }
            }
            // Stichart der Schrift (Handbuch Kap. 20): Alphabete satin | step | muster, Computer-Schriften auch muster
            let ist_text = matches!(o.art, ObjektArt::Text { .. });
            let zahl = |k: &str| roh.and_then(|r| r.get(k)).and_then(Value::as_f64);
            let stichart = match roh.and_then(|r| r.get("stichart")).and_then(Value::as_str) {
                Some("step") if ist_text => Some(SchriftStichart::Step),
                Some("muster") => {
                    let m = roh
                        .and_then(|r| r.get("muster"))
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            format!("{wo}: Stichart muster braucht „muster“ (Name oder Nummer, stick muster)")
                        })?;
                    if crate::muster::finden(m).is_none() {
                        return Err(format!("{wo}: Füllmuster „{m}“ unbekannt (stick muster)"));
                    }
                    Some(SchriftStichart::Muster {
                        muster: m.to_string(),
                        winkel: zahl("winkel"),
                        groesse: zahl("groesse").unwrap_or(100.0),
                    })
                }
                None | Some("satin") if ist_text => None,
                Some(x) if ist_text => {
                    return Err(format!("{wo}: Stichart „{x}“ unbekannt (satin, step, muster)"));
                }
                _ => None,
            };
            let huelle = match roh.and_then(|r| r.get("elastisch")) {
                Some(h) => Some(huelle_lesen(h).map_err(|e| format!("{wo}: elastisch: {e}"))?),
                None => None,
            };
            if !farben.is_empty() || !einzeln.is_empty() || stichart.is_some() || huelle.is_some() {
                buchstaben.insert(
                    i,
                    TextZusatz {
                        farben,
                        einzeln,
                        stichart,
                        huelle,
                    },
                );
            }
        }
        let doppellauf = v
            .get("objekte")
            .and_then(Value::as_array)
            .map(|l| {
                l.iter()
                    .enumerate()
                    .filter(|(_, o)| o.get("doppellauf").and_then(Value::as_bool) == Some(true))
                    .map(|(i, _)| i)
                    .collect()
            })
            .unwrap_or_default();
        let mut auto_jump = BTreeMap::new();
        if let Some(l) = v.get("objekte").and_then(Value::as_array) {
            for (i, o) in l.iter().enumerate() {
                let wo = format!("Objekt {}", i + 1);
                let aj = match o.get("auto_jump") {
                    None => continue,
                    Some(Value::Bool(an)) => AutoJump {
                        an: *an,
                        ..AutoJump::default()
                    },
                    Some(w) => {
                        #[derive(Deserialize)]
                        #[serde(deny_unknown_fields)]
                        struct AutoJumpJson {
                            max: Option<f64>,
                            sprung: Option<f64>,
                        }
                        let j: AutoJumpJson = serde_json::from_value(w.clone())
                            .map_err(|e| format!("{wo}: auto_jump: true, false oder {{\"max\", \"sprung\"}}: {e}"))?;
                        let s = AutoJump::default();
                        AutoJump {
                            an: true,
                            max: j.max.unwrap_or(s.max),
                            sprung: j.sprung.unwrap_or(s.sprung),
                        }
                    }
                };
                if !(crate::einheiten::MIN_STICH_MM..=crate::einheiten::MAX_STICH_MM).contains(&aj.max)
                    || !(crate::einheiten::MIN_STICH_MM..=crate::einheiten::MAX_STICH_MM).contains(&aj.sprung)
                {
                    return Err(format!("{wo}: auto_jump: max und sprung zwischen 0,3 und 12,7 mm"));
                }
                auto_jump.insert(i, aj);
            }
        }
        let mut step_muster = BTreeMap::new();
        if let Some(l) = v.get("objekte").and_then(Value::as_array) {
            for (i, o) in l.iter().enumerate() {
                if let Some(w) = o.get("step_muster") {
                    let nr = w.as_u64().unwrap_or(0) as usize;
                    TatamiEinstellung::default()
                        .step_muster(nr)
                        .map_err(|e| format!("Objekt {}: {e}", i + 1))?;
                    step_muster.insert(i, nr);
                }
            }
        }
        Ok(Design {
            transformationen,
            transformation,
            buchstaben,
            doppellauf,
            auto_jump,
            step_muster,
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
        let (o, f, _) = self.aufgeloest_mit_herkunft()?;
        Ok((o, f))
    }

    /// Wie `objekte_aufgeloest`, dazu je Objekt die Nummer des Objekts im Design, aus dem es stammt.
    fn aufgeloest_mit_herkunft(&self) -> Result<Aufgeloest, String> {
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
        let (out, herkunft) = self.aussparen_aufloesen(out, &herkunft)?;
        Ok((out, fehlend, herkunft))
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
    fn aussparen_aufloesen(
        &self,
        objekte: Vec<Objekt>,
        herkunft: &[usize],
    ) -> Result<(Vec<Objekt>, Vec<usize>), String> {
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
        let mut her = Vec::with_capacity(objekte.len());
        for ((o, n), &h) in objekte.into_iter().zip(neu).zip(herkunft) {
            let Some(teile) = n else {
                out.push(o);
                her.push(h);
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
                her.push(h);
            }
        }
        Ok((out, her))
    }

    /// Stiche berechnen: Objekte -> Wege -> verbinden -> Bernina-Regeln (Mindeststich, Auto Jump).
    pub fn berechnen(&self) -> Result<StichFolge, String> {
        let (objekte, _, herkunft) = self.aufgeloest_mit_herkunft()?;
        type Teile = Vec<(String, Vec<Linie>, Option<(f64, f64)>)>;
        let teile_ab = |start: Punkt| -> Result<Teile, String> {
            let mut teile = Vec::new();
            let mut pos = start;
            for (obj, h) in objekte.iter().zip(&herkunft) {
                let eigene;
                let e = match self.step_muster.get(h) {
                    Some(&nr) => {
                        let mut x = self.einstellungen.clone();
                        x.tatami.step_muster(nr)?;
                        eigene = x;
                        &eigene
                    }
                    None => &self.einstellungen,
                };
                let mut wege: Vec<Linie> = obj.wege(e, Some(pos))?.into_iter().filter(|w| !w.is_empty()).collect();
                if self.doppellauf.contains(h) {
                    doppellauf(&mut wege);
                }
                // Auto Jump: Angabe am Objekt, sonst Stil (Schrift aus, alles andere an mit Standardwerten)
                let aj = self.auto_jump.get(h).copied().unwrap_or(AutoJump {
                    an: !matches!(
                        self.objekte.get(*h).map(|o| &o.art),
                        Some(ObjektArt::Text { .. } | ObjektArt::TtfText { .. })
                    ),
                    ..AutoJump::default()
                });
                let eigen = if aj.an {
                    (aj != AutoJump::default()).then_some((aj.max, aj.sprung))
                } else {
                    wege.iter_mut()
                        .for_each(|w| lange_stiche_einstechen(w, crate::einheiten::MAX_STICH_MM));
                    None
                };
                if let Some(letzter) = wege.last().and_then(|w| w.last()) {
                    pos = *letzter;
                    teile.push((obj.farbe.clone(), wege, eigen));
                }
            }
            Ok(teile)
        };
        let null = p(0.0, 0.0);
        let mut teile = teile_ab(null)?;
        // Mitte bzw. Unterkante des Designs aus den Stichen (umschließendes Rechteck)
        let punkte = || teile.iter().flat_map(|(_, w, _)| w.iter().flatten());
        let (x0, x1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let (y0, y1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
        let (mitte, unten) = if x0 <= x1 {
            (p((x0 + x1) / 2.0, (y0 + y1) / 2.0), p((x0 + x1) / 2.0, y0))
        } else {
            (null, null)
        };
        let erster = teile
            .first()
            .and_then(|(_, w, _)| w.first())
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
        let (teile, auto_jump): (Vec<_>, Vec<_>) = teile.into_iter().map(|(f, w, a)| ((f, w), a)).unzip();
        let folge = wege_verbinden_mit(&teile, &self.einstellungen.verbinder, start, &auto_jump);
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

    /// Double Run: Lauf wird rückwärts wiederholt und endet am Anfang; Satinlinie mit Versatz liegt links der Linie
    #[test]
    fn doppellauf_und_satin_versatz() {
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "lauf", "punkte": [[0, 0], [10, 0]], "laenge": 2.5, "doppellauf": true}]}"##,
        )
        .unwrap();
        let st = d.berechnen().unwrap();
        let xs: Vec<f64> = st.stiche.iter().map(|s| s.p().x).collect();
        // hin 0 … 10, zurück … 0 (danach Sprung zum Ende in der Rahmenmitte = 0)
        assert!(xs.iter().any(|&x| (x - 10.0).abs() < 1e-6), "{xs:?}");
        let n10 = xs.iter().position(|&x| (x - 10.0).abs() < 1e-6).unwrap();
        let hin = xs[..n10].iter().filter(|&&x| x > 0.1 && x < 9.9).count();
        let zurueck = xs[n10 + 1..].iter().filter(|&&x| x > 0.1 && x < 9.9).count();
        assert!(hin >= 3 && zurueck >= 3, "{xs:?}");
        assert!(xs.last().unwrap().abs() < 0.5, "{xs:?}");
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "satinlinie", "punkte": [[0, 0], [20, 0]], "breite": 2, "versatz": "seite",
                "unterlage": "keine"}]}"##,
        )
        .unwrap();
        let ys: Vec<f64> = d.berechnen().unwrap().stiche.iter().map(|s| s.p().y).collect();
        // Säule zwischen y = 0 und 2 (links der Laufrichtung nach +x), Zugausgleich ragt etwas darüber hinaus
        assert!(ys.iter().all(|&y| y > -0.5 && y < 2.5), "{ys:?}");
        assert!(ys.iter().any(|&y| y > 1.9));
        assert!(
            Design::aus_json(r#"{"objekte": [{"art": "satinlinie", "punkte": [[0, 0], [1, 0]], "versatz": "x"}]}"#)
                .is_err()
        );
    }

    /// Auto Jump je Objekt: an (Sprünge), aus (echte Stiche ≤ 12,7 mm), eigene Werte (Sprünge ab 6 mm)
    #[test]
    fn auto_jump_je_objekt() {
        let zaehle = |aj: &str| {
            let d = Design::aus_json(&format!(
                r#"{{"objekte": [{{"art": "satin", "seite_a": [[0, 0], [0, 4]], "seite_b": [[20, 0], [20, 4]],
                    "unterlage": "keine", "abstand": 0.5 {aj}}}]}}"#
            ))
            .unwrap();
            let st = d.berechnen().unwrap();
            let lang = st.stichlaengen().iter().filter(|&&l| l > 12.7 + 1e-6).count();
            (
                st.anzahl(crate::stiche::Art::Sprung),
                lang,
                st.stichlaengen().into_iter().fold(0.0, f64::max),
            )
        };
        // Säule 4 mm lang, Stiche 20 mm quer
        let (sp_an, lang_an, _) = zaehle("");
        let (sp_aus, lang_aus, max_aus) = zaehle(r#", "auto_jump": false"#);
        let (sp_6, _, max_6) = zaehle(r#", "auto_jump": {"max": 6, "sprung": 3}"#);
        assert_eq!((lang_an, lang_aus), (0, 0));
        assert!(sp_an > sp_aus, "an {sp_an}, aus {sp_aus}");
        assert!(max_aus <= 12.7 + 1e-6 && max_aus > 7.0, "{max_aus}");
        assert!(sp_6 > sp_an && max_6 <= 3.0 + 1e-6, "{sp_6} {max_6}");
        assert!(
            Design::aus_json(r#"{"objekte": [{"art": "lauf", "punkte": [[0, 0], [1, 0]], "auto_jump": {"max": 20}}]}"#)
                .is_err()
        );
    }

    /// Fläche mit Winkellinien: Stiche folgen den Linien (längs, wie Satin), Länge ≤ Step-Länge + Rand
    #[test]
    fn wendefuellung_im_design() {
        let d = Design::aus_json(
            r#"{"objekte": [{"art": "tatami", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 30, "hoehe": 10}},
                "unterlage": false, "winkellinien": [[[-10, -6], [-10, 6]], [[10, -6], [0, 6]]]}]}"#,
        )
        .unwrap();
        assert!(matches!(d.objekte[0].art, ObjektArt::Wendefuellung { .. }));
        let st = d.berechnen().unwrap();
        assert!(st.stiche.len() > 300);
        assert!(st.stichlaengen().iter().all(|&l| l <= 4.0 + 0.4 + 1e-6));
        // bei x ≈ −10 senkrecht (wie die erste Linie)
        let pts: Vec<Punkt> = st.stiche.iter().map(|s| s.p()).collect();
        let senkrecht = pts
            .windows(2)
            .filter(|w| (w[0].x + 10.0).abs() < 0.5 && (w[1].x + 10.0).abs() < 0.5 && abstand(w[0], w[1]) > 2.0)
            .count();
        assert!(senkrecht > 3, "{senkrecht}");
        assert!(
            Design::aus_json(
                r#"{"objekte": [{"art": "tatami", "umriss": [[0, 0], [10, 0], [10, 10], [0, 10]],
                "loecher": [[[4, 4], [6, 4], [6, 6]]], "winkellinien": [[[5, -1], [5, 11]]]}]}"#
            )
            .is_err()
        );
    }

    /// Monogramm → Text mit Diamond2/Diamond3 (Zeichenbelegung aus den Alphabeten)
    #[test]
    fn monogramm_als_text() {
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "monogramm", "initialen": "shp", "rahmen": "raute", "rahmen_farbe": "#aa0000",
                "farbe": "#000000"}, {"art": "monogramm", "initialen": "AB", "rahmen": "zier"}]}"##,
        )
        .unwrap();
        let ObjektArt::Text { alphabet, vorgabe } = &d.objekte[0].art else {
            panic!()
        };
        // S links, H in der Mitte ('!' + 7 = '('), p rechts, Farbwechsel, Raute
        assert_eq!((alphabet.as_str(), vorgabe.text.as_str()), ("Diamond3", "S(p^]"));
        assert_eq!(vorgabe.hoehe, MONOGRAMM_HOEHE_MM);
        assert_eq!(d.buchstaben[&0].farben, vec!["#aa0000".to_string()]);
        let ObjektArt::Text { alphabet, vorgabe } = &d.objekte[1].art else {
            panic!()
        };
        assert_eq!((alphabet.as_str(), vorgabe.text.as_str()), ("Diamond2", "Ab["));
        assert!(Design::aus_json(r#"{"objekte": [{"art": "monogramm", "initialen": "ÄB"}]}"#).is_err());
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
