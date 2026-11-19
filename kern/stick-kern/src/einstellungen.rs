//! Standardwerte der Sticharten – nach dem Vorbild BERNINA artista V4.
//!
//! Quellen (Sicherheit in Klammern): `Analyse/ANALYSE.md` §5 und
//! `Analyse/code/10_algorithmen_schmale_bahnen.md` (entschlüsselte NORMAL.AMT, Stil „0 Software Defaults“,
//! DEFAULTS.DFL, Handbuch). Werte mit „eigene Festlegung“ stammen nicht aus dem Original und werden nach
//! `ARCHITECTURE.md` §1a durch das Original-Verhalten ersetzt.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Satin „Auto Spacing“: Säulenbreite (mm) -> Stichabstand (mm), Stil „0 Software Defaults“
/// (AMT 0x2734/0x2735, hoch). Zwischen den Stützstellen linear interpoliert (mittel-hoch).
pub const AUTO_SPACING_TABELLE: [(f64, f64); 11] = [
    (0.01, 0.54),
    (0.5, 0.51),
    (1.0, 0.49),
    (1.5, 0.47),
    (2.0, 0.46),
    (3.0, 0.42),
    (4.0, 0.40),
    (5.0, 0.37),
    (6.0, 0.36),
    (7.0, 0.34),
    (8.0, 0.34),
];

/// Schrift wird 10 % dichter gestickt (PRESET_LETTERING, Faktor 0,9, hoch).
pub const SCHRIFT_FAKTOR: f64 = 0.9;

/// Satin-Stichabstand für eine Säulenbreite (Tabelle, linear interpoliert, außen konstant).
pub fn auto_spacing(breite_mm: f64, faktor: f64) -> f64 {
    let t = &AUTO_SPACING_TABELLE;
    let abstand = if breite_mm <= t[0].0 {
        t[0].1
    } else if breite_mm >= t[t.len() - 1].0 {
        t[t.len() - 1].1
    } else {
        let mut a = t[t.len() - 1].1;
        for w in t.windows(2) {
            let ((b0, a0), (b1, a1)) = (w[0], w[1]);
            if b0 <= breite_mm && breite_mm <= b1 {
                a = a0 + (a1 - a0) * (breite_mm - b0) / (b1 - b0);
                break;
            }
        }
        a
    };
    abstand * faktor
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaufEinstellung {
    /// Kontur-Lauf 2,5 mm (AMT 0x27a8, hoch)
    pub laenge: f64,
    /// min. 0,8 mm (AMT 0x27a8, hoch)
    pub min_laenge: f64,
    /// „feste Länge“ ist Standard (hoch)
    pub variabel: bool,
    /// Chord Gap bei variabler Länge (Lauf in Flächen, hoch)
    pub sehnenabstand: f64,
    /// ab diesem Knick wird an der Ecke eingestochen (eigene Festlegung)
    pub ecke_grad: f64,
}

impl Default for LaufEinstellung {
    fn default() -> Self {
        LaufEinstellung {
            laenge: 2.5,
            min_laenge: 0.8,
            variabel: false,
            sehnenabstand: 0.09,
            ecke_grad: 30.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SatinEinstellung {
    pub auto_spacing: bool,
    /// AMT 0x2723 (hoch)
    pub abstand_fest: f64,
    /// Stoff-/Stilfaktor AMT 0x2736 (hoch)
    pub faktor: f64,
    /// Handbuch S. 199 „automatically set at 0.2 mm“ (hoch)
    pub zugausgleich: f64,
}

impl Default for SatinEinstellung {
    fn default() -> Self {
        SatinEinstellung {
            auto_spacing: true,
            abstand_fest: 0.40,
            faktor: 1.0,
            zugausgleich: 0.20,
        }
    }
}

impl SatinEinstellung {
    pub fn abstand(&self, breite_mm: f64) -> f64 {
        if self.auto_spacing {
            auto_spacing(breite_mm, self.faktor)
        } else {
            self.abstand_fest
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TatamiEinstellung {
    /// Reihenabstand AMT 0x2716 (hoch)
    pub abstand: f64,
    /// Stichlänge AMT 0x2717 (hoch)
    pub laenge: f64,
    /// Mindeststich am Rand AMT 0x2718 (hoch)
    pub min_rand: f64,
    /// Reihenversatz AMT 0x2719 (Wert hoch; Wirkung als Anteil der Stichlänge je Reihe: mittel)
    pub versatz: f64,
    /// Stichwinkel (eigene Festlegung)
    pub winkel: f64,
    /// wie Satin (Handbuch S. 199)
    pub zugausgleich: f64,
}

impl Default for TatamiEinstellung {
    fn default() -> Self {
        TatamiEinstellung {
            abstand: 0.40,
            laenge: 4.0,
            min_rand: 0.40,
            versatz: 0.25,
            winkel: 45.0,
            zugausgleich: 0.20,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnterlageEinstellung {
    /// Unterlagenrand AMT 0x2769/0x276a (mittel)
    pub rand: f64,
    /// AMT 0x2743 (hoch)
    pub zugausgleich: f64,
    /// AMT 0x2747 = „Edge Walk“ (hoch: DIALOG.DLL bindet 0x2747 an Text 2599 „Edge Walk“ in BERESENU.DLL,
    /// Läufe liegen in den Musterdesigns am Rand; Analyse_Teil3/04)
    pub edge_walk_laenge: f64,
    /// AMT 0x2746 = „Center Run“ (hoch: Text 2598, Läufe liegen auf der Säulenmitte; Analyse_Teil3/04)
    pub mittellauf_laenge: f64,
    /// AMT 0x2744/0x2724/0x2725 (mittel); hier gedeutet als Weg entlang der Säule je Zick+Zack (niedrig)
    pub zickzack_laenge: f64,
    /// AMT 0x2745/0x2724/0x2725 (mittel)
    pub doppelzickzack_laenge: f64,
    /// „+ Lauf 3,0 mm“ zu Zickzack-Unterlagen (AMT 0x2744/0x271c/0x271d, mittel)
    pub rueck_lauf_laenge: f64,
    /// Tatami-Unterlage AMT 0x275a (mittel)
    pub tatami_abstand: f64,
    pub tatami_laenge: f64,
    pub tatami_randlauf: f64,
    /// Winkel 90° zur Deckstichrichtung (AMT 0x2759)
    pub tatami_winkel_versatz: f64,
    /// Läufe der Unterlagen sind im Original „variabel“ (0x271e = 1) mit eigenem Mindeststich (0x2767) und
    /// Sehnenabstand (0x2768) je Unterlagenart (Stil „0 Software Defaults“, in allen Stilen gleich, hoch):
    pub edge_walk_lauf: LaufGrenzen,
    pub mittellauf_lauf: LaufGrenzen,
    pub zickzack_lauf: LaufGrenzen,
    pub doppelzickzack_lauf: LaufGrenzen,
    pub tatami_randlauf_lauf: LaufGrenzen,
}

/// Mindeststich und Sehnenabstand eines variablen Laufs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaufGrenzen {
    pub min_laenge: f64,
    pub sehnenabstand: f64,
}

/// Wilcom-Einheiten (1 mm = 180) in mm.
const fn w(einheiten: f64) -> f64 {
    einheiten / 180.0
}

impl Default for UnterlageEinstellung {
    fn default() -> Self {
        UnterlageEinstellung {
            rand: 0.20,
            zugausgleich: 0.20,
            edge_walk_laenge: 2.2,
            mittellauf_laenge: 2.2,
            zickzack_laenge: 5.0,
            doppelzickzack_laenge: 7.0,
            rueck_lauf_laenge: 3.0,
            tatami_abstand: 3.0,
            tatami_laenge: 4.0,
            tatami_randlauf: 2.5,
            tatami_winkel_versatz: 90.0,
            edge_walk_lauf: LaufGrenzen {
                min_laenge: w(72.0),
                sehnenabstand: w(13.0),
            },
            mittellauf_lauf: LaufGrenzen {
                min_laenge: w(131.0),
                sehnenabstand: w(11.0),
            },
            zickzack_lauf: LaufGrenzen {
                min_laenge: w(72.0),
                sehnenabstand: w(36.0),
            },
            doppelzickzack_lauf: LaufGrenzen {
                min_laenge: w(179.0),
                sehnenabstand: w(16.0),
            },
            tatami_randlauf_lauf: LaufGrenzen {
                min_laenge: w(72.0),
                sehnenabstand: w(13.0),
            },
        }
    }
}

/// Verbinder (Connectors) – Stil-Container AMT 0x272a, in allen 22 Stilen gleich.
/// Zuordnung Tag ↔ Dialogfeld „Connectors“ aus DIALOG.DLL 0x10035e89–0x10036032 (hoch), Analyse_Teil3/01.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerbinderEinstellung {
    /// Tie In: „Always Tie In“ (0x2753 = 0)
    pub anfang_immer: bool,
    /// Tie In If: „After Trim / Color Change“ (0x2750 = 1)
    pub anfang_nach_trim_farbwechsel: bool,
    /// Tie In If: „Previous Connector Longer Than“ (0x272d = 1) …
    pub anfang_bei_langem_verbinder: bool,
    /// … Länge (0x2755 = 3,0 mm)
    pub anfang_verbinder_ab: f64,
    /// Tie Off: „Always Tie Off“ (0x2752 = 0)
    pub ende_immer: bool,
    /// Tie Off If: „Before Trim / Color Change“ (0x2733 = 1)
    pub ende_vor_trim_farbwechsel: bool,
    /// Tie Off If: „Next Connector Longer Than“ (0x272c = 1) …
    pub ende_bei_langem_verbinder: bool,
    /// … Länge (0x2754 = 3,0 mm)
    pub ende_verbinder_ab: f64,
    /// „Always Tie Off Last Object“ (0x276c = 0): letztes Objekt wird im Original nicht vernäht
    pub letztes_objekt_vernaehen: bool,
    /// Trim, wenn der nächste Verbinder länger ist als (0x272b = 1, 0x272e = 3,0 mm; Stoffe 6,0 mm).
    /// Die artista 185 schneidet nicht (DFL trims_for_jump = 0); der Wert zählt nur für „nach Trim vernähen“.
    pub trim_ab: f64,
    /// Länge der Vernähstiche (0x2730/0x2731 = 1,0 mm; Bedeutung aus Wert und Lage, mittel)
    pub vernaeh_stichlaenge: f64,
    /// Anzahl der Vernähstiche (0x272f/0x275d = 2; Bedeutung aus Wert und Lage, mittel)
    pub vernaeh_stiche: usize,
}

impl Default for VerbinderEinstellung {
    fn default() -> Self {
        VerbinderEinstellung {
            anfang_immer: false,
            anfang_nach_trim_farbwechsel: true,
            anfang_bei_langem_verbinder: true,
            anfang_verbinder_ab: 3.0,
            ende_immer: false,
            ende_vor_trim_farbwechsel: true,
            ende_bei_langem_verbinder: true,
            ende_verbinder_ab: 3.0,
            letztes_objekt_vernaehen: false,
            trim_ab: 3.0,
            vernaeh_stichlaenge: 1.0,
            vernaeh_stiche: 2,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Einstellungen {
    pub lauf: LaufEinstellung,
    pub satin: SatinEinstellung,
    pub tatami: TatamiEinstellung,
    pub unterlage: UnterlageEinstellung,
    pub verbinder: VerbinderEinstellung,
    pub stoff: String,
}

impl Einstellungen {
    /// Einzelne Werte überschreiben, z. B. `{"satin": {"zugausgleich": 0.3}}`. Unbekannte Namen sind ein Fehler.
    pub fn ueberschreiben(&self, werte: &Value) -> Result<Einstellungen, String> {
        fn mischen(ziel: &mut Value, quelle: &Value, pfad: &str) -> Result<(), String> {
            let (Value::Object(z), Value::Object(q)) = (&mut *ziel, quelle) else {
                return Err(format!("Einstellung '{pfad}' muss ein Objekt sein"));
            };
            for (k, v) in q {
                let unterpfad = if pfad.is_empty() {
                    k.clone()
                } else {
                    format!("{pfad}.{k}")
                };
                match z.get_mut(k) {
                    None => return Err(format!("Unbekannte Einstellung '{unterpfad}'")),
                    Some(alt @ Value::Object(_)) if v.is_object() => mischen(alt, v, &unterpfad)?,
                    Some(alt) => *alt = v.clone(),
                }
            }
            Ok(())
        }
        let mut basis = serde_json::to_value(self).map_err(|e| e.to_string())?;
        mischen(&mut basis, werte, "")?;
        serde_json::from_value(basis).map_err(|e| format!("Einstellungen ungültig: {e}"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stoff {
    pub name: &'static str,
    pub satin_abstand_fest: f64,
    pub tatami_abstand: f64,
    pub auto_spacing_faktor: f64,
    pub zugausgleich: f64,
    pub verbinder_trim: f64,
}

const fn stoff(name: &'static str, satin: f64, tatami: f64, faktor: f64, zug: f64, trim: f64) -> Stoff {
    Stoff {
        name,
        satin_abstand_fest: satin,
        tatami_abstand: tatami,
        auto_spacing_faktor: faktor,
        zugausgleich: zug,
        verbinder_trim: trim,
    }
}

/// Stoffe (Settings → Fabric Settings), `Analyse/code/10` §E (hoch).
/// Zugausgleich „0 Software Defaults“: Tabelle 0,17 (mittel), Handbuch 0,20 (hoch) -> 0,20.
pub const STOFFE: [Stoff; 9] = [
    stoff("0 Software Defaults", 0.40, 0.40, 1.00, 0.20, 3.0),
    stoff("Knit Light Weight", 0.43, 0.45, 1.08, 0.20, 6.0),
    stoff("Knit Medium Weight", 0.44, 0.45, 1.10, 0.25, 6.0),
    stoff("Knit Heavy Weight", 0.38, 0.40, 1.00, 0.25, 6.0),
    stoff("Terry Medium Weight", 0.42, 0.42, 1.05, 0.25, 6.0),
    stoff("Terry Heavy Weight", 0.38, 0.38, 0.95, 0.25, 6.0),
    stoff("Woven Light Weight", 0.42, 0.45, 1.05, 0.20, 6.0),
    stoff("Woven Medium Weight", 0.43, 0.40, 1.08, 0.25, 6.0),
    stoff("Woven Heavy Weight", 0.38, 0.38, 1.13, 0.25, 6.0),
];

/// Einstellungen für einen der 8 Stoffe (oder die Software Defaults).
pub fn fuer_stoff(name: &str) -> Result<Einstellungen, String> {
    let Some(s) = STOFFE.iter().find(|s| s.name == name) else {
        let bekannt: Vec<_> = STOFFE.iter().map(|s| s.name).collect();
        return Err(format!("Unbekannter Stoff '{name}'. Bekannt: {}", bekannt.join(", ")));
    };
    let mut e = Einstellungen {
        stoff: name.to_string(),
        ..Default::default()
    };
    e.satin.abstand_fest = s.satin_abstand_fest;
    e.satin.faktor = s.auto_spacing_faktor;
    e.satin.zugausgleich = s.zugausgleich;
    e.tatami.abstand = s.tatami_abstand;
    e.tatami.zugausgleich = s.zugausgleich;
    e.verbinder.trim_ab = s.verbinder_trim;
    if name != "0 Software Defaults" {
        // Bei allen 8 Stoffen gleich, anders als in den Defaults (Analyse/code/10 §E, hoch)
        e.unterlage.zickzack_laenge = 3.0;
        e.unterlage.doppelzickzack_laenge = 3.0;
    }
    Ok(e)
}
