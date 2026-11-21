//! Knopflöcher wie „Add Buttonhole“ des Originals (Handbuch Kap. 23, ROADMAP 4b.6; Bericht
//! `Analyse_Teil3/19_knopfloecher.md`).
//!
//! **Vorlagen:** Die 9 Knopfloch-Typen sind Zeichen der Datei `wletter/ButtonHoles.ESB` (gelesen von
//! `stick_formate::esb::knopfloecher_laden`). Jedes Teil trägt einen Namen (Datensatz 0x28): `<L>` unteres Ende,
//! `<U>` oberes Ende, `<B>` Raupe (Bead), ohne Namen = unteres Ende; mit Zusatz `E`/`I`/`M` ist es kein Stichteil,
//! sondern ein Umriss (außen, innen = Schlitz, Mitte) – MOD1 `ButtonHole::ButtonHole(int, AlphaCharacter)`
//! 0x100257b0 mit den Namensprüfungen 0x10026eb0/0x10026fd0/0x100270f0 (hoch).
//!
//! **Länge ändern** (MOD1 0x10026680/0x10026970, hoch; am Musterdesign `Samples/Reference/Buttonholes.ART` bis auf
//! 0,01 mm bestätigt): Vorlage so drehen, dass der Schlitz längs liegt; Δ = neue − alte Schlitzlänge; Raupenteile
//! werden in Schlitzrichtung auf ihr umschließendes Rechteck + Δ gestreckt, obere Teile um Δ verschoben, untere
//! bleiben (bezogen auf den unteren Schlitzpunkt).
//! **Raupenabstand** (`ButtonHole::SetValues` 0x10027480, hoch): gilt nur für die Satinteile `<B…>`; die Enden behalten
//! den Abstand der Vorlage (0,5 mm).
//! **Standardwerte** (`AllButtonHolesValues` 0x10028060, hoch): Typ Standard (0x3ff9), Schlitz 3600 = 20,0 mm,
//! Raupenabstand 90 = 0,5 mm, Winkel π/2 (senkrecht).

use crate::design::{Objekt, ObjektArt, SatinOptionen, Unterlage};
use crate::geometrie::{Linie, Punkt, abstand, p};

/// Standard-Schlitzlänge in mm (MOD1 0x100280b7: 0xe10 = 3600 Einheiten)
pub const STANDARD_LAENGE: f64 = 20.0;
/// Standard-Raupenabstand in mm (MOD1 0x100280bd: 0x5a = 90 Einheiten)
pub const STANDARD_ABSTAND: f64 = 0.5;
/// Standardwinkel in Grad (MOD1 0x10028083: π/2)
pub const STANDARD_WINKEL: f64 = 90.0;
/// Erlaubte Schlitzlänge in mm – **eigene Festlegung** (Grenzen des Dialogs nicht gelesen)
pub const LAENGE_BEREICH: (f64, f64) = (3.0, 60.0);
/// Erlaubter Raupenabstand in mm – **eigene Festlegung** (Handbuch zeigt 0,3 / 0,4 / 1,0 mm)
pub const ABSTAND_BEREICH: (f64, f64) = (0.2, 2.0);

/// Die 9 Typen: (Wert im JSON, Name der Vorlage in ButtonHoles.ESB, Typnummer im Original, Name im Dialog).
/// Reihenfolge wie die Liste „Buttonhole Type“ (BERESENU); Typnummer ↔ Vorlage: MOD1 0x10027b40.
pub const TYPEN: [(&str, &str, u32, &str); 9] = [
    ("standard", "Standard", 0x3ff9, "Standard"),
    ("standard_schmal", "StandardNarrow", 0x3ffa, "Standard Narrow"),
    ("rund", "Round", 0x3ffb, "Round"),
    ("dehnbar", "Stretch", 0x4000, "Stretch"),
    ("heirloom", "Heirloom", 0x4001, "Heirloom"),
    ("rundes_ende", "RoundEnd", 0x3ffc, "Round End"),
    ("auge_eckig", "KeyholeSquare", 0x3ffd, "Keyhole Square"),
    ("auge_rund", "KeyholeRound", 0x3ffe, "Keyhole Round"),
    ("auge_spitz", "KeyholePoint", 0x3fff, "Keyhole Point"),
];

/// Name der Vorlage zu einem JSON-Wert (`standard` → `Standard`).
pub fn vorlage_name(typ: &str) -> Option<&'static str> {
    TYPEN.iter().find(|t| t.0 == typ).map(|t| t.1)
}

/// Lage eines Teils (Name `<L>`, `<U>`, `<B>`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lage {
    Unten,
    Oben,
    Raupe,
}

/// Stichteil einer Vorlage (Koordinaten in mm, Ursprung = unterer Schlitzpunkt, Schlitz längs +y).
#[derive(Clone, Debug, PartialEq)]
pub enum TeilArt {
    /// ManualObject (0x0b): jeder Punkt ist ein Einstich (Vernähen, Riegel)
    Handstiche(Linie),
    /// EmbroideryCurve (0x05) mit Lauf 0x271c: `ecken` = Knoten ohne Anfasser
    Lauf {
        punkte: Linie,
        ecken: Vec<bool>,
        laenge: Option<f64>,
    },
    /// InputCCurve (0x0c) mit zwei Schienen: Satinsäule
    Satin {
        seite_a: Linie,
        seite_b: Linie,
        /// fester Abstand der Vorlage (0x15)
        abstand: Option<f64>,
        /// Zugausgleich der Vorlage je Seite (0x8c, 31 Einheiten = 0,17 mm; am Muster gemessen)
        zugausgleich: Option<f64>,
        /// ShortStitchesEffect (0x23) vorhanden
        kurzstiche: bool,
        unterlagen: Vec<String>,
    },
    /// Säule mit Blanket 0x1a (Stichart 0x2727, Heirloom): Rücken auf einer Schiene, Zähne bis zur anderen
    Blanket {
        ruecken: Linie,
        aussen: Linie,
        /// Zahnabstand der Vorlage (0x15; Heirloom 270 = 1,5 mm)
        abstand: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Teil {
    pub lage: Lage,
    pub art: TeilArt,
}

/// Umriss-Linie einer Vorlage (`<LE>`, `<BI>`, `<UM>` …)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UmrissArt {
    Aussen,
    Innen,
    Mitte,
}

/// Eine Knopfloch-Vorlage (ein Zeichen von ButtonHoles.ESB).
#[derive(Clone, Debug, PartialEq)]
pub struct Vorlage {
    pub name: String,
    /// Schlitzlänge der Vorlage in mm (Grundlinie 0x2e)
    pub laenge: f64,
    /// Stichteile in Stickreihenfolge
    pub teile: Vec<Teil>,
    pub umrisse: Vec<(UmrissArt, Lage, Linie)>,
}

/// Werte eines Knopflochs im Design (Object Properties → Buttonhole).
#[derive(Clone, Debug, PartialEq)]
pub struct KnopflochVorgabe {
    /// Name der Vorlage („Standard“)
    pub vorlage: String,
    /// Mitte des Schlitzes in mm
    pub mitte: Punkt,
    /// Schlitzlänge in mm („Slit Length“)
    pub laenge: f64,
    /// Raupenabstand in mm („Bead Stitch Spacing“); None = wie die Vorlage (Musterdesign: Standard 0,5, Heirloom 1,5)
    pub abstand: Option<f64>,
    /// Richtung des Schlitzes in Grad, 90 = senkrecht („Angle“)
    pub winkel: f64,
}

impl Default for KnopflochVorgabe {
    fn default() -> Self {
        KnopflochVorgabe {
            vorlage: "Standard".into(),
            mitte: p(0.0, 0.0),
            laenge: STANDARD_LAENGE,
            abstand: None,
            winkel: STANDARD_WINKEL,
        }
    }
}

impl KnopflochVorgabe {
    pub fn pruefen(&self) -> Result<(), String> {
        if !(LAENGE_BEREICH.0..=LAENGE_BEREICH.1).contains(&self.laenge) {
            return Err(format!(
                "Knopfloch: Schlitzlänge {} mm außerhalb {}–{} mm",
                self.laenge, LAENGE_BEREICH.0, LAENGE_BEREICH.1
            ));
        }
        if let Some(a) = self.abstand
            && !(ABSTAND_BEREICH.0..=ABSTAND_BEREICH.1).contains(&a)
        {
            return Err(format!(
                "Knopfloch: Raupenabstand {a} mm außerhalb {}–{} mm",
                ABSTAND_BEREICH.0, ABSTAND_BEREICH.1
            ));
        }
        Ok(())
    }
}

fn teil_punkte(a: &TeilArt) -> Vec<Punkt> {
    match a {
        TeilArt::Handstiche(l) | TeilArt::Lauf { punkte: l, .. } => l.clone(),
        TeilArt::Satin { seite_a, seite_b, .. } => seite_a.iter().chain(seite_b).copied().collect(),
        TeilArt::Blanket { ruecken, aussen, .. } => ruecken.iter().chain(aussen).copied().collect(),
    }
}

/// Abbildung Vorlage → Design für ein Teil: strecken/verschieben wie MOD1 0x10026970, dann um die Schlitzmitte
/// drehen und an die Position setzen.
struct Abbildung {
    d: f64,
    halbe: f64,
    sin: f64,
    cos: f64,
    mitte: Punkt,
}

impl Abbildung {
    fn neu(v: &Vorlage, g: &KnopflochVorgabe) -> Self {
        let w = (g.winkel - 90.0).to_radians();
        Abbildung {
            d: g.laenge - v.laenge,
            halbe: g.laenge / 2.0,
            sin: w.sin(),
            cos: w.cos(),
            mitte: g.mitte,
        }
    }

    /// Funktion für ein Teil der Lage `lage` mit Ausdehnung `ymin..ymax` (Rechteck der Raupe)
    fn fuer(&self, lage: Lage, ymin: f64, ymax: f64) -> impl Fn(Punkt) -> Punkt + '_ {
        move |q: Punkt| {
            let y = match lage {
                Lage::Unten => q.y,
                Lage::Oben => q.y + self.d,
                Lage::Raupe if ymax - ymin > 1e-9 => ymin + (q.y - ymin) * (ymax - ymin + self.d) / (ymax - ymin),
                Lage::Raupe => q.y + self.d / 2.0,
            };
            let (x, y) = (q.x, y - self.halbe);
            p(
                self.mitte.x + x * self.cos - y * self.sin,
                self.mitte.y + x * self.sin + y * self.cos,
            )
        }
    }
}

fn ausdehnung(pts: &[Punkt]) -> (f64, f64) {
    pts.iter()
        .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)))
}

/// Einzelne Umrisslinien zu einem geschlossenen Ring verketten (nächstes Ende zuerst).
fn ring(mut stuecke: Vec<Linie>) -> Option<Linie> {
    stuecke.retain(|s| s.len() >= 2);
    if stuecke.is_empty() {
        return None;
    }
    let mut out = stuecke.remove(0);
    while !stuecke.is_empty() {
        let ende = *out.last().unwrap();
        let (k, rueck) = (0..stuecke.len())
            .flat_map(|k| [(k, false), (k, true)])
            .min_by(|&(a, ra), &(b, rb)| {
                let pa = if ra { *stuecke[a].last().unwrap() } else { stuecke[a][0] };
                let pb = if rb { *stuecke[b].last().unwrap() } else { stuecke[b][0] };
                abstand(ende, pa).total_cmp(&abstand(ende, pb))
            })
            .unwrap();
        let mut s = stuecke.remove(k);
        if rueck {
            s.reverse();
        }
        out.extend(s.into_iter().skip_while(|q| abstand(*q, ende) < 1e-6));
    }
    if out.len() >= 3 { Some(out) } else { None }
}

/// Knopfloch → Objekt des Rechenkerns (`ObjektArt::Kette`: Teile in Reihenfolge der Vorlage, ohne Sprünge
/// verbunden; `loch` = mittlerer Umriss zum Einarbeiten in darunterliegende Flächen).
pub fn knopfloch_objekt(v: &Vorlage, g: &KnopflochVorgabe, farbe: &str, name: &str) -> Objekt {
    let ab = Abbildung::neu(v, g);
    let mut teile = Vec::new();
    let mut umkehrbar = Vec::new();
    for t in &v.teile {
        let (ymin, ymax) = ausdehnung(&teil_punkte(&t.art));
        let f = ab.fuer(t.lage, ymin, ymax);
        let lin = |l: &Linie| -> Linie { l.iter().map(|&q| f(q)).collect() };
        let art = match &t.art {
            TeilArt::Handstiche(l) => ObjektArt::Lauf {
                punkte: lin(l),
                ecken: vec![true; l.len()],
                laenge: Some(crate::einheiten::MAX_STICH_MM),
                dreifach: false,
                geschlossen: false,
            },
            TeilArt::Lauf { punkte, ecken, laenge } => ObjektArt::Lauf {
                punkte: lin(punkte),
                ecken: ecken.clone(),
                laenge: *laenge,
                dreifach: false,
                geschlossen: false,
            },
            TeilArt::Satin {
                seite_a,
                seite_b,
                abstand,
                zugausgleich,
                kurzstiche,
                unterlagen,
            } => ObjektArt::Satin {
                seite_a: lin(seite_a),
                seite_b: lin(seite_b),
                unterlage: Unterlage::Arten(unterlagen.clone()),
                optionen: SatinOptionen {
                    abstand: if t.lage == Lage::Raupe {
                        g.abstand.or(*abstand).or(Some(STANDARD_ABSTAND))
                    } else {
                        abstand.or(Some(STANDARD_ABSTAND))
                    },
                    kurzstiche: Some(*kurzstiche),
                    zugausgleich: *zugausgleich,
                    ..Default::default()
                },
            },
            TeilArt::Blanket {
                ruecken,
                aussen,
                abstand,
            } => {
                let s = if t.lage == Lage::Raupe {
                    g.abstand.or(*abstand)
                } else {
                    *abstand
                };
                let punkte = blanket_punkte(&lin(ruecken), &lin(aussen), s.unwrap_or(1.5));
                let n = punkte.len();
                ObjektArt::Lauf {
                    punkte,
                    ecken: vec![true; n],
                    laenge: Some(crate::einheiten::MAX_STICH_MM),
                    dreifach: false,
                    geschlossen: false,
                }
            }
        };
        umkehrbar.push(matches!(t.art, TeilArt::Satin { .. } | TeilArt::Blanket { .. }));
        teile.push(Objekt {
            farbe: farbe.to_string(),
            name: name.to_string(),
            art,
        });
    }
    // mittlerer Umriss (Merge Buttonhole – Zuordnung **niedrig**, Bericht 19 §5): je Linie wie ihr Teil abbilden
    let raupen_pts: Vec<Punkt> = v
        .umrisse
        .iter()
        .filter(|(a, l, _)| *a == UmrissArt::Mitte && *l == Lage::Raupe)
        .flat_map(|(_, _, l)| l.iter().copied())
        .collect();
    let (rmin, rmax) = ausdehnung(&raupen_pts);
    let mitte: Vec<Linie> = v
        .umrisse
        .iter()
        .filter(|(a, _, _)| *a == UmrissArt::Mitte)
        .map(|(_, lage, l)| {
            let (ymin, ymax) = if *lage == Lage::Raupe {
                (rmin, rmax)
            } else {
                ausdehnung(l)
            };
            let f = ab.fuer(*lage, ymin, ymax);
            l.iter().map(|&q| f(q)).collect()
        })
        .collect();
    Objekt {
        farbe: farbe.to_string(),
        name: name.to_string(),
        art: ObjektArt::Kette {
            teile,
            umkehrbar,
            loch: ring(mitte),
        },
    }
}

/// Blanket-Raupe: Zähne vom Rücken zur Außenschiene, Abstand auf die Länge verteilt (Musterdesign Heirloom: 1,5 mm
/// → 1,46–1,48 mm). Je Stelle Rücken → außen → Rücken, dann längs des Rückens weiter (wie im Muster gemessen).
fn blanket_punkte(ruecken: &[Punkt], aussen: &[Punkt], abstand: f64) -> Linie {
    use crate::geometrie::{kumuliert, punkt_bei};
    if ruecken.len() < 2 || aussen.len() < 2 {
        return ruecken.to_vec();
    }
    let (kr, ka) = (kumuliert(ruecken), kumuliert(aussen));
    let (lr, la) = (*kr.last().unwrap(), *ka.last().unwrap());
    let n = ((lr / abstand.max(0.1)).round() as usize).max(1);
    let mut out = Vec::new();
    for k in 0..=n {
        let t = k as f64 / n as f64;
        let r = punkt_bei(ruecken, t * lr, &kr);
        out.extend([r, punkt_bei(aussen, t * la, &ka), r]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vereinfachte Vorlage wie „Standard“: Länge 19,36, Raupe von −0,33 bis 19,56, Riegel oben bei 19,6
    fn vorlage() -> Vorlage {
        Vorlage {
            name: "Test".into(),
            laenge: 19.36,
            teile: vec![
                Teil {
                    lage: Lage::Unten,
                    art: TeilArt::Handstiche(vec![p(-0.4, -0.3), p(-1.4, -0.3)]),
                },
                Teil {
                    lage: Lage::Raupe,
                    art: TeilArt::Satin {
                        seite_a: vec![p(-2.54, -0.33), p(-2.54, 19.56)],
                        seite_b: vec![p(-0.54, -0.33), p(-0.54, 19.56)],
                        abstand: Some(0.5),
                        zugausgleich: Some(31.0 / 180.0),
                        kurzstiche: true,
                        unterlagen: vec![],
                    },
                },
                Teil {
                    lage: Lage::Oben,
                    art: TeilArt::Handstiche(vec![p(2.89, 19.57), p(-2.56, 19.6)]),
                },
            ],
            umrisse: vec![
                (UmrissArt::Mitte, Lage::Raupe, vec![p(-1.6, -0.3), p(-1.6, 19.7)]),
                (UmrissArt::Mitte, Lage::Oben, vec![p(-1.6, 19.7), p(1.7, 19.7)]),
                (UmrissArt::Mitte, Lage::Raupe, vec![p(1.7, 19.7), p(1.7, -0.3)]),
                (UmrissArt::Mitte, Lage::Unten, vec![p(1.7, -0.3), p(-1.6, -0.3)]),
            ],
        }
    }

    #[test]
    fn laenge_wie_original() {
        // senkrecht, Schlitzmitte (0, 10) → unterer Schlitzpunkt (0, 0)
        let g = KnopflochVorgabe {
            mitte: p(0.0, 10.0),
            ..Default::default()
        };
        let o = knopfloch_objekt(&vorlage(), &g, "#000000", "K");
        let ObjektArt::Kette { teile, loch, .. } = o.art else {
            panic!()
        };
        let ObjektArt::Lauf { punkte, .. } = &teile[0].art else {
            panic!()
        };
        assert!((punkte[0].y + 0.3).abs() < 1e-9, "unten bleibt");
        let ObjektArt::Satin { seite_a, optionen, .. } = &teile[1].art else {
            panic!()
        };
        // Raupe: unten fest, oben + Δ (0,64) – im Musterdesign 19,56 → 20,20
        assert!((seite_a[0].y + 0.33).abs() < 1e-9 && (seite_a[1].y - 20.20).abs() < 1e-9);
        assert_eq!(optionen.abstand, Some(0.5), "ohne Angabe wie die Vorlage");
        let ObjektArt::Lauf { punkte, .. } = &teile[2].art else {
            panic!()
        };
        assert!((punkte[0].y - 20.21).abs() < 1e-9, "oben um Δ");
        assert!(loch.is_some_and(|l| l.len() >= 4));
    }

    #[test]
    fn winkel_und_abstand() {
        let g = KnopflochVorgabe {
            winkel: 0.0,
            abstand: Some(0.3),
            laenge: 19.36,
            ..Default::default()
        };
        let o = knopfloch_objekt(&vorlage(), &g, "#000000", "K");
        let ObjektArt::Kette { teile, .. } = o.art else {
            panic!()
        };
        let ObjektArt::Satin { seite_a, optionen, .. } = &teile[1].art else {
            panic!()
        };
        // waagrecht: Schlitz längs +x, unterer Punkt bei x = −9,68
        assert!((seite_a[0].x - (-0.33 - 9.68)).abs() < 1e-9 && (seite_a[0].y - 2.54).abs() < 1e-9);
        assert_eq!(optionen.abstand, Some(0.3));
        assert!(g.pruefen().is_ok());
        assert!(KnopflochVorgabe { laenge: 1.0, ..g }.pruefen().is_err());
    }
}
