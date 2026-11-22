//! Schriften für die Schriftauswahl (Wunsch Nutzer 21.11.2026): installierte Computer-Schriften, Google Fonts
//! (Katalog, Herunterladen in die Ablage des Programms), Vorschau eines Probetexts, „stickbar ab … mm“
//! (`stick_kern::stickbarkeit`, eigene Erweiterung) und Schrift wechseln (Original-Alphabet ↔ Computer-Schrift).
//!
//! Das Verzeichnis der Schriften (Systemordner + Ablage) wird einmal gelesen und zwischengespeichert – das Lesen aller
//! Schriftdateien dauert Sekunden. Nach einem Download wird es neu gelesen, auf Wunsch auch mit `neu_einlesen`.

use crate::befehle::{DesignWert, ObjektWert};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use stick_formate::{google_fonts, schriftstile};
use stick_formate::ttf::{self, SchriftEintrag};
use stick_kern::format::ObjektJson;
use stick_kern::stickbarkeit::{Strichstaerken, stickbar_ab};

/// Zustand der Schriften im Kontext.
pub(crate) struct SchriftZustand {
    /// Ablage heruntergeladener Schriften (App: Datenordner/schriften)
    ablage: Mutex<PathBuf>,
    verzeichnis: Mutex<Option<Vec<SchriftEintrag>>>,
    staerken: Mutex<HashMap<PathBuf, Option<Strichstaerken>>>,
}

impl Default for SchriftZustand {
    fn default() -> Self {
        let ablage = std::env::var("STICK_SCHRIFTEN_ABLAGE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_schriften"));
        SchriftZustand {
            ablage: Mutex::new(ablage),
            verzeichnis: Mutex::new(None),
            staerken: Mutex::new(HashMap::new()),
        }
    }
}

impl Kontext {
    /// Ablage für heruntergeladene Schriften setzen (App: `<Datenordner>/schriften`).
    pub fn schriften_ablage(&self, ordner: PathBuf) {
        if let Ok(mut a) = self.schriften_zustand.ablage.lock() {
            *a = ordner;
        }
        self.verzeichnis_vergessen();
    }

    fn ablage(&self) -> PathBuf {
        self.schriften_zustand
            .ablage
            .lock()
            .map(|a| a.clone())
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_schriften"))
    }

    fn verzeichnis_vergessen(&self) {
        if let Ok(mut v) = self.schriften_zustand.verzeichnis.lock() {
            *v = None;
        }
    }

    /// Alle Computer-Schriften: Systemordner (bzw. `schriften`) und Ablage – zwischengespeichert.
    pub(crate) fn schrift_verzeichnis(&self) -> Vec<SchriftEintrag> {
        if let Ok(v) = self.schriften_zustand.verzeichnis.lock()
            && let Some(l) = v.as_ref()
        {
            return l.clone();
        }
        let mut ordner = if self.schriften.is_empty() {
            ttf::system_ordner()
        } else {
            self.schriften.clone()
        };
        let ablage = self.ablage();
        if ablage.is_dir() {
            ordner.push(ablage);
        }
        let l = ttf::verzeichnis(&ordner);
        if let Ok(mut v) = self.schriften_zustand.verzeichnis.lock() {
            *v = Some(l.clone());
        }
        l
    }

    /// Computer-Schriften eines Designs laden; fehlt eine, die es bei Google Fonts gibt, sagt der Fehler das.
    pub(crate) fn ttf_laden(&self, design: &mut stick_kern::Design) -> Result<(), Fehler> {
        let bedarf = design.ttf_bedarf();
        if bedarf.is_empty() {
            return Ok(());
        }
        let liste = self.schrift_verzeichnis();
        for name in bedarf.keys() {
            if ttf::finden(name, &liste).is_none()
                && let Some(g) = google_fonts::finden(name)
            {
                return Err(format!(
                    "Schrift „{name}“ ist auf diesem Rechner nicht vorhanden – es gibt sie kostenlos bei Google Fonts: \
                     „Schrift wählen“ → Google Fonts → „{}“ herunterladen",
                    g.familie
                )
                .into());
            }
        }
        ttf::design_schriften_laden_aus(design, &liste).map_err(Fehler::from)
    }

    fn strichstaerken(&self, e: &SchriftEintrag) -> Option<Strichstaerken> {
        if let Ok(c) = self.schriften_zustand.staerken.lock()
            && let Some(s) = c.get(&e.pfad)
        {
            return *s;
        }
        let s = ttf::strichstaerken(e);
        if let Ok(mut c) = self.schriften_zustand.staerken.lock() {
            c.insert(e.pfad.clone(), s);
        }
        s
    }
}

// ---------------------------------------------------------------- installierte Schriften

/// Computer-Schrift (TrueType/OpenType).
#[derive(Debug, Serialize, JsonSchema)]
pub struct SchriftInfo {
    /// Name wie im Design-Feld `schrift` („Arial Bold“)
    pub name: String,
    pub familie: String,
    pub stil: String,
    pub pfad: String,
    /// aus Google Fonts heruntergeladen (liegt in der Ablage des Programms)
    pub heruntergeladen: bool,
    /// Stile (Namen aus `schrift_stile`): eigene Einordnung gängiger Schriften, sonst die von Google, sonst leer
    pub stile: Vec<String>,
}

/// Eingabe von `schriften_liste`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SchriftenListeEingabe {
    /// Schriftordner neu lesen (z. B. nach dem Installieren einer Schrift im System)
    #[serde(default)]
    pub neu_einlesen: bool,
}

fn info(e: &SchriftEintrag, ablage: &Path) -> SchriftInfo {
    SchriftInfo {
        name: e.name.clone(),
        familie: e.familie.clone(),
        stil: e.stil.clone(),
        pfad: e.pfad.display().to_string(),
        heruntergeladen: e.pfad.starts_with(ablage),
        stile: schriftstile::schrift(&e.familie, &e.name),
    }
}

pub fn schriften_liste(k: &Kontext, e: SchriftenListeEingabe) -> Result<Vec<SchriftInfo>, Fehler> {
    if e.neu_einlesen {
        k.verzeichnis_vergessen();
    }
    let ablage = k.ablage();
    Ok(k.schrift_verzeichnis().iter().map(|e| info(e, &ablage)).collect())
}

// ---------------------------------------------------------------- Google Fonts

/// Eine Schrift aus dem Katalog der Google Fonts.
#[derive(Debug, Serialize, JsonSchema)]
pub struct GoogleSchriftInfo {
    /// Name der Familie – nach dem Herunterladen auch der Name für das Design-Feld `schrift`
    pub familie: String,
    /// Ohne Serifen, Mit Serifen, Zierschrift, Handschrift, Schreibmaschine
    pub kategorie: String,
    /// Stile (deutsche Namen aus `GoogleKatalog::stile`)
    pub stile: Vec<String>,
    /// Größe der Datei in Byte
    pub groesse: u64,
    pub lizenz: String,
    /// schon heruntergeladen
    pub geladen: bool,
}

/// Ein Stil der Schriftauswahl.
#[derive(Debug, Serialize, JsonSchema)]
pub struct StilInfo {
    /// „Gotisch“, „Schreibschrift“ … – so in den Feldern `stile` der Listen
    pub name: String,
    /// kurze Erklärung für den Anwender (Tooltip)
    pub beschreibung: String,
}

/// Stile der Schriftauswahl (eigene Einordnung, `Analyse_Teil3/21_schriften.md` §6).
#[derive(Debug, Serialize, JsonSchema)]
pub struct SchriftStile {
    /// Woher die Einordnung stammt
    pub quelle: String,
    /// alle Stile in Anzeigereihenfolge (Filter der Oberfläche)
    pub stile: Vec<StilInfo>,
}

pub fn schrift_stile(_: &Kontext, _: crate::befehle::Leer) -> Result<SchriftStile, Fehler> {
    let d = schriftstile::daten();
    Ok(SchriftStile {
        quelle: d.quelle.clone(),
        stile: d
            .stile
            .iter()
            .map(|s| StilInfo { name: s.name.clone(), beschreibung: s.beschreibung.clone() })
            .collect(),
    })
}

/// Katalog der Google Fonts (freie Lizenzen; nach Beliebtheit sortiert).
#[derive(Debug, Serialize, JsonSchema)]
pub struct GoogleKatalog {
    /// Stand des Katalogs (Datum)
    pub stand: String,
    /// alle Stile in Anzeigereihenfolge (Filter der Oberfläche)
    pub stile: Vec<String>,
    pub schriften: Vec<GoogleSchriftInfo>,
}

pub fn google_schriften_liste(k: &Kontext, _: crate::befehle::Leer) -> Result<GoogleKatalog, Fehler> {
    let kat = google_fonts::katalog();
    let ablage = k.ablage();
    Ok(GoogleKatalog {
        stand: kat.stand.clone(),
        stile: kat.stile.clone(),
        schriften: kat
            .schriften
            .iter()
            .map(|s| GoogleSchriftInfo {
                familie: s.familie.clone(),
                kategorie: s.kategorie.clone(),
                stile: s.stile.clone(),
                groesse: s.groesse,
                lizenz: s.lizenz.clone(),
                geladen: ablage.join(s.dateiname()).is_file(),
            })
            .collect(),
    })
}

/// Eingabe von `google_schrift_laden`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GoogleLadenEingabe {
    /// Familie wie im Katalog („Parisienne“)
    pub familie: String,
}

/// Größte Schriftdatei, die heruntergeladen wird (Byte)
const GROESSTER_DOWNLOAD: u64 = 50 * 1024 * 1024;

pub fn google_schrift_laden(k: &Kontext, e: GoogleLadenEingabe) -> Result<SchriftInfo, Fehler> {
    let g = google_fonts::finden(&e.familie)
        .ok_or_else(|| Fehler::from(format!("„{}“ gibt es im Katalog der Google Fonts nicht", e.familie)))?;
    let ablage = k.ablage();
    let ziel = ablage.join(g.dateiname());
    if !ziel.is_file() {
        std::fs::create_dir_all(&ablage).map_err(|x| Fehler::from(format!("{}: {x}", ablage.display())))?;
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(120)))
            .build()
            .into();
        let mut antwort = agent
            .get(&g.url())
            .header("User-Agent", "artista-stickprogramm")
            .call()
            .map_err(|x| Fehler::from(format!("Herunterladen von „{}“ nicht möglich (Internet?): {x}", g.familie)))?;
        let daten = antwort
            .body_mut()
            .with_config()
            .limit(GROESSTER_DOWNLOAD)
            .read_to_vec()
            .map_err(|x| Fehler::from(format!("Herunterladen von „{}“ abgebrochen: {x}", g.familie)))?;
        // muss eine lesbare Schrift sein
        ttf::schrift_aus_daten(&g.familie, &daten, 0, None, &"Aa".chars().collect())
            .map_err(|x| Fehler::from(format!("Datei von Google ist keine lesbare Schrift: {x}")))?;
        let tmp = ziel.with_extension("laden");
        std::fs::write(&tmp, &daten).map_err(|x| Fehler::from(format!("{}: {x}", tmp.display())))?;
        std::fs::rename(&tmp, &ziel).map_err(|x| Fehler::from(format!("{}: {x}", ziel.display())))?;
    }
    k.verzeichnis_vergessen();
    let liste = k.schrift_verzeichnis();
    let eintrag = liste
        .iter()
        .find(|x| x.pfad == ziel)
        .ok_or_else(|| Fehler::from(format!("{} nach dem Herunterladen nicht lesbar", ziel.display())))?;
    Ok(SchriftInfo {
        // Name für das Design: die Familie (findet Regular bzw. die Standardinstanz)
        name: g.familie.clone(),
        ..info(eintrag, &ablage)
    })
}

// ---------------------------------------------------------------- Vorschau und Stickbarkeit

/// Eingabe von `schrift_vorschau`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SchriftVorschauEingabe {
    /// Name wie im Design-Feld `schrift` (installiert oder heruntergeladen)
    pub schrift: String,
    /// Probetext (Standard „Abc“)
    #[serde(default)]
    pub text: Option<String>,
    /// Höhe in mm (Standard 10)
    #[serde(default)]
    pub hoehe: Option<f64>,
}

/// Wie gut sich eine Schrift sticken lässt (eigene Erweiterung, `stick_kern::stickbarkeit`).
#[derive(Debug, Serialize, JsonSchema)]
pub struct Stickbarkeit {
    /// kleinste Höhe der Großbuchstaben in mm, ab der die dünnen Striche mindestens 0,8 mm breit sind (kalibriert an
    /// den empfohlenen Höhen der Original-Alphabete)
    pub stickbar_ab: f64,
    /// dünne Striche in Teilen der Höhe (10 % der Strichlänge sind dünner)
    pub duenn: f64,
    /// dicke Striche in Teilen der Höhe (10 % der Strichlänge sind dicker)
    pub dick: f64,
}

impl From<Strichstaerken> for Stickbarkeit {
    fn from(s: Strichstaerken) -> Self {
        Stickbarkeit {
            stickbar_ab: stickbar_ab(&s),
            duenn: s.duenn,
            dick: s.dick,
        }
    }
}

/// Vorschau einer Computer-Schrift.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SchriftVorschau {
    /// Umrisse des Probetexts in mm (geschlossene Linien, füllen nach Nonzero-Regel)
    pub linien: Vec<stick_kern::design::FormLinie>,
    /// umschließendes Rechteck [x0, y0, x1, y1] in mm
    pub begrenzung: Option<[f64; 4]>,
    /// Zeichen des Probetexts, die die Schrift nicht hat
    pub fehlende_zeichen: String,
    /// fehlt, wenn die Schrift nicht messbar ist
    pub stickbarkeit: Option<Stickbarkeit>,
}

pub fn schrift_vorschau(k: &Kontext, e: SchriftVorschauEingabe) -> Result<SchriftVorschau, Fehler> {
    let liste = k.schrift_verzeichnis();
    let eintrag = ttf::finden(&e.schrift, &liste)
        .ok_or_else(|| Fehler::from(format!("Schrift „{}“ nicht gefunden", e.schrift)))?;
    let text = e.text.filter(|t| !t.trim().is_empty()).unwrap_or_else(|| "Abc".into());
    let design = serde_json::json!({
        "garnkarte": "keine",
        "objekte": [{"art": "ttf", "schrift": e.schrift, "text": text, "hoehe": e.hoehe.unwrap_or(10.0),
                     "stichart": "lauf"}]
    });
    let d = k.design(&crate::befehle::DesignEingabe {
        design: DesignWert(design),
        rahmen: Some("artista 400 x 150 MEGA".into()),
        stoff: None,
    })?;
    let fehlende_zeichen = d.fehlende_zeichen()?.into_iter().map(|(_, z)| z).collect();
    let f = d.formen()?.into_iter().next();
    Ok(SchriftVorschau {
        begrenzung: f
            .as_ref()
            .and_then(|f| f.begrenzung.as_ref())
            .and_then(|b| b.as_slice().try_into().ok()),
        linien: f.map(|f| f.linien).unwrap_or_default(),
        fehlende_zeichen,
        stickbarkeit: k.strichstaerken(&eintrag).map(Stickbarkeit::from),
    })
}

// ---------------------------------------------------------------- Schrift wechseln

/// Eingabe von `schrift_wechseln`: genau eins von `alphabet` und `schrift`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SchriftWechselEingabe {
    pub design: DesignWert,
    /// Nummer des Objekts (Text oder Computer-Schrift, ab 0)
    pub objekt: usize,
    /// neues Original-Alphabet → Objekt wird `text`
    #[serde(default)]
    pub alphabet: Option<String>,
    /// neue Computer-Schrift → Objekt wird `ttf`
    #[serde(default)]
    pub schrift: Option<String>,
}

/// Felder eines Schema-Typs (für „was geht mit“).
fn felder<T: JsonSchema>() -> Vec<String> {
    schemars::schema_for!(T)
        .as_value()
        .get("properties")
        .and_then(Value::as_object)
        .map(|p| p.keys().cloned().collect())
        .unwrap_or_default()
}

/// Schrift eines Texts wechseln: Original-Alphabet (`text`) ↔ Computer-Schrift (`ttf`). Text, Höhe, Unterlage, Satz
/// (Ausrichtung, Grundlinie, Abstände, Farben, Buchstaben, Verzerren) und gemeinsame Felder (Farbe, Garn, Name,
/// Transformation, Gruppe …) bleiben. Stichart: auto ↔ auto, satin ↔ satin, Step ↔ Tatami, Muster ↔ Muster; Text ohne
/// Angabe (= Satin) → satin; Computer-Schrift ohne Angabe, Lauf, Dreifach → auto.
pub fn schrift_wechseln(_: &Kontext, e: SchriftWechselEingabe) -> Result<ObjektWert, Fehler> {
    let alt = e
        .design
        .0
        .get("objekte")
        .and_then(|o| o.get(e.objekt))
        .and_then(Value::as_object)
        .cloned()
        .ok_or_else(|| Fehler::from(format!("Objekt {} gibt es nicht", e.objekt + 1)))?;
    let von = alt.get("art").and_then(Value::as_str).unwrap_or("").to_string();
    if von != "text" && von != "ttf" {
        return Err(format!("Schrift wechseln geht bei Text und Computer-Schrift, nicht bei „{von}“").into());
    }
    let (zu, feld, wert) = match (&e.alphabet, &e.schrift) {
        (Some(a), None) => ("text", "alphabet", a.clone()),
        (None, Some(s)) => ("ttf", "schrift", s.clone()),
        _ => return Err("Genau eins angeben: „alphabet“ oder „schrift“".into()),
    };
    let neu = if zu == von {
        let mut o = alt;
        o.insert(feld.into(), wert.into());
        Value::Object(o)
    } else {
        let mut o = serde_json::Map::new();
        o.insert("art".into(), zu.into());
        let mut mit = felder::<stick_kern::format::Gemeinsam>();
        mit.extend(felder::<stick_kern::format::SchriftOptionen>());
        mit.extend(["text".to_string(), "hoehe".to_string(), "unterlage".to_string()]);
        for k in mit {
            if let Some(v) = alt.get(&k) {
                o.insert(k, v.clone());
            }
        }
        let neu_art = match (zu, alt.get("stichart").and_then(Value::as_str)) {
            (_, Some(a @ ("auto" | "satin" | "muster"))) => a,
            ("ttf", Some("step")) => "tatami",
            ("text", Some("tatami")) => "step",
            // Text ohne Angabe ist Satin (Original); Computer-Schrift ohne Angabe nur Vorgabe → auto
            ("ttf", None) => "satin",
            _ => "auto",
        };
        o.insert("stichart".into(), neu_art.into());
        // Stichwinkel gilt in beiden Arten (Füllmuster steckt in den Schrift-Optionen)
        if let Some(v) = alt.get("winkel") {
            o.insert("winkel".into(), v.clone());
        }
        o.insert(feld.into(), wert.into());
        Value::Object(o)
    };
    serde_json::from_value::<ObjektJson>(neu.clone()).map_err(|x| Fehler::from(format!("Ergebnis ungültig: {x}")))?;
    Ok(ObjektWert(neu))
}
