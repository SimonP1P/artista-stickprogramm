//! Regeln des Maschinenformats „Bernina“ und Verbinden der Wege zu einer Stichfolge.

use crate::einheiten::{AUTO_JUMP_TEILLAENGE_MM, MAX_STICH_MM, MIN_STICH_MM};
use crate::einstellungen::VerbinderEinstellung;
use crate::geometrie::{Linie, Punkt, abstand, kumuliert, p, punkt_bei};
use crate::stiche::{Art, Stich, StichFolge};

/// Anfangs-Vernähstiche: ein kurzes Stück entlang des Weges vor und zurück (eigene Festlegung).
pub fn vernaehen_anfang(weg: &[Punkt], e: &VerbinderEinstellung) -> Linie {
    if weg.len() < 2 || e.vernaeh_stiche == 0 {
        return weg.to_vec();
    }
    let kum = kumuliert(weg);
    let l = e.vernaeh_stichlaenge.min(*kum.last().unwrap());
    if l < MIN_STICH_MM {
        return weg.to_vec();
    }
    let q = punkt_bei(weg, l, &kum);
    let mut extra: Vec<Punkt> = (0..e.vernaeh_stiche - 1)
        .map(|i| if i % 2 == 0 { q } else { weg[0] })
        .collect();
    if extra.last() == Some(&q) {
        extra.push(weg[0]);
    }
    let mut out = vec![weg[0]];
    out.extend(extra);
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
/// Zwischen zwei Wegen: Sprung. Vernäht wird am Anfang und Ende, bei Farbwechsel und wenn der Sprung
/// länger als `vernaehen_ab` ist (AMT 0x272a: 3,0 mm).
pub fn wege_verbinden(teile: &[(String, Vec<Linie>)], e: &VerbinderEinstellung, start: Punkt) -> StichFolge {
    let alle: Vec<(&str, &Linie)> = teile
        .iter()
        .flat_map(|(f, wege)| wege.iter().filter(|w| !w.is_empty()).map(move |w| (f.as_str(), w)))
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
        let anfang_vernaehen =
            vorher.is_none_or(|(vf, vw)| vf != farbe || abstand(*vw.last().unwrap(), weg[0]) > e.vernaehen_ab);
        let ende_vernaehen =
            nachher.is_none_or(|(nf, nw)| nf != farbe || abstand(*weg.last().unwrap(), nw[0]) > e.vernaehen_ab);
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
        for &q in &w {
            folge.stich(q);
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
        assert_eq!(&pts[..3], &[p(0.0, 0.0), p(0.8, 0.0), p(0.0, 0.0)]);
        let i5 = pts.iter().position(|&q| q == p(5.0, 0.0)).unwrap();
        assert_eq!(pts[i5 + 1], p(6.0, 0.0)); // Verbinder 1 mm: kein Vernähen
    }
}
