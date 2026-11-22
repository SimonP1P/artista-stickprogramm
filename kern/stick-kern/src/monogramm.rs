//! **Monogramm mit Rahmen aus dem Katalog** – eigene Erweiterung (Nutzer 21.11.2026).
//!
//! Das Original kennt nur die Monogramm-Alphabete Diamond2/Diamond3 mit Raute bzw. Zierwinkel (Handbuch Kap. 21,
//! `design.rs` `monogramm_als_text`); sein Tipp: Rahmen und Schrift als zwei Objekte zusammensetzen (Handbuch PDF-S. 192
//! „Use a monogram border around normal lettering“). Das macht dieses Objekt in einem Schritt:
//!
//! - **Rahmen** aus dem Katalog (`stick-vektor` `monogramm.rs`: 47 SVG-Vorlagen in 8 Kategorien, Linien 2–4 mm →
//!   Satinkanten nach EasyStitch wie „SVG einlesen“), Mitte der Zeichnung im Nullpunkt; füllt der Aufrufer
//!   (`Design::monogramm_rahmen`).
//! - **Initialen** (1–3) in einer Computer-Schrift (ohne Angabe: Schriftvorschlag der Vorlage) oder einem
//!   Original-Alphabet. Höhen (Versalhöhe) aus der Vorlage, nebeneinander, senkrecht mittig zwischen den Rahmenteilen
//!   über und unter der Mitte, und **gemeinsam verkleinert**, bis rundum `ABSTAND` frei bleibt.
//! - Reihenfolge: erst die Initialen, dann der Rahmen (wie bei Diamond: das Rahmenzeichen steht hinter den
//!   Buchstaben), Rahmenteile nach Farbe – die Farbe der Initialen zuerst, dann wie in der Vorlage.
//!
//! Alle Maße hier sind **eigene Festlegung**.

use crate::aussparen::{Kontur, belegt, belegt_mit_rand, schnittflaeche};
use crate::design::Objekt;
use crate::geometrie::{Punkt, p};
use crate::schrift::TextVorgabe;
use crate::transformation::{Matrix, abbilden};

/// Lücke zwischen den Initialen in Teilen der größten Höhe – eigene Festlegung
pub const LUECKE_ANTEIL: f64 = 0.08;
/// Freier Rand zwischen Initialen und Rahmen in mm – eigene Festlegung
pub const ABSTAND: f64 = 2.0;
/// Weiter als auf diesen Anteil der Vorlage werden die Initialen nicht verkleinert – eigene Festlegung
pub const MIN_ANTEIL: f64 = 0.3;
/// Seitliche Initialen, wenn eine Vorlage für 2 Buchstaben 3 bekommt: Anteil der Höhe (Mittel der Vorlagen für 3
/// Buchstaben, 17/25 ≈ 0,68) – eigene Festlegung
pub const SEITE_ANTEIL: f64 = 0.68;

/// Rahmen aus dem Katalog, geladen vom Aufrufer.
#[derive(Clone, Debug, PartialEq)]
pub struct RahmenVorlage {
    pub name: String,
    /// Teile des Rahmens in mm, Mitte der Zeichnung im Nullpunkt, Farben der Vorlage
    pub teile: Vec<Objekt>,
    /// Versalhöhen der Initialen in mm bei gezeichneter Größe: links, rechts bzw. links, Mitte, rechts
    pub hoehen: Vec<f64>,
    /// Schriftvorschlag der Vorlage (Google Fonts)
    pub schrift: String,
}

impl RahmenVorlage {
    /// Größe wie gezeichnet: längere Seite der Fläche, die die Teile belegen (mm).
    pub fn groesse(&self) -> f64 {
        let k: Vec<Kontur> = self.teile.iter().flat_map(|o| belegt(&o.art)).collect();
        let [x0, y0, x1, y1] = begrenzung(&k).unwrap_or([0.0, 0.0, 0.0, 0.0]);
        (x1 - x0).max(y1 - y0)
    }

    /// Farben der Vorlage in der Reihenfolge ihres ersten Auftretens.
    pub fn farben(&self) -> Vec<String> {
        let mut f: Vec<String> = Vec::new();
        for o in &self.teile {
            if !f.iter().any(|x| x.eq_ignore_ascii_case(&o.farbe)) {
                f.push(o.farbe.clone());
            }
        }
        f
    }
}

/// Schrift der Initialen.
#[derive(Clone, Debug, PartialEq)]
pub enum InitialenSchrift {
    /// Computer-Schrift (None = Schriftvorschlag der Vorlage)
    Ttf {
        schrift: Option<String>,
        stichart: crate::schrift_ttf::TtfStichart,
    },
    /// Original-Alphabet; Stichart wie bei Text (`TextZusatz::stichart`, None = Satin)
    Alphabet {
        alphabet: String,
        stichart: Option<crate::schrift::SchriftStichart>,
    },
}

/// Monogramm mit Rahmen aus dem Katalog (JSON `monogramm` mit `vorlage`).
#[derive(Clone, Debug, PartialEq)]
pub struct MonogrammVorgabe {
    pub vorlage: String,
    pub initialen: Vec<char>,
    pub schrift: InitialenSchrift,
    /// längere Seite des Rahmens in mm (None = wie gezeichnet)
    pub rahmen_groesse: Option<f64>,
    /// Farben statt der 1., 2., 3. Farbe der Vorlage
    pub rahmen_farben: Vec<String>,
    /// Vorgabe für jede Initiale (Breite, Kursiv, Unterlage, Farbe; Mitte des Monogramms = `position`)
    pub text: TextVorgabe,
}

impl MonogrammVorgabe {
    /// Computer-Schrift der Initialen (bei Alphabet None).
    pub fn ttf_schrift<'a>(&'a self, r: Option<&'a RahmenVorlage>) -> Option<&'a str> {
        match &self.schrift {
            InitialenSchrift::Ttf { schrift: Some(s), .. } => Some(s),
            InitialenSchrift::Ttf { schrift: None, .. } => r.map(|r| r.schrift.as_str()),
            InitialenSchrift::Alphabet { .. } => None,
        }
    }
}

/// Höhen für `n` Initialen aus den Höhen der Vorlage (2 Werte: links/rechts, 3: links/Mitte/rechts). Passt die Anzahl
/// nicht: 1 Buchstabe = größte Höhe; 3 auf einer Vorlage für 2 = seitlich `SEITE_ANTEIL`; 2 auf einer für 3 = Mitte.
pub fn hoehen(n: usize, vorlage: &[f64]) -> Vec<f64> {
    let gross = vorlage.iter().copied().fold(0.0, f64::max);
    match (n, vorlage.len()) {
        (1, _) => vec![gross],
        (a, b) if a == b => vorlage.to_vec(),
        (3, _) => vec![gross * SEITE_ANTEIL, gross, gross * SEITE_ANTEIL],
        _ => vec![gross; n],
    }
}

/// Begrenzung [x0, y0, x1, y1] von Konturen.
fn begrenzung(k: &[Kontur]) -> Option<[f64; 4]> {
    let mut b: Option<[f64; 4]> = None;
    for q in k.iter().flatten() {
        let x = b.get_or_insert([q[0], q[1], q[0], q[1]]);
        *x = [x[0].min(q[0]), x[1].min(q[1]), x[2].max(q[0]), x[3].max(q[1])];
    }
    b
}

fn verschiebung(dx: f64, dy: f64) -> Matrix {
    Matrix {
        e: dx,
        f: dy,
        ..Matrix::EINS
    }
}

/// Skalieren um `mitte` mit `k`.
fn skalierung(mitte: Punkt, k: f64) -> Matrix {
    Matrix {
        a: k,
        d: k,
        e: mitte.x * (1.0 - k),
        f: mitte.y * (1.0 - k),
        ..Matrix::EINS
    }
}

fn konturen_abbilden(k: &[Kontur], m: &Matrix) -> Vec<Kontur> {
    k.iter()
        .map(|r| {
            r.iter()
                .map(|q| {
                    let x = m.punkt(p(q[0], q[1]));
                    [x.x, x.y]
                })
                .collect()
        })
        .collect()
}

/// Senkrechte Mitte des Innenraums: auf der Mittelachse (x = 0) der nächste Rahmenrand über und unter der Mitte;
/// fehlt einer (offener Rahmen, z. B. Lorbeer), bleibt die Mitte 0.
fn innen_mitte(rahmen: &[Kontur]) -> f64 {
    let (mut oben, mut unten) = (f64::INFINITY, f64::NEG_INFINITY);
    for r in rahmen {
        for i in 0..r.len() {
            let (a, b) = (r[i], r[(i + 1) % r.len()]);
            if (a[0] <= 0.0) == (b[0] <= 0.0) || (b[0] - a[0]).abs() < 1e-12 {
                continue;
            }
            let y = a[1] + (b[1] - a[1]) * (0.0 - a[0]) / (b[0] - a[0]);
            if y > 0.0 {
                oben = oben.min(y);
            } else {
                unten = unten.max(y);
            }
        }
    }
    if oben.is_finite() && unten.is_finite() {
        (oben + unten) / 2.0
    } else {
        0.0
    }
}

/// Monogramm auflösen. `setzen` setzt einen Text (eine Initiale) in der Schrift der Initialen: (Objekte, fehlende
/// Zeichen, Nummer des Buchstabens je Objekt). `winkel_standard` = Stichwinkel des Stils (für `abbilden`).
/// Ergebnis: Objekte (erst die Initialen, dann der Rahmen), je Objekt die Nummer der Initiale (1 … n, Rahmen 0),
/// fehlende Zeichen und der Anteil, auf den die Initialen verkleinert wurden (1 = Höhen der Vorlage).
pub fn aufloesen(
    r: &RahmenVorlage,
    m: &MonogrammVorgabe,
    name: &str,
    winkel_standard: f64,
    setzen: impl Fn(&TextVorgabe) -> (Vec<Objekt>, Vec<char>, Vec<usize>),
) -> Result<(Vec<Objekt>, Vec<usize>, Vec<char>, f64), String> {
    let gezeichnet = r.groesse();
    if gezeichnet <= 0.0 {
        return Err(format!("Monogramm-Rahmen „{}“ ist leer", r.name));
    }
    let f = m.rahmen_groesse.map(|g| g / gezeichnet).unwrap_or(1.0);
    if !(f > 0.0 && f.is_finite()) {
        return Err("rahmen_groesse: größer als 0".into());
    }

    // Rahmen: skaliert, Farben ersetzt
    let farben = r.farben();
    let groesser = skalierung(p(0.0, 0.0), f);
    let rahmen: Vec<Objekt> = r
        .teile
        .iter()
        .map(|o| {
            let mut x = abbilden(o, &groesser, winkel_standard);
            if let Some(i) = farben.iter().position(|c| c.eq_ignore_ascii_case(&o.farbe))
                && let Some(c) = m.rahmen_farben.get(i).filter(|c| !c.is_empty())
            {
                x.farbe = c.clone();
            }
            x.name = format!("{name} (Rahmen)");
            x
        })
        .collect();
    let zone = belegt_mit_rand(&rahmen.iter().map(|o| &o.art).collect::<Vec<_>>(), ABSTAND);
    let rahmen_flaeche: Vec<Kontur> = rahmen.iter().flat_map(|o| belegt(&o.art)).collect();
    let mitte_y = innen_mitte(&rahmen_flaeche);

    // Initialen in Vorlagenhöhe nebeneinander, Versalmitte auf y = 0
    let hs: Vec<f64> = hoehen(m.initialen.len(), &r.hoehen).into_iter().map(|h| h * f).collect();
    let luecke = LUECKE_ANTEIL * hs.iter().copied().fold(0.0, f64::max);
    let mut buchstaben: Vec<(Vec<Objekt>, [f64; 4])> = Vec::new();
    let mut fehlend = Vec::new();
    for (ch, &h) in m.initialen.iter().zip(&hs) {
        let v = TextVorgabe {
            text: ch.to_string(),
            hoehe: h,
            position: p(0.0, -h / 2.0),
            ausrichtung: 0.5,
            grundlinie: None,
            ..m.text.clone()
        };
        let (obj, f2, _) = setzen(&v);
        fehlend.extend(f2);
        let k: Vec<Kontur> = obj.iter().flat_map(|o| belegt(&o.art)).collect();
        if let Some(b) = begrenzung(&k) {
            buchstaben.push((obj, b));
        }
    }
    let breite: f64 = buchstaben.iter().map(|(_, b)| b[2] - b[0]).sum::<f64>()
        + luecke * buchstaben.len().saturating_sub(1) as f64;
    let mut x = -breite / 2.0;
    let mut initialen: Vec<(Objekt, usize)> = Vec::new();
    for (i, (obj, b)) in buchstaben.into_iter().enumerate() {
        let m = verschiebung(x - b[0], mitte_y);
        initialen.extend(obj.iter().map(|o| (abbilden(o, &m, winkel_standard), i + 1)));
        x += b[2] - b[0] + luecke;
    }

    // gemeinsam verkleinern, bis die Initialen den Rand um den Rahmen nicht mehr berühren (Halbierung)
    let flaeche: Vec<Kontur> = initialen.iter().flat_map(|(o, _)| belegt(&o.art)).collect();
    let zentrum = p(0.0, mitte_y);
    let passt = |k: f64| schnittflaeche(&konturen_abbilden(&flaeche, &skalierung(zentrum, k)), &zone) < 0.01;
    let anteil = if zone.is_empty() || passt(1.0) {
        1.0
    } else if !passt(MIN_ANTEIL) {
        MIN_ANTEIL
    } else {
        let (mut lo, mut hi) = (MIN_ANTEIL, 1.0);
        for _ in 0..12 {
            let mid = (lo + hi) / 2.0;
            if passt(mid) { lo = mid } else { hi = mid }
        }
        lo
    };

    // Lage: Mitte des Monogramms = position
    let pos = m.text.position;
    let hin = verschiebung(pos.x, pos.y);
    let klein = skalierung(zentrum, anteil);
    let mut out: Vec<Objekt> = Vec::new();
    let mut nummern = Vec::new();
    for (o, nr) in &initialen {
        let o = if anteil < 1.0 { abbilden(o, &klein, winkel_standard) } else { o.clone() };
        out.push(abbilden(&o, &hin, winkel_standard));
        nummern.push(*nr);
    }
    // Rahmenteile nach Farbe: die der Initialen zuerst, dann in der Reihenfolge der Vorlage; je Farbe immer das
    // nächstgelegene Teil (Mitte zu Mitte, wie „Closest Join“ – eigene Festlegung), Start bei der letzten Initiale
    let farbe_initialen = m.text.farbe.to_ascii_lowercase();
    let mut reihen: Vec<String> = vec![farbe_initialen];
    for o in &rahmen {
        let c = o.farbe.to_ascii_lowercase();
        if !reihen.contains(&c) {
            reihen.push(c);
        }
    }
    let mitte = |o: &Objekt| crate::transformation::mitte(std::slice::from_ref(o));
    let mut hier = out.last().map(mitte).unwrap_or(pos);
    for c in &reihen {
        let mut offen: Vec<Objekt> = rahmen
            .iter()
            .filter(|o| o.farbe.to_ascii_lowercase() == *c)
            .map(|o| abbilden(o, &hin, winkel_standard))
            .collect();
        while !offen.is_empty() {
            let (i, _) = offen
                .iter()
                .enumerate()
                .map(|(i, o)| (i, crate::geometrie::abstand(mitte(o), hier)))
                .fold((0, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
            let o = offen.remove(i);
            hier = mitte(&o);
            out.push(o);
            nummern.push(0);
        }
    }
    Ok((out, nummern, fehlend, anteil))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::ObjektArt;

    #[test]
    fn hoehen_nach_anzahl() {
        assert_eq!(hoehen(3, &[17.0, 27.0, 17.0]), vec![17.0, 27.0, 17.0]);
        assert_eq!(hoehen(2, &[26.0, 26.0]), vec![26.0, 26.0]);
        assert_eq!(hoehen(1, &[17.0, 27.0, 17.0]), vec![27.0]);
        assert_eq!(hoehen(2, &[17.0, 27.0, 17.0]), vec![27.0, 27.0]);
        let h = hoehen(3, &[25.0, 25.0]);
        assert!((h[0] - 17.0).abs() < 0.01 && h[1] == 25.0, "{h:?}");
    }

    #[test]
    fn innenraum_mitte() {
        // Ring oben bei y = 30 … 32, unten bei −22 … −20 → Mitte 5
        let band = |y0: f64, y1: f64| vec![[-40.0, y0], [40.0, y0], [40.0, y1], [-40.0, y1]];
        assert!((innen_mitte(&[band(30.0, 32.0), band(-22.0, -20.0)]) - 5.0).abs() < 1e-9);
        // nur seitliche Teile: Mitte 0
        let seite = vec![[30.0, -20.0], [34.0, -20.0], [34.0, 20.0], [30.0, 20.0]];
        assert_eq!(innen_mitte(&[seite]), 0.0);
    }

    /// Kreisrahmen r = 30, Initialen aus Quadraten: zu große Höhen werden verkleinert, bis 2 mm frei bleiben
    #[test]
    fn initialen_passen_in_den_rahmen() {
        let kreis: Vec<Punkt> = (0..=120)
            .map(|i| {
                let w = i as f64 / 120.0 * std::f64::consts::TAU;
                p(30.0 * w.cos(), 30.0 * w.sin())
            })
            .collect();
        let r = RahmenVorlage {
            name: "Kreis".into(),
            teile: vec![Objekt {
                farbe: "#222222".into(),
                name: String::new(),
                art: ObjektArt::SatinLinie {
                    punkte: kreis,
                    breite: 3.0,
                    unterlage: crate::design::Unterlage::Auto,
                    mittig: true,
                    optionen: Default::default(),
                },
            }],
            hoehen: vec![30.0, 30.0],
            schrift: "X".into(),
        };
        let m = MonogrammVorgabe {
            vorlage: "Kreis".into(),
            initialen: vec!['A', 'B'],
            schrift: InitialenSchrift::Alphabet {
                alphabet: "x".into(),
                stichart: None,
            },
            rahmen_groesse: None,
            rahmen_farben: vec!["#ff0000".into()],
            text: TextVorgabe {
                text: String::new(),
                hoehe: 10.0,
                breite_prozent: 100.0,
                abstand: None,
                position: p(10.0, 5.0),
                ausrichtung: 0.5,
                unterlage: None,
                farbe: "#000000".into(),
                wortabstand: None,
                zeilenabstand: None,
                grundlinie: None,
                kursiv: 0.0,
            },
        };
        // jede Initiale: Quadrat Höhe × Höhe auf der Grundlinie, mittig um position.x
        let setzen = |v: &TextVorgabe| {
            let (h, q) = (v.hoehe, v.position);
            let umriss = vec![p(q.x - h / 2.0, q.y), p(q.x + h / 2.0, q.y), p(q.x + h / 2.0, q.y + h), p(q.x - h / 2.0, q.y + h)];
            let o = Objekt {
                farbe: v.farbe.clone(),
                name: v.text.clone(),
                art: ObjektArt::Tatami {
                    umriss,
                    loecher: vec![],
                    winkel: None,
                    unterlage: None,
                    aussparen: Default::default(),
                    welle: None,
                },
            };
            (vec![o], vec![], vec![1])
        };
        let (out, nummern, _, anteil) = aufloesen(&r, &m, "AB", 0.0, setzen).unwrap();
        assert_eq!(nummern, vec![1, 2, 0]);
        // Innenrand r = 28,5, minus 2 mm frei → Ecke des Quadratpaars höchstens 26,5 vom Mittelpunkt (10, 5); Vieleck ±0,1
        let k: Vec<Kontur> = out[..2].iter().flat_map(|o| belegt(&o.art)).collect();
        let weiteste = k.iter().flatten().map(|q| (q[0] - 10.0).hypot(q[1] - 5.0)).fold(0.0, f64::max);
        assert!(anteil < 1.0 && weiteste <= 26.6 && weiteste > 26.0, "{anteil} {weiteste}");
        assert_eq!(out[2].farbe, "#ff0000");
        // Rahmen in neuer Größe: 60 → 30 mm Durchmesser (Mitte der Satinlinie)
        let m2 = MonogrammVorgabe { rahmen_groesse: Some(r.groesse() / 2.0), ..m };
        let (out2, _, _, _) = aufloesen(&r, &m2, "AB", 0.0, setzen).unwrap();
        let ObjektArt::SatinLinie { punkte, breite, .. } = &out2[2].art else { panic!() };
        assert!((punkte[0].x - 25.0).abs() < 0.01 && (breite - 1.5).abs() < 0.01, "{:?} {breite}", punkte[0]);
    }
}
