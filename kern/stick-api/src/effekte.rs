//! Farben mischen (Color Blending, Handbuch S. 220–222): „The effect is created by duplicating the object, then
//! applying Gradient Fill and Travel on Edges to both layers, which are then grouped.“ Untere Lage mit dem gewählten
//! Profil, obere Lage in der zweiten Farbe mit dem Gegenstück (Linear ↔ Dec-Linear …). Standardwerte aus dem Stil
//! (`<0 Software Defaults>` 0x27cf): größter Abstand unten 0x27d7 = 360 = 2,0 mm, oben 0x27d8 = 396 = 2,2 mm; kleinster
//! Abstand wie der Verlauf-Datensatz 0x34 (0,4 mm). Travel on Edges entspricht den Laufwegen am Rand, die der Kern
//! ohnehin nimmt (`sticharten::tatami`). Die obere Lage ohne Unterlage (Handbuch-Tipp S. 219/222: sonst scheint die
//! Unterlage durch; eigene Festlegung, dass das automatisch geschieht).

use crate::befehle::DesignWert;
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use stick_kern::effekte::Profil;

/// Leitlinie für den Wave-Effekt vorschlagen (Inspektor „Welle“ einschalten): quer durch die Fläche in Stichrichtung
/// mit einem Bogen in der Mitte, in Objektkoordinaten (`effekte.welle`); nur Step- und Musterflächen.
pub fn welle_vorschlag(k: &Kontext, e: crate::befehle::ObjektEingabe) -> Result<Value, Fehler> {
    let eingabe = crate::befehle::DesignEingabe {
        design: e.design,
        rahmen: None,
        stoff: None,
    };
    let d = k.design(&eingabe)?;
    let fehler = |m: &str| Fehler {
        objekt: Some(e.objekt),
        ..Fehler::from(m)
    };
    let o = d.objekte.get(e.objekt).ok_or_else(|| fehler("Objekt gibt es nicht"))?;
    let ws = d.einstellungen_fuer(e.objekt).tatami.winkel;
    let (umriss, w) = match &o.art {
        stick_kern::design::ObjektArt::Tatami { umriss, winkel, .. } => (umriss, winkel.unwrap_or(ws)),
        stick_kern::design::ObjektArt::Musterfuellung { umriss, winkel, muster, .. } => (
            umriss,
            winkel
                .or_else(|| stick_kern::muster::finden(muster).and_then(|m| m.winkel))
                .unwrap_or(ws),
        ),
        _ => return Err(fehler("Welle geht nur bei Step- und Musterflächen")),
    };
    stick_kern::effekte::welle_vorschlag(umriss, w).ok_or_else(|| fehler("Fläche ohne Umriss"))
}

/// Objektarten, die Farben mischen können (Satin, Step, Muster als Fläche; Satin als Linie)
const MISCHBAR: [&str; 5] = ["tatami", "muster", "satin", "satinlinie", "satinflaeche"];

fn profil_linear() -> Profil {
    Profil::Linear
}

/// Eingabe für `farben_mischen`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FarbenMischenEingabe {
    pub design: DesignWert,
    /// Nummer des Objekts (ab 0): Step-, Muster- oder Satinfläche, Satinsäule oder Satinlinie
    pub objekt: usize,
    /// Farbe der oberen Lage „#rrggbb“
    pub farbe: String,
    /// Profil der unteren Lage (die obere bekommt das Gegenstück; Dialog „Color Blending“)
    #[serde(default = "profil_linear")]
    pub profil: Profil,
    /// größter Abstand der unteren Lage in mm (Standard 2,0 wie der Stil)
    #[serde(default)]
    pub max_unten: Option<f64>,
    /// größter Abstand der oberen Lage in mm (Standard 2,2 wie der Stil)
    #[serde(default)]
    pub max_oben: Option<f64>,
    /// kleinster Abstand beider Lagen in mm (Standard 0,4)
    #[serde(default)]
    pub min: Option<f64>,
}

/// Ergebnis von `farben_mischen`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct FarbenMischenErgebnis {
    /// geändertes Design
    pub design: DesignWert,
    /// Nummern der beiden Lagen (unten, oben)
    pub objekte: [usize; 2],
    /// Schlüssel der neuen Gruppe
    pub gruppe: String,
}

/// Objekt in zwei Lagen mit gegenläufigem Verlauf teilen und gruppieren (Color Blending).
pub fn farben_mischen(_: &Kontext, e: FarbenMischenEingabe) -> Result<FarbenMischenErgebnis, Fehler> {
    let mut design = e.design.0;
    stick_kern::Design::aus_value(&design).map_err(|x| Fehler::from(format!("Design: {x}")))?;
    if !e.farbe.starts_with('#') || e.farbe.len() != 7 || u32::from_str_radix(&e.farbe[1..], 16).is_err() {
        return Err("farbe als „#rrggbb“ angeben".into());
    }
    let (min, max_u, max_o) = (e.min.unwrap_or(0.4), e.max_unten.unwrap_or(2.0), e.max_oben.unwrap_or(2.2));
    for (n, v) in [("min", min), ("max_unten", max_u), ("max_oben", max_o)] {
        if !(0.1..=10.0).contains(&v) {
            return Err(format!("{n} zwischen 0,1 und 10 mm").into());
        }
    }
    let zo = design.as_object_mut().ok_or("Das Design muss ein JSON-Objekt sein")?;
    let mut objekte: Vec<Value> = zo.get("objekte").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut gruppen = zo.get("gruppen").and_then(Value::as_object).cloned().unwrap_or_default();
    let unten = objekte
        .get(e.objekt)
        .cloned()
        .ok_or_else(|| Fehler::from(format!("Objekt {} gibt es nicht", e.objekt + 1)))?;
    let art = unten.get("art").and_then(Value::as_str).unwrap_or("").to_string();
    if !MISCHBAR.contains(&art.as_str()) {
        return Err(Fehler {
            objekt: Some(e.objekt),
            ..Fehler::from("Farben mischen geht mit Step-, Muster- und Satinflächen, Satinsäulen und Satinlinien")
        });
    }
    let verlauf = |o: &mut Value, profil: Profil, max: f64| {
        let fx = o.as_object_mut().unwrap().entry("effekte").or_insert_with(|| json!({}));
        if !fx.is_object() {
            *fx = json!({});
        }
        fx["verlauf"] = json!({ "profil": profil.text(), "min": min, "max": max });
    };
    let name = unten.get("name").and_then(Value::as_str).unwrap_or("").to_string();
    let basis = if name.is_empty() { "Farben mischen".to_string() } else { name.clone() };
    // Die untere Lage behält den Namen (Aussparen nennt Objekte beim Namen), die obere heißt „… oben“
    let mut u = unten.clone();
    verlauf(&mut u, e.profil, max_u);
    let mut o = unten;
    verlauf(&mut o, e.profil.gegenstueck(), max_o);
    o["name"] = format!("{basis} oben").into();
    o["farbe"] = e.farbe.clone().into();
    let om = o.as_object_mut().unwrap();
    om.remove("garn");
    om.remove("einstich");
    om.remove("umrandung");
    om.insert("unterlage".into(), if matches!(art.as_str(), "tatami" | "muster") { json!(false) } else { json!("keine") });
    // Gruppe: vorhandene Gruppe des Objekts behalten (keine verschachtelten Gruppen), sonst neu
    let gruppe = match u.get("gruppe").and_then(Value::as_str) {
        Some(k) => k.to_string(),
        None => {
            let k = (1..).map(|n| format!("g{n}")).find(|k| !gruppen.contains_key(k)).unwrap();
            gruppen.insert(k.clone(), json!({ "name": format!("{basis} (Farben mischen)") }));
            k
        }
    };
    u["gruppe"] = gruppe.clone().into();
    o["gruppe"] = gruppe.clone().into();
    u.as_object_mut().unwrap().remove("ausstich");
    objekte[e.objekt] = u;
    objekte.insert(e.objekt + 1, o);
    zo.insert("objekte".into(), objekte.into());
    zo.insert("gruppen".into(), gruppen.into());
    stick_kern::Design::aus_value(&design).map_err(|x| Fehler::from(format!("Ergebnis: {x}")))?;
    Ok(FarbenMischenErgebnis {
        design: DesignWert(design),
        objekte: [e.objekt, e.objekt + 1],
        gruppe,
    })
}
