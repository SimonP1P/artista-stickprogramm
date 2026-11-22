//! Programm „Artista Stickprogramm“: Fenster und **ein** Befehl `kern`, der alles an den Rechenkern weiterreicht
//! (`kern/stick-api`, ROADMAP 4b.0) – auch die Maschinen-Befehle (`maschine_*`, seit Etappe E3). Fortschritt der
//! Maschine kommt als Ereignis `maschine_fortschritt` an die Oberfläche, Mitschnitte liegen im Log-Ordner.

use serde_json::Value;
use std::sync::Arc;
use tauri::{Emitter, Manager, State};

/// **Der** Befehl der Oberfläche an den Rechenkern: reicht an `stick_api::ausfuehren` weiter (Einbindung wie im Kopf
/// von `kern/stick-api/src/lib.rs`). Die Oberfläche ruft ihn nur über `kern(…)` aus `kern/schnittstelle/kern.ts` auf;
/// gerechnet wird im Hintergrund, damit das Fenster beim Tippen und bei Maschinen-Vorgängen nicht hängt.
#[tauri::command]
async fn kern(
    befehl: String,
    daten: Value,
    k: State<'_, Arc<stick_api::Kontext>>,
) -> Result<Value, stick_api::Fehler> {
    let k = k.inner().clone();
    tauri::async_runtime::spawn_blocking(move || stick_api::ausfuehren(&k, &befehl, daten))
        .await
        .map_err(|e| stick_api::Fehler::from(e.to_string()))?
}

/// Startoptionen aus der Befehlszeile als JSON für die Oberfläche (`window.__START__`, `app/src/lib/arbeit.svelte.ts`):
/// `artista-stickprogramm [DATEI.json] [--experte] [--dunkel|--hell] [--auswahl N]`.
fn startoptionen(args: impl Iterator<Item = String>) -> Value {
    let mut o = serde_json::Map::new();
    let mut args = args.peekable();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--experte" => {
                o.insert("modus".into(), "experte".into());
            }
            "--dunkel" | "--hell" => {
                o.insert("dunkel".into(), (a == "--dunkel").into());
            }
            "--auswahl" => {
                if let Some(n) = args.next().and_then(|n| n.parse::<u64>().ok()) {
                    o.insert("auswahl".into(), n.into());
                }
            }
            _ if !a.starts_with('-') => {
                o.insert("datei".into(), a.into());
            }
            _ => {}
        }
    }
    Value::Object(o)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Kontext des Kerns: Alphabete und Garnkarten aus den Ressourcen des Programms
            let k = stick_api::Kontext::finden(app.path().resource_dir().ok().as_deref());
            let h = app.handle().clone();
            k.fortschritt_melden(move |f| {
                let _ = h.emit("maschine_fortschritt", f);
            });
            if let Ok(log) = app.path().app_log_dir() {
                k.mitschnitte_nach(log);
            }
            // heruntergeladene Google Fonts bleiben im Datenordner des Programms (nicht im System installiert)
            if let Ok(daten) = app.path().app_data_dir() {
                k.schriften_ablage(daten.join("schriften"));
                // eigene Motivsätze (Create Pattern) ebenfalls im Datenordner
                k.motive_ablage(daten.join("motive"));
            }
            app.manage(Arc::new(k));

            // Hauptfenster mit Startoptionen (Datei öffnen, Modus, Darstellung) – für „Öffnen mit“ und Bildschirmfotos
            let fenster = app
                .config()
                .app
                .windows
                .iter()
                .find(|w| w.label == "main")
                .cloned()
                .ok_or("Fenster „main“ fehlt in tauri.conf.json")?;
            tauri::WebviewWindowBuilder::from_config(app.handle(), &fenster)?
                .initialization_script(format!(
                    "window.__START__ = {};",
                    startoptionen(std::env::args().skip(1))
                ))
                // kein „Inspect Element“ im Rechtsklick-Menü (Nutzerwunsch 21.11.2026); zur Fehlersuche mit
                // Umgebungsvariable ARTISTA_INSPEKTOR=1 starten
                .devtools(std::env::var_os("ARTISTA_INSPEKTOR").is_some())
                .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![kern])
        .run(tauri::generate_context!())
        .expect("Fehler beim Start des Programms");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startoptionen_lesen() {
        let a = ["d.json", "--experte", "--hell", "--auswahl", "2"].map(String::from);
        let o = startoptionen(a.into_iter());
        assert_eq!(o["datei"], "d.json");
        assert_eq!(o["modus"], "experte");
        assert_eq!(o["dunkel"], false);
        assert_eq!(o["auswahl"], 2);
    }

    #[test]
    fn kern_rechnet_das_beispiel() {
        let k = stick_api::Kontext::finden(None);
        let d = stick_api::ausfuehren(&k, "beispiel_design", Value::Null).unwrap();
        let e = stick_api::ausfuehren(&k, "design_berechnen", serde_json::json!({ "design": d })).unwrap();
        assert_eq!(e["ausserhalb"], 0);
    }
}
