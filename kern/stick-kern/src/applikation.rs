//! Applikation (Stoffteil aufnähen) – Roadmap Phase 4 Punkt 6, wie „Auto Appliqué“ im Original (Handbuch PDF-S.
//! 245–248): **Platzierungslinie → Stopp (Stoff auflegen) → Heftstich → Stopp (zuschneiden) → Abdeckstich**. Die
//! Maschine hält zwischen den Lagen an.
//!
//! Werte (`Analyse_Teil3/12_applikation.md`):
//! - Lagen und Stopps: Handbuch; die Stopps sind die drei „Garne“ der Original-Garnkarte `Applique.tch`
//!   („Applique Guide“, „Place Applique & Tack“, „Trim Applique & Cover“) – so zeigt die Maschine beim Anhalten an,
//!   was zu tun ist. Hoch.
//! - Platzierungslinie Lauf 2,0 mm, Heftstich Zickzack 3,0 mm breit mit 2,0 mm Abstand: Original-Alphabet
//!   „Easy Appliqué“ (Stile 0x271c 360, 0x2724 540, Abstand 360). Mittel (Werte eines Alphabets, nicht der
//!   Auto-Appliqué-Vorgabe, die noch nicht gefunden ist).
//! - Abdeckstich Satin **2,5 mm** breit, mittig auf dem Umriss: Handbuch-Beispiel (S. 247: „Satin Width 2.5 mm“);
//!   **eigene Festlegung** bis die Vorgabe des Originals (`AutoAppliqueValues`, MOD1) gefunden ist.
//! - Stil „Pre-cut“ (Standard, ohne Schnittlinie) oder „Trim in Place“ (`zuschneiden`): nach der Platzierungslinie
//!   eine **Schnittlinie** (Handbuch S. 246/248: „Place the fabric patch after the placement line has been stitched,
//!   then trim after the cutting line has been stitched“). Schnittlinie = Lauf 2,0 mm auf dem Umriss in der Farbe
//!   „Place Applique & Tack“, der Heftstich folgt nach dem Zuschneiden in „Trim Applique & Cover“ (Stichlänge und
//!   Zuordnung der Stopps: **eigene Festlegung**; Standard Pre-cut: eigene Festlegung, `AutoAppliqueValues` offen).
//! - Abdeckung **Blanket** (Handbuch S. 247): ohne Heftstich („Tacking is not used with blanket cover stitch“), Zähne
//!   nach innen in den Applikationsstoff (Schieberegler: „Blanket immer innen“), Rücken auf dem Umriss; Breite wie die
//!   Abdeckbreite, Abstand 2,0 mm (Vorlage 0x2728).

use crate::design::{Objekt, ObjektArt, SatinOptionen, Unterlage};
use crate::geometrie::{Linie, flaeche_vorzeichen, geschlossen_ohne_endpunkt, knick_grad, versatz_ring};

/// „Garne“ der Original-Garnkarte `Applique.tch` (Bildschirmfarben der Karte).
pub const FARBE_FUEHRUNG: &str = "#ff3cac";
pub const FARBE_HEFTEN: &str = "#1a8c67";
pub const FARBE_ABDECKEN: &str = "#5500aa";

/// Vorgaben einer Applikation (mm).
#[derive(Clone, Debug, PartialEq)]
pub struct ApplikationVorgabe {
    /// Umriss des Stoffteils (geschlossen)
    pub umriss: Linie,
    /// Löcher (z. B. das Innere eines „O“)
    pub loecher: Vec<Linie>,
    /// Garnfarbe des Abdeckstichs
    pub farbe: String,
    /// Breite des Abdeck-Satins
    pub breite: f64,
    /// fester Satinabstand des Abdeckstichs (None = Stoff/Auto Spacing)
    pub abstand: Option<f64>,
    /// Breite und Abstand des Zickzack-Heftstichs
    pub heft_breite: f64,
    pub heft_abstand: f64,
    /// Stichlänge der Platzierungslinie
    pub platz_laenge: f64,
    /// Abdeckung Blanket statt Satin (ohne Heftstich)
    pub blanket: bool,
    /// „Trim in Place“: Schnittlinie nach der Platzierungslinie
    pub zuschneiden: bool,
}

impl Default for ApplikationVorgabe {
    fn default() -> Self {
        ApplikationVorgabe {
            umriss: Vec::new(),
            loecher: Vec::new(),
            farbe: "#000000".into(),
            breite: 2.5,
            abstand: None,
            heft_breite: 3.0,
            heft_abstand: 2.0,
            platz_laenge: 2.0,
            blanket: false,
            zuschneiden: false,
        }
    }
}

/// Säule der Breite `breite` mittig auf einem geschlossenen Ring: (Seite innen, Seite außen), geschlossen,
/// Punkt für Punkt zugeordnet.
fn ring_saeule(ring: &[crate::geometrie::Punkt], breite: f64) -> (Linie, Linie) {
    let schliessen = |mut l: Linie| {
        if let Some(&q) = l.first() {
            l.push(q);
        }
        l
    };
    (
        schliessen(versatz_ring(ring, breite / 2.0, 4.0)),
        schliessen(versatz_ring(ring, -breite / 2.0, 4.0)),
    )
}

/// Applikation in drei Lagen auflösen: alle Platzierungslinien, alle Heftstiche, alle Abdeckstiche (je Lage erst der
/// Umriss, dann die Löcher). Jede Lage hat ihre eigene Farbe → Stopp an der Maschine.
pub fn applikation_objekte(v: &ApplikationVorgabe, name: &str) -> Vec<Objekt> {
    let ringe: Vec<Linie> = std::iter::once(&v.umriss)
        .chain(&v.loecher)
        .map(|r| geschlossen_ohne_endpunkt(r))
        .filter(|r| r.len() >= 3)
        .collect();
    // Abdeckfarbe gleich der Heftfarbe → kein Stopp zum Zuschneiden; dann die Abdeck-„Farbe“ der Karte
    let abdeck = if v.farbe.eq_ignore_ascii_case(FARBE_HEFTEN) {
        FARBE_ABDECKEN.to_string()
    } else {
        v.farbe.clone()
    };
    let lauf_ring = |r: &Linie, farbe: &str, was: &str| {
        let n = r.len();
        let mut punkte = r.clone();
        punkte.push(r[0]);
        let mut ecken: Vec<bool> = (0..n)
            .map(|i| knick_grad(r[(i + n - 1) % n], r[i], r[(i + 1) % n]) > 30.0)
            .collect();
        ecken.push(ecken[0]);
        Objekt {
            farbe: farbe.into(),
            name: format!("{name}: {was}"),
            art: ObjektArt::Lauf {
                punkte,
                ecken,
                laenge: Some(v.platz_laenge),
                dreifach: false,
                geschlossen: true,
            },
        }
    };
    let mut out: Vec<Objekt> = ringe
        .iter()
        .map(|r| lauf_ring(r, FARBE_FUEHRUNG, "Platzierung"))
        .collect();
    if v.zuschneiden {
        out.extend(ringe.iter().map(|r| lauf_ring(r, FARBE_HEFTEN, "Schnittlinie")));
    }
    // Heften (nicht bei Blanket); bei „Trim in Place“ nach dem Zuschneiden
    let heft_farbe = if v.zuschneiden { FARBE_ABDECKEN } else { FARBE_HEFTEN };
    for r in ringe.iter().filter(|_| !v.blanket) {
        let (a, b) = ring_saeule(r, v.heft_breite);
        out.push(Objekt {
            farbe: heft_farbe.into(),
            name: format!("{name}: Heften"),
            art: ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(Vec::new()),
                optionen: SatinOptionen {
                    abstand: Some(v.heft_abstand),
                    faktor: None,
                    kurzstiche: Some(false),
                    step: false,
                    zickzack: false,
                },
            },
        });
    }
    for (i, r) in ringe.iter().enumerate() {
        let art = if v.blanket {
            // Zähne nach innen in den Stoff: Umriss gegen den Uhrzeiger, Löcher im Uhrzeiger (Zähne links)
            let mut q = r.clone();
            if (flaeche_vorzeichen(&q) > 0.0) != (i == 0) {
                q.reverse();
            }
            ObjektArt::Blanket {
                punkte: q,
                geschlossen: true,
                abstand: v.abstand.unwrap_or(crate::sticharten::blanket::ABSTAND),
                breite: v.breite,
                mittig: false,
            }
        } else {
            let (a, b) = ring_saeule(r, v.breite);
            ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(Vec::new()),
                optionen: SatinOptionen {
                    abstand: v.abstand,
                    faktor: None,
                    kurzstiche: None,
                    step: false,
                    zickzack: false,
                },
            }
        };
        out.push(Objekt {
            farbe: abdeck.clone(),
            name: format!("{name}: Abdecken"),
            art,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Design;
    use crate::geometrie::{abstand, kreis, p};

    #[test]
    fn drei_lagen_mit_stopps() {
        let v = ApplikationVorgabe {
            umriss: kreis(p(0.0, 0.0), 20.0, 0.05, 0.0),
            loecher: vec![kreis(p(0.0, 0.0), 6.0, 0.05, 0.0)],
            farbe: "#c0392b".into(),
            ..Default::default()
        };
        let o = applikation_objekte(&v, "Test");
        assert_eq!(o.len(), 6);
        let farben: Vec<&str> = o.iter().map(|x| x.farbe.as_str()).collect();
        assert_eq!(
            farben,
            [
                FARBE_FUEHRUNG,
                FARBE_FUEHRUNG,
                FARBE_HEFTEN,
                FARBE_HEFTEN,
                "#c0392b",
                "#c0392b"
            ]
        );
        // Abdeck-Säule: 2,5 mm breit, mittig auf dem Kreis
        let ObjektArt::Satin { seite_a, seite_b, .. } = &o[4].art else {
            panic!()
        };
        let w = abstand(seite_a[0], seite_b[0]);
        let r = abstand(p(0.0, 0.0), crate::geometrie::zwischen(seite_a[0], seite_b[0], 0.5));
        assert!(
            (w - 2.5).abs() < 0.01 && (r - 20.0).abs() < 0.05,
            "Breite {w}, Radius {r}"
        );
    }

    #[test]
    fn im_design_drei_farbbloecke() {
        let json = r##"{"rahmen": "artista 130 x 100", "objekte": [
            {"art": "applikation", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 40, "hoehe": 30}}, "farbe": "#2e86c1"}
        ]}"##;
        let d = Design::aus_json(json).unwrap();
        let f = d.berechnen().unwrap();
        assert_eq!(f.farben, vec![FARBE_FUEHRUNG, FARBE_HEFTEN, "#2e86c1"]);
        assert_eq!(f.anzahl(crate::Art::Farbwechsel), 2);
        assert!(d.ausserhalb(&f).is_empty());
    }

    #[test]
    fn blanket_und_zuschneiden() {
        let basis = ApplikationVorgabe {
            umriss: kreis(p(0.0, 0.0), 20.0, 0.05, 0.0),
            farbe: "#c0392b".into(),
            ..Default::default()
        };
        // Blanket: kein Heftstich
        let o = applikation_objekte(
            &ApplikationVorgabe {
                blanket: true,
                ..basis.clone()
            },
            "B",
        );
        let farben: Vec<&str> = o.iter().map(|x| x.farbe.as_str()).collect();
        assert_eq!(farben, vec![FARBE_FUEHRUNG, "#c0392b"]);
        assert!(matches!(o[1].art, ObjektArt::Blanket { .. }));
        // Trim in Place: Platzierung, Schnittlinie, Heften, Abdecken
        let o = applikation_objekte(
            &ApplikationVorgabe {
                zuschneiden: true,
                ..basis.clone()
            },
            "Z",
        );
        let farben: Vec<&str> = o.iter().map(|x| x.farbe.as_str()).collect();
        assert_eq!(farben, vec![FARBE_FUEHRUNG, FARBE_HEFTEN, FARBE_ABDECKEN, "#c0392b"]);
        assert!(o[1].name.ends_with("Schnittlinie"));
        // Blanket-Zähne liegen innen
        let d = Design::aus_json(
            r##"{"rahmen": "artista 130 x 100", "objekte": [{"art": "applikation", "abdeckung": "blanket",
                "umriss": {"kreis": {"mitte": [0, 0], "radius": 20}}, "farbe": "#c0392b"}]}"##,
        )
        .unwrap();
        let f = d.berechnen().unwrap();
        let max_r = f.stiche.iter().map(|s| abstand(s.p(), p(0.0, 0.0))).fold(0.0, f64::max);
        assert!(max_r < 20.3, "Blanket nach außen: {max_r}");
    }
}
