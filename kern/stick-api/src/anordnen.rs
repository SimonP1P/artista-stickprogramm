//! Anordnen (Handbuch Kap. 15; `Analyse_Teil3/26_anordnen.md`): Ausrichten, Mirror-Merge, Kranz mit Kaleidoskop –
//! Rechnung in `stick_kern::anordnen`.

use crate::befehle::{DesignEingabe, DesignWert};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stick_kern::Design;
use stick_kern::anordnen::{self as an, AusrichtArt};
use stick_kern::geometrie::Punkt;
use stick_kern::transformation::Spiegeln;

fn bereiter(k: &Kontext) -> impl Fn(&Value) -> Result<Design, String> + '_ {
    move |v: &Value| {
        k.design(&DesignEingabe {
            design: DesignWert(v.clone()),
            rahmen: None,
            stoff: None,
        })
        .map_err(|e| e.meldung)
    }
}

/// Eingabe von `ausrichten`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AusrichtenEingabe {
    pub design: DesignWert,
    /// gewählte Objekte in Klickfolge; das **zuletzt** gewählte ist der Bezug (wie im Original). Gruppen zählen als
    /// Ganzes
    pub objekte: Vec<usize>,
    pub art: AusrichtArt,
}

/// Ergebnis von `ausrichten`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct AusrichtenErgebnis {
    /// geändertes Design (Objekte bzw. Gruppen verschoben)
    pub design: DesignWert,
}

pub fn ausrichten(k: &Kontext, e: AusrichtenEingabe) -> Result<AusrichtenErgebnis, Fehler> {
    let d = an::ausrichten(&e.design.0, &e.objekte, e.art, &bereiter(k))?;
    Ok(AusrichtenErgebnis { design: DesignWert(d) })
}

/// Ergebnis von `spiegeln_kopieren` und `kranz`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct KopienErgebnis {
    /// geändertes Design mit den Kopien (direkt hinter der Auswahl)
    pub design: DesignWert,
    /// Nummern der neuen Objekte (für die Auswahl danach)
    pub neu: Vec<usize>,
    /// eine Kopie überlappt eine Fläche – die Oberfläche fragt wie das Original „Überlappende Objekte
    /// zusammenführen?“
    pub ueberlappt: bool,
    /// Zahl der zusammengeführten Flächen
    pub zusammengefuehrt: usize,
}

impl From<an::Kopien> for KopienErgebnis {
    fn from(k: an::Kopien) -> Self {
        KopienErgebnis {
            design: DesignWert(k.design),
            neu: k.neu,
            ueberlappt: k.ueberlappt,
            zusammengefuehrt: k.zusammengefuehrt,
        }
    }
}

/// Eingabe von `spiegeln_kopieren` (Mirror-Merge).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpiegelnKopierenEingabe {
    pub design: DesignWert,
    /// gewählte Objekte (Gruppen als Ganzes)
    pub objekte: Vec<usize>,
    /// `links_rechts` = an einer senkrechten Linie (Mirror-Merge Horizontal), `oben_unten` = an einer waagrechten
    /// (Vertical), `beide` = drei Kopien (Horizontal & Vertical)
    pub art: Spiegeln,
    /// Ankerpunkt [x, y] in mm (Anzeige): die Spiegellinie(n) gehen durch ihn
    pub achse: Punkt,
    /// überlappende Flächen (Step, Muster, Stippling, Satinfläche) mit dem Original zu einem Objekt vereinigen
    #[serde(default)]
    pub zusammenfuehren: bool,
}

pub fn spiegeln_kopieren(k: &Kontext, e: SpiegelnKopierenEingabe) -> Result<KopienErgebnis, Fehler> {
    Ok(an::spiegeln_kopieren(&e.design.0, &e.objekte, e.art, e.achse, e.zusammenfuehren, &bereiter(k))?.into())
}

fn sechs() -> usize {
    6
}

/// Eingabe von `kranz` (Wreath).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KranzEingabe {
    pub design: DesignWert,
    /// gewählte Objekte (Gruppen als Ganzes)
    pub objekte: Vec<usize>,
    /// Punkte des Kranzes (Original + Kopien), 2 bis 36; Standard 6 wie im Handbuch-Beispiel
    #[serde(default = "sechs")]
    pub punkte: usize,
    /// Kaleidoskop: jede zweite Kopie gespiegelt (nur gerade Punktzahl)
    #[serde(default)]
    pub kaleidoskop: bool,
    /// Mittelpunkt [x, y] in mm (Anzeige)
    pub mitte: Punkt,
    /// überlappende Flächen vereinigen (wie `spiegeln_kopieren`)
    #[serde(default)]
    pub zusammenfuehren: bool,
}

pub fn kranz(k: &Kontext, e: KranzEingabe) -> Result<KopienErgebnis, Fehler> {
    Ok(an::kranz(&e.design.0, &e.objekte, e.punkte, e.kaleidoskop, e.mitte, e.zusammenfuehren, &bereiter(k))?.into())
}
