//! Prüfung der Übertragung: Der Rust-Kern muss für dieselben Designs byte-genau dieselben EXP-Dateien
//! erzeugen wie der Python-Prototyp (`stickprogramm/`). Referenzen in `testdaten/python/` wurden mit
//! `stickprogramm erzeugen testdaten/vergleich_*.json -o testdaten/python` erzeugt.
//!
//! Sobald der Kern bewusst vom Prototyp abweicht (Original-Verhalten statt eigener Festlegung,
//! ARCHITECTURE.md §1a), werden die betroffenen Referenzen ersetzt und die Änderung dokumentiert.

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
    let python = std::fs::read(testdaten().join("python").join(format!("{name}.exp"))).unwrap();
    if rust != python {
        let erste = rust
            .iter()
            .zip(&python)
            .position(|(a, b)| a != b)
            .unwrap_or(rust.len().min(python.len()));
        let (r, p) = (exp_lesen(&rust), exp_lesen(&python));
        panic!(
            "{name}: EXP weicht ab (Rust {} Byte, Python {} Byte, erste Abweichung bei Byte {erste}); \
             Stiche Rust {} / Python {}",
            rust.len(),
            python.len(),
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
fn beispiel_gleich_wie_python() {
    vergleichen("vergleich_beispiel");
}

#[test]
fn unterlagen_und_formen_gleich_wie_python() {
    vergleichen("vergleich_unterlagen");
}

#[test]
fn stoff_und_einstellungen_gleich_wie_python() {
    vergleichen("vergleich_stoff");
}
