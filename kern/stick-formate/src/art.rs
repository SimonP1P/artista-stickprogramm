//! ART-Designs der BERNINA artista V4 (Wilcom) öffnen – mit den Objekten des Originals und ihren Stichen
//! (Nachtauftrag N11, ROADMAP 4b.13c; `Analyse_Teil3/27_stickdateien.md` §6).
//!
//! Aufbau und Belege:
//! - OLE-Datei; neuere ART: Stream `DesignDocument` = Twofish-128-CFB1 (Schlüssel „1d43f2ae“ + Format-Schlüssel 0xD0,
//!   IV `a885f60d…`, IMPORT.DLL 0x101164e0 ff.), ältere: Stream `Contents` = Substitution mit Format-Schlüssel 0xD0;
//!   danach u32 Länge + zlib (`Analyse/code/05`).
//! - Datenblock: Kopf 29 × u32 (Magic 0x193866de, Prüfsumme), Abschnitt 1 = Stichpunkte, Abschnitt 2 = Segmente,
//!   Objektteil = Datensätze (`Analyse_Teil3/02`, `03`).
//! - Je Objekt ein Datensatz der Klasse 0x03 (Leser IMPORT.DLL 0x10017180): Ein- und Ausstich, Liste der
//!   Stichsegmente (Nummer + 1 = Segment in Abschnitt 2, an allen Musterdesigns geprüft), je Farbfolge ein Eintrag mit
//!   der **Garnnummer** (i32) in der Garnbibliothek 0x41 (je Garn RGB, Nummer, Marke, Name).
//! - Segmente der Art 0 sind Verbinder: ihre Stiche werden Sprünge (eigene Festlegung, mittel – Häufung bei 7 mm =
//!   Sprunglänge 0x2712).

use crate::esb::{packed, packed_wort, substitution_entschluesseln};
use crate::stickdatei::{Block, STANDARD, Stickdatei, zentrieren};
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read};

/// Format-Schlüssel der ART-Designs (Format-ID 0xD0, `Analyse/code/05`)
const FORMAT_SCHLUESSEL_ART: u32 = 0x01c0_d932;
/// Twofish-Schlüssel „1d43f2ae%08x%08x%08x“ mit dem Format-Schlüssel 0xD0 und zweimal 0
const SCHLUESSEL: [u8; 16] = [0x1d, 0x43, 0xf2, 0xae, 0x01, 0xc0, 0xd9, 0x32, 0, 0, 0, 0, 0, 0, 0, 0];
const IV: [u8; 16] = [
    0xa8, 0x85, 0xf6, 0x0d, 0x87, 0xe9, 0x0b, 0x71, 0xc2, 0x0a, 0x81, 0xbc, 0xc3, 0x49, 0x8a, 0xf1,
];

// ------------------------------------------------------------------------------------------------ Twofish

/// Twofish mit 128-Bit-Schlüssel wie die Referenzimplementierung in IMPORT.DLL (makeKey 0x101164e0,
/// blockEncrypt 0x101165c0); nur Verschlüsseln, mehr braucht CFB1 nicht.
mod twofish {
    const MDS: [[u8; 4]; 4] = [
        [0x01, 0xEF, 0x5B, 0x5B],
        [0x5B, 0xEF, 0xEF, 0x01],
        [0xEF, 0x5B, 0x01, 0xEF],
        [0xEF, 0x01, 0xEF, 0x5B],
    ];
    const RS: [[u8; 8]; 4] = [
        [0x01, 0xA4, 0x55, 0x87, 0x5A, 0x58, 0xDB, 0x9E],
        [0xA4, 0x56, 0x82, 0xF3, 0x1E, 0xC6, 0x68, 0xE5],
        [0x02, 0xA1, 0xFC, 0xC1, 0x47, 0xAE, 0x3D, 0x19],
        [0xA4, 0x55, 0x87, 0x5A, 0x58, 0xDB, 0x9E, 0x03],
    ];
    /// Nibble-Tabellen der Permutationen q0 und q1 (Twofish-Spezifikation)
    const T: [[[u8; 16]; 4]; 2] = [
        [
            [8, 1, 7, 13, 6, 15, 3, 2, 0, 11, 5, 9, 14, 12, 10, 4],
            [14, 12, 11, 8, 1, 2, 3, 5, 15, 4, 10, 6, 7, 0, 9, 13],
            [11, 10, 5, 14, 6, 13, 9, 0, 12, 8, 15, 3, 2, 4, 7, 1],
            [13, 7, 15, 4, 1, 2, 6, 14, 9, 11, 3, 0, 8, 5, 12, 10],
        ],
        [
            [2, 8, 11, 13, 15, 7, 6, 14, 3, 1, 9, 4, 0, 10, 12, 5],
            [1, 14, 2, 11, 4, 12, 3, 7, 6, 13, 10, 5, 15, 9, 0, 8],
            [4, 12, 7, 5, 1, 6, 9, 10, 0, 14, 13, 8, 2, 11, 3, 15],
            [11, 9, 5, 1, 12, 3, 13, 14, 6, 4, 7, 15, 2, 0, 8, 10],
        ],
    ];

    fn gfmul(a: u8, b: u8, poly: u32) -> u8 {
        let (mut r, mut x, mut b) = (0u32, a as u32, b);
        while b != 0 {
            if b & 1 != 0 {
                r ^= x;
            }
            x <<= 1;
            if x & 0x100 != 0 {
                x ^= poly;
            }
            b >>= 1;
        }
        r as u8
    }

    fn q_tabelle(t: &[[u8; 16]; 4]) -> [u8; 256] {
        let ror4 = |x: u8| ((x >> 1) | (x << 3)) & 15;
        let mut q = [0u8; 256];
        for (x, z) in q.iter_mut().enumerate() {
            let (a0, b0) = ((x >> 4) as u8, (x & 15) as u8);
            let (a1, b1) = (a0 ^ b0, (a0 ^ ror4(b0) ^ (8 * a0)) & 15);
            let (a2, b2) = (t[0][a1 as usize], t[1][b1 as usize]);
            let (a3, b3) = (a2 ^ b2, (a2 ^ ror4(b2) ^ (8 * a2)) & 15);
            let (a4, b4) = (t[2][a3 as usize], t[3][b3 as usize]);
            *z = (b4 << 4) | a4;
        }
        q
    }

    pub struct Twofish {
        k: [u32; 40],
        /// g-Funktion je Byte-Stelle: MDS-Spalte · q-Kette mit den Schlüssel-S-Boxen
        g: [[u32; 256]; 4],
    }

    impl Twofish {
        pub fn neu(schluessel: &[u8; 16]) -> Twofish {
            let q = [q_tabelle(&T[0]), q_tabelle(&T[1])];
            let (q0, q1) = (&q[0], &q[1]);
            let w = |i: usize| u32::from_le_bytes(schluessel[4 * i..4 * i + 4].try_into().unwrap());
            let (me, mo) = ([w(0), w(2)], [w(1), w(3)]);
            let mut s = [0u32; 2];
            for i in 0..2 {
                let mut b = [0u8; 4];
                for (r, z) in b.iter_mut().enumerate() {
                    *z = (0..8).fold(0, |v, c| v ^ gfmul(RS[r][c], schluessel[8 * i + c], 0x14D));
                }
                s[1 - i] = u32::from_le_bytes(b);
            }
            // h(X, L) wie die Referenz: y = q(q(q(x) ^ l1) ^ l0) je Byte, dann MDS
            let kette = |stelle: usize, x: u8, l: &[u32; 2]| -> u8 {
                let (l0, l1) = (l[0].to_le_bytes()[stelle], l[1].to_le_bytes()[stelle]);
                match stelle {
                    0 => q1[(q0[(q0[x as usize] ^ l1) as usize] ^ l0) as usize],
                    1 => q0[(q0[(q1[x as usize] ^ l1) as usize] ^ l0) as usize],
                    2 => q1[(q1[(q0[x as usize] ^ l1) as usize] ^ l0) as usize],
                    _ => q0[(q1[(q1[x as usize] ^ l1) as usize] ^ l0) as usize],
                }
            };
            let spalte = |stelle: usize, y: u8| -> u32 {
                (0..4).fold(0u32, |z, i| z | (gfmul(MDS[i][stelle], y, 0x169) as u32) << (8 * i))
            };
            let h = |x: u32, l: &[u32; 2]| -> u32 {
                (0..4).fold(0, |z, st| z ^ spalte(st, kette(st, x.to_le_bytes()[st], l)))
            };
            let mut k = [0u32; 40];
            for i in 0..20u32 {
                let a = h(2 * i * 0x0101_0101, &me);
                let b = h((2 * i + 1) * 0x0101_0101, &mo).rotate_left(8);
                k[2 * i as usize] = a.wrapping_add(b);
                k[2 * i as usize + 1] = a.wrapping_add(b.wrapping_mul(2)).rotate_left(9);
            }
            let mut g = [[0u32; 256]; 4];
            for (st, t) in g.iter_mut().enumerate() {
                for (x, z) in t.iter_mut().enumerate() {
                    *z = spalte(st, kette(st, x as u8, &s));
                }
            }
            Twofish { k, g }
        }

        fn gf(&self, x: u32) -> u32 {
            let b = x.to_le_bytes();
            self.g[0][b[0] as usize] ^ self.g[1][b[1] as usize] ^ self.g[2][b[2] as usize] ^ self.g[3][b[3] as usize]
        }

        pub fn verschluesseln(&self, ein: &[u8; 16]) -> [u8; 16] {
            let k = &self.k;
            let mut r = [0u32; 4];
            for (i, z) in r.iter_mut().enumerate() {
                *z = u32::from_le_bytes(ein[4 * i..4 * i + 4].try_into().unwrap()) ^ k[i];
            }
            for runde in 0..16 {
                let t0 = self.gf(r[0]);
                let t1 = self.gf(r[1].rotate_left(8));
                let f0 = t0.wrapping_add(t1).wrapping_add(k[2 * runde + 8]);
                let f1 = t0.wrapping_add(t1.wrapping_mul(2)).wrapping_add(k[2 * runde + 9]);
                r[2] = (r[2] ^ f0).rotate_right(1);
                r[3] = r[3].rotate_left(1) ^ f1;
                r = [r[2], r[3], r[0], r[1]];
            }
            let c = [r[2] ^ k[4], r[3] ^ k[5], r[0] ^ k[6], r[1] ^ k[7]];
            let mut aus = [0u8; 16];
            for (i, w) in c.iter().enumerate() {
                aus[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
            }
            aus
        }
    }

    /// CFB1 entschlüsseln (je Bit ein Block, Chiffretext-Bit geht ins IV) wie `twofish_cfb1.c`
    pub fn cfb1_entschluesseln(tf: &Twofish, iv: &[u8; 16], ein: &[u8]) -> Vec<u8> {
        let mut v = u128::from_be_bytes(*iv);
        let mut aus = vec![0u8; ein.len()];
        for (n, &c) in ein.iter().enumerate() {
            let mut p = 0u8;
            for sh in 0..8 {
                let x = tf.verschluesseln(&v.to_be_bytes());
                let bit = (c >> (7 - sh)) & 1;
                p |= (bit ^ (x[0] >> 7)) << (7 - sh);
                v = (v << 1) | bit as u128;
            }
            aus[n] = p;
        }
        aus
    }
}

// ------------------------------------------------------------------------------------------------ Datenblock

fn zlib_mit_laenge(d: &[u8]) -> Option<Vec<u8>> {
    let laenge = u32::from_le_bytes(d.get(..4)?.try_into().ok()?) as usize;
    let mut out = Vec::with_capacity(laenge);
    (ZlibDecoder::new(&d[4..]).read_to_end(&mut out).is_ok() && out.len() == laenge).then_some(out)
}

/// Klartext des Designdatenblocks aus der OLE-Datei: `DesignDocument` (neuere ART) oder `Contents` (ältere).
pub fn datenblock(daten: &[u8]) -> Result<Vec<u8>, String> {
    let mut ole = cfb::CompoundFile::open(Cursor::new(daten)).map_err(|e| format!("ART: keine OLE-Datei ({e})"))?;
    let mut lies = |name: &str| -> Option<Vec<u8>> {
        let mut roh = Vec::new();
        ole.open_stream(name).ok()?.read_to_end(&mut roh).ok()?;
        Some(roh)
    };
    if let Some(roh) = lies("/DesignDocument") {
        let tf = twofish::Twofish::neu(&SCHLUESSEL);
        let klar = twofish::cfb1_entschluesseln(&tf, &IV, &roh);
        return zlib_mit_laenge(&klar).ok_or_else(|| "ART: DesignDocument nicht lesbar".to_string());
    }
    let roh = lies("/Contents").ok_or("ART: weder DesignDocument noch Contents")?;
    substitution_entschluesseln(&roh, FORMAT_SCHLUESSEL_ART).ok_or_else(|| "ART: Contents nicht lesbar".to_string())
}

fn kopf(d: &[u8]) -> Result<[u32; 29], String> {
    if d.len() < 0x74 {
        return Err("ART: Datenblock zu kurz".into());
    }
    let mut h = [0u32; 29];
    for (i, z) in h.iter_mut().enumerate() {
        *z = u32::from_le_bytes(d[4 * i..4 * i + 4].try_into().unwrap());
    }
    if h[0] != 0x1938_66de {
        return Err("ART: kein Designdatenblock (Magic fehlt)".into());
    }
    if h[..28].iter().fold(0u32, |s, &x| s.wrapping_add(x)) != h[28] {
        return Err("ART: Prüfsumme des Kopfes falsch".into());
    }
    Ok(h)
}

/// Abschnitt 1 (IMPORT.DLL 0x10037330): absolute Punkte, x in 1/180 mm, y in 1/360 mm
fn stichpunkte(d: &[u8], start: usize, anzahl: usize) -> Option<Vec<(i64, i64)>> {
    #[derive(Default)]
    struct Achse {
        pos: i64,
        schritt: i64,
        betrag: i64,
        vorzeichen: i64,
    }
    impl Achse {
        fn anwenden(&mut self, c: i64) -> i64 {
            let s;
            if c < 8 {
                let d = if c & 1 != 0 { !(c >> 1) } else { c >> 1 };
                s = self.schritt + d;
                (self.betrag, self.vorzeichen) = if s >= 0 { (s, 1) } else { (!s, -1) };
            } else {
                let c2 = (c & 7) | ((c >> 1) & 0x7FFF_FFF8);
                let m = if c2 & 2 != 0 { !(c2 >> 2) } else { c2 >> 2 };
                self.betrag += m;
                if c2 & 1 != 0 {
                    self.vorzeichen = -self.vorzeichen;
                }
                s = if self.vorzeichen != -1 {
                    self.betrag
                } else {
                    !self.betrag
                };
            }
            self.schritt = s;
            self.pos += s;
            self.pos
        }
    }
    let (mut ax, mut ay) = (
        Achse {
            vorzeichen: 1,
            ..Default::default()
        },
        Achse {
            vorzeichen: 1,
            ..Default::default()
        },
    );
    let mut i = start;
    let mut out = Vec::with_capacity(anzahl);
    for _ in 0..anzahl {
        let b = *d.get(i)? as i64;
        i += 1;
        let (mut cx, mut cy) = (b & 0xF, b >> 4);
        if cx & 8 != 0 {
            cx |= (packed(d, &mut i)? as i64) << 4;
        }
        if cy & 8 != 0 {
            cy |= (packed(d, &mut i)? as i64) << 4;
        }
        out.push((ax.anwenden(cx), ay.anwenden(cy)));
    }
    Some(out)
}

/// Abschnitt 2 (IMPORT.DLL 0x10036640): je Segment Art (0 = Verbinder/Grenze, 1 = Stiche) und Stichzahl
fn segmente(d: &[u8], start: usize, anzahl: usize, version: u32) -> Option<Vec<(u8, usize)>> {
    let mut i = start;
    let mut out = Vec::with_capacity(anzahl);
    for _ in 0..anzahl {
        let u = packed(d, &mut i)?;
        for _ in 0..4 {
            packed_wort(d, &mut i)?;
        }
        packed(d, &mut i)?;
        let k = packed(d, &mut i)? as usize;
        let je = 3 + usize::from(version >= 6) + if version >= 0x14 { 8 } else { 0 };
        i += k * je;
        out.push(((u & 1) as u8, (u >> 1) as usize));
    }
    (i <= d.len()).then_some(out)
}

/// Datensatzkopf (IMPORT.DLL 0x100d9bf0): Klasse, Verweis?, Länge bzw. Ziel, Anfang der Nutzdaten
fn datensatz(d: &[u8], i: &mut usize) -> Option<(u64, bool, usize)> {
    let u = packed(d, i)?;
    let (klasse, verweis) = (u >> 1, u & 1 == 1);
    if klasse == 0 {
        return Some((0, false, 0));
    }
    let v = packed(d, i)?;
    Some((klasse, verweis, if verweis { v as usize } else { (v >> 2) as usize }))
}

/// Ein Garn der Garnbibliothek (Klasse 0x41)
#[derive(Debug, Clone, PartialEq)]
pub struct Garn {
    pub rgb: [u8; 3],
    pub nummer: String,
    pub marke: String,
    pub name: String,
}

/// Ein Objekt des Originals mit seinen Segmenten
#[derive(Debug, Clone, PartialEq)]
pub struct Objekt {
    /// Klasse des Objekts (0x07 ComplexFillGroup, 0x05 EmbroideryCurve …)
    pub klasse: u64,
    /// Nummern der Segmente in Abschnitt 2
    pub segmente: Vec<usize>,
    /// Garnnummer in der Garnbibliothek
    pub garn: Option<usize>,
}

/// Name der Objektklasse für die Oberfläche (Klassennamen aus IMPORT.DLL, `Analyse_Teil3/03` §3, `art_design.py`)
pub fn klassenname(k: u64) -> &'static str {
    match k {
        0x05 => "Linie",                     // EmbroideryCurve
        0x06 | 0x07 => "Fläche",             // EmbroideryPatch, ComplexFillGroup
        0x08 | 0x49 => "Gruppe",             // EmbroideryBoundGroup, EmbroideryGroup
        0x0b => "Manuelle Stiche",           // ManualObject
        0x0c => "Satinsäule",                // InputCCurve
        0x2f => "Buchstabe",                 // LetterObj
        0x30 | 0x7b => "Text",               // OldAlphaText, LetterText
        0x7a => "Motivlinie",                // MotifText
        0x36 => "Ellipse",                   // EmbroideryEllipse
        0x37 => "Ring",                      // EmbroideryRing
        0x38 => "Stern",                     // EmbroideryStar
        0x72 | 0x73 | 0x93 => "Applikation", // AutoAppliqueGroup, AppliqueCurve, Applique
        0x95 => "Knopfloch",                 // ButtonHole
        _ => "Objekt",
    }
}

/// Klassen, die ein Objekt beginnen (`art_design.py` OBJEKT_KLASSEN); Füllungen wie MotifFill 0x3d gehören zum Objekt
const OBJEKT_KLASSEN: [u64; 19] = [
    0x05, 0x06, 0x07, 0x08, 0x0b, 0x0c, 0x2f, 0x30, 0x36, 0x37, 0x38, 0x49, 0x71, 0x72, 0x73, 0x7a, 0x7b, 0x93, 0x95,
];

/// Objektteil lesen: Objekte (je Datensatz 0x03) und Garnbibliothek (0x41). Der Rahmen endet bei Klassen mit eigenem
/// Rohdatenteil (z. B. Bitmap der Auto-Digitalisierung) – dann bleibt es bei dem, was bis dahin gelesen ist.
fn objektteil(d: &[u8], start: usize) -> (Vec<Objekt>, Vec<Garn>) {
    let mut i = start;
    let (mut objekte, mut garne) = (Vec::new(), Vec::new());
    let mut klasse = 0u64;
    let mut liste = false;
    while i < d.len() {
        let Some((k, verweis, l)) = datensatz(d, &mut i) else {
            break;
        };
        if k == 0 || verweis {
            continue;
        }
        if (k > 0x9d && k != 0x2a2) || l > d.len() - i {
            break;
        }
        let nutz = &d[i..i + l];
        i += l;
        if k == 0x0a {
            liste = true;
        }
        if !liste {
            continue;
        }
        if OBJEKT_KLASSEN.contains(&k) {
            klasse = k;
        }
        if k == 0x03
            && let Some(o) = klasse3(nutz, klasse)
        {
            objekte.push(o);
        }
        if k == 0x41 {
            garne = garnbibliothek(nutz);
            break;
        }
    }
    (objekte, garne)
}

/// Datensatz 0x03 (IMPORT.DLL 0x10017180): 4 Wörter Ein-/Ausstich, Segmentliste, je Farbfolge Garnnummer
fn klasse3(n: &[u8], klasse: u64) -> Option<Objekt> {
    let mut q = 0;
    for _ in 0..4 {
        packed_wort(n, &mut q)?;
    }
    let anzahl = packed(n, &mut q)? as usize;
    let mut segmente = Vec::with_capacity(anzahl);
    for _ in 0..anzahl {
        segmente.push(packed_wort(n, &mut q)? as usize + 1);
    }
    let folgen = packed(n, &mut q)?;
    q += 2;
    let garn = (folgen > 0)
        .then(|| n.get(q + 2..q + 6).map(|b| i32::from_le_bytes(b.try_into().unwrap())))
        .flatten()
        .and_then(|g| usize::try_from(g).ok());
    Some(Objekt { klasse, segmente, garn })
}

/// Garnbibliothek 0x41: u16 Anzahl, je Garn R G B 0, drei Längen, Nummer + Marke + Name, 0
fn garnbibliothek(n: &[u8]) -> Vec<Garn> {
    let mut out = Vec::new();
    let Some(anzahl) = n.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]])) else {
        return out;
    };
    let mut q = 2;
    for _ in 0..anzahl {
        let Some(k) = n.get(q..q + 7) else { break };
        let (l1, l2, l3) = (k[4] as usize, k[5] as usize, k[6] as usize);
        let rgb = [k[0], k[1], k[2]];
        q += 7;
        let Some(s) = n.get(q..q + l1 + l2 + l3) else { break };
        let text = |a: usize, b: usize| String::from_utf8_lossy(&s[a..b]).trim().to_string();
        out.push(Garn {
            rgb,
            nummer: text(0, l1),
            marke: text(l1, l1 + l2),
            name: text(l1 + l2, l1 + l2 + l3),
        });
        q += l1 + l2 + l3 + 1;
    }
    out
}

/// ART-Design lesen: je Objekt des Originals ein Block mit dessen Stichen (Züge an Verbindern geteilt), Farbe aus der
/// Garnbibliothek. Ohne lesbare Objekte (Auto-Digitalisier-Designs) ein Block mit allen Stichen.
pub fn lesen(daten: &[u8]) -> Result<Stickdatei, String> {
    let d = datenblock(daten)?;
    let h = kopf(&d)?;
    let punkte = stichpunkte(&d, h[6] as usize, h[5] as usize).ok_or("ART: Stichpunkte nicht lesbar")?;
    let segs = segmente(&d, h[9] as usize, h[8] as usize, h[4]).ok_or("ART: Segmente nicht lesbar")?;
    let (objekte, garne) = objektteil(&d, h[14] as usize);
    let mm = |k: usize| -> Option<(f64, f64)> { punkte.get(k).map(|&(x, y)| (x as f64 / 180.0, y as f64 / 360.0)) };
    // Anfang (Punktnummer) jedes Segments
    let mut anfang = Vec::with_capacity(segs.len());
    let mut c = 0usize;
    for &(_, n) in &segs {
        anfang.push(c);
        c += n;
    }
    // Segment → Objekt; nicht zugeordnete Stich-Segmente gehören zum Objekt davor
    let mut gehoert: Vec<Option<usize>> = vec![None; segs.len()];
    for (o, obj) in objekte.iter().enumerate() {
        if obj.klasse == 0x71 {
            continue; // AutoStartEndObject: nur Verbinder zum Anfangs-/Endpunkt
        }
        for &s in &obj.segmente {
            if let Some(g) = gehoert.get_mut(s) {
                *g = Some(o);
            }
        }
    }
    let mut hinweise = Vec::new();
    let zugeordnet = gehoert.iter().flatten().count();
    let ohne_objekte = zugeordnet == 0;
    if ohne_objekte {
        hinweise.push("ART: Objekte nicht lesbar (z. B. Auto-Digitalisierung) – alle Stiche als ein Objekt".into());
    }
    let mut letztes: Option<usize> = None;
    let mut bloecke: Vec<(usize, Block)> = Vec::new();
    let mut zug_neu = true;
    for (k, &(art, n)) in segs.iter().enumerate() {
        if art == 0 {
            // Verbinder: Stiche = Sprünge; der Endpunkt ist der Anfang des nächsten Zugs
            if n > 0 {
                zug_neu = true;
            }
            continue;
        }
        if n == 0 {
            continue;
        }
        let o = if ohne_objekte { Some(0) } else { gehoert[k].or(letztes) };
        let Some(o) = o else { continue };
        let objekt_neu = letztes != Some(o) || bloecke.is_empty();
        if objekt_neu {
            let garn = objekte.get(o).and_then(|x| x.garn).and_then(|g| garne.get(g));
            let klasse = if ohne_objekte { 0 } else { objekte[o].klasse };
            let farbe = garn.map(|g| g.rgb).unwrap_or(STANDARD[bloecke.len() % STANDARD.len()]);
            let mut name = klassenname(klasse).to_string();
            if let Some(g) = garn {
                name = format!("{name} · {} {} {}", g.marke, g.nummer, g.name)
                    .trim_end()
                    .to_string();
            }
            bloecke.push((
                o,
                Block {
                    farbe,
                    name: Some(name),
                    zuege: Vec::new(),
                },
            ));
            zug_neu = true;
        }
        letztes = Some(o);
        let b = &mut bloecke.last_mut().unwrap().1;
        if zug_neu || b.zuege.is_empty() {
            let a = mm(anfang[k]).unwrap_or_default();
            b.zuege.push(vec![a]);
            zug_neu = false;
        }
        let z = b.zuege.last_mut().unwrap();
        z.extend((1..=n).filter_map(|t| mm(anfang[k] + t)));
    }
    let mut s = Stickdatei {
        format: "ART",
        name: None,
        bloecke: bloecke
            .into_iter()
            .map(|(_, b)| b)
            .filter(|b| !b.zuege.is_empty())
            .collect(),
        hinweise,
    };
    zentrieren(&mut s);
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twofish_testvektor() {
        // Twofish-Spezifikation: Schlüssel 0, Klartext 0 → 9F589F5CF6122C32B6BFEC2F2AE8C35A
        let tf = twofish::Twofish::neu(&[0; 16]);
        let c = tf.verschluesseln(&[0; 16]);
        let soll = [
            0x9F, 0x58, 0x9F, 0x5C, 0xF6, 0x12, 0x2C, 0x32, 0xB6, 0xBF, 0xEC, 0x2F, 0x2A, 0xE8, 0xC3, 0x5A,
        ];
        assert_eq!(c, soll);
    }

    #[test]
    fn seal_aus_den_musterdesigns() {
        // Samples/Seal.ART der Original-CD (kern/testdaten/art): 11 Objekte, davon 2 AutoStartEnd → 9 mit Stichen
        let pfad = concat!(env!("CARGO_MANIFEST_DIR"), "/../testdaten/art/Seal.ART");
        let s = lesen(&std::fs::read(pfad).unwrap()).unwrap();
        assert_eq!(s.format, "ART");
        assert!(s.hinweise.is_empty(), "{:?}", s.hinweise);
        assert_eq!(s.bloecke.len(), 9);
        assert_eq!(s.bloecke[0].farbe, [0x66, 0x99, 0xff]);
        assert_eq!(
            s.bloecke[0].name.as_deref(),
            Some("Ellipse · Madeira 40 1029 Medium blue")
        );
        assert_eq!(s.bloecke[1].name.as_deref(), Some("Fläche · Madeira 40 1118 Grey"));
        assert!(
            s.bloecke[4..]
                .iter()
                .all(|b| b.farbe == [0, 0, 0] && b.name.as_deref().unwrap().starts_with("Linie"))
        );
        let (x0, y0, x1, y1) = s.grenzen().unwrap();
        assert!(
            ((x1 - x0) - 52.63).abs() < 0.01 && ((y1 - y0) - 85.42).abs() < 0.01,
            "{x0} {y0} {x1} {y1}"
        );
        // Mitte auf 0,0
        assert!((x0 + x1).abs() < 1e-6 && (y0 + y1).abs() < 1e-6);
        // alle Stiche der Segmente mit Art 1 sind zugeordnet (Einstiche = Stiche + Anfangspunkt je Zug) – bis auf
        // Segment 0 mit 2 Stichen vor dem ersten Objekt (Anfangspunkt, keinem Objekt zugeordnet)
        let d = datenblock(&std::fs::read(pfad).unwrap()).unwrap();
        let h = kopf(&d).unwrap();
        let segs = segmente(&d, h[9] as usize, h[8] as usize, h[4]).unwrap();
        let soll: usize = segs.iter().filter(|x| x.0 == 1).map(|x| x.1).sum();
        let zuege: usize = s.bloecke.iter().map(|b| b.zuege.len()).sum();
        assert_eq!(segs[0], (1, 2));
        assert_eq!(s.stiche(), soll - 2 + zuege);
    }

    #[test]
    fn garnbibliothek_lesen() {
        let mut n = vec![2, 0];
        n.extend([0xe6, 0x33, 0x7d, 0, 4, 5, 8]);
        n.extend(b"1191Sulkydk. rose\0");
        n.extend([0, 0xff, 0, 0, 1, 7, 5]);
        n.extend(b"1DefaultGreen\0");
        let g = garnbibliothek(&n);
        assert_eq!(g.len(), 2);
        assert_eq!(g[0].rgb, [0xe6, 0x33, 0x7d]);
        assert_eq!(
            (g[0].nummer.as_str(), g[0].marke.as_str(), g[0].name.as_str()),
            ("1191", "Sulky", "dk. rose")
        );
        assert_eq!((g[1].nummer.as_str(), g[1].name.as_str()), ("1", "Green"));
    }
}
