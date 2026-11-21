//! **Befehlsschicht für die Oberfläche** (ROADMAP 4b.0): alle Befehle an einer Stelle, ein Verteiler
//! `ausfuehren(kontext, befehl, daten)`. Die App bindet nur **einen** Tauri-Befehl ein, der hierher weiterreicht –
//! neue Kern-Befehle brauchen dann keine Änderung an der App:
//!
//! ```ignore
//! #[tauri::command]
//! async fn kern(befehl: String, daten: serde_json::Value, k: tauri::State<'_, std::sync::Arc<stick_api::Kontext>>)
//!     -> Result<serde_json::Value, stick_api::Fehler> {
//!     let k = k.inner().clone();
//!     tauri::async_runtime::spawn_blocking(move || stick_api::ausfuehren(&k, &befehl, daten)).await
//!         .map_err(|e| stick_api::Fehler::from(e.to_string()))?
//! }
//! ```
//!
//! Aus denselben Typen entstehen `kern/schnittstelle/` (JSON-Schemas, `kern.ts`) und `SCHNITTSTELLE.md`
//! (`stick api exportieren`). Die Tests schlagen fehl, wenn diese Dateien veraltet sind.
//!
//! Die Befehle für die Maschine (`maschine_*`, Modul [`maschine`]) laufen ebenfalls hierüber; den Fortschritt meldet
//! die App als Ereignis `maschine_fortschritt` ([`Kontext::fortschritt_melden`]).

pub mod befehle;
pub mod erzeugen;
pub mod maschine;

use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use stick_kern::Design;
use stick_kern::schrift::Alphabet;

/// Fehler eines Befehls: Meldung (deutsch) und, wenn er zu einem Objekt gehört, dessen Nummer (ab 0) – damit die
/// Oberfläche das Objekt markieren kann.
#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
pub struct Fehler {
    pub meldung: String,
    /// Nummer des Objekts in `objekte` (ab 0), wenn bekannt
    pub objekt: Option<usize>,
    /// Art des Fehlers bei Maschinen-Befehlen (für Zustände und nächste Schritte in der Oberfläche)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FehlerCode>,
}

/// Art eines Fehlers der Maschine (ANFORDERUNGEN §4.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FehlerCode {
    /// keine Verbindung: Kabel, Port, Maschine aus
    KeineVerbindung,
    /// Stickmodul nicht erreicht
    Stickmodul,
    /// Maschine antwortet nicht mehr (aus- und einschalten)
    AntwortetNicht,
    /// zurückgelesen weicht ab – Senden abgebrochen, nichts gestickt
    ZurueckgelesenWeichtAb,
    /// Garnfarbtabelle passt nicht
    Farbtabelle,
    /// Design zu groß für die Maschine
    ZuGross,
    /// Design außerhalb des Stickfelds
    Ausserhalb,
    /// vom Anwender abgebrochen
    Abgebrochen,
    /// es läuft schon eine Aktion mit der Maschine
    Belegt,
}

impl Fehler {
    /// Fehler mit Code (Maschine).
    pub fn mit_code(meldung: impl Into<String>, code: FehlerCode) -> Fehler {
        Fehler {
            meldung: meldung.into(),
            objekt: None,
            code: Some(code),
        }
    }
}

impl From<String> for Fehler {
    /// Objektnummer aus „Objekt N …“ der Kern-Meldungen (N ab 1).
    fn from(meldung: String) -> Self {
        let objekt = meldung.match_indices("Objekt ").find_map(|(i, t)| {
            let rest = &meldung[i + t.len()..];
            let ziffern: String = rest.chars().take_while(char::is_ascii_digit).collect();
            ziffern.parse::<usize>().ok().filter(|&n| n >= 1).map(|n| n - 1)
        });
        Fehler {
            meldung,
            objekt,
            code: None,
        }
    }
}

impl From<&str> for Fehler {
    fn from(m: &str) -> Self {
        Fehler::from(m.to_string())
    }
}

impl std::fmt::Display for Fehler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.meldung)
    }
}

/// Ordner der Daten (Alphabete, Garnkarten, Schriften) und Zwischenspeicher – einmal je Programm anlegen.
pub struct Kontext {
    /// Original-Alphabete (ESB)
    pub alphabete: PathBuf,
    /// Garnkarten (.tch)
    pub garnkarten: PathBuf,
    /// Ordner der Computer-Schriften (leer = Schriften des Systems)
    pub schriften: Vec<PathBuf>,
    geladen: Mutex<HashMap<String, Alphabet>>,
    /// Motivsätze (ESB aus `userletw/`): `STICK_MOTIVE`, sonst Ordner `motive` neben den Alphabeten
    pub motive: PathBuf,
    motive_geladen: Mutex<HashMap<String, stick_kern::motiv::Motivsatz>>,
    /// Zustand der Maschinen-Befehle (Abbrechen, Sperre, Fortschritt, Mitschnitte)
    pub(crate) maschine: maschine::MaschinenZustand,
}

impl Kontext {
    pub fn neu(alphabete: PathBuf, garnkarten: PathBuf) -> Kontext {
        let motive = std::env::var("STICK_MOTIVE")
            .map(PathBuf::from)
            .ok()
            .or_else(|| stick_formate::esb::motiv_ordner(&alphabete))
            .unwrap_or_else(|| alphabete.with_file_name("motive"));
        Kontext {
            alphabete,
            garnkarten,
            motive,
            motive_geladen: Mutex::new(HashMap::new()),
            schriften: Vec::new(),
            geladen: Mutex::new(HashMap::new()),
            maschine: Default::default(),
        }
    }

    /// Ordner wie die App: Umgebungsvariablen `STICK_ALPHABETE`/`STICK_GARNKARTEN`, sonst `ressourcen/alphabete`
    /// bzw. `ressourcen/garnkarten` (gebautes Programm), sonst die Ordner der App im Quellbaum.
    pub fn finden(ressourcen: Option<&Path>) -> Kontext {
        let ordner = |var: &str, name: &str| -> PathBuf {
            if let Ok(p) = std::env::var(var) {
                return p.into();
            }
            if let Some(r) = ressourcen.map(|r| r.join(name))
                && r.is_dir()
            {
                return r;
            }
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../app/src-tauri")
                .join(name)
        };
        Kontext::neu(
            ordner("STICK_ALPHABETE", "alphabete"),
            ordner("STICK_GARNKARTEN", "garnkarten"),
        )
    }

    /// Design aus der Eingabe: Rahmen/Stoff der Oberfläche einsetzen, Alphabete (zwischengespeichert) und
    /// Computer-Schriften laden.
    pub fn design(&self, e: &befehle::DesignEingabe) -> Result<Design, Fehler> {
        let mut v = e.design.0.clone();
        let o = v.as_object_mut().ok_or("Das Design muss ein JSON-Objekt sein")?;
        if let Some(r) = &e.rahmen {
            o.insert("rahmen".into(), r.clone().into());
        }
        if let Some(s) = &e.stoff {
            o.insert("stoff".into(), s.clone().into());
        }
        let mut design = Design::aus_value(&v)?;
        self.alphabete_einsetzen(&mut design)?;
        for n in design.motiv_namen() {
            let s = self.motivsatz(&n)?;
            design.motivsaetze.insert(n, s);
        }
        stick_formate::ttf::design_schriften_laden(&mut design, &self.schriften)?;
        Ok(design)
    }

    /// Motivsatz nach Namen (Kopf oder Dateiname) laden, zwischengespeichert.
    pub fn motivsatz(&self, name: &str) -> Result<stick_kern::motiv::Motivsatz, Fehler> {
        let mut g = self.motive_geladen.lock().map_err(|e| Fehler::from(e.to_string()))?;
        if let Some(s) = g.get(name) {
            return Ok(s.clone());
        }
        let ohne = |x: &str| x.trim_start_matches('_').to_lowercase();
        let pfad = stick_formate::esb::verzeichnis(&self.motive)
            .into_iter()
            .find(|(n, p)| {
                ohne(n) == ohne(name) || p.file_stem().is_some_and(|s| ohne(&s.to_string_lossy()) == ohne(name))
            })
            .map(|(_, p)| p)
            .ok_or_else(|| format!("Motivsatz „{name}“ nicht im Ordner {} gefunden", self.motive.display()))?;
        let s = stick_formate::esb::motive_laden(&pfad)?;
        g.insert(name.to_string(), s.clone());
        Ok(s)
    }

    fn alphabete_einsetzen(&self, design: &mut Design) -> Result<(), Fehler> {
        let namen = design.alphabet_namen();
        if namen.is_empty() {
            return Ok(());
        }
        let mut geladen = self.geladen.lock().map_err(|e| Fehler::from(e.to_string()))?;
        if namen.iter().any(|n| !geladen.contains_key(n)) {
            let mut d = Design {
                alphabete: Default::default(),
                ..design.clone()
            };
            stick_formate::esb::design_alphabete_laden(&mut d, &self.alphabete)?;
            geladen.extend(d.alphabete);
        }
        for n in namen {
            if let Some(a) = geladen.get(&n) {
                design.alphabete.insert(n, a.clone());
            }
        }
        Ok(())
    }
}

/// Ein Befehl der Oberfläche: Name, Beschreibung, Schema von Ein- und Ausgabe, Ausführung.
pub struct Befehl {
    pub name: &'static str,
    /// Beschreibung (Doc-Kommentar der Liste in `befehle!`)
    pub beschreibung: &'static str,
    pub eingabe: fn(&mut SchemaGenerator) -> Schema,
    pub ausgabe: fn(&mut SchemaGenerator) -> Schema,
    lauf: fn(&Kontext, Value) -> Result<Value, Fehler>,
}

fn eingabe_lesen<T: serde::de::DeserializeOwned>(v: Value) -> Result<T, Fehler> {
    let v = if v.is_null() {
        Value::Object(Default::default())
    } else {
        v
    };
    serde_json::from_value(v.clone()).map_err(|e| {
        // Fehler im Design: mit Objektnummer wie beim Lesen im Kern
        if let Some(d) = v.get("design")
            && let Err(m) = Design::aus_value(d)
        {
            return Fehler::from(m);
        }
        Fehler::from(format!(
            "Eingabe ungültig: {}",
            stick_kern::format::fehler_deutsch(&e.to_string())
        ))
    })
}

macro_rules! befehle {
    ($( $(#[doc = $doc:literal])+ $name:ident : $ein:ty => $aus:ty ),+ $(,)?) => {
        /// Alle Befehle in der Reihenfolge der Schnittstellen-Doku.
        pub fn alle() -> Vec<Befehl> {
            vec![$(Befehl {
                name: stringify!($name),
                beschreibung: concat!($($doc),+),
                eingabe: |g| g.subschema_for::<$ein>(),
                ausgabe: |g| g.subschema_for::<$aus>(),
                lauf: |k, v| {
                    let e: $ein = eingabe_lesen(v)?;
                    let a: $aus = befehle_aller::$name(k, e)?;
                    serde_json::to_value(a).map_err(|x| Fehler::from(x.to_string()))
                },
            }),+]
        }
    };
}

befehle! {
    /// Stickrahmen der artista 185 mit Stickfeld und Umriss
    rahmen_liste: befehle::Leer => Vec<befehle::RahmenInfo>,
    /// Stoffe (8 + Software Defaults) mit den wichtigsten Werten
    stoffe_liste: befehle::Leer => Vec<befehle::StoffInfo>,
    /// Original-Alphabete (50)
    alphabete_liste: befehle::Leer => Vec<befehle::AlphabetInfo>,
    /// Motivsätze (14 der CD) für die Motivlinie
    motive_liste: befehle::Leer => Vec<befehle::MotivsatzInfo>,
    /// Motive eines Motivsatzes mit Maßen und Linien für die Vorschau
    motivsatz_motive: befehle::MotivsatzName => Vec<befehle::MotivInfo>,
    /// Vorschau eines Original-Alphabets: Umrisse eines Probetexts und Zeichenvorrat
    alphabet_vorschau: befehle::AlphabetVorschauEingabe => befehle::AlphabetVorschau,
    /// Namen der Garnkarten (32 der CD)
    garnkarten_liste: befehle::Leer => Vec<String>,
    /// Garne einer Garnkarte (für die Auswahl von Hand)
    garnkarte_garne: befehle::GarnkartenName => Vec<befehle::GarnInfo>,
    /// Füllmuster (242) mit Linien einer Kachel für die Vorschau
    fuellmuster_liste: befehle::Leer => Vec<befehle::MusterInfo>,
    /// installierte Computer-Schriften (TrueType/OpenType)
    schriften_liste: befehle::Leer => Vec<befehle::SchriftInfo>,
    /// Step-Muster 1–20 (Tabelle des Originals) mit Einstichen einer Probefläche für Vorschau-Kacheln
    step_muster_liste: befehle::Leer => Vec<befehle::StepMusterInfo>,
    /// Parameterkatalog: jeder einstellbare Wert mit Standard, Bereich, Quelle, Modus einfach/experte/automatisch
    parameter_katalog: befehle::Leer => Vec<stick_kern::parameter::Parameter>,
    /// Beispieldesign mit allen Sticharten der ersten Ausbaustufe
    beispiel_design: befehle::Leer => befehle::DesignWert,
    /// JSON-Schema des Design-Formats (Felder, Standardwerte, Auswahlwerte, Beschreibungen)
    design_schema: befehle::Leer => Value,
    /// Design prüfen, ohne Stiche zu berechnen: Fehler (mit Objektnummer) und Hinweise
    design_pruefen: befehle::DesignEingabe => befehle::Pruefung,
    /// Stiche berechnen – bei jeder Änderung am Design aufrufen (Stiche werden nie skaliert, immer neu berechnet)
    design_berechnen: befehle::DesignEingabe => befehle::Ergebnis,
    /// Formen je Objekt (Umrisse, Linien, Satinseiten, Ein-/Ausstich) zum Zeichnen und Bearbeiten mit der Maus
    design_formen: befehle::DesignEingabe => Vec<stick_kern::design::ObjektForm>,
    /// Objekt in Knoten umwandeln (Formen → Punkte/Bézier, Transformation angewandt) – für das Knoten-Werkzeug
    in_knoten_umwandeln: befehle::ObjektEingabe => befehle::ObjektWert,
    /// Stiche als EXP-Datei speichern (Ergebnis: Zahl der Bytes)
    exp_speichern: befehle::ExpEingabe => usize,
    /// Objekte umordnen wie Menü „Arrange“: wie ausgewählt, an Anfang/Ende, vor/nach Nummer, nach Farbe
    reihenfolge_aendern: befehle::ReihenfolgeEingabe => befehle::ReihenfolgeErgebnis,
    /// Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken
    ueberdeckungen: befehle::DesignEingabe => Vec<[usize; 2]>,
    /// Fläche in Satinsäulen zerlegen (Vorschau der automatischen Stichrichtung)
    saeulen_vorschau: befehle::SaeulenEingabe => Vec<befehle::SaeuleInfo>,
    /// Design einfügen wie „Insert Design“: Objekte anhängen, auf Wunsch als eine Gruppe (Bild, SVG, Datei als Element)
    design_einfuegen: befehle::EinfuegenEingabe => befehle::EinfuegenErgebnis,
    /// Bild in Stickflächen umwandeln (neues Design)
    bild_umwandeln: befehle::BildEingabe => befehle::BildErgebnis,
    /// SVG-Datei einlesen (z. B. aus Inkscape): Stichart nach Breite wie EasyStitch, neues Design
    svg_lesen: befehle::SvgEingabe => befehle::SvgErgebnis,
    /// Textdatei lesen
    datei_lesen: befehle::Pfad => String,
    /// Textdatei schreiben
    text_speichern: befehle::TextDatei => (),
    /// Maschine: serielle Ports (COM…)
    maschine_ports: befehle::Leer => Vec<String>,
    /// Maschine prüfen – nur lesen: Firmware, Garnfarbtabelle; PC-Modus wird verlassen
    maschine_pruefen: maschine::Anschluss => maschine::MaschinenInfo,
    /// Senden planen – nur lesen: Prüfungen wie das Original, geplante Schreibzugriffe zum Bestätigen
    maschine_planen: maschine::SendeEingabe => maschine::SendePlanInfo,
    /// **Schreibt in die Maschine** (nur nach bestätigtem Plan): DATEN, ICON, INFO, Garnfarbtabelle; mit `wahl.sticken` Befehl 0x08 (Rahmen fährt)
    maschine_senden: maschine::SendeEingabe => maschine::SendeInfo,
    /// Laufende Maschinen-Aktion abbrechen
    maschine_abbrechen: befehle::Leer => bool,
    /// Designliste der Maschine – nur lesen
    maschine_liste: maschine::Anschluss => maschine::ListenInfo,
    /// Design aus der Maschine lesen (Befehl 0x400, nur lesen) zum Ansehen
    maschine_lesen: maschine::LeseEingabe => maschine::GelesenInfo,
    /// Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern
    gelesen_exp_speichern: maschine::ExpZiel => usize,
}

/// Befehl ausführen: Eingabe und Ausgabe als JSON (Typen: `kern/schnittstelle/befehle.schema.json`).
pub fn ausfuehren(k: &Kontext, befehl: &str, daten: Value) -> Result<Value, Fehler> {
    let b = alle()
        .into_iter()
        .find(|b| b.name == befehl)
        .ok_or_else(|| Fehler::from(format!("Befehl „{befehl}“ unbekannt")))?;
    (b.lauf)(k, daten)
}

/// Alle Befehlsfunktionen unter einem Namen (für `befehle!`).
mod befehle_aller {
    pub use crate::befehle::*;
    pub use crate::maschine::*;
}

#[cfg(test)]
mod tests;
