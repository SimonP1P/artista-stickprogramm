//! **Wirksame Werte eines Objekts** (Wunsch Nutzer 21.11.2026: „bei den Experteneinstellungen immer aktuelle Werte
//! reinschreiben“): für Felder, die im Design leer sind, der Wert, mit dem der Kern tatsächlich rechnet, und woher er
//! kommt. Die Auflösung folgt genau der Stichberechnung (`design.rs`, `schrift.rs`, `motiv.rs` …): Stoff → Design →
//! Objekt (`einstellungen`), Step-Muster des Objekts, Alphabet, Füllmuster, Motivsatz, Vorlagewerte des Originals.
//! Nur Anzeige – am Design ändert sich nichts.

use crate::design::{Design, ObjektArt};
use crate::einstellungen::{Einstellungen, STEP_MUSTER, auto_spacing};
use crate::format::{KnopflochTyp, ObjektJson, SchriftStichart, TtfStichart};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::{Value, json};

/// Wert eines Felds, mit dem der Kern rechnet, wenn das Feld leer ist.
#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
pub struct WirksamerWert {
    /// Feld des Objekts wie im Design-JSON (z. B. „winkel“, „step_muster“)
    pub feld: String,
    /// Wert (Zahl in der Einheit des Felds, Wahrheitswert oder Text); fehlt, wenn es nicht **ein** Wert ist (z. B.
    /// Auto Spacing ändert den Abstand mit der Breite) – dann steht die Anzeige in `text`
    pub wert: Option<Value>,
    /// Anzeige statt `wert`, z. B. „auto 0,40–0,51“
    pub text: Option<String>,
    /// Herkunft: `stil` (Stoff/Design/Objekt-Einstellungen), `alphabet`, `muster` (Füllmuster), `motivsatz`,
    /// `vorlage` (feste Vorlage des Originals), `schrift` (Computer-Schrift), `handbuch` (Beispielwert des Handbuchs)
    pub herkunft: String,
}

fn wert(feld: &str, w: Value, herkunft: &str) -> WirksamerWert {
    WirksamerWert {
        feld: feld.into(),
        wert: Some(w),
        text: None,
        herkunft: herkunft.into(),
    }
}
fn text(feld: &str, t: String, herkunft: &str) -> WirksamerWert {
    WirksamerWert {
        feld: feld.into(),
        wert: None,
        text: Some(t),
        herkunft: herkunft.into(),
    }
}
/// auf 2 Stellen gerundet (Anzeige)
fn r2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
fn zahl_de(x: f64) -> String {
    format!("{:.2}", x).replace('.', ",")
}

/// Nummer des Step-Musters (1–20), das die Tatami-Einstellung ergibt; None = eigener Versatz.
pub fn step_muster_nr(e: &Einstellungen) -> Option<usize> {
    let t = &e.tatami;
    if t.zufall != 0.0 {
        return None;
    }
    let v2 = t.versatz2.unwrap_or(t.versatz);
    STEP_MUSTER
        .iter()
        .position(|&(a, b)| (a - t.versatz).abs() < 1e-9 && (b - v2).abs() < 1e-9)
        .map(|k| k + 1)
}

/// Satinabstand mit Auto Spacing über eine Breitenspanne (mm) als Anzeige „auto a–b“.
fn auto_text(e: &Einstellungen, faktor: Option<f64>, breiten: Option<(f64, f64)>) -> Option<String> {
    if !e.satin.auto_spacing {
        return None;
    }
    let f = faktor.unwrap_or(e.satin.faktor);
    Some(match breiten {
        Some((lo, hi)) => {
            let (a, b) = (auto_spacing(hi, f), auto_spacing(lo, f));
            if (a - b).abs() < 0.005 {
                format!("auto {}", zahl_de(a))
            } else {
                format!("auto {}–{}", zahl_de(a), zahl_de(b))
            }
        }
        None => "auto nach Breite".into(),
    })
}

/// Wirksame Werte des Objekts `nr` (Felder des Objekts im Design-JSON `v`, aufgelöstes Design `d`).
pub fn objekt_werte(d: &Design, v: &Value, nr: usize) -> Result<Vec<WirksamerWert>, String> {
    let roh = v
        .get("objekte")
        .and_then(|o| o.get(nr))
        .ok_or_else(|| format!("Objekt {} gibt es nicht", nr + 1))?;
    let o: ObjektJson = serde_json::from_value(roh.clone()).map_err(|e| format!("Objekt {}: {e}", nr + 1))?;
    let basis = d.einstellungen_fuer(nr);
    // Step-Muster des Objekts wirkt wie in der Stichberechnung auf die Tatami-Werte
    let mut eigene = basis.clone();
    if let Some(&s) = d.step_muster.get(&nr) {
        eigene.tatami.step_muster(s)?;
    }
    let e = &eigene;
    let aufgeloest = d.objekte.get(nr).map(|x| &x.art);
    let mut w = Vec::new();
    let step = |w: &mut Vec<WirksamerWert>| match step_muster_nr(basis) {
        Some(n) => w.push(wert("step_muster", json!(n), "stil")),
        // kein Muster des Dialogs (z. B. Stoffe mit 0,25/0,75): Versatz der Reihen nennen
        None => w.push(text(
            "step_muster",
            format!(
                "Versatz {}/{}",
                zahl_de(basis.tatami.versatz),
                zahl_de(basis.tatami.versatz2.unwrap_or(basis.tatami.versatz))
            ),
            "stil",
        )),
    };
    let muster_winkel = |name: Option<&str>| {
        name.and_then(crate::muster::finden)
            .and_then(|m| m.winkel)
            .map(|x| (x, "muster"))
            .unwrap_or((e.tatami.winkel, "stil"))
    };
    match &o {
        ObjektJson::Lauf(_) | ObjektJson::Dreifach(_) => w.push(wert("laenge", json!(e.lauf.laenge), "stil")),
        ObjektJson::Rueckstich(_) => rueckstich_werte(&mut w),
        ObjektJson::Stielstich(_) => stielstich_werte(&mut w),
        ObjektJson::Stiche(_) => {}
        ObjektJson::Photosnap(_) => {
            use crate::photosnap as ps;
            w.push(wert("aufloesung", json!("mittel"), "vorlage"));
            w.push(wert("hintergrund", json!("hell"), "vorlage"));
            w.push(wert("lagen", json!("gekreuzt"), "vorlage"));
            w.push(wert("winkel", json!(0.0), "vorlage"));
            w.push(wert("abstand_min", json!(ps::ABSTAND_MIN), "vorlage"));
            w.push(wert("abstand_max", json!(ps::ABSTAND_MAX), "vorlage"));
        }
        ObjektJson::Stippling(_) => {
            use crate::sticharten::stippling as s;
            w.push(wert("laenge", json!(s::LAENGE), "handbuch"));
            w.push(wert("abstand", json!(s::ABSTAND), "handbuch"));
        }
        ObjektJson::Blackwork(b) => match b.stich {
            crate::format::BlackworkStich::Lauf | crate::format::BlackworkStich::Dreifach => {
                w.push(wert("laenge", json!(e.lauf.laenge), "stil"))
            }
            crate::format::BlackworkStich::Rueckstich => rueckstich_werte(&mut w),
            crate::format::BlackworkStich::Stielstich => stielstich_werte(&mut w),
        },
        ObjektJson::Satin(s) => {
            let breiten = match aufgeloest {
                Some(ObjektArt::Satin { seite_a, seite_b, .. }) if !seite_a.is_empty() => {
                    let b: Vec<f64> = seite_a
                        .iter()
                        .zip(seite_b)
                        .map(|(a, b)| crate::geometrie::abstand(*a, *b))
                        .collect();
                    Some((b.iter().cloned().fold(f64::INFINITY, f64::min), b.iter().cloned().fold(0.0, f64::max)))
                }
                _ => None,
            };
            match auto_text(e, s.faktor, breiten) {
                Some(t) => w.push(text("abstand", t, "stil")),
                None => w.push(wert("abstand", json!(e.satin.abstand_fest), "stil")),
            }
            w.push(wert("faktor", json!(e.satin.faktor), "stil"));
            w.push(wert("kurzstiche", json!(e.satin.kurzstiche.an), "stil"));
        }
        ObjektJson::Satinflaeche(s) => {
            match auto_text(e, s.faktor, None) {
                Some(t) => w.push(text("abstand", t, "stil")),
                None => w.push(wert("abstand", json!(e.satin.abstand_fest), "stil")),
            }
            w.push(wert("faktor", json!(e.satin.faktor), "stil"));
            w.push(wert("kurzstiche", json!(e.satin.kurzstiche.an), "stil"));
            w.push(wert("knickwinkel", json!(0.0), "vorlage"));
        }
        ObjektJson::Satinlinie(_) => w.push(wert("breite", json!(2.0), "vorlage")),
        ObjektJson::Blanket(_) => {
            w.push(wert("abstand", json!(crate::sticharten::blanket::ABSTAND), "vorlage"));
            w.push(wert("breite", json!(crate::sticharten::blanket::BREITE), "vorlage"));
        }
        ObjektJson::Tatami(_) => {
            w.push(wert("winkel", json!(e.tatami.winkel), "stil"));
            w.push(wert("unterlage", json!(e.unterlage.auto_an), "stil"));
            w.push(wert("kurzstiche", json!(e.satin.kurzstiche.an), "stil"));
            step(&mut w);
        }
        ObjektJson::Muster(m) => {
            let (x, h) = muster_winkel(Some(&m.muster));
            w.push(wert("winkel", json!(x), h));
            w.push(wert("unterlage", json!(e.unterlage.auto_an), "stil"));
        }
        ObjektJson::Text(t) => {
            if let Some(ObjektArt::Text { alphabet, vorgabe }) = aufgeloest {
                if let Some(a) = d
                    .alphabete
                    .get(alphabet)
                    .or_else(|| d.alphabete.values().find(|a| a.name.eq_ignore_ascii_case(alphabet)))
                {
                    w.push(wert("abstand", json!(r2(a.abstand_anteil * vorgabe.hoehe)), "alphabet"));
                }
            }
            schrift_werte(&mut w, t.schrift.muster.as_deref(), t.stichart == Some(SchriftStichart::Muster), &muster_winkel);
            step(&mut w);
        }
        ObjektJson::Ttf(t) => {
            w.push(text("abstand", "0 (wie die Schrift)".into(), "schrift"));
            if t.stichart == TtfStichart::Tatami {
                w.push(wert("winkel", json!(e.tatami.winkel), "stil"));
            }
            w.push(wert("laenge", json!(e.lauf.laenge), "stil"));
            schrift_werte(
                &mut w,
                t.schrift_optionen.muster.as_deref(),
                t.stichart == TtfStichart::Muster,
                &muster_winkel,
            );
            step(&mut w);
        }
        ObjektJson::Monogramm(m) => {
            // Diamond: Text mit Diamond2/3 (aufgelöst); mit Vorlage setzt der Kern jede Initiale einzeln
            if m.vorlage.is_none() {
                if let Some(ObjektArt::Text { alphabet, vorgabe }) = aufgeloest {
                    if let Some(a) = d
                        .alphabete
                        .get(alphabet)
                        .or_else(|| d.alphabete.values().find(|a| a.name.eq_ignore_ascii_case(alphabet)))
                    {
                        w.push(wert("abstand", json!(r2(a.abstand_anteil * vorgabe.hoehe)), "alphabet"));
                    }
                }
            }
            schrift_werte(&mut w, m.schrift.muster.as_deref(), m.stichart == Some(SchriftStichart::Muster), &muster_winkel);
            step(&mut w);
        }
        ObjektJson::Applikation(_) => {
            if let Some(ObjektArt::Applikation(a)) = aufgeloest {
                w.push(wert("abdeckung", json!(if a.blanket { "blanket" } else { "satin" }), "vorlage"));
                w.push(wert("stil", json!(if a.zuschneiden { "zuschneiden" } else { "vorgeschnitten" }), "vorlage"));
                w.push(wert("breite", json!(a.breite), "vorlage"));
                w.push(wert("heft_breite", json!(a.heft_breite), "vorlage"));
                w.push(wert("heft_abstand", json!(a.heft_abstand), "vorlage"));
                w.push(wert("platz_laenge", json!(a.platz_laenge), "vorlage"));
                let abstand = a.abstand.unwrap_or(if a.blanket {
                    crate::sticharten::blanket::ABSTAND
                } else {
                    crate::applikation::ABSTAND
                });
                w.push(wert("abstand", json!(abstand), "vorlage"));
                w.push(wert("versatz", json!(r2(a.versatz * 100.0)), "vorlage"));
            }
        }
        ObjektJson::Knopfloch(k) => {
            let a = if k.typ == KnopflochTyp::Heirloom { 1.5 } else { crate::knopfloch::STANDARD_ABSTAND };
            w.push(wert("abstand", json!(a), "vorlage"));
        }
        ObjektJson::Motivlinie(m) => {
            let satz = d.motivsaetze.get(&m.satz).or_else(|| {
                d.motivsaetze
                    .values()
                    .find(|s| s.name.trim_start_matches('_').eq_ignore_ascii_case(m.satz.trim_start_matches('_')))
            });
            if let Some(z) = satz.and_then(|s| s.motive.get(&m.motiv)) {
                let [x1, y1, x2, y2] = z.grundlinie;
                let g = r2((x2 - x1).hypot(y2 - y1) / crate::schrift::EINHEIT);
                w.push(wert("groesse", json!(g), "motivsatz"));
                w.push(wert("abstand", json!(m.groesse.map(r2).unwrap_or(g)), "motivsatz"));
            }
        }
        ObjektJson::Motivfuellung(m) => {
            let satz = d.motivsaetze.get(&m.satz).or_else(|| {
                d.motivsaetze
                    .values()
                    .find(|s| s.name.trim_start_matches('_').eq_ignore_ascii_case(m.satz.trim_start_matches('_')))
            });
            if let Some(z) = satz.and_then(|s| s.motive.get(&m.motiv)) {
                let [x1, y1, x2, y2] = z.grundlinie;
                let l0 = (x2 - x1).hypot(y2 - y1) / crate::schrift::EINHEIT;
                w.push(wert("groesse", json!(r2(l0)), "motivsatz"));
                if let Some((a, b, c, e)) = crate::motiv::motiv_kasten(z) {
                    let f = m.groesse.map_or(1.0, |g| g / l0);
                    w.push(wert("spaltenabstand", json!(r2((b - a) * f)), "motivsatz"));
                    w.push(wert("reihenabstand", json!(r2((e - c) * f)), "motivsatz"));
                }
            }
        }
    }
    Ok(w)
}

/// Rückstich: Werte des Stils 0x2850 (Überlappung in %).
fn rueckstich_werte(w: &mut Vec<WirksamerWert>) {
    use crate::sticharten::handstich as h;
    w.push(wert("laenge", json!(h::RUECK_LAENGE), "stil"));
    w.push(wert("staerke", json!(h::RUECK_STAERKE), "stil"));
    w.push(wert("ueberlappung", json!(h::RUECK_UEBERLAPPUNG * 100.0), "stil"));
    w.push(wert("striche", json!(h::RUECK_STRICHE), "stil"));
}

/// Stielstich: Werte des Stils 0x2855.
fn stielstich_werte(w: &mut Vec<WirksamerWert>) {
    use crate::sticharten::handstich as h;
    w.push(wert("breite", json!(h::STIEL_BREITE), "stil"));
    w.push(wert("abstand", json!(h::STIEL_ABSTAND), "stil"));
    w.push(wert("winkel", json!(h::STIEL_WINKEL), "stil"));
    w.push(wert("dreifach", json!(h::STIEL_DREIFACH), "stil"));
    w.push(wert("staerke", json!(h::STIEL_STAERKE), "stil"));
}

/// Werte, die Text, Computer-Schrift und Monogramm gemeinsam haben: Wort-/Zeilenabstand (Vorlage), Mustergröße und
/// Musterwinkel (nur bei Stichart „muster“).
fn schrift_werte(
    w: &mut Vec<WirksamerWert>,
    muster: Option<&str>,
    ist_muster: bool,
    muster_winkel: &dyn Fn(Option<&str>) -> (f64, &'static str),
) {
    w.push(wert("wortabstand", json!(crate::schrift::WORTABSTAND_ANTEIL * 100.0), "vorlage"));
    w.push(wert("zeilenabstand", json!(crate::schrift::ZEILENABSTAND_ANTEIL * 100.0), "vorlage"));
    w.push(wert("groesse", json!(100.0), "vorlage"));
    if ist_muster {
        let (x, h) = muster_winkel(muster);
        w.push(wert("winkel", json!(x), h));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn werte(v: Value) -> Vec<WirksamerWert> {
        let d = Design::aus_value(&v).unwrap();
        objekt_werte(&d, &v, 0).unwrap()
    }
    fn finden<'a>(w: &'a [WirksamerWert], f: &str) -> &'a WirksamerWert {
        w.iter().find(|x| x.feld == f).unwrap_or_else(|| panic!("{f} fehlt"))
    }

    /// Tatami: Stichwinkel 45° und Step-Muster 1 der Software Defaults; Step-Muster des Objekts zählt nicht als Stil
    #[test]
    fn tatami_aus_dem_stil() {
        let w = werte(json!({"rahmen": "artista 130 x 100", "objekte": [
            {"art": "tatami", "umriss": [[0, 0], [20, 0], [20, 20], [0, 20]]}]}));
        assert_eq!(finden(&w, "winkel").wert, Some(json!(45.0)));
        assert_eq!(finden(&w, "step_muster").wert, Some(json!(1)));
        // Objekt-Einstellung überschreibt den Stil
        let w = werte(json!({"rahmen": "artista 130 x 100", "objekte": [
            {"art": "tatami", "umriss": [[0, 0], [20, 0], [20, 20], [0, 20]], "einstellungen": {"tatami": {"winkel": 30}}}]}));
        assert_eq!(finden(&w, "winkel").wert, Some(json!(30.0)));
    }

    /// Satin: Auto Spacing über die Breiten der Säule (2 bis 6 mm)
    #[test]
    fn satin_auto_spacing() {
        let w = werte(json!({"rahmen": "artista 130 x 100", "objekte": [
            {"art": "satin", "seite_a": [[0, 0], [20, 0]], "seite_b": [[0, 2], [20, 6]]}]}));
        let a = finden(&w, "abstand");
        assert!(a.wert.is_none());
        let erwartet = format!("auto {}–{}", zahl_de(auto_spacing(6.0, 1.0)), zahl_de(auto_spacing(2.0, 1.0)));
        assert_eq!(a.text.as_deref(), Some(erwartet.as_str()));
    }

    /// Vorlagewerte: Blanket, Knopfloch Heirloom
    #[test]
    fn vorlagen() {
        let w = werte(json!({"rahmen": "artista 130 x 100", "objekte": [
            {"art": "blanket", "punkte": [[0, 0], [20, 0]]}]}));
        assert_eq!(finden(&w, "abstand").wert, Some(json!(crate::sticharten::blanket::ABSTAND)));
        let w = werte(json!({"rahmen": "artista 130 x 100", "objekte": [{"art": "knopfloch", "typ": "heirloom"}]}));
        assert_eq!(finden(&w, "abstand").wert, Some(json!(1.5)));
    }
}
