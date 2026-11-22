//! Aussparen (Knockout) – Roadmap Phase 4 Punkt 9.
//!
//! Original: „Remove Overlay Stitching“ (Menü Arrange) und „Lettering Remove Overlay“ (Handbuch PDF-S. 208–209): das
//! untere Füllobjekt behält seine Form, die Fläche des oberen Objekts (oder der Schrift) wird als Loch ausgespart,
//! damit nicht zwei Lagen übereinander liegen; nur für Füllflächen („Closed Curve“). An den 287 Musterdesigns gemessen
//! (`Analyse_Teil3/14_aussparen.md`): Löcher, die genau einem anderen Objekt entsprechen, liegen **exakt** auf dessen
//! Umriss (≤ 0,05 mm); von Hand digitalisierte Löcher sind im Median 0,15 mm kleiner (Überlappung). Deshalb:
//! Loch = Fläche des oberen Objekts, wahlweise um `ueberlappung` verkleinert.
//!
//! Polygon-Differenz mit `i_overlay` (freie Geometrie-Bibliothek, neutrale Technik).

use crate::design::ObjektArt;
use crate::geometrie::{Linie, Punkt, flaeche_vorzeichen, geschlossen_ohne_endpunkt, p, versatz_ring};
use crate::sticharten::satin::seiten_aus_mittellinie;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;

/// Aussparung an einer Füllfläche.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Aussparen {
    /// Namen der Objekte, deren Fläche aus dieser Fläche ausgespart wird
    pub namen: Vec<String>,
    /// Überlappung in mm: das Loch bleibt um so viel kleiner (0 = wie „Remove Overlay Stitching“, exakt)
    pub ueberlappung: f64,
}

pub type Kontur = Vec<[f64; 2]>;

/// Teil einer Fläche: Umriss und Löcher
pub type Teil = (Linie, Vec<Linie>);

/// Teile unter dieser Fläche (mm²) entfallen (eigene Festlegung: kleiner als ein Quadrat von 0,7 mm)
const MIN_TEIL: f64 = 0.5;

fn kontur(r: &[Punkt], gegen_uhrzeiger: bool) -> Kontur {
    let mut q = geschlossen_ohne_endpunkt(r);
    if (flaeche_vorzeichen(&q) > 0.0) != gegen_uhrzeiger {
        q.reverse();
    }
    q.iter().map(|a| [a.x, a.y]).collect()
}

/// Satinsäule als Vierecke zwischen benachbarten Stichpaaren; Seiten je Paar um `u` zur Mitte gerückt.
fn saeule(a: &[Punkt], b: &[Punkt], u: f64) -> Vec<Kontur> {
    let n = a.len().min(b.len());
    let rueck = |x: Punkt, y: Punkt| {
        let d = ((y.x - x.x).hypot(y.y - x.y)).max(1e-9);
        let t = (u / d).min(0.5);
        p(x.x + (y.x - x.x) * t, x.y + (y.y - x.y) * t)
    };
    let a2: Vec<Punkt> = (0..n).map(|i| rueck(a[i], b[i])).collect();
    let b2: Vec<Punkt> = (0..n).map(|i| rueck(b[i], a[i])).collect();
    (1..n)
        .map(|i| [a2[i - 1], a2[i], b2[i], b2[i - 1]])
        .filter(|q| flaeche_vorzeichen(q).abs() > 1e-6)
        .map(|q| kontur(&q, true))
        .collect()
}

/// Fläche, die ein Objekt bedeckt, als Konturen (außen gegen den Uhrzeiger, Löcher im Uhrzeiger; Vereinigung nach
/// der Nicht-Null-Regel), um `u` verkleinert. Offene Läufe bedecken keine Fläche.
pub fn bedeckt(art: &ObjektArt, u: f64) -> Vec<Kontur> {
    let flaeche = |umriss: &Linie, loecher: &[Linie]| {
        let mut k = vec![kontur(&versatz_ring(umriss, u, 3.0), true)];
        k.extend(loecher.iter().map(|l| kontur(&versatz_ring(l, -u, 3.0), false)));
        k
    };
    match art {
        ObjektArt::Satin { seite_a, seite_b, .. } => saeule(seite_a, seite_b, u),
        ObjektArt::SatinLinie {
            punkte, breite, mittig, ..
        } => {
            let (a, b) = crate::sticharten::satin::satinlinie_seiten(punkte, *breite, *mittig);
            saeule(&a, &b, u)
        }
        ObjektArt::Tatami { umriss, loecher, .. } | ObjektArt::Musterfuellung { umriss, loecher, .. } => {
            flaeche(umriss, loecher)
        }
        ObjektArt::Lauf {
            punkte,
            geschlossen: true,
            ..
        } if punkte.len() >= 3 => flaeche(punkte, &[]),
        ObjektArt::Gruppe { teile, .. } => teile.iter().flat_map(|t| bedeckt(&t.art, u)).collect(),
        // Knopfloch: Loch = mittlerer Umriss (Merge Buttonhole, `knopfloch.rs`), sonst die Teile
        ObjektArt::Kette { loch: Some(l), .. } => flaeche(l, &[]),
        ObjektArt::Kette { teile, .. } => teile.iter().flat_map(|t| bedeckt(&t.art, u)).collect(),
        ObjektArt::Wendefuellung { umriss, .. } => flaeche(umriss, &[]),
        ObjektArt::SatinFlaeche { umriss, loecher, .. } => flaeche(umriss, loecher),
        ObjektArt::Blanket {
            punkte,
            geschlossen,
            breite,
            mittig,
            ..
        } => {
            // Säule zwischen Rücken und Spitzen (Zähne nach links der Laufrichtung)
            let a: Linie = if *geschlossen {
                let mut r = geschlossen_ohne_endpunkt(punkte);
                r.push(r[0]);
                r
            } else {
                punkte.clone()
            };
            if *mittig {
                let (l, r) = seiten_aus_mittellinie(&a, *breite);
                return saeule(&l, &r, u);
            }
            // „Offset“: Säule zwischen der Linie und ihrer linken Seite im Abstand der Breite
            let (l, r) = seiten_aus_mittellinie(&a, 2.0 * *breite);
            let links = a.len() >= 2 && {
                let (d, q) = (p(a[1].x - a[0].x, a[1].y - a[0].y), p(l[0].x - a[0].x, l[0].y - a[0].y));
                d.x * q.y - d.y * q.x > 0.0
            };
            saeule(&a, if links { &l } else { &r }, u)
        }
        _ => Vec::new(),
    }
}

/// Breite, mit der eine Laufstich-Linie bei der Prüfung auf Überdeckung zählt (mm). **Eigene Festlegung** (etwa die
/// Breite eines Fadens 40er Garn mit Einstich), das Original prüft Überdeckungen nicht (`reihenfolge`).
pub const LINIE_BREITE: f64 = 0.5;

/// Fläche, die ein Objekt im Stickbild belegt – wie `bedeckt(…, 0)`, Laufstich-Linien (offen und geschlossen) aber als
/// Band der Breite `LINIE_BREITE` (eine geschlossene Linie belegt nur ihren Rand, nicht das Innere).
pub fn belegt(art: &ObjektArt) -> Vec<Kontur> {
    match art {
        ObjektArt::Lauf {
            punkte, geschlossen, ..
        } if punkte.len() >= 2 => {
            let mut l = punkte.clone();
            if *geschlossen && l.len() >= 3 {
                l.push(l[0]);
            }
            let (a, b) = seiten_aus_mittellinie(&l, LINIE_BREITE);
            saeule(&a, &b, 0.0)
        }
        ObjektArt::Gruppe { teile, .. } => teile.iter().flat_map(|t| belegt(&t.art)).collect(),
        ObjektArt::Kette { teile, .. } => teile.iter().flat_map(|t| belegt(&t.art)).collect(),
        _ => bedeckt(art, 0.0),
    }
}

/// Größe der gemeinsamen Fläche zweier Konturlisten (mm², Nicht-Null-Regel).
pub fn schnittflaeche(a: &[Kontur], b: &[Kontur]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let r = a
        .to_vec()
        .overlay(&b.to_vec(), OverlayRule::Intersect, FillRule::NonZero);
    let linie = |k: &Kontur| -> Linie { k.iter().map(|q| p(q[0], q[1])).collect() };
    r.iter()
        .filter(|form| !form.is_empty())
        .map(|form| {
            flaeche_vorzeichen(&linie(&form[0])).abs()
                - form[1..]
                    .iter()
                    .map(|k| flaeche_vorzeichen(&linie(k)).abs())
                    .sum::<f64>()
        })
        .sum()
}

/// Fläche (Umriss mit Löchern) minus die Schneideflächen: Teile als (Umriss, Löcher), größter zuerst.
pub fn flaeche_minus(umriss: &[Punkt], loecher: &[Linie], schneider: &[Kontur]) -> Vec<Teil> {
    let mut subj: Vec<Kontur> = vec![kontur(umriss, true)];
    subj.extend(loecher.iter().map(|l| kontur(l, false)));
    if schneider.is_empty() {
        return vec![(umriss.to_vec(), loecher.to_vec())];
    }
    let clip: Vec<Kontur> = schneider.to_vec();
    teile_aus(subj.overlay(&clip, OverlayRule::Difference, FillRule::NonZero))
}

/// Geschlossener Umriss als Kontur (gegen den Uhrzeiger), z. B. für `vereinigen`.
pub fn ring_kontur(r: &[Punkt]) -> Kontur {
    kontur(r, true)
}

/// Ringe einer Schrift nach der Nicht-Null-Regel (wie TrueType/CFF gefüllt werden) zu Flächen mit Löchern vereinigen:
/// überlappende Konturen werden eine Fläche (variable Schriften wie Google Sans zeichnen das „A“ aus dem Λ und zwei
/// Hälften des Querbalkens, die darüber liegen), gegenläufige innere Ringe werden Löcher. Teile, größter zuerst.
pub fn vereinigen_nonzero(ringe: &[Linie]) -> Vec<Teil> {
    let k: Vec<Kontur> = ringe
        .iter()
        .map(|r| geschlossen_ohne_endpunkt(r).iter().map(|a| [a.x, a.y]).collect::<Kontur>())
        .filter(|k| k.len() >= 3)
        .collect();
    if k.is_empty() {
        return Vec::new();
    }
    let leer: Vec<Kontur> = Vec::new();
    teile_aus(k.overlay(&leer, OverlayRule::Union, FillRule::NonZero))
}

/// Vereinigung von Flächen (Konturen wie aus `bedeckt`/`ring_kontur`, Nicht-Null-Regel): Teile, größter zuerst.
pub fn vereinigen(konturen: &[Kontur]) -> Vec<Teil> {
    if konturen.is_empty() {
        return Vec::new();
    }
    let leer: Vec<Kontur> = Vec::new();
    teile_aus(konturen.to_vec().overlay(&leer, OverlayRule::Union, FillRule::NonZero))
}

/// Flächen aus Ringen mit Füllregel wie in SVG (`gerade_ungerade` = „evenodd“, sonst „nonzero“; die Richtung der Ringe
/// bleibt, wie sie ist): Teile (Umriss, Löcher), größter zuerst.
pub fn flaechen_aus_ringen(ringe: &[Linie], gerade_ungerade: bool) -> Vec<Teil> {
    let subj: Vec<Kontur> = ringe
        .iter()
        .map(|r| geschlossen_ohne_endpunkt(r))
        .filter(|r| r.len() >= 3)
        .map(|r| r.iter().map(|a| [a.x, a.y]).collect())
        .collect();
    if subj.is_empty() {
        return Vec::new();
    }
    let regel = if gerade_ungerade {
        FillRule::EvenOdd
    } else {
        FillRule::NonZero
    };
    let leer: Vec<Kontur> = Vec::new();
    teile_aus(subj.overlay(&leer, OverlayRule::Subject, regel))
}

/// **Umrandung** (eigene Erweiterung, `design::Umrandung`): Ringe um die vereinigte Fläche der Konturen, um `einzug`
/// mm nach innen versetzt (Versatz mit `i_overlay`, Gehrungsecken). Je Teil erst der Umriss, dann seine Löcher; Teile
/// von links nach rechts (Schrift: Buchstabe für Buchstabe). Ringe geschlossen (letzter Punkt = erster).
pub fn umrandung_ringe(konturen: &[Kontur], einzug: f64) -> Vec<Linie> {
    use i_overlay::mesh::float::outline::offset::OutlineOffset;
    use i_overlay::mesh::float::style::{LineJoin, OutlineStyle};
    if konturen.is_empty() {
        return Vec::new();
    }
    let leer: Vec<Kontur> = Vec::new();
    let vereint = konturen.to_vec().overlay(&leer, OverlayRule::Union, FillRule::NonZero);
    let formen = if einzug > 1e-6 {
        // Gehrung bis 20° Innenwinkel, spitzere Ecken abgeschrägt (eigene Festlegung)
        let stil = OutlineStyle::new(-einzug).line_join(LineJoin::Miter(20f64.to_radians()));
        vereint.outline(&stil)
    } else {
        vereint
    };
    let mut teile = teile_aus(formen);
    let links = |r: &Linie| r.iter().map(|q| q.x).fold(f64::INFINITY, f64::min);
    teile.sort_by(|a, b| links(&a.0).total_cmp(&links(&b.0)));
    teile
        .into_iter()
        .flat_map(|(u, l)| std::iter::once(u).chain(l))
        .map(|mut r| {
            if let Some(&q) = r.first() {
                r.push(q);
            }
            r
        })
        .collect()
}

/// Teile als Konturen (außen gegen den Uhrzeiger, Löcher im Uhrzeiger), z. B. zum Ausschneiden.
pub fn teile_konturen(teile: &[Teil]) -> Vec<Kontur> {
    teile
        .iter()
        .flat_map(|(u, l)| std::iter::once(kontur(u, true)).chain(l.iter().map(|x| kontur(x, false))))
        .collect()
}

/// Ergebnis einer Überlagerung als Teile (Umriss, Löcher), kleine Reste entfallen, größter zuerst.
fn teile_aus(ergebnis: Vec<Vec<Kontur>>) -> Vec<Teil> {
    let linie = |k: &Kontur| -> Linie { k.iter().map(|q| p(q[0], q[1])).collect() };
    let mut teile: Vec<Teil> = ergebnis
        .iter()
        .filter(|form| !form.is_empty())
        .map(|form| {
            let aussen = linie(&form[0]);
            let innen: Vec<Linie> = form[1..]
                .iter()
                .map(linie)
                .filter(|l| flaeche_vorzeichen(l).abs() >= MIN_TEIL)
                .collect();
            (aussen, innen)
        })
        .filter(|(a, _)| flaeche_vorzeichen(a).abs() >= MIN_TEIL)
        .collect();
    teile.sort_by(|x, y| {
        flaeche_vorzeichen(&y.0)
            .abs()
            .total_cmp(&flaeche_vorzeichen(&x.0).abs())
    });
    teile
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rechteck(x0: f64, y0: f64, x1: f64, y1: f64) -> Linie {
        vec![p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)]
    }

    fn netto(t: &[(Linie, Vec<Linie>)]) -> f64 {
        t.iter()
            .map(|(a, l)| flaeche_vorzeichen(a).abs() - l.iter().map(|x| flaeche_vorzeichen(x).abs()).sum::<f64>())
            .sum()
    }

    #[test]
    fn loch_und_teilung() {
        let f = rechteck(0.0, 0.0, 20.0, 10.0);
        // Quadrat in der Mitte → ein Loch
        let t = flaeche_minus(&f, &[], &[kontur(&rechteck(5.0, 3.0, 9.0, 7.0), true)]);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].1.len(), 1);
        assert!((netto(&t) - 184.0).abs() < 1e-6);
        // Balken quer durch → zwei Teile
        let t = flaeche_minus(&f, &[], &[kontur(&rechteck(8.0, -1.0, 12.0, 11.0), true)]);
        assert_eq!(t.len(), 2);
        assert!((netto(&t) - 160.0).abs() < 1e-6);
    }

    #[test]
    fn satinsaeule_mit_ueberlappung() {
        let a = vec![p(0.0, 0.0), p(10.0, 0.0)];
        let b = vec![p(0.0, 2.0), p(10.0, 2.0)];
        let k = saeule(&a, &b, 0.0);
        assert_eq!(k.len(), 1);
        let t = flaeche_minus(&rechteck(-5.0, -5.0, 15.0, 7.0), &[], &k);
        assert!((netto(&t) - (240.0 - 20.0)).abs() < 1e-6);
        // 0,2 mm Überlappung: Säule je Seite 0,2 schmaler → Loch 10 × 1,6
        let t = flaeche_minus(&rechteck(-5.0, -5.0, 15.0, 7.0), &[], &saeule(&a, &b, 0.2));
        assert!((netto(&t) - (240.0 - 16.0)).abs() < 1e-6);
    }

    /// Satinbalken über einer Tatami-Fläche: die Fläche zerfällt in zwei Teile, unter dem Balken kein Einstich
    #[test]
    fn knockout_im_design() {
        let json = r##"{"rahmen": "artista 130 x 100", "objekte": [
            {"art": "tatami", "name": "Grund", "farbe": "#336699", "aussparen": ["Balken"],
             "umriss": {"rechteck": {"mitte": [0, 0], "breite": 40, "hoehe": 20}}},
            {"art": "satinlinie", "name": "Balken", "farbe": "#cc0000", "punkte": [[0, -15], [0, 15]], "breite": 4}
        ]}"##;
        let d = crate::Design::aus_json(json).unwrap();
        let (obj, _) = d.objekte_aufgeloest().unwrap();
        assert_eq!(obj.len(), 3, "zwei Teile + Balken");
        let f = d.berechnen().unwrap();
        let grund: Vec<_> = f
            .stiche
            .iter()
            .take_while(|s| s.art != crate::Art::Farbwechsel)
            .filter(|s| s.art == crate::Art::Stich)
            .collect();
        assert!(grund.len() > 500);
        // Zugausgleich 0,2 mm: Einstiche bis 0,2 mm ins Loch, nicht weiter
        let drin = grund.iter().filter(|s| s.p().x.abs() < 2.0 - 0.25).count();
        assert_eq!(drin, 0);
        let ohne = crate::Design::aus_json(&json.replace(r#""aussparen": ["Balken"],"#, "")).unwrap();
        assert_eq!(ohne.objekte_aufgeloest().unwrap().0.len(), 2);
        let fehler = crate::Design::aus_json(&json.replace(r#"["Balken"]"#, r#"["Gibts nicht"]"#)).unwrap();
        assert!(fehler.berechnen().is_err());
    }
}
