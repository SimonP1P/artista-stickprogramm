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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// ab diesem Knick wird an der Ecke eingestochen – nur für berechnete Linien (Unterlagen; eigene Festlegung).
    /// Formen aus dem Design: Einstich an jedem Eckpunkt wie im Original (`Analyse_Teil3/07` §1).
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SatinEinstellung {
    pub auto_spacing: bool,
    /// AMT 0x2723 (hoch)
    pub abstand_fest: f64,
    /// Stoff-/Stilfaktor AMT 0x2736 (hoch)
    pub faktor: f64,
    /// Handbuch S. 199 „automatically set at 0.2 mm“ (hoch)
    pub zugausgleich: f64,
    /// Kurzstiche an Innenkurven (Effekt 0x273a, `Analyse_Teil3/10` §2)
    #[serde(default)]
    pub kurzstiche: KurzstichEinstellung,
    /// Zickzack als Deckstich (Stichart „Zigzag“ 0x2724, BERESENU STR 2236): Abstand 0x2725 = Weg zwischen zwei
    /// Einstichen derselben Seite – Software Defaults 360 = 2,0 mm, alle 8 Stoffe 180 = 1,0 mm (Stil-Liste 0x27a6,
    /// `Analyse/daten/entschluesselt/styles.txt`, hoch); 0x2726 = 3,0 mm (Bedeutung offen)
    #[serde(default = "zickzack_abstand")]
    pub zickzack_abstand: f64,
    /// Effekte des Objekts (Verlauf, ausgefranster Rand; `crate::effekte`) – kein Stilwert
    #[serde(skip)]
    #[schemars(skip)]
    pub effekte: crate::effekte::Effekte,
}

fn zickzack_abstand() -> f64 {
    2.0
}

impl Default for SatinEinstellung {
    fn default() -> Self {
        SatinEinstellung {
            auto_spacing: true,
            abstand_fest: 0.40,
            faktor: 1.0,
            zugausgleich: 0.20,
            kurzstiche: KurzstichEinstellung::default(),
            zickzack_abstand: zickzack_abstand(),
            effekte: Default::default(),
        }
    }
}

/// Kurzstiche (MOD1 `ShortStitchesEffect`, Werte aus dem Stil 0x273a; `Analyse_Teil3/10_regeln_original.md` §2).
/// Im Stil „Software Defaults“ und bei den Stoffen **aus** (Flag 0x2001), im Stil „Schrift“ **an** (Analyse_Teil3/05).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KurzstichEinstellung {
    pub an: bool,
    /// 0x2819 = 0,5: Stelle gilt als zu dicht, wenn zwei Nachbarstiche auf einer Seite näher als Schwelle × Abstand
    /// zusammenkommen (MOD1 0x100a858a: Effekt +0x0C × Abstand)
    pub schwelle: f64,
    /// 0x281a = 1: Faktor zufällig um −0,04 / 0 / +0,04 verändert (MOD1 0x10186a60, Tabelle 0x10240660)
    pub zufall: bool,
    /// 0x281b = 5: höchstens so viele Stiche je Gruppe (MOD1 0x10186ad0)
    pub max_gruppe: usize,
    /// 0x281c: Kürzungsfaktoren als Dreieckstabelle – Gruppe aus n Stichen: n Faktoren (n = 1 … 5)
    pub faktoren: Vec<f64>,
}

impl Default for KurzstichEinstellung {
    fn default() -> Self {
        KurzstichEinstellung {
            an: false,
            schwelle: 0.5,
            zufall: true,
            max_gruppe: 5,
            faktoren: vec![
                0.80, 0.85, 0.72, 0.70, 0.90, 0.70, 0.70, 0.90, 0.80, 0.70, 0.70, 0.87, 0.65, 0.83, 0.70,
            ],
        }
    }
}

impl KurzstichEinstellung {
    /// Faktor für Stich `k` einer Gruppe aus `n` Stichen (MOD1 0x10186a60: Index (n−2)(n−1)/2 + (n−1) + k = n(n−1)/2 + k).
    pub fn faktor(&self, n: usize, k: usize) -> f64 {
        let n = n.clamp(1, self.max_gruppe.max(1));
        let i = (n - 1) * n / 2 + k.min(n - 1);
        self.faktoren.get(i).copied().unwrap_or(1.0)
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TatamiEinstellung {
    /// Abstand AMT 0x2716 (hoch) = Weg zwischen zwei **Hinreihen** (Handbuch PDF-S. 74: „distance between two forward
    /// rows“); benachbarte Reihen liegen beim halben Abstand (an den Musterdesigns gemessen 0,49 × bzw. 0,98 × Abstand,
    /// `Analyse_Teil3/06_messungen_tatami.md`)
    pub abstand: f64,
    /// Stichlänge AMT 0x2717 (hoch)
    pub laenge: f64,
    /// Mindeststich am Rand AMT 0x2718 (hoch)
    pub min_rand: f64,
    /// Reihenversatz AMT 0x2719 (hoch): Raster je Reihe um diesen Anteil der Stichlänge verschoben – gemessen
    /// bei 0,25/0,25: Nachbarreihe 0,25, übernächste Reihe 0,50 (`Analyse_Teil3/06`)
    pub versatz: f64,
    /// Zweiter Reihenversatz AMT 0x271a: die Reihen verschieben sich **abwechselnd** um `versatz` und `versatz2`
    /// (gemessen an den Step-Mustern 1–11 der Musterdesigns, `Analyse_Teil3/06` §5). None = wie `versatz`.
    #[serde(default)]
    pub versatz2: Option<f64>,
    /// Zufallsversatz AMT 0x271b: jeder innere Einstich um bis zu ± zufall/2 × Stichlänge verschoben (Musterdesigns
    /// „Step stitch pattern 11–20“: Stiche 0,6 … 1,4 × Länge bei 0,5); Verteilung **eigene Festlegung**. Alle 20
    /// Step-Muster des Dialogs haben 0.
    #[serde(default)]
    pub zufall: f64,
    /// Stichwinkel (eigene Festlegung)
    pub winkel: f64,
    /// wie Satin (Handbuch S. 199)
    pub zugausgleich: f64,
    /// Laufweg zwischen Teilbereichen der Fläche: Lauf 0x271c unter jeder Füllart (1,8 mm, variabel, Mindeststich
    /// 0,40 mm, Sehnenabstand 0,089 mm; hoch, `Analyse_Teil3/01` §4). Das Original verbindet Teilbereiche fast immer
    /// mit Laufstichen (92 % der Tatami-Blöcke ohne langen Stich), zu 55 % am Rand, 45 % durchs Innere
    /// (`Analyse_Teil3/08_laufwege_flaechen.md`); hier am Rand entlang (Auswahl-Regel: eigene Festlegung).
    #[serde(default = "laufweg_laenge")]
    pub laufweg_laenge: f64,
    #[serde(default = "laufweg_grenzen")]
    pub laufweg: LaufGrenzen,
    /// Effekte des Objekts (Verlauf, ausgefranster Rand, Welle; `crate::effekte`) – kein Stilwert
    #[serde(skip)]
    #[schemars(skip)]
    pub effekte: crate::effekte::Effekte,
}

fn laufweg_laenge() -> f64 {
    1.8
}

fn laufweg_grenzen() -> LaufGrenzen {
    LaufGrenzen {
        min_laenge: w(72.0),
        sehnenabstand: w(16.0),
    }
}

/// Step-Muster 1–20 des Dialogs „Fill Stitch“ (BERESENU DLG 1190, Knöpfe „1“ … „20“): Versatz 0x2719 / 0x271a.
/// Quelle: Tabellen in DIALOG.DLL (0x1017fdb0, 0x1017fe78; dritte Tabelle 0x1017ff40 = 0x271b, überall 0), Code
/// 0x1006f15b sucht die Werte des Objekts darin und markiert die Musternummer (hoch). Die Stichlänge gehört nicht zum
/// Muster (eigenes Feld). Muster 1–10 an den Musterdesigns bestätigt (`Analyse_Teil3/06` §5).
pub const STEP_MUSTER: [(f64, f64); 20] = [
    (0.25, 0.25),
    (0.33, 0.33),
    (0.5, 0.5),
    (0.1, 0.1),
    (0.2, 0.4),
    (0.1, 0.4),
    (0.0, 0.25),
    (0.0, 0.5),
    (0.3, 0.6),
    (0.2, 0.6),
    (0.5, 0.25),
    (0.5, 0.15),
    (0.9, 0.15),
    (0.6, 0.6),
    (0.9, 0.9),
    (1.0, 0.25),
    (0.15, 0.15),
    (0.0, 0.0),
    (0.5, 0.0),
    (0.3, 0.3),
];

impl TatamiEinstellung {
    /// Step-Muster 1–20 setzen (Versatz; Zufall 0).
    pub fn step_muster(&mut self, nr: usize) -> Result<(), String> {
        let (v1, v2) = *STEP_MUSTER
            .get(nr.wrapping_sub(1))
            .ok_or_else(|| format!("Step-Muster {nr} gibt es nicht (1 … 20)"))?;
        self.versatz = v1;
        self.versatz2 = (v2 != v1).then_some(v2);
        self.zufall = 0.0;
        Ok(())
    }

    /// Lage des Einstichrasters der Reihe `k` in Teilen der Stichlänge: abwechselnd um `versatz` und `versatz2`.
    pub fn phase(&self, k: i64) -> f64 {
        match self.versatz2 {
            None => (k as f64 * self.versatz) % 1.0,
            Some(v2) => ((k + 1).div_euclid(2) as f64 * self.versatz + k.div_euclid(2) as f64 * v2) % 1.0,
        }
    }

    /// Abstand benachbarter Reihen (Hin- und Rückreihe): halber Abstand der Hinreihen.
    pub fn reihenabstand(&self) -> f64 {
        self.abstand / 2.0
    }
}

impl Default for TatamiEinstellung {
    fn default() -> Self {
        TatamiEinstellung {
            abstand: 0.40,
            laenge: 4.0,
            min_rand: 0.40,
            versatz: 0.25,
            versatz2: None,
            zufall: 0.0,
            winkel: 45.0,
            zugausgleich: 0.20,
            laufweg_laenge: laufweg_laenge(),
            laufweg: laufweg_grenzen(),
            effekte: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// AMT 0x2744/0x2724/0x2725 (mittel); Weg zwischen Einstichen derselben Seite (an den Musterdesigns gemessen, mittel-hoch, `Analyse_Teil3/07` §2)
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
    /// **Auto Underlay** (Werkzeug „Auto Underlay“, Handbuch PDF-S. 197–198): In der Unterlagenliste 0x27a7 der Stile
    /// tragen alle Einträge Bit 0x2000 (= aus) in „Software Defaults“ und den 8 Stoffen, bei Schrift keiner (= an);
    /// MOD1 `DesignDocument::SetUnderlayOn` setzt die aktuelle Unterlage auf die vorgewählte. Sicherheit mittel-hoch
    /// (Stil-Flags + Handbuch + Code), `Analyse_Teil3/05_effekte_und_unterlagen.md`.
    #[serde(default)]
    pub auto_an: bool,
    /// Vorgewählte Art der Auto-Unterlage: Eintrag mit Bit 0x0001 = Zickzack (alle Standard-, Stoff-, Schrift- und
    /// EasyStitch-Stile).
    #[serde(default = "zickzack")]
    pub auto_art: Vec<String>,
    /// Läufe der Unterlagen sind im Original „variabel“ (0x271e = 1) mit eigenem Mindeststich (0x2767) und
    /// Sehnenabstand (0x2768) je Unterlagenart (Stil „0 Software Defaults“, in allen Stilen gleich, hoch):
    pub edge_walk_lauf: LaufGrenzen,
    pub mittellauf_lauf: LaufGrenzen,
    pub zickzack_lauf: LaufGrenzen,
    pub doppelzickzack_lauf: LaufGrenzen,
    pub tatami_randlauf_lauf: LaufGrenzen,
}

fn zickzack() -> Vec<String> {
    vec!["zickzack".into()]
}

/// Mindeststich und Sehnenabstand eines variablen Laufs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
            auto_an: false,
            auto_art: zickzack(),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
    /// Verbindertyp zum nächsten Objekt („Connector After Object“ 0x2803, Tag 0x2805; in allen 22 Stilen 0 = Jump):
    /// `sprung` wie das Original, `lauf` = „Run“ (gerader Laufstich, kein Vernähen davor – im Original sind die
    /// Tie-Off-Felder bei „Run“ gesperrt, DIALOG.DLL 0x10036ea0), `verdeckt` = eigene Erweiterung: Lauf nur, wo später
    /// gestickte Flächen ihn ganz bedecken, sonst Sprung. Bei Farbwechsel immer Sprung.
    #[serde(default)]
    pub art: VerbinderArt,
    /// Stichlänge des Lauf-Verbinders zum nächsten Objekt (0x2803 → 0x2711 = 12,7 mm, Feld neben „Run“)
    #[serde(default = "lauf_laenge_nach")]
    pub lauf_laenge: f64,
    /// Verbindertyp innerhalb des Objekts – zwischen Teilen eines Objekts, z. B. Buchstaben eines Textes („Connector
    /// Inside Object“ 0x2804, Tag 0x2805 = 0 = Jump); Werte wie `art`
    #[serde(default)]
    pub innen_art: VerbinderArt,
    /// Stichlänge des Lauf-Verbinders innerhalb des Objekts (0x2804 → 0x2711 = 1,8 mm, wie die Laufwege in Flächen)
    #[serde(default = "lauf_laenge_innen")]
    pub innen_lauf_laenge: f64,
}

auswahl! {
    /// Verbindertyp (Dialog „Connectors“, Knöpfe „Jump“/„Run“ = Tag 0x2805; `verdeckt` = eigene Erweiterung)
    #[derive(Default)]
    pub enum VerbinderArt {
        #[default]
        Sprung = "sprung",
        Lauf = "lauf",
        Verdeckt = "verdeckt",
    }
}

fn lauf_laenge_nach() -> f64 {
    12.7
}

fn lauf_laenge_innen() -> f64 {
    1.8
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
            art: VerbinderArt::Sprung,
            lauf_laenge: lauf_laenge_nach(),
            innen_art: VerbinderArt::Sprung,
            innen_lauf_laenge: lauf_laenge_innen(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
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
        // Deck-Step der Stoffe: Versatz 0x2719/0x271a = 0,25 / 0,75 (Stil-Liste 0x27a6, alle 8 Stoffe; Software
        // Defaults 0,25 / 0,25) – Reihen abwechselnd +¼ und −¼ (`Analyse_Teil3/06` §5)
        e.tatami.versatz2 = Some(0.75);
        // Zickzack-Deckstich 0x2725 = 1,0 mm (Defaults 2,0 mm)
        e.satin.zickzack_abstand = 1.0;
    }
    Ok(e)
}
