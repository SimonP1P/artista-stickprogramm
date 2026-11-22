//! Motive (Handbuch Kap. 25 „Pattern Runs and Fills“): Motivsätze aus `userletw/` und die Motivlinie (Pattern Run).
//!
//! Ein Motivsatz hat denselben Aufbau wie ein Alphabet (ESB): je Motiv eine Grundlinie aus zwei Bezugspunkten
//! („Reference points set the standard orientation of the pattern … also set the spacing between each repetition“,
//! Handbuch S. 231) und Objekte (meist Läufe). Lesen: `stick-formate::esb::motive_laden`.
//!
//! **Motivlinie** – gemessen an den Musterdesigns „Sampler - Pattern Outlines …“ (`Analyse_Teil3/20_motive.md` §3):
//! - jede Kopie behält die Größe (Länge der Grundlinie = Feld „Size“), Drehung = Richtung der Linie;
//! - Anzahl n = Linienlänge / Wiederholabstand **gerundet**, Abstand der Kopien = Linienlänge / n – das ist die Regel
//!   des Handbuchs (S. 225: Rest < halbes Motiv → gleichmäßig verteilt, sonst ein Motiv mehr und leicht überlappend);
//! - Kopie i beginnt bei Bogenlänge i · Linienlänge / n.

use crate::design::{Objekt, ObjektArt, Unterlage};
use crate::geometrie::{Linie, Punkt, p};
use crate::schrift::{EINHEIT, Zeichen, zeichen_teile};
use std::collections::BTreeMap;

/// Auto-Spacing-Faktor im Stil „Motif Run“ (NORMAL.AMT `<PRESET_MOTIF_RUN_1>`, 0x2736 = 1,0)
pub const MOTIV_FAKTOR: f64 = 1.0;

/// Ein Motivsatz (Pattern Set): Name aus dem Kopf (z. B. „Heirloom“, „_BW004a“) und Motive nach Namen („709“).
#[derive(Clone, Debug, PartialEq)]
pub struct Motivsatz {
    pub name: String,
    pub motive: BTreeMap<String, Zeichen>,
}

/// Maße eines Motivs in mm: Länge der Grundlinie (Wiederholabstand) und Höhe der Formen quer zur Grundlinie.
pub fn motiv_masse(z: &Zeichen) -> (f64, f64) {
    let [x1, y1, x2, y2] = z.grundlinie;
    let l = (x2 - x1).hypot(y2 - y1);
    let (ux, uy) = if l > 0.0 { ((x2 - x1) / l, (y2 - y1) / l) } else { (1.0, 0.0) };
    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
    for o in &z.objekte {
        for u in &o.umrisse {
            for q in crate::schrift::umriss_punkte(u) {
                let h = -(q.0 - x1) * uy + (q.1 - y1) * ux;
                lo = lo.min(h);
                hi = hi.max(h);
            }
        }
    }
    (l / EINHEIT, if hi >= lo { (hi - lo) / EINHEIT } else { 0.0 })
}

/// Linien eines Motivs in mm, Grundlinie von (0, 0) nach (Länge, 0) – für die Vorschau in der Auswahl.
pub fn motiv_linien(z: &Zeichen) -> Vec<Linie> {
    let [x1, y1, x2, y2] = z.grundlinie;
    let w = (y2 - y1).atan2(x2 - x1);
    let (c, s) = (w.cos(), w.sin());
    z.objekte
        .iter()
        .flat_map(|o| o.umrisse.iter())
        .map(|u| {
            crate::schrift::umriss_punkte(u)
                .into_iter()
                .map(|q| {
                    let (dx, dy) = (q.0 - x1, q.1 - y1);
                    p((dx * c + dy * s) / EINHEIT, (-dx * s + dy * c) / EINHEIT)
                })
                .collect()
        })
        .collect()
}

/// Vorgaben einer Motivlinie (Object Properties – Outline Stitch „Pattern Run“).
#[derive(Clone, Debug, PartialEq)]
pub struct MotivVorgabe {
    /// Motivsatz (Name im Kopf oder Dateiname, z. B. „Heirloom“)
    pub satz: String,
    /// Motiv (Name im Satz, z. B. „709“)
    pub motiv: String,
    /// Linie in mm (bereits aus Bézier/Kreis aufgelöst)
    pub linie: Linie,
    pub geschlossen: bool,
    /// Länge eines Motivs entlang der Linie in mm („Size“); None = Größe im Motivsatz
    pub groesse: Option<f64>,
    /// Wiederholabstand in mm („Spacing“); None = Größe (Motive stoßen aneinander)
    pub abstand: Option<f64>,
    /// Motive an der Linie spiegeln (andere Seite)
    pub spiegeln: bool,
}

/// Linie mit Bogenlänge; vor dem Anfang/nach dem Ende geradlinig verlängert, geschlossen ringsum.
struct Weg {
    pts: Vec<Punkt>,
    kum: Vec<f64>,
    geschlossen: bool,
}

impl Weg {
    fn neu(linie: &[Punkt], geschlossen: bool) -> Weg {
        let mut pts: Vec<Punkt> = Vec::new();
        for &q in linie {
            if pts.last().is_none_or(|l: &Punkt| (l.x - q.x).hypot(l.y - q.y) > 1e-9) {
                pts.push(q);
            }
        }
        if geschlossen
            && pts.len() > 2
            && let (Some(a), Some(b)) = (pts.first().copied(), pts.last().copied())
            && (a.x - b.x).hypot(a.y - b.y) > 1e-9
        {
            pts.push(a);
        }
        let mut kum = vec![0.0];
        for w in pts.windows(2) {
            kum.push(kum[kum.len() - 1] + (w[1].x - w[0].x).hypot(w[1].y - w[0].y));
        }
        Weg { pts, kum, geschlossen }
    }

    fn laenge(&self) -> f64 {
        self.kum.last().copied().unwrap_or(0.0)
    }

    fn bei(&self, s: f64) -> Punkt {
        let l = self.laenge();
        let s = if self.geschlossen && l > 0.0 { s.rem_euclid(l) } else { s };
        let n = self.pts.len();
        let i = self.kum[..n - 1].iter().rposition(|&k| k <= s).unwrap_or(0);
        let (a, b) = (self.pts[i], self.pts[i + 1]);
        let t = (s - self.kum[i]) / (self.kum[i + 1] - self.kum[i]);
        p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
    }
}

/// Wiederholungen einer Motivlinie: (Anfang, Ende) der Grundlinie je Kopie auf der Linie, in mm.
pub fn motivlinie_lagen(linie: &[Punkt], geschlossen: bool, groesse: f64, abstand: f64) -> Vec<(Punkt, Punkt)> {
    let w = Weg::neu(linie, geschlossen);
    let l = w.laenge();
    if w.pts.len() < 2 || l <= 0.0 || groesse <= 0.0 || abstand <= 0.0 {
        return vec![];
    }
    let n = ((l / abstand).round() as usize).max(1);
    let schritt = l / n as f64;
    (0..n)
        .map(|i| {
            let s = i as f64 * schritt;
            (w.bei(s), w.bei(s + groesse))
        })
        .collect()
}

/// Motivlinie in Stickobjekte umsetzen: eine `Kette` aller Teile (Kopie für Kopie, Teile in der Reihenfolge des
/// Motivs – „Digitize the object from left to right so that stitching does not cross over the shape when joined“,
/// Handbuch S. 230).
pub fn motivlinie_objekt(satz: &Motivsatz, v: &MotivVorgabe, farbe: &str, name: &str) -> Result<Objekt, String> {
    let z = satz
        .motive
        .get(&v.motiv)
        .ok_or_else(|| format!("Motiv „{}“ nicht im Motivsatz „{}“", v.motiv, satz.name))?;
    let [x1, y1, x2, y2] = z.grundlinie;
    let l0 = (x2 - x1).hypot(y2 - y1);
    if l0 <= 0.0 {
        return Err(format!("Motiv „{}“: Grundlinie ohne Länge", v.motiv));
    }
    let groesse = v.groesse.unwrap_or(l0 / EINHEIT);
    if groesse.is_nan() || groesse <= 0.0 {
        return Err("Motivgröße muss größer als 0 sein".into());
    }
    let abstand = v.abstand.unwrap_or(groesse);
    if abstand.is_nan() || abstand <= 0.0 {
        return Err("Motivabstand muss größer als 0 sein".into());
    }
    let w0 = (y2 - y1).atan2(x2 - x1);
    let k = groesse / (l0 / EINHEIT) / EINHEIT; // Motiv-Einheit → mm
    let spiegel = if v.spiegeln { -1.0 } else { 1.0 };
    let mut teile = Vec::new();
    for (a, b) in motivlinie_lagen(&v.linie, v.geschlossen, groesse, abstand) {
        let w = (b.y - a.y).atan2(b.x - a.x);
        let (c0, s0) = (w0.cos(), w0.sin());
        let (c, s) = (w.cos(), w.sin());
        let abb = |q: (f64, f64)| -> Punkt {
            let (dx, dy) = (q.0 - x1, q.1 - y1);
            let u = (dx * c0 + dy * s0) * k;
            let t = (-dx * s0 + dy * c0) * k * spiegel;
            p(a.x + u * c - t * s, a.y + u * s + t * c)
        };
        let keine = |_: &crate::schrift::ZeichenObjekt| Unterlage::Arten(vec![]);
        let eigene = |o: &crate::schrift::ZeichenObjekt| Unterlage::Arten(o.unterlagen.clone());
        let unterlage: &dyn Fn(&crate::schrift::ZeichenObjekt) -> Unterlage =
            if z.objekte.iter().any(|o| !o.unterlagen.is_empty()) { &eigene } else { &keine };
        let (t, _, _) = zeichen_teile(z, &abb, unterlage, MOTIV_FAKTOR, farbe, name, true);
        teile.extend(t);
    }
    if teile.is_empty() {
        return Err("Motivlinie: Linie zu kurz oder Motiv ohne Stiche".into());
    }
    let n = teile.len();
    Ok(Objekt {
        farbe: farbe.to_string(),
        name: name.to_string(),
        art: ObjektArt::Kette {
            teile,
            umkehrbar: vec![false; n],
            loch: None,
        },
    })
}

// ---------------------------------------------------------------- Motivfüllung (Pattern Fill)

/// Standard der Motivfüllung wie das Original (NORMAL.AMT `<PRESET_STANDARD_MOTIF_FILL>`, Füll-Stichart 0x27ee: Satz
/// „NP004a“, Motiv „K0001a“, Größe 0x3ab8 = 2158 = 12,0 mm = Höhe des Motivs, Spalten-/Reihenabstand 0x27b6/0x27b9 =
/// 2158 = Kante an Kante; Bericht 20 §2, korrigiert 22.11.2026 in Bericht 24 §1).
pub const FUELL_SATZ: &str = "NP004a";
pub const FUELL_MOTIV: &str = "K0001a";
/// Standard der Motivlinie wie das Original (`<PRESET_STANDARD_MOTIF_RUN>`, Kontur-Stichart 0x2798: Satz „NP004a“,
/// Motiv „K0001a“, Abstand 0x27df = 2146 = 11,92 mm = Länge der Grundlinie).
pub const LINIE_SATZ: &str = "NP004a";
pub const LINIE_MOTIV: &str = "K0001a";
/// Lücke zwischen zwei Motiven, bis zu der sie ohne Sprung verbunden werden (mm, eigene Festlegung)
pub const FUELL_VERBINDEN_BIS: f64 = 1.0;

/// Vorgaben einer Motivfüllung (Handbuch S. 227–229 „Pattern Fill“: Motive in parallelen Reihen; Spalten-/
/// Reihenabstand, Reihenversatz, Größe und Drehung über die Leitmotive).
#[derive(Clone, Debug, PartialEq)]
pub struct MotivFuellVorgabe {
    pub satz: String,
    pub motiv: String,
    /// Fläche in mm (aufgelöst) und Löcher
    pub umriss: Linie,
    pub loecher: Vec<Linie>,
    /// Länge der Grundlinie eines Motivs in mm; None = Größe im Motivsatz
    pub groesse: Option<f64>,
    /// Abstand der Motive in einer Reihe (mm); None = Breite des Motivs (wie das Original: 0x27b9 ≈ Breite)
    pub spaltenabstand: Option<f64>,
    /// Abstand der Reihen (mm); None = Höhe des Motivs (0x27b6 ≈ Höhe)
    pub reihenabstand: Option<f64>,
    /// Reihenversatz in mm: jede Reihe um so viel weiter verschoben als die vorige („row offset … distorts the rows“)
    pub versatz: f64,
    /// Drehung der Reihen in Grad (Leitmotiv in der Mitte; „The stitch angle has no effect on pattern layout“)
    pub winkel: f64,
}

/// Kasten der Motivlinien (mm, Grundlinie von (0, 0) nach (Länge, 0), Größe im Satz): (u_min, u_max, t_min, t_max).
pub fn motiv_kasten(z: &Zeichen) -> Option<(f64, f64, f64, f64)> {
    let l = motiv_linien(z);
    let pts = l.iter().flatten();
    let (mut a, mut b, mut c, mut d) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for q in pts {
        a = a.min(q.x);
        b = b.max(q.x);
        c = c.min(q.y);
        d = d.max(q.y);
    }
    (b >= a && d >= c).then_some((a, b, c, d))
}

/// Strecke a–b an den Rändern der Fläche teilen; Stücke mit Mitte in der Fläche (Löcher zählen als außen).
fn innen_stuecke(linie: &[Punkt], ringe: &[Linie]) -> Vec<Linie> {
    let mut out: Vec<Linie> = Vec::new();
    let mut akt: Linie = Vec::new();
    for w in linie.windows(2) {
        let (a, b) = (w[0], w[1]);
        let mut ts = vec![0.0, 1.0];
        for r in ringe {
            let n = r.len();
            for i in 0..n {
                let (c, d) = (r[i], r[(i + 1) % n]);
                let den = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);
                if den.abs() < 1e-12 {
                    continue;
                }
                let t = ((c.x - a.x) * (d.y - c.y) - (c.y - a.y) * (d.x - c.x)) / den;
                let u = ((c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)) / den;
                if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                    ts.push(t);
                }
            }
        }
        ts.sort_by(f64::total_cmp);
        for t in ts.windows(2) {
            if t[1] - t[0] < 1e-9 {
                continue;
            }
            let bei = |s: f64| p(a.x + (b.x - a.x) * s, a.y + (b.y - a.y) * s);
            let (q0, q1) = (bei(t[0]), bei(t[1]));
            if crate::geometrie::im_polygon(bei((t[0] + t[1]) / 2.0), ringe) {
                if akt.last().is_none_or(|l| crate::geometrie::abstand(*l, q0) > 1e-9) {
                    if akt.len() >= 2 {
                        out.push(std::mem::take(&mut akt));
                    }
                    akt = vec![q0];
                }
                akt.push(q1);
            } else if akt.len() >= 2 {
                out.push(std::mem::take(&mut akt));
            } else {
                akt.clear();
            }
        }
    }
    if akt.len() >= 2 {
        out.push(akt);
    }
    out
}

/// Linie eines Teils (für Längen und das Abschneiden): Lauf = Punkte, Satin = beide Seiten.
fn teil_linien(o: &Objekt) -> Vec<Linie> {
    match &o.art {
        ObjektArt::Lauf { punkte, geschlossen, .. } => {
            let mut l = punkte.clone();
            if *geschlossen && let Some(&a) = l.first() {
                l.push(a);
            }
            vec![l]
        }
        ObjektArt::Satin { seite_a, seite_b, .. } => vec![seite_a.clone(), seite_b.clone()],
        _ => vec![],
    }
}

/// Lagen der Motive einer Motivfüllung: je Reihe (oben nach unten) die Motive von links nach rechts als Ursprung der
/// Grundlinie (u, t im gedrehten Rahmen). Das Raster liegt mit einem Motiv in der Mitte des Flächenkastens (eigene
/// Festlegung – im Original legen es die Leitmotive fest, Handbuch S. 228).
#[allow(clippy::too_many_arguments)]
fn fuell_lagen(
    ringe: &[Linie],
    winkel: f64,
    kasten: (f64, f64, f64, f64),
    sp: f64,
    ra: f64,
    versatz: f64,
) -> Vec<Vec<(f64, f64)>> {
    let w = winkel.to_radians();
    let (c, s) = (w.cos(), w.sin());
    let (mut u0, mut u1, mut t0, mut t1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for q in ringe.iter().flatten() {
        let (u, t) = (q.x * c + q.y * s, -q.x * s + q.y * c);
        u0 = u0.min(u);
        u1 = u1.max(u);
        t0 = t0.min(t);
        t1 = t1.max(t);
    }
    if u1 < u0 {
        return vec![];
    }
    let (km, kt) = ((kasten.0 + kasten.1) / 2.0, (kasten.2 + kasten.3) / 2.0);
    let (uc, tc) = ((u0 + u1) / 2.0 - km, (t0 + t1) / 2.0 - kt);
    let (hb, hh) = (kasten.1 - kasten.0, kasten.3 - kasten.2);
    let r_von = ((t0 - tc - hh) / ra).floor() as i64;
    let r_bis = ((t1 - tc + hh) / ra).ceil() as i64;
    let mut reihen = Vec::new();
    for r in (r_von..=r_bis).rev() {
        let t = tc + r as f64 * ra;
        let schub = r as f64 * versatz;
        let c_von = ((u0 - uc - schub - hb) / sp).floor() as i64;
        let c_bis = ((u1 - uc - schub + hb) / sp).ceil() as i64;
        reihen.push((c_von..=c_bis).map(|k| (uc + schub + k as f64 * sp, t)).collect());
    }
    reihen
}

/// Motivfüllung in Stickobjekte umsetzen (Handbuch S. 227: „repeats patterns in parallel rows to fill the shape“).
/// Reihe für Reihe, in der Reihe von links nach rechts (Motive sind „from left to right“ digitalisiert, S. 230).
/// Ragt ein Motiv **mehr als zur Hälfte** über den Rand, wird es am Rand abgeschnitten („If more than half a pattern
/// is outside the boundary, it will be clipped along the boundary“, S. 227); sonst bleibt es ganz (Deutung, mittel).
/// Motive, die aneinanderstoßen (Lücke ≤ 1 mm), werden ohne Sprung verbunden; dazwischen Sprung (eigene Festlegung).
pub fn motivfuellung_objekte(satz: &Motivsatz, v: &MotivFuellVorgabe, farbe: &str, name: &str) -> Result<Vec<Objekt>, String> {
    let z = satz
        .motive
        .get(&v.motiv)
        .ok_or_else(|| format!("Motiv „{}“ nicht im Motivsatz „{}“", v.motiv, satz.name))?;
    let [x1, y1, x2, y2] = z.grundlinie;
    let l0 = (x2 - x1).hypot(y2 - y1);
    if l0 <= 0.0 {
        return Err(format!("Motiv „{}“: Grundlinie ohne Länge", v.motiv));
    }
    let groesse = v.groesse.unwrap_or(l0 / EINHEIT);
    if groesse.is_nan() || groesse <= 0.0 {
        return Err("Motivgröße muss größer als 0 sein".into());
    }
    let f = groesse / (l0 / EINHEIT);
    let k0 = motiv_kasten(z).ok_or("Motiv ohne Linien")?;
    let kasten = (k0.0 * f, k0.1 * f, k0.2 * f, k0.3 * f);
    let sp = v.spaltenabstand.unwrap_or(kasten.1 - kasten.0);
    let ra = v.reihenabstand.unwrap_or(kasten.3 - kasten.2);
    if !(sp > 0.3 && ra > 0.3) {
        return Err("Spalten- und Reihenabstand müssen größer als 0,3 mm sein".into());
    }
    if v.umriss.len() < 3 {
        return Err("Motivfüllung braucht eine geschlossene Fläche".into());
    }
    let mut ringe = vec![v.umriss.clone()];
    ringe.extend(v.loecher.iter().filter(|l| l.len() >= 3).cloned());
    let lagen = fuell_lagen(&ringe, v.winkel, kasten, sp, ra, v.versatz);
    let anzahl: usize = lagen.iter().map(|r| r.len()).sum();
    if anzahl > 20_000 {
        return Err(format!("Motivfüllung: {anzahl} Motive – Motiv zu klein für die Fläche"));
    }
    let w0 = (y2 - y1).atan2(x2 - x1);
    let (c0, s0) = (w0.cos(), w0.sin());
    let kk = f / EINHEIT;
    let w = v.winkel.to_radians();
    let (c, s) = (w.cos(), w.sin());
    let keine = |_: &crate::schrift::ZeichenObjekt| Unterlage::Arten(vec![]);
    let eigene = |o: &crate::schrift::ZeichenObjekt| Unterlage::Arten(o.unterlagen.clone());
    let unterlage: &dyn Fn(&crate::schrift::ZeichenObjekt) -> Unterlage =
        if z.objekte.iter().any(|o| !o.unterlagen.is_empty()) { &eigene } else { &keine };

    // Teile mit Markierung „gehört zum vorigen Teil derselben Kopie“ (wird ohne Sprung angeschlossen)
    let mut folge: Vec<(Objekt, bool)> = Vec::new();
    for reihe in &lagen {
        for &(ou, ot) in reihe {
            let abb = |q: (f64, f64)| -> Punkt {
                let (dx, dy) = (q.0 - x1, q.1 - y1);
                let u = ou + (dx * c0 + dy * s0) * kk;
                let t = ot + (-dx * s0 + dy * c0) * kk;
                p(u * c - t * s, u * s + t * c)
            };
            let (teile, _, _) = zeichen_teile(z, &abb, unterlage, MOTIV_FAKTOR, farbe, name, true);
            let lang = |l: &Linie| crate::geometrie::laenge(l);
            let gesamt: f64 = teile.iter().flat_map(teil_linien).map(|l| lang(&l)).sum();
            let innen: f64 = teile
                .iter()
                .flat_map(teil_linien)
                .flat_map(|l| innen_stuecke(&l, &ringe))
                .map(|l| lang(&l))
                .sum();
            if gesamt <= 0.0 || innen <= 1e-6 {
                continue;
            }
            if innen >= gesamt * 0.5 - 1e-9 {
                // höchstens zur Hälfte draußen: ganzes Motiv
                for (i, t) in teile.into_iter().enumerate() {
                    folge.push((t, i > 0));
                }
                continue;
            }
            // mehr als zur Hälfte draußen: am Rand abschneiden
            for t in teile {
                match &t.art {
                    ObjektArt::Lauf { laenge, dreifach, .. } => {
                        for l in teil_linien(&t).iter().flat_map(|l| innen_stuecke(l, &ringe)) {
                            if lang(&l) < 0.3 {
                                continue;
                            }
                            let n = l.len();
                            folge.push((
                                Objekt {
                                    art: ObjektArt::Lauf {
                                        punkte: l,
                                        ecken: vec![true; n],
                                        laenge: *laenge,
                                        dreifach: *dreifach,
                                        geschlossen: false,
                                    },
                                    ..t.clone()
                                },
                                false,
                            ));
                        }
                    }
                    _ => {
                        let l = teil_linien(&t);
                        let pts: Vec<Punkt> = l.iter().flatten().copied().collect();
                        if pts.is_empty() {
                            continue;
                        }
                        let m = p(
                            pts.iter().map(|q| q.x).sum::<f64>() / pts.len() as f64,
                            pts.iter().map(|q| q.y).sum::<f64>() / pts.len() as f64,
                        );
                        if crate::geometrie::im_polygon(m, &ringe) {
                            folge.push((t, false));
                        }
                    }
                }
            }
        }
    }
    if folge.is_empty() {
        return Err("Motivfüllung: kein Motiv liegt in der Fläche (Fläche zu klein?)".into());
    }
    // zu Ketten zusammenfassen: neue Kette (= Sprung), wenn ein Teil weiter als 1 mm vom vorigen Ende beginnt
    let ende = |o: &Objekt| teil_linien(o).first().and_then(|l| l.last().copied());
    let anfang = |o: &Objekt| teil_linien(o).first().and_then(|l| l.first().copied());
    let mut ketten: Vec<Vec<Objekt>> = Vec::new();
    for (t, gehoert) in folge {
        let nah = match (ketten.last().and_then(|k| k.last()).and_then(ende), anfang(&t)) {
            (Some(a), Some(b)) => crate::geometrie::abstand(a, b) <= FUELL_VERBINDEN_BIS,
            _ => false,
        };
        if (gehoert || nah) && let Some(k) = ketten.last_mut() {
            k.push(t);
        } else {
            ketten.push(vec![t]);
        }
    }
    Ok(ketten
        .into_iter()
        .map(|teile| {
            let n = teile.len();
            Objekt {
                farbe: farbe.to_string(),
                name: name.to_string(),
                art: ObjektArt::Kette {
                    teile,
                    umkehrbar: vec![false; n],
                    loch: None,
                },
            }
        })
        .collect())
}

// ---------------------------------------------------------------- Eigene Motive (Create Pattern)

/// Ein eigenes Motiv (Handbuch S. 230–231 „Create Pattern“): Objekte des Designs (JSON ohne Transformation, Formen
/// als Punktlisten/Bézier wie nach `in_knoten_umwandeln`) und die Grundlinie aus zwei Bezugspunkten in mm.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct EigenesMotiv {
    pub name: String,
    /// Bezugspunkte [[x1, y1], [x2, y2]] in mm: Lage und Wiederholabstand des Motivs
    pub grundlinie: [[f64; 2]; 2],
    pub objekte: Vec<serde_json::Value>,
}

/// Eigener Motivsatz (Datei `<Datenordner>/motive/<Name>.json`; eigene Erweiterung – das Original speichert ESB).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct EigenerSatz {
    pub name: String,
    pub motive: Vec<EigenesMotiv>,
}

/// Objekt eines eigenen Motivs in ein Objekt des Motivsatzes umsetzen (Einheiten 1/180 mm). Läufe bleiben Läufe
/// (Dreifach wird einfach), Satinsäulen bleiben Säulen, Flächen werden Step-Flächen (eigene Festlegung – im Original
/// behält ein Motiv seine Objekte).
fn eigenes_objekt(v: &serde_json::Value) -> Result<crate::schrift::ZeichenObjekt, String> {
    use crate::format::ObjektJson;
    use crate::schrift::{Stichart, ZeichenObjekt};
    let o: ObjektJson = serde_json::from_value(v.clone()).map_err(|e| format!("Motiv: {e}"))?;
    let abb = |q: Punkt| (q.x * EINHEIT, q.y * EINHEIT);
    let um = |a: &crate::format::PunktAngabe, g: bool| crate::umwandeln::knoten_umriss(a, g, &abb);
    let neu = |klasse: u32, umrisse, stichart| ZeichenObjekt {
        klasse,
        umrisse,
        winkel: vec![],
        stichart,
        unterlagen: vec![],
        laufweg: None,
    };
    Ok(match &o {
        ObjektJson::Lauf(l) | ObjektJson::Dreifach(l) => {
            neu(0x05, vec![um(&l.punkte, l.geschlossen)], Stichart::Lauf { laenge: l.laenge })
        }
        ObjektJson::Satin(s) => neu(
            0x0c,
            vec![um(&s.seite_a, false), um(&s.seite_b, false)],
            Stichart::Satin {
                abstand: s.abstand,
                auto_spacing: s.abstand.is_none(),
            },
        ),
        ObjektJson::Tatami(f) => {
            let mut u = vec![um(&f.umriss, true)];
            u.extend(f.loecher.iter().map(|l| um(l, true)));
            neu(0x06, u, Stichart::Tatami)
        }
        ObjektJson::Muster(f) => {
            let mut u = vec![um(&f.umriss, true)];
            u.extend(f.loecher.iter().map(|l| um(l, true)));
            neu(0x06, u, Stichart::Tatami)
        }
        ObjektJson::Satinflaeche(f) => {
            let mut u = vec![um(&f.umriss, true)];
            u.extend(f.loecher.iter().map(|l| um(l, true)));
            neu(0x06, u, Stichart::Tatami)
        }
        _ => return Err("Motive können nur aus Läufen, Satinsäulen und Flächen bestehen".into()),
    })
}

/// Eigenen Motivsatz in einen Motivsatz des Kerns umsetzen.
pub fn eigener_satz(s: &EigenerSatz) -> Result<Motivsatz, String> {
    let mut motive = BTreeMap::new();
    for m in &s.motive {
        let objekte: Result<Vec<_>, String> = m.objekte.iter().map(eigenes_objekt).collect();
        let [a, b] = m.grundlinie;
        motive.insert(
            m.name.clone(),
            Zeichen {
                zeichen: ' ',
                grundlinie: [a[0] * EINHEIT, a[1] * EINHEIT, b[0] * EINHEIT, b[1] * EINHEIT],
                objekte: objekte.map_err(|e| format!("Motiv „{}“: {e}", m.name))?,
            },
        );
    }
    Ok(Motivsatz {
        name: s.name.clone(),
        motive,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regel aus den Musterdesigns (Analyse_Teil3/20 §3): Heirloom 705, Linie 22 148 Einheiten, Größe 2038 → 11
    /// Kopien im Abstand 2013,5 (gemessen 2013); Children 929, Größe 1802 → 12 Kopien, Abstand 1845,7 (gemessen 1846).
    #[test]
    fn anzahl_und_abstand_wie_das_original() {
        let e = EINHEIT;
        let linie = vec![p(0.0, 0.0), p(22148.0 / e, 0.0)];
        let l = motivlinie_lagen(&linie, false, 2038.0 / e, 2038.0 / e);
        assert_eq!(l.len(), 11);
        assert!(((l[1].0.x - l[0].0.x) * e - 2013.5).abs() < 0.5);
        assert!(((l[0].1.x - l[0].0.x) * e - 2038.0).abs() < 1e-6, "Größe bleibt");
        let l = motivlinie_lagen(&linie, false, 1802.0 / e, 1802.0 / e);
        assert_eq!(l.len(), 12);
        assert!(((l[1].0.x - l[0].0.x) * e - 1845.7).abs() < 0.5);
        // letzte Kopie ragt über das Ende hinaus (gerade verlängert)
        let l = motivlinie_lagen(&linie, false, 2038.0 / e, 2038.0 / e);
        assert!(l[10].1.x * e > 22148.0);
    }

    #[test]
    fn geschlossene_linie_ringsum() {
        let q = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let l = motivlinie_lagen(&q, true, 5.0, 5.0);
        assert_eq!(l.len(), 8);
        // letzte Kopie endet wieder am Anfang
        let (_, e) = l[7];
        assert!(e.x.abs() < 1e-9 && e.y.abs() < 1e-9);
    }
}
