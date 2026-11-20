//! Regeln des Maschinenformats „Bernina“ und Verbinden der Wege zu einer Stichfolge.

use crate::einheiten::{AUTO_JUMP_TEILLAENGE_MM, MAX_STICH_MM, MIN_STICH_MM};
use crate::einstellungen::VerbinderEinstellung;
use crate::geometrie::{Linie, Punkt, abstand, p, richtung, verschieben};
use crate::stiche::{Art, Stich, StichFolge};

/// Anfangs-Vernähstiche wie im Original (gemessen an den ART-Musterdesigns, `Analyse_Teil3/02`, hoch):
/// vom Anfang S entlang des **ersten Stichs** `n` Schritte vor und `n` zurück, Schrittlänge
/// t = min(`vernaeh_stichlaenge`, erster Stich / n); danach geht der Weg normal weiter.
/// Beispiel n = 2: S, S+t, S+2t, S+t, S, … (AMT 0x272f = 2, 0x2730 = 1,0 mm; MOD1 TieParameters).
/// Noch nicht nachgebaut: Sonderfälle des Originals bei sehr kurzem erstem Stich (Rückgriff über mehrere
/// Stiche mit 30°-Knickgrenze, MOD1 TieOffGenerator::GetStitchPoints 0x101ca750).
pub fn vernaehen_anfang(weg: &[Punkt], e: &VerbinderEinstellung) -> Linie {
    if weg.len() < 2 || e.vernaeh_stiche == 0 {
        return weg.to_vec();
    }
    let n = e.vernaeh_stiche;
    let t = e.vernaeh_stichlaenge.min(abstand(weg[0], weg[1]) / n as f64);
    if t < MIN_STICH_MM {
        return weg.to_vec();
    }
    let u = richtung(weg[0], weg[1]);
    let mut out = vec![weg[0]];
    out.extend((1..=n).map(|k| verschieben(weg[0], u, t * k as f64)));
    out.extend((0..n).rev().map(|k| verschieben(weg[0], u, t * k as f64)));
    out.extend_from_slice(&weg[1..]);
    out
}

pub fn vernaehen_ende(weg: &[Punkt], e: &VerbinderEinstellung) -> Linie {
    let umgekehrt: Linie = weg.iter().rev().copied().collect();
    let mut v = vernaehen_anfang(&umgekehrt, e);
    v.reverse();
    v
}

/// Wege (je Objekt: Farbe + Wege) zu einer Stichfolge verbinden.
///
/// Zwischen zwei Wegen: Sprung. Vernähen nach dem Dialog „Connectors“ des Originals (`VerbinderEinstellung`):
/// Anfang, wenn nach Trim/Farbwechsel oder der vorige Verbinder > 3,0 mm; Ende, wenn vor Trim/Farbwechsel oder
/// der nächste Verbinder > 3,0 mm; letztes Objekt nur bei „Always Tie Off Last Object“ (aus).
/// Annahme (mittel): Der Designanfang zählt wie „nach Trim“.
pub fn wege_verbinden(teile: &[(String, Vec<Linie>)], e: &VerbinderEinstellung, start: Punkt) -> StichFolge {
    wege_verbinden_mit(teile, e, start, &[])
}

/// Wie `wege_verbinden`; `auto_jump[k]` = eigene Auto-Jump-Werte des Teils k (Höchststich, Sprunglänge in mm):
/// längere Stiche des Teils werden schon hier in Sprünge geteilt (Effects-Dialog „Auto Jump“ je Objekt, Handbuch
/// Kap. 22). Ohne Angabe gilt die Standardregel in `bernina_regeln` (12,7 / 7,0 mm).
pub fn wege_verbinden_mit(
    teile: &[(String, Vec<Linie>)],
    e: &VerbinderEinstellung,
    start: Punkt,
    auto_jump: &[Option<(f64, f64)>],
) -> StichFolge {
    let alle: Vec<(&str, &Linie)> = teile
        .iter()
        .flat_map(|(f, wege)| wege.iter().filter(|w| !w.is_empty()).map(move |w| (f.as_str(), w)))
        .collect();
    let teil_nr: Vec<usize> = teile
        .iter()
        .enumerate()
        .flat_map(|(k, (_, wege))| wege.iter().filter(|w| !w.is_empty()).map(move |_| k))
        .collect();
    let mut folge = StichFolge::default();
    if alle.is_empty() {
        return folge;
    }
    folge.farben.push(alle[0].0.to_string());
    let mut pos = start;
    for (i, &(farbe, weg)) in alle.iter().enumerate() {
        let vorher = if i > 0 { Some(alle[i - 1]) } else { None };
        let nachher = alle.get(i + 1).copied();
        let verbinder_davor = vorher.map(|(_, vw)| abstand(*vw.last().unwrap(), weg[0]));
        let verbinder_danach = nachher.map(|(_, nw)| abstand(*weg.last().unwrap(), nw[0]));
        let farbwechsel_davor = vorher.is_some_and(|(vf, _)| vf != farbe);
        let farbwechsel_danach = nachher.is_some_and(|(nf, _)| nf != farbe);
        let trim_davor = verbinder_davor.is_none_or(|d| d > e.trim_ab);
        let trim_danach = verbinder_danach.is_some_and(|d| d > e.trim_ab);
        let anfang_vernaehen = e.anfang_immer
            || (e.anfang_nach_trim_farbwechsel && (trim_davor || farbwechsel_davor))
            || (e.anfang_bei_langem_verbinder && verbinder_davor.is_some_and(|d| d > e.anfang_verbinder_ab));
        let ende_vernaehen = e.ende_immer
            || (e.ende_vor_trim_farbwechsel && (trim_danach || farbwechsel_danach))
            || (e.ende_bei_langem_verbinder && verbinder_danach.is_some_and(|d| d > e.ende_verbinder_ab))
            || (nachher.is_none() && e.letztes_objekt_vernaehen);
        let mut w = weg.clone();
        if anfang_vernaehen {
            w = vernaehen_anfang(&w, e);
        }
        if ende_vernaehen {
            w = vernaehen_ende(&w, e);
        }
        if vorher.is_some_and(|(vf, _)| vf != farbe) {
            folge.farbwechsel(farbe);
        }
        if abstand(pos, w[0]) > 1e-9 {
            folge.sprung(w[0]);
        }
        let aj = auto_jump.get(teil_nr[i]).copied().flatten();
        let mut letzter: Option<Punkt> = None;
        for &q in &w {
            if let (Some((max, teil)), Some(a)) = (aj, letzter) {
                let d = abstand(a, q);
                if d > max + 1e-9 {
                    let n = (d / teil - 1e-9).ceil() as i64;
                    for j in 1..n {
                        let t = j as f64 / n as f64;
                        folge.sprung(p(a.x + (q.x - a.x) * t, a.y + (q.y - a.y) * t));
                    }
                }
            }
            folge.stich(q);
            letzter = Some(q);
        }
        pos = *w.last().unwrap();
    }
    folge
}

/// Stiche < 0,3 mm entfernen (DEFAULTS.DFL). Der letzte Stich vor Sprung/Farbwechsel bleibt.
pub fn kurze_stiche_entfernen(folge: &StichFolge, min_mm: f64) -> StichFolge {
    let mut neu = StichFolge {
        stiche: Vec::new(),
        farben: folge.farben.clone(),
    };
    let st = &folge.stiche;
    let mut letzter: Option<Punkt> = None;
    for (i, s) in st.iter().enumerate() {
        if s.art != Art::Stich {
            neu.stiche.push(*s);
            letzter = None; // erster Einstich nach Sprung/Farbwechsel bleibt immer
            continue;
        }
        let blockende = st.get(i + 1).is_none_or(|n| n.art != Art::Stich);
        if let Some(l) = letzter
            && abstand(l, s.p()) < min_mm
        {
            if !blockende {
                continue;
            }
            // Blockende: lieber den vorletzten Einstich ersetzen, damit das Ende stimmt
            let n = neu.stiche.len();
            if n >= 2 && neu.stiche[n - 1].art == Art::Stich && neu.stiche[n - 2].art == Art::Stich {
                neu.stiche[n - 1] = *s;
                letzter = Some(s.p());
                continue;
            }
            if abstand(l, s.p()) < 1e-9 && neu.stiche.last().is_some_and(|x| x.art == Art::Stich) {
                continue;
            }
        }
        neu.stiche.push(*s);
        letzter = Some(s.p());
    }
    neu
}

/// Auto Jump: Stiche > 12,7 mm in Sprünge zu ≤ 7,0 mm zerlegen, Einstich nur am Ende.
pub fn lange_stiche_teilen(folge: &StichFolge, max_mm: f64, teil_mm: f64) -> StichFolge {
    let mut neu = StichFolge {
        stiche: Vec::new(),
        farben: folge.farben.clone(),
    };
    let mut pos: Option<Punkt> = None;
    for s in &folge.stiche {
        if s.art == Art::Stich
            && let Some(a) = pos
        {
            let d = abstand(a, s.p());
            if d > max_mm + 1e-9 {
                let n = (d / teil_mm - 1e-9).ceil() as i64;
                for j in 1..n {
                    let t = j as f64 / n as f64;
                    let q = p(a.x + (s.x - a.x) * t, a.y + (s.y - a.y) * t);
                    neu.stiche.push(Stich {
                        x: q.x,
                        y: q.y,
                        art: Art::Sprung,
                    });
                }
            }
        }
        neu.stiche.push(*s);
        pos = Some(s.p());
    }
    neu
}

/// Standardregeln des Bernina-Formats anwenden (Mindeststich, Auto Jump).
pub fn bernina_regeln(folge: &StichFolge) -> StichFolge {
    lange_stiche_teilen(
        &kurze_stiche_entfernen(folge, MIN_STICH_MM),
        MAX_STICH_MM,
        AUTO_JUMP_TEILLAENGE_MM,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kurze_entfernt_erster_nach_sprung_bleibt() {
        let mut f = StichFolge {
            farben: vec!["#000".into()],
            ..Default::default()
        };
        f.sprung(p(0.0, 0.0));
        for q in [p(0.0, 0.0), p(0.1, 0.0), p(2.0, 0.0), p(2.1, 0.0)] {
            f.stich(q);
        }
        let g = kurze_stiche_entfernen(&f, MIN_STICH_MM);
        let pts: Vec<Punkt> = g.stiche.iter().filter(|s| s.art == Art::Stich).map(|s| s.p()).collect();
        assert_eq!(pts[0], p(0.0, 0.0));
        assert!(!pts.contains(&p(0.1, 0.0)));
        assert_eq!(*pts.last().unwrap(), p(2.1, 0.0));
        assert!(g.stichlaengen().iter().all(|&l| l >= MIN_STICH_MM));
    }

    #[test]
    fn auto_jump() {
        let mut f = StichFolge::default();
        f.stich(p(0.0, 0.0));
        f.stich(p(20.0, 0.0));
        let g = lange_stiche_teilen(&f, MAX_STICH_MM, AUTO_JUMP_TEILLAENGE_MM);
        let arten: Vec<Art> = g.stiche.iter().map(|s| s.art).collect();
        assert_eq!(arten, [Art::Stich, Art::Sprung, Art::Sprung, Art::Stich]);
        assert!(g.stiche.windows(2).all(|w| w[1].x - w[0].x <= 7.0 + 1e-9));
    }

    #[test]
    fn verbinden_vernaehen_farbwechsel() {
        let teile = vec![
            ("#000".to_string(), vec![vec![p(0.0, 0.0), p(5.0, 0.0)]]),
            ("#000".to_string(), vec![vec![p(6.0, 0.0), p(10.0, 0.0)]]),
            ("#f00".to_string(), vec![vec![p(10.0, 5.0), p(20.0, 5.0)]]),
        ];
        let f = wege_verbinden(&teile, &VerbinderEinstellung::default(), p(0.0, 0.0));
        assert_eq!(f.anzahl(Art::Farbwechsel), 1);
        assert_eq!(f.farben, ["#000", "#f00"]);
        let pts: Vec<Punkt> = f.stiche.iter().filter(|s| s.art == Art::Stich).map(|s| s.p()).collect();
        // Designanfang zählt wie „nach Trim“: 2 vor, 2 zurück à 1,0 mm entlang des ersten Stichs (wie im Original)
        assert_eq!(
            &pts[..6],
            &[
                p(0.0, 0.0),
                p(1.0, 0.0),
                p(2.0, 0.0),
                p(1.0, 0.0),
                p(0.0, 0.0),
                p(5.0, 0.0)
            ]
        );
        let i5 = pts.iter().position(|&q| q == p(5.0, 0.0)).unwrap();
        assert_eq!(pts[i5 + 1], p(6.0, 0.0)); // Verbinder 1 mm: kein Vernähen
        // vor dem Farbwechsel vernäht: …, 10, 9, 8, 9, 10
        let i10 = pts.iter().position(|&q| q == p(10.0, 0.0)).unwrap();
        assert_eq!(
            &pts[i10..i10 + 5],
            &[p(10.0, 0.0), p(9.0, 0.0), p(8.0, 0.0), p(9.0, 0.0), p(10.0, 0.0)]
        );
        // letztes Objekt: „Always Tie Off Last Object“ = aus -> endet ohne Vernähen
        assert_eq!(&pts[pts.len() - 2..], &[p(10.0, 5.0), p(20.0, 5.0)]);
    }

    #[test]
    fn letztes_objekt_nur_mit_option_vernaehen() {
        let teile = vec![("#000".to_string(), vec![vec![p(0.0, 0.0), p(10.0, 0.0)]])];
        let mut e = VerbinderEinstellung::default();
        let ohne: Vec<Punkt> = wege_verbinden(&teile, &e, p(0.0, 0.0))
            .stiche
            .iter()
            .map(|s| s.p())
            .collect();
        assert_eq!(
            ohne,
            [
                p(0.0, 0.0),
                p(1.0, 0.0),
                p(2.0, 0.0),
                p(1.0, 0.0),
                p(0.0, 0.0),
                p(10.0, 0.0)
            ]
        );
        e.letztes_objekt_vernaehen = true;
        let mit: Vec<Punkt> = wege_verbinden(&teile, &e, p(0.0, 0.0))
            .stiche
            .iter()
            .map(|s| s.p())
            .collect();
        assert_eq!(
            &mit[mit.len() - 5..],
            &[p(10.0, 0.0), p(9.0, 0.0), p(8.0, 0.0), p(9.0, 0.0), p(10.0, 0.0)]
        );
    }

    #[test]
    fn kurzer_erster_stich_halbe_laenge() {
        // erster Stich 1,8 mm -> Vernähschritte 0,9 mm (gemessen: HT106 usw.)
        let e = VerbinderEinstellung::default();
        let v = vernaehen_anfang(&[p(0.0, 0.0), p(1.8, 0.0), p(4.0, 0.0)], &e);
        let xs: Vec<f64> = v.iter().map(|q| (q.x * 1000.0).round() / 1000.0).collect();
        assert_eq!(xs, [0.0, 0.9, 1.8, 0.9, 0.0, 1.8, 4.0]);
    }
}
