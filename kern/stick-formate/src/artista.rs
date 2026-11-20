//! Designdaten der artista 185: Puffer DATEN im Maschinenspeicher und der „BE“-Container (Anhang).
//!
//! Quellen: `Analyse_Teil2/03` (Schreiben BEAGLink 0x100032b0, Lesen 0x10004310), `Analyse_Teil2/05` (Container und
//! Verschleierung 0x1000a370 / 0x1000a4c0 / 0x1000a640). **Am Gerät bestätigt** mit allen 51 Designs aus dem
//! Maschinenspeicher (`Analyse_Teil2/07` Test v3.6, `Arbeitsdaten/maschinendesigns/`): Lesen und neu Zusammensetzen
//! ergibt dieselben Bytes, die Flags der Designliste folgen aus dem Kopf (Test `rundweg_maschinendesigns`).
//!
//! Puffer DATEN (N = Verwaltungsblock Byte 0, artista 185: 28):
//! ```text
//! 0       N × u16 BE   Tabelle A (aus der Maschine gelesen: alle 0)
//! 2N      N × u16 BE   Tabelle B
//! 4N      N × u16 BE   Tabelle C
//! 6N      u32 BE       Länge 1 = Stichdaten einschließlich Endkennung 80 81
//! 6N+4    u32 BE       Länge 2 = Anhang (BE-Container, verschleiert); 0 = kein Anhang
//! 6N+8    Stichdaten (EXP-Kodierung), danach der Anhang
//! ```
//! Achtung beim Lesen: Die Maschine setzt Länge 2 bei Befehl 0x400 nicht zurück – ein Anhang gilt nur, wenn die
//! Containerlänge (u32 LE bei +2) gleich Länge 2 ist ([`Puffer::anhang_gueltig`]).

use stick_kern::{Art, StichFolge};

/// Anzahl der Tabelleneinträge N bei der artista 185 (Verwaltungsblock 0xFFFD24, Byte 0; am Gerät gelesen).
pub const N_185: usize = 28;
/// Endkennung der Stichdaten.
pub const ENDKENNUNG: [u8; 2] = [0x80, 0x81];
/// Chunk-Kennungen im Container (`Analyse_Teil2/05`).
pub const CHUNK_KOPF: u16 = 0x0A;
pub const CHUNK_FARBEN: u16 = 0x14;
pub const CHUNK_STICHE: u16 = 0x1E;
pub const CHUNK_ZUSATZ: u16 = 0x28;
pub const CHUNK_ICON: u16 = 0x32;
/// Länge des Kopf-Chunks einschließlich 6 Byte Chunkkopf.
pub const KOPF_LAENGE: usize = 0x134;
/// Länge eines Eintrags der Farbtabelle (Chunk 0x14).
pub const FARB_EINTRAG: usize = 0x86;

fn u32_le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn u16_le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

// ---------------------------------------------------------------------------------------------------------------
// Verschleierung
// ---------------------------------------------------------------------------------------------------------------

/// Startzustand der Verschleierung (BEAGLink 0x1000a370) für n verschleierte Bytes.
fn be_startzustand(n: usize) -> ([u8; 8], usize) {
    let a = (n % 255) as u8;
    let b = ((n >> 8) % 255) as u8;
    let mut s = [0u8; 8];
    s[0] = b'B';
    s[1] = b'E';
    if n <= 0xfff {
        s[4] = a ^ 0x76;
        s[2] = (a ^ 0x76) ^ 0xb3;
        s[3] = 0xb3;
        s[5] = (a ^ 0x0d) << 2;
        s[6] = (a << 4) ^ 0x73;
        s[7] = (a << 5) ^ 0xd5;
    } else if n <= 0x3fff {
        s[4] = b ^ 0xea;
        s[7] = (a << 1) ^ 0xc9;
        s[6] = (a << 6) ^ 0x30;
        s[5] = a ^ 0x56;
        s[3] = 0x24;
        s[2] = (b ^ 0xea) ^ (a ^ 0x56) ^ 0x24;
    } else if n <= 0xcfff {
        s[7] = (a << 1) ^ 0xc9;
        s[4] = b ^ 0x4c;
        s[6] = a ^ 0x27;
        s[5] = a ^ 0xf3;
        s[3] = 0xd1;
        s[2] = (b ^ 0x4c) ^ (a ^ 0xf3) ^ (a ^ 0x27) ^ 0xd1;
    } else {
        s[5] = a ^ 0x11;
        s[6] = a ^ 0x89;
        s[7] = a ^ 0x52;
        s[4] = b ^ 0x6a;
        s[3] = 0x38;
        s[2] = (b ^ 0x6a) ^ (a ^ 0x11) ^ (a ^ 0x89) ^ (a ^ 0x52) ^ 0x38;
    }
    for (i, v) in s.iter_mut().enumerate() {
        *v ^= (i as u8) << 3;
    }
    (s, (n >> 1) & 7)
}

/// Ganzen Container ab Byte 8 umkodieren (BEAGLink 0x1000a640): `verschluesseln = true` beim Senden (Modus 1),
/// `false` beim Lesen (Modus 0). Die Bytes 0..8 bleiben unverändert; die Länge steht als u32 LE bei Byte 2.
pub fn be_umkodieren(c: &mut [u8], verschluesseln: bool) {
    if c.len() < 8 {
        return;
    }
    let l = (u32_le(c, 2) as usize).min(c.len());
    let (mut s, mut i) = be_startzustand(l.saturating_sub(8));
    for byte in c.iter_mut().take(l).skip(8) {
        let ein = *byte;
        let aus = ein ^ s[(i + 2) & 7] ^ s[(i + 7) & 7] ^ s[(i + 1) & 7];
        s[i] = if verschluesseln { aus } else { ein }; // Zustand nimmt immer den Chiffretext auf
        *byte = aus;
        i = (i + 5) & 7; // i − 3
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------------------------------------------

/// Ein Chunk des Containers; `daten` ohne den 6-Byte-Chunkkopf (u16 Kennung, u32 Länge einschließlich Kopf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: u16,
    pub daten: Vec<u8>,
}

/// BE-Container: 8 Byte Kopf (u16 Kennung, u32 Gesamtlänge LE, u16), dann Chunks (`Analyse_Teil2/05`).
/// Aus der Maschine gelesen: Kennung und Wort 0, Chunks 0x0A (Kopf) und 0x14 (Farbtabelle).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Container {
    pub kennung: u16,
    pub wort: u16,
    pub chunks: Vec<Chunk>,
}

impl Container {
    /// Entschleierten Container zerlegen.
    pub fn aus_klar(b: &[u8]) -> Result<Container, String> {
        if b.len() < 8 {
            return Err(format!("Container zu kurz ({} Byte)", b.len()));
        }
        let l = u32_le(b, 2) as usize;
        if l != b.len() {
            return Err(format!("Containerlänge {l} ≠ {} Byte", b.len()));
        }
        let mut chunks = Vec::new();
        let mut o = 8;
        while o + 6 <= l {
            let id = u16_le(b, o);
            let n = u32_le(b, o + 2) as usize;
            if n < 6 || o + n > l {
                return Err(format!("Chunk 0x{id:02X} bei {o}: Länge {n} passt nicht"));
            }
            chunks.push(Chunk {
                id,
                daten: b[o + 6..o + n].to_vec(),
            });
            o += n;
        }
        if o != l {
            return Err(format!("{} Byte Rest nach den Chunks", l - o));
        }
        Ok(Container {
            kennung: u16_le(b, 0),
            wort: u16_le(b, 6),
            chunks,
        })
    }

    /// Verschleierten Container (wie im Puffer DATEN) entschleiern und zerlegen.
    pub fn aus_verschleiert(b: &[u8]) -> Result<Container, String> {
        let mut v = b.to_vec();
        if v.len() >= 8 && u32_le(&v, 2) as usize != v.len() {
            return Err(format!("Containerlänge {} ≠ {} Byte", u32_le(&v, 2), v.len()));
        }
        be_umkodieren(&mut v, false);
        Container::aus_klar(&v)
    }

    /// Container im Klartext.
    pub fn klar(&self) -> Vec<u8> {
        let l = 8 + self.chunks.iter().map(|c| c.daten.len() + 6).sum::<usize>();
        let mut v = Vec::with_capacity(l);
        v.extend(self.kennung.to_le_bytes());
        v.extend((l as u32).to_le_bytes());
        v.extend(self.wort.to_le_bytes());
        for c in &self.chunks {
            v.extend(c.id.to_le_bytes());
            v.extend(((c.daten.len() + 6) as u32).to_le_bytes());
            v.extend(&c.daten);
        }
        v
    }

    /// Container verschleiert, so wie er an die Maschine geht (Modus 1).
    pub fn verschleiert(&self) -> Vec<u8> {
        let mut v = self.klar();
        be_umkodieren(&mut v, true);
        v
    }

    pub fn chunk(&self, id: u16) -> Option<&[u8]> {
        self.chunks.iter().find(|c| c.id == id).map(|c| c.daten.as_slice())
    }

    /// Kopf-Chunk 0x0A, falls vorhanden.
    pub fn kopf(&self) -> Option<Kopf> {
        self.chunk(CHUNK_KOPF).and_then(|d| Kopf::aus_chunk(d).ok())
    }

    /// Farbtabelle (Chunk 0x14): n × 0x86 Byte.
    pub fn farben(&self) -> Vec<Farbe> {
        self.chunk(CHUNK_FARBEN)
            .map(|d| {
                d.as_chunks::<FARB_EINTRAG>()
                    .0
                    .iter()
                    .map(|e| Farbe::lesen(e))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Container für den Anhang so, wie das Original ihn bei Firmware ≥ 3.09 (artista 185: 4.04) in den Puffer DATEN
    /// legt (BEAGLink 0x100035b6–0x1000374d): **ohne** Stichdaten (0x1E), Zusatzdaten (0x28), Icon (0x32) und
    /// Farbtabelle (0x14). Die Farben gehen getrennt an die Garnfarbtabelle ([`farbtabelle_maschine`], Schritt 12),
    /// das Icon an den Puffer ICON ([`Icon::bytes`], Schritt 8). Sicherheit: hoch (Code).
    pub fn anhang_wie_original(&self) -> Container {
        Container {
            kennung: self.kennung,
            wort: self.wort,
            chunks: self
                .chunks
                .iter()
                .filter(|c| ![CHUNK_FARBEN, CHUNK_STICHE, CHUNK_ZUSATZ, CHUNK_ICON].contains(&c.id))
                .cloned()
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Kopf-Chunk 0x0A
// ---------------------------------------------------------------------------------------------------------------

/// Kopf-Chunk 0x0A, 0x134 Byte **einschließlich** 6-Byte-Chunkkopf – die Offsets sind dieselben wie im Code
/// (BEAGLink 0x100032b0) und in `Analyse_Teil2/03`/`07`.
///
/// Felder (Offsets inkl. Chunkkopf; Bedeutung aus Code und den 38 Designs mit Anhang, `Analyse_Teil2/07`):
/// - 0x14: **Name**, 32 Byte (Leerzeichen am Ende werden beim Schreiben ins INFO-Feld zu 0) – hoch
/// - 0x94: Sammlung/Karte („Outdoorsman card“, „Collection 7“) – mittel
/// - 0x114: Farbanzahl (Karten-Designs, Zeichensätze 4/6/7) bzw. Farbanzahl − 1 („Collection 7“, Drifter) – mittel
/// - 0x116 … 0x126: **Funktionsfelder** → Flags der Designliste ([`Kopf::flags`]) – hoch (38 von 38)
/// - 0x128/0x12A: 100/100 oder 0/0 (vermutlich Maßstab in %) – niedrig
/// - 0x12C/0x12E: **Breite/Höhe** in 0,1 mm (Karten-Designs: genau die Ausdehnung der Stiche) bzw. 2000/1550 =
///   großer Rahmen 155 × 200 mm („Collection 7“; Stickmodul-Anleitung) – mittel
/// - 0x130: u16 ≈ 0,40 × Stichzahl (vermutlich Stickzeit in s); beim Schreiben Tabelle A Eintrag 0 – niedrig
/// - 0x132: u16 (Bytes 01 00 / 01 1f / 01 e0) – offen
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kopf(pub [u8; KOPF_LAENGE]);

impl Kopf {
    /// Aus den Chunk-Daten (ohne Chunkkopf, 0x12E Byte).
    pub fn aus_chunk(daten: &[u8]) -> Result<Kopf, String> {
        if daten.len() + 6 != KOPF_LAENGE {
            return Err(format!("Kopf-Chunk hat {} statt {} Byte", daten.len() + 6, KOPF_LAENGE));
        }
        let mut k = [0u8; KOPF_LAENGE];
        k[..2].copy_from_slice(&CHUNK_KOPF.to_le_bytes());
        k[2..6].copy_from_slice(&(KOPF_LAENGE as u32).to_le_bytes());
        k[6..].copy_from_slice(daten);
        Ok(Kopf(k))
    }

    /// Chunk-Daten ohne Chunkkopf.
    pub fn chunk_daten(&self) -> Vec<u8> {
        self.0[6..].to_vec()
    }

    pub fn feld(&self, o: usize) -> u16 {
        u16_le(&self.0, o)
    }

    pub fn setze(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }

    /// Name ab 0x14 (32 Byte, bis zur ersten 0, Leerzeichen am Ende entfernt).
    pub fn name(&self) -> String {
        let roh = &self.0[0x14..0x34];
        let ende = roh.iter().position(|&b| b == 0).unwrap_or(roh.len());
        roh[..ende]
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    /// Name für das INFO-Feld (Schritt 11): 32 Byte ab 0x14, Leerzeichen am Ende → 0.
    pub fn name_info(&self) -> [u8; 32] {
        let mut n = [0u8; 32];
        n.copy_from_slice(&self.0[0x14..0x34]);
        for b in n.iter_mut().rev() {
            match *b {
                b' ' | 0 => *b = 0,
                _ => break,
            }
        }
        n
    }

    /// Funktions-Flags für die Designliste (INFO + 2N+1), wie BEAGLink 0x100032b0 Schritt 10 sie bildet: je ein Bit,
    /// wenn das Kopffeld ≠ 0. Maschinenspeicher: Feld 0x126 → Bit 0x80 (Karte: 0x40). Geprüft an allen 38 Designs
    /// mit Anhang aus dem Maschinenspeicher (0x84, 0x8C, 0xA4, 0xAC).
    pub fn flags(&self) -> u8 {
        let mut f = 0u8;
        if self.feld(0x11C) != 0 || self.feld(0x11A) != 0 {
            f |= 0x01;
        }
        for (bit, o) in [
            (0x02, 0x116),
            (0x04, 0x118),
            (0x08, 0x11E),
            (0x10, 0x120),
            (0x20, 0x124),
            (0x80, 0x126),
        ] {
            if self.feld(o) != 0 {
                f |= bit;
            }
        }
        f
    }

    /// Neuer Kopf für ein eigenes Design. **Eigene Festlegung** nach dem Muster der Designs von der
    /// „Outdoorsman card“ (Nr. 8, 39–41 im Maschinenspeicher): Farbanzahl in 0x114, Felder 0x118/0x122/0x126 = 1
    /// (Flags 0x84), Breite/Höhe in 0,1 mm, 0x130 = 0,40 × Stichzahl, 0x132 = 1. Wie die Original-Software den Kopf
    /// füllt, ist noch offen (Erzeuger des Containers nicht gefunden).
    pub fn neu(name: &str, farben: u16, breite: u16, hoehe: u16, stiche: usize) -> Kopf {
        let mut k = [0u8; KOPF_LAENGE];
        k[..2].copy_from_slice(&CHUNK_KOPF.to_le_bytes());
        k[2..6].copy_from_slice(&(KOPF_LAENGE as u32).to_le_bytes());
        let mut kopf = Kopf(k);
        for (i, b) in name.bytes().filter(|b| (0x20..0x7f).contains(b)).take(31).enumerate() {
            kopf.0[0x14 + i] = b;
        }
        kopf.setze(0x114, farben);
        kopf.setze(0x118, 1);
        kopf.setze(0x122, 1);
        kopf.setze(0x126, 1);
        kopf.setze(0x12C, breite);
        kopf.setze(0x12E, hoehe);
        kopf.setze(
            0x130,
            ((stiche as f64 * 0.4).round() as u64).min(u16::MAX as u64) as u16,
        );
        kopf.setze(0x132, 1);
        kopf
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Farbtabelle (Chunk 0x14)
// ---------------------------------------------------------------------------------------------------------------

/// Eintrag der Farbtabelle, 0x86 Byte: R, G, B, 0, u16 LE Garnnummer, Marke (64 Byte), Farbname (64 Byte).
/// Beispiel aus dem Maschinenspeicher (Nr. 8): `ff ff fa 00 0a 00` „Isacord 40“ „white“. Sicherheit: hoch (Aufbau),
/// mittel (Nummer).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Farbe {
    pub rgb: [u8; 3],
    pub nummer: u16,
    pub marke: String,
    pub name: String,
}

fn text_lesen(b: &[u8]) -> String {
    let ende = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    b[..ende].iter().map(|&c| c as char).collect()
}

fn text_schreiben(ziel: &mut [u8], t: &str) {
    let platz = ziel.len() - 1; // letztes Byte bleibt 0
    for (z, c) in ziel.iter_mut().take(platz).zip(t.chars()) {
        *z = if (c as u32) < 0x100 { c as u8 } else { b'?' };
    }
}

impl Farbe {
    pub fn lesen(e: &[u8]) -> Farbe {
        Farbe {
            rgb: [e[0], e[1], e[2]],
            nummer: u16_le(e, 4),
            marke: text_lesen(&e[6..0x46]),
            name: text_lesen(&e[0x46..0x86]),
        }
    }

    pub fn bytes(&self) -> [u8; FARB_EINTRAG] {
        let mut e = [0u8; FARB_EINTRAG];
        e[..3].copy_from_slice(&self.rgb);
        e[4..6].copy_from_slice(&self.nummer.to_le_bytes());
        text_schreiben(&mut e[6..0x46], &self.marke);
        text_schreiben(&mut e[0x46..0x86], &self.name);
        e
    }

    /// Farbe aus `#rrggbb` (ohne Garnangabe).
    pub fn aus_hex(hex: &str) -> Farbe {
        let h = hex.trim_start_matches('#');
        let k = |i: usize| {
            h.get(i..i + 2)
                .and_then(|s| u8::from_str_radix(s, 16).ok())
                .unwrap_or(0)
        };
        Farbe {
            rgb: [k(0), k(2), k(4)],
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Garnfarbtabelle der Maschine (Schritt 12, Firmware ≥ 3.09)
// ---------------------------------------------------------------------------------------------------------------

/// Zeiger (u32 BE) auf den 18-Byte-Zusatzblock (Verbindungsaufbau BEAGLink 0x10005cfe; artista 185: 0x20772).
pub const ZEIGER_ZUSATZBLOCK: u32 = 0x24044;
/// Zeiger (u32 BE) auf den Bereich T der Garnfarbtabelle (BEAGLink 0x10003a74; artista 185: 0x2083A).
pub const ZEIGER_FARBTABELLE: u32 = 0x24048;
/// Länge des Zusatzblocks.
pub const ZUSATZBLOCK_LAENGE: usize = 0x12;
/// Nach der Tabelle schreibt das Original diese 3 Byte an den Anfang von T (0x10003d03–0x10003d2a); beim Lesen muss
/// Byte 0 von T gleich 0 sein (0x100048d6).
pub const FARBTABELLE_FREIGABE: [u8; 3] = [0, 0, 0];

/// Grenzen der Garnfarbtabelle aus dem Zusatzblock (BEAGLink 0x10003a99 ff. beim Schreiben, 0x10004886 ff. beim
/// Lesen). Die Werte der artista 185 sind noch nicht gelesen (Zeiger 0x24044 → 0x20772 bekannt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FarbGrenzen {
    /// Byte +0x0C: höchste Farbanzahl
    pub max_farben: u8,
    /// u16 BE +0x10: Größe des Bereichs T (Vorspann + Tabelle müssen kleiner sein; nur gültig, wenn < 0xFD00)
    pub groesse: u16,
}

impl FarbGrenzen {
    pub fn aus_zusatzblock(z: &[u8]) -> Result<FarbGrenzen, String> {
        if z.len() < ZUSATZBLOCK_LAENGE {
            return Err(format!("Zusatzblock hat {} statt {ZUSATZBLOCK_LAENGE} Byte", z.len()));
        }
        Ok(FarbGrenzen {
            max_farben: z[0x0C],
            groesse: u16::from_be_bytes([z[0x10], z[0x11]]),
        })
    }

    /// Vorspann in T vor der Tabelle: höchste Farbanzahl + 6 Byte. Die Tabelle steht bei T + Vorspann.
    pub fn vorspann(&self) -> usize {
        self.max_farben as usize + 6
    }

    /// Prüfung wie das Original vor dem Schreiben (0x10003abb, 0x10003c8c–0x10003cc2); sonst schreibt es keine
    /// Tabelle (ohne Fehlermeldung).
    pub fn pruefen(&self, farben: usize, tabelle: &[u8]) -> Result<(), String> {
        if self.groesse >= 0xFD00 {
            return Err(format!("Bereich ungültig (Größe 0x{:04X} ≥ 0xFD00)", self.groesse));
        }
        if farben > self.max_farben as usize {
            return Err(format!("{farben} Farben, die Maschine erlaubt {}", self.max_farben));
        }
        if self.vorspann() + tabelle.len() >= self.groesse as usize {
            return Err(format!(
                "Tabelle zu groß: Vorspann {} + {} Byte ≥ {}",
                self.vorspann(),
                tabelle.len(),
                self.groesse
            ));
        }
        Ok(())
    }
}

/// Text eines 64-Byte-Feldes so, wie das Original ihn in die Tabelle übernimmt (0x10003b4b–0x10003b7b): Leerzeichen
/// am Ende (Stellen 0x3E … 1) abschneiden, dann bis zur ersten 0.
fn text_fuer_tabelle(feld: &[u8]) -> Vec<u8> {
    let mut f = [0u8; 0x41];
    f[..feld.len().min(0x40)].copy_from_slice(&feld[..feld.len().min(0x40)]);
    let mut i = 0x3E;
    while i > 0 {
        if f[i] != b' ' {
            f[i + 1] = 0;
            break;
        }
        i -= 1;
    }
    let ende = f.iter().position(|&b| b == 0).unwrap_or(0x40);
    f[..ende].to_vec()
}

/// Garnfarbtabelle, wie BEAGLink 0x100032b0 (Schritt 12, 0x10003a5d–0x10003d2f) sie an **T + Vorspann** schreibt.
/// Sicherheit: hoch (Maschinencode Byte für Byte; Lesen 0x10004850 prüft dieselben Felder), am Gerät noch nicht
/// geschrieben.
/// ```text
/// 0   00 00 00 01            u32 BE 1
/// 4   00 00 00 08            u32 BE 8
/// 8   u32 BE                 Länge ab Byte 8 (Gesamtlänge − 8)
/// 12  00 00 + u16 BE         Anzahl Farben (das Lesen nimmt u32 BE)
/// 16  je Farbe: u16 BE Datensatzlänge (mit diesen 2 Byte) | u16 BE Nummer ab 1 | R G B | u16 BE Garnnummer |
///     Marke, 0 | Farbname, 0
/// ```
pub fn farbtabelle_maschine(farben: &[Farbe]) -> Vec<u8> {
    let mut v = vec![0, 0, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, f) in farben.iter().enumerate() {
        let e = f.bytes();
        let mut d = Vec::new();
        d.extend(((i + 1) as u16).to_be_bytes());
        d.extend(&e[..3]);
        d.extend(f.nummer.to_be_bytes());
        d.extend(text_fuer_tabelle(&e[6..0x46]));
        d.push(0);
        d.extend(text_fuer_tabelle(&e[0x46..0x86]));
        d.push(0);
        v.extend(((d.len() + 2) as u16).to_be_bytes());
        v.extend(d);
    }
    let rest = (v.len() - 8) as u32;
    v[8..12].copy_from_slice(&rest.to_be_bytes());
    v[14..16].copy_from_slice(&(farben.len() as u16).to_be_bytes());
    v
}

/// Garnfarbtabelle aus dem gelesenen Bereich ab T lesen und prüfen wie BEAGLink 0x10004850–0x10004a97. `bereich`
/// muss mindestens Vorspann + 16 Byte lang sein; Datensätze werden gelesen, soweit vorhanden.
pub fn farbtabelle_lesen(bereich: &[u8], g: FarbGrenzen) -> Result<Vec<Farbe>, String> {
    let k = g.vorspann();
    if bereich.len() < k + 16 {
        return Err(format!("Bereich zu kurz ({} Byte, nötig {})", bereich.len(), k + 16));
    }
    let u32_be = |o: usize| u32::from_be_bytes([bereich[o], bereich[o + 1], bereich[o + 2], bereich[o + 3]]);
    if bereich[0] != 0 {
        return Err(format!("Byte 0 von T ist 0x{:02X} (keine gültige Tabelle)", bereich[0]));
    }
    if u32_be(k) != 1 || u32_be(k + 4) != 8 {
        return Err(format!(
            "Kennung {} / {} statt 1 / 8 (keine Tabelle)",
            u32_be(k),
            u32_be(k + 4)
        ));
    }
    let laenge = u32_be(k + 8) as usize;
    let anzahl = u32_be(k + 12) as usize;
    if laenge >= g.groesse as usize || g.groesse >= 0xFD00 || anzahl >= 0xFF || anzahl * 0x86 >= 0xFF00 {
        return Err(format!("Länge {laenge} / Anzahl {anzahl} außerhalb der Grenzen"));
    }
    let ende = (k + 8 + laenge).min(bereich.len());
    let mut farben = Vec::new();
    let mut o = k + 16;
    while farben.len() < anzahl && o + 9 <= ende {
        let dl = u16::from_be_bytes([bereich[o], bereich[o + 1]]) as usize;
        if dl < 11 || o + dl > ende {
            break;
        }
        let d = &bereich[o..o + dl];
        let text = |von: usize| -> (String, usize) {
            let n = d[von..].iter().position(|&b| b == 0).unwrap_or(d.len() - von);
            (d[von..von + n].iter().map(|&b| b as char).collect(), von + n + 1)
        };
        let (marke, p) = text(9);
        let (name, p) = text(p.min(dl));
        if p != dl {
            return Err(format!(
                "Datensatz {}: Länge {dl} passt nicht zu den Texten",
                farben.len() + 1
            ));
        }
        farben.push(Farbe {
            rgb: [d[4], d[5], d[6]],
            nummer: u16::from_be_bytes([d[7], d[8]]),
            marke,
            name,
        });
        o += dl;
    }
    Ok(farben)
}

// ---------------------------------------------------------------------------------------------------------------
// Puffer DATEN
// ---------------------------------------------------------------------------------------------------------------

/// Inhalt des Puffers DATEN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puffer {
    /// Tabellen A, B, C (je N Einträge)
    pub tabellen: [Vec<u16>; 3],
    /// Stichdaten in EXP-Kodierung einschließlich Endkennung `80 81`
    pub stiche: Vec<u8>,
    /// Anhang (verschleierter Container) – beim Lesen so, wie Länge 2 ihn angibt, auch wenn er ungültig ist
    pub anhang: Vec<u8>,
}

/// Felder der Designliste, die beim Schreiben neben DATEN gesetzt werden (BEAGLink 0x100032b0 Schritte 9–11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoFelder {
    /// INFO + (N+1): Farbanzahl − 1
    pub farben_minus_1: u8,
    /// INFO + (2N+1): Funktions-Flags
    pub flags: u8,
    /// INFO + (3N+1): Name, 32 Byte
    pub name: [u8; 32],
}

impl Puffer {
    /// Puffer aus den gelesenen Bytes zerlegen (N = 28 bei der artista 185).
    pub fn lesen(roh: &[u8], n: usize) -> Result<Puffer, String> {
        let k = 6 * n + 8;
        if roh.len() < k {
            return Err(format!("Puffer zu kurz ({} Byte, Kopf {k})", roh.len()));
        }
        let tab = |t: usize| -> Vec<u16> {
            (0..n)
                .map(|i| u16::from_be_bytes([roh[2 * (t * n + i)], roh[2 * (t * n + i) + 1]]))
                .collect()
        };
        let len1 = u32::from_be_bytes([roh[6 * n], roh[6 * n + 1], roh[6 * n + 2], roh[6 * n + 3]]) as usize;
        let len2 = u32::from_be_bytes([roh[6 * n + 4], roh[6 * n + 5], roh[6 * n + 6], roh[6 * n + 7]]) as usize;
        if k + len1 + len2 != roh.len() {
            return Err(format!(
                "Längen passen nicht: Kopf {k} + Stiche {len1} + Anhang {len2} ≠ {} Byte",
                roh.len()
            ));
        }
        Ok(Puffer {
            tabellen: [tab(0), tab(1), tab(2)],
            stiche: roh[k..k + len1].to_vec(),
            anhang: roh[k + len1..].to_vec(),
        })
    }

    /// Bytes für den Puffer DATEN (Schritt 6 des Schreibens).
    pub fn bytes(&self) -> Vec<u8> {
        let n = self.tabellen[0].len();
        let mut v = Vec::with_capacity(6 * n + 8 + self.stiche.len() + self.anhang.len());
        for t in &self.tabellen {
            for w in t {
                v.extend(w.to_be_bytes());
            }
        }
        v.extend((self.stiche.len() as u32).to_be_bytes());
        v.extend((self.anhang.len() as u32).to_be_bytes());
        v.extend(&self.stiche);
        v.extend(&self.anhang);
        v
    }

    /// Anhang echt? Containerlänge (u32 LE bei +2) = Länge 2. Sonst ist es der Rest eines vorher geladenen Designs.
    pub fn anhang_gueltig(&self) -> bool {
        self.anhang.len() >= 8 && u32_le(&self.anhang, 2) as usize == self.anhang.len()
    }

    /// Container des Anhangs, wenn der Anhang gültig ist.
    pub fn container(&self) -> Option<Result<Container, String>> {
        self.anhang_gueltig().then(|| Container::aus_verschleiert(&self.anhang))
    }

    /// Endet mit `80 81`?
    pub fn endkennung(&self) -> bool {
        self.stiche.ends_with(&ENDKENNUNG)
    }

    /// Neuer Puffer aus EXP-Stichdaten (ohne oder mit Endkennung) und optionalem Container. Tabellen A–C wie beim
    /// Lesen alle 0 (Code: Tabelle A Eintrag 0 = Kopf 0x130 – **offen**, ob die Maschine ihn braucht).
    pub fn neu(exp: &[u8], container: Option<&Container>) -> Puffer {
        let mut stiche = exp.to_vec();
        if !stiche.ends_with(&ENDKENNUNG) {
            stiche.extend(ENDKENNUNG);
        }
        Puffer {
            tabellen: [vec![0; N_185], vec![0; N_185], vec![0; N_185]],
            stiche,
            anhang: container.map(Container::verschleiert).unwrap_or_default(),
        }
    }

    /// INFO-Felder für das Schreiben (Schritte 9–11). Ohne Kopf: Flags 0x86 und Name leer wie die Designs ohne
    /// Anhang im Maschinenspeicher (Nr. 42–51) – **eigene Festlegung**.
    pub fn info_felder(&self, farben: usize) -> InfoFelder {
        let kopf = self.container().and_then(|c| c.ok()).and_then(|c| c.kopf());
        InfoFelder {
            farben_minus_1: farben.saturating_sub(1).min(255) as u8,
            flags: kopf.as_ref().map_or(0x86, Kopf::flags),
            name: kopf.as_ref().map_or([0; 32], Kopf::name_info),
        }
    }
}

/// Anzahl der Farben in EXP-Stichdaten (Farbwechsel `80 01` + 1), Ende bei `80 81`.
pub fn farben_in_stichen(d: &[u8]) -> usize {
    let mut i = 0;
    let mut n = 1;
    while i + 1 < d.len() {
        if d[i] == 0x80 {
            match d[i + 1] {
                0x81 => break,
                0x01 => {
                    n += 1;
                    i += 4;
                }
                0x82 => i += 6,
                0x02..=0x0F => i += 4,
                _ => i += 2,
            }
        } else {
            i += 2;
        }
    }
    n
}

// ---------------------------------------------------------------------------------------------------------------
// Icon (Puffer ICON, Schritt 8)
// ---------------------------------------------------------------------------------------------------------------

/// Länge des Icon-Kopfs im Puffer ICON (BEAGLink 0x1000382c: Bitmap ab 0xAE; am Gerät gelesen).
pub const ICON_KOPF: usize = 0xAE;
/// Größe der Icons der artista 185: alle 51 gelesenen Icons haben 9 Byte je Zeile und 62 Zeilen (`Analyse_Teil2/07`).
pub const ICON_BREITE: usize = 72;
pub const ICON_HOEHE: usize = 62;

/// Icon: 1 Bit je Punkt, Zeilen von oben nach unten, höchstes Bit links, **Bit gesetzt = Stich (schwarz)** – so wie
/// im Puffer ICON der Maschine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Icon {
    pub bytes_je_zeile: usize,
    pub hoehe: usize,
    pub bits: Vec<u8>,
}

impl Icon {
    pub fn leer(breite: usize, hoehe: usize) -> Icon {
        let bytes_je_zeile = breite.div_ceil(8);
        Icon {
            bytes_je_zeile,
            hoehe,
            bits: vec![0; bytes_je_zeile * hoehe],
        }
    }

    pub fn punkt(&self, x: usize, y: usize) -> bool {
        x < self.bytes_je_zeile * 8
            && y < self.hoehe
            && self.bits[y * self.bytes_je_zeile + x / 8] & (0x80 >> (x % 8)) != 0
    }

    pub fn setze(&mut self, x: usize, y: usize) {
        if x < self.bytes_je_zeile * 8 && y < self.hoehe {
            self.bits[y * self.bytes_je_zeile + x / 8] |= 0x80 >> (x % 8);
        }
    }

    /// Bytes für den Puffer ICON wie BEAGLink Schritt 8 (0x1000382c–0x100038b8): Kopf 0xAE Byte mit [2] = Bytes je
    /// Zeile, [3] = Höhe, [4] = 0xFF, sonst 0; dahinter die Bitmap.
    pub fn bytes(&self) -> Vec<u8> {
        let mut v = vec![0u8; ICON_KOPF];
        v[2] = self.bytes_je_zeile as u8;
        v[3] = self.hoehe as u8;
        v[4] = 0xFF;
        v.extend(&self.bits);
        v
    }

    /// Aus dem gelesenen Puffer ICON (fehlende Zeilen bleiben leer).
    pub fn lesen(b: &[u8]) -> Result<Icon, String> {
        if b.len() < ICON_KOPF {
            return Err(format!("Icon zu kurz ({} Byte, Kopf {ICON_KOPF})", b.len()));
        }
        let mut ic = Icon {
            bytes_je_zeile: b[2] as usize,
            hoehe: b[3] as usize,
            bits: vec![0; b[2] as usize * b[3] as usize],
        };
        let n = ic.bits.len().min(b.len() - ICON_KOPF);
        ic.bits[..n].copy_from_slice(&b[ICON_KOPF..ICON_KOPF + n]);
        Ok(ic)
    }

    /// Aus Chunk 0x32 des Containers wie das Original (0x100037bc ff. und 0x10003e20): Daten[0] = Breite, [2] = Höhe,
    /// ab [4] die Bitmap mit der **untersten Zeile zuerst** und **invertiert** (Bit gesetzt = weiß).
    pub fn aus_chunk(d: &[u8]) -> Result<Icon, String> {
        if d.len() < 4 {
            return Err("Icon-Chunk zu kurz".into());
        }
        let (breite, hoehe) = (d[0] as usize, d[2] as usize);
        let bpz = breite.div_ceil(8);
        if d.len() - 4 != bpz * hoehe {
            return Err(format!("Icon-Chunk: {} Byte Bitmap statt {}", d.len() - 4, bpz * hoehe));
        }
        let mut ic = Icon::leer(breite, hoehe);
        for (z, zeile) in d[4..].chunks(bpz).enumerate() {
            let ziel = (hoehe - 1 - z) * bpz;
            for (i, &b) in zeile.iter().enumerate() {
                ic.bits[ziel + i] = !b;
            }
        }
        Ok(ic)
    }

    /// Chunk 0x32 (Umkehrung von [`Icon::aus_chunk`]).
    pub fn chunk_daten(&self) -> Vec<u8> {
        let mut v = vec![(self.bytes_je_zeile * 8) as u8, 0, self.hoehe as u8, 0];
        for zeile in self.bits.chunks(self.bytes_je_zeile).rev() {
            v.extend(zeile.iter().map(|b| !b));
        }
        v
    }

    /// Icon 72 × 62 aus den Stichen zeichnen: jede Stichlinie (nicht Sprünge), auf den Bereich x 2…70, y 2…60
    /// eingepasst (Seitenverhältnis bleibt, mittig). **Regel aus den Icons der Maschine abgeleitet** (43 Motive
    /// Nr. 8–51 ohne 41: 96,9 % der Icon-Punkte getroffen, jedes Motiv ≥ 80 %; Test `icons_der_maschine`) – wie die
    /// Original-Software das Icon (Chunk 0x32) zeichnet, ist noch nicht analysiert. Sicherheit: mittel.
    pub fn aus_stichen(folge: &StichFolge) -> Icon {
        let mut ic = Icon::leer(ICON_BREITE, ICON_HOEHE);
        let mut linien = Vec::new();
        for (a, b) in folge.stiche.iter().zip(folge.stiche.iter().skip(1)) {
            if b.art == Art::Stich {
                linien.push(((a.x, a.y), (b.x, b.y)));
            }
        }
        if linien.is_empty() {
            return ic;
        }
        let pkt = linien.iter().flat_map(|&(a, b)| [a, b]);
        let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for (x, y) in pkt {
            (x0, x1, y0, y1) = (x0.min(x), x1.max(x), y0.min(y), y1.max(y));
        }
        let (l, r, o, u) = (2.0, 70.0, 2.0, 60.0);
        let (w, h) = ((x1 - x0).max(0.1), (y1 - y0).max(0.1));
        let s = ((r - l) / w).min((u - o) / h);
        let ox = l + ((r - l) - w * s) / 2.0;
        let oy = o + ((u - o) - h * s) / 2.0;
        // in doppelter Auflösung zeichnen, je 2 × 2 Feinpunkte ein Icon-Punkt (passt besser zu den treppigen
        // Schräglinien der Maschinen-Icons als eine einfache Linie: 96,9 % statt 96,4 % getroffen, kein Motiv < 80 %)
        let bild = |(x, y): (f64, f64)| -> (i64, i64) {
            (
                ((ox + (x - x0) * s) * 2.0).round_ties_even() as i64,
                ((oy + (y1 - y) * s) * 2.0).round_ties_even() as i64,
            )
        };
        for (a, b) in linien {
            let ((xa, ya), (xb, yb)) = (bild(a), bild(b));
            let n = (xb - xa).abs().max((yb - ya).abs()).max(1);
            for k in 0..=n {
                let x = ((xa as f64 + (xb - xa) as f64 * k as f64 / n as f64) / 2.0).floor();
                let y = ((ya as f64 + (yb - ya) as f64 * k as f64 / n as f64) / 2.0).floor();
                if x >= 0.0 && y >= 0.0 {
                    ic.setze(x as usize, y as usize);
                }
            }
        }
        ic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn verschleiern_hin_und_rueck() {
        // alle vier Längenbereiche des Startzustands
        for l in [40usize, 3000, 9000, 40000, 70000] {
            let mut c: Vec<u8> = (0..l).map(|i| (i * 7 + i / 13) as u8).collect();
            c[..8].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
            c[2..6].copy_from_slice(&(l as u32).to_le_bytes());
            let mut v = c.clone();
            be_umkodieren(&mut v, true);
            assert_ne!(v, c);
            assert_eq!(v[..8], c[..8]);
            be_umkodieren(&mut v, false);
            assert_eq!(v, c);
        }
    }

    #[test]
    fn eigener_puffer_rundweg() {
        let farben = vec![
            Farbe {
                rgb: [255, 255, 250],
                nummer: 10,
                marke: "Isacord 40".into(),
                name: "white".into(),
            },
            Farbe::aus_hex("#c0392b"),
        ];
        let tabelle: Vec<u8> = farben.iter().flat_map(|f| f.bytes()).collect();
        let kopf = Kopf::neu("Test 1", 2, 100, 50, 1000);
        let c = Container {
            kennung: 0,
            wort: 0,
            chunks: vec![
                Chunk {
                    id: CHUNK_KOPF,
                    daten: kopf.chunk_daten(),
                },
                Chunk {
                    id: CHUNK_FARBEN,
                    daten: tabelle,
                },
            ],
        };
        let exp = [0x80, 0x04, 10, 0, 5, 5, 0x80, 0x01, 0, 0, 3, 3];
        let p = Puffer::neu(&exp, Some(&c));
        let roh = p.bytes();
        assert_eq!(roh.len(), 6 * N_185 + 8 + exp.len() + 2 + c.klar().len());
        let q = Puffer::lesen(&roh, N_185).unwrap();
        assert_eq!(q, p);
        assert!(q.anhang_gueltig() && q.endkennung());
        let c2 = q.container().unwrap().unwrap();
        assert_eq!(c2, c);
        assert_eq!(c2.kopf().unwrap().name(), "Test 1");
        assert_eq!(c2.farben(), farben);
        let info = q.info_felder(farben_in_stichen(&q.stiche));
        assert_eq!(info.farben_minus_1, 1);
        assert_eq!(info.flags, 0x84);
        assert_eq!(&info.name[..7], b"Test 1\0");
    }

    #[test]
    fn farbtabelle_wie_original() {
        let farben = vec![
            Farbe {
                rgb: [255, 255, 250],
                nummer: 10,
                marke: "Isacord 40".into(),
                name: "white".into(),
            },
            Farbe {
                rgb: [0xc0, 0x39, 0x2b],
                nummer: 0x1234,
                // mit 0 aufgefüllt: das Original schneidet die Leerzeichen dann NICHT ab (nur bei Auffüllung mit
                // Leerzeichen bis Stelle 0x3E, 0x10003b4b)
                marke: "Madeira   ".into(),
                name: String::new(),
            },
        ];
        let t = farbtabelle_maschine(&farben);
        let mut erwartet = vec![0, 0, 0, 1, 0, 0, 0, 8];
        erwartet.extend((16u32 + 26 + 21 - 8).to_be_bytes());
        erwartet.extend([0, 0, 0, 2]);
        erwartet.extend([0, 26, 0, 1, 0xff, 0xff, 0xfa, 0, 10]);
        erwartet.extend(b"Isacord 40\0white\0");
        erwartet.extend([0, 21, 0, 2, 0xc0, 0x39, 0x2b, 0x12, 0x34]);
        erwartet.extend(b"Madeira   \0\0");
        assert_eq!(t, erwartet);
        // Feld mit Leerzeichen aufgefüllt (wie Wilcom-Texte fester Länge): wird abgeschnitten
        let mut feld = [b' '; 64];
        feld[..4].copy_from_slice(b"Sulk");
        assert_eq!(text_fuer_tabelle(&feld), b"Sulk");

        // Bereich T wie in der Maschine: Vorspann (Byte 0 = 0), dann die Tabelle
        let g = FarbGrenzen::aus_zusatzblock(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0x10, 0x00]).unwrap();
        assert_eq!((g.max_farben, g.groesse, g.vorspann()), (50, 0x1000, 56));
        g.pruefen(farben.len(), &t).unwrap();
        let mut bereich = vec![0u8; g.vorspann()];
        bereich.extend(&t);
        let gelesen = farbtabelle_lesen(&bereich, g).unwrap();
        assert_eq!(gelesen[0], farben[0]);
        assert_eq!(gelesen[1].marke, "Madeira   ");
        assert_eq!(gelesen[1].nummer, 0x1234);
        bereich[0] = 1;
        assert!(farbtabelle_lesen(&bereich, g).is_err());
        assert!(g.pruefen(51, &t).is_err());
        assert!(
            FarbGrenzen {
                max_farben: 50,
                groesse: 80
            }
            .pruefen(2, &t)
            .is_err()
        );
    }

    #[test]
    fn icon_chunk_und_puffer() {
        let mut ic = Icon::leer(ICON_BREITE, ICON_HOEHE);
        ic.setze(0, 0);
        ic.setze(71, 61);
        ic.setze(10, 3);
        let b = ic.bytes();
        assert_eq!(b.len(), ICON_KOPF + 9 * 62);
        assert_eq!(&b[..6], &[0, 0, 9, 62, 0xff, 0]);
        assert_eq!(Icon::lesen(&b).unwrap(), ic);
        let d = ic.chunk_daten();
        assert_eq!(&d[..4], &[72, 0, 62, 0]);
        assert_eq!(d[4 + 61 * 9], 0x7f); // oberste Zeile steht im Chunk zuletzt, invertiert
        assert_eq!(Icon::aus_chunk(&d).unwrap(), ic);
        assert!(ic.punkt(10, 3) && !ic.punkt(11, 3));
    }

    #[test]
    fn anhang_ohne_farben() {
        let c = Container {
            kennung: 0,
            wort: 0,
            chunks: [CHUNK_KOPF, CHUNK_FARBEN, CHUNK_ICON, CHUNK_ZUSATZ]
                .iter()
                .map(|&id| Chunk { id, daten: vec![1, 2] })
                .collect(),
        };
        let a = c.anhang_wie_original();
        assert_eq!(a.chunks.len(), 1);
        assert_eq!(a.chunks[0].id, CHUNK_KOPF);
    }

    /// Icons aus dem Maschinenspeicher (je 43 von 62 Zeilen gelesen) mit [`Icon::aus_stichen`] vergleichen. Die
    /// Zeichensätze Nr. 1–7 (Icon = Schriftprobe) und Nr. 41 (Rahmen-Vorlage) zeigen nicht ihre Stiche.
    #[test]
    fn icons_der_maschine() {
        let ordner = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Arbeitsdaten/maschinendesigns");
        let Ok(eintraege) = std::fs::read_dir(&ordner) else {
            eprintln!("übersprungen: {} fehlt", ordner.display());
            return;
        };
        let (mut treffer, mut punkte, mut zuviel, mut designs) = (0usize, 0usize, 0usize, 0);
        for e in eintraege.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let Some(nr) = name.strip_prefix("Icon_Nr").and_then(|r| r[..2].parse::<u32>().ok()) else {
                continue;
            };
            if nr < 8 || nr == 41 {
                continue;
            }
            let Ok(exp) = std::fs::read(ordner.join(format!("Nr{nr:02}/stiche.exp"))) else {
                continue;
            };
            let roh = std::fs::read(e.path()).unwrap();
            let zeilen = (roh.len() - ICON_KOPF) / 9;
            let echt = Icon::lesen(&roh).unwrap();
            let neu = Icon::aus_stichen(&crate::exp::exp_lesen(&exp));
            let (mut t, mut p, mut z) = (0, 0, 0);
            for y in 0..zeilen.min(ICON_HOEHE) {
                for x in 0..ICON_BREITE {
                    let (a, b) = (echt.punkt(x, y), neu.punkt(x, y));
                    p += a as usize;
                    t += (a && b) as usize;
                    z += (b && !a) as usize;
                }
            }
            assert!(t * 10 >= p * 8, "Nr {nr}: nur {t} von {p} Icon-Punkten getroffen");
            (treffer, punkte, zuviel, designs) = (treffer + t, punkte + p, zuviel + z, designs + 1);
        }
        if designs == 0 {
            eprintln!("übersprungen: keine Icons in {}", ordner.display());
            return;
        }
        eprintln!("{designs} Icons: {treffer} von {punkte} Punkten getroffen, {zuviel} zusätzlich");
        assert!(designs >= 40);
        assert!(treffer * 1000 >= punkte * 965, "nur {treffer} von {punkte} getroffen");
    }

    /// Flags je Designnummer aus `Arbeitsdaten/maschinendesigns/UEBERSICHT.md` (Spalte „Flags“ der Designliste).
    fn flags_der_liste(ordner: &std::path::Path) -> BTreeMap<u32, u8> {
        let text = std::fs::read_to_string(ordner.join("UEBERSICHT.md")).unwrap_or_default();
        text.lines()
            .filter_map(|z| {
                let s: Vec<&str> = z.split('|').map(str::trim).collect();
                let nr = s.get(1)?.parse().ok()?;
                let fl = u8::from_str_radix(s.get(3)?.strip_prefix("0x")?, 16).ok()?;
                Some((nr, fl))
            })
            .collect()
    }

    /// Alle 51 Designs aus dem Maschinenspeicher: zerlegen und neu zusammensetzen ergibt dieselben Bytes;
    /// gültige Anhänge entschleiern → verschleiern ergibt denselben Anhang; Flags aus dem Kopf = Flags der Liste.
    #[test]
    fn rundweg_maschinendesigns() {
        let ordner = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Arbeitsdaten/maschinendesigns");
        let Ok(eintraege) = std::fs::read_dir(&ordner) else {
            eprintln!("übersprungen: {} fehlt", ordner.display());
            return;
        };
        let liste = flags_der_liste(&ordner);
        // je Nummer die neueste Datei
        let mut neueste: BTreeMap<u32, PathBuf> = BTreeMap::new();
        for e in eintraege.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if let Some(rest) = name.strip_prefix("Design_Nr")
                && name.ends_with(".bin")
                && let Ok(nr) = rest[..2].parse::<u32>()
                && neueste
                    .get(&nr)
                    .is_none_or(|alt| *alt.file_name().unwrap().to_string_lossy() < *name)
            {
                neueste.insert(nr, e.path());
            }
        }
        if neueste.is_empty() {
            eprintln!("übersprungen: keine Designs in {}", ordner.display());
            return;
        }
        let (mut mit_anhang, mut flags_gleich) = (0, 0);
        for (nr, pfad) in &neueste {
            let roh = std::fs::read(pfad).unwrap();
            let p = Puffer::lesen(&roh, N_185).unwrap_or_else(|e| panic!("Nr {nr}: {e}"));
            assert_eq!(p.bytes(), roh, "Nr {nr}: neu zusammengesetzt ≠ gelesen");
            assert!(p.endkennung(), "Nr {nr}: keine Endkennung");
            assert!(
                p.tabellen.iter().all(|t| t.iter().all(|&w| w == 0)),
                "Nr {nr}: Tabellen nicht 0"
            );
            if let Some(c) = p.container() {
                let c = c.unwrap_or_else(|e| panic!("Nr {nr}: {e}"));
                assert_eq!(c.verschleiert(), p.anhang, "Nr {nr}: Anhang nicht byte-gleich");
                let kopf = c.kopf().expect("Kopf-Chunk");
                assert_eq!(c.farben().len(), farben_in_stichen(&p.stiche), "Nr {nr}: Farbtabelle");
                if let Some(&fl) = liste.get(nr) {
                    assert_eq!(kopf.flags(), fl, "Nr {nr}: Flags");
                    flags_gleich += 1;
                }
                mit_anhang += 1;
            }
        }
        assert_eq!(neueste.len(), 51);
        assert_eq!(mit_anhang, 38);
        assert_eq!(flags_gleich, 38, "Flags der Liste (UEBERSICHT.md) nicht geprüft");
    }
}
