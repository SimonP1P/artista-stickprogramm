//! Reihenfolge der Objekte ändern – ROADMAP 4b.5, Handbuch Kap. 14 „Resequencing embroidery objects“ (PDF-S. 130–133),
//! Befund `Analyse_Teil3/16_reihenfolge.md`.
//!
//! Original (Menü Arrange, BERESENU Menü 5/129; Kontextmenü 700 der Color Film):
//! - **Sequence As Selected** (0x8163): Objekte in der Reihenfolge, in der sie ausgewählt wurden. MOD1
//!   `DesignDocument::Resequence` (0x1006db20): ab 2 Objekten, kopiert die Auswahl in Auswahlreihenfolge, löscht die
//!   Originale und fügt die Kopie **an der Stelle des ersten ausgewählten Objekts** ein (wahlweise mit Closest Join).
//! - **Sequence to Start / to End** (0x81A9/0x81AA): ausgewählte Objekte an den Anfang bzw. ans Ende.
//! - **Sequence By Color** (0x81AB): Dialog listet die Farben der ausgewählten Objekte, „Move Up/Down“, OK – „reduces the
//!   number of color changes“.
//! - **Resequence by Number** (0x824C, DLG 1280): vor/nach „the object #“.
//!
//! Die Befehle selbst stehen in ARTISTA.EXE, das verschlüsselt ist (HASP-Hülle, Entropie 8,0); lesbar ist nur das
//! Resequence aus MOD1. Was sich daraus nicht ergibt, ist als **eigene Festlegung** markiert. Eine Prüfung auf
//! Überdeckungen hat das Original **nicht** (Handbuch: Bild „black first / black last“ ohne Hinweis; MOD1 ohne solche
//! Funktion) – `ueberdeckungen`, `vertauscht` und `nach_farbe` mit `ueberdeckung` sind **eigene Erweiterungen**, damit
//! beim Sortieren nach Farbe nichts unter eine Fläche gerät, die vorher darunter lag.
//!
//! Nummern hier: Stelle in `objekte` ab 0 (die CLI zählt ab 1 wie die Color Film).

use crate::Design;
use crate::aussparen::{Kontur, belegt, schnittflaeche};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Überdeckung erst ab dieser gemeinsamen Fläche (mm²) – **eigene Festlegung**: 1 mm Laufstich (Band 0,5 mm) über einer
/// Fläche zählt, bloßes Berühren am Rand nicht.
pub const MIN_UEBERDECKUNG: f64 = 0.5;

/// Befehl zum Umordnen (Original-Namen in Klammern).
#[derive(Clone, Debug, PartialEq)]
pub enum Befehl {
    /// „Sequence As Selected“: Auswahl in der angegebenen Reihenfolge an die Stelle des ersten ausgewählten Objekts
    WieAusgewaehlt(Vec<usize>),
    /// „Sequence to Start“
    AnAnfang(Vec<usize>),
    /// „Sequence to End“
    AnsEnde(Vec<usize>),
    /// „Resequence by Number“: Auswahl vor (`vor` = true) bzw. nach das Objekt `ziel`
    NachNummer {
        auswahl: Vec<usize>,
        ziel: usize,
        vor: bool,
    },
    /// „Sequence By Color“: Auswahl (leer = alle) nach Farben; `farben` = gewünschte Reihenfolge („Move Up/Down“), nicht
    /// genannte Farben folgen in der Reihenfolge ihres ersten Auftretens; `ueberdeckung` = eigene Erweiterung: Objekte,
    /// die sich überdecken, behalten ihre Reihenfolge untereinander
    NachFarbe {
        auswahl: Vec<usize>,
        farben: Vec<String>,
        ueberdeckung: bool,
    },
}

/// Ergebnis eines Befehls.
#[derive(Clone, Debug, PartialEq)]
pub struct Ergebnis {
    /// neue Reihenfolge: an Stelle k steht das bisherige Objekt `reihenfolge[k]`
    pub reihenfolge: Vec<usize>,
    /// Design-JSON mit umgeordneten `objekte`
    pub design: Value,
    pub farbwechsel_vorher: usize,
    pub farbwechsel_nachher: usize,
    /// Paare (oben, unten) in bisheriger Nummerierung: `oben` lag über `unten` (später gestickt, Flächen überdecken
    /// sich) und liegt nach dem Umordnen darunter. Nur berechnet, wenn ein Design übergeben wurde.
    pub vertauscht: Vec<(usize, usize)>,
}

fn pruefen(n: usize, auswahl: &[usize]) -> Result<(), String> {
    if auswahl.is_empty() {
        return Err("keine Objekte ausgewählt".into());
    }
    let mut gesehen = BTreeSet::new();
    for &i in auswahl {
        if i >= n {
            return Err(format!("Objekt {} gibt es nicht (das Design hat {n})", i + 1));
        }
        if !gesehen.insert(i) {
            return Err(format!("Objekt {} doppelt ausgewählt", i + 1));
        }
    }
    Ok(())
}

/// Die Objekte `block` an Stelle `pos` (gezählt unter den übrigen Objekten) einsetzen.
fn einsetzen(n: usize, block: &[usize], pos: usize) -> Vec<usize> {
    let drin: BTreeSet<usize> = block.iter().copied().collect();
    let rest: Vec<usize> = (0..n).filter(|i| !drin.contains(i)).collect();
    let pos = pos.min(rest.len());
    let mut out = rest[..pos].to_vec();
    out.extend_from_slice(block);
    out.extend_from_slice(&rest[pos..]);
    out
}

/// „Sequence As Selected“ (MOD1 Resequence 0x1006db20): Block in Auswahlreihenfolge an die Stelle des ersten
/// ausgewählten Objekts.
pub fn wie_ausgewaehlt(n: usize, auswahl: &[usize]) -> Result<Vec<usize>, String> {
    pruefen(n, auswahl)?;
    let pos = (0..auswahl[0]).filter(|i| !auswahl.contains(i)).count();
    Ok(einsetzen(n, auswahl, pos))
}

fn sortiert(auswahl: &[usize]) -> Vec<usize> {
    let mut s = auswahl.to_vec();
    s.sort_unstable();
    s
}

/// „Sequence to Start“: Auswahl an den Anfang; untereinander bleibt die bisherige Reihenfolge (eigene Festlegung für
/// mehrere Objekte – das Handbuch beschreibt ein Objekt).
pub fn an_anfang(n: usize, auswahl: &[usize]) -> Result<Vec<usize>, String> {
    pruefen(n, auswahl)?;
    Ok(einsetzen(n, &sortiert(auswahl), 0))
}

/// „Sequence to End“: Auswahl ans Ende (Reihenfolge untereinander wie bisher).
pub fn ans_ende(n: usize, auswahl: &[usize]) -> Result<Vec<usize>, String> {
    pruefen(n, auswahl)?;
    Ok(einsetzen(n, &sortiert(auswahl), n))
}

/// „Resequence by Number“ (DLG 1280 „Before/After the object #“): Auswahl vor bzw. nach Objekt `ziel`.
pub fn nach_nummer(n: usize, auswahl: &[usize], ziel: usize, vor: bool) -> Result<Vec<usize>, String> {
    pruefen(n, auswahl)?;
    if ziel >= n {
        return Err(format!("Objekt {} gibt es nicht (das Design hat {n})", ziel + 1));
    }
    if auswahl.contains(&ziel) {
        return Err(format!("Objekt {} ist selbst ausgewählt", ziel + 1));
    }
    let pos = (0..ziel).filter(|i| !auswahl.contains(i)).count() + usize::from(!vor);
    Ok(einsetzen(n, &sortiert(auswahl), pos))
}

/// Farbe vergleichbar machen (`#RRGGBB` → `#rrggbb`).
pub fn farbe_norm(f: &str) -> String {
    f.trim().to_ascii_lowercase()
}

/// Rang jeder Farbe: zuerst die gewünschten (`farben`), dann die übrigen in der Reihenfolge ihres ersten Auftretens.
fn farb_rang(farben: &[String], auswahl: &[usize], wunsch: &[String]) -> BTreeMap<String, usize> {
    let mut rang = BTreeMap::new();
    for f in wunsch
        .iter()
        .map(|f| farbe_norm(f))
        .chain(auswahl.iter().map(|&i| farbe_norm(&farben[i])))
    {
        let k = rang.len();
        rang.entry(f).or_insert(k);
    }
    rang
}

/// „Sequence By Color“: Auswahl (leer = alle) nach Farben ordnen, innerhalb einer Farbe bleibt die Reihenfolge. Der Block
/// kommt an die Stelle des ersten ausgewählten Objekts (eigene Festlegung wie „Sequence As Selected“ – der Befehl
/// selbst ist im verschlüsselten ARTISTA.EXE). Mit `ueberdeckung` (eigene Erweiterung) bleiben Objekte, die sich
/// überdecken (`paare` aus `ueberdeckungen`), in ihrer Reihenfolge; sonst wird die aktuelle Farbe fortgesetzt, solange
/// ein Objekt dieser Farbe frei ist, dann die nächste Farbe nach Rang.
pub fn nach_farbe(
    farben: &[String],
    auswahl: &[usize],
    wunsch: &[String],
    paare: Option<&[(usize, usize)]>,
) -> Result<Vec<usize>, String> {
    let n = farben.len();
    let auswahl: Vec<usize> = if auswahl.is_empty() {
        (0..n).collect()
    } else {
        pruefen(n, auswahl)?;
        sortiert(auswahl)
    };
    if auswahl.is_empty() {
        return Ok(Vec::new());
    }
    let rang = farb_rang(farben, &auswahl, wunsch);
    let r = |i: usize| rang[&farbe_norm(&farben[i])];
    let block: Vec<usize> = match paare {
        None => {
            let mut b = auswahl.clone();
            b.sort_by_key(|&i| (r(i), i));
            b
        }
        Some(paare) => {
            // Kahn: ein Objekt ist frei, wenn alle früheren Objekte der Auswahl, die es überdeckt, gestickt sind
            let drin: BTreeSet<usize> = auswahl.iter().copied().collect();
            let mut vorher: BTreeMap<usize, usize> = auswahl.iter().map(|&i| (i, 0)).collect();
            let mut nachher: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for &(a, b) in paare {
                let (a, b) = (a.min(b), a.max(b));
                if a != b && drin.contains(&a) && drin.contains(&b) {
                    *vorher.get_mut(&b).unwrap() += 1;
                    nachher.entry(a).or_default().push(b);
                }
            }
            let mut frei: BTreeSet<usize> = auswahl.iter().copied().filter(|i| vorher[i] == 0).collect();
            let mut b = Vec::with_capacity(auswahl.len());
            let mut farbe: Option<usize> = None;
            while !frei.is_empty() {
                let wahl = farbe
                    .and_then(|f| frei.iter().copied().find(|&i| r(i) == f))
                    .unwrap_or_else(|| *frei.iter().min_by_key(|&&i| (r(i), i)).unwrap());
                frei.remove(&wahl);
                farbe = Some(r(wahl));
                b.push(wahl);
                for &k in nachher.get(&wahl).into_iter().flatten() {
                    let z = vorher.get_mut(&k).unwrap();
                    *z -= 1;
                    if *z == 0 {
                        frei.insert(k);
                    }
                }
            }
            b
        }
    };
    let pos = (0..auswahl[0]).filter(|i| !auswahl.contains(i)).count();
    Ok(einsetzen(n, &block, pos))
}

/// Anzahl Farbwechsel in der Reihenfolge `folge` (Objektfarben, `farbe_norm`).
pub fn farbwechsel(farben: &[String], folge: &[usize]) -> usize {
    folge
        .windows(2)
        .filter(|w| farbe_norm(&farben[w[0]]) != farbe_norm(&farben[w[1]]))
        .count()
}

/// Paare (i, j), i < j, von Objekten des Designs (Stelle in `objekte`), deren belegte Flächen sich überdecken
/// (gemeinsame Fläche ≥ `MIN_UEBERDECKUNG`; Laufstich als Band, `aussparen::belegt`). Texte zählen mit ihren
/// Buchstaben, Aussparungen sind berücksichtigt (ausgesparte Flächen überdecken die Schrift nicht). Eigene Erweiterung.
pub fn ueberdeckungen(d: &Design) -> Result<Vec<(usize, usize)>, String> {
    let (objekte, _, herkunft) = d.aufgeloest_mit_herkunft()?;
    type Teil = (usize, Vec<Kontur>, [f64; 4]);
    let teile: Vec<Teil> = objekte
        .iter()
        .zip(&herkunft)
        .filter_map(|(o, &h)| {
            let k = belegt(&o.art);
            let mut b = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
            for q in k.iter().flatten() {
                b = [b[0].min(q[0]), b[1].min(q[1]), b[2].max(q[0]), b[3].max(q[1])];
            }
            (!k.is_empty()).then_some((h, k, b))
        })
        .collect();
    let mut paare = BTreeSet::new();
    for (x, (ha, ka, ba)) in teile.iter().enumerate() {
        for (hb, kb, bb) in &teile[x + 1..] {
            if ha == hb || paare.contains(&(*ha.min(hb), *ha.max(hb))) {
                continue;
            }
            if ba[0] > bb[2] || bb[0] > ba[2] || ba[1] > bb[3] || bb[1] > ba[3] {
                continue;
            }
            if schnittflaeche(ka, kb) >= MIN_UEBERDECKUNG {
                paare.insert((*ha.min(hb), *ha.max(hb)));
            }
        }
    }
    Ok(paare.into_iter().collect())
}

/// Überdeckende Paare, deren Reihenfolge `folge` umkehrt: (oben, unten) in bisheriger Nummerierung – `oben` wurde
/// bisher später gestickt (lag obenauf) und kommt jetzt vor `unten`.
pub fn vertauscht(paare: &[(usize, usize)], folge: &[usize]) -> Vec<(usize, usize)> {
    let mut stelle = vec![0; folge.len()];
    for (k, &i) in folge.iter().enumerate() {
        stelle[i] = k;
    }
    paare
        .iter()
        .filter(|&&(a, b)| (a < b) == (stelle[a] > stelle[b]))
        .map(|&(a, b)| (a.max(b), a.min(b)))
        .collect()
}

/// Farben der Objekte aus dem Design-JSON (`farbe`, Standard schwarz wie im Kern).
pub fn objekt_farben(v: &Value) -> Vec<String> {
    v.get("objekte")
        .and_then(Value::as_array)
        .map(|l| {
            l.iter()
                .map(|o| farbe_norm(o.get("farbe").and_then(Value::as_str).unwrap_or("#000000")))
                .collect()
        })
        .unwrap_or_default()
}

/// `objekte` des Design-JSON in der Reihenfolge `folge` (alle übrigen Angaben bleiben; Bezüge zwischen Objekten
/// laufen über Namen, z. B. `aussparen`).
pub fn anwenden(v: &Value, folge: &[usize]) -> Result<Value, String> {
    let mut v = v.clone();
    let liste = v
        .get_mut("objekte")
        .and_then(Value::as_array_mut)
        .ok_or("Design ohne „objekte“")?;
    if folge.len() != liste.len() || folge.iter().collect::<BTreeSet<_>>().len() != liste.len() {
        return Err("Reihenfolge passt nicht zu den Objekten".into());
    }
    let alt = std::mem::take(liste);
    *liste = folge.iter().map(|&i| alt[i].clone()).collect();
    Ok(v)
}

/// Befehl auf ein Design-JSON anwenden. `design` (aus demselben JSON, mit geladenen Alphabeten/Schriften) wird für die
/// Prüfung auf Überdeckungen gebraucht; ohne Design bleibt `vertauscht` leer, `ueberdeckung` geht dann nicht.
pub fn ausfuehren(v: &Value, design: Option<&Design>, befehl: &Befehl) -> Result<Ergebnis, String> {
    let farben = objekt_farben(v);
    let n = farben.len();
    let paare = match design {
        Some(d) => Some(ueberdeckungen(d)?),
        None => None,
    };
    let folge = match befehl {
        Befehl::WieAusgewaehlt(a) => wie_ausgewaehlt(n, a)?,
        Befehl::AnAnfang(a) => an_anfang(n, a)?,
        Befehl::AnsEnde(a) => ans_ende(n, a)?,
        Befehl::NachNummer { auswahl, ziel, vor } => nach_nummer(n, auswahl, *ziel, *vor)?,
        Befehl::NachFarbe {
            auswahl,
            farben: wunsch,
            ueberdeckung,
        } => {
            let p = if *ueberdeckung {
                Some(
                    paare
                        .as_deref()
                        .ok_or("Sortieren mit Überdeckung braucht das berechenbare Design")?,
                )
            } else {
                None
            };
            nach_farbe(&farben, auswahl, wunsch, p)?
        }
    };
    let alt: Vec<usize> = (0..n).collect();
    Ok(Ergebnis {
        design: anwenden(v, &folge)?,
        farbwechsel_vorher: farbwechsel(&farben, &alt),
        farbwechsel_nachher: farbwechsel(&farben, &folge),
        vertauscht: paare.as_deref().map(|p| vertauscht(p, &folge)).unwrap_or_default(),
        reihenfolge: folge,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(l: &[&str]) -> Vec<String> {
        l.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn wie_ausgewaehlt_an_stelle_des_ersten() {
        // Auswahl 5, 2, 7 (in dieser Klickfolge) → Block an der Stelle von 5
        assert_eq!(wie_ausgewaehlt(8, &[5, 2, 7]).unwrap(), vec![0, 1, 3, 4, 5, 2, 7, 6]);
        assert_eq!(wie_ausgewaehlt(4, &[3, 0]).unwrap(), vec![1, 2, 3, 0]);
        assert!(wie_ausgewaehlt(4, &[1, 1]).is_err());
        assert!(wie_ausgewaehlt(4, &[4]).is_err());
    }

    #[test]
    fn anfang_ende_nummer() {
        assert_eq!(an_anfang(5, &[3, 1]).unwrap(), vec![1, 3, 0, 2, 4]);
        assert_eq!(ans_ende(5, &[3, 1]).unwrap(), vec![0, 2, 4, 1, 3]);
        assert_eq!(nach_nummer(5, &[0], 3, true).unwrap(), vec![1, 2, 0, 3, 4]);
        assert_eq!(nach_nummer(5, &[0], 3, false).unwrap(), vec![1, 2, 3, 0, 4]);
        assert_eq!(nach_nummer(5, &[4, 3], 0, true).unwrap(), vec![3, 4, 0, 1, 2]);
        assert!(nach_nummer(5, &[1], 1, true).is_err());
    }

    #[test]
    fn farbe_sortiert_stabil() {
        let fa = f(&["#ff0000", "#000000", "#FF0000", "#00ff00", "#000000"]);
        let alle = nach_farbe(&fa, &[], &[], None).unwrap();
        assert_eq!(alle, vec![0, 2, 1, 4, 3]);
        assert_eq!(farbwechsel(&fa, &[0, 1, 2, 3, 4]), 4);
        assert_eq!(farbwechsel(&fa, &alle), 2);
        // gewünschte Reihenfolge: schwarz zuerst („Move Up“)
        assert_eq!(
            nach_farbe(&fa, &[], &f(&["#000000"]), None).unwrap(),
            vec![1, 4, 0, 2, 3]
        );
        // nur 2, 3, 4 ausgewählt, schwarz zuerst: Block an der Stelle von 2
        assert_eq!(
            nach_farbe(&fa, &[4, 3, 2], &f(&["#000000"]), None).unwrap(),
            vec![0, 1, 4, 2, 3]
        );
    }

    #[test]
    fn farbe_mit_ueberdeckung() {
        // 0 rot (Fläche), 1 schwarz (Kontur über 0), 2 rot (Fläche über 1), 3 schwarz (frei)
        let fa = f(&["r", "s", "r", "s"]);
        let paare = [(0, 1), (1, 2)];
        let ohne = nach_farbe(&fa, &[], &[], None).unwrap();
        assert_eq!(ohne, vec![0, 2, 1, 3]);
        assert_eq!(vertauscht(&paare, &ohne), vec![(2, 1)]);
        let mit = nach_farbe(&fa, &[], &[], Some(&paare)).unwrap();
        assert_eq!(mit, vec![0, 1, 3, 2]);
        assert!(vertauscht(&paare, &mit).is_empty());
        assert_eq!(farbwechsel(&fa, &mit), 2);
    }

    #[test]
    fn ueberdeckung_im_design() {
        let json = r##"{"objekte": [
            {"art": "tatami", "farbe": "#ff0000", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 20, "hoehe": 20}}},
            {"art": "lauf", "farbe": "#000000", "punkte": [[-15, 0], [15, 0]]},
            {"art": "tatami", "farbe": "#ff0000", "umriss": {"rechteck": {"mitte": [40, 0], "breite": 10, "hoehe": 10}}},
            {"art": "satinlinie", "farbe": "#000000", "punkte": [[40, -10], [40, 10]], "breite": 3},
            {"art": "lauf", "farbe": "#000000", "punkte": [[-15, 30], [15, 30]]}
        ]}"##;
        let v: Value = serde_json::from_str(json).unwrap();
        let d = Design::aus_value(&v).unwrap();
        assert_eq!(ueberdeckungen(&d).unwrap(), vec![(0, 1), (2, 3)]);
        let e = ausfuehren(
            &v,
            Some(&d),
            &Befehl::NachFarbe {
                auswahl: vec![],
                farben: vec![],
                ueberdeckung: false,
            },
        )
        .unwrap();
        assert_eq!(e.reihenfolge, vec![0, 2, 1, 3, 4]);
        assert!(
            e.vertauscht.is_empty(),
            "rot vor schwarz: beide Konturen bleiben obenauf"
        );
        assert_eq!((e.farbwechsel_vorher, e.farbwechsel_nachher), (3, 1));
        let schwarz_zuerst = Befehl::NachFarbe {
            auswahl: vec![],
            farben: vec!["#000000".into()],
            ueberdeckung: false,
        };
        let e = ausfuehren(&v, Some(&d), &schwarz_zuerst).unwrap();
        assert_eq!(e.reihenfolge, vec![1, 3, 4, 0, 2]);
        assert_eq!(e.vertauscht, vec![(1, 0), (3, 2)]);
        let farben: Vec<&str> = e.design["objekte"]
            .as_array()
            .unwrap()
            .iter()
            .map(|o| o["farbe"].as_str().unwrap())
            .collect();
        assert_eq!(farben, ["#000000", "#000000", "#000000", "#ff0000", "#ff0000"]);
        // mit Überdeckung: Konturen bleiben über ihren Flächen, nur die freie Linie rückt nach vorn
        let e = ausfuehren(
            &v,
            Some(&d),
            &Befehl::NachFarbe {
                auswahl: vec![],
                farben: vec!["#000000".into()],
                ueberdeckung: true,
            },
        )
        .unwrap();
        assert_eq!(e.reihenfolge, vec![4, 0, 2, 1, 3]);
        assert!(e.vertauscht.is_empty());
        assert_eq!(e.farbwechsel_nachher, 2);
    }
}
