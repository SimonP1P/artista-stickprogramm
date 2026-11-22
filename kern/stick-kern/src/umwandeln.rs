//! Form und Transformation eines Objekts in Knoten umwandeln (Oberfläche v3, W5 „Knoten bearbeiten“).
//!
//! Im Original besteht jede Form aus **Bézier-Knoten** (GeoCurve: Knoten mit Anfassern, `Analyse_Teil3/03_art_objekte.md`);
//! Kreise und Ovale sind dort ebenfalls Knoten, und Transformationen ändern die Knoten selbst (Handbuch Kap. 15: Formen
//! werden transformiert, Stiche neu berechnet). Dieses Modul bringt ein Objekt des Design-JSON in diese Form:
//! - Punktlisten bleiben Punktlisten, Rechtecke werden zu ihren 4 Ecken;
//! - Kreis, Oval und Bogen werden zu kubischen Bézierstücken (je höchstens 90°, Anfasser 4/3·tan(φ/4) – Abweichung
//!   vom Kreis ≤ 0,03 % des Radius), Bézier bleibt Bézier; mehrere Stücke stehen unter `"verbinden"`;
//! - die Transformation (`"transform"`) wird auf alle Knoten angewandt (affin, also exakt für Bézier-Anfasser) und
//!   entfällt; Stichwinkel und Satinbreiten werden wie in `transformation::abbilden` mitgeführt.
//!
//! Danach rechnet der Kern die Stiche wie gewohnt neu. Abweichung zum Objekt vorher: nur durch die Annäherung der
//! Kreise und durch das Abtasten der Bézierstücke (32 Teile je Stück) – im Test unter 0,05 mm. Text, Computer-Schrift,
//! Monogramm und Knopfloch haben keine Knoten (eigene Festlegung: Fehlermeldung).

use crate::design::{Design, ObjektArt};
use crate::format::{Form, PunktAngabe};
use crate::geometrie::{Punkt, p, rechteck};
use crate::transformation::Matrix;
use serde_json::{Map, Value, json};

/// Ein Stück einer Linie: Eckpunkte oder eine kubische Bézierkurve.
#[derive(Clone, Debug, PartialEq)]
enum Stueck {
    Liste(Vec<Punkt>),
    Bezier([Punkt; 4]),
}

/// Kreis- oder Ellipsenbogen (Mitte, Halbachsen, Winkel in Grad) als Bézierstücke von höchstens 90°.
fn bogen_stuecke(mitte: Punkt, rx: f64, ry: f64, von: f64, bis: f64, out: &mut Vec<Stueck>) {
    let spanne = bis - von;
    let n = ((spanne.abs() / 90.0) - 1e-9).ceil().max(1.0) as usize;
    let d = (spanne / n as f64).to_radians();
    let k = 4.0 / 3.0 * (d / 4.0).tan();
    let auf = |x: f64, y: f64| p(mitte.x + rx * x, mitte.y + ry * y);
    for i in 0..n {
        let a0 = von.to_radians() + d * i as f64;
        let a1 = a0 + d;
        let (s0, c0) = a0.sin_cos();
        let (s1, c1) = a1.sin_cos();
        out.push(Stueck::Bezier([
            auf(c0, s0),
            auf(c0 - k * s0, s0 + k * c0),
            auf(c1 + k * s1, s1 - k * c1),
            auf(c1, s1),
        ]));
    }
}

fn stuecke(a: &PunktAngabe, out: &mut Vec<Stueck>) {
    match a {
        PunktAngabe::Liste(l) => out.push(Stueck::Liste(l.clone())),
        PunktAngabe::Form(Form::Rechteck { mitte, breite, hoehe }) => {
            out.push(Stueck::Liste(rechteck(*mitte, *breite, *hoehe)))
        }
        PunktAngabe::Form(Form::Kreis { mitte, radius, start }) => {
            bogen_stuecke(*mitte, *radius, *radius, *start, start + 360.0, out)
        }
        PunktAngabe::Form(Form::Oval {
            mitte,
            breite,
            hoehe,
            start,
        }) => bogen_stuecke(*mitte, breite / 2.0, hoehe / 2.0, *start, start + 360.0, out),
        PunktAngabe::Form(Form::Bogen { mitte, radius, von, bis }) => {
            bogen_stuecke(*mitte, *radius, *radius, *von, *bis, out)
        }
        PunktAngabe::Form(Form::Bezier(q)) => out.push(Stueck::Bezier(*q)),
        PunktAngabe::Form(Form::Verbinden(teile)) => teile.iter().for_each(|t| stuecke(t, out)),
        PunktAngabe::Form(Form::Pfad { punkte, geschlossen }) => {
            for s in crate::format::pfad_stuecke(punkte, *geschlossen) {
                match s {
                    crate::format::PfadStueck::Gerade(a, b) => match out.last_mut() {
                        // Geraden hintereinander in einer Punktliste (Eckpunkte des Knoten-Werkzeugs)
                        Some(Stueck::Liste(l)) if l.last() == Some(&a) => l.push(b),
                        _ => out.push(Stueck::Liste(vec![a, b])),
                    },
                    crate::format::PfadStueck::Bezier(q) => out.push(Stueck::Bezier(q)),
                }
            }
        }
    }
}

fn runden(q: Punkt) -> Value {
    let r = |v: f64| {
        let x = (v * 10_000.0).round() / 10_000.0;
        if x == 0.0 { 0.0 } else { x }
    };
    json!([r(q.x), r(q.y)])
}

fn als_json(s: &[Stueck]) -> Value {
    let eins = |t: &Stueck| match t {
        Stueck::Liste(l) => Value::Array(l.iter().map(|&q| runden(q)).collect()),
        Stueck::Bezier(q) => json!({ "bezier": q.iter().map(|&x| runden(x)).collect::<Vec<_>>() }),
    };
    match s {
        [t] => eins(t),
        _ => json!({ "verbinden": s.iter().map(eins).collect::<Vec<_>>() }),
    }
}

/// Punktangabe (JSON) in Knoten umwandeln, abbilden und – wenn `umkehren` – die Laufrichtung umdrehen.
fn feld_umwandeln(v: &Value, m: &Matrix, umkehren: bool) -> Result<Value, String> {
    let a: PunktAngabe = serde_json::from_value(v.clone()).map_err(|e| format!("Punkte: {e}"))?;
    let mut s = Vec::new();
    stuecke(&a, &mut s);
    for t in s.iter_mut() {
        match t {
            Stueck::Liste(l) => l.iter_mut().for_each(|q| *q = m.punkt(*q)),
            Stueck::Bezier(q) => q.iter_mut().for_each(|x| *x = m.punkt(*x)),
        }
    }
    if umkehren {
        s.reverse();
        for t in s.iter_mut() {
            match t {
                Stueck::Liste(l) => l.reverse(),
                Stueck::Bezier(q) => q.reverse(),
            }
        }
    }
    Ok(als_json(&s))
}

/// Hat das Objekt etwas umzuwandeln (eine Form statt Punktliste oder eine Transformation)?
pub fn umwandelbar(objekt: &Value) -> bool {
    let o = match objekt.as_object() {
        Some(o) => o,
        None => return false,
    };
    let form = |v: &Value| v.is_object();
    o.get("transform").is_some_and(|t| !t.is_null())
        || ["punkte", "seite_a", "seite_b", "umriss"]
            .iter()
            .any(|f| o.get(*f).is_some_and(form))
        || o.get("loecher").and_then(Value::as_array).is_some_and(|l| l.iter().any(form))
}

/// Objekt Nr. `nr` (ab 0) des Designs in Knoten umwandeln: Formen → Punktlisten bzw. Bézierstücke, Transformation
/// angewandt. `json` ist das Design-JSON, aus dem `design` gebaut wurde. Liefert das neue Objekt-JSON (alle übrigen
/// Felder bleiben).
pub fn in_punkte(design: &Design, json: &Value, nr: usize) -> Result<Value, String> {
    let json = crate::format::migrieren(json)?;
    let obj = json
        .get("objekte")
        .and_then(Value::as_array)
        .and_then(|l| l.get(nr))
        .and_then(Value::as_object)
        .ok_or_else(|| format!("Objekt {} gibt es nicht", nr + 1))?;
    let intern = design
        .objekte
        .get(nr)
        .ok_or_else(|| format!("Objekt {} gibt es nicht", nr + 1))?;
    match &intern.art {
        ObjektArt::Text { .. } | ObjektArt::TtfText { .. } | ObjektArt::Knopfloch(_) => {
            return Err(format!(
                "Objekt {}: Text, Monogramm und Knopfloch haben keine Knoten – Größe und Lage über „transform“",
                nr + 1
            ));
        }
        ObjektArt::Gruppe { .. } | ObjektArt::Kette { .. } => {
            return Err(format!("Objekt {}: lässt sich nicht in Knoten umwandeln", nr + 1));
        }
        _ => {}
    }
    let m = design.objekt_matrix(nr)?.unwrap_or(Matrix::EINS);
    let flaeche = m.det().abs().sqrt();
    let gespiegelt = m.det() < 0.0;
    let ws = design.einstellungen.tatami.winkel;

    let mut neu: Map<String, Value> = obj.clone();
    neu.remove("transform");
    let umkehren = match &intern.art {
        ObjektArt::SatinLinie { mittig, .. } => gespiegelt && !*mittig,
        ObjektArt::Blanket { .. } => gespiegelt,
        _ => false,
    };
    for f in ["punkte", "seite_a", "seite_b", "umriss"] {
        if let Some(v) = obj.get(f) {
            neu.insert(f.into(), feld_umwandeln(v, &m, umkehren)?);
        }
    }
    if let Some(l) = obj.get("loecher").and_then(Value::as_array) {
        let l: Result<Vec<Value>, String> = l.iter().map(|v| feld_umwandeln(v, &m, false)).collect();
        neu.insert("loecher".into(), Value::Array(l?));
    }
    if let Some(l) = obj.get("winkellinien").and_then(Value::as_array) {
        let l: Result<Vec<Value>, String> = l
            .iter()
            .map(|v| {
                let [a, b]: [Punkt; 2] = serde_json::from_value(v.clone()).map_err(|e| format!("Winkellinien: {e}"))?;
                Ok(json!([runden(m.punkt(a)), runden(m.punkt(b))]))
            })
            .collect();
        neu.insert("winkellinien".into(), Value::Array(l?));
    }
    if m != Matrix::EINS {
        let zahl = |v: f64| json!((v * 10_000.0).round() / 10_000.0);
        match &intern.art {
            ObjektArt::Tatami { winkel, .. } => {
                neu.insert("winkel".into(), zahl(m.winkel(winkel.unwrap_or(ws))));
            }
            ObjektArt::Musterfuellung { muster, winkel, .. } => {
                let w = winkel
                    .or_else(|| crate::muster::finden(muster).and_then(|x| x.winkel))
                    .unwrap_or(ws);
                neu.insert("winkel".into(), zahl(m.winkel(w)));
            }
            ObjektArt::SatinLinie { breite, .. } | ObjektArt::Blanket { breite, .. } => {
                neu.insert("breite".into(), zahl(breite * flaeche));
            }
            ObjektArt::Applikation(v) => {
                neu.insert("breite".into(), zahl(v.breite * flaeche));
                neu.insert("heft_breite".into(), zahl(v.heft_breite * flaeche));
            }
            _ => {}
        }
    }
    Ok(Value::Object(neu))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stiche(d: &Value) -> Vec<(f64, f64)> {
        let s = Design::aus_value(d).unwrap().berechnen().unwrap();
        s.stiche.iter().map(|x| (x.x, x.y)).collect()
    }

    /// Objekt umwandeln und die Stiche vorher/nachher vergleichen: Stichzahl ±`toleranz_anzahl`, umschließendes
    /// Rechteck auf `max_mm` gleich; ohne Kurven (`exakt`) jeder Einstich gleich.
    fn pruefen(objekt: Value, toleranz_anzahl: usize, max_mm: f64, exakt: bool) -> Value {
        let d = json!({"objekte": [objekt]});
        let design = Design::aus_value(&d).unwrap();
        let neu = in_punkte(&design, &d, 0).unwrap();
        assert!(neu.get("transform").is_none());
        let d2 = json!({"objekte": [neu.clone()]});
        let (a, b) = (stiche(&d), stiche(&d2));
        assert!(
            a.len().abs_diff(b.len()) <= toleranz_anzahl,
            "Stichzahl {} → {} ({neu})",
            a.len(),
            b.len()
        );
        let rahmen = |s: &[(f64, f64)]| {
            s.iter().fold([f64::MAX, f64::MAX, f64::MIN, f64::MIN], |r, q| {
                [r[0].min(q.0), r[1].min(q.1), r[2].max(q.0), r[3].max(q.1)]
            })
        };
        let (ra, rb) = (rahmen(&a), rahmen(&b));
        for k in 0..4 {
            assert!((ra[k] - rb[k]).abs() <= max_mm, "Rechteck {ra:?} → {rb:?} ({neu})");
        }
        if exakt {
            for (x, y) in a.iter().zip(&b) {
                assert!((x.0 - y.0).abs() < 1e-3 && (x.1 - y.1).abs() < 1e-3, "{x:?} → {y:?}");
            }
        }
        neu
    }

    #[test]
    fn bezierstuecke_liegen_auf_dem_kreis() {
        let mut s = Vec::new();
        bogen_stuecke(p(3.0, -2.0), 40.0, 40.0, 10.0, 370.0, &mut s);
        assert_eq!(s.len(), 4);
        for t in &s {
            let Stueck::Bezier(q) = t else { panic!() };
            for x in crate::geometrie::bezier(q[0], q[1], q[2], q[3], 32) {
                let r = ((x.x - 3.0).powi(2) + (x.y + 2.0).powi(2)).sqrt();
                assert!((r - 40.0).abs() < 40.0 * 3e-4, "r = {r}");
            }
        }
        // geschlossen: Ende = Anfang
        let (Stueck::Bezier(a), Stueck::Bezier(z)) = (&s[0], &s[3]) else { panic!() };
        assert!((a[0].x - z[3].x).abs() < 1e-9 && (a[0].y - z[3].y).abs() < 1e-9);
    }

    #[test]
    fn rechteck_und_transformation_als_punktliste() {
        let neu = pruefen(
            json!({"art": "tatami", "umriss": {"rechteck": {"mitte": [5, 0], "breite": 30, "hoehe": 16}},
                   "transform": {"drehen": 30, "skalieren": 120, "verschieben": [2, 3]}}),
            0,
            0.001,
            true,
        );
        assert!(neu["umriss"].is_array());
        assert_eq!(neu["umriss"].as_array().unwrap().len(), 4);
        // Stichwinkel dreht mit (Software Defaults 45° → 75°)
        assert!((neu["winkel"].as_f64().unwrap() - 75.0).abs() < 1e-6, "{}", neu["winkel"]);
    }

    #[test]
    fn kreis_wird_zu_bezierstuecken() {
        let neu = pruefen(
            json!({"art": "tatami", "umriss": {"kreis": {"mitte": [0, 0], "radius": 20}}}),
            1,
            0.05,
            false,
        );
        assert_eq!(neu["umriss"]["verbinden"].as_array().unwrap().len(), 4);
        assert!(neu.get("winkel").is_none(), "ohne Transformation bleibt der Winkel geerbt");
    }

    #[test]
    fn lauf_kreis_und_satinlinie_gespiegelt() {
        let neu = pruefen(
            json!({"art": "lauf", "punkte": {"kreis": {"mitte": [0, 0], "radius": 15}}, "geschlossen": true}),
            1,
            0.05,
            false,
        );
        // alle Einstiche auf dem Kreis (Vernähstiche liegen auf Sehnen, daher 0,06 mm)
        // (ohne den Sprung zurück zur Rahmenmitte am Ende)
        for (x, y) in stiche(&json!({"objekte": [neu]})).into_iter().filter(|q| q.0.hypot(q.1) > 1.0) {
            assert!((x.hypot(y) - 15.0).abs() < 0.06, "({x}, {y})");
        }
        let neu = pruefen(
            json!({"art": "satinlinie", "versatz": "seite", "breite": 3,
                   "punkte": [[-20, 0], [0, 8], [20, 0]],
                   "transform": {"spiegeln": "links_rechts", "skalieren": [150, 100]}}),
            0,
            0.001,
            true,
        );
        assert!((neu["breite"].as_f64().unwrap() - 3.0 * 1.5f64.sqrt()).abs() < 1e-3);
    }

    #[test]
    fn text_hat_keine_knoten() {
        let d = json!({"objekte": [{"art": "knopfloch"}]});
        let design = Design::aus_value(&d).unwrap();
        assert!(in_punkte(&design, &d, 0).is_err());
        assert!(!umwandelbar(&json!({"art": "lauf", "punkte": [[0, 0], [1, 1]]})));
        assert!(umwandelbar(&json!({"art": "lauf", "punkte": {"kreis": {"mitte": [0, 0], "radius": 1}}})));
    }
}
