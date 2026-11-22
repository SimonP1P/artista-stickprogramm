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
| E2 Einfacher Modus | Assistent in 4 Schritten: **1** Rahmen-Karten (Umriss aus `rahmen_liste`), „quer einspannen“, 8 Stoffe + Software Defaults mit Werten (`stoffe_liste`) · **2** Startkacheln Text, Monogramm, Bild umwandeln, SVG einlesen, Öffnen; Text (Alphabet-Dialog mit Vorschau aus den ESB-Umrissen und empfohlener Höhe, Höhe (Warnung außerhalb der Empfehlung; grünes Band entfernt 21.11. auf Wunsch), Ausrichtung, Form gerade/Bogen, Garn, Größe/Drehen des Designs, Mittig, Einpassen, fehlende Zeichen, Passt/Passt nicht); Monogramm (Initialen, Rahmen, Höhe) · **3** Farbstopps mit Garn (Marke · Nummer · Name), „Garn ändern“ (Garn von Hand, Garnkarte wählbar), Applikations-Anleitung je Stopp, Kennzahlen, Hinweise für die Person an der Maschine · **4** Speichern/EXP (Senden folgt in E3) | ✅ 21.11.2026 |
| E3 Maschine | Maschinen-Befehle in `kern/stick-api` (App hat nur noch `kern`); Maschinen-Knopf im Kopf (Anschluss, Prüfen, Designs aus der Maschine), Simulationsband; **Senden in 4 Stufen**: Anschluss (+ Anleitung nach Rahmenwechsel, Pflicht-Haken, wenn der Rahmen seit dem letzten Senden gewechselt hat) → Plan nur gelesen (Schreibzugriffe mit Größe, Vorschaubild/Garnfarben wählbar, Hinweise) → „Bestätigen und schreiben“ → Übertragen (Fortschritt je Teil, Wiederholungen, **Abbrechen**, „Im Hintergrund weiter“) → Ergebnis (Farbfolge mit Garnen, Mitschnitt); Fehler mit Code und nächsten Schritten; **Sticken nur über den Gefahrendialog** (3 Haken; überträgt erneut und gibt Befehl 0x08); „Designs aus der Maschine“ (Liste, lesen, ansehen, EXP); Assistent Schritt 4; Senden gesperrt mit Grund. Am Gerät noch nicht erprobt – nur Simulation | ✅ 21.11.2026 (Simulation) |
| E4 Experte I | Menüs **Einfügen** (alle 13 Objektarten inkl. Knopfloch, Bild, SVG; „kommt noch“ deaktiviert) und **Anordnen** (an Anfang/Ende, eins vor/zurück, wie ausgewählt, vor/nach Nr., nach Farbe mit Farbfolge, Überdeckung und Vorschau der Farbwechsel) über `reihenfolge_aendern`, **Warnung bei Überdeckung** mit Rückgängig; Werkzeugleiste (fügt Objekte mit Startform ein); **Objektliste** (Color Film: Ziehen, Duplizieren, Löschen, Marken „über N“ aus `ueberdeckungen`, Fehler); **Inspektor** Objekt (Felder aus dem Schema des Kerns: oben die E-Felder je Art, „Stichart – Feinheiten“ mit X, Garn, Alphabet, Schrift, Füllmuster, Step-Muster, Aussparen, Größe und Lage, Verstärken, Auto Jump, Ein-/Ausstich) und Design (Rahmen, Stoff, Garnkarte, ganzes Design, Start/Ende, kürzeste Verbindung, Kennzahlen, Farbstopps); Tasten Entf, Strg+D, Alt+↑/↓ | ✅ 21.11.2026 |
| E5 Experte II | Vorschau: **Griffe** (gewählte Objekte ziehen = verschieben, Eckgriffe = skalieren, Drehgriff = drehen, Umschalt = 15°-Schritte; Live-Vorschau, danach rechnet der Kern neu), Pfeiltasten 0,5/5 mm, **Ein-/Ausstich setzen** (Werkzeuge, Marken E/A); **Abspielen** (Stickfolge mit Nadel, Farbstopp-Marken, Regler); Inspektor **Einzelwerte** (Parameterkatalog, Ebene Design/Objekt, Herkunft Standard/Stoff/Design/geändert, Zurücksetzen) und **JSON** (Prüfung beim Tippen mit `design_pruefen`, Übernehmen); Dialoge **Füllmuster** (Kacheln aus `fuellmuster_liste`) und **Step-Muster** (Kacheln aus `step_muster_liste`); **einzelne Buchstaben** (W9, nachgeholt): Text wählen, Buchstaben anklicken (`Ergebnis.buchstabe`) → Inspektor „Einzelne Buchstaben“ (`inspektor/Buchstaben.svelte`) | ✅ 21.11.2026 |
| E6 Abschluss | **Knoten-Werkzeug** (W5, Oberfläche: Punkte der Punktlisten `punkte`/`umriss`/`seite_a`/`seite_b`/Löcher ziehen, Entf löscht, Einfg fügt ein – bei Objekten ohne Transformation; **seit W5-Rest:** Bézier-Knoten mit Anfassern (□ Knoten, ○ Anfasser; Nahtstellen ziehen beide Stücke mit), Knopf „In Knoten umwandeln“ für Formen und transformierte Objekte → Kern `in_knoten_umwandeln`; Logik `src/lib/knoten.ts` – nur JSON, keine Geometrie), Darstellung hell/dunkel (Menü Datei, sonst wie das System), Mac: anfangs EXP statt Senden (seit 21.11. Senden wie Windows) | ✅ 21.11.2026 |
| Test Nutzer 21.11. | **Elemente** im einfachen Modus Schritt 2: Kacheln → Element bearbeiten; Knopf „Elemente (n)“ (Liste: antippen = bearbeiten, Papierkorb), „+ Hinzufügen“ öffnet die Kacheln; Text/Monogramm wie bisher, Bild/SVG/Gruppe: Name, Größe, Drehen, Mittig (je Element), Garn je Farbe (`garn`-Dialog mit `alle`), Löschen; „Ganzes Design“ in der Liste. **Gruppen** (`arbeit.svelte.ts`: `elemente`, `auswahlGruppe`, `auswahlTransformieren`, `gruppieren`, `gruppeAufloesen`): Klick wählt die Gruppe, Griffe/Pfeiltasten auf die Gruppe, Anordnen → Gruppieren/Auflösen (Strg+G). Bild/SVG/Design werden mit `design_einfuegen` als Gruppe **eingefügt**. Entf/⌫ in beiden Modi, Griffe halten bis zum neuen Ergebnis (`arbeit.stand`), grünes Höhenband entfernt, **Senden auch am Mac** | ✅ 21.11.2026 |
| Bild umwandeln 21.11. | Dialog „Bild umwandeln“ (`EinlesenDialog.svelte`) mit Vorschau (`bild_vorschau`, erste sofort, danach 250 ms gebündelt): vereinfachtes Bild auf einer Leinwand, weggelassene Farben kariert; Farben, Vereinfachen, Garnfarben (aus bei Karte „Default“), Farbliste mit Haken wie „Omitted Colors“; Experte: Stichart Step/Auto, kleinste Fläche, Glättung. **Bau:** `[profile.dev.package."*"] opt-level = 2` in `src-tauri/Cargo.toml` – der Rechenkern ist auch im Debug-Bau schnell (Foto 35 MP: ~0,4 s statt ~10 s); erster Bau danach dauert länger | ✅ 21.11.2026 |
| Werkzeuge 21.11. | Expertenmodus nach dem Original: Werkzeugleiste mit **Formen** (`einfuegen.ts` `FORMEN`: Linie, Kreis/Oval, Rechteck, freie Form, Säule) statt 14 Sticharten; **Füllung/Umriss** mit Stichart-Menü (`arbeit.vorwahl`; Wahl gilt auch für die gewählten Linien/Flächen, Kern `stichart_aendern`); Inspektor „Stichart“ + „Umrandung dazu“ (`arbeit.umrandungDazu`, Linie gleicher Form, gruppiert); Menü Einfügen: Formen · Schrift · Besondere; **schnelle Tooltips** (`src/lib/Tooltip.svelte`: übernimmt jedes `title`, 0,18 s; `data-tip-rechts` an der Werkzeugleiste); neue Symbole (`icons.ts`: Formen, Sticharten wie gestickt, E/A-Marken) | ✅ 21.11.2026 |
| 4b.7 Motivlinie | Einfügen → **Motivlinie**; Inspektor „Motiv“ öffnet `src/lib/dialoge/MotivDialog.svelte` (Satz wählen, Kacheln aus `motivsatz_motive`); Bild `bildschirmfotos/w_motivlinie.png`, `w_motiv_dialog.png` | ✅ 21.11.2026 |
| Schriften 21.11. | Dialog **„Schrift wählen“** (`src/lib/dialoge/SchriftDialog.svelte`, Kachel `SchriftKachel.svelte`, gefüllte Buchstaben `Umrissbild.svelte`) für beide Modi: Reiter Stick-Alphabete · Auf dem Computer · Google Fonts (Herunterladen per Klick, `google_schrift_laden`) · ★ Favoriten (`localStorage`); Suche, Stil-Filter, Vorschautext, Vorschau erst sichtbarer Kacheln, „stickbar ab … mm“; **Stil-Filter in allen Reitern** (`schrift_stile`, Anzahl je Stil) und **eigene Stile** (Etikett an der Kachel → Leiste „Stile für …“, `localStorage` „schrift_stile_eigen“); Wechsel über `arbeit.schriftWechseln` (`schrift_wechseln`). App setzt die Ablage der Schriften auf den Datenordner (`src-tauri/src/lib.rs`). Bilder `bildschirmfotos/schrift_alphabete.png`, `schrift_computer.png`, `schrift_stile.png`. Das Bildschirmfoto-Werkzeug zeigt Google-Vorschauen nur in der Ersatzschrift (Chrome macht das Bild vor dem Laden der Web-Schrift) | ✅ 21.11.2026 |

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
| `src-tauri/` | Rust-Teil: **nur** der Befehl `kern`, Fortschritt-Ereignis, Fenster mit Startoptionen; Ressourcen `alphabete/` (50 ESB), `garnkarten/` (32 .tch), `motive/` (14 Motivsätze ESB aus `userletw/`) |
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
npm run tauri build -- --debug --no-bundle    # Programm bauen
```
Mac: `CARGO_TARGET_DIR="$PWD/../kern/target"` davorsetzen (**absoluter** Pfad – ein relativer gilt für Cargo ab
`src-tauri/` und legt sonst 2,6 GB unter `app/kern/target` an). Das gebaute Programm liegt dann in
`kern/target/debug/artista-stickprogramm` und findet Alphabete/Garnkarten im Projektordner.

**Web-Inspektor** ist aus (kein „Inspect Element“ im Rechtsklick-Menü, Nutzerwunsch 21.11.2026). Zur Fehlersuche das
Programm mit `ARTISTA_INSPEKTOR=1` starten (Mac: `ARTISTA_INSPEKTOR=1 kern/target/debug/artista-stickprogramm`;
Windows: `$env:ARTISTA_INSPEKTOR=1` vor dem Start).

**Weißes Fenster auf dem Mac (behoben 21.11.2026):** WebKit meldet bei `tauri://localhost` für `location.pathname` „/“,
für `new URL(location.href).pathname` aber „“; das Hash-Routing von SvelteKit hielt die Seite deshalb für fremd und
lud endlos neu. `src/app.html` stellt die Adresse vor dem Start auf `/index.html` (Windows und `file://` unberührt).

### Selbst testen (Kurzanleitung für den Nutzer)
1. Mac: `kern/target/debug/artista-stickprogramm app/werkzeuge/beispiele/handtuch_anna.json` (aus dem Projektordner),
   oder Entwicklungsmodus: in `app/` `. ~/.cargo/env; CARGO_TARGET_DIR="$PWD/../kern/target" npm run tauri dev`.
   Windows: wie oben bauen, Programm `F:\Entwicklung\target\debug\artista-stickprogramm.exe`.
2. **Einfach:** Rahmen/Stoff → Text oder Monogramm → Farbstopps/Garn → Speichern/EXP.
3. **Experte:** Einfügen, Griffe in der Vorschau, Inspektor (Objekt, Design, Einzelwerte, JSON), Anordnen,
   Abspielen, Knoten-Werkzeug (bei Kreisen erst „In Knoten umwandeln“).
4. **Maschine:** Anschluss „Simulation“ ist voreingestellt – Senden-Ablauf gefahrlos durchspielen. Mit COM3 spricht das
   Programm mit der echten Maschine (Schreiben nur nach Bestätigung, Sticken nur über den Gefahrendialog; Ablauf
   `../ROADMAP.md` „Vor Ort“ V10).
Startoptionen: `artista-stickprogramm [DESIGN.json] [--experte] [--dunkel|--hell] [--auswahl N]` (Datei öffnen, Modus,
Darstellung, gewähltes Objekt – für „Öffnen mit“ und Bildschirmfotos).

Bildschirmfoto: `npm run build` und `cargo build -p stick-cli` (in `kern/`), dann
`python3 werkzeuge/bildschirmfoto.py ../stickprogramm/beispiele/beispiel.json bildschirmfotos/x.png --experte --auswahl 2`
(Optionen `--ansicht form`, `--dunkel`, `--schritt 2`, `--dialog '{"art":"alphabet","objekt":0}'`, `--groesse 1280x800`; Beispiel `werkzeuge/beispiele/handtuch_anna.json`). Echte Fensterfotos auf dem Mac bräuchten die
Berechtigung „Bildschirmaufnahme“ für das Terminal.
