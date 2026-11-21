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
