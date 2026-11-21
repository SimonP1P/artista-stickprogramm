//! Transformationen (Roadmap Phase 4b.1) wie „Arranging and Transforming Objects“ im Original (Handbuch Kap. 15):
//! Skalieren, Drehen, Spiegeln, Schrägstellen und Verschieben – je Objekt und für das ganze Design.
//!
//! Wie im Original werden **die Formen** transformiert und die Stiche danach **neu berechnet** (Dichte, Satinabstand
//! und Stichlängen bleiben, die Anzahl der Stiche ändert sich). Bezugspunkt ist die Mitte des umschließenden Rechtecks
//! (Original: Drehen/Skalieren um die Mitte der Auswahl). Stichwinkel drehen und spiegeln mit; Satinbreiten von
//! Satin- und Blanket-Linien skalieren mit der Fläche (√|det|); Musterteilung und Stichwerte bleiben (eigene
//! Festlegungen, wo das Handbuch nichts sagt). Reihenfolge: Spiegeln → Skalieren → Schrägstellen → Drehen → Verschieben.

use crate::design::{Objekt, ObjektArt};
use crate::geometrie::{Linie, Punkt, p};
use serde::Deserialize;

auswahl! {
    /// Spiegeln (Mirror)
    pub enum Spiegeln {
        /// waagrecht spiegeln (Mirror Horizontal)
        LinksRechts = "links_rechts",
        /// senkrecht spiegeln (Mirror Vertical)
        ObenUnten = "oben_unten",
        /// beides
        Beide = "beide",
    }
}

/// Transformation in der JSON-Form (`"transform"` am Objekt oder am Design).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Transformation {
    /// Skalierung in %: eine Zahl (beide Richtungen) oder [x, y]
    #[serde(default)]
    pub skalieren: Option<Skalierung>,
    /// Drehung in Grad gegen den Uhrzeiger
    #[serde(default)]
    pub drehen: f64,
    #[serde(default)]
    pub spiegeln: Option<Spiegeln>,
    /// Schrägstellen in Grad (x verschiebt sich mit y, wie Skew im Original)
    #[serde(default)]
    pub schraeg: f64,
    /// Verschieben in mm
    #[serde(default)]
    pub verschieben: Option<Punkt>,
}

/// Skalierung in %: eine Zahl (beide Richtungen) oder [x, y]
#[derive(Clone, Debug, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Skalierung {
    Gleich(f64),
    Getrennt([f64; 2]),
}

/// Affine Abbildung x' = a·x + b·y + e, y' = c·x + d·y + f.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix {
    pub const EINS: Matrix = Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    /// self nach m (erst m, dann self)
    fn nach(&self, m: &Matrix) -> Matrix {
        Matrix {
            a: self.a * m.a + self.b * m.c,
            b: self.a * m.b + self.b * m.d,
            c: self.c * m.a + self.d * m.c,
            d: self.c * m.b + self.d * m.d,
            e: self.a * m.e + self.b * m.f + self.e,
            f: self.c * m.e + self.d * m.f + self.f,
        }
    }

    pub fn punkt(&self, q: Punkt) -> Punkt {
        p(
            self.a * q.x + self.b * q.y + self.e,
            self.c * q.x + self.d * q.y + self.f,
        )
    }

    pub fn linie(&self, l: &[Punkt]) -> Linie {
        l.iter().map(|&q| self.punkt(q)).collect()
    }

    pub fn det(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Richtung (Grad) nach der Abbildung
    pub fn winkel(&self, grad: f64) -> f64 {
        let (s, c) = grad.to_radians().sin_cos();
        let (x, y) = (self.a * c + self.b * s, self.c * c + self.d * s);
        y.atan2(x).to_degrees().rem_euclid(180.0)
    }
}

impl Transformation {
    pub fn ist_leer(&self) -> bool {
        *self == Transformation::default()
    }

    /// Matrix um den Bezugspunkt `mitte`.
    pub fn matrix(&self, mitte: Punkt) -> Result<Matrix, String> {
        let (sx, sy) = match self.skalieren {
            None => (1.0, 1.0),
            Some(Skalierung::Gleich(s)) => (s / 100.0, s / 100.0),
            Some(Skalierung::Getrennt([x, y])) => (x / 100.0, y / 100.0),
        };
        if sx <= 0.0 || sy <= 0.0 {
            return Err("Skalierung muss größer als 0 % sein".into());
        }
        let (mx, my) = match self.spiegeln {
            None => (1.0, 1.0),
            Some(Spiegeln::LinksRechts) => (-1.0, 1.0),
            Some(Spiegeln::ObenUnten) => (1.0, -1.0),
            Some(Spiegeln::Beide) => (-1.0, -1.0),
        };
        let k = self.schraeg.to_radians().tan();
        let (s, c) = self.drehen.to_radians().sin_cos();
        let v = self.verschieben.unwrap_or(p(0.0, 0.0));
        let schieben = |dx: f64, dy: f64| Matrix {
            e: dx,
            f: dy,
            ..Matrix::EINS
        };
        let lineare = Matrix {
            a: c,
            b: -s,
            c: s,
            d: c,
            e: 0.0,
            f: 0.0,
        }
        .nach(&Matrix { b: k, ..Matrix::EINS })
        .nach(&Matrix {
            a: sx * mx,
            d: sy * my,
            ..Matrix::EINS
        });
        Ok(schieben(mitte.x + v.x, mitte.y + v.y)
            .nach(&lineare)
            .nach(&schieben(-mitte.x, -mitte.y)))
    }
}

/// Alle Formpunkte eines Objekts (für den Bezugspunkt).
pub fn formpunkte(art: &ObjektArt, out: &mut Vec<Punkt>) {
    match art {
        ObjektArt::Lauf { punkte, .. } | ObjektArt::SatinLinie { punkte, .. } | ObjektArt::Blanket { punkte, .. } => {
            out.extend(punkte)
        }
        ObjektArt::Satin { seite_a, seite_b, .. } => {
            out.extend(seite_a);
            out.extend(seite_b);
        }
        ObjektArt::Tatami { umriss, loecher, .. } | ObjektArt::Musterfuellung { umriss, loecher, .. } => {
            out.extend(umriss);
            out.extend(loecher.iter().flatten());
        }
        ObjektArt::Gruppe { teile, .. } => teile.iter().for_each(|t| formpunkte(&t.art, out)),
        ObjektArt::Wendefuellung { umriss, .. } => out.extend(umriss),
        ObjektArt::SatinFlaeche { umriss, loecher, .. } => {
            out.extend(umriss);
            out.extend(loecher.iter().flatten());
        }
        ObjektArt::Applikation(v) => {
            out.extend(&v.umriss);
            out.extend(v.loecher.iter().flatten());
        }
        ObjektArt::Kette { teile, .. } => teile.iter().for_each(|t| formpunkte(&t.art, out)),
        ObjektArt::Knopfloch(k) => out.push(k.mitte),
        ObjektArt::Motivlinie(m) => out.extend(&m.linie),
        ObjektArt::Text { .. } | ObjektArt::TtfText { .. } => {}
    }
}

/// Mitte des umschließenden Rechtecks.
pub fn mitte(objekte: &[Objekt]) -> Punkt {
    let mut pts = Vec::new();
    objekte.iter().for_each(|o| formpunkte(&o.art, &mut pts));
    if pts.is_empty() {
        return p(0.0, 0.0);
    }
    let (x0, x1) = pts
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
    let (y0, y1) = pts
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
    p((x0 + x1) / 2.0, (y0 + y1) / 2.0)
}

/// Objekt abbilden; `winkel_standard` = Stichwinkel der Füllung, wenn das Objekt keinen eigenen hat.
pub fn abbilden(o: &Objekt, m: &Matrix, winkel_standard: f64) -> Objekt {
    let flaeche = m.det().abs().sqrt();
    let gespiegelt = m.det() < 0.0;
    let art = match &o.art {
        ObjektArt::Lauf {
            punkte,
            ecken,
            laenge,
            dreifach,
            geschlossen,
        } => ObjektArt::Lauf {
            punkte: m.linie(punkte),
            ecken: ecken.clone(),
            laenge: *laenge,
            dreifach: *dreifach,
            geschlossen: *geschlossen,
        },
        ObjektArt::Satin {
            seite_a,
            seite_b,
            unterlage,
            optionen,
        } => ObjektArt::Satin {
            seite_a: m.linie(seite_a),
            seite_b: m.linie(seite_b),
            unterlage: unterlage.clone(),
            optionen: optionen.clone(),
        },
        ObjektArt::SatinLinie {
            punkte,
            breite,
            unterlage,
            mittig,
            optionen,
        } => {
            // gespiegelt: Laufrichtung umkehren, damit eine versetzte Säule auf derselben Seite der Linie bleibt
            let mut q = m.linie(punkte);
            if gespiegelt && !*mittig {
                q.reverse();
            }
            ObjektArt::SatinLinie {
                punkte: q,
                breite: breite * flaeche,
                unterlage: unterlage.clone(),
                mittig: *mittig,
                optionen: optionen.clone(),
            }
        }
        ObjektArt::Blanket {
            punkte,
            geschlossen,
            abstand,
            breite,
            mittig,
        } => {
            // gespiegelt: Laufrichtung umkehren, damit die Zähne auf derselben Seite der Form bleiben
            let mut q = m.linie(punkte);
            if gespiegelt {
                q.reverse();
            }
            ObjektArt::Blanket {
                punkte: q,
                geschlossen: *geschlossen,
                abstand: *abstand,
                breite: breite * flaeche,
                mittig: *mittig,
            }
        }
        ObjektArt::Tatami {
            umriss,
            loecher,
            winkel,
            unterlage,
            aussparen,
        } => ObjektArt::Tatami {
            umriss: m.linie(umriss),
            loecher: loecher.iter().map(|l| m.linie(l)).collect(),
            winkel: Some(m.winkel(winkel.unwrap_or(winkel_standard))),
            unterlage: *unterlage,
            aussparen: aussparen.clone(),
        },
        ObjektArt::Musterfuellung {
            umriss,
            loecher,
            muster,
            winkel,
            groesse,
            unterlage,
            aussparen,
        } => {
            let w = winkel
                .or_else(|| crate::muster::finden(muster).and_then(|x| x.winkel))
                .unwrap_or(winkel_standard);
            ObjektArt::Musterfuellung {
                umriss: m.linie(umriss),
                loecher: loecher.iter().map(|l| m.linie(l)).collect(),
                muster: muster.clone(),
                winkel: Some(m.winkel(w)),
                groesse: *groesse,
                unterlage: *unterlage,
                aussparen: aussparen.clone(),
            }
        }
        ObjektArt::SatinFlaeche {
            umriss,
            loecher,
            unterlage,
            optionen,
            knickwinkel,
        } => ObjektArt::SatinFlaeche {
            umriss: m.linie(umriss),
            loecher: loecher.iter().map(|l| m.linie(l)).collect(),
            unterlage: unterlage.clone(),
            optionen: optionen.clone(),
            knickwinkel: *knickwinkel,
        },
        ObjektArt::Wendefuellung {
            umriss,
            winkellinien,
            unterlage,
            kurzstiche,
        } => ObjektArt::Wendefuellung {
            umriss: m.linie(umriss),
            winkellinien: winkellinien.iter().map(|l| [m.punkt(l[0]), m.punkt(l[1])]).collect(),
            unterlage: *unterlage,
            kurzstiche: *kurzstiche,
        },
        ObjektArt::Gruppe {
            teile,
            umrisse,
            laufweg,
        } => ObjektArt::Gruppe {
            teile: teile.iter().map(|t| abbilden(t, m, winkel_standard)).collect(),
            umrisse: umrisse.iter().map(|l| m.linie(l)).collect(),
            laufweg: *laufweg,
        },
        ObjektArt::Kette { teile, umkehrbar, loch } => ObjektArt::Kette {
            teile: teile.iter().map(|t| abbilden(t, m, winkel_standard)).collect(),
            umkehrbar: umkehrbar.clone(),
            loch: loch.as_ref().map(|l| m.linie(l)),
        },
        andere => andere.clone(),
    };
    Objekt {
        farbe: o.farbe.clone(),
        name: o.name.clone(),
        art,
    }
}

/// Linie in Stücke ≤ `seg` teilen; zweiter Wert: welche Punkte die ursprünglichen sind.
fn teilen(l: &[Punkt], seg: f64, ring: bool) -> (Linie, Vec<bool>) {
    let (mut out, mut alt) = (Vec::new(), Vec::new());
    let n = l.len();
    for i in 0..n {
        out.push(l[i]);
        alt.push(true);
        let j = if i + 1 < n {
            i + 1
        } else if ring && n > 2 {
            0
        } else {
            break;
        };
        let k = (crate::geometrie::abstand(l[i], l[j]) / seg).ceil() as usize;
        for t in 1..k {
            out.push(crate::geometrie::zwischen(l[i], l[j], t as f64 / k as f64));
            alt.push(false);
        }
    }
    (out, alt)
}

/// Seiten einer Satinsäule mit gleich vielen Punkten paarweise teilen (Punkt i von A gehört zu Punkt i von B).
fn paare_teilen(a: &[Punkt], b: &[Punkt], seg: f64) -> (Linie, Linie) {
    let (mut x, mut y) = (Vec::new(), Vec::new());
    for i in 0..a.len() {
        x.push(a[i]);
        y.push(b[i]);
        if i + 1 == a.len() {
            break;
        }
        let d = crate::geometrie::abstand(a[i], a[i + 1]).max(crate::geometrie::abstand(b[i], b[i + 1]));
        let k = (d / seg).ceil() as usize;
        for t in 1..k {
            let s = t as f64 / k as f64;
            x.push(crate::geometrie::zwischen(a[i], a[i + 1], s));
            y.push(crate::geometrie::zwischen(b[i], b[i + 1], s));
        }
    }
    (x, y)
}

/// Objekt mit einer beliebigen Punktabbildung verformen (Elastic Lettering). Linien werden vorher in Stücke ≤ `seg` mm
/// geteilt, damit auch gerade Kanten der Verformung folgen; Stichwinkel und Breiten bleiben, die Stiche werden neu
/// berechnet.
pub fn verformen(o: &Objekt, f: &dyn Fn(Punkt) -> Punkt, seg: f64) -> Objekt {
    let lin = |l: &[Punkt], ring: bool| -> Linie { teilen(l, seg, ring).0.into_iter().map(f).collect() };
    let art = match &o.art {
        ObjektArt::Lauf {
            punkte,
            ecken,
            laenge,
            dreifach,
            geschlossen,
        } => {
            let (q, alt) = teilen(punkte, seg, *geschlossen);
            let mut e = ecken.iter();
            ObjektArt::Lauf {
                punkte: q.into_iter().map(f).collect(),
                ecken: alt.iter().map(|&a| a && *e.next().unwrap_or(&false)).collect(),
                laenge: *laenge,
                dreifach: *dreifach,
                geschlossen: *geschlossen,
            }
        }
        ObjektArt::Satin {
            seite_a,
            seite_b,
            unterlage,
            optionen,
        } => {
            let (a, b) = if seite_a.len() == seite_b.len() {
                let (a, b) = paare_teilen(seite_a, seite_b, seg);
                (a.into_iter().map(f).collect(), b.into_iter().map(f).collect())
            } else {
                (lin(seite_a, false), lin(seite_b, false))
            };
            ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: unterlage.clone(),
                optionen: optionen.clone(),
            }
        }
        ObjektArt::SatinLinie {
            punkte,
            breite,
            unterlage,
            mittig,
            optionen,
        } => ObjektArt::SatinLinie {
            punkte: lin(punkte, false),
            breite: *breite,
            unterlage: unterlage.clone(),
            mittig: *mittig,
            optionen: optionen.clone(),
        },
        ObjektArt::Blanket {
            punkte,
            geschlossen,
            abstand,
            breite,
            mittig,
        } => ObjektArt::Blanket {
            punkte: lin(punkte, *geschlossen),
            geschlossen: *geschlossen,
            abstand: *abstand,
            breite: *breite,
            mittig: *mittig,
        },
        ObjektArt::Tatami {
            umriss,
            loecher,
            winkel,
            unterlage,
            aussparen,
        } => ObjektArt::Tatami {
            umriss: lin(umriss, true),
            loecher: loecher.iter().map(|l| lin(l, true)).collect(),
            winkel: *winkel,
            unterlage: *unterlage,
            aussparen: aussparen.clone(),
        },
        ObjektArt::Musterfuellung {
            umriss,
            loecher,
            muster,
            winkel,
            groesse,
            unterlage,
            aussparen,
        } => ObjektArt::Musterfuellung {
            umriss: lin(umriss, true),
            loecher: loecher.iter().map(|l| lin(l, true)).collect(),
            muster: muster.clone(),
            winkel: *winkel,
            groesse: *groesse,
            unterlage: *unterlage,
            aussparen: aussparen.clone(),
        },
        ObjektArt::SatinFlaeche {
            umriss,
            loecher,
            unterlage,
            optionen,
            knickwinkel,
        } => ObjektArt::SatinFlaeche {
            umriss: lin(umriss, true),
            loecher: loecher.iter().map(|l| lin(l, true)).collect(),
            unterlage: unterlage.clone(),
            optionen: optionen.clone(),
            knickwinkel: *knickwinkel,
        },
        ObjektArt::Wendefuellung {
            umriss,
            winkellinien,
            unterlage,
            kurzstiche,
        } => ObjektArt::Wendefuellung {
            umriss: lin(umriss, true),
            winkellinien: winkellinien.iter().map(|l| [f(l[0]), f(l[1])]).collect(),
            unterlage: *unterlage,
            kurzstiche: *kurzstiche,
        },
        ObjektArt::Gruppe {
            teile,
            umrisse,
            laufweg,
        } => ObjektArt::Gruppe {
            teile: teile.iter().map(|t| verformen(t, f, seg)).collect(),
            umrisse: umrisse.iter().map(|l| lin(l, true)).collect(),
            laufweg: *laufweg,
        },
        ObjektArt::Kette { teile, umkehrbar, loch } => ObjektArt::Kette {
            teile: teile.iter().map(|t| verformen(t, f, seg)).collect(),
            umkehrbar: umkehrbar.clone(),
            loch: loch.as_ref().map(|l| lin(l, true)),
        },
        andere => andere.clone(),
    };
    Objekt {
        farbe: o.farbe.clone(),
        name: o.name.clone(),
        art,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::abstand;

    #[test]
    fn matrix_um_die_mitte() {
        let t = Transformation {
            drehen: 90.0,
            ..Default::default()
        };
        let m = t.matrix(p(10.0, 0.0)).unwrap();
        let q = m.punkt(p(12.0, 0.0));
        assert!(abstand(q, p(10.0, 2.0)) < 1e-9, "{q:?}");
        assert!((m.winkel(0.0) - 90.0).abs() < 1e-9);
        let s = Transformation {
            skalieren: Some(Skalierung::Getrennt([200.0, 50.0])),
            spiegeln: Some(Spiegeln::LinksRechts),
            ..Default::default()
        };
        let m = s.matrix(p(0.0, 0.0)).unwrap();
        assert_eq!(m.punkt(p(1.0, 2.0)), p(-2.0, 1.0));
        assert!(m.det() < 0.0);
        assert!(serde_json::from_str::<Transformation>(r#"{"spiegeln": "quer"}"#).is_err());
    }

    /// Objekt drehen und skalieren, Design als Ganzes verkleinern – Stiche werden neu berechnet
    #[test]
    fn transform_im_design() {
        let json = r##"{"rahmen": "artista 130 x 100", "objekte": [
            {"art": "tatami", "name": "Balken", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 40, "hoehe": 10}},
             "transform": {"drehen": 90}},
            {"art": "satinlinie", "name": "Linie", "punkte": [[-20, 25], [20, 25]], "breite": 3,
             "transform": {"skalieren": 50}}
        ]}"##;
        let d = crate::Design::aus_json(json).unwrap();
        let (obj, _) = d.objekte_aufgeloest().unwrap();
        let mut pts = Vec::new();
        formpunkte(&obj[0].art, &mut pts);
        let breite =
            pts.iter().map(|q| q.x).fold(f64::MIN, f64::max) - pts.iter().map(|q| q.x).fold(f64::MAX, f64::min);
        assert!((breite - 10.0).abs() < 1e-9, "gedreht: Breite {breite}");
        let ObjektArt::SatinLinie { punkte, breite, .. } = &obj[1].art else {
            panic!()
        };
        assert!((abstand(punkte[0], punkte[1]) - 20.0).abs() < 1e-9 && (breite - 1.5).abs() < 1e-9);
        // ganzes Design auf 50 %: halb so groß, deutlich weniger Stiche (Dichte bleibt)
        let voll = d.berechnen().unwrap().stiche.len();
        let klein =
            crate::Design::aus_json(&json.replace(r#""rahmen""#, r#""transform": {"skalieren": 50}, "rahmen""#))
                .unwrap()
                .berechnen()
                .unwrap()
                .stiche
                .len();
        assert!(klein * 2 < voll, "{klein} gegen {voll}");
    }

    /// Gruppe („Group“): zwei Flächen werden gemeinsam um die Mitte der Gruppe skaliert, verschoben – der Abstand
    /// zwischen ihnen wächst mit; ein Objekt außerhalb der Gruppe bleibt. Unbekannte Gruppe ist ein Fehler.
    #[test]
    fn gruppe_gemeinsam_transformieren() {
        let json = r##"{"gruppen": {"b": {"name": "Bild", "herkunft": "bild", "transform": {"skalieren": 200, "verschieben": [5, 0]}}},
          "objekte": [
            {"art": "tatami", "gruppe": "b", "umriss": {"rechteck": {"mitte": [-10, 0], "breite": 4, "hoehe": 4}}},
            {"art": "tatami", "gruppe": "b", "umriss": {"rechteck": {"mitte": [10, 0], "breite": 4, "hoehe": 4}}},
            {"art": "tatami", "umriss": {"rechteck": {"mitte": [0, 30], "breite": 4, "hoehe": 4}}}
        ]}"##;
        let d = crate::Design::aus_json(json).unwrap();
        let (obj, _) = d.objekte_aufgeloest().unwrap();
        let m = |o: &Objekt| {
            let mut pts = Vec::new();
            formpunkte(&o.art, &mut pts);
            let x0 = pts.iter().map(|q| q.x).fold(f64::MAX, f64::min);
            let x1 = pts.iter().map(|q| q.x).fold(f64::MIN, f64::max);
            let y0 = pts.iter().map(|q| q.y).fold(f64::MAX, f64::min);
            let y1 = pts.iter().map(|q| q.y).fold(f64::MIN, f64::max);
            ((x0 + x1) / 2.0, (y0 + y1) / 2.0, x1 - x0)
        };
        let (a, b, c) = (m(&obj[0]), m(&obj[1]), m(&obj[2]));
        assert!((a.0 + 15.0).abs() < 1e-9 && (b.0 - 25.0).abs() < 1e-9, "{a:?} {b:?}");
        assert!((a.2 - 8.0).abs() < 1e-9 && (b.2 - 8.0).abs() < 1e-9);
        assert!((c.0).abs() < 1e-9 && (c.1 - 30.0).abs() < 1e-9 && (c.2 - 4.0).abs() < 1e-9);
        let falsch = json.replace(r#""gruppe": "b", "umriss": {"rechteck": {"mitte": [10"#, r#""gruppe": "x", "umriss": {"rechteck": {"mitte": [10"#);
        assert!(crate::Design::aus_json(&falsch).unwrap_err().contains("gruppe „x“"));
    }

    #[test]
    fn oval_als_form() {
        let d = crate::Design::aus_json(
            r##"{"objekte": [{"art": "tatami", "umriss": {"oval": {"mitte": [5, 0], "breite": 40, "hoehe": 20}}}]}"##,
        )
        .unwrap();
        let mut pts = Vec::new();
        formpunkte(&d.objekte[0].art, &mut pts);
        let bx = pts.iter().map(|q| q.x).fold(f64::MIN, f64::max) - pts.iter().map(|q| q.x).fold(f64::MAX, f64::min);
        let by = pts.iter().map(|q| q.y).fold(f64::MIN, f64::max) - pts.iter().map(|q| q.y).fold(f64::MAX, f64::min);
        assert!((bx - 40.0).abs() < 0.05 && (by - 20.0).abs() < 0.05, "{bx} × {by}");
    }

    /// Kreis-Grundlinie: Buchstabe in der Textmitte sitzt oben auf dem Kreis, rechts davon im Uhrzeiger weiter
    #[test]
    fn kreis_grundlinie() {
        use crate::schrift::{Grundlinie, TextVorgabe};
        let buchstabe = |x: f64| Objekt {
            farbe: "#000000".into(),
            name: "B".into(),
            art: ObjektArt::Tatami {
                umriss: vec![p(x - 1.0, 0.0), p(x + 1.0, 0.0), p(x + 1.0, 5.0), p(x - 1.0, 5.0)],
                loecher: vec![],
                winkel: None,
                unterlage: None,
                aussparen: Default::default(),
            },
        };
        let zeile = TextVorgabe {
            text: "xx".into(),
            hoehe: 5.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(0.0, 0.0),
            ausrichtung: 0.5,
            unterlage: None,
            farbe: "#000000".into(),
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
            kursiv: 0.0,
        };
        let g = Grundlinie {
            radius: 20.0,
            ..Default::default()
        };
        let o = g.anwenden(
            vec![buchstabe(0.0), buchstabe(20.0 * std::f64::consts::FRAC_PI_2)],
            &zeile,
            p(0.0, 0.0),
            45.0,
        );
        let mut a = Vec::new();
        formpunkte(&o[0].art, &mut a);
        // erster Buchstabe oben: Fuß auf y = 20, Kopf bei y = 25
        assert!(a.iter().all(|q| q.y > 19.9 && q.y < 25.1), "{a:?}");
        let mut b = Vec::new();
        formpunkte(&o[1].art, &mut b);
        // Viertelkreis weiter (Bogenlänge 20·π/2): rechts auf dem Kreis, nach außen gedreht
        assert!(b.iter().all(|q| q.x > 19.9 && q.x < 25.1 && q.y.abs() < 1.1), "{b:?}");
        let unten = Grundlinie {
            lage: Some("unten".into()),
            ..g
        };
        let o = unten.anwenden(vec![buchstabe(0.0)], &zeile, p(0.0, 0.0), 45.0);
        let mut c = Vec::new();
        formpunkte(&o[0].art, &mut c);
        // unten (Circle CCW, Standard „Below“): Kopf auf y = −20, Fuß außen bei −25, aufrecht lesbar
        assert!(c.iter().all(|q| q.y > -25.1 && q.y < -19.9), "{c:?}");
    }
}
