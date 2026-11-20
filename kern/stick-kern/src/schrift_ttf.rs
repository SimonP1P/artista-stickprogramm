//! Computer-Schriften (TrueType/OpenType) als Stickobjekte – Roadmap Phase 4 Punkt 7.
//!
//! Das Original kann jede installierte TrueType-Schrift wie ein Alphabet benutzen (Handbuch PDF-S. 193, „Converting
//! TrueType fonts“); mit welcher Stichart, sagt es nicht, und die Musterdesigns enthalten keine TrueType-Schrift.
//! **Eigene Festlegung:** jeder Buchstabe wird als **Tatami-Fläche** (Umriss mit Löchern, Winkel 45°) gestickt – die
//! häufigste Flächenart im Original (Flächengruppen: 1 683 Tatami, 242 Satin) –, wahlweise nur der Umriss als Lauf-
//! oder Dreifachstich. Ein Buchstabe ist eine Gruppe (Laufweg statt Sprung, wie bei den Alphabeten).
//!
//! Das Lesen der Schriftdatei steckt in `stick-formate::ttf`; hier stehen das Modell und der Satz des Texts.

use crate::design::{Objekt, ObjektArt};
use crate::geometrie::{Linie, Punkt, im_polygon, knick_grad, p};
use crate::schrift::TextVorgabe;
use std::collections::BTreeMap;

/// Ein Zeichen: Umrisse (geschlossene Ringe, schon in Strecken zerlegt) in Schrift-Einheiten, y nach oben.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TtfZeichen {
    pub ringe: Vec<Vec<(f64, f64)>>,
    /// Vorschub in Schrift-Einheiten
    pub vorschub: f64,
}

/// Geladene Computer-Schrift (nur die gebrauchten Zeichen).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TtfSchrift {
    /// Name, wie das Design ihn nennt
    pub name: String,
    /// Höhe der Großbuchstaben in Schrift-Einheiten (Bezug für die Buchstabenhöhe, wie bei den Alphabeten)
    pub versal_hoehe: f64,
    /// Vorschub des Leerzeichens in Schrift-Einheiten
    pub leerzeichen: f64,
    pub zeichen: BTreeMap<char, TtfZeichen>,
}

/// Wie die Buchstaben gestickt werden.
#[derive(Clone, Debug, PartialEq)]
pub enum TtfStichart {
    /// Fläche je Buchstabe (Winkel, Unterlage: None = Auto Underlay des Stils)
    Tatami {
        winkel: Option<f64>,
        unterlage: Option<bool>,
    },
    /// nur die Umrisse
    Lauf { laenge: Option<f64>, dreifach: bool },
}

/// Ringe nach Verschachtelung ordnen (gerade Tiefe = Umriss, ungerade = Loch): (Umriss, Löcher). Unabhängig von der
/// Drehrichtung (TrueType und CFF zeichnen Außenringe verschieden herum).
pub fn ringe_ordnen(ringe: &[Linie]) -> Vec<(Linie, Vec<Linie>)> {
    let flaeche = |r: &Linie| -> f64 {
        (0..r.len())
            .map(|i| {
                let (a, b) = (r[i], r[(i + 1) % r.len()]);
                a.x * b.y - b.x * a.y
            })
            .sum::<f64>()
            .abs()
            / 2.0
    };
    let enthaelt = |aussen: &Linie, innen: &Linie| {
        innen
            .first()
            .is_some_and(|&q| im_polygon(q, std::slice::from_ref(aussen)))
    };
    let n = ringe.len();
    let tiefe: Vec<usize> = (0..n)
        .map(|i| (0..n).filter(|&j| j != i && enthaelt(&ringe[j], &ringe[i])).count())
        .collect();
    let mut formen: Vec<(usize, Linie, Vec<Linie>)> = (0..n)
        .filter(|&i| tiefe[i].is_multiple_of(2))
        .map(|i| (i, ringe[i].clone(), Vec::new()))
        .collect();
    for i in (0..n).filter(|&i| !tiefe[i].is_multiple_of(2)) {
        // kleinster Umriss mit Tiefe − 1, der das Loch enthält
        if let Some(f) = formen
            .iter_mut()
            .filter(|(k, u, _)| tiefe[*k] + 1 == tiefe[i] && enthaelt(u, &ringe[i]))
            .min_by(|a, b| flaeche(&a.1).total_cmp(&flaeche(&b.1)))
        {
            f.2.push(ringe[i].clone());
        }
    }
    formen.into_iter().map(|(_, u, l)| (u, l)).collect()
}

/// Text in Stickobjekte umsetzen (je Buchstabe eine Gruppe). Liefert die Objekte und die fehlenden Zeichen.
/// Höhe = Höhe der Großbuchstaben; Abstand (mm) kommt zum Vorschub der Schrift dazu (ohne Angabe 0).
pub fn ttf_text_objekte(schrift: &TtfSchrift, v: &TextVorgabe, art: &TtfStichart) -> (Vec<Objekt>, Vec<char>) {
    let bezug = if schrift.versal_hoehe > 0.0 {
        schrift.versal_hoehe
    } else {
        700.0
    };
    let s = v.hoehe / bezug;
    let sx = s * v.breite_prozent / 100.0;
    let abstand = v.abstand.unwrap_or(0.0) * v.breite_prozent / 100.0;
    let vorschub = |ch: char| -> Option<f64> {
        if ch == ' ' {
            return Some(schrift.leerzeichen * sx + abstand);
        }
        schrift.zeichen.get(&ch).map(|z| z.vorschub * sx + abstand)
    };
    let breite: f64 = v.text.chars().filter_map(vorschub).sum::<f64>() - if v.text.is_empty() { 0.0 } else { abstand };
    let mut x = v.position.x - v.ausrichtung * breite.max(0.0);
    let mut objekte = Vec::new();
    let mut fehlend = Vec::new();
    for ch in v.text.chars() {
        if ch == ' ' {
            x += schrift.leerzeichen * sx + abstand;
            continue;
        }
        let Some(z) = schrift.zeichen.get(&ch) else {
            if !ch.is_whitespace() {
                fehlend.push(ch);
            }
            continue;
        };
        let ringe: Vec<Linie> = z
            .ringe
            .iter()
            .filter(|r| r.len() >= 3)
            .map(|r| r.iter().map(|&(u, w)| p(x + u * sx, v.position.y + w * s)).collect())
            .collect();
        let name = format!("{} {ch}", schrift.name);
        let mut teile = Vec::new();
        let mut umrisse = Vec::new();
        match art {
            TtfStichart::Tatami { winkel, unterlage } => {
                for (umriss, loecher) in ringe_ordnen(&ringe) {
                    umrisse.push(umriss.clone());
                    teile.push(Objekt {
                        farbe: v.farbe.clone(),
                        name: name.clone(),
                        art: ObjektArt::Tatami {
                            umriss,
                            loecher,
                            winkel: *winkel,
                            unterlage: *unterlage,
                        },
                    });
                }
            }
            TtfStichart::Lauf { laenge, dreifach } => {
                for r in ringe {
                    let n = r.len();
                    // Einstich an Ecken (Knick > 30°), nicht an den Zwischenpunkten der Kurven
                    let ecken: Vec<bool> = (0..n)
                        .map(|i| knick_grad(r[(i + n - 1) % n], r[i], r[(i + 1) % n]) > 30.0)
                        .collect();
                    umrisse.push(Vec::<Punkt>::new());
                    teile.push(Objekt {
                        farbe: v.farbe.clone(),
                        name: name.clone(),
                        art: ObjektArt::Lauf {
                            punkte: r,
                            ecken,
                            laenge: *laenge,
                            dreifach: *dreifach,
                            geschlossen: true,
                        },
                    });
                }
            }
        }
        if !teile.is_empty() {
            objekte.push(Objekt {
                farbe: v.farbe.clone(),
                name,
                art: ObjektArt::Gruppe {
                    teile,
                    umrisse,
                    laufweg: crate::schrift::SCHRIFT_LAUFWEG,
                },
            });
        }
        x += z.vorschub * sx + abstand;
    }
    (objekte, fehlend)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quadrat(x0: f64, y0: f64, a: f64, rechts: bool) -> Vec<(f64, f64)> {
        let mut r = vec![(x0, y0), (x0 + a, y0), (x0 + a, y0 + a), (x0, y0 + a)];
        if rechts {
            r.reverse();
        }
        r
    }

    fn schrift() -> TtfSchrift {
        // „O“: Quadrat mit Loch; „I“: Balken; Großbuchstaben 700 Einheiten hoch
        let mut zeichen = BTreeMap::new();
        zeichen.insert(
            'O',
            TtfZeichen {
                ringe: vec![quadrat(0.0, 0.0, 700.0, false), quadrat(200.0, 200.0, 300.0, true)],
                vorschub: 800.0,
            },
        );
        zeichen.insert(
            'I',
            TtfZeichen {
                ringe: vec![vec![(0.0, 0.0), (150.0, 0.0), (150.0, 700.0), (0.0, 700.0)]],
                vorschub: 250.0,
            },
        );
        TtfSchrift {
            name: "Test".into(),
            versal_hoehe: 700.0,
            leerzeichen: 300.0,
            zeichen,
        }
    }

    fn vorgabe(text: &str) -> TextVorgabe {
        TextVorgabe {
            text: text.into(),
            hoehe: 14.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(0.0, 0.0),
            ausrichtung: 0.0,
            unterlage: None,
            farbe: "#000000".into(),
        }
    }

    #[test]
    fn loch_und_satz() {
        let (o, fehlend) = ttf_text_objekte(
            &schrift(),
            &vorgabe("OI Ä"),
            &TtfStichart::Tatami {
                winkel: None,
                unterlage: None,
            },
        );
        assert_eq!(fehlend, vec!['Ä']);
        assert_eq!(o.len(), 2);
        // „O“: eine Fläche mit einem Loch, 14 mm hoch (700 Einheiten → 14 mm)
        let ObjektArt::Gruppe { teile, .. } = &o[0].art else {
            panic!()
        };
        let ObjektArt::Tatami { umriss, loecher, .. } = &teile[0].art else {
            panic!()
        };
        assert_eq!(loecher.len(), 1);
        let ymax = umriss.iter().map(|q| q.y).fold(f64::MIN, f64::max);
        assert!((ymax - 14.0).abs() < 1e-9);
        // „I“ beginnt nach dem Vorschub des „O“ (800 → 16 mm)
        let ObjektArt::Gruppe { teile, .. } = &o[1].art else {
            panic!()
        };
        let ObjektArt::Tatami { umriss, .. } = &teile[0].art else {
            panic!()
        };
        let xmin = umriss.iter().map(|q| q.x).fold(f64::MAX, f64::min);
        assert!((xmin - 16.0).abs() < 1e-9);
    }

    #[test]
    fn umriss_als_lauf_und_ausrichtung() {
        let mut v = vorgabe("O");
        v.ausrichtung = 0.5;
        let (o, _) = ttf_text_objekte(
            &schrift(),
            &v,
            &TtfStichart::Lauf {
                laenge: None,
                dreifach: true,
            },
        );
        let ObjektArt::Gruppe { teile, .. } = &o[0].art else {
            panic!()
        };
        assert_eq!(teile.len(), 2, "Außen- und Innenring");
        let ObjektArt::Lauf { punkte, ecken, .. } = &teile[0].art else {
            panic!()
        };
        assert!(ecken.iter().all(|&e| e), "Quadrat: vier Ecken");
        let xmin = punkte.iter().map(|q| q.x).fold(f64::MAX, f64::min);
        assert!((xmin + 8.0).abs() < 1e-9, "mittig: 16 mm breit → ab −8 mm");
    }
}
