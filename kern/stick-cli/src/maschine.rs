//! `stick maschine …`: die artista 185 über den seriellen Port (oder die Simulation) ansprechen – wie die
//! Testprogramme v3.6/v4.1, aber aus dem Rechenkern. Jeder Lauf verlässt den PC-Modus wie das Original
//! (0x100 + `T r M E`) und schreibt einen Mitschnitt.

use crate::{option, sendedaten_laden};
use std::io::Write;
use std::path::{Path, PathBuf};
use stick_maschine::maschine::{Abbruch, FarbBereich};
use stick_maschine::{Leitung, Maschine, SendeOptionen, SimOptionen, Simulation, Speicher, zeitstempel};

pub const HILFE: &str = "maschine ports | maschine pruefen | maschine liste [--karte] | maschine lesen NR [-o ORDNER] | \
maschine senden EINGABE [--schreiben [--sticken]] [--ohne-icon] [--ohne-farben] [--ohne-info] [--ohne-nachfrage]   \
(jeweils --port COM3 oder --simulation [--sim-baud 57600]; Mitschnitt nach --log ORDNER)";

fn leitung(args: &[String]) -> Result<(Box<dyn Leitung>, String), String> {
    if args.iter().any(|a| a == "--simulation") {
        let baud = option(args, "--sim-baud").and_then(|b| b.parse().ok()).unwrap_or(19200);
        return Ok((
            Box::new(Simulation::neu(SimOptionen {
                baud,
                ..Default::default()
            })),
            "SIMULATION".into(),
        ));
    }
    #[cfg(feature = "port")]
    {
        let name = option(args, "--port").unwrap_or_else(|| "COM3".into());
        let p = stick_maschine::port::SeriellerPort::oeffnen(&name, 19200, args.iter().any(|a| a == "--dtr-rts"))?;
        Ok((Box::new(p), name))
    }
    #[cfg(not(feature = "port"))]
    Err("ohne Merkmal „port“ gebaut – nur --simulation".into())
}

fn mitschnitt(args: &[String], art: &str, zeilen: &[String]) {
    let ordner = option(args, "--log").map_or_else(|| PathBuf::from("."), PathBuf::from);
    let pfad = ordner.join(format!("Mitschnitt_stick_{art}_{}.log", zeitstempel()));
    match std::fs::write(&pfad, zeilen.join("\r\n") + "\r\n") {
        Ok(()) => println!("  Mitschnitt: {}", pfad.display()),
        Err(e) => eprintln!("  Mitschnitt nicht geschrieben ({}): {e}", pfad.display()),
    }
}

fn fortschritt() -> impl FnMut(&str, usize, usize) {
    |was: &str, n: usize, g: usize| {
        print!("\r  {was}: {n} von {g} Byte      ");
        if n >= g {
            println!();
        }
        let _ = std::io::stdout().flush();
    }
}

fn farbbereich_zeigen(fb: &FarbBereich) {
    println!(
        "  Garnfarbbereich: höchste Farbanzahl {}, Größe {} Byte, Bereich T 0x{:06X}, Tabelle bei T + {}",
        fb.grenzen.max_farben,
        fb.grenzen.groesse,
        fb.t,
        fb.grenzen.vorspann()
    );
    println!("  jetzt in der Maschine: {}", fb.text);
    for (i, f) in fb.farben.iter().enumerate() {
        println!(
            "     {:2}: RGB {:02X}{:02X}{:02X}  Garn-Nr. {:<5} {} {}",
            i + 1,
            f.rgb[0],
            f.rgb[1],
            f.rgb[2],
            f.nummer,
            f.marke,
            f.name
        );
    }
}

fn frage(text: &str) -> bool {
    println!();
    println!("{text}");
    print!("Fortfahren? (J/N) ");
    let _ = std::io::stdout().flush();
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).is_ok() && matches!(a.trim().chars().next(), Some('j' | 'J' | 'y' | 'Y'))
}

pub fn ausfuehren(args: &[String]) -> Result<bool, String> {
    let befehl = args.get(1).map(String::as_str);
    if befehl == Some("ports") {
        #[cfg(feature = "port")]
        for p in stick_maschine::port::SeriellerPort::namen() {
            println!("  {p}");
        }
        return Ok(true);
    }
    // senden: Daten vor dem Verbinden laden und prüfen
    let sendung = if befehl == Some("senden") {
        let eingabe = PathBuf::from(args.get(2).ok_or(HILFE)?);
        Some(sendedaten_laden(Path::new(&eingabe), args)?)
    } else {
        None
    };
    if !matches!(befehl, Some("pruefen" | "liste" | "lesen" | "senden")) {
        return Err(HILFE.into());
    }
    let (l, port) = leitung(args)?;
    println!("Verbinde über {port} …");
    let mut m = match Maschine::verbinden(l, 19200) {
        Ok(m) => m,
        Err(Abbruch { fehler, protokoll }) => {
            mitschnitt(args, befehl.unwrap_or(""), &protokoll);
            return Err(fehler);
        }
    };
    println!(
        "  verbunden: Nähmaschine {}, Stickmodul {}, {} Baud",
        m.firmware_naehmaschine, m.firmware_stickmodul, m.v.baud
    );
    let vw = &m.verwaltung;
    println!(
        "  Verwaltungsblock: N {}, INFO 0x{:X}, AUSWAHL 0x{:X}, ICON 0x{:X}, DATEN 0x{:X}, Grenze {} Byte",
        vw.n,
        vw.info,
        vw.auswahl,
        vw.icon,
        vw.daten,
        vw.grenze()
    );
    let ergebnis = ablauf(&mut m, befehl.unwrap_or(""), args, sendung);
    let (ok, v) = m.verlassen();
    println!(
        "  PC-Modus verlassen (0x100 + T r M E): {}",
        if ok {
            "bestätigt"
        } else {
            "NICHT bestätigt – Display ansehen, ggf. Maschine aus/ein"
        }
    );
    mitschnitt(args, befehl.unwrap_or(""), &v.protokoll);
    ergebnis
}

fn ablauf<L: Leitung>(
    m: &mut Maschine<L>,
    befehl: &str,
    args: &[String],
    sendung: Option<stick_formate::artista::Sendedaten>,
) -> Result<bool, String> {
    let mut f = fortschritt();
    match befehl {
        "pruefen" => {
            farbbereich_zeigen(&m.farbbereich_lesen()?);
            Ok(true)
        }
        "liste" => {
            let ziel = if args.iter().any(|a| a == "--karte") {
                Speicher::Karte
            } else {
                Speicher::Maschine
            };
            for e in m.designliste(ziel)? {
                println!(
                    "   Nr {:3}  Flags 0x{:02X}  Feld {:3}  {}",
                    e.nr, e.flags, e.feld_farben, e.name
                );
            }
            Ok(true)
        }
        "lesen" => {
            let nr: usize = args
                .get(2)
                .and_then(|n| n.parse().ok())
                .ok_or("Nummer fehlt: stick maschine lesen NR")?;
            let d = m.design_lesen(nr, &mut f)?;
            let ordner = option(args, "-o").map_or_else(|| PathBuf::from("."), PathBuf::from);
            let z = zeitstempel();
            let pd = ordner.join(format!("Design_Nr{nr:02}_{z}.bin"));
            let pi = ordner.join(format!("Icon_Nr{nr:02}_{z}.bin"));
            std::fs::write(&pd, &d.puffer).map_err(|e| format!("{}: {e}", pd.display()))?;
            std::fs::write(&pi, &d.icon).map_err(|e| format!("{}: {e}", pi.display()))?;
            println!(
                "  Design Nr. {nr} „{}“: {} Byte → {}",
                d.name,
                d.puffer.len(),
                pd.display()
            );
            println!("  Icon: {} Byte → {}", d.icon.len(), pi.display());
            farbbereich_zeigen(&m.farbbereich_lesen()?);
            Ok(true)
        }
        "senden" => {
            let s = sendung.ok_or("keine Sendedaten")?;
            let o = SendeOptionen {
                icon: !args.iter().any(|a| a == "--ohne-icon"),
                info: !args.iter().any(|a| a == "--ohne-info"),
                farben: !args.iter().any(|a| a == "--ohne-farben"),
                sticken: args.iter().any(|a| a == "--sticken"),
            };
            let plan = m.planen(&s, &o)?;
            if let Some(fb) = &plan.farbbereich {
                farbbereich_zeigen(fb);
            }
            println!("  Geplante Schreibzugriffe:");
            for z in &plan.schritte {
                println!("    {z}");
            }
            for h in &plan.hinweise {
                println!("  Hinweis: {h}");
            }
            if !args.iter().any(|a| a == "--schreiben") {
                println!("  TROCKENLAUF: nichts geschrieben (Schalter --schreiben fehlt).");
                return Ok(true);
            }
            let fragen = !args.iter().any(|a| a == "--ohne-nachfrage");
            if fragen
                && !frage(
                    "JETZT WIRD IN DIE MASCHINE GESCHRIEBEN (siehe oben). Gespeicherte Designs bleiben unverändert.",
                )
            {
                println!("  Abgebrochen – nichts geschrieben.");
                return Ok(false);
            }
            if o.sticken
                && fragen
                && !frage(
                    "DANACH BEFEHL 0x08 (STICKEN): Ist jemand an der Maschine? Rahmen eingesetzt, Nadel oben, Hände weg?",
                )
            {
                println!("  Abgebrochen – nichts geschrieben.");
                return Ok(false);
            }
            let b = m.senden(&s, &o, &mut f)?;
            for z in &b.schritte {
                println!("  {z}");
            }
            Ok(b.sticken.is_none_or(|r| r.ok))
        }
        _ => Err(HILFE.into()),
    }
}
