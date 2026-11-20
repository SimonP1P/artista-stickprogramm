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

/// Zeitstempel JJJJMMTT_HHMMSS (UTC) für Mitschnitte.
pub fn zeitstempel() -> String {
    let s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as i64;
    let (tage, rest) = (s.div_euclid(86400), s.rem_euclid(86400));
    // Kalender aus Tagen seit 1970 (Howard Hinnant, civil_from_days)
    let z = tage + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}{m:02}{d:02}_{:02}{:02}{:02}",
        rest / 3600,
        rest / 60 % 60,
        rest % 60
    )
}

#[cfg(test)]
mod tests;
