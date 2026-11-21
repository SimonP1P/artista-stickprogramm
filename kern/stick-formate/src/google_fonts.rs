//! Katalog der Google Fonts (1 833 Familien mit lateinischer Schrift, freie Lizenzen OFL/Apache/UFL) für die
//! Schriftauswahl: Familie, Kategorie, Stile (deutsche Namen), Dateipfad im Repo `github.com/google/fonts`.
//! Erzeugt mit `Arbeitsdaten/werkzeuge/google_fonts_katalog.py` (Stile = eigene Zuordnung der Google-Tags);
//! das Herunterladen steckt in `stick-api` (Netz ist keine Dateiformat-Frage).

use serde::Deserialize;
use std::sync::OnceLock;

/// Eine Familie im Katalog.
#[derive(Clone, Debug, Deserialize)]
pub struct GoogleSchrift {
    pub familie: String,
    /// Ohne Serifen, Mit Serifen, Zierschrift, Handschrift, Schreibmaschine
    pub kategorie: String,
    /// deutsche Stilnamen (Reihenfolge wie `Katalog::stile`)
    pub stile: Vec<String>,
    /// Pfad im Repo, z. B. `ofl/parisienne/Parisienne-Regular.ttf`
    pub pfad: String,
    /// Größe der Datei in Byte
    pub groesse: u64,
    /// Rang der Beliebtheit bei Google (klein = beliebt); der Katalog ist danach sortiert
    pub beliebtheit: u32,
    pub lizenz: String,
}

#[derive(Debug, Deserialize)]
pub struct Katalog {
    /// Datum der Erzeugung
    pub stand: String,
    pub quelle: String,
    /// alle Stilnamen in Anzeigereihenfolge
    pub stile: Vec<String>,
    pub schriften: Vec<GoogleSchrift>,
}

/// Basisadresse der Dateien
pub const BASIS: &str = "https://raw.githubusercontent.com/google/fonts/main/";

/// Der mitgelieferte Katalog.
pub fn katalog() -> &'static Katalog {
    static K: OnceLock<Katalog> = OnceLock::new();
    K.get_or_init(|| {
        serde_json::from_str(include_str!("../daten/google_fonts.json")).expect("google_fonts.json ungültig")
    })
}

/// Familie nach Namen (Groß-/Kleinschreibung egal).
pub fn finden(familie: &str) -> Option<&'static GoogleSchrift> {
    katalog().schriften.iter().find(|s| s.familie.eq_ignore_ascii_case(familie.trim()))
}

impl GoogleSchrift {
    /// Dateiname (letzter Teil des Pfads), z. B. `Roboto[wdth,wght].ttf`
    pub fn dateiname(&self) -> &str {
        self.pfad.rsplit('/').next().unwrap_or(&self.pfad)
    }

    /// Adresse zum Herunterladen (Klammern und Kommas variabler Schriften kodiert)
    pub fn url(&self) -> String {
        let mut s = String::from(BASIS);
        for c in self.pfad.chars() {
            match c {
                '[' => s.push_str("%5B"),
                ']' => s.push_str("%5D"),
                ',' => s.push_str("%2C"),
                ' ' => s.push_str("%20"),
                _ => s.push(c),
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn katalog_lesbar() {
        let k = katalog();
        assert!(k.schriften.len() > 1500);
        assert!(k.stile.iter().any(|s| s == "Gotisch"));
        let r = finden("roboto").unwrap();
        assert_eq!(r.url(), format!("{BASIS}ofl/roboto/Roboto%5Bwdth%2Cwght%5D.ttf"));
        assert_eq!(r.dateiname(), "Roboto[wdth,wght].ttf");
        assert!(finden("UnifrakturMaguntia").unwrap().stile.iter().any(|s| s == "Gotisch"));
        // jede Familie hat eine .ttf-Datei, alle Stile sind bekannt
        for s in &k.schriften {
            assert!(s.pfad.ends_with(".ttf"), "{}", s.familie);
            assert!(s.stile.iter().all(|x| k.stile.contains(x)), "{}", s.familie);
        }
    }
}
