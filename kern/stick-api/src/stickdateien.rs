//! Fremde Stickdateien öffnen (Nachtauftrag N7, ROADMAP 4b.13; Handbuch Kap. 28 „Opening stitch files without object
//! conversion“, `Analyse_Teil3/27_stickdateien.md`): DST, PES/PEC, JEF, VP3, EXP → Design mit einem Objekt `stiche`
//! je Farbe (feste Einstiche, „Grade D“). ART-Designs der artista V4 (N11, Bericht 27 §6): ein Objekt `stiche` je
//! Objekt des Originals, mit Objektart und Garn im Namen und der Garnfarbe.

use crate::befehle::{DesignWert, hex};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;
use stick_formate::stickdatei;
use stick_kern::geometrie::p;

/// Eingabe von `stickdatei_lesen`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StickdateiEingabe {
    /// Stickdatei (Endung .art, .dst, .pes, .pec, .jef, .vp3, .exp)
    pub pfad: String,
    /// Rahmen des neuen Designs (ohne Angabe: der kleinste Rahmen der 185, in den alle Stiche passen)
    #[serde(default)]
    pub rahmen: Option<String>,
}

/// Ergebnis von `stickdatei_lesen`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct StickdateiErgebnis {
    /// neues Design: ein Objekt `stiche` je Farbe (ART: je Objekt des Originals), Mitte des Designs = Rahmenmitte
    pub design: DesignWert,
    /// Format („ART“, „DST“, „PES“ …)
    pub format: String,
    /// Zahl der Einstiche
    pub stiche: usize,
    /// Zahl der Farben (Farbwechsel + 1)
    pub farben: usize,
    /// Zahl der Objekte (Stickdateien: je Farbe eins; ART: Objekte des Originals)
    pub objekte: usize,
    /// Größe in mm
    pub breite: f64,
    pub hoehe: f64,
    /// Hinweise (unbekannte Farbnummer, größer als jeder Rahmen …)
    pub hinweise: Vec<String>,
}

/// Endungen, die `stickdatei_lesen` kennt (für den Dateidialog).
pub fn stickdatei_endungen(_: &Kontext, _: crate::befehle::Leer) -> Result<Vec<String>, Fehler> {
    Ok(stickdatei::ENDUNGEN.iter().map(|e| e.to_string()).collect())
}

pub fn stickdatei_lesen(_: &Kontext, e: StickdateiEingabe) -> Result<StickdateiErgebnis, Fehler> {
    let pfad = Path::new(&e.pfad);
    let endung = pfad.extension().and_then(|x| x.to_str()).unwrap_or_default();
    let daten = std::fs::read(pfad).map_err(|x| Fehler::from(format!("{}: {x}", e.pfad)))?;
    let s = stickdatei::lesen(&daten, endung)?;
    let datei = pfad.file_name().and_then(|x| x.to_str()).unwrap_or_default().to_string();
    let stamm = pfad.file_stem().and_then(|x| x.to_str()).unwrap_or("Stickdatei").to_string();
    let quelle = format!("{datei} ({})", s.format);
    let (x0, y0, x1, y1) = s.grenzen().unwrap_or_default();
    let mut hinweise = s.hinweise.clone();
    let rahmen = match e.rahmen {
        Some(r) => r,
        None => {
            // kleinster Rahmen nach Fläche; der Freiarm-Rahmen nur, wenn er der einzige passende wäre (eigene Festlegung)
            let mut alle = stick_kern::rahmen::alle_rahmen();
            alle.sort_by(|a, b| {
                (a.breite * a.hoehe)
                    .total_cmp(&(b.breite * b.hoehe))
                    .then(a.name.contains("Free Arm").cmp(&b.name.contains("Free Arm")))
            });
            let ecken = [p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)];
            match alle.iter().find(|r| ecken.iter().all(|&q| r.enthaelt(q))) {
                Some(r) => r.name.clone(),
                None => {
                    hinweise.push(format!(
                        "Das Design ({:.1} × {:.1} mm) ist größer als jeder Rahmen der artista 185",
                        x1 - x0,
                        y1 - y0
                    ));
                    alle.last().map(|r| r.name.clone()).unwrap_or_default()
                }
            }
        }
    };
    let einfarbig = s.bloecke.len() == 1;
    let objekte: Vec<Value> = s
        .bloecke
        .iter()
        .enumerate()
        .map(|(k, b)| {
            let name = match (&b.name, einfarbig) {
                // ART: Objektart · Garn (die Nummer zeigt die Objektliste)
                (Some(n), _) if s.format == "ART" => n.clone(),
                (Some(n), _) => format!("{} – {n}", k + 1),
                (None, true) => stamm.clone(),
                (None, false) => format!("Farbe {}", k + 1),
            };
            json!({"art": "stiche", "name": name, "farbe": hex(b.farbe),
                   "stiche": stick_kern::stichdaten::kodieren(&b.zuege), "quelle": quelle})
        })
        .collect();
    Ok(StickdateiErgebnis {
        design: DesignWert(json!({"name": s.name.clone().unwrap_or(stamm), "rahmen": rahmen, "objekte": objekte})),
        format: s.format.to_string(),
        stiche: s.stiche(),
        farben: s.bloecke.windows(2).filter(|w| w[0].farbe != w[1].farbe).count() + usize::from(!s.bloecke.is_empty()),
        objekte: s.bloecke.len(),
        breite: x1 - x0,
        hoehe: y1 - y0,
        hinweise,
    })
}
