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

/// Motivfüllung: Standard wie das Original (NP004a K0001a), Raster Kante an Kante, Motive bleiben in der Fläche
/// (abgeschnitten, wenn mehr als zur Hälfte draußen; ganz, wenn höchstens zur Hälfte draußen)
#[test]
fn motivfuellung() {
    let k = kontext();
    let d = json!({"objekte": [{"art": "motivfuellung", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 60, "hoehe": 40}}}]});
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": d.clone() })).unwrap();
    let b: [f64; 4] = serde_json::from_value(e["begrenzung"].clone()).unwrap();
    // ganze Motive ragen höchstens zur Hälfte über den Rand – nach dem Kasten (12 × 12 mm) weniger als ein ganzes Motiv
    assert!(b[0] > -30.0 - 12.0 && b[2] < 30.0 + 12.0 && b[1] > -20.0 - 12.0 && b[3] < 20.0 + 12.0, "{b:?}");
    assert!(b[2] - b[0] > 55.0 && b[3] - b[1] > 35.0, "Fläche gefüllt: {b:?}");
    // wirksame Werte: Größe und Abstände aus dem Motivsatz
    let w = ausfuehren(&k, "objekt_werte", json!({"design": d, "objekt": 0})).unwrap();
    let wert = |f: &str| w.as_array().unwrap().iter().find(|x| x["feld"] == f).unwrap()["wert"].as_f64().unwrap();
    assert!((wert("groesse") - 2146.0 / 180.0).abs() < 0.02, "{}", wert("groesse"));
    // wie das Original (`<PRESET_STANDARD_MOTIF_FILL>`): Spalten und Reihen 2158 (1/180 mm) ≈ Kasten des Motivs
    assert!((wert("spaltenabstand") - 2158.0 / 180.0).abs() < 0.15, "{}", wert("spaltenabstand"));
    assert!((wert("reihenabstand") - 2158.0 / 180.0).abs() < 0.15, "{}", wert("reihenabstand"));
    // Fläche ohne Platz für ein Motiv → Fehler mit Objektnummer
    let d = json!({"objekte": [{"art": "motivfuellung", "satz": "Children", "motiv": "901", "umriss": [[0, 0], [0.5, 0], [0.5, 0.5]]}]});
    assert_eq!(ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap_err().objekt, Some(0));
}

/// Effekte am Objekt (Verlauf, Rand, Welle) und Farben mischen (Color Blending)
#[test]
fn effekte_und_farben_mischen() {
    let k = kontext();
    let flaeche = json!({"art": "tatami", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 40, "hoehe": 30}}, "farbe": "#c00000"});
    let stiche = |d: &serde_json::Value| ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap()["stiche"].as_array().unwrap().len();
    let ohne = stiche(&json!({"objekte": [flaeche.clone()]}));
    let mut mit = flaeche.clone();
    mit["effekte"] = json!({"verlauf": {}, "rand": {"seite": "beide"}, "welle": [[-25, 0], [0, 4], [25, 0]]});
    let n = stiche(&json!({"objekte": [mit.clone()]}));
    assert!(n < ohne * 2 / 3, "Verlauf 0,4 → 2,0 mm: weniger Stiche ({n} gegen {ohne})");
    // falsche Werte → Fehler mit Objektnummer
    let mut falsch = flaeche.clone();
    falsch["effekte"] = json!({"verlauf": {"max": 20}});
    assert_eq!(ausfuehren(&k, "design_berechnen", json!({"design": {"objekte": [falsch]}})).unwrap_err().objekt, Some(0));
    // Satin mit Verlauf
    let satin = json!({"art": "satin", "seite_a": [[0, 0], [40, 0]], "seite_b": [[0, 4], [40, 4]], "effekte": {"verlauf": {"profil": "abnehmend"}}});
    assert!(stiche(&json!({"objekte": [satin]})) > 20);
    // Farben mischen: zwei Lagen, gegenläufige Profile, obere Lage andere Farbe ohne Unterlage, gruppiert
    let d = json!({"objekte": [{"art": "lauf", "punkte": [[0, 0], [10, 0]]}, flaeche]});
    let r = ausfuehren(&k, "farben_mischen", json!({"design": d, "objekt": 1, "farbe": "#ffd000", "profil": "konvex"})).unwrap();
    let o = r["design"]["objekte"].as_array().unwrap();
    assert_eq!(o.len(), 3);
    assert_eq!(r["objekte"], json!([1, 2]));
    assert_eq!(o[1]["effekte"]["verlauf"]["profil"], "konvex");
    assert_eq!(o[2]["effekte"]["verlauf"]["profil"], "konkav");
    assert_eq!(o[2]["farbe"], "#ffd000");
    assert_eq!(o[2]["unterlage"], false);
    assert!((o[2]["effekte"]["verlauf"]["max"].as_f64().unwrap() - 2.2).abs() < 1e-9);
    assert_eq!(o[1]["gruppe"], o[2]["gruppe"]);
    assert!(r["design"]["gruppen"][r["gruppe"].as_str().unwrap()].is_object());
    stiche(&r["design"]);
    // Welle vorschlagen: Pfad mit Kurvenpunkt quer durch die Fläche; mit Transformation → in Knoten umwandeln nimmt
    // die Leitlinie mit
    let w = ausfuehren(&k, "welle_vorschlag", json!({"design": {"objekte": [flaeche]}, "objekt": 0})).unwrap();
    assert_eq!(w["pfad"]["punkte"].as_array().unwrap().len(), 3);
    let mut gedreht = flaeche.clone();
    gedreht["effekte"] = json!({"welle": w});
    gedreht["transform"] = json!({"verschieben": [5, 0]});
    let u = ausfuehren(&k, "in_knoten_umwandeln", json!({"design": {"objekte": [gedreht]}, "objekt": 0})).unwrap();
    // Pfad wird zu Bézierstücken (wie der Umriss), um 5 mm verschoben
    let a0 = u["effekte"]["welle"]["verbinden"][0]["bezier"][0][0].as_f64().unwrap();
    assert!((a0 - (w["pfad"]["punkte"][0][0].as_f64().unwrap() + 5.0)).abs() < 1e-6, "{u}");
    // Lauf kann nicht gemischt werden
    let f = ausfuehren(&k, "farben_mischen", json!({"design": d, "objekt": 0, "farbe": "#ffd000"})).unwrap_err();
    assert_eq!(f.objekt, Some(0));
}

/// Eigene Motive (Create Pattern): speichern, in Motivlinie/Motivfüllung benutzen, löschen
#[test]
fn eigene_motive() {
    let k = kontext();
    let ablage = std::env::temp_dir().join(format!("stick_api_test_motive_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&ablage);
    k.motive_ablage(ablage.clone());
    // Zickzack-Lauf (gedreht über transform) + kleiner Kreis als Fläche
    let d = json!({"objekte": [
        {"art": "lauf", "punkte": [[0, 0], [2, 3], [4, 0], [6, 3], [8, 0]], "transform": {"verschieben": [10, 0]}},
        {"art": "tatami", "umriss": {"kreis": {"mitte": [14, 1.5], "radius": 1}}},
        {"art": "text", "text": "A", "alphabet": "Swiss Block"}
    ]});
    // Name eines Satzes der CD ist gesperrt, Text kann kein Motiv sein
    assert!(ausfuehren(&k, "motiv_speichern", json!({"design": d, "objekte": [0], "satz": "Heirloom", "name": "z"})).is_err());
    let f = ausfuehren(&k, "motiv_speichern", json!({"design": d, "objekte": [2], "satz": "Meine", "name": "z"})).unwrap_err();
    assert_eq!(f.objekt, Some(2), "{}", f.meldung);
    let i = ausfuehren(&k, "motiv_speichern", json!({"design": d, "objekte": [0, 1], "satz": "Meine", "name": "Zack"})).unwrap();
    assert_eq!(i["eigen"], true);
    let l = ausfuehren(&k, "motive_liste", json!({})).unwrap();
    assert!(l.as_array().unwrap().iter().any(|s| s["name"] == "Meine" && s["eigen"] == true));
    let m = ausfuehren(&k, "motivsatz_motive", json!({"satz": "Meine"})).unwrap();
    // Grundlinie = Breite der Objekte (Zickzack 10 … 18 mm nach der Verschiebung) auf halber Höhe
    assert!((m[0]["laenge"].as_f64().unwrap() - 8.0).abs() < 0.05, "{}", m[0]["laenge"]);
    let d2 = json!({"objekte": [
        {"art": "motivlinie", "satz": "Meine", "motiv": "Zack", "punkte": [[-20, 0], [20, 0]]},
        {"art": "motivfuellung", "satz": "Meine", "motiv": "Zack", "umriss": {"rechteck": {"mitte": [0, -20], "breite": 30, "hoehe": 15}}}
    ]});
    let e = ausfuehren(&k, "design_berechnen", json!({ "design": d2 })).unwrap();
    assert!(e["stiche"].as_array().is_some_and(|s| s.len() > 100), "Stiche aus dem eigenen Motiv");
    // löschen: Motiv weg, mit dem letzten Motiv auch der Satz
    let l = ausfuehren(&k, "motiv_loeschen", json!({"satz": "Meine", "motiv": "Zack"})).unwrap();
    assert!(!l.as_array().unwrap().iter().any(|s| s["name"] == "Meine"));
    assert!(ausfuehren(&k, "motiv_loeschen", json!({"satz": "Heirloom", "motiv": "705"})).is_err());
    let _ = std::fs::remove_dir_all(&ablage);
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
    // Garnfarben aus: Farben des Bildes, Garn nur als Hinweis in der Vorschau
    let h = ausfuehren(&k, "bild_vorschau", json!({"pfad": p, "farben": 3, "garnkarte": "Default", "garnfarben": false})).unwrap();
    assert!(h["farben"].as_array().unwrap().iter().any(|f| f["farbe"] == "#c81e1e" && f["garn"]["name"].is_string()));
    assert!(n(json!({"garnkarte": "Default", "garnfarben": false})).iter().all(|x| x.get("garn").is_none()));
    // Satin für schmale Flächen auf Wunsch
    assert!(n(json!({"fuellung": "auto"})).iter().any(|x| x["art"] == "tatami" || x["art"] == "satinflaeche"));
    let _ = std::fs::remove_file(&pfad);
}

/// Schrift mit Stichart „auto“: klein Satin, groß Step (ganzer Text); Satin groß gibt eine Warnung
#[test]
fn schrift_stichart_auto() {
    let k = kontext();
    let rechne = |hoehe: f64, stichart: &str, skalieren: Option<f64>| {
        let mut t = json!({"art": "text", "text": "HM", "alphabet": "Swiss Block", "hoehe": hoehe, "stichart": stichart});
        if let Some(f) = skalieren {
            t["transform"] = json!({"skalieren": [f, f]});
        }
        let d = json!({"rahmen": "artista 200 x 155", "objekte": [t]});
        ausfuehren(&k, "design_berechnen", json!({"design": d})).unwrap()
    };
    let arten = |e: &Value| -> Vec<String> {
        e["hinweise"]
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h["art"].as_str().unwrap().to_string())
            .filter(|a| a != "ausserhalb")
            .collect()
    };
    let klein = rechne(15.0, "auto", None);
    assert!(arten(&klein).is_empty(), "15 mm bleibt Satin: {:?}", klein["hinweise"]);
    let gross = rechne(90.0, "auto", None);
    assert_eq!(arten(&gross), ["schrift_step"], "{:?}", gross["hinweise"]);
    // Step hat mehr Stiche als Satin gleicher Größe
    let satin = rechne(90.0, "satin", None);
    assert_eq!(arten(&satin), ["lange_satinstiche"]);
    assert_ne!(gross["statistik"]["stiche"], satin["statistik"]["stiche"]);
    // Größe über die Transformation zählt mit
    assert_eq!(arten(&rechne(15.0, "auto", Some(600.0))), ["schrift_step"]);
}

/// Schriftauswahl: Google-Katalog, Schrift wechseln (Alphabet ↔ Computer-Schrift), Vorschau mit Stickbarkeit
#[test]
fn schriften_auswahl() {
    let k = kontext();
    let g = ausfuehren(&k, "google_schriften_liste", json!({})).unwrap();
    assert!(g["schriften"].as_array().unwrap().len() > 1500);
    assert!(g["stile"].as_array().unwrap().iter().any(|s| s == "Gotisch"));

    let d = json!({"objekte": [{"art": "text", "text": "Mia", "alphabet": "Drifter", "hoehe": 20, "stichart": "auto",
                                 "unterlage": ["mittellauf", "zickzack"],
                                 "farbe": "#ff0000", "ausrichtung": "links", "name": "Name"}]});
    let o = ausfuehren(&k, "schrift_wechseln", json!({"design": d, "objekt": 0, "schrift": "Arial"})).unwrap();
    assert_eq!(o["art"], "ttf");
    assert_eq!(o["schrift"], "Arial");
    assert_eq!((o["text"].clone(), o["farbe"].clone(), o["ausrichtung"].clone()), (json!("Mia"), json!("#ff0000"), json!("links")));
    // Stichart „auto“ und Unterlage bleiben
    assert_eq!((o["stichart"].clone(), o["unterlage"].clone()), (json!("auto"), json!(["mittellauf", "zickzack"])));
    assert!(o.get("alphabet").is_none());
    let t = ausfuehren(&k, "schrift_wechseln", json!({"design": {"objekte": [o]}, "objekt": 0, "alphabet": "Lisa"})).unwrap();
    assert_eq!((t["art"].clone(), t["alphabet"].clone(), t["stichart"].clone()), (json!("text"), json!("Lisa"), json!("auto")));
    assert!(t.get("schrift").is_none());
    // Computer-Schrift ohne Stichart (Vorgabe Tatami) → auto; ausdrücklich Tatami → Step
    let ttf = |x: serde_json::Value| json!({"objekte": [x]});
    let t2 = ausfuehren(&k, "schrift_wechseln", json!({"design": ttf(json!({"art": "ttf", "text": "A", "schrift": "Arial"})), "objekt": 0, "alphabet": "Lisa"})).unwrap();
    assert_eq!(t2["stichart"], "auto");
    let t3 = ausfuehren(&k, "schrift_wechseln", json!({"design": ttf(json!({"art": "ttf", "text": "A", "schrift": "Arial", "stichart": "tatami"})), "objekt": 0, "alphabet": "Lisa"})).unwrap();
    assert_eq!(t3["stichart"], "step");
    assert!(ausfuehren(&k, "schrift_wechseln", json!({"design": d, "objekt": 0})).is_err());

    let liste = ausfuehren(&k, "schriften_liste", json!({})).unwrap();
    let namen: Vec<String> = liste.as_array().unwrap().iter().map(|s| s["name"].as_str().unwrap().to_string()).collect();
    let Some(name) = namen.iter().find(|n| *n == "Arial").or(namen.first()) else {
        eprintln!("übersprungen: keine Schriften installiert");
        return;
    };
    let v = ausfuehren(&k, "schrift_vorschau", json!({"schrift": name, "text": "Abc"})).unwrap();
    assert!(!v["linien"].as_array().unwrap().is_empty());

    // Unterlage der Computer-Schrift in Satin (Stil „Schrift“ wie im Original; Mittellauf + Zickzack = Vorgabe der
    // Oberfläche): keine < auto (Zickzack) < Mittellauf + Zickzack; alte Schreibweise true/false = auto/keine
    let stiche = |u: serde_json::Value| -> usize {
        let d = json!({"rahmen": "artista 130 x 100", "objekte": [{"art": "ttf", "text": "Anna", "schrift": name,
                       "hoehe": 20, "stichart": "satin", "unterlage": u}]});
        ausfuehren(&k, "design_berechnen", json!({"design": d})).unwrap()["stiche"].as_array().unwrap().len()
    };
    let (keine, auto, beide) = (stiche(json!("keine")), stiche(json!("auto")), stiche(json!(["mittellauf", "zickzack"])));
    assert!(keine < auto && auto < beide, "{name}: keine {keine}, auto {auto}, beide {beide}");
    assert_eq!((stiche(json!(true)), stiche(json!(false))), (auto, keine));
    if name == "Arial" {
        let ab = v["stickbarkeit"]["stickbar_ab"].as_f64().unwrap();
        assert!((6.0..=10.0).contains(&ab), "Arial stickbar ab {ab}");
    }
}

/// Google-Schrift herunterladen (braucht Internet): `cargo test -p stick-api google_laden -- --ignored`
#[test]
#[ignore]
fn google_laden() {
    let ablage = std::env::temp_dir().join("artista_schriften_test");
    let _ = std::fs::remove_dir_all(&ablage);
    let k = kontext();
    k.schriften_ablage(ablage.clone());
    let s = ausfuehren(&k, "google_schrift_laden", json!({"familie": "Pirata One"})).unwrap();
    assert_eq!(s["name"], "Pirata One");
    assert_eq!(s["heruntergeladen"], true);
    let g = ausfuehren(&k, "google_schriften_liste", json!({})).unwrap();
    let p = g["schriften"].as_array().unwrap().iter().find(|x| x["familie"] == "Pirata One").unwrap().clone();
    assert_eq!(p["geladen"], true);
    let d = json!({"objekte": [{"art": "ttf", "text": "Mia", "schrift": "Pirata One", "hoehe": 30}]});
    let e = ausfuehren(&k, "design_berechnen", json!({"design": d})).unwrap();
    assert!(e["stiche"].as_array().unwrap().len() > 100);
    let v = ausfuehren(&k, "schrift_vorschau", json!({"schrift": "Pirata One", "text": "Mia"})).unwrap();
    eprintln!("Pirata One stickbar ab {}", v["stickbarkeit"]["stickbar_ab"]);
    let _ = std::fs::remove_dir_all(&ablage);
}

/// Dicke Umrandung (eigene Erweiterung, Nutzer 21.11.2026): Satinkante außen in Prozent der Buchstabenhöhe wird als Band
/// in Satinsäulen zerlegt; bei kleiner Schrift (unter 1 mm Kante) wird daraus eine kräftige Linie statt Satin
#[test]
fn umrandung_kleine_schrift() {
    use stick_kern::design::{Design, ObjektArt};
    let k = kontext();
    let arten = |skalieren: f64| {
        let mut d = Design::aus_json(
            &json!({"objekte": [{"art": "text", "text": "anne", "alphabet": "Drifter", "hoehe": 20,
                "transform": {"skalieren": skalieren}, "umrandung": {"art": "satin", "breite_anteil": 6, "aussen": true}}]})
            .to_string(),
        )
        .unwrap();
        k.alphabete_einsetzen(&mut d).unwrap();
        let (obj, _) = d.objekte_aufgeloest().unwrap();
        obj.into_iter()
            .filter(|o| o.name.ends_with("(Umrandung)"))
            .map(|o| match o.art {
                ObjektArt::Gruppe { teile, .. } if teile.iter().all(|t| matches!(t.art, ObjektArt::Satin { .. })) => "satin",
                // kräftige Linie mit kurzen Stichen: 8 mm Schrift → 1 mm
                ObjektArt::Lauf { dreifach: true, laenge: Some(l), .. } if (l - 1.0).abs() < 1e-9 => "dreifach",
                _ => "anders",
            })
            .collect::<Vec<_>>()
    };
    // 20 mm: 1,2 mm Satinkante – je Buchstabe ein Band (a, n, n, e; Innenräume gehören zum Band oder sind eigene Bänder)
    let gross = arten(100.0);
    assert!(gross.len() >= 4 && gross.iter().all(|&a| a == "satin"), "{gross:?}");
    // 40 % → 8 mm: 0,48 mm wäre zu schmal → Dreifach um jeden Buchstaben und Innenraum
    let klein = arten(40.0);
    assert!(klein.len() >= 4 && klein.iter().all(|&a| a == "dreifach"), "{klein:?}");
}

/// Craft Stitches: Vorlagen rechnen mit einer Form, Rückstich/Stielstich/Stippling mit Werten des Originals,
/// Blackwork Run fasst Linien zusammen, Stichart wechseln kennt die neuen Arten
#[test]
fn craft_stitches() {
    let k = kontext();
    let v = ausfuehren(&k, "craft_vorlagen", json!({})).unwrap();
    let v = v.as_array().unwrap();
    assert_eq!(v.len(), 8);
    for x in v {
        let mut o = x["felder"].clone();
        if x["form"] == "linie" {
            o["punkte"] = json!([[-30, 0], [0, 10], [30, 0]]);
        } else {
            o["umriss"] = json!({"rechteck": {"mitte": [0, 0], "breite": 40, "hoehe": 30}});
        }
        let e = ausfuehren(&k, "design_berechnen", json!({"design": {"objekte": [o]}}))
            .unwrap_or_else(|f| panic!("{}: {}", x["id"], f.meldung));
        assert!(e["stiche"].as_array().unwrap().len() > 20, "{}", x["id"]);
    }
    // wirksame Werte des Rückstichs aus dem Stil
    let d = json!({"objekte": [{"art": "rueckstich", "punkte": [[0, 0], [30, 0]]}]});
    let w = ausfuehren(&k, "objekt_werte", json!({"design": d, "objekt": 0})).unwrap();
    let wert = |f: &str| w.as_array().unwrap().iter().find(|x| x["feld"] == f).unwrap()["wert"].clone();
    assert_eq!(wert("laenge"), json!(3.0));
    assert_eq!(wert("ueberlappung"), json!(50.0));
    // Werte außerhalb des Bereichs → Fehler mit Objektnummer
    let d = json!({"objekte": [{"art": "rueckstich", "punkte": [[0, 0], [30, 0]], "striche": 4}]});
    assert_eq!(ausfuehren(&k, "design_berechnen", json!({ "design": d })).unwrap_err().objekt, Some(0));
    // Blackwork Run: zwei Läufe und eine Satinlinie (ausgelassen) → ein Objekt mit zwei Linien
    let d = json!({"objekte": [
        {"art": "satinlinie", "punkte": [[0, 30], [20, 30]]},
        {"art": "rueckstich", "punkte": [[-20, 0], [20, 0]], "farbe": "#112233", "striche": 5},
        {"art": "lauf", "punkte": {"kreis": {"mitte": [0, 0], "radius": 10}}, "transform": {"verschieben": [5, 0]}}
    ]});
    let r = ausfuehren(&k, "blackwork_lauf", json!({"design": d, "objekte": [1, 2, 0]})).unwrap();
    assert_eq!(r["objekt"], json!(1));
    assert_eq!(r["hinweise"].as_array().unwrap().len(), 1);
    let o = &r["design"]["objekte"][1];
    assert_eq!(o["art"], "blackwork");
    assert_eq!(o["stich"], "rueckstich");
    assert_eq!(o["striche"], json!(5));
    assert_eq!(o["farbe"], "#112233");
    assert_eq!(o["linien"].as_array().unwrap().len(), 2);
    assert_eq!(r["design"]["objekte"].as_array().unwrap().len(), 2);
    let e = ausfuehren(&k, "design_berechnen", json!({"design": r["design"].clone()})).unwrap();
    // Kreis verschoben (Transformation angewandt): rechts bis 15 mm
    let b: [f64; 4] = serde_json::from_value(e["begrenzung"].clone()).unwrap();
    assert!(b[2] > 19.0 && b[3] > 29.0, "{b:?}");
    // nur ein Objekt → Fehler
    assert!(ausfuehren(&k, "blackwork_lauf", json!({"design": d, "objekte": [1, 0]})).is_err());
    // Stichart wechseln: Lauf → Rückstich, Tatami → Stippling
    let d = json!({"objekte": [
        {"art": "lauf", "punkte": [[0, 0], [20, 0]], "geschlossen": false},
        {"art": "tatami", "umriss": {"kreis": {"mitte": [0, 0], "radius": 10}}, "loecher": [{"kreis": {"mitte": [0, 0], "radius": 3}}]}
    ]});
    let r = ausfuehren(&k, "stichart_aendern", json!({"design": d, "objekt": 0, "art": "rueckstich"})).unwrap();
    assert_eq!(r["objekt"]["art"], "rueckstich");
    let r = ausfuehren(&k, "stichart_aendern", json!({"design": d, "objekt": 1, "art": "stippling"})).unwrap();
    assert_eq!(r["objekt"]["art"], "stippling");
    assert_eq!(r["objekt"]["loecher"].as_array().unwrap().len(), 1);
}
