//! Erzeugte Schnittstellen-Dateien: JSON-Schemas, TypeScript-Typen (`kern.ts`) und die lesbare Feld- und
//! Befehlsliste (`SCHNITTSTELLE.md`). **Nicht von Hand ändern** – Quelle sind `stick_kern::format` und die Befehle;
//! neu erzeugen mit `stick api exportieren` (oder `STICK_API_SCHREIBEN=1 cargo test -p stick-api`).

use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Kopfzeile aller erzeugten Dateien.
const ERZEUGT: &str = "ERZEUGT aus dem Rust-Kern (stick-kern/src/format.rs, stick-api) – nicht von Hand ändern; \
                       neu erzeugen: stick api exportieren";

/// Schema aller Befehle: je Befehl Beschreibung, Eingabe, Ausgabe; gemeinsame Typen unter `$defs` (auch das Design).
pub fn befehle_schema() -> Value {
    let mut g = schemars::generate::SchemaSettings::draft2020_12().into_generator();
    let mut befehle = Map::new();
    for b in crate::alle() {
        let ein = (b.eingabe)(&mut g).to_value();
        let aus = (b.ausgabe)(&mut g).to_value();
        befehle.insert(
            b.name.into(),
            json!({ "beschreibung": b.beschreibung.trim(), "eingabe": ein, "ausgabe": aus }),
        );
    }
    // Ereignisse an die Oberfläche (kein Befehl, aber ein Typ): `maschine_fortschritt`
    let mut ereignisse = Map::new();
    ereignisse.insert(
        "maschine_fortschritt".into(),
        g.subschema_for::<crate::maschine::Fortschritt>().to_value(),
    );
    let defs: Map<String, Value> = g.take_definitions(true).into_iter().collect();
    json!({
        "ereignisse": ereignisse,
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://artista-stickprogramm/befehle.schema.json",
        "description": ERZEUGT,
        "x-version": stick_kern::format::VERSION,
        "befehle": befehle,
        "$defs": defs,
    })
}

fn design_schema() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(stick_kern::format::schema)
}

fn ref_name(r: &str) -> &str {
    r.rsplit('/').next().unwrap_or(r)
}

/// Eigenschaften eines Schemas samt `$ref`/`allOf` (ohne Tiefe – reicht für die Objektarten).
fn eigenschaften<'a>(s: &'a Value, defs: &'a Map<String, Value>, out: &mut BTreeMap<String, &'a Value>) {
    if let Some(r) = s.get("$ref").and_then(Value::as_str)
        && let Some(d) = defs.get(ref_name(r))
    {
        eigenschaften(d, defs, out);
    }
    for teil in s.get("allOf").and_then(Value::as_array).into_iter().flatten() {
        eigenschaften(teil, defs, out);
    }
    if let Some(p) = s.get("properties").and_then(Value::as_object) {
        for (k, v) in p {
            out.insert(k.clone(), v);
        }
    }
}

/// Objektarten des Formats: `art` → Schema der Variante.
fn objektarten(schema: &Value) -> Vec<(String, &Value)> {
    let defs = schema["$defs"].as_object().expect("$defs");
    defs["ObjektJson"]["oneOf"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| Some((v["properties"]["art"]["const"].as_str()?.to_string(), v)))
        .collect()
}

/// Felder, die der Kern nicht liest (oben im Design und je Objekt) – als Hinweis für die Oberfläche (Tippfehler,
/// veraltete Felder). Unbekannte Objektarten meldet schon das Lesen als Fehler.
pub fn unbekannte_felder(design: &Value) -> Vec<String> {
    let schema = design_schema();
    let defs = schema["$defs"].as_object().expect("$defs");
    let mut out = Vec::new();
    let oben: BTreeSet<&String> = schema["properties"]
        .as_object()
        .map(|p| p.keys().collect())
        .unwrap_or_default();
    for k in design.as_object().map(|o| o.keys()).into_iter().flatten() {
        if !oben.contains(k) {
            out.push(format!("Design: Feld „{k}“ wird nicht gelesen"));
        }
    }
    let arten = objektarten(schema);
    for (i, o) in design
        .get("objekte")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let art = o.get("art").and_then(Value::as_str).unwrap_or("");
        let Some((_, v)) = arten.iter().find(|(a, _)| a == art) else {
            continue;
        };
        let mut felder = BTreeMap::new();
        eigenschaften(v, defs, &mut felder);
        for k in o.as_object().map(|o| o.keys()).into_iter().flatten() {
            if !felder.contains_key(k) {
                out.push(format!("Objekt {} ({art}): Feld „{k}“ wird nicht gelesen", i + 1));
            }
        }
    }
    out
}

// ---------------------------------------------------------------- TypeScript

fn ts_literal(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

fn klammern(t: String) -> String {
    if t.contains(" | ") || t.contains(" & ") {
        format!("({t})")
    } else {
        t
    }
}

fn kommentar(s: &Value, einzug: &str) -> String {
    let mut zeilen: Vec<String> = s
        .get("description")
        .and_then(Value::as_str)
        .map(|d| d.lines().map(str::to_string).collect())
        .unwrap_or_default();
    if let Some(d) = s.get("default").filter(|d| !d.is_null() && !d.is_object()) {
        zeilen.push(format!("@default {}", ts_literal(d)));
    }
    if zeilen.is_empty() {
        return String::new();
    }
    if zeilen.len() == 1 {
        return format!("{einzug}/** {} */\n", zeilen[0].replace("*/", "*\\/"));
    }
    let mut t = format!("{einzug}/**\n");
    for z in zeilen {
        t.push_str(&format!("{einzug} * {}\n", z.replace("*/", "*\\/")));
    }
    t.push_str(&format!("{einzug} */\n"));
    t
}

/// TypeScript-Typ eines Schemas (Entwurf 2020-12, wie schemars ihn erzeugt).
pub fn ts_typ(s: &Value, einzug: usize) -> String {
    if s == &Value::Bool(true) || s.as_object().is_some_and(Map::is_empty) {
        return "unknown".into();
    }
    let mut teile: Vec<String> = Vec::new();
    if let Some(r) = s.get("$ref").and_then(Value::as_str) {
        teile.push(ref_name(r).into());
    }
    if let Some(c) = s.get("const") {
        teile.push(ts_literal(c));
    }
    if let Some(e) = s.get("enum").and_then(Value::as_array) {
        teile.push(e.iter().map(ts_literal).collect::<Vec<_>>().join(" | "));
    }
    for (schl, trenner) in [("oneOf", " | "), ("anyOf", " | "), ("allOf", " & ")] {
        if let Some(l) = s.get(schl).and_then(Value::as_array) {
            let t: Vec<String> = l.iter().map(|x| klammern(ts_typ(x, einzug))).collect();
            teile.push(klammern(t.join(trenner)));
        }
    }
    let typen: Vec<&str> = match s.get("type") {
        Some(Value::String(t)) => vec![t.as_str()],
        Some(Value::Array(l)) => l.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    let hat_eigenes = s.get("const").is_some() || s.get("enum").is_some();
    if !typen.is_empty() && !hat_eigenes {
        let mut u: Vec<String> = Vec::new();
        for t in &typen {
            u.push(match *t {
                "string" => "string".into(),
                "number" | "integer" => "number".into(),
                "boolean" => "boolean".into(),
                "null" => "null".into(),
                "array" => {
                    if let Some(p) = s.get("prefixItems").and_then(Value::as_array) {
                        format!(
                            "[{}]",
                            p.iter().map(|x| ts_typ(x, einzug)).collect::<Vec<_>>().join(", ")
                        )
                    } else if let Some(i) = s.get("items") {
                        format!("{}[]", klammern(ts_typ(i, einzug)))
                    } else {
                        "unknown[]".into()
                    }
                }
                "object" => ts_objekt(s, einzug),
                x => x.into(),
            });
        }
        teile.push(u.join(" | "));
    } else if typen.is_empty() && s.get("properties").is_some() {
        teile.push(ts_objekt(s, einzug));
    } else if typen.contains(&"null") && hat_eigenes {
        teile.push("null".into());
    }
    match teile.len() {
        0 => "unknown".into(),
        1 => teile.remove(0),
        _ => teile.into_iter().map(klammern).collect::<Vec<_>>().join(" & "),
    }
}

fn ts_objekt(s: &Value, einzug: usize) -> String {
    let Some(p) = s.get("properties").and_then(Value::as_object) else {
        return match s.get("additionalProperties") {
            Some(a) if a.is_object() => format!("Record<string, {}>", ts_typ(a, einzug)),
            _ => "Record<string, unknown>".into(),
        };
    };
    let pflicht: BTreeSet<&str> = s
        .get("required")
        .and_then(Value::as_array)
        .map(|r| r.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let innen = "  ".repeat(einzug + 1);
    let mut t = String::from("{\n");
    for (k, v) in p {
        t.push_str(&kommentar(v, &innen));
        let frage = if pflicht.contains(k.as_str()) { "" } else { "?" };
        t.push_str(&format!("{innen}{k}{frage}: {};\n", ts_typ(v, einzug + 1)));
    }
    t.push_str(&format!("{}}}", "  ".repeat(einzug)));
    t
}

/// `kern.ts`: alle Typen, die Befehlsliste mit Ein-/Ausgabe und eine Aufruf-Funktion für Tauri.
pub fn typescript(befehle: &Value) -> String {
    let mut t = format!(
        "// {ERZEUGT}\n// Design-Format Fassung {}. Objektnummern ab 0. Koordinaten in mm, Ursprung Rahmenmitte, +y oben.\n\n",
        stick_kern::format::VERSION
    );
    t.push_str("import { invoke } from \"@tauri-apps/api/core\";\n\n");
    for (name, s) in befehle["$defs"].as_object().into_iter().flatten() {
        t.push_str(&kommentar(s, ""));
        t.push_str(&format!("export type {name} = {};\n\n", ts_typ(s, 0)));
    }
    t.push_str("/** Ein Design (Wurzel von design.schema.json). */\nexport type Design = DesignJson;\n\n");
    t.push_str("/** Alle Befehle des Kerns mit Ein- und Ausgabe. */\nexport interface Befehle {\n");
    for (name, b) in befehle["befehle"].as_object().into_iter().flatten() {
        t.push_str(&kommentar(&json!({"description": b["beschreibung"]}), "  "));
        t.push_str(&format!(
            "  {name}: {{ eingabe: {}; ausgabe: {} }};\n",
            ts_typ(&b["eingabe"], 1),
            ts_typ(&b["ausgabe"], 1)
        ));
    }
    t.push_str("}\n\n");
    t.push_str("/** Namen aller Befehle. */\nexport const BEFEHLE = [\n");
    for name in befehle["befehle"].as_object().into_iter().flatten().map(|(n, _)| n) {
        t.push_str(&format!("  \"{name}\",\n"));
    }
    t.push_str("] as const;\n\n");
    t.push_str(
        "/** Fehler eines Befehls (Meldung deutsch, Objektnummer ab 0 wenn bekannt). */\n\
         export function istFehler(x: unknown): x is Fehler {\n  \
         return typeof x === \"object\" && x !== null && \"meldung\" in x;\n}\n\n\
         /**\n * Befehl des Kerns aufrufen (Tauri-Befehl `kern`, siehe kern/stick-api/src/lib.rs).\n \
         * Wirft bei Fehlern ein Objekt vom Typ `Fehler`.\n */\n\
         export function kern<N extends keyof Befehle>(befehl: N, daten: Befehle[N][\"eingabe\"]): Promise<Befehle[N][\"ausgabe\"]> {\n  \
         return invoke(\"kern\", { befehl, daten });\n}\n",
    );
    t
}

// ---------------------------------------------------------------- Markdown

fn md_typ(s: &Value) -> String {
    let t = ts_typ(s, 0).replace(" | null", "").replace("null | ", "");
    let t = match t.strip_prefix('(').and_then(|x| x.strip_suffix(')')) {
        Some(innen) if !innen.contains('(') => innen.to_string(),
        _ => t,
    };
    if t.contains('\n') {
        "Objekt".into()
    } else {
        t.replace('|', "\\|")
    }
}

fn md_zelle(t: &str) -> String {
    t.replace('\n', " ").replace('|', "\\|")
}

fn md_standard(s: &Value) -> String {
    match s.get("default") {
        None | Some(Value::Null) => "–".into(),
        Some(Value::Object(_)) => "(siehe Typ)".into(),
        Some(d) => format!("`{}`", ts_literal(d)),
    }
}

fn md_tabelle(felder: &BTreeMap<String, &Value>, pflicht: &BTreeSet<String>, ohne: &BTreeSet<String>) -> String {
    let mut t = String::from("| Feld | Typ | Pflicht | Standard | Beschreibung |\n|---|---|---|---|---|\n");
    for (k, v) in felder {
        if ohne.contains(k) {
            continue;
        }
        t.push_str(&format!(
            "| `{k}` | {} | {} | {} | {} |\n",
            md_typ(v),
            if pflicht.contains(k) { "ja" } else { "" },
            md_standard(v),
            md_zelle(v.get("description").and_then(Value::as_str).unwrap_or(""))
        ));
    }
    t
}

fn pflichtfelder(s: &Value, defs: &Map<String, Value>) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = s
        .get("required")
        .and_then(Value::as_array)
        .map(|r| r.iter().filter_map(Value::as_str).map(String::from).collect())
        .unwrap_or_default();
    if let Some(r) = s.get("$ref").and_then(Value::as_str)
        && let Some(d) = defs.get(ref_name(r))
    {
        out.extend(pflichtfelder(d, defs));
    }
    out
}

/// `SCHNITTSTELLE.md`: Design-Format (Felder je Objektart, Auswahlwerte) und Befehle – lesbar für Designer und
/// Frontend.
pub fn markdown(befehle: &Value) -> String {
    let schema = design_schema();
    let defs = schema["$defs"].as_object().expect("$defs");
    let mut t = format!(
        "# Schnittstelle Kern ↔ Oberfläche\n\n> {ERZEUGT}\n\n\
         **Design-Format Fassung {}.** Maschinenlesbar: `kern/schnittstelle/design.schema.json` (Design), \
         `kern/schnittstelle/befehle.schema.json` (Befehle), `kern/schnittstelle/kern.ts` (TypeScript-Typen + Aufruf). \
         Was jede Funktion für den Anwender bedeutet und welcher Modus (einfach/experte): `UI_Mockups/FUNKTIONEN.md`. \
         Alle Einzelwerte (`einstellungen`) mit Standard je Stoff: `PARAMETER.md`.\n\n\
         Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. Objektnummern in Befehlen und Ergebnissen ab 0.\n\n",
        stick_kern::format::VERSION
    );

    t.push_str("## 1. Design (oberste Ebene)\n\n");
    let mut oben = BTreeMap::new();
    eigenschaften(schema, defs, &mut oben);
    t.push_str(&md_tabelle(&oben, &pflichtfelder(schema, defs), &BTreeSet::new()));

    // Objektarten: gemeinsame Felder einmal, dann je Art die eigenen
    let arten = objektarten(schema);
    // je Objektart: art, Titel, Felder, Pflichtfelder
    type ArtFelder<'a> = (String, String, BTreeMap<String, &'a Value>, BTreeSet<String>);
    let felder_je_art: Vec<ArtFelder> = arten
        .iter()
        .map(|(art, v)| {
            let mut f = BTreeMap::new();
            eigenschaften(v, defs, &mut f);
            f.remove("art");
            let titel = v.get("description").and_then(Value::as_str).unwrap_or("").to_string();
            (art.clone(), titel, f, pflichtfelder(v, defs))
        })
        .collect();
    let gemeinsam: BTreeSet<String> = felder_je_art
        .iter()
        .map(|(_, _, f, _)| f.keys().cloned().collect::<BTreeSet<_>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or_default();
    t.push_str("\n## 2. Objekte (`objekte`, Feld `art`)\n\n### Felder aller Objekte\n\n");
    if let Some((_, _, f, p)) = felder_je_art.first() {
        let nur: BTreeMap<String, &Value> = f
            .iter()
            .filter(|(k, _)| gemeinsam.contains(*k))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        t.push_str(&md_tabelle(&nur, p, &BTreeSet::new()));
    }
    for (art, titel, f, p) in &felder_je_art {
        t.push_str(&format!("\n### `{art}` – {titel}\n\n"));
        let def = arten
            .iter()
            .find(|(a, _)| a == art)
            .and_then(|(_, v)| v.get("$ref").and_then(Value::as_str))
            .and_then(|r| defs.get(ref_name(r)));
        if let Some(d) = def.and_then(|d| d.get("description")).and_then(Value::as_str) {
            t.push_str(&format!("{}\n\n", d.replace('\n', " ")));
        }
        t.push_str(&md_tabelle(f, p, &gemeinsam));
    }

    t.push_str("\n## 3. Auswahlwerte\n\n");
    for (name, s) in defs {
        let werte: Vec<(String, String)> = if let Some(e) = s.get("enum").and_then(Value::as_array) {
            e.iter().map(|w| (ts_literal(w), String::new())).collect()
        } else if let Some(o) = s
            .get("oneOf")
            .and_then(Value::as_array)
            .filter(|o| o.iter().all(|x| x.get("const").is_some()))
        {
            o.iter()
                .map(|x| {
                    (
                        ts_literal(&x["const"]),
                        x.get("description").and_then(Value::as_str).unwrap_or("").to_string(),
                    )
                })
                .collect()
        } else {
            continue;
        };
        let titel = s.get("description").and_then(Value::as_str).unwrap_or("");
        t.push_str(&format!("**{name}** – {}\n\n", md_zelle(titel)));
        for (w, d) in werte {
            if d.is_empty() {
                t.push_str(&format!("- `{w}`\n"));
            } else {
                t.push_str(&format!("- `{w}` – {}\n", md_zelle(&d)));
            }
        }
        t.push('\n');
    }

    t.push_str("## 4. Weitere Typen des Formats\n\n");
    let varianten: BTreeSet<&str> = arten
        .iter()
        .filter_map(|(_, v)| v.get("$ref").and_then(Value::as_str).map(ref_name))
        .collect();
    for (name, s) in defs {
        if varianten.contains(name.as_str()) || s.get("properties").is_none() {
            continue;
        }
        let mut f = BTreeMap::new();
        eigenschaften(s, defs, &mut f);
        t.push_str(&format!(
            "### {name}\n\n{}\n\n",
            md_zelle(s.get("description").and_then(Value::as_str).unwrap_or(""))
        ));
        t.push_str(&md_tabelle(&f, &pflichtfelder(s, defs), &BTreeSet::new()));
        t.push('\n');
    }

    t.push_str(
        "## 5. Befehle\n\nAufruf aus der Oberfläche: `kern(\"name\", eingabe)` aus `kern.ts` (ein Tauri-Befehl `kern`, \
         Einbindung siehe `kern/stick-api/src/lib.rs`); zum Ausprobieren ohne Oberfläche: \
         `stick api NAME '{…}'`. Fehler: `{meldung, objekt}`.\n\n| Befehl | Eingabe | Ausgabe | Beschreibung |\n|---|---|---|---|\n",
    );
    for (name, b) in befehle["befehle"].as_object().into_iter().flatten() {
        t.push_str(&format!(
            "| `{name}` | {} | {} | {} |\n",
            md_typ(&b["eingabe"]),
            md_typ(&b["ausgabe"]),
            md_zelle(b["beschreibung"].as_str().unwrap_or(""))
        ));
    }
    t.push_str(
        "\n**Ereignis** `maschine_fortschritt` (Typ `Fortschritt`: `was`, `erledigt`, `gesamt`, `wiederholungen`): \
         Fortschritt von `maschine_senden` und `maschine_lesen`, von der App gesendet. Fehler der Maschine tragen \
         zusätzlich `code` (`FehlerCode`). **Sicherheit:** nur `maschine_senden` schreibt in die Maschine – erst nach \
         `maschine_planen` und ausdrücklicher Bestätigung; `wahl.sticken` nur nach dem Gefahrendialog.\n",
    );
    t
}

// ---------------------------------------------------------------- Dateien

/// Wurzel des Repositorys (zwei Ebenen über `kern/stick-api`).
pub fn repo_wurzel() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Alle erzeugten Dateien: Pfad relativ zur Repository-Wurzel → Inhalt.
pub fn dateien() -> Vec<(String, String)> {
    let befehle = befehle_schema();
    let schoen = |v: &Value| serde_json::to_string_pretty(v).unwrap_or_default() + "\n";
    let mut design = design_schema().clone();
    if let Some(o) = design.as_object_mut() {
        o.insert("description".into(), ERZEUGT.into());
    }
    vec![
        ("kern/schnittstelle/design.schema.json".into(), schoen(&design)),
        ("kern/schnittstelle/befehle.schema.json".into(), schoen(&befehle)),
        ("kern/schnittstelle/kern.ts".into(), typescript(&befehle)),
        ("SCHNITTSTELLE.md".into(), markdown(&befehle)),
    ]
}

/// Erzeugte Dateien schreiben (unter `wurzel`); Ergebnis = geänderte Pfade.
pub fn schreiben(wurzel: &Path) -> Result<Vec<String>, String> {
    let mut geaendert = Vec::new();
    for (pfad, inhalt) in dateien() {
        let ziel = wurzel.join(&pfad);
        if std::fs::read_to_string(&ziel).ok().as_deref() == Some(inhalt.as_str()) {
            continue;
        }
        if let Some(o) = ziel.parent() {
            std::fs::create_dir_all(o).map_err(|e| format!("{}: {e}", o.display()))?;
        }
        std::fs::write(&ziel, inhalt).map_err(|e| format!("{}: {e}", ziel.display()))?;
        geaendert.push(pfad);
    }
    Ok(geaendert)
}

/// Veraltete erzeugte Dateien (leer = alles aktuell). Zeilenenden werden nicht verglichen (Git unter Windows).
pub fn veraltet(wurzel: &Path) -> Vec<String> {
    dateien()
        .into_iter()
        .filter(|(pfad, inhalt)| {
            std::fs::read_to_string(wurzel.join(pfad))
                .map(|t| t.replace("\r\n", "\n"))
                .ok()
                .as_deref()
                != Some(inhalt.as_str())
        })
        .map(|(p, _)| p)
        .collect()
}
