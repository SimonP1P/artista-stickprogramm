# Architektur – Stickprogramm für die BERNINA artista 185

Stand: 19.11.2026 (vom Nutzer festgelegt). Roadmap: `ROADMAP.md`.

## 1. Rahmenbedingungen
| Punkt | Festlegung |
|---|---|
| Zweck | Eigene Digitalisier-Software nach dem Vorbild BERNINA artista V4 (Wilcom): **Rahmen wählen → Design anpassen → an die Maschine senden**, voller Funktionsumfang |
| Zielmaschine | nur **BERNINA artista 185** (Stickmodul, serielles Kabel, am Windows-PC an COM3) |
| Plattform | **Windows und Mac**, offline |
| Nutzung | **nur für den Nutzer selbst** (19.11.2026) – GPL-Bausteine (Potrace, Ink/Stitch-Schriften) sind damit erlaubt; die Programme werden nicht weitergegeben |

## 1a. Grundsatz: das Vorbild ist maßgeblich (Nutzer, 19.11.2026)
- **Stichberechnung, Werte und Abläufe kommen aus dem analysierten Original** (BERNINA artista V4 / Wilcom):
  Vorlagen und Stile aus NORMAL.AMT, DEFAULTS.DFL, Handbuch, Maschinencode. Keine freien Ersatz-Algorithmen,
  nur weil sie verfügbar sind.
- **Fehlt in der Analyse etwas, wird zuerst das Original weiter untersucht** (Vorlagen, Disassembly, Handbuch,
  Vergleichsstickerei), bevor etwas selbst festgelegt wird. Eigene Festlegungen sind Notlösungen, werden als solche
  markiert (wie in `stickprogramm/README.md`) und später durch das Original-Verhalten ersetzt.
- **Freie Bibliotheken nur für neutrale Technik:** Programmhülle, Oberfläche, reine Geometrie (Versatz, Schnittmengen),
  Schriftdateien lesen, Bild → Umriss, serieller Port. Sie entscheiden **nicht**, wie gestickt wird.
- **Ink/Stitch** dient höchstens als Vergleich der Ergebnisse, nicht als Quelle für Algorithmen oder Werte.

## 2. Bausteine
| Bereich | Wahl | Warum / Hinweis |
|---|---|---|
| Programm-Hülle | **Tauri 2** | echtes Programm für Windows + Mac aus einem Code; klein (wenige MB statt 80–200 MB bei Electron); offline; Zugriff auf Dateien und seriellen Port |
| Oberfläche | **TypeScript + Svelte** | schnelle, moderne Oberfläche; Stichvorschau im Canvas |
| Rechenkern | **Rust** | schnell genug für große Motive, gleiche Ergebnisse auf allen Systemen, gute Geometrie-Bibliotheken |
| Geometrie | Rust: `geo`, `i_overlay` bzw. Clipper2 | Versatz (Unterlagenrand, Zugausgleich), Knockout, Überlappung, Mittellinien – ersetzt die einfachen Eigenbau-Hilfen des Python-Prototyps |
| Vektorisierung | **Potrace** (1–2 Farben, Text; GPL, privat erlaubt) + **VTracer** (mehrfarbig, MIT, in Rust) | Bild → Umrisse |
| Schrift-Umrisse | Rust-Font-Bibliothek (z. B. `ttf-parser`) | Computer-Schriften in Umrisse wandeln |
| Stickschriften | **zuerst die Original-Alphabete der CD** (`Analyse/daten/alphabets.md`, Format noch zu analysieren); Ink/Stitch-Schriften nur, wenn ein Alphabet dort fehlt | Grundsatz 1a; Schrift-Regeln (Faktor 0,9, kein Auto Jump, Mindesthöhen) aus der Analyse |
| Dateiformate | eigener **EXP-Writer** in der Bernina-Variante (ohne Trim-Codes, `Analyse/code/07_exp_format.md`); **pyembroidery** (MIT) als Vergleich | EXP ist für die 185 nur **Zwischenformat**: auf die Maschine kommt der Designcontainer mit „BE“-Verschleierung (`Analyse_Teil2/05_designcontainer_verschleierung.md`). DST u. a. sind **kein** Ziel |
| Maschinen-Anbindung | serieller Port aus Rust (z. B. Crate `serialport`) | Protokoll nach `Analyse_Teil2/01–08`. **Stand:** Verbinden, Stickmodul, Designliste und alle 51 Designs lesen am Gerät bestätigt (Testprogramme v3.3–v3.6); Design senden **noch nicht** (Phase 2, `ROADMAP.md`) |
| **Maßstab für Stiche** | **Analyse der Original-Software** (`Analyse/`, `Analyse_Teil2/`) | Grundsatz 1a; Werte mit Quellen bereits in `stickprogramm/src/stickprogramm/einstellungen.py`, 47 Prüffälle im Python-Prototyp |
| Vergleich | Ink/Stitch (GPL-3) | nur Ergebnisse nebeneinanderlegen, keine Quelle |

## 3. Aufbau (geplant)
```
app/                      Tauri-Programm
  src/                    Svelte-Oberfläche (Rahmen, Design, Vorschau, Senden)
  src-tauri/              Rust: Fenster, Befehle an den Kern
kern/                     Rust-Workspace
  stick-kern/             Geometrie, Sticharten, Unterlagen, Nachbearbeitung, Einstellungen, Stoffe, Rahmen   ✅ 19.11.2026
  stick-formate/          EXP lesen/schreiben, artista-Designcontainer, später ART lesen   ✅ EXP + SVG + ESB
  stick-maschine/         serielles Protokoll artista 185 (mit Simulation wie die Testprogramme)
  stick-vektor/           Bild → Umrisse (Potrace, VTracer)
  stick-schrift/          Schriften → Umrisse, Stickschriften
stickprogramm/            Python-Prototyp (Phase 1) – Referenz und Prüforakel
```
**Datenfluss:** Formen / Bild / Schrift → Umrisse → **Design-Modell** (Objekte mit Stichart und Parametern, Rahmen, Stoff)
→ Rechenkern → **Stichfolge** (Stich / Sprung / Farbwechsel, mm) → Vorschau · EXP · Designcontainer → serieller Port.

**Gemeinsame Festlegungen**
- Koordinaten in **mm**, Ursprung = **Rahmenmitte**, **+y nach oben** (wie EXP). Für ART-Dateien: 1 mm = 180 Einheiten.
- Das **Design-JSON** des Prototyps (`stickprogramm/README.md`) bleibt Austauschformat. Damit lassen sich Rust-Kern und
  Python-Prototyp mit denselben Eingaben vergleichen (Anzahl Stiche, Positionen mit Toleranz, Regeln 0,3–12,7 mm).
- Jeder Wert im Kern nennt seine Quelle (Analyse mit Sicherheitsstufe oder „eigene Festlegung“), wie im Prototyp.
- Maschinenzugriffe folgen den Regeln 1/1a in `CLAUDE.md`: Schreiben, Speichern, Löschen, Sticken nur nach
  ausdrücklicher Bestätigung; jede Sitzung mit Mitschnitt.

## 4. Was aus Phase 1 (Python) übernommen wird
Der Rechenkern wird in Rust **neu geschrieben, nicht Zeile für Zeile übersetzt** – mit richtigen Geometrie-Bibliotheken
(Versatz, Wege innerhalb von Flächen statt Sprüngen). Übernommen werden:
- alle Standardwerte und Stofftabellen mit Quellen (`einstellungen.py`),
- die Prüffälle (47 Tests) als Rust-Tests,
- die Liste der eigenen Festlegungen (`stickprogramm/README.md`) – **nicht** als Dauerlösung, sondern als Arbeitsliste:
  jede wird durch das Original-Verhalten ersetzt (Grundsatz 1a). Ansatzpunkte in der Analyse: Unterlagenwahl und
  -werte je Stil (AMT `0x27a7`), Vernähen/Tie-in/Tie-off (Stil-Tags im AMT), Tatami-Reihenversatz (zwei Werte
  `0x2719`), Satin-Auto-Split (`0x2722`), Korrekturwert `0x2739`, Verbinder (`0x272a`), Zickzack-Unterlage (`0x2744/45`),
- der Python-Prototyp selbst als Vergleich (gleiches JSON rein → Ergebnisse vergleichen).

## 5. Entwicklungsumgebung
| Rechner | Stand 19.11.2026 | Folgerung |
|---|---|---|
| Windows-PC (an der Maschine) | C: 15,5 GB frei, **F: 113 GB frei** (NTFS, „Flugsimulator“) | Rust, Node und alle Build-Ausgaben auf **F:**: `RUSTUP_HOME=F:\Entwicklung\rustup`, `CARGO_HOME=F:\Entwicklung\cargo`, `CARGO_TARGET_DIR=F:\Entwicklung\target`, npm-Cache `F:\Entwicklung\npm-cache`. MSVC Build Tools / Windows SDK (für Tauri unter Windows nötig) legen Teile zwingend auf C: ab – dafür reicht der Platz. WebView2 ist vorhanden |
| Windows-PC – **eingerichtet 19.11.2026** | Rust 1.100.0 (rustup, Profil minimal + clippy/rustfmt), Node 26.13.1 LTS (ZIP), npm-Cache – alles unter **F:\Entwicklung** (`rustup`, `cargo`, `node`, `target`, `npm-cache`, `downloads`). Prüfsummen von rustup-init und Node geprüft. Benutzer-Variablen: `RUSTUP_HOME`, `CARGO_HOME`, `CARGO_TARGET_DIR=F:\Entwicklung\target`; PATH ergänzt um `F:\Entwicklung\cargo\bin`, `F:\Entwicklung\node` und `C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64` (dort liegt `rc.exe`, das Tauri sonst nicht findet). **Keine** neue Installation mit Adminrechten: vorhandene **VS Build Tools 2019** (MSVC 14.29) + Windows SDK 10.0.19041 genügen. Erster Build `app/` (Debug): 90 s, Programm startet |
| Mac | ~1,8 GB frei | Rust-Builds dort erst nach Aufräumen oder mit externer Platte. Alternative: Mac-Programm über **GitHub Actions** (macOS-Runner, `tauri-action`) bauen lassen |

## 6. Offene Punkte / Risiken
1. **Übertragung zur 185** (Phase 2): Lesen ist erledigt (Liste, alle 51 Designs, Container verstanden). Offen ist das
   Senden – Container vorher ohne Maschine byte-gleich nachbauen, dann am Gerät zuerst „sticken ohne speichern“
   (`ROADMAP.md` Phase 2).
2. **Neustart der Maschine:** bisher nach jedem Test aus/ein. Laut Code beendet das Original mit Befehl 0x100 den PC-Modus.
   **v3.3 (19.11.2026):** 0x100 wird angenommen (Status 0x0000), die Maschine bleibt auf 57600, ein zweiter Durchlauf
   ohne Aus/Ein klappt (Synchronisation bei 57600). Offen: Display danach normal? Ziel: im fertigen Programm kein Neustart nötig.
3. Unsignierte Mac-Programme: beim ersten Start „Rechtsklick → Öffnen“ (privat ausreichend).
