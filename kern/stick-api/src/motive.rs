//! Eigene Motive (Handbuch Kap. 25 „Creating your own pattern sets“, S. 230–231): Objekte des Designs als Motiv in
//! einen eigenen Motivsatz speichern und wieder löschen. Ablage: `<Datenordner>/motive/<Satz>.json` (eigene
//! Erweiterung – das Original schreibt ESB-Dateien nach `Userletw`). Die Motivsätze der CD bleiben unverändert.

use crate::befehle::{DesignEingabe, DesignWert, MotivsatzInfo};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use stick_kern::motiv::{EigenerSatz, EigenesMotiv};

/// Ablage der eigenen Motivsätze im Kontext.
pub(crate) struct MotivZustand {
    ablage: Mutex<PathBuf>,
}

impl Default for MotivZustand {
    fn default() -> Self {
        let ablage = std::env::var("STICK_EIGENE_MOTIVE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_motive"));
        MotivZustand {
            ablage: Mutex::new(ablage),
        }
    }
}

/// Dateiname eines Satzes: Buchstaben, Ziffern, Leerzeichen, - und _ bleiben, alles andere wird _.
fn dateiname(name: &str) -> String {
    let s: String = name
        .trim()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    format!("{s}.json")
}

fn gleich(a: &str, b: &str) -> bool {
    a.trim_start_matches('_').eq_ignore_ascii_case(b.trim_start_matches('_'))
}

impl Kontext {
    /// Ablage der eigenen Motivsätze setzen (App: `<Datenordner>/motive`).
    pub fn motive_ablage(&self, ordner: PathBuf) {
        if let Ok(mut a) = self.motiv_zustand.ablage.lock() {
            *a = ordner;
        }
    }

    pub(crate) fn eigene_ablage(&self) -> PathBuf {
        self.motiv_zustand
            .ablage
            .lock()
            .map(|a| a.clone())
            .unwrap_or_else(|_| std::env::temp_dir().join("artista_motive"))
    }

    /// Eigene Motivsätze (Name, Datei), nach Namen sortiert.
    pub(crate) fn eigene_saetze(&self) -> Vec<(EigenerSatz, PathBuf)> {
        let mut out: Vec<(EigenerSatz, PathBuf)> = std::fs::read_dir(self.eigene_ablage())
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("json")))
            .filter_map(|p| {
                let s: EigenerSatz = serde_json::from_slice(&std::fs::read(&p).ok()?).ok()?;
                Some((s, p))
            })
            .collect();
        out.sort_by(|a, b| a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()));
        out
    }

    /// Eigener Motivsatz nach Namen (für `motivsatz`).
    pub(crate) fn eigener_satz(&self, name: &str) -> Option<EigenerSatz> {
        self.eigene_saetze().into_iter().map(|(s, _)| s).find(|s| gleich(&s.name, name))
    }

    fn motivsatz_vergessen(&self, name: &str) {
        if let Ok(mut g) = self.motive_geladen.lock() {
            g.retain(|k, _| !gleich(k, name));
        }
    }
}

fn speichern(pfad: &Path, s: &EigenerSatz) -> Result<(), Fehler> {
    if let Some(o) = pfad.parent() {
        std::fs::create_dir_all(o).map_err(|e| format!("Ordner {}: {e}", o.display()))?;
    }
    let text = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    std::fs::write(pfad, text).map_err(|e| Fehler::from(format!("{}: {e}", pfad.display())))
}

/// Eingabe für `motiv_speichern`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotivSpeichernEingabe {
    pub design: DesignWert,
    /// Nummern der Objekte (ab 0), die zusammen das Motiv bilden (Läufe, Satinsäulen, Flächen)
    pub objekte: Vec<usize>,
    /// Name des eigenen Motivsatzes (neu oder vorhanden; nicht der Name eines Satzes der CD)
    pub satz: String,
    /// Name des Motivs im Satz (vorhandenes gleichen Namens wird ersetzt – vorher nachfragen)
    pub name: String,
    /// Bezugspunkte [[x1, y1], [x2, y2]] in mm (Handbuch: „Reference points set the standard orientation … and the
    /// spacing“). Ohne Angabe: linke und rechte Kante der Objekte auf halber Höhe (eigene Festlegung)
    #[serde(default)]
    pub grundlinie: Option<[[f64; 2]; 2]>,
}

/// Ausgewählte Objekte als Motiv in einen eigenen Motivsatz speichern (Settings > Create Pattern).
pub fn motiv_speichern(k: &Kontext, e: MotivSpeichernEingabe) -> Result<MotivsatzInfo, Fehler> {
    let (satz, name) = (e.satz.trim().to_string(), e.name.trim().to_string());
    if satz.is_empty() || name.is_empty() {
        return Err("Namen für Motivsatz und Motiv angeben".into());
    }
    if stick_formate::esb::verzeichnis(&k.motive)
        .iter()
        .any(|(n, p)| gleich(n, &satz) || p.file_stem().is_some_and(|s| gleich(&s.to_string_lossy(), &satz)))
    {
        return Err(format!("„{satz}“ ist ein Motivsatz der CD – bitte einen anderen Namen wählen").into());
    }
    if e.objekte.is_empty() {
        return Err("Keine Objekte gewählt".into());
    }
    let eingabe = DesignEingabe {
        design: e.design,
        rahmen: None,
        stoff: None,
    };
    let d = k.design(&eingabe)?;
    let mut objekte = Vec::new();
    for &nr in &e.objekte {
        let mut v = stick_kern::umwandeln::in_punkte(&d, &eingabe.design.0, nr).map_err(|m| Fehler {
            objekt: Some(nr),
            ..Fehler::from(m)
        })?;
        if let Some(o) = v.as_object_mut() {
            for f in ["einstich", "ausstich", "gruppe", "name"] {
                o.remove(f);
            }
        }
        objekte.push(v);
    }
    // Objekte prüfen und Grundlinie bestimmen (Kasten in 1/180 mm über die Umrisse des umgesetzten Motivs)
    let probe = EigenerSatz {
        name: satz.clone(),
        motive: vec![EigenesMotiv {
            name: name.clone(),
            grundlinie: [[0.0, 0.0], [1.0, 0.0]],
            objekte: objekte.clone(),
        }],
    };
    let z = stick_kern::motiv::eigener_satz(&probe)?;
    let grundlinie = match e.grundlinie {
        Some(g) => g,
        None => {
            let pts: Vec<(f64, f64)> = z.motive[&name]
                .objekte
                .iter()
                .flat_map(|o| o.umrisse.iter())
                .flat_map(stick_kern::schrift::umriss_punkte)
                .collect();
            let e_ = stick_kern::schrift::EINHEIT;
            let (x0, x1) = pts.iter().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.0), b.max(q.0)));
            let (y0, y1) = pts.iter().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.1), b.max(q.1)));
            let r = |v: f64| (v / e_ * 100.0).round() / 100.0;
            [[r(x0), r((y0 + y1) / 2.0)], [r(x1), r((y0 + y1) / 2.0)]]
        }
    };
    let [a, b] = grundlinie;
    if (b[0] - a[0]).hypot(b[1] - a[1]) < 0.3 {
        return Err("Die Bezugspunkte liegen zu nah beieinander (Motiv zu schmal?)".into());
    }
    let (mut s, pfad) = match k.eigene_saetze().into_iter().find(|(s, _)| gleich(&s.name, &satz)) {
        Some(x) => x,
        None => (
            EigenerSatz {
                name: satz.clone(),
                motive: vec![],
            },
            k.eigene_ablage().join(dateiname(&satz)),
        ),
    };
    s.motive.retain(|m| m.name != name);
    s.motive.push(EigenesMotiv {
        name,
        grundlinie,
        objekte,
    });
    speichern(&pfad, &s)?;
    k.motivsatz_vergessen(&s.name);
    Ok(MotivsatzInfo {
        name: s.name,
        datei: pfad.file_name().unwrap_or_default().to_string_lossy().into(),
        eigen: true,
    })
}

/// Eingabe für `motiv_loeschen`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotivLoeschenEingabe {
    /// eigener Motivsatz
    pub satz: String,
    /// Motiv im Satz
    pub motiv: String,
}

/// Motiv aus einem eigenen Motivsatz löschen (Handbuch S. 231 „Deleting patterns“); der letzte löscht den Satz.
/// Motive der CD lassen sich nicht löschen.
pub fn motiv_loeschen(k: &Kontext, e: MotivLoeschenEingabe) -> Result<Vec<MotivsatzInfo>, Fehler> {
    let (mut s, pfad) = k
        .eigene_saetze()
        .into_iter()
        .find(|(s, _)| gleich(&s.name, &e.satz))
        .ok_or_else(|| Fehler::from(format!("„{}“ ist kein eigener Motivsatz (Motive der CD bleiben)", e.satz)))?;
    let vorher = s.motive.len();
    s.motive.retain(|m| m.name != e.motiv);
    if s.motive.len() == vorher {
        return Err(format!("Motiv „{}“ nicht im Satz „{}“", e.motiv, s.name).into());
    }
    if s.motive.is_empty() {
        std::fs::remove_file(&pfad).map_err(|x| Fehler::from(format!("{}: {x}", pfad.display())))?;
    } else {
        speichern(&pfad, &s)?;
    }
    k.motivsatz_vergessen(&s.name);
    crate::befehle::motive_liste(k, crate::befehle::Leer {})
}
