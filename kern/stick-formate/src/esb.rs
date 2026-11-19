//! Original-Alphabete der BERNINA artista V4 lesen (ESB-Dateien, Ordner `wletter/`).
//!
//! Aufbau und Belege: `Analyse_Teil3/09_esb_alphabete.md`; Python-Vorlage `Arbeitsdaten/werkzeuge/esb_lesen.py`.
//! - OLE-Datei: Stream `\x06HEADER` (Name, Buchstabenabstand), je Zeichen ein Stream (Name = Zeichencode hex).
//! - Zeichen-Streams: Substitution mit Format-Schlüssel 0xD2 (IMPORT.DLL 0x10114c00/0x10114b30) + zlib.
//! - Inhalt: 0x24-Byte-Kopf mit Prüfsumme, Datensätze wie im ART-Objektteil (IMPORT.DLL 0x100d9bf0):
//!   0x2e Grundlinie, 0x3a Zeichen, Objekte (0x06 Satinsäule mit Winkellinien, 0x01 Umriss, Stichart …).

use flate2::read::ZlibDecoder;
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use stick_kern::schrift::{Alphabet, EINHEIT, Knoten, Stichart, Umriss, Winkellinie, Zeichen, ZeichenObjekt};

/// Format-Schlüssel der ESB-Alphabete (Format-ID 0xD2, Analyse/code/05)
const FORMAT_SCHLUESSEL_ESB: u32 = 0x0024_70e8;

// ---------------------------------------------------------------- Entschlüsselung (Substitution)

fn schluessel_ableiten(format: u32, extra: u32, modus: u32) -> u32 {
    // IMPORT.DLL 0x10114c00
    let s = format.wrapping_add(extra);
    let sb = s.to_le_bytes();
    let h = (sb.iter().map(|&b| b as u32).sum::<u32>().wrapping_mul(13) & 0xff) as u8;
    let mb = modus.to_le_bytes();
    let mut out = [0u8; 4];
    for k in 0..4 {
        out[3 - k] = mb[k].rotate_left(1) ^ sb[3 - k].wrapping_add((17 * k) as u8).wrapping_add(h);
    }
    u32::from_le_bytes(out)
}

fn permutation(i: u32, key: u32) -> u8 {
    // IMPORT.DLL 0x10114b30
    let x = ((key >> 10) & 0xff) ^ (i & 0xff) ^ ((key >> 2) & 0xff);
    let x = x << (8 - (key & 3));
    ((x | (x >> 8)) & 0xff) as u8
}

fn tabelle(key: u32, umgekehrt: bool) -> [u8; 256] {
    let mut t = [0u8; 256];
    for i in 0..256u32 {
        t[permutation(i, key) as usize] = i as u8;
    }
    if !umgekehrt {
        return t;
    }
    let mut inv = [0u8; 256];
    for (j, &v) in t.iter().enumerate() {
        inv[v as usize] = j as u8;
    }
    inv
}

/// Zeichen-Stream entschlüsseln: Substitution, dann u32 Länge + zlib.
pub fn stream_entschluesseln(roh: &[u8]) -> Option<Vec<u8>> {
    let key = schluessel_ableiten(FORMAT_SCHLUESSEL_ESB, 0, 0);
    for umgekehrt in [false, true] {
        let t = tabelle(key, umgekehrt);
        let d: Vec<u8> = roh.iter().map(|&b| t[b as usize]).collect();
        if d.len() < 6 {
            continue;
        }
        let laenge = u32::from_le_bytes([d[0], d[1], d[2], d[3]]) as usize;
        let mut out = Vec::with_capacity(laenge);
        if ZlibDecoder::new(&d[4..]).read_to_end(&mut out).is_ok() && out.len() == laenge {
            return Some(out);
        }
    }
    None
}

// ---------------------------------------------------------------- gepackte Zahlen (WESSYS)

fn packed(d: &[u8], i: &mut usize) -> Option<u64> {
    let mut b = *d.get(*i)? as u64;
    *i += 1;
    let mut v = b;
    let mut shift = 7;
    while b & 0x80 != 0 {
        b = *d.get(*i)? as u64;
        *i += 1;
        v = (v ^ (1 << shift)) | (b << shift);
        shift += 7;
    }
    Some(v)
}

fn packed_wort(d: &[u8], i: &mut usize) -> Option<u64> {
    let mut v = *d.get(*i)? as u64 | ((*d.get(*i + 1)? as u64) << 8);
    *i += 2;
    if v & 0x8000 != 0 {
        let mut shift = 15;
        loop {
            let b = *d.get(*i)? as u64;
            *i += 1;
            v = (v ^ (1 << shift)) | (b << shift);
            shift += 7;
            if b & 0x80 == 0 {
                break;
            }
        }
    }
    Some(v)
}

fn packed_wort_s(d: &[u8], i: &mut usize) -> Option<i64> {
    let v = packed_wort(d, i)? as i64;
    Some(if v & 1 != 0 { !(v >> 1) } else { v >> 1 })
}

// ---------------------------------------------------------------- Datensätze

#[derive(Clone, Debug)]
struct Satz {
    klasse: u32,
    verweis: bool,
    daten: Vec<u8>,
    /// bei Verweisen: Indizes der Zielkette (Ziel samt Basisklassen bis 0x0d)
    ziel: Vec<usize>,
}

fn saetze_lesen(d: &[u8], start: usize) -> Vec<Satz> {
    let mut folge: Vec<Satz> = Vec::new();
    let mut geteilt: Vec<usize> = Vec::new();
    let mut i = start;
    while i < d.len() {
        let Some(u) = packed(d, &mut i) else { break };
        let (klasse, verweis) = ((u >> 1) as u32, u & 1 == 1);
        if klasse == 0 {
            folge.push(Satz {
                klasse: 0,
                verweis: false,
                daten: vec![],
                ziel: vec![],
            });
            continue;
        }
        let Some(v) = packed(d, &mut i) else { break };
        if verweis {
            // Zielkette: der geteilte Datensatz samt allem, was zu ihm gehört – bis vor die Form (0x56/0x01) oder das
            // nächste Objekt. Listenknoten (0x0f/0x0e, z. B. Füllung 0x2748) tragen ihre Einträge (Satin, Laufweg)
            // erst hinter der Kennung 0x0d; die zweite Säule eines Buchstabens verweist auf diese Liste.
            let mut ziel = Vec::new();
            if v > 0 && (v as usize) <= geteilt.len() {
                let mut n = geteilt[v as usize - 1];
                ziel.push(n);
                while n + 1 < folge.len() {
                    let k = folge[n + 1].klasse;
                    if k == 0x56 || k == 0x01 || (!folge[n + 1].verweis && OBJEKT_KLASSEN.contains(&k)) {
                        break;
                    }
                    n += 1;
                    ziel.push(n);
                }
            }
            folge.push(Satz {
                klasse,
                verweis: true,
                daten: vec![],
                ziel,
            });
            continue;
        }
        let (laenge, a) = ((v >> 2) as usize, v & 1 == 1);
        if laenge > d.len() - i {
            break;
        }
        if a {
            geteilt.push(folge.len());
        }
        folge.push(Satz {
            klasse,
            verweis: false,
            daten: d[i..i + laenge].to_vec(),
            ziel: vec![],
        });
        i += laenge;
    }
    folge
}

fn aufloesen(folge: &[Satz], idx: usize, tiefe: usize, out: &mut Vec<usize>) {
    let s = &folge[idx];
    if s.verweis {
        if tiefe < 8 {
            for &z in &s.ziel {
                aufloesen(folge, z, tiefe + 1, out);
            }
        }
    } else {
        out.push(idx);
    }
}

fn knoten_lesen(d: &[u8], i: &mut usize) -> Option<Vec<Knoten>> {
    let n = packed(d, i)?;
    let (mut px, mut py) = (0i64, 0i64);
    let mut knoten = Vec::new();
    for _ in 0..n {
        let c = *d.get(*i)?;
        *i += 1;
        let mut lies = |px: &mut i64, py: &mut i64| -> Option<(f64, f64)> {
            *px += packed_wort_s(d, i)?;
            *py += packed_wort_s(d, i)?;
            Some((*px as f64, *py as f64))
        };
        let ein = if c & 1 != 0 {
            Some(lies(&mut px, &mut py)?)
        } else {
            None
        };
        let punkt = lies(&mut px, &mut py)?;
        let aus = if c & 2 != 0 {
            Some(lies(&mut px, &mut py)?)
        } else {
            None
        };
        knoten.push(Knoten { ein, punkt, aus });
    }
    Some(knoten)
}

fn winkellinien(d: &[u8]) -> Vec<Winkellinie> {
    let mut out = Vec::new();
    if d.len() < 2 {
        return out;
    }
    let n = u16::from_le_bytes([d[0], d[1]]);
    let mut i = 2;
    for _ in 0..n {
        let (Some(a), Some(x)) = (packed(d, &mut i), d.get(i..i + 4)) else {
            break;
        };
        let x = f32::from_le_bytes([x[0], x[1], x[2], x[3]]) as f64;
        i += 4;
        let (Some(c), Some(y)) = (packed(d, &mut i), d.get(i..i + 4)) else {
            break;
        };
        let y = f32::from_le_bytes([y[0], y[1], y[2], y[3]]) as f64;
        i += 4;
        out.push(Winkellinie {
            a: a as u32,
            x,
            c: c as u32,
            y,
        });
    }
    out
}

fn u32_bei(d: &[u8], o: usize) -> Option<u32> {
    d.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// Klassen, die ein neues Objekt beginnen (IMPORT.DLL, Analyse_Teil3/03)
const OBJEKT_KLASSEN: [u32; 19] = [
    0x05, 0x06, 0x07, 0x08, 0x0b, 0x0c, 0x2f, 0x30, 0x36, 0x37, 0x38, 0x49, 0x71, 0x72, 0x73, 0x7a, 0x7b, 0x93, 0x95,
];

/// Gefundene Sticharten eines Objekts: Satin (mit Abstand), Tatami, Lauf (mit Länge)
/// Satin (Abstand), Tatami, Lauf (Länge), Auto Spacing (Effekt 0x21 vorhanden)
type ArtFunde = (Option<Option<f64>>, bool, Option<Option<f64>>, bool);

/// Einen entschlüsselten Zeichen-Stream lesen (IMPORT.DLL 0x100c6230).
pub fn zeichen_lesen(d: &[u8]) -> Result<Zeichen, String> {
    let kopf: Vec<u32> = (0..9).filter_map(|k| u32_bei(d, 4 * k)).collect();
    if kopf.len() < 9
        || kopf[0] != 0x0d0a_1a0d
        || kopf[1] != 0xeaae_8356
        || kopf[..8].iter().fold(0u32, |s, &v| s.wrapping_add(v)) != kopf[8]
    {
        return Err("Zeichen-Kopf ungültig".into());
    }
    let folge = saetze_lesen(d, 0x24);
    let mut grundlinie = None;
    let mut zeichen = None;
    let mut objekte: Vec<ZeichenObjekt> = Vec::new();
    // je Objekt: (Satin gesehen, Tatami, Lauf) zum Bestimmen der Stichart
    let mut arten: Vec<ArtFunde> = Vec::new();
    let mut vorige = 0u32; // Klasse des zuletzt verarbeiteten Datensatzes (Basisklasse folgt der abgeleiteten)
    for (idx, s) in folge.iter().enumerate() {
        if !s.verweis && s.klasse == 0x2e && grundlinie.is_none() {
            let mut i = 0;
            let w: Vec<f64> = (0..4)
                .filter_map(|_| packed_wort_s(&s.daten, &mut i))
                .map(|v| v as f64)
                .collect();
            if w.len() == 4 {
                grundlinie = Some([w[0], w[1], w[2], w[3]]);
            }
            continue;
        }
        if !s.verweis && s.klasse == 0x3a && zeichen.is_none() {
            if let Some(n) = u32_bei(&s.daten, 0) {
                let utf: Vec<u16> = (0..n as usize)
                    .filter_map(|k| {
                        s.daten
                            .get(4 + 2 * k..6 + 2 * k)
                            .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    })
                    .collect();
                zeichen = String::from_utf16_lossy(&utf).chars().next();
            }
            continue;
        }
        if !s.verweis && OBJEKT_KLASSEN.contains(&s.klasse) {
            objekte.push(ZeichenObjekt {
                klasse: s.klasse,
                umrisse: vec![],
                winkel: if s.klasse == 0x06 {
                    winkellinien(&s.daten)
                } else {
                    vec![]
                },
                stichart: Stichart::Unbekannt,
                unterlagen: vec![],
                laufweg: None,
            });
            arten.push((None, false, None, false));
            continue;
        }
        let (Some(o), Some(art)) = (objekte.last_mut(), arten.last_mut()) else {
            continue;
        };
        let mut blaetter = Vec::new();
        aufloesen(&folge, idx, 0, &mut blaetter);
        for &b in &blaetter {
            let sb = &folge[b];
            match sb.klasse {
                0x01 if !sb.daten.is_empty() => {
                    let mut i = 0;
                    if let Some(f) = packed(&sb.daten, &mut i)
                        && let Some(kn) = knoten_lesen(&sb.daten, &mut i)
                    {
                        o.umrisse.push(Umriss {
                            geschlossen: f & 1 == 1,
                            knoten: kn,
                        });
                    }
                }
                0x16 => {
                    if art.0.is_none() {
                        art.0 = Some(None);
                    }
                }
                // Satin 0x16 → Basisklasse 0x15 Abstand (direkt danach)
                0x15 if vorige == 0x16 => {
                    if let Some(None) = art.0 {
                        art.0 = Some(u32_bei(&sb.daten, 0).map(|v| v as f64 / EINHEIT));
                    }
                }
                0x1b => art.1 = true,
                // AutoSpacingEffect am Objekt: vorhanden = aktiv (Analyse_Teil3/05 §1)
                0x21 => art.3 = true,
                0x18 | 0x19 => {
                    if art.2.is_none() {
                        art.2 = Some(u32_bei(&sb.daten, 0).map(|v| v as f64 / EINHEIT));
                    }
                }
                0x1d..=0x20 => {
                    let n = match sb.klasse {
                        0x1d => "zickzack",
                        0x1e => "doppelzickzack",
                        0x1f => "mittellauf",
                        _ => "edge_walk",
                    };
                    if !o.unterlagen.iter().any(|u| u == n) {
                        o.unterlagen.push(n.into());
                    }
                }
                _ => {}
            }
            vorige = sb.klasse;
        }
    }
    for (o, art) in objekte.iter_mut().zip(&arten) {
        o.stichart = match art {
            (Some(abstand), _, _, auto) => Stichart::Satin {
                abstand: *abstand,
                auto_spacing: *auto,
            },
            (None, true, _, _) => Stichart::Tatami,
            (None, false, Some(laenge), _) => Stichart::Lauf { laenge: *laenge },
            _ => Stichart::Unbekannt,
        };
        // Satin/Tatami: der Lauf 0x271c in der Füllung ist der Laufweg
        if !matches!(o.stichart, Stichart::Lauf { .. }) {
            o.laufweg = art.2.flatten();
        }
    }
    // Gruppen (0x08) und Objekte ohne Form tragen keine Stiche
    objekte.retain(|o| !o.umrisse.is_empty());
    Ok(Zeichen {
        zeichen: zeichen.ok_or("Zeichen fehlt (0x3a)")?,
        grundlinie: grundlinie.ok_or("Grundlinie fehlt (0x2e)")?,
        objekte,
    })
}

/// Kopf `\x06HEADER` (IMPORT.DLL 0x100040f0): Name und Buchstabenabstand (Anteil der Höhe).
fn kopf_lesen(h: &[u8]) -> Result<(String, f64), String> {
    if u32_bei(h, 0) != Some(0x0d0a_1aed) || h.len() < 10 {
        return Err("Alphabet-Kopf ungültig".into());
    }
    let mut i = 10 + if h[9] >= 2 { 4 } else { 0 };
    let name_roh: Vec<u16> = (0..0x51)
        .filter_map(|k| {
            h.get(i + 2 * k..i + 2 * k + 2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
        })
        .take_while(|&c| c != 0)
        .collect();
    i += 0xa2;
    let f2 = h
        .get(i + 4..i + 8)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64)
        .unwrap_or(0.0);
    Ok((String::from_utf16_lossy(&name_roh), f2))
}

/// Ein Alphabet aus einer ESB-Datei laden. Defekte Zeichen werden übersprungen.
pub fn laden(pfad: &Path) -> Result<Alphabet, String> {
    let mut datei = cfb::open(pfad).map_err(|e| format!("{}: {e}", pfad.display()))?;
    let eintraege: Vec<String> = datei
        .read_root_storage()
        .filter(|e| e.is_stream())
        .map(|e| e.name().to_string())
        .collect();
    let mut name = String::new();
    let mut abstand_anteil = 0.0;
    let mut zeichen = BTreeMap::new();
    for n in eintraege {
        let mut roh = Vec::new();
        datei
            .open_stream(format!("/{n}"))
            .and_then(|mut s| s.read_to_end(&mut roh))
            .map_err(|e| format!("{}: Stream {n}: {e}", pfad.display()))?;
        if n.ends_with("HEADER") {
            (name, abstand_anteil) = kopf_lesen(&roh)?;
            continue;
        }
        if n.len() != 4 || u32::from_str_radix(&n, 16).is_err() {
            continue; // AUX_INFO u. ä.
        }
        if let Some(d) = stream_entschluesseln(&roh)
            && let Ok(z) = zeichen_lesen(&d)
        {
            zeichen.insert(z.zeichen, z);
        }
    }
    if name.is_empty() {
        name = pfad
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
    }
    Ok(Alphabet {
        name,
        abstand_anteil,
        zeichen,
    })
}

/// Alle ESB-Dateien eines Ordners: (Name aus dem Kopf, Pfad), nach Name sortiert.
pub fn verzeichnis(ordner: &Path) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(ordner) else { return out };
    for e in rd.flatten() {
        let p = e.path();
        if !p.extension().is_some_and(|x| x.eq_ignore_ascii_case("esb")) {
            continue;
        }
        let name = cfb::open(&p)
            .ok()
            .and_then(|mut f| {
                let mut h = Vec::new();
                f.open_stream("/\u{6}HEADER").ok()?.read_to_end(&mut h).ok()?;
                kopf_lesen(&h).ok().map(|k| k.0)
            })
            .unwrap_or_else(|| p.file_stem().unwrap_or_default().to_string_lossy().into_owned());
        out.push((name, p));
    }
    out.sort_by_key(|a| a.0.to_lowercase());
    out
}

/// Die Alphabete, die die Texte eines Designs brauchen, aus einem Ordner laden (Name wie im Kopf oder Dateiname,
/// ohne Groß-/Kleinschreibung).
pub fn design_alphabete_laden(design: &mut stick_kern::Design, ordner: &Path) -> Result<(), String> {
    let namen = design.alphabet_namen();
    if namen.is_empty() {
        return Ok(());
    }
    let liste = verzeichnis(ordner);
    for n in namen {
        if design.alphabete.contains_key(&n) {
            continue;
        }
        let pfad = liste
            .iter()
            .find(|(name, p)| {
                name.eq_ignore_ascii_case(&n)
                    || p.file_stem()
                        .is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case(&n))
            })
            .map(|(_, p)| p.clone())
            .ok_or_else(|| format!("Alphabet „{n}“ nicht im Ordner {} gefunden", ordner.display()))?;
        design.alphabete.insert(n, laden(&pfad)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schluessel_wie_python() {
        // art_decrypt.derive_key(0x002470e8) – Wert aus dem Python-Werkzeug
        let k = schluessel_ableiten(FORMAT_SCHLUESSEL_ESB, 0, 0);
        let t = tabelle(k, false);
        let mut gesehen = [false; 256];
        for &v in &t {
            gesehen[v as usize] = true;
        }
        assert!(gesehen.iter().all(|&g| g), "Substitution muss eine Permutation sein");
    }

    #[test]
    fn gepackt() {
        let d = [0x37u8, 0xa7, 0x02];
        let mut i = 0;
        assert_eq!(packed(&d, &mut i), Some(0x37));
        assert_eq!(packed(&d, &mut i), Some(0x27 | (2 << 7)));
        assert_eq!(i, 3);
    }

    /// Mit der Original-CD (entpackt unter F:\Entwicklung\artista_ziel): Drifter vollständig lesen.
    #[test]
    fn drifter_von_der_cd() {
        let p = Path::new("F:/Entwicklung/artista_ziel/Main/PROGRAM/wletter/DRIFTER.ESB");
        if !p.exists() {
            return; // CD nicht entpackt (z. B. auf dem Mac)
        }
        let a = laden(p).unwrap();
        assert_eq!(a.name, "Drifter");
        assert_eq!(a.zeichen.len(), 141);
        assert!((a.abstand_anteil - 0.1).abs() < 1e-6);
        let z = &a.zeichen[&'A'];
        assert_eq!(z.grundlinie, [-37788.0, -26925.0, -30975.0, -26925.0]);
        assert_eq!(z.objekte.len(), 2);
        assert_eq!(z.objekte[0].umrisse[0].knoten.len(), 14);
        assert_eq!(z.objekte[0].winkel.len(), 7);
        let satin = Stichart::Satin {
            abstand: Some(0.4),
            auto_spacing: true,
        };
        assert_eq!(z.objekte[0].stichart, satin);
        assert_eq!(z.objekte[0].laufweg, Some(3.0));
        assert_eq!(z.objekte[1].umrisse[0].knoten.len(), 22);
        assert_eq!(z.objekte[1].stichart, satin);
        // alle Zeichen haben mindestens eine Säule mit Stichart
        for z in a.zeichen.values() {
            assert!(
                z.objekte.iter().all(|o| o.stichart != Stichart::Unbekannt),
                "{}",
                z.zeichen
            );
        }
    }
}
