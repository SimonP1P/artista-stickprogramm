//! **Stichart nach Breite wie EasyStitch** des Originals (automatisches Digitalisieren, `Analyse/code/10` §C).
//!
//! Klassifikationstabelle 0x3f54 in jedem EasyStitch-Stil der Vorlage NORMAL.AMT (Sicherheit hoch):
//! Linie ≤ 0,3 mm → `<PRESET_RUN_1>`, Linie ≤ 2,0 mm → `<PRESET_RUN_2>`, Fläche ≤ 8,0 mm → `<PRESET_SATIN_1>`,
//! ≤ 12,0 mm → `<PRESET_TATAMI_1>`, darüber → `<PRESET_TATAMI_2>`.
//!
//! Die Vorlagen entsprechen den fünf Stilen `<PRESET_EASYSTITCH>…` (21.11.2026, Flags der Sticharten-Gruppen in
//! `Analyse/daten/entschluesselt/styles.txt`, vorgewählt = 0x0001, Gruppe aus = 0x2000):
//! - „Thin Lines“ und „Thick Lines“: Füllung aus, **Kontur Laufstich** vorgewählt, Länge **2,0 mm** (0x271d; Software
//!   Defaults 2,5). Beide Stile haben dieselben Werte – worin sich die kräftige Linie im Original unterscheidet, ist
//!   nicht gefunden; hier beide als Laufstich.
//! - „Narrow Objects“: Füllung **Satin** vorgewählt.
//! - „Wide Objects“ und „Other Objects“: Füllung **Tatami** vorgewählt, Abstand **0,45 mm** (0x2716; Defaults 0,40).
//! - Zugausgleich 0x273d: Narrow/Wide/Other **72 → 0,40 mm**, Thin/Thick Lines 30 → 0,17 mm wie die Software Defaults
//!   (bestätigt `Analyse/code/10` §B; Handbuch: „Use 0.4 when using automatic digitizing“).
//!
//! Sonst gleichen die Stile den Software Defaults (Vergleich aller Werte, 21.11.2026).
//! Zuordnung Vorlage ↔ Stil über die Stichart (Satin ↔ Narrow, Tatami ↔ Wide/Other, Lauf ↔ Lines): mittel.
//!
//! Die Breite einer Fläche ist ihre Dicke = Durchmesser des größten Innenkreises (`saeulen::dicke`; eigene
//! Festlegung, wie EasyStitch misst, ist nicht analysiert); Linien haben ihre Strichbreite. Schmale Flächen (≤ 2 mm) werden zu Laufstichen auf der Mittellinie (EasyStitch macht
//! Objekte unter 2 mm zu Linien).

use serde_json::{Value, json};
use stick_kern::geometrie::{Linie, Punkt, flaeche_vorzeichen, p};

/// Linie bis zu dieser Breite: einfacher Laufstich (0x3f77 = 54 → 0,3 mm)
pub const LINIE_DUENN_MM: f64 = 0.3;
/// Linie bis zu dieser Breite: „kräftige Linie“ (360 → 2,0 mm)
pub const LINIE_KRAEFTIG_MM: f64 = 2.0;
/// Fläche bis zu dieser Breite: Satin (1440 → 8,0 mm)
pub const SATIN_BIS_MM: f64 = 8.0;
/// Fläche bis zu dieser Breite: Tatami 1, darüber Tatami 2 (2160 → 12,0 mm)
pub const TATAMI1_BIS_MM: f64 = 12.0;
/// Kleinere Flächen entfallen: 0x3f57 = 32 400 = 1 mm² in derselben Tabelle. **Annahme** (Rolle im Original nicht
/// bestimmt): Mindestfläche eines Objekts.
pub const MIN_FLAECHE_MM2: f64 = 1.0;
/// Stichlänge des Laufstichs der Stile Thin/Thick Lines (0x271d = 2,00 mm)
pub const LAUF_LAENGE_MM: f64 = 2.0;
/// Tatami-Abstand der Stile Wide/Other Objects (0x2716 = 0,45 mm)
pub const TATAMI_ABSTAND_MM: f64 = 0.45;
/// Zugausgleich der Stile Narrow/Wide/Other Objects (0x273d = 72 → 0,40 mm)
pub const ZUGAUSGLEICH_MM: f64 = 0.40;

/// Ergebnis der Einteilung nach Breite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Klasse {
    /// ≤ 0,3 mm: Laufstich (Thin Lines)
    DuenneLinie,
    /// ≤ 2 mm: Laufstich (Thick Lines)
    KraeftigeLinie,
    /// ≤ 8 mm: Satin (Narrow Objects)
    Satin,
    /// ≤ 12 mm: Tatami (Wide Objects)
    Tatami1,
    /// darüber: Tatami (Other Objects)
    Tatami2,
}

impl Klasse {
    pub fn nach_breite(breite_mm: f64) -> Klasse {
        // Grenzen einschließlich (≤), Rundung beim Umrechnen aus Pixeln abfangen
        let breite_mm = breite_mm - 1e-6;
        if breite_mm <= LINIE_DUENN_MM {
            Klasse::DuenneLinie
        } else if breite_mm <= LINIE_KRAEFTIG_MM {
            Klasse::KraeftigeLinie
        } else if breite_mm <= SATIN_BIS_MM {
            Klasse::Satin
        } else if breite_mm <= TATAMI1_BIS_MM {
            Klasse::Tatami1
        } else {
            Klasse::Tatami2
        }
    }

    /// Name wie die Stile des Originals.
    pub fn name(self) -> &'static str {
        match self {
            Klasse::DuenneLinie => "Thin Lines",
            Klasse::KraeftigeLinie => "Thick Lines",
            Klasse::Satin => "Narrow Objects",
            Klasse::Tatami1 => "Wide Objects",
            Klasse::Tatami2 => "Other Objects",
        }
    }
}

/// Punkte für das Design-JSON (auf 0,01 mm gerundet).
pub fn punkte_json(l: &[Punkt]) -> Value {
    l.iter()
        .map(|q| json!([(q.x * 100.0).round() / 100.0, (q.y * 100.0).round() / 100.0]))
        .collect()
}

fn mit_name(mut o: Value, name: &str) -> Value {
    if !name.is_empty() {
        o["name"] = Value::from(name);
    }
    o
}

/// Objekte für eine Fläche (Umriss mit Löchern). Leere Liste, wenn die Fläche kleiner als 1 mm² ist.
pub fn flaeche_objekte(
    umriss: &Linie,
    loecher: &[Linie],
    farbe: &str,
    name: &str,
) -> Result<(Vec<Value>, Option<Klasse>), String> {
    let flaeche = flaeche_vorzeichen(umriss).abs() - loecher.iter().map(|l| flaeche_vorzeichen(l).abs()).sum::<f64>();
    if flaeche < MIN_FLAECHE_MM2 {
        return Ok((Vec::new(), None));
    }
    let breite = stick_kern::saeulen::dicke(umriss, loecher)?;
    let klasse = Klasse::nach_breite(breite);
    let objekte = match klasse {
        Klasse::DuenneLinie | Klasse::KraeftigeLinie => {
            // Laufstich auf der Mittellinie jeder Säule
            let s = stick_kern::saeulen::zerlegen(umriss, loecher, &Default::default())?;
            s.iter()
                .map(|x| {
                    x.seite_a
                        .iter()
                        .zip(&x.seite_b)
                        .map(|(a, b)| p((a.x + b.x) / 2.0, (a.y + b.y) / 2.0))
                        .collect::<Linie>()
                })
                .filter(|l| l.len() >= 2)
                .map(|l| {
                    mit_name(
                        json!({"art": "lauf", "farbe": farbe, "punkte": punkte_json(&l), "laenge": LAUF_LAENGE_MM}),
                        name,
                    )
                })
                .collect()
        }
        Klasse::Satin => vec![mit_name(
            json!({"art": "satinflaeche", "farbe": farbe, "umriss": punkte_json(umriss),
                   "einstellungen": {"satin": {"zugausgleich": ZUGAUSGLEICH_MM}},
                   "loecher": loecher.iter().map(|l| punkte_json(l)).collect::<Vec<_>>()}),
            name,
        )],
        Klasse::Tatami1 | Klasse::Tatami2 => vec![mit_name(
            json!({"art": "tatami", "farbe": farbe, "umriss": punkte_json(umriss),
                   "loecher": loecher.iter().map(|l| punkte_json(l)).collect::<Vec<_>>(),
                   "einstellungen": {"tatami": {"abstand": TATAMI_ABSTAND_MM, "zugausgleich": ZUGAUSGLEICH_MM}}}),
            name,
        )],
    };
    Ok((objekte, Some(klasse)))
}

/// Objekte für eine Linie mit Strichbreite: Laufstich bis 2 mm, darüber Satinlinie dieser Breite (Satin wie
/// „Narrow Objects“). Breiter als 8 mm bleibt Satinlinie (Hinweis an den Aufrufer).
pub fn linie_objekte(punkte: &Linie, geschlossen: bool, breite_mm: f64, farbe: &str, name: &str) -> (Value, Klasse) {
    let klasse = Klasse::nach_breite(breite_mm);
    let o = match klasse {
        Klasse::DuenneLinie | Klasse::KraeftigeLinie => json!({
            "art": "lauf", "farbe": farbe, "punkte": punkte_json(punkte), "geschlossen": geschlossen,
            "laenge": LAUF_LAENGE_MM
        }),
        _ => {
            let mut l = punkte.clone();
            if geschlossen && l.len() > 1 && l.first() != l.last() {
                l.push(l[0]);
            }
            json!({"art": "satinlinie", "farbe": farbe, "punkte": punkte_json(&l),
                   "breite": (breite_mm * 100.0).round() / 100.0,
                   "einstellungen": {"satin": {"zugausgleich": ZUGAUSGLEICH_MM}}})
        }
    };
    (mit_name(o, name), klasse)
}
