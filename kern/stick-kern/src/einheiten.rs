//! Einheiten und feste Grenzen des Maschinenformats.
//!
//! Koordinaten sind Millimeter (f64), Ursprung = Rahmenmitte, +y nach oben
//! (wie EXP, `Analyse/code/07_exp_format.md`). Gerundet wird erst beim Export.

/// Interne Einheit der Original-Software: 1 mm = 180 (`Analyse/code/06_dataitem_und_tags.md`).
pub const EINHEITEN_PRO_MM: f64 = 180.0;

/// EXP: Schritte in 0,1 mm, vorzeichenbehaftet 8 Bit (hoch).
pub const EXP_AUFLOESUNG_MM: f64 = 0.1;
pub const EXP_MAX_SCHRITT: i64 = 127;

/// Maschinenformat „Bernina“, Maschinentyp 17 (DEFAULTS.DFL, hoch).
pub const MIN_STICH_MM: f64 = 0.3;
pub const MAX_STICH_MM: f64 = 12.7;

/// Auto Jump: Stiche > 12,7 mm werden in Sprünge zu 7,0 mm zerlegt (AMT 0x2711/0x2712, Handbuch S. 203, hoch).
pub const AUTO_JUMP_TEILLAENGE_MM: f64 = 7.0;

pub fn mm_zu_einheiten(mm: f64) -> i64 {
    (mm * EINHEITEN_PRO_MM).round() as i64
}

pub fn einheiten_zu_mm(einheiten: i64) -> f64 {
    einheiten as f64 / EINHEITEN_PRO_MM
}
