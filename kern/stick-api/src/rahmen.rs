//! Eigene Rahmen (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“; Nachtauftrag N7b, `Analyse_Teil3/27` §5):
//! Rechteck oder Oval nach eigenen Maßen, gespeichert in `<Datenordner>/rahmen.json` (eigene Festlegung – das
//! Original schreibt `hooplibr.dsc`). Ein Design trägt seinen eigenen Rahmen im Feld `eigener_rahmen` mit.

use crate::befehle::{Leer, RahmenInfo};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Mutex;
use stick_kern::format::{EigenerRahmenJson, RahmenForm};
use stick_kern::rahmen::{Rahmen, alle_rahmen};

/// Ablage der eigenen Rahmen im Kontext.
pub(crate) struct RahmenZustand {
    datei: Mutex<PathBuf>,
}

impl Default for RahmenZustand {
    fn default() -> Self {
        let datei = std::env::var("STICK_EIGENE_RAHMEN")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_rahmen.json"));
        RahmenZustand {
            datei: Mutex::new(datei),
        }
    }
}

/// Rahmen aus der Angabe im Design bzw. in der Liste
pub fn eigener_rahmen(e: &EigenerRahmenJson) -> Result<Rahmen, String> {
    Rahmen::eigener(&e.name, e.form == RahmenForm::Oval, e.breite, e.hoehe, e.gerade)
}

impl Kontext {
    /// Datei der eigenen Rahmen setzen (App: `<Datenordner>/rahmen.json`).
    pub fn rahmen_ablage(&self, datei: PathBuf) {
        if let Ok(mut d) = self.rahmen_zustand.datei.lock() {
            *d = datei;
        }
    }

    fn rahmen_datei(&self) -> PathBuf {
        self.rahmen_zustand
            .datei
            .lock()
            .map(|d| d.clone())
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_rahmen.json"))
    }

    /// Eigene Rahmen aus der Ablage (fehlende oder kaputte Datei = keine).
    pub fn eigene_rahmen(&self) -> Vec<EigenerRahmenJson> {
        std::fs::read_to_string(self.rahmen_datei())
            .ok()
            .and_then(|t| serde_json::from_str::<Vec<EigenerRahmenJson>>(&t).ok())
            .unwrap_or_default()
    }

    fn eigene_schreiben(&self, liste: &[EigenerRahmenJson]) -> Result<(), Fehler> {
        let datei = self.rahmen_datei();
        if let Some(o) = datei.parent() {
            std::fs::create_dir_all(o).map_err(|x| Fehler::from(format!("{}: {x}", o.display())))?;
        }
        let text = serde_json::to_string_pretty(liste).map_err(|x| Fehler::from(x.to_string()))?;
        std::fs::write(&datei, text + "\n").map_err(|x| Fehler::from(format!("{}: {x}", datei.display())))
    }
}

/// Rahmen der 185 und die eigenen (diese mit Angabe `eigen`).
pub(crate) fn liste(k: &Kontext) -> Vec<RahmenInfo> {
    let mut out: Vec<RahmenInfo> = alle_rahmen().iter().map(RahmenInfo::from).collect();
    for e in k.eigene_rahmen() {
        if let Ok(r) = eigener_rahmen(&e) {
            let mut i = RahmenInfo::from(&r);
            i.eigen = Some(e);
            out.push(i);
        }
    }
    out
}

/// Eingabe von `rahmen_speichern`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RahmenSpeichernEingabe {
    pub rahmen: EigenerRahmenJson,
}

/// Eigenen Rahmen speichern (gleicher Name = ersetzen); Ergebnis: neue Rahmenliste.
pub fn rahmen_speichern(k: &Kontext, e: RahmenSpeichernEingabe) -> Result<Vec<RahmenInfo>, Fehler> {
    let mut r = e.rahmen;
    r.name = r.name.trim().to_string();
    eigener_rahmen(&r)?;
    if alle_rahmen().iter().any(|x| x.name.eq_ignore_ascii_case(&r.name)) {
        return Err(format!("„{}“ ist ein Rahmen der artista 185 – bitte anderen Namen wählen", r.name).into());
    }
    if r.form == RahmenForm::Rechteck {
        r.gerade = 0.0;
    }
    let mut liste = k.eigene_rahmen();
    liste.retain(|x| !x.name.eq_ignore_ascii_case(&r.name));
    liste.push(r);
    liste.sort_by_key(|x| x.name.to_lowercase());
    k.eigene_schreiben(&liste)?;
    Ok(self::liste(k))
}

/// Eingabe von `rahmen_loeschen`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RahmenLoeschenEingabe {
    pub name: String,
}

/// Eigenen Rahmen löschen; Ergebnis: neue Rahmenliste.
pub fn rahmen_loeschen(k: &Kontext, e: RahmenLoeschenEingabe) -> Result<Vec<RahmenInfo>, Fehler> {
    let mut l = k.eigene_rahmen();
    let n = l.len();
    l.retain(|x| x.name != e.name);
    if l.len() == n {
        return Err(format!("Eigenen Rahmen „{}“ gibt es nicht", e.name).into());
    }
    k.eigene_schreiben(&l)?;
    Ok(liste(k))
}

/// Liste aller Rahmen (Befehl `rahmen_liste`).
pub fn rahmen_liste(k: &Kontext, _: Leer) -> Result<Vec<RahmenInfo>, Fehler> {
    Ok(liste(k))
}
