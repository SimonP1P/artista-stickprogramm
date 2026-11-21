//! Kommandozeile zum Rechenkern:
//!
//!   stick erzeugen DESIGN.json [-o ORDNER] [--alphabete ORDNER]  Stiche berechnen -> .exp + .svg
//!   stick alphabete [ORDNER]                               Original-Alphabete (ESB) auflisten
//!   stick anzeigen DATEI.exp [-o DATEI.svg] [--rahmen NAME] EXP als SVG darstellen
//!   stick rahmen | stoffe                                  Rahmen bzw. Stoffe auflisten
//!   stick puffer EINGABE [-o PUFFER.bin] [--name NAME] [--ohne-anhang] [--farben-im-anhang]
//!                                                          Daten für die artista 185 bauen: Puffer DATEN,
//!                                                          Garnfarbtabelle und Icon (wie das Original)
//!                                                          (EINGABE: Design-JSON, .exp oder Design_NrXX_*.bin)
//!   stick bild BILD [--breite MM] [--farben N] [-o D.json]  Bild → Umrisse (Tatami-Flächen als Design-JSON)
//!   stick muster                                           Füllmuster der Vorlage auflisten
//!   stick saeulen FORMEN.json [-o AUS.json] [--svg AUS.svg] Flächen in Satinsäulen zerlegen (Stichrichtung
//!                                                          automatisch, `stick_kern::saeulen`); Eingabe
//!                                                          {"formen": [{"umriss": [[x, y], …], "loecher": […]}]}
//!   stick reihenfolge DESIGN.json [BEFEHL] [-o NEU.json]   Objekte auflisten, Überdeckungen prüfen, umordnen:
//!                                                          --nach-farbe [FARBEN] [--ueberdeckung] [--auswahl NR,…],
//!                                                          --wie-ausgewaehlt NR,… | --an-anfang NR,… | --ans-ende NR,…
//!                                                          | --vor NR --auswahl NR,… | --nach NR --auswahl NR,…
//!   stick schriften [--schriften ORDNER]                  Computer-Schriften (TrueType/OpenType) auflisten
//!   stick garnkarten [--garnkarten ORDNER]                 Garnkarten (.tch) auflisten
//!   stick maschine ports|pruefen|liste|lesen NR|senden EINGABE [--port COM3 | --simulation] …
//!                                                          artista 185 ansprechen (Protokoll der Testprogramme;
//!                                                          senden schreibt nur mit --schreiben, nach Rückfrage)

mod maschine;

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use stick_formate::artista::{N_185, Sendedaten, farben_in_stichen};
use stick_formate::esb;
use stick_formate::exp::{exp_bytes, exp_lesen};
use stick_formate::garnkarte;
use stick_formate::svg::{SvgOptionen, svg_text};
use stick_formate::ttf;
use stick_kern::einstellungen::STOFFE;
use stick_kern::rahmen::{alle_rahmen, rahmen};
use stick_kern::{Art, Design, StichFolge};

fn statistik(folge: &StichFolge) -> String {
    let mut z = vec![
        format!("  Stiche:       {}", folge.anzahl(Art::Stich)),
        format!("  Sprünge:      {}", folge.anzahl(Art::Sprung)),
        format!(
            "  Farbwechsel:  {}  (Farben: {})",
            folge.anzahl(Art::Farbwechsel),
            folge.farben.join(", ")
        ),
    ];
    if let Some((x0, y0, x1, y1)) = folge.begrenzung() {
        z.push(format!(
            "  Größe:        {:.1} × {:.1} mm  (x {x0:.1} … {x1:.1}, y {y0:.1} … {y1:.1})",
            x1 - x0,
            y1 - y0
        ));
    }
    let l = folge.stichlaengen();
    if !l.is_empty() {
        let (min, max) = l
            .iter()
            .fold((f64::INFINITY, 0.0f64), |(a, b), &x| (a.min(x), b.max(x)));
        z.push(format!("  Stichlänge:   min {min:.2} mm, max {max:.2} mm"));
    }
    z.join("\n")
}

fn option(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

/// Ordner der Alphabete: Schalter --alphabete, sonst Umgebungsvariable STICK_ALPHABETE, sonst „alphabete“.
fn alphabet_ordner(args: &[String]) -> PathBuf {
    option(args, "--alphabete")
        .or_else(|| std::env::var("STICK_ALPHABETE").ok())
        .unwrap_or_else(|| "alphabete".into())
        .into()
}

/// Ordner der Garnkarten: Schalter --garnkarten, sonst STICK_GARNKARTEN, sonst „garnkarten“, sonst die Karten der CD
/// im Projekt (`Analyse/daten/garnkarten`).
fn garnkarten_ordner(args: &[String]) -> PathBuf {
    if let Some(o) = option(args, "--garnkarten").or_else(|| std::env::var("STICK_GARNKARTEN").ok()) {
        return o.into();
    }
    let hier = PathBuf::from("garnkarten");
    if hier.is_dir() {
        return hier;
    }
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Analyse/daten/garnkarten"))
}

/// Ordner der Computer-Schriften: Schalter --schriften, sonst die Schriftordner des Systems.
fn schrift_ordner(args: &[String]) -> Vec<PathBuf> {
    option(args, "--schriften")
        .map(|o| vec![PathBuf::from(o)])
        .unwrap_or_default()
}

fn erzeugen(design_pfad: &Path, ordner: &Path, alphabete: &Path) -> Result<bool, String> {
    let text = std::fs::read_to_string(design_pfad).map_err(|e| format!("{}: {e}", design_pfad.display()))?;
    let mut design = Design::aus_json(&text)?;
    esb::design_alphabete_laden(&mut design, alphabete)?;
    ttf::design_schriften_laden(&mut design, &[])?;
    let (_, fehlend) = design.objekte_aufgeloest()?;
    if !fehlend.is_empty() {
        println!(
            "  Hinweis: Zeichen ohne Entsprechung im Alphabet: {}",
            fehlend.iter().collect::<String>()
        );
    }
    let folge = design.berechnen()?;
    std::fs::create_dir_all(ordner).map_err(|e| e.to_string())?;
    let basis = ordner.join(design.name.replace(' ', "_"));
    let daten = exp_bytes(&folge);
    let (exp, svg) = (basis.with_extension("exp"), basis.with_extension("svg"));
    std::fs::write(&exp, &daten).map_err(|e| e.to_string())?;
    let o = SvgOptionen {
        titel: design.name.clone(),
        ..Default::default()
    };
    std::fs::write(&svg, svg_text(&folge, Some(&design.rahmen), &o)).map_err(|e| e.to_string())?;
    println!(
        "Design '{}' – Rahmen {}, Stoff {}",
        design.name, design.rahmen.name, design.einstellungen.stoff
    );
    println!("{}", statistik(&folge));
    println!("  EXP:          {} ({} Byte)", exp.display(), daten.len());
    println!("  Vorschau:     {}", svg.display());
    let aussen = design.ausserhalb(&folge);
    if !aussen.is_empty() {
        println!("  WARNUNG: {} Einstiche liegen außerhalb des Stickfelds!", aussen.len());
    }
    Ok(aussen.is_empty())
}

/// Nummern „3,1,7“ (ab 1 wie die Color Film) → Stellen ab 0.
fn nummern(t: &str) -> Result<Vec<usize>, String> {
    t.split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|x| match x.trim().parse::<usize>() {
            Ok(n) if n >= 1 => Ok(n - 1),
            _ => Err(format!("Objektnummer „{x}“ ungültig (ab 1)")),
        })
        .collect()
}

/// `stick saeulen`: Flächen in Satinsäulen zerlegen (`stick_kern::saeulen`), Ergebnis als JSON und auf Wunsch als SVG.
fn saeulen(args: &[String], hilfe: &str) -> Result<bool, String> {
    use serde_json::{Value, json};
    use stick_kern::geometrie::{Linie, p};
    let pfad = PathBuf::from(args.get(1).filter(|a| !a.starts_with('-')).ok_or(hilfe)?);
    let text = std::fs::read_to_string(&pfad).map_err(|e| format!("{}: {e}", pfad.display()))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| format!("JSON ungültig: {e}"))?;
    let linie = |x: &Value| -> Linie {
        x.as_array()
            .map(|l| {
                l.iter()
                    .filter_map(|q| Some(p(q.get(0)?.as_f64()?, q.get(1)?.as_f64()?)))
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut werte = stick_kern::saeulen::SaeulenWerte::default();
    if let Some(t) = option(args, "--toleranz").and_then(|x| x.replace(',', ".").parse().ok()) {
        werte.linien_toleranz = t;
    }
    if let Some(t) = option(args, "--knick").and_then(|x| x.replace(',', ".").parse().ok()) {
        werte.knickwinkel = t;
    }
    let pt = |q: &stick_kern::geometrie::Punkt| json!([q.x, q.y]);
    let mut aus = Vec::new();
    let mut svg = String::new();
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    let formen = v
        .get("formen")
        .and_then(Value::as_array)
        .ok_or("Eingabe ohne „formen“")?;
    let mut anzahl = 0;
    for f in formen {
        let umriss = linie(&f["umriss"]);
        let loecher: Vec<Linie> = f
            .get("loecher")
            .and_then(Value::as_array)
            .map(|l| l.iter().map(linie).collect())
            .unwrap_or_default();
        let s = stick_kern::saeulen::zerlegen(&umriss, &loecher, &werte)?;
        anzahl += s.len();
        for r in std::iter::once(&umriss).chain(&loecher) {
            for q in r {
                (x0, y0, x1, y1) = (x0.min(q.x), y0.min(q.y), x1.max(q.x), y1.max(q.y));
            }
            svg.push_str(&format!(
                "<polygon points=\"{}\" fill=\"#eef\" stroke=\"#448\" stroke-width=\"0.05\"/>",
                r.iter()
                    .map(|q| format!("{:.3},{:.3}", q.x, -q.y))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        for x in &s {
            for (a, b) in x.seite_a.iter().zip(&x.seite_b) {
                svg.push_str(&format!(
                    "<line x1=\"{:.3}\" y1=\"{:.3}\" x2=\"{:.3}\" y2=\"{:.3}\" stroke=\"#999\" stroke-width=\"0.03\"/>",
                    a.x, -a.y, b.x, -b.y
                ));
            }
            for [a, b] in &x.linien {
                svg.push_str(&format!(
                    "<line x1=\"{:.3}\" y1=\"{:.3}\" x2=\"{:.3}\" y2=\"{:.3}\" stroke=\"#c00\" stroke-width=\"0.06\"/>",
                    a.x, -a.y, b.x, -b.y
                ));
            }
        }
        aus.push(json!({"saeulen": s.iter().map(|x| json!({
            "seite_a": x.seite_a.iter().map(pt).collect::<Vec<_>>(),
            "seite_b": x.seite_b.iter().map(pt).collect::<Vec<_>>(),
            "linien": x.linien.iter().map(|[a, b]| json!([pt(a), pt(b)])).collect::<Vec<_>>(),
        })).collect::<Vec<_>>()}));
    }
    let ziel = option(args, "-o")
        .map(PathBuf::from)
        .unwrap_or_else(|| pfad.with_extension("saeulen.json"));
    std::fs::write(
        &ziel,
        serde_json::to_string(&json!({"formen": aus})).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", ziel.display()))?;
    println!("  {} Formen → {} Säulen: {}", formen.len(), anzahl, ziel.display());
    if let Some(z) = option(args, "--svg") {
        let (w, h) = (x1 - x0 + 4.0, y1 - y0 + 4.0);
        let doc = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {w} {h}\" width=\"{:.0}\" height=\"{:.0}\">\
             <rect x=\"{}\" y=\"{}\" width=\"{w}\" height=\"{h}\" fill=\"white\"/>{svg}</svg>",
            x0 - 2.0,
            -y1 - 2.0,
            (w * 12.0).min(4000.0),
            (h * 12.0).min(4000.0),
            x0 - 2.0,
            -y1 - 2.0
        );
        std::fs::write(&z, doc).map_err(|e| format!("{z}: {e}"))?;
        println!("  Vorschau: {z}");
    }
    Ok(true)
}

/// `stick reihenfolge`: Objekte mit Farbe auflisten, Überdeckungen und Farbwechsel zeigen, auf Wunsch umordnen
/// (Handbuch Kap. 14, `stick_kern::reihenfolge`).
fn reihenfolge(args: &[String], hilfe: &str) -> Result<bool, String> {
    use stick_kern::reihenfolge::{Befehl, ausfuehren, farbwechsel, objekt_farben, ueberdeckungen};
    let pfad = PathBuf::from(args.get(1).filter(|a| !a.starts_with('-')).ok_or(hilfe)?);
    let text = std::fs::read_to_string(&pfad).map_err(|e| format!("{}: {e}", pfad.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("JSON ungültig: {e}"))?;
    // berechenbares Design für die Prüfung auf Überdeckungen (Texte brauchen die Alphabete)
    let design = (|| -> Result<Design, String> {
        let mut d = Design::aus_value(&v)?;
        esb::design_alphabete_laden(&mut d, &alphabet_ordner(args))?;
        ttf::design_schriften_laden(&mut d, &schrift_ordner(args))?;
        Ok(d)
    })();
    let schalter = |n: &str| args.iter().any(|a| a == n);
    let liste = |n: &str| option(args, n).map(|t| nummern(&t)).transpose();
    let auswahl = liste("--auswahl")?.unwrap_or_default();
    let befehl = if schalter("--nach-farbe") {
        let farben = option(args, "--nach-farbe")
            .filter(|x| !x.starts_with("--"))
            .map(|x| {
                x.split(',')
                    .map(|f| f.trim().to_string())
                    .filter(|f| !f.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        Some(Befehl::NachFarbe {
            auswahl,
            farben,
            ueberdeckung: schalter("--ueberdeckung"),
        })
    } else if let Some(a) = liste("--wie-ausgewaehlt")? {
        Some(Befehl::WieAusgewaehlt(a))
    } else if let Some(a) = liste("--an-anfang")? {
        Some(Befehl::AnAnfang(a))
    } else if let Some(a) = liste("--ans-ende")? {
        Some(Befehl::AnsEnde(a))
    } else if let Some(z) = liste("--vor")?.or(liste("--nach")?) {
        let [ziel] = z[..] else {
            return Err("--vor/--nach: genau eine Objektnummer".into());
        };
        Some(Befehl::NachNummer {
            auswahl,
            ziel,
            vor: schalter("--vor"),
        })
    } else {
        None
    };
    let farben = objekt_farben(&v);
    let objekte = v
        .get("objekte")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let beschreibung = |i: usize| {
        let o = &objekte[i];
        let name = o.get("name").and_then(serde_json::Value::as_str).unwrap_or("");
        let art = o.get("art").and_then(serde_json::Value::as_str).unwrap_or("?");
        format!("{:>3}  {:<8} {:<11} {}", i + 1, farben[i], art, name)
    };
    let paare = match &design {
        Ok(d) => Some(ueberdeckungen(d)?),
        Err(e) => {
            println!("  Hinweis: Überdeckungen nicht geprüft ({e})");
            None
        }
    };
    let Some(befehl) = befehl else {
        println!("Objekte in Stickreihenfolge (Nr, Farbe, Art, Name):");
        for i in 0..objekte.len() {
            println!("  {}", beschreibung(i));
        }
        let alle: Vec<usize> = (0..objekte.len()).collect();
        println!("  Farbwechsel: {}", farbwechsel(&farben, &alle));
        if let Some(p) = &paare {
            println!("  Überdeckungen (früher → später gestickt): {}", p.len());
            for (a, b) in p {
                println!("    {} → {}", a + 1, b + 1);
            }
        }
        return Ok(true);
    };
    let e = ausfuehren(&v, design.as_ref().ok(), &befehl)?;
    println!("Neue Reihenfolge (bisherige Nummern):");
    for &i in &e.reihenfolge {
        println!("  {}", beschreibung(i));
    }
    println!("  Farbwechsel: {} → {}", e.farbwechsel_vorher, e.farbwechsel_nachher);
    for (oben, unten) in &e.vertauscht {
        println!(
            "  WARNUNG: Objekt {} lag über Objekt {} und liegt jetzt darunter (Flächen überdecken sich)",
            oben + 1,
            unten + 1
        );
    }
    let ziel = option(args, "-o").map(PathBuf::from).unwrap_or_else(|| {
        let stamm = pfad.file_stem().unwrap_or_default().to_string_lossy().to_string();
        pfad.with_file_name(format!("{stamm}_reihenfolge.json"))
    });
    std::fs::write(
        &ziel,
        serde_json::to_string_pretty(&e.design).map_err(|x| x.to_string())?,
    )
    .map_err(|x| format!("{}: {x}", ziel.display()))?;
    println!("  Design: {}", ziel.display());
    Ok(e.vertauscht.is_empty())
}

/// `stick api`: Befehle der Oberfläche ohne Oberfläche ausprobieren (`stick-api`), Schnittstelle neu erzeugen.
fn api(args: &[String]) -> Result<bool, String> {
    let mut k = stick_api::Kontext::finden(None);
    if let Some(o) = option(args, "--alphabete") {
        k.alphabete = o.into();
    }
    if let Some(o) = option(args, "--garnkarten") {
        k.garnkarten = o.into();
    }
    k.schriften = schrift_ordner(args);
    match args.get(1).map(String::as_str) {
        None | Some("liste") => {
            for b in stick_api::alle() {
                println!("  {:<22} {}", b.name, b.beschreibung.trim());
            }
            println!("  Aufruf: stick api NAME [JSON | DATEI.json | -]   ·   stick api exportieren");
            Ok(true)
        }
        Some("exportieren") => {
            let wurzel = args
                .get(2)
                .filter(|a| !a.starts_with('-'))
                .map(PathBuf::from)
                .unwrap_or_else(stick_api::erzeugen::repo_wurzel);
            let geaendert = stick_api::erzeugen::schreiben(&wurzel)?;
            if geaendert.is_empty() {
                println!("  Schnittstelle ist aktuell");
            }
            for p in geaendert {
                println!("  neu erzeugt: {p}");
            }
            Ok(true)
        }
        Some(name) => {
            let daten: serde_json::Value = match args.get(2).filter(|a| !a.starts_with("--")).map(String::as_str) {
                None => serde_json::Value::Null,
                Some("-") => {
                    let mut t = String::new();
                    std::io::Read::read_to_string(&mut std::io::stdin(), &mut t).map_err(|e| e.to_string())?;
                    serde_json::from_str(&t).map_err(|e| format!("JSON ungültig: {e}"))?
                }
                Some(t) if t.trim_start().starts_with(['{', '[']) => {
                    serde_json::from_str(t).map_err(|e| format!("JSON ungültig: {e}"))?
                }
                Some(pfad) => {
                    let t = std::fs::read_to_string(pfad).map_err(|e| format!("{pfad}: {e}"))?;
                    serde_json::from_str(&t).map_err(|e| format!("{pfad}: JSON ungültig: {e}"))?
                }
            };
            match stick_api::ausfuehren(&k, name, daten) {
                Ok(v) => {
                    let text = serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?;
                    match option(args, "-o") {
                        Some(z) => std::fs::write(&z, text).map_err(|e| format!("{z}: {e}"))?,
                        None => println!("{text}"),
                    }
                    Ok(true)
                }
                Err(f) => Err(serde_json::to_string(&f).unwrap_or(f.meldung)),
            }
        }
    }
}

/// Sendedaten aus einer Eingabedatei: Design-JSON (berechnen), .exp oder aus der Maschine gelesenes Design (.bin).
fn sendedaten_laden(eingabe: &Path, args: &[String]) -> Result<Sendedaten, String> {
    let farben_im_anhang = args.iter().any(|a| a == "--farben-im-anhang");
    let stamm = eingabe.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let endung = eingabe.extension().unwrap_or_default().to_string_lossy().to_lowercase();
    let name = option(args, "--name");
    let mut karte = option(args, "--garnkarte");
    let lesen = || std::fs::read(eingabe).map_err(|e| format!("{}: {e}", eingabe.display()));
    let mut s = match endung.as_str() {
        "bin" => Sendedaten::aus_gelesenem(&lesen()?, farben_im_anhang)?,
        "exp" => {
            let d = lesen()?;
            println!("{}", statistik(&exp_lesen(&d)));
            Sendedaten::aus_exp(&name.unwrap_or(stamm), &d, farben_im_anhang)
        }
        "json" => {
            let text = std::fs::read_to_string(eingabe).map_err(|e| format!("{}: {e}", eingabe.display()))?;
            if karte.is_none() {
                karte = serde_json::from_str::<serde_json::Value>(&text)
                    .ok()
                    .and_then(|v| v.get("garnkarte")?.as_str().map(String::from));
            }
            let mut design = Design::aus_json(&text)?;
            esb::design_alphabete_laden(&mut design, &alphabet_ordner(args))?;
            ttf::design_schriften_laden(&mut design, &schrift_ordner(args))?;
            let f = design.berechnen()?;
            println!("{}", statistik(&f));
            Sendedaten::eigenes_design(&name.unwrap_or_else(|| design.name.clone()), &f, farben_im_anhang)
        }
        _ => {
            return Err(format!(
                "unbekannte Eingabe {} (erwartet .json, .exp oder .bin)",
                eingabe.display()
            ));
        }
    };
    if args.iter().any(|a| a == "--ohne-anhang") {
        s.ohne_anhang();
    }
    // Garnangaben: Farben ohne Marke dem nächsten Garn der Karte zuordnen (Standard „Default“, wie die Maschine)
    let karte = karte.unwrap_or_else(|| "Default".into());
    if karte != "keine"
        && let Ok(a) = garnkarte::laden(&garnkarten_ordner(args), "Applique")
    {
        s.garne_exakt_zuordnen(&a); // Applikations-Lagen: Anweisung im Display
    }
    if karte != "keine" && s.farben.iter().any(|f| f.marke.is_empty()) {
        let k = garnkarte::laden(&garnkarten_ordner(args), &karte)?;
        for (i, f) in s.garne_zuordnen(&k).iter().enumerate() {
            println!(
                "  Garn {}: {} {} {} (RGB {:02X}{:02X}{:02X})",
                i + 1,
                f.marke,
                f.nummer,
                f.name,
                f.rgb[0],
                f.rgb[1],
                f.rgb[2]
            );
        }
    }
    Ok(s)
}

/// Daten für die artista 185 bauen wie BEAGLink 0x100032b0 bei Firmware ≥ 3.09 ([`Sendedaten`]):
/// - NAME_puffer.bin: Bytes für den Puffer DATEN; der Anhang enthält nur den Kopf-Chunk (Farben und Icon gehen
///   getrennt; mit `--farben-im-anhang` wie im Gerätetest v4 vom 19.11.2026 mit Farbtabelle im Anhang),
/// - NAME_farben.bin: Garnfarbtabelle für T + Vorspann (Schritt 12; nur wenn das Design Farben hat),
/// - NAME_icon.bin: Puffer ICON (Schritt 8; Icon aus den Stichen gezeichnet),
/// - NAME_puffer.txt: Längen, INFO-Felder und Dateinamen für das Testprogramm.
///
/// Aus der Maschine gelesene Designs (Design_NrXX_*.bin): ein ungültiger Anhang (Rest des vorher geladenen Designs)
/// wird entfernt, ein gültiger wie oben ohne Farbtabelle neu verschleiert.
fn puffer_erzeugen(args: &[String], hilfe: &str) -> Result<bool, String> {
    let eingabe = PathBuf::from(args.get(1).ok_or(hilfe)?);
    let farben_im_anhang = args.iter().any(|a| a == "--farben-im-anhang");
    let s = sendedaten_laden(&eingabe, args)?;
    let stamm = eingabe.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let ziel = option(args, "-o")
        .map(PathBuf::from)
        .unwrap_or_else(|| eingabe.with_file_name(format!("{stamm}_puffer.bin")));
    let bytes = s.puffer.bytes();
    std::fs::write(&ziel, &bytes).map_err(|e| format!("{}: {e}", ziel.display()))?;
    let basis = ziel.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let basis = basis.strip_suffix("_puffer").unwrap_or(&basis).to_string();
    // Garnfarbtabelle (Schritt 12) – ohne Farben schreibt das Original keine Tabelle
    let ziel_farben = ziel.with_file_name(format!("{basis}_farben.bin"));
    let tabelle = s.farbtabelle();
    match &tabelle {
        Some(t) => std::fs::write(&ziel_farben, t).map_err(|e| format!("{}: {e}", ziel_farben.display()))?,
        None => {
            let _ = std::fs::remove_file(&ziel_farben);
        }
    }
    // Icon (Schritt 8), aus den Stichen gezeichnet
    let icon = s.icon.bytes();
    let ziel_icon = ziel.with_file_name(format!("{basis}_icon.bin"));
    std::fs::write(&ziel_icon, &icon).map_err(|e| format!("{}: {e}", ziel_icon.display()))?;
    let dateiname = |p: &Path| p.file_name().unwrap_or_default().to_string_lossy().to_string();
    let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02X}")).collect::<String>();
    let zeilen = [
        format!(
            "# Puffer DATEN für die artista 185 (stick puffer), Quelle {}",
            eingabe.display()
        ),
        format!("laenge={}", bytes.len()),
        format!("laenge_stiche={}", s.puffer.stiche.len()),
        format!("laenge_anhang={}", s.puffer.anhang.len()),
        format!("farben={}", farben_in_stichen(&s.puffer.stiche)),
        format!("info_farben_minus_1={:02X}", s.info.farben_minus_1),
        format!("info_flags={:02X}", s.info.flags),
        format!("info_name={}", hex(&s.info.name)),
        format!("name={}", s.name),
        format!(
            "farben_im_anhang={}",
            u8::from(farben_im_anhang && !s.puffer.anhang.is_empty())
        ),
        format!(
            "datei_farbtabelle={}",
            if tabelle.is_some() {
                dateiname(&ziel_farben)
            } else {
                String::new()
            }
        ),
        format!("farbtabelle_farben={}", s.farben.len()),
        format!("farbtabelle_laenge={}", tabelle.as_ref().map_or(0, Vec::len)),
        format!("datei_icon={}", dateiname(&ziel_icon)),
        format!("icon_laenge={}", icon.len()),
    ];
    let txt = zeilen.join("\r\n") + "\r\n";
    let ziel_txt = ziel.with_extension("txt");
    std::fs::write(&ziel_txt, txt).map_err(|e| format!("{}: {e}", ziel_txt.display()))?;
    println!(
        "  Puffer DATEN: {} ({} Byte: Kopf {}, Stiche {}, Anhang {})",
        ziel.display(),
        bytes.len(),
        6 * N_185 + 8,
        s.puffer.stiche.len(),
        s.puffer.anhang.len()
    );
    println!(
        "  INFO-Felder:  Farben−1 {:02X}, Flags {:02X}, Name '{}'  → {}",
        s.info.farben_minus_1,
        s.info.flags,
        s.name,
        ziel_txt.display()
    );
    match &tabelle {
        None => println!("  Farbtabelle:  keine (Design ohne Farbangaben)"),
        Some(t) => println!(
            "  Farbtabelle:  {} ({} Farben, {} Byte){}",
            ziel_farben.display(),
            s.farben.len(),
            t.len(),
            if farben_im_anhang {
                " – Farben zusätzlich im Anhang"
            } else {
                ""
            }
        ),
    }
    println!("  Icon:         {} ({} Byte, 72 × 62)", ziel_icon.display(), icon.len());
    Ok(true)
}

fn ausfuehren(args: &[String]) -> Result<bool, String> {
    let hilfe = "Befehle: erzeugen DESIGN.json [-o ORDNER] [--alphabete ORDNER] | anzeigen DATEI.exp [-o DATEI.svg] [--rahmen NAME] | alphabete [ORDNER] | knopfloecher [ORDNER] | rahmen | stoffe | puffer EINGABE [-o PUFFER.bin] [--name NAME] [--ohne-anhang] [--farben-im-anhang] [--garnkarte NAME|keine] | garnkarten | schriften [--schriften ORDNER] | muster | saeulen FORMEN.json [-o AUS.json] [--svg AUS.svg] | reihenfolge DESIGN.json [--nach-farbe [FARBEN] [--ueberdeckung] | --wie-ausgewaehlt NR,… | --an-anfang NR,… | --ans-ende NR,… | --vor NR | --nach NR] [--auswahl NR,…] [-o NEU.json] | parameter [--modus einfach|experte|automatisch] [--markdown] | bild BILD [--breite MM] [--farben N] | svg DATEI.svg [--breite MM] [--rahmen NAME] [--ohne-aussparen] [-o DESIGN.json] | api [liste | exportieren | NAME [JSON|DATEI|-] [-o AUS.json]] | maschine …";
    match args.first().map(String::as_str) {
        Some("erzeugen") => {
            let design = args.get(1).ok_or(hilfe)?;
            let ordner = option(args, "-o").unwrap_or_else(|| ".".into());
            erzeugen(Path::new(design), Path::new(&ordner), &alphabet_ordner(args))
        }
        Some("knopfloecher") => {
            let ordner = args
                .get(1)
                .filter(|a| !a.starts_with('-'))
                .map(PathBuf::from)
                .unwrap_or_else(|| alphabet_ordner(args));
            let datei = esb::knopfloch_datei(&ordner)
                .ok_or_else(|| format!("ButtonHoles.ESB nicht in {}", ordner.display()))?;
            let v = esb::knopfloecher_laden(&datei)?;
            for (json, name, nr, dialog) in stick_kern::knopfloch::TYPEN {
                let Some(t) = v.get(name) else {
                    println!("  {json:<16} fehlt in ButtonHoles.ESB");
                    continue;
                };
                println!("  {json:<16} {dialog:<15} Typ {nr:#06x}  Schlitz {:.2} mm", t.laenge);
                for teil in &t.teile {
                    use stick_kern::knopfloch::TeilArt;
                    let (art, pts) = match &teil.art {
                        TeilArt::Handstiche(l) => ("Handstiche".to_string(), l.clone()),
                        TeilArt::Lauf { punkte, laenge, .. } => (format!("Lauf {laenge:?}"), punkte.clone()),
                        TeilArt::Satin {
                            seite_a,
                            seite_b,
                            abstand,
                            unterlagen,
                            ..
                        } => (
                            format!("Satin Abstand {abstand:?} Unterlage {unterlagen:?}"),
                            seite_a.iter().chain(seite_b).copied().collect(),
                        ),
                        TeilArt::Blanket {
                            ruecken,
                            aussen,
                            abstand,
                        } => (
                            format!("Blanket Abstand {abstand:?}"),
                            ruecken.iter().chain(aussen).copied().collect(),
                        ),
                    };
                    let (x0, x1, y0, y1) = pts
                        .iter()
                        .fold((f64::MAX, f64::MIN, f64::MAX, f64::MIN), |(a, b, c, d), q| {
                            (a.min(q.x), b.max(q.x), c.min(q.y), d.max(q.y))
                        });
                    println!(
                        "      {:?} {art}: {} Punkte, x {x0:.2} … {x1:.2}, y {y0:.2} … {y1:.2}",
                        teil.lage,
                        pts.len()
                    );
                }
            }
            Ok(true)
        }
        Some("alphabete") => {
            let ordner = args
                .get(1)
                .filter(|a| !a.starts_with('-'))
                .map(PathBuf::from)
                .unwrap_or_else(|| alphabet_ordner(args));
            let liste = esb::verzeichnis(&ordner);
            if liste.is_empty() {
                return Err(format!("keine ESB-Dateien in {}", ordner.display()));
            }
            for (name, pfad) in liste {
                let a = esb::laden(&pfad)?;
                println!(
                    "  {:<24} {:3} Zeichen  Abstand {:4.0} %  {}",
                    name,
                    a.zeichen.len(),
                    a.abstand_anteil * 100.0,
                    pfad.file_name().unwrap_or_default().to_string_lossy()
                );
            }
            Ok(true)
        }
        Some("anzeigen") => {
            let exp = PathBuf::from(args.get(1).ok_or(hilfe)?);
            let daten = std::fs::read(&exp).map_err(|e| format!("{}: {e}", exp.display()))?;
            let folge = exp_lesen(&daten);
            let r = option(args, "--rahmen").map(|n| rahmen(&n)).transpose()?;
            let ziel = option(args, "-o")
                .map(PathBuf::from)
                .unwrap_or_else(|| exp.with_extension("svg"));
            let o = SvgOptionen {
                titel: exp.file_name().unwrap_or_default().to_string_lossy().into(),
                ..Default::default()
            };
            std::fs::write(&ziel, svg_text(&folge, r.as_ref(), &o)).map_err(|e| e.to_string())?;
            println!("{}", statistik(&folge));
            println!("  Vorschau:     {}", ziel.display());
            Ok(true)
        }
        Some("puffer") => puffer_erzeugen(args, hilfe),
        Some("reihenfolge") => reihenfolge(args, hilfe),
        Some("saeulen") => saeulen(args, hilfe),
        Some("maschine") => maschine::ausfuehren(args),
        Some("api") => api(args),
        Some("svg") => {
            let pfad = PathBuf::from(args.get(1).ok_or(
                "stick svg DATEI.svg [--breite MM] [--rahmen NAME] [--ohne-aussparen] [--ohne-closest-join] [-o DESIGN.json]",
            )?);
            let st = stick_vektor::svg::SvgOptionen::default();
            let o = stick_vektor::svg::SvgOptionen {
                breite_mm: option(args, "--breite").and_then(|x| x.replace(',', ".").parse().ok()),
                aussparen: !args.iter().any(|a| a == "--ohne-aussparen"),
                kuerzeste_verbindung: !args.iter().any(|a| a == "--ohne-closest-join"),
                rahmen: option(args, "--rahmen").unwrap_or(st.rahmen),
            };
            let e = stick_vektor::svg::svg_datei(&pfad, &o)?;
            let ziel = option(args, "-o")
                .map(PathBuf::from)
                .unwrap_or_else(|| pfad.with_extension("json"));
            std::fs::write(
                &ziel,
                serde_json::to_string_pretty(&e.design).map_err(|x| x.to_string())?,
            )
            .map_err(|x| format!("{}: {x}", ziel.display()))?;
            println!(
                "  Zeichnung {:.1} × {:.1} mm, {} Objekte",
                e.breite_mm,
                e.hoehe_mm,
                e.design["objekte"].as_array().map_or(0, Vec::len)
            );
            for (k, n) in &e.klassen {
                println!("    {k:<22} {n}");
            }
            for h in &e.hinweise {
                println!("  Hinweis: {h}");
            }
            println!("  Design: {} (mit „stick erzeugen“ berechnen)", ziel.display());
            Ok(true)
        }
        Some("bild") => {
            let pfad = PathBuf::from(args.get(1).ok_or(
                "stick bild BILD [--breite MM] [--farben N] [--rahmen NAME] [--mit-hintergrund] [-o DESIGN.json]",
            )?);
            let st = stick_vektor::BildOptionen::default();
            let zahl = |n: &str, v: f64| {
                option(args, n)
                    .and_then(|x| x.replace(',', ".").parse().ok())
                    .unwrap_or(v)
            };
            let o = stick_vektor::BildOptionen {
                breite_mm: zahl("--breite", st.breite_mm),
                farben: zahl("--farben", st.farben as f64) as usize,
                min_flaeche: zahl("--min-flaeche", st.min_flaeche),
                glaettung: zahl("--glaettung", st.glaettung),
                ohne_hintergrund: !args.iter().any(|a| a == "--mit-hintergrund"),
            };
            let bild = stick_vektor::bild_laden(&pfad)?;
            let f = stick_vektor::ordnen(stick_vektor::vektorisieren(&bild, &o));
            let stamm = pfad.file_stem().unwrap_or_default().to_string_lossy().to_string();
            let rahmen = option(args, "--rahmen").unwrap_or_else(|| "artista 130 x 100".into());
            let json = stick_vektor::design_json(&f, &stamm, &rahmen);
            let ziel = option(args, "-o")
                .map(PathBuf::from)
                .unwrap_or_else(|| pfad.with_extension("json"));
            std::fs::write(&ziel, serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?)
                .map_err(|e| format!("{}: {e}", ziel.display()))?;
            let mut farben: Vec<[u8; 3]> = f.iter().map(|x| x.farbe).collect();
            farben.dedup();
            println!(
                "  {} Flächen in {} Farben, Bild {} × {} Punkte → {:.0} mm breit",
                f.len(),
                farben.len(),
                bild.width(),
                bild.height(),
                o.breite_mm
            );
            println!(
                "  Design: {} (Tatami-Flächen, mit „stick erzeugen“ berechnen)",
                ziel.display()
            );
            Ok(true)
        }
        Some("parameter") => {
            let modus = option(args, "--modus");
            if args.iter().any(|a| a == "--markdown") {
                print!("{}", stick_kern::parameter::markdown(modus.as_deref()));
                return Ok(true);
            }
            let k: Vec<_> = stick_kern::parameter::katalog()
                .into_iter()
                .filter(|p| modus.as_deref().is_none_or(|m| p.modus == m))
                .collect();
            for p in &k {
                let wo = [
                    p.json_design.clone(),
                    p.json_objekt.as_ref().map(|f| format!("Objekt: {f}")),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" / ");
                println!(
                    "  [{:<11}] {:<24} {:<36} = {} {}  ({wo})",
                    p.modus,
                    p.gruppe,
                    p.name,
                    stick_kern::parameter::text(&p.standard),
                    p.einheit
                );
            }
            println!("  {} Parameter (Stand {})", k.len(), stick_kern::parameter::stand());
            Ok(true)
        }
        Some("muster") => {
            for m in stick_kern::muster::alle() {
                println!(
                    "  {:<28} Winkel {:>4}  Kachel A ({:.1}, {:.1}) B ({:.1}, {:.1}) mm",
                    m.name,
                    m.winkel.map_or("–".into(), |w| format!("{w}°")),
                    m.a[0],
                    m.a[1],
                    m.b[0],
                    m.b[1]
                );
            }
            println!("  {} Füllmuster", stick_kern::muster::alle().len());
            Ok(true)
        }
        Some("schriften") => {
            let liste = ttf::verzeichnis(&match schrift_ordner(args) {
                o if o.is_empty() => ttf::system_ordner(),
                o => o,
            });
            for e in &liste {
                println!("  {:<44} {}", e.name, e.pfad.display());
            }
            println!("  {} Schriften", liste.len());
            Ok(true)
        }
        Some("garnkarten") => {
            let ordner = garnkarten_ordner(args);
            let liste = garnkarte::verzeichnis(&ordner);
            if liste.is_empty() {
                return Err(format!("keine Garnkarten in {}", ordner.display()));
            }
            for (name, pfad) in liste {
                let k = garnkarte::Garnkarte::lesen(&pfad)?;
                println!("  {:<38} {:4} Garne  Marke {}", name, k.garne.len(), k.marke());
            }
            Ok(true)
        }
        Some("rahmen") => {
            for r in alle_rahmen() {
                println!("  {:<36} {:6.1} × {:6.1} mm  {}", r.name, r.breite, r.hoehe, r.hinweis);
            }
            Ok(true)
        }
        Some("stoffe") => {
            println!("  {:<22} Satin fest  Tatami  Faktor  Zugausgl.", "Stoff");
            for s in &STOFFE {
                println!(
                    "  {:<22} {:9.2}  {:6.2}  {:6.2}  {:9.2}",
                    s.name, s.satin_abstand_fest, s.tatami_abstand, s.auto_spacing_faktor, s.zugausgleich
                );
            }
            Ok(true)
        }
        _ => Err(hilfe.into()),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match ausfuehren(&args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("Fehler: {e}");
            ExitCode::from(2)
        }
    }
}
