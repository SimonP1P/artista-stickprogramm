//! Tests der Befehlsschicht und **Wächter**, die die Schnittstelle synchron halten (ROADMAP 4b.0).

use super::*;
use serde_json::json;

fn kontext() -> Kontext {
    Kontext::finden(None)
}

#[test]
fn befehle_rechnen() {
    let k = kontext();
    let r = ausfuehren(&k, "rahmen_liste", Value::Null).unwrap();
    assert!(r.as_array().unwrap().len() >= 5);
    let beispiel = ausfuehren(&k, "beispiel_design", json!({})).unwrap();
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": beispiel })).unwrap();
    let stiche = e["stiche"].as_array().unwrap().len();
    assert!(stiche > 1000);
    assert_eq!(e["objekt"].as_array().unwrap().len(), stiche, "Objektnummer je Stich");
    assert_eq!(e["ausserhalb"], 0);
    assert_eq!(
        e["garne"].as_array().unwrap().len(),
        e["farben"].as_array().unwrap().len()
    );
    assert!(e["hinweise"].as_array().unwrap().is_empty(), "{}", e["hinweise"]);
    let f = ausfuehren(&k, "design_formen", json!({ "design": beispiel })).unwrap();
    assert_eq!(
        f.as_array().unwrap().len(),
        beispiel["objekte"].as_array().unwrap().len()
    );
    assert!(
        ausfuehren(&k, "fuellmuster_liste", Value::Null)
            .unwrap()
            .as_array()
            .unwrap()
            .len()
            == 242
    );
    assert!(
        !ausfuehren(&k, "parameter_katalog", Value::Null)
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty()
    );
    let s = ausfuehren(
        &k,
        "saeulen_vorschau",
        json!({"umriss": [[0, 0], [20, 0], [20, 3], [0, 3]]}),
    )
    .unwrap();
    assert_eq!(s.as_array().unwrap().len(), 1);
}

#[test]
fn fehler_mit_objektnummer() {
    let k = kontext();
    let design = json!({"objekte": [
        {"art": "lauf", "punkte": [[0, 0], [5, 0]]},
        {"art": "satinlinie", "punkte": [[0, 0], [5, 0]], "versatz": "quer"}
    ]});
    let f = ausfuehren(&k, "design_berechnen", json!({ "design": design })).unwrap_err();
    assert_eq!(f.objekt, Some(1), "{f:?}");
    assert!(f.meldung.contains("„quer“"), "{f:?}");
    let p = ausfuehren(&k, "design_pruefen", json!({ "design": design })).unwrap();
    assert_eq!(p["fehler"]["objekt"], 1);
    assert!(ausfuehren(&k, "gibt_es_nicht", Value::Null).is_err());
    assert!(ausfuehren(&k, "rahmen_liste", json!({"x": 1})).is_err());
}

#[test]
fn unbekannte_felder_als_hinweis() {
    let h = erzeugen::unbekannte_felder(&json!({
        "nmae": "x",
        "objekte": [{"art": "tatami", "umriss": [[0, 0], [5, 0], [5, 5]], "winkle": 30, "transform": {"drehen": 5}}]
    }));
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(h[0].contains("„nmae“") && h[1].contains("Objekt 1 (tatami)") && h[1].contains("„winkle“"));
}

/// Wünsche der Oberfläche v3 (UEBERGABE §4): Hinweise mit Objektnummer, Begrenzung, Datengröße, Garn von Hand,
/// Farbblöcke mit Applikationslagen.
#[test]
fn ergebnis_fuer_die_oberflaeche() {
    let k = kontext();
    let design = json!({"garnkarte": "Isacord 40 wt-numerical", "objekte": [
        {"art": "text", "alphabet": "Swiss Block", "text": "A€b", "garn": "0010", "farbe": "#ffffff", "fo": 1},
        {"art": "applikation", "umriss": [[-30, -30], [30, -30], [30, 30], [-30, 30]], "farbe": "#c0392b",
         "stil": "zuschneiden"},
        {"art": "lauf", "punkte": [[0, 0], [200, 0]]}
    ]});
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": design })).unwrap();
    let h = e["hinweise"].as_array().unwrap();
    let arten: Vec<(&str, u64)> = h
        .iter()
        .map(|x| (x["art"].as_str().unwrap(), x["objekt"].as_u64().unwrap()))
        .collect();
    assert_eq!(
        arten,
        [("feld", 0), ("fehlende_zeichen", 0), ("ausserhalb", 2)],
        "{h:?}"
    );
    assert_eq!(e["fehlende_zeichen"], "€");
    let aussen = e["ausserhalb_stiche"].as_array().unwrap();
    assert_eq!(aussen.len() as u64, e["ausserhalb"].as_u64().unwrap());
    assert!(aussen.iter().all(|i| e["objekt"][i.as_u64().unwrap() as usize] == 2));
    assert_eq!(e["begrenzung"][2], 200.0);
    assert!(e["daten_bytes"].as_u64().unwrap() > 1000);
    assert_eq!(e["daten_grenze"], befehle::DATEN_GRENZE);
    // Garn von Hand: Nummer der Karte statt nächster Farbe
    assert_eq!(e["garne"][0]["nummer"], "0010");
    let b = e["bloecke"].as_array().unwrap();
    assert_eq!(b.len(), e["farben"].as_array().unwrap().len());
    assert_eq!(b[0]["von_hand"], true);
    let lagen: Vec<&str> = b
        .iter()
        .flat_map(|x| x["applikation"].as_array().unwrap())
        .map(|l| l.as_str().unwrap())
        .collect();
    assert_eq!(lagen, ["platzierung", "schnittlinie", "heften", "abdecken"]);
    // unbekanntes Garn → Fehler mit Objektnummer
    let mut falsch = design.clone();
    falsch["objekte"][0]["garn"] = json!("9999x");
    let f = ausfuehren(&k, "design_berechnen", json!({ "design": falsch })).unwrap_err();
    assert_eq!(f.objekt, Some(0), "{f}");
}

#[test]
fn alphabete_und_step_muster_fuer_auswahllisten() {
    let k = kontext();
    let a = ausfuehren(&k, "alphabete_liste", Value::Null).unwrap();
    let ohne: Vec<&str> = a
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| x["min_hoehe"].is_null())
        .map(|x| x["name"].as_str().unwrap())
        .collect();
    // Handbuch nennt für Zurich keine größte Höhe (alphabets.md: „6 mm | ?“)
    assert_eq!(ohne, ["Zurich"]);
    let v = ausfuehren(&k, "alphabet_vorschau", json!({"alphabet": "Swiss Block"})).unwrap();
    assert!(v["zeichen"].as_str().unwrap().contains('A'));
    assert!(!v["linien"].as_array().unwrap().is_empty());
    let s = ausfuehren(&k, "step_muster_liste", Value::Null).unwrap();
    let s = s.as_array().unwrap();
    assert_eq!(s.len(), 20);
    assert!(s.iter().all(|m| m["einstiche"].as_array().unwrap().len() > 50));
    assert_ne!(
        s[0]["einstiche"], s[7]["einstiche"],
        "Muster 1 und 8 sehen verschieden aus"
    );
}

/// Wächter 1: die erzeugten Dateien (`kern/schnittstelle/`, `SCHNITTSTELLE.md`) passen zum Code.
/// Neu erzeugen: `stick api exportieren` oder `STICK_API_SCHREIBEN=1 cargo test -p stick-api`.
#[test]
fn erzeugte_dateien_aktuell() {
    let wurzel = erzeugen::repo_wurzel();
    if std::env::var_os("STICK_API_SCHREIBEN").is_some() {
        erzeugen::schreiben(&wurzel).unwrap();
    }
    let alt = erzeugen::veraltet(&wurzel);
    assert!(
        alt.is_empty(),
        "Schnittstelle veraltet: {alt:?} – neu erzeugen mit `stick api exportieren` \
         (oder STICK_API_SCHREIBEN=1 cargo test -p stick-api) und mitcommitten"
    );
}

/// Wächter 2: `UI_Mockups/FUNKTIONEN.md` nennt jede Objektart und jeden Befehl – neue Funktionen werden dort für
/// Designer und Frontend beschrieben (einfach/experte, Standard).
#[test]
fn funktionen_md_nennt_alles() {
    let text = std::fs::read_to_string(erzeugen::repo_wurzel().join("UI_Mockups/FUNKTIONEN.md")).unwrap();
    let schema = stick_kern::format::schema();
    let mut fehlt = Vec::new();
    for v in schema["$defs"]["ObjektJson"]["oneOf"].as_array().unwrap() {
        let art = v["properties"]["art"]["const"].as_str().unwrap();
        if !text.contains(&format!("`{art}`")) {
            fehlt.push(format!("Objektart `{art}`"));
        }
    }
    for b in alle() {
        if !text.contains(&format!("`{}`", b.name)) {
            fehlt.push(format!("Befehl `{}`", b.name));
        }
    }
    assert!(
        fehlt.is_empty(),
        "UI_Mockups/FUNKTIONEN.md nennt nicht: {fehlt:?} – Funktion dort beschreiben (Anwender stellt ein, Standard, \
         JSON, Modus)"
    );
}

/// Wächter 3: Jedes Feld, das der Parameterkatalog am Objekt oder im Design nennt, gibt es im Format.
#[test]
fn parameterkatalog_passt_zum_format() {
    let schema = stick_kern::format::schema();
    let defs = schema["$defs"].as_object().unwrap();
    let oben: Vec<&String> = schema["properties"].as_object().unwrap().keys().collect();
    let felder_der_art = |art: &str| -> Vec<String> {
        let v = schema["$defs"]["ObjektJson"]["oneOf"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["properties"]["art"]["const"] == art);
        let Some(v) = v else { return Vec::new() };
        let r = v["$ref"].as_str().unwrap_or("");
        let def = &defs[r.rsplit('/').next().unwrap()];
        def["properties"]
            .as_object()
            .map(|p| p.keys().cloned().collect())
            .unwrap_or_default()
    };
    let arten: Vec<String> = schema["$defs"]["ObjektJson"]["oneOf"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["properties"]["art"]["const"].as_str().unwrap().to_string())
        .collect();
    let mut falsch = Vec::new();
    for p in stick_kern::parameter::katalog() {
        if let Some(d) = &p.json_design {
            let erstes = d.split('.').next().unwrap_or("");
            if !oben.iter().any(|k| *k == erstes) {
                falsch.push(format!("{}: json_design „{d}“", p.id));
            }
        }
        if let Some(f) = &p.json_objekt {
            let erstes = f.split('.').next().unwrap_or("");
            // Objektart = erstes Wort („text (Step)“ = Objektart text mit Erläuterung)
            let pruefen: Vec<String> = if p.gilt_fuer.is_empty() {
                arten.clone()
            } else {
                p.gilt_fuer
                    .iter()
                    .map(|a| a.split_whitespace().next().unwrap_or("").to_string())
                    .collect()
            };
            let bekannt: Vec<&String> = pruefen.iter().filter(|a| arten.contains(a)).collect();
            if bekannt.len() != pruefen.len() {
                falsch.push(format!(
                    "{}: gilt_fuer {:?} nennt unbekannte Objektart",
                    p.id, p.gilt_fuer
                ));
            }
            if !bekannt.iter().any(|a| felder_der_art(a).iter().any(|x| x == erstes)) {
                falsch.push(format!("{}: json_objekt „{f}“ an {:?}", p.id, p.gilt_fuer));
            }
        }
    }
    assert!(
        falsch.is_empty(),
        "Parameterkatalog nennt Felder, die das Format nicht hat: {falsch:#?}"
    );
}
