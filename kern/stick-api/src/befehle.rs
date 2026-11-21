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
use stick_kern::format::{DesignJson, ObjektJson};
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

/// Motivsatz (Pattern Set, Ordner `userletw/` der CD; Handbuch Kap. 25).
#[derive(Debug, Serialize, JsonSchema)]
pub struct MotivsatzInfo {
    /// Name wie im Design-Feld `satz` der Motivlinie
    pub name: String,
    pub datei: String,
}

pub fn motive_liste(k: &Kontext, _: Leer) -> Result<Vec<MotivsatzInfo>, Fehler> {
    Ok(esb::verzeichnis(&k.motive)
        .into_iter()
        .map(|(name, p)| MotivsatzInfo {
            datei: p.file_name().unwrap_or_default().to_string_lossy().into(),
            name,
        })
        .collect())
}

/// Name eines Motivsatzes.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotivsatzName {
    /// Name wie in `motive_liste`
    pub satz: String,
}

/// Ein Motiv mit Linien für die Vorschau (mm, Grundlinie von [0, 0] nach [laenge, 0], +y nach oben).
#[derive(Debug, Serialize, JsonSchema)]
pub struct MotivInfo {
    /// Name wie im Design-Feld `motiv`
    pub name: String,
    /// Länge der Grundlinie = Größe im Motivsatz (mm)
    pub laenge: f64,
    /// Höhe quer zur Grundlinie (mm)
    pub hoehe: f64,
    pub linien: Vec<Vec<Punkt>>,
}

pub fn motivsatz_motive(k: &Kontext, e: MotivsatzName) -> Result<Vec<MotivInfo>, Fehler> {
    let s = k.motivsatz(&e.satz)?;
    let r = |v: f64| (v * 100.0).round() / 100.0;
    Ok(s.motive
        .iter()
        .map(|(name, z)| {
            let (laenge, hoehe) = stick_kern::motiv::motiv_masse(z);
            MotivInfo {
                name: name.clone(),
                laenge: r(laenge),
                hoehe: r(hoehe),
                linien: stick_kern::motiv::motiv_linien(z)
                    .into_iter()
                    .map(|l| l.into_iter().map(|q| stick_kern::geometrie::p(r(q.x), r(q.y))).collect())
                    .collect(),
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
    /// je Stich die Nummer des Buchstabens bei Text/Monogramm (1 = erster ohne Leerraum und `^`, wie
    /// `buchstaben[].nr`), 0 = kein Buchstabe – zum Anklicken einzelner Buchstaben
    pub buchstabe: Vec<usize>,
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
    let (folge, buchstabe) = design.berechnen_mit_buchstaben()?;
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
        buchstabe,
        objekt: folge.objekt,
        farben: folge.farben,
        stiche: folge.stiche,
    })
}

pub fn design_formen(k: &Kontext, e: DesignEingabe) -> Result<Vec<ObjektForm>, Fehler> {
    Ok(k.design(&e)?.formen()?)
}

/// Ein Objekt im Format `design.schema.json` (Eintrag in `objekte`).
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ObjektWert(#[schemars(with = "ObjektJson")] pub Value);

/// Ein Objekt eines Designs.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ObjektEingabe {
    pub design: DesignWert,
    /// Nummer des Objekts (ab 0)
    pub objekt: usize,
}

/// Objekt in Knoten umwandeln (Knoten-Werkzeug): Kreis, Oval, Bogen → Bézierstücke, Rechteck → Eckpunkte,
/// `transform` auf die Knoten angewandt (Stichwinkel und Satinbreiten mitgeführt). Ergebnis ersetzt das Objekt;
/// Stiche danach wie gewohnt neu berechnen. Text, Monogramm, Knopfloch: Fehler (keine Knoten).
pub fn in_knoten_umwandeln(k: &Kontext, e: ObjektEingabe) -> Result<ObjektWert, Fehler> {
    let eingabe = DesignEingabe {
        design: e.design,
        rahmen: None,
        stoff: None,
    };
    let d = k.design(&eingabe)?;
    stick_kern::umwandeln::in_punkte(&d, &eingabe.design.0, e.objekt)
        .map(ObjektWert)
        .map_err(|m| Fehler {
            objekt: Some(e.objekt),
            ..Fehler::from(m)
        })
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

/// Bild in Stickflächen umwandeln wie der „Auto Digitizer“ des Originals (Schritte nach Handbuch Kap. 12/13, Verfahren
/// eigene Festlegung – `stick-vektor`). Dieselbe Eingabe nimmt `bild_vorschau`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BildEingabe {
    /// Bilddatei (PNG, JPG, BMP …)
    pub pfad: String,
    /// Breite des Ergebnisses in mm (Standard 80)
    #[serde(default)]
    pub breite: Option<f64>,
    /// Zahl der Farben im Bild, auch die weggelassenen (Standard 8; mit Garnkarte können es weniger werden, wenn zwei
    /// Farben dasselbe Garn bekommen)
    #[serde(default)]
    pub farben: Option<usize>,
    /// Bild vereinfachen 0–5 (Standard 2): glättet Fotorauschen und feine Details vor der Farbreduktion, Kanten bleiben
    /// scharf (wie „Artwork Preparation“ im Original: Rauschen entfernen, unnötige Details weglassen)
    #[serde(default)]
    pub vereinfachen: Option<u32>,
    /// kleinste Fläche in mm² (Standard 4) – kleinere gehen in der Nachbarfarbe auf (keine Löcher)
    #[serde(default)]
    pub min_flaeche: Option<f64>,
    /// Glättung der Umrisse in mm (Standard 0,2)
    #[serde(default)]
    pub glaettung: Option<f64>,
    /// Farben, die **nicht** gestickt werden („Omitted Colors“ im Original), als „#rrggbb“ wie in `bild_vorschau`.
    /// Ohne Angabe: Vorschlag des Programms (weißer Hintergrund wie „white background omitted“); leere Liste = alles
    /// sticken
    #[serde(default)]
    pub weglassen: Option<Vec<String>>,
    /// Garnkarte (Name wie `garnkarten_liste`): nennt zu jeder Farbe das nächste Garn (CIEDE2000) und zieht die Farben
    /// auf diese Garne (`garnfarben`); gleiche Garne werden eine Farbe, die Objekte bekommen `garn`. Ohne Angabe oder
    /// „keine“: Farben des Bildes
    #[serde(default)]
    pub garnkarte: Option<String>,
    /// Farben auf die Garne der Karte ziehen (Standard an; aus = Farben des Bildes, die Garne nur als Hinweis)
    #[serde(default)]
    pub garnfarben: Option<bool>,
    /// Stichart der Flächen: „step“ (Standard, alles Step/Tatami – bei Fotos am saubersten) oder „auto“ (wie
    /// EasyStitch/„Fills: Auto“: bis 8 mm dick Satinfläche, darüber Step – für Logos und Zeichnungen)
    #[serde(default)]
    pub fuellung: Option<BildFuellung>,
    /// veraltet: alles sticken (wie `weglassen: []`)
    #[serde(default)]
    pub mit_hintergrund: bool,
    /// Rahmen des neuen Designs
    #[serde(default)]
    pub rahmen: Option<String>,
}

/// Stichart der Flächen beim Bild umwandeln.
#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum BildFuellung {
    Auto,
    Step,
}

/// Eine Farbe des vorbereiteten Bildes.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BildFarbe {
    /// „#rrggbb“ – genau so in der Vorschau (zum Weglassen in `weglassen` angeben)
    pub farbe: String,
    /// Anteil am Bild in Prozent
    pub anteil: f64,
    /// wird weggelassen (Vorschlag oder `weglassen`)
    pub weglassen: bool,
    /// nächstes Garn der Karte (mit `garnkarte`; mit `garnfarben` ist die Farbe genau dieses Garn)
    pub garn: Option<GarnInfo>,
}

/// Ergebnis von `bild_vorschau`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BildVorschau {
    /// das vereinfachte Bild mit den reduzierten Farben als PNG („data:image/png;base64,…“) – weggelassene Farben sind
    /// enthalten, die Oberfläche blendet sie aus
    pub bild: String,
    pub breite_px: u32,
    pub hoehe_px: u32,
    /// Farben, häufigste zuerst
    pub farben: Vec<BildFarbe>,
}

/// Ergebnis der Bildumwandlung.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BildErgebnis {
    /// neues Design (Satin- und Step-Flächen nach Farben)
    pub design: DesignWert,
    pub flaechen: usize,
    pub farben: usize,
    /// Zahl der Objekte je Objektart (`satinflaeche`, `tatami`)
    pub klassen: std::collections::BTreeMap<String, usize>,
    /// Bildgröße in Punkten
    pub breite_px: u32,
    pub hoehe_px: u32,
}

fn hex_farbe(t: &str) -> Result<[u8; 3], Fehler> {
    let h = t.trim().trim_start_matches('#');
    if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("Farbe „{t}“: erwartet „#rrggbb“").into());
    }
    Ok([0, 2, 4].map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0)))
}

fn hex(c: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

/// Base64 (für das Vorschaubild als data-URL)
fn base64(daten: &[u8]) -> String {
    const Z: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(daten.len().div_ceil(3) * 4);
    for b in daten.chunks(3) {
        let n = (u32::from(b[0]) << 16) | (u32::from(*b.get(1).unwrap_or(&0)) << 8) | u32::from(*b.get(2).unwrap_or(&0));
        for i in 0..4 {
            if i <= b.len() {
                out.push(Z[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// Eingabe → Optionen von `stick-vektor` (+ Garnkarte, wenn angegeben)
fn bild_optionen(k: &Kontext, e: &BildEingabe) -> Result<(stick_vektor::BildOptionen, Option<garnkarte::Garnkarte>), Fehler> {
    let st = stick_vektor::BildOptionen::default();
    let karte = match e.garnkarte.as_deref() {
        None | Some("") | Some("keine") => None,
        Some(n) => Some(garnkarte::laden(&k.garnkarten, n)?),
    };
    let weglassen = match &e.weglassen {
        Some(l) => Some(l.iter().map(|t| hex_farbe(t)).collect::<Result<Vec<_>, _>>()?),
        None if e.mit_hintergrund => Some(Vec::new()),
        None => None,
    };
    Ok((
        stick_vektor::BildOptionen {
            breite_mm: e.breite.unwrap_or(st.breite_mm),
            farben: e.farben.unwrap_or(st.farben),
            min_flaeche: e.min_flaeche.unwrap_or(st.min_flaeche),
            glaettung: e.glaettung.unwrap_or(st.glaettung),
            vereinfachen: e.vereinfachen.unwrap_or(st.vereinfachen),
            weglassen,
            palette: karte
                .as_ref()
                .filter(|_| e.garnfarben.unwrap_or(true))
                .map(|k| k.garne.iter().map(|g| g.rgb).collect()),
            fuellung: match e.fuellung {
                Some(BildFuellung::Auto) => stick_vektor::Fuellung::Auto,
                _ => stick_vektor::Fuellung::Step,
            },
        },
        karte,
    ))
}

/// Vorschau der Bildumwandlung: vereinfachtes Bild und Farbliste (zum Weglassen wählen), bevor Stiche entstehen.
pub fn bild_vorschau(k: &Kontext, e: BildEingabe) -> Result<BildVorschau, Fehler> {
    let (o, karte) = bild_optionen(k, &e)?;
    let bild = stick_vektor::bild_laden(&PathBuf::from(&e.pfad))?;
    let v = stick_vektor::vorbereiten(&bild, &o);
    let weg = v.weggelassen(&o);
    let summe = v.punkte.iter().sum::<usize>().max(1) as f64;
    Ok(BildVorschau {
        bild: format!("data:image/png;base64,{}", base64(&v.png())),
        breite_px: v.breite as u32,
        hoehe_px: v.hoehe as u32,
        farben: v
            .farben
            .iter()
            .enumerate()
            .map(|(i, c)| BildFarbe {
                farbe: hex(*c),
                anteil: (1000.0 * v.punkte[i] as f64 / summe).round() / 10.0,
                weglassen: weg.contains(&i),
                garn: karte.as_ref().map(|kt| GarnInfo::from(kt.naechstes(*c))),
            })
            .collect(),
    })
}

pub fn bild_umwandeln(k: &Kontext, e: BildEingabe) -> Result<BildErgebnis, Fehler> {
    let (o, karte) = bild_optionen(k, &e)?;
    let pfad = PathBuf::from(&e.pfad);
    let bild = stick_vektor::bild_laden(&pfad)?;
    let v = stick_vektor::vorbereiten(&bild, &o);
    let f = stick_vektor::ordnen(stick_vektor::umrisse(&v, &o));
    let stamm = pfad.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let rahmen = e.rahmen.clone().unwrap_or_else(|| "artista 130 x 100".into());
    let (mut design, klassen) = stick_vektor::design_json_mit(&f, &stamm, &rahmen, o.fuellung);
    // mit Garnfarben: Garn je Objekt festhalten (Farbe ist genau die des Garns)
    let karte = karte.filter(|_| o.palette.is_some());
    if let (Some(kt), Some(objekte)) = (&karte, design.get_mut("objekte").and_then(Value::as_array_mut)) {
        for ob in objekte {
            if let Some(c) = ob.get("farbe").and_then(Value::as_str).and_then(|t| hex_farbe(t).ok())
                && let Some(g) = kt.garne.iter().find(|g| g.rgb == c)
            {
                ob["garn"] = Value::from(g.nummer.clone());
            }
        }
    }
    let mut farben: Vec<[u8; 3]> = f.iter().map(|x| x.farbe).collect();
    farben.sort();
    farben.dedup();
    Ok(BildErgebnis {
        design: DesignWert(design),
        flaechen: f.len(),
        farben: farben.len(),
        klassen,
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

/// Neue Gruppe beim Einfügen (`design_einfuegen`).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NeueGruppe {
    /// Name der Gruppe (Elementliste), z. B. der Dateiname
    #[serde(default)]
    pub name: String,
    /// Herkunft (nur Anzeige)
    #[serde(default)]
    pub herkunft: Option<stick_kern::format::GruppenHerkunft>,
}

/// Ein Design in ein anderes einfügen.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EinfuegenEingabe {
    /// Design, in das eingefügt wird
    pub design: DesignWert,
    /// Design, dessen Objekte angehängt werden (z. B. Ergebnis von `bild_umwandeln`/`svg_lesen` oder eine Datei)
    pub einfuegen: DesignWert,
    /// Alle eingefügten Objekte zu **einer** Gruppe zusammenfassen (Transformation des eingefügten Designs wird die
    /// der Gruppe). Hat das eingefügte Design schon Gruppen, bleiben diese stattdessen erhalten (keine verschachtelten
    /// Gruppen)
    #[serde(default)]
    pub gruppe: Option<NeueGruppe>,
}

/// Ergebnis von `design_einfuegen`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct EinfuegenErgebnis {
    /// zusammengefügtes Design
    pub design: DesignWert,
    /// Nummern der eingefügten Objekte im neuen Design
    pub objekte: Vec<usize>,
    /// Schlüssel der neuen Gruppe (wenn gruppiert)
    pub gruppe: Option<String>,
}

/// Design einfügen wie „Insert Design“ des Originals (Handbuch S. 135 „Combining designs“): die Objekte werden
/// hinten angehängt, Farben bleiben (gleiche Farbe = gleiches Garn); das Handbuch rät, das eingefügte Design zu
/// gruppieren (S. 134) – mit `gruppe` geschieht das gleich. Doppelte Objektnamen bekommen „(2)“ … angehängt, damit
/// `aussparen` (nach Namen) eindeutig bleibt; Gruppenschlüssel werden eindeutig gemacht.
pub fn design_einfuegen(_: &Kontext, e: EinfuegenEingabe) -> Result<EinfuegenErgebnis, Fehler> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut ziel = e.design.0;
    let neu = e.einfuegen.0;
    // beide müssen gültige Designs sein
    stick_kern::Design::aus_value(&ziel).map_err(|x| Fehler::from(format!("Design: {x}")))?;
    stick_kern::Design::aus_value(&neu).map_err(|x| Fehler::from(format!("Einzufügendes Design: {x}")))?;
    let zo = ziel.as_object_mut().ok_or("Das Design muss ein JSON-Objekt sein")?;
    let leer = Value::Object(Default::default());
    let mut objekte: Vec<Value> = zo.get("objekte").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut gruppen = zo.get("gruppen").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut namen: BTreeSet<String> = objekte
        .iter()
        .filter_map(|o| o.get("name").and_then(Value::as_str).map(str::to_string))
        .collect();
    let eindeutig = |basis: &str, belegt: &dyn Fn(&str) -> bool| -> String {
        if !belegt(basis) {
            return basis.to_string();
        }
        (2..).map(|n| format!("{basis} ({n})")).find(|k| !belegt(k)).unwrap()
    };

    let n_neu = neu.get("objekte").and_then(Value::as_array).cloned().unwrap_or_default();
    let neu_gruppen = neu.get("gruppen").and_then(Value::as_object).cloned().unwrap_or_default();
    // Objektnamen eindeutig machen, `aussparen` mitziehen
    let mut umbenannt: BTreeMap<String, String> = BTreeMap::new();
    let mut eingefuegt: Vec<Value> = Vec::with_capacity(n_neu.len());
    for o in n_neu {
        let mut o = o;
        if let Some(n) = o.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()).map(str::to_string) {
            let k = eindeutig(&n, &|x| namen.contains(x));
            namen.insert(k.clone());
            if k != n {
                umbenannt.entry(n).or_insert_with(|| k.clone());
                o["name"] = k.into();
            }
        }
        eingefuegt.push(o);
    }
    for o in &mut eingefuegt {
        if let Some(a) = o.get_mut("aussparen").and_then(Value::as_array_mut) {
            for x in a.iter_mut() {
                if let Some(k) = x.as_str().and_then(|s| umbenannt.get(s)) {
                    *x = k.clone().into();
                }
            }
        }
    }

    let gruppe_frei = |gruppen: &serde_json::Map<String, Value>, basis: &str| -> String {
        (1..).map(|n| format!("{basis}{n}")).find(|k| !gruppen.contains_key(k)).unwrap()
    };
    let mut neue_gruppe = None;
    if let (Some(g), true) = (&e.gruppe, neu_gruppen.is_empty()) {
        let k = gruppe_frei(&gruppen, "g");
        let mut eintrag = serde_json::Map::new();
        eintrag.insert("name".into(), g.name.clone().into());
        if let Some(h) = g.herkunft {
            eintrag.insert("herkunft".into(), serde_json::to_value(h).map_err(|x| Fehler::from(x.to_string()))?);
        }
        if let Some(t) = neu.get("transform").filter(|t| t.as_object().is_some_and(|m| !m.is_empty())) {
            eintrag.insert("transform".into(), t.clone());
        }
        gruppen.insert(k.clone(), Value::Object(eintrag));
        for o in &mut eingefuegt {
            o["gruppe"] = k.clone().into();
        }
        neue_gruppe = Some(k);
    } else {
        // vorhandene Gruppen übernehmen, Schlüssel eindeutig
        let mut schluessel: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in &neu_gruppen {
            let neu_k = if gruppen.contains_key(k) { gruppe_frei(&gruppen, &format!("{k}_")) } else { k.clone() };
            gruppen.insert(neu_k.clone(), v.clone());
            schluessel.insert(k.clone(), neu_k);
        }
        for o in &mut eingefuegt {
            if let Some(k) = o.get("gruppe").and_then(Value::as_str).and_then(|k| schluessel.get(k)) {
                o["gruppe"] = k.clone().into();
            }
        }
        if neu.get("transform").is_some_and(|t| t != &leer && !t.is_null()) {
            return Err("Das einzufügende Design hat eine eigene Transformation und Gruppen – bitte zuerst in einer \
                        Gruppe zusammenfassen"
                .into());
        }
    }
    let von = objekte.len();
    objekte.extend(eingefuegt);
    let bis = objekte.len();
    zo.insert("objekte".into(), objekte.into());
    if !gruppen.is_empty() {
        zo.insert("gruppen".into(), gruppen.into());
    }
    stick_kern::Design::aus_value(&ziel).map_err(|x| Fehler::from(format!("Ergebnis: {x}")))?;
    Ok(EinfuegenErgebnis { design: DesignWert(ziel), objekte: (von..bis).collect(), gruppe: neue_gruppe })
}

/// Sticharten für Linien („Outline“ des Originals): Geometrie in `punkte`.
pub const UMRISS_ARTEN: [&str; 5] = ["lauf", "dreifach", "satinlinie", "blanket", "motivlinie"];
/// Sticharten für geschlossene Formen („Fill“ des Originals): Geometrie in `umriss` (+ `loecher`).
pub const FUELL_ARTEN: [&str; 3] = ["tatami", "satinflaeche", "muster"];

/// Stichart eines Objekts wechseln.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StichartEingabe {
    pub design: DesignWert,
    /// Nummer des Objekts (ab 0)
    pub objekt: usize,
    /// neue Objektart: Umriss lauf, dreifach, satinlinie, blanket, motivlinie · Füllung tatami, satinflaeche, muster
    pub art: String,
}

/// Ergebnis von `stichart_aendern`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct StichartErgebnis {
    /// das Objekt mit neuer Stichart (ersetzt das alte; Stiche danach neu berechnen)
    pub objekt: ObjektWert,
    /// Hinweise (z. B. Löcher entfallen beim Wechsel zur Linie)
    pub hinweise: Vec<String>,
}

/// Stichart wechseln wie „Outline/Fill“ + Stichart-Liste im Original (Handbuch S. 63/64: „You can change it at any
/// stage“): die Form bleibt, gemeinsame Felder (Farbe, Name, Garn, Transformation, Gruppe, Werte je Objekt …) bleiben,
/// passende Felder (Löcher, Winkel, Aussparen, Versatz, Breite, geschlossen) gehen mit, der Rest nimmt die Standardwerte.
/// Linie → Füllung nur bei geschlossener Linie (offene Linien lassen sich im Original nicht füllen). Füllung → Linie:
/// der Umriss wird eine geschlossene Linie, Löcher entfallen (Hinweis). Satinsäule, Text, Applikation, Knopfloch: Fehler.
pub fn stichart_aendern(_: &Kontext, e: StichartEingabe) -> Result<StichartErgebnis, Fehler> {
    use stick_kern::format::{Gemeinsam, PunktAngabe};
    let alt = e
        .design
        .0
        .get("objekte")
        .and_then(|o| o.get(e.objekt))
        .cloned()
        .ok_or_else(|| Fehler::from(format!("Objekt {} gibt es nicht", e.objekt + 1)))?;
    let von = alt.get("art").and_then(Value::as_str).unwrap_or("").to_string();
    let ist = |a: &str, l: &[&str]| l.contains(&a);
    let alle: Vec<&str> = UMRISS_ARTEN.iter().chain(FUELL_ARTEN.iter()).copied().collect();
    if !ist(&von, &alle) {
        return Err(format!("Stichart wechseln geht bei Linien und Flächen, nicht bei „{von}“").into());
    }
    if !ist(&e.art, &alle) {
        return Err(format!("Unbekannte Stichart „{}“ (erlaubt: {})", e.art, alle.join(", ")).into());
    }
    let von_flaeche = ist(&von, &FUELL_ARTEN);
    let zu_flaeche = ist(&e.art, &FUELL_ARTEN);
    let mut neu = serde_json::Map::new();
    neu.insert("art".into(), e.art.clone().into());
    // gemeinsame Felder (aus dem Schema von `Gemeinsam`, bleibt so von selbst aktuell)
    let schema = schemars::schema_for!(Gemeinsam);
    let gemeinsam: Vec<String> = schema
        .as_value()
        .get("properties")
        .and_then(Value::as_object)
        .map(|p| p.keys().cloned().collect())
        .unwrap_or_default();
    let o = alt.as_object().ok_or("Objekt ist kein JSON-Objekt")?;
    for k in &gemeinsam {
        if let Some(v) = o.get(k) {
            neu.insert(k.clone(), v.clone());
        }
    }
    let mut hinweise = Vec::new();
    // Geometrie
    let geo = o
        .get(if von_flaeche { "umriss" } else { "punkte" })
        .cloned()
        .ok_or("Objekt ohne Form")?;
    let angabe: PunktAngabe =
        serde_json::from_value(geo.clone()).map_err(|x| Fehler::from(format!("Form ungültig: {x}")))?;
    let pts = angabe.punkte();
    let zu = |a: &stick_kern::geometrie::Punkt, b: &stick_kern::geometrie::Punkt| (a.x - b.x).hypot(a.y - b.y) < 1e-6;
    let von_hause_zu = pts.len() > 2 && zu(&pts[0], &pts[pts.len() - 1]);
    let geschlossen = von_flaeche || o.get("geschlossen").and_then(Value::as_bool).unwrap_or(false) || von_hause_zu;
    const MIT_GESCHLOSSEN: [&str; 4] = ["lauf", "dreifach", "blanket", "motivlinie"];
    if zu_flaeche {
        if !geschlossen {
            return Err("Eine offene Linie lässt sich nicht füllen – erst schließen (Feld „geschlossen“) oder eine \
                        geschlossene Form verwenden"
                .into());
        }
        neu.insert("umriss".into(), geo);
    } else if von_flaeche && !ist(&e.art, &MIT_GESCHLOSSEN) && !von_hause_zu {
        // Satinlinie kennt kein „geschlossen“: Linie zum ersten Punkt zurückführen
        let p0 = pts.first().ok_or("Form ohne Punkte")?;
        neu.insert("punkte".into(), serde_json::json!({ "verbinden": [geo, [[p0.x, p0.y]]] }));
    } else {
        neu.insert("punkte".into(), geo);
    }
    if !zu_flaeche && geschlossen && ist(&e.art, &MIT_GESCHLOSSEN) && !von_hause_zu {
        neu.insert("geschlossen".into(), true.into());
    }
    if von_flaeche && !zu_flaeche && o.get("loecher").and_then(Value::as_array).is_some_and(|l| !l.is_empty()) {
        hinweise.push("Die Löcher der Fläche entfallen – die Linie folgt nur dem äußeren Umriss".into());
    }
    // passende Felder mitnehmen, wenn die neue Art sie kennt
    let felder: &[&str] = match e.art.as_str() {
        "tatami" => &["loecher", "winkel", "aussparen", "ueberlappung"],
        "muster" => &["loecher", "winkel", "aussparen", "ueberlappung"],
        "satinflaeche" => &["loecher"],
        "satinlinie" => &["breite", "versatz"],
        "blanket" => &["breite", "versatz"],
        "lauf" | "dreifach" => &["laenge"],
        _ => &[],
    };
    for k in felder {
        if let Some(v) = o.get(*k).filter(|_| zu_flaeche == von_flaeche || *k == "loecher" && zu_flaeche) {
            neu.insert((*k).into(), v.clone());
        }
    }
    if e.art == "muster" {
        neu.entry("muster").or_insert_with(|| "001".into());
    }
    let neu = Value::Object(neu);
    // Ergebnis muss ein gültiges Objekt sein
    serde_json::from_value::<ObjektJson>(neu.clone()).map_err(|x| Fehler::from(format!("Ergebnis ungültig: {x}")))?;
    Ok(StichartErgebnis { objekt: ObjektWert(neu), hinweise })
}
