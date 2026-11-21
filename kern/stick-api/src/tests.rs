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

/// Maschinen-Befehle mit der Simulation (schreibt nur in die Simulation): prüfen, planen, senden mit Fortschritt,
/// Liste, lesen; Fehlercodes.
#[test]
fn buchstabe_je_stich() {
    let k = kontext();
    let d = json!({"objekte": [
        {"art": "lauf", "punkte": [[-40, -30], [-30, -30]]},
        {"art": "text", "text": "A B^C", "alphabet": "Swiss Block", "hoehe": 12, "farben": ["#ff0000"],
         "buchstaben": [{"nr": 2, "transform": {"drehen": 10}}]}
    ]});
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap();
    let b: Vec<u64> = serde_json::from_value(e["buchstabe"].clone()).unwrap();
    let o: Vec<u64> = serde_json::from_value(e["objekt"].clone()).unwrap();
    assert_eq!(b.len(), o.len());
    // Lauf ohne Buchstaben, Text mit den Nummern 1, 2, 3 – in Stickfolge
    assert!(b.iter().zip(&o).all(|(&b, &o)| (o == 0) == (b == 0)));
    let mut folge: Vec<u64> = b.iter().copied().filter(|&x| x > 0).collect();
    folge.dedup();
    assert_eq!(folge, vec![1, 2, 3]);
}

#[test]
fn motivlinie() {
    let k = kontext();
    let l = ausfuehren(&k, "motive_liste", json!({})).unwrap();
    assert_eq!(l.as_array().unwrap().len(), 14);
    let m = ausfuehren(&k, "motivsatz_motive", json!({"satz": "Heirloom"})).unwrap();
    let m705 = m.as_array().unwrap().iter().find(|x| x["name"] == "705").unwrap();
    assert!((m705["laenge"].as_f64().unwrap() - 2037.0 / 180.0).abs() < 0.02);
    assert!(!m705["linien"].as_array().unwrap().is_empty());
    // 100 mm Linie, Motiv 705 (11,32 mm) → round(8,83) = 9 Kopien
    let d = json!({"objekte": [{"art": "motivlinie", "satz": "Heirloom", "motiv": "705", "punkte": [[-50, 0], [50, 0]]}]});
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap();
    let b: [f64; 4] = serde_json::from_value(e["begrenzung"].clone()).unwrap();
    let (lo, hi) = (b[0], b[2]);
    assert!(lo > -50.5 && hi < 50.5 + 11.4 - 100.0 / 9.0 + 0.5, "{lo} … {hi}");
    // unbekanntes Motiv → Fehler mit Objektnummer
    let d = json!({"objekte": [{"art": "motivlinie", "satz": "Heirloom", "motiv": "xyz", "punkte": [[0, 0], [10, 0]]}]});
    let f = ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap_err();
    assert_eq!(f.objekt, Some(0));
}

#[test]
fn in_knoten_umwandeln() {
    let k = kontext();
    let d = json!({"objekte": [
        {"art": "tatami", "umriss": {"kreis": {"mitte": [0, 0], "radius": 10}}, "transform": {"drehen": 20}},
        {"art": "text", "text": "A", "alphabet": "Swiss Block"}
    ]});
    let o = ausfuehren(&k, "in_knoten_umwandeln", json!({"design": d, "objekt": 0})).unwrap();
    assert!(o.get("transform").is_none());
    assert_eq!(o["umriss"]["verbinden"].as_array().unwrap().len(), 4);
    let f = ausfuehren(&k, "in_knoten_umwandeln", json!({"design": d, "objekt": 1})).unwrap_err();
    assert_eq!(f.objekt, Some(1));
}

#[test]
fn maschine_in_der_simulation() {
    let k = kontext();
    let mitschnitte = std::env::temp_dir().join("stick_api_test_mitschnitte");
    k.mitschnitte_nach(mitschnitte.clone());
    let gemeldet = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let g = gemeldet.clone();
    k.fortschritt_melden(move |f| g.lock().unwrap().push(f.was.clone()));
    let sim = json!({"port": "", "simulation": true});
    let p = ausfuehren(&k, "maschine_pruefen", sim.clone()).unwrap();
    assert_eq!(p["sitzung"]["port"], "Simulation");
    assert_eq!(p["sitzung"]["verlassen"], true);
    let design = json!({"name": "Test", "garnkarte": "Isacord 40 wt-numerical", "objekte": [
        {"art": "satinlinie", "punkte": [[-10, 0], [10, 0]], "farbe": "#ffffff", "garn": "0015"}
    ]});
    let wahl = json!({"icon": true, "farben": true, "sticken": false});
    let ein = json!({"design": design, "port": "", "simulation": true, "wahl": wahl});
    let plan = ausfuehren(&k, "maschine_planen", ein.clone()).unwrap();
    assert!(plan["bytes_daten"].as_u64().unwrap() > 0);
    assert_eq!(plan["garne"][0]["nummer"], 15, "Garn von Hand in der Garnfarbtabelle");
    let s = ausfuehren(&k, "maschine_senden", ein).unwrap();
    assert!(s["gestickt"].is_null());
    assert!(
        gemeldet.lock().unwrap().iter().any(|w| w == "DATEN"),
        "Fortschritt gemeldet"
    );
    let l = ausfuehren(&k, "maschine_liste", sim.clone()).unwrap();
    assert!(!l["designs"].as_array().unwrap().is_empty());
    let nr = l["designs"][0]["nr"].clone();
    let r = ausfuehren(&k, "maschine_lesen", json!({"simulation": true, "nr": nr})).unwrap();
    assert!(!r["ergebnis"]["stiche"].as_array().unwrap().is_empty());
    let ziel = mitschnitte.join("gelesen.exp");
    let n = ausfuehren(&k, "gelesen_exp_speichern", json!({"pfad": ziel.display().to_string()})).unwrap();
    assert!(n.as_u64().unwrap() > 0);
    // außerhalb des Stickfelds → Fehler mit Code, nichts verbunden
    let weit = json!({"design": {"objekte": [{"art": "lauf", "punkte": [[0, 0], [300, 0]]}]},
        "port": "", "simulation": true, "wahl": wahl});
    let f = ausfuehren(&k, "maschine_planen", weit).unwrap_err();
    assert_eq!(f.code, Some(FehlerCode::Ausserhalb), "{f}");
    // ohne laufende Aktion: Abbrechen meldet „lief nicht“
    assert_eq!(ausfuehren(&k, "maschine_abbrechen", json!({})).unwrap(), json!(false));
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

/// „Insert Design“: Objekte anhängen, als Gruppe; doppelte Namen und `aussparen` werden mitgezogen.
#[test]
fn design_einfuegen_als_gruppe() {
    let k = kontext();
    let basis = json!({"objekte": [{"art": "text", "text": "Anna", "alphabet": "Drifter", "hoehe": 10, "name": "Blatt"}]});
    let bild = json!({"transform": {"skalieren": 50}, "objekte": [
        {"art": "tatami", "name": "Blatt", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 20, "hoehe": 20}}},
        {"art": "tatami", "name": "Stiel", "aussparen": ["Blatt"], "umriss": {"rechteck": {"mitte": [5, 0], "breite": 20, "hoehe": 20}}}
    ]});
    let r = ausfuehren(&k, "design_einfuegen", json!({"design": basis, "einfuegen": bild, "gruppe": {"name": "blume.png", "herkunft": "bild"}})).unwrap();
    assert_eq!(r["objekte"], json!([1, 2]));
    assert_eq!(r["gruppe"], "g1");
    let d = &r["design"];
    assert_eq!(d["objekte"][1]["name"], "Blatt (2)");
    assert_eq!(d["objekte"][2]["aussparen"], json!(["Blatt (2)"]));
    assert_eq!(d["objekte"][2]["gruppe"], "g1");
    assert_eq!(d["gruppen"]["g1"]["transform"]["skalieren"], 50);
    assert_eq!(d["gruppen"]["g1"]["herkunft"], "bild");
    // noch einmal: neue Gruppe g2
    let r2 = ausfuehren(&k, "design_einfuegen", json!({"design": d, "einfuegen": bild, "gruppe": {"name": "b"}})).unwrap();
    assert_eq!(r2["gruppe"], "g2");
    assert_eq!(r2["design"]["objekte"][3]["name"], "Blatt (3)");
    ausfuehren(&k, "design_berechnen", json!({"design": r2["design"]})).unwrap();
}

/// Stichart wechseln: Fläche ↔ Linie, Form und gemeinsame Felder bleiben; offene Linie lässt sich nicht füllen.
#[test]
fn stichart_wechseln() {
    let k = kontext();
    let d = json!({"gruppen": {"g1": {"name": "x"}}, "objekte": [
        {"art": "tatami", "name": "Kreis", "farbe": "#ff0000", "gruppe": "g1", "winkel": 30,
         "transform": {"drehen": 10}, "umriss": {"kreis": {"mitte": [0, 0], "radius": 10}},
         "loecher": [{"kreis": {"mitte": [0, 0], "radius": 3}}]},
        {"art": "lauf", "punkte": [[0, 0], [10, 0], [10, 10]]},
        {"art": "tatami", "umriss": [[0, 0], [20, 0], [20, 20], [0, 20]]}
    ]});
    let wechsel = |nr: usize, art: &str| ausfuehren(&k, "stichart_aendern", json!({"design": d, "objekt": nr, "art": art}));
    // Fläche → Muster: Löcher und Winkel bleiben, Muster bekommt einen Standard
    let m = wechsel(0, "muster").unwrap();
    assert_eq!(m["objekt"]["loecher"].as_array().unwrap().len(), 1);
    assert_eq!(m["objekt"]["winkel"], 30);
    assert_eq!(m["objekt"]["muster"], "001");
    assert_eq!(m["objekt"]["gruppe"], "g1");
    assert_eq!(m["objekt"]["farbe"], "#ff0000");
    // Fläche → Laufstich: Umriss wird Linie, Löcher entfallen mit Hinweis
    let l = wechsel(0, "lauf").unwrap();
    assert!(l["objekt"]["punkte"]["kreis"].is_object());
    assert_eq!(l["hinweise"].as_array().unwrap().len(), 1);
    assert_eq!(l["objekt"]["transform"]["drehen"], 10);
    // Vieleck-Fläche → Satinlinie: zum ersten Punkt zurückgeführt; → Blanket: geschlossen
    let s = wechsel(2, "satinlinie").unwrap();
    assert!(s["objekt"]["punkte"]["verbinden"].is_array());
    assert_eq!(wechsel(2, "blanket").unwrap()["objekt"]["geschlossen"], true);
    // offene Linie → Fläche: Fehler; Text: Fehler
    assert!(wechsel(1, "tatami").is_err());
    assert!(wechsel(1, "dreifach").is_ok());
    // alle Ergebnisse rechnen
    for (nr, art) in [(0, "satinflaeche"), (0, "satinlinie"), (0, "motivlinie"), (2, "satinlinie"), (2, "lauf")] {
        let mut d2 = d.clone();
        d2["objekte"][nr] = wechsel(nr, art).unwrap()["objekt"].clone();
        ausfuehren(&k, "design_berechnen", json!({"design": d2})).unwrap_or_else(|e| panic!("{nr} {art}: {e:?}"));
    }
}

/// Bild: Vorschau mit Farbliste, Weglassen nach Farbe, Garnkarte, Unterlage an
#[test]
fn bild_vorschau_und_umwandeln() {
    let k = kontext();
    let pfad = std::env::temp_dir().join(format!("stick_bild_test_{}.png", std::process::id()));
    let mut b = image::RgbaImage::from_pixel(200, 100, image::Rgba([255, 255, 255, 255]));
    for y in 20..80u32 {
        for x in 20..90u32 {
            b.put_pixel(x, y, image::Rgba([200, 30, 30, 255]));
        }
        for x in 110..180u32 {
            b.put_pixel(x, y, image::Rgba([30, 60, 200, 255]));
        }
    }
    b.save(&pfad).unwrap();
    let p = pfad.to_string_lossy().to_string();
    let v = ausfuehren(&k, "bild_vorschau", json!({"pfad": p, "farben": 3, "breite": 100})).unwrap();
    assert!(v["bild"].as_str().unwrap().starts_with("data:image/png;base64,"));
    let farben = v["farben"].as_array().unwrap();
    assert_eq!(farben.len(), 3);
    let weiss = farben.iter().find(|f| f["weglassen"] == true).expect("Weiß vorgeschlagen");
    assert_eq!(weiss["farbe"], "#ffffff");
    // Vorschlag: 2 Flächen; alles sticken: 3; Rot weglassen: 2 (Weiß + Blau)
    let n = |extra: Value| {
        let mut e = json!({"pfad": p, "farben": 3, "breite": 100});
        e.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        let r = ausfuehren(&k, "bild_umwandeln", e).unwrap();
        r["design"]["objekte"].as_array().unwrap().clone()
    };
    let o = n(json!({}));
    assert_eq!(o.len(), 2);
    assert!(o.iter().all(|x| x["art"] == "tatami" && x["unterlage"] == true));
    assert_eq!(n(json!({"weglassen": []})).len(), 3);
    let rot = farben.iter().find(|f| f["farbe"].as_str().unwrap().starts_with("#c")).unwrap()["farbe"].clone();
    assert!(n(json!({"weglassen": [rot]})).iter().all(|x| x["farbe"] != rot));
    // Garnkarte Default: Farben = Garne, Objekte tragen die Nummer
    let g = ausfuehren(&k, "bild_vorschau", json!({"pfad": p, "farben": 3, "garnkarte": "Default"})).unwrap();
    assert!(g["farben"][0]["garn"]["nummer"].is_string());
    let og = n(json!({"garnkarte": "Default"}));
    assert!(og.iter().all(|x| x["garn"].is_string()));
    // Satin für schmale Flächen auf Wunsch
    assert!(n(json!({"fuellung": "auto"})).iter().any(|x| x["art"] == "tatami" || x["art"] == "satinflaeche"));
    let _ = std::fs::remove_file(&pfad);
}
