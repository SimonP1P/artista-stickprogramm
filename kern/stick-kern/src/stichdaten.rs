//! **Stichdaten** (Objektart `stiche`): feste Einstiche aus einer fremden Stickdatei, wie „Grade D“ im Original
//! (Handbuch Kap. 28 S. 256–257: „The design opens as a collection of stitches with only general properties“).
//! Die Stiche werden nicht neu berechnet; Transformationen bewegen die Einstiche (Skalieren ändert die Dichte, nicht
//! die Stichzahl – das Original empfiehlt höchstens ±5 %). Kein zusätzliches Vernähen (die Datei hat ihr eigenes).
//!
//! Speicherung im Design (eigene Festlegung): Base64 einer Folge von Datensätzen (x, y) als i16 (Little Endian) in
//! 0,1 mm, +y oben; der Datensatz (−32768, 0) beginnt einen neuen Zug (Sprung/Fadenschnitt). 0,1 mm ist die Auflösung
//! aller gelesenen Formate.

use crate::geometrie::{Linie, p};

/// Höchstzahl der Einstiche je Objekt (eigene Festlegung; die 185 fasst ~145 KB ≈ 70 000 Stiche)
pub const MAX_STICHE: usize = 500_000;
/// Abweichung der Größe, ab der ein Hinweis kommt (Handbuch S. 256: „more than ±5 %“)
pub const SKALIEREN_HINWEIS: f64 = 0.05;

const ZUG: i16 = i16::MIN;

/// Züge (mm) → Base64
pub fn kodieren(zuege: &[Vec<(f64, f64)>]) -> String {
    let mut b = Vec::new();
    let wert = |v: f64| (v * 10.0).round().clamp(-32767.0, 32767.0) as i16;
    for (k, z) in zuege.iter().enumerate() {
        if k > 0 {
            b.extend(ZUG.to_le_bytes());
            b.extend(0i16.to_le_bytes());
        }
        for &(x, y) in z {
            b.extend(wert(x).to_le_bytes());
            b.extend(wert(y).to_le_bytes());
        }
    }
    base64_schreiben(&b)
}

/// Base64 → Züge (mm)
pub fn dekodieren(t: &str) -> Result<Vec<Linie>, String> {
    let b = crate::photosnap::base64_lesen(t)?;
    if b.len() % 4 != 0 {
        return Err("stiche: Länge ist kein Vielfaches von 4 Byte".into());
    }
    if b.len() / 4 > MAX_STICHE {
        return Err(format!("stiche: mehr als {MAX_STICHE} Einstiche"));
    }
    let mut zuege: Vec<Linie> = vec![Vec::new()];
    for c in b.chunks_exact(4) {
        let x = i16::from_le_bytes([c[0], c[1]]);
        let y = i16::from_le_bytes([c[2], c[3]]);
        if x == ZUG {
            zuege.push(Vec::new());
        } else {
            zuege.last_mut().unwrap().push(p(x as f64 / 10.0, y as f64 / 10.0));
        }
    }
    zuege.retain(|z| !z.is_empty());
    if zuege.is_empty() {
        return Err("stiche: keine Einstiche".into());
    }
    Ok(zuege)
}

/// Base64 (Standardalphabet mit „=“)
pub fn base64_schreiben(daten: &[u8]) -> String {
    const Z: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(daten.len().div_ceil(3) * 4);
    for b in daten.chunks(3) {
        let n = (u32::from(b[0]) << 16) | (u32::from(*b.get(1).unwrap_or(&0)) << 8) | u32::from(*b.get(2).unwrap_or(&0));
        for i in 0..4 {
            out.push(if i <= b.len() { Z[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' });
        }
    }
    out
}

/// Konvexe Hülle (für die Form in der Oberfläche: Auswahl, Griffe)
pub fn huelle(zuege: &[Linie]) -> Linie {
    let mut q: Vec<crate::geometrie::Punkt> = zuege.iter().flatten().copied().collect();
    q.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    q.dedup_by(|a, b| (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9);
    if q.len() < 3 {
        return q;
    }
    let kreuz = |o: crate::geometrie::Punkt, a: crate::geometrie::Punkt, b: crate::geometrie::Punkt| {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };
    let mut h: Vec<crate::geometrie::Punkt> = Vec::new();
    for pass in 0..2 {
        let start = h.len();
        let it: Box<dyn Iterator<Item = &crate::geometrie::Punkt>> =
            if pass == 0 { Box::new(q.iter()) } else { Box::new(q.iter().rev()) };
        for &c in it {
            while h.len() >= start + 2 && kreuz(h[h.len() - 2], h[h.len() - 1], c) <= 0.0 {
                h.pop();
            }
            h.push(c);
        }
        h.pop();
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hin_und_zurueck() {
        let z = vec![vec![(0.0, 0.0), (1.2, -3.4)], vec![(5.0, 5.0)]];
        let t = kodieren(&z);
        let d = dekodieren(&t).unwrap();
        assert_eq!(d, vec![vec![p(0.0, 0.0), p(1.2, -3.4)], vec![p(5.0, 5.0)]]);
        assert!(dekodieren("").is_err());
        assert!(dekodieren("AAA=").is_err());
        let h = huelle(&[vec![p(0.0, 0.0), p(4.0, 0.0), p(4.0, 3.0), p(0.0, 3.0), p(2.0, 1.0)]]);
        assert_eq!(h.len(), 4);
    }
}
