//! **Effekte** des Originals (Handbuch Kap. 24 „Artistic Stitch Effects“, Dialog „Effects“; Nachtauftrag N2, ROADMAP
//! 4b.8). Bericht: `Analyse_Teil3/23_effekte.md`.
//!
//! - **Verlauf** (Gradient Fill, MOD1 `GradientSpacingEffect`, ART-Datensatz 0x34 = u16 kleinster Abstand, u16 größter
//!   Abstand, Byte Profil): der Abstand wechselt quer über die Fläche zwischen dicht und offen; der feste Abstand des
//!   Objekts gilt dann nicht („the current spacing settings are ignored“, S. 218). Gemessen an den Musterdesigns
//!   Tomatos/AIRLINER: der Abstand zwischen zwei Hinreihen läuft **linear** über die ganze Form von einem Wert zum
//!   anderen (hoch für Profil 1/2).
//! - **Ausgefranster Rand** (Textured Edge, MOD1 `JaggedEdgeEffect`, Datensatz 0x32 = Byte Seite, float Spanne in
//!   1/180 mm, float Texture): die Reihenenden auf einer oder beiden Seiten streuen um den Rand.
//! - **Welle** (Wave Effect, MOD1 `CurveFillEffect` mit Leitkurve `OneCurveD2Wrap`, Datensätze 0x6d/0x6e): die Reihen
//!   folgen einer Leitlinie, Dichte und Einstichmuster bleiben gleich (S. 217).
//! - **Farben mischen** (Color Blending, S. 220–222): zwei Lagen derselben Form mit gegenläufigem Verlauf – im Kern
//!   über `farben_mischen` in `stick-api`, hier nur die Profile.

use crate::geometrie::{Punkt, abstand};

auswahl! {
    /// Profil des Verlaufs (Dialog „Gradient Fill“: Linear, Dec-Linear, Convex, Concave; „Color Blending“ zusätzlich
    /// Inc_Exp/Dec-Exp und Convex-Concave/Concave-Convex). Abstand vom kleinsten zum größten Wert über die Fläche
    pub enum Profil {
        /// vom kleinsten zum größten Abstand, gleichmäßig („Linear“, Datensatz 1)
        Linear = "linear",
        /// vom größten zum kleinsten Abstand („Dec-Linear“, Datensatz 2)
        Abnehmend = "abnehmend",
        /// zunehmend, erst schnell, dann langsam („Convex“, Datensatz 3; Kurvenform eigene Festlegung)
        Konvex = "konvex",
        /// abnehmend, erst schnell, dann langsam („Concave“, Datensatz 4; Kurvenform eigene Festlegung)
        Konkav = "konkav",
        /// zunehmend, erst langsam, dann schnell („Inc_Exp“, nur Farben mischen; Kurvenform eigene Festlegung)
        ExpZunehmend = "exp_zunehmend",
        /// abnehmend, erst langsam, dann schnell („Dec-Exp“; Kurvenform eigene Festlegung)
        ExpAbnehmend = "exp_abnehmend",
        /// S-Kurve zunehmend („Convex - Concave“; Kurvenform eigene Festlegung)
        KonvexKonkav = "konvex_konkav",
        /// S-Kurve abnehmend („Concave - Convex“; Kurvenform eigene Festlegung)
        KonkavKonvex = "konkav_konvex",
    }
}

auswahl! {
    /// Seite des ausgefransten Rands, in Stichrichtung gesehen (Dialog „Textured Edge“: Left, Both, Right)
    pub enum RandSeite {
        /// Anfang der Reihen (bei Satin: Seite A)
        Links = "links",
        /// Ende der Reihen (bei Satin: Seite B; Standard wie das Original: Seite 2)
        Rechts = "rechts",
        /// beide Seiten
        Beide = "beide",
    }
}

impl Profil {
    /// Gegenstück für die obere Lage beim Farbenmischen (Dialog „Color Blending“: Linear ↔ Dec-Linear, Inc_Exp ↔
    /// Dec-Exp, Convex ↔ Concave, Convex-Concave ↔ Concave-Convex)
    pub fn gegenstueck(self) -> Profil {
        match self {
            Profil::Linear => Profil::Abnehmend,
            Profil::Abnehmend => Profil::Linear,
            Profil::Konvex => Profil::Konkav,
            Profil::Konkav => Profil::Konvex,
            Profil::ExpZunehmend => Profil::ExpAbnehmend,
            Profil::ExpAbnehmend => Profil::ExpZunehmend,
            Profil::KonvexKonkav => Profil::KonkavKonvex,
            Profil::KonkavKonvex => Profil::KonvexKonkav,
        }
    }

    /// Anteil 0 … 1 zwischen kleinstem und größtem Abstand an der Stelle `t` (0 = Anfang, 1 = Ende)
    pub fn anteil(self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let exp = |t: f64| ((3.0 * t).exp() - 1.0) / (3.0f64.exp() - 1.0);
        let s = |t: f64| t * t * (3.0 - 2.0 * t);
        match self {
            Profil::Linear => t,
            Profil::Abnehmend => 1.0 - t,
            Profil::Konvex => t * (2.0 - t),
            Profil::Konkav => (1.0 - t) * (1.0 - t),
            Profil::ExpZunehmend => exp(t),
            Profil::ExpAbnehmend => exp(1.0 - t),
            Profil::KonvexKonkav => s(t),
            Profil::KonkavKonvex => 1.0 - s(t),
        }
    }
}

/// Verlauf (Gradient Fill): Abstand je Stelle zwischen `min` und `max` (mm).
#[derive(Clone, Debug, PartialEq)]
pub struct Verlauf {
    pub profil: Profil,
    pub min: f64,
    pub max: f64,
}

impl Verlauf {
    /// Abstand an der Stelle `t` (0 = Anfang der Fläche bzw. Säule, 1 = Ende)
    pub fn abstand(&self, t: f64) -> f64 {
        self.min + (self.max - self.min) * self.profil.anteil(t)
    }

    /// Lagen der Reihen von `y0` bis `y1`: Schritt = halber Abstand an der Stelle (Nachbarreihen liegen beim halben
    /// Abstand zweier Hinreihen wie bei Tatami, `Analyse_Teil3/06`); die erste Reihe einen halben Schritt vom Rand.
    pub fn reihen(&self, y0: f64, y1: f64) -> Vec<f64> {
        let h = (y1 - y0).max(1e-9);
        let mut out = Vec::new();
        let mut y = y0 + self.abstand(0.0) / 4.0;
        while y < y1 {
            out.push(y);
            y += (self.abstand((y - y0) / h) / 2.0).max(0.05);
        }
        out
    }
}

/// Ausgefranster Rand (Textured Edge): Reihenenden um bis zu ± `spanne` (mm) um den Rand gestreut; `glaette` 0 … 1
/// verbindet benachbarte Reihen (0 = jede Reihe frei, rau; 1 = gleichmäßig).
#[derive(Clone, Debug, PartialEq)]
pub struct Rand {
    pub seite: RandSeite,
    pub spanne: f64,
    pub glaette: f64,
}

impl Rand {
    /// Versatz der Enden (links, rechts) für `n` Reihen bzw. Stiche in mm; positiv = nach innen. Zufall Park–Miller
    /// wie die Kurzstiche (Startwert 0x66ec8f), Verteilung gleichmäßig in ± spanne – an Tomatos (Spanne 0,75 mm:
    /// −0,66 … +0,43 mm) gemessen, Verteilung **eigene Festlegung**.
    pub fn versaetze(&self, n: usize) -> Vec<(f64, f64)> {
        let mut z: u64 = 0x66ec8f;
        let mut zufall = || {
            z = z * 16807 % 2_147_483_647;
            z as f64 / 2_147_483_647.0 * 2.0 - 1.0
        };
        let g = self.glaette.clamp(0.0, 1.0);
        let (mut l, mut r) = (zufall(), zufall());
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            l = g * l + (1.0 - g) * zufall();
            r = g * r + (1.0 - g) * zufall();
            let (li, re) = match self.seite {
                RandSeite::Links => (l, 0.0),
                RandSeite::Rechts => (0.0, r),
                RandSeite::Beide => (l, r),
            };
            out.push((li * self.spanne, re * self.spanne));
        }
        out
    }
}

/// Effekte je Objekt, an die Stich-Einstellungen gehängt (nicht Teil des Stils).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Effekte {
    pub verlauf: Option<Verlauf>,
    pub rand: Option<Rand>,
    /// Leitlinie der Welle in mm (nur Step- und Musterflächen; wird mit dem Objekt transformiert)
    pub welle: Option<crate::geometrie::Linie>,
}

/// Welle: Höhe der Leitlinie über x im gedrehten Rahmen (Reihen waagerecht). Die Leitlinie wird dicht abgetastet und
/// als Funktion von x gelesen (bei Rückläufen zählt der erste Abschnitt), außerhalb gleich wie am Ende.
pub struct Welle {
    xs: Vec<f64>,
    ys: Vec<f64>,
}

impl Welle {
    pub fn neu(leit: &[Punkt]) -> Option<Welle> {
        let mut pts: Vec<Punkt> = Vec::new();
        for w in leit.windows(2) {
            let n = (abstand(w[0], w[1]) / 0.25).ceil().max(1.0) as usize;
            for j in 0..n {
                let t = j as f64 / n as f64;
                pts.push(Punkt {
                    x: w[0].x + (w[1].x - w[0].x) * t,
                    y: w[0].y + (w[1].y - w[0].y) * t,
                });
            }
        }
        pts.extend(leit.last());
        if pts.len() < 2 {
            return None;
        }
        if pts[0].x > pts.last().unwrap().x {
            pts.reverse();
        }
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        for q in pts {
            if xs.last().is_none_or(|&x| q.x > x + 1e-9) {
                xs.push(q.x);
                ys.push(q.y);
            }
        }
        if xs.len() < 2 || xs.last().unwrap() - xs[0] < 0.5 {
            return None;
        }
        Some(Welle { xs, ys })
    }

    /// Höhe der Leitlinie an der Stelle x
    pub fn hoehe(&self, x: f64) -> f64 {
        let n = self.xs.len();
        if x <= self.xs[0] {
            return self.ys[0];
        }
        if x >= self.xs[n - 1] {
            return self.ys[n - 1];
        }
        let i = self.xs.partition_point(|&v| v <= x).max(1);
        let t = (x - self.xs[i - 1]) / (self.xs[i] - self.xs[i - 1]);
        self.ys[i - 1] + (self.ys[i] - self.ys[i - 1]) * t
    }
}

/// Vorschlag für die Leitlinie einer Fläche (Wave Effect einschalten; im Original `CurveFillEffect::CreateDefaultShape`
/// – Form dort nicht gelesen, **eigene Festlegung**): quer durch die Fläche in Stichrichtung, 1 mm über den Rand
/// hinaus, mit einem Bogen von 15 % der Flächenhöhe in der Mitte (Kurvenpunkt), in Objektkoordinaten (vor `transform`).
pub fn welle_vorschlag(umriss: &[Punkt], winkel: f64) -> Option<serde_json::Value> {
    use crate::geometrie::{drehen, p};
    if umriss.len() < 3 {
        return None;
    }
    let null = p(0.0, 0.0);
    let g: Vec<Punkt> = umriss.iter().map(|&q| drehen(q, -winkel, null)).collect();
    let (x0, x1) = g.iter().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
    let (y0, y1) = g.iter().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
    let (cx, cy, h) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0, y1 - y0);
    let r = |q: Punkt| {
        let q = drehen(q, winkel, null);
        [(q.x * 100.0).round() / 100.0, (q.y * 100.0).round() / 100.0]
    };
    let (a, m, b) = (r(p(x0 - 1.0, cy)), r(p(cx, cy + 0.15 * h)), r(p(x1 + 1.0, cy)));
    Some(serde_json::json!({ "pfad": { "punkte": [a, { "kurve": m }, b] } }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::p;

    #[test]
    fn verlauf_profile() {
        let v = Verlauf {
            profil: Profil::Linear,
            min: 0.4,
            max: 2.0,
        };
        assert!((v.abstand(0.0) - 0.4).abs() < 1e-12 && (v.abstand(1.0) - 2.0).abs() < 1e-12);
        assert!((v.abstand(0.5) - 1.2).abs() < 1e-12);
        for p_ in [
            Profil::Linear,
            Profil::Konvex,
            Profil::ExpZunehmend,
            Profil::KonvexKonkav,
        ] {
            assert!(p_.anteil(0.0).abs() < 1e-12 && (p_.anteil(1.0) - 1.0).abs() < 1e-12, "{p_:?}");
            let g = p_.gegenstueck();
            assert!((g.anteil(0.0) - 1.0).abs() < 1e-12 && g.anteil(1.0).abs() < 1e-12, "{g:?}");
            assert_eq!(g.gegenstueck(), p_);
        }
        // Reihen: dicht am Anfang, offen am Ende; Schritt = halber Abstand
        let ys = v.reihen(0.0, 20.0);
        let d: Vec<f64> = ys.windows(2).map(|w| w[1] - w[0]).collect();
        assert!(d[0] < 0.25 && *d.last().unwrap() > 0.9, "{d:?}");
        assert!(d.windows(2).all(|w| w[1] >= w[0] - 1e-9));
    }

    #[test]
    fn rand_seiten_und_spanne() {
        let r = Rand {
            seite: RandSeite::Rechts,
            spanne: 0.75,
            glaette: 0.0,
        };
        let v = r.versaetze(200);
        assert!(v.iter().all(|&(l, re)| l == 0.0 && re.abs() <= 0.75));
        assert!(v.iter().any(|&(_, re)| re > 0.5) && v.iter().any(|&(_, re)| re < -0.5));
        let glatt = Rand { glaette: 0.9, ..r.clone() }.versaetze(200);
        let sprung = |v: &[(f64, f64)]| v.windows(2).map(|w| (w[1].1 - w[0].1).abs()).sum::<f64>();
        assert!(sprung(&glatt) < sprung(&v) / 3.0);
    }

    #[test]
    fn welle_als_funktion() {
        let w = Welle::neu(&[p(-10.0, 0.0), p(0.0, 3.0), p(10.0, 0.0)]).unwrap();
        assert!((w.hoehe(0.0) - 3.0).abs() < 1e-9 && (w.hoehe(-5.0) - 1.5).abs() < 1e-9);
        assert!((w.hoehe(-20.0)).abs() < 1e-9 && w.hoehe(20.0).abs() < 1e-9);
        // umgekehrt digitalisiert: gleiche Funktion
        let w2 = Welle::neu(&[p(10.0, 0.0), p(0.0, 3.0), p(-10.0, 0.0)]).unwrap();
        assert!((w2.hoehe(5.0) - 1.5).abs() < 1e-9);
        assert!(Welle::neu(&[p(0.0, 0.0), p(0.0, 5.0)]).is_none());
    }
}
