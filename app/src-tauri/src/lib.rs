//! Programm „Artista Stickprogramm“: Befehle der Oberfläche an den Rechenkern (`kern/`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use stick_formate::artista::{Farbe, N_185, Puffer, Sendedaten};
use stick_formate::esb;
use stick_formate::exp::{exp_bytes, exp_lesen};
use stick_kern::einstellungen::STOFFE;
use stick_kern::rahmen::{Rahmen, alle_rahmen};
use stick_kern::schrift::Alphabet;
use stick_kern::{Art, Design, Stich, StichFolge};
use stick_maschine::maschine::FarbBereich;
use stick_maschine::{
    Leitung, Maschine, SendeOptionen, SimOptionen, Simulation, Speicher, zeitstempel,
};
use tauri::{AppHandle, Emitter, Manager, State};

/// Original-Alphabete (ESB): Ordner und bereits geladene Alphabete (Laden dauert, Neuberechnung beim Tippen nicht).
struct Alphabete {
    ordner: PathBuf,
    geladen: Mutex<HashMap<String, Alphabet>>,
}

impl Alphabete {
    /// Ordner: Umgebungsvariable STICK_ALPHABETE, sonst Ressource „alphabete“ des Programms, sonst Quellordner.
    fn finden(ressourcen: Option<PathBuf>) -> PathBuf {
        if let Ok(p) = std::env::var("STICK_ALPHABETE") {
            return p.into();
        }
        if let Some(r) = ressourcen.map(|r| r.join("alphabete"))
            && r.is_dir()
        {
            return r;
        }
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/alphabete"))
    }

    /// Die Alphabete, die das Design braucht, einsetzen (aus dem Zwischenspeicher oder von der Platte).
    fn einsetzen(&self, design: &mut Design) -> Result<(), String> {
        let namen = design.alphabet_namen();
        if namen.is_empty() {
            return Ok(());
        }
        let mut geladen = self.geladen.lock().map_err(|e| e.to_string())?;
        let fehlend: Vec<&String> = namen.iter().filter(|n| !geladen.contains_key(*n)).collect();
        if !fehlend.is_empty() {
            let mut d = Design {
                alphabete: Default::default(),
                ..design.clone()
            };
            esb::design_alphabete_laden(&mut d, &self.ordner)?;
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

#[derive(Serialize)]
struct AlphabetInfo {
    name: String,
    datei: String,
}

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
    /// Zeichen eines Texts, die das Alphabet nicht hat
    fehlende_zeichen: String,
}

/// Design aus JSON; Rahmen und Stoff aus der Oberfläche haben Vorrang vor den Angaben im JSON.
fn design_laden(
    json: &str,
    rahmen: Option<String>,
    stoff: Option<String>,
) -> Result<Design, String> {
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
fn alphabete_liste(alphabete: State<'_, Alphabete>) -> Vec<AlphabetInfo> {
    esb::verzeichnis(&alphabete.ordner)
        .into_iter()
        .filter(|(n, _)| n != "ButtonHoles")
        .map(|(name, p)| AlphabetInfo {
            name,
            datei: p.file_name().unwrap_or_default().to_string_lossy().into(),
        })
        .collect()
}

#[tauri::command]
async fn design_berechnen(
    json: String,
    rahmen: Option<String>,
    stoff: Option<String>,
    alphabete: State<'_, Alphabete>,
) -> Result<Ergebnis, String> {
    berechnen_mit(&json, rahmen, stoff, &alphabete)
}

fn statistik_von(folge: &StichFolge) -> Statistik {
    let (x0, y0, x1, y1) = folge.begrenzung().unwrap_or((0.0, 0.0, 0.0, 0.0));
    let laengen = folge.stichlaengen();
    Statistik {
        stiche: folge.anzahl(Art::Stich),
        spruenge: folge.anzahl(Art::Sprung),
        farbwechsel: folge.anzahl(Art::Farbwechsel),
        breite: x1 - x0,
        hoehe: y1 - y0,
        min_stich: if laengen.is_empty() {
            0.0
        } else {
            laengen.iter().copied().fold(f64::INFINITY, f64::min)
        },
        max_stich: laengen.iter().copied().fold(0.0, f64::max),
    }
}

fn berechnen_mit(
    json: &str,
    rahmen: Option<String>,
    stoff: Option<String>,
    alphabete: &Alphabete,
) -> Result<Ergebnis, String> {
    let mut design = design_laden(json, rahmen, stoff)?;
    alphabete.einsetzen(&mut design)?;
    let (_, fehlend) = design.objekte_aufgeloest()?;
    let folge = design.berechnen()?;
    let statistik = statistik_von(&folge);
    Ok(Ergebnis {
        name: design.name.clone(),
        rahmen: RahmenInfo::from(&design.rahmen),
        stoff: design.einstellungen.stoff.clone(),
        ausserhalb: design.ausserhalb(&folge).len(),
        fehlende_zeichen: fehlend.into_iter().collect(),
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
async fn exp_speichern(
    json: String,
    rahmen: Option<String>,
    stoff: Option<String>,
    pfad: String,
    alphabete: State<'_, Alphabete>,
) -> Result<usize, String> {
    let mut design = design_laden(&json, rahmen, stoff)?;
    alphabete.einsetzen(&mut design)?;
    let daten = exp_bytes(&design.berechnen()?);
    std::fs::write(&pfad, &daten).map_err(|e| format!("{pfad}: {e}"))?;
    Ok(daten.len())
}

// ---------------------------------------------------------------------------------------------------------------
// Schritt 4: an die Maschine (kern/stick-maschine, Protokoll der Testprogramme v3.6/v4.1)
// ---------------------------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct GarnInfo {
    rgb: String,
    nummer: u16,
    marke: String,
    name: String,
}

impl From<&Farbe> for GarnInfo {
    fn from(f: &Farbe) -> Self {
        GarnInfo {
            rgb: format!("#{:02x}{:02x}{:02x}", f.rgb[0], f.rgb[1], f.rgb[2]),
            nummer: f.nummer,
            marke: f.marke.clone(),
            name: f.name.clone(),
        }
    }
}

/// Garnfarbbereich der Maschine (Schritt 12 des Originals).
#[derive(Serialize)]
struct FarbInfo {
    max_farben: u8,
    groesse: u16,
    text: String,
    farben: Vec<GarnInfo>,
}

impl From<&FarbBereich> for FarbInfo {
    fn from(f: &FarbBereich) -> Self {
        FarbInfo {
            max_farben: f.grenzen.max_farben,
            groesse: f.grenzen.groesse,
            text: f.text.clone(),
            farben: f.farben.iter().map(GarnInfo::from).collect(),
        }
    }
}

/// Angaben zu einer Sitzung mit der Maschine.
#[derive(Serialize)]
struct Sitzung {
    port: String,
    naehmaschine: String,
    stickmodul: String,
    baud: u32,
    /// PC-Modus mit 0x100 + T r M E verlassen
    verlassen: bool,
    mitschnitt: String,
}

#[derive(Serialize)]
struct MaschinenInfo {
    sitzung: Sitzung,
    farbbereich: FarbInfo,
}

#[derive(Serialize)]
struct PlanInfo {
    sitzung: Sitzung,
    schritte: Vec<String>,
    hinweise: Vec<String>,
    farbbereich: Option<FarbInfo>,
}

#[derive(Serialize)]
struct SendeInfo {
    sitzung: Sitzung,
    schritte: Vec<String>,
    /// Befehl 0x08 bestätigt (None = nicht gestickt)
    gestickt: Option<bool>,
}

/// Was beim Senden geschrieben wird (die INFO-Felder immer, wie das Original).
#[derive(Deserialize, Clone, Copy)]
struct SendeWahl {
    icon: bool,
    farben: bool,
    sticken: bool,
}

impl From<SendeWahl> for SendeOptionen {
    fn from(w: SendeWahl) -> Self {
        SendeOptionen {
            icon: w.icon,
            info: true,
            farben: w.farben,
            sticken: w.sticken,
        }
    }
}

#[derive(Serialize, Clone)]
struct Fortschritt {
    was: String,
    erledigt: usize,
    gesamt: usize,
}

/// Fortschritt großer Lese-/Schreibvorgänge als Ereignis `maschine_fortschritt` an die Oberfläche.
fn melder(app: &AppHandle) -> impl FnMut(&str, usize, usize) + use<> {
    let app = app.clone();
    move |was: &str, erledigt: usize, gesamt: usize| {
        let _ = app.emit(
            "maschine_fortschritt",
            Fortschritt {
                was: was.into(),
                erledigt,
                gesamt,
            },
        );
    }
}

fn leitung_oeffnen(port: &str, simulation: bool) -> Result<Box<dyn Leitung>, String> {
    if simulation {
        // wie nach einer früheren Sitzung: Maschine steht auf 57600 (Synchronisation wird mitgeprüft)
        return Ok(Box::new(Simulation::neu(SimOptionen {
            baud: 57600,
            ..Default::default()
        })));
    }
    Ok(Box::new(stick_maschine::port::SeriellerPort::oeffnen(
        port, 19200, false,
    )?))
}

fn mitschnitt_speichern(app: &AppHandle, art: &str, zeilen: &[String]) -> String {
    let ordner = app
        .path()
        .app_log_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    let _ = std::fs::create_dir_all(&ordner);
    let pfad = ordner.join(format!("Mitschnitt_{art}_{}.log", zeitstempel()));
    match std::fs::write(&pfad, zeilen.join("\r\n") + "\r\n") {
        Ok(()) => pfad.display().to_string(),
        Err(e) => format!("nicht gespeichert: {e}"),
    }
}

/// Verbinden, `arbeit` ausführen und den PC-Modus **immer** verlassen (0x100 + T r M E); Mitschnitt speichern.
fn mit_maschine<T>(
    app: &AppHandle,
    port: &str,
    simulation: bool,
    art: &str,
    arbeit: impl FnOnce(&mut Maschine<Box<dyn Leitung>>) -> Result<T, String>,
) -> Result<(T, Sitzung), String> {
    let anzeige = if simulation { "Simulation" } else { port };
    let l = leitung_oeffnen(port, simulation)?;
    let mut m = match Maschine::verbinden(l, 19200) {
        Ok(m) => m,
        Err(a) => {
            let pfad = mitschnitt_speichern(app, art, &a.protokoll);
            return Err(format!("{} (Mitschnitt: {pfad})", a.fehler));
        }
    };
    let r = arbeit(&mut m);
    let (naehmaschine, stickmodul, baud) = (
        m.firmware_naehmaschine.clone(),
        m.firmware_stickmodul.clone(),
        m.v.baud,
    );
    let (verlassen, v) = m.verlassen();
    let mitschnitt = mitschnitt_speichern(app, art, &v.protokoll);
    let t = r.map_err(|e| format!("{e} (Mitschnitt: {mitschnitt})"))?;
    Ok((
        t,
        Sitzung {
            port: anzeige.into(),
            naehmaschine,
            stickmodul,
            baud,
            verlassen,
            mitschnitt,
        },
    ))
}

/// Was das Original für dieses Design senden würde (Puffer, Icon, Garnfarben, INFO-Felder).
fn sendedaten(json: &str, alphabete: &Alphabete) -> Result<Sendedaten, String> {
    let mut design = design_laden(json, None, None)?;
    alphabete.einsetzen(&mut design)?;
    let folge = design.berechnen()?;
    let aussen = design.ausserhalb(&folge).len();
    if aussen > 0 {
        return Err(format!(
            "{aussen} Einstiche liegen außerhalb des Stickfelds – erst anpassen"
        ));
    }
    Ok(Sendedaten::eigenes_design(&design.name, &folge, false))
}

async fn im_hintergrund<T: Send + 'static>(
    f: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| format!("Hintergrundarbeit abgebrochen: {e}"))?
}

#[tauri::command]
fn maschine_ports() -> Vec<String> {
    stick_maschine::port::SeriellerPort::namen()
}

/// Nur lesen: verbinden, Garnfarbbereich lesen, PC-Modus verlassen.
#[tauri::command]
async fn maschine_pruefen(
    app: AppHandle,
    port: String,
    simulation: bool,
) -> Result<MaschinenInfo, String> {
    im_hintergrund(move || {
        let (fb, sitzung) = mit_maschine(&app, &port, simulation, "pruefen", |m| {
            m.farbbereich_lesen()
        })?;
        Ok(MaschinenInfo {
            sitzung,
            farbbereich: FarbInfo::from(&fb),
        })
    })
    .await
}

/// Nur lesen: Senden planen (Prüfungen wie das Original, Garnfarbbereich) – zum Bestätigen in der Oberfläche.
#[tauri::command]
async fn maschine_planen(
    app: AppHandle,
    json: String,
    port: String,
    simulation: bool,
    wahl: SendeWahl,
    alphabete: State<'_, Alphabete>,
) -> Result<PlanInfo, String> {
    let s = sendedaten(&json, &alphabete)?;
    im_hintergrund(move || {
        let (plan, sitzung) = mit_maschine(&app, &port, simulation, "planen", |m| {
            m.planen(&s, &wahl.into())
        })?;
        Ok(PlanInfo {
            sitzung,
            schritte: plan.schritte,
            hinweise: plan.hinweise,
            farbbereich: plan.farbbereich.as_ref().map(FarbInfo::from),
        })
    })
    .await
}

/// **Schreibt in die Maschine** (DATEN, ICON, INFO, Garnfarbtabelle; mit `sticken` Befehl 0x08). Die Oberfläche ruft
/// das nur nach der ausdrücklichen Bestätigung des Plans auf. Fortschritt als Ereignis `maschine_fortschritt`.
#[tauri::command]
async fn maschine_senden(
    app: AppHandle,
    json: String,
    port: String,
    simulation: bool,
    wahl: SendeWahl,
    alphabete: State<'_, Alphabete>,
) -> Result<SendeInfo, String> {
    let s = sendedaten(&json, &alphabete)?;
    im_hintergrund(move || {
        let mut fortschritt = melder(&app);
        let (b, sitzung) = mit_maschine(&app, &port, simulation, "senden", |m| {
            m.senden(&s, &wahl.into(), &mut fortschritt)
        })?;
        Ok(SendeInfo {
            sitzung,
            schritte: b.schritte,
            gestickt: b.sticken.map(|r| r.ok),
        })
    })
    .await
}

#[derive(Serialize)]
struct ListenZeile {
    nr: usize,
    name: String,
    flags: u8,
}

#[derive(Serialize)]
struct ListenInfo {
    sitzung: Sitzung,
    designs: Vec<ListenZeile>,
}

/// Aus der Maschine gelesenes Design zum Ansehen.
#[derive(Serialize)]
struct GelesenInfo {
    sitzung: Sitzung,
    nr: usize,
    ergebnis: Ergebnis,
    /// Garnfarben, die die Maschine nach dem Laden in die Garnfarbtabelle legt
    garne: Vec<GarnInfo>,
    bytes: usize,
}

/// Zuletzt gelesenes Maschinendesign (Stichdaten als EXP) – für „Als EXP speichern“.
struct Gelesen(Mutex<Option<(String, Vec<u8>)>>);

/// Kleinster Rahmen, in den alle Einstiche passen (sonst der größte).
fn passender_rahmen(folge: &StichFolge) -> Rahmen {
    let mut alle = alle_rahmen();
    alle.sort_by(|a, b| (a.breite * a.hoehe).total_cmp(&(b.breite * b.hoehe)));
    alle.iter()
        .find(|r| folge.stiche.iter().all(|s| r.enthaelt(s.p())))
        .or(alle.last())
        .cloned()
        .unwrap_or_else(|| alle_rahmen().remove(0))
}

/// Designliste aus dem Maschinenspeicher (nur lesen; Befehle 0xA0/0x30/0x20/0x60, Auswahl).
#[tauri::command]
async fn maschine_liste(
    app: AppHandle,
    port: String,
    simulation: bool,
) -> Result<ListenInfo, String> {
    im_hintergrund(move || {
        let (l, sitzung) = mit_maschine(&app, &port, simulation, "liste", |m| {
            m.designliste(Speicher::Maschine)
        })?;
        Ok(ListenInfo {
            sitzung,
            designs: l
                .into_iter()
                .map(|e| ListenZeile {
                    nr: e.nr,
                    name: e.name,
                    flags: e.flags,
                })
                .collect(),
        })
    })
    .await
}

/// Ein Design aus dem Maschinenspeicher lesen (Befehl 0x400, nur lesen) und zum Ansehen aufbereiten.
#[tauri::command]
async fn maschine_lesen(
    app: AppHandle,
    port: String,
    simulation: bool,
    nr: usize,
    gelesen: State<'_, Gelesen>,
) -> Result<GelesenInfo, String> {
    let app2 = app.clone();
    let (d, fb, sitzung) = im_hintergrund(move || {
        let mut fortschritt = melder(&app2);
        let ((d, fb), sitzung) = mit_maschine(&app2, &port, simulation, "lesen", |m| {
            let d = m.design_lesen(nr, &mut fortschritt)?;
            let fb = m.farbbereich_lesen()?;
            Ok((d, fb))
        })?;
        Ok((d, fb, sitzung))
    })
    .await?;
    let p = Puffer::lesen(&d.puffer, N_185)?;
    let mut folge = exp_lesen(&p.stiche);
    let name = p
        .container()
        .and_then(Result::ok)
        .and_then(|c| c.kopf())
        .map(|k| k.name())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| d.name.clone());
    let garne: Vec<GarnInfo> = fb.farben.iter().map(GarnInfo::from).collect();
    for (i, g) in garne.iter().enumerate() {
        if let Some(f) = folge.farben.get_mut(i) {
            *f = g.rgb.clone();
        }
    }
    let rahmen = passender_rahmen(&folge);
    let ausserhalb = folge
        .stiche
        .iter()
        .filter(|s| !rahmen.enthaelt(s.p()))
        .count();
    let titel = format!("Nr. {nr} „{name}“ (aus der Maschine)");
    *gelesen.0.lock().map_err(|e| e.to_string())? =
        Some((format!("Maschine_Nr{nr:02}"), p.stiche.clone()));
    Ok(GelesenInfo {
        sitzung,
        nr,
        bytes: d.puffer.len(),
        garne,
        ergebnis: Ergebnis {
            name: titel,
            rahmen: RahmenInfo::from(&rahmen),
            stoff: "–".into(),
            statistik: statistik_von(&folge),
            ausserhalb,
            fehlende_zeichen: String::new(),
            farben: std::mem::take(&mut folge.farben),
            stiche: folge.stiche,
        },
    })
}

/// Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern (unverändert, mit Endkennung 80 81).
#[tauri::command]
async fn gelesen_exp_speichern(pfad: String, gelesen: State<'_, Gelesen>) -> Result<usize, String> {
    let g = gelesen.0.lock().map_err(|e| e.to_string())?;
    let (_, exp) = g
        .as_ref()
        .ok_or("Es wurde noch kein Design aus der Maschine gelesen")?;
    std::fs::write(&pfad, exp).map_err(|e| format!("{pfad}: {e}"))?;
    Ok(exp.len())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let ordner = Alphabete::finden(app.path().resource_dir().ok());
            app.manage(Alphabete {
                ordner,
                geladen: Mutex::new(HashMap::new()),
            });
            app.manage(Gelesen(Mutex::new(None)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            rahmen_liste,
            stoffe_liste,
            alphabete_liste,
            beispiel_design,
            design_berechnen,
            datei_lesen,
            text_speichern,
            exp_speichern,
            maschine_ports,
            maschine_pruefen,
            maschine_planen,
            maschine_senden,
            maschine_liste,
            maschine_lesen,
            gelesen_exp_speichern
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Start des Programms");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beispiel_rechnet_und_passt_in_den_rahmen() {
        let a = Alphabete {
            ordner: Alphabete::finden(None),
            geladen: Mutex::new(HashMap::new()),
        };
        let e = berechnen_mit(BEISPIEL, None, None, &a).unwrap();
        assert!(e.statistik.stiche > 1000);
        assert_eq!(e.ausserhalb, 0);
        assert_eq!(e.farben.len(), e.statistik.farbwechsel + 1);
    }

    #[test]
    fn text_mit_alphabet() {
        let a = Alphabete {
            ordner: Alphabete::finden(None),
            geladen: Mutex::new(HashMap::new()),
        };
        let json = r#"{"objekte":[{"art":"text","text":"Anna","alphabet":"Drifter","hoehe":20}]}"#;
        let mut d = design_laden(json, None, None).unwrap();
        a.einsetzen(&mut d).unwrap();
        let f = d.berechnen().unwrap();
        assert!(f.anzahl(Art::Stich) > 500);
        assert_eq!(a.geladen.lock().unwrap().len(), 1);
    }

    #[test]
    fn sendedaten_aus_dem_beispiel() {
        let a = Alphabete {
            ordner: Alphabete::finden(None),
            geladen: Mutex::new(HashMap::new()),
        };
        let s = sendedaten(BEISPIEL, &a).unwrap();
        assert!(s.puffer.endkennung());
        assert_eq!(s.farben.len(), s.info.farben_minus_1 as usize + 1);
        assert_eq!(s.icon.bytes().len(), 732);
        // Design außerhalb des Stickfelds wird nicht gesendet
        let gross =
            r#"{"rahmen":"artista 40 x 60","objekte":[{"art":"lauf","punkte":[[-50,0],[50,0]]}]}"#;
        assert!(sendedaten(gross, &a).unwrap_err().contains("außerhalb"));
    }

    #[test]
    fn oberflaeche_ueberschreibt_rahmen_und_stoff() {
        let d = design_laden(
            BEISPIEL,
            Some("artista 40 x 60".into()),
            Some("Knit Light Weight".into()),
        )
        .unwrap();
        assert_eq!(d.rahmen.name, "artista 40 x 60");
        assert_eq!(d.einstellungen.stoff, "Knit Light Weight");
        assert!(design_laden("[1, 2]", None, None).is_err());
    }
}
