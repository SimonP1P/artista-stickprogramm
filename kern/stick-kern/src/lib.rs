//! Rechenkern des Stickprogramms für die BERNINA artista 185 – nach dem Vorbild BERNINA artista V4.
//!
//! Grundsatz (`ARCHITECTURE.md` §1a): Werte und Abläufe kommen aus der Analyse der Original-Software.
//! Erste Fassung = getreue Übertragung des Python-Prototyps `stickprogramm/` (Prüfung: gleiche EXP-Bytes).

pub mod applikation;
pub mod aussparen;
pub mod design;
pub mod einheiten;
pub mod einstellungen;
pub mod geometrie;
pub mod muster;
pub mod nachbearbeitung;
pub mod parameter;
pub mod rahmen;
pub mod schrift;
pub mod schrift_ttf;
pub mod sticharten;
pub mod stiche;
pub mod transformation;

pub use design::Design;
pub use stiche::{Art, Stich, StichFolge};
