//! **Das Design-Format (JSON)** – einzige Quelle für Kern, Schema und Oberfläche (ROADMAP 4b.0).
//!
//! Jedes Feld, das der Kern aus einem Design liest, steht hier. Daraus entstehen automatisch das JSON-Schema
//! (`schema()`), die TypeScript-Typen und die Feldliste `SCHNITTSTELLE.md` (`stick api exportieren`; der Wächter-Test
//! in `stick-api` schlägt fehl, wenn die erzeugten Dateien veraltet sind). **Die Doc-Kommentare der Felder sind die
//! Beschreibung im Schema** – deutsch, für Designer und Frontend; Standardwerte stehen in `#[serde(default = …)]`.
//!
//! Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. Neue Funktion im Kern = neues Feld hier (mit Doc-Kommentar
//! und Standard), Umsetzung in `design.rs`.

use crate::geometrie::{Linie, Punkt, bezier, bogen, kreis, p, rechteck};
use crate::transformation::Transformation;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::Deserialize;
use serde_json::Value;

/// Fassung des Design-Formats. Erhöhen, wenn sich die **Bedeutung** eines vorhandenen Felds ändert, und in
/// `migrieren` alte Fassungen umschreiben. Neue Felder mit Standardwert brauchen keine neue Fassung.
pub const VERSION: u32 = 1;

/// Alte Fassungen auf die aktuelle bringen (bisher gibt es nur Fassung 1). Neuere Fassungen als die des Programms
/// sind ein Fehler – sonst würden Felder still falsch gelesen.
pub fn migrieren(v: &Value) -> Result<Value, String> {
    let fassung = match v.get("version") {
        None | Some(Value::Null) => VERSION,
        Some(x) => x
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .ok_or("version muss eine ganze Zahl sein")?,
    };
    if fassung > VERSION {
        return Err(format!(
            "Design hat Format-Fassung {fassung}, dieses Programm kennt nur bis {VERSION} – bitte das Programm aktualisieren"
        ));
    }
    Ok(v.clone())
}

/// Meldungen von serde ins Deutsche (für die Oberfläche): unbekannte Werte/Felder, fehlende Felder, falsche Typen.
pub fn fehler_deutsch(m: &str) -> String {
    // Ortsangabe „ at line 1 column 7“ entfällt (bei Werten aus der Oberfläche ohne Bedeutung)
    let m = match m.find(" at line ") {
        Some(i) => &m[..i],
        None => m,
    };
    let liste = |rest: &str| -> String {
        rest.replace("expected one of ", "")
            .replace("expected ", "")
            .replace(" or ", ", ")
            .replace('`', "")
    };
    let wert = |s: &str| -> String { s.trim_matches('`').to_string() };
    if let Some(r) = m.strip_prefix("unknown variant ")
        && let Some((w, erlaubt)) = r.split_once(", ")
    {
        return format!("Wert „{}“ unbekannt (erlaubt: {})", wert(w), liste(erlaubt));
    }
    if let Some(r) = m.strip_prefix("unknown field ") {
        if let Some((w, erlaubt)) = r.split_once(", ") {
            return format!("Feld „{}“ unbekannt (erlaubt: {})", wert(w), liste(erlaubt));
        }
        return format!("Feld „{}“ unbekannt", wert(r));
    }
    if let Some(r) = m.strip_prefix("missing field ") {
        return format!("Feld „{}“ fehlt", wert(r));
    }
    if let Some(r) = m.strip_prefix("invalid type: ")
        && let Some((ist, soll)) = r.split_once(", expected ")
    {
        return format!("falscher Typ: {ist} statt {soll}");
    }
    if let Some(r) = m.strip_prefix("data did not match any variant of untagged enum ") {
        return format!("Wert passt zu keiner erlaubten Form ({r})");
    }
    if let Some(r) = m.strip_prefix("invalid length ") {
        return format!("falsche Länge {r}");
    }
    m.to_string()
}

// ---------------------------------------------------------------- Auswahlfelder

auswahl! {
    /// Stichart einer Satinsäule, Satinlinie oder Satinfläche
    pub enum SatinStichart {
        /// Satin (Standard)
        Satin = "satin",
        /// Zickzack (offen, fester Abstand; Stichart „Zigzag“ 0x2724)
        Zickzack = "zickzack",
    }
}

auswahl! {
    /// Lage einer Linie zur gezeichneten Linie
    pub enum Versatz {
        /// mittig auf der Linie („Center“, Standard)
        Mitte = "mitte",
        /// zur Seite: Linie = Kante, Stiche links davon („Offset“)
        Seite = "seite",
    }
}

auswahl! {
    /// Abdeckstich der Applikation
    pub enum Abdeckung {
        /// Satin mit Heftstich (Standard)
        Satin = "satin",
        /// Blanket (Langette) ohne Heftstich
        Blanket = "blanket",
    }
}

auswahl! {
    /// Arbeitsweise der Applikation
    pub enum ApplikationStil {
        /// Stoff vorgeschnitten („Pre-cut“, Standard)
        Vorgeschnitten = "vorgeschnitten",
        /// im Rahmen zuschneiden („Trim in Place“, mit Schnittlinie)
        Zuschneiden = "zuschneiden",
    }
}

auswahl! {
    /// Knopfloch-Typ (Dialog „Buttonhole Type“; Vorlagen aus `ButtonHoles.ESB`)
    pub enum KnopflochTyp {
        /// Standard (zwei Raupen, Riegel oben und unten)
        Standard = "standard",
        /// Standard Narrow (schmale Raupen)
        StandardSchmal = "standard_schmal",
        /// Round (beide Enden rund)
        Rund = "rund",
        /// Stretch (dehnbar, Zickzack-Läufe statt Satin)
        Dehnbar = "dehnbar",
        /// Heirloom (Raupen mit Kreuzstichen)
        Heirloom = "heirloom",
        /// Round End (ein Ende rund)
        RundesEnde = "rundes_ende",
        /// Keyhole Square (Augenknopfloch, eckiges Auge)
        AugeEckig = "auge_eckig",
        /// Keyhole Round (Augenknopfloch, rundes Auge)
        AugeRund = "auge_rund",
        /// Keyhole Point (Augenknopfloch, spitzes Auge)
        AugeSpitz = "auge_spitz",
    }
}

auswahl! {
    /// Ausrichtung eines Texts
    pub enum Ausrichtung {
        Links = "links",
        /// Standard
        Mitte = "mitte",
        Rechts = "rechts",
        /// Blocksatz („Full“): nur mit Grundlinie fester Länge (kurve, gerade mit laenge, kreis mit bogen)
        Block = "block",
    }
}

auswahl! {
    /// Stichart einer Schrift mit Original-Alphabet
    pub enum SchriftStichart {
        /// Satin (Standard, wie das Alphabet)
        Satin = "satin",
        /// Step: Reihen folgen den Stichwinkeln der Säulen
        Step = "step",
        /// Füllmuster über die Buchstabenfläche (Feld `muster`)
        Muster = "muster",
    }
}

auswahl! {
    /// Stichart einer Computer-Schrift
    pub enum TtfStichart {
        /// Tatami-Fläche mit Löchern (Standard, eigene Festlegung)
        Tatami = "tatami",
        /// Umriss als Laufstich
        Lauf = "lauf",
        /// Umriss als Dreifachstich
        Dreifach = "dreifach",
        /// Satinsäulen wie die TrueType-Umwandlung des Originals
        Satin = "satin",
        /// Füllmuster (Feld `muster`)
        Muster = "muster",
    }
}

auswahl! {
    /// Art der Grundlinie (Dialog „Layout“)
    pub enum GrundlinieArt {
        /// Kreis CW/CCW, auch Oval (radius_y) und Bogen (bogen)
        Kreis = "kreis",
        /// gerade mit Winkel und optional fester Länge
        Gerade = "gerade",
        /// senkrecht
        Senkrecht = "senkrecht",
        /// eigene Kurve (Feld `punkte`)
        Kurve = "kurve",
        /// mehrfach: oben – Mitte – unten (höchstens drei Zeilen)
        Mehrfach = "mehrfach",
    }
}

auswahl! {
    /// Lage auf der Kreis-Grundlinie
    pub enum GrundlinieLage {
        /// oben, im Uhrzeiger (Circle CW)
        Oben = "oben",
        /// unten, gegen den Uhrzeiger (Circle CCW)
        Unten = "unten",
    }
}

auswahl! {
    /// Stand der Buchstaben zur Grundlinie (Position Above/Through/Below)
    pub enum GrundlinieStand {
        Ueber = "ueber",
        Durch = "durch",
        Unter = "unter",
    }
}

auswahl! {
    /// Form der Verzerrung (Elastic Lettering)
    pub enum HuelleArt {
        /// gerade Kanten (Wimpel, schräg): oben/unten = [links, rechts]
        Gerade = "gerade",
        /// gebogene Kanten (Bogen, Bauch): oben/unten = Mitte oder [links, Mitte, rechts]
        Gebogen = "gebogen",
    }
}

auswahl! {
    /// Rahmen um ein Monogramm
    pub enum MonogrammRahmen {
        /// Raute
        Raute = "raute",
        /// Zierwinkel
        Zier = "zier",
        /// kein Rahmen (Standard)
        Keine = "keine",
    }
}

auswahl! {
    /// Startpunkt des Designs (Dialog „Start and End“)
    pub enum Start {
        /// Rahmenmitte = Ursprung (Standard)
        #[serde(alias = "mitte")]
        Rahmenmitte = "rahmenmitte",
        /// Mitte des Designs (umschließendes Rechteck) – wie „Auto Start“ des Originals
        Designmitte = "designmitte",
        /// Mitte der Unterkante
        UntenMitte = "unten_mitte",
        /// erster Stich (kein Verbinder)
        ErsterStich = "erster_stich",
    }
}

auswahl! {
    /// Endpunkt des Designs (Dialog „Start and End“)
    pub enum Ende {
        /// Sprung zurück zur Rahmenmitte (Standard; 34 von 51 Maschinendesigns enden auf 0,0)
        #[serde(alias = "mitte")]
        Rahmenmitte = "rahmenmitte",
        /// Mitte des Designs
        Designmitte = "designmitte",
        /// Mitte der Unterkante
        UntenMitte = "unten_mitte",
        /// letzter Stich (kein Sprung)
        LetzterStich = "letzter_stich",
        /// zurück zum Startpunkt
        Start = "start",
    }
}

auswahl! {
    /// Art einer Unterlage (in Stickfolge)
    pub enum UnterlageArt {
        /// Mittellauf (0x2746)
        Mittellauf = "mittellauf",
        /// Edge Walk entlang der Kanten (0x2747)
        EdgeWalk = "edge_walk",
        /// Zickzack
        Zickzack = "zickzack",
        /// doppelter Zickzack
        Doppelzickzack = "doppelzickzack",
    }
}

auswahl! {
    /// Unterlage automatisch oder keine
    pub enum UnterlageWahl {
        /// wie „Auto Underlay“ des Stils (Standard)
        Auto = "auto",
        /// keine Unterlage (z. B. kleine Schrift)
        Keine = "keine",
    }
}

// ---------------------------------------------------------------- Formen

/// Punkte einer Linie oder Fläche: Punktliste `[[x, y], …]` in mm oder eine Form.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum PunktAngabe {
    /// Punktliste [[x, y], …] in mm (Eckpunkte)
    Liste(Vec<Punkt>),
    /// Form (Kreis, Rechteck, Oval, Bogen, Bézier, Verbinden)
    Form(Form),
}

/// Form statt Punktliste, z. B. `{"kreis": {"mitte": [0, 0], "radius": 10}}`.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Form {
    /// Kreis
    Kreis {
        /// Mittelpunkt [x, y] in mm
        mitte: Punkt,
        /// Radius in mm
        radius: f64,
        /// Startwinkel in Grad
        #[serde(default)]
        start: f64,
    },
    /// Rechteck (Eckpunkte)
    Rechteck { mitte: Punkt, breite: f64, hoehe: f64 },
    /// Ellipse („Circle/Oval“ des Originals) mit Breite und Höhe in mm
    Oval {
        mitte: Punkt,
        breite: f64,
        hoehe: f64,
        #[serde(default)]
        start: f64,
    },
    /// Kreisbogen von … bis (Grad)
    Bogen {
        mitte: Punkt,
        radius: f64,
        von: f64,
        bis: f64,
    },
    /// kubische Bézierkurve: Start, Kontrollpunkt 1, Kontrollpunkt 2, Ende
    Bezier([Punkt; 4]),
    /// mehrere Stücke hintereinander (Punktlisten oder Formen)
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

// ---------------------------------------------------------------- Standardwerte

fn schwarz() -> String {
    "#000000".into()
}
fn auto() -> UnterlageJson {
    UnterlageJson::Text("auto".into())
}
fn hundert() -> f64 {
    100.0
}
fn text_hoehe() -> f64 {
    crate::schrift::STANDARD_HOEHE_MM
}
fn monogramm_hoehe() -> f64 {
    MONOGRAMM_HOEHE_MM
}
fn ausrichtung_mitte() -> Ausrichtung {
    Ausrichtung::Mitte
}
fn ttf_tatami() -> TtfStichart {
    TtfStichart::Tatami
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
fn standard_garnkarte() -> String {
    "Default".into()
}

/// Standardhöhe eines Monogramms: kleinste empfohlene Höhe der Alphabete Diamond2/Diamond3 (18 mm,
/// `Analyse/daten/alphabets.md`) – die Lettering-Vorlage (10 mm) liegt darunter.
pub const MONOGRAMM_HOEHE_MM: f64 = 18.0;

// ---------------------------------------------------------------- Bausteine

/// Unterlage: `"auto"` (Standard, wie „Auto Underlay“ des Stils), `"keine"` oder eine Liste der Arten in Stickfolge.
/// Alte Schreibweise: Arten mit „+“ verbunden (`"edge_walk+zickzack"`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum UnterlageJson {
    /// "auto" oder "keine"
    Text(#[schemars(with = "UnterlageWahl")] String),
    /// Arten in Stickfolge, z. B. ["edge_walk", "zickzack"]
    Liste(#[schemars(with = "Vec<UnterlageArt>")] Vec<String>),
}

/// Auto Jump eines Objekts: `true`/`false` oder Werte in mm.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum AutoJumpJson {
    /// an (Standardwerte 12,7 / 7,0 mm) oder aus (lange Stiche werden geteilt)
    An(bool),
    /// an mit eigenen Werten (kleiner Höchststich = Satin „mit mehr Loft“)
    Werte(AutoJumpWerte),
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AutoJumpWerte {
    /// Höchststich in mm, 0,3 … 12,7 (Standard 12,7); länger = Sprung
    pub max: Option<f64>,
    /// Länge der Sprünge in mm, 0,3 … 12,7 (Standard 7,0)
    pub sprung: Option<f64>,
}

/// Felder, die **jedes** Objekt haben kann (Funktionsübersicht §5 „Bearbeiten“).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Gemeinsam {
    /// Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
    #[serde(default = "schwarz")]
    pub farbe: String,
    /// Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
    #[serde(default)]
    pub name: String,
    /// Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet
    #[serde(default)]
    pub transform: Option<Transformation>,
    /// Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
    #[serde(default)]
    pub doppellauf: bool,
    /// Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm)
    #[serde(default)]
    pub auto_jump: Option<AutoJumpJson>,
    /// Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil)
    #[serde(default)]
    pub step_muster: Option<usize>,
    /// Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort
    #[serde(default)]
    pub einstich: Option<Punkt>,
    /// Ausstich („exit point“) [x, y] in mm: das Objekt endet dort
    #[serde(default)]
    pub ausstich: Option<Punkt>,
    /// **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
    /// `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
    /// Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
    #[serde(default)]
    pub einstellungen: Option<EinstellungenJson>,
}

/// Grundlinie eines Texts (`schrift::Grundlinie`; Dialog „Layout“ des Originals). Beim Kreis ist `position` des Texts
/// die Kreismitte.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GrundlinieJson {
    pub art: GrundlinieArt,
    /// Radius X in mm (Vorlage 50)
    pub radius: Option<f64>,
    /// Radius Y in mm (Oval)
    pub radius_y: Option<f64>,
    /// Kreis: Justif. Angle (Grad); gerade: Line Angle
    pub winkel: Option<f64>,
    /// Tilt des Ovals in Grad
    #[serde(default)]
    pub neigung: f64,
    /// Arc Angle in Grad (nur ein Bogen; Text wird eingepasst)
    pub bogen: Option<f64>,
    /// Length der geraden Grundlinie in mm (Text wird eingepasst)
    pub laenge: Option<f64>,
    /// oben (Circle CW) | unten (Circle CCW)
    pub lage: Option<GrundlinieLage>,
    /// ueber | durch | unter (Standard: über der CW-, unter der CCW-Grundlinie)
    pub stand: Option<GrundlinieStand>,
    /// Punkte oder Form der eigenen Kurve (art = kurve)
    pub punkte: Option<PunktAngabe>,
}

/// Griffe einer Kante beim Verzerren: eine Zahl (Mitte) oder Liste [links, rechts] bzw. [links, Mitte, rechts] in mm.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Griffe {
    Mitte(f64),
    Liste(Vec<f64>),
}

/// Elastic Lettering: `{"art": "gerade", "oben": [links, rechts], "unten": [links, rechts]}` oder
/// `{"art": "gebogen", "oben": mitte | [links, mitte, rechts], "unten": …}` (mm, + nach oben).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HuelleJson {
    pub art: HuelleArt,
    /// Verschiebung der oberen Kante in mm
    #[serde(default)]
    pub oben: Option<Griffe>,
    /// Verschiebung der unteren Kante in mm
    #[serde(default)]
    pub unten: Option<Griffe>,
}

/// Einzelner Buchstabe eines Texts (`"buchstaben"`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BuchstabeJson {
    /// Nummer des Buchstabens (1 = erster, ohne Leerzeichen und Zeilenumbrüche)
    pub nr: usize,
    /// eigene Farbe
    pub farbe: Option<String>,
    /// verschieben, drehen, skalieren … um die Mitte des Buchstabens
    pub transform: Option<Transformation>,
    /// neue Stichwinkel: Strecken [[x1, y1], [x2, y2]] quer über die Säulen
    #[serde(default)]
    pub winkellinien: Vec<[Punkt; 2]>,
}

/// Einstellungen, die Text, Computer-Schrift und Monogramm gemeinsam haben.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct SchriftOptionen {
    /// Breite in % der Höhe
    #[serde(default = "hundert")]
    pub breite: f64,
    /// Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe)
    pub abstand: Option<f64>,
    /// Wortabstand in % der Höhe (Vorlage 25)
    pub wortabstand: Option<f64>,
    /// Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text
    pub zeilenabstand: Option<f64>,
    /// Grundlinie (ohne Angabe: gerade waagrecht)
    pub grundlinie: Option<GrundlinieJson>,
    /// Kursiv in Grad (−45 … 45)
    #[serde(default)]
    pub kursiv: f64,
    /// Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte)
    #[serde(default)]
    pub position: Option<Punkt>,
    #[serde(default = "ausrichtung_mitte")]
    pub ausrichtung: Ausrichtung,
    /// Farben für die Farbwechsel `^` im Text, in Reihenfolge
    #[serde(default)]
    pub farben: Vec<String>,
    /// einzelne Buchstaben: Farbe, Transformation, Stichwinkel
    #[serde(default)]
    pub buchstaben: Vec<BuchstabeJson>,
    /// Verzerren (Elastic Lettering)
    pub elastisch: Option<HuelleJson>,
    /// Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“)
    pub muster: Option<String>,
    /// Größe des Füllmusters in % (Stichart „muster“, Standard 100)
    pub groesse: Option<f64>,
}

// ---------------------------------------------------------------- Objekte

/// Laufstich bzw. Dreifachstich entlang einer Linie.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct LaufJson {
    pub punkte: PunktAngabe,
    /// Stichlänge in mm (ohne Angabe: Stil, 2,5 mm)
    pub laenge: Option<f64>,
    /// Dreifachstich (bei art „dreifach“ immer)
    pub dreifach: Option<bool>,
    /// Linie schließen (zurück zum ersten Punkt)
    #[serde(default)]
    pub geschlossen: bool,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct SatinJson {
    pub seite_a: PunktAngabe,
    pub seite_b: PunktAngabe,
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    /// fester Satinabstand in mm statt Auto Spacing (wie im Alphabet: 0x15)
    pub abstand: Option<f64>,
    /// Auto-Spacing-Faktor des Objekts (Stil: 0x2736; Schrift 0,9)
    pub faktor: Option<f64>,
    /// Kurzstiche an Innenkurven (Effekt 0x273a; Software Defaults: aus)
    pub kurzstiche: Option<bool>,
    pub stichart: Option<SatinStichart>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Satinbahn konstanter Breite entlang einer Linie.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct SatinLinieJson {
    pub punkte: PunktAngabe,
    /// Breite in mm (Standard 2)
    pub breite: Option<f64>,
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    pub versatz: Option<Versatz>,
    pub stichart: Option<SatinStichart>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Blanket (Langettenstich, 0x2727) entlang einer Linie.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct BlanketJson {
    pub punkte: PunktAngabe,
    #[serde(default)]
    pub geschlossen: bool,
    /// Abstand der Zähne in mm (Vorlage 2,0)
    pub abstand: Option<f64>,
    /// Breite in mm (Vorlage 3,0)
    pub breite: Option<f64>,
    pub versatz: Option<Versatz>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Tatami-Fläche (Step) mit Löchern; mit `winkellinien` Fläche mit mehreren Stichwinkeln (Turning).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct TatamiJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Stichwinkel in Grad (ohne Angabe: Stil, 45°)
    pub winkel: Option<f64>,
    /// Unterlage an/aus (ohne Angabe: Auto Underlay des Stils)
    pub unterlage: Option<bool>,
    /// Namen der Objekte, deren Fläche ausgespart wird (Knockout)
    #[serde(default)]
    pub aussparen: Vec<String>,
    /// Überlappung der Aussparung in mm (Standard 0 = exakt wie das Original)
    #[serde(default)]
    pub ueberlappung: f64,
    /// mehrere Stichwinkel: Strecken [[x1, y1], [x2, y2]], die den Umriss je zweimal schneiden (ohne Löcher)
    #[serde(default)]
    pub winkellinien: Vec<[Punkt; 2]>,
    /// Kurzstiche an Innenkurven (nur mit Winkellinien wirksam; ohne Angabe: Stil)
    pub kurzstiche: Option<bool>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Fläche als Satinsäulen mit automatischer Stichrichtung (`saeulen.rs`, ROADMAP 4b.17).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct SatinFlaecheJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    /// fester Satinabstand in mm statt Auto Spacing
    pub abstand: Option<f64>,
    /// Auto-Spacing-Faktor
    pub faktor: Option<f64>,
    /// Kurzstiche an Innenkurven
    pub kurzstiche: Option<bool>,
    pub stichart: Option<SatinStichart>,
    /// Säulen an Knicken teilen: Innenwinkel in Grad, 0 … 180 (0 = nie wie der Auto Digitizer; 87 = TrueType)
    pub knickwinkel: Option<f64>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Text mit einem Original-Alphabet (50 Alphabete, Befehl `alphabete_liste`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct TextJson {
    /// Text; `\n` = neue Zeile, `^` = Farbwechsel (Farben in `farben`)
    pub text: String,
    /// Name des Alphabets („Drifter“)
    pub alphabet: String,
    /// Buchstabenhöhe in mm (Original 1–200 mm; Vorlage 10)
    #[serde(default = "text_hoehe")]
    pub hoehe: f64,
    /// Unterlage (auto = wie Stil Schrift: Zickzack)
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    pub stichart: Option<SchriftStichart>,
    /// Stichwinkel der Musterfüllung (Stichart „muster“)
    pub winkel: Option<f64>,
    #[serde(flatten)]
    pub schrift: SchriftOptionen,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Text mit einer Computer-Schrift (TrueType/OpenType, Befehl `schriften_liste`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct TtfJson {
    /// Text; `\n` = neue Zeile, `^` = Farbwechsel
    pub text: String,
    /// Name der Schrift („Arial“, „Arial Bold“) oder Pfad einer .ttf/.otf-Datei
    pub schrift: String,
    /// Höhe der Großbuchstaben in mm
    #[serde(default = "text_hoehe")]
    pub hoehe: f64,
    #[serde(default = "ttf_tatami")]
    pub stichart: TtfStichart,
    /// Stichwinkel (Tatami, Muster)
    pub winkel: Option<f64>,
    /// Unterlage an/aus (Tatami, Satin; ohne Angabe: Stil)
    pub unterlage: Option<bool>,
    /// Stichlänge bei Lauf/Dreifach in mm
    pub laenge: Option<f64>,
    #[serde(flatten)]
    pub schrift_optionen: SchriftOptionen,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Monogramm (Handbuch Kap. 21): 2 oder 3 Initialen mit den Alphabeten Diamond2/Diamond3, Rahmen wahlweise. Wird im
/// Kern zu einem Text – alle Schrift-Einstellungen gelten.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MonogrammJson {
    /// 2 oder 3 Buchstaben A–Z
    pub initialen: String,
    #[serde(default)]
    pub rahmen: Option<MonogrammRahmen>,
    /// Farbe des Rahmens (nur mit Rahmen)
    pub rahmen_farbe: Option<String>,
    /// Höhe in mm (18 = kleinste empfohlene Höhe von Diamond2/3)
    #[serde(default = "monogramm_hoehe")]
    pub hoehe: f64,
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    pub stichart: Option<SchriftStichart>,
    pub winkel: Option<f64>,
    #[serde(flatten)]
    pub schrift: SchriftOptionen,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct ApplikationJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Breite des Abdeck-Satins in mm (Standard 2,5)
    pub breite: Option<f64>,
    /// fester Satinabstand des Abdeckstichs in mm
    pub abstand: Option<f64>,
    /// Breite des Heftstichs in mm
    pub heft_breite: Option<f64>,
    /// Abstand des Heftstichs in mm
    pub heft_abstand: Option<f64>,
    /// Stichlänge der Platzierungslinie in mm
    pub platz_laenge: Option<f64>,
    pub abdeckung: Option<Abdeckung>,
    pub stil: Option<ApplikationStil>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Fläche mit Füllmuster (Program Split / Fancy Fill; 242 Muster, Befehl `fuellmuster_liste`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MusterJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Name des Musters („001 Diamond (90)“) oder seine Nummer („001“)
    pub muster: String,
    /// Stichwinkel (ohne Angabe: Winkel aus dem Musternamen, sonst der Füllung)
    pub winkel: Option<f64>,
    /// Größe des Musters in %
    #[serde(default = "hundert")]
    pub groesse: f64,
    /// Unterlage an/aus (ohne Angabe: Stil)
    pub unterlage: Option<bool>,
    /// Namen der Objekte, deren Fläche ausgespart wird (Knockout)
    #[serde(default)]
    pub aussparen: Vec<String>,
    /// Überlappung der Aussparung in mm
    #[serde(default)]
    pub ueberlappung: f64,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Knopfloch (Handbuch Kap. 23 „Adding buttonholes“): Vorlage aus `ButtonHoles.ESB`, auf Schlitzlänge gestreckt.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct KnopflochJson {
    /// Typ (Standard „standard“ wie das Original)
    #[serde(default = "knopfloch_typ")]
    pub typ: KnopflochTyp,
    /// Mitte des Schlitzes [x, y] in mm (ohne Angabe: Rahmenmitte)
    #[serde(default)]
    pub position: Option<Punkt>,
    /// Schlitzlänge in mm, 3 … 60 („Slit Length“, Standard 20 wie das Original)
    #[serde(default = "knopfloch_laenge")]
    pub laenge: f64,
    /// Raupenabstand in mm, 0,2 … 2,0 („Bead Stitch Spacing“); gilt nur für die Raupen, nicht die Enden. Ohne Angabe
    /// wie die Vorlage (0,5 mm; Heirloom 1,5 mm)
    #[serde(default)]
    pub abstand: Option<f64>,
    /// Richtung des Schlitzes in Grad („Angle“; 90 = senkrecht, Standard wie das Original)
    #[serde(default = "knopfloch_winkel")]
    pub winkel: f64,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

fn knopfloch_typ() -> KnopflochTyp {
    KnopflochTyp::Standard
}
fn knopfloch_laenge() -> f64 {
    crate::knopfloch::STANDARD_LAENGE
}
fn knopfloch_winkel() -> f64 {
    crate::knopfloch::STANDARD_WINKEL
}

/// Ein Stickobjekt. `art` wählt die Objektart; Reihenfolge in `objekte` = Stickreihenfolge.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "art", rename_all = "lowercase")]
pub enum ObjektJson {
    /// Laufstich
    Lauf(LaufJson),
    /// Dreifachstich
    Dreifach(LaufJson),
    /// Satinsäule
    Satin(SatinJson),
    /// Satinlinie (Kontur)
    Satinlinie(SatinLinieJson),
    /// Blanket (Langette)
    Blanket(BlanketJson),
    /// Fläche Tatami (Step)
    Tatami(TatamiJson),
    /// Satinfläche mit automatischer Stichrichtung
    Satinflaeche(SatinFlaecheJson),
    /// Text mit Original-Alphabet
    Text(TextJson),
    /// Text mit Computer-Schrift
    Ttf(TtfJson),
    /// Monogramm
    Monogramm(MonogrammJson),
    /// Applikation
    Applikation(ApplikationJson),
    /// Fläche mit Füllmuster
    Muster(MusterJson),
    /// Knopfloch (9 Typen; in Flächen einarbeiten: Namen des Knopflochs in `aussparen` der Fläche)
    Knopfloch(KnopflochJson),
}

impl ObjektJson {
    /// Felder, die alle Objekte haben.
    pub fn gemeinsam(&self) -> &Gemeinsam {
        match self {
            ObjektJson::Lauf(x) | ObjektJson::Dreifach(x) => &x.g,
            ObjektJson::Satin(x) => &x.g,
            ObjektJson::Satinlinie(x) => &x.g,
            ObjektJson::Blanket(x) => &x.g,
            ObjektJson::Tatami(x) => &x.g,
            ObjektJson::Satinflaeche(x) => &x.g,
            ObjektJson::Text(x) => &x.g,
            ObjektJson::Ttf(x) => &x.g,
            ObjektJson::Monogramm(x) => &x.g,
            ObjektJson::Applikation(x) => &x.g,
            ObjektJson::Muster(x) => &x.g,
            ObjektJson::Knopfloch(x) => &x.g,
        }
    }
}

/// **Ein Design**: Rahmen, Stoff, Garnkarte, Objekte. Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[schemars(title = "Design")]
pub struct DesignJson {
    /// Fassung des Formats (ohne Angabe: aktuelle Fassung)
    #[serde(default)]
    pub version: Option<u32>,
    /// Name des Designs (erscheint in der Designliste der Maschine)
    #[serde(default = "design_name")]
    pub name: String,
    /// Stickrahmen (Befehl `rahmen_liste`)
    #[serde(default = "standard_rahmen")]
    #[schemars(schema_with = "rahmen_schema")]
    pub rahmen: String,
    /// Stoff: ändert Dichte, Unterlage, Zugausgleich (8 Stoffe + Software Defaults)
    #[serde(default = "standard_stoff")]
    #[schemars(schema_with = "stoff_schema")]
    pub stoff: String,
    /// Garnkarte (Befehl `garnkarten_liste`): jede Farbe → nächstes Garn; „keine“ = ohne (Standard „Default“ wie die
    /// Maschine)
    #[serde(default = "standard_garnkarte")]
    pub garnkarte: String,
    /// Einzelwerte des Stoffs/Stils überschreiben, z. B. {"satin": {"zugausgleich": 0.3}} (Parameterkatalog)
    #[serde(default)]
    pub einstellungen: Option<EinstellungenJson>,
    /// Stickobjekte in Stickreihenfolge
    #[serde(default)]
    pub objekte: Vec<ObjektJson>,
    /// Startpunkt (Standard: Rahmenmitte)
    #[serde(default)]
    pub start: Option<Start>,
    /// Endpunkt (Standard: Sprung zurück zur Rahmenmitte)
    #[serde(default)]
    pub ende: Option<Ende>,
    /// Closest Join: jedes Objekt ohne eigenen Einstich beginnt nahe dem vorigen Ende (nicht bei Schrift)
    #[serde(default)]
    pub kuerzeste_verbindung: bool,
    /// Transformation des ganzen Designs (um seine Mitte)
    #[serde(default)]
    pub transform: Option<Transformation>,
}

fn rahmen_schema(_: &mut SchemaGenerator) -> Schema {
    let namen: Vec<String> = crate::rahmen::alle_rahmen().into_iter().map(|r| r.name).collect();
    json_schema!({ "type": "string", "enum": namen, "default": standard_rahmen() })
}

fn stoff_schema(_: &mut SchemaGenerator) -> Schema {
    let namen: Vec<&str> = crate::einstellungen::STOFFE.iter().map(|s| s.name).collect();
    json_schema!({ "type": "string", "enum": namen, "default": standard_stoff() })
}

/// Einzelwerte des Stils überschreiben – am Design und je Objekt (dort ohne `stoff`). Aufbau wie
/// `einstellungen::Einstellungen`, jedes Feld darf fehlen; geprüft wird beim Lesen (`Einstellungen::ueberschreiben`).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(transparent)]
pub struct EinstellungenJson(pub Value);

impl JsonSchema for EinstellungenJson {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "EinstellungenJson".into()
    }
    fn json_schema(g: &mut SchemaGenerator) -> Schema {
        einstellungen_schema(g)
    }
}

/// Schema der Einstellungen zum Überschreiben: wie `Einstellungen`, aber jedes Feld darf fehlen; Standardwerte =
/// Software Defaults (die Werte je Stoff stehen im Parameterkatalog).
fn einstellungen_schema(_: &mut SchemaGenerator) -> Schema {
    let mut s = schemars::generate::SchemaSettings::draft2020_12();
    s.inline_subschemas = true;
    let mut schema = s
        .into_generator()
        .into_root_schema_for::<crate::einstellungen::Einstellungen>();
    let mut v = schema.as_value().clone();
    let standard = crate::einstellungen::fuer_stoff(&standard_stoff())
        .ok()
        .and_then(|e| serde_json::to_value(e).ok())
        .unwrap_or(Value::Null);
    fn lockern(v: &mut Value, standard: &Value) {
        let Some(o) = v.as_object_mut() else {
            return;
        };
        o.remove("required");
        o.remove("$schema");
        o.remove("title");
        if !standard.is_object() && !standard.is_null() {
            o.insert("default".into(), standard.clone());
        }
        if let Some(Value::Object(props)) = o.get_mut("properties") {
            for (k, x) in props.iter_mut() {
                lockern(x, standard.get(k).unwrap_or(&Value::Null));
            }
        }
    }
    lockern(&mut v, &standard);
    if let Some(o) = v.as_object_mut() {
        o.insert(
            "description".into(),
            "Einzelwerte des Stils überschreiben (Parameterkatalog `PARAMETER.md`); jedes Feld darf fehlen; `stoff` nur \
             am Design"
                .into(),
        );
        o.insert("type".into(), Value::from(vec!["object", "null"]));
    }
    schema = Schema::try_from(v).unwrap_or_default();
    schema
}

/// JSON-Schema des Design-Formats (Entwurf 2020-12), Standardwerte und Beschreibungen aus diesem Modul.
pub fn schema() -> Value {
    let g = schemars::generate::SchemaSettings::draft2020_12().into_generator();
    let mut v = g.into_root_schema_for::<DesignJson>().to_value();
    if let Some(o) = v.as_object_mut() {
        o.insert("$id".into(), "https://artista-stickprogramm/design.schema.json".into());
        o.insert("x-version".into(), VERSION.into());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Alle Beispieldesigns und Referenz-Designs lassen sich lesen (Format bleibt rückwärts verträglich).
    #[test]
    fn beispiele_lesen() {
        let mut n = 0;
        for ordner in ["../../stickprogramm/beispiele", "../testdaten"] {
            let ordner = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(ordner);
            let mut stapel = vec![ordner];
            while let Some(o) = stapel.pop() {
                for e in std::fs::read_dir(&o).unwrap().flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        stapel.push(p);
                    } else if p.extension().is_some_and(|x| x == "json") {
                        let text = std::fs::read_to_string(&p).unwrap();
                        let v: Value = serde_json::from_str(&text).unwrap();
                        if v.get("objekte").is_none() {
                            continue;
                        }
                        crate::Design::aus_value(&v).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
                        n += 1;
                    }
                }
            }
        }
        assert!(n >= 5, "nur {n} Designs gefunden");
    }

    #[test]
    fn fassung_und_fehler() {
        assert!(migrieren(&serde_json::json!({"version": VERSION + 1})).is_err());
        assert!(migrieren(&serde_json::json!({"version": 1})).is_ok());
        let e = crate::Design::aus_json(
            r#"{"objekte": [{"art": "lauf", "punkte": [[0, 0], [1, 0]]},
            {"art": "satinlinie", "punkte": [[0, 0], [5, 0]], "versatz": "quer"}]}"#,
        )
        .unwrap_err();
        assert!(
            e.starts_with("Objekt 2 (satinlinie): Wert „quer“ unbekannt (erlaubt: mitte, seite)"),
            "{e}"
        );
        let e = crate::Design::aus_json(r#"{"objekte": [{"art": "kreis"}]}"#).unwrap_err();
        assert!(e.contains("Objekt 1") && e.contains("„kreis“ unbekannt"), "{e}");
        let e = crate::Design::aus_json(r#"{"objekte": [{"art": "tatami"}]}"#).unwrap_err();
        assert!(e.contains("Feld „umriss“ fehlt"), "{e}");
    }

    #[test]
    fn schema_ausgeben() {
        let s = schema();
        std::fs::write(
            std::env::temp_dir().join("design.schema.json"),
            serde_json::to_string_pretty(&s).unwrap(),
        )
        .unwrap();
        let defs = s["$defs"].as_object().unwrap();
        assert!(defs.contains_key("ObjektJson") && defs.contains_key("Punkt"));
    }
}
