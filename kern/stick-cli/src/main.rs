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

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use stick_formate::artista::{
    CHUNK_FARBEN, CHUNK_KOPF, Chunk, Container, Farbe, Icon, Kopf, N_185, Puffer, farben_in_stichen,
    farbtabelle_maschine,
};
use stick_formate::esb;
use stick_formate::exp::{exp_bytes, exp_lesen};
use stick_formate::svg::{SvgOptionen, svg_text};
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

fn erzeugen(design_pfad: &Path, ordner: &Path, alphabete: &Path) -> Result<bool, String> {
    let text = std::fs::read_to_string(design_pfad).map_err(|e| format!("{}: {e}", design_pfad.display()))?;
    let mut design = Design::aus_json(&text)?;
    esb::design_alphabete_laden(&mut design, alphabete)?;
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

/// Container (Kopf + Farbtabelle) für ein eigenes Design; Kopf nach dem Muster der Karten-Designs
/// (`Kopf::neu`, eigene Festlegung).
fn container_fuer(name: &str, folge: &StichFolge) -> Container {
    let (breite, hoehe) = folge
        .begrenzung()
        .map_or((0.0, 0.0), |(x0, y0, x1, y1)| (x1 - x0, y1 - y0));
    let farben = folge.anzahl(Art::Farbwechsel) + 1;
    let zehntel = |v: f64| (v * 10.0).round().clamp(0.0, u16::MAX as f64) as u16;
    let kopf = Kopf::neu(
        name,
        farben as u16,
        zehntel(breite),
        zehntel(hoehe),
        folge.anzahl(Art::Stich),
    );
    let tabelle: Vec<u8> = (0..farben)
        .flat_map(|i| Farbe::aus_hex(folge.farben.get(i).map_or("#000000", String::as_str)).bytes())
        .collect();
    Container {
        kennung: 0,
        wort: 0,
        chunks: vec![
            Chunk {
                id: CHUNK_KOPF,
                daten: kopf.chunk_daten(),
            },
            Chunk {
                id: CHUNK_FARBEN,
                daten: tabelle,
            },
        ],
    }
}

/// Daten für die artista 185 bauen wie BEAGLink 0x100032b0 bei Firmware ≥ 3.09:
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
    let ohne_anhang = args.iter().any(|a| a == "--ohne-anhang");
    let farben_im_anhang = args.iter().any(|a| a == "--farben-im-anhang");
    let anhang_aus = |c: &Container| -> Vec<u8> {
        if farben_im_anhang {
            c.verschleiert()
        } else {
            c.anhang_wie_original().verschleiert()
        }
    };
    let stamm = eingabe.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let endung = eingabe.extension().unwrap_or_default().to_string_lossy().to_lowercase();
    let mut name = option(args, "--name").unwrap_or_else(|| stamm.clone());
    let (mut puffer, container, folge) = match endung.as_str() {
        "bin" => {
            let roh = std::fs::read(&eingabe).map_err(|e| format!("{}: {e}", eingabe.display()))?;
            let mut p = Puffer::lesen(&roh, N_185)?;
            let c = p.container().transpose()?;
            match &c {
                Some(c) => p.anhang = anhang_aus(c),
                None => p.anhang.clear(),
            }
            let f = exp_lesen(&p.stiche);
            (p, c, f)
        }
        "exp" | "json" => {
            let (exp, folge) = if endung == "exp" {
                let d = std::fs::read(&eingabe).map_err(|e| format!("{}: {e}", eingabe.display()))?;
                let f = exp_lesen(&d);
                (d, f)
            } else {
                let text = std::fs::read_to_string(&eingabe).map_err(|e| format!("{}: {e}", eingabe.display()))?;
                let mut design = Design::aus_json(&text)?;
                esb::design_alphabete_laden(&mut design, &alphabet_ordner(args))?;
                if option(args, "--name").is_none() {
                    name = design.name.clone();
                }
                let f = design.berechnen()?;
                (exp_bytes(&f), f)
            };
            println!("{}", statistik(&folge));
            let c = container_fuer(&name, &folge);
            let mut p = Puffer::neu(&exp, None);
            p.anhang = anhang_aus(&c);
            (p, Some(c), folge)
        }
        _ => {
            return Err(format!(
                "unbekannte Eingabe {} (erwartet .json, .exp oder .bin)",
                eingabe.display()
            ));
        }
    };
    if ohne_anhang {
        puffer.anhang.clear();
    }
    let ziel = option(args, "-o")
        .map(PathBuf::from)
        .unwrap_or_else(|| eingabe.with_file_name(format!("{stamm}_puffer.bin")));
    let bytes = puffer.bytes();
    std::fs::write(&ziel, &bytes).map_err(|e| format!("{}: {e}", ziel.display()))?;
    let basis = ziel.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let basis = basis.strip_suffix("_puffer").unwrap_or(&basis).to_string();
    // Garnfarbtabelle (Schritt 12) aus Chunk 0x14 – ohne Farben schreibt das Original keine Tabelle
    let garn = container.as_ref().map(Container::farben).unwrap_or_default();
    let ziel_farben = ziel.with_file_name(format!("{basis}_farben.bin"));
    let tabelle = farbtabelle_maschine(&garn);
    if garn.is_empty() {
        let _ = std::fs::remove_file(&ziel_farben);
    } else {
        std::fs::write(&ziel_farben, &tabelle).map_err(|e| format!("{}: {e}", ziel_farben.display()))?;
    }
    // Icon (Schritt 8), aus den Stichen gezeichnet
    let icon = Icon::aus_stichen(&folge).bytes();
    let ziel_icon = ziel.with_file_name(format!("{basis}_icon.bin"));
    std::fs::write(&ziel_icon, &icon).map_err(|e| format!("{}: {e}", ziel_icon.display()))?;
    let dateiname = |p: &Path| p.file_name().unwrap_or_default().to_string_lossy().to_string();
    let farben = farben_in_stichen(&puffer.stiche);
    let info = puffer.info_felder(farben);
    let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02X}")).collect::<String>();
    let kopf_name = puffer
        .container()
        .and_then(|c| c.ok())
        .and_then(|c| c.kopf())
        .map(|k| k.name())
        .unwrap_or_default();
    let zeilen = [
        format!(
            "# Puffer DATEN für die artista 185 (stick puffer), Quelle {}",
            eingabe.display()
        ),
        format!("laenge={}", bytes.len()),
        format!("laenge_stiche={}", puffer.stiche.len()),
        format!("laenge_anhang={}", puffer.anhang.len()),
        format!("farben={farben}"),
        format!("info_farben_minus_1={:02X}", info.farben_minus_1),
        format!("info_flags={:02X}", info.flags),
        format!("info_name={}", hex(&info.name)),
        format!("name={kopf_name}"),
        format!(
            "farben_im_anhang={}",
            u8::from(farben_im_anhang && !puffer.anhang.is_empty())
        ),
        format!(
            "datei_farbtabelle={}",
            if garn.is_empty() {
                String::new()
            } else {
                dateiname(&ziel_farben)
            }
        ),
        format!("farbtabelle_farben={}", garn.len()),
        format!("farbtabelle_laenge={}", if garn.is_empty() { 0 } else { tabelle.len() }),
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
        puffer.stiche.len(),
        puffer.anhang.len()
    );
    println!(
        "  INFO-Felder:  Farben−1 {:02X}, Flags {:02X}, Name '{}'  → {}",
        info.farben_minus_1,
        info.flags,
        kopf_name,
        ziel_txt.display()
    );
    if garn.is_empty() {
        println!("  Farbtabelle:  keine (Design ohne Farbangaben)");
    } else {
        println!(
            "  Farbtabelle:  {} ({} Farben, {} Byte){}",
            ziel_farben.display(),
            garn.len(),
            tabelle.len(),
            if farben_im_anhang {
                " – Farben zusätzlich im Anhang"
            } else {
                ""
            }
        );
    }
    println!("  Icon:         {} ({} Byte, 72 × 62)", ziel_icon.display(), icon.len());
    Ok(true)
}

fn ausfuehren(args: &[String]) -> Result<bool, String> {
    let hilfe = "Befehle: erzeugen DESIGN.json [-o ORDNER] [--alphabete ORDNER] | anzeigen DATEI.exp [-o DATEI.svg] [--rahmen NAME] | alphabete [ORDNER] | rahmen | stoffe | puffer EINGABE [-o PUFFER.bin] [--name NAME] [--ohne-anhang] [--farben-im-anhang]";
    match args.first().map(String::as_str) {
        Some("erzeugen") => {
            let design = args.get(1).ok_or(hilfe)?;
            let ordner = option(args, "-o").unwrap_or_else(|| ".".into());
            erzeugen(Path::new(design), Path::new(&ordner), &alphabet_ordner(args))
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
