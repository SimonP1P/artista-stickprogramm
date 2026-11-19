//! Kommandozeile zum Rechenkern:
//!
//!   stick erzeugen DESIGN.json [-o ORDNER] [--alphabete ORDNER]  Stiche berechnen -> .exp + .svg
//!   stick alphabete [ORDNER]                               Original-Alphabete (ESB) auflisten
//!   stick anzeigen DATEI.exp [-o DATEI.svg] [--rahmen NAME] EXP als SVG darstellen
//!   stick rahmen | stoffe                                  Rahmen bzw. Stoffe auflisten

use std::path::{Path, PathBuf};
use std::process::ExitCode;
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

fn ausfuehren(args: &[String]) -> Result<bool, String> {
    let hilfe = "Befehle: erzeugen DESIGN.json [-o ORDNER] [--alphabete ORDNER] | anzeigen DATEI.exp [-o DATEI.svg] [--rahmen NAME] | alphabete [ORDNER] | rahmen | stoffe";
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
