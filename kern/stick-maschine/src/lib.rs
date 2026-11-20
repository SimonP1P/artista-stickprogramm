//! Anbindung der BERNINA artista 185 (Stickmodul, serielles Kabel) – Phase 5 der Roadmap.
//!
//! Das Protokoll ist aus den am Gerät erprobten Testprogrammen übernommen (v3.6 Lesen, v4.1 Senden; Befunde in
//! `Analyse_Teil2/07`). Schichten:
//! - [`leitung`]: Bytes senden/empfangen, Baudrate, Uhr – echter Port ([`port`], Merkmal `port`) oder
//!   [`simulation`] (virtuelle Uhr, Werte der echten 185),
//! - [`protokoll`]: `T r M E`, `R`/`N` lesen, `W`/`P S` schreiben, Befehlsfeld, Synchronisieren, Baudwechsel,
//! - [`maschine`]: verbinden, Designliste, Design lesen, Garnfarbbereich, planen und senden, PC-Modus verlassen.
//!
//! Geschrieben wird nur in [`maschine::Maschine::senden`]; davor zeigt der Aufrufer den Plan aus
//! [`maschine::Maschine::planen`] und fragt nach (CLAUDE.md Regel 1a).

pub mod leitung;
pub mod maschine;
#[cfg(feature = "port")]
pub mod port;
pub mod protokoll;
pub mod simulation;

pub use leitung::Leitung;
pub use maschine::{Maschine, SendeOptionen, Speicher};
pub use simulation::{SimOptionen, Simulation};

#[cfg(test)]
mod tests;
