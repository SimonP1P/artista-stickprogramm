//! Garnkarten der Original-Software (`PROGRAM/RES/*.tch`, `Analyse/code/09_garnkarten_tch.md`): CSV in Latin-1,
//! je Zeile `Nummer,Marke,Farbname,Dichte,R,G,B`. Daraus bekommen die Farben eines Designs Marke, Garnnummer und
//! Namen für die Garnfarbtabelle der Maschine (Schritt 12).
//!
//! Am Gerät bestätigt (20.11.2026): Die artista 185 legt für Designs ohne Farben „Default“ Nr. 2 „Blue“ (0,0,255) an –
//! Zeile 2 von `Default.tch`; die Isacord-Farben von Design Nr. 21 („rich brown“ 1311, „brass“ 821) haben genau die
//! RGB-Werte dieser Karte.

use crate::artista::Farbe;
use stick_kern::farbe::{de2000, lab};
use std::path::{Path, PathBuf};

/// Ein Garn einer Karte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Garn {
    /// Herstellernummer als Text (führende Nullen bleiben, z. B. „0010“)
    pub nummer: String,
    pub marke: String,
    pub name: String,
    pub rgb: [u8; 3],
}

impl Garn {
    /// Eintrag für die Farbtabelle (Garnnummer als Zahl: „0010“ → 10, wie bei den Maschinendesigns).
    pub fn farbe(&self) -> Farbe {
        let ziffern: String = self.nummer.chars().filter(char::is_ascii_digit).collect();
        Farbe {
            rgb: self.rgb,
            nummer: ziffern.parse::<u32>().map_or(0, |n| n.min(u32::from(u16::MAX)) as u16),
            marke: self.marke.clone(),
            name: self.name.clone(),
        }
    }
}

/// Garnkarte (eine `.tch`-Datei).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Garnkarte {
    /// Dateiname ohne Endung, z. B. „Isacord 40 wt-numerical“
    pub name: String,
    pub garne: Vec<Garn>,
}

impl Garnkarte {
    /// Aus dem Text einer `.tch`-Datei. **Marke:** die der ersten Zeile für alle Garne – eigene Festlegung, weil
    /// `Isacord 40 wt-numerical.tch` und `Mettler 40 wt-numerical.tch` im Markenfeld fortlaufend „Isacord 40“ …
    /// „Isacord 354“ tragen (Fehler in den Daten der CD; die Maschinendesigns schreiben „Isacord 40“).
    pub fn aus_text(name: &str, text: &str) -> Result<Garnkarte, String> {
        let mut garne = Vec::new();
        let mut marke: Option<String> = None;
        for (i, zeile) in text.lines().enumerate() {
            let z = zeile.trim();
            if z.is_empty() {
                continue;
            }
            let f: Vec<&str> = z.split(',').map(str::trim).collect();
            if f.len() < 7 {
                return Err(format!("{name}, Zeile {}: {} statt 7 Felder", i + 1, f.len()));
            }
            let kanal = |s: &str| {
                s.parse::<u8>()
                    .map_err(|_| format!("{name}, Zeile {}: Farbwert „{s}“", i + 1))
            };
            let m = marke.get_or_insert_with(|| f[1].to_string()).clone();
            garne.push(Garn {
                nummer: f[0].to_string(),
                marke: m,
                name: f[2].to_string(),
                rgb: [kanal(f[4])?, kanal(f[5])?, kanal(f[6])?],
            });
        }
        if garne.is_empty() {
            return Err(format!("{name}: keine Garne"));
        }
        Ok(Garnkarte {
            name: name.to_string(),
            garne,
        })
    }

    /// Datei lesen (Latin-1).
    pub fn lesen(pfad: &Path) -> Result<Garnkarte, String> {
        let roh = std::fs::read(pfad).map_err(|e| format!("{}: {e}", pfad.display()))?;
        let text: String = roh.iter().map(|&b| b as char).collect();
        let name = pfad.file_stem().unwrap_or_default().to_string_lossy();
        Garnkarte::aus_text(&name, &text)
    }

    /// Marke der Karte (erste Zeile).
    pub fn marke(&self) -> &str {
        &self.garne[0].marke
    }

    /// Garn mit dieser Nummer („10“ findet auch „0010“).
    pub fn finden(&self, nummer: &str) -> Option<&Garn> {
        let ohne = |s: &str| s.trim_start_matches('0').to_string();
        self.garne.iter().find(|g| ohne(&g.nummer) == ohne(nummer))
    }

    /// Nächstes Garn zu einer Bildschirmfarbe: kleinster Farbabstand **CIEDE2000**. **Eigene Festlegung** – wie die
    /// Original-Software Farben einer Karte zuordnet, ist noch nicht analysiert. Verglichen (20.11.2026, Default- und
    /// Isacord-Karte, 8 Farben): reiner RGB-Abstand und „redmean“ machen aus Grün #27ae60 „Grey“, CIE76 aus Blau
    /// #2e86c1 „Grey“; CIEDE2000 liefert Dark Green, Powder Blue, Dark Red usw.
    pub fn naechstes(&self, rgb: [u8; 3]) -> &Garn {
        let ziel = lab(rgb);
        self.garne
            .iter()
            .map(|g| (de2000(ziel, lab(g.rgb)), g))
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, g)| g)
            .expect("Garnkarte ist nie leer")
    }
}

/// Garnkarten (`*.tch`) eines Ordners, nach Namen sortiert.
pub fn verzeichnis(ordner: &Path) -> Vec<(String, PathBuf)> {
    let mut v: Vec<(String, PathBuf)> = std::fs::read_dir(ordner)
        .map(|it| {
            it.flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("tch")))
                .map(|p| (p.file_stem().unwrap_or_default().to_string_lossy().to_string(), p))
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

/// Karte nach Namen aus einem Ordner laden.
pub fn laden(ordner: &Path, name: &str) -> Result<Garnkarte, String> {
    let (_, pfad) = verzeichnis(ordner)
        .into_iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("Garnkarte „{name}“ nicht gefunden in {}", ordner.display()))?;
    Garnkarte::lesen(&pfad)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ordner() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Analyse/daten/garnkarten")
    }

    #[test]
    fn text_und_naechstes() {
        let k = Garnkarte::aus_text(
            "Test",
            "1,Default,Green,A,0,255,0\r\n2,Default,Blue,A,0,0,255\r\n3,Default,Red,A,255,0,0\r\n",
        )
        .unwrap();
        assert_eq!(k.naechstes([10, 10, 200]).name, "Blue");
        assert_eq!(k.naechstes([0xc0, 0x39, 0x2b]).name, "Red");
        assert_eq!(k.finden("02").unwrap().name, "Blue");
        let f = k.garne[1].farbe();
        assert_eq!((f.nummer, f.marke.as_str(), f.rgb), (2, "Default", [0, 0, 255]));
        assert!(Garnkarte::aus_text("x", "1,a,b").is_err());
    }

    /// Wie die Maschine (20.11.2026): Standardfarbe „Default“ Nr. 2 „Blue“; Isacord-Farben von Nr. 21.
    #[test]
    fn karten_der_cd() {
        let o = ordner();
        if !o.is_dir() {
            eprintln!("übersprungen: {} fehlt", o.display());
            return;
        }
        assert!(verzeichnis(&o).len() >= 30);
        let d = laden(&o, "Default").unwrap();
        assert_eq!(d.garne.len(), 20);
        assert_eq!(
            d.garne[1].farbe(),
            Farbe {
                rgb: [0, 0, 255],
                nummer: 2,
                marke: "Default".into(),
                name: "Blue".into()
            }
        );
        let i = laden(&o, "Isacord 40 wt-numerical").unwrap();
        assert_eq!(i.marke(), "Isacord 40");
        let braun = i.finden("1311").unwrap().farbe();
        assert_eq!(
            (braun.rgb, braun.nummer, braun.marke.as_str(), braun.name.as_str()),
            ([240, 140, 75], 1311, "Isacord 40", "rich brown")
        );
        assert_eq!(i.naechstes([240, 204, 51]).nummer, "0821");
        // CIEDE2000: sinnvolle Zuordnung auch bei der kleinen Default-Karte
        for (hex, name) in [
            ([0x27, 0xae, 0x60], "Dark Green"),
            ([0xc0, 0x39, 0x2b], "Dark Red"),
            ([0x2e, 0x86, 0xc1], "Powder Blue"),
            ([0x1f, 0x3a, 0x93], "Dark Blue"),
            ([0xf3, 0x9c, 0x12], "Orange"),
        ] {
            assert_eq!(d.naechstes(hex).name, name);
        }
        // Referenzwert aus Sharma et al. 2005 (Paar 1): ΔE00 = 2,0425
        let e = de2000([50.0, 2.6772, -79.7751], [50.0, 0.0, -82.7485]);
        assert!((e - 2.0425).abs() < 1e-4, "{e}");
        assert_eq!(i.finden("10").unwrap().farbe().nummer, 10);
    }
}
