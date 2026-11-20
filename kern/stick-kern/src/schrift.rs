//! Schrift mit den Original-Alphabeten der BERNINA artista V4 (ESB, `Analyse_Teil3/09_esb_alphabete.md`).
//!
//! Ein Alphabet besteht aus Zeichen; jedes Zeichen hat eine Grundlinie und Objekte wie im Design des Originals –
//! meist Satinsäulen (EmbroideryPatch) aus einem geschlossenen Umriss mit **Winkellinien**. Das Lesen der Datei steckt
//! in `stick-formate::esb`; hier stehen das Modell, die Umwandlung der Säulen in die zwei Seiten des Rechenkerns und
//! der Satz eines Texts (Höhe, Breite, Buchstabenabstand, Position).

use crate::design::{Objekt, ObjektArt, SatinOptionen, Unterlage};
use crate::geometrie::{Linie, Punkt, p};
use std::collections::BTreeMap;

/// Einheit der Alphabete: 1 mm = 180 (Analyse/code/06)
pub const EINHEIT: f64 = 180.0;
/// Referenzhöhe: Die Zeichensätze im Maschinenspeicher sind alle genau im Maßstab 0,5 gestickt (auch Zurich mit
/// 61,5 mm hohem „H“) → für alle Alphabete gleiche Referenz, Großbuchstaben ≈ 50 mm (`Analyse_Teil3/09` §4,
/// mittel-hoch; im Code noch nicht bestätigt).
pub const REFERENZ_HOEHE_MM: f64 = 50.0;
/// Wortabstand (Leerzeichen) in Teilen der Buchstabenhöhe: Vorlage NORMAL.AMT, Schriftvorgaben 0x3e82 →
/// **0x3e85 = 0,25**; Lettering-Dialog „Word Spacing … % of Height“ (RESENU DLG Lettering); gleiches Feld in jedem
/// Schriftobjekt der Musterdesigns (Datensatz 0x7c, 0,25 oder 0,5). Mittel (Zuordnung über Reihenfolge im Dialog).
pub const WORTABSTAND_ANTEIL: f64 = 0.25;
/// Zeilenabstand (Lücke zwischen zwei Zeilen) in Teilen der Höhe: Vorlage **0x3f2d = 0,6**, Dialog „Line Spacing … %
/// of Height“; in den Musterdesigns 0,6 oder 0,5 am Ende des Datensatzes 0x7c. Mittel (ob Lücke oder Abstand der
/// Grundlinien: als Lücke gedeutet, weil 60 % als Grundlinienabstand Zeilen überlappen ließe).
pub const ZEILENABSTAND_ANTEIL: f64 = 0.6;
/// Standardhöhe einer neuen Schrift: Vorlage **0x3e88 = 1800 → 10 mm**.
pub const STANDARD_HOEHE_MM: f64 = 10.0;
/// Stil „Schrift“ (NORMAL.AMT, `Analyse_Teil3/05`, `Analyse/code/10`): Auto Spacing an mit Faktor 0,9 (0x2736;
/// in den Schrift-Musterdesigns 0,9 bzw. 0,87), Kurzstiche an (0x273a), Auto-Unterlage an mit vorgewählter Art
/// Zickzack (0x27a7) – Handbuch PDF-S. 186/379: Schrift hat Auto-Unterlage, bei kleiner Schrift abschalten.
pub const SCHRIFT_FAKTOR: f64 = 0.9;
pub const SCHRIFT_UNTERLAGE: &str = "zickzack";
/// Laufweg im Buchstaben, wenn das Alphabet keinen nennt (Drifter: Lauf 0x271c 540 = 3,0 mm)
pub const SCHRIFT_LAUFWEG: f64 = 3.0;

/// Bézier-Knoten in Alphabet-Einheiten (1/180 mm), Anfasser optional.
#[derive(Clone, Debug, PartialEq)]
pub struct Knoten {
    pub ein: Option<(f64, f64)>,
    pub punkt: (f64, f64),
    pub aus: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Umriss {
    pub geschlossen: bool,
    pub knoten: Vec<Knoten>,
}

/// Winkellinie einer Satinsäule: von Segment `a` beim Anteil `x` zu Segment `c` beim Anteil `y` des Umrisses
/// (IMPORT.DLL 0x10018970).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Winkellinie {
    pub a: u32,
    pub x: f64,
    pub c: u32,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stichart {
    /// Satin: fester Abstand in mm (0x15, z. B. 72 = 0,40 mm); hängt der Effekt Auto Spacing (0x21) am Objekt, gilt
    /// stattdessen die Tabelle × Faktor (`Analyse_Teil3/05` §1: vorhanden = aktiv)
    Satin {
        abstand: Option<f64>,
        auto_spacing: bool,
    },
    Tatami,
    /// Laufstich mit Stichlänge in mm
    Lauf {
        laenge: Option<f64>,
    },
    Unbekannt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZeichenObjekt {
    /// Klasse im Original (0x06 EmbroideryPatch, 0x05 EmbroideryCurve, 0x0c InputCCurve …)
    pub klasse: u32,
    pub umrisse: Vec<Umriss>,
    pub winkel: Vec<Winkellinie>,
    pub stichart: Stichart,
    /// Unterlagen des Objekts im Alphabet: mittellauf, edge_walk, zickzack, doppelzickzack
    pub unterlagen: Vec<String>,
    /// Stichlänge des Laufwegs (Lauf 0x271c in der Füllung), mm
    pub laufweg: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Zeichen {
    pub zeichen: char,
    /// Grundlinie x1, y1, x2, y2 (Alphabet-Einheiten); Vorschub = x2 − x1
    pub grundlinie: [f64; 4],
    pub objekte: Vec<ZeichenObjekt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alphabet {
    pub name: String,
    /// Buchstabenabstand in Teilen der Höhe (Kopf, float 2: Drifter 0,10) – „automatically as a percentage of letter
    /// height“ (Handbuch PDF-S. 168)
    pub abstand_anteil: f64,
    pub zeichen: BTreeMap<char, Zeichen>,
}

// ---------------------------------------------------------------- Geometrie der Umrisse

fn segmente(u: &Umriss) -> usize {
    if u.geschlossen {
        u.knoten.len()
    } else {
        u.knoten.len().saturating_sub(1)
    }
}

/// Punkt auf Segment `s` beim Anteil `t` (kubische Bézierkurve; fehlender Anfasser = Ankerpunkt).
pub fn segment_punkt(u: &Umriss, s: usize, t: f64) -> (f64, f64) {
    let n = u.knoten.len();
    let s = s.min(segmente(u).saturating_sub(1));
    let a = &u.knoten[s];
    let b = &u.knoten[(s + 1) % n];
    let (p0, p3) = (a.punkt, b.punkt);
    let p1 = a.aus.unwrap_or(p0);
    let p2 = b.ein.unwrap_or(p3);
    let k = [
        (1.0 - t).powi(3),
        3.0 * (1.0 - t).powi(2) * t,
        3.0 * (1.0 - t) * t * t,
        t.powi(3),
    ];
    (
        k[0] * p0.0 + k[1] * p1.0 + k[2] * p2.0 + k[3] * p3.0,
        k[0] * p0.1 + k[1] * p1.1 + k[2] * p2.1 + k[3] * p3.1,
    )
}

fn punkt_bei_parameter(u: &Umriss, pos: f64) -> (f64, f64) {
    let n = segmente(u);
    let mut q = pos;
    if u.geschlossen {
        q = q.rem_euclid(n as f64);
    }
    let s = (q.floor() as usize).min(n.saturating_sub(1));
    segment_punkt(u, s, (q - s as f64).clamp(0.0, 1.0))
}

/// Punkte des Umrisses von Parameter `von` nach `bis` (Parameter = Segment + Anteil) in Richtung ±1.
fn weg(u: &Umriss, von: f64, bis: f64, richtung: f64) -> Vec<(f64, f64)> {
    let n = segmente(u) as f64;
    let d = if u.geschlossen {
        if richtung > 0.0 {
            (bis - von).rem_euclid(n)
        } else {
            (von - bis).rem_euclid(n)
        }
    } else {
        (bis - von) * richtung
    };
    let schritte = ((d.abs() * 16.0).ceil() as usize).max(1);
    (0..=schritte)
        .map(|i| punkt_bei_parameter(u, von + richtung * d * i as f64 / schritte as f64))
        .collect()
}

fn kumuliert(l: &[(f64, f64)]) -> Vec<f64> {
    let mut k = vec![0.0];
    for w in l.windows(2) {
        let last = *k.last().unwrap();
        k.push(last + ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt());
    }
    k
}

fn bei(l: &[(f64, f64)], kum: &[f64], s: f64) -> (f64, f64) {
    if s <= 0.0 {
        return l[0];
    }
    for i in 1..l.len() {
        if kum[i] >= s {
            let t = if kum[i] > kum[i - 1] {
                (s - kum[i - 1]) / (kum[i] - kum[i - 1])
            } else {
                0.0
            };
            return (
                l[i - 1].0 + (l[i].0 - l[i - 1].0) * t,
                l[i - 1].1 + (l[i].1 - l[i - 1].1) * t,
            );
        }
    }
    *l.last().unwrap()
}

/// Punktfolge in Alphabet-Einheiten
pub type Zug = Vec<(f64, f64)>;

/// Satinsäule mit Winkellinien → Seiten A und B (Alphabet-Einheiten) mit Punkt-zu-Punkt-Zuordnung.
///
/// Seite A läuft über den Umriss von der ersten zur letzten Winkellinie (Richtung der a-Werte), Seite B in
/// Gegenrichtung. Zwischen zwei Winkellinien werden beide Teilwege mit gleich vielen Punkten nach relativer
/// Bogenlänge abgetastet (1 Punkt je mm), sodass die Winkellinien selbst exakt zugeordnet bleiben.
/// Grundlage `Analyse_Teil3/09` §2 (hoch); die Abtastung ist eine **eigene Festlegung**.
pub fn saeule_seiten(u: &Umriss, w: &[Winkellinie]) -> Option<(Zug, Zug)> {
    if w.len() < 2 || u.knoten.len() < 2 {
        return None;
    }
    let n = segmente(u) as f64;
    let pa: Vec<f64> = w.iter().map(|l| l.a as f64 + l.x).collect();
    let pb: Vec<f64> = w.iter().map(|l| l.c as f64 + l.y).collect();
    let d = if u.geschlossen {
        (pa[1] - pa[0]).rem_euclid(n)
    } else {
        pa[1] - pa[0]
    };
    let ra = if u.geschlossen {
        if d <= n / 2.0 { 1.0 } else { -1.0 }
    } else if d >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let (mut sa, mut sb) = (Vec::new(), Vec::new());
    for k in 0..w.len() - 1 {
        let wa = weg(u, pa[k], pa[k + 1], ra);
        let wb = weg(u, pb[k], pb[k + 1], -ra);
        let (ka, kb) = (kumuliert(&wa), kumuliert(&wb));
        let (la, lb) = (*ka.last().unwrap(), *kb.last().unwrap());
        let teile = ((la.max(lb) / EINHEIT).ceil() as usize).max(1);
        let letzte = k == w.len() - 2;
        for i in 0..(teile + usize::from(letzte)) {
            let t = i as f64 / teile as f64;
            sa.push(bei(&wa, &ka, t * la));
            sb.push(bei(&wb, &kb, t * lb));
        }
    }
    Some((sa, sb))
}

/// Umriss als Punktliste (Alphabet-Einheiten), je Segment 12 Teilstücke.
pub fn umriss_punkte(u: &Umriss) -> Vec<(f64, f64)> {
    let n = segmente(u);
    let mut out = Vec::new();
    for s in 0..n {
        for i in 0..12 {
            out.push(segment_punkt(u, s, i as f64 / 12.0));
        }
    }
    if let Some(l) = u.knoten.get(if u.geschlossen { 0 } else { u.knoten.len() - 1 }) {
        out.push(l.punkt);
    }
    out
}

// ---------------------------------------------------------------- Textsatz

/// Vorgaben für einen Text (wie Object Properties – Lettering im Original).
#[derive(Clone, Debug, PartialEq)]
pub struct TextVorgabe {
    pub text: String,
    /// Buchstabenhöhe in mm (Original 1–200 mm, Handbuch S. 165)
    pub hoehe: f64,
    /// Breite in % der Höhe (100 = Original)
    pub breite_prozent: f64,
    /// Buchstabenabstand in mm; None = Anteil des Alphabets × Höhe
    pub abstand: Option<f64>,
    /// Bezugspunkt auf der Grundlinie (mm)
    pub position: Punkt,
    /// 0 = Text beginnt bei `position`, 0,5 = mittig, 1 = endet dort (wie Ausrichtung links/Mitte/rechts)
    pub ausrichtung: f64,
    /// Unterlage: None = wie Stil „Schrift“ (Zickzack, bzw. die Unterlage des Alphabets), Some(leer) = keine
    pub unterlage: Option<Vec<String>>,
    pub farbe: String,
    /// Wortabstand in Teilen der Höhe (None = Vorlage 0,25)
    pub wortabstand: Option<f64>,
    /// Zeilenabstand (Lücke) in Teilen der Höhe (None = Vorlage 0,6)
    pub zeilenabstand: Option<f64>,
    /// Grundlinie: None = gerade; Kreis (Lettering-Dialog „Baseline“, Handbuch Kap. 19)
    pub grundlinie: Option<Grundlinie>,
}

/// Kreis-Grundlinie (Handbuch Kap. 19 „create circle baselines“; Dialog Baseline: Radius X/Y, Justif. Angle,
/// Position Above/Through/Below). Der Text wird zuerst gerade gesetzt, dann jeder Buchstabe starr auf den Kreis gelegt
/// (Bogenlänge = Abstand auf der geraden Grundlinie). `position` des Texts ist dann die **Kreismitte**.
#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grundlinie {
    /// „kreis“
    pub art: String,
    /// Radius in mm (Vorlage 0x3e8b/0x3e8c = 9000 → 50 mm, Zuordnung mittel)
    #[serde(default = "kreis_radius")]
    pub radius: f64,
    /// Lage der Textmitte auf dem Kreis in Grad (Justif. Angle; Standard 90 = oben, bei „unten“ 270)
    pub winkel: Option<f64>,
    /// „oben“ (Above: Buchstaben außen, im Uhrzeiger lesbar) oder „unten“ (Below: innen, gegen den Uhrzeiger)
    pub lage: Option<String>,
}

fn kreis_radius() -> f64 {
    50.0
}

impl Grundlinie {
    pub fn pruefen(&self) -> Result<(), String> {
        if self.art != "kreis" {
            return Err(format!("Grundlinie „{}“ unbekannt (kreis)", self.art));
        }
        if self.radius <= 0.0 {
            return Err("Radius der Grundlinie muss größer als 0 sein".into());
        }
        match self.lage.as_deref() {
            None | Some("oben") | Some("unten") => Ok(()),
            Some(x) => Err(format!("Lage „{x}“ unbekannt (oben, unten)")),
        }
    }

    /// Buchstaben einer gerade gesetzten Zeile auf den Kreis legen. `zeile` = Vorgabe der Zeile (Grundlinie y,
    /// Ankerpunkt x), `mitte` = Kreismitte (Position des Texts). Folgezeilen liegen innen (oben) bzw. außen (unten).
    pub fn anwenden(
        &self,
        objekte: Vec<Objekt>,
        zeile: &TextVorgabe,
        mitte: Punkt,
        winkel_standard: f64,
    ) -> Vec<Objekt> {
        use crate::transformation::{Matrix, abbilden, formpunkte};
        let unten = self.lage.as_deref() == Some("unten");
        let versatz = zeile.position.y - mitte.y; // 0 für die erste Zeile, negativ für Folgezeilen
        let r = if unten {
            self.radius - versatz
        } else {
            self.radius + versatz
        }
        .max(1.0);
        let w0 = self.winkel.unwrap_or(if unten { 270.0 } else { 90.0 }).to_radians();
        objekte
            .into_iter()
            .map(|o| {
                let mut pts = Vec::new();
                formpunkte(&o.art, &mut pts);
                if pts.is_empty() {
                    return o;
                }
                let x0 = pts.iter().map(|q| q.x).fold(f64::MAX, f64::min);
                let x1 = pts.iter().map(|q| q.x).fold(f64::MIN, f64::max);
                let xc = (x0 + x1) / 2.0;
                let s = xc - zeile.position.x;
                let (theta, drehung) = if unten {
                    (w0 + s / r, w0 + s / r + std::f64::consts::FRAC_PI_2)
                } else {
                    (w0 - s / r, w0 - s / r - std::f64::consts::FRAC_PI_2)
                };
                let ziel = p(mitte.x + r * theta.cos(), mitte.y + r * theta.sin());
                let (sn, cs) = drehung.sin_cos();
                let m = Matrix {
                    a: cs,
                    b: -sn,
                    c: sn,
                    d: cs,
                    e: ziel.x - (cs * xc - sn * zeile.position.y),
                    f: ziel.y - (sn * xc + cs * zeile.position.y),
                };
                abbilden(&o, &m, winkel_standard)
            })
            .collect()
    }
}

impl TextVorgabe {
    /// Text in Zeilen (Zeilenumbruch `\n`): jede Zeile eine eigene Vorgabe, Grundlinie je Zeile um
    /// Höhe × (1 + Zeilenabstand) nach unten; Ausrichtung gilt je Zeile.
    pub fn zeilen(&self) -> Vec<TextVorgabe> {
        let schritt = self.hoehe * (1.0 + self.zeilenabstand.unwrap_or(ZEILENABSTAND_ANTEIL));
        self.text
            .split('\n')
            .enumerate()
            .map(|(i, t)| TextVorgabe {
                text: t.trim_end_matches('\r').to_string(),
                position: p(self.position.x, self.position.y - i as f64 * schritt),
                ..self.clone()
            })
            .collect()
    }

    fn wort(&self) -> f64 {
        self.wortabstand.unwrap_or(WORTABSTAND_ANTEIL) * self.hoehe * self.breite_prozent / 100.0
    }
}

/// Unterlage einer Säule: Vorgabe des Texts; sonst die Unterlage aus dem Alphabet; sonst die des Stils „Schrift“.
fn unterlage_fuer(o: &ZeichenObjekt, v: &TextVorgabe) -> Unterlage {
    match &v.unterlage {
        Some(l) => Unterlage::Arten(l.clone()),
        None if !o.unterlagen.is_empty() => Unterlage::Arten(o.unterlagen.clone()),
        None => Unterlage::Arten(vec![SCHRIFT_UNTERLAGE.into()]),
    }
}

fn satin_optionen(abstand: Option<f64>, auto_spacing: bool) -> SatinOptionen {
    SatinOptionen {
        abstand: if auto_spacing { None } else { abstand },
        faktor: Some(SCHRIFT_FAKTOR),
        kurzstiche: Some(true),
    }
}

/// Text in Stickobjekte des Rechenkerns umsetzen. Liefert die Objekte und die Zeichen ohne Entsprechung im Alphabet.
pub fn text_objekte(alpha: &Alphabet, v: &TextVorgabe) -> (Vec<Objekt>, Vec<char>) {
    let s = v.hoehe / REFERENZ_HOEHE_MM / EINHEIT; // Alphabet-Einheit → mm
    let sx = s * v.breite_prozent / 100.0;
    let abstand = v.abstand.unwrap_or(alpha.abstand_anteil * v.hoehe);
    let vorschub = |ch: char| -> Option<f64> {
        if ch == ' ' {
            return Some(v.wort());
        }
        alpha
            .zeichen
            .get(&ch)
            .map(|z| (z.grundlinie[2] - z.grundlinie[0]) * sx + abstand * v.breite_prozent / 100.0)
    };
    let breite: f64 = v.text.chars().filter_map(vorschub).sum::<f64>()
        - if v.text.chars().last().is_some_and(|c| c != ' ') {
            abstand * v.breite_prozent / 100.0
        } else {
            0.0
        };
    let mut x = v.position.x - v.ausrichtung * breite.max(0.0);
    let mut objekte = Vec::new();
    let mut fehlend = Vec::new();
    for ch in v.text.chars() {
        if ch == ' ' {
            x += v.wort();
            continue;
        }
        let Some(z) = alpha.zeichen.get(&ch) else {
            fehlend.push(ch);
            continue;
        };
        let [gx, gy, gx2, _] = z.grundlinie;
        let abb = |q: (f64, f64)| p(x + (q.0 - gx) * sx, v.position.y + (q.1 - gy) * s);
        let abb_linie = |l: &[(f64, f64)]| -> Linie { l.iter().map(|&q| abb(q)).collect() };
        let mut teile: Vec<Objekt> = Vec::new();
        let mut umrisse: Vec<Linie> = Vec::new();
        let mut laufweg = SCHRIFT_LAUFWEG;
        for o in &z.objekte {
            if let Some(l) = o.laufweg {
                laufweg = l;
            }
            let art = match (&o.stichart, o.klasse) {
                (Stichart::Satin { abstand, auto_spacing }, 0x06) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    let Some((a, b)) = saeule_seiten(u, &o.winkel) else {
                        continue;
                    };
                    umrisse.push(abb_linie(&umriss_punkte(u)));
                    ObjektArt::Satin {
                        seite_a: abb_linie(&a),
                        seite_b: abb_linie(&b),
                        unterlage: unterlage_fuer(o, v),
                        optionen: satin_optionen(*abstand, *auto_spacing),
                    }
                }
                (Stichart::Satin { abstand, auto_spacing }, 0x0c) if o.umrisse.len() >= 2 => {
                    let (a, b) = (umriss_punkte(&o.umrisse[0]), umriss_punkte(&o.umrisse[1]));
                    umrisse.push(abb_linie(&a.iter().chain(b.iter().rev()).copied().collect::<Vec<_>>()));
                    ObjektArt::Satin {
                        seite_a: abb_linie(&a),
                        seite_b: abb_linie(&b),
                        unterlage: unterlage_fuer(o, v),
                        optionen: satin_optionen(*abstand, *auto_spacing),
                    }
                }
                (Stichart::Tatami, _) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    umrisse.push(abb_linie(&umriss_punkte(u)));
                    ObjektArt::Tatami {
                        umriss: abb_linie(&umriss_punkte(u)),
                        loecher: o.umrisse[1..].iter().map(|l| abb_linie(&umriss_punkte(l))).collect(),
                        winkel: None,
                        unterlage: Some(!o.unterlagen.is_empty()),
                        aussparen: Default::default(),
                    }
                }
                (Stichart::Lauf { laenge }, _) => {
                    let Some(u) = o.umrisse.first() else { continue };
                    umrisse.push(Vec::new());
                    let pts = abb_linie(&umriss_punkte(u));
                    let n = pts.len();
                    ObjektArt::Lauf {
                        punkte: pts,
                        ecken: vec![false; n],
                        laenge: *laenge,
                        dreifach: false,
                        geschlossen: u.geschlossen,
                    }
                }
                _ => continue,
            };
            teile.push(Objekt {
                farbe: v.farbe.clone(),
                name: format!("{} {}", alpha.name, ch),
                art,
            });
        }
        if !teile.is_empty() {
            objekte.push(Objekt {
                farbe: v.farbe.clone(),
                name: format!("{} {}", alpha.name, ch),
                art: ObjektArt::Gruppe {
                    teile,
                    umrisse,
                    laufweg,
                },
            });
        }
        x += (gx2 - gx) * sx + abstand * v.breite_prozent / 100.0;
    }
    (objekte, fehlend)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn knoten(x: f64, y: f64) -> Knoten {
        Knoten {
            ein: None,
            punkt: (x, y),
            aus: None,
        }
    }

    /// Rechteckige Säule 0…10 mm × 0…2 mm wie im Alphabet: Umriss 4 Knoten, Winkellinien 0↔3 und 1↔2.
    fn saeule() -> (Umriss, Vec<Winkellinie>) {
        let e = EINHEIT;
        let u = Umriss {
            geschlossen: true,
            knoten: vec![
                knoten(0.0, 0.0),
                knoten(10.0 * e, 0.0),
                knoten(10.0 * e, 2.0 * e),
                knoten(0.0, 2.0 * e),
            ],
        };
        // Segment 0 (Knoten 0→1) bei 0 ↔ Segment 2 (Knoten 2→3) bei 1 = Knoten 3; Knoten 1 ↔ Knoten 2
        let w = vec![
            Winkellinie {
                a: 0,
                x: 0.0,
                c: 2,
                y: 1.0,
            },
            Winkellinie {
                a: 1,
                x: 0.0,
                c: 2,
                y: 0.0,
            },
        ];
        (u, w)
    }

    #[test]
    fn seiten_aus_winkellinien() {
        let (u, w) = saeule();
        let (a, b) = saeule_seiten(&u, &w).unwrap();
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), 11); // 10 mm → 10 Teile + Endpunkt
        assert!((a[0].0).abs() < 1e-9 && (a[0].1).abs() < 1e-9);
        assert!((b[0].1 - 2.0 * EINHEIT).abs() < 1e-9 && b[0].0.abs() < 1e-9);
        assert!((a[10].0 - 10.0 * EINHEIT).abs() < 1e-6);
        assert!((b[10].0 - 10.0 * EINHEIT).abs() < 1e-6 && (b[10].1 - 2.0 * EINHEIT).abs() < 1e-6);
    }

    #[test]
    fn text_setzen_massstab_und_vorschub() {
        let (u, w) = saeule();
        let z = Zeichen {
            zeichen: 'I',
            grundlinie: [0.0, 0.0, 10.0 * EINHEIT, 0.0],
            objekte: vec![ZeichenObjekt {
                klasse: 0x06,
                umrisse: vec![u],
                winkel: w,
                stichart: Stichart::Satin {
                    abstand: Some(0.4),
                    auto_spacing: true,
                },
                unterlagen: vec![],
                laufweg: None,
            }],
        };
        let alpha = Alphabet {
            name: "Test".into(),
            abstand_anteil: 0.1,
            zeichen: [('I', z)].into_iter().collect(),
        };
        let v = TextVorgabe {
            text: "II?".into(),
            hoehe: 25.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(-10.0, 0.0),
            ausrichtung: 0.0,
            farbe: "#000000".into(),
            unterlage: None,
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
        };
        let (obj, fehlend) = text_objekte(&alpha, &v);
        assert_eq!(fehlend, vec!['?']);
        assert_eq!(obj.len(), 2);
        // Stil Schrift: Auto Spacing × 0,9, Kurzstiche, Zickzack-Unterlage
        let (_, u, o) = saeule_aus(&obj[0]);
        assert_eq!(u, Unterlage::Arten(vec!["zickzack".into()]));
        assert_eq!(
            o,
            SatinOptionen {
                abstand: None,
                faktor: Some(0.9),
                kurzstiche: Some(true)
            }
        );
        // Maßstab 25/50 = 0,5: Säule 5 mm lang; zweites I beginnt bei −10 + 5 + 2,5 (10 % von 25 mm)
        let (seite_a, _, _) = saeule_aus(&obj[1]);
        assert!((seite_a[0].x - (-2.5)).abs() < 1e-9, "{}", seite_a[0].x);
        assert!((seite_a.last().unwrap().x - 2.5).abs() < 1e-6);
        // mittig: Breite 5 + 2,5 + 5 = 12,5 → Beginn bei −6,25
        let (obj, _) = text_objekte(
            &alpha,
            &TextVorgabe {
                text: "II".into(),
                position: p(0.0, 0.0),
                ausrichtung: 0.5,
                ..v
            },
        );
        let (seite_a, _, _) = saeule_aus(&obj[0]);
        assert!((seite_a[0].x + 6.25).abs() < 1e-9, "{}", seite_a[0].x);
    }

    /// erste Säule eines Buchstabens (Gruppe)
    fn saeule_aus(o: &Objekt) -> (Linie, Unterlage, SatinOptionen) {
        let ObjektArt::Gruppe { teile, .. } = &o.art else {
            panic!("Buchstabe = Gruppe")
        };
        let ObjektArt::Satin {
            seite_a,
            unterlage,
            optionen,
            ..
        } = &teile[0].art
        else {
            panic!("Säule")
        };
        (seite_a.clone(), unterlage.clone(), optionen.clone())
    }

    /// Zeilen: Grundlinie je Zeile um Höhe × (1 + 0,6) tiefer (Vorlage 0x3f2d), Wortabstand 0,25 × Höhe (0x3e85)
    #[test]
    fn zeilen_und_wortabstand() {
        let v = TextVorgabe {
            text: "Hallo\r\nWelt\nzwei".into(),
            hoehe: 10.0,
            breite_prozent: 100.0,
            abstand: None,
            position: p(5.0, 20.0),
            ausrichtung: 0.5,
            unterlage: None,
            farbe: "#000000".into(),
            wortabstand: None,
            zeilenabstand: None,
            grundlinie: None,
        };
        let z = v.zeilen();
        assert_eq!(
            z.iter().map(|x| x.text.as_str()).collect::<Vec<_>>(),
            ["Hallo", "Welt", "zwei"]
        );
        assert_eq!(z[1].position, p(5.0, 4.0));
        assert!((z[2].position.y - (20.0 - 32.0)).abs() < 1e-9);
        assert!((v.wort() - 2.5).abs() < 1e-9);
        let eng = TextVorgabe {
            zeilenabstand: Some(0.0),
            wortabstand: Some(0.5),
            ..v
        };
        assert_eq!(eng.zeilen()[1].position, p(5.0, 10.0));
        assert!((eng.wort() - 5.0).abs() < 1e-9);
    }
}
