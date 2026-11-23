//! Applikation (Stoffteil aufnähen) – Roadmap Phase 4 Punkt 6, wie „Auto Appliqué“ im Original (Handbuch PDF-S.
//! 245–248): **Platzierungslinie → Stopp (Stoff auflegen) → Heftstich → Stopp (zuschneiden) → Abdeckstich**. Die
//! Maschine hält zwischen den Lagen an.
//!
//! Werte (`Analyse_Teil3/12_applikation.md`):
//! - Lagen und Stopps: Handbuch; die Stopps sind die drei „Garne“ der Original-Garnkarte `Applique.tch`
//!   („Applique Guide“, „Place Applique & Tack“, „Trim Applique & Cover“) – so zeigt die Maschine beim Anhalten an,
//!   was zu tun ist. Hoch.
//! - **Vorgaben wie „Auto Appliqué“** (`AutoAppliqueValues`, Konstruktor MOD1 0x101f3c70 = NORMAL.AMT DataItem 0x3f19,
//!   Zuordnung über den Dialog BERESENU DLG 1455 – hoch, Bericht 12 §8): Platzierungslinie Lauf **1,5 mm**
//!   (0x3f1a), Heftstich Zickzack **4,0 mm** breit (0x3f1d) mit **5,0 mm** Abstand (0x3f21), Abdeckstich Satin
//!   **5,0 mm** breit (0x3f27) mit **0,5 mm** Abstand (0x3f30), Stil **Pre-cut** (0x3f31 = 1), Versatz 0 (mittig,
//!   0x3f28), ohne Teil-Abdeckung (0x3f26). Bis 22.11.2026 galten die Werte des Alphabets „Easy Appliqué“ (2,0 /
//!   3,0 / 2,0 mm) und 2,5 mm (Handbuch-Beispiel).
//! - **Versatz** (Schieberegler „Inside ↔ Outside“, −1 … +1 in 21 Stufen; derselbe Wert für Abdeck- und Heftstich,
//!   DIALOG.DLL 0x1001ce70/0x1001d36e): +1 = Band ganz im Stoff, 0 = mittig auf der Platzierungslinie, −1 = ganz
//!   außen. Bei Blanket gesperrt („always 100% inside“, Handbuch S. 248).
//! - **Teil-Abdeckung** (Partial Appliqué, Handbuch S. 248–249, `AppliqueOverlapProcessor`): der Abdeckstich entfällt,
//!   wo eine spätere Applikation den Rand überdeckt – „overlapping effect without doubling-up borders“. Platzierung
//!   und Heften bleiben ganz (eigene Festlegung; das Original schneidet über `FormIntersections`, nicht gelesen).
//! - Stil „Pre-cut“ (Standard, ohne Schnittlinie) oder „Trim in Place“ (`zuschneiden`): nach der Platzierungslinie
//!   eine **Schnittlinie** (Handbuch S. 246/248: „Place the fabric patch after the placement line has been stitched,
//!   then trim after the cutting line has been stitched“). Schnittlinie = Lauf 2,0 mm auf dem Umriss in der Farbe
//!   „Place Applique & Tack“, der Heftstich folgt nach dem Zuschneiden in „Trim Applique & Cover“ (Stichlänge und
//!   Zuordnung der Stopps: **eigene Festlegung**).
//! - Abdeckung **Blanket** (Handbuch S. 247): ohne Heftstich („Tacking is not used with blanket cover stitch“), Zähne
//!   nach innen in den Applikationsstoff (Schieberegler: „Blanket immer innen“), Rücken auf dem Umriss; Breite wie die
//!   Abdeckbreite, Abstand ohne Angabe 2,0 mm (Vorlage 0x2728; die 0,5 mm der Satin-Vorgabe wären für Blanket zu dicht
//!   – eigene Festlegung).
//! - **Herausfahren** (Frame Out, `herausfahren`; Bericht 12 §9): das Original fügt vor jedem Halt (Platzierung,
//!   Schnittlinie, Heften: `Applique::RegenerateComponents` 0x1000e6c0, Bausteine 0x10012ca0/0x10012e50/0x10013010)
//!   ein Handstich-Objekt der Stichart Sprung (0x2713) mit einem Punkt ein: `GetJumpoutPoint` 0x1000f2f0 = Mitte der
//!   Oberkante (`GetFullExtents`) + Versatz 0x3f2a/0x3f2b = (0; 2160 = **12 mm**). Hoch (Code), Wirkung an der 185
//!   offen (Vor Ort). Im Kern als Option, Standard aus (Entscheidung des Nutzers 22.11.2026); der Punkt bleibt im
//!   Stickfeld (eigene Festlegung). Umgesetzt beim Verbinden (`design.rs`, `nachbearbeitung::Zusatz::herausfahren`).

use crate::design::{Objekt, ObjektArt, SatinOptionen, Unterlage};
use crate::geometrie::{Linie, flaeche_vorzeichen, geschlossen_ohne_endpunkt, knick_grad, versatz_ring};

/// „Garne“ der Original-Garnkarte `Applique.tch` (Bildschirmfarben der Karte).
pub const FARBE_FUEHRUNG: &str = "#ff3cac";
pub const FARBE_HEFTEN: &str = "#1a8c67";
pub const FARBE_ABDECKEN: &str = "#5500aa";

/// Stichlänge der Platzierungslinie (0x3f1a = 270)
pub const PLATZ_LAENGE: f64 = 1.5;
/// Breite des Zickzack-Heftstichs (0x3f1d = 720)
pub const HEFT_BREITE: f64 = 4.0;
/// Abstand des Heftstichs (0x3f21 = 900)
pub const HEFT_ABSTAND: f64 = 5.0;
/// Breite des Abdeckstichs (0x3f27 = 900)
pub const BREITE: f64 = 5.0;
/// Satinabstand des Abdeckstichs (0x3f30 = 90)
pub const ABSTAND: f64 = 0.5;
/// Herausfahren: so weit über der Oberkante der Applikation (0x3f2b = 2160; 0x3f2a = 0 → Mitte)
pub const HERAUSFAHREN: f64 = 12.0;

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
    /// Abstand des Abdeckstichs (None = Vorgabe: Satin 0,5 mm, Blanket 2,0 mm)
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
    /// Versatz von Abdeck- und Heftstich gegen die Platzierungslinie: −1 (außen) … 0 (mittig) … +1 (im Stoff)
    pub versatz: f64,
    /// Teil-Abdeckung: kein Abdeckstich, wo eine spätere Applikation den Rand überdeckt
    pub teilabdeckung: bool,
    /// Herausfahren (Frame Out) vor jedem Halt
    pub herausfahren: bool,
}

impl Default for ApplikationVorgabe {
    fn default() -> Self {
        ApplikationVorgabe {
            umriss: Vec::new(),
            loecher: Vec::new(),
            farbe: "#000000".into(),
            breite: BREITE,
            abstand: None,
            heft_breite: HEFT_BREITE,
            heft_abstand: HEFT_ABSTAND,
            platz_laenge: PLATZ_LAENGE,
            blanket: false,
            zuschneiden: false,
            versatz: 0.0,
            teilabdeckung: false,
            herausfahren: false,
        }
    }
}

/// Säule der Breite `breite` auf einem geschlossenen Ring, geschlossen und Punkt für Punkt zugeordnet. `versatz`
/// −1 … +1 schiebt das Band in den Stoff (+1: ganz innen) bzw. hinaus (−1); `loch` = der Ring ist ein Loch (der Stoff
/// liegt außerhalb). Rückgabe (Seite zum Ringinneren, Seite nach außen).
fn ring_saeule(ring: &[crate::geometrie::Punkt], breite: f64, versatz: f64, loch: bool) -> (Linie, Linie) {
    let schliessen = |mut l: Linie| {
        if let Some(&q) = l.first() {
            l.push(q);
        }
        l
    };
    // Anteil im Stoff
    let f = (1.0 + versatz.clamp(-1.0, 1.0)) / 2.0;
    let (rein, raus) = if loch {
        ((1.0 - f) * breite, -f * breite)
    } else {
        (f * breite, -(1.0 - f) * breite)
    };
    (
        schliessen(versatz_ring(ring, rein, 4.0)),
        schliessen(versatz_ring(ring, raus, 4.0)),
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
    for (i, r) in ringe.iter().enumerate().filter(|_| !v.blanket) {
        let (a, b) = ring_saeule(r, v.heft_breite, v.versatz, i > 0);
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
                    zugausgleich: None,
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
            let (a, b) = ring_saeule(r, v.breite, v.versatz, i > 0);
            ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(Vec::new()),
                optionen: SatinOptionen {
                    abstand: Some(v.abstand.unwrap_or(ABSTAND)),
                    faktor: None,
                    kurzstiche: None,
                    step: false,
                    zickzack: false,
                    zugausgleich: None,
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

/// **Herausfahr-Punkt** (Frame Out, `Applique::GetJumpoutPoint` 0x1000f2f0): Mitte der Oberkante aller Punkte der
/// Applikation (Original: `GetFullExtents` über alle Lagen) plus 12 mm nach oben (+y). Liegt er außerhalb des
/// Stickfelds (`im_feld`), rückt er in 0,5-mm-Schritten zur Oberkante zurück (eigene Festlegung; das Original prüft
/// das hier nicht).
pub fn herausfahr_punkt<'a>(
    punkte: impl Iterator<Item = &'a crate::geometrie::Punkt>,
    im_feld: &dyn Fn(crate::geometrie::Punkt) -> bool,
) -> Option<crate::geometrie::Punkt> {
    let (mut x0, mut x1, mut y1) = (f64::MAX, f64::MIN, f64::MIN);
    for q in punkte {
        x0 = x0.min(q.x);
        x1 = x1.max(q.x);
        y1 = y1.max(q.y);
    }
    if x0 > x1 {
        return None;
    }
    let mitte = (x0 + x1) / 2.0;
    let mut d = HERAUSFAHREN;
    while d > 0.0 && !im_feld(crate::geometrie::p(mitte, y1 + d)) {
        d -= 0.5;
    }
    Some(crate::geometrie::p(mitte, y1 + d.max(0.0)))
}

/// Teil-Abdeckung (Partial Appliqué): einen Abdeckstich (geschlossene Satinsäule oder Blanket-Ring) dort weglassen, wo
/// `bedeckt` gilt (Punkt liegt im Stoff einer späteren Applikation). Die übrigen Stücke werden offene Objekte; jedes
/// reicht einen Punkt weit unter den Rand der oberen Applikation, damit keine Lücke bleibt. `None` = unverändert.
pub fn teil_abdecken(art: &ObjektArt, bedeckt: &dyn Fn(crate::geometrie::Punkt) -> bool) -> Option<Vec<ObjektArt>> {
    // Stücke der offenen Indexfolge 0..n eines Rings, deren Punkt frei ist (mit je einem bedeckten Nachbarn)
    let stuecke = |frei: &[bool]| -> Option<Vec<Vec<usize>>> {
        let n = frei.len();
        let start = frei.iter().position(|f| !f)?;
        if frei.iter().all(|f| !f) {
            return Some(Vec::new());
        }
        let mut out = Vec::new();
        let mut cur: Vec<usize> = Vec::new();
        for k in 0..=n {
            let i = (start + k) % n;
            if frei[i] && k < n {
                if cur.is_empty() {
                    cur.push((i + n - 1) % n);
                }
                cur.push(i);
            } else if !cur.is_empty() {
                cur.push(i);
                out.push(std::mem::take(&mut cur));
            }
        }
        Some(out)
    };
    match art {
        ObjektArt::Satin {
            seite_a,
            seite_b,
            unterlage,
            optionen,
        } if seite_a.len() == seite_b.len() && seite_a.len() > 3 && seite_a.first() == seite_a.last() => {
            let n = seite_a.len() - 1;
            let frei: Vec<bool> = (0..n)
                .map(|i| !bedeckt(crate::geometrie::zwischen(seite_a[i], seite_b[i], 0.5)))
                .collect();
            if frei.iter().all(|&f| f) {
                return None;
            }
            let teile = stuecke(&frei)?;
            Some(
                teile
                    .into_iter()
                    .filter(|t| t.len() >= 2)
                    .map(|t| ObjektArt::Satin {
                        seite_a: t.iter().map(|&i| seite_a[i]).collect(),
                        seite_b: t.iter().map(|&i| seite_b[i]).collect(),
                        unterlage: unterlage.clone(),
                        optionen: optionen.clone(),
                    })
                    .collect(),
            )
        }
        ObjektArt::Blanket {
            punkte,
            geschlossen: true,
            abstand,
            breite,
            mittig,
        } => {
            let q = geschlossen_ohne_endpunkt(punkte);
            if q.len() < 3 {
                return None;
            }
            let frei: Vec<bool> = q.iter().map(|&x| !bedeckt(x)).collect();
            if frei.iter().all(|&f| f) {
                return None;
            }
            let teile = stuecke(&frei)?;
            Some(
                teile
                    .into_iter()
                    .filter(|t| t.len() >= 2)
                    .map(|t| ObjektArt::Blanket {
                        punkte: t.iter().map(|&i| q[i]).collect(),
                        geschlossen: false,
                        abstand: *abstand,
                        breite: *breite,
                        mittig: *mittig,
                    })
                    .collect(),
            )
        }
        _ => None,
    }
}

/// Lage einer Applikation – für die Anleitung an den Farbstopps (Handbuch Kap. „Appliqué“, `Analyse_Teil3/12` §1):
/// nach der Platzierungslinie Stoff auflegen, nach der Schnittlinie bzw. dem Heften zuschneiden, dann Abdecken.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Lage {
    /// Platzierungslinie („Applique Guide“) – danach Stoff auflegen
    Platzierung,
    /// Schnittlinie (Stil „zuschneiden“ / Trim in Place) – danach zuschneiden
    Schnittlinie,
    /// Heftstich („Tack“) – bei Pre-cut danach überstehenden Stoff zuschneiden
    Heften,
    /// Abdeckstich (Satin bzw. Blanket)
    Abdecken,
}

/// Lagen der Applikation in Stickreihenfolge mit ihrer Farbe (= Farbblock an der Maschine). Aus denselben Objekten
/// wie [`applikation_objekte`] abgeleitet, damit Anleitung und Stiche nie auseinanderlaufen.
pub fn lagen(v: &ApplikationVorgabe) -> Vec<(String, Lage)> {
    let mut out: Vec<(String, Lage)> = Vec::new();
    for o in applikation_objekte(v, "") {
        let lage = match o.name.rsplit(": ").next().unwrap_or_default() {
            "Platzierung" => Lage::Platzierung,
            "Schnittlinie" => Lage::Schnittlinie,
            "Heften" => Lage::Heften,
            _ => Lage::Abdecken,
        };
        if out
            .last()
            .is_none_or(|(f, l)| !f.eq_ignore_ascii_case(&o.farbe) || *l != lage)
        {
            out.push((o.farbe.to_ascii_lowercase(), lage));
        }
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
        // Abdeck-Säule: 5,0 mm breit (Vorgabe des Originals), mittig auf dem Kreis
        let ObjektArt::Satin { seite_a, seite_b, .. } = &o[4].art else {
            panic!()
        };
        let w = abstand(seite_a[0], seite_b[0]);
        let r = abstand(p(0.0, 0.0), crate::geometrie::zwischen(seite_a[0], seite_b[0], 0.5));
        assert!(
            (w - BREITE).abs() < 0.02 && (r - 20.0).abs() < 0.05,
            "Breite {w}, Radius {r}"
        );
    }

    /// Versatz: +1 = Band ganz im Stoff (beim Loch außerhalb des Lochs), −1 = ganz außen
    #[test]
    fn versatz_innen_aussen() {
        let radien = |versatz: f64, nr: usize| {
            let v = ApplikationVorgabe {
                umriss: kreis(p(0.0, 0.0), 20.0, 0.05, 0.0),
                loecher: vec![kreis(p(0.0, 0.0), 8.0, 0.05, 0.0)],
                versatz,
                ..Default::default()
            };
            let o = applikation_objekte(&v, "V");
            let ObjektArt::Satin { seite_a, seite_b, .. } = &o[nr].art else {
                panic!()
            };
            let (ra, rb) = (abstand(p(0.0, 0.0), seite_a[0]), abstand(p(0.0, 0.0), seite_b[0]));
            (ra.min(rb), ra.max(rb))
        };
        // Umriss (Abdecken = Objekt 4): innen 15 … 20, außen 20 … 25
        let (a, b) = radien(1.0, 4);
        assert!((a - 15.0).abs() < 0.05 && (b - 20.0).abs() < 0.05, "{a} {b}");
        let (a, b) = radien(-1.0, 4);
        assert!((a - 20.0).abs() < 0.05 && (b - 25.0).abs() < 0.05, "{a} {b}");
        // Loch (Objekt 5): im Stoff = außerhalb des Lochs, 8 … 13
        let (a, b) = radien(1.0, 5);
        assert!((a - 8.0).abs() < 0.05 && (b - 13.0).abs() < 0.05, "{a} {b}");
        // Heftstich (Objekt 2) mit demselben Versatz: 4 mm im Stoff
        let (a, b) = radien(1.0, 2);
        assert!((a - 16.0).abs() < 0.05 && (b - 20.0).abs() < 0.05, "{a} {b}");
    }

    /// Teil-Abdeckung: die untere Applikation verliert den Abdeckstich unter der oberen, nicht mehr
    #[test]
    fn teilabdeckung_unter_der_oberen() {
        let json = |teil: bool| {
            format!(
                r##"{{"rahmen": "artista 130 x 100", "objekte": [
                {{"art": "applikation", "umriss": {{"kreis": {{"mitte": [-10, 0], "radius": 15}}}}, "farbe": "#2e86c1",
                  "teilabdeckung": {teil}}},
                {{"art": "applikation", "umriss": {{"kreis": {{"mitte": [10, 0], "radius": 15}}}}, "farbe": "#c0392b"}}
            ]}}"##
            )
        };
        let voll = Design::aus_json(&json(false)).unwrap().berechnen().unwrap();
        let teil = Design::aus_json(&json(true)).unwrap().berechnen().unwrap();
        // Abdeckstiche der unteren (blau) – rechts von x = 0 liegt die obere
        let blau = |f: &crate::StichFolge| {
            f.farbbloecke()
                .into_iter()
                .filter(|(c, _)| c.eq_ignore_ascii_case("#2e86c1"))
                .flat_map(|(_, s)| s)
                .filter(|s| s.art == crate::Art::Stich)
                .collect::<Vec<_>>()
        };
        let (bv, bt) = (blau(&voll), blau(&teil));
        assert!(bv.iter().any(|s| s.x > 3.0), "ohne Teil-Abdeckung ganz herum");
        // unter der oberen nur bis knapp hinter den Rand (ein Punkt Überstand)
        assert!(bt.iter().all(|s| s.x < 3.0), "{:?}", bt.iter().map(|s| s.x).fold(f64::MIN, f64::max));
        assert!(bt.len() < bv.len() * 9 / 10 && bt.len() > bv.len() / 2, "{} von {}", bt.len(), bv.len());
        // Platzierung und Heften bleiben ganz
        assert_eq!(voll.farben, teil.farben);
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

    #[test]
    fn herausfahren_vor_jedem_halt() {
        // Halte-Punkte: (Position des Farbwechsels, Stich davor ist ein Sprung?)
        let halte = |json: &str| {
            let d = Design::aus_json(json).unwrap();
            let f = d.berechnen().unwrap();
            assert!(d.ausserhalb(&f).is_empty(), "außerhalb des Stickfelds");
            let oben = f.stiche.iter().filter(|s| s.art == crate::Art::Stich).map(|s| s.y).fold(f64::MIN, f64::max);
            let wechsel: Vec<(crate::geometrie::Punkt, bool)> = f
                .stiche
                .windows(2)
                .filter(|w| w[1].art == crate::Art::Farbwechsel)
                .map(|w| (w[1].p(), w[0].art == crate::Art::Sprung))
                .collect();
            (oben, wechsel)
        };
        let rechteck = |mitte_y: f64, extra: &str| {
            format!(
                r##"{{"rahmen": "artista 130 x 100", "objekte": [{{"art": "applikation", {extra}
                "umriss": {{"rechteck": {{"mitte": [0, {mitte_y}], "breite": 40, "hoehe": 20}}}}, "farbe": "#2e86c1"}}]}}"##
            )
        };
        // ohne: der Rahmen hält am Ende der Lage
        let (oben, w) = halte(&rechteck(0.0, ""));
        assert_eq!(w.len(), 2);
        assert!(w.iter().all(|(q, _)| q.y <= oben + 1e-6));
        // mit: vor beiden Halten Sprung zur Mitte der Oberkante + 12 mm
        let (oben, w) = halte(&rechteck(0.0, r#""herausfahren": true,"#));
        assert_eq!(w.len(), 2);
        for (q, sprung) in &w {
            assert!(sprung, "kein Sprung vor dem Halt");
            assert!(q.x.abs() < 1e-6 && (q.y - (oben + HERAUSFAHREN)).abs() < 1e-6, "{q:?}, oben {oben}");
        }
        // Zuschneiden: drei Halte, alle mit Herausfahren
        let (_, w) = halte(&rechteck(0.0, r#""herausfahren": true, "stil": "zuschneiden","#));
        assert_eq!(w.len(), 3);
        assert!(w.iter().all(|(q, s)| *s && q.y > 20.0));
        // nah am oberen Rand (Stickfeld 100 × 130, bis y = 65): Punkt bleibt im Feld
        let (oben, w) = halte(&rechteck(50.0, r#""herausfahren": true,"#));
        assert!(oben + HERAUSFAHREN > 65.0);
        assert!(w.iter().all(|(q, _)| q.y <= 65.0 + 1e-6 && q.y > oben), "{w:?}");
    }
}
