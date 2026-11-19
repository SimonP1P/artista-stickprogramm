# Architektur – Stickprogramm für die BERNINA artista 185

Stand: 19.11.2026 (vom Nutzer festgelegt). Roadmap: `CLAUDE.md` §6.

## 1. Rahmenbedingungen
| Punkt | Festlegung |
|---|---|
| Zweck | Eigene Digitalisier-Software nach dem Vorbild BERNINA artista V4 (Wilcom): **Rahmen wählen → Design anpassen → an die Maschine senden**, voller Funktionsumfang |
| Zielmaschine | nur **BERNINA artista 185** (Stickmodul, serielles Kabel, am Windows-PC an COM3) |
| Plattform | **Windows und Mac**, offline |
| Nutzung | **nur für den Nutzer selbst** (19.11.2026) – GPL-Bausteine (Potrace, Ink/Stitch-Schriften) sind damit erlaubt; die Programme werden nicht weitergegeben |

## 2. Bausteine
| Bereich | Wahl | Warum / Hinweis |
|---|---|---|
| Programm-Hülle | **Tauri 2** | echtes Programm für Windows + Mac aus einem Code; klein (wenige MB statt 80–200 MB bei Electron); offline; Zugriff auf Dateien und seriellen Port |
| Oberfläche | **TypeScript + Svelte** | schnelle, moderne Oberfläche; Stichvorschau im Canvas |
| Rechenkern | **Rust** | schnell genug für große Motive, gleiche Ergebnisse auf allen Systemen, gute Geometrie-Bibliotheken |
| Geometrie | Rust: `geo`, `i_overlay` bzw. Clipper2 | Versatz (Unterlagenrand, Zugausgleich), Knockout, Überlappung, Mittellinien – ersetzt die einfachen Eigenbau-Hilfen des Python-Prototyps |
| Vektorisierung | **Potrace** (1–2 Farben, Text; GPL, privat erlaubt) + **VTracer** (mehrfarbig, MIT, in Rust) | Bild → Umrisse |
| Schrift-Umrisse | Rust-Font-Bibliothek (z. B. `ttf-parser`) | Computer-Schriften in Umrisse wandeln |
| Stickschriften | **Ink/Stitch-Schriftbibliothek** (von Hand digitalisiert) | beste Qualität; Lizenz je Schrift in der Schriftdatei vermerken (privat unkritisch). Später prüfen: Original-Alphabete der CD (`Analyse/daten/alphabets.md`) – Format noch nicht analysiert |
| Dateiformate | eigener **EXP-Writer** in der Bernina-Variante (ohne Trim-Codes, `Analyse/code/07_exp_format.md`); **pyembroidery** (MIT) als Vergleich | EXP ist für die 185 nur **Zwischenformat**: auf die Maschine kommt der Designcontainer mit „BE“-Verschleierung (`Analyse_Teil2/05_designcontainer_verschleierung.md`). DST u. a. sind **kein** Ziel |
| Maschinen-Anbindung | serieller Port aus Rust (z. B. Crate `serialport`) | Protokoll nach `Analyse_Teil2/01–08`. **Stand:** Verbinden, Stickmodul, Lesen am Gerät bestätigt; Designliste lesen und Design schreiben **noch nicht** (Phase 2) |
| Qualitäts-Referenz | **Ink/Stitch** (freie Profi-Software, GPL-3) | Vergleichsmaßstab, Ideen für Parameter aus deren Doku |
| Werte-Referenz | Analyse der Original-Software + **Python-Prototyp** `stickprogramm/` | Standardwerte mit Quellen (`stickprogramm/src/stickprogramm/einstellungen.py`), 47 Prüffälle |

## 3. Aufbau (geplant)
```
app/                      Tauri-Programm
  src/                    Svelte-Oberfläche (Rahmen, Design, Vorschau, Senden)
  src-tauri/              Rust: Fenster, Befehle an den Kern
kern/                     Rust-Workspace
  stick-kern/             Geometrie, Sticharten, Unterlagen, Nachbearbeitung, Einstellungen, Stoffe, Rahmen
  stick-formate/          EXP lesen/schreiben, artista-Designcontainer, später ART lesen
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
- die Liste der eigenen Festlegungen (`stickprogramm/README.md`), die in Probestickereien geprüft werden,
- der Python-Prototyp selbst als Vergleich (gleiches JSON rein → Ergebnisse vergleichen).

## 5. Entwicklungsumgebung
| Rechner | Stand 19.11.2026 | Folgerung |
|---|---|---|
| Windows-PC (an der Maschine) | C: 15,5 GB frei, **F: 113 GB frei** (NTFS, „Flugsimulator“) | Rust, Node und alle Build-Ausgaben auf **F:**: `RUSTUP_HOME=F:\Entwicklung\rustup`, `CARGO_HOME=F:\Entwicklung\cargo`, `CARGO_TARGET_DIR=F:\Entwicklung\target`, npm-Cache `F:\Entwicklung\npm-cache`. MSVC Build Tools / Windows SDK (für Tauri unter Windows nötig) legen Teile zwingend auf C: ab – dafür reicht der Platz. WebView2 ist vorhanden |
| Mac | ~1,8 GB frei | Rust-Builds dort erst nach Aufräumen oder mit externer Platte. Alternative: Mac-Programm über **GitHub Actions** (macOS-Runner, `tauri-action`) bauen lassen |

## 6. Offene Punkte / Risiken
1. **Übertragung zur 185** (Phase 2) ist das größte Risiko und unabhängig von der Architektur: Designliste lesen (v3.3),
   dann ein echtes Design auslesen, Designcontainer verstehen, zuerst „sticken ohne speichern“.
2. **Neustart der Maschine:** bisher nach jedem Test aus/ein. Laut Code beendet das Original mit Befehl 0x100 den PC-Modus;
   mit v3.3 prüfen, ob ein zweiter Durchlauf ohne Aus/Ein klappt. Ziel: im fertigen Programm kein Neustart nötig.
3. Unsignierte Mac-Programme: beim ersten Start „Rechtsklick → Öffnen“ (privat ausreichend).
