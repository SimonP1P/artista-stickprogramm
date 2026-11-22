//! Computer-Schriften (TrueType/OpenType, auch Sammlungen .ttc) lesen: Umrisse der gebrauchten Zeichen, Vorschub,
//! Höhe der Großbuchstaben. Bibliothek `ttf-parser` (nur Dateiformat). Modell und Satz: `stick_kern::schrift_ttf`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use stick_kern::Design;
use stick_kern::schrift_ttf::{TtfSchrift, TtfZeichen};
use ttf_parser::{Face, OutlineBuilder, name_id};

/// Eine installierte Schrift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchriftEintrag {
    /// vollständiger Name („Arial Bold“)
    pub name: String,
    pub familie: String,
    pub stil: String,
    pub pfad: PathBuf,
    /// Index in einer Sammlung (.ttc), sonst 0
    pub index: u32,
    /// Variable Schrift: Strichstärke (Achse `wght`), die gesetzt wird; None = Grundeinstellung der Datei
    pub gewicht: Option<u16>,
}

/// Strichstärken, die eine variable Schrift zusätzlich anbietet, wenn ihre Grundeinstellung eine andere ist: viele
/// Google Fonts liegen als variable Datei vor, deren Grundeinstellung die dünnste Stärke ist („Montserrat Thin“,
/// „Cormorant Garamond Light“) – zum Sticken zu dünn. Wer die Familie wählt, bekommt „Regular“ (`finden`).
/// Auswahl Regular/Bold: **eigene Festlegung**.
pub const STAERKEN: [(u16, &str); 2] = [(400, "Regular"), (700, "Bold")];

/// Umrisse sammeln; Kurven gleichmäßig in Strecken zerlegt (Toleranz in Schrift-Einheiten).
struct Sammler {
    ringe: Vec<Vec<(f64, f64)>>,
    aktuell: Vec<(f64, f64)>,
    toleranz: f64,
}

impl Sammler {
    fn schliessen(&mut self) {
        let mut r = std::mem::take(&mut self.aktuell);
        if r.len() >= 2 && r.first() == r.last() {
            r.pop();
        }
        if r.len() >= 3 {
            self.ringe.push(r);
        }
    }

    fn letzter(&self) -> (f64, f64) {
        *self.aktuell.last().unwrap_or(&(0.0, 0.0))
    }

    /// Zahl der Teilstrecken aus der Länge des Kontrollpolygons
    fn teile(&self, punkte: &[(f64, f64)]) -> usize {
        let l: f64 = punkte
            .windows(2)
            .map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1))
            .sum();
        ((l / self.toleranz).sqrt().ceil() as usize).clamp(2, 64)
    }
}

impl OutlineBuilder for Sammler {
    fn move_to(&mut self, x: f32, y: f32) {
        self.schliessen();
        self.aktuell.push((f64::from(x), f64::from(y)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.aktuell.push((f64::from(x), f64::from(y)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p0 = self.letzter();
        let (p1, p2) = ((f64::from(x1), f64::from(y1)), (f64::from(x), f64::from(y)));
        let n = self.teile(&[p0, p1, p2]);
        for k in 1..=n {
            let t = k as f64 / n as f64;
            let u = 1.0 - t;
            self.aktuell.push((
                u * u * p0.0 + 2.0 * u * t * p1.0 + t * t * p2.0,
                u * u * p0.1 + 2.0 * u * t * p1.1 + t * t * p2.1,
            ));
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p0 = self.letzter();
        let (p1, p2, p3) = (
            (f64::from(x1), f64::from(y1)),
            (f64::from(x2), f64::from(y2)),
            (f64::from(x), f64::from(y)),
        );
        let n = self.teile(&[p0, p1, p2, p3]);
        for k in 1..=n {
            let t = k as f64 / n as f64;
            let u = 1.0 - t;
            let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
            self.aktuell.push((
                a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0,
                a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1,
            ));
        }
    }

    fn close(&mut self) {
        self.schliessen();
    }
}

/// Schrift aus den Bytes einer Datei: nur die gebrauchten Zeichen. Höhe der Großbuchstaben aus OS/2, sonst aus dem „H“.
/// `gewicht`: Strichstärke einer variablen Schrift (Achse `wght`, `SchriftEintrag::gewicht`).
pub fn schrift_aus_daten(
    name: &str,
    daten: &[u8],
    index: u32,
    gewicht: Option<u16>,
    zeichen: &BTreeSet<char>,
) -> Result<TtfSchrift, String> {
    let mut face = Face::parse(daten, index).map_err(|e| format!("Schrift „{name}“: {e}"))?;
    if let Some(w) = gewicht {
        face.set_variation(ttf_parser::Tag::from_bytes(b"wght"), f32::from(w));
    }
    let upem = f64::from(face.units_per_em());
    let toleranz = (upem / 1000.0).max(0.5);
    let h_hoehe = face
        .glyph_index('H')
        .and_then(|g| face.glyph_bounding_box(g))
        .map(|b| f64::from(b.y_max));
    let versal_hoehe = face
        .capital_height()
        .map(f64::from)
        .filter(|&h| h > 0.0)
        .or(h_hoehe)
        .unwrap_or(f64::from(face.ascender()) * 0.7);
    let leerzeichen = face
        .glyph_index(' ')
        .and_then(|g| face.glyph_hor_advance(g))
        .map_or(upem * 0.25, f64::from);
    let mut s = TtfSchrift {
        name: name.to_string(),
        versal_hoehe,
        leerzeichen,
        zeichen: Default::default(),
    };
    for &ch in zeichen.iter().filter(|c| !c.is_whitespace()) {
        let Some(g) = face.glyph_index(ch) else {
            continue; // fehlt in der Schrift → wird beim Satz gemeldet
        };
        let mut sammler = Sammler {
            ringe: Vec::new(),
            aktuell: Vec::new(),
            toleranz,
        };
        let _ = face.outline_glyph(g, &mut sammler);
        sammler.schliessen();
        s.zeichen.insert(
            ch,
            TtfZeichen {
                ringe: sammler.ringe,
                vorschub: face.glyph_hor_advance(g).map_or(upem * 0.5, f64::from),
            },
        );
    }
    Ok(s)
}

/// Strichstärken einer Schrift (`stick_kern::stickbarkeit`, Probezeichen `PROBE`).
pub fn strichstaerken(e: &SchriftEintrag) -> Option<stick_kern::stickbarkeit::Strichstaerken> {
    use stick_kern::stickbarkeit::{PROBE, strichstaerken, ttf_flaechen};
    let daten = std::fs::read(&e.pfad).ok()?;
    let s = schrift_aus_daten(&e.name, &daten, e.index, e.gewicht, &PROBE.chars().collect()).ok()?;
    let (z, bezug) = ttf_flaechen(&s, PROBE);
    strichstaerken(&z, bezug)
}

/// Ordner mit installierten Schriften (Windows, macOS, Linux).
pub fn system_ordner() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(w) = std::env::var("WINDIR") {
        v.push(PathBuf::from(w).join("Fonts"));
    }
    if let Ok(l) = std::env::var("LOCALAPPDATA") {
        v.push(PathBuf::from(l).join("Microsoft").join("Windows").join("Fonts"));
    }
    v.extend(
        [
            "/System/Library/Fonts",
            "/Library/Fonts",
            "/usr/share/fonts",
            "/usr/local/share/fonts",
        ]
        .map(PathBuf::from),
    );
    if let Ok(h) = std::env::var("HOME") {
        v.push(PathBuf::from(&h).join("Library").join("Fonts"));
        v.push(PathBuf::from(&h).join(".fonts"));
    }
    v.into_iter().filter(|p| p.is_dir()).collect()
}

/// Obere Hälfte (0x80–0xFF) der Kodierung MacRoman – Namen älterer Apple-Schriften stehen nur so in der Datei.
const MAC_ROMAN: &str = "ÄÅÇÉÑÖÜáàâäãåçéèêëíìîïñóòôöõúùûü†°¢£§•¶ß®©™´¨≠ÆØ∞±≤≥¥µ∂∑∏π∫ªºΩæø¿¡¬√ƒ≈∆«»…\u{a0}ÀÃÕŒœ–—“”‘’÷◊ÿŸ⁄€‹›ﬁﬂ‡·‚„‰ÂÊÁËÈÍÎÏÌÓÔ\u{f8ff}ÒÚÛÙıˆ˜¯˘˙˚¸˝˛ˇ";

/// Text eines Namenseintrags: Unicode (Windows/Unicode-Plattform) oder MacRoman (Mac-Plattform, Kodierung 0).
fn name_text(n: &ttf_parser::name::Name) -> Option<String> {
    if let Some(t) = n.to_string() {
        return Some(t);
    }
    if n.platform_id == ttf_parser::PlatformId::Macintosh && n.encoding_id == 0 {
        let oben: Vec<char> = MAC_ROMAN.chars().collect();
        return Some(n.name.iter().map(|&b| if b < 0x80 { b as char } else { oben[b as usize - 0x80] }).collect());
    }
    None
}

fn namen(face: &Face) -> (String, String, String) {
    // je Namensart: Englisch (Windows 0x0409, Mac 0) vor anderen Sprachen
    let lies = |ids: &[u16]| -> Option<String> {
        for &id in ids {
            let mut beste: Option<String> = None;
            for n in face.names().into_iter().filter(|n| n.name_id == id) {
                if let Some(t) = name_text(&n).map(|t| t.trim().to_string()).filter(|t| !t.is_empty()) {
                    let englisch = match n.platform_id {
                        ttf_parser::PlatformId::Windows => n.language_id == 0x0409,
                        ttf_parser::PlatformId::Macintosh => n.language_id == 0,
                        _ => false,
                    };
                    if englisch && n.platform_id == ttf_parser::PlatformId::Windows {
                        return Some(t);
                    }
                    if englisch || beste.is_none() {
                        beste = Some(t);
                    }
                }
            }
            if beste.is_some() {
                return beste;
            }
        }
        None
    };
    let familie = lies(&[name_id::TYPOGRAPHIC_FAMILY, name_id::FAMILY]).unwrap_or_default();
    let stil = lies(&[name_id::TYPOGRAPHIC_SUBFAMILY, name_id::SUBFAMILY]).unwrap_or_default();
    let voll = lies(&[name_id::FULL_NAME]).unwrap_or_else(|| format!("{familie} {stil}"));
    (voll, familie, stil)
}

/// Schriftdateien über dieser Größe (Byte) werden nicht gelesen.
pub const GROESSTE_DATEI: u64 = 40_000_000;

/// Alle Schriften mit lateinischen Buchstaben in den Ordnern (rekursiv), nach Namen sortiert; Dateien über
/// `GROESSTE_DATEI` und interne Schriften von macOS („.…“) fehlen.
pub fn verzeichnis(ordner: &[PathBuf]) -> Vec<SchriftEintrag> {
    fn dateien(o: &Path, out: &mut Vec<PathBuf>, tiefe: usize) {
        let Ok(it) = std::fs::read_dir(o) else { return };
        for e in it.flatten() {
            let p = e.path();
            if p.is_dir() && tiefe < 4 {
                dateien(&p, out, tiefe + 1);
            } else if p
                .extension()
                .is_some_and(|x| ["ttf", "otf", "ttc", "otc"].iter().any(|e| x.eq_ignore_ascii_case(e)))
            {
                out.push(p);
            }
        }
    }
    let mut pfade = Vec::new();
    for o in ordner {
        dateien(o, &mut pfade, 0);
    }
    let mut v = Vec::new();
    for pfad in pfade {
        // sehr große Dateien (Emoji, CJK-Sammlungen) sind für Stickschrift ohne Nutzen und kosten Sekunden
        if std::fs::metadata(&pfad).is_ok_and(|m| m.len() > GROESSTE_DATEI) {
            continue;
        }
        let Ok(daten) = std::fs::read(&pfad) else { continue };
        let anzahl = ttf_parser::fonts_in_collection(&daten).unwrap_or(1);
        for index in 0..anzahl {
            if let Ok(face) = Face::parse(&daten, index) {
                let (name, familie, stil) = namen(&face);
                // „.SF NS“, „.Aqua Kana“ …: interne Schriften von macOS, nicht für Anwender gedacht; ohne lateinische
                // Buchstaben (arabische, Braille-, Symbolschriften …) für Stickschrift ohne Nutzen
                if name.is_empty() || name.starts_with('.') || !"AZaz".chars().all(|c| face.glyph_index(c).is_some()) {
                    continue;
                }
                // variable Schrift mit anderer Grundeinstellung: dazu Regular und Bold (`STAERKEN`)
                if let Some(achse) = face
                    .variation_axes()
                    .into_iter()
                    .find(|a| a.tag == ttf_parser::Tag::from_bytes(b"wght"))
                {
                    for (w, s) in STAERKEN {
                        let wf = f32::from(w);
                        if (achse.min_value..=achse.max_value).contains(&wf) && (achse.def_value - wf).abs() > 0.5 {
                            v.push(SchriftEintrag {
                                name: format!("{familie} {s}"),
                                familie: familie.clone(),
                                stil: s.into(),
                                pfad: pfad.clone(),
                                index,
                                gewicht: Some(w),
                            });
                        }
                    }
                }
                v.push(SchriftEintrag {
                    name,
                    familie,
                    stil,
                    pfad: pfad.clone(),
                    index,
                    gewicht: None,
                });
            }
        }
    }
    v.sort_by_key(|e| e.name.to_lowercase());
    v.dedup_by(|a, b| a.name == b.name);
    v
}

/// Schrift nach Namen suchen: Pfad einer Datei, voller Name, Familie (bevorzugt „Regular“), Dateiname.
pub fn finden(name: &str, liste: &[SchriftEintrag]) -> Option<SchriftEintrag> {
    let datei = Path::new(name);
    if datei.is_file() {
        return Some(SchriftEintrag {
            name: name.into(),
            familie: name.into(),
            stil: String::new(),
            pfad: datei.to_path_buf(),
            index: 0,
            gewicht: None,
        });
    }
    let gleich = |a: &str| a.eq_ignore_ascii_case(name);
    let normal = |s: &str| {
        ["regular", "normal", "standard", "roman", "book"]
            .iter()
            .any(|n| s.eq_ignore_ascii_case(n))
    };
    liste
        .iter()
        .find(|e| gleich(&e.name))
        .or_else(|| liste.iter().find(|e| gleich(&e.familie) && normal(&e.stil)))
        .or_else(|| liste.iter().find(|e| gleich(&e.familie)))
        .or_else(|| {
            liste.iter().find(|e| {
                e.pfad
                    .file_stem()
                    .is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case(name))
            })
        })
        .cloned()
}

/// Die Computer-Schriften laden, die das Design braucht (`Design::ttf_bedarf`), aus den Ordnern (sonst
/// [`system_ordner`]).
pub fn design_schriften_laden(design: &mut Design, ordner: &[PathBuf]) -> Result<(), String> {
    if design.ttf_bedarf().is_empty() {
        return Ok(());
    }
    let ordner = if ordner.is_empty() {
        system_ordner()
    } else {
        ordner.to_vec()
    };
    design_schriften_laden_aus(design, &verzeichnis(&ordner))
}

/// Wie `design_schriften_laden`, aus einem schon gelesenen Verzeichnis (Zwischenspeicher der Oberfläche).
pub fn design_schriften_laden_aus(design: &mut Design, liste: &[SchriftEintrag]) -> Result<(), String> {
    let bedarf = design.ttf_bedarf();
    for (name, zeichen) in bedarf {
        let e = finden(&name, liste)
            .ok_or_else(|| format!("Schrift „{name}“ nicht gefunden (installierte Schriften: stick schriften)"))?;
        let daten = std::fs::read(&e.pfad).map_err(|err| format!("{}: {err}", e.pfad.display()))?;
        design
            .ttf_schriften
            .insert(name.clone(), schrift_aus_daten(&name, &daten, e.index, e.gewicht, &zeichen)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mit einer installierten Schrift (Windows: Arial; sonst die erste gefundene): Text berechnen.
    #[test]
    fn systemschrift_text() {
        let liste = verzeichnis(&system_ordner());
        let Some(e) = finden("Arial", &liste).or_else(|| liste.first().cloned()) else {
            eprintln!("übersprungen: keine Schriften installiert");
            return;
        };
        let json = format!(
            r#"{{"objekte":[{{"art":"ttf","text":"AOg","schrift":"{}","hoehe":20}}]}}"#,
            e.pfad.display().to_string().replace('\\', "/")
        );
        let mut d = Design::aus_json(&json).unwrap();
        design_schriften_laden(&mut d, &[]).unwrap();
        let s = &d.ttf_schriften.values().next().unwrap();
        assert!(s.versal_hoehe > 0.0);
        let (objekte, fehlend) = d.objekte_aufgeloest().unwrap();
        assert!(fehlend.is_empty());
        assert_eq!(objekte.len(), 3);
        let f = d.berechnen().unwrap();
        assert!(
            f.anzahl(stick_kern::Art::Stich) > 500,
            "{}",
            f.anzahl(stick_kern::Art::Stich)
        );
        let (_, y0, _, y1) = f.begrenzung().unwrap();
        // „A“/„O“ 20 mm hoch, „g“ reicht unter die Grundlinie
        assert!(y1 > 18.0 && y1 < 23.0 && y0 < -2.0, "y {y0} … {y1}");
    }
}
