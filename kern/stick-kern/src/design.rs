//! Design = Rahmen + Stoff + Stickobjekte. `berechnen()` liefert die fertige Stichfolge.
//!
//! Eingabe im selben JSON-Format wie der Python-Prototyp (`stickprogramm/README.md`): Koordinaten in mm,
//! Ursprung = Rahmenmitte, +y nach oben. Statt Punktlisten gehen Formen (kreis, rechteck, bogen, bezier, verbinden).

use crate::applikation::{ApplikationVorgabe, applikation_objekte};
use crate::aussparen::{Aussparen, Teil, bedeckt, flaeche_minus};
use crate::einstellungen::{Einstellungen, SatinEinstellung, TatamiEinstellung, fuer_stoff};
use crate::geometrie::{Linie, Punkt, abstand, abstand_punkt_strecke, im_polygon, p, zwischen};
use crate::knopfloch::{KnopflochVorgabe, Vorlage, knopfloch_objekt};
use crate::nachbearbeitung::{bernina_regeln, wege_verbinden_je_teil};
use crate::rahmen::{Rahmen, rahmen};
use crate::schrift::{
    Alphabet, Buchstabe, SchriftStichart, TextVorgabe, TextZusatz, buchstaben_zahl, text_objekte_idx, text_setzen_nr,
};
use crate::schrift_ttf::{TtfSchrift, TtfStichart, ttf_text_objekte_idx};
use crate::sticharten::blanket::blanket;
use crate::sticharten::lauf::{dreifach, laufstich_mit_ecken};
use crate::sticharten::satin::{mittellinie, paare_bilden, satin_einstiche, satinlinie_seiten};
use crate::sticharten::tatami::{tatami, tatami_mit_teilung};
use crate::sticharten::unterlagen::{auto_satin_unterlage, flaechen_unterlage, satin_unterlage};
use crate::stiche::StichFolge;
use crate::transformation::{Transformation, abbilden, mitte as formmitte};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

// ---------------------------------------------------------------- JSON-Eingabe (Format: `format.rs`)

use crate::format::{
    Ausrichtung, DesignJson, Ende, Griffe, GrundlinieJson, HuelleArt, HuelleJson, LaufJson, MonogrammJson,
    MonogrammRahmen, ObjektJson, SatinStichart, SchriftOptionen, Start, TextJson, UnterlageJson, Versatz,
};
pub use crate::format::{Form, MONOGRAMM_HOEHE_MM, PunktAngabe};

impl GrundlinieJson {
    fn grundlinie(self, blocksatz: bool) -> crate::schrift::Grundlinie {
        crate::schrift::Grundlinie {
            art: self.art.text().into(),
            radius: self.radius.unwrap_or(crate::schrift::KREIS_RADIUS_MM),
            radius_y: self.radius_y,
            winkel: self.winkel,
            neigung: self.neigung,
            bogen: self.bogen,
            laenge: self.laenge,
            lage: self.lage.map(|x| x.text().into()),
            stand: self.stand.map(|x| x.text().into()),
            punkte: self.punkte.map(|q| q.punkte()).unwrap_or_default(),
            blocksatz,
        }
    }
}

/// Elastic Lettering: Griffe der Kanten (`format::HuelleJson`).
fn huelle_lesen(h: &HuelleJson) -> Result<crate::schrift::Huelle, String> {
    let gebogen = h.art == HuelleArt::Gebogen;
    let griffe = |w: &Option<Griffe>| -> Result<[f64; 3], String> {
        let l: Vec<f64> = match w {
            None => Vec::new(),
            Some(Griffe::Mitte(n)) => vec![*n],
            Some(Griffe::Liste(l)) => l.clone(),
        };
        match (gebogen, l.as_slice()) {
            (_, []) => Ok([0.0; 3]),
            (false, [a, b]) => Ok([*a, (a + b) / 2.0, *b]),
            (true, [m]) => Ok([0.0, *m, 0.0]),
            (true, [a, m, b]) => Ok([*a, *m, *b]),
            _ => Err(if gebogen {
                "gebogen: oben/unten = Mitte oder [links, Mitte, rechts]".into()
            } else {
                "gerade: oben/unten = [links, rechts]".into()
            }),
        }
    };
    Ok(crate::schrift::Huelle {
        gebogen,
        oben: griffe(&h.oben)?,
        unten: griffe(&h.unten)?,
    })
}

/// Monogramm (Handbuch Kap. 21 „Creating monograms“): Initialen mit den Monogramm-Alphabeten **Diamond2** (2 Buchstaben)
/// bzw. **Diamond3** (3). Belegung der Zeichen aus den Alphabeten gelesen (`Analyse_Teil3/09` §7): `A`–`Z` = linker
/// Buchstabe, `a`–`z` = rechter, `!` … `:` = mittlerer (nur Diamond3, 18 statt 16 mm hoch), `]` = Raute als Rahmen,
/// `[` = Zierwinkel – beide reichen von ihrem Platz nach links und liegen mittig um die Buchstaben, wenn sie direkt
/// nach ihnen kommen. Das Objekt wird hier in ein Text-Objekt umgeschrieben (alle Schriftoptionen gelten).
fn monogramm_als_text(m: MonogrammJson) -> Result<TextJson, String> {
    let ini: Vec<char> = m.initialen.to_uppercase().chars().collect();
    if !(2..=3).contains(&ini.len()) || !ini.iter().all(char::is_ascii_uppercase) {
        return Err("initialen: 2 oder 3 Buchstaben A–Z (Diamond2/Diamond3)".into());
    }
    let mut text = String::new();
    text.push(ini[0]);
    if ini.len() == 3 {
        text.push((b'!' + (ini[1] as u8 - b'A')) as char);
    }
    text.push(ini[ini.len() - 1].to_ascii_lowercase());
    let rahmen = match m.rahmen {
        None | Some(MonogrammRahmen::Keine) => None,
        Some(MonogrammRahmen::Raute) => Some(']'),
        Some(MonogrammRahmen::Zier) => Some('['),
    };
    let mut schrift = m.schrift;
    if let Some(f) = m.rahmen_farbe {
        if rahmen.is_none() {
            return Err("rahmen_farbe ohne rahmen".into());
        }
        text.push('^');
        schrift.farben.push(f);
    }
    if let Some(z) = rahmen {
        text.push(z);
    }
    let alphabet = if ini.len() == 3 { "Diamond3" } else { "Diamond2" };
    Ok(TextJson {
        text,
        alphabet: alphabet.into(),
        hoehe: m.hoehe,
        unterlage: m.unterlage,
        stichart: m.stichart,
        winkel: m.winkel,
        schrift,
        g: m.g,
    })
}

fn ausrichtung_anteil(a: Ausrichtung) -> f64 {
    match a {
        Ausrichtung::Links | Ausrichtung::Block => 0.0,
        Ausrichtung::Rechts => 1.0,
        Ausrichtung::Mitte => 0.5,
    }
}

/// Satz eines Texts aus den gemeinsamen Schriftoptionen.
fn text_vorgabe(
    text: String,
    hoehe: f64,
    farbe: &str,
    s: &SchriftOptionen,
    unterlage: Option<Vec<String>>,
) -> TextVorgabe {
    TextVorgabe {
        text,
        hoehe,
        breite_prozent: s.breite,
        abstand: s.abstand,
        position: s.position.unwrap_or(p(0.0, 0.0)),
        ausrichtung: ausrichtung_anteil(s.ausrichtung),
        farbe: farbe.to_string(),
        wortabstand: s.wortabstand.map(|w| w / 100.0),
        zeilenabstand: s.zeilenabstand.map(|z| z / 100.0),
        grundlinie: s
            .grundlinie
            .clone()
            .map(|g| g.grundlinie(s.ausrichtung == Ausrichtung::Block)),
        kursiv: s.kursiv,
        unterlage,
    }
}

/// Transformation prüfen (Matrix muss sich bilden lassen, z. B. Skalierung ≠ 0).
fn transform_pruefen(t: &Transformation, wo: &str) -> Result<(), String> {
    t.matrix(p(0.0, 0.0)).map(|_| ()).map_err(|e| format!("{wo}: {e}"))
}

/// Farben für `^`, einzelne Buchstaben, Stichart und Verzerrung eines Texts.
fn schrift_zusatz(
    wo: &str,
    text: &str,
    s: &SchriftOptionen,
    stichart: Option<SchriftStichart>,
) -> Result<Option<TextZusatz>, String> {
    let wechsel = text.matches('^').count();
    if wechsel > s.farben.len() {
        return Err(format!(
            "{wo}: {wechsel} Farbwechsel (^) im Text, aber {} Farben in „farben“",
            s.farben.len()
        ));
    }
    let n = buchstaben_zahl(text);
    let mut einzeln = Vec::new();
    for b in &s.buchstaben {
        if b.nr == 0 || b.nr > n {
            return Err(format!("{wo}: Buchstabe {} gibt es nicht (1 … {n})", b.nr));
        }
        let transform = b.transform.clone().unwrap_or_default();
        transform_pruefen(&transform, &format!("{wo}, Buchstabe {}", b.nr))?;
        einzeln.push(Buchstabe {
            nr: b.nr,
            farbe: b.farbe.clone(),
            transform,
            winkellinien: b.winkellinien.clone(),
        });
    }
    let huelle = match &s.elastisch {
        Some(h) => Some(huelle_lesen(h).map_err(|e| format!("{wo}: elastisch: {e}"))?),
        None => None,
    };
    if s.farben.is_empty() && einzeln.is_empty() && stichart.is_none() && huelle.is_none() {
        return Ok(None);
    }
    Ok(Some(TextZusatz {
        farben: s.farben.clone(),
        einzeln,
        stichart,
        huelle,
    }))
}

/// Stichart „muster“ einer Schrift: Muster muss angegeben und bekannt sein.
fn schrift_muster(wo: &str, s: &SchriftOptionen, winkel: Option<f64>) -> Result<SchriftStichart, String> {
    let m = s
        .muster
        .as_deref()
        .ok_or_else(|| format!("{wo}: Stichart muster braucht „muster“ (Name oder Nummer, stick muster)"))?;
    if crate::muster::finden(m).is_none() {
        return Err(format!("{wo}: Füllmuster „{m}“ unbekannt (stick muster)"));
    }
    Ok(SchriftStichart::Muster {
        muster: m.to_string(),
        winkel,
        groesse: s.groesse.unwrap_or(100.0),
    })
}

/// Fehler beim Lesen eines Designs: wenn möglich mit Nummer des Objekts, Meldung auf Deutsch.
fn design_fehler(v: &Value, e: serde_json::Error) -> String {
    if let Some(liste) = v.get("objekte").and_then(Value::as_array) {
        for (i, o) in liste.iter().enumerate() {
            if let Err(f) = ObjektJson::deserialize(o) {
                let art = o.get("art").and_then(Value::as_str).unwrap_or("?");
                return format!(
                    "Objekt {} ({art}): {}",
                    i + 1,
                    crate::format::fehler_deutsch(&f.to_string())
                );
            }
        }
    }
    format!("Design ungültig: {}", crate::format::fehler_deutsch(&e.to_string()))
}

/// Start- bzw. Endpunkt des Designs (Dialog „Start and End“, Menü Arrange; Handbuch PDF-S. 204–205). Das Original
/// setzt bei „Auto Start/End“ die **Mitte des Designs** (umschließendes Rechteck): 142 von 143 Musterdesigns mit
/// AutoStartEndObject (Klasse 0x71) beginnen und enden dort (`Analyse_Teil3/10` §6). Standard hier bleibt die
/// Rahmenmitte (Ursprung), bis am Gerät geprüft ist, wie die artista 185 ein Design mit anderem Start platziert.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Bezugspunkt {
    /// Ursprung = Rahmenmitte
    #[default]
    Rahmenmitte,
    /// Mitte des umschließenden Rechtecks der Stiche („Auto Start/End at: center of design“)
    Designmitte,
    /// Mitte der Unterkante („bottom middle“)
    UntenMitte,
    /// kein Verbinder: erster bzw. letzter Stich („First Stitch“/„Last Stitch“)
    Stich,
    /// nur Ende: zurück zum Startpunkt („return to start“)
    Start,
}

impl From<Start> for Bezugspunkt {
    fn from(s: Start) -> Self {
        match s {
            Start::Rahmenmitte => Bezugspunkt::Rahmenmitte,
            Start::Designmitte => Bezugspunkt::Designmitte,
            Start::UntenMitte => Bezugspunkt::UntenMitte,
            Start::ErsterStich => Bezugspunkt::Stich,
        }
    }
}

impl From<Ende> for Bezugspunkt {
    fn from(e: Ende) -> Self {
        match e {
            Ende::Rahmenmitte => Bezugspunkt::Rahmenmitte,
            Ende::Designmitte => Bezugspunkt::Designmitte,
            Ende::UntenMitte => Bezugspunkt::UntenMitte,
            Ende::LetzterStich => Bezugspunkt::Stich,
            Ende::Start => Bezugspunkt::Start,
        }
    }
}

// ---------------------------------------------------------------- Modell

#[derive(Clone, Debug, PartialEq)]
pub enum Unterlage {
    /// wie „Auto Underlay“ des Originals: aus oder die im Stil vorgewählte Art (`UnterlageEinstellung::auto_an`)
    Auto,
    /// Liste aus mittellauf, edge_walk, zickzack, doppelzickzack (leer = keine)
    Arten(Vec<String>),
}

impl From<UnterlageJson> for Unterlage {
    fn from(u: UnterlageJson) -> Self {
        match u {
            UnterlageJson::Text(t) if t == "auto" => Unterlage::Auto,
            UnterlageJson::Text(t) if t == "keine" => Unterlage::Arten(vec![]),
            UnterlageJson::Text(t) => Unterlage::Arten(t.split('+').map(|s| s.trim().to_string()).collect()),
            UnterlageJson::Liste(l) => Unterlage::Arten(l),
            UnterlageJson::Schalter(true) => Unterlage::Auto,
            UnterlageJson::Schalter(false) => Unterlage::Arten(vec![]),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjektArt {
    /// Laufstich oder Dreifachstich entlang einer Linie
    Lauf {
        punkte: Linie,
        /// je Punkt: Eckpunkt (Einstich wie im Original) oder Kurvenpunkt
        ecken: Vec<bool>,
        laenge: Option<f64>,
        dreifach: bool,
        geschlossen: bool,
    },
    /// Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B)
    Satin {
        seite_a: Linie,
        seite_b: Linie,
        unterlage: Unterlage,
        optionen: SatinOptionen,
    },
    /// Blanket (Langettenstich, 0x2727) entlang einer Linie
    Blanket {
        punkte: Linie,
        geschlossen: bool,
        abstand: f64,
        breite: f64,
        /// „Center“ (Standard) oder „Offset“ (Rücken auf der Linie, Zähne nach links)
        mittig: bool,
    },
    /// Satinbahn konstanter Breite entlang einer Mittellinie
    SatinLinie {
        punkte: Linie,
        breite: f64,
        unterlage: Unterlage,
        /// „Center“ (Standard) oder „Offset“ (Linie = Kante, Säule links davon)
        mittig: bool,
        /// Abweichungen vom Stil (z. B. Zickzack)
        optionen: SatinOptionen,
    },
    /// Tatami-Füllung einer Fläche mit optionalen Löchern
    Tatami {
        umriss: Linie,
        loecher: Vec<Linie>,
        winkel: Option<f64>,
        /// None = Auto Underlay des Stils; mit Unterlage: Tatami-Unterlage „Step“ (Art für Flächen: eigene
        /// Festlegung, die vorgewählte Art des Stils ist Zickzack)
        unterlage: Option<bool>,
        /// Flächen anderer Objekte aussparen (Knockout, `aussparen.rs`); wird vor dem Berechnen aufgelöst
        aussparen: Aussparen,
    },
    /// Text mit einem Original-Alphabet; wird vor dem Berechnen in Satin-/Lauf-Objekte aufgelöst
    Text { alphabet: String, vorgabe: TextVorgabe },
    /// Fläche mit mehreren Stichwinkeln (Turning, `sticharten::wendung`): Step-Stiche längs der Winkellinien
    Wendefuellung {
        umriss: Linie,
        winkellinien: Vec<[Punkt; 2]>,
        unterlage: Option<bool>,
        /// Kurzstiche an Innenkurven (None = Stil); 770 solcher Step-Flächen in den Musterdesigns mit Kurzstichen
        kurzstiche: Option<bool>,
    },
    /// Fläche mit Füllmuster (Program Split): Tatami-Reihen, Einstiche auf den gekachelten Musterlinien
    Musterfuellung {
        umriss: Linie,
        loecher: Vec<Linie>,
        muster: String,
        winkel: Option<f64>,
        groesse: f64,
        unterlage: Option<bool>,
        aussparen: Aussparen,
    },
    /// Applikation (Platzierung, Heften, Abdecken mit Stopps); wird vor dem Berechnen aufgelöst
    Applikation(ApplikationVorgabe),
    /// Text mit einer Computer-Schrift (TrueType/OpenType); wird vor dem Berechnen aufgelöst
    TtfText {
        schrift: String,
        vorgabe: TextVorgabe,
        stichart: TtfStichart,
    },
    /// Fläche als Satinsäulen mit automatischer Stichrichtung (`saeulen.rs`); wird vor dem Berechnen in eine Gruppe
    /// von Säulen aufgelöst
    SatinFlaeche {
        umriss: Linie,
        loecher: Vec<Linie>,
        unterlage: Unterlage,
        optionen: SatinOptionen,
        /// Innenwinkel, unter dem Säulen am Knick geteilt werden (0 = nie)
        knickwinkel: f64,
    },
    /// Zusammengehörige Teile (ein Buchstabe): Reihenfolge nach Nähe, verbunden mit Laufweg statt Sprung, wenn der
    /// Weg unter noch zu stickenden Teilen liegt (`Analyse_Teil3/10` §4)
    Gruppe {
        teile: Vec<Objekt>,
        /// Umrisse der Teile (gleiche Reihenfolge) – dort darf der Laufweg liegen
        umrisse: Vec<Linie>,
        /// Stichlänge des Laufwegs in mm (Alphabet: Lauf 0x271c in der Füllung, 3,0 mm)
        laufweg: f64,
    },
    /// Knopfloch (Vorlage aus ButtonHoles.ESB, `knopfloch.rs`); wird vor dem Berechnen zu einer `Kette` aufgelöst
    Knopfloch(KnopflochVorgabe),
    /// Motivlinie (Pattern Run, `motiv.rs`); wird vor dem Berechnen zu einer `Kette` aufgelöst
    Motivlinie(crate::motiv::MotivVorgabe),
    /// Teile in fester Reihenfolge, ohne Sprünge zu einem Zug verbunden (Knopfloch). Satinteile beginnen an dem Ende,
    /// das dem vorigen Stich am nächsten liegt (am Musterdesign beobachtet). `loch` = Fläche zum Aussparen
    /// (Merge Buttonhole: mittlerer Umriss).
    Kette {
        teile: Vec<Objekt>,
        /// je Teil: darf rückwärts gestickt werden (Raupen)
        umkehrbar: Vec<bool>,
        loch: Option<Linie>,
    },
}

/// Einstellungen einer Satinsäule, die vom Stil abweichen (wie die Effekte am Objekt im Original).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SatinOptionen {
    /// fester Abstand in mm (None = Einstellung des Stoffs/Stils, Auto Spacing)
    pub abstand: Option<f64>,
    /// Auto-Spacing-Faktor (None = Stil)
    pub faktor: Option<f64>,
    /// Kurzstiche (None = Stil)
    pub kurzstiche: Option<bool>,
    /// Step statt Satin in der Säule (Schrift mit Stichart Step, `sticharten::satin::step_reihen`)
    pub step: bool,
    /// Zickzack statt Satin (Stichart „Zigzag“ 0x2724): fester Abstand `zickzack_abstand`, ohne Kurzstiche
    pub zickzack: bool,
    /// Zugausgleich je Seite in mm (None = Stil); Knopfloch-Vorlagen: 0x8c = 0,17 mm
    pub zugausgleich: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Objekt {
    pub farbe: String,
    pub name: String,
    pub art: ObjektArt,
}

fn satin_wege(
    a: &[Punkt],
    b: &[Punkt],
    unterlage: &Unterlage,
    o: &SatinOptionen,
    e: &Einstellungen,
) -> Result<Vec<Linie>, String> {
    let (unter, deck) = satin_getrennt(a, b, unterlage, o, e)?;
    if unter.is_empty() {
        Ok(vec![deck])
    } else {
        // in einem Zug: Unterlage endet am Säulenanfang
        Ok(vec![unter.into_iter().chain(deck).collect()])
    }
}

/// Aufgelöste Objekte, fehlende Zeichen, Herkunft (Nummer des Objekts im Design) je Objekt
/// (aufgelöste Objekte, fehlende Zeichen je Objektnummer, Herkunft je aufgelöstem Objekt)
/// Aufgelöste Objekte, fehlende Zeichen (Objekt, Zeichen), je Objekt die Nummer im Design und die Nummer des
/// Buchstabens (0 = kein Buchstabe).
pub(crate) type Aufgeloest = (Vec<Objekt>, Vec<(usize, char)>, Vec<usize>, Vec<usize>);
/// Objekte mit Nummer im Design und Buchstabennummer.
type MitHerkunft = (Vec<Objekt>, Vec<usize>, Vec<usize>);

/// Double Run: dieselben Stiche rückwärts anhängen (beginnen, wo das Objekt endet – kein Verbindungsstich).
fn doppellauf(wege: &mut Vec<Linie>) {
    let rueck: Vec<Linie> = wege.iter().rev().map(|w| w.iter().rev().copied().collect()).collect();
    for (i, r) in rueck.into_iter().enumerate() {
        if i == 0
            && let Some(letzte) = wege.last_mut()
        {
            letzte.extend(r.into_iter().skip(1));
        } else {
            wege.push(r);
        }
    }
}

/// Satinsäule als (Unterlage, Decklage); die Unterlage endet am Säulenanfang (`Analyse_Teil3/07` §3), leer ohne
/// Unterlage.
fn satin_getrennt(
    a: &[Punkt],
    b: &[Punkt],
    unterlage: &Unterlage,
    o: &SatinOptionen,
    e: &Einstellungen,
) -> Result<(Linie, Linie), String> {
    let (a, b) = paare_bilden(a, b)?;
    let mut satin_e: SatinEinstellung = e.satin.clone();
    if let Some(x) = o.abstand {
        satin_e.auto_spacing = false;
        satin_e.abstand_fest = x;
    }
    if let Some(f) = o.faktor {
        satin_e.faktor = f;
    }
    if let Some(k) = o.kurzstiche {
        satin_e.kurzstiche.an = k;
    }
    if let Some(z) = o.zugausgleich {
        satin_e.zugausgleich = z;
    }
    let arten = match unterlage {
        Unterlage::Auto => auto_satin_unterlage(&e.unterlage),
        Unterlage::Arten(l) => l.clone(),
    };
    let unter = if arten.is_empty() {
        Vec::new()
    } else {
        satin_unterlage(&a, &b, &arten, &e.unterlage)?
    };
    if o.zickzack {
        satin_e.auto_spacing = false;
        satin_e.abstand_fest = e.satin.zickzack_abstand;
        satin_e.kurzstiche.an = false;
    }
    let deck = if o.step {
        // Step-Reihen: Abstand und Zugausgleich der Füllung; Kurzstiche wie am Objekt (Schrift: an)
        let mut se = satin_e.clone();
        se.auto_spacing = false;
        se.abstand_fest = e.tatami.abstand;
        se.zugausgleich = e.tatami.zugausgleich;
        crate::sticharten::satin::step_reihen(&satin_einstiche(&a, &b, &se, false)?, &e.tatami)
    } else {
        // Zickzack (0x2724) wie bisher symmetrisch, Satin gerade quer + schräg vor (`satin.rs`)
        satin_einstiche(&a, &b, &satin_e, !o.zickzack)?
    };
    Ok((unter, deck))
}

/// Schneiden sich die Strecken a1–a2 und b1–b2?
fn strecken_schneiden(a1: Punkt, a2: Punkt, b1: Punkt, b2: Punkt) -> bool {
    let kreuz = |o: Punkt, a: Punkt, b: Punkt| (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
    let (d1, d2) = (kreuz(b1, b2, a1), kreuz(b1, b2, a2));
    let (d3, d4) = (kreuz(a1, a2, b1), kreuz(a1, a2, b2));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

/// Berühren sich zwei Umrisse (überlappen, kreuzen sich oder liegen näher als 0,15 mm)?
fn umrisse_beruehren(a: &Linie, b: &Linie) -> bool {
    let ring = |l: &Linie| -> Vec<(Punkt, Punkt)> { (0..l.len()).map(|i| (l[i], l[(i + 1) % l.len()])).collect() };
    let (ka, kb) = (ring(a), ring(b));
    a.iter().any(|&q| im_polygon(q, std::slice::from_ref(b)))
        || b.iter().any(|&q| im_polygon(q, std::slice::from_ref(a)))
        || ka
            .iter()
            .any(|&(a1, a2)| kb.iter().any(|&(b1, b2)| strecken_schneiden(a1, a2, b1, b2)))
        || a.iter()
            .any(|&q| kb.iter().any(|&(b1, b2)| abstand_punkt_strecke(q, b1, b2) < 0.15))
        || b.iter()
            .any(|&q| ka.iter().any(|&(a1, a2)| abstand_punkt_strecke(q, a1, a2) < 0.15))
}

/// Teile einer Gruppe (Buchstabe) sticken. Zwischen zwei Teilen ein **Laufweg** (Stichlänge `laufweg`), wenn er
/// verdeckt ist, d. h. unter Teilen liegt, die danach noch gestickt werden – sonst ein Sprung:
/// 1. gerade zum Anfang des nächsten Teils, oder
/// 2. auf kurzem Weg zur Mittellinie des nächsten Teils und auf ihr zurück zu seinem Anfang (liegt unter ihm).
///
/// Die Reihenfolge der Teile wird so gewählt, dass möglichst wenige Sprünge nötig sind, dann möglichst kurze Wege
/// (bis 7 Teile alle Reihenfolgen, sonst „nächster Anfang“).
/// Grundlage: Maschinen-Zeichensatz Drifter „A“ – erst der Querbalken, dann Laufweg im linken Bein (etwa auf seiner
/// Mitte) unter dem späteren Bogen (`Analyse_Teil3/10` §4). Wegführung und Bewertung sind **eigene Festlegungen**.
///
/// **Unterlagen zuerst** (gemessen an den Schrift-Musterdesigns des Originals, `Analyse_Teil3/11`): Teile, deren
/// Umrisse sich berühren, bilden eine Gruppe; je Gruppe stickt das Original **erst alle Unterlagen, in umgekehrter
/// Reihenfolge der Säulen**, dann die Säulen in ihrer Reihenfolge (1 Unterlage: 85 von 85 Gruppen vor den Säulen;
/// mehrere: 43 von 62 genau so, der Rest Messgrenzen). Getrennte Teile (Punkt des „i“) kommen nacheinander.
fn gruppe_wege(
    teile: &[Objekt],
    umrisse: &[Linie],
    laufweg: f64,
    e: &Einstellungen,
    start: Option<Punkt>,
) -> Result<Vec<Linie>, String> {
    // je Teil die möglichen Richtungen: wie im Alphabet und – bei Satinsäulen – rückwärts. Das Original stickt Säulen
    // in beiden Richtungen und wählt Reihenfolge und Richtung selbst („Closest Join“; gemessen an 4558 Satinstücken
    // der Schrift-Musterdesigns: 62 % vorwärts, 38 % rückwärts, nur 150 von 938 Buchstaben mit ≥ 2 Säulen in
    // Alphabet-Reihenfolge, `Analyse_Teil3/11` §5). Je Richtung: Unterlage, Decklage (Wege), Mittellinie.
    struct Richtung {
        unter: Linie,
        wege: Vec<Linie>,
        mitte: Option<Linie>,
    }
    let mut stuecke: Vec<(Vec<Richtung>, Option<&Linie>)> = Vec::new();
    for (i, t) in teile.iter().enumerate() {
        let mut richt = Vec::new();
        match &t.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => {
                for rueck in [false, true] {
                    let (a, b): (Linie, Linie) = if rueck {
                        (
                            seite_a.iter().rev().copied().collect(),
                            seite_b.iter().rev().copied().collect(),
                        )
                    } else {
                        (seite_a.clone(), seite_b.clone())
                    };
                    let (unter, d) = satin_getrennt(&a, &b, unterlage, optionen, e)?;
                    if d.is_empty() {
                        continue;
                    }
                    let mitte = paare_bilden(&a, &b).ok().map(|(x, y)| mittellinie(&x, &y));
                    richt.push(Richtung {
                        unter,
                        wege: vec![d],
                        mitte,
                    });
                }
            }
            _ => {
                let w: Vec<Linie> = t.wege(e, None)?.into_iter().filter(|w| !w.is_empty()).collect();
                if !w.is_empty() {
                    richt.push(Richtung {
                        unter: Vec::new(),
                        wege: w,
                        mitte: None,
                    });
                }
            }
        }
        if !richt.is_empty() {
            stuecke.push((richt, umrisse.get(i).filter(|u| u.len() >= 3)));
        }
    }
    let n = stuecke.len().min(63);
    if n == 0 {
        return Ok(Vec::new());
    }
    // Knoten = (Teil, Richtung)
    let knoten: Vec<(usize, usize)> = (0..n)
        .flat_map(|j| (0..stuecke[j].0.len()).map(move |d| (j, d)))
        .collect();
    let r = |v: usize| &stuecke[knoten[v].0].0[knoten[v].1];
    let teil = |v: usize| knoten[v].0;
    let m = knoten.len();
    let anf: Vec<Punkt> = (0..m).map(|v| r(v).wege[0][0]).collect();
    let end: Vec<Punkt> = (0..m)
        .map(|v| *r(v).wege.last().and_then(|l| l.last()).unwrap())
        .collect();
    let bedeckt = |q: Punkt| -> u64 {
        stuecke[..n]
            .iter()
            .enumerate()
            .filter(|(_, (_, u))| u.is_some_and(|r| im_polygon(q, std::slice::from_ref(r))))
            .fold(0u64, |mk, (k, _)| mk | (1 << k))
    };
    // Prüfpunkte alle 0,5 mm auf einer Strecke; die Enden (je 0,6 mm) nicht, weil der Zugausgleich Anfang und Ende
    // der Säulen über den Umriss schiebt
    let masken = |a: Punkt, b: Punkt| -> Vec<u64> {
        let d = abstand(a, b);
        let pruef = ((d / 0.5).ceil() as usize).max(1);
        (1..pruef)
            .map(|k| k as f64 / pruef as f64)
            .filter(|t| t * d >= 0.6 && (1.0 - t) * d >= 0.6)
            .map(|t| bedeckt(zwischen(a, b, t)))
            .collect()
    };
    // je Verbindung v → w: gerade (Masken, Länge) und über die Mittellinie von w (Masken des Zugangs, Weg)
    struct Verbindung {
        gerade: Vec<u64>,
        gerade_laenge: f64,
        ueber_mitte: Option<(Vec<u64>, Linie)>,
    }
    let mut verb: Vec<Vec<Option<Verbindung>>> = (0..m).map(|_| (0..m).map(|_| None).collect()).collect();
    for v in 0..m {
        for w in (0..m).filter(|&w| teil(w) != teil(v)) {
            let ueber_mitte = r(w).mitte.as_ref().filter(|mt| mt.len() >= 2).map(|mt| {
                let k = (0..mt.len())
                    .min_by(|&x, &y| abstand(end[v], mt[x]).total_cmp(&abstand(end[v], mt[y])))
                    .unwrap();
                let mut weg: Linie = vec![end[v]];
                weg.extend(mt[..=k].iter().rev().copied());
                weg.push(anf[w]);
                (masken(end[v], mt[k]), weg)
            });
            verb[v][w] = Some(Verbindung {
                gerade: masken(end[v], anf[w]),
                gerade_laenge: abstand(end[v], anf[w]),
                ueber_mitte,
            });
        }
    }
    let weglaenge = |w: &Linie| w.windows(2).map(|s| abstand(s[0], s[1])).sum::<f64>();
    // Laufweg v → w, wenn verdeckt (`rest` = noch ausstehende Teile einschließlich w): Punkte von end[v] bis anf[w]
    let laufweg_fuer = |v: usize, w: usize, rest: u64| -> Option<Linie> {
        let vb = verb[v][w].as_ref()?;
        if vb.gerade.iter().all(|&mk| mk & rest != 0) {
            return Some(vec![end[v], anf[w]]);
        }
        let (zugang, weg) = vb.ueber_mitte.as_ref()?;
        zugang.iter().all(|&mk| mk & rest != 0).then(|| weg.clone())
    };
    let alle: u64 = (1u64 << n) - 1;
    let anfahrt = |w: usize| start.map_or(0.0, |s| abstand(s, anf[w]));
    // nächster Knoten (Rückfall bei vielen Teilen)
    let gierig = || -> Vec<usize> {
        let mut folge = Vec::new();
        let mut rest = alle;
        let mut pos = start.unwrap_or(anf[0]);
        while rest != 0 {
            let w = (0..m)
                .filter(|&w| rest & (1 << teil(w)) != 0)
                .min_by(|&x, &y| abstand(pos, anf[x]).total_cmp(&abstand(pos, anf[y])))
                .unwrap();
            folge.push(w);
            rest &= !(1 << teil(w));
            pos = end[w];
        }
        folge
    };

    // beste Reihenfolge und Richtung: (Sprünge, Weglänge) möglichst klein – eigene Festlegung der Bewertung
    let mut reihenfolge: Vec<usize> = Vec::new();
    if n <= 7 {
        let mut beste: (usize, f64) = (usize::MAX, f64::INFINITY);
        let mut stapel: Vec<(Vec<usize>, u64, usize, f64)> = (0..m)
            .map(|w| (vec![w], alle & !(1 << teil(w)), 0, anfahrt(w)))
            .collect();
        stapel.sort_by(|x, y| y.3.total_cmp(&x.3));
        let mut schritte = 0usize;
        while let Some((weg, rest, sprunge, laenge)) = stapel.pop() {
            schritte += 1;
            if schritte > 200_000 {
                break;
            }
            if sprunge > beste.0 || (sprunge == beste.0 && laenge >= beste.1) {
                continue;
            }
            if rest == 0 {
                beste = (sprunge, laenge);
                reihenfolge = weg;
                continue;
            }
            let v = *weg.last().unwrap();
            let mut neu: Vec<(Vec<usize>, u64, usize, f64)> = (0..m)
                .filter(|&w| rest & (1 << teil(w)) != 0)
                .map(|w| {
                    let (s, l) = match laufweg_fuer(v, w, rest) {
                        Some(lw) => (sprunge, laenge + weglaenge(&lw)),
                        None => (
                            sprunge + 1,
                            laenge + verb[v][w].as_ref().map_or(0.0, |vb| vb.gerade_laenge),
                        ),
                    };
                    let mut wg = weg.clone();
                    wg.push(w);
                    (wg, rest & !(1 << teil(w)), s, l)
                })
                .collect();
            // die günstigsten zuletzt auf den Stapel, damit sie zuerst geprüft werden
            neu.sort_by(|x, y| (y.2, y.3).partial_cmp(&(x.2, x.3)).unwrap_or(std::cmp::Ordering::Equal));
            stapel.extend(neu);
        }
    }
    if reihenfolge.len() != n {
        reihenfolge = gierig();
    }

    // berührende Teile bilden Gruppen (Union-Find); Gruppen in der Reihenfolge ihres ersten Teils
    let mut eltern: Vec<usize> = (0..n).collect();
    fn wurzel(e: &mut [usize], mut k: usize) -> usize {
        while e[k] != k {
            e[k] = e[e[k]];
            k = e[k];
        }
        k
    }
    for i in 0..n {
        for j in i + 1..n {
            if let (Some(a), Some(b)) = (stuecke[i].1, stuecke[j].1)
                && umrisse_beruehren(a, b)
            {
                let (wi, wj) = (wurzel(&mut eltern, i), wurzel(&mut eltern, j));
                eltern[wj] = wi;
            }
        }
    }
    let mut gruppen: Vec<Vec<usize>> = Vec::new();
    let mut gruppe_von: Vec<Option<usize>> = vec![None; n];
    for &v in &reihenfolge {
        let w = wurzel(&mut eltern, teil(v));
        let g = match gruppe_von[w] {
            Some(g) => g,
            None => {
                gruppen.push(Vec::new());
                gruppe_von[w] = Some(gruppen.len() - 1);
                gruppen.len() - 1
            }
        };
        gruppen[g].push(v);
    }

    // Laufweg von `von` nach `nach` (Ziel ist Knoten `ziel`), wenn verdeckt von noch ausstehenden Teilen `rest`:
    // gerade, oder über die Mittellinie des Ziels
    let laufweg_zu = |von: Punkt, nach: Punkt, ziel: usize, rest: u64| -> Option<Linie> {
        if masken(von, nach).iter().all(|&mk| mk & rest != 0) {
            return Some(vec![von, nach]);
        }
        let mt = r(ziel).mitte.as_ref().filter(|mt| mt.len() >= 2)?;
        let naechster = |q: Punkt| {
            (0..mt.len())
                .min_by(|&x, &y| abstand(q, mt[x]).total_cmp(&abstand(q, mt[y])))
                .unwrap()
        };
        let (k, t) = (naechster(von), naechster(nach));
        if !masken(von, mt[k]).iter().all(|&mk| mk & rest != 0) {
            return None;
        }
        let mut weg: Linie = vec![von];
        if k <= t {
            weg.extend(mt[k..=t].iter().copied());
        } else {
            weg.extend(mt[t..=k].iter().rev().copied());
        }
        weg.push(nach);
        Some(weg)
    };
    // Stück anhängen: mit Laufweg an den letzten Zug, sonst als neuer Zug (Sprung)
    let mut out: Vec<Linie> = Vec::new();
    let anhaengen = |out: &mut Vec<Linie>, stueck: Linie, ziel: usize, rest: u64| {
        let Some(letzter) = out.last_mut() else {
            out.push(stueck);
            return;
        };
        let von = *letzter.last().unwrap();
        if abstand(von, stueck[0]) < 1e-6 {
            letzter.extend(stueck.into_iter().skip(1));
            return;
        }
        match laufweg_zu(von, stueck[0], ziel, rest) {
            Some(weg) => {
                // Laufstiche höchstens `laufweg` lang entlang des Wegs, Eckpunkte des Wegs als Einstiche
                for s in weg.windows(2) {
                    let schritte = ((abstand(s[0], s[1]) / laufweg.max(0.5)).ceil() as usize).max(1);
                    letzter.extend((1..=schritte).map(|k| zwischen(s[0], s[1], k as f64 / schritte as f64)));
                }
                letzter.pop(); // der Anfang des Stücks kommt mit dem Stück selbst
                letzter.extend(stueck);
            }
            None => out.push(stueck),
        }
    };
    let mut rest = alle;
    for g in &gruppen {
        // erst alle Unterlagen der Gruppe, rückwärts – die letzte endet am Anfang der ersten Säule
        for &v in g.iter().rev() {
            if !r(v).unter.is_empty() {
                anhaengen(&mut out, r(v).unter.clone(), v, rest);
            }
        }
        // dann die Decklagen in ihrer Reihenfolge
        for &v in g {
            let mut wege = r(v).wege.clone().into_iter();
            if let Some(erster) = wege.next() {
                anhaengen(&mut out, erster, v, rest);
            }
            out.extend(wege);
            rest &= !(1 << teil(v));
        }
    }
    // weitere Teile über 63 (kommt bei Buchstaben nicht vor): ohne Laufweg anhängen
    for (richt, _) in stuecke.iter().skip(n) {
        if !richt[0].unter.is_empty() {
            out.push(richt[0].unter.clone());
        }
        out.extend(richt[0].wege.iter().cloned());
    }
    Ok(out)
}

/// Teile einer Kette (Knopfloch) nacheinander zu **einem** Zug verbinden. Satinsäulen in der Richtung, deren Anfang
/// dem vorigen Ende am nächsten liegt (Musterdesign `Buttonholes.ART`: Raupen beginnen dort, wo der vorige Teil
/// endet; Läufe und Handstiche wie in der Vorlage).
fn kette_wege(
    teile: &[Objekt],
    umkehrbar: &[bool],
    e: &Einstellungen,
    start: Option<Punkt>,
) -> Result<Vec<Linie>, String> {
    let mut zug: Linie = Vec::new();
    for (k, t) in teile.iter().enumerate() {
        let ende = zug.last().copied().or(start);
        let vor = t.wege(e, None)?;
        let rueck_art = match &t.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => Some(ObjektArt::Satin {
                seite_a: seite_a.iter().rev().copied().collect(),
                seite_b: seite_b.iter().rev().copied().collect(),
                unterlage: unterlage.clone(),
                optionen: optionen.clone(),
            }),
            ObjektArt::Lauf {
                punkte,
                ecken,
                laenge,
                dreifach,
                geschlossen,
            } => Some(ObjektArt::Lauf {
                punkte: punkte.iter().rev().copied().collect(),
                ecken: ecken.iter().rev().copied().collect(),
                laenge: *laenge,
                dreifach: *dreifach,
                geschlossen: *geschlossen,
            }),
            _ => None,
        }
        .filter(|_| umkehrbar.get(k).copied().unwrap_or(false));
        let wege: Vec<Linie> = match rueck_art {
            Some(art) => {
                let rueck = Objekt { art, ..t.clone() }.wege(e, None)?;
                let anfang = |w: &[Linie]| w.first().and_then(|l| l.first()).copied();
                match (ende, anfang(&vor), anfang(&rueck)) {
                    (Some(q), Some(a), Some(b)) if abstand(q, b) < abstand(q, a) => rueck,
                    _ => vor,
                }
            }
            None => vor,
        };
        for w in wege {
            let ueberspringen = usize::from(
                zug.last()
                    .is_some_and(|&l| w.first().is_some_and(|&f| abstand(l, f) < 1e-9)),
            );
            zug.extend(w.into_iter().skip(ueberspringen));
        }
    }
    Ok(if zug.is_empty() { vec![] } else { vec![zug] })
}

impl Objekt {
    pub fn wege(&self, e: &Einstellungen, start: Option<Punkt>) -> Result<Vec<Linie>, String> {
        match &self.art {
            ObjektArt::Lauf {
                punkte,
                ecken,
                laenge,
                dreifach: drei,
                geschlossen,
            } => {
                let (mut pts, mut ecken) = (punkte.clone(), ecken.clone());
                if ecken.len() != pts.len() {
                    ecken = vec![true; pts.len()];
                }
                if *geschlossen && !pts.is_empty() && abstand(pts[0], *pts.last().unwrap()) > 1e-9 {
                    pts.push(pts[0]);
                    ecken.push(true);
                }
                let lauf = laufstich_mit_ecken(&pts, Some(&ecken), &e.lauf, *laenge);
                Ok(vec![if *drei { dreifach(&lauf) } else { lauf }])
            }
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => satin_wege(seite_a, seite_b, unterlage, optionen, e),
            ObjektArt::SatinLinie {
                punkte,
                breite,
                unterlage,
                mittig,
                optionen,
            } => {
                let (a, b) = satinlinie_seiten(punkte, *breite, *mittig);
                satin_wege(&a, &b, unterlage, optionen, e)
            }
            ObjektArt::Gruppe {
                teile,
                umrisse,
                laufweg,
            } => gruppe_wege(teile, umrisse, *laufweg, e, start),
            ObjektArt::Kette { teile, umkehrbar, .. } => kette_wege(teile, umkehrbar, e, start),
            ObjektArt::Knopfloch(_) => Err("Knopfloch wurde nicht aufgelöst".into()),
            ObjektArt::Motivlinie(_) => Err("Motivlinie wurde nicht aufgelöst".into()),
            ObjektArt::Blanket {
                punkte,
                geschlossen,
                abstand,
                breite,
                mittig,
            } => Ok(vec![blanket(punkte, *geschlossen, *abstand, *breite, *mittig)]),
            ObjektArt::Text { alphabet, .. } => Err(format!("Text mit Alphabet „{alphabet}“ wurde nicht aufgelöst")),
            ObjektArt::TtfText { schrift, .. } => Err(format!("Text mit Schrift „{schrift}“ wurde nicht aufgelöst")),
            ObjektArt::Applikation(_) => Err("Applikation wurde nicht aufgelöst".into()),
            ObjektArt::SatinFlaeche { .. } => Err("Satinfläche wurde nicht aufgelöst".into()),
            ObjektArt::Tatami {
                umriss,
                loecher,
                winkel,
                unterlage,
                ..
            } => {
                let w = winkel.unwrap_or(e.tatami.winkel);
                let mut wege = Vec::new();
                let mut start = start;
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    wege.extend(flaechen_unterlage(umriss, loecher, w, &e.unterlage, start));
                    start = wege.last().and_then(|l: &Linie| l.last().copied());
                }
                wege.extend(tatami(umriss, loecher, &e.tatami, Some(w), start));
                Ok(wege)
            }
            ObjektArt::Wendefuellung {
                umriss,
                winkellinien,
                unterlage,
                kurzstiche,
            } => {
                let (a, b) = crate::sticharten::wendung::wende_seiten(umriss, winkellinien)?;
                let l = &winkellinien[0];
                let w = (l[1].y - l[0].y).atan2(l[1].x - l[0].x).to_degrees();
                let mut wege = Vec::new();
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    // Unterlage wie bei Tatami, Deckrichtung = erste Winkellinie (eigene Festlegung)
                    wege.extend(flaechen_unterlage(umriss, &[], w, &e.unterlage, start));
                }
                // Step-Reihen längs der Winkellinien: Abstand, Zugausgleich, Länge, Versatz der Step-Einstellung
                let mut se = e.satin.clone();
                se.auto_spacing = false;
                se.abstand_fest = e.tatami.abstand;
                se.zugausgleich = e.tatami.zugausgleich;
                // Kurzstiche wie bei Satin auf die Reihen, danach Teilung in Step-Stiche (Effekt im allgemeinen
                // Füllgenerator, `Analyse_Teil3/10` §1–2)
                se.kurzstiche.an = kurzstiche.unwrap_or(e.satin.kurzstiche.an);
                wege.push(crate::sticharten::satin::step_reihen(
                    &satin_einstiche(&a, &b, &se, false)?,
                    &e.tatami,
                ));
                Ok(wege)
            }
            ObjektArt::Musterfuellung {
                umriss,
                loecher,
                muster,
                winkel,
                groesse,
                unterlage,
                ..
            } => {
                let m = crate::muster::finden(muster).ok_or_else(|| format!("Füllmuster „{muster}“ gibt es nicht"))?;
                let w = winkel.or(m.winkel).unwrap_or(e.tatami.winkel);
                let mut wege = Vec::new();
                let mut start = start;
                if unterlage.unwrap_or(e.unterlage.auto_an) {
                    wege.extend(flaechen_unterlage(umriss, loecher, w, &e.unterlage, start));
                    start = wege.last().and_then(|l: &Linie| l.last().copied());
                }
                let t = m.teilung(*groesse);
                wege.extend(tatami_mit_teilung(umriss, loecher, &e.tatami, Some(w), start, Some(&t)));
                Ok(wege)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Design {
    pub name: String,
    pub rahmen: Rahmen,
    /// Garnkarte (JSON `"garnkarte"`, Standard „Default“ wie die Maschine; „keine“ = ohne) – ordnet der Aufrufer zu
    pub garnkarte: String,
    pub einstellungen: Einstellungen,
    pub objekte: Vec<Objekt>,
    /// geladene Alphabete (Name wie im Alphabet-Kopf, z. B. „Drifter“); füllt der Aufrufer, siehe `alphabet_namen`
    pub alphabete: BTreeMap<String, Alphabet>,
    /// Designende wie „Start und Ende“ im Original (Handbuch PDF-S. 205: Auto End – Mitte): nach dem letzten Stich ein
    /// Sprung zurück zum Ursprung (Rahmenmitte). Belegt an den Designs der Maschine: 34 von 51 enden genau auf 0,0
    /// (die Zeichensätze, deren Ursprung am Zeilenanfang liegt, auf der Grundlinie). Standard an.
    /// Start- und Endpunkt des Designs (Dialog „Start and End“)
    pub start: Bezugspunkt,
    pub ende: Bezugspunkt,
    /// geladene Computer-Schriften (Name wie im Design); füllt der Aufrufer, siehe `ttf_bedarf`
    pub ttf_schriften: BTreeMap<String, TtfSchrift>,
    /// geladene Knopfloch-Vorlagen (Name wie in ButtonHoles.ESB, „Standard“); füllt der Aufrufer, siehe
    /// `knopfloch_bedarf`
    pub knopfloecher: BTreeMap<String, Vorlage>,
    /// geladene Motivsätze (Name wie im Design-Feld `satz`); füllt der Aufrufer, siehe `motiv_namen`
    pub motivsaetze: BTreeMap<String, crate::motiv::Motivsatz>,
    /// Transformation je Objekt (Nummer in `objekte`), JSON `"transform"` am Objekt (`transformation.rs`)
    pub transformationen: BTreeMap<usize, Transformation>,
    /// Transformation des ganzen Designs (um seine Mitte), JSON `"transform"` oben im Design
    pub transformation: Transformation,
    /// Gruppen („Group“ im Original): Objektnummern (in `objekte`) und Transformation der Gruppe (um die Mitte ihrer
    /// Objekte), JSON `"gruppen"` am Design und `"gruppe"` am Objekt. Nur Gruppen mit Transformation
    pub gruppen: Vec<(Vec<usize>, Transformation)>,
    /// Farben für `^` und einzelne Buchstaben je Text (Nummer in `objekte`), JSON `"farben"`/`"buchstaben"`
    pub buchstaben: BTreeMap<usize, TextZusatz>,
    /// Double Run (Handbuch Kap. 23 „Reinforcing outlines“): diese Objekte (Nummer in `objekte`) werden nach dem
    /// Sticken mit umgekehrter Stichfolge noch einmal gestickt, gleiche Farbe, ohne Verbindungsstiche. JSON
    /// `"doppellauf": true` am Objekt.
    pub doppellauf: std::collections::BTreeSet<usize>,
    /// Auto Jump je Objekt (Nummer in `objekte`), JSON `"auto_jump"`; ohne Angabe: Standard des Stils
    pub auto_jump: BTreeMap<usize, AutoJump>,
    /// Step-Muster 1–20 je Objekt (Nummer in `objekte`), JSON `"step_muster"` – für Tatami-Flächen und Step in Schrift
    pub step_muster: BTreeMap<usize, usize>,
    /// Garn von Hand je Objekt (Nummer in `objekte`), JSON `"garn"`: Nummer in der Garnkarte – ordnet der Aufrufer zu
    pub garn: BTreeMap<usize, String>,
    /// Einstich je Objekt (Nummer in `objekte`), JSON `"einstich": [x, y]` – „entry point“ (`ein_ausstich`)
    pub einstich: BTreeMap<usize, Punkt>,
    /// Ausstich je Objekt, JSON `"ausstich": [x, y]` – „exit point“
    pub ausstich: BTreeMap<usize, Punkt>,
    /// Closest Join für alle Objekte ohne eigenen Einstich (außer Schrift), JSON `"kuerzeste_verbindung": true`
    pub kuerzeste_verbindung: bool,
    /// Werte je Objekt (Nummer in `objekte`), JSON `"einstellungen"` am Objekt: vollständige Einstellungen
    /// (Stoff → Design → Objekt). Wie im Original: jedes Objekt einer ART-Datei trägt eine eigene Kopie des Stil-Baums
    /// (Lauf, Satin, Tatami, Unterlagen, Verbinder; `Analyse_Teil3/03` §4, hoch)
    pub objekt_einstellungen: BTreeMap<usize, Einstellungen>,
}

/// Auto Jump eines Objekts (Effects-Dialog, Handbuch Kap. 22 „Preserving long stitches with Auto Jump“): lange Stiche
/// werden zu Sprüngen (kein Einstich in der Mitte), sonst in kürzere Stiche geteilt. Standard an mit 12,7 / 7,0 mm
/// (AMT 0x2711/0x2712/0x2758), im Stil Schrift **aus** (`Analyse/code/10` Tabelle Stile). Kleiner Höchststich (z. B.
/// 6 mm) ergibt nach Handbuch eine Säule „with more loft“.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoJump {
    pub an: bool,
    /// Höchststich in mm (darüber Sprünge bzw. Teilung)
    pub max: f64,
    /// Länge der Sprünge in mm
    pub sprung: f64,
}

impl Default for AutoJump {
    fn default() -> Self {
        AutoJump {
            an: true,
            max: crate::einheiten::MAX_STICH_MM,
            sprung: crate::einheiten::AUTO_JUMP_TEILLAENGE_MM,
        }
    }
}

/// Auto Jump aus: Stiche über dem Höchststich der Maschine in gleich lange echte Stiche teilen (Handbuch: „it is
/// broken into smaller stitches“; gleich lange Stücke: eigene Festlegung).
fn lange_stiche_einstechen(w: &mut Linie, max: f64) {
    let mut out = Vec::with_capacity(w.len());
    for (i, &q) in w.iter().enumerate() {
        if i > 0 {
            let a = w[i - 1];
            let d = abstand(a, q);
            if d > max + 1e-9 {
                let n = (d / max - 1e-9).ceil() as usize;
                out.extend((1..n).map(|j| zwischen(a, q, j as f64 / n as f64)));
            }
        }
        out.push(q);
    }
    *w = out;
}

impl Design {
    pub fn aus_json(text: &str) -> Result<Design, String> {
        let v: Value = serde_json::from_str(text).map_err(|e| format!("JSON ungültig: {e}"))?;
        Design::aus_value(&v)
    }

    /// Design aus JSON (Format `format.rs`; Schema `format::schema()`).
    pub fn aus_value(v: &Value) -> Result<Design, String> {
        let v = &crate::format::migrieren(v)?;
        let d: DesignJson = serde_json::from_value(v.clone()).map_err(|e| design_fehler(v, e))?;
        let mut e = fuer_stoff(&d.stoff)?;
        if let Some(werte) = &d.einstellungen {
            e = e.ueberschreiben(&werte.0)?;
        }
        // Monogramme werden zu Text (Diamond2/Diamond3)
        let roh: Vec<ObjektJson> = d
            .objekte
            .into_iter()
            .enumerate()
            .map(|(i, o)| match o {
                ObjektJson::Monogramm(m) => monogramm_als_text(m)
                    .map(ObjektJson::Text)
                    .map_err(|e| format!("Objekt {}: {e}", i + 1)),
                x => Ok(x),
            })
            .collect::<Result<_, _>>()?;

        // Prüfungen, die das Format allein nicht ausdrückt
        for (i, o) in roh.iter().enumerate() {
            let wo = format!("Objekt {}", i + 1);
            let block = match o {
                ObjektJson::Text(t) => t.schrift.ausrichtung == Ausrichtung::Block && t.schrift.grundlinie.is_none(),
                ObjektJson::Ttf(t) => {
                    t.schrift_optionen.ausrichtung == Ausrichtung::Block && t.schrift_optionen.grundlinie.is_none()
                }
                _ => false,
            };
            if block {
                return Err(format!(
                    "{wo}: Ausrichtung „block“ braucht eine Grundlinie mit fester Länge (kurve, gerade mit laenge, \
                     kreis mit bogen)"
                ));
            }
            if let ObjektJson::Tatami(t) = o
                && !t.winkellinien.is_empty()
                && (!t.loecher.is_empty() || !t.aussparen.is_empty())
            {
                return Err(format!(
                    "{wo}: Fläche mit Winkellinien: ohne Löcher und Aussparen (noch nicht möglich)"
                ));
            }
            if let ObjektJson::Satinflaeche(s) = o
                && s.knickwinkel.is_some_and(|k| !(0.0..180.0).contains(&k))
            {
                return Err(format!("{wo}: Satinfläche: knickwinkel zwischen 0 und 180 Grad"));
            }
        }

        // Felder, die jedes Objekt haben kann (Abschnitt „Bearbeiten“)
        let mut transformationen = BTreeMap::new();
        let mut doppellauf = std::collections::BTreeSet::new();
        let mut auto_jump = BTreeMap::new();
        let mut step_muster = BTreeMap::new();
        let mut garn = BTreeMap::new();
        let mut einstich = BTreeMap::new();
        let mut ausstich = BTreeMap::new();
        let mut objekt_einstellungen = BTreeMap::new();
        let mut gruppe_von: Vec<Option<String>> = Vec::with_capacity(roh.len());
        for (i, o) in roh.iter().enumerate() {
            let wo = format!("Objekt {}", i + 1);
            let g = o.gemeinsam();
            if let Some(k) = &g.gruppe
                && !d.gruppen.contains_key(k)
            {
                return Err(format!("{wo}: gruppe „{k}“ fehlt in „gruppen“ des Designs"));
            }
            gruppe_von.push(g.gruppe.clone());
            if let Some(t) = &g.transform {
                transform_pruefen(t, &format!("{wo}: transform"))?;
                if !t.ist_leer() {
                    transformationen.insert(i, t.clone());
                }
            }
            if g.doppellauf {
                doppellauf.insert(i);
            }
            if let Some(aj) = g.auto_jump {
                let s = AutoJump::default();
                let aj = match aj {
                    crate::format::AutoJumpJson::An(an) => AutoJump { an, ..s },
                    crate::format::AutoJumpJson::Werte(w) => AutoJump {
                        an: true,
                        max: w.max.unwrap_or(s.max),
                        sprung: w.sprung.unwrap_or(s.sprung),
                    },
                };
                let bereich = crate::einheiten::MIN_STICH_MM..=crate::einheiten::MAX_STICH_MM;
                if !bereich.contains(&aj.max) || !bereich.contains(&aj.sprung) {
                    return Err(format!("{wo}: auto_jump: max und sprung zwischen 0,3 und 12,7 mm"));
                }
                auto_jump.insert(i, aj);
            }
            if let Some(nr) = g.step_muster {
                TatamiEinstellung::default()
                    .step_muster(nr)
                    .map_err(|e| format!("{wo}: {e}"))?;
                step_muster.insert(i, nr);
            }
            if let Some(n) = &g.garn {
                if n.trim().is_empty() {
                    return Err(format!("{wo}: garn: Nummer fehlt"));
                }
                garn.insert(i, n.trim().to_string());
            }
            if let Some(q) = g.einstich {
                einstich.insert(i, q);
            }
            if let Some(q) = g.ausstich {
                ausstich.insert(i, q);
            }
            if let Some(w) = &g.einstellungen {
                if w.0.get("stoff").is_some() {
                    return Err(format!("{wo}: einstellungen: „stoff“ gilt nur für das ganze Design"));
                }
                let eigene = e
                    .ueberschreiben(&w.0)
                    .map_err(|x| format!("{wo}: einstellungen: {x}"))?;
                objekt_einstellungen.insert(i, eigene);
            }
        }

        // Farben für `^`, einzelne Buchstaben, Stichart und Verzerrung je Text
        let mut buchstaben = BTreeMap::new();
        for (i, o) in roh.iter().enumerate() {
            let wo = format!("Objekt {}", i + 1);
            let zusatz = match o {
                ObjektJson::Text(t) => {
                    let stichart = match t.stichart {
                        None | Some(crate::format::SchriftStichart::Satin) => None,
                        Some(crate::format::SchriftStichart::Auto) => Some(SchriftStichart::Auto),
                        Some(crate::format::SchriftStichart::Step) => Some(SchriftStichart::Step),
                        Some(crate::format::SchriftStichart::Muster) => {
                            Some(schrift_muster(&wo, &t.schrift, t.winkel)?)
                        }
                    };
                    schrift_zusatz(&wo, &t.text, &t.schrift, stichart)?
                }
                ObjektJson::Ttf(t) => {
                    let stichart = match t.stichart {
                        crate::format::TtfStichart::Muster => Some(schrift_muster(&wo, &t.schrift_optionen, t.winkel)?),
                        _ => None,
                    };
                    schrift_zusatz(&wo, &t.text, &t.schrift_optionen, stichart)?
                }
                _ => None,
            };
            if let Some(z) = zusatz {
                buchstaben.insert(i, z);
            }
        }

        let objekte: Vec<Objekt> = roh.into_iter().map(objekt_aus_json).collect();
        for (i, o) in objekte.iter().enumerate() {
            if let ObjektArt::Text { vorgabe, .. } | ObjektArt::TtfText { vorgabe, .. } = &o.art {
                if let Some(g) = &vorgabe.grundlinie {
                    g.pruefen(&vorgabe.text).map_err(|e| format!("Objekt {}: {e}", i + 1))?;
                }
                if vorgabe.kursiv.abs() > 45.0 {
                    return Err(format!(
                        "Objekt {}: Kursiv {}° – höchstens 45° (Handbuch)",
                        i + 1,
                        vorgabe.kursiv
                    ));
                }
            }
        }
        let transformation = d.transform.unwrap_or_default();
        transform_pruefen(&transformation, "Design: transform")?;
        let mut gruppen = Vec::new();
        for (schluessel, g) in &d.gruppen {
            let Some(t) = g.transform.as_ref().filter(|t| !t.ist_leer()) else {
                continue;
            };
            transform_pruefen(t, &format!("Gruppe „{schluessel}“: transform"))?;
            let nr: Vec<usize> = (0..gruppe_von.len())
                .filter(|&i| gruppe_von[i].as_deref() == Some(schluessel.as_str()))
                .collect();
            if !nr.is_empty() {
                gruppen.push((nr, t.clone()));
            }
        }
        Ok(Design {
            transformationen,
            transformation,
            gruppen,
            buchstaben,
            doppellauf,
            auto_jump,
            step_muster,
            garn,
            einstich,
            ausstich,
            kuerzeste_verbindung: d.kuerzeste_verbindung,
            objekt_einstellungen,
            name: d.name,
            rahmen: rahmen(&d.rahmen)?,
            garnkarte: d.garnkarte,
            einstellungen: e,
            objekte,
            alphabete: BTreeMap::new(),
            start: d.start.map(Bezugspunkt::from).unwrap_or_default(),
            ende: d.ende.map(Bezugspunkt::from).unwrap_or_default(),
            ttf_schriften: BTreeMap::new(),
            knopfloecher: BTreeMap::new(),
            motivsaetze: BTreeMap::new(),
        })
    }

    /// Einstellungen eines Objekts (Nummer in `objekte`): eigene Werte je Objekt, sonst die des Designs.
    pub fn einstellungen_fuer(&self, nr: usize) -> &Einstellungen {
        self.objekt_einstellungen.get(&nr).unwrap_or(&self.einstellungen)
    }

    /// Namen der Alphabete, die die Texte des Designs brauchen.
    pub fn alphabet_namen(&self) -> Vec<String> {
        let mut n: Vec<String> = self
            .objekte
            .iter()
            .filter_map(|o| match &o.art {
                ObjektArt::Text { alphabet, .. } => Some(alphabet.clone()),
                _ => None,
            })
            .collect();
        n.sort();
        n.dedup();
        n
    }

    /// Namen der Motivsätze, die die Motivlinien des Designs brauchen.
    pub fn motiv_namen(&self) -> Vec<String> {
        let mut n: Vec<String> = self
            .objekte
            .iter()
            .filter_map(|o| match &o.art {
                ObjektArt::Motivlinie(m) => Some(m.satz.clone()),
                _ => None,
            })
            .collect();
        n.sort();
        n.dedup();
        n
    }

    /// Braucht das Design Knopfloch-Vorlagen (ButtonHoles.ESB)?
    pub fn knopfloch_bedarf(&self) -> bool {
        self.objekte.iter().any(|o| matches!(o.art, ObjektArt::Knopfloch(_)))
    }

    /// Computer-Schriften, die das Design braucht: Name → benutzte Zeichen (zum Laden durch den Aufrufer).
    pub fn ttf_bedarf(&self) -> BTreeMap<String, std::collections::BTreeSet<char>> {
        let mut m: BTreeMap<String, std::collections::BTreeSet<char>> = BTreeMap::new();
        for o in &self.objekte {
            if let ObjektArt::TtfText { schrift, vorgabe, .. } = &o.art {
                m.entry(schrift.clone()).or_default().extend(vorgabe.text.chars());
            }
        }
        m
    }

    /// Objekte mit aufgelösten Texten (Buchstaben als Satin-/Lauf-Objekte). Fehlende Zeichen werden übersprungen
    /// und im zweiten Wert gemeldet.
    pub fn objekte_aufgeloest(&self) -> Result<(Vec<Objekt>, Vec<char>), String> {
        let (o, f, _, _) = self.aufgeloest_mit_herkunft()?;
        Ok((o, f.into_iter().map(|(_, z)| z).collect()))
    }

    /// Zeichen, die das Alphabet bzw. die Schrift nicht hat, je Objekt (Nummer in `objekte`, ab 0) – in der Reihenfolge
    /// des Texts, ohne Doppelte.
    pub fn fehlende_zeichen(&self) -> Result<Vec<(usize, String)>, String> {
        let (_, f, _, _) = self.aufgeloest_mit_herkunft()?;
        let mut out: Vec<(usize, String)> = Vec::new();
        for (nr, z) in f {
            match out.iter_mut().find(|(n, _)| *n == nr) {
                Some((_, s)) if !s.contains(z) => s.push(z),
                Some(_) => {}
                None => out.push((nr, z.to_string())),
            }
        }
        Ok(out)
    }

    /// Satinstiche der Texte (Original-Alphabete, Monogramme, Computer-Schriften in Satin) nach Größe und Transformation: je Text (Nummer in
    /// `objekte`) der längste Satinstich in mm und ob Stichart „auto“ ihn deshalb auf Step stellt
    /// (`schrift::AUTO_STEP_AB_MM`). Texte, die schon Step/Muster sind, fehlen.
    pub fn schrift_satinstiche(&self) -> Result<Vec<(usize, f64, bool)>, String> {
        let (out, _, herkunft, _) = self.ungedreht_aufgeloest()?;
        let mut out = self.transformieren(out, &herkunft)?;
        Ok(self.schrift_auto_anwenden(&mut out, &herkunft))
    }

    /// Stichart „auto“ der Texte entscheiden (nach der Transformation, weil Größe und Verzerrung die Stichlänge
    /// bestimmen) und anwenden; Ergebnis wie `schrift_satinstiche`.
    fn schrift_auto_anwenden(&self, out: &mut [Objekt], herkunft: &[usize]) -> Vec<(usize, f64, bool)> {
        let mut info = Vec::new();
        for (nr, o) in self.objekte.iter().enumerate() {
            let auto = match &o.art {
                ObjektArt::Text { .. } => {
                    self.buchstaben.get(&nr).and_then(|z| z.stichart.as_ref()) == Some(&SchriftStichart::Auto)
                }
                ObjektArt::TtfText { stichart, .. } => matches!(stichart, TtfStichart::Satin { auto: true, .. }),
                _ => continue,
            };
            let idx: Vec<usize> = (0..out.len()).filter(|&j| herkunft.get(j) == Some(&nr)).collect();
            let teil: Vec<Objekt> = idx.iter().map(|&j| out[j].clone()).collect();
            let laengster = crate::schrift::laengster_satinstich(&teil);
            if laengster <= 0.0 {
                continue;
            }
            let step = auto && laengster > crate::schrift::AUTO_STEP_AB_MM;
            if step {
                let ttf = matches!(o.art, ObjektArt::TtfText { .. });
                for &j in &idx {
                    out[j] = if ttf {
                        crate::schrift::auf_tatami(out[j].clone())
                    } else {
                        crate::schrift::auf_step(out[j].clone())
                    };
                }
            }
            info.push((nr, laengster, step));
        }
        info
    }

    /// Wie `objekte_aufgeloest`, dazu je Objekt die Nummer des Objekts im Design, aus dem es stammt.
    pub(crate) fn aufgeloest_mit_herkunft(&self) -> Result<Aufgeloest, String> {
        let (out, fehlend, herkunft, buchstabe) = self.ungedreht_aufgeloest()?;
        let mut out = self.transformieren(out, &herkunft)?;
        self.schrift_auto_anwenden(&mut out, &herkunft);
        let (out, herkunft, buchstabe) = self.aussparen_aufloesen(out, &herkunft, &buchstabe)?;
        Ok((out, fehlend, herkunft, buchstabe))
    }

    /// Matrix der Transformation von Objekt `nr` (`"transform"`, um die Mitte seiner aufgelösten Teile wie in
    /// `transformieren`); `None` ohne Transformation. Für `umwandeln::in_punkte`.
    pub fn objekt_matrix(&self, nr: usize) -> Result<Option<crate::transformation::Matrix>, String> {
        let Some(t) = self.transformationen.get(&nr) else {
            return Ok(None);
        };
        let (out, _, herkunft, _) = self.ungedreht_aufgeloest()?;
        let teil: Vec<Objekt> = out
            .into_iter()
            .zip(herkunft)
            .filter(|(_, h)| *h == nr)
            .map(|(o, _)| o)
            .collect();
        Ok(Some(t.matrix(formmitte(&teil))?))
    }

    /// Objekte aufgelöst (Text, Knopfloch, Applikation, Satinfläche), noch ohne Transformation und Aussparen.
    fn ungedreht_aufgeloest(&self) -> Result<Aufgeloest, String> {
        let mut out = Vec::new();
        let mut buchstabe: Vec<usize> = Vec::new();
        let mut fehlend = Vec::new();
        // je aufgelöstem Objekt: Nummer des Objekts im Design (für das Aussparen nach Namen)
        let mut herkunft: Vec<usize> = Vec::new();
        for (nr, o) in self.objekte.iter().enumerate() {
            match &o.art {
                ObjektArt::Text { alphabet, vorgabe } => {
                    let a = self
                        .alphabete
                        .get(alphabet)
                        .or_else(|| self.alphabete.values().find(|a| a.name.eq_ignore_ascii_case(alphabet)))
                        .ok_or_else(|| format!("Alphabet „{alphabet}“ nicht geladen"))?;
                    let abstand = vorgabe.abstand.unwrap_or(a.abstand_anteil * vorgabe.hoehe);
                    let (obj, f, nummern) = text_setzen_nr(
                        vorgabe,
                        abstand,
                        self.buchstaben.get(&nr),
                        self.einstellungen_fuer(nr).tatami.winkel,
                        |z| text_objekte_idx(a, z),
                    );
                    out.extend(obj);
                    buchstabe.extend(nummern);
                    fehlend.extend(f.into_iter().map(|z| (nr, z)));
                }
                ObjektArt::Knopfloch(k) => {
                    let v = self
                        .knopfloecher
                        .get(&k.vorlage)
                        .ok_or_else(|| format!("Knopfloch-Vorlage „{}“ nicht geladen (ButtonHoles.ESB)", k.vorlage))?;
                    k.pruefen().map_err(|e| format!("Objekt {}: {e}", nr + 1))?;
                    out.push(knopfloch_objekt(
                        v,
                        k,
                        &o.farbe,
                        if o.name.is_empty() { "Knopfloch" } else { &o.name },
                    ));
                }
                ObjektArt::Motivlinie(m) => {
                    let satz = self
                        .motivsaetze
                        .get(&m.satz)
                        .or_else(|| {
                            self.motivsaetze.values().find(|s| {
                                s.name.trim_start_matches('_').eq_ignore_ascii_case(m.satz.trim_start_matches('_'))
                            })
                        })
                        .ok_or_else(|| format!("Motivsatz „{}“ nicht geladen", m.satz))?;
                    out.push(
                        crate::motiv::motivlinie_objekt(
                            satz,
                            m,
                            &o.farbe,
                            if o.name.is_empty() { "Motivlinie" } else { &o.name },
                        )
                        .map_err(|e| format!("Objekt {}: {e}", nr + 1))?,
                    );
                }
                ObjektArt::Applikation(v) => {
                    out.extend(applikation_objekte(
                        v,
                        if o.name.is_empty() { "Applikation" } else { &o.name },
                    ));
                }
                ObjektArt::SatinFlaeche {
                    umriss,
                    loecher,
                    unterlage,
                    optionen,
                    knickwinkel,
                } => {
                    // mit Teilung am Knick wie die Turning-Umwandlung: Säulen überlappen an den Schnitten
                    let werte = crate::saeulen::SaeulenWerte {
                        knickwinkel: *knickwinkel,
                        ueberlappung: if *knickwinkel > 0.0 { crate::saeulen::UEBERLAPPUNG_TURNING } else { 0.0 },
                        ..Default::default()
                    };
                    let s = crate::saeulen::zerlegen(umriss, loecher, &werte)
                        .map_err(|e| format!("Objekt {}: Satinfläche: {e}", nr + 1))?;
                    let teile: Vec<Objekt> = s
                        .iter()
                        .map(|x| Objekt {
                            farbe: o.farbe.clone(),
                            name: o.name.clone(),
                            art: ObjektArt::Satin {
                                seite_a: x.seite_a.clone(),
                                seite_b: x.seite_b.clone(),
                                unterlage: unterlage.clone(),
                                optionen: optionen.clone(),
                            },
                        })
                        .collect();
                    // wie ein Buchstabe: Säulen nach Nähe, verbunden mit Laufweg (Lauf 0x271c der Füllung)
                    out.push(Objekt {
                        farbe: o.farbe.clone(),
                        name: o.name.clone(),
                        art: ObjektArt::Gruppe {
                            teile,
                            umrisse: s.into_iter().map(|x| x.umriss).collect(),
                            laufweg: self.einstellungen_fuer(nr).tatami.laufweg_laenge,
                        },
                    });
                }
                ObjektArt::TtfText {
                    schrift,
                    vorgabe,
                    stichart,
                } => {
                    let s = self
                        .ttf_schriften
                        .get(schrift)
                        .ok_or_else(|| format!("Schrift „{schrift}“ nicht geladen"))?;
                    let abstand = vorgabe
                        .abstand
                        .unwrap_or(crate::schrift::BUCHSTABENABSTAND_ANTEIL * vorgabe.hoehe);
                    let (obj, f, nummern) = text_setzen_nr(
                        vorgabe,
                        abstand,
                        self.buchstaben.get(&nr),
                        self.einstellungen_fuer(nr).tatami.winkel,
                        |z| ttf_text_objekte_idx(s, z, stichart),
                    );
                    out.extend(obj);
                    buchstabe.extend(nummern);
                    fehlend.extend(f.into_iter().map(|z| (nr, z)));
                }
                _ => out.push(o.clone()),
            }
            herkunft.resize(out.len(), nr);
            buchstabe.resize(out.len(), 0);
        }
        Ok((out, fehlend, herkunft, buchstabe))
    }

    /// Transformationen anwenden: erst je Objekt (um die Mitte seiner aufgelösten Teile), dann je Gruppe, dann das ganze
    /// Design.
    fn transformieren(&self, mut out: Vec<Objekt>, herkunft: &[usize]) -> Result<Vec<Objekt>, String> {
        let ws = self.einstellungen.tatami.winkel;
        for (nr, t) in &self.transformationen {
            let idx: Vec<usize> = (0..out.len()).filter(|&j| herkunft[j] == *nr).collect();
            let teil: Vec<Objekt> = idx.iter().map(|&j| out[j].clone()).collect();
            let m = t.matrix(formmitte(&teil))?;
            for &j in &idx {
                out[j] = abbilden(&out[j], &m, ws);
            }
        }
        // dann je Gruppe um die Mitte ihrer (schon transformierten) Objekte
        for (nr, t) in &self.gruppen {
            let idx: Vec<usize> = (0..out.len()).filter(|&j| nr.contains(&herkunft[j])).collect();
            let teil: Vec<Objekt> = idx.iter().map(|&j| out[j].clone()).collect();
            let m = t.matrix(formmitte(&teil))?;
            for &j in &idx {
                out[j] = abbilden(&out[j], &m, ws);
            }
        }
        if !self.transformation.ist_leer() {
            let m = self.transformation.matrix(formmitte(&out))?;
            out = out.iter().map(|o| abbilden(o, &m, ws)).collect();
        }
        Ok(out)
    }

    /// Aussparungen (Knockout) auflösen: Füllflächen mit `aussparen` verlieren die Flächen der genannten Objekte;
    /// zerfällt eine Fläche dabei, folgen die Teile als eigene Objekte gleicher Farbe (größter zuerst).
    fn aussparen_aufloesen(
        &self,
        objekte: Vec<Objekt>,
        herkunft: &[usize],
        buchstabe: &[usize],
    ) -> Result<MitHerkunft, String> {
        let mut neu: Vec<Option<Vec<Teil>>> = Vec::with_capacity(objekte.len());
        for (i, o) in objekte.iter().enumerate() {
            let (umriss, loecher, a) = match &o.art {
                ObjektArt::Tatami {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                }
                | ObjektArt::Musterfuellung {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                } if !aussparen.namen.is_empty() => (umriss, loecher, aussparen),
                _ => {
                    neu.push(None);
                    continue;
                }
            };
            let mut schneider = Vec::new();
            for name in &a.namen {
                let quellen: Vec<usize> = (0..self.objekte.len())
                    .filter(|&k| self.objekte[k].name == *name)
                    .collect();
                if quellen.is_empty() {
                    return Err(format!("Objekt „{name}“ zum Aussparen gibt es nicht"));
                }
                for (j, x) in objekte.iter().enumerate() {
                    if j != i && quellen.contains(&herkunft[j]) {
                        schneider.extend(bedeckt(&x.art, a.ueberlappung));
                    }
                }
            }
            neu.push(Some(flaeche_minus(umriss, loecher, &schneider)));
        }
        let mut out = Vec::with_capacity(objekte.len());
        let mut her = Vec::with_capacity(objekte.len());
        let mut bu = Vec::with_capacity(objekte.len());
        for (((o, n), &h), &b) in objekte.into_iter().zip(neu).zip(herkunft).zip(buchstabe) {
            let Some(teile) = n else {
                out.push(o);
                her.push(h);
                bu.push(b);
                continue;
            };
            for (u, l) in teile {
                let mut t = o.clone();
                if let ObjektArt::Tatami {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                }
                | ObjektArt::Musterfuellung {
                    umriss,
                    loecher,
                    aussparen,
                    ..
                } = &mut t.art
                {
                    (*umriss, *loecher, *aussparen) = (u, l, Aussparen::default());
                }
                out.push(t);
                her.push(h);
                bu.push(b);
            }
        }
        Ok((out, her, bu))
    }

    /// Stiche berechnen: Objekte -> Wege -> verbinden -> Bernina-Regeln (Mindeststich, Auto Jump).
    pub fn berechnen(&self) -> Result<StichFolge, String> {
        Ok(self.berechnen_mit_buchstaben()?.0)
    }

    /// Wie `berechnen`, dazu je Stich die Nummer des Buchstabens (1 = erster, wie `buchstaben[].nr`; 0 = kein
    /// Buchstabe) – für „einzelne Buchstaben“ per Klick in der Oberfläche (W9).
    pub fn berechnen_mit_buchstaben(&self) -> Result<(StichFolge, Vec<usize>), String> {
        let (objekte, _, herkunft, buchstabe) = self.aufgeloest_mit_herkunft()?;
        // je Teil: Farbe, Wege, eigene Auto-Jump-Werte, Nummer des Objekts im Design, Nummer des Buchstabens
        type Teile = Vec<(String, Vec<Linie>, Option<(f64, f64)>, usize, usize)>;
        let teile_ab = |start: Punkt| -> Result<Teile, String> {
            let mut teile = Vec::new();
            let mut pos = start;
            for (k, (obj, h)) in objekte.iter().zip(&herkunft).enumerate() {
                // Ein-/Ausstich (`ein_ausstich`): Einstich gilt für das erste, Ausstich für das letzte aufgelöste
                // Teil eines Objekts; Closest Join setzt den Einstich aller übrigen auf das vorige Ende (nicht bei
                // Schrift – dort gilt die Reihenfolge der Buchstaben und Säulen, Bericht 11)
                let erster = k == 0 || herkunft[k - 1] != *h;
                let letzter = k + 1 == herkunft.len() || herkunft[k + 1] != *h;
                let schrift = matches!(
                    self.objekte.get(*h).map(|o| &o.art),
                    Some(ObjektArt::Text { .. } | ObjektArt::TtfText { .. })
                );
                let ein = if erster { self.einstich.get(h).copied() } else { None }
                    .or_else(|| (self.kuerzeste_verbindung && !schrift).then_some(pos));
                let aus = if letzter { self.ausstich.get(h).copied() } else { None };
                let ausgerichtet;
                let obj = if ein.is_some() || aus.is_some() {
                    ausgerichtet = Objekt {
                        art: crate::ein_ausstich::ausrichten(&obj.art, ein, aus),
                        ..obj.clone()
                    };
                    &ausgerichtet
                } else {
                    obj
                };
                let beginn = crate::ein_ausstich::start_hinweis(&obj.art, ein, aus).unwrap_or(pos);
                let eigene;
                // Werte je Objekt (Stoff → Design → Objekt), dazu das Step-Muster des Objekts
                let basis = self.einstellungen_fuer(*h);
                let e = match self.step_muster.get(h) {
                    Some(&nr) => {
                        let mut x = basis.clone();
                        x.tatami.step_muster(nr)?;
                        eigene = x;
                        &eigene
                    }
                    None => basis,
                };
                let mut wege: Vec<Linie> = obj
                    .wege(e, Some(beginn))?
                    .into_iter()
                    .filter(|w| !w.is_empty())
                    .collect();
                if self.doppellauf.contains(h) {
                    doppellauf(&mut wege);
                }
                // Auto Jump: Angabe am Objekt, sonst Stil (Schrift aus, alles andere an mit Standardwerten)
                let aj = self.auto_jump.get(h).copied().unwrap_or(AutoJump {
                    an: !matches!(
                        self.objekte.get(*h).map(|o| &o.art),
                        Some(ObjektArt::Text { .. } | ObjektArt::TtfText { .. })
                    ),
                    ..AutoJump::default()
                });
                let eigen = if aj.an {
                    (aj != AutoJump::default()).then_some((aj.max, aj.sprung))
                } else {
                    wege.iter_mut()
                        .for_each(|w| lange_stiche_einstechen(w, crate::einheiten::MAX_STICH_MM));
                    None
                };
                if let Some(letzter) = wege.last().and_then(|w| w.last()) {
                    pos = *letzter;
                    teile.push((obj.farbe.clone(), wege, eigen, *h, buchstabe[k]));
                }
            }
            Ok(teile)
        };
        let null = p(0.0, 0.0);
        let mut teile = teile_ab(null)?;
        // Mitte bzw. Unterkante des Designs aus den Stichen (umschließendes Rechteck)
        let punkte = || teile.iter().flat_map(|(_, w, _, _, _)| w.iter().flatten());
        let (x0, x1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let (y0, y1) = punkte().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
        let (mitte, unten) = if x0 <= x1 {
            (p((x0 + x1) / 2.0, (y0 + y1) / 2.0), p((x0 + x1) / 2.0, y0))
        } else {
            (null, null)
        };
        let erster = teile
            .first()
            .and_then(|(_, w, _, _, _)| w.first())
            .and_then(|l| l.first())
            .copied();
        let start = match self.start {
            Bezugspunkt::Designmitte => mitte,
            Bezugspunkt::UntenMitte => unten,
            Bezugspunkt::Stich => erster.unwrap_or(null),
            Bezugspunkt::Rahmenmitte | Bezugspunkt::Start => null,
        };
        if start != null {
            teile = teile_ab(start)?;
        }
        let herkunft: Vec<usize> = teile.iter().map(|t| t.3).collect();
        let buchstabe_teil: Vec<usize> = teile.iter().map(|t| t.4).collect();
        let (teile, auto_jump): (Vec<_>, Vec<_>) = teile.into_iter().map(|(f, w, a, _, _)| ((f, w), a)).unzip();
        // Verbinder je Teil: eigene Werte des Objekts (im Original trägt jedes Objekt seinen Verbinder 0x272a)
        let verbinder: Vec<Option<&crate::einstellungen::VerbinderEinstellung>> = herkunft
            .iter()
            .map(|h| self.objekt_einstellungen.get(h).map(|e| &e.verbinder))
            .collect();
        let folge = wege_verbinden_je_teil(&teile, &self.einstellungen.verbinder, start, &auto_jump, &verbinder);
        let mut folge = bernina_regeln(&folge);
        // Teil → Buchstabe, Teil → Objekt im Design
        let mut buchstabe: Vec<usize> = folge.objekt.iter().map(|&t| buchstabe_teil[t]).collect();
        for o in &mut folge.objekt {
            *o = herkunft[*o];
        }
        let ende = match self.ende {
            Bezugspunkt::Rahmenmitte => Some(null),
            Bezugspunkt::Designmitte => Some(mitte),
            Bezugspunkt::UntenMitte => Some(unten),
            Bezugspunkt::Start => Some(start),
            Bezugspunkt::Stich => None,
        };
        if let Some(z) = ende
            && let Some(letzter) = folge.stiche.last()
            && abstand(letzter.p(), z) > 1e-9
        {
            folge.sprung(z);
            if let Some(&o) = folge.objekt.last() {
                folge.objekt.push(o);
                buchstabe.push(buchstabe.last().copied().unwrap_or(0));
            }
        }
        debug_assert!(folge.objekt.len() == folge.stiche.len());
        debug_assert!(buchstabe.len() == folge.objekt.len());
        Ok((folge, buchstabe))
    }

    /// **Formen je Objekt** für die Oberfläche (Zeichnen, Auswahl, Griffe): aufgelöste Umrisse, Löcher, Linien und
    /// Satinseiten nach Text-Satz, Transformation und Aussparen – gruppiert nach der Nummer des Objekts im Design.
    /// Ein Objekt ohne Form (z. B. Text ohne bekannte Zeichen) fehlt in der Liste nicht, es hat nur keine Linien.
    pub fn formen(&self) -> Result<Vec<ObjektForm>, String> {
        let (objekte, _, herkunft, _) = self.aufgeloest_mit_herkunft()?;
        let mut out: Vec<ObjektForm> = self
            .objekte
            .iter()
            .enumerate()
            .map(|(i, o)| ObjektForm {
                objekt: i,
                farbe: o.farbe.clone(),
                name: o.name.clone(),
                linien: Vec::new(),
                begrenzung: None,
                einstich: self.einstich.get(&i).copied(),
                ausstich: self.ausstich.get(&i).copied(),
            })
            .collect();
        for (o, &h) in objekte.iter().zip(&herkunft) {
            if let Some(f) = out.get_mut(h) {
                form_linien(&o.art, &mut f.linien);
            }
        }
        for f in &mut out {
            f.begrenzung = crate::geometrie::begrenzung(f.linien.iter().flat_map(|l| l.punkte.iter().copied()))
                .map(|(x0, y0, x1, y1)| [x0, y0, x1, y1]);
        }
        Ok(out)
    }

    /// Einstiche außerhalb des Stickfelds.
    pub fn ausserhalb(&self, folge: &StichFolge) -> Vec<Punkt> {
        folge
            .stiche
            .iter()
            .map(|s| s.p())
            .filter(|&q| !self.rahmen.enthaelt(q))
            .collect()
    }
}

auswahl! {
    /// Bedeutung einer Linie in der Formansicht
    pub enum FormArt {
        /// Außenkante einer Fläche
        Umriss = "umriss",
        /// Loch in einer Fläche
        Loch = "loch",
        /// gezeichnete Linie (Lauf, Dreifach, Blanket, Satinlinie)
        Linie = "linie",
        /// Seite A einer Satinsäule
        SeiteA = "seite_a",
        /// Seite B einer Satinsäule
        SeiteB = "seite_b",
        /// Stichwinkel-Linie
        Winkellinie = "winkellinie",
    }
}

/// Eine Linie der Formansicht.
#[derive(Clone, Debug, PartialEq, serde::Serialize, schemars::JsonSchema)]
pub struct FormLinie {
    pub art: FormArt,
    /// Punkte [x, y] in mm
    pub punkte: Vec<Punkt>,
    /// letzter Punkt mit dem ersten verbunden
    pub geschlossen: bool,
}

/// Formen eines Objekts des Designs (`Design::formen`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, schemars::JsonSchema)]
pub struct ObjektForm {
    /// Nummer des Objekts in `objekte` (0 = erstes)
    pub objekt: usize,
    pub farbe: String,
    pub name: String,
    pub linien: Vec<FormLinie>,
    /// umschließendes Rechteck [x0, y0, x1, y1] in mm (None = keine Form)
    pub begrenzung: Option<[f64; 4]>,
    /// Einstich [x, y], wenn gesetzt
    pub einstich: Option<Punkt>,
    /// Ausstich [x, y], wenn gesetzt
    pub ausstich: Option<Punkt>,
}

fn form_linien(art: &ObjektArt, out: &mut Vec<FormLinie>) {
    let mut linie = |art: FormArt, punkte: &[Punkt], geschlossen: bool| {
        if !punkte.is_empty() {
            out.push(FormLinie {
                art,
                punkte: punkte.to_vec(),
                geschlossen,
            });
        }
    };
    match art {
        ObjektArt::Lauf {
            punkte, geschlossen, ..
        }
        | ObjektArt::Blanket {
            punkte, geschlossen, ..
        } => linie(FormArt::Linie, punkte, *geschlossen),
        ObjektArt::SatinLinie { punkte, .. } => linie(FormArt::Linie, punkte, false),
        ObjektArt::Satin { seite_a, seite_b, .. } => {
            linie(FormArt::SeiteA, seite_a, false);
            linie(FormArt::SeiteB, seite_b, false);
        }
        ObjektArt::Tatami { umriss, loecher, .. }
        | ObjektArt::Musterfuellung { umriss, loecher, .. }
        | ObjektArt::SatinFlaeche { umriss, loecher, .. } => {
            linie(FormArt::Umriss, umriss, true);
            for l in loecher {
                linie(FormArt::Loch, l, true);
            }
        }
        ObjektArt::Wendefuellung {
            umriss, winkellinien, ..
        } => {
            linie(FormArt::Umriss, umriss, true);
            for w in winkellinien {
                linie(FormArt::Winkellinie, w, false);
            }
        }
        ObjektArt::Applikation(v) => {
            linie(FormArt::Umriss, &v.umriss, true);
            for l in &v.loecher {
                linie(FormArt::Loch, l, true);
            }
        }
        ObjektArt::Gruppe { teile, .. } => {
            for t in teile {
                form_linien(&t.art, out);
            }
        }
        ObjektArt::Kette { teile, loch, .. } => {
            if let Some(l) = loch {
                linie(FormArt::Loch, l, true);
            }
            for t in teile {
                form_linien(&t.art, out);
            }
        }
        // werden vor dem Zeichnen aufgelöst
        ObjektArt::Motivlinie(m) => linie(FormArt::Linie, &m.linie, m.geschlossen),
        ObjektArt::Text { .. } | ObjektArt::TtfText { .. } | ObjektArt::Knopfloch(_) => {}
    }
}

fn lauf_objekt(l: LaufJson, art_dreifach: bool) -> Objekt {
    let (punkte, ecken) = l.punkte.punkte_mit_ecken();
    Objekt {
        farbe: l.g.farbe,
        name: l.g.name,
        art: ObjektArt::Lauf {
            punkte,
            ecken,
            laenge: l.laenge,
            dreifach: l.dreifach.unwrap_or(art_dreifach),
            geschlossen: l.geschlossen,
        },
    }
}

fn satin_optionen(
    abstand: Option<f64>,
    faktor: Option<f64>,
    kurzstiche: Option<bool>,
    stichart: Option<SatinStichart>,
) -> SatinOptionen {
    SatinOptionen {
        abstand,
        faktor,
        kurzstiche,
        step: false,
        zickzack: stichart == Some(SatinStichart::Zickzack),
        zugausgleich: None,
    }
}

/// Objekt des Formats → Objekt des Rechenkerns (Monogramme sind schon Text).
fn objekt_aus_json(o: ObjektJson) -> Objekt {
    match o {
        ObjektJson::Lauf(l) => lauf_objekt(l, false),
        ObjektJson::Dreifach(l) => lauf_objekt(l, true),
        ObjektJson::Satin(s) => Objekt {
            farbe: s.g.farbe,
            name: s.g.name,
            art: ObjektArt::Satin {
                seite_a: s.seite_a.punkte(),
                seite_b: s.seite_b.punkte(),
                unterlage: s.unterlage.into(),
                optionen: satin_optionen(s.abstand, s.faktor, s.kurzstiche, s.stichart),
            },
        },
        ObjektJson::Text(t) => {
            let unterlage = match Unterlage::from(t.unterlage) {
                Unterlage::Auto => None,
                Unterlage::Arten(l) => Some(l),
            };
            let name = if t.g.name.is_empty() { t.text.clone() } else { t.g.name };
            Objekt {
                art: ObjektArt::Text {
                    alphabet: t.alphabet,
                    vorgabe: text_vorgabe(t.text, t.hoehe, &t.g.farbe, &t.schrift, unterlage),
                },
                name,
                farbe: t.g.farbe,
            }
        }
        ObjektJson::Ttf(t) => {
            let name = if t.g.name.is_empty() { t.text.clone() } else { t.g.name };
            let unterlage = Unterlage::from(t.unterlage);
            let stichart = match t.stichart {
                crate::format::TtfStichart::Lauf => TtfStichart::Lauf {
                    laenge: t.laenge,
                    dreifach: false,
                },
                crate::format::TtfStichart::Dreifach => TtfStichart::Lauf {
                    laenge: t.laenge,
                    dreifach: true,
                },
                crate::format::TtfStichart::Satin => TtfStichart::Satin { unterlage, auto: false },
                crate::format::TtfStichart::Auto => TtfStichart::Satin { unterlage, auto: true },
                // Stil „Schrift“: Auto-Unterlage an (Flächen: Step) – nur „keine“ schaltet sie ab
                crate::format::TtfStichart::Tatami | crate::format::TtfStichart::Muster => TtfStichart::Tatami {
                    winkel: t.winkel,
                    unterlage: Some(unterlage != Unterlage::Arten(vec![])),
                },
            };
            Objekt {
                art: ObjektArt::TtfText {
                    schrift: t.schrift,
                    vorgabe: text_vorgabe(t.text, t.hoehe, &t.g.farbe, &t.schrift_optionen, None),
                    stichart,
                },
                name,
                farbe: t.g.farbe,
            }
        }
        // wird vorher in Text umgeschrieben (`monogramm_als_text`)
        ObjektJson::Monogramm(_) => unreachable!("Monogramm wird vor dem Umwandeln zu Text"),
        ObjektJson::Muster(m) => Objekt {
            farbe: m.g.farbe,
            name: m.g.name,
            art: ObjektArt::Musterfuellung {
                umriss: m.umriss.punkte(),
                loecher: m.loecher.iter().map(|l| l.punkte()).collect(),
                muster: m.muster,
                winkel: m.winkel,
                groesse: m.groesse,
                unterlage: m.unterlage,
                aussparen: Aussparen {
                    namen: m.aussparen,
                    ueberlappung: m.ueberlappung,
                },
            },
        },
        ObjektJson::Applikation(a) => {
            let st = ApplikationVorgabe::default();
            Objekt {
                name: a.g.name.clone(),
                art: ObjektArt::Applikation(ApplikationVorgabe {
                    umriss: a.umriss.punkte(),
                    loecher: a.loecher.iter().map(PunktAngabe::punkte).collect(),
                    farbe: a.g.farbe.clone(),
                    breite: a.breite.unwrap_or(st.breite),
                    abstand: a.abstand,
                    heft_breite: a.heft_breite.unwrap_or(st.heft_breite),
                    heft_abstand: a.heft_abstand.unwrap_or(st.heft_abstand),
                    platz_laenge: a.platz_laenge.unwrap_or(st.platz_laenge),
                    blanket: a.abdeckung == Some(crate::format::Abdeckung::Blanket),
                    zuschneiden: a.stil == Some(crate::format::ApplikationStil::Zuschneiden),
                }),
                farbe: a.g.farbe,
            }
        }
        ObjektJson::Motivlinie(m) => Objekt {
            art: ObjektArt::Motivlinie(crate::motiv::MotivVorgabe {
                satz: m.satz,
                motiv: m.motiv,
                linie: m.punkte.punkte(),
                geschlossen: m.geschlossen,
                groesse: m.groesse,
                abstand: m.abstand,
                spiegeln: m.spiegeln,
            }),
            farbe: m.g.farbe,
            name: m.g.name,
        },
        ObjektJson::Knopfloch(k) => Objekt {
            farbe: k.g.farbe,
            name: k.g.name,
            art: ObjektArt::Knopfloch(KnopflochVorgabe {
                vorlage: crate::knopfloch::vorlage_name(k.typ.text())
                    .unwrap_or("Standard")
                    .to_string(),
                mitte: k.position.unwrap_or(p(0.0, 0.0)),
                laenge: k.laenge,
                abstand: k.abstand,
                winkel: k.winkel,
            }),
        },
        ObjektJson::Blanket(b) => Objekt {
            farbe: b.g.farbe,
            name: b.g.name,
            art: ObjektArt::Blanket {
                punkte: b.punkte.punkte(),
                geschlossen: b.geschlossen,
                abstand: b.abstand.unwrap_or(crate::sticharten::blanket::ABSTAND),
                breite: b.breite.unwrap_or(crate::sticharten::blanket::BREITE),
                mittig: b.versatz != Some(Versatz::Seite),
            },
        },
        ObjektJson::Satinlinie(s) => Objekt {
            farbe: s.g.farbe,
            name: s.g.name,
            art: ObjektArt::SatinLinie {
                punkte: s.punkte.punkte(),
                breite: s.breite.unwrap_or(2.0),
                unterlage: s.unterlage.into(),
                mittig: s.versatz != Some(Versatz::Seite),
                optionen: satin_optionen(None, None, None, s.stichart),
            },
        },
        ObjektJson::Tatami(t) if !t.winkellinien.is_empty() => Objekt {
            farbe: t.g.farbe,
            name: t.g.name,
            art: ObjektArt::Wendefuellung {
                umriss: t.umriss.punkte(),
                winkellinien: t.winkellinien,
                unterlage: t.unterlage,
                kurzstiche: t.kurzstiche,
            },
        },
        ObjektJson::Satinflaeche(s) => Objekt {
            farbe: s.g.farbe,
            name: s.g.name,
            art: ObjektArt::SatinFlaeche {
                umriss: s.umriss.punkte(),
                loecher: s.loecher.iter().map(|l| l.punkte()).collect(),
                unterlage: s.unterlage.into(),
                optionen: satin_optionen(s.abstand, s.faktor, s.kurzstiche, s.stichart),
                knickwinkel: s.knickwinkel.unwrap_or(0.0),
            },
        },
        ObjektJson::Tatami(t) => Objekt {
            farbe: t.g.farbe,
            name: t.g.name,
            art: ObjektArt::Tatami {
                umriss: t.umriss.punkte(),
                loecher: t.loecher.iter().map(|l| l.punkte()).collect(),
                winkel: t.winkel,
                unterlage: t.unterlage,
                aussparen: Aussparen {
                    namen: t.aussparen,
                    ueberlappung: t.ueberlappung,
                },
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::einstellungen::fuer_stoff;

    fn saeule(x0: f64, x1: f64, y0: f64, y1: f64, waagrecht: bool) -> (Objekt, Linie) {
        // Seiten A/B und Umriss eines Rechtecks; waagrecht: von x0 nach x1, sonst von y0 nach y1
        let (a, b) = if waagrecht {
            (vec![p(x0, y1), p(x1, y1)], vec![p(x0, y0), p(x1, y0)])
        } else {
            (vec![p(x0, y0), p(x0, y1)], vec![p(x1, y0), p(x1, y1)])
        };
        let umriss = vec![
            p(x0.min(x1), y0),
            p(x0.max(x1), y0),
            p(x0.max(x1), y1),
            p(x0.min(x1), y1),
        ];
        let o = Objekt {
            farbe: "#000000".into(),
            name: String::new(),
            art: ObjektArt::Satin {
                seite_a: a,
                seite_b: b,
                unterlage: Unterlage::Arten(vec![]),
                optionen: SatinOptionen::default(),
            },
        };
        (o, umriss)
    }

    /// wie Drifter „A“: Querbalken zuerst (endet über dem linken Bein), dann Laufweg im Bein unter der Säule
    #[test]
    fn gruppe_laufweg_statt_sprung() {
        let (bein, u_bein) = saeule(4.0, 0.0, 0.0, 10.0, false);
        let (balken, u_balken) = saeule(10.0, 1.0, 4.0, 6.0, true);
        let e = fuer_stoff("0 Software Defaults").unwrap();
        // Reihenfolge in der Eingabe: Bein zuerst – die Gruppe stickt trotzdem den Balken zuerst
        let wege = gruppe_wege(&[bein, balken], &[u_bein, u_balken], 3.0, &e, Some(p(0.0, 0.0))).unwrap();
        assert_eq!(wege.len(), 1, "ein Zug ohne Sprung");
        assert!(wege[0][0].x > 9.0, "beginnt am Balken");
    }

    fn mit_unterlage(mut o: Objekt) -> Objekt {
        if let ObjektArt::Satin { unterlage, .. } = &mut o.art {
            *unterlage = Unterlage::Arten(vec!["zickzack".into()]);
        }
        o
    }

    fn stelle(wege: &[Linie], q: Punkt) -> usize {
        let alle: Vec<Punkt> = wege.iter().flatten().copied().collect();
        alle.iter()
            .position(|&x| abstand(x, q) < 1e-9)
            .unwrap_or_else(|| panic!("Punkt {q:?} nicht in den Wegen"))
    }

    /// wie das Original (Schrift-Musterdesigns, `Analyse_Teil3/11`): berührende Säulen – erst alle Unterlagen
    /// rückwärts, dann die Säulen; getrennte Teile nacheinander
    #[test]
    fn gruppe_unterlagen_zuerst() {
        let e = fuer_stoff("0 Software Defaults").unwrap();
        let (bein, u_bein) = saeule(4.0, 0.0, 0.0, 10.0, false);
        let (balken, u_balken) = saeule(10.0, 1.0, 4.0, 6.0, true);
        let (bein, balken) = (mit_unterlage(bein), mit_unterlage(balken));
        let teile = [bein.clone(), balken.clone()];
        let wege = gruppe_wege(&teile, &[u_bein.clone(), u_balken.clone()], 3.0, &e, Some(p(0.0, 0.0))).unwrap();
        let getrennt = |o: &Objekt| match &o.art {
            ObjektArt::Satin {
                seite_a,
                seite_b,
                unterlage,
                optionen,
            } => satin_getrennt(seite_a, seite_b, unterlage, optionen, &e).unwrap(),
            _ => unreachable!(),
        };
        let (u1, d1) = getrennt(&balken); // Balken zuerst (wie ohne Unterlage)
        let (u2, d2) = getrennt(&bein);
        assert!(!u1.is_empty() && !u2.is_empty());
        let (iu2, iu1, id1, id2) = (
            stelle(&wege, u2[0]),
            stelle(&wege, u1[0]),
            stelle(&wege, d1[0]),
            stelle(&wege, d2[1]),
        );
        assert!(
            iu2 < iu1 && iu1 < id1 && id1 < id2,
            "Folge U2 {iu2}, U1 {iu1}, S1 {id1}, S2 {id2}"
        );
        assert_eq!(wege.len(), 1, "Laufwege unter den Säulen, kein Sprung");

        // getrennte Teile (Punkt und Stamm des „i“): je Teil Unterlage, dann Säule
        let (stamm, u_stamm) = saeule(0.0, 1.0, 0.0, 6.0, false);
        let (punkt, u_punkt) = saeule(0.0, 1.0, 8.0, 9.5, false);
        let (stamm, punkt) = (mit_unterlage(stamm), mit_unterlage(punkt));
        let wege = gruppe_wege(
            &[stamm.clone(), punkt.clone()],
            &[u_stamm, u_punkt],
            3.0,
            &e,
            Some(p(0.5, 0.0)),
        )
        .unwrap();
        let (us, ds) = getrennt(&stamm);
        let (up, dp) = getrennt(&punkt);
        let (a, b) = (
            stelle(&wege, us[0]).min(stelle(&wege, up[0])),
            stelle(&wege, ds[1]).min(stelle(&wege, dp[1])),
        );
        assert!(a < b);
        // die erste Säule folgt direkt auf ihre eigene Unterlage, nicht auf die des anderen Teils
        let erst_stamm = stelle(&wege, us[0]) < stelle(&wege, up[0]);
        let (u_erst, d_erst, u_zweit) = if erst_stamm { (&us, &ds, &up) } else { (&up, &dp, &us) };
        assert!(stelle(&wege, u_erst[0]) < stelle(&wege, d_erst[1]));
        assert!(
            stelle(&wege, d_erst[1]) < stelle(&wege, u_zweit[0]),
            "getrennte Teile nacheinander"
        );
    }

    /// Double Run: Lauf wird rückwärts wiederholt und endet am Anfang; Satinlinie mit Versatz liegt links der Linie
    #[test]
    fn doppellauf_und_satin_versatz() {
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "lauf", "punkte": [[0, 0], [10, 0]], "laenge": 2.5, "doppellauf": true}]}"##,
        )
        .unwrap();
        let st = d.berechnen().unwrap();
        let xs: Vec<f64> = st.stiche.iter().map(|s| s.p().x).collect();
        // hin 0 … 10, zurück … 0 (danach Sprung zum Ende in der Rahmenmitte = 0)
        assert!(xs.iter().any(|&x| (x - 10.0).abs() < 1e-6), "{xs:?}");
        let n10 = xs.iter().position(|&x| (x - 10.0).abs() < 1e-6).unwrap();
        let hin = xs[..n10].iter().filter(|&&x| x > 0.1 && x < 9.9).count();
        let zurueck = xs[n10 + 1..].iter().filter(|&&x| x > 0.1 && x < 9.9).count();
        assert!(hin >= 3 && zurueck >= 3, "{xs:?}");
        assert!(xs.last().unwrap().abs() < 0.5, "{xs:?}");
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "satinlinie", "punkte": [[0, 0], [20, 0]], "breite": 2, "versatz": "seite",
                "unterlage": "keine"}]}"##,
        )
        .unwrap();
        let ys: Vec<f64> = d.berechnen().unwrap().stiche.iter().map(|s| s.p().y).collect();
        // Säule zwischen y = 0 und 2 (links der Laufrichtung nach +x), Zugausgleich ragt etwas darüber hinaus
        assert!(ys.iter().all(|&y| y > -0.5 && y < 2.5), "{ys:?}");
        assert!(ys.iter().any(|&y| y > 1.9));
        assert!(
            Design::aus_json(r#"{"objekte": [{"art": "satinlinie", "punkte": [[0, 0], [1, 0]], "versatz": "x"}]}"#)
                .is_err()
        );
    }

    /// Auto Jump je Objekt: an (Sprünge), aus (echte Stiche ≤ 12,7 mm), eigene Werte (Sprünge ab 6 mm)
    #[test]
    fn auto_jump_je_objekt() {
        let zaehle = |aj: &str| {
            let d = Design::aus_json(&format!(
                r#"{{"objekte": [{{"art": "satin", "seite_a": [[0, 0], [0, 4]], "seite_b": [[20, 0], [20, 4]],
                    "unterlage": "keine", "abstand": 0.5 {aj}}}]}}"#
            ))
            .unwrap();
            let st = d.berechnen().unwrap();
            let lang = st.stichlaengen().iter().filter(|&&l| l > 12.7 + 1e-6).count();
            (
                st.anzahl(crate::stiche::Art::Sprung),
                lang,
                st.stichlaengen().into_iter().fold(0.0, f64::max),
            )
        };
        // Säule 4 mm lang, Stiche 20 mm quer
        let (sp_an, lang_an, _) = zaehle("");
        let (sp_aus, lang_aus, max_aus) = zaehle(r#", "auto_jump": false"#);
        let (sp_6, _, max_6) = zaehle(r#", "auto_jump": {"max": 6, "sprung": 3}"#);
        assert_eq!((lang_an, lang_aus), (0, 0));
        assert!(sp_an > sp_aus, "an {sp_an}, aus {sp_aus}");
        assert!(max_aus <= 12.7 + 1e-6 && max_aus > 7.0, "{max_aus}");
        assert!(sp_6 > sp_an && max_6 <= 3.0 + 1e-6, "{sp_6} {max_6}");
        assert!(
            Design::aus_json(r#"{"objekte": [{"art": "lauf", "punkte": [[0, 0], [1, 0]], "auto_jump": {"max": 20}}]}"#)
                .is_err()
        );
    }

    /// Fläche mit Winkellinien: Stiche folgen den Linien (längs, wie Satin), Länge ≤ Step-Länge + Rand
    #[test]
    fn wendefuellung_im_design() {
        let d = Design::aus_json(
            r#"{"objekte": [{"art": "tatami", "umriss": {"rechteck": {"mitte": [0, 0], "breite": 30, "hoehe": 10}},
                "unterlage": false, "winkellinien": [[[-10, -6], [-10, 6]], [[10, -6], [0, 6]]]}]}"#,
        )
        .unwrap();
        assert!(matches!(d.objekte[0].art, ObjektArt::Wendefuellung { .. }));
        let st = d.berechnen().unwrap();
        assert!(st.stiche.len() > 300);
        assert!(st.stichlaengen().iter().all(|&l| l <= 4.0 + 0.4 + 1e-6));
        // bei x ≈ −10 senkrecht (wie die erste Linie)
        let pts: Vec<Punkt> = st.stiche.iter().map(|s| s.p()).collect();
        let senkrecht = pts
            .windows(2)
            .filter(|w| (w[0].x + 10.0).abs() < 0.5 && (w[1].x + 10.0).abs() < 0.5 && abstand(w[0], w[1]) > 2.0)
            .count();
        assert!(senkrecht > 3, "{senkrecht}");
        assert!(
            Design::aus_json(
                r#"{"objekte": [{"art": "tatami", "umriss": [[0, 0], [10, 0], [10, 10], [0, 10]],
                "loecher": [[[4, 4], [6, 4], [6, 6]]], "winkellinien": [[[5, -1], [5, 11]]]}]}"#
            )
            .is_err()
        );
    }

    /// Kurzstiche in der Wendefüllung: im engen Bogen werden innere Einstiche zurückgenommen (weniger Einstiche nah
    /// am Innenrand)
    #[test]
    fn wendefuellung_kurzstiche() {
        let bogen = |kurz: bool| {
            Design::aus_json(&format!(
                r#"{{"objekte": [{{"art": "tatami", "unterlage": false, "kurzstiche": {kurz},
                "umriss": [[3, 0], [12, 0], [8.49, 8.49], [0, 12], [0, 3], [2.12, 2.12]],
                "winkellinien": [[[2, 0.3], [13, 0.3]], [[1.5, 1.6], [9, 9.1]], [[0.3, 2], [0.3, 13]]]}}]}}"#
            ))
            .unwrap()
            .berechnen()
            .unwrap()
            .stiche
            .iter()
            .filter(|s| s.art == crate::stiche::Art::Stich && s.x.hypot(s.y) < 3.4)
            .count()
        };
        let (ohne, mit) = (bogen(false), bogen(true));
        assert!(mit < ohne, "ohne {ohne}, mit {mit}");
    }

    /// Zickzack-Deckstich: offener Abstand (Software Defaults 2,0 mm, Stoffe 1,0 mm) – weniger Stiche als Satin
    #[test]
    fn zickzack_deckstich() {
        let zaehle = |stoff: &str, art: &str| {
            Design::aus_json(&format!(
                r#"{{"stoff": "{stoff}", "objekte": [{{"art": "satinlinie", "punkte": [[-20, 0], [20, 0]], "breite": 3,
                    "unterlage": "keine", "stichart": "{art}"}}]}}"#
            ))
            .unwrap()
            .berechnen()
            .unwrap()
            .anzahl(crate::stiche::Art::Stich)
        };
        let (satin, zz, zz_stoff) = (
            zaehle("0 Software Defaults", "satin"),
            zaehle("0 Software Defaults", "zickzack"),
            zaehle("Woven Medium Weight", "zickzack"),
        );
        // 40 mm: Zickzack 2,0 mm → ~40 Einstiche, Stoff 1,0 mm → ~80, Satin deutlich mehr
        assert!((35..60).contains(&zz), "{zz}");
        assert!((70..110).contains(&zz_stoff), "{zz_stoff}");
        assert!(satin > 2 * zz_stoff, "{satin}");
        assert!(Design::aus_json(r#"{"objekte": [{"art": "satin", "seite_a": [[0, 0], [0, 4]], "seite_b": [[2, 0], [2, 4]], "stichart": "x"}]}"#).is_err());
    }

    /// Monogramm → Text mit Diamond2/Diamond3 (Zeichenbelegung aus den Alphabeten)
    #[test]
    fn monogramm_als_text() {
        let d = Design::aus_json(
            r##"{"objekte": [{"art": "monogramm", "initialen": "shp", "rahmen": "raute", "rahmen_farbe": "#aa0000",
                "farbe": "#000000"}, {"art": "monogramm", "initialen": "AB", "rahmen": "zier"}]}"##,
        )
        .unwrap();
        let ObjektArt::Text { alphabet, vorgabe } = &d.objekte[0].art else {
            panic!()
        };
        // S links, H in der Mitte ('!' + 7 = '('), p rechts, Farbwechsel, Raute
        assert_eq!((alphabet.as_str(), vorgabe.text.as_str()), ("Diamond3", "S(p^]"));
        assert_eq!(vorgabe.hoehe, MONOGRAMM_HOEHE_MM);
        assert_eq!(d.buchstaben[&0].farben, vec!["#aa0000".to_string()]);
        let ObjektArt::Text { alphabet, vorgabe } = &d.objekte[1].art else {
            panic!()
        };
        assert_eq!((alphabet.as_str(), vorgabe.text.as_str()), ("Diamond2", "Ab["));
        assert!(Design::aus_json(r#"{"objekte": [{"art": "monogramm", "initialen": "ÄB"}]}"#).is_err());
    }

    /// Satinfläche: ein „T“ aus zwei Balken wird zu drei Satinsäulen (Sternschnitt an der Abzweigung) mit Stichen quer
    /// zu jedem Balken; alle Einstiche in der Fläche (bis Zugausgleich)
    #[test]
    fn satinflaeche_t_form() {
        let json = r##"{"objekte": [{"art": "satinflaeche", "farbe": "#204080", "umriss":
            [[-15, 10], [15, 10], [15, 13], [1.5, 13], [1.5, 13.0001], [-1.5, 13.0001], [-1.5, 13], [-15, 13]]}]}"##;
        // (Balken oben 30 × 3); dazu ein senkrechter Balken nach unten als zweites Objekt wäre keine Abzweigung –
        // deshalb die T-Form als ein Umriss:
        let json = json.replace(
            "[[-15, 10], [15, 10], [15, 13], [1.5, 13], [1.5, 13.0001], [-1.5, 13.0001], [-1.5, 13], [-15, 13]]",
            "[[-15, 10], [-1.5, 10], [-1.5, -15], [1.5, -15], [1.5, 10], [15, 10], [15, 13], [-15, 13]]",
        );
        let d = Design::aus_json(&json).unwrap();
        let (o, _) = d.objekte_aufgeloest().unwrap();
        let ObjektArt::Gruppe { teile, .. } = &o[0].art else {
            panic!("Gruppe erwartet")
        };
        assert_eq!(teile.len(), 3, "drei Säulen");
        let f = d.berechnen().unwrap();
        let st: Vec<Punkt> = f
            .stiche
            .iter()
            .filter(|s| s.art == crate::Art::Stich)
            .map(|s| s.p())
            .collect();
        assert!(st.len() > 200);
        let innen = |q: &Punkt| {
            (q.y >= 10.0 - 0.6 && q.y <= 13.0 + 0.6 && q.x.abs() <= 15.6) || (q.x.abs() <= 1.5 + 0.6 && q.y >= -15.6)
        };
        assert!(st.iter().all(innen), "{:?}", st.iter().find(|q| !innen(q)));
        // im senkrechten Balken laufen die Stiche waagrecht, im waagrechten senkrecht
        let mut senkrecht = 0;
        let mut waagrecht = 0;
        for w in st.windows(2) {
            let (a, b) = (w[0], w[1]);
            let (dx, dy) = ((b.x - a.x).abs(), (b.y - a.y).abs());
            if a.y < 5.0 && b.y < 5.0 && abstand(a, b) > 2.5 {
                waagrecht += usize::from(dx > 3.0 * dy);
            }
            if a.x.abs() > 5.0 && b.x.abs() > 5.0 && abstand(a, b) > 2.5 {
                senkrecht += usize::from(dy > 3.0 * dx);
            }
        }
        assert!(waagrecht > 20 && senkrecht > 40, "{waagrecht} {senkrecht}");
        assert!(Design::aus_json(&json.replace(r#""farbe""#, r#""knickwinkel": 200, "farbe""#)).is_err());
    }

    /// Sprungweg gesamt (mm) und Stichpunkte einer Folge
    fn spruenge_und_stiche(d: &Design) -> (f64, Vec<Punkt>) {
        let f = d.berechnen().unwrap();
        let mut weg = 0.0;
        for w in f.stiche.windows(2) {
            if w[1].art == crate::Art::Sprung {
                weg += abstand(w[0].p(), w[1].p());
            }
        }
        let st = f
            .stiche
            .iter()
            .filter(|s| s.art == crate::Art::Stich)
            .map(|s| s.p())
            .collect();
        (weg, st)
    }

    /// Ein-/Ausstich je Objekt und Closest Join (ROADMAP 4b.5, `ein_ausstich`)
    #[test]
    fn ein_ausstich_und_kuerzeste_verbindung() {
        let json = r##"{"ende": "letzter_stich", "objekte": [
            {"art": "lauf", "punkte": [[0, 0], [20, 0]]},
            {"art": "lauf", "punkte": [[50, 0], [25, 0]]},
            {"art": "lauf", "geschlossen": true, "punkte": [[30, 10], [40, 10], [40, 20], [30, 20]]}
        ]}"##;
        let (weg, st) = spruenge_und_stiche(&Design::aus_json(json).unwrap());
        // wie gezeichnet: 0 → 20, Sprung nach 50, zurück bis 25, Sprung zur Ecke (30, 10)
        assert!(st.iter().any(|q| abstand(*q, p(50.0, 0.0)) < 0.01));
        let (weg_k, st_k) = spruenge_und_stiche(
            &Design::aus_json(&json.replace(r#""objekte""#, r#""kuerzeste_verbindung": true, "objekte""#)).unwrap(),
        );
        assert!(
            weg_k < weg - 20.0,
            "Closest Join spart den Weg nach 50 und zurück: {weg} → {weg_k}"
        );
        // zweite Linie jetzt von 25 nach 50, das Quadrat beginnt an seiner nächsten Stelle zu (50, 0): Ecke (40, 10)
        let i25 = st_k.iter().position(|q| abstand(*q, p(25.0, 0.0)) < 0.01).unwrap();
        let i50 = st_k.iter().position(|q| abstand(*q, p(50.0, 0.0)) < 0.01).unwrap();
        assert!(i25 < i50);
        let quadrat = *st_k[i50..].iter().find(|q| q.y > 5.0).unwrap();
        assert!(abstand(quadrat, p(40.0, 10.0)) < 3.0, "{quadrat:?}");
        // eigener Einstich am Quadrat (rechte Kante oben) und Ausstich der ersten Linie am Anfang
        let eigen = json
            .replace(
                r#""geschlossen": true,"#,
                r#""geschlossen": true, "einstich": [41, 18],"#,
            )
            .replace(r#"[[0, 0], [20, 0]]}"#, r#"[[0, 0], [20, 0]], "ausstich": [-1, 0]}"#);
        let (_, st_e) = spruenge_und_stiche(&Design::aus_json(&eigen).unwrap());
        assert!(
            abstand(st_e[0], p(20.0, 0.0)) < 0.01,
            "erste Linie rückwärts: {:?}",
            st_e[0]
        );
        assert!(
            abstand(*st_e.last().unwrap(), p(40.0, 18.0)) < 0.01,
            "Quadrat endet am Einstich"
        );
        assert!(
            Design::aus_json(&json.replace(r#""punkte": [[0, 0]"#, r#""einstich": 5, "punkte": [[0, 0]"#)).is_err()
        );
    }

    /// „Start and End“: Auto Start/End in der Designmitte wie die Musterdesigns mit AutoStartEndObject
    #[test]
    fn start_und_ende_designmitte() {
        let json = r##"{"rahmen": "artista 130 x 100", "start": "designmitte", "ende": "designmitte", "objekte": [
            {"art": "tatami", "umriss": {"rechteck": {"mitte": [20, 10], "breite": 10, "hoehe": 6}}}
        ]}"##;
        let d = Design::aus_json(json).unwrap();
        let f = d.berechnen().unwrap();
        let st: Vec<Punkt> = f
            .stiche
            .iter()
            .filter(|s| s.art == crate::Art::Stich)
            .map(|s| s.p())
            .collect();
        let (x0, x1) = st
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let (y0, y1) = st
            .iter()
            .fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.y), b.max(q.y)));
        let ende = f.stiche.last().unwrap().p();
        assert!(
            abstand(ende, p((x0 + x1) / 2.0, (y0 + y1) / 2.0)) < 0.3,
            "Ende {ende:?}"
        );
        // letzter Stich: kein Sprung am Ende
        let l = Design::aus_json(&json.replace(r#""ende": "designmitte""#, r#""ende": "letzter_stich""#)).unwrap();
        assert_eq!(l.berechnen().unwrap().stiche.last().unwrap().art, crate::Art::Stich);
        assert!(Design::aus_json(&json.replace("designmitte\", \"ende", "irgendwo\", \"ende")).is_err());
    }

    /// Objektnummer je Stich und Formen je Objekt (Schnittstelle für die Oberfläche, ROADMAP 4b.0)
    #[test]
    fn objektnummer_und_formen() {
        let d = Design::aus_json(
            r##"{"objekte": [
                {"art": "lauf", "punkte": [[-20, 0], [-10, 0]]},
                {"art": "tatami", "umriss": {"rechteck": {"mitte": [10, 0], "breite": 10, "hoehe": 8}},
                 "loecher": [{"kreis": {"mitte": [10, 0], "radius": 2}}], "farbe": "#ff0000"},
                {"art": "satin", "seite_a": [[0, 20], [10, 20]], "seite_b": [[0, 22], [10, 22]], "einstich": [10, 21]}
            ]}"##,
        )
        .unwrap();
        let f = d.berechnen().unwrap();
        assert_eq!(f.objekt.len(), f.stiche.len());
        assert!(f.objekt.windows(2).all(|w| w[0] <= w[1]), "Objekte in Stickfolge");
        for nr in 0..3 {
            assert!(f.objekt.contains(&nr), "Objekt {nr} hat Stiche");
        }
        // Stiche des Lauf-Objekts liegen auf seiner Linie
        for (s, &o) in f.stiche.iter().zip(&f.objekt) {
            if o == 0 && s.art == crate::Art::Stich {
                assert!(s.y.abs() < 1e-6 && s.x <= -9.9, "{s:?}");
            }
        }
        let formen = d.formen().unwrap();
        assert_eq!(formen.len(), 3);
        assert_eq!(
            formen[1].linien.iter().map(|l| l.art).collect::<Vec<_>>(),
            [FormArt::Umriss, FormArt::Loch]
        );
        assert_eq!(formen[2].linien[0].art, FormArt::SeiteA);
        assert_eq!(formen[2].einstich, Some(p(10.0, 21.0)));
        let b = formen[1].begrenzung.unwrap();
        assert!((b[0] - 5.0).abs() < 1e-9 && (b[3] - 4.0).abs() < 1e-9, "{b:?}");
    }

    /// Werte je Objekt (ROADMAP 4b.16): Stil-Werte am einzelnen Objekt überschreiben – nur dieses Objekt ändert sich
    #[test]
    fn werte_je_objekt() {
        let flaeche = |extra: &str| {
            format!(
                r##"{{"art": "tatami", "umriss": {{"rechteck": {{"mitte": [0, 0], "breite": 20, "hoehe": 20}}}},
                    "unterlage": false{extra}}}"##
            )
        };
        let zwei = |a: &str, b: &str| {
            Design::aus_json(&format!(
                r##"{{"objekte": [{}, {}]}}"##,
                flaeche(a),
                flaeche(&format!(r#", "transform": {{"verschieben": [30, 0]}}{b}"#))
            ))
            .unwrap()
        };
        let stiche_je_objekt = |d: &Design| {
            let f = d.berechnen().unwrap();
            let mut n = [0usize; 2];
            for (s, &o) in f.stiche.iter().zip(&f.objekt) {
                if s.art == crate::Art::Stich {
                    n[o] += 1;
                }
            }
            n
        };
        let gleich = stiche_je_objekt(&zwei("", ""));
        let d = zwei("", r#", "einstellungen": {"tatami": {"abstand": 0.8}}"#);
        assert!((d.einstellungen_fuer(1).tatami.abstand - 0.8).abs() < 1e-12);
        assert!((d.einstellungen_fuer(0).tatami.abstand - d.einstellungen.tatami.abstand).abs() < 1e-12);
        let locker = stiche_je_objekt(&d);
        assert_eq!(locker[0], gleich[0], "erstes Objekt unverändert");
        assert!(
            (locker[1] as f64) < gleich[1] as f64 * 0.7,
            "doppelter Abstand → deutlich weniger Stiche: {locker:?} gegen {gleich:?}"
        );

        // Reihenfolge Stoff → Design → Objekt: Design-Wert gilt, wo das Objekt nichts sagt
        let d = Design::aus_json(&format!(
            r##"{{"stoff": "Woven Heavy Weight", "einstellungen": {{"tatami": {{"laenge": 3.0}}}},
                "objekte": [{}]}}"##,
            flaeche(r#", "einstellungen": {"tatami": {"abstand": 0.6}}"#)
        ))
        .unwrap();
        let e = d.einstellungen_fuer(0);
        assert!((e.tatami.abstand - 0.6).abs() < 1e-12 && (e.tatami.laenge - 3.0).abs() < 1e-12);
        assert_eq!(e.stoff, "Woven Heavy Weight");

        // Verbinder je Objekt: ohne Vernähen am Anfang hat das zweite Objekt weniger Stiche
        let ohne = r#", "einstellungen": {"verbinder": {"anfang_immer": false, "anfang_nach_trim_farbwechsel": false,
            "anfang_bei_langem_verbinder": false}}"#;
        let n = stiche_je_objekt(&zwei("", ohne));
        assert!(n[1] < gleich[1], "{n:?} gegen {gleich:?}");

        // Fehler mit Objektnummer; Stoff nur am Design
        let e = Design::aus_json(&format!(
            r#"{{"objekte": [{}]}}"#,
            flaeche(r#", "einstellungen": {"tatami": {"abstnd": 1}}"#)
        ))
        .unwrap_err();
        assert!(
            e.starts_with("Objekt 1: einstellungen:") && e.contains("tatami.abstnd"),
            "{e}"
        );
        let e = Design::aus_json(&format!(
            r#"{{"objekte": [{}]}}"#,
            flaeche(r#", "einstellungen": {"stoff": "x"}"#)
        ))
        .unwrap_err();
        assert!(e.contains("stoff"), "{e}");
    }
}
