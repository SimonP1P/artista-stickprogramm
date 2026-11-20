//! Ein- und Ausstich je Objekt – ROADMAP 4b.5, Befund `Analyse_Teil3/16_reihenfolge.md`.
//!
//! Original: Jedes Objekt hat einen Ein- und einen Ausstichpunkt (Handbuch Kap. 7 „Key to control points: entry point,
//! exit point“; Kap. 14 „start and end points adjusted: fewer connectors“), der Anwender verschiebt sie mit Reshape.
//! **Closest Join** (MOD1 `DesignDocument::ResequenceByClosestJoin` 0x1006d0c0, GEOMETRY `FindExitEntry` 0x10056544):
//! für je zwei aufeinanderfolgende Objekte die beiden **nächsten Punkte ihrer Umrisse** (exakter kleinster Abstand)
//! → Ausstich des ersten, Einstich des zweiten; nur eines mit Umriss: nächster Umrisspunkt zum Stich des anderen.
//! Benutzt von „Sequence As Selected“ (Schalter in Resequence 0x1006db20) und vom Auto Digitizer (Handbuch Kap. 13).
//!
//! Gemessen an den Musterdesigns (`Arbeitsdaten/werkzeuge/art_reihenfolge.py`): ohne Eingriff stickt das Original **wie
//! digitalisiert** – offene Linien zu 75 % vom ersten zum letzten Knoten (13 % rückwärts, darunter Double-Run-Kopien),
//! geschlossene Linien beginnen zu 88 % am ersten Knoten und enden zu 80 % wieder dort, Flächen beginnen zu 69 % am
//! ersten Knoten. Der Kern lässt deshalb ohne Angabe alles wie gezeichnet (Flächen beginnen wie bisher am Punkt nächst
//! dem vorigen Ende – JSON-Formen wie `rechteck`/`kreis` haben keinen gezeichneten ersten Punkt, eigene Festlegung).
//!
//! Umsetzung (die Stichberechnung des Kerns kennt keinen frei gelegten Ausstich – **eigene Festlegungen**):
//! - offene Linie, Satinsäule, Satinlinie (mittig), Blanket (mittig): Richtung so, dass Anfang nahe `einstich` und Ende
//!   nahe `ausstich` liegt (wie im Original liegen Ein-/Ausstich an den Enden);
//! - geschlossene Linie, Satinlinie, Blanket: Beginn (= Ende, ganze Runde) am Umrisspunkt nächst `einstich`, ohne
//!   Einstich nächst `ausstich`;
//! - Flächen und Buchstaben: Beginn nächst `einstich`; nur `ausstich`: Beginn am Umrisspunkt, der am weitesten vom
//!   Ausstich entfernt ist (die Reihen enden dann auf der Seite des Ausstichs);
//! - Säule/Blanket/Satinlinie mit Versatz zur Seite werden nicht umgedreht (die Richtung bestimmt die Seite).

use crate::design::ObjektArt;
use crate::geometrie::{Linie, Punkt, abstand, abstand_punkt_strecke, geschlossen_ohne_endpunkt};

const GLEICH: f64 = 1e-6;
/// Näher als das an einem Knoten: kein neuer Knoten (mm)
const AN_KNOTEN: f64 = 0.05;

fn ist_ring(l: &[Punkt]) -> bool {
    l.len() >= 3 && abstand(l[0], l[l.len() - 1]) < GLEICH
}

/// Lotfußpunkt von `q` auf der Strecke a–b.
fn lot(q: Punkt, a: Punkt, b: Punkt) -> Punkt {
    let (vx, vy) = (b.x - a.x, b.y - a.y);
    let l2 = vx * vx + vy * vy;
    let t = if l2 < 1e-18 {
        0.0
    } else {
        (((q.x - a.x) * vx + (q.y - a.y) * vy) / l2).clamp(0.0, 1.0)
    };
    Punkt {
        x: a.x + t * vx,
        y: a.y + t * vy,
    }
}

/// Ring (ohne Endpunkt) so drehen, dass er am Punkt nächst `q` beginnt; liegt der nicht an einem Knoten, kommt dort ein
/// Knoten hinzu (Kurvenpunkt, `ecke` = false). Liefert Punkte und Eckenkennung (ohne Endpunkt).
pub fn ring_ab(ring: &[Punkt], ecken: Option<&[bool]>, q: Punkt) -> (Linie, Vec<bool>) {
    let n = ring.len();
    let ecke = |i: usize| ecken.and_then(|e| e.get(i).copied()).unwrap_or(true);
    if n < 2 {
        return (ring.to_vec(), (0..n).map(ecke).collect());
    }
    let (mut beste, mut bd) = (0, f64::MAX);
    for i in 0..n {
        let d = abstand_punkt_strecke(q, ring[i], ring[(i + 1) % n]);
        if d < bd - 1e-12 {
            (beste, bd) = (i, d);
        }
    }
    let (a, b) = (ring[beste], ring[(beste + 1) % n]);
    let f = lot(q, a, b);
    let (mut pkt, mut eck) = (Vec::with_capacity(n + 1), Vec::with_capacity(n + 1));
    let start = if abstand(f, a) <= AN_KNOTEN {
        beste
    } else if abstand(f, b) <= AN_KNOTEN {
        (beste + 1) % n
    } else {
        pkt.push(f);
        eck.push(false);
        (beste + 1) % n
    };
    let neu = !pkt.is_empty();
    for k in 0..n {
        let i = (start + k) % n;
        pkt.push(ring[i]);
        eck.push(ecke(i));
    }
    if neu {
        // der Ring endet am eingefügten Punkt: Knoten a (= Vorgänger) steht zuletzt
        debug_assert!(abstand(*pkt.last().unwrap(), a) < GLEICH);
    }
    (pkt, eck)
}

/// Offene Linie umdrehen, wenn ihr Anfang so näher an `ein` und ihr Ende näher an `aus` liegt.
fn umdrehen(l: &[Punkt], ein: Option<Punkt>, aus: Option<Punkt>) -> bool {
    let (Some(&a), Some(&z)) = (l.first(), l.last()) else {
        return false;
    };
    let kosten = |s: Punkt, e: Punkt| ein.map_or(0.0, |q| abstand(q, s)) + aus.map_or(0.0, |q| abstand(q, e));
    kosten(z, a) < kosten(a, z) - GLEICH
}

/// Ziel für den Beginn einer geschlossenen Form: Einstich, sonst Ausstich (Runde endet, wo sie beginnt).
fn ring_ziel(ein: Option<Punkt>, aus: Option<Punkt>) -> Option<Punkt> {
    ein.or(aus)
}

/// Objekt so ausrichten, dass es nahe `ein` beginnt und nahe `aus` endet (Linien, Säulen, geschlossene Formen).
/// Flächen und Gruppen bleiben, für sie gilt `start_hinweis`.
pub fn ausrichten(art: &ObjektArt, ein: Option<Punkt>, aus: Option<Punkt>) -> ObjektArt {
    if ein.is_none() && aus.is_none() {
        return art.clone();
    }
    let mut art = art.clone();
    match &mut art {
        ObjektArt::Lauf {
            punkte,
            ecken,
            geschlossen,
            ..
        } => {
            if *geschlossen || ist_ring(punkte) {
                let mit_ende = !*geschlossen;
                let mut e = ecken.clone();
                let ring = geschlossen_ohne_endpunkt(punkte);
                e.truncate(ring.len());
                if let Some(q) = ring_ziel(ein, aus) {
                    let (mut p, mut k) = ring_ab(&ring, Some(&e), q);
                    if mit_ende {
                        p.push(p[0]);
                        k.push(k[0]);
                    }
                    (*punkte, *ecken) = (p, k);
                }
            } else if umdrehen(punkte, ein, aus) {
                punkte.reverse();
                ecken.reverse();
            }
        }
        ObjektArt::Satin { seite_a, seite_b, .. } => {
            let mitte: Linie = seite_a
                .iter()
                .zip(seite_b.iter())
                .map(|(a, b)| Punkt {
                    x: (a.x + b.x) / 2.0,
                    y: (a.y + b.y) / 2.0,
                })
                .collect();
            if umdrehen(&mitte, ein, aus) {
                seite_a.reverse();
                seite_b.reverse();
            }
        }
        ObjektArt::SatinLinie { punkte, mittig, .. } => {
            if ist_ring(punkte) {
                if let Some(q) = ring_ziel(ein, aus) {
                    let (mut p, _) = ring_ab(&geschlossen_ohne_endpunkt(punkte), None, q);
                    p.push(p[0]);
                    *punkte = p;
                }
            } else if *mittig && umdrehen(punkte, ein, aus) {
                punkte.reverse();
            }
        }
        ObjektArt::Blanket {
            punkte,
            geschlossen,
            mittig,
            ..
        } => {
            if *geschlossen || ist_ring(punkte) {
                if let Some(q) = ring_ziel(ein, aus) {
                    let mit_ende = !*geschlossen;
                    let (mut p, _) = ring_ab(&geschlossen_ohne_endpunkt(punkte), None, q);
                    if mit_ende {
                        p.push(p[0]);
                    }
                    *punkte = p;
                }
            } else if *mittig && umdrehen(punkte, ein, aus) {
                punkte.reverse();
            }
        }
        _ => {}
    }
    art
}

/// Umrisse einer Fläche bzw. Gruppe (für den Beginn nach Ausstich).
fn umrisse(art: &ObjektArt) -> Vec<&Linie> {
    match art {
        ObjektArt::Tatami { umriss, .. }
        | ObjektArt::Musterfuellung { umriss, .. }
        | ObjektArt::Wendefuellung { umriss, .. } => vec![umriss],
        ObjektArt::Gruppe { umrisse, .. } => umrisse.iter().collect(),
        _ => Vec::new(),
    }
}

/// Startpunkt für Flächen und Gruppen (deren Stichberechnung beginnt am Punkt nächst diesem): Einstich; nur Ausstich:
/// der Umrisspunkt, der am weitesten vom Ausstich entfernt liegt. Sonst `None` (Beginn nächst dem vorigen Ende).
pub fn start_hinweis(art: &ObjektArt, ein: Option<Punkt>, aus: Option<Punkt>) -> Option<Punkt> {
    let u = umrisse(art);
    if u.is_empty() {
        return None;
    }
    if ein.is_some() {
        return ein;
    }
    let aus = aus?;
    u.iter()
        .flat_map(|l| l.iter())
        .copied()
        .max_by(|a, b| abstand(*a, aus).total_cmp(&abstand(*b, aus)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrie::p;

    fn lauf(pkt: Linie, geschlossen: bool) -> ObjektArt {
        ObjektArt::Lauf {
            ecken: vec![true; pkt.len()],
            punkte: pkt,
            laenge: None,
            dreifach: false,
            geschlossen,
        }
    }

    #[test]
    fn offene_linie_dreht_um() {
        let l = lauf(vec![p(0.0, 0.0), p(10.0, 0.0), p(20.0, 0.0)], false);
        let ObjektArt::Lauf { punkte, .. } = ausrichten(&l, Some(p(21.0, 1.0)), None) else {
            panic!()
        };
        assert_eq!(punkte[0], p(20.0, 0.0));
        // Ausstich am Anfang: ebenfalls umdrehen
        let ObjektArt::Lauf { punkte, .. } = ausrichten(&l, None, Some(p(-1.0, 0.0))) else {
            panic!()
        };
        assert_eq!(punkte[0], p(20.0, 0.0));
        // passt schon: bleibt
        assert_eq!(ausrichten(&l, Some(p(-1.0, 0.0)), Some(p(25.0, 0.0))), l);
    }

    #[test]
    fn geschlossene_linie_beginnt_am_naechsten_punkt() {
        let q = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let ObjektArt::Lauf { punkte, ecken, .. } = ausrichten(&lauf(q.clone(), true), Some(p(12.0, 5.0)), None) else {
            panic!()
        };
        // neuer Knoten (Kurvenpunkt) mitten auf der rechten Kante, danach die Ecken in alter Richtung
        assert_eq!(
            punkte,
            vec![p(10.0, 5.0), p(10.0, 10.0), p(0.0, 10.0), p(0.0, 0.0), p(10.0, 0.0)]
        );
        assert_eq!(ecken, vec![false, true, true, true, true]);
        // an einer Ecke: kein neuer Knoten
        let ObjektArt::Lauf { punkte, .. } = ausrichten(&lauf(q, true), Some(p(-1.0, 11.0)), None) else {
            panic!()
        };
        assert_eq!(punkte[0], p(0.0, 10.0));
        assert_eq!(punkte.len(), 4);
    }

    #[test]
    fn flaeche_beginnt_gegenueber_dem_ausstich() {
        let t = ObjektArt::Tatami {
            umriss: vec![p(0.0, 0.0), p(30.0, 0.0), p(30.0, 10.0), p(0.0, 10.0)],
            loecher: vec![],
            winkel: None,
            unterlage: None,
            aussparen: Default::default(),
        };
        assert_eq!(start_hinweis(&t, None, Some(p(31.0, 11.0))), Some(p(0.0, 0.0)));
        assert_eq!(
            start_hinweis(&t, Some(p(5.0, 5.0)), Some(p(31.0, 11.0))),
            Some(p(5.0, 5.0))
        );
        assert_eq!(start_hinweis(&t, None, None), None);
        assert_eq!(
            start_hinweis(&lauf(vec![p(0.0, 0.0)], false), Some(p(1.0, 1.0)), None),
            None
        );
    }
}
