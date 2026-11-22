//! **Vorlagen** (Handbuch Kap. 9 „Object Properties and Templates“, S. 66–70; Nachtauftrag N8, `Analyse_Teil3/28`):
//! Das Original speichert Designs als Vorlage (AMT) im Ordner `artista\template`; „Neu“ beginnt mit einer Kopie der
//! Vorlage NORMAL, deren Werte („default properties“) die aktuellen Werte des neuen Designs werden. „Save“ im Dialog
//! Object Properties schreibt nur die Werte (nicht die Objekte) in die Vorlage, aus der das Design entstand; „Revert“
//! stellt NORMAL wieder her.
//!
//! Hier: eine Vorlage = Design-JSON in `<Datenordner>/vorlagen/<Name>.json` (eigene Festlegung statt AMT). Die Werte
//! stehen wie im Design unter `einstellungen` (Parameterkatalog), dazu Rahmen, Stoff, Garnkarte und auf Wunsch Objekte
//! (z. B. Mustertext). „Normal“ ist die Standardvorlage für „Neu“; fehlt die Datei, gilt die Werkseinstellung.

use crate::befehle::DesignWert;
use crate::motive::dateiname;
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::Mutex;

/// Name der Standardvorlage (Original: NORMAL.AMT)
pub const NORMAL: &str = "Normal";

/// Ablage der Vorlagen im Kontext.
pub(crate) struct VorlagenZustand {
    ordner: Mutex<PathBuf>,
}

impl Default for VorlagenZustand {
    fn default() -> Self {
        let ordner = std::env::var("STICK_VORLAGEN")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_vorlagen"));
        VorlagenZustand {
            ordner: Mutex::new(ordner),
        }
    }
}

impl Kontext {
    /// Ordner der Vorlagen setzen (App: `<Datenordner>/vorlagen`).
    pub fn vorlagen_ablage(&self, ordner: PathBuf) {
        if let Ok(mut o) = self.vorlagen_zustand.ordner.lock() {
            *o = ordner;
        }
    }

    fn vorlagen_ordner(&self) -> PathBuf {
        self.vorlagen_zustand
            .ordner
            .lock()
            .map(|o| o.clone())
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_vorlagen"))
    }

    /// Gespeicherte Vorlagen (Name, Datei, Inhalt), nach Namen sortiert; Normal zuerst.
    fn vorlagen(&self) -> Vec<(String, PathBuf, Value)> {
        let mut out: Vec<(String, PathBuf, Value)> = std::fs::read_dir(self.vorlagen_ordner())
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .filter_map(|e| {
                let v: Value = serde_json::from_str(&std::fs::read_to_string(e.path()).ok()?).ok()?;
                let name = v
                    .get("vorlage")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| e.path().file_stem().unwrap_or_default().to_string_lossy().into_owned());
                Some((name, e.path(), v))
            })
            .collect();
        out.sort_by_key(|(n, _, _)| (!n.eq_ignore_ascii_case(NORMAL), n.to_lowercase()));
        out
    }

    fn vorlage_finden(&self, name: &str) -> Option<(String, PathBuf, Value)> {
        self.vorlagen().into_iter().find(|(n, _, _)| n.eq_ignore_ascii_case(name.trim()))
    }
}

/// Werkseinstellung der Vorlage Normal: leeres Design mit den Standardwerten des Formats.
fn werk() -> Value {
    json!({"name": "Design", "vorlage": NORMAL})
}

/// Zahl der einzelnen Werte unter `einstellungen` (Blätter des Baums).
fn werte_zahl(v: Option<&Value>) -> usize {
    match v {
        Some(Value::Object(m)) => m.values().map(|x| werte_zahl(Some(x))).sum(),
        Some(Value::Null) | None => 0,
        Some(_) => 1,
    }
}

/// Eintrag der Vorlagenliste.
#[derive(Debug, Serialize, JsonSchema)]
pub struct VorlageInfo {
    pub name: String,
    /// Standardvorlage für „Neu“
    pub standard: bool,
    /// Werkseinstellung (keine Datei – nur bei Normal)
    pub werk: bool,
    pub rahmen: Option<String>,
    pub stoff: Option<String>,
    pub garnkarte: Option<String>,
    /// Zahl der Objekte (z. B. Mustertext)
    pub objekte: usize,
    /// Zahl der gespeicherten Einzelwerte (`einstellungen`)
    pub werte: usize,
}

fn info(name: &str, v: &Value, werk: bool) -> VorlageInfo {
    let s = |k: &str| v.get(k).and_then(Value::as_str).map(str::to_string);
    VorlageInfo {
        name: name.to_string(),
        standard: name.eq_ignore_ascii_case(NORMAL),
        werk,
        rahmen: s("rahmen"),
        stoff: s("stoff"),
        garnkarte: s("garnkarte"),
        objekte: v.get("objekte").and_then(Value::as_array).map_or(0, Vec::len),
        werte: werte_zahl(v.get("einstellungen")),
    }
}

/// Alle Vorlagen; Normal steht immer vorn (ohne Datei als Werkseinstellung).
pub fn vorlagen_liste(k: &Kontext, _: crate::befehle::Leer) -> Result<Vec<VorlageInfo>, Fehler> {
    let mut out: Vec<VorlageInfo> = k.vorlagen().iter().map(|(n, _, v)| info(n, v, false)).collect();
    if !out.iter().any(|i| i.standard) {
        out.insert(0, info(NORMAL, &werk(), true));
    }
    Ok(out)
}

/// Was in die Vorlage kommt
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum VorlageUmfang {
    /// das ganze Design: Werte, Rahmen, Stoff, Garnkarte und Objekte („Save As … Templates (AMT)“)
    Ganz,
    /// nur die Werte (`einstellungen`) in eine vorhandene Vorlage – Objekte und Rest der Vorlage bleiben („Save“ im
    /// Dialog Object Properties: „Only the current object properties … are saved to the template“)
    Werte,
}

fn umfang_ganz() -> VorlageUmfang {
    VorlageUmfang::Ganz
}

/// Eingabe von `vorlage_speichern`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VorlageSpeichernEingabe {
    pub design: DesignWert,
    /// Name der Vorlage („Normal“ = Standardvorlage für „Neu“)
    pub name: String,
    #[serde(default = "umfang_ganz")]
    pub umfang: VorlageUmfang,
    /// bei `ganz`: Objekte mitspeichern (z. B. Mustertext); ohne = nur Werte, Rahmen, Stoff, Garnkarte
    #[serde(default = "ja")]
    pub objekte: bool,
}

fn ja() -> bool {
    true
}

/// Vorlage speichern (gleicher Name ersetzt); Ergebnis: neue Liste.
pub fn vorlage_speichern(k: &Kontext, e: VorlageSpeichernEingabe) -> Result<Vec<VorlageInfo>, Fehler> {
    let name = e.name.trim().to_string();
    if name.is_empty() || name.chars().all(|c| c == '.' || c == '_') {
        return Err("Name der Vorlage fehlt".into());
    }
    let d = e.design.0.as_object().ok_or("Das Design muss ein JSON-Objekt sein")?;
    // nur gültige Designs ablegen
    stick_kern::Design::aus_value(&e.design.0).map_err(|x| Fehler::from(format!("Vorlage nicht gespeichert: {x}")))?;
    let vorhanden = k.vorlage_finden(&name);
    let mut v = match e.umfang {
        VorlageUmfang::Ganz => {
            let mut v = d.clone();
            if !e.objekte {
                for f in ["objekte", "gruppen", "start", "ende", "transform"] {
                    v.remove(f);
                }
            }
            Value::Object(v)
        }
        VorlageUmfang::Werte => {
            let mut v = vorhanden.as_ref().map(|(_, _, v)| v.clone()).unwrap_or_else(|| {
                if name.eq_ignore_ascii_case(NORMAL) { werk() } else { json!({}) }
            });
            let o = v.as_object_mut().ok_or("Vorlage ist kein JSON-Objekt")?;
            match d.get("einstellungen") {
                Some(w) if !w.is_null() => {
                    o.insert("einstellungen".into(), w.clone());
                }
                _ => {
                    o.remove("einstellungen");
                }
            }
            v
        }
    };
    let o = v.as_object_mut().ok_or("Vorlage ist kein JSON-Objekt")?;
    o.insert("vorlage".into(), Value::String(name.clone()));
    o.insert("name".into(), Value::String("Design".into()));
    let ordner = k.vorlagen_ordner();
    std::fs::create_dir_all(&ordner).map_err(|x| Fehler::from(format!("{}: {x}", ordner.display())))?;
    let ziel = vorhanden.map(|(_, p, _)| p).unwrap_or_else(|| ordner.join(dateiname(&name)));
    let text = serde_json::to_string_pretty(&v).map_err(|x| Fehler::from(x.to_string()))?;
    std::fs::write(&ziel, text + "\n").map_err(|x| Fehler::from(format!("{}: {x}", ziel.display())))?;
    vorlagen_liste(k, crate::befehle::Leer {})
}

/// Eingabe von `vorlage_neu`/`vorlage_loeschen`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VorlageName {
    pub name: String,
}

/// Neues Design aus einer Vorlage (Kopie; Name „Design“, Feld `vorlage` = Name der Vorlage). Normal ohne Datei =
/// Werkseinstellung (nur `name` und `vorlage` – alles andere Standard des Formats).
pub fn vorlage_neu(k: &Kontext, e: VorlageName) -> Result<DesignWert, Fehler> {
    let mut v = match k.vorlage_finden(&e.name) {
        Some((_, _, v)) => v,
        None if e.name.trim().eq_ignore_ascii_case(NORMAL) => werk(),
        None => return Err(format!("Vorlage „{}“ gibt es nicht", e.name).into()),
    };
    if let Some(o) = v.as_object_mut() {
        o.insert("name".into(), Value::String("Design".into()));
    }
    Ok(DesignWert(v))
}

/// Vorlage löschen. Normal löschen = zurück zur Werkseinstellung („Revert to Factory Template“). Ergebnis: neue Liste.
pub fn vorlage_loeschen(k: &Kontext, e: VorlageName) -> Result<Vec<VorlageInfo>, Fehler> {
    let (_, pfad, _) = k
        .vorlage_finden(&e.name)
        .ok_or_else(|| Fehler::from(format!("Vorlage „{}“ gibt es nicht als Datei", e.name)))?;
    std::fs::remove_file(&pfad).map_err(|x| Fehler::from(format!("{}: {x}", pfad.display())))?;
    vorlagen_liste(k, crate::befehle::Leer {})
}
