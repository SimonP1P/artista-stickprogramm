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
    /// Woher eine Gruppe stammt (nur für die Anzeige in der Oberfläche)
    pub enum GruppenHerkunft {
        /// von Hand gruppiert („Group“, Standard)
        Gruppiert = "gruppiert",
        /// Bild umwandeln (`bild_umwandeln`)
        Bild = "bild",
        /// SVG einlesen (`svg_lesen`)
        Svg = "svg",
    }
}

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
    /// Auflösung von PhotoSnap = Breite der Stichreihen
    pub enum PhotoAufloesung {
        /// grob (Reihen 3,0 mm)
        Grob = "grob",
        /// mittel (Reihen 2,0 mm, Standard)
        Mittel = "mittel",
        /// fein (Reihen 1,0 mm)
        Fein = "fein",
    }
}

auswahl! {
    /// Stoff hinter PhotoSnap
    pub enum PhotoHintergrund {
        /// heller Stoff: dunkle Stellen bekommen viel Faden (Standard)
        Hell = "hell",
        /// dunkler Stoff: helle Stellen bekommen viel Faden
        Dunkel = "dunkel",
    }
}

auswahl! {
    /// Lage der Farbschichten von PhotoSnap zueinander (nur mit mehreren Lagen)
    pub enum PhotoLagen {
        /// je Lage 45° weiter gedreht: 0°, 45°, 90°, 135° (Standard)
        Gekreuzt = "gekreuzt",
        /// gleiche Richtung, Reihen je Lage 0,4 mm versetzt
        Versetzt = "versetzt",
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
        /// automatisch: Satin, solange kein Satinstich länger als 8 mm wird, sonst der ganze Text Step (eigene
        /// Erweiterung – das Original wechselt bei Schrift nicht selbst; Grenze wie EasyStitch, Handbuch S. 118)
        Auto = "auto",
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
        /// Satinsäulen; würde ein Satinstich nach Größe/Transformation länger als 8 mm, der ganze Text Step – wie
        /// Stichart „auto“ der Texte (eigene Erweiterung, `Analyse_Teil3/10` §8)
        Auto = "auto",
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
    /// **Pfad aus Referenzpunkten** wie beim Digitalisieren im Original (Handbuch S. 58: linke Maustaste = Eckpunkt,
    /// rechte = Kurvenpunkt; zwei Eckpunkte verbindet eine Gerade, an Kurvenpunkten läuft die Linie glatt durch).
    /// Kurvenstücke: kubische Bézier mit Catmull-Rom-Tangenten (eigene Festlegung – wie das Original Kurvenpunkte
    /// in Bézier umsetzt, ist nicht analysiert). Geschlossen: Ring ohne doppelten Endpunkt
    Pfad {
        /// Referenzpunkte: [x, y] = Eckpunkt, {"kurve": [x, y]} = Kurvenpunkt
        punkte: Vec<PfadPunkt>,
        /// letzter Punkt mit dem ersten verbunden (Flächen, geschlossene Linien)
        #[serde(default)]
        geschlossen: bool,
    },
}

/// Referenzpunkt eines Pfads: Eckpunkt `[x, y]` oder Kurvenpunkt `{"kurve": [x, y]}`.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum PfadPunkt {
    Ecke(Punkt),
    Kurve { kurve: Punkt },
}
impl PfadPunkt {
    pub fn p(&self) -> Punkt {
        match self {
            PfadPunkt::Ecke(q) | PfadPunkt::Kurve { kurve: q } => *q,
        }
    }
    pub fn ist_kurve(&self) -> bool {
        matches!(self, PfadPunkt::Kurve { .. })
    }
}

/// Stück eines Pfads: Gerade zwischen zwei Eckpunkten oder Bézier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PfadStueck {
    Gerade(Punkt, Punkt),
    Bezier([Punkt; 4]),
}

/// Referenzpunkte → Stücke. Tangente an einem Kurvenpunkt: (nächster − voriger) / 2 (Catmull-Rom; an offenen Enden
/// einseitig), an Eckpunkten keine – zwei Eckpunkte ergeben eine Gerade (eigene Festlegung, siehe `Form::Pfad`).
pub fn pfad_stuecke(punkte: &[PfadPunkt], geschlossen: bool) -> Vec<PfadStueck> {
    let n = punkte.len();
    if n < 2 {
        return Vec::new();
    }
    let q = |i: usize| punkte[i].p();
    let tangente = |i: usize| -> (f64, f64) {
        if !punkte[i].ist_kurve() {
            return (0.0, 0.0);
        }
        let (vor, nach) = if geschlossen {
            ((i + n - 1) % n, (i + 1) % n)
        } else {
            (i.saturating_sub(1), (i + 1).min(n - 1))
        };
        let f = if !geschlossen && (i == 0 || i == n - 1) { 1.0 } else { 0.5 };
        ((q(nach).x - q(vor).x) * f, (q(nach).y - q(vor).y) * f)
    };
    let m = if geschlossen { n } else { n - 1 };
    (0..m)
        .map(|i| {
            let j = (i + 1) % n;
            if !punkte[i].ist_kurve() && !punkte[j].ist_kurve() {
                return PfadStueck::Gerade(q(i), q(j));
            }
            let (ti, tj) = (tangente(i), tangente(j));
            PfadStueck::Bezier([
                q(i),
                p(q(i).x + ti.0 / 3.0, q(i).y + ti.1 / 3.0),
                p(q(j).x - tj.0 / 3.0, q(j).y - tj.1 / 3.0),
                q(j),
            ])
        })
        .collect()
}

/// Pfad als Punkte mit Eckmarken (Referenz-Eckpunkte = Ecke; Ring ohne doppelten Endpunkt).
fn pfad_punkte(punkte: &[PfadPunkt], geschlossen: bool) -> (Linie, Vec<bool>) {
    let (mut out, mut ecken): (Linie, Vec<bool>) = (Vec::new(), Vec::new());
    if punkte.len() == 1 {
        return (vec![punkte[0].p()], vec![true]);
    }
    let stuecke = pfad_stuecke(punkte, geschlossen);
    let n = punkte.len();
    for (i, s) in stuecke.iter().enumerate() {
        let pts = match s {
            PfadStueck::Gerade(a, b) => vec![*a, *b],
            PfadStueck::Bezier(b) => bezier(b[0], b[1], b[2], b[3], 32),
        };
        let start = usize::from(!out.is_empty());
        for (k, x) in pts.iter().enumerate().skip(start) {
            out.push(*x);
            let ende = k + 1 == pts.len();
            ecken.push(if k == 0 { !punkte[i].ist_kurve() } else if ende { !punkte[(i + 1) % n].ist_kurve() } else { false });
        }
    }
    if geschlossen && out.len() > 1 {
        // Ring: der letzte Punkt ist wieder der erste
        out.pop();
        ecken.pop();
    }
    (out, ecken)
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
            PunktAngabe::Form(Form::Pfad { punkte, geschlossen }) => pfad_punkte(punkte, *geschlossen).0,
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
    /// sind Ecken. Beim Verbinden zählt die Nahtstelle als Ecke – außer zwischen zwei Bézierstücken, die ohne Knick
    /// ineinander übergehen (Anfasser auf einer Linie, < 1°): das ist ein Kurvenknoten wie im Original (eigene
    /// Festlegung; so bleiben in Knoten umgewandelte Kreise ohne Ecken, `umwandeln`).
    pub fn punkte_mit_ecken(&self) -> (Linie, Vec<bool>) {
        match self {
            PunktAngabe::Liste(_) | PunktAngabe::Form(Form::Rechteck { .. }) => {
                let pts = self.punkte();
                let n = pts.len();
                (pts, vec![true; n])
            }
            PunktAngabe::Form(Form::Pfad { punkte, geschlossen }) => pfad_punkte(punkte, *geschlossen),
            PunktAngabe::Form(Form::Verbinden(teile)) => {
                let (mut out, mut ecken): (Linie, Vec<bool>) = (Vec::new(), Vec::new());
                let mut vorher: Option<&PunktAngabe> = None;
                for t in teile {
                    let (pts, e) = t.punkte_mit_ecken();
                    let start = usize::from(matches!((out.last(), pts.first()), (Some(l), Some(f)) if l == f));
                    if let (1, Some(letzte)) = (start, ecken.last_mut()) {
                        *letzte = !vorher.is_some_and(|v| ohne_knick(v, t));
                    }
                    vorher = Some(t);
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

/// Gehen zwei Bézierstücke ohne Knick ineinander über (Richtung am Ende von `a` = Richtung am Anfang von `b`)?
fn ohne_knick(a: &PunktAngabe, b: &PunktAngabe) -> bool {
    let (PunktAngabe::Form(Form::Bezier(a)), PunktAngabe::Form(Form::Bezier(b))) = (a, b) else {
        return false;
    };
    let (u, v) = ((a[3].x - a[2].x, a[3].y - a[2].y), (b[1].x - b[0].x, b[1].y - b[0].y));
    let (lu, lv) = (u.0.hypot(u.1), v.0.hypot(v.1));
    if lu < 1e-9 || lv < 1e-9 {
        return false;
    }
    let kreuz = (u.0 * v.1 - u.1 * v.0) / (lu * lv);
    let punkt = (u.0 * v.0 + u.1 * v.1) / (lu * lv);
    punkt > 0.0 && kreuz.abs() < 1f64.to_radians().sin()
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
    /// an/aus – alte Schreibweise der Computer-Schrift: `true` = "auto", `false` = "keine"
    Schalter(bool),
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

auswahl! {
    /// Stichart der Umrandung
    pub enum UmrandungArt {
        /// Laufstich (dünne Linie)
        Lauf = "lauf",
        /// Dreifachstich (kräftige Linie, Standard)
        Dreifach = "dreifach",
        /// Satinkante
        Satin = "satin",
    }
}

fn umrandung_art() -> UmrandungArt {
    UmrandungArt::Dreifach
}
fn umrandung_einzug() -> f64 {
    crate::design::UMRANDUNG_EINZUG
}
fn umrandung_breite() -> f64 {
    2.0
}

/// **Umrandung** – eigene Erweiterung (Wunsch Nutzer 21.11.2026; im Original legt man dafür von Hand eine zweite Linie
/// an, Handbuch S. 114): eine Linie um die Fläche des Objekts, gestickt gleich nach dem Objekt in eigener Farbe. Bei
/// Text läuft sie um jeden Buchstaben, bei mehreren Teilen um jeden Teil (Löcher bekommen auch eine Linie). Sie liegt um
/// `einzug` nach innen, damit nach dem Zusammenziehen des Stoffs keine Lücke zwischen Füllung und Umrandung bleibt.
/// Wirkt bei Objekten mit Fläche (Flächen, Satin, Satinlinie, Text, Monogramm, Applikation); offene Linien haben keine.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UmrandungJson {
    /// Stichart der Linie (Standard Dreifach)
    #[serde(default = "umrandung_art")]
    pub art: UmrandungArt,
    /// Garnfarbe „#rrggbb“ (Standard schwarz)
    #[serde(default = "schwarz")]
    pub farbe: String,
    /// Versatz der Linie nach innen in mm, 0 … 3 (Standard 0,3 – eigene Festlegung, am Kalibrierblatt V12 prüfen); bei
    /// `aussen`: wie weit die Innenkante der Satinkante in der Fläche liegt
    #[serde(default = "umrandung_einzug")]
    pub einzug: f64,
    /// Breite der Satinkante in mm, 0,5 … 10 (nur bei `art` satin; Standard 2 wie die Satinlinie). Die Kante ist ein Band
    /// um die Fläche, in Satinsäulen zerlegt wie „Satin automatisch“ (`satinflaeche`)
    #[serde(default = "umrandung_breite")]
    pub breite: f64,
    /// Breite der Satinkante in **Prozent der Buchstabenhöhe**, 1 … 30 (nur Text und Monogramm, gilt statt `breite`;
    /// Größe/Skalieren zählt mit). Einfacher Modus: 6 → 3 mm bei 50 mm hohen Buchstaben; unter 1 mm wird daraus eine
    /// kräftige Linie (Dreifach, Stichlänge 12 % der Höhe, 1–2,5 mm) wie bei EasyStitch (Wunsch Nutzer 21.11.2026,
    /// eigene Festlegung)
    #[serde(default)]
    pub breite_anteil: Option<f64>,
    /// Satinkante **außen** um die Fläche (nur bei `art` satin): die Innenkante liegt `einzug` mm in der Fläche, die
    /// Kante wächst nach außen, die Farbe des Objekts bleibt ganz sichtbar (dicke Umrandung im einfachen Modus). Ohne:
    /// Mitte der Kante `einzug` mm in der Fläche (eigene Festlegung, Wunsch Nutzer 21.11.2026)
    #[serde(default)]
    pub aussen: bool,
    /// Stichlänge in mm bei Lauf/Dreifach, 0,3 … 12,7 (ohne Angabe: Stil)
    #[serde(default)]
    pub laenge: Option<f64>,
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
    /// **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
    /// Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
    /// Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
    #[serde(default)]
    pub gruppe: Option<String>,
    /// Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
    #[serde(default)]
    pub doppellauf: bool,
    /// **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
    /// 0,3 mm nach innen
    #[serde(default)]
    pub umrandung: Option<UmrandungJson>,
    /// Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm)
    #[serde(default)]
    pub auto_jump: Option<AutoJumpJson>,
    /// Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil)
    #[serde(default)]
    pub step_muster: Option<usize>,
    /// **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
    /// „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
    /// die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
    #[serde(default)]
    pub garn: Option<String>,
    /// Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort
    #[serde(default)]
    pub einstich: Option<Punkt>,
    /// Ausstich („exit point“) [x, y] in mm: das Objekt endet dort
    #[serde(default)]
    pub ausstich: Option<Punkt>,
    /// **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
    /// `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
    /// Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
    #[serde(default)]
    pub kuerzeste: bool,
    /// **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
    /// `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
    /// Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
    #[serde(default)]
    pub einstellungen: Option<EinstellungenJson>,
    /// **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
    /// Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
    /// Flächen), sonst ohne Wirkung. An = Feld vorhanden
    #[serde(default)]
    pub effekte: Option<EffekteJson>,
}

/// Effekte eines Objekts (`stick_kern::effekte`, Bericht `Analyse_Teil3/23_effekte.md`).
#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EffekteJson {
    /// **Verlauf** (Gradient Fill): Abstand wechselt quer über die Fläche bzw. entlang der Säule zwischen `min` und
    /// `max`; der Abstand des Objekts gilt dann nicht (Handbuch S. 218)
    #[serde(default)]
    pub verlauf: Option<VerlaufJson>,
    /// **Ausgefranster Rand** (Textured Edge): Reihen- bzw. Stichenden streuen um den Rand (Handbuch S. 213–214)
    #[serde(default)]
    pub rand: Option<RandJson>,
    /// **Welle** (Wave Effect): Leitlinie [[x, y], …] in mm quer durch die Fläche, die Reihen folgen ihr (nur
    /// `tatami` und `muster`; Handbuch S. 217–218: Stichlänge ≤ 4 mm empfohlen)
    #[serde(default)]
    pub welle: Option<PunktAngabe>,
}

/// Verlauf (Gradient Fill, ART-Datensatz 0x34). Standard wie der Effekt-Datensatz im Stil der Musterdesigns
/// (0,4 / 2,0 mm, Profil 1).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VerlaufJson {
    /// Profil (Dialog „Gradient Fill“ und „Color Blending“)
    #[serde(default = "profil_linear")]
    pub profil: crate::effekte::Profil,
    /// kleinster Abstand in mm (dichteste Stelle; 0,1 … 10)
    #[serde(default = "verlauf_min")]
    pub min: f64,
    /// größter Abstand in mm (offenste Stelle, „Maximum Spacing“; 0,1 … 10)
    #[serde(default = "verlauf_max")]
    pub max: f64,
}

fn profil_linear() -> crate::effekte::Profil {
    crate::effekte::Profil::Linear
}
fn verlauf_min() -> f64 {
    0.4
}
fn verlauf_max() -> f64 {
    2.0
}

/// Ausgefranster Rand (Textured Edge, ART-Datensatz 0x32: Seite, Spanne, Texture). Standard wie der Effekt-Datensatz
/// im Stil der Musterdesigns (Seite 2, Spanne 1,5 mm).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RandJson {
    /// Seite in Stichrichtung (links = Anfang der Reihen bzw. Satin-Seite A)
    #[serde(default = "rand_rechts")]
    pub seite: crate::effekte::RandSeite,
    /// Spanne in mm („Span“): Enden streuen bis so weit nach innen und außen (0,1 … 10)
    #[serde(default = "rand_spanne")]
    pub spanne: f64,
    /// Glätte 0 … 1 („Texture“-Regler: 0 = rau, jede Reihe frei; 1 = glatt)
    #[serde(default = "rand_glaette")]
    pub glaette: f64,
}

fn rand_rechts() -> crate::effekte::RandSeite {
    crate::effekte::RandSeite::Rechts
}
fn rand_spanne() -> f64 {
    1.5
}
fn rand_glaette() -> f64 {
    0.1
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

/// Rückstich (Backstitch, Craft Stitch 0x2850): Stiche vor und zurück mit Überlappung, wie von Hand gestickt – eine
/// feine, durchgehende Linie. Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct RueckstichJson {
    pub punkte: PunktAngabe,
    /// Linie schließen (zurück zum ersten Punkt)
    #[serde(default)]
    pub geschlossen: bool,
    /// Stichlänge in mm (0,5 … 12,7; Original 3,0)
    pub laenge: Option<f64>,
    /// Fadenstärke in mm = Abstand der Hin- und Rückstiche (0 … 1; Original 0,2)
    pub staerke: Option<f64>,
    /// Überlappung in % des Stichs, um die jeder Rückstich in den vorigen greift (0 … 100; Original 50)
    pub ueberlappung: Option<f64>,
    /// Striche je Stich: 3 oder 5 (Original 3)
    pub striche: Option<u8>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Stielstich (Stemstitch, Craft Stitch 0x2855): schräge, sich überlappende Stiche zu einer kordelartigen Linie.
/// Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct StielstichJson {
    pub punkte: PunktAngabe,
    /// Linie schließen (zurück zum ersten Punkt)
    #[serde(default)]
    pub geschlossen: bool,
    /// Linienstärke in mm = Breite der Kordel (0,5 … 10; Original 2,0)
    pub breite: Option<f64>,
    /// Abstand der Stiche in mm, quer zu den Stichen gemessen (0,2 … 10; Original 1,5)
    pub abstand: Option<f64>,
    /// Winkel der Stiche zur Linie in Grad (10 … 90; Original 50)
    pub winkel: Option<f64>,
    /// dreifach („Triple“: drei Fäden nebeneinander) statt einfach (Original: dreifach)
    pub dreifach: Option<bool>,
    /// Fadenstärke in mm = Abstand der drei Fäden bei dreifach (0 … 1; Original 0,2)
    pub staerke: Option<f64>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Stippling (Craft Stitch „Stipple“): die Fläche wird mit einer mäandernden Laufstichlinie gefüllt, die sich nicht
/// kreuzt.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct StipplingJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Stichlänge in mm (0,5 … 12,7; Handbuch-Beispiel 2,0)
    pub laenge: Option<f64>,
    /// Schlaufenabstand in mm = Abstand benachbarter Bahnen (0,5 … 20; Handbuch-Beispiel 3,0)
    pub abstand: Option<f64>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

auswahl! {
    /// Stichart eines Blackwork-Zugs
    pub enum BlackworkStich {
        /// Laufstich (Standard)
        Lauf = "lauf",
        /// Dreifachstich
        Dreifach = "dreifach",
        /// Rückstich
        Rueckstich = "rueckstich",
        /// Stielstich
        Stielstich = "stielstich",
    }
}

fn blackwork_stich() -> BlackworkStich {
    BlackworkStich::Lauf
}

/// Blackwork Run (Handbuch S. 131): mehrere sich überlappende Linien als **ein** Objekt; die Reihenfolge ergibt sich
/// automatisch (ein Zug je zusammenhängendem Netz, Abschnitte nur doppelt, wo es nötig ist, ein Ein- und ein
/// Ausstich). Jede Linie bleibt einzeln formbar. Werte der Stichart wie bei `lauf`, `rueckstich`, `stielstich`.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct BlackworkJson {
    /// die Linien (mindestens eine)
    pub linien: Vec<PunktAngabe>,
    /// Stichart aller Linien (Standard Laufstich)
    #[serde(default = "blackwork_stich")]
    pub stich: BlackworkStich,
    /// Stichlänge in mm (Lauf/Dreifach: Stil 2,5; Rückstich 3,0)
    pub laenge: Option<f64>,
    /// Rückstich/Stielstich: Fadenstärke in mm (0,2)
    pub staerke: Option<f64>,
    /// Rückstich: Überlappung in % (50)
    pub ueberlappung: Option<f64>,
    /// Rückstich: 3 oder 5 Striche (3)
    pub striche: Option<u8>,
    /// Stielstich: Linienstärke in mm (2,0)
    pub breite: Option<f64>,
    /// Stielstich: Abstand in mm (1,5)
    pub abstand: Option<f64>,
    /// Stielstich: Winkel in Grad (50)
    pub winkel: Option<f64>,
    /// Stielstich: dreifach (ja)
    pub dreifach: Option<bool>,
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
    /// Unterlage: `"auto"` (Standard, Stil „Schrift“ wie im Original: Zickzack unter den Satinsäulen, Step unter
    /// Flächen), `"keine"` oder Arten in Stickfolge für Satin/auto, z. B. `["mittellauf", "zickzack"]` (Tatami: jede
    /// Liste = an). `true`/`false` (alte Schreibweise) = auto/keine.
    #[serde(default = "auto")]
    pub unterlage: UnterlageJson,
    /// Stichlänge bei Lauf/Dreifach in mm
    pub laenge: Option<f64>,
    #[serde(flatten)]
    pub schrift_optionen: SchriftOptionen,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Monogramm (Handbuch Kap. 21): 2 oder 3 Initialen mit den Alphabeten Diamond2/Diamond3, Rahmen wahlweise. Wird im
/// Kern zu einem Text – alle Schrift-Einstellungen gelten. **Mit `vorlage`** (eigene Erweiterung): Rahmen aus dem
/// Katalog der Monogramm-Rahmen, Initialen in einer Computer-Schrift oder einem Alphabet (`monogramm.rs`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MonogrammJson {
    /// 2 oder 3 Buchstaben A–Z (Diamond); mit `vorlage` 1–3 Zeichen
    pub initialen: String,
    #[serde(default)]
    pub rahmen: Option<MonogrammRahmen>,
    /// Farbe des Rahmens (nur mit Rahmen)
    pub rahmen_farbe: Option<String>,
    /// **Rahmen aus dem Katalog** (eigene Erweiterung): Name eines Monogramm-Rahmens (Befehl `monogramm_rahmen`, z. B.
    /// „Doppeloval“). Dann stehen die Initialen in der Computer-Schrift `schrift` (ohne Angabe: Schrift der Vorlage)
    /// oder im Alphabet `alphabet`, Höhen nach der Vorlage, mittig im Rahmen und so weit verkleinert, dass rundum 2 mm
    /// frei bleiben. `rahmen`, `rahmen_farbe` und `hoehe` gelten dann nicht
    pub vorlage: Option<String>,
    /// Computer-Schrift der Initialen (nur mit `vorlage`; ohne Angabe: Schriftvorschlag der Vorlage, Google Fonts)
    #[serde(rename = "schrift")]
    pub ttf: Option<String>,
    /// Original-Alphabet der Initialen (nur mit `vorlage`, statt `schrift`)
    pub alphabet: Option<String>,
    /// Größe des Rahmens in mm: längere Seite (nur mit `vorlage`; ohne Angabe wie gezeichnet, 70–88 mm). Die
    /// Initialen wachsen mit
    pub rahmen_groesse: Option<f64>,
    /// Farben des Rahmens statt der Farben der Vorlage: 1., 2., 3. Farbe der Vorlage (nur mit `vorlage`; fehlende
    /// bleiben, wie sie sind)
    #[serde(default)]
    pub rahmen_farben: Vec<String>,
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

/// Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`). Ohne Angabe gelten die Vorgaben von
/// „Auto Appliqué“ im Original (`AutoAppliqueValues`).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct ApplikationJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Breite des Abdeckstichs in mm (0,5 … 10; Original 5,0)
    pub breite: Option<f64>,
    /// Abstand des Abdeckstichs in mm (0,2 … 10; Original Satin 0,5, Blanket 2,0)
    pub abstand: Option<f64>,
    /// Breite des Zickzack-Heftstichs in mm (0,5 … 10; Original 4,0)
    pub heft_breite: Option<f64>,
    /// Abstand des Heftstichs in mm (0,5 … 12,7; Original 5,0)
    pub heft_abstand: Option<f64>,
    /// Stichlänge der Platzierungslinie in mm (0,5 … 12,7; Original 1,5)
    pub platz_laenge: Option<f64>,
    pub abdeckung: Option<Abdeckung>,
    pub stil: Option<ApplikationStil>,
    /// Versatz von Abdeck- und Heftstich gegen die Platzierungslinie in % (Schieberegler „Inside ↔ Outside“ des
    /// Originals): −100 = ganz außen, 0 = mittig (Standard), 100 = ganz im Stoff. Bei Blanket ohne Wirkung (immer
    /// innen).
    pub versatz: Option<f64>,
    /// Teil-Abdeckung (Partial Appliqué): kein Abdeckstich, wo eine spätere Applikation im Design den Rand überdeckt –
    /// überlappende Stoffteile ohne doppelte Ränder
    #[serde(default)]
    pub teilabdeckung: bool,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Eine Farblage von PhotoSnap.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PhotoLageJson {
    /// Garnfarbe der Lage „#rrggbb“ (bei nur einer Lage gilt `farbe` des Objekts)
    pub farbe: String,
    /// Deckung je Bildpunkt als Base64 (ein Byte je Punkt, zeilenweise von oben; 0 = kein Faden, 255 = voll – für
    /// hellen Stoff)
    pub werte: String,
}

/// Bildraster von PhotoSnap (vom Befehl `photosnap` erzeugt, nicht von Hand).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PhotoRasterJson {
    /// Bildpunkte waagrecht
    pub breite: usize,
    /// Bildpunkte senkrecht
    pub hoehe: usize,
    /// Größe eines Bildpunkts in mm
    pub mm: f64,
    /// Farblagen in Stickreihenfolge: eine (Graustufen) oder vier (Cyan, Magenta, Gelb, Schwarz)
    pub lagen: Vec<PhotoLageJson>,
}

/// PhotoSnap (Handbuch S. 119–120): Foto als Reihen von Zickzackstichen, deren Abstand der Helligkeit folgt („line
/// printer“). Bildmitte = Mitte des Objekts; Lage und Größe über `transform`.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct PhotosnapJson {
    pub raster: PhotoRasterJson,
    /// Auflösung = Breite der Reihen (Standard mittel 2,0 mm)
    pub aufloesung: Option<PhotoAufloesung>,
    /// Stoff hell (Standard) oder dunkel
    pub hintergrund: Option<PhotoHintergrund>,
    /// Winkel der Reihen in Grad (Standard 0 wie im Original; das Handbuch-Beispiel zeigt 45°)
    pub winkel: Option<f64>,
    /// Farblagen gekreuzt (Standard) oder versetzt
    pub lagen: Option<PhotoLagen>,
    /// kleinster Abstand in mm an den dunkelsten Stellen (0,4 … 5; Standard 0,8 – Stil 0x2828)
    pub abstand_min: Option<f64>,
    /// größter Abstand in mm an den hellsten Stellen (1 … 12; Standard 10 – Stil 0x2828)
    pub abstand_max: Option<f64>,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// **Stichdaten** aus einer fremden Stickdatei (DST, PES, JEF, VP3, EXP; Befehl `stickdatei_lesen`): feste Einstiche
/// wie „Grade D“ im Original – werden nicht neu berechnet und nicht zusätzlich vernäht. Transformationen bewegen die
/// Einstiche; Skalieren ändert die Dichte, nicht die Stichzahl (Hinweis ab ±5 % wie im Handbuch S. 256).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct SticheJson {
    /// Einstiche, Base64 (vom Befehl `stickdatei_lesen` erzeugt – nicht von Hand bearbeiten): Datensätze x, y als i16
    /// in 0,1 mm, +y oben; (−32768, 0) beginnt einen neuen Zug (Sprung)
    pub stiche: String,
    /// Herkunft, z. B. „rose.pes (PES)“ – nur zur Anzeige
    #[serde(default)]
    pub quelle: Option<String>,
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

/// Motivlinie (Handbuch Kap. 25 „Pattern Run“): ein Motiv aus einem Motivsatz (`motive_liste`) wird entlang der Linie
/// wiederholt. Anzahl = Linienlänge / Abstand gerundet, die Motive werden gleichmäßig verteilt (wie das Original).
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MotivlinieJson {
    pub punkte: PunktAngabe,
    /// Linie schließen (Motive ringsum)
    #[serde(default)]
    pub geschlossen: bool,
    /// Motivsatz, z. B. „Heirloom“ (Befehl `motive_liste`; Standard „NP004a“ wie das Original,
    /// `<PRESET_STANDARD_MOTIF_RUN>`)
    #[serde(default = "motiv_satz")]
    pub satz: String,
    /// Motiv im Satz, z. B. „705“ (Standard „K0001a“ wie das Original)
    #[serde(default = "motiv_name")]
    pub motiv: String,
    /// Länge eines Motivs entlang der Linie in mm („Size“); ohne Angabe: Größe im Motivsatz
    #[serde(default)]
    pub groesse: Option<f64>,
    /// Wiederholabstand in mm („Spacing“); ohne Angabe = Größe (Motive stoßen aneinander)
    #[serde(default)]
    pub abstand: Option<f64>,
    /// Motive auf die andere Seite der Linie spiegeln
    #[serde(default)]
    pub spiegeln: bool,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

/// Motivfüllung (Handbuch Kap. 25 „Pattern Fill“): ein Motiv aus einem Motivsatz wird in parallelen Reihen über die
/// Fläche wiederholt. Motive, die mehr als zur Hälfte über den Rand ragen, werden am Rand abgeschnitten.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MotivfuellungJson {
    pub umriss: PunktAngabe,
    #[serde(default)]
    pub loecher: Vec<PunktAngabe>,
    /// Motivsatz (Befehl `motive_liste`; Standard „NP004a“ wie das Original, `<PRESET_STANDARD_MOTIF_FILL>`)
    #[serde(default = "fuell_satz")]
    pub satz: String,
    /// Motiv im Satz (Standard „K0001a“ wie das Original)
    #[serde(default = "fuell_motiv")]
    pub motiv: String,
    /// Länge eines Motivs (Grundlinie) in mm; ohne Angabe: Größe im Motivsatz
    #[serde(default)]
    pub groesse: Option<f64>,
    /// Abstand der Motive in einer Reihe in mm („column spacing“); ohne Angabe: Breite des Motivs
    #[serde(default)]
    pub spaltenabstand: Option<f64>,
    /// Abstand der Reihen in mm („row spacing“); ohne Angabe: Höhe des Motivs
    #[serde(default)]
    pub reihenabstand: Option<f64>,
    /// Reihenversatz in mm („row offset“): jede Reihe um so viel weiter verschoben als die vorige (0 = Spalten
    /// genau untereinander; halber Spaltenabstand = versetzt wie Ziegel)
    #[serde(default)]
    pub versatz: f64,
    /// Drehung der Reihen in Grad (der Stichwinkel wirkt hier nicht, Handbuch S. 227)
    #[serde(default)]
    pub winkel: f64,
    #[serde(flatten)]
    pub g: Gemeinsam,
}

fn fuell_satz() -> String {
    crate::motiv::FUELL_SATZ.into()
}
fn fuell_motiv() -> String {
    crate::motiv::FUELL_MOTIV.into()
}

fn motiv_satz() -> String {
    crate::motiv::LINIE_SATZ.into()
}
fn motiv_name() -> String {
    crate::motiv::LINIE_MOTIV.into()
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
    /// Motivlinie (Motiv aus einem Motivsatz entlang einer Linie)
    Motivlinie(MotivlinieJson),
    /// Motivfüllung (Motiv aus einem Motivsatz in Reihen über die Fläche)
    Motivfuellung(MotivfuellungJson),
    /// Rückstich (Backstitch)
    Rueckstich(RueckstichJson),
    /// Stielstich (Stemstitch)
    Stielstich(StielstichJson),
    /// Stippling (mäandernde Laufstichfüllung)
    Stippling(StipplingJson),
    /// Blackwork Run: mehrere Linien als ein automatisch geordneter Zug
    Blackwork(BlackworkJson),
    /// PhotoSnap: Foto als Reihen von Zickzackstichen mit wechselndem Abstand (Befehl `photosnap` legt es an)
    Photosnap(PhotosnapJson),
    /// Stichdaten aus einer fremden Stickdatei (feste Einstiche, Befehl `stickdatei_lesen` legt sie an)
    Stiche(SticheJson),
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
            ObjektJson::Motivlinie(x) => &x.g,
            ObjektJson::Motivfuellung(x) => &x.g,
            ObjektJson::Rueckstich(x) => &x.g,
            ObjektJson::Stielstich(x) => &x.g,
            ObjektJson::Stippling(x) => &x.g,
            ObjektJson::Blackwork(x) => &x.g,
            ObjektJson::Photosnap(x) => &x.g,
            ObjektJson::Stiche(x) => &x.g,
        }
    }
}

auswahl! {
    /// Form eines eigenen Rahmens (Handbuch Kap. 6: Format Rectangle / Oval)
    pub enum RahmenForm {
        /// Rechteck Breite × Höhe
        Rechteck = "rechteck",
        /// Oval: gerader Mittelteil (`gerade`), oben und unten rund bis zur Höhe
        Oval = "oval",
    }
}

fn rahmen_form() -> RahmenForm {
    RahmenForm::Rechteck
}

/// **Eigener Rahmen** (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“): Stickfeld nach eigenen Maßen. Zum Gestalten
/// frei; beim Senden prüft das Programm, in welchen Rahmen der artista 185 die Stiche passen, und nennt ihn (passt
/// keiner, wird nicht gesendet). Die Liste der eigenen Rahmen verwalten `rahmen_speichern`/`rahmen_loeschen`.
#[derive(Clone, Debug, PartialEq, Deserialize, serde::Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EigenerRahmenJson {
    /// Name (Rahmenliste; nicht der Name eines Rahmens der 185)
    pub name: String,
    #[serde(default = "rahmen_form")]
    pub form: RahmenForm,
    /// Breite des Stickfelds in mm (10 … 1000)
    pub breite: f64,
    /// Höhe des Stickfelds in mm (10 … 1000; Oval: Gesamthöhe „Total Height“)
    pub hoehe: f64,
    /// Oval: Höhe des geraden Mittelteils in mm („Rectangle Height“; 0 = Ellipse)
    #[serde(default)]
    pub gerade: f64,
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
    /// Stickrahmen (Befehl `rahmen_liste`); bei einem eigenen Rahmen sein Name (gilt dann `eigener_rahmen`)
    #[serde(default = "standard_rahmen")]
    #[schemars(schema_with = "rahmen_schema")]
    pub rahmen: String,
    /// **Eigener Rahmen** (Rechteck/Oval nach eigenen Maßen): ist das Feld gesetzt, gilt es statt `rahmen`
    #[serde(default)]
    pub eigener_rahmen: Option<EigenerRahmenJson>,
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
    /// **Gruppen** („Group“ im Original): Schlüssel → Name und Transformation. Objekte nennen ihre Gruppe im Feld
    /// `gruppe`. Reihenfolge der Transformationen: je Objekt → je Gruppe (um die Mitte ihrer Objekte) → ganzes Design
    #[serde(default)]
    pub gruppen: std::collections::BTreeMap<String, GruppeJson>,
    /// Vorlage, aus der das Design entstand (Handbuch Kap. 9: „the template from which the design was created“) – nur
    /// zur Anzeige und als Vorschlag bei „Werte in Vorlage speichern“; die Werte selbst stehen in `einstellungen`
    #[serde(default)]
    pub vorlage: Option<String>,
}

/// Eine Gruppe von Objekten (Feld `gruppen` des Designs).
#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GruppeJson {
    /// Name der Gruppe (Elementliste der Oberfläche), z. B. der Dateiname des Bilds
    #[serde(default)]
    pub name: String,
    /// Herkunft (nur Anzeige)
    #[serde(default)]
    pub herkunft: Option<GruppenHerkunft>,
    /// Verschieben, Skalieren, Drehen … der ganzen Gruppe um die Mitte ihrer Objekte – Stiche werden neu berechnet
    #[serde(default)]
    pub transform: Option<Transformation>,
}

fn rahmen_schema(_: &mut SchemaGenerator) -> Schema {
    let namen: Vec<String> = crate::rahmen::alle_rahmen().into_iter().map(|r| r.name).collect();
    // Rahmen der 185 als Beispiele – ein eigener Rahmen (`eigener_rahmen`) hat einen freien Namen
    json_schema!({ "type": "string", "examples": namen, "default": standard_rahmen() })
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

    /// Pfad aus Referenzpunkten: Eckpunkte → Geraden, Kurvenpunkte → glatte Bézier; geschlossen = Ring
    #[test]
    fn pfad_ecken_und_kurven() {
        let a: PunktAngabe = serde_json::from_value(serde_json::json!(
            {"pfad": {"punkte": [[0, 0], [10, 0], [10, 10]]}}
        ))
        .unwrap();
        let (pts, ecken) = a.punkte_mit_ecken();
        assert_eq!(pts.len(), 3);
        assert_eq!(ecken, vec![true, true, true]);
        // Kurvenpunkt in der Mitte: viele Punkte, nur die Enden sind Ecken, die Linie geht durch den Kurvenpunkt
        let k: PunktAngabe = serde_json::from_value(serde_json::json!(
            {"pfad": {"punkte": [[0, 0], {"kurve": [10, 5]}, [20, 0]]}}
        ))
        .unwrap();
        let (pts, ecken) = k.punkte_mit_ecken();
        assert!(pts.len() > 30);
        assert!(ecken[0] && *ecken.last().unwrap() && ecken.iter().filter(|e| **e).count() == 2);
        assert!(pts.iter().any(|q| (q.x - 10.0).abs() < 1e-9 && (q.y - 5.0).abs() < 1e-9));
        // geschlossen: Ring ohne doppelten Endpunkt, Kurve um alle vier Punkte
        let r: PunktAngabe = serde_json::from_value(serde_json::json!(
            {"pfad": {"punkte": [{"kurve": [0, 10]}, {"kurve": [10, 0]}, {"kurve": [0, -10]}, {"kurve": [-10, 0]}], "geschlossen": true}}
        ))
        .unwrap();
        let pts = r.punkte();
        assert_ne!(pts.first(), pts.last());
        let r_max = pts.iter().map(|q| q.x.hypot(q.y)).fold(0.0, f64::max);
        assert!(r_max > 9.9 && r_max < 11.5, "{r_max}");
    }
}
