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
            .map(|d| d.as_chunks::<FARB_EINTRAG>().0.iter().map(|e| Farbe::lesen(e)).collect())
            .unwrap_or_default()
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
