//! EXP-Format (Melco „Expanded“), wie BERNINA es liest – `Analyse/code/07_exp_format.md`.
//!
//! Kein Dateikopf; 2-Byte-Datensätze (dx, dy) in 0,1 mm, vorzeichenbehaftet, +dy nach oben.
//! Steuerung: `80 01 dx dy` = Farbwechsel/Stopp (danach Bewegung), `80 04 dx dy` = Sprung,
//! `80 82 xh xl yh yl` = langer Sprung (16 Bit). Lesen: hoch (aus dem Code); Schreiben: mittel.
//! Gerundet wird wie im Prototyp „halb auf gerade“ (Pythons `round`).

use stick_kern::einheiten::EXP_MAX_SCHRITT;
use stick_kern::{Art, Stich, StichFolge};

/// Standardfarben beim Lesen (EXP enthält keine Farben).
pub const LESE_FARBEN: [&str; 8] = [
    "#1f3a93", "#c0392b", "#27ae60", "#f39c12", "#8e44ad", "#16a085", "#2c3e50", "#d35400",
];

fn runden(v: f64) -> i64 {
    v.round_ties_even() as i64
}

fn byte(v: i64) -> u8 {
    (v & 0xFF) as u8
}

fn teilschritte(out: &mut Vec<u8>, dx: i64, dy: i64, n: i64, bis: i64) -> (i64, i64) {
    let (mut px, mut py) = (0, 0);
    for j in 1..=bis {
        let qx = runden(dx as f64 * j as f64 / n as f64);
        let qy = runden(dy as f64 * j as f64 / n as f64);
        out.extend([0x80, 0x04, byte(qx - px), byte(qy - py)]);
        (px, py) = (qx, qy);
    }
    (px, py)
}

/// Stichfolge (mm, Ursprung = Rahmenmitte) als EXP. Start ist die Rahmenmitte.
pub fn exp_bytes(folge: &StichFolge) -> Vec<u8> {
    let mut out = Vec::new();
    let (mut cx, mut cy) = (0i64, 0i64);
    let mut vorher = Art::Sprung; // erster Einstich gilt als „nach Sprung“
    for s in &folge.stiche {
        if s.art == Art::Farbwechsel {
            out.extend([0x80, 0x01, 0, 0]);
            vorher = Art::Farbwechsel;
            continue;
        }
        let (tx, ty) = (runden(s.x * 10.0), runden(s.y * 10.0));
        let (mut dx, mut dy) = (tx - cx, ty - cy);
        let gross = dx.abs().max(dy.abs());
        if s.art == Art::Sprung {
            if dx == 0 && dy == 0 {
                continue;
            }
            let n = ((gross as f64 / EXP_MAX_SCHRITT as f64).ceil() as i64).max(1);
            teilschritte(&mut out, dx, dy, n, n);
            (cx, cy) = (tx, ty);
            vorher = Art::Sprung;
            continue;
        }
        if dx == 0 && dy == 0 && vorher == Art::Stich {
            continue; // doppelter Einstich
        }
        if gross > EXP_MAX_SCHRITT {
            // Sicherheitsnetz (sollte nach dem Auto Jump nicht vorkommen): Sprünge, dann Einstich
            let n = (gross as f64 / EXP_MAX_SCHRITT as f64).ceil() as i64;
            let (px, py) = teilschritte(&mut out, dx, dy, n, n - 1);
            (dx, dy) = (dx - px, dy - py);
        }
        out.extend([byte(dx), byte(dy)]);
        (cx, cy) = (tx, ty);
        vorher = Art::Stich;
    }
    out
}

fn s8(v: u8) -> i64 {
    v as i8 as i64
}

/// EXP lesen wie der BERNINA-Leser (BEAGLink.dll ab 0x10007c05).
pub fn exp_lesen(daten: &[u8]) -> StichFolge {
    let mut folge = StichFolge {
        stiche: Vec::new(),
        farben: vec![LESE_FARBEN[0].into()],
    };
    let (mut x, mut y) = (0i64, 0i64);
    let mut i = 0;
    let n = daten.len();
    while i + 1 < n {
        let (mut b0, mut b1) = (daten[i], daten[i + 1]);
        i += 2;
        let mut art = Art::Stich;
        if b0 == 0x80 {
            if (1..=15).contains(&b1) {
                if b1 & 1 == 1 {
                    folge.stiche.push(Stich {
                        x: x as f64 / 10.0,
                        y: y as f64 / 10.0,
                        art: Art::Farbwechsel,
                    });
                    let f = LESE_FARBEN[folge.farben.len() % LESE_FARBEN.len()];
                    folge.farben.push(f.into());
                }
                art = Art::Sprung;
                if i + 1 >= n {
                    break;
                }
                (b0, b1) = (daten[i], daten[i + 1]);
                i += 2;
            } else if b1 == 0x82 {
                if i + 3 >= n {
                    break;
                }
                let vx = i64::from(u16::from_be_bytes([daten[i], daten[i + 1]]) as i16);
                let vy = i64::from(u16::from_be_bytes([daten[i + 2], daten[i + 3]]) as i16);
                i += 4;
                x += vx;
                y += vy;
                folge.stiche.push(Stich {
                    x: x as f64 / 10.0,
                    y: y as f64 / 10.0,
                    art: Art::Sprung,
                });
                continue;
            } else {
                continue; // unbekannt: überspringen
            }
        }
        x += s8(b0);
        y += s8(b1);
        if art == Art::Sprung && s8(b0) == 0 && s8(b1) == 0 {
            continue; // Farbwechsel ohne Bewegung
        }
        folge.stiche.push(Stich {
            x: x as f64 / 10.0,
            y: y as f64 / 10.0,
            art,
        });
    }
    folge
}

#[cfg(test)]
mod tests {
    use super::*;
    use stick_kern::geometrie::p;

    #[test]
    fn einfach() {
        let mut f = StichFolge {
            farben: vec!["#000".into()],
            ..Default::default()
        };
        f.sprung(p(1.0, 0.0));
        f.stich(p(1.0, 0.0));
        f.stich(p(2.0, -0.5));
        assert_eq!(exp_bytes(&f), [0x80, 0x04, 10, 0, 0, 0, 10, 0xFB]);
    }

    #[test]
    fn farbwechsel_und_langer_sprung() {
        let mut f = StichFolge {
            farben: vec!["#000".into()],
            ..Default::default()
        };
        f.stich(p(0.0, 0.0));
        f.farbwechsel("#f00");
        f.sprung(p(30.0, 0.0));
        f.stich(p(30.0, 0.0));
        let b = exp_bytes(&f);
        assert_eq!(&b[..2], &[0, 0]);
        assert_eq!(&b[2..6], &[0x80, 0x01, 0, 0]);
        let spruenge: Vec<&[u8]> = b[6..b.len() - 2].chunks(4).collect();
        assert_eq!(spruenge.len(), 3);
        assert!(spruenge.iter().all(|s| s[..2] == [0x80, 0x04]));
        assert_eq!(spruenge.iter().map(|s| s[2] as u32).sum::<u32>(), 300);
    }

    #[test]
    fn langer_sprung_16_bit_lesen() {
        let f = exp_lesen(&[0x80, 0x82, 0x01, 0x2C, 0xFF, 0x38, 0x00, 0x00]);
        assert_eq!(
            (f.stiche[0].x, f.stiche[0].y, f.stiche[0].art),
            (30.0, -20.0, Art::Sprung)
        );
    }
}
