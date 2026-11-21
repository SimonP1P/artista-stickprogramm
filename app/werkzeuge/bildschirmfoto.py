"""Bildschirmfoto der Oberfläche ohne Fenster: die gebaute Oberfläche (app/build, `npm run build`) in Chrome/Edge headless
laden; die Antworten des Kerns kommen vorher aus demselben Kern über die Befehlszeile (`stick api …`).

Gebraucht, weil Claude Code auf dem Mac keine Bildschirmaufnahme des echten Fensters machen darf. Die Bilder zeigen die
echte Svelte-Oberfläche mit echten Stichen; was ein Befehl liefert, den das Werkzeug nicht vorbereitet, meldet die
Oberfläche als Fehler.

Aufruf (aus app/):
  python3 werkzeuge/bildschirmfoto.py DESIGN.json ZIEL.png [--experte] [--dunkel] [--auswahl N] [--ansicht form]
          [--groesse 1280x800] [--stick PFAD] [--browser PFAD]
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile

HIER = os.path.dirname(os.path.abspath(__file__))
APP = os.path.dirname(HIER)
REPO = os.path.dirname(APP)

BROWSER = [
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
]


def stick_finden(angabe):
    if angabe:
        return angabe
    for z in [os.environ.get("CARGO_TARGET_DIR", ""), os.path.join(REPO, "kern", "target")]:
        for n in ("stick", "stick.exe"):
            p = os.path.join(z, "debug", n)
            if z and os.path.exists(p):
                return p
    sys.exit("stick nicht gefunden – `cargo build -p stick-cli` in kern/ oder --stick angeben")


def api(stick, befehl, daten):
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8") as f:
        json.dump(daten, f)
        pfad = f.name
    try:
        r = subprocess.run([stick, "api", befehl, pfad], capture_output=True, text=True, encoding="utf-8")
    finally:
        os.unlink(pfad)
    if r.returncode != 0:
        # Fehler des Kerns wie in der App: {meldung, objekt}
        text = (r.stdout + r.stderr).strip()
        start = text.find("{")
        try:
            return {"__fehler__": json.loads(text[start:])}
        except ValueError:
            return {"__fehler__": {"meldung": text, "objekt": None}}
    return json.loads(r.stdout)


def main():
    a = argparse.ArgumentParser()
    a.add_argument("design")
    a.add_argument("ziel")
    a.add_argument("--experte", action="store_true")
    a.add_argument("--dunkel", action="store_true")
    a.add_argument("--auswahl", type=int)
    a.add_argument("--ansicht")
    a.add_argument("--schritt", type=int)
    a.add_argument("--maschine", choices=["anschluss", "plan", "fertig", "sticken", "liste"],
                   help="Maschinen-Ablauf mit der Simulation bis zu dieser Stufe")
    a.add_argument("--dialog", help='JSON, z. B. {"art": "alphabet", "objekt": 0}')
    a.add_argument("--groesse", default="1280x800")
    a.add_argument("--stick")
    a.add_argument("--browser")
    o = a.parse_args()

    stick = stick_finden(o.stick)
    design_pfad = os.path.abspath(o.design)
    design = json.load(open(design_pfad, encoding="utf-8"))
    antworten = {
        "rahmen_liste": api(stick, "rahmen_liste", {}),
        "datei_lesen": json.dumps(design),
    }
    for b in ("design_berechnen", "design_formen", "design_pruefen"):
        antworten[b] = api(stick, b, {"design": design})
    for b in ("alphabete_liste", "stoffe_liste", "garnkarten_liste", "design_schema", "fuellmuster_liste",
              "step_muster_liste", "parameter_katalog"):
        antworten[b] = api(stick, b, {})
    antworten["ueberdeckungen"] = api(stick, "ueberdeckungen", {"design": design})
    sim = {"port": "", "simulation": True}
    antworten["maschine_ports"] = []
    if o.maschine:
        ein = {"design": design, **sim, "wahl": {"icon": True, "farben": True, "sticken": False}}
        antworten["maschine_planen"] = api(stick, "maschine_planen", ein)
        antworten["maschine_senden"] = api(stick, "maschine_senden", ein)
        if o.maschine == "liste":
            antworten["maschine_liste"] = api(stick, "maschine_liste", sim)
            nr = antworten["maschine_liste"]["designs"][0]["nr"]
            antworten["maschine_lesen"] = api(stick, "maschine_lesen", {**sim, "nr": nr})
    karte = design.get("garnkarte", "Default")
    if karte != "keine":
        antworten["garnkarte_garne:" + karte] = api(stick, "garnkarte_garne", {"name": karte})
    if o.dialog and json.loads(o.dialog).get("art") == "alphabet":
        for a_ in antworten["alphabete_liste"]:
            antworten["alphabet_vorschau:" + a_["name"]] = api(
                stick, "alphabet_vorschau", {"alphabet": a_["name"], "text": "Aa Bb"})

    build = os.path.join(APP, "build")
    if not os.path.exists(os.path.join(build, "index.html")):
        sys.exit("app/build fehlt – erst `npm run build`")
    tmp = tempfile.mkdtemp(prefix="bildschirmfoto_")
    seite = os.path.join(tmp, "seite")
    shutil.copytree(build, seite)
    start = {"datei": design_pfad}
    if o.experte:
        start["modus"] = "experte"
    start["dunkel"] = o.dunkel
    if o.ansicht:
        start["ansicht"] = o.ansicht
    if o.schritt:
        start["schritt"] = o.schritt
    if o.maschine:
        start["maschine"] = o.maschine
    if o.dialog:
        start["dialog"] = json.loads(o.dialog)
    if o.auswahl is not None:
        start["auswahl"] = o.auswahl
    vorspann = f"""<script>
window.__START__ = {json.dumps(start)};
const ANTWORTEN = {json.dumps(antworten)};
window.__TAURI_INTERNALS__ = {{
  transformCallback: () => 0,
  invoke: (cmd, args) => {{
    if (cmd !== "kern") return Promise.reject({{ meldung: "Bildschirmfoto: Befehl " + cmd, objekt: null }});
    const d = args.daten || {{}};
    const a = ANTWORTEN[args.befehl + ":" + (d.alphabet || d.name || "")] ?? ANTWORTEN[args.befehl];
    if (a === undefined) return Promise.reject({{ meldung: "Bildschirmfoto: " + args.befehl + " nicht vorbereitet", objekt: null }});
    if (a && a.__fehler__) return Promise.reject(a.__fehler__);
    return Promise.resolve(JSON.parse(JSON.stringify(a)));
  }},
}};
</script>"""
    index = os.path.join(seite, "index.html")
    html = open(index, encoding="utf-8").read().replace("<head>", "<head>" + vorspann, 1)
    # die Ersatzseite von SvelteKit nutzt absolute Pfade – für file:// relativ machen
    html = html.replace('"/_app/', '"./_app/').replace('"/favicon', '"./favicon')
    open(index, "w", encoding="utf-8").write(html)

    browser = o.browser or next((b for b in BROWSER if os.path.exists(b)), None)
    if not browser:
        sys.exit("Chrome/Edge nicht gefunden – --browser angeben")
    b, h = o.groesse.split("x")
    ziel = os.path.abspath(o.ziel)
    url = "file:///" + index.replace(os.sep, "/").lstrip("/") + "#/"
    subprocess.run(
        [browser, "--headless=new", "--disable-gpu", "--hide-scrollbars", "--allow-file-access-from-files",
         f"--user-data-dir={os.path.join(tmp, 'profil')}", f"--window-size={b},{h}", "--virtual-time-budget=6000",
         f"--screenshot={ziel}", url],
        capture_output=True,
    )
    shutil.rmtree(tmp, ignore_errors=True)
    print(ziel if os.path.exists(ziel) else "kein Bild entstanden")


if __name__ == "__main__":
    main()
