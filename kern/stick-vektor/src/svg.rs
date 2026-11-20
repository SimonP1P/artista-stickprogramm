//! **SVG → Design** (ROADMAP 4b.17): in Inkscape (oder einem anderen Vektorprogramm) zeichnen, laden, sticken.
//!
//! Ablauf:
//! 1. SVG lesen mit der freien Bibliothek `usvg` (neutrale Technik): Gruppen, Transformationen, CSS/Stile, `use`,
//!    Grundformen → Pfade; Kurven werden auf 0,02 mm genau in Linien zerlegt.
//! 2. Maßstab: 1 px = 25,4/96 mm wie in SVG festgelegt (Inkscape mit `width="…mm"` und `viewBox` ergibt die echte
//!    Größe); auf Wunsch eine feste Breite. Die Mitte der Zeichnung liegt in der Rahmenmitte, +y nach oben.
//! 3. Gefüllte Pfade → Flächen mit Löchern nach der Füllregel der SVG (`nonzero`/`evenodd`); Konturen (`stroke`) →
//!    Linien mit ihrer Strichbreite.
//! 4. **Überdeckung:** In der SVG verdeckt, was später gezeichnet ist. Deshalb spart jede Füllfläche die Flächen aus,
//!    die später gezeichnet werden (Linien sparen nichts aus) – **eigene Festlegung** nach der Bedeutung von SVG,
//!    wie „Remove Overlay Stitching“ des Originals exakt ohne Überlappung (abschaltbar).
//! 5. Stichart nach Breite wie **EasyStitch** des Originals (`easystitch.rs`, `Analyse/code/10` §C).
//! 6. Reihenfolge = Zeichenreihenfolge (je Pfad erst Füllung, dann Kontur wie in SVG); dazu „Closest Join“ wie der
//!    Auto Digitizer des Originals (Handbuch PDF-S. 207). Farben bleiben als Farbwerte; das Garn ordnet die Garnkarte
//!    beim Berechnen zu.
//!
//! Text muss in Inkscape in Pfade umgewandelt sein (Pfad → Objekt in Pfad); Bilder werden übergangen. Verläufe und
//! Muster nehmen die Farbe ihres mittleren Farbstopps.

use crate::easystitch::{self, Klasse};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use stick_kern::aussparen::{Kontur, Teil, flaeche_minus, flaechen_aus_ringen, teile_konturen};
use stick_kern::geometrie::{Linie, p};
use usvg::tiny_skia_path::PathSegment;

/// Einstellungen des SVG-Imports.
#[derive(Clone, Debug)]
pub struct SvgOptionen {
    /// Breite des Ergebnisses in mm (None = Größe aus der SVG-Datei)
    pub breite_mm: Option<f64>,
    /// Füllflächen sparen aus, was später gezeichnet ist (wie die SVG aussieht)
    pub aussparen: bool,
    /// Closest Join wie der Auto Digitizer
    pub kuerzeste_verbindung: bool,
    /// Rahmen des neuen Designs
    pub rahmen: String,
}

impl Default for SvgOptionen {
    fn default() -> Self {
        SvgOptionen {
            breite_mm: None,
            aussparen: true,
            kuerzeste_verbindung: true,
            rahmen: "artista 130 x 100".into(),
        }
    }
}

/// Ergebnis des Imports.
#[derive(Clone, Debug)]
pub struct SvgErgebnis {
    /// neues Design (Format `stick_kern::format`)
    pub design: Value,
    /// Größe der Zeichnung in mm
    pub breite_mm: f64,
    pub hoehe_mm: f64,
    /// Zahl der Objekte je Stichart-Klasse (Namen wie die EasyStitch-Stile des Originals)
    pub klassen: BTreeMap<String, usize>,
    /// Hinweise (Text, Bilder, Verläufe, zu breite Linien, zu große Zeichnung …)
    pub hinweise: Vec<String>,
}

/// Millimeter je SVG-Pixel (CSS: 96 px je Zoll).
pub const MM_JE_PX: f64 = 25.4 / 96.0;
/// Genauigkeit beim Zerlegen der Kurven in mm.
const KURVE_TOLERANZ_MM: f64 = 0.02;

/// Ein gezeichneter Pfad in Pixeln: Unterpfade (Punkte, geschlossen), Füllung, Kontur.
struct Element {
    id: String,
    /// Farbe und Füllregel „evenodd“
    fuellung: Option<(String, bool)>,
    /// Farbe und Strichbreite in px
    kontur: Option<(String, f64)>,
    zuege: Vec<(Vec<[f64; 2]>, bool)>,
}

fn hex(c: usvg::Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.red, c.green, c.blue)
}

fn farbe(paint: &usvg::Paint, hinweise: &mut Vec<String>) -> String {
    let mitte = |stops: &[usvg::Stop]| stops.get(stops.len() / 2).map(|s| hex(s.color()));
    let f = match paint {
        usvg::Paint::Color(c) => Some(hex(*c)),
        usvg::Paint::LinearGradient(g) => mitte(g.stops()),
        usvg::Paint::RadialGradient(g) => mitte(g.stops()),
        usvg::Paint::Pattern(_) => None,
    };
    if !matches!(paint, usvg::Paint::Color(_)) {
        let h = "Verläufe/Muster: es gilt die Farbe des mittleren Farbstopps".to_string();
        if !hinweise.contains(&h) {
            hinweise.push(h);
        }
    }
    f.unwrap_or_else(|| "#000000".into())
}

/// Kubische Bézierkurve in Linien zerlegen (rekursiv halbieren, bis die Kontrollpunkte nahe an der Sehne liegen).
fn bezier_flach(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2], tol: f64, tiefe: u32, out: &mut Vec<[f64; 2]>) {
    let abst = |q: [f64; 2]| {
        let (dx, dy) = (d[0] - a[0], d[1] - a[1]);
        let l = dx.hypot(dy);
        if l < 1e-12 {
            (q[0] - a[0]).hypot(q[1] - a[1])
        } else {
            ((q[0] - a[0]) * dy - (q[1] - a[1]) * dx).abs() / l
        }
    };
    if tiefe >= 18 || (abst(b) <= tol && abst(c) <= tol) {
        out.push(d);
        return;
    }
    let m = |u: [f64; 2], v: [f64; 2]| [(u[0] + v[0]) / 2.0, (u[1] + v[1]) / 2.0];
    let (ab, bc, cd) = (m(a, b), m(b, c), m(c, d));
    let (abc, bcd) = (m(ab, bc), m(bc, cd));
    let mitte = m(abc, bcd);
    bezier_flach(a, ab, abc, mitte, tol, tiefe + 1, out);
    bezier_flach(mitte, bcd, cd, d, tol, tiefe + 1, out);
}

/// Pfad (mit Transformation) in Unterpfade aus Punkten zerlegen (px).
fn zuege(pfad: &usvg::Path, tol_px: f64) -> Vec<(Vec<[f64; 2]>, bool)> {
    let t = pfad.abs_transform();
    let tp = |q: usvg::tiny_skia_path::Point| {
        let (x, y) = (f64::from(q.x), f64::from(q.y));
        [
            f64::from(t.sx) * x + f64::from(t.kx) * y + f64::from(t.tx),
            f64::from(t.ky) * x + f64::from(t.sy) * y + f64::from(t.ty),
        ]
    };
    let mut out: Vec<(Vec<[f64; 2]>, bool)> = Vec::new();
    let mut akt: Vec<[f64; 2]> = Vec::new();
    for s in pfad.data().segments() {
        match s {
            PathSegment::MoveTo(q) => {
                if akt.len() >= 2 {
                    out.push((std::mem::take(&mut akt), false));
                }
                akt = vec![tp(q)];
            }
            PathSegment::LineTo(q) => akt.push(tp(q)),
            PathSegment::QuadTo(c, q) => {
                let a = *akt.last().unwrap_or(&tp(c));
                let (c, d) = (tp(c), tp(q));
                // quadratisch → kubisch
                let b1 = [a[0] + 2.0 / 3.0 * (c[0] - a[0]), a[1] + 2.0 / 3.0 * (c[1] - a[1])];
                let b2 = [d[0] + 2.0 / 3.0 * (c[0] - d[0]), d[1] + 2.0 / 3.0 * (c[1] - d[1])];
                bezier_flach(a, b1, b2, d, tol_px, 0, &mut akt);
            }
            PathSegment::CubicTo(c1, c2, q) => {
                let a = *akt.last().unwrap_or(&tp(c1));
                bezier_flach(a, tp(c1), tp(c2), tp(q), tol_px, 0, &mut akt);
            }
            PathSegment::Close => {
                if akt.len() >= 2 {
                    let erster = akt[0];
                    out.push((std::mem::take(&mut akt), true));
                    akt = vec![erster];
                }
            }
        }
    }
    if akt.len() >= 2 {
        out.push((akt, false));
    }
    out
}

fn sammeln(g: &usvg::Group, tol_px: f64, out: &mut Vec<Element>, hinweise: &mut Vec<String>) {
    for n in g.children() {
        match n {
            usvg::Node::Group(gr) => sammeln(gr, tol_px, out, hinweise),
            usvg::Node::Path(pfad) => {
                if !pfad.is_visible() {
                    continue;
                }
                let fuellung = pfad
                    .fill()
                    .filter(|f| f.opacity().get() > 0.0)
                    .map(|f| (farbe(f.paint(), hinweise), f.rule() == usvg::FillRule::EvenOdd));
                let t = pfad.abs_transform();
                let massstab = (f64::from(t.sx) * f64::from(t.sy) - f64::from(t.kx) * f64::from(t.ky))
                    .abs()
                    .sqrt();
                let kontur = pfad
                    .stroke()
                    .filter(|s| s.opacity().get() > 0.0)
                    .map(|s| (farbe(s.paint(), hinweise), f64::from(s.width().get()) * massstab));
                if fuellung.is_none() && kontur.is_none() {
                    continue;
                }
                out.push(Element {
                    id: pfad.id().to_string(),
                    fuellung,
                    kontur,
                    zuege: zuege(pfad, tol_px),
                });
            }
            usvg::Node::Image(_) => {
                let h = "Bilder in der SVG werden übergangen (Bild → Umrisse: Befehl bild_umwandeln)".to_string();
                if !hinweise.contains(&h) {
                    hinweise.push(h);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }
}

/// SVG-Daten in ein Design umwandeln.
pub fn svg_lesen(daten: &[u8], name: &str, o: &SvgOptionen) -> Result<SvgErgebnis, String> {
    let baum = usvg::Tree::from_data(daten, &usvg::Options::default()).map_err(|e| format!("SVG ungültig: {e}"))?;
    let mut hinweise = Vec::new();
    if String::from_utf8_lossy(daten).contains("<text") {
        hinweise.push(
            "Text wird übergangen – in Inkscape vorher in Pfade umwandeln (Pfad → Objekt in Pfad umwandeln)".into(),
        );
    }
    let groesse = baum.size();
    let tol_grob = f64::from(groesse.width().max(groesse.height())) / 20_000.0;
    let mut elemente = Vec::new();
    sammeln(baum.root(), tol_grob.max(1e-3), &mut elemente, &mut hinweise);

    // Größe der Zeichnung (px) → Maßstab und Mitte
    let alle = elemente.iter().flat_map(|e| e.zuege.iter().flat_map(|(z, _)| z.iter()));
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for q in alle {
        (x0, y0, x1, y1) = (x0.min(q[0]), y0.min(q[1]), x1.max(q[0]), y1.max(q[1]));
    }
    if x0 > x1 {
        return Err("Die SVG enthält keine sichtbaren Pfade mit Füllung oder Kontur".into());
    }
    let k = match o.breite_mm {
        Some(b) if b > 0.0 && x1 > x0 => b / (x1 - x0),
        Some(_) => return Err("breite muss größer als 0 sein".into()),
        None => MM_JE_PX,
    };
    // feinere Zerlegung, wenn die grobe Toleranz im Ergebnis über 0,02 mm läge
    if tol_grob * k > KURVE_TOLERANZ_MM {
        elemente.clear();
        hinweise.retain(|h| !h.starts_with("Verläufe") && !h.starts_with("Bilder"));
        sammeln(baum.root(), KURVE_TOLERANZ_MM / k, &mut elemente, &mut hinweise);
    }
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    let mm = |q: &[f64; 2]| p((q[0] - cx) * k, -(q[1] - cy) * k);
    let (breite_mm, hoehe_mm) = ((x1 - x0) * k, (y1 - y0) * k);

    // Flächen je Element (mm), dann Überdeckung: was später gezeichnet ist, liegt oben
    let mut flaechen: Vec<Vec<Teil>> = elemente
        .iter()
        .map(|e| match &e.fuellung {
            Some((_, gerade_ungerade)) => {
                let ringe: Vec<Linie> = e.zuege.iter().map(|(z, _)| z.iter().map(mm).collect()).collect();
                flaechen_aus_ringen(&ringe, *gerade_ungerade)
            }
            None => Vec::new(),
        })
        .collect();
    if o.aussparen {
        let mut oben: Vec<Kontur> = Vec::new();
        for f in flaechen.iter_mut().rev() {
            let ganz = teile_konturen(f);
            if !oben.is_empty() && !f.is_empty() {
                *f = f.iter().flat_map(|(u, l)| flaeche_minus(u, l, &oben)).collect();
            }
            oben.extend(ganz);
        }
    }

    let mut objekte: Vec<Value> = Vec::new();
    let mut klassen: BTreeMap<String, usize> = BTreeMap::new();
    let mut zu_klein = 0;
    let mut zu_breit = 0;
    for (e, teile) in elemente.iter().zip(&flaechen) {
        if let Some((f, _)) = &e.fuellung {
            for (u, l) in teile {
                let (obj, klasse) = easystitch::flaeche_objekte(u, l, f, &e.id)?;
                match klasse {
                    Some(c) => *klassen.entry(c.name().into()).or_default() += obj.len(),
                    None => zu_klein += 1,
                }
                objekte.extend(obj);
            }
        }
        if let Some((f, w_px)) = &e.kontur {
            let w = w_px * k;
            for (z, geschlossen) in &e.zuege {
                let punkte: Linie = z.iter().map(mm).collect();
                let (obj, klasse) = easystitch::linie_objekte(&punkte, *geschlossen, w, f, &e.id);
                if w > easystitch::SATIN_BIS_MM {
                    zu_breit += 1;
                }
                let schluessel = match klasse {
                    Klasse::DuenneLinie | Klasse::KraeftigeLinie => klasse.name().to_string(),
                    _ => "Satinlinie (Kontur)".to_string(),
                };
                *klassen.entry(schluessel).or_default() += 1;
                objekte.push(obj);
            }
        }
    }
    if zu_klein > 0 {
        hinweise.push(format!("{zu_klein} Flächen unter 1 mm² entfallen"));
    }
    if zu_breit > 0 {
        hinweise.push(format!(
            "{zu_breit} Konturen sind breiter als 8 mm und werden Satinlinien – besser als Fläche zeichnen"
        ));
    }
    if let Ok(r) = stick_kern::rahmen::rahmen(&o.rahmen)
        && (breite_mm > r.breite || hoehe_mm > r.hoehe)
    {
        hinweise.push(format!(
            "Zeichnung {breite_mm:.1} × {hoehe_mm:.1} mm ist größer als das Stickfeld {:.0} × {:.0} mm – kleiner \
             einlesen (breite) oder größeren Rahmen wählen",
            r.breite, r.hoehe
        ));
    }
    if objekte.is_empty() {
        return Err("Keine Stickobjekte: alle Flächen sind kleiner als 1 mm²".into());
    }
    let design = json!({
        "version": stick_kern::format::VERSION,
        "name": name,
        "rahmen": o.rahmen,
        "kuerzeste_verbindung": o.kuerzeste_verbindung,
        "objekte": objekte,
    });
    Ok(SvgErgebnis {
        design,
        breite_mm,
        hoehe_mm,
        klassen,
        hinweise,
    })
}

/// SVG-Datei lesen (Name des Designs = Dateiname ohne Endung).
pub fn svg_datei(pfad: &std::path::Path, o: &SvgOptionen) -> Result<SvgErgebnis, String> {
    let daten = std::fs::read(pfad).map_err(|e| format!("{}: {e}", pfad.display()))?;
    let name = pfad.file_stem().unwrap_or_default().to_string_lossy().to_string();
    svg_lesen(&daten, &name, o)
}

#[cfg(test)]
mod tests {
    use super::*;
    use stick_kern::Design;

    fn arten(e: &SvgErgebnis) -> Vec<String> {
        e.design["objekte"]
            .as_array()
            .unwrap()
            .iter()
            .map(|o| o["art"].as_str().unwrap().to_string())
            .collect()
    }

    /// Inkscape-Seite in mm: Breiten wie gezeichnet, Stichart nach Breite wie EasyStitch, berechenbar
    #[test]
    fn stichart_nach_breite() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="100mm" height="80mm" viewBox="0 0 100 80">
            <rect x="10" y="10" width="30" height="30" fill="#ff0000" id="gross"/>
            <rect x="50" y="10" width="10" height="30" fill="#00ff00"/>
            <rect x="65" y="10" width="5" height="30" fill="#0000ff"/>
            <rect x="75" y="10" width="1" height="30" fill="#000000"/>
            <path d="M 10 60 L 90 60" stroke="#333333" stroke-width="0.2" fill="none"/>
            <path d="M 10 70 C 30 60 60 80 90 70" stroke="#333333" stroke-width="4" fill="none"/>
        </svg>"##;
        let e = svg_lesen(svg, "Test", &SvgOptionen::default()).unwrap();
        assert_eq!(
            arten(&e),
            ["tatami", "tatami", "satinflaeche", "lauf", "lauf", "satinlinie"],
            "{:?}",
            e.klassen
        );
        assert!((e.breite_mm - 80.0).abs() < 0.1, "{}", e.breite_mm);
        let o = &e.design["objekte"];
        assert_eq!(o[0]["name"], "gross");
        assert_eq!(o[0]["einstellungen"]["tatami"]["abstand"], 0.45);
        assert_eq!(o[0]["einstellungen"]["tatami"]["zugausgleich"], 0.4);
        assert_eq!(o[2]["einstellungen"]["satin"]["zugausgleich"], 0.4);
        assert_eq!(o[5]["breite"], 4.0);
        assert_eq!(e.klassen["Other Objects"], 1);
        assert_eq!(e.klassen["Wide Objects"], 1);
        assert_eq!(e.klassen["Narrow Objects"], 1);
        // 1 mm breites Rechteck → Laufstich auf der Mittellinie (x = 75,5 − 50 = 25,5 mm)
        let lauf = o[3]["punkte"].as_array().unwrap();
        assert!(
            lauf.iter().all(|q| (q[0].as_f64().unwrap() - 25.5).abs() < 0.2),
            "{lauf:?}"
        );
        let d = Design::aus_value(&e.design).unwrap();
        assert!(d.berechnen().unwrap().stiche.len() > 1000);
    }

    /// Löcher nach Füllregel; später Gezeichnetes spart frühere Flächen aus
    #[test]
    fn loecher_und_ueberdeckung() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="60mm" height="40mm" viewBox="0 0 60 40">
            <path fill-rule="evenodd" fill="#123456" d="M0 0 H40 V40 H0 Z M10 10 H30 V30 H10 Z"/>
            <circle cx="50" cy="20" r="9" fill="#ff8800"/>
            <rect x="35" y="15" width="10" height="10" fill="#00aa00"/>
        </svg>"##;
        let e = svg_lesen(svg, "Test", &SvgOptionen::default()).unwrap();
        let o = e.design["objekte"].as_array().unwrap();
        assert_eq!(o.len(), 3, "{o:?}");
        // Ring: Loch aus evenodd, dazu die Ecke des grünen Quadrats ausgespart → ein Loch
        assert_eq!(o[0]["loecher"].as_array().unwrap().len(), 1, "{}", o[0]);
        let ring = Design::aus_value(&json!({"objekte": [o[0]]})).unwrap();
        let x_max = ring.formen().unwrap()[0].begrenzung.unwrap()[2];
        // Zeichnung x 0 … 59 (Kreisrand) → Mitte 29,5
        assert!((x_max - (40.0 - 29.5)).abs() < 0.05, "Ring bis x = 40: {x_max}");
        // Kreis: das Quadrat liegt darüber → Kreis ohne den überdeckten Teil (Einbuchtung, kein Loch)
        let kreis = Design::aus_value(&json!({"objekte": [o[1]]})).unwrap();
        let b = kreis.formen().unwrap()[0].begrenzung.unwrap();
        // Quadrat x 35 … 45, y 15 … 25 schneidet den Kreis (x ab 41) an; ober-/unterhalb reicht er bis
        // x = 50 − √(81 − 25) ≈ 42,52
        assert!(
            (b[0] - (42.517 - 29.5)).abs() < 0.05,
            "linker Rand vom Quadrat abgeschnitten: {b:?}"
        );
        // ohne Aussparen: Kreis vollständig
        let ohne = svg_lesen(
            svg,
            "Test",
            &SvgOptionen {
                aussparen: false,
                ..Default::default()
            },
        )
        .unwrap();
        let kreis = Design::aus_value(&json!({"objekte": [ohne.design["objekte"][1]]})).unwrap();
        assert!((kreis.formen().unwrap()[0].begrenzung.unwrap()[0] - (41.0 - 29.5)).abs() < 0.05);
    }

    #[test]
    fn breite_und_hinweise() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="100">
            <g transform="translate(10 10) scale(2)">
              <rect width="100" height="20" fill="url(#v)"/>
            </g>
            <defs><linearGradient id="v"><stop offset="0" stop-color="#000"/><stop offset="1" stop-color="#fff"/></linearGradient></defs>
            <text x="0" y="90">Hallo</text>
        </svg>"##;
        let e = svg_lesen(
            svg,
            "Test",
            &SvgOptionen {
                breite_mm: Some(120.0),
                ..Default::default()
            },
        )
        .unwrap();
        assert!((e.breite_mm - 120.0).abs() < 1e-6 && (e.hoehe_mm - 24.0).abs() < 0.01);
        assert!(e.hinweise.iter().any(|h| h.starts_with("Text")), "{:?}", e.hinweise);
        assert!(e.hinweise.iter().any(|h| h.starts_with("Verläufe")), "{:?}", e.hinweise);
        assert!(
            e.hinweise.iter().any(|h| h.contains("größer als das Stickfeld")),
            "{:?}",
            e.hinweise
        );
        assert!(
            svg_lesen(
                b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
                "x",
                &SvgOptionen::default()
            )
            .is_err()
        );
        assert!(svg_lesen(b"kein svg", "x", &SvgOptionen::default()).is_err());
    }
}
