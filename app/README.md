# Artista Stickprogramm – Programm (Tauri 2 + SvelteKit + Rust)

> **Neue Oberfläche nach Entwurf v3** (`../UI_Mockups/v3_2026-11-21/UEBERGABE.md`, verbindlich seit 21.11.2026), Umsetzung
> in Etappen E0–E6 (`../ROADMAP.md` Phase 5). Die alte Oberfläche (Prototyp vom 19./20.11.) liegt unter
> `archiv/prototyp_2026-11-19/` – nur zum Nachschlagen, nicht mehr im Bau.

## Grundsätze
- **Ein Tauri-Befehl `kern`** (`src-tauri/src/lib.rs`) reicht alles an `stick_api::ausfuehren` weiter (Rechnung im
  Hintergrund). Die Oberfläche ruft nur `kern(…)` aus `kern/schnittstelle/kern.ts` auf – eingebunden per Alias
  **`$kern`** (`svelte.config.js`), nicht kopiert; die Typen passen damit immer zum Kern.
- **Keine Geometrie und keine Stichberechnung in TypeScript:** Vorschau aus `design_berechnen`, Formen/Griffe aus
  `design_formen`, Auswahl über `objekt` je Stich (Trefferprüfung des Browsers auf den Pfaden je Objekt), Fehler als
  `{meldung, objekt}`, Hinweise mit Objektnummer.
- Fehlt ein Feld oder Befehl: im Kern ergänzen (`CLAUDE.md` §4 Regel 7), dann `stick api exportieren`.
- Auch die Maschinen-Befehle (`maschine_*`) laufen über `kern` (`kern/stick-api/src/maschine.rs`, seit E3); Fortschritt
  als Ereignis `maschine_fortschritt`, Mitschnitte im Log-Ordner. Schreiben/Sticken nur nach Plan + Bestätigung
  (`ANFORDERUNGEN.md` §4); Zustand und Abläufe der Oberfläche: `src/lib/maschine.svelte.ts`.

## Stand
| Etappe | Inhalt | Stand |
|---|---|---|
| E1 Grundgerüst | Fensteraufbau (Kopf, Assistent/Objektliste, Vorschau, Statusleiste), Modus Einfach/Experte, Hell/Dunkel, Datei-Menü (Neu, Öffnen, Speichern, Beispiel, EXP), Rückgängig/Wiederholen (Strg+Z/Y, Tippen zusammengefasst), Berechnung entprellt (140 ms) mit Verwerfen veralteter Antworten, Stichvorschau (Zoom, Verschieben, Sprünge/Einstiche/Raster, Auswahl per Klick, Auswahlrahmen mit Maß, Formansicht, Einstiche außerhalb, Fehler am Objekt), Statusleiste (Kennzahlen, Datengröße/145 KB, Stickzeit als Schätzung, Hinweise mit „Zeigen“) | ✅ 21.11.2026 |
| E2 Einfacher Modus | Assistent in 4 Schritten: **1** Rahmen-Karten (Umriss aus `rahmen_liste`), „quer einspannen“, 8 Stoffe + Software Defaults mit Werten (`stoffe_liste`) · **2** Startkacheln Text, Monogramm, Bild umwandeln, SVG einlesen, Öffnen; Text (Alphabet-Dialog mit Vorschau aus den ESB-Umrissen und empfohlener Höhe, Höhe mit grünem Band, Ausrichtung, Form gerade/Bogen, Garn, Größe/Drehen des Designs, Mittig, Einpassen, fehlende Zeichen, Passt/Passt nicht); Monogramm (Initialen, Rahmen, Höhe) · **3** Farbstopps mit Garn (Marke · Nummer · Name), „Garn ändern“ (Garn von Hand, Garnkarte wählbar), Applikations-Anleitung je Stopp, Kennzahlen, Hinweise für die Person an der Maschine · **4** Speichern/EXP (Senden folgt in E3) | ✅ 21.11.2026 |
| E3 Maschine | Maschinen-Befehle in `kern/stick-api` (App hat nur noch `kern`); Maschinen-Knopf im Kopf (Anschluss, Prüfen, Designs aus der Maschine), Simulationsband; **Senden in 4 Stufen**: Anschluss (+ Anleitung nach Rahmenwechsel, Pflicht-Haken, wenn der Rahmen seit dem letzten Senden gewechselt hat) → Plan nur gelesen (Schreibzugriffe mit Größe, Vorschaubild/Garnfarben wählbar, Hinweise) → „Bestätigen und schreiben“ → Übertragen (Fortschritt je Teil, Wiederholungen, **Abbrechen**, „Im Hintergrund weiter“) → Ergebnis (Farbfolge mit Garnen, Mitschnitt); Fehler mit Code und nächsten Schritten; **Sticken nur über den Gefahrendialog** (3 Haken; überträgt erneut und gibt Befehl 0x08); „Designs aus der Maschine“ (Liste, lesen, ansehen, EXP); Assistent Schritt 4; Senden gesperrt mit Grund. Am Gerät noch nicht erprobt – nur Simulation | ✅ 21.11.2026 (Simulation) |
| E4 Experte I | Menüs **Einfügen** (alle 13 Objektarten inkl. Knopfloch, Bild, SVG; „kommt noch“ deaktiviert) und **Anordnen** (an Anfang/Ende, eins vor/zurück, wie ausgewählt, vor/nach Nr., nach Farbe mit Farbfolge, Überdeckung und Vorschau der Farbwechsel) über `reihenfolge_aendern`, **Warnung bei Überdeckung** mit Rückgängig; Werkzeugleiste (fügt Objekte mit Startform ein); **Objektliste** (Color Film: Ziehen, Duplizieren, Löschen, Marken „über N“ aus `ueberdeckungen`, Fehler); **Inspektor** Objekt (Felder aus dem Schema des Kerns: oben die E-Felder je Art, „Stichart – Feinheiten“ mit X, Garn, Alphabet, Schrift, Füllmuster, Step-Muster, Aussparen, Größe und Lage, Verstärken, Auto Jump, Ein-/Ausstich) und Design (Rahmen, Stoff, Garnkarte, ganzes Design, Start/Ende, kürzeste Verbindung, Kennzahlen, Farbstopps); Tasten Entf, Strg+D, Alt+↑/↓ | ✅ 21.11.2026 |
| E5 Experte II | Vorschau: **Griffe** (gewählte Objekte ziehen = verschieben, Eckgriffe = skalieren, Drehgriff = drehen, Umschalt = 15°-Schritte; Live-Vorschau, danach rechnet der Kern neu), Pfeiltasten 0,5/5 mm, **Ein-/Ausstich setzen** (Werkzeuge, Marken E/A); **Abspielen** (Stickfolge mit Nadel, Farbstopp-Marken, Regler); Inspektor **Einzelwerte** (Parameterkatalog, Ebene Design/Objekt, Herkunft Standard/Stoff/Design/geändert, Zurücksetzen) und **JSON** (Prüfung beim Tippen mit `design_pruefen`, Übernehmen); Dialoge **Füllmuster** (Kacheln aus `fuellmuster_liste`) und **Step-Muster** (Kacheln aus `step_muster_liste`) | ✅ 21.11.2026 |
| E6 | siehe `../ROADMAP.md` Phase 5 | ⬜ |

## Ordner
| Pfad | Inhalt |
|---|---|
| `src/routes/+page.svelte` | Hauptfenster |
| `src/lib/arbeit.svelte.ts` | Zustand: Design-JSON, Verlauf, Auswahl, Modus, Ergebnisse des Kerns, Startoptionen |
| `src/lib/dateien.svelte.ts` | Menü Datei (Dialoge von Tauri, Lesen/Schreiben über den Kern) |
| `src/lib/Vorschau.svelte`, `Statusleiste.svelte`, `Icon.svelte`, `icons.ts`, `zahl.ts` | Bausteine |
| `src/lib/stil/industry.css`, `app.css` | Designsystem „Industry“ und Stile des Entwurfs (unverändert übernommen); Schriften Barlow über `@fontsource` (offline) |
| `src/lib/Objektliste.svelte`, `inspektor/` (Inspektor, Feldzeile, Abschnitt), `schema.svelte.ts` (Felder aus `design_schema`), `einfuegen.ts`, `dialoge/NachFarbeDialog`, `WarnungDialog` | Expertenmodus |
| `src/lib/maschine.svelte.ts`, `dialoge/SendenDialog.svelte`, `StickenDialog.svelte`, `AusMaschineDialog.svelte` | Maschine: Zustand, Senden in Stufen, Gefahrendialog, Designs aus der Maschine |
| `src-tauri/` | Rust-Teil: **nur** der Befehl `kern`, Fortschritt-Ereignis, Fenster mit Startoptionen; Ressourcen `alphabete/` (50 ESB), `garnkarten/` (32 .tch) |
| `werkzeuge/bildschirmfoto.py` | Bildschirmfoto ohne Fenster (gebaute Oberfläche in Chrome/Edge headless, Antworten aus `stick api`) |
| `bildschirmfotos/` | Bilder je Etappe |
| `archiv/prototyp_2026-11-19/` | alte Oberfläche |

## Starten und bauen
Mac (Rust in `~/.cargo`, seit 21.11.2026) und Windows-PC (Umgebung `../ARCHITECTURE.md` §5, `CLAUDE.md` §5):
```sh
cd app
npm install                                   # einmalig
npm run check                                 # Typprüfung (svelte-check)
npm run tauri dev                             # Entwicklungsmodus
npm run tauri build -- --debug --no-bundle    # Programm bauen (Mac: CARGO_TARGET_DIR=../kern/target)
```
Startoptionen: `artista-stickprogramm [DESIGN.json] [--experte] [--dunkel|--hell] [--auswahl N]` (Datei öffnen, Modus,
Darstellung, gewähltes Objekt – für „Öffnen mit“ und Bildschirmfotos).

Bildschirmfoto: `npm run build` und `cargo build -p stick-cli` (in `kern/`), dann
`python3 werkzeuge/bildschirmfoto.py ../stickprogramm/beispiele/beispiel.json bildschirmfotos/x.png --experte --auswahl 2`
(Optionen `--ansicht form`, `--dunkel`, `--schritt 2`, `--dialog '{"art":"alphabet","objekt":0}'`, `--groesse 1280x800`; Beispiel `werkzeuge/beispiele/handtuch_anna.json`). Echte Fensterfotos auf dem Mac bräuchten die
Berechtigung „Bildschirmaufnahme“ für das Terminal.
