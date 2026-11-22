//! Rechenkern des Stickprogramms für die BERNINA artista 185 – nach dem Vorbild BERNINA artista V4.
//!
//! Grundsatz (`ARCHITECTURE.md` §1a): Werte und Abläufe kommen aus der Analyse der Original-Software.
//! Erste Fassung = getreue Übertragung des Python-Prototyps `stickprogramm/` (Prüfung: gleiche EXP-Bytes).

/// Auswahlfeld des Design-Formats (`format.rs`): Aufzählung mit festen Textwerten – im Schema als Werteliste, in der
/// Oberfläche als Auswahlliste. `text()` liefert den Wert wie im JSON.
macro_rules! auswahl {
    ($(#[$m:meta])* $v:vis enum $name:ident { $($(#[$vm:meta])* $var:ident = $txt:literal),+ $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
        $v enum $name { $($(#[$vm])* #[serde(rename = $txt)] $var),+ }
        impl $name {
            /// Wert wie im JSON
            pub fn text(self) -> &'static str {
                match self { $($name::$var => $txt),+ }
            }
        }
    };
}

pub mod anordnen;
pub mod applikation;
pub mod aussparen;
pub mod blackwork;
pub mod design;
pub mod effekte;
pub mod ein_ausstich;
pub mod einheiten;
pub mod einstellungen;
pub mod farbe;
pub mod format;
pub mod geometrie;
pub mod knopfloch;
pub mod monogramm;
pub mod motiv;
pub mod muster;
pub mod nachbearbeitung;
pub mod parameter;
pub mod photosnap;
pub mod rahmen;
pub mod reihenfolge;
pub mod saeulen;
pub mod schrift;
pub mod schrift_ttf;
pub mod sticharten;
pub mod stickbarkeit;
pub mod stichdaten;
pub mod stiche;
pub mod transformation;
pub mod umwandeln;
pub mod werte;

pub use design::Design;
pub use stiche::{Art, Stich, StichFolge};
