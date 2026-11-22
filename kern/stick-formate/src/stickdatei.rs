//! **Fremde Stickdateien lesen** (Nachtauftrag N7, ROADMAP 4b.13, Bericht `Analyse_Teil3/27_stickdateien.md`):
//! DST (Tajima), PES/PEC (Brother, Babylock, Bernette Deco), JEF (Janome), VP3 (Husqvarna/Viking/Pfaff) und EXP.
//!
//! Wie im Original ohne Objektumwandlung („Grade D“, Handbuch Kap. 28 S. 256–257): Das Ergebnis sind die Einstiche je
//! Farbe, in Züge geteilt an Sprüngen und Fadenschnitten. Koordinaten in mm, +y oben, Mitte des Designs = 0,0
//! (eigene Festlegung).
//!
//! Die Dateiformate selbst sind neutrale Technik (allgemein bekannte Aufbauten). Die **Farben** von PES und JEF stehen
//! in der Datei nur als Nummer: PES-Nummer → Farbname nach dem PEC-Format, Farbname → RGB aus der Original-Garnkarte
//! `Thread Colors - PES.tch`; JEF-Nummer → Janome-Garnnummer, RGB aus `Thread Colors - SEW.tch` (Janome) des Originals.
//! DST und EXP haben keine Farben (Standardfolge der Original-Karte `Default.tch`).
//!
//! ART-Designs des Originals liest `crate::art` (je Objekt des Originals ein Block, N11, Bericht 27 §6).

/// Endungen, die `lesen` kennt (klein geschrieben).
pub const ENDUNGEN: &[&str] = &["art", "dst", "pes", "pec", "jef", "vp3", "exp"];

/// Ein Farbblock: Einstiche in Zügen (ein Zug endet an einem Sprung oder Fadenschnitt).
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Farbe aus der Datei (ohne Angabe: Standardfolge)
    pub farbe: [u8; 3],
    /// Farbname aus der Datei oder Farbtabelle
    pub name: Option<String>,
    /// Züge in mm, +y oben
    pub zuege: Vec<Vec<(f64, f64)>>,
}

/// Gelesene Stickdatei.
#[derive(Debug, Clone, PartialEq)]
pub struct Stickdatei {
    /// Format („DST“, „PES“ …)
    pub format: &'static str,
    /// Name aus dem Dateikopf (DST „LA:“, PEC-Label), falls vorhanden
    pub name: Option<String>,
    pub bloecke: Vec<Block>,
    /// Hinweise beim Lesen (z. B. unbekannte Farbnummer)
    pub hinweise: Vec<String>,
}

impl Stickdatei {
    /// Zahl der Einstiche
    pub fn stiche(&self) -> usize {
        self.bloecke.iter().flat_map(|b| &b.zuege).map(|z| z.len()).sum()
    }
    /// Umschließendes Rechteck (x0, y0, x1, y1) in mm
    pub fn grenzen(&self) -> Option<(f64, f64, f64, f64)> {
        let mut it = self.bloecke.iter().flat_map(|b| &b.zuege).flatten();
        let &(x, y) = it.next()?;
        Some(it.fold((x, y, x, y), |(a, b, c, d), &(x, y)| (a.min(x), b.min(y), c.max(x), d.max(y))))
    }
}

/// Standardfolge ohne Farben in der Datei: die 20 Farben der Original-Karte `Default.tch` (Analyse/daten/garnkarten).
pub(crate) const STANDARD: [[u8; 3]; 20] = [
    [0, 255, 0],
    [0, 0, 255],
    [255, 0, 0],
    [255, 0, 255],
    [255, 255, 0],
    [0, 255, 255],
    [128, 0, 0],
    [0, 128, 0],
    [0, 0, 128],
    [128, 128, 0],
    [128, 0, 128],
    [0, 128, 128],
    [192, 192, 192],
    [128, 128, 128],
    [255, 128, 0],
    [255, 128, 128],
    [128, 255, 128],
    [128, 128, 255],
    [255, 255, 255],
    [0, 0, 0],
];

/// PEC-Farbnummer 1–64 → Name (Reihenfolge des PEC-Formats) und RGB aus `Thread Colors - PES.tch` (Original).
const PEC: [(&str, [u8; 3]); 64] = [
    ("Prussian Blue", [26, 10, 148]),
    ("Blue", [15, 117, 255]),
    ("Teal Green", [0, 147, 76]),
    ("Corn Flower Blue", [186, 189, 254]),
    ("Red", [236, 0, 0]),
    ("Reddish Brown", [228, 153, 90]),
    ("Magenta", [204, 72, 171]),
    ("Light Lilac", [253, 196, 250]),
    ("Lilac", [221, 132, 205]),
    ("Mint Green", [107, 211, 138]),
    ("Deep Gold", [228, 169, 69]),
    ("Orange", [255, 189, 66]),
    ("Yellow", [255, 230, 0]),
    ("Lime Green", [108, 217, 0]),
    ("Brass", [193, 169, 65]),
    ("Silver", [181, 173, 151]),
    ("Russet Brown", [186, 156, 95]),
    ("Cream Brown", [250, 245, 158]),
    ("Pewter", [128, 128, 128]),
    ("Black", [0, 0, 0]),
    ("Ultramarine", [0, 28, 223]),
    ("Royal Purple", [223, 0, 184]),
    ("Dark Gray", [98, 98, 98]),
    ("Dark Brown", [105, 38, 13]),
    ("Deep Rose", [255, 0, 96]),
    ("Light Brown", [191, 130, 0]),
    ("Salmon Pink", [243, 145, 120]),
    ("Vermilion", [255, 104, 5]),
    ("White", [240, 240, 240]),
    ("Violet", [200, 50, 205]),
    ("Seacrest", [176, 191, 155]),
    ("Sky Blue", [101, 191, 235]),
    ("Pumpkin", [255, 186, 4]),
    ("Cream Yellow", [255, 240, 108]),
    ("Khaki", [254, 202, 21]),
    ("Clay Brown", [243, 129, 1]),
    ("Leaf Green", [55, 169, 35]),
    ("Peacock Blue", [35, 70, 95]),
    ("Gray", [166, 166, 149]),
    ("Warm Gray", [206, 191, 166]),
    ("Dark Olive", [150, 170, 2]),
    ("Linen", [255, 227, 198]),
    ("Pink", [255, 153, 215]),
    ("Deep Green", [0, 112, 4]),
    ("Lavender", [237, 204, 251]),
    ("Wisteria Violet", [192, 137, 216]),
    ("Beige", [231, 217, 180]),
    ("Carmine", [233, 14, 134]),
    ("Amber Red", [207, 104, 41]),
    ("Olive Green", [64, 134, 21]),
    ("Dark Fuchsia", [219, 23, 151]),
    ("Tangerine", [255, 167, 4]),
    ("Light Blue", [185, 255, 255]),
    ("Emerald Green", [34, 137, 39]),
    ("Purple", [182, 18, 205]),
    ("Moss Green", [0, 170, 0]),
    ("Flesh Pink", [254, 169, 220]),
    ("Harvest Gold", [254, 213, 16]),
    ("Electric Blue", [0, 151, 223]),
    ("Lemon Yellow", [255, 255, 132]),
    ("Fresh Green", [207, 231, 116]),
    ("Applique Material", [255, 200, 100]),
    ("Applique Position", [255, 200, 200]),
    ("Applique", [255, 200, 200]),
];

/// JEF-Farbnummer 1–30 → Janome-Name, RGB aus `Thread Colors - SEW.tch` (Original) bzw. – wo die Karte die Farbe nicht
/// hat – verbreitete Werte (Sicherheit mittel, Bericht 27 §3).
const JEF: [(&str, [u8; 3]); 30] = [
    ("Black", [0, 0, 0]),
    ("White", [240, 240, 240]),
    ("Yellow", [255, 255, 23]),
    ("Orange", [255, 102, 0]),
    ("Olive Green", [47, 89, 51]),
    ("Green", [35, 115, 54]),
    ("Sky", [101, 194, 200]),
    ("Purple", [171, 90, 150]),
    ("Pink", [246, 105, 160]),
    ("Red", [255, 0, 0]),
    ("Brown", [156, 100, 69]),
    ("Blue", [11, 47, 132]),
    ("Gold", [228, 195, 93]),
    ("Dark Brown", [72, 26, 5]),
    ("Pale Violet", [172, 156, 199]),
    ("Pale Yellow", [253, 245, 181]),
    ("Pale Pink", [249, 153, 183]),
    ("Peach", [250, 179, 129]),
    ("Beige", [215, 189, 164]),
    ("Wine Red", [151, 5, 51]),
    ("Pale Sky", [160, 184, 204]),
    ("Yellow Green", [127, 194, 28]),
    ("Silver Grey", [229, 229, 229]),
    ("Gray", [136, 155, 155]),
    ("Pale Aqua", [152, 214, 189]),
    ("Baby Blue", [178, 225, 227]),
    ("Powder Blue", [54, 139, 160]),
    ("Bright Blue", [79, 131, 171]),
    ("Slate Blue", [56, 106, 145]),
    ("Navy Blue", [7, 22, 80]),
];

/// Sammelt Einstiche (in 0,1 mm, +y oben) zu Blöcken und Zügen.
struct Sammler {
    x: i64,
    y: i64,
    bloecke: Vec<Block>,
    neuer_zug: bool,
}

impl Sammler {
    fn neu(farbe: [u8; 3], name: Option<String>) -> Sammler {
        Sammler {
            x: 0,
            y: 0,
            bloecke: vec![Block {
                farbe,
                name,
                zuege: Vec::new(),
            }],
            neuer_zug: true,
        }
    }
    fn stich(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
        let q = (self.x as f64 / 10.0, self.y as f64 / 10.0);
        let b = self.bloecke.last_mut().unwrap();
        if self.neuer_zug || b.zuege.is_empty() {
            b.zuege.push(vec![q]);
            self.neuer_zug = false;
        } else {
            b.zuege.last_mut().unwrap().push(q);
        }
    }
    /// Sprung (ohne Einstich): der nächste Einstich beginnt einen neuen Zug
    fn sprung(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
        self.neuer_zug = true;
    }
    fn schnitt(&mut self) {
        self.neuer_zug = true;
    }
    fn farbwechsel(&mut self, farbe: [u8; 3], name: Option<String>) {
        // leere Blöcke (zwei Farbwechsel hintereinander) zählen nicht
        let b = self.bloecke.last_mut().unwrap();
        if b.zuege.is_empty() {
            b.farbe = farbe;
            b.name = name;
        } else {
            self.bloecke.push(Block {
                farbe,
                name,
                zuege: Vec::new(),
            });
        }
        self.neuer_zug = true;
    }
    fn fertig(mut self, format: &'static str, name: Option<String>, hinweise: Vec<String>) -> Stickdatei {
        self.bloecke.retain(|b| !b.zuege.is_empty());
        let mut d = Stickdatei {
            format,
            name: name.filter(|n| !n.is_empty()),
            bloecke: self.bloecke,
            hinweise,
        };
        zentrieren(&mut d);
        d
    }
}

/// Mitte des Designs auf 0,0
pub(crate) fn zentrieren(d: &mut Stickdatei) {
    if let Some((x0, y0, x1, y1)) = d.grenzen() {
        let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        for q in d.bloecke.iter_mut().flat_map(|b| &mut b.zuege).flatten() {
            *q = (rund(q.0 - mx), rund(q.1 - my));
        }
    }
}

fn rund(v: f64) -> f64 {
    (v * 1e6).round() / 1e6
}

/// Farben der Standardfolge reihum
struct Folge(usize);
impl Folge {
    fn naechste(&mut self) -> [u8; 3] {
        let c = STANDARD[self.0 % STANDARD.len()];
        self.0 += 1;
        c
    }
}

/// Stickdatei lesen; `endung` ohne Punkt (Groß/klein egal).
pub fn lesen(daten: &[u8], endung: &str) -> Result<Stickdatei, String> {
    let d = match endung.to_ascii_lowercase().as_str() {
        "dst" => dst(daten),
        "pes" => pes(daten),
        "pec" => pec_datei(daten),
        "jef" => jef(daten),
        "vp3" => vp3(daten),
        "exp" => exp(daten),
        "art" => crate::art::lesen(daten),
        e => Err(format!("Format „{e}“ wird nicht gelesen (möglich: ART, DST, PES, PEC, JEF, VP3, EXP)")),
    }?;
    if d.bloecke.is_empty() {
        return Err(format!("{}: keine Stiche in der Datei", d.format));
    }
    Ok(d)
}

// ------------------------------------------------------------------------------------------------ DST (Tajima)

/// Ein DST-Datensatz (3 Byte): Bewegung in 0,1 mm (+y oben) und Art.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DstArt {
    Stich,
    Sprung,
    Farbwechsel,
    Ende,
}

/// DST-Datensatz entschlüsseln: Bits für ±1, ±3, ±9, ±27, ±81 je Achse.
pub(crate) fn dst_satz(b: [u8; 3]) -> (i64, i64, DstArt) {
    let bit = |byte: u8, n: u8| (byte >> n) & 1 == 1;
    let mut x = 0i64;
    let mut y = 0i64;
    // (Byte, Bit, Wert)
    for (byte, n, w) in [(0, 0, 1), (0, 1, -1), (0, 2, 9), (0, 3, -9), (1, 0, 3), (1, 1, -3), (1, 2, 27), (1, 3, -27)] {
        if bit(b[byte], n) {
            x += w;
        }
    }
    for (byte, n, w) in [(0, 7, 1), (0, 6, -1), (0, 5, 9), (0, 4, -9), (1, 7, 3), (1, 6, -3), (1, 5, 27), (1, 4, -27)] {
        if bit(b[byte], n) {
            y += w;
        }
    }
    if bit(b[2], 2) {
        x += 81;
    }
    if bit(b[2], 3) {
        x -= 81;
    }
    if bit(b[2], 5) {
        y += 81;
    }
    if bit(b[2], 4) {
        y -= 81;
    }
    let art = if b[2] & 0xF3 == 0xF3 {
        DstArt::Ende
    } else if b[2] & 0xC3 == 0xC3 {
        DstArt::Farbwechsel
    } else if b[2] & 0x83 == 0x83 {
        DstArt::Sprung
    } else {
        DstArt::Stich
    };
    (x, y, art)
}

/// Feld „LA:“ im DST-Kopf
fn dst_name(kopf: &[u8]) -> Option<String> {
    let t: String = kopf.iter().map(|&c| c as char).collect();
    let i = t.find("LA:")?;
    let n: String = t[i + 3..].chars().take_while(|&c| c != '\r' && c != '\n' && c != '\u{1a}').take(16).collect();
    Some(n.trim().to_string())
}

fn dst(daten: &[u8]) -> Result<Stickdatei, String> {
    if daten.len() < 512 {
        return Err("DST: Datei zu kurz (Kopf 512 Byte fehlt)".into());
    }
    let mut farben = Folge(0);
    let mut s = Sammler::neu(farben.naechste(), None);
    for c in daten[512..].chunks_exact(3) {
        let (dx, dy, art) = dst_satz([c[0], c[1], c[2]]);
        match art {
            DstArt::Ende => break,
            DstArt::Stich => s.stich(dx, dy),
            DstArt::Sprung => s.sprung(dx, dy),
            DstArt::Farbwechsel => {
                s.sprung(dx, dy);
                s.farbwechsel(farben.naechste(), None);
            }
        }
    }
    Ok(s.fertig("DST", dst_name(&daten[..512]), Vec::new()))
}

// ------------------------------------------------------------------------------------------------ PES / PEC (Brother)

fn u32le(d: &[u8], i: usize) -> Option<u32> {
    d.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn pes(daten: &[u8]) -> Result<Stickdatei, String> {
    if daten.len() < 12 || &daten[..4] != b"#PES" {
        return Err("PES: Kennung „#PES“ fehlt".into());
    }
    let pec = u32le(daten, 8).unwrap() as usize;
    if pec >= daten.len() {
        return Err("PES: Verweis auf den PEC-Teil zeigt hinter das Dateiende".into());
    }
    pec_block(&daten[pec..], "PES")
}

fn pec_datei(daten: &[u8]) -> Result<Stickdatei, String> {
    if daten.len() < 8 || &daten[..8] != b"#PEC0001" {
        return Err("PEC: Kennung „#PEC0001“ fehlt".into());
    }
    pec_block(&daten[8..], "PEC")
}

/// PEC-Teil ab „LA:“: Name (16 Zeichen), bei 48 Zahl der Farbwechsel, ab 49 die Farbnummern, ab 0x210 die Stiche.
fn pec_block(d: &[u8], format: &'static str) -> Result<Stickdatei, String> {
    if d.len() < 0x214 || &d[..3] != b"LA:" {
        return Err(format!("{format}: PEC-Teil unvollständig"));
    }
    let name: String = d[3..19].iter().map(|&c| c as char).collect::<String>().trim().to_string();
    let n = d[48] as usize + 1;
    let mut hinweise = Vec::new();
    let farben: Vec<([u8; 3], Option<String>)> = d[49..49 + n]
        .iter()
        .map(|&i| match PEC.get((i as usize).wrapping_sub(1)) {
            Some((name, rgb)) => (*rgb, Some(name.to_string())),
            None => {
                hinweise.push(format!("unbekannte PEC-Farbnummer {i}"));
                ([220, 220, 220], None)
            }
        })
        .collect();
    let mut nr = 0;
    let mut s = Sammler::neu(farben[0].0, farben[0].1.clone());
    let mut i = 0x210;
    let byte = |i: usize| d.get(i).copied().ok_or_else(|| format!("{format}: Stichdaten enden ohne Endkennung"));
    // 12-Bit-Wert mit Kennbits (0x80 lang, 0x20 Schnitt, 0x10 Sprung) oder 7-Bit-Wert
    let lesen = |i: &mut usize, sprung: &mut bool, schnitt: &mut bool| -> Result<i64, String> {
        let a = byte(*i)?;
        *i += 1;
        if a & 0x80 != 0 {
            *schnitt |= a & 0x20 != 0;
            *sprung |= a & 0x10 != 0;
            let b = byte(*i)?;
            *i += 1;
            let mut v = (((a & 0x0F) as i64) << 8) | b as i64;
            if v & 0x800 != 0 {
                v -= 0x1000;
            }
            Ok(v)
        } else {
            Ok(if a > 0x3F { a as i64 - 0x80 } else { a as i64 })
        }
    };
    loop {
        let (a, b) = (byte(i)?, byte(i + 1)?);
        if a == 0xFF && b == 0x00 {
            break;
        }
        if a == 0xFE && b == 0xB0 {
            i += 3;
            nr += 1;
            let (f, n) = farben.get(nr).cloned().unwrap_or(([220, 220, 220], None));
            s.farbwechsel(f, n);
            continue;
        }
        let (mut sprung, mut schnitt) = (false, false);
        let dx = lesen(&mut i, &mut sprung, &mut schnitt)?;
        let dy = lesen(&mut i, &mut sprung, &mut schnitt)?;
        // PEC: +y nach unten
        if sprung || schnitt {
            s.sprung(dx, -dy);
            if schnitt {
                s.schnitt();
            }
        } else {
            s.stich(dx, -dy);
        }
    }
    Ok(s.fertig(format, Some(name), hinweise))
}

// ------------------------------------------------------------------------------------------------ JEF (Janome)

fn jef(d: &[u8]) -> Result<Stickdatei, String> {
    let start = u32le(d, 0).ok_or("JEF: Kopf fehlt")? as usize;
    let n = u32le(d, 24).ok_or("JEF: Kopf fehlt")? as usize;
    if start > d.len() || n > 256 || 116 + 8 * n > start {
        return Err("JEF: Kopf ungültig".into());
    }
    let mut hinweise = Vec::new();
    let farben: Vec<([u8; 3], Option<String>)> = (0..n)
        .map(|k| {
            let i = u32le(d, 116 + 4 * k).unwrap() as usize;
            match JEF.get(i.wrapping_sub(1)) {
                Some((name, rgb)) => (*rgb, Some(name.to_string())),
                None => {
                    hinweise.push(format!("Janome-Farbnummer {i} ohne Farbtabelle"));
                    (STANDARD[k % STANDARD.len()], None)
                }
            }
        })
        .collect();
    let erste = farben.first().cloned().unwrap_or((STANDARD[0], None));
    let mut s = Sammler::neu(erste.0, erste.1);
    let mut nr = 0;
    let s8 = |b: u8| b as i8 as i64;
    let mut i = start;
    while i + 1 < d.len() {
        let (a, b) = (d[i], d[i + 1]);
        i += 2;
        if a != 0x80 {
            s.stich(s8(a), s8(b));
            continue;
        }
        match b {
            0x10 => break,
            0x01 | 0x02 => {
                let (Some(&x), Some(&y)) = (d.get(i), d.get(i + 1)) else { break };
                i += 2;
                s.sprung(s8(x), s8(y));
                if b == 0x01 {
                    nr += 1;
                    let (f, n) = farben.get(nr).cloned().unwrap_or((STANDARD[nr % STANDARD.len()], None));
                    s.farbwechsel(f, n);
                }
            }
            _ => {
                hinweise.push(format!("JEF: unbekannte Steuerkennung 80 {b:02x} – Lesen beendet"));
                break;
            }
        }
    }
    Ok(s.fertig("JEF", None, hinweise))
}

// ------------------------------------------------------------------------------------------------ VP3 (Viking/Pfaff)

struct Leser<'a> {
    d: &'a [u8],
    i: usize,
}

impl<'a> Leser<'a> {
    fn bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
        let b = self.d.get(self.i..self.i + n).ok_or("VP3: Datei endet zu früh")?;
        self.i += n;
        Ok(b)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.bytes(1)?[0])
    }
    fn u16be(&mut self) -> Result<usize, String> {
        let b = self.bytes(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]) as usize)
    }
    fn i32be(&mut self) -> Result<i64, String> {
        let b = self.bytes(4)?;
        Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]) as i64)
    }
    fn u32be(&mut self) -> Result<usize, String> {
        Ok(self.i32be()? as u32 as usize)
    }
    /// Zeichenkette mit vorangestellter Länge (16 Bit, groß-endian)
    fn text(&mut self) -> Result<String, String> {
        let n = self.u16be()?;
        Ok(self.bytes(n)?.iter().filter(|&&c| c != 0).map(|&c| c as char).collect())
    }
    fn ueberspringen(&mut self, n: usize) -> Result<(), String> {
        self.bytes(n).map(|_| ())
    }
}

/// VP3: Kopf „%vsm%“, Texte mit Längenangabe, Designmitte (1/1000 mm), Farbblöcke (Kennung 00 05 00, Länge, Anfang,
/// Garn mit RGB und Namen, dann Stiche in 0,1 mm; 80 01 = langer Stich mit 16-Bit-Werten, 80 03 = Schnitt).
/// Aufbau nach der allgemein bekannten Beschreibung des Formats – **ohne Beispieldatei geprüft** (Bericht 27 §4).
fn vp3(d: &[u8]) -> Result<Stickdatei, String> {
    if d.len() < 6 || &d[..5] != b"%vsm%" {
        return Err("VP3: Kennung „%vsm%“ fehlt".into());
    }
    let mut r = Leser { d, i: 6 };
    r.text()?; // „Produced by …“
    r.ueberspringen(7)?; // 00 02 00, Länge bis Dateiende
    r.text()?; // Bemerkung
    r.ueberspringen(32)?; // Grenzen und Kennwerte
    let mx = r.i32be()?;
    let my = r.i32be()?;
    r.ueberspringen(27)?;
    r.text()?;
    r.ueberspringen(24)?;
    r.text()?;
    let n = r.u16be()?;
    if n == 0 || n > 256 {
        return Err(format!("VP3: unplausible Zahl der Farben ({n})"));
    }
    let mut s: Option<Sammler> = None;
    for _ in 0..n {
        if r.bytes(3)? != [0, 5, 0] {
            return Err("VP3: Farbblock ohne Kennung 00 05 00".into());
        }
        let laenge = r.u32be()?;
        let ende = r.i + laenge;
        if ende > d.len() {
            return Err("VP3: Farbblock reicht über das Dateiende".into());
        }
        // Anfang des Blocks (1/1000 mm, +y oben) relativ zur Designmitte
        let ax = (r.i32be()? + mx) / 100;
        let ay = (r.i32be()? + my) / 100;
        // Garn: Zahl der Farben, Übergang, je Farbe RGB + 2 Byte, Art, Stärke, Nummer, Name, Marke
        let teile = r.u8()? as usize;
        r.u8()?;
        let mut rgb = [128u8, 128, 128];
        for k in 0..teile {
            let b = r.bytes(5)?;
            if k == 0 {
                rgb = [b[0], b[1], b[2]];
            }
        }
        r.ueberspringen(2)?;
        let nummer = r.text()?;
        let name = r.text()?;
        let marke = r.text()?;
        let bez = [marke, nummer, name].into_iter().filter(|t| !t.trim().is_empty()).collect::<Vec<_>>().join(" ");
        let bez = (!bez.is_empty()).then_some(bez);
        r.ueberspringen(15)?;
        let samm = match s.as_mut() {
            None => {
                s = Some(Sammler::neu(rgb, bez));
                s.as_mut().unwrap()
            }
            Some(x) => {
                x.farbwechsel(rgb, bez);
                x
            }
        };
        // zum Anfang des Blocks springen
        let (dx, dy) = (ax - samm.x, ay - samm.y);
        samm.sprung(dx, dy);
        let s8 = |b: u8| b as i8 as i64;
        while r.i + 1 < ende {
            let (a, b) = (d[r.i], d[r.i + 1]);
            r.i += 2;
            if a != 0x80 {
                samm.stich(s8(a), -s8(b));
                continue;
            }
            match b {
                0x01 => {
                    if r.i + 4 > ende {
                        break;
                    }
                    let x = i16::from_be_bytes([d[r.i], d[r.i + 1]]) as i64;
                    let y = i16::from_be_bytes([d[r.i + 2], d[r.i + 3]]) as i64;
                    r.i += 4;
                    samm.stich(x, -y);
                    // 80 02 schließt den langen Stich ab
                    if d.get(r.i..r.i + 2) == Some(&[0x80, 0x02]) {
                        r.i += 2;
                    }
                }
                0x03 => samm.schnitt(),
                _ => {}
            }
        }
        r.i = ende;
    }
    Ok(s.unwrap().fertig("VP3", None, Vec::new()))
}

// ------------------------------------------------------------------------------------------------ EXP

fn exp(daten: &[u8]) -> Result<Stickdatei, String> {
    use stick_kern::stiche::Art;
    let f = crate::exp::exp_lesen(daten);
    let mut farben = Folge(0);
    let mut s = Sammler::neu(farben.naechste(), None);
    let (mut x, mut y) = (0i64, 0i64);
    for st in &f.stiche {
        let (nx, ny) = ((st.x * 10.0).round() as i64, (st.y * 10.0).round() as i64);
        let (dx, dy) = (nx - x, ny - y);
        (x, y) = (nx, ny);
        match st.art {
            Art::Stich => s.stich(dx, dy),
            Art::Sprung => s.sprung(dx, dy),
            Art::Farbwechsel => {
                s.sprung(dx, dy);
                s.farbwechsel(farben.naechste(), None);
            }
        }
    }
    Ok(s.fertig("EXP", None, Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DST-Datensatz kodieren (Gegenstück zu `dst_satz`, ausgeglichene Dreierzahlen) – nur für Tests
    fn dst_kodieren(dx: i64, dy: i64, art: DstArt) -> [u8; 3] {
        // Ziffern −1/0/1 zu 1, 3, 9, 27, 81
        fn ziffern(mut v: i64) -> [i64; 5] {
            let mut z = [0; 5];
            for d in &mut z {
                let r = v.rem_euclid(3);
                *d = if r == 2 { -1 } else { r };
                v = (v - *d) / 3;
            }
            assert_eq!(v, 0, "zu groß");
            z
        }
        let (zx, zy) = (ziffern(dx), ziffern(dy));
        let mut b = [0u8, 0, 0x03];
        let setze = |b: &mut [u8; 3], z: i64, plus: (usize, u8), minus: (usize, u8)| {
            if z == 1 {
                b[plus.0] |= 1 << plus.1;
            } else if z == -1 {
                b[minus.0] |= 1 << minus.1;
            }
        };
        // x: 1 (0,0/0,1), 3 (1,0/1,1), 9 (0,2/0,3), 27 (1,2/1,3), 81 (2,2/2,3)
        setze(&mut b, zx[0], (0, 0), (0, 1));
        setze(&mut b, zx[1], (1, 0), (1, 1));
        setze(&mut b, zx[2], (0, 2), (0, 3));
        setze(&mut b, zx[3], (1, 2), (1, 3));
        setze(&mut b, zx[4], (2, 2), (2, 3));
        // y: 1 (0,7/0,6), 3 (1,7/1,6), 9 (0,5/0,4), 27 (1,5/1,4), 81 (2,5/2,4)
        setze(&mut b, zy[0], (0, 7), (0, 6));
        setze(&mut b, zy[1], (1, 7), (1, 6));
        setze(&mut b, zy[2], (0, 5), (0, 4));
        setze(&mut b, zy[3], (1, 5), (1, 4));
        setze(&mut b, zy[4], (2, 5), (2, 4));
        b[2] |= match art {
            DstArt::Stich => 0,
            DstArt::Sprung => 0x80,
            DstArt::Farbwechsel => 0xC0,
            DstArt::Ende => 0xF3,
        };
        b
    }

    #[test]
    fn dst_saetze_alle_werte() {
        for dx in -121..=121 {
            for dy in [-121, -40, -1, 0, 1, 13, 121] {
                let b = dst_kodieren(dx, dy, DstArt::Stich);
                assert_eq!(dst_satz(b), (dx, dy, DstArt::Stich), "{dx} {dy} {b:02x?}");
            }
        }
        assert_eq!(dst_satz([0, 0, 0xF3]).2, DstArt::Ende);
        assert_eq!(dst_satz(dst_kodieren(5, -7, DstArt::Sprung)), (5, -7, DstArt::Sprung));
        assert_eq!(dst_satz(dst_kodieren(0, 0, DstArt::Farbwechsel)).2, DstArt::Farbwechsel);
        // bekannte Datensätze: Ende 00 00 F3, Farbwechsel 00 00 C3
        assert_eq!(dst_satz([0, 0, 0xC3]), (0, 0, DstArt::Farbwechsel));
        assert_eq!(dst_satz([0x01, 0, 0x03]), (1, 0, DstArt::Stich));
        assert_eq!(dst_satz([0x80, 0, 0x03]), (0, 1, DstArt::Stich));
        assert_eq!(dst_satz([0, 0, 0x23]), (0, 81, DstArt::Stich));
    }

    fn dst_datei(saetze: &[(i64, i64, DstArt)]) -> Vec<u8> {
        let mut kopf = b"LA:Test Herz       \rST:      5\rCO:  1\r".to_vec();
        kopf.resize(512, b' ');
        kopf[511] = 0x1A;
        for &(x, y, a) in saetze {
            kopf.extend(dst_kodieren(x, y, a));
        }
        kopf
    }

    #[test]
    fn dst_lesen() {
        let d = dst_datei(&[
            (0, 0, DstArt::Stich),
            (20, 0, DstArt::Stich),
            (20, 10, DstArt::Stich),
            (50, 0, DstArt::Sprung),
            (10, 0, DstArt::Stich),
            (0, 0, DstArt::Farbwechsel),
            (0, -30, DstArt::Stich),
            (0, 0, DstArt::Ende),
        ]);
        let s = lesen(&d, "DST").unwrap();
        assert_eq!(s.format, "DST");
        assert_eq!(s.name.as_deref(), Some("Test Herz"));
        assert_eq!(s.bloecke.len(), 2);
        assert_eq!(s.bloecke[0].zuege.len(), 2);
        assert_eq!(s.stiche(), 5);
        // Grenzen 0 … 10 mm × −2 … 1 mm → Mitte (5, −0,5) auf 0,0
        assert_eq!(s.bloecke[0].zuege[0], vec![(-5.0, 0.5), (-3.0, 0.5), (-1.0, 1.5)]);
        assert_eq!(s.bloecke[0].zuege[1], vec![(5.0, 1.5)]);
        assert_eq!(s.bloecke[1].zuege[0], vec![(5.0, -1.5)]);
        assert_eq!(s.bloecke[0].farbe, STANDARD[0]);
        assert_eq!(s.bloecke[1].farbe, STANDARD[1]);
    }

    /// PEC-Wert kodieren: kurz (7 Bit) oder lang (12 Bit mit Kennbits)
    fn pec_wert(v: i64, kenn: u8, lang: bool, out: &mut Vec<u8>) {
        if !lang && (-64..64).contains(&v) {
            out.push((v & 0x7F) as u8);
        } else {
            let w = (v & 0xFFF) as u16;
            out.push(0x80 | kenn | (w >> 8) as u8);
            out.push((w & 0xFF) as u8);
        }
    }

    fn pec_teil(farben: &[u8], stiche: &[u8]) -> Vec<u8> {
        let mut p = b"LA:Rose           \r".to_vec();
        p.resize(48, 0x20);
        p.push(farben.len() as u8 - 1);
        p.extend(farben);
        p.resize(0x210, 0x20);
        p.extend(stiche);
        p
    }

    #[test]
    fn pes_lesen() {
        let mut st = Vec::new();
        // Sprung zum Anfang (lang, Kennung 0x10), dann Stiche (PEC: +y unten)
        pec_wert(-100, 0x10, true, &mut st);
        pec_wert(50, 0x10, true, &mut st);
        pec_wert(0, 0, false, &mut st);
        pec_wert(0, 0, false, &mut st);
        pec_wert(30, 0, false, &mut st);
        pec_wert(-20, 0, false, &mut st);
        pec_wert(200, 0, true, &mut st); // langer Stich ohne Kennbits
        pec_wert(0, 0, false, &mut st);
        st.extend([0xFE, 0xB0, 0x02]); // Farbwechsel
        pec_wert(0, 0, false, &mut st);
        pec_wert(10, 0, false, &mut st);
        st.extend([0xFF, 0x00]);
        let pec = pec_teil(&[20, 5], &st);
        let mut d = b"#PES0001".to_vec();
        d.extend(22u32.to_le_bytes());
        d.resize(22, 0);
        d.extend(&pec);
        let s = lesen(&d, "pes").unwrap();
        assert_eq!(s.name.as_deref(), Some("Rose"));
        assert_eq!(s.bloecke.len(), 2);
        assert_eq!(s.bloecke[0].name.as_deref(), Some("Black"));
        assert_eq!(s.bloecke[1].farbe, [236, 0, 0]);
        // absolute Punkte (0,1 mm, +y oben): (−100,−50), (−70,−30), (130,−30), (130,−40); Mitte (15, −40)
        let alle: Vec<(f64, f64)> = s.bloecke.iter().flat_map(|b| &b.zuege).flatten().copied().collect();
        assert_eq!(alle, vec![(-11.5, -1.0), (-8.5, 1.0), (11.5, 1.0), (11.5, 0.0)]);
        // als PEC-Datei
        let mut p = b"#PEC0001".to_vec();
        p.extend(&pec);
        assert_eq!(lesen(&p, "PEC").unwrap().bloecke, s.bloecke);
    }

    #[test]
    fn jef_lesen() {
        let mut d = vec![0u8; 116 + 16];
        let start = d.len() as u32;
        d[0..4].copy_from_slice(&start.to_le_bytes());
        d[24..28].copy_from_slice(&2u32.to_le_bytes());
        d[116..120].copy_from_slice(&2u32.to_le_bytes()); // Weiß
        d[120..124].copy_from_slice(&10u32.to_le_bytes()); // Rot
        for b in [0, 0, 20, 0, 20, 236u8, 0x80, 0x02, 30, 0, 5, 5, 0x80, 0x01, 0, 0, 0, 10, 0x80, 0x10] {
            d.push(b);
        }
        let s = lesen(&d, "jef").unwrap();
        assert_eq!(s.bloecke.len(), 2);
        assert_eq!(s.bloecke[0].name.as_deref(), Some("White"));
        assert_eq!(s.bloecke[1].farbe, [255, 0, 0]);
        let z: Vec<&Vec<(f64, f64)>> = s.bloecke.iter().flat_map(|b| &b.zuege).collect();
        assert_eq!(z.len(), 3);
        // (0,0) (2,0) (4,−2) | Sprung +3 → (7,−2)+(0,5;0,5) = (7,5;−1,5) | Farbe → (7,5;−0,5)
        assert_eq!(z[0].len(), 3);
        assert_eq!(z[1].len(), 1);
        assert_eq!(z[2].len(), 1);
    }

    #[test]
    fn vp3_aufbau() {
        // Datei nach dem beschriebenen Aufbau (Selbstprüfung; eine echte Datei fehlt noch)
        let text = |t: &str, d: &mut Vec<u8>| {
            d.extend((t.len() as u16).to_be_bytes());
            d.extend(t.as_bytes());
        };
        let mut d = b"%vsm%\0".to_vec();
        text("Produced by Test", &mut d);
        d.extend([0, 2, 0, 0, 0, 0, 0]);
        text("", &mut d);
        d.extend([0u8; 32]);
        d.extend(1000i32.to_be_bytes());
        d.extend(2000i32.to_be_bytes());
        d.extend([0u8; 27]);
        text("", &mut d);
        d.extend([0u8; 24]);
        text("", &mut d);
        d.extend(1u16.to_be_bytes());
        d.extend([0, 5, 0]);
        let mut block = Vec::new();
        block.extend(0i32.to_be_bytes());
        block.extend(0i32.to_be_bytes());
        block.extend([1, 0, 200, 10, 20, 0, 0, 5, 40]);
        text("1805", &mut block);
        text("Red", &mut block);
        text("Isacord", &mut block);
        block.extend([0u8; 15]);
        block.extend([10, 0, 0, 10, 0x80, 0x01, 0x01, 0x00, 0x00, 0x00, 0x80, 0x02]);
        d.extend((block.len() as u32).to_be_bytes());
        d.extend(&block);
        let s = lesen(&d, "vp3").unwrap();
        assert_eq!(s.bloecke.len(), 1);
        assert_eq!(s.bloecke[0].farbe, [200, 10, 20]);
        assert_eq!(s.bloecke[0].name.as_deref(), Some("Isacord 1805 Red"));
        assert_eq!(s.stiche(), 3);
    }

    #[test]
    fn exp_ueber_kern() {
        let mut f = stick_kern::stiche::StichFolge::default();
        f.stich(stick_kern::geometrie::p(0.0, 0.0));
        f.stich(stick_kern::geometrie::p(3.0, 0.0));
        f.farbwechsel("#f00");
        f.stich(stick_kern::geometrie::p(3.0, 4.0));
        let s = lesen(&crate::exp::exp_bytes(&f), "EXP").unwrap();
        assert_eq!(s.bloecke.len(), 2);
        assert_eq!(s.stiche(), 3);
        assert!(lesen(b"xx", "hus").is_err());
    }
}
