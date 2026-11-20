//! Parameterkatalog (ROADMAP M1): alle einstellbaren Werte mit Standard, Bereich, Quelle und **Modus** – `einfach`
//! (im einfachen Modus der Oberfläche sichtbar), `experte` (nur im Expertenmodus), `automatisch` (wird nicht
//! angezeigt; fester oder abgeleiteter Wert, nur dokumentiert). Für Oberfläche und Backend dieselbe Liste.
//!
//! Daten: `daten/parameter.json`, erzeugt von `Arbeitsdaten/werkzeuge/parameter_katalog.py`. Lesbare Fassung:
//! `stick parameter --markdown` → `PARAMETER.md`. Die Tests prüfen, dass jeder Wert unter `einstellungen` im Katalog
//! steht, sein Standard mit dem Code übereinstimmt und „stoffabhängig“ stimmt – so veraltet der Katalog nicht.

use crate::einstellungen::{Einstellungen, STOFFE, fuer_stoff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Ein einstellbarer Wert des Parameterkatalogs (`PARAMETER.md`).
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    /// eindeutiger Schlüssel, z. B. „satin.abstand_fest“
    pub id: String,
    pub gruppe: String,
    pub name: String,
    pub wirkung: String,
    /// design | objekt | beide | werkzeug
    pub ebene: String,
    /// Objektarten (`art` im Design-JSON), für die der Parameter gilt
    pub gilt_fuer: Vec<String>,
    /// Pfad im Design-JSON (z. B. „einstellungen.satin.abstand_fest“, „transform.drehen“, „stoff“)
    pub json_design: Option<String>,
    /// Feld am Objekt (z. B. „abstand“, „auto_jump.max“)
    pub json_objekt: Option<String>,
    pub einheit: String,
    /// Standard in „0 Software Defaults“ (null = ohne Angabe / vom Stil)
    pub standard: Value,
    pub bereich: Option<[f64; 2]>,
    pub werte: Vec<String>,
    /// ändert sich mit dem Stoff (Werte je Stoff: `wert(&fuer_stoff(name)?, pfad)`)
    pub stoff: bool,
    /// Fundstelle im Original oder „eigene Festlegung“
    pub quelle: String,
    /// einfach | experte | automatisch
    pub modus: String,
}

#[derive(Deserialize)]
struct Datei {
    stand: String,
    parameter: Vec<Parameter>,
}

fn datei() -> Datei {
    serde_json::from_str(include_str!("../daten/parameter.json")).expect("daten/parameter.json ungültig")
}

/// Alle Parameter in Katalogreihenfolge (nach Gruppen).
pub fn katalog() -> Vec<Parameter> {
    datei().parameter
}

/// Stand des Katalogs (Datum).
pub fn stand() -> String {
    datei().stand
}

/// Wert eines Design-Parameters (`einstellungen.…`) in den gegebenen Einstellungen.
pub fn wert(e: &Einstellungen, pfad: &str) -> Option<Value> {
    let rest = pfad.strip_prefix("einstellungen.")?;
    let mut v = serde_json::to_value(e).ok()?;
    for teil in rest.split('.') {
        v = v.get(teil)?.clone();
    }
    Some(v)
}

/// Werte je Stoff (Name, Wert) für einen stoffabhängigen Design-Parameter.
pub fn je_stoff(p: &Parameter) -> Vec<(String, Value)> {
    let Some(pfad) = &p.json_design else {
        return Vec::new();
    };
    STOFFE
        .iter()
        .filter_map(|s| Some((s.name.to_string(), wert(&fuer_stoff(s.name).ok()?, pfad)?)))
        .collect()
}

/// Zwei JSON-Werte gleich (Zahlen auf 1e-9 genau)?
pub fn gleich(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            (x.as_f64().unwrap_or(f64::NAN) - y.as_f64().unwrap_or(f64::NAN)).abs() < 1e-9
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| gleich(p, q)),
        _ => a == b,
    }
}

/// Wert lesbar (deutsches Komma, an/aus, „–“ für ohne Angabe).
pub fn text(v: &Value) -> String {
    match v {
        Value::Null => "–".into(),
        Value::Bool(b) => if *b { "an" } else { "aus" }.into(),
        Value::Number(n) => {
            let x = n.as_f64().unwrap_or(0.0);
            let t = if (x - x.round()).abs() < 1e-9 {
                format!("{}", x.round() as i64)
            } else {
                let t = format!("{x:.3}");
                t.trim_end_matches('0').to_string()
            };
            t.replace('.', ",")
        }
        Value::String(s) => s.clone(),
        Value::Array(a) => format!("[{}]", a.iter().map(text).collect::<Vec<_>>().join("; ")),
        Value::Object(_) => v.to_string(),
    }
}

/// Lesbare Fassung als Markdown (für `PARAMETER.md`), optional nur ein Modus.
pub fn markdown(modus: Option<&str>) -> String {
    let k: Vec<Parameter> = katalog()
        .into_iter()
        .filter(|p| modus.is_none_or(|m| p.modus == m))
        .collect();
    let zahl = |m: &str| k.iter().filter(|p| p.modus == m).count();
    let mut o = String::new();
    o.push_str("# Parameterkatalog\n\n");
    o.push_str(&format!(
        "Stand {} · **{} Parameter**: {} einfach, {} experte, {} automatisch. Erzeugt mit `stick parameter --markdown` aus \
         `kern/stick-kern/daten/parameter.json` (Quelle: `Arbeitsdaten/werkzeuge/parameter_katalog.py`) – **nicht von Hand \
         ändern**. Die Tests im Kern prüfen, dass jeder Wert unter `einstellungen` hier steht und die Standards stimmen.\n\n",
        stand(),
        k.len(),
        zahl("einfach"),
        zahl("experte"),
        zahl("automatisch")
    ));
    o.push_str(
        "**Modus:** *einfach* = im einfachen Modus sichtbar · *experte* = nur im Expertenmodus · *automatisch* = nicht \
         anzeigen (fester oder abgeleiteter Wert, nur dokumentiert).  \n**Ebene:** *design* = gilt für das ganze Design · \
         *objekt* = am einzelnen Objekt · *beide* = Design-Standard, am Objekt überschreibbar · *werkzeug* = Bild → Umrisse.  \n\
         **JSON:** Pfad im Design-JSON bzw. `Objekt: feld` am Objekt. **Standard** = „0 Software Defaults“; *Stoff* = ändert \
         sich mit dem Stoff (Tabelle am Ende). Bereiche mit „Vorschlag“ in der Quelle sind eigene Festlegungen.\n",
    );
    let mut gruppen: Vec<&str> = Vec::new();
    for p in &k {
        if !gruppen.contains(&p.gruppe.as_str()) {
            gruppen.push(&p.gruppe);
        }
    }
    for g in gruppen {
        o.push_str(&format!(
            "\n## {g}\n\n| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |\n\
             |---|---|---|---|---|---|---|---|---|---|\n"
        ));
        for p in k.iter().filter(|p| p.gruppe == g) {
            let json = [
                p.json_design.as_ref().map(|d| format!("`{d}`")),
                p.json_objekt.as_ref().map(|f| format!("Objekt: `{f}`")),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" · ");
            let bereich = match (&p.bereich, p.werte.is_empty()) {
                (Some([a, b]), _) => format!("{} … {}", text(&serde_json::json!(a)), text(&serde_json::json!(b))),
                (None, false) => p.werte.join(", "),
                _ => String::new(),
            };
            let einheit = if p.einheit.is_empty() || matches!(p.einheit.as_str(), "Auswahl" | "an/aus") {
                String::new()
            } else {
                format!(" {}", p.einheit)
            };
            o.push_str(&format!(
                "| **{}** | {} | {} | {} | {}{} | {} | {} | {} | {} | {} |\n",
                p.name,
                p.wirkung,
                p.gilt_fuer.join(", "),
                json,
                text(&p.standard),
                if p.standard.is_null() { "" } else { einheit.as_str() },
                bereich,
                if p.stoff { "ja" } else { "" },
                p.ebene,
                p.modus,
                p.quelle
            ));
        }
    }
    let stoffabh: Vec<&Parameter> = k.iter().filter(|p| p.stoff).collect();
    if !stoffabh.is_empty() {
        o.push_str("\n## Werte je Stoff\n\n| Parameter |");
        for s in STOFFE.iter() {
            o.push_str(&format!(" {} |", s.name));
        }
        o.push_str(&format!("\n|---|{}\n", "---|".repeat(STOFFE.len())));
        for p in stoffabh {
            o.push_str(&format!("| {} ({}) |", p.name, p.einheit));
            for (_, v) in je_stoff(p) {
                o.push_str(&format!(" {} |", text(&v)));
            }
            o.push('\n');
        }
    }
    o
}

/// Alle Blätter (Pfad, Wert) eines JSON-Objekts; Listen zählen als ein Wert.
#[cfg(test)]
fn blaetter(v: &Value, pfad: &str, out: &mut Vec<(String, Value)>) {
    match v {
        Value::Object(m) => {
            for (k, w) in m {
                let p = if pfad.is_empty() {
                    k.clone()
                } else {
                    format!("{pfad}.{k}")
                };
                blaetter(w, &p, out);
            }
        }
        _ => out.push((pfad.to_string(), v.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn katalog_ist_gueltig() {
        let k = katalog();
        assert!(k.len() > 100);
        let ids: BTreeSet<&str> = k.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids.len(), k.len(), "doppelte IDs");
        for p in &k {
            assert!(
                ["einfach", "experte", "automatisch"].contains(&p.modus.as_str()),
                "{}",
                p.id
            );
            assert!(
                ["design", "objekt", "beide", "werkzeug"].contains(&p.ebene.as_str()),
                "{}",
                p.id
            );
            assert!(!p.quelle.is_empty(), "{}: Quelle fehlt", p.id);
        }
    }

    /// Jeder Wert unter `einstellungen` steht im Katalog, mit dem Standard aus dem Code
    #[test]
    fn katalog_deckt_einstellungen_ab() {
        let e = fuer_stoff("0 Software Defaults").unwrap();
        let mut alle = Vec::new();
        blaetter(&serde_json::to_value(&e).unwrap(), "", &mut alle);
        let k = katalog();
        for (pfad, v) in alle.iter().filter(|(p, _)| p != "stoff") {
            let voll = format!("einstellungen.{pfad}");
            let p = k
                .iter()
                .find(|p| p.json_design.as_deref() == Some(voll.as_str()))
                .unwrap_or_else(|| panic!("{voll} fehlt im Parameterkatalog (parameter_katalog.py ergänzen)"));
            assert!(gleich(&p.standard, v), "{voll}: Katalog {} – Code {v}", p.standard);
        }
        for p in k.iter().filter(|p| {
            p.json_design
                .as_deref()
                .is_some_and(|d| d.starts_with("einstellungen."))
        }) {
            assert!(
                wert(&e, p.json_design.as_deref().unwrap()).is_some(),
                "{}: Pfad gibt es nicht",
                p.id
            );
        }
    }

    /// „stoffabhängig“ stimmt: genau die markierten Werte ändern sich mit dem Stoff
    #[test]
    fn stoffabhaengig_stimmt() {
        for p in katalog() {
            let Some(d) = p.json_design.as_deref().filter(|d| d.starts_with("einstellungen.")) else {
                continue;
            };
            let werte = je_stoff(&p);
            let aendert = werte.iter().any(|(_, v)| !gleich(v, &werte[0].1));
            assert_eq!(aendert, p.stoff, "{d}: stoff = {} im Katalog", p.stoff);
        }
    }
}
