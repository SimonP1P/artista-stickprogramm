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
}

pub fn alphabete_liste(k: &Kontext, _: Leer) -> Result<Vec<AlphabetInfo>, Fehler> {
    Ok(esb::verzeichnis(&k.alphabete)
        .into_iter()
        .filter(|(n, _)| n != "ButtonHoles")
        .map(|(name, p)| AlphabetInfo {
            name,
            datei: p.file_name().unwrap_or_default().to_string_lossy().into(),
        })
        .collect())
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

/// Ergebnis der Prüfung eines Designs (ohne Stiche zu berechnen).
#[derive(Debug, Serialize, JsonSchema)]
pub struct Pruefung {
    /// Fehler, der das Berechnen verhindert (None = in Ordnung)
    pub fehler: Option<Fehler>,
    /// Hinweise, z. B. Felder, die der Kern nicht liest (Tippfehler?)
    pub hinweise: Vec<String>,
}

pub fn design_pruefen(k: &Kontext, e: DesignEingabe) -> Result<Pruefung, Fehler> {
    let hinweise = crate::erzeugen::unbekannte_felder(&e.design.0);
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
    /// Garn je Farbblock aus der Garnkarte des Designs (leer bei „keine“)
    pub garne: Vec<GarnInfo>,
    pub statistik: Statistik,
    /// Zahl der Einstiche außerhalb des Stickfelds (muss 0 sein zum Senden)
    pub ausserhalb: usize,
    /// Zeichen eines Texts, die das Alphabet nicht hat
    pub fehlende_zeichen: String,
    /// Hinweise (z. B. Felder, die der Kern nicht liest)
    pub hinweise: Vec<String>,
}

pub fn design_berechnen(k: &Kontext, e: DesignEingabe) -> Result<Ergebnis, Fehler> {
    let design = k.design(&e)?;
    let (_, fehlend) = design.objekte_aufgeloest()?;
    let folge = design.berechnen()?;
    let garne = if design.garnkarte == "keine" {
        Vec::new()
    } else {
        let karte = garnkarte::laden(&k.garnkarten, &design.garnkarte)?;
        folge
            .farben
            .iter()
            .map(|hex| GarnInfo::from(karte.naechstes(stick_formate::artista::Farbe::aus_hex(hex).rgb)))
            .collect()
    };
    Ok(Ergebnis {
        name: design.name.clone(),
        rahmen: RahmenInfo::from(&design.rahmen),
        stoff: design.einstellungen.stoff.clone(),
        ausserhalb: design.ausserhalb(&folge).len(),
        fehlende_zeichen: fehlend.into_iter().collect(),
        hinweise: crate::erzeugen::unbekannte_felder(&e.design.0),
        statistik: statistik_von(&folge),
        garne,
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
