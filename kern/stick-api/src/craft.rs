//! Craft Stitches (Handbuch Kap. 26, Craft-Stitch-Werkzeugleiste; `Analyse_Teil3/24_craft_stitches.md`):
//! - `craft_vorlagen`: die Werkzeuge der Leiste als fertige Objektvorlagen – Rückstich, Stielstich, Stippling,
//!   Blackwork/Candlewicking-Rand und -Füllung, Lacework (Motivsätze und Größen aus den Stilen
//!   `<PRESET_MOTIF_FILL_1…3>` und `<PRESET_MOTIF_RUN_1/2>` von NORMAL.AMT, Zuordnung über die Strings 4211–4215 ↔
//!   4221–4225 von RESENU.DLL).
//! - `blackwork_lauf`: Blackwork Run (S. 131–132) – gewählte Linien zu einem automatisch geordneten Objekt.

use crate::befehle::DesignWert;
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Eine Vorlage der Craft-Stitch-Leiste.
#[derive(Debug, Serialize, JsonSchema)]
pub struct CraftVorlage {
    /// Schlüssel (stabil), z. B. „blackwork_rand“
    pub id: String,
    /// Name für die Oberfläche
    pub name: String,
    /// Name im Original (Werkzeugleiste)
    pub original: String,
    /// „linie“ (offene oder geschlossene Linie) oder „flaeche“ (geschlossene Form)
    pub form: String,
    /// Felder des neuen Objekts ohne Form (`art` und Werte); die Oberfläche ergänzt `punkte` bzw. `umriss`
    pub felder: Value,
    /// kurze Beschreibung (Handbuch)
    pub beschreibung: String,
}

fn vorlage(id: &str, name: &str, original: &str, form: &str, felder: Value, beschreibung: &str) -> CraftVorlage {
    CraftVorlage {
        id: id.into(),
        name: name.into(),
        original: original.into(),
        form: form.into(),
        felder,
        beschreibung: beschreibung.into(),
    }
}

/// Vorlagen der Craft-Stitch-Leiste. Werte, die fehlen, nimmt der Kern aus dem Stil des Originals.
pub fn vorlagen() -> Vec<CraftVorlage> {
    vec![
        vorlage(
            "rueckstich",
            "Rückstich",
            "Backstitch",
            "linie",
            json!({"art": "rueckstich"}),
            "Feine, durchgehende Linie aus überlappenden Vor- und Rückstichen – folgt auch engen Kurven.",
        ),
        vorlage(
            "stielstich",
            "Stielstich",
            "Stemstitch",
            "linie",
            json!({"art": "stielstich"}),
            "Kordelartige Linie aus schrägen Stichen – für Stiele, Ranken und Umrisse.",
        ),
        // <PRESET_MOTIF_RUN_1> (STR 4224): Motivlinie 0x2798 _BW004a/B0001a, Größe 0x27b6/0x27b9 = 1440, Abstand 0x27df = 1440
        vorlage(
            "blackwork_rand",
            "Blackwork-Rand",
            "Blackwork Border",
            "linie",
            json!({"art": "motivlinie", "satz": "_BW004a", "motiv": "B0001a", "groesse": 8.0, "abstand": 8.0}),
            "Rand aus Blackwork-Mustern (Doppellauf-Ornamente) – traditionell schwarz auf weißem Leinen.",
        ),
        // <PRESET_MOTIF_RUN_2> (STR 4225): _CW004a/Knot-3mm, Größe 540 = 3,0 mm, Abstand 0x27df = 1261 = 7,0 mm
        vorlage(
            "candlewicking_rand",
            "Candlewicking-Rand",
            "Candlewicking Border",
            "linie",
            json!({"art": "motivlinie", "satz": "_CW004a", "motiv": "Knot-3mm", "groesse": 3.0, "abstand": 7.0}),
            "Reihe aus Knötchen wie beim Candlewicking (weiß auf weiß).",
        ),
        // <PRESET_MOTIF_FILL_1> (STR 4221): Motivfüllung 0x27ee _BW004a/B0001a, Größe 0x3ab8 = 1440, Raster 1440/1440
        vorlage(
            "blackwork_fuellung",
            "Blackwork-Füllung",
            "Blackwork Fill",
            "flaeche",
            json!({"art": "motivfuellung", "satz": "_BW004a", "motiv": "B0001a", "groesse": 8.0,
                   "spaltenabstand": 8.0, "reihenabstand": 8.0}),
            "Fläche aus wiederholten Blackwork-Mustern.",
        ),
        // <PRESET_MOTIF_FILL_2> (STR 4222): _CW004a/Knot-3mm, Größe 540, Raster 540/540
        vorlage(
            "candlewicking_fuellung",
            "Candlewicking-Füllung",
            "Candlewicking Fill",
            "flaeche",
            json!({"art": "motivfuellung", "satz": "_CW004a", "motiv": "Knot-3mm", "groesse": 3.0,
                   "spaltenabstand": 3.0, "reihenabstand": 3.0}),
            "Fläche aus Knötchen im Raster.",
        ),
        // <PRESET_MOTIF_FILL_3> (STR 4223): _LW004a/L0001a, Größe 720, Raster 720/720
        vorlage(
            "lacework",
            "Lacework",
            "Lacework",
            "flaeche",
            json!({"art": "motivfuellung", "satz": "_LW004a", "motiv": "L0001a", "groesse": 4.0,
                   "spaltenabstand": 4.0, "reihenabstand": 4.0}),
            "Offenes, gitterartiges Muster – die Fläche wirkt wie Spitze.",
        ),
        vorlage(
            "stippling",
            "Stippling",
            "Stipple",
            "flaeche",
            json!({"art": "stippling"}),
            "Fläche aus einer mäandernden Laufstichlinie, die sich nicht kreuzt.",
        ),
    ]
}

/// Vorlagen der Craft-Stitch-Leiste (Rückstich, Stielstich, Blackwork, Candlewicking, Lacework, Stippling).
pub fn craft_vorlagen(_: &Kontext, _: crate::befehle::Leer) -> Result<Vec<CraftVorlage>, Fehler> {
    Ok(vorlagen())
}

/// Eingabe für `blackwork_lauf`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlackworkEingabe {
    pub design: DesignWert,
    /// Nummern der Objekte (ab 0); das erste geeignete gibt Stichart, Farbe und Effekte vor
    pub objekte: Vec<usize>,
}

/// Ergebnis von `blackwork_lauf`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BlackworkErgebnis {
    /// geändertes Design
    pub design: DesignWert,
    /// Nummer des neuen Blackwork-Objekts
    pub objekt: usize,
    /// Hinweise (z. B. ausgelassene Objekte)
    pub hinweise: Vec<String>,
}

/// Linienarten, die Blackwork Run verbinden kann (Handbuch: Einfach-/Dreifachlauf, Stielstich, Rückstich; „Outlines
/// with Pattern Run stitch types cannot be used“) – auch vorhandene Blackwork-Objekte.
const GEEIGNET: [&str; 5] = ["lauf", "dreifach", "rueckstich", "stielstich", "blackwork"];

/// Blackwork Run: gewählte Linien zu einem automatisch geordneten Objekt (`art: blackwork`) zusammenfassen.
pub fn blackwork_lauf(_: &Kontext, e: BlackworkEingabe) -> Result<BlackworkErgebnis, Fehler> {
    let design_json = e.design.0;
    let design = stick_kern::Design::aus_value(&design_json).map_err(|x| Fehler::from(format!("Design: {x}")))?;
    let objekte: Vec<Value> = design_json.get("objekte").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut nummern = e.objekte.clone();
    nummern.dedup();
    let mut hinweise = Vec::new();
    let mut linien: Vec<Value> = Vec::new();
    let mut genommen: Vec<usize> = Vec::new();
    let mut erstes: Option<Value> = None;
    for &nr in &nummern {
        let o = objekte
            .get(nr)
            .ok_or_else(|| Fehler::from(format!("Objekt {} gibt es nicht", nr + 1)))?;
        let art = o.get("art").and_then(Value::as_str).unwrap_or("");
        if !GEEIGNET.contains(&art) {
            hinweise.push(format!("Objekt {} ({art}) ausgelassen – nur Lauf, Dreifach, Rückstich und Stielstich", nr + 1));
            continue;
        }
        // Formen und Transformation in Punkte umwandeln (Linien in Designkoordinaten)
        let o = if stick_kern::umwandeln::umwandelbar(o) {
            stick_kern::umwandeln::in_punkte(&design, &design_json, nr).map_err(Fehler::from)?
        } else {
            o.clone()
        };
        if art == "blackwork" {
            linien.extend(o.get("linien").and_then(Value::as_array).cloned().unwrap_or_default());
        } else {
            let geo = o.get("punkte").cloned().ok_or("Objekt ohne Form")?;
            let geschlossen = o.get("geschlossen").and_then(Value::as_bool).unwrap_or(false);
            if geschlossen {
                let angabe: stick_kern::format::PunktAngabe =
                    serde_json::from_value(geo.clone()).map_err(|x| Fehler::from(format!("Form ungültig: {x}")))?;
                let p0 = *angabe.punkte().first().ok_or("Form ohne Punkte")?;
                linien.push(json!({"verbinden": [geo, [[p0.x, p0.y]]]}));
            } else {
                linien.push(geo);
            }
        }
        genommen.push(nr);
        if erstes.is_none() {
            erstes = Some(o);
        }
    }
    if genommen.len() < 2 {
        return Err("Blackwork Run braucht mindestens zwei Linien (Lauf, Dreifach, Rückstich oder Stielstich)".into());
    }
    let erstes = erstes.unwrap();
    let eo = erstes.as_object().unwrap();
    let art = eo.get("art").and_then(Value::as_str).unwrap_or("lauf");
    let mut neu = serde_json::Map::new();
    neu.insert("art".into(), "blackwork".into());
    neu.insert("linien".into(), linien.into());
    // Stichart des ersten Objekts mit seinen Werten
    let stich = match art {
        "blackwork" => eo.get("stich").and_then(Value::as_str).unwrap_or("lauf").to_string(),
        "lauf" if eo.get("dreifach").and_then(Value::as_bool) == Some(true) => "dreifach".into(),
        a => a.to_string(),
    };
    neu.insert("stich".into(), stich.into());
    let werte = ["laenge", "staerke", "ueberlappung", "striche", "breite", "abstand", "winkel"];
    for k in werte {
        if let Some(v) = eo.get(k) {
            neu.insert(k.into(), v.clone());
        }
    }
    if art == "stielstich" || art == "blackwork" {
        if let Some(v) = eo.get("dreifach") {
            neu.insert("dreifach".into(), v.clone());
        }
    }
    // gemeinsame Felder des ersten Objekts (Farbe, Garn, Name, Effekte, Gruppe …), ohne Transformation (schon
    // angewandt) und ohne Ausstich (der Zug endet anders)
    let schema = schemars::schema_for!(stick_kern::format::Gemeinsam);
    if let Some(p) = schema.as_value().get("properties").and_then(Value::as_object) {
        for k in p.keys() {
            if k == "transform" || k == "ausstich" {
                continue;
            }
            if let Some(v) = eo.get(k) {
                neu.insert(k.clone(), v.clone());
            }
        }
    }
    if !neu.contains_key("name") {
        neu.insert("name".into(), "Blackwork".into());
    }
    let ziel = genommen[0];
    let mut liste: Vec<Value> = Vec::new();
    let mut neue_nr = 0;
    for (i, o) in objekte.into_iter().enumerate() {
        if i == ziel {
            neue_nr = liste.len();
            liste.push(Value::Object(neu.clone()));
        } else if !genommen.contains(&i) {
            liste.push(o);
        }
    }
    let mut d = design_json.clone();
    d["objekte"] = liste.into();
    stick_kern::Design::aus_value(&d).map_err(|x| Fehler::from(format!("Ergebnis: {x}")))?;
    Ok(BlackworkErgebnis {
        design: DesignWert(d),
        objekt: neue_nr,
        hinweise,
    })
}
