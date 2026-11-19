//! Programm „Artista Stickprogramm“: Befehle der Oberfläche an den Rechenkern (`kern/`).

use serde::Serialize;
use serde_json::Value;
use stick_formate::exp::exp_bytes;
use stick_kern::einstellungen::STOFFE;
use stick_kern::rahmen::{Rahmen, alle_rahmen};
use stick_kern::{Art, Design, Stich};

/// Beispieldesign mit allen Sticharten der ersten Ausbaustufe.
const BEISPIEL: &str = include_str!("../../../stickprogramm/beispiele/beispiel.json");

#[derive(Serialize)]
struct RahmenInfo {
    name: String,
    breite: f64,
    hoehe: f64,
    form: Vec<[f64; 2]>,
    hinweis: String,
}

impl From<&Rahmen> for RahmenInfo {
    fn from(r: &Rahmen) -> Self {
        RahmenInfo {
            name: r.name.clone(),
            breite: r.breite,
            hoehe: r.hoehe,
            form: r.form.iter().map(|&q| q.into()).collect(),
            hinweis: r.hinweis.clone(),
        }
    }
}

#[derive(Serialize)]
struct Statistik {
    stiche: usize,
    spruenge: usize,
    farbwechsel: usize,
    breite: f64,
    hoehe: f64,
    min_stich: f64,
    max_stich: f64,
}

#[derive(Serialize)]
struct Ergebnis {
    name: String,
    rahmen: RahmenInfo,
    stoff: String,
    stiche: Vec<Stich>,
    farben: Vec<String>,
    statistik: Statistik,
    ausserhalb: usize,
}

/// Design aus JSON; Rahmen und Stoff aus der Oberfläche haben Vorrang vor den Angaben im JSON.
fn design_laden(json: &str, rahmen: Option<String>, stoff: Option<String>) -> Result<Design, String> {
    let mut v: Value = serde_json::from_str(json).map_err(|e| format!("JSON ungültig: {e}"))?;
    let Some(obj) = v.as_object_mut() else {
        return Err("Das Design muss ein JSON-Objekt sein".into());
    };
    if let Some(r) = rahmen {
        obj.insert("rahmen".into(), r.into());
    }
    if let Some(s) = stoff {
        obj.insert("stoff".into(), s.into());
    }
    Design::aus_value(&v)
}

#[tauri::command]
fn rahmen_liste() -> Vec<RahmenInfo> {
    alle_rahmen().iter().map(RahmenInfo::from).collect()
}

#[tauri::command]
fn stoffe_liste() -> Vec<String> {
    STOFFE.iter().map(|s| s.name.to_string()).collect()
}

#[tauri::command]
fn beispiel_design() -> String {
    BEISPIEL.to_string()
}

#[tauri::command]
async fn design_berechnen(json: String, rahmen: Option<String>, stoff: Option<String>) -> Result<Ergebnis, String> {
    let design = design_laden(&json, rahmen, stoff)?;
    let folge = design.berechnen()?;
    let (x0, y0, x1, y1) = folge.begrenzung().unwrap_or((0.0, 0.0, 0.0, 0.0));
    let laengen = folge.stichlaengen();
    let statistik = Statistik {
        stiche: folge.anzahl(Art::Stich),
        spruenge: folge.anzahl(Art::Sprung),
        farbwechsel: folge.anzahl(Art::Farbwechsel),
        breite: x1 - x0,
        hoehe: y1 - y0,
        min_stich: if laengen.is_empty() { 0.0 } else { laengen.iter().copied().fold(f64::INFINITY, f64::min) },
        max_stich: laengen.iter().copied().fold(0.0, f64::max),
    };
    Ok(Ergebnis {
        name: design.name.clone(),
        rahmen: RahmenInfo::from(&design.rahmen),
        stoff: design.einstellungen.stoff.clone(),
        ausserhalb: design.ausserhalb(&folge).len(),
        statistik,
        farben: folge.farben,
        stiche: folge.stiche,
    })
}

#[tauri::command]
async fn datei_lesen(pfad: String) -> Result<String, String> {
    std::fs::read_to_string(&pfad).map_err(|e| format!("{pfad}: {e}"))
}

#[tauri::command]
async fn text_speichern(pfad: String, text: String) -> Result<(), String> {
    std::fs::write(&pfad, text).map_err(|e| format!("{pfad}: {e}"))
}

#[tauri::command]
async fn exp_speichern(json: String, rahmen: Option<String>, stoff: Option<String>, pfad: String) -> Result<usize, String> {
    let design = design_laden(&json, rahmen, stoff)?;
    let daten = exp_bytes(&design.berechnen()?);
    std::fs::write(&pfad, &daten).map_err(|e| format!("{pfad}: {e}"))?;
    Ok(daten.len())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            rahmen_liste,
            stoffe_liste,
            beispiel_design,
            design_berechnen,
            datei_lesen,
            text_speichern,
            exp_speichern
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Start des Programms");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beispiel_rechnet_und_passt_in_den_rahmen() {
        let e = tauri::async_runtime::block_on(design_berechnen(BEISPIEL.into(), None, None)).unwrap();
        assert!(e.statistik.stiche > 1000);
        assert_eq!(e.ausserhalb, 0);
        assert_eq!(e.farben.len(), e.statistik.farbwechsel + 1);
    }

    #[test]
    fn oberflaeche_ueberschreibt_rahmen_und_stoff() {
        let d = design_laden(BEISPIEL, Some("artista 40 x 60".into()), Some("Knit Light Weight".into())).unwrap();
        assert_eq!(d.rahmen.name, "artista 40 x 60");
        assert_eq!(d.einstellungen.stoff, "Knit Light Weight");
        assert!(design_laden("[1, 2]", None, None).is_err());
    }
}
