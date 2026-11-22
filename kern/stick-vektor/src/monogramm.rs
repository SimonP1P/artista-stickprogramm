//! **Katalog der Monogramm-Rahmen** – eigene Erweiterung (Nutzer 21.11.2026).
//!
//! 47 Rahmen in 8 Kategorien (Entwurf des Nutzers mit ChatGPT; Quelle `Arbeitsdaten/monogramm_rahmen/`, Katalog
//! erzeugt mit `Arbeitsdaten/werkzeuge/monogramm_rahmen_katalog.py` → `daten/monogramm_rahmen.json`): je Rahmen eine
//! SVG (90 × 90 mm, nur Pfade, Linien 2–4 mm, höchstens 3 Farben), Schriftvorschlag (Google Fonts) und Versalhöhen
//! der Initialen. Die SVG wird wie beim Befehl „SVG einlesen“ in Stickobjekte umgesetzt (`svg.rs`: Linien nach Breite
//! wie EasyStitch, 2–4 mm → Satinkante); das Monogramm selbst setzt der Kern (`stick_kern::monogramm`).

use serde::{Deserialize, Serialize};
use stick_kern::Design;
use stick_kern::monogramm::RahmenVorlage;
use std::sync::OnceLock;

static KATALOG_JSON: &str = include_str!("../daten/monogramm_rahmen.json");

/// Ein Rahmen im Katalog.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RahmenEintrag {
    /// Nummer im Entwurf (1–47)
    pub nr: u32,
    /// Name (Design-Feld `vorlage`)
    pub name: String,
    pub kategorie: String,
    /// Dateiname der SVG in `Arbeitsdaten/monogramm_rahmen/`
    pub datei: String,
    /// Schriftvorschlag (Google Fonts, Name der Familie)
    pub schrift: String,
    /// Versalhöhen der Initialen in mm: links, rechts bzw. links, Mitte, rechts
    pub hoehen: Vec<f64>,
    pub beschreibung: String,
    /// Zeichnung (SVG-Text)
    pub svg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Katalog {
    pub quelle: String,
    /// Kategorien in der Reihenfolge des Entwurfs
    pub kategorien: Vec<String>,
    pub rahmen: Vec<RahmenEintrag>,
}

/// Der eingebaute Katalog.
pub fn katalog() -> &'static Katalog {
    static K: OnceLock<Katalog> = OnceLock::new();
    K.get_or_init(|| serde_json::from_str(KATALOG_JSON).expect("daten/monogramm_rahmen.json lesbar"))
}

/// Rahmen nach Namen (Groß-/Kleinschreibung egal) oder Dateinamen.
pub fn finden(name: &str) -> Option<&'static RahmenEintrag> {
    let n = name.trim().to_lowercase();
    katalog()
        .rahmen
        .iter()
        .find(|r| r.name.to_lowercase() == n || r.datei.to_lowercase().trim_end_matches(".svg") == n.trim_end_matches(".svg"))
}

/// Rahmen als Vorlage für den Kern: SVG wie „SVG einlesen“ (Größe wie gezeichnet, Mitte der Zeichnung im Nullpunkt).
pub fn vorlage(name: &str) -> Result<RahmenVorlage, String> {
    let e = finden(name).ok_or_else(|| format!("Monogramm-Rahmen „{name}“ gibt es nicht"))?;
    let o = crate::svg::SvgOptionen {
        kuerzeste_verbindung: false,
        ..Default::default()
    };
    let r = crate::svg::svg_lesen(e.svg.as_bytes(), &e.name, &o)?;
    let d = Design::aus_value(&r.design)?;
    Ok(RahmenVorlage {
        name: e.name.clone(),
        teile: d.objekte,
        hoehen: e.hoehen.clone(),
        schrift: e.schrift.clone(),
    })
}

/// Monogramm-Rahmen eines Designs laden (`Design::monogramm_vorlagen`), ohne Zwischenspeicher (Befehlszeile).
pub fn design_rahmen_laden(d: &mut Design) -> Result<(), String> {
    for n in d.monogramm_vorlagen() {
        if !d.monogramm_rahmen.contains_key(&n) {
            let v = vorlage(&n)?;
            d.monogramm_rahmen.insert(n, v);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Alle 47 Rahmen: lesbar, nur Satinkanten und kräftige Linien, Größe 70–88 mm, 1–3 Farben
    #[test]
    fn alle_rahmen_lesbar() {
        let k = katalog();
        assert_eq!(k.rahmen.len(), 47);
        assert_eq!(k.kategorien.len(), 8);
        for e in &k.rahmen {
            assert!(k.kategorien.contains(&e.kategorie), "{}", e.name);
            assert!(matches!(e.hoehen.len(), 2 | 3), "{}", e.name);
            let v = vorlage(&e.name).unwrap_or_else(|f| panic!("{}: {f}", e.name));
            let g = v.groesse();
            assert!((70.0..=88.0).contains(&g), "{}: {g}", e.name);
            assert!((1..=3).contains(&v.farben().len()), "{}", e.name);
            for t in &v.teile {
                assert!(
                    matches!(
                        t.art,
                        stick_kern::design::ObjektArt::SatinLinie { .. } | stick_kern::design::ObjektArt::Lauf { .. }
                    ),
                    "{}: {:?}",
                    e.name,
                    t.art
                );
            }
        }
        assert_eq!(finden("doppeloval").unwrap().nr, 1);
        assert_eq!(finden("01_doppeloval.svg").unwrap().name, "Doppeloval");
        assert!(finden("gibt es nicht").is_none());
    }
}
