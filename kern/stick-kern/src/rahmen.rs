//! Stickrahmen der artista (Stickfeld in mm, Ursprung = Rahmenmitte).
//!
//! Quelle: RES/hooplibr.dsc -> `Analyse/daten/hoops_table.md`, Form des Ovals `Analyse/code/08_rahmen_dsc.md` (hoch).
//! Welche Rahmen an der artista 185 tatsächlich passen, ist noch nicht am Gerät geprüft.

use crate::geometrie::{Linie, Punkt, drehen, im_polygon, p, rechteck};
use std::f64::consts::PI;

/// Kreisbogen von p1 über p2 nach p3 (ohne p1, mit p3).
fn bogen_durch_drei(p1: Punkt, p2: Punkt, p3: Punkt, schritte: usize) -> Linie {
    let (ax, ay, bx, by, cx, cy) = (p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
    let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
    let ux = ((ax * ax + ay * ay) * (by - cy) + (bx * bx + by * by) * (cy - ay) + (cx * cx + cy * cy) * (ay - by)) / d;
    let uy = ((ax * ax + ay * ay) * (cx - bx) + (bx * bx + by * by) * (ax - cx) + (cx * cx + cy * cy) * (bx - ax)) / d;
    let r = (ax - ux).hypot(ay - uy);
    let w1 = (ay - uy).atan2(ax - ux);
    let w2 = (by - uy).atan2(bx - ux);
    let w3 = (cy - uy).atan2(cx - ux);
    let norm = |w: f64| w.rem_euclid(2.0 * PI);
    let ccw = norm(w3 - w1);
    let spanne = if norm(w2 - w1) <= ccw { ccw } else { ccw - 2.0 * PI };
    (1..=schritte)
        .map(|i| {
            let w = w1 + spanne * i as f64 / schritte as f64;
            p(ux + r * w.cos(), uy + r * w.sin())
        })
        .collect()
}

fn oval_145x255() -> Linie {
    let mut f = vec![p(72.5, -60.0), p(72.5, 60.0), p(65.0, 90.0)];
    f.extend(bogen_durch_drei(p(65.0, 90.0), p(0.0, 127.5), p(-65.0, 90.0), 24));
    f.extend([p(-72.5, 60.0), p(-72.5, -60.0), p(-65.0, -90.0)]);
    f.extend(bogen_durch_drei(p(-65.0, -90.0), p(0.0, -127.5), p(65.0, -90.0), 24));
    f
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rahmen {
    pub name: String,
    /// Stickfeld x (mm)
    pub breite: f64,
    /// Stickfeld y (mm)
    pub hoehe: f64,
    /// Umriss des Stickfelds
    pub form: Linie,
    pub hinweis: String,
}

impl Rahmen {
    fn neu(name: &str, breite: f64, hoehe: f64, form: Option<Linie>, hinweis: &str) -> Rahmen {
        Rahmen {
            name: name.into(),
            breite,
            hoehe,
            form: form.unwrap_or_else(|| rechteck(p(0.0, 0.0), breite, hoehe)),
            hinweis: hinweis.into(),
        }
    }

    pub fn enthaelt(&self, q: Punkt) -> bool {
        let tol = 1e-6;
        if q.x.abs() > self.breite / 2.0 + tol || q.y.abs() > self.hoehe / 2.0 + tol {
            return false;
        }
        let ringe = [self.form.clone()];
        im_polygon(q, &ringe)
            || [(tol, 0.0), (-tol, 0.0), (0.0, tol), (0.0, -tol)]
                .iter()
                .any(|&(dx, dy)| im_polygon(p(q.x + dx, q.y + dy), &ringe))
    }

    pub fn gedreht(&self) -> Rahmen {
        Rahmen {
            name: format!("{} Rotated", self.name),
            breite: self.hoehe,
            hoehe: self.breite,
            form: self.form.iter().map(|&q| drehen(q, 90.0, p(0.0, 0.0))).collect(),
            hinweis: self.hinweis.clone(),
        }
    }
}

/// Alle bekannten Rahmen, Grundformen und gedrehte Varianten.
pub fn alle_rahmen() -> Vec<Rahmen> {
    let hinweis_rund = "eine Seite gerundet – hier als Rechteck genähert";
    let grund = [
        Rahmen::neu(
            "artista 255 x 145 Oval",
            145.0,
            255.0,
            Some(oval_145x255()),
            "großer Ovalrahmen",
        ),
        Rahmen::neu("artista 200 x 155", 155.0, 200.0, None, ""),
        Rahmen::neu("artista 130 x 100", 100.0, 130.0, None, "mittlerer Rahmen"),
        Rahmen::neu("artista 50 x 72 Free Arm", 72.0, 50.0, None, hinweis_rund),
        Rahmen::neu("artista 50 x 72 Normal", 72.0, 50.0, None, hinweis_rund),
        Rahmen::neu("artista 40 x 60", 60.0, 40.0, None, ""),
        Rahmen::neu(
            "artista 400 x 150 MEGA",
            150.0,
            400.0,
            None,
            "Mega-Rahmen, mehrere Positionen",
        ),
    ];
    let mut out = Vec::new();
    for r in grund {
        let g = (r.breite != r.hoehe).then(|| r.gedreht());
        out.push(r);
        out.extend(g);
    }
    out
}

pub fn rahmen(name: &str) -> Result<Rahmen, String> {
    let alle = alle_rahmen();
    match alle.iter().find(|r| r.name == name) {
        Some(r) => Ok(r.clone()),
        None => {
            let namen: Vec<_> = alle.iter().map(|r| r.name.as_str()).collect();
            Err(format!("Unbekannter Rahmen '{name}'. Bekannt: {}", namen.join(", ")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oval() {
        let r = rahmen("artista 255 x 145 Oval").unwrap();
        assert!(r.enthaelt(p(0.0, 127.0)) && r.enthaelt(p(72.0, 0.0)));
        assert!(!r.enthaelt(p(72.0, 120.0)) && !r.enthaelt(p(0.0, 128.0)));
        let g = rahmen("artista 255 x 145 Oval Rotated").unwrap();
        assert!(g.enthaelt(p(127.0, 0.0)) && !g.enthaelt(p(0.0, 127.0)));
        assert!(rahmen("artista 130 x 100 Rotated").is_ok());
        assert!(rahmen("Brother").is_err());
    }
}
