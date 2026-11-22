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
    /// eigener Rahmen (nicht in der Liste der artista 185)
    pub eigen: bool,
}

/// Kleinste und größte Seite eines eigenen Rahmens in mm (eigene Festlegung: auch größer als jeder Rahmen der 185,
/// damit sich große Designs gestalten lassen – gesendet wird nur, was in einen Rahmen der 185 passt)
pub const EIGEN_MIN: f64 = 10.0;
pub const EIGEN_MAX: f64 = 1000.0;

impl Rahmen {
    fn neu(name: &str, breite: f64, hoehe: f64, form: Option<Linie>, hinweis: &str) -> Rahmen {
        Rahmen {
            name: name.into(),
            breite,
            hoehe,
            form: form.unwrap_or_else(|| rechteck(p(0.0, 0.0), breite, hoehe)),
            hinweis: hinweis.into(),
            eigen: false,
        }
    }

    /// **Eigener Rahmen** (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“): Rechteck Breite × Höhe, oder Oval mit
    /// geradem Mittelteil `gerade` mm hoch („Rectangle Height“) und oben/unten Halbellipsen bis zur Gesamthöhe
    /// („Total Height“); `gerade` = 0 ergibt eine Ellipse.
    pub fn eigener(name: &str, oval: bool, breite: f64, hoehe: f64, gerade: f64) -> Result<Rahmen, String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("eigener_rahmen: Name fehlt".into());
        }
        for (feld, v) in [("breite", breite), ("hoehe", hoehe)] {
            if !(EIGEN_MIN..=EIGEN_MAX).contains(&v) {
                return Err(format!("eigener_rahmen: {feld} {v} mm außerhalb {EIGEN_MIN} … {EIGEN_MAX}"));
            }
        }
        if oval && !(0.0..hoehe).contains(&gerade) {
            return Err(format!("eigener_rahmen: gerade {gerade} mm muss kleiner als die Höhe sein"));
        }
        let form = if oval {
            let (a, g) = (breite / 2.0, gerade / 2.0);
            let c = (hoehe - gerade) / 2.0;
            let n = 48;
            let bogen = |mitte: f64, von: f64| -> Vec<Punkt> {
                (0..=n)
                    .map(|i| {
                        let w = von + PI * i as f64 / n as f64;
                        p(a * w.cos(), mitte + c * w.sin())
                    })
                    .collect()
            };
            let mut f = bogen(g, 0.0);
            f.extend(bogen(-g, PI));
            f.dedup_by(|x, y| (x.x - y.x).abs() < 1e-9 && (x.y - y.y).abs() < 1e-9);
            Some(f)
        } else {
            None
        };
        let mut r = Rahmen::neu(name, breite, hoehe, form, "eigener Rahmen – die artista 185 kennt ihn nicht");
        r.eigen = true;
        Ok(r)
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
            eigen: self.eigen,
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

    #[test]
    fn eigene() {
        let r = Rahmen::eigener("Kissen", false, 300.0, 200.0, 0.0).unwrap();
        assert!(r.eigen && r.enthaelt(p(149.0, 99.0)) && !r.enthaelt(p(151.0, 0.0)));
        let o = Rahmen::eigener("Oval", true, 100.0, 160.0, 60.0).unwrap();
        // gerader Teil ±30 mm bei x = ±50, Scheitel bei y = ±80
        assert!(o.enthaelt(p(49.9, 29.0)) && o.enthaelt(p(0.0, 79.9)) && !o.enthaelt(p(49.0, 75.0)));
        let e = Rahmen::eigener("Ellipse", true, 100.0, 60.0, 0.0).unwrap();
        assert!(e.enthaelt(p(49.9, 0.0)) && !e.enthaelt(p(40.0, 25.0)));
        assert!(Rahmen::eigener("", false, 100.0, 100.0, 0.0).is_err());
        assert!(Rahmen::eigener("x", false, 5.0, 100.0, 0.0).is_err());
        assert!(Rahmen::eigener("x", true, 100.0, 100.0, 100.0).is_err());
    }
}
