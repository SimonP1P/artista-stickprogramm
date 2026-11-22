//! Anordnen (Roadmap 4b.12; Handbuch Kap. 15 „Arranging and Transforming Objects“, S. 135–142;
//! `Analyse_Teil3/26_anordnen.md`): **Ausrichten**, **Mirror-Merge** und **Kranz** (Wreath) mit **Kaleidoskop**.
//!
//! Aus dem Original:
//! - Ausrichten (BERESENU STR 33233–33239): links, rechts, oben, unten, Mitten senkrecht/waagrecht, Mitten – „Objects
//!   are aligned with the last object selected“ (S. 135).
//! - Mirror-Merge (DESVIEW `MirrorMergeView{Horizontal,Vertical,Cross}::GetReflectionTransforms` 0x10044100 /
//!   0x100445b0 / 0x10044b90): Spiegeln an der senkrechten bzw. waagrechten Linie durch den Ankerpunkt
//!   (`AffineTransform::Mirror(p, 0, x, y)`); „Kreuz“ = drei Kopien (waagrecht, senkrecht, beides).
//! - Kranz (`MirrorMergeViewWreath::GetReflectionTransforms` 0x100454b0): Kopie i = 1 … n−1 um den Mittelpunkt gedreht
//!   um −i · 2π/n (Konstante 0x1009e838 = 2π); mit **Kaleidoskop** (+0xc8) wird jede ungerade Kopie zusätzlich an der
//!   Linie Mittelpunkt → Objekt gespiegelt („Kaleidoscope only works with an even number“).
//! - Überlappt eine Kopie das Original, fragt das Original „Do you want to merge the overlapped objects?“ (RESENU
//!   STR 4159): hier `zusammenfuehren`.
//!
//! **Umsetzung:** Kopien bekommen die zusammengesetzte Transformation (Form bleibt, Stiche werden neu berechnet wie im
//! Original); Gruppen werden als Ganzes kopiert bzw. verschoben. Eigene Festlegungen: Lage der Kopien in der
//! Stickreihenfolge (direkt hinter der Auswahl, Kopie für Kopie), Zusammenführen nur für Step-, Muster-, Stippling- und
//! Satinflächen ohne Gruppe (Vereinigung der Umrisse, Stichwinkel des Originals), Überlappung ab 0,5 mm².

use crate::aussparen::{Kontur, schnittflaeche, vereinigen};
use crate::design::Design;
use crate::geometrie::{Punkt, p};
use crate::transformation::{Matrix, Skalierung, Spiegeln, Transformation};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

auswahl! {
    /// Ausrichten (Arrange-Leiste des Originals) – nach dem zuletzt gewählten Objekt
    pub enum AusrichtArt {
        /// linke Kanten (Align Left)
        Links = "links",
        /// rechte Kanten (Align Right)
        Rechts = "rechts",
        /// obere Kanten (Align Top)
        Oben = "oben",
        /// untere Kanten (Align Bottom)
        Unten = "unten",
        /// Mitten senkrecht übereinander (Align Centers Vertical: gleiche waagrechte Lage der Mitte)
        MitteSenkrecht = "mitte_senkrecht",
        /// Mitten waagrecht nebeneinander (Align Centers Horizontal: gleiche Höhe der Mitte)
        MitteWaagrecht = "mitte_waagrecht",
        /// Mitten aufeinander (Align Centers)
        Mitte = "mitte",
    }
}

/// gemeinsame Fläche, ab der Kopien als überlappend gelten (mm², eigene Festlegung)
pub const UEBERLAPPUNG_MIN: f64 = 0.5;
/// Punkte eines Kranzes (eigene Festlegung; im Original Eingabefeld „Wreath Points“)
pub const KRANZ_MIN: usize = 2;
pub const KRANZ_MAX: usize = 36;

/// Ergebnis von Mirror-Merge und Kranz.
#[derive(Clone, Debug)]
pub struct Kopien {
    pub design: Value,
    /// Nummern der neuen Objekte im Design
    pub neu: Vec<usize>,
    /// eine Kopie überlappt eine Fläche (Frage des Originals: zusammenführen?)
    pub ueberlappt: bool,
    /// zusammengeführte Flächen
    pub zusammengefuehrt: usize,
}

/// Einheit der Auswahl: ein Objekt ohne Gruppe oder eine ganze Gruppe.
#[derive(Clone, Debug, PartialEq)]
enum Einheit {
    Objekt(usize),
    Gruppe(String, Vec<usize>),
}

impl Einheit {
    fn nummern(&self) -> Vec<usize> {
        match self {
            Einheit::Objekt(i) => vec![*i],
            Einheit::Gruppe(_, m) => m.clone(),
        }
    }
}

fn objekte(v: &Value) -> Vec<Value> {
    v.get("objekte").and_then(Value::as_array).cloned().unwrap_or_default()
}

fn einheiten(v: &Value, auswahl: &[usize]) -> Result<Vec<Einheit>, String> {
    let o = objekte(v);
    let gruppen = v.get("gruppen").and_then(Value::as_object);
    let mut out: Vec<Einheit> = Vec::new();
    for &i in auswahl {
        let x = o.get(i).ok_or_else(|| format!("Objekt {} gibt es nicht", i + 1))?;
        let e = match x.get("gruppe").and_then(Value::as_str) {
            Some(k) if gruppen.is_some_and(|g| g.contains_key(k)) => Einheit::Gruppe(
                k.to_string(),
                (0..o.len()).filter(|&j| o[j].get("gruppe").and_then(Value::as_str) == Some(k)).collect(),
            ),
            _ => Einheit::Objekt(i),
        };
        if !out.contains(&e) {
            out.push(e);
        }
    }
    if out.is_empty() {
        return Err("Keine Objekte gewählt".into());
    }
    Ok(out)
}

/// Umkehrabbildung
pub fn invers(m: &Matrix) -> Result<Matrix, String> {
    let det = m.det();
    if det.abs() < 1e-12 {
        return Err("Transformation lässt sich nicht umkehren".into());
    }
    let (a, b, c, d) = (m.d / det, -m.b / det, -m.c / det, m.a / det);
    Ok(Matrix {
        a,
        b,
        c,
        d,
        e: -(a * m.e + b * m.f),
        f: -(c * m.e + d * m.f),
    })
}

/// Drehung um `q` um `grad` (gegen den Uhrzeiger)
pub fn drehung(q: Punkt, grad: f64) -> Matrix {
    let (s, c) = grad.to_radians().sin_cos();
    Matrix {
        a: c,
        b: -s,
        c: s,
        d: c,
        e: q.x - c * q.x + s * q.y,
        f: q.y - s * q.x - c * q.y,
    }
}

/// Spiegelung an der Geraden durch `q` mit Richtung `grad`
pub fn spiegelung(q: Punkt, grad: f64) -> Matrix {
    let (s, c) = (2.0 * grad).to_radians().sin_cos();
    Matrix {
        a: c,
        b: s,
        c: s,
        d: -c,
        e: q.x - c * q.x - s * q.y,
        f: q.y - s * q.x + c * q.y,
    }
}

fn normal_grad(w: f64) -> f64 {
    let w = w.rem_euclid(360.0);
    if w > 180.0 { w - 360.0 } else { w }
}

fn runden(x: f64) -> f64 {
    let r = (x * 1e4).round() / 1e4;
    if r == 0.0 { 0.0 } else { r }
}

/// Lineare Abbildung [[a, b], [c, d]] als Drehen · Schrägstellen · Skalieren · Spiegeln (Reihenfolge der
/// `Transformation`); von den gleichwertigen Zerlegungen die mit dem kleinsten Drehwinkel.
fn zerlegen(m: &Matrix) -> Result<(f64, f64, f64, f64, Option<Spiegeln>), String> {
    if m.det().abs() < 1e-12 {
        return Err("Transformation lässt sich nicht zerlegen".into());
    }
    let mut beste: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for mx in [1.0, -1.0] {
        // Spalten von M · diag(mx, 1)
        let (x1, y1) = (m.a * mx, m.c * mx);
        let (x2, y2) = (m.b, m.d);
        let r11 = x1.hypot(y1);
        let th = y1.atan2(x1);
        let (s, c) = th.sin_cos();
        let r12 = c * x2 + s * y2;
        let r22 = -s * x2 + c * y2;
        let grad = normal_grad(th.to_degrees());
        let kandidat = (grad, (r12 / r22).atan().to_degrees(), r11, r22.abs(), mx, r22.signum());
        if beste.is_none_or(|b| grad.abs() < b.0.abs() - 1e-9) {
            beste = Some(kandidat);
        }
    }
    let (grad, schraeg, sx, sy, mx, my) = beste.unwrap();
    let sp = match (mx < 0.0, my < 0.0) {
        (false, false) => None,
        (true, false) => Some(Spiegeln::LinksRechts),
        (false, true) => Some(Spiegeln::ObenUnten),
        (true, true) => Some(Spiegeln::Beide),
    };
    Ok((grad, schraeg, sx, sy, sp))
}

/// Transformation `t` um den Bezugspunkt `c`, danach die Abbildung `a`: neue Transformation um denselben Bezugspunkt.
pub fn danach(t: &Transformation, c: Punkt, a: &Matrix) -> Result<Transformation, String> {
    let g = a.nach(&t.matrix(c)?);
    let (drehen, schraeg, sx, sy, spiegeln) = zerlegen(&g)?;
    let z = g.punkt(c);
    let skalieren = if (sx - 1.0).abs() < 1e-9 && (sy - 1.0).abs() < 1e-9 {
        None
    } else if (sx - sy).abs() < 1e-9 {
        Some(Skalierung::Gleich(runden(sx * 100.0)))
    } else {
        Some(Skalierung::Getrennt([runden(sx * 100.0), runden(sy * 100.0)]))
    };
    let v = p(runden(z.x - c.x), runden(z.y - c.y));
    Ok(Transformation {
        skalieren,
        drehen: runden(drehen),
        spiegeln,
        schraeg: runden(schraeg),
        verschieben: (v.x != 0.0 || v.y != 0.0).then_some(v),
    })
}

/// Transformation als JSON (leere Felder weggelassen); None = keine Transformation
pub fn transform_json(t: &Transformation) -> Option<Value> {
    let mut m = Map::new();
    match &t.skalieren {
        Some(Skalierung::Gleich(s)) if (s - 100.0).abs() > 1e-9 => {
            m.insert("skalieren".into(), json!(s));
        }
        Some(Skalierung::Getrennt([x, y])) if (x - 100.0).abs() > 1e-9 || (y - 100.0).abs() > 1e-9 => {
            m.insert("skalieren".into(), json!([x, y]));
        }
        _ => {}
    }
    if t.drehen != 0.0 {
        m.insert("drehen".into(), json!(t.drehen));
    }
    if let Some(s) = t.spiegeln {
        m.insert("spiegeln".into(), json!(s.text()));
    }
    if t.schraeg != 0.0 {
        m.insert("schraeg".into(), json!(t.schraeg));
    }
    if let Some(v) = t.verschieben.filter(|v| v.x != 0.0 || v.y != 0.0) {
        m.insert("verschieben".into(), json!([v.x, v.y]));
    }
    (!m.is_empty()).then_some(Value::Object(m))
}

fn transform_lesen(x: Option<&Value>) -> Result<Transformation, String> {
    match x {
        None | Some(Value::Null) => Ok(Transformation::default()),
        Some(v) => serde_json::from_value(v.clone()).map_err(|e| format!("transform: {e}")),
    }
}

fn transform_setzen(o: &mut Value, t: &Transformation) {
    match transform_json(t) {
        Some(j) => o["transform"] = j,
        None => {
            if let Some(m) = o.as_object_mut() {
                m.remove("transform");
            }
        }
    }
}

/// Verschiebung `d` (mm) zur Transformation addieren
fn verschieben(o: &mut Value, d: Punkt) -> Result<(), String> {
    let mut t = transform_lesen(o.get("transform"))?;
    let v = t.verschieben.unwrap_or(p(0.0, 0.0));
    let n = p(runden(v.x + d.x), runden(v.y + d.y));
    t.verschieben = (n.x != 0.0 || n.y != 0.0).then_some(n);
    transform_setzen(o, &t);
    Ok(())
}

/// Umschließendes Rechteck (x0, y0, x1, y1) der Einheit aus `formen`
fn rechteck(b: &[Option<[f64; 4]>], e: &Einheit) -> Option<[f64; 4]> {
    e.nummern().iter().filter_map(|&i| b.get(i).copied().flatten()).reduce(|a, c| {
        [a[0].min(c[0]), a[1].min(c[1]), a[2].max(c[2]), a[3].max(c[3])]
    })
}

fn begrenzungen(d: &Design) -> Result<Vec<Option<[f64; 4]>>, String> {
    Ok(d.formen()?.into_iter().map(|f| f.begrenzung).collect())
}

/// Ändert sich durch neue Objekte die Mitte des ganzen Designs, verschiebt sich dessen Transformation (um die Mitte)
/// mit – das wird hier über `verschieben` des Designs ausgeglichen, damit alles an seinem Platz bleibt.
fn design_ausgleichen(
    v: &mut Value,
    alt: Option<Matrix>,
    bereiten: &dyn Fn(&Value) -> Result<Design, String>,
) -> Result<(), String> {
    let Some(alt) = alt else { return Ok(()) };
    let neu = bereiten(v)?.bezugspunkte(&[])?.design;
    if let Some(neu) = neu {
        let dv = p(neu.e - alt.e, neu.f - alt.f);
        if dv.x.abs() > 1e-9 || dv.y.abs() > 1e-9 {
            verschieben(v, p(-dv.x, -dv.y))?;
        }
    }
    Ok(())
}

/// **Ausrichten** der gewählten Objekte (Nummern in Klickfolge; Gruppen als Ganzes) nach der Einheit des zuletzt
/// gewählten Objekts.
pub fn ausrichten(
    v: &Value,
    auswahl: &[usize],
    art: AusrichtArt,
    bereiten: &dyn Fn(&Value) -> Result<Design, String>,
) -> Result<Value, String> {
    let es = einheiten(v, auswahl)?;
    if es.len() < 2 {
        return Err("Zum Ausrichten mindestens zwei Objekte oder Gruppen wählen".into());
    }
    let d = bereiten(v)?;
    let b = begrenzungen(&d)?;
    let bezug = d.bezugspunkte(&[])?.design;
    let letzt = *auswahl.last().unwrap();
    let ref_e = es.iter().find(|e| e.nummern().contains(&letzt)).unwrap();
    let r = rechteck(&b, ref_e).ok_or("Das Bezugsobjekt hat keine Form")?;
    let lin_inv = match bezug {
        Some(m) => Some(invers(&Matrix { e: 0.0, f: 0.0, ..m })?),
        None => None,
    };
    let mut out = v.clone();
    for e in &es {
        if e == ref_e {
            continue;
        }
        let Some(u) = rechteck(&b, e) else { continue };
        let mx = |q: [f64; 4]| (q[0] + q[2]) / 2.0;
        let my = |q: [f64; 4]| (q[1] + q[3]) / 2.0;
        let delta = match art {
            AusrichtArt::Links => p(r[0] - u[0], 0.0),
            AusrichtArt::Rechts => p(r[2] - u[2], 0.0),
            AusrichtArt::Oben => p(0.0, r[3] - u[3]),
            AusrichtArt::Unten => p(0.0, r[1] - u[1]),
            AusrichtArt::MitteSenkrecht => p(mx(r) - mx(u), 0.0),
            AusrichtArt::MitteWaagrecht => p(0.0, my(r) - my(u)),
            AusrichtArt::Mitte => p(mx(r) - mx(u), my(r) - my(u)),
        };
        // Verschiebung vor der Transformation des ganzen Designs
        let delta = lin_inv.map(|m| m.punkt(delta)).unwrap_or(delta);
        if delta.x.abs() < 1e-9 && delta.y.abs() < 1e-9 {
            continue;
        }
        match e {
            Einheit::Objekt(i) => verschieben(&mut out["objekte"][*i], delta)?,
            Einheit::Gruppe(k, _) => verschieben(&mut out["gruppen"][k.as_str()], delta)?,
        }
    }
    design_ausgleichen(&mut out, bezug, bereiten)?;
    Ok(out)
}

/// **Mirror-Merge**: Kopie(n) der Auswahl gespiegelt an der senkrechten (`links_rechts`) bzw. waagrechten
/// (`oben_unten`) Linie durch `achse`; `beide` = drei Kopien (links/rechts, oben/unten, beides) wie „Mirror-Merge
/// Horizontal & Vertical“.
pub fn spiegeln_kopieren(
    v: &Value,
    auswahl: &[usize],
    art: Spiegeln,
    achse: Punkt,
    zusammenfuehren: bool,
    bereiten: &dyn Fn(&Value) -> Result<Design, String>,
) -> Result<Kopien, String> {
    let h = spiegelung(achse, 90.0);
    let w = spiegelung(achse, 0.0);
    let abb = match art {
        Spiegeln::LinksRechts => vec![h],
        Spiegeln::ObenUnten => vec![w],
        Spiegeln::Beide => vec![h, w, drehung(achse, 180.0)],
    };
    kopieren(v, auswahl, &abb, zusammenfuehren, bereiten)
}

/// **Kranz** (Wreath): `punkte` − 1 Kopien der Auswahl um `mitte`, im Uhrzeigersinn je 360°/`punkte`; mit
/// `kaleidoskop` jede zweite Kopie gespiegelt (nur gerade Zahl).
pub fn kranz(
    v: &Value,
    auswahl: &[usize],
    punkte: usize,
    kaleidoskop: bool,
    mitte: Punkt,
    zusammenfuehren: bool,
    bereiten: &dyn Fn(&Value) -> Result<Design, String>,
) -> Result<Kopien, String> {
    if !(KRANZ_MIN..=KRANZ_MAX).contains(&punkte) {
        return Err(format!("punkte: {KRANZ_MIN} bis {KRANZ_MAX}"));
    }
    if kaleidoskop && punkte % 2 == 1 {
        return Err("Kaleidoskop geht nur mit einer geraden Zahl von Punkten (wie im Original)".into());
    }
    // Linie Mittelpunkt → Objekt (Mitte der Auswahl) für das Kaleidoskop
    let es = einheiten(v, auswahl)?;
    let b = begrenzungen(&bereiten(v)?)?;
    let r = es
        .iter()
        .filter_map(|e| rechteck(&b, e))
        .reduce(|a, c| [a[0].min(c[0]), a[1].min(c[1]), a[2].max(c[2]), a[3].max(c[3])])
        .ok_or("Die Auswahl hat keine Form")?;
    let ziel = p((r[0] + r[2]) / 2.0, (r[1] + r[3]) / 2.0);
    let achse = (ziel.y - mitte.y).atan2(ziel.x - mitte.x).to_degrees();
    let schritt = 360.0 / punkte as f64;
    let abb: Vec<Matrix> = (1..punkte)
        .map(|i| {
            let d = drehung(mitte, -(i as f64) * schritt);
            if kaleidoskop && i % 2 == 1 { d.nach(&spiegelung(mitte, achse)) } else { d }
        })
        .collect();
    kopieren(v, &es.iter().flat_map(Einheit::nummern).collect::<Vec<_>>(), &abb, zusammenfuehren, bereiten)
}

/// Kopien der Auswahl je Abbildung (Koordinaten der Anzeige, nach der Transformation des ganzen Designs).
fn kopieren(
    v: &Value,
    auswahl: &[usize],
    abb: &[Matrix],
    zusammenfuehren: bool,
    bereiten: &dyn Fn(&Value) -> Result<Design, String>,
) -> Result<Kopien, String> {
    let es = einheiten(v, auswahl)?;
    let d = bereiten(v)?;
    let gruppen_mengen: Vec<Vec<usize>> = es
        .iter()
        .filter_map(|e| match e {
            Einheit::Gruppe(_, m) => Some(m.clone()),
            _ => None,
        })
        .collect();
    let bp = d.bezugspunkte(&gruppen_mengen)?;
    let (dm, dinv) = match bp.design {
        Some(m) => (Some(m), Some(invers(&m)?)),
        None => (None, None),
    };
    let mut alle = objekte(v);
    let mut gruppen = v.get("gruppen").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut namen: BTreeSet<String> = alle
        .iter()
        .filter_map(|o| o.get("name").and_then(Value::as_str).map(str::to_string))
        .collect();
    let mut gruppen_namen: BTreeSet<String> = gruppen
        .values()
        .filter_map(|g| g.get("name").and_then(Value::as_str).map(str::to_string))
        .collect();
    let eindeutig = |basis: &str, belegt: &BTreeSet<String>| -> String {
        if !belegt.contains(basis) {
            return basis.to_string();
        }
        (2..).map(|n| format!("{basis} ({n})")).find(|k| !belegt.contains(k)).unwrap()
    };
    let punkt_json = |o: &mut Value, feld: &str, a: &Matrix| {
        if let Some(q) = o.get(feld).and_then(Value::as_array).filter(|q| q.len() == 2) {
            let (x, y) = (q[0].as_f64().unwrap_or(0.0), q[1].as_f64().unwrap_or(0.0));
            let z = a.punkt(p(x, y));
            o[feld] = json!([runden(z.x), runden(z.y)]);
        }
    };

    let mut neue: Vec<Value> = Vec::new();
    // je Original-Objekt (ohne Gruppe) seine Kopien (Index in `neue`) – für das Zusammenführen
    let mut kopien_von: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for a in abb {
        let a_vor = match (dm, dinv) {
            (Some(m), Some(mi)) => mi.nach(&a.nach(&m)),
            _ => *a,
        };
        let mut gi = 0;
        for e in &es {
            match e {
                Einheit::Objekt(i) => {
                    let mut o = alle[*i].clone();
                    let t = transform_lesen(o.get("transform"))?;
                    transform_setzen(&mut o, &danach(&t, bp.objekte[*i], &a_vor)?);
                    if let Some(n) = o.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                        let k = eindeutig(n, &namen);
                        namen.insert(k.clone());
                        o["name"] = k.into();
                    }
                    punkt_json(&mut o, "einstich", a);
                    punkt_json(&mut o, "ausstich", a);
                    kopien_von.entry(*i).or_default().push(neue.len());
                    neue.push(o);
                }
                Einheit::Gruppe(k, mitglieder) => {
                    let mut g = gruppen.get(k).cloned().unwrap_or_else(|| json!({}));
                    let t = transform_lesen(g.get("transform"))?;
                    transform_setzen(&mut g, &danach(&t, bp.mengen[gi], &a_vor)?);
                    gi += 1;
                    let gname = g.get("name").and_then(Value::as_str).unwrap_or("Gruppe").to_string();
                    let gname = eindeutig(&gname, &gruppen_namen);
                    gruppen_namen.insert(gname.clone());
                    g["name"] = gname.into();
                    let schluessel = (1..).map(|n| format!("{k}-{n}")).find(|x| !gruppen.contains_key(x)).unwrap();
                    gruppen.insert(schluessel.clone(), g);
                    for &i in mitglieder {
                        let mut o = alle[i].clone();
                        o["gruppe"] = schluessel.clone().into();
                        if let Some(n) = o.get("name").and_then(Value::as_str).filter(|n| !n.is_empty()) {
                            let kn = eindeutig(n, &namen);
                            namen.insert(kn.clone());
                            o["name"] = kn.into();
                        }
                        punkt_json(&mut o, "einstich", a);
                        punkt_json(&mut o, "ausstich", a);
                        neue.push(o);
                    }
                }
            }
        }
    }
    // hinter dem letzten Objekt der Auswahl einfügen
    let hinter = es.iter().flat_map(Einheit::nummern).max().unwrap() + 1;
    let n_neu = neue.len();
    alle.splice(hinter..hinter, neue);
    let mut out = v.clone();
    out["objekte"] = Value::Array(alle);
    if !gruppen.is_empty() {
        out["gruppen"] = Value::Object(gruppen);
    }
    let mut neu: Vec<usize> = (hinter..hinter + n_neu).collect();

    // überlappende Flächen: Original und seine Kopien (ohne Gruppe)
    const FLAECHEN: [&str; 4] = ["tatami", "muster", "stippling", "satinflaeche"];
    let mut ueberlappt = false;
    let mut zusammengefuehrt = 0;
    let kandidaten: Vec<(usize, Vec<usize>)> = kopien_von
        .iter()
        .filter(|(i, _)| out["objekte"][**i].get("art").and_then(Value::as_str).is_some_and(|a| FLAECHEN.contains(&a)))
        .map(|(&i, k)| (i, k.iter().map(|j| hinter + j).collect()))
        .collect();
    if !kandidaten.is_empty() {
        let fl = bereiten(&out)?.flaechen_eigen()?;
        let mut loeschen: BTreeSet<usize> = BTreeSet::new();
        for (orig, kopien) in &kandidaten {
            let menge: Vec<usize> = std::iter::once(*orig).chain(kopien.iter().copied()).collect();
            // Zusammenhang über Überlappung (Union-Find)
            let mut wurzel: Vec<usize> = (0..menge.len()).collect();
            fn finde(w: &mut [usize], i: usize) -> usize {
                if w[i] != i {
                    let r = finde(w, w[i]);
                    w[i] = r;
                }
                w[i]
            }
            for x in 0..menge.len() {
                for y in x + 1..menge.len() {
                    let (ka, kb): (&Vec<Kontur>, &Vec<Kontur>) = (&fl[menge[x]].0, &fl[menge[y]].0);
                    if schnittflaeche(ka, kb) > UEBERLAPPUNG_MIN {
                        ueberlappt = true;
                        let (rx, ry) = (finde(&mut wurzel, x), finde(&mut wurzel, y));
                        wurzel[rx.max(ry)] = rx.min(ry);
                    }
                }
            }
            if !zusammenfuehren {
                continue;
            }
            let mut teile: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for x in 0..menge.len() {
                let r = finde(&mut wurzel, x);
                teile.entry(r).or_default().push(menge[x]);
            }
            for glieder in teile.values().filter(|g| g.len() > 1) {
                let konturen: Vec<Kontur> = glieder.iter().flat_map(|&j| fl[j].0.clone()).collect();
                let vereint = vereinigen(&konturen);
                if vereint.len() != 1 {
                    continue;
                }
                let (umriss, loecher) = &vereint[0];
                let basis = glieder[0];
                let o = &mut out["objekte"][basis];
                let pj = |l: &[Punkt]| -> Value { l.iter().map(|q| json!([runden(q.x), runden(q.y)])).collect() };
                o["umriss"] = pj(umriss);
                if loecher.is_empty() {
                    o.as_object_mut().unwrap().remove("loecher");
                } else {
                    o["loecher"] = loecher.iter().map(|l| pj(l)).collect();
                }
                // Form steckt jetzt in den Punkten: Transformation und Winkellinien entfallen, Stichwinkel bleibt
                let hatte = o.as_object_mut().unwrap().remove("transform").is_some();
                if hatte {
                    o.as_object_mut().unwrap().remove("winkellinien");
                    if let Some(w) = fl[basis].1 {
                        o["winkel"] = json!(runden(w));
                    }
                }
                zusammengefuehrt += 1;
                loeschen.extend(glieder[1..].iter().copied());
            }
        }
        if !loeschen.is_empty() {
            let arr = out["objekte"].as_array_mut().unwrap();
            for &j in loeschen.iter().rev() {
                arr.remove(j);
            }
            neu = neu
                .into_iter()
                .filter(|j| !loeschen.contains(j))
                .map(|j| j - loeschen.iter().filter(|&&l| l < j).count())
                .collect();
        }
    }
    design_ausgleichen(&mut out, dm, bereiten)?;
    Ok(Kopien {
        design: out,
        neu,
        ueberlappt,
        zusammengefuehrt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bereiten(v: &Value) -> Result<Design, String> {
        Design::aus_value(v)
    }

    fn quadrat(x: f64, y: f64, s: f64) -> Value {
        json!({"art": "tatami", "name": "Q", "umriss": [[x, y], [x + s, y], [x + s, y + s], [x, y + s]]})
    }

    #[test]
    fn zerlegen_und_danach() {
        // Drehung um 30° und Spiegeln: wieder als Transformation darstellbar, gleiche Abbildung
        let t = Transformation {
            drehen: 20.0,
            skalieren: Some(Skalierung::Getrennt([120.0, 80.0])),
            schraeg: 10.0,
            ..Default::default()
        };
        let c = p(3.0, -2.0);
        for a in [drehung(p(10.0, 5.0), -60.0), spiegelung(p(-4.0, 1.0), 33.0), spiegelung(p(0.0, 0.0), 90.0)] {
            let t2 = danach(&t, c, &a).unwrap();
            let m1 = a.nach(&t.matrix(c).unwrap());
            let m2 = t2.matrix(c).unwrap();
            for q in [p(0.0, 0.0), p(7.0, 3.0), p(-5.0, 9.0)] {
                let (u, w) = (m1.punkt(q), m2.punkt(q));
                assert!((u.x - w.x).abs() < 1e-3 && (u.y - w.y).abs() < 1e-3, "{t2:?}");
            }
        }
        // Spiegeln links/rechts ohne Drehung bleibt „links_rechts“
        let t2 = danach(&Transformation::default(), p(0.0, 0.0), &spiegelung(p(5.0, 0.0), 90.0)).unwrap();
        assert_eq!(t2.spiegeln, Some(Spiegeln::LinksRechts));
        assert_eq!(t2.drehen, 0.0);
        assert_eq!(t2.verschieben, Some(p(10.0, 0.0)));
    }

    #[test]
    fn ausrichten_links_und_mitte() {
        let v = json!({"objekte": [quadrat(0.0, 0.0, 10.0), quadrat(20.0, 5.0, 4.0), quadrat(-7.0, 30.0, 6.0)]});
        // nach dem zuletzt gewählten (Nr. 0)
        let a = ausrichten(&v, &[1, 2, 0], AusrichtArt::Links, &bereiten).unwrap();
        let b = begrenzungen(&bereiten(&a).unwrap()).unwrap();
        assert!(b.iter().all(|r| r.unwrap()[0].abs() < 1e-6), "{b:?}");
        let a = ausrichten(&v, &[0, 1], AusrichtArt::Mitte, &bereiten).unwrap();
        let b = begrenzungen(&bereiten(&a).unwrap()).unwrap();
        let m = |r: [f64; 4]| ((r[0] + r[2]) / 2.0, (r[1] + r[3]) / 2.0);
        let (m0, m1) = (m(b[0].unwrap()), m(b[1].unwrap()));
        assert!((m0.0 - m1.0).abs() < 1e-6 && (m0.1 - m1.1).abs() < 1e-6);
        assert!(ausrichten(&v, &[0], AusrichtArt::Links, &bereiten).is_err());
    }

    #[test]
    fn mirror_merge_und_zusammenfuehren() {
        let v = json!({"objekte": [quadrat(0.0, 0.0, 10.0)]});
        // an der rechten Kante: berührt nur → keine Überlappung
        let k = spiegeln_kopieren(&v, &[0], Spiegeln::LinksRechts, p(10.0, 0.0), false, &bereiten).unwrap();
        assert_eq!(k.neu, vec![1]);
        assert!(!k.ueberlappt);
        let b = begrenzungen(&bereiten(&k.design).unwrap()).unwrap();
        assert!((b[1].unwrap()[0] - 10.0).abs() < 1e-6 && (b[1].unwrap()[2] - 20.0).abs() < 1e-6);
        assert_eq!(k.design["objekte"][1]["name"], "Q (2)");
        // Kreuz: drei Kopien
        let k = spiegeln_kopieren(&v, &[0], Spiegeln::Beide, p(10.0, 0.0), false, &bereiten).unwrap();
        assert_eq!(k.neu.len(), 3);
        // Achse in der Fläche: überlappt; zusammengeführt ein Objekt von −? bis 14 mm
        let k = spiegeln_kopieren(&v, &[0], Spiegeln::LinksRechts, p(7.0, 0.0), false, &bereiten).unwrap();
        assert!(k.ueberlappt);
        let k = spiegeln_kopieren(&v, &[0], Spiegeln::LinksRechts, p(7.0, 0.0), true, &bereiten).unwrap();
        assert_eq!(k.zusammengefuehrt, 1);
        assert_eq!(k.design["objekte"].as_array().unwrap().len(), 1);
        assert!(k.neu.is_empty());
        let b = begrenzungen(&bereiten(&k.design).unwrap()).unwrap();
        assert!((b[0].unwrap()[2] - 14.0).abs() < 1e-6, "{b:?}");
    }

    #[test]
    fn kranz_mit_kaleidoskop() {
        // Dreieck rechts vom Mittelpunkt (0, 0)
        let v = json!({"objekte": [{"art": "tatami", "umriss": [[10, 0], [16, 1], [10, 3]]}]});
        let k = kranz(&v, &[0], 6, false, p(0.0, 0.0), false, &bereiten).unwrap();
        assert_eq!(k.neu.len(), 5);
        // erste Kopie im Uhrzeigersinn um 60° gedreht
        let b = begrenzungen(&bereiten(&k.design).unwrap()).unwrap();
        let m = |r: [f64; 4]| ((r[0] + r[2]) / 2.0, (r[1] + r[3]) / 2.0);
        assert!(m(b[1].unwrap()).1 < -5.0, "{b:?}");
        // Kaleidoskop: ungerade Kopien gespiegelt, ungerade Zahl abgelehnt
        let k = kranz(&v, &[0], 6, true, p(0.0, 0.0), false, &bereiten).unwrap();
        assert_eq!(k.design["objekte"][1]["transform"]["spiegeln"].is_string(), true, "{}", k.design);
        assert!(kranz(&v, &[0], 5, true, p(0.0, 0.0), false, &bereiten).is_err());
        assert!(kranz(&v, &[0], 1, false, p(0.0, 0.0), false, &bereiten).is_err());
    }

    #[test]
    fn gruppe_wird_ganz_kopiert_und_design_transform_bleibt() {
        let mut a = quadrat(0.0, 0.0, 4.0);
        a["gruppe"] = "g1".into();
        let mut b = quadrat(6.0, 0.0, 4.0);
        b["gruppe"] = "g1".into();
        let v = json!({"objekte": [a, b, quadrat(-20.0, -20.0, 3.0)], "gruppen": {"g1": {"name": "Paar"}},
                       "transform": {"drehen": 15.0}});
        let vorher = begrenzungen(&bereiten(&v).unwrap()).unwrap();
        let k = spiegeln_kopieren(&v, &[0], Spiegeln::ObenUnten, p(0.0, 20.0), false, &bereiten).unwrap();
        assert_eq!(k.neu, vec![2, 3]);
        assert_eq!(k.design["gruppen"].as_object().unwrap().len(), 2);
        let nachher = begrenzungen(&bereiten(&k.design).unwrap()).unwrap();
        // vorhandene Objekte bleiben trotz neuer Design-Mitte an ihrem Platz
        for (x, y) in [(0, 0), (1, 1), (2, 4)] {
            let (u, w) = (vorher[x].unwrap(), nachher[y].unwrap());
            assert!(u.iter().zip(w).all(|(a, b)| (a - b).abs() < 1e-3), "{u:?} {w:?}");
        }
        // Kopie gespiegelt an y = 20 (in der Anzeige)
        let (o, c) = (vorher[0].unwrap(), nachher[2].unwrap());
        assert!((o[1] + c[3] - 40.0).abs() < 1e-3 && (o[3] + c[1] - 40.0).abs() < 1e-3, "{o:?} {c:?}");
    }
}
