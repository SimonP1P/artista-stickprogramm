//! **Befehle für die Maschine** (artista 185 am seriellen Port, `stick-maschine`; ROADMAP Phase 5, Etappe E3, Wunsch
//! W10 der Oberfläche v3). Abläufe unverändert aus der App (Protokoll der Testprogramme v3.6/v4.1, am Gerät erprobt):
//! verbinden → Arbeit → **immer** PC-Modus verlassen (0x100 + `T r M E`) → Mitschnitt speichern.
//!
//! Neu gegenüber der App: **Abbrechen** (`maschine_abbrechen`), Fortschritt mit **Wiederholungen**, Fehler mit **Code**
//! (`Fehler::code`), Sperre gegen gleichzeitige Maschinen-Aktionen, Garn von Hand (`garn`) in der Garnfarbtabelle.
//!
//! **Sicherheit** (CLAUDE.md §4 Regel 1/1a, ANFORDERUNGEN §4): Nur `maschine_senden` schreibt in die Maschine; die
//! Oberfläche ruft es erst nach `maschine_planen` und ausdrücklicher Bestätigung auf. Sticken (Befehl 0x08) nur mit
//! `wahl.sticken`, das die Oberfläche nur nach dem eigenen Gefahrendialog setzt.

use crate::befehle::{DesignEingabe, DesignWert, Ergebnis, Leer, RahmenInfo, statistik_von};
use crate::{Fehler, FehlerCode, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use stick_formate::artista::{Farbe, N_185, Puffer, Sendedaten};
use stick_formate::exp::exp_lesen;
use stick_formate::garnkarte;
use stick_kern::rahmen::{Rahmen, alle_rahmen};
use stick_kern::{Design, StichFolge};
use stick_maschine::maschine::FarbBereich;
use stick_maschine::{Leitung, Maschine, SendeOptionen, SimOptionen, Simulation, Speicher, zeitstempel};

// ---------------------------------------------------------------- Ein- und Ausgaben

/// Anschluss: COM-Port oder Simulation (ohne Maschine, es wird nichts gesendet).
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Anschluss {
    /// Name des Ports (z. B. „COM3“); bei `simulation` ohne Bedeutung
    #[serde(default)]
    pub port: String,
    /// Simulation statt echter Maschine
    #[serde(default)]
    pub simulation: bool,
}

/// Was beim Senden geschrieben wird (die INFO-Felder immer, wie das Original).
#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SendeWahl {
    /// Vorschaubild (ICON 72 × 62) mitsenden
    pub icon: bool,
    /// Garnfarbtabelle mitsenden (Garnnamen am Display)
    pub farben: bool,
    /// danach sticken (Befehl 0x08, „sticken ohne speichern“) – **nur** nach dem Gefahrendialog („Person an der
    /// Maschine“, Rahmen fährt)
    pub sticken: bool,
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

/// Design an die Maschine: Design, Anschluss, Wahl.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SendeEingabe {
    pub design: DesignWert,
    pub port: String,
    #[serde(default)]
    pub simulation: bool,
    pub wahl: SendeWahl,
}

/// Design Nr. … aus der Maschine lesen.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LeseEingabe {
    #[serde(default)]
    pub port: String,
    #[serde(default)]
    pub simulation: bool,
    /// Nummer in der Designliste (ab 1)
    pub nr: usize,
}

/// Garn der Maschine (Eintrag der Garnfarbtabelle).
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct MaschinenGarn {
    /// Farbe „#rrggbb“
    pub rgb: String,
    pub nummer: u16,
    pub marke: String,
    pub name: String,
}

impl From<&Farbe> for MaschinenGarn {
    fn from(f: &Farbe) -> Self {
        MaschinenGarn {
            rgb: format!("#{:02x}{:02x}{:02x}", f.rgb[0], f.rgb[1], f.rgb[2]),
            nummer: f.nummer,
            marke: f.marke.clone(),
            name: f.name.clone(),
        }
    }
}

/// Garnfarbbereich der Maschine (Schritt 12 des Originals).
#[derive(Debug, Serialize, JsonSchema)]
pub struct FarbInfo {
    /// höchste Farbanzahl der Tabelle (artista 185: 60)
    pub max_farben: u8,
    pub groesse: u16,
    /// Zustand in Worten
    pub text: String,
    pub farben: Vec<MaschinenGarn>,
}

impl From<&FarbBereich> for FarbInfo {
    fn from(f: &FarbBereich) -> Self {
        FarbInfo {
            max_farben: f.grenzen.max_farben,
            groesse: f.grenzen.groesse,
            text: f.text.clone(),
            farben: f.farben.iter().map(MaschinenGarn::from).collect(),
        }
    }
}

/// Angaben zu einer Sitzung mit der Maschine.
#[derive(Debug, Serialize, JsonSchema)]
pub struct Sitzung {
    /// Port oder „Simulation“
    pub port: String,
    pub simulation: bool,
    /// Firmware der Nähmaschine (z. B. „04.04“)
    pub naehmaschine: String,
    /// Firmware des Stickmoduls
    pub stickmodul: String,
    pub baud: u32,
    /// PC-Modus mit 0x100 + T r M E verlassen
    pub verlassen: bool,
    /// Pfad des Mitschnitts
    pub mitschnitt: String,
}

/// Ergebnis von `maschine_pruefen`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct MaschinenInfo {
    pub sitzung: Sitzung,
    pub farbbereich: FarbInfo,
}

/// Plan des Sendens (nur gelesen) – zum Bestätigen.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SendePlanInfo {
    pub sitzung: Sitzung,
    /// geplante Schreibzugriffe in Worten (DATEN, ICON, INFO, Garnfarbtabelle, Befehl 0x08)
    pub schritte: Vec<String>,
    pub hinweise: Vec<String>,
    pub farbbereich: Option<FarbInfo>,
    /// Größe der Teile in Byte: Daten (Puffer), Vorschaubild, Garnfarbtabelle (0 = wird nicht geschrieben)
    pub bytes_daten: usize,
    pub bytes_icon: usize,
    pub bytes_farben: usize,
    /// Garne, die an der Maschine erscheinen (je Farbblock)
    pub garne: Vec<MaschinenGarn>,
}

/// Ergebnis von `maschine_senden`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SendeInfo {
    pub sitzung: Sitzung,
    pub schritte: Vec<String>,
    /// Befehl 0x08 bestätigt (None = nicht gestickt)
    pub gestickt: Option<bool>,
}

/// Eintrag der Designliste der Maschine.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ListenZeile {
    pub nr: usize,
    pub name: String,
    pub flags: u8,
}

/// Ergebnis von `maschine_liste`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ListenInfo {
    pub sitzung: Sitzung,
    pub designs: Vec<ListenZeile>,
}

/// Aus der Maschine gelesenes Design zum Ansehen (Vorschau wie `design_berechnen`).
#[derive(Debug, Serialize, JsonSchema)]
pub struct GelesenInfo {
    pub sitzung: Sitzung,
    pub nr: usize,
    pub ergebnis: Ergebnis,
    /// Garnfarben, die die Maschine nach dem Laden in die Garnfarbtabelle legt
    pub garne: Vec<MaschinenGarn>,
    pub bytes: usize,
}

/// Zieldatei.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExpZiel {
    pub pfad: String,
}

/// Fortschritt großer Lese-/Schreibvorgänge (Ereignis `maschine_fortschritt` der App).
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Fortschritt {
    /// Teil („DATEN“, „ICON“, „Garnfarbtabelle“ …)
    pub was: String,
    pub erledigt: usize,
    pub gesamt: usize,
    /// Wiederholungen bisher in diesem Teil (Übertragungsfehler werden automatisch wiederholt)
    pub wiederholungen: usize,
}

// ---------------------------------------------------------------- Zustand

/// Empfänger des Fortschritts (App: Ereignis an die Oberfläche).
type Melder = Arc<dyn Fn(&Fortschritt) + Send + Sync>;

/// Zustand der Maschinen-Befehle im Kontext.
pub struct MaschinenZustand {
    /// Abbrechen angefordert (gilt für die laufende Aktion)
    abbruch: Arc<AtomicBool>,
    /// eine Maschinen-Aktion läuft
    belegt: AtomicBool,
    /// Fortschritt an die Oberfläche (App: Ereignis `maschine_fortschritt`)
    melder: std::sync::Mutex<Option<Melder>>,
    /// Ordner der Mitschnitte (App: Log-Ordner)
    mitschnitte: std::sync::Mutex<std::path::PathBuf>,
    /// zuletzt gelesenes Maschinendesign (Name, Stichdaten EXP) für `gelesen_exp_speichern`
    gelesen: std::sync::Mutex<Option<(String, Vec<u8>)>>,
}

impl Default for MaschinenZustand {
    fn default() -> Self {
        MaschinenZustand {
            abbruch: Arc::new(AtomicBool::new(false)),
            belegt: AtomicBool::new(false),
            melder: std::sync::Mutex::new(None),
            mitschnitte: std::sync::Mutex::new(std::env::temp_dir().join("artista_mitschnitte")),
            gelesen: std::sync::Mutex::new(None),
        }
    }
}

impl Kontext {
    /// Fortschritt der Maschine melden (App: als Ereignis an die Oberfläche).
    pub fn fortschritt_melden(&self, f: impl Fn(&Fortschritt) + Send + Sync + 'static) {
        if let Ok(mut m) = self.maschine.melder.lock() {
            *m = Some(Arc::new(f));
        }
    }

    /// Ordner der Mitschnitte setzen (App: Log-Ordner des Programms).
    pub fn mitschnitte_nach(&self, ordner: std::path::PathBuf) {
        if let Ok(mut m) = self.maschine.mitschnitte.lock() {
            *m = ordner;
        }
    }
}

/// Belegt die Maschine für eine Aktion; gibt sie beim Ende wieder frei.
struct Belegung<'a>(&'a MaschinenZustand);

impl Drop for Belegung<'_> {
    fn drop(&mut self) {
        self.0.belegt.store(false, Ordering::SeqCst);
    }
}

fn belegen(z: &MaschinenZustand) -> Result<Belegung<'_>, Fehler> {
    if z.belegt.swap(true, Ordering::SeqCst) {
        return Err(Fehler::mit_code(
            "Es läuft schon eine Aktion mit der Maschine – bitte warten.",
            FehlerCode::Belegt,
        ));
    }
    z.abbruch.store(false, Ordering::SeqCst);
    Ok(Belegung(z))
}

/// Leitung, die nach „Abbrechen“ nichts mehr sendet und empfängt – die laufende Übertragung scheitert an der
/// nächsten Stelle. Vor dem Verlassen des PC-Modus wird sie wieder durchlässig (`aktiv = false`).
struct Abbrechbar {
    innen: Box<dyn Leitung>,
    abbruch: Arc<AtomicBool>,
    aktiv: Arc<AtomicBool>,
}

impl Abbrechbar {
    fn gesperrt(&self) -> bool {
        self.aktiv.load(Ordering::SeqCst) && self.abbruch.load(Ordering::SeqCst)
    }
}

impl Leitung for Abbrechbar {
    fn senden(&mut self, daten: &[u8]) -> Result<(), String> {
        if self.gesperrt() {
            return Err("abgebrochen".into());
        }
        self.innen.senden(daten)
    }
    fn empfangen(&mut self, ms: u64) -> Option<u8> {
        if self.gesperrt() {
            return None;
        }
        self.innen.empfangen(ms)
    }
    fn baud_setzen(&mut self, baud: u32) -> Result<(), String> {
        self.innen.baud_setzen(baud)
    }
    fn warten(&mut self, ms: u64) {
        self.innen.warten(ms)
    }
    fn jetzt_ms(&mut self) -> u64 {
        self.innen.jetzt_ms()
    }
}

/// Fehlercode aus der Meldung von `stick-maschine` (die Meldungen sind deutsch und fest, `maschine.rs`/`protokoll.rs`).
fn code_aus(meldung: &str, abgebrochen: bool) -> FehlerCode {
    let m = meldung.to_lowercase();
    if abgebrochen {
        FehlerCode::Abgebrochen
    } else if m.contains("keine verbindung") || m.contains("lässt sich nicht öffnen") {
        FehlerCode::KeineVerbindung
    } else if m.contains("stickmodul") {
        FehlerCode::Stickmodul
    } else if m.contains("weicht ab") {
        FehlerCode::ZurueckgelesenWeichtAb
    } else if m.contains("garnfarbtabelle") || m.contains("tabelle zu groß") {
        FehlerCode::Farbtabelle
    } else if m.contains("zu groß") {
        FehlerCode::ZuGross
    } else if m.contains("außerhalb") {
        FehlerCode::Ausserhalb
    } else {
        FehlerCode::AntwortetNicht
    }
}

fn mitschnitt_speichern(k: &Kontext, art: &str, zeilen: &[String]) -> String {
    let ordner = k
        .maschine
        .mitschnitte
        .lock()
        .map(|o| o.clone())
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
    k: &Kontext,
    a: &Anschluss,
    art: &str,
    arbeit: impl FnOnce(&mut Maschine<Abbrechbar>, &mut dyn FnMut(&str, usize, usize, usize)) -> Result<T, String>,
) -> Result<(T, Sitzung), Fehler> {
    let z = &k.maschine;
    let _b = belegen(z)?;
    let innen: Box<dyn Leitung> = if a.simulation {
        // wie nach einer früheren Sitzung: Maschine steht auf 57600 (Synchronisation wird mitgeprüft)
        Box::new(Simulation::neu(SimOptionen {
            baud: 57600,
            ..Default::default()
        }))
    } else {
        Box::new(
            stick_maschine::port::SeriellerPort::oeffnen(&a.port, 19200, false)
                .map_err(|m| Fehler::mit_code(m, FehlerCode::KeineVerbindung))?,
        )
    };
    let aktiv = Arc::new(AtomicBool::new(true));
    let l = Abbrechbar {
        innen,
        abbruch: z.abbruch.clone(),
        aktiv: aktiv.clone(),
    };
    let abgebrochen = || z.abbruch.load(Ordering::SeqCst);
    let mut m = match Maschine::verbinden(l, 19200) {
        Ok(m) => m,
        Err(x) => {
            let pfad = mitschnitt_speichern(k, art, &x.protokoll);
            return Err(Fehler::mit_code(
                format!("{} (Mitschnitt: {pfad})", x.fehler),
                code_aus(&x.fehler, abgebrochen()),
            ));
        }
    };
    let melder = z.melder.lock().ok().and_then(|m| m.clone());
    let mut fortschritt = |was: &str, erledigt: usize, gesamt: usize, wiederholungen: usize| {
        if let Some(f) = &melder {
            f(&Fortschritt {
                was: was.into(),
                erledigt,
                gesamt,
                wiederholungen,
            });
        }
    };
    let r = arbeit(&mut m, &mut fortschritt);
    let (naehmaschine, stickmodul, baud) = (m.firmware_naehmaschine.clone(), m.firmware_stickmodul.clone(), m.v.baud);
    // nach Abbruch die Leitung wieder freigeben, damit der PC-Modus sicher verlassen wird
    aktiv.store(false, Ordering::SeqCst);
    let (verlassen, v) = m.verlassen();
    let mitschnitt = mitschnitt_speichern(k, art, &v.protokoll);
    let t = r.map_err(|e| {
        let ab = abgebrochen();
        let text = if ab {
            "Abgebrochen – es wurde nichts gestickt".to_string()
        } else {
            e.clone()
        };
        Fehler::mit_code(format!("{text} (Mitschnitt: {mitschnitt})"), code_aus(&e, ab))
    })?;
    Ok((
        t,
        Sitzung {
            port: if a.simulation {
                "Simulation".into()
            } else {
                a.port.clone()
            },
            simulation: a.simulation,
            naehmaschine,
            stickmodul,
            baud,
            verlassen,
            mitschnitt,
        },
    ))
}

/// Was das Original für dieses Design senden würde: Puffer, Icon, INFO-Felder, Garnfarben (Garn von Hand `garn` vor
/// dem nächsten Garn der Karte).
fn sendedaten(k: &Kontext, design: DesignWert) -> Result<(Sendedaten, Design), Fehler> {
    let d = k.design(&DesignEingabe {
        design,
        rahmen: None,
        stoff: None,
    })?;
    let folge = d.berechnen()?;
    let aussen = d.ausserhalb(&folge).len();
    if aussen > 0 {
        return Err(Fehler::mit_code(
            format!("{aussen} Einstiche liegen außerhalb des Stickfelds – erst anpassen"),
            FehlerCode::Ausserhalb,
        ));
    }
    let mut s = Sendedaten::eigenes_design(&d.name, &folge, false);
    if d.garnkarte != "keine" {
        let karte = garnkarte::laden(&k.garnkarten, &d.garnkarte)?;
        let bloecke = crate::befehle::bloecke_von(&d, &folge);
        for (f, b) in s.farben.iter_mut().zip(&bloecke) {
            if let Some(nr) = d.garn.get(&b.objekt) {
                let g = karte.garne.iter().find(|g| g.nummer == *nr).ok_or_else(|| Fehler {
                    meldung: format!("Objekt {}: Garn „{nr}“ gibt es in der Garnkarte nicht", b.objekt + 1),
                    objekt: Some(b.objekt),
                    code: None,
                })?;
                *f = g.farbe();
            }
        }
        s.garne_zuordnen(&karte);
    }
    Ok((s, d))
}

fn anschluss(port: &str, simulation: bool) -> Anschluss {
    Anschluss {
        port: port.into(),
        simulation,
    }
}

// ---------------------------------------------------------------- Befehle

pub fn maschine_ports(_: &Kontext, _: Leer) -> Result<Vec<String>, Fehler> {
    Ok(stick_maschine::port::SeriellerPort::namen())
}

/// Nur lesen: verbinden, Garnfarbbereich lesen, PC-Modus verlassen.
pub fn maschine_pruefen(k: &Kontext, a: Anschluss) -> Result<MaschinenInfo, Fehler> {
    let (fb, sitzung) = mit_maschine(k, &a, "pruefen", |m, _| m.farbbereich_lesen())?;
    Ok(MaschinenInfo {
        sitzung,
        farbbereich: FarbInfo::from(&fb),
    })
}

/// Nur lesen: Senden planen (Prüfungen wie das Original, Garnfarbbereich) – zum Bestätigen.
pub fn maschine_planen(k: &Kontext, e: SendeEingabe) -> Result<SendePlanInfo, Fehler> {
    let (s, _) = sendedaten(k, e.design)?;
    let o: SendeOptionen = e.wahl.into();
    let (plan, sitzung) = mit_maschine(k, &anschluss(&e.port, e.simulation), "planen", |m, _| m.planen(&s, &o))?;
    Ok(SendePlanInfo {
        sitzung,
        schritte: plan.schritte,
        hinweise: plan.hinweise,
        farbbereich: plan.farbbereich.as_ref().map(FarbInfo::from),
        bytes_daten: s.puffer.bytes().len(),
        bytes_icon: if plan.icon_schreiben { s.icon.bytes().len() } else { 0 },
        bytes_farben: if plan.farben_schreiben {
            s.farbtabelle().map(|t| t.len()).unwrap_or(0)
        } else {
            0
        },
        garne: s.farben.iter().map(MaschinenGarn::from).collect(),
    })
}

/// **Schreibt in die Maschine** (DATEN, ICON, INFO, Garnfarbtabelle; mit `wahl.sticken` Befehl 0x08). Nur nach der
/// Bestätigung des Plans aufrufen. Fortschritt über den Melder (App: Ereignis `maschine_fortschritt`).
pub fn maschine_senden(k: &Kontext, e: SendeEingabe) -> Result<SendeInfo, Fehler> {
    let (s, _) = sendedaten(k, e.design)?;
    let o: SendeOptionen = e.wahl.into();
    let (b, sitzung) = mit_maschine(k, &anschluss(&e.port, e.simulation), "senden", |m, f| {
        m.senden(&s, &o, f)
    })?;
    Ok(SendeInfo {
        sitzung,
        schritte: b.schritte,
        gestickt: b.sticken.map(|r| r.ok),
    })
}

/// Laufende Maschinen-Aktion abbrechen (die Übertragung bricht an der nächsten Stelle ab, der PC-Modus wird
/// verlassen). Ergebnis: ob eine Aktion lief.
pub fn maschine_abbrechen(k: &Kontext, _: Leer) -> Result<bool, Fehler> {
    let lief = k.maschine.belegt.load(Ordering::SeqCst);
    if lief {
        k.maschine.abbruch.store(true, Ordering::SeqCst);
    }
    Ok(lief)
}

/// Designliste aus dem Maschinenspeicher (nur lesen; Befehle 0xA0/0x30/0x20/0x60, Auswahl).
pub fn maschine_liste(k: &Kontext, a: Anschluss) -> Result<ListenInfo, Fehler> {
    let (l, sitzung) = mit_maschine(k, &a, "liste", |m, _| m.designliste(Speicher::Maschine))?;
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
}

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

/// Ein Design aus dem Maschinenspeicher lesen (Befehl 0x400, nur lesen) und zum Ansehen aufbereiten.
pub fn maschine_lesen(k: &Kontext, e: LeseEingabe) -> Result<GelesenInfo, Fehler> {
    let nr = e.nr;
    let ((d, fb), sitzung) = mit_maschine(k, &anschluss(&e.port, e.simulation), "lesen", |m, f| {
        let d = m.design_lesen(nr, f)?;
        let fb = m.farbbereich_lesen()?;
        Ok((d, fb))
    })?;
    let p = Puffer::lesen(&d.puffer, N_185)?;
    let mut folge = exp_lesen(&p.stiche);
    let name = p
        .container()
        .and_then(Result::ok)
        .and_then(|c| c.kopf())
        .map(|k| k.name())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| d.name.clone());
    let garne: Vec<MaschinenGarn> = fb.farben.iter().map(MaschinenGarn::from).collect();
    for (i, g) in garne.iter().enumerate() {
        if let Some(f) = folge.farben.get_mut(i) {
            *f = g.rgb.clone();
        }
    }
    let rahmen = passender_rahmen(&folge);
    let ausserhalb_stiche: Vec<usize> = folge
        .stiche
        .iter()
        .enumerate()
        .filter(|(_, s)| !rahmen.enthaelt(s.p()))
        .map(|(i, _)| i)
        .collect();
    if let Ok(mut g) = k.maschine.gelesen.lock() {
        *g = Some((format!("Maschine_Nr{nr:02}"), p.stiche.clone()));
    }
    let n = folge.stiche.len();
    Ok(GelesenInfo {
        sitzung,
        nr,
        bytes: d.puffer.len(),
        ergebnis: Ergebnis {
            name: format!("Nr. {nr} „{name}“ (aus der Maschine)"),
            rahmen: RahmenInfo::from(&rahmen),
            stoff: "–".into(),
            statistik: statistik_von(&folge),
            begrenzung: folge.begrenzung().map(|(a, b, c, d)| [a, b, c, d]),
            ausserhalb: ausserhalb_stiche.len(),
            ausserhalb_stiche,
            fehlende_zeichen: String::new(),
            garne: Vec::new(),
            bloecke: Vec::new(),
            daten_bytes: p.stiche.len() + p.anhang.len() + 2,
            daten_grenze: crate::befehle::DATEN_GRENZE,
            hinweise: Vec::new(),
            objekt: vec![0; n],
            farben: std::mem::take(&mut folge.farben),
            stiche: folge.stiche,
        },
        garne,
    })
}

/// Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern (unverändert, mit Endkennung 80 81).
pub fn gelesen_exp_speichern(k: &Kontext, e: ExpZiel) -> Result<usize, Fehler> {
    let g = k.maschine.gelesen.lock().map_err(|x| Fehler::from(x.to_string()))?;
    let (_, exp) = g.as_ref().ok_or("Es wurde noch kein Design aus der Maschine gelesen")?;
    std::fs::write(&e.pfad, exp).map_err(|x| Fehler::from(format!("{}: {x}", e.pfad)))?;
    Ok(exp.len())
}
