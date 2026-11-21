//! Stil-Einordnung der Schriften für die Schriftauswahl (eigene Festlegung, `Analyse_Teil3/21_schriften.md` §6):
//! 23 Stile (1–19 wie der Katalog der Google Fonts, dazu Applikation, Monogramm, Großbuchstaben, Andere Schriften),
//! Stile der 50 Original-Alphabete und gängiger Schriften von Windows, Office und macOS nach dem Aussehen
//! (`daten/schrift_stile.json`, von Hand gepflegt). Installierte Schriften, die nicht in der Liste stehen, bekommen
//! die Einordnung von Google, wenn es die Familie dort gibt.

use crate::google_fonts;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Ein Stil (Filter in der Schriftauswahl).
#[derive(Clone, Debug, Deserialize)]
pub struct Stil {
    pub name: String,
    /// kurze Erklärung für den Anwender
    pub beschreibung: String,
}

#[derive(Debug, Deserialize)]
pub struct StilDaten {
    pub stand: String,
    pub quelle: String,
    /// alle Stile in Anzeigereihenfolge
    pub stile: Vec<Stil>,
    /// Original-Alphabet → Stile
    pub alphabete: BTreeMap<String, Vec<String>>,
    /// Familie, voller Name oder Namensanfang einer Computer-Schrift → Stile
    pub schriften: BTreeMap<String, Vec<String>>,
}

/// Die mitgelieferte Einordnung.
pub fn daten() -> &'static StilDaten {
    static D: OnceLock<StilDaten> = OnceLock::new();
    D.get_or_init(|| {
        serde_json::from_str(include_str!("../daten/schrift_stile.json")).expect("schrift_stile.json ungültig")
    })
}

/// Stile eines Original-Alphabets (leer, wenn nicht eingeordnet).
pub fn alphabet(name: &str) -> Vec<String> {
    daten().alphabete.get(name).cloned().unwrap_or_default()
}

/// Stile einer Computer-Schrift: eigene Liste (Familie, voller Name, längster passender Namensanfang), sonst
/// Google (Familie, voller Name), sonst leer. Groß-/Kleinschreibung egal.
pub fn schrift(familie: &str, name: &str) -> Vec<String> {
    let liste = &daten().schriften;
    let gleich = |k: &str| liste.iter().find(|(x, _)| x.eq_ignore_ascii_case(k.trim())).map(|(_, v)| v.clone());
    if let Some(v) = gleich(familie).or_else(|| gleich(name)) {
        return v;
    }
    let klein = name.to_lowercase();
    let anfang = liste
        .iter()
        .filter(|(k, _)| klein.starts_with(&format!("{} ", k.to_lowercase())))
        .max_by_key(|(k, _)| k.len());
    if let Some((_, v)) = anfang {
        return v.clone();
    }
    google_fonts::finden(familie)
        .or_else(|| google_fonts::finden(name))
        .map(|g| g.stile.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stile_bekannt_und_wie_google() {
        let d = daten();
        let namen: Vec<&str> = d.stile.iter().map(|s| s.name.as_str()).collect();
        // die Stile des Google-Katalogs stehen in derselben Reihenfolge am Anfang
        let g = &google_fonts::katalog().stile;
        assert_eq!(&namen[..g.len()], g.iter().map(String::as_str).collect::<Vec<_>>().as_slice());
        for (k, v) in d.alphabete.iter().chain(d.schriften.iter()) {
            assert!(!v.is_empty(), "{k} ohne Stil");
            for s in v {
                assert!(namen.contains(&s.as_str()), "{k}: unbekannter Stil {s}");
            }
        }
        assert_eq!(d.alphabete.len(), 50);
    }

    #[test]
    fn schrift_zuordnung() {
        assert_eq!(schrift("Arial Black", "Arial Black"), ["Modern", "Kräftig"]);
        assert_eq!(schrift("Helvetica Neue", "Helvetica Neue Condensed Black"), ["Modern"]);
        // Namensanfang: „Segoe UI Semibold“ → Segoe UI
        assert_eq!(schrift("", "Segoe UI Semibold"), ["Modern"]);
        // nicht in der Liste, aber bei Google
        assert!(schrift("Pirata One", "Pirata One").contains(&"Gotisch".to_string()));
        assert!(schrift("Unbekannt", "Unbekannt Bold").is_empty());
        assert_eq!(alphabet("Olde English"), ["Gotisch"]);
    }
}
