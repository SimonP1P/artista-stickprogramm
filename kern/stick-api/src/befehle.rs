//! Die Befehle der Oberfläche mit ihren Ein- und Ausgabetypen. **Neuer Befehl:** Funktion hier schreiben (Eingabe- und
//! Ausgabetyp mit `Deserialize`/`Serialize` + `JsonSchema`, Doc-Kommentare = Beschreibung) und in `lib.rs` in der
//! Liste `befehle!` eintragen – Schema, TypeScript und `SCHNITTSTELLE.md` folgen automatisch (`stick api exportieren`).
//!
//! Objektnummern sind überall **ab 0** (Stelle in `objekte`), wie `objekt` je Stich im Ergebnis.

use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use stick_formate::{esb, exp, garnkarte, ttf};
use stick_kern::design::ObjektForm;
use stick_kern::einstellungen::STOFFE;
use stick_kern::format::DesignJson;
use stick_kern::geometrie::Punkt;
use stick_kern::parameter::Parameter;
use stick_kern::rahmen::{Rahmen, alle_rahmen};
use stick_kern::{Art, Stich, StichFolge};

// ---------------------------------------------------------------- gemeinsame Typen

/// Keine Eingabe.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Leer {}

/// Ein Design im Format `design.schema.json` (als JSON-Objekt, nicht als Text).
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct DesignWert(#[schemars(with = "DesignJson")] pub Value);

/// Ein Design und optional Rahmen/Stoff aus der Oberfläche (haben Vorrang vor den Angaben im Design).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DesignEingabe {
    pub design: DesignWert,
    /// Rahmen (Befehl `rahmen_liste`) statt `design.rahmen`
    #[serde(default)]
    pub rahmen: Option<String>,
    /// Stoff (Befehl `stoffe_liste`) statt `design.stoff`
    #[serde(default)]
    pub stoff: Option<String>,
}

/// Stickrahmen: Stickfeld und Umriss.
#[derive(Debug, Serialize, JsonSchema)]
pub struct RahmenInfo {
    pub name: String,
    /// Stickfeld x in mm
    pub breite: f64,
    /// Stickfeld y in mm
    pub hoehe: f64,
    /// Umriss des Stickfelds [[x, y], …] in mm
    pub form: Vec<Punkt>,
    pub hinweis: String,
}

impl From<&Rahmen> for RahmenInfo {
    fn from(r: &Rahmen) -> Self {
        RahmenInfo {
            name: r.name.clone(),
            breite: r.breite,
            hoehe: r.hoehe,
            form: r.form.clone(),
            hinweis: r.hinweis.clone(),
        }
    }
}

/// Garn einer Garnkarte.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct GarnInfo {
    /// Farbe „#rrggbb“
    pub rgb: String,
    /// Herstellernummer (führende Nullen bleiben)
    pub nummer: String,
    pub marke: String,
    pub name: String,
}

impl From<&garnkarte::Garn> for GarnInfo {
    fn from(g: &garnkarte::Garn) -> Self {
        GarnInfo {
            rgb: format!("#{:02x}{:02x}{:02x}", g.rgb[0], g.rgb[1], g.rgb[2]),
            nummer: g.nummer.clone(),
            marke: g.marke.clone(),
            name: g.name.clone(),
        }
    }
}

// ---------------------------------------------------------------- Listen

pub fn rahmen_liste(_: &Kontext, _: Leer) -> Result<Vec<RahmenInfo>, Fehler> {
    Ok(alle_rahmen().iter().map(RahmenInfo::from).collect())
}

/// Stoff mit den Werten, die er ändert (alle Werte je Stoff: Parameterkatalog).
#[derive(Debug, Serialize, JsonSchema)]
pub struct StoffInfo {
    pub name: String,
    /// fester Satinabstand in mm
    pub satin_abstand_fest: f64,
    /// Tatami-Abstand in mm
    pub tatami_abstand: f64,
    /// Auto-Spacing-Faktor
    pub auto_spacing_faktor: f64,
    /// Zugausgleich in mm
    pub zugausgleich: f64,
}

pub fn stoffe_liste(_: &Kontext, _: Leer) -> Result<Vec<StoffInfo>, Fehler> {
    Ok(STOFFE
        .iter()
        .map(|s| StoffInfo {
            name: s.name.into(),
            satin_abstand_fest: s.satin_abstand_fest,
            tatami_abstand: s.tatami_abstand,
            auto_spacing_faktor: s.auto_spacing_faktor,
            zugausgleich: s.zugausgleich,
        })
        .collect())
}

/// Original-Alphabet (ESB-Datei).
#[derive(Debug, Serialize, JsonSchema)]
pub struct AlphabetInfo {
    /// Name wie im Design-Feld `alphabet`
    pub name: String,
    pub datei: String,
    /// kleinste empfohlene Höhe in mm (Handbuch „Alphabet Samples“; fehlt, wenn das Handbuch keine nennt)
    pub min_hoehe: Option<f64>,
    /// größte empfohlene Höhe in mm (Handbuch)
    pub max_hoehe: Option<f64>,
}

pub fn alphabete_liste(k: &Kontext, _: Leer) -> Result<Vec<AlphabetInfo>, Fehler> {
    Ok(esb::verzeichnis(&k.alphabete)
        .into_iter()
        .filter(|(n, _)| n != "ButtonHoles")
        .map(|(name, p)| {
            let h = stick_kern::schrift::empfohlene_hoehe(&name);
            AlphabetInfo {
                datei: p.file_name().unwrap_or_default().to_string_lossy().into(),
                min_hoehe: h.map(|x| x.0),
                max_hoehe: h.map(|x| x.1),
                name,
            }
        })
        .collect())
}

/// Alphabet und Probetext für die Vorschau.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AlphabetVorschauEingabe {
    /// Name wie im Design-Feld `alphabet`
    pub alphabet: String,
    /// Probetext (Standard „Abc“)
    #[serde(default)]
    pub text: Option<String>,
    /// Höhe in mm (Standard 10)
    #[serde(default)]
    pub hoehe: Option<f64>,
}

/// Umrisse eines Probetexts und der Zeichenvorrat des Alphabets.
#[derive(Debug, Serialize, JsonSchema)]
pub struct AlphabetVorschau {
    /// alle Zeichen, die das Alphabet hat
    pub zeichen: String,
    /// Linien des Probetexts (Umrisse, Satinseiten …) in mm, wie `design_formen`
    pub linien: Vec<stick_kern::design::FormLinie>,
    /// umschließendes Rechteck [x0, y0, x1, y1] in mm
    pub begrenzung: Option<[f64; 4]>,
}

/// Vorschau eines Original-Alphabets: Umrisse des Probetexts (aus der ESB-Datei) und Zeichenvorrat.
pub fn alphabet_vorschau(k: &Kontext, e: AlphabetVorschauEingabe) -> Result<AlphabetVorschau, Fehler> {
    let text = e.text.unwrap_or_else(|| "Abc".into());
    let design = serde_json::json!({
        "garnkarte": "keine",
        "objekte": [{"art": "text", "alphabet": e.alphabet, "text": text, "hoehe": e.hoehe.unwrap_or(10.0)}]
    });
    let d = k.design(&DesignEingabe {
        design: DesignWert(design),
        rahmen: Some("artista 400 x 150 MEGA".into()),
        stoff: None,
    })?;
    let zeichen = d
        .alphabete
        .values()
        .next()
        .map(|a| a.zeichen.keys().collect())
        .unwrap_or_default();
    let f = d.formen()?.into_iter().next();
    Ok(AlphabetVorschau {
        zeichen,
        begrenzung: f
            .as_ref()
            .and_then(|f| f.begrenzung.as_ref())
            .and_then(|b| b.as_slice().try_into().ok()),
        linien: f.map(|f| f.linien).unwrap_or_default(),
    })
}

pub fn garnkarten_liste(k: &Kontext, _: Leer) -> Result<Vec<String>, Fehler> {
    Ok(garnkarte::verzeichnis(&k.garnkarten)
        .into_iter()
        .map(|(n, _)| n)
        .collect())
}

/// Name einer Garnkarte.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GarnkartenName {
    pub name: String,
}

pub fn garnkarte_garne(k: &Kontext, e: GarnkartenName) -> Result<Vec<GarnInfo>, Fehler> {
    let karte = garnkarte::laden(&k.garnkarten, &e.name)?;
    Ok(karte.garne.iter().map(GarnInfo::from).collect())
}

/// Füllmuster mit den Linien einer Kachel (für Vorschau-Kacheln).
#[derive(Debug, Serialize, JsonSchema)]
pub struct MusterInfo {
    /// Name wie im Design-Feld `muster` („001 Diamond (90)“)
    pub name: String,
    /// Nummer („001“) – auch als `muster` erlaubt
    pub nummer: String,
    /// Stichwinkel aus dem Namen in Grad
    pub winkel: Option<f64>,
    /// Gittervektor A in mm (das Muster wiederholt sich um i·A + j·B)
    pub a: [f64; 2],
    /// Gittervektor B in mm
    pub b: [f64; 2],
    /// Linienzüge einer Kachel in mm
    pub zuege: Vec<Vec<[f64; 2]>>,
}

pub fn fuellmuster_liste(_: &Kontext, _: Leer) -> Result<Vec<MusterInfo>, Fehler> {
    Ok(stick_kern::muster::alle()
        .iter()
        .map(|m| MusterInfo {
            name: m.name.clone(),
            nummer: m.name.split(' ').next().unwrap_or_default().into(),
            winkel: m.winkel,
            a: m.a,
            b: m.b,
            zuege: m.zuege.clone(),
        })
        .collect())
}

/// Computer-Schrift (TrueType/OpenType).
#[derive(Debug, Serialize, JsonSchema)]
pub struct SchriftInfo {
    /// Name wie im Design-Feld `schrift` („Arial Bold“)
    pub name: String,
    pub familie: String,
    pub stil: String,
    pub pfad: String,
}

pub fn schriften_liste(k: &Kontext, _: Leer) -> Result<Vec<SchriftInfo>, Fehler> {
    let ordner = if k.schriften.is_empty() {
        ttf::system_ordner()
    } else {
        k.schriften.clone()
    };
    Ok(ttf::verzeichnis(&ordner)
        .into_iter()
        .map(|e| SchriftInfo {
            name: e.name,
            familie: e.familie,
            stil: e.stil,
            pfad: e.pfad.display().to_string(),
        })
        .collect())
}

/// Step-Muster des Dialogs „Fill Stitch“ mit den Einstichen einer Probefläche.
#[derive(Debug, Serialize, JsonSchema)]
pub struct StepMusterInfo {
    /// Nummer 1–20 (Design-Feld `step_muster`)
    pub nr: usize,
    /// Reihenversatz in Teilen der Stichlänge (AMT 0x2719)
    pub versatz: f64,
    /// zweiter Reihenversatz, abwechselnd (AMT 0x271a)
    pub versatz2: f64,
    /// Einstiche einer Tatami-Probefläche 12 × 8 mm (Software Defaults, Winkel 0°), in mm – für Vorschau-Kacheln
    pub einstiche: Vec<Punkt>,
}

/// Step-Muster 1–20 (Tabelle aus DIALOG.DLL) mit Einstichen einer Probefläche, berechnet vom Kern.
pub fn step_muster_liste(_: &Kontext, _: Leer) -> Result<Vec<StepMusterInfo>, Fehler> {
    use stick_kern::geometrie::p;
    let umriss = vec![p(0.0, 0.0), p(12.0, 0.0), p(12.0, 8.0), p(0.0, 8.0)];
    let basis = stick_kern::einstellungen::fuer_stoff("0 Software Defaults")?.tatami;
    stick_kern::einstellungen::STEP_MUSTER
        .iter()
        .enumerate()
        .map(|(i, &(v1, v2))| {
            let mut t = basis.clone();
            t.step_muster(i + 1)?;
            let wege = stick_kern::sticharten::tatami::tatami(&umriss, &[], &t, Some(0.0), None);
            Ok(StepMusterInfo {
                nr: i + 1,
                versatz: v1,
                versatz2: v2,
                einstiche: wege.into_iter().flatten().collect(),
            })
        })
        .collect::<Result<_, String>>()
        .map_err(Fehler::from)
}

pub fn parameter_katalog(_: &Kontext, _: Leer) -> Result<Vec<Parameter>, Fehler> {
    Ok(stick_kern::parameter::katalog())
}

/// Beispieldesign mit allen Sticharten der ersten Ausbaustufe.
const BEISPIEL: &str = include_str!("../../../stickprogramm/beispiele/beispiel.json");

pub fn beispiel_design(_: &Kontext, _: Leer) -> Result<DesignWert, Fehler> {
    serde_json::from_str(BEISPIEL)
        .map(DesignWert)
        .map_err(|e| Fehler::from(e.to_string()))
}

/// JSON-Schema des Design-Formats (dasselbe wie `kern/schnittstelle/design.schema.json`).
pub fn design_schema(_: &Kontext, _: Leer) -> Result<Value, Fehler> {
    Ok(stick_kern::format::schema())
}

// ---------------------------------------------------------------- Design

/// Art eines Hinweises.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HinweisArt {
    /// Feld, das der Kern nicht liest (Tippfehler?)
    Feld,
    /// Zeichen, die das Alphabet bzw. die Schrift nicht hat
    FehlendeZeichen,
    /// Einstiche außerhalb des Stickfelds
    Ausserhalb,
}

/// Hinweis (kein Fehler – das Design lässt sich berechnen) mit der Nummer des Objekts, wenn er zu einem gehört.
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct Hinweis {
    pub meldung: String,
    /// Nummer des Objekts in `objekte` (ab 0), wenn bekannt
    pub objekt: Option<usize>,
    pub art: HinweisArt,
}

fn feld_hinweise(design: &Value) -> Vec<Hinweis> {
    crate::erzeugen::unbekannte_felder(design)
        .into_iter()
        .map(|m| {
            let f = Fehler::from(m);
            Hinweis {
                meldung: f.meldung,
                objekt: f.objekt,
                art: HinweisArt::Feld,
            }
        })
        .collect()
}

/// Ergebnis der Prüfung eines Designs (ohne Stiche zu berechnen).
#[derive(Debug, Serialize, JsonSchema)]
pub struct Pruefung {
    /// Fehler, der das Berechnen verhindert (None = in Ordnung)
    pub fehler: Option<Fehler>,
    /// Hinweise, z. B. Felder, die der Kern nicht liest (Tippfehler?)
    pub hinweise: Vec<Hinweis>,
}

pub fn design_pruefen(k: &Kontext, e: DesignEingabe) -> Result<Pruefung, Fehler> {
    let hinweise = feld_hinweise(&e.design.0);
    Ok(Pruefung {
        fehler: k.design(&e).err(),
        hinweise,
    })
}

/// Kennzahlen der Stichfolge.
#[derive(Debug, Serialize, JsonSchema)]
pub struct Statistik {
    pub stiche: usize,
    pub spruenge: usize,
    pub farbwechsel: usize,
    /// Breite der Stiche in mm
    pub breite: f64,
    /// Höhe der Stiche in mm
    pub hoehe: f64,
    /// kürzester Stich in mm
    pub min_stich: f64,
    /// längster Stich in mm
    pub max_stich: f64,
}

pub(crate) fn statistik_von(folge: &StichFolge) -> Statistik {
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

/// Grenze für die Daten eines Designs in Byte: Kapazität des Maschinenspeichers, an der artista 185 gelesen
/// (Verwaltungsblock +0x1A = 145 426; Original 0x100044ad nimmt das Maximum, `Analyse_Teil2/07`, hoch). Beim Senden
/// prüft `maschine_planen` gegen den Wert der angeschlossenen Maschine.
pub const DATEN_GRENZE: usize = 145_426;

/// Ein Farbblock (Abschnitt zwischen zwei Farbwechseln = ein Stopp an der Maschine).
#[derive(Debug, Serialize, JsonSchema)]
pub struct FarbBlock {
    /// Nummer des Objekts (ab 0), mit dem der Block beginnt
    pub objekt: usize,
    /// alle Objekte (ab 0), die in diesem Block Stiche haben
    pub objekte: Vec<usize>,
    /// Garn von Hand gewählt (Feld `garn`) statt nächstes Garn der Karte
    pub von_hand: bool,
    /// Applikation: Lagen in diesem Block (Platzierung, Schnittlinie, Heften, Abdecken) – für die Anleitung
    /// („nach diesem Block: Stoff auflegen / zuschneiden“); leer bei anderen Objekten
    pub applikation: Vec<stick_kern::applikation::Lage>,
}

/// Berechnetes Design: Stiche, Farben, Garne, Kennzahlen, Prüfungen.
#[derive(Debug, Serialize, JsonSchema)]
pub struct Ergebnis {
    pub name: String,
    pub rahmen: RahmenInfo,
    pub stoff: String,
    /// alle Stiche in Stickfolge
    pub stiche: Vec<Stich>,
    /// je Stich die Nummer des Objekts (ab 0) – zum Auswählen und Hervorheben
    pub objekt: Vec<usize>,
    /// Farbe je Farbblock „#rrggbb“ (Block i beginnt nach dem i-ten Farbwechsel)
    pub farben: Vec<String>,
    /// Garn je Farbblock aus der Garnkarte des Designs (leer bei „keine“); Feld `garn` am Objekt hat Vorrang
    pub garne: Vec<GarnInfo>,
    /// je Farbblock: Objekte, Garn von Hand, Lagen einer Applikation
    pub bloecke: Vec<FarbBlock>,
    pub statistik: Statistik,
    /// umschließendes Rechteck der Stiche [x0, y0, x1, y1] in mm (für Einpassen und Mittig); fehlt ohne Stiche
    pub begrenzung: Option<[f64; 4]>,
    /// Zahl der Einstiche außerhalb des Stickfelds (muss 0 sein zum Senden)
    pub ausserhalb: usize,
    /// Nummern der Stiche (in `stiche`) außerhalb des Stickfelds – zum Markieren
    pub ausserhalb_stiche: Vec<usize>,
    /// Zeichen eines Texts, die das Alphabet nicht hat
    pub fehlende_zeichen: String,
    /// Datengröße für die Maschine in Byte (Stichdaten + Anhang wie beim Senden)
    pub daten_bytes: usize,
    /// Grenze der Datengröße in Byte (Kapazität der artista 185, `DATEN_GRENZE`)
    pub daten_grenze: usize,
    /// Hinweise mit Objektnummer (Felder, die der Kern nicht liest; fehlende Zeichen; außerhalb des Stickfelds)
    pub hinweise: Vec<Hinweis>,
}

/// Objekte je Farbblock (Block i beginnt nach dem i-ten Farbwechsel), in der Reihenfolge ihres ersten Stichs.
fn objekte_je_block(folge: &StichFolge) -> Vec<Vec<usize>> {
    let mut out = vec![Vec::new(); folge.farben.len().max(1)];
    let mut block = 0;
    for (i, s) in folge.stiche.iter().enumerate() {
        if s.art == Art::Farbwechsel {
            block += 1;
            continue;
        }
        if let (Some(&o), Some(b)) = (folge.objekt.get(i), out.get_mut(block))
            && !b.contains(&o)
        {
            b.push(o);
        }
    }
    out.truncate(folge.farben.len());
    out
}

/// Farbblöcke mit Applikationslagen: je Applikation die Lagen der Reihe nach den Blöcken ihrer Farbe zuordnen.
pub(crate) fn bloecke_von(design: &stick_kern::Design, folge: &StichFolge) -> Vec<FarbBlock> {
    use std::collections::HashMap;
    let mut offen: HashMap<usize, std::collections::VecDeque<(String, stick_kern::applikation::Lage)>> = HashMap::new();
    objekte_je_block(folge)
        .into_iter()
        .enumerate()
        .map(|(b, objekte)| {
            let farbe = folge.farben.get(b).map(|f| f.to_ascii_lowercase()).unwrap_or_default();
            let mut applikation = Vec::new();
            for &o in &objekte {
                if let Some(stick_kern::design::ObjektArt::Applikation(v)) = design.objekte.get(o).map(|x| &x.art) {
                    let q = offen
                        .entry(o)
                        .or_insert_with(|| stick_kern::applikation::lagen(v).into());
                    while q.front().is_some_and(|(f, _)| *f == farbe) {
                        if let Some((_, l)) = q.pop_front() {
                            applikation.push(l);
                        }
                    }
                }
            }
            FarbBlock {
                objekt: objekte.first().copied().unwrap_or(0),
                von_hand: objekte.first().is_some_and(|o| design.garn.contains_key(o)),
                objekte,
                applikation,
            }
        })
        .collect()
}

pub fn design_berechnen(k: &Kontext, e: DesignEingabe) -> Result<Ergebnis, Fehler> {
    let design = k.design(&e)?;
    let fehlend = design.fehlende_zeichen()?;
    let folge = design.berechnen()?;
    let bloecke = bloecke_von(&design, &folge);
    let garne = if design.garnkarte == "keine" {
        Vec::new()
    } else {
        let karte = garnkarte::laden(&k.garnkarten, &design.garnkarte)?;
        folge
            .farben
            .iter()
            .zip(&bloecke)
            .map(|(hex, b)| match design.garn.get(&b.objekt) {
                Some(nr) => karte
                    .garne
                    .iter()
                    .find(|g| g.nummer == *nr)
                    .map(GarnInfo::from)
                    .ok_or_else(|| Fehler {
                        meldung: format!(
                            "Objekt {}: Garn „{nr}“ gibt es in der Garnkarte „{}“ nicht",
                            b.objekt + 1,
                            design.garnkarte
                        ),
                        objekt: Some(b.objekt),
                        code: None,
                    }),
                None => Ok(GarnInfo::from(
                    karte.naechstes(stick_formate::artista::Farbe::aus_hex(hex).rgb),
                )),
            })
            .collect::<Result<_, _>>()?
    };
    let ausserhalb_stiche: Vec<usize> = folge
        .stiche
        .iter()
        .enumerate()
        .filter(|(_, s)| !design.rahmen.enthaelt(s.p()))
        .map(|(i, _)| i)
        .collect();
    let mut hinweise = feld_hinweise(&e.design.0);
    for (o, z) in &fehlend {
        hinweise.push(Hinweis {
            meldung: format!("Objekt {}: Zeichen „{z}“ fehlen im Alphabet", o + 1),
            objekt: Some(*o),
            art: HinweisArt::FehlendeZeichen,
        });
    }
    let mut aussen_je_objekt: Vec<(usize, usize)> = Vec::new();
    for &i in &ausserhalb_stiche {
        let o = folge.objekt.get(i).copied().unwrap_or(0);
        match aussen_je_objekt.iter_mut().find(|(x, _)| *x == o) {
            Some((_, n)) => *n += 1,
            None => aussen_je_objekt.push((o, 1)),
        }
    }
    for (o, n) in aussen_je_objekt {
        hinweise.push(Hinweis {
            meldung: format!("Objekt {}: {n} Einstiche außerhalb des Stickfelds", o + 1),
            objekt: Some(o),
            art: HinweisArt::Ausserhalb,
        });
    }
    let daten = stick_formate::artista::Sendedaten::eigenes_design(&design.name, &folge, false);
    let mut fehlende_zeichen = String::new();
    for (_, z) in &fehlend {
        for c in z.chars() {
            if !fehlende_zeichen.contains(c) {
                fehlende_zeichen.push(c);
            }
        }
    }
    Ok(Ergebnis {
        name: design.name.clone(),
        rahmen: RahmenInfo::from(&design.rahmen),
        stoff: design.einstellungen.stoff.clone(),
        ausserhalb: ausserhalb_stiche.len(),
        ausserhalb_stiche,
        fehlende_zeichen,
        daten_bytes: daten.puffer.stiche.len() + daten.puffer.anhang.len() + 2,
        daten_grenze: DATEN_GRENZE,
        hinweise,
        statistik: statistik_von(&folge),
        begrenzung: folge.begrenzung().map(|(a, b, c, d)| [a, b, c, d]),
        garne,
        bloecke,
        objekt: folge.objekt,
        farben: folge.farben,
        stiche: folge.stiche,
    })
}

pub fn design_formen(k: &Kontext, e: DesignEingabe) -> Result<Vec<ObjektForm>, Fehler> {
    Ok(k.design(&e)?.formen()?)
}

/// Design und Zieldatei.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExpEingabe {
    pub design: DesignWert,
    #[serde(default)]
    pub rahmen: Option<String>,
    #[serde(default)]
    pub stoff: Option<String>,
    /// Zieldatei (.exp)
    pub pfad: String,
}

/// Stiche als EXP-Datei speichern; Ergebnis = Zahl der Bytes.
pub fn exp_speichern(k: &Kontext, e: ExpEingabe) -> Result<usize, Fehler> {
    let d = k.design(&DesignEingabe {
        design: e.design,
        rahmen: e.rahmen,
        stoff: e.stoff,
    })?;
    let daten = exp::exp_bytes(&d.berechnen()?);
    std::fs::write(&e.pfad, &daten).map_err(|x| Fehler::from(format!("{}: {x}", e.pfad)))?;
    Ok(daten.len())
}

// ---------------------------------------------------------------- Reihenfolge

/// Umordnen wie Menü „Arrange“ des Originals (Objektnummern ab 0).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "art", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReihenfolgeBefehl {
    /// „Sequence As Selected“: Auswahl in Klickfolge an die Stelle des ersten ausgewählten Objekts
    WieAusgewaehlt { auswahl: Vec<usize> },
    /// „Sequence to Start“
    AnAnfang { auswahl: Vec<usize> },
    /// „Sequence to End“
    AnsEnde { auswahl: Vec<usize> },
    /// „Resequence by Number“: Auswahl vor das Objekt `ziel`
    Vor { auswahl: Vec<usize>, ziel: usize },
    /// „Resequence by Number“: Auswahl nach das Objekt `ziel`
    Nach { auswahl: Vec<usize>, ziel: usize },
    /// „Sequence By Color“: Auswahl (leer = alle) nach Farben; `farben` = gewünschte Farbfolge; `ueberdeckung` (eigene
    /// Erweiterung): Objekte, die sich überdecken, behalten ihre Reihenfolge
    NachFarbe {
        #[serde(default)]
        auswahl: Vec<usize>,
        #[serde(default)]
        farben: Vec<String>,
        #[serde(default)]
        ueberdeckung: bool,
    },
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReihenfolgeEingabe {
    pub design: DesignWert,
    pub befehl: ReihenfolgeBefehl,
}

/// Ergebnis des Umordnens.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ReihenfolgeErgebnis {
    /// Design mit neuer Reihenfolge der `objekte`
    pub design: DesignWert,
    /// an Stelle i steht das bisherige Objekt `reihenfolge[i]`
    pub reihenfolge: Vec<usize>,
    pub farbwechsel_vorher: usize,
    pub farbwechsel_nachher: usize,
    /// Warnungen: [oben, unten] (bisherige Nummern) – `oben` lag über `unten` und liegt jetzt darunter
    pub vertauscht: Vec<[usize; 2]>,
}

pub fn reihenfolge_aendern(k: &Kontext, e: ReihenfolgeEingabe) -> Result<ReihenfolgeErgebnis, Fehler> {
    use stick_kern::reihenfolge::{Befehl, ausfuehren};
    let befehl = match e.befehl {
        ReihenfolgeBefehl::WieAusgewaehlt { auswahl } => Befehl::WieAusgewaehlt(auswahl),
        ReihenfolgeBefehl::AnAnfang { auswahl } => Befehl::AnAnfang(auswahl),
        ReihenfolgeBefehl::AnsEnde { auswahl } => Befehl::AnsEnde(auswahl),
        ReihenfolgeBefehl::Vor { auswahl, ziel } => Befehl::NachNummer {
            auswahl,
            ziel,
            vor: true,
        },
        ReihenfolgeBefehl::Nach { auswahl, ziel } => Befehl::NachNummer {
            auswahl,
            ziel,
            vor: false,
        },
        ReihenfolgeBefehl::NachFarbe {
            auswahl,
            farben,
            ueberdeckung,
        } => Befehl::NachFarbe {
            auswahl,
            farben,
            ueberdeckung,
        },
    };
    // für die Überdeckungsprüfung muss das Design berechenbar sein; sonst ohne Prüfung
    let eingabe = DesignEingabe {
        design: e.design,
        rahmen: None,
        stoff: None,
    };
    let design = k.design(&eingabe).ok();
    let r = ausfuehren(&eingabe.design.0, design.as_ref(), &befehl)?;
    Ok(ReihenfolgeErgebnis {
        design: DesignWert(r.design),
        reihenfolge: r.reihenfolge,
        farbwechsel_vorher: r.farbwechsel_vorher,
        farbwechsel_nachher: r.farbwechsel_nachher,
        vertauscht: r.vertauscht.into_iter().map(|(a, b)| [a, b]).collect(),
    })
}

/// Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken (für die Anzeige).
pub fn ueberdeckungen(k: &Kontext, e: DesignEingabe) -> Result<Vec<[usize; 2]>, Fehler> {
    let d = k.design(&e)?;
    Ok(stick_kern::reihenfolge::ueberdeckungen(&d)?
        .into_iter()
        .map(|(a, b)| [a, b])
        .collect())
}

// ---------------------------------------------------------------- Werkzeuge

/// Fläche, die in Satinsäulen zerlegt werden soll.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaeulenEingabe {
    /// Umriss [[x, y], …] in mm
    pub umriss: Vec<Punkt>,
    #[serde(default)]
    pub loecher: Vec<Vec<Punkt>>,
    /// Säulen an Knicken teilen (Innenwinkel in Grad, 0 = nie)
    #[serde(default)]
    pub knickwinkel: Option<f64>,
}

/// Eine Satinsäule der Zerlegung.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SaeuleInfo {
    pub seite_a: Vec<Punkt>,
    pub seite_b: Vec<Punkt>,
    /// Stichlinien (Richtung der Stiche)
    pub linien: Vec<[Punkt; 2]>,
    pub umriss: Vec<Punkt>,
}

/// Fläche in Satinsäulen zerlegen wie `satinflaeche` (Vorschau der Stichrichtung).
pub fn saeulen_vorschau(_: &Kontext, e: SaeulenEingabe) -> Result<Vec<SaeuleInfo>, Fehler> {
    let mut werte = stick_kern::saeulen::SaeulenWerte::default();
    if let Some(k) = e.knickwinkel {
        werte.knickwinkel = k;
    }
    Ok(stick_kern::saeulen::zerlegen(&e.umriss, &e.loecher, &werte)?
        .into_iter()
        .map(|s| SaeuleInfo {
            seite_a: s.seite_a,
            seite_b: s.seite_b,
            linien: s.linien,
            umriss: s.umriss,
        })
        .collect())
}

/// Bild in Stickflächen umwandeln (eigene Festlegung, Auto Digitizer des Originals nicht nachgebaut).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BildEingabe {
    /// Bilddatei (PNG, JPG, BMP …)
    pub pfad: String,
    /// Breite des Ergebnisses in mm (Standard 80)
    #[serde(default)]
    pub breite: Option<f64>,
    /// Zahl der Farben (Standard 6)
    #[serde(default)]
    pub farben: Option<usize>,
    /// kleinste Fläche in mm² (Standard 4)
    #[serde(default)]
    pub min_flaeche: Option<f64>,
    /// Glättung in mm (Standard 0,2)
    #[serde(default)]
    pub glaettung: Option<f64>,
    /// Hintergrund mitsticken
    #[serde(default)]
    pub mit_hintergrund: bool,
    /// Rahmen des neuen Designs
    #[serde(default)]
    pub rahmen: Option<String>,
}

/// Ergebnis der Bildumwandlung.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BildErgebnis {
    /// neues Design (Tatami-Flächen nach Farben)
    pub design: DesignWert,
    pub flaechen: usize,
    pub farben: usize,
    /// Bildgröße in Punkten
    pub breite_px: u32,
    pub hoehe_px: u32,
}

pub fn bild_umwandeln(_: &Kontext, e: BildEingabe) -> Result<BildErgebnis, Fehler> {
    let st = stick_vektor::BildOptionen::default();
    let o = stick_vektor::BildOptionen {
        breite_mm: e.breite.unwrap_or(st.breite_mm),
        farben: e.farben.unwrap_or(st.farben),
        min_flaeche: e.min_flaeche.unwrap_or(st.min_flaeche),
        glaettung: e.glaettung.unwrap_or(st.glaettung),
        ohne_hintergrund: !e.mit_hintergrund,
    };
    let pfad = PathBuf::from(&e.pfad);
    let bild = stick_vektor::bild_laden(&pfad)?;
    let f = stick_vektor::ordnen(stick_vektor::vektorisieren(&bild, &o));
    let stamm = pfad.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let rahmen = e.rahmen.unwrap_or_else(|| "artista 130 x 100".into());
    let mut farben: Vec<[u8; 3]> = f.iter().map(|x| x.farbe).collect();
    farben.sort();
    farben.dedup();
    Ok(BildErgebnis {
        design: DesignWert(stick_vektor::design_json(&f, &stamm, &rahmen)),
        flaechen: f.len(),
        farben: farben.len(),
        breite_px: bild.width(),
        hoehe_px: bild.height(),
    })
}

fn ja() -> bool {
    true
}

/// SVG-Datei einlesen (in Inkscape zeichnen → laden → sticken).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SvgEingabe {
    /// SVG-Datei (Text in Inkscape vorher in Pfade umwandeln)
    pub pfad: String,
    /// Breite des Ergebnisses in mm (ohne Angabe: Größe aus der SVG, 1 px = 25,4/96 mm)
    #[serde(default)]
    pub breite: Option<f64>,
    /// Rahmen des neuen Designs
    #[serde(default)]
    pub rahmen: Option<String>,
    /// Füllflächen sparen aus, was später gezeichnet ist (wie die SVG aussieht)
    #[serde(default = "ja")]
    pub aussparen: bool,
    /// Closest Join wie der Auto Digitizer
    #[serde(default = "ja")]
    pub kuerzeste_verbindung: bool,
}

/// Ergebnis des SVG-Imports.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SvgErgebnis {
    /// neues Design
    pub design: DesignWert,
    /// Größe der Zeichnung in mm
    pub breite: f64,
    pub hoehe: f64,
    /// Zahl der Objekte je Stichart (Namen wie die EasyStitch-Stile des Originals)
    pub klassen: std::collections::BTreeMap<String, usize>,
    /// Hinweise (Text, Bilder, Verläufe, zu breite Linien, größer als der Rahmen …)
    pub hinweise: Vec<String>,
}

/// SVG → Design: Flächen und Konturen, Stichart nach Breite wie EasyStitch, Zeichenreihenfolge, Überdeckung.
pub fn svg_lesen(_: &Kontext, e: SvgEingabe) -> Result<SvgErgebnis, Fehler> {
    let st = stick_vektor::svg::SvgOptionen::default();
    let o = stick_vektor::svg::SvgOptionen {
        breite_mm: e.breite,
        aussparen: e.aussparen,
        kuerzeste_verbindung: e.kuerzeste_verbindung,
        rahmen: e.rahmen.unwrap_or(st.rahmen),
    };
    let r = stick_vektor::svg::svg_datei(std::path::Path::new(&e.pfad), &o)?;
    Ok(SvgErgebnis {
        design: DesignWert(r.design),
        breite: r.breite_mm,
        hoehe: r.hoehe_mm,
        klassen: r.klassen,
        hinweise: r.hinweise,
    })
}

// ---------------------------------------------------------------- Dateien

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Pfad {
    pub pfad: String,
}

/// Textdatei lesen (z. B. Design-JSON).
pub fn datei_lesen(_: &Kontext, e: Pfad) -> Result<String, Fehler> {
    std::fs::read_to_string(&e.pfad).map_err(|x| Fehler::from(format!("{}: {x}", e.pfad)))
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextDatei {
    pub pfad: String,
    pub text: String,
}

/// Textdatei schreiben (z. B. Design-JSON speichern).
pub fn text_speichern(_: &Kontext, e: TextDatei) -> Result<(), Fehler> {
    std::fs::write(&e.pfad, e.text).map_err(|x| Fehler::from(format!("{}: {x}", e.pfad)))
}
