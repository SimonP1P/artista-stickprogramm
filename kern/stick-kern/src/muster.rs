//! Füllmuster („Program Split“, im Handbuch „Fancy Fill“) – Roadmap Phase 4 Punkt 6.
//!
//! Die 242 Muster der Original-Vorlage NORMAL.AMT (Linienzüge + Gitter A/B, `Analyse_Teil3/13_fuellmuster.md`,
//! gelesen mit `Arbeitsdaten/werkzeuge/muster_lesen.py`) liegen als `daten/fuellmuster.json` im Programm.
//! Gestickt wird wie Tatami (Reihen im Abstand der Füllung), aber die Einstiche jeder Reihe liegen dort, wo sie die
//! gekachelten Musterlinien kreuzt – so zeichnet sich das Muster in der Fläche ab (gemessen an den Samplern
//! „Sampler Fancy fills 001–150“: Reihen 0,20 mm, Einstiche auf den Musterlinien, kürzester Stich 0,40 mm).

use crate::geometrie::{Linie, Punkt, p};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Fuellmuster {
    pub name: String,
    /// Stichwinkel in Grad aus dem Namen („001 Diamond (90)“), sonst None
    pub winkel: Option<f64>,
    /// Gittervektoren in mm: das Muster wiederholt sich um i·A + j·B
    pub a: [f64; 2],
    pub b: [f64; 2],
    /// Linienzüge in mm, relativ zur linken unteren Ecke des Musters
    pub zuege: Vec<Vec<[f64; 2]>>,
}

static MUSTER: OnceLock<Vec<Fuellmuster>> = OnceLock::new();

/// Alle Muster der Original-Vorlage.
pub fn alle() -> &'static [Fuellmuster] {
    MUSTER.get_or_init(|| serde_json::from_str(include_str!("../daten/fuellmuster.json")).unwrap_or_default())
}

/// Muster nach Namen („001 Diamond (90)“; ohne Unterscheidung von Groß-/Kleinschreibung), sonst nach Nummer
/// („001“ → erstes Muster mit dieser Nummer).
pub fn finden(name: &str) -> Option<&'static Fuellmuster> {
    let n = name.trim();
    alle()
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(n))
        .or_else(|| alle().iter().find(|m| m.name.split(' ').next() == Some(n)))
}

/// Musterteilung für die Tatami-Reihen: Linienzüge (mm, Designkoordinaten) und Gitter, mit Skalierung.
#[derive(Clone, Debug)]
pub struct Teilung {
    pub zuege: Vec<Linie>,
    pub a: Punkt,
    pub b: Punkt,
    /// längster Stich zwischen zwei Musterlinien; längere Lücken werden gleichmäßig geteilt (eigene Festlegung:
    /// 7,0 mm wie die Sprunglänge der Vorlage 0x2712 = 1260; in den Samplern Stiche bis etwa 7–9 mm)
    pub max_stich: f64,
}

impl Fuellmuster {
    /// Teilung mit Skalierung in Prozent (100 = Original).
    pub fn teilung(&self, skalierung: f64) -> Teilung {
        let s = skalierung / 100.0;
        Teilung {
            zuege: self
                .zuege
                .iter()
                .map(|z| z.iter().map(|q| p(q[0] * s, q[1] * s)).collect())
                .collect(),
            a: p(self.a[0] * s, self.a[1] * s),
            b: p(self.b[0] * s, self.b[1] * s),
            max_stich: 7.0,
        }
    }
}

impl Teilung {
    /// Schnittstellen einer waagrechten Reihe (y, von x0 bis x1) mit dem gekachelten Muster, sortiert. Erwartet die
    /// Teilung schon im gedrehten Koordinatensystem der Reihen.
    pub fn schnitte(&self, y: f64, x0: f64, x1: f64) -> Vec<f64> {
        let (mut px0, mut px1, mut py0, mut py1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for q in self.zuege.iter().flatten() {
            (px0, px1, py0, py1) = (px0.min(q.x), px1.max(q.x), py0.min(q.y), py1.max(q.y));
        }
        if px0 > px1 {
            return Vec::new();
        }
        // Gitterzahlen (u, v) für die Ecken des Bereichs, in dem eine Kachel die Reihe treffen kann
        let det = self.a.x * self.b.y - self.a.y * self.b.x;
        if det.abs() < 1e-9 {
            return Vec::new();
        }
        let gitter = |x: f64, y: f64| ((x * self.b.y - y * self.b.x) / det, (self.a.x * y - self.a.y * x) / det);
        let ecken = [
            gitter(x0 - px1, y - py1),
            gitter(x1 - px0, y - py1),
            gitter(x0 - px1, y - py0),
            gitter(x1 - px0, y - py0),
        ];
        let (u0, u1) = ecken
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), e| (a.min(e.0), b.max(e.0)));
        let (v0, v1) = ecken
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), e| (a.min(e.1), b.max(e.1)));
        let mut xs = Vec::new();
        for i in (u0.floor() as i64 - 1)..=(u1.ceil() as i64 + 1) {
            for j in (v0.floor() as i64 - 1)..=(v1.ceil() as i64 + 1) {
                let (tx, ty) = (
                    i as f64 * self.a.x + j as f64 * self.b.x,
                    i as f64 * self.a.y + j as f64 * self.b.y,
                );
                if ty + py0 > y || ty + py1 < y || tx + px1 < x0 || tx + px0 > x1 {
                    continue;
                }
                for z in &self.zuege {
                    for s in z.windows(2) {
                        let (a, b) = (p(s[0].x + tx, s[0].y + ty), p(s[1].x + tx, s[1].y + ty));
                        if (a.y - y) * (b.y - y) > 0.0 || (a.y - b.y).abs() < 1e-12 {
                            continue;
                        }
                        let x = a.x + (b.x - a.x) * (y - a.y) / (b.y - a.y);
                        if x > x0 && x < x1 {
                            xs.push(x);
                        }
                    }
                }
            }
        }
        xs.sort_by(f64::total_cmp);
        xs
    }

    /// Teilung um `grad` drehen (für das Koordinatensystem der Reihen).
    pub fn gedreht(&self, grad: f64) -> Teilung {
        let d = |q: Punkt| crate::geometrie::drehen(q, grad, p(0.0, 0.0));
        Teilung {
            zuege: self.zuege.iter().map(|z| z.iter().map(|&q| d(q)).collect()).collect(),
            a: d(self.a),
            b: d(self.b),
            max_stich: self.max_stich,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muster_der_vorlage() {
        assert!(alle().len() >= 240, "{}", alle().len());
        let m = finden("001 Diamond (90)").unwrap();
        assert_eq!(m.winkel, Some(90.0));
        assert_eq!((m.a, m.b), ([7.0, 0.0], [0.0, 7.0]));
        assert!(finden("027 heart2 (90)").is_some());
        assert_eq!(finden("030").unwrap().name.split(' ').next(), Some("030"));
    }

    #[test]
    fn schnitte_einer_reihe() {
        // senkrechte Linie bei x = 1 in jeder 5-mm-Kachel: eine Reihe von 0 bis 20 trifft x = 1, 6, 11, 16
        let t = Teilung {
            zuege: vec![vec![p(1.0, 0.0), p(1.0, 5.0)]],
            a: p(5.0, 0.0),
            b: p(0.0, 5.0),
            max_stich: 7.0,
        };
        let xs = t.schnitte(2.5, 0.0, 20.0);
        assert_eq!(xs, vec![1.0, 6.0, 11.0, 16.0]);
    }

    /// Wie im Sampler: Reihen 0,20 mm, Einstiche auf den Rautenlinien, kein Stich unter 0,40 mm
    #[test]
    fn rautenfuellung_im_design() {
        let json = r##"{"rahmen": "artista 130 x 100", "objekte": [
            {"art": "muster", "muster": "001 Diamond (90)", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 30, "hoehe": 30}}}
        ]}"##;
        let d = crate::Design::aus_json(json).unwrap();
        let f = d.berechnen().unwrap();
        let st: Vec<_> = f.stiche.iter().filter(|s| s.art == crate::Art::Stich).collect();
        assert!(st.len() > 1000, "{}", st.len());
        let laengen: Vec<f64> = st
            .windows(2)
            .map(|w| crate::geometrie::abstand(w[0].p(), w[1].p()))
            .collect();
        let max = laengen.iter().copied().fold(0.0, f64::max);
        assert!(max <= 7.2, "längster Stich {max}");
        assert!(d.ausserhalb(&f).is_empty());
        assert!(
            crate::Design::aus_json(&json.replace("001 Diamond (90)", "999 Gibts nicht"))
                .unwrap()
                .berechnen()
                .is_err()
        );
    }
}
