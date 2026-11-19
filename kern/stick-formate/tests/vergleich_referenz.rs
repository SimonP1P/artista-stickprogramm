//! Regressionsprüfung: Der Rust-Kern muss für `testdaten/vergleich_*.json` byte-genau die Referenz-EXP in
//! `testdaten/referenz/` erzeugen. Ausgangspunkt war der Python-Prototyp (byte-gleich, Commit 50be48d); jede
//! bewusste Änderung (Original-Verhalten statt eigener Festlegung, ARCHITECTURE.md §1a) ersetzt die Referenzen
//! und steht in `testdaten/referenz/README.md`.

use std::path::PathBuf;
use stick_formate::exp::{exp_bytes, exp_lesen};
use stick_kern::{Art, Design};

fn testdaten() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("testdaten")
}

fn vergleichen(name: &str) {
    let json = std::fs::read_to_string(testdaten().join(format!("{name}.json"))).unwrap();
    let design = Design::aus_json(&json).unwrap();
    let folge = design.berechnen().unwrap();
    let rust = exp_bytes(&folge);
    let referenz = std::fs::read(testdaten().join("referenz").join(format!("{name}.exp"))).unwrap();
    if rust != referenz {
        let erste = rust
            .iter()
            .zip(&referenz)
            .position(|(a, b)| a != b)
            .unwrap_or(rust.len().min(referenz.len()));
        let (r, p) = (exp_lesen(&rust), exp_lesen(&referenz));
        panic!(
            "{name}: EXP weicht ab (Rust {} Byte, Referenz {} Byte, erste Abweichung bei Byte {erste}); \
             Stiche Rust {} / Referenz {}",
            rust.len(),
            referenz.len(),
            r.anzahl(Art::Stich),
            p.anzahl(Art::Stich)
        );
    }
    assert!(
        design.ausserhalb(&folge).is_empty(),
        "{name}: Einstiche außerhalb des Stickfelds"
    );
}

#[test]
fn beispiel_wie_referenz() {
    vergleichen("vergleich_beispiel");
}

#[test]
fn unterlagen_und_formen_wie_referenz() {
    vergleichen("vergleich_unterlagen");
}

#[test]
fn stoff_und_einstellungen_wie_referenz() {
    vergleichen("vergleich_stoff");
}
