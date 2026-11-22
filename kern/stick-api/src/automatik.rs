//! Automatisches Digitalisieren (Handbuch Kap. 13; `Analyse_Teil3/25_automatik.md`):
//! - `zauberstab`: „Magic Wand“ (S. 114–115) – Flächen im vorbereiteten Bild anklicken; jede angeklickte Fläche wird
//!   ein Objekt, gefüllt (Step/Satin/Auto) oder als Umriss (Lauf/Dreifach/Satinlinie), in der Reihenfolge der Klicks.
//! - `photosnap`: „PhotoSnap“ (S. 119–120) – Foto als Reihen von Zickzackstichen mit wechselndem Abstand
//!   (`stick-kern::photosnap`); der Befehl bereitet das Raster vor.

use crate::befehle::{BildEingabe, DesignWert, bild_optionen, hex};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;

/// Stichart einer angeklickten Fläche (wie die gewählte Stichart vor dem Klick im Original: Füllstich → Fläche,
/// Konturstich → Umriss).
#[derive(Debug, Clone, Copy, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ZauberStich {
    /// Fläche mit Step (Tatami)
    Step,
    /// Fläche mit Satin (automatische Stichrichtung, `satinflaeche`)
    Satin,
    /// Fläche nach der Dicke wie EasyStitch: bis 8 mm Satin, darüber Step
    Auto,
    /// Umriss im Laufstich
    Lauf,
    /// Umriss im Dreifachstich
    Dreifach,
    /// Umriss als Satinlinie (Breite 2 mm wie `satinlinie`)
    Satinlinie,
}

/// Ein Klick des Zauberstabs.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ZauberKlick {
    /// Lage im Vorschaubild von `bild_vorschau`, 0 … 1 von links
    pub x: f64,
    /// Lage im Vorschaubild, 0 … 1 von oben
    pub y: f64,
    pub stichart: ZauberStich,
}

/// Eingabe von `zauberstab`: das Bild wie bei `bild_vorschau` und die Klicks.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ZauberstabEingabe {
    pub bild: BildEingabe,
    pub klicks: Vec<ZauberKlick>,
}

/// Ergebnis von `zauberstab`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ZauberstabErgebnis {
    /// neues Design mit einem Objekt je getroffener Fläche (Koordinaten wie bei `bild_umwandeln`: Bildmitte = 0)
    pub design: DesignWert,
    /// Zahl der Objekte
    pub objekte: usize,
    /// Klicks ohne Fläche (durchsichtig oder zu klein)
    pub hinweise: Vec<String>,
}

pub fn zauberstab(k: &Kontext, e: ZauberstabEingabe) -> Result<ZauberstabErgebnis, Fehler> {
    use stick_vektor::easystitch::{LAUF_LAENGE_MM, SATIN_BIS_MM, ZUGAUSGLEICH_MM, punkte_json};
    if e.klicks.is_empty() {
        return Err("Keine Fläche angeklickt".into());
    }
    let (o, karte) = bild_optionen(k, &e.bild)?;
    let bild = stick_vektor::bild_laden(&PathBuf::from(&e.bild.pfad))?;
    let v = stick_vektor::vorbereiten(&bild, &o);
    let karte = karte.filter(|_| o.palette.is_some());
    let mut objekte: Vec<Value> = Vec::new();
    let mut hinweise = Vec::new();
    for (n, kl) in e.klicks.iter().enumerate() {
        let px = (kl.x.clamp(0.0, 1.0) * v.breite as f64).min(v.breite as f64 - 1.0) as usize;
        let py = (kl.y.clamp(0.0, 1.0) * v.hoehe as f64).min(v.hoehe as f64 - 1.0) as usize;
        let Some(f) = stick_vektor::bereich_bei(&v, &o, px, py) else {
            hinweise.push(format!("Klick {}: keine Fläche (durchsichtig oder kleiner als 1 mm²)", n + 1));
            continue;
        };
        let farbe = hex(f.farbe);
        let nr = objekte.len() + 1;
        let mut neu: Vec<Value> = match kl.stichart {
            ZauberStich::Step | ZauberStich::Satin | ZauberStich::Auto => {
                let satin = match kl.stichart {
                    ZauberStich::Satin => true,
                    ZauberStich::Auto => {
                        stick_kern::saeulen::dicke(&f.umriss, &f.loecher).is_ok_and(|d| d <= SATIN_BIS_MM)
                    }
                    _ => false,
                };
                vec![stick_vektor::flaeche_objekt(&f, nr, satin)]
            }
            art => std::iter::once(&f.umriss)
                .chain(&f.loecher)
                .enumerate()
                .map(|(i, r)| {
                    let name = if i == 0 { format!("Umriss {nr}") } else { format!("Umriss {nr}.{i}") };
                    match art {
                        ZauberStich::Satinlinie => {
                            let mut l = r.clone();
                            l.push(l[0]);
                            json!({"art": "satinlinie", "name": name, "farbe": farbe, "punkte": punkte_json(&l),
                                   "einstellungen": {"satin": {"zugausgleich": ZUGAUSGLEICH_MM}}})
                        }
                        _ => json!({"art": if art == ZauberStich::Dreifach { "dreifach" } else { "lauf" },
                                    "name": name, "farbe": farbe, "punkte": punkte_json(r), "geschlossen": true,
                                    "laenge": LAUF_LAENGE_MM}),
                    }
                })
                .collect(),
        };
        if let Some(kt) = &karte
            && let Some(g) = kt.garne.iter().find(|g| g.rgb == f.farbe)
        {
            for ob in &mut neu {
                ob["garn"] = Value::from(g.nummer.clone());
            }
        }
        objekte.extend(neu);
    }
    if objekte.is_empty() {
        return Err(hinweise.join(" · ").into());
    }
    let stamm = PathBuf::from(&e.bild.pfad).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let rahmen = e.bild.rahmen.clone().unwrap_or_else(|| "artista 130 x 100".into());
    Ok(ZauberstabErgebnis {
        objekte: objekte.len(),
        design: DesignWert(json!({"name": stamm, "rahmen": rahmen, "kuerzeste_verbindung": true, "objekte": objekte})),
        hinweise,
    })
}

/// Eingabe von `photosnap`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PhotosnapEingabe {
    /// Bilddatei (PNG, JPG, BMP …)
    pub pfad: String,
    /// Breite in mm (Standard 80)
    #[serde(default)]
    pub breite: Option<f64>,
    /// farbig in vier Lagen (Cyan, Magenta, Gelb, Schwarz) statt einfarbig (Standard einfarbig wie im Original)
    #[serde(default)]
    pub farbig: bool,
    /// Garnkarte: die Lagen bekommen die nächsten Garne (CIEDE2000); ohne Angabe die Druckfarben
    #[serde(default)]
    pub garnkarte: Option<String>,
    #[serde(default)]
    pub aufloesung: Option<stick_kern::format::PhotoAufloesung>,
    #[serde(default)]
    pub hintergrund: Option<stick_kern::format::PhotoHintergrund>,
    /// Winkel der Reihen in Grad (Standard 0)
    #[serde(default)]
    pub winkel: Option<f64>,
    /// Rahmen des neuen Designs
    #[serde(default)]
    pub rahmen: Option<String>,
}

/// Ergebnis von `photosnap`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct PhotosnapErgebnis {
    /// neues Design mit einem Objekt `photosnap`
    pub design: DesignWert,
    /// Zahl der Farblagen (1 oder 4)
    pub lagen: usize,
    /// Größe in mm
    pub breite: f64,
    pub hoehe: f64,
}

/// Größe eines Rasterpunkts in mm (eigene Festlegung: halbe Reihenbreite „fein“)
pub const PHOTO_MM_PX: f64 = 0.5;

pub fn photosnap(k: &Kontext, e: PhotosnapEingabe) -> Result<PhotosnapErgebnis, Fehler> {
    use stick_kern::format::PhotoHintergrund;
    let breite = e.breite.unwrap_or(80.0);
    if !(5.0..=400.0).contains(&breite) {
        return Err("breite: 5 bis 400 mm".into());
    }
    let bild = stick_vektor::bild_laden(&PathBuf::from(&e.pfad))?;
    let (w, h, lagen) = stick_vektor::photo_raster(&bild, breite, PHOTO_MM_PX, e.farbig);
    let karte = match e.garnkarte.as_deref() {
        None | Some("") | Some("keine") => None,
        Some(n) => Some(stick_formate::garnkarte::laden(&k.garnkarten, n)?),
    };
    let dunkel = e.hintergrund == Some(PhotoHintergrund::Dunkel);
    // Druckfarben (eigene Festlegung; die Lagenfarben des Originals werden zur Laufzeit gesetzt, nicht gelesen)
    let druck: Vec<[u8; 3]> = if e.farbig {
        vec![[0, 174, 239], [236, 0, 140], [255, 242, 0], [0, 0, 0]]
    } else if dunkel {
        vec![[255, 255, 255]]
    } else {
        vec![[0, 0, 0]]
    };
    let farben: Vec<[u8; 3]> = druck
        .iter()
        .map(|c| karte.as_ref().map(|kt| kt.naechstes(*c).rgb).unwrap_or(*c))
        .collect();
    let lagen_json: Vec<Value> = lagen
        .iter()
        .zip(&farben)
        .map(|(l, c)| json!({"farbe": hex(*c), "werte": crate::befehle::base64(l)}))
        .collect();
    let mut o = json!({
        "art": "photosnap", "name": "PhotoSnap", "farbe": hex(farben[0]),
        "raster": {"breite": w, "hoehe": h, "mm": PHOTO_MM_PX, "lagen": lagen_json},
    });
    for (feld, wert) in [
        ("aufloesung", serde_json::to_value(e.aufloesung).ok()),
        ("hintergrund", serde_json::to_value(e.hintergrund).ok()),
        ("winkel", e.winkel.map(Value::from)),
    ] {
        if let Some(v) = wert.filter(|v| !v.is_null()) {
            o[feld] = v;
        }
    }
    let stamm = PathBuf::from(&e.pfad).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let rahmen = e.rahmen.clone().unwrap_or_else(|| "artista 130 x 100".into());
    Ok(PhotosnapErgebnis {
        design: DesignWert(json!({"name": stamm, "rahmen": rahmen, "objekte": [o]})),
        lagen: lagen.len(),
        breite: w as f64 * PHOTO_MM_PX,
        hoehe: h as f64 * PHOTO_MM_PX,
    })
}
