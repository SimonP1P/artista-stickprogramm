//! Bild → Kreuzstich wie „Auto Stitch Image“ des Kreuzstich-Programms (`BLOT.EXE`, Formular
//! `TAutoStitchImageForm`; `Analyse_Teil3/29` §3): das Bild wird auf das Raster verkleinert (ein Bildpunkt = ein
//! Kreuz), die Farben werden reduziert („Maximum Number of Colours“: Restricted 2 … 255) und auf die Palette bzw. eine
//! Garnkarte gezogen („Map Image To: Current Palette / Specific Thread Chart“), der Hintergrund kann wegfallen („Omit
//! Background“). Ergebnis: je Farbe ein Kreuzstich-Objekt mit seinen Zellen.

use crate::befehle::{BildFarbe, BildVorschau, DesignWert, GarnInfo, base64, hex, hex_farbe};
use crate::{Fehler, Kontext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use stick_formate::garnkarte;
use stick_kern::format::KreuzArt;
use stick_kern::sticharten::kreuzstich as ks;

/// Eingabe von `kreuzstich_vorschau` und `kreuzstich_bild`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KreuzstichBildEingabe {
    /// Bilddatei (PNG, JPG, BMP, GIF)
    pub pfad: String,
    /// Breite des Ergebnisses in mm (Standard 80; die Höhe folgt dem Bild)
    #[serde(default)]
    pub breite: Option<f64>,
    /// Kreuzgröße (Rasterweite) in mm (Standard 2,54 = 10 Kreuze je Zoll)
    #[serde(default)]
    pub groesse: Option<f64>,
    /// höchstens so viele Farben (2 … 64; Original „Restricted“ 2 … 255; Standard 8)
    #[serde(default)]
    pub farben: Option<usize>,
    /// Farben, die nicht gestickt werden („#rrggbb“ aus der Vorschau); ohne Angabe: Vorschlag (weißer Hintergrund –
    /// „Omit Background“); `[]` = alles sticken
    #[serde(default)]
    pub weglassen: Option<Vec<String>>,
    /// Garnkarte („Specific Thread Chart“)
    #[serde(default)]
    pub garnkarte: Option<String>,
    /// Farben auf die Garne der Karte ziehen (Standard an, wenn eine Garnkarte angegeben ist)
    #[serde(default)]
    pub garnfarben: Option<bool>,
    /// Art der Kreuze (Standard voll)
    #[serde(default)]
    pub kreuz: Option<KreuzArt>,
    /// Fäden 2, 4, 6 (Standard 2)
    #[serde(default)]
    pub faeden: Option<u8>,
    /// Rahmen des neuen Designs
    #[serde(default)]
    pub rahmen: Option<String>,
}

/// Höchstzahl der Farben (eigene Festlegung; das Original erlaubt bis 255)
const FARBEN_MAX: usize = 64;
/// Höchstzahl der Spalten/Reihen (eigene Festlegung: 500 × 2,54 mm = 1,27 m)
const ZELLEN_MAX: u32 = 500;

struct Raster {
    v: stick_vektor::Vorbereitet,
    weg: Vec<usize>,
    karte: Option<garnkarte::Garnkarte>,
    mit_palette: bool,
    raster: f64,
}

fn vorbereiten(k: &Kontext, e: &KreuzstichBildEingabe) -> Result<Raster, Fehler> {
    let raster = e.groesse.unwrap_or(ks::RASTER);
    if !(ks::RASTER_MIN..=ks::RASTER_MAX).contains(&raster) {
        return Err(format!("groesse: {raster} mm außerhalb {} … {}", ks::RASTER_MIN, ks::RASTER_MAX).into());
    }
    let breite = e.breite.unwrap_or(80.0);
    if !(5.0..=1000.0).contains(&breite) {
        return Err(format!("breite: {breite} mm außerhalb 5 … 1000").into());
    }
    let bild = stick_vektor::bild_laden(&PathBuf::from(&e.pfad))?;
    let spalten = ((breite / raster).round() as u32).clamp(2, ZELLEN_MAX);
    let reihen = ((f64::from(spalten) * f64::from(bild.height()) / f64::from(bild.width().max(1))).round() as u32)
        .clamp(1, ZELLEN_MAX);
    let klein = stick_vektor::auf_groesse(&bild, spalten, reihen);
    let karte = match e.garnkarte.as_deref() {
        None | Some("") | Some("keine") => None,
        Some(n) => Some(garnkarte::laden(&k.garnkarten, n)?),
    };
    let mit_palette = karte.is_some() && e.garnfarben.unwrap_or(true);
    let weglassen = match &e.weglassen {
        Some(l) => Some(l.iter().map(|t| hex_farbe(t)).collect::<Result<Vec<_>, _>>()?),
        None => None,
    };
    let o = stick_vektor::BildOptionen {
        breite_mm: f64::from(spalten) * raster,
        farben: e.farben.unwrap_or(8).clamp(2, FARBEN_MAX),
        // jedes Kreuz zählt: keine Mindestfläche, kein Glätten
        min_flaeche: 0.0,
        glaettung: 0.0,
        vereinfachen: 0,
        weglassen,
        palette: karte.as_ref().filter(|_| mit_palette).map(|k| k.garne.iter().map(|g| g.rgb).collect()),
        fuellung: Default::default(),
    };
    let v = stick_vektor::vorbereiten(&klein, &o);
    let weg = v.weggelassen(&o);
    Ok(Raster {
        v,
        weg,
        karte,
        mit_palette,
        raster,
    })
}

/// Vorschau: das Bild auf dem Raster (ein Bildpunkt je Kreuz) mit den reduzierten Farben und der Farbliste.
pub fn kreuzstich_vorschau(k: &Kontext, e: KreuzstichBildEingabe) -> Result<BildVorschau, Fehler> {
    let r = vorbereiten(k, &e)?;
    let summe = r.v.punkte.iter().sum::<usize>().max(1) as f64;
    Ok(BildVorschau {
        bild: format!("data:image/png;base64,{}", base64(&r.v.png())),
        breite_px: r.v.breite as u32,
        hoehe_px: r.v.hoehe as u32,
        farben: r
            .v
            .farben
            .iter()
            .enumerate()
            .map(|(i, c)| BildFarbe {
                farbe: hex(*c),
                anteil: (1000.0 * r.v.punkte[i] as f64 / summe).round() / 10.0,
                weglassen: r.weg.contains(&i),
                garn: r.karte.as_ref().map(|kt| GarnInfo::from(kt.naechstes(*c))),
            })
            .collect(),
    })
}

/// Ergebnis von `kreuzstich_bild`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct KreuzstichBildErgebnis {
    /// neues Design: je Farbe ein Kreuzstich-Objekt (`zellen`), Mitte des Bildes auf der Rahmenmitte
    pub design: DesignWert,
    pub spalten: usize,
    pub reihen: usize,
    /// Zahl der Kreuze
    pub kreuze: usize,
    pub farben: usize,
    /// Größe in mm
    pub breite: f64,
    pub hoehe: f64,
}

/// Bild → Kreuzstich-Design.
pub fn kreuzstich_bild(k: &Kontext, e: KreuzstichBildEingabe) -> Result<KreuzstichBildErgebnis, Fehler> {
    let r = vorbereiten(k, &e)?;
    let faeden = e.faeden.unwrap_or(2);
    if ![2, 4, 6].contains(&faeden) {
        return Err(format!("faeden: {faeden} – erlaubt 2, 4 oder 6").into());
    }
    let (w, h) = (r.v.breite as i32, r.v.hoehe as i32);
    let mut zellen: Vec<Vec<[i32; 2]>> = vec![Vec::new(); r.v.farben.len()];
    for (n, z) in r.v.zuordnung.iter().enumerate() {
        if let Some(c) = z
            && !r.weg.contains(c)
        {
            let (x, y) = (n as i32 % w, n as i32 / w);
            // Mitte des Bildes auf 0,0; Reihe 0 unten (+y oben)
            zellen[*c].push([x - w / 2, (h - 1 - y) - h / 2]);
        }
    }
    let stamm = PathBuf::from(&e.pfad).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let kreuz = e.kreuz.map(|a| a.text()).unwrap_or("voll");
    let mut objekte = Vec::new();
    let mut kreuze = 0;
    for (i, z) in zellen.into_iter().enumerate() {
        if z.is_empty() {
            continue;
        }
        kreuze += z.len();
        let farbe = r.v.farben[i];
        let mut o = json!({
            "art": "kreuzstich", "name": format!("Kreuzstich {}", objekte.len() + 1), "farbe": hex(farbe),
            "zellen": z, "groesse": r.raster, "kreuz": kreuz, "faeden": faeden,
        });
        if r.mit_palette
            && let Some(g) = r.karte.as_ref().and_then(|kt| kt.garne.iter().find(|g| g.rgb == farbe))
        {
            o["garn"] = Value::from(g.nummer.clone());
        }
        objekte.push(o);
    }
    if objekte.is_empty() {
        return Err("Alle Farben sind weggelassen – nichts zu sticken".into());
    }
    let farben = objekte.len();
    let mut d = json!({"name": stamm, "objekte": objekte});
    if let Some(rh) = &e.rahmen {
        d["rahmen"] = Value::from(rh.clone());
    }
    stick_kern::Design::aus_value(&d).map_err(|x| Fehler::from(format!("Ergebnis: {x}")))?;
    Ok(KreuzstichBildErgebnis {
        design: DesignWert(d),
        spalten: w as usize,
        reihen: h as usize,
        kreuze,
        farben,
        breite: f64::from(w) * r.raster,
        hoehe: f64::from(h) * r.raster,
    })
}
