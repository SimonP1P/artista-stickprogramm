# CLAUDE.md – Projekt „Bernina artista“

> **Diese Datei zuerst lesen.** Sie sagt, wo welches Wissen liegt. Alle Fakten zu Formaten, Werten und Protokollen stammen aus der Analyse der Original-Software und aus Tests an der echten Maschine. **Nicht aus dem Gedächtnis antworten, sondern die hier genannten Dateien lesen und als Quelle angeben.**

## 1. Worum es geht
- **Ausgangsmaterial:** Installations-CD „BERNINA artista Version 4“ (Wilcom-Software, © 1997–2003) in diesem Ordner, dazu eine **BERNINA artista 185** mit Stickmodul beim Nutzer zu Hause (Windows-PC, Original-PC-Kabel an **COM3**). Der Nutzer arbeitet derzeit aus der Ferne (Fernwartung, Person vor Ort schaltet die Maschine).
- **Projektziel (vom Nutzer festgelegt am 18.11.2026):** **Eigene Digitalisier-Software, die Stickbahnen erzeugt – nach dem Vorbild der analysierten BERNINA artista V4 (Wilcom).** Ablauf für den Anwender: **Stickrahmen auswählen → Design anpassen → los geht’s** (an die Maschine senden). **Umfang: alles sticken können** – voller Funktionsumfang wie das Vorbild (alle Sticharten: Lauf, Dreifach, Satin, Tatami/Step, Fancy/Muster, Applikation, Schrift, Unterlagen, Stoffe …). Feine/filigrane Stickerei (schmale Satinbahnen, kleine Schrift, 60er Garn) ist **ein** Anwendungsfall unter vielen, **kein** Schwerpunkt (Korrektur des Nutzers, 18.11.2026).
- **Zielmaschine:** **nur die BERNINA artista 185** (Stickmodul, serielles Kabel). Andere Maschinen (200/730, USB-Stick, INF/BMP) sind **kein** Ziel.
- **Plattform:** **Windows und Mac.** (Die Maschine hängt am Windows-PC; entwickelt wird auch auf dem Mac.)
- **Architektur (19.11.2026):** Tauri 2 + TypeScript/Svelte + Rechenkern in **Rust**; Details, Bausteine und Entwicklungsumgebung in **`ARCHITECTURE.md`**. Das Programm ist **nur für den Nutzer selbst** – GPL-Bausteine (Potrace, Ink/Stitch-Schriften) sind erlaubt.
- **Grundsatz (Nutzer, 19.11.2026):** Stichberechnung, Werte und Abläufe kommen aus dem **analysierten Original** (Vorlagen, Stile, Maschinencode) – keine freien Ersatz-Algorithmen. Lücken zuerst durch weitere Analyse schließen; eigene Festlegungen nur als markierte Notlösung. Freie Bibliotheken nur für neutrale Technik (Oberfläche, Geometrie, Dateien, Port). Ink/Stitch höchstens zum Vergleich (`ARCHITECTURE.md` §1a).
- **Sprache:** Mit dem Nutzer immer **Deutsch** sprechen; Dokumente auf Deutsch.

## 2. Wo steht was – zuerst hier nachsehen
| Thema | Datei (maßgeblich) |
|---|---|
| **Gesamtbericht** (8 Ausgangsfragen, Antworten, Sicherheitsstufen) | `Analyse/ANALYSE.md` |
| Maschinen & Übertragungswege (165–185 seriell, 200 USB/ActiveSync) | `Analyse/ANALYSE.md` §1, `Analyse/code/03_artista200_adx.md` |
| **Standardwerte** (Satin, Tatami, Lauf, Unterlage, Zugausgleich, min/max Stich) | `Analyse/ANALYSE.md` §5, `Analyse/code/10_algorithmen_schmale_bahnen.md` |
| **Algorithmen schmale Bahnen** (Auto-Spacing-Tabelle, EasyStitch-Schwellen) | `Analyse/code/10_algorithmen_schmale_bahnen.md` |
| **Stoffe** (8 Stück, was sich ändert) | `Analyse/code/10_algorithmen_schmale_bahnen.md` §E, Rohdaten `Analyse/daten/entschluesselt/fabrics_diff.txt` |
| Rahmen (Maße, Formen) | `Analyse/daten/hoops_table.md`, Format `Analyse/code/08_rahmen_dsc.md` |
| Garnfarbkarten | `Analyse/daten/garnkarten/` (+ `UEBERSICHT.md`), Format `Analyse/code/09_garnkarten_tch.md` |
| Alphabete, empfohlene Min-/Max-Höhen | `Analyse/daten/alphabets.md` |
| **EXP-Format** | `Analyse/code/07_exp_format.md` |
| **ART/AMT/ESB entschlüsseln** (Format-Keys, Twofish) | `Analyse/code/05_art_amt_verschluesselung.md` |
| DFL-Dateien, Maschinenformat „Bernina“ | `Analyse/code/04_dfl_verschluesselung.md` |
| DataItem-Format, Einheit **1 mm = 180**, Tag-Bedeutungen | `Analyse/code/06_dataitem_und_tags.md` |
| **Serielles Protokoll artista 165–185** (Erkennung, Befehle) | `Analyse_Teil2/01_verbindung_und_erkennung.md`, `Analyse/code/02_serielles_protokoll.md` |
| **Maschinenspeicher, Befehlsfeld 0xFFFED0, Fehlercodes** | `Analyse_Teil2/02_maschinenspeicher_und_kommandos.md` |
| **Design schreiben Schritt für Schritt** (Adressen, Reihenfolge) | `Analyse_Teil2/03_design_schreiben_schritt_fuer_schritt.md` |
| Kartenabbild (128 KB), Verzeichnisfelder | `Analyse_Teil2/04_kartenabbild_felder.md` |
| Designcontainer + „BE“-Verschleierung | `Analyse_Teil2/05_designcontainer_verschleierung.md` |
| **Befund an der echten artista 185** (bestätigt/korrigiert) | `Analyse_Teil2/07_befund_am_geraet.md` |
| **Alle Befehle an die Maschine** (Zeichen, Befehlsfeld, Schreibziele, Risiko, Teststatus) | `Analyse_Teil2/08_befehlsliste.md` |
| Disassembler bedienen, x86 lesen | `Analyse_Teil2/06_disassembler_anleitung.md`, `Analyse/code/11_werkzeuge.md` |
| Handbuch (522 S.) als Text, seitenweise | `Arbeitsdaten/texte/man_raw_pg.txt` (Seitenmarken `=====PAGE n=====`; gedruckte Seite = PDF-Seite − 2) |
| Alle Programm-Strings / Dialoge | `Arbeitsdaten/strings/`, `Arbeitsdaten/ressourcen/` (RESENU = englische Oberfläche) |
| Disassembly BEAGLink.dll / DIALOG.DLL | `Arbeitsdaten/disassembly/`, Einzelfunktionen `Analyse_Teil2/daten/funktionen/` |
| Entschlüsselte Vorlage NORMAL.AMT, alle 22 Stile | `Arbeitsdaten/entschluesselt/NORMAL.AMT_streams/`, `Analyse/daten/entschluesselt/styles.txt`, `software_defaults_flat.txt` |
| Echte Gerätetests (Ergebnis + Mitschnitt) | `Arbeitsdaten/geraetetests/`, `Analyse_Teil2/daten/Mitschnitt_artista185_v*.log` |
| Testprogramme für Windows | `Testprogramm_artista185_Windows/` (v1 Lesetest), `Testprogramm_artista185_Windows_v2/` (v2 + Stickmodul), `Testprogramm_artista185_Windows_v3/` (v3 Designliste – am Gerät gescheitert, Transfer fehlte), `Testprogramm_artista185_Windows_v3_1/` (v3.1 – Transfer nicht bestätigt), `Testprogramm_artista185_Windows_v3_2/` (v3.2: 115200 Baud vor Transfer – am Gerät gescheitert, Ansatz laut Code falsch für die 185), `Testprogramm_artista185_Windows_v3_3/` (v3.3: ohne 0xFFFEE0/115200, Ziele 0xA0 + 0x50 nacheinander; **am Gerät erfolgreich 19.11.2026**: Designliste mit 51 Designs gelesen), `Testprogramm_artista185_Windows_v3_4/` (v3.4: ein Design lesen wie „Card/Machine Read“ mit Befehl **0x400** – am Gerät 0x400 ok, Lesen an Längengrenze/Echo-Aussetzer gescheitert), `Testprogramm_artista185_Windows_v3_5/` (v3.5: Design Nr. 1 vollständig gelesen; `N`-Blöcke wie das Original, Wiederholung + Teil-Datei), `Testprogramm_artista185_Windows_v3_6/` (**v3.6, aktuell, am Gerät erfolgreich 19.11.2026**: mehrere/alle Designs in einem Durchgang `-Nummern 2-51|alle`, Anhang-Prüfung, `-OhneGrenze`; alle 51 Designs gelesen; 0x400 nur mit `-MitLadebefehl` bzw. `START_DESIGN_LESEN.bat`/`START_ALLE_LESEN.bat`; für Lesetests freigegeben, Regel 1a), ZIPs daneben; Quellen in `Arbeitsdaten/testprogramm_quellen/` |
| **Aus der Maschine gelesene Designs** (alle 51: 7 Zeichensätze, 34 Motive, 10 kleine; Puffer DATEN + Icon, Stiche als EXP, Anhang entschleiert, Vorschau, `UEBERSICHT.md`, `kontaktbogen.png`) | `Arbeitsdaten/maschinendesigns/`; Werkzeug `Arbeitsdaten/werkzeuge/maschinen_design.py`; Befund `Analyse_Teil2/07_befund_am_geraet.md` (Test v3.5) |
| Verifikationsprotokoll | `VERIFIKATION.md` |
| Übersicht Rohdaten | `Arbeitsdaten/README.md` |
| **Architektur** (Bausteine, geplanter Aufbau, Datenfluss, Entwicklungsumgebung, Risiken) | `ARCHITECTURE.md` |
| **Original-Verhalten für die eigene Software** (Stile, Verbinder/Vernähen, Unterlagen-Läufe, Dreifach) | `Analyse_Teil3/01_stile_und_verbinder.md` |
| **Messungen an den Original-Stichen**: Objekt ↔ Stiche, **Satinabstand = gleiche Seite**, **0x2746 = Mittellauf / 0x2747 = Edge Walk** (korrigiert) | `Analyse_Teil3/04_messungen_satin.md`; Werkzeug `art_messen.py`, Rohdaten `Arbeitsdaten/messungen/` |
| **Effekte und Auto-Unterlage** (Auto Spacing = Effekt 0x21 am Satin, Stil-Flags 0x2000 = aus / 0x0001 = vorgewählt, Unterlage in Software Defaults aus, Zickzack vorgewählt) | `Analyse_Teil3/05_effekte_und_unterlagen.md` |
| **Tatami (Step) gemessen**: Abstand = zwischen zwei Hinreihen (Nachbarreihen halber Abstand), Versatz 0,25 je Reihe | `Analyse_Teil3/06_messungen_tatami.md` |
| **Laufstich an Eckpunkten, Zickzack-Unterlage, Reihenfolge Unterlage** (gemessen) | `Analyse_Teil3/07_lauf_ecken.md` |
| **Laufwege in Flächen** (Original verbindet Teilbereiche mit Laufstichen statt Sprüngen) | `Analyse_Teil3/08_laufwege_flaechen.md` |
| **Objekte der ART-Musterdesigns** (Datensatzrahmen, 131 Klassen mit Namen/Kennung, Formen als Bézier-Knoten, Satin/Lauf/Tatami/Unterlagen-Parameter) | `Analyse_Teil3/03_art_objekte.md`; Werkzeuge `art_objekte.py`, `art_design.py` |
| **Echte Stiche des Originals** aus den 287 ART-Musterdesigns (Datenblock-Kopf, Stichkodierung, Segmente, y doppelt, gemessenes Vernähen) | `Analyse_Teil3/02_art_stichdaten.md`; Werkzeuge `Arbeitsdaten/werkzeuge/art_lesen.py`, `art_stiche.py` |
| **Original-Alphabete (ESB)**: 50 Alphabete lesbar (Grundlinie, Umrisse, Winkellinien der Satinsäulen, Stichart); Referenzhöhe vermutlich 50 mm; Zeichensätze Nr. 1–7 der Maschine = Drifter, Lisa, Blackboard, Swiss Block, Zurich, Alice, Bamboo bei 50 %; **im Programm: Text mit Original-Alphabet** (`stick-formate::esb`, `stick-kern::schrift`) | `Analyse_Teil3/09_esb_alphabete.md`; Werkzeuge `Arbeitsdaten/werkzeuge/esb_lesen.py`, `esb_satin.py`, Bild `Arbeitsdaten/alphabete/` |
| **Rechenkern Rust** (stick-kern, stick-formate, stick-cli; Vergleich mit Python byte-genau) | `kern/README.md`, Code `kern/`, Vergleichsdesigns `kern/testdaten/` |
| **Programm (Tauri + Svelte)** | `app/README.md`, Code `app/` |
| **Python-Prototyp Phase 1** (Stichberechnung, EXP, Vorschau; Werte mit Quellen, eigene Festlegungen) – Referenz für den Rust-Kern | `stickprogramm/README.md`, Code `stickprogramm/src/stickprogramm/`, Tests `stickprogramm/tests/` |

**Quellen-Hierarchie bei Widersprüchen:** (1) Gerätetest > (2) Maschinencode/entschlüsselte Daten > (3) Handbuch > (4) Ableitung/Vermutung. Die Berichte nennen zu jeder Aussage Fundstelle und Sicherheit (hoch/mittel/niedrig). Diese Kennzeichnung beibehalten.

## 3. Kernfakten (Kurzreferenz – Details in den Dateien oben)
- **Einheit intern:** 1 mm = 180 Einheiten. Stiche im Bernina-Format **min 0,3 mm, max 12,7 mm**; Auto Jump 12,7 → Sprünge zu 7 mm. Maschinentyp **17**.
- **Software Defaults:** Satin 0,40 mm · Tatami 0,40 mm / 4,0 mm · Kontur-Lauf 2,5 mm · Zugausgleich 0,20 mm · Edge Walk 2,2 mm.
- **Auto-Spacing Satin:** 0,5 mm Breite → 0,51 mm Abstand … 4 mm → 0,40 … 8 mm → 0,34; mal Stoff-Faktor (0,95–1,13), Schrift ×0,9.
- **EasyStitch:** Linie ≤ 0,3 mm Lauf, ≤ 2 mm kräftige Linie; Fläche ≤ 8 mm Satin, ≤ 12 mm Tatami 1, darüber Tatami 2.
- **EXP:** 2-Byte-Datensätze (dx, dy, signed, 0,1 mm, +y oben); `80 01` Farbwechsel, `80 02/04` Sprung, `80 82` langer Sprung.
- **ART/AMT/ESB:** OLE; `Contents` Substitution (Key 0xD0 ART / 0xD1 AMT / 0xD2 ESB), `DesignDocument` Twofish-128-CFB1, Schlüssel `"1d43f2ae%08x%08x%08x"`, IV `a885f60d87e90b71c20a81bcc3498af1`.
- **artista 185 (am Gerät bestätigt):** COM3, 19200 8N1 → `T r M E` → Nähmaschine `NMMV04.04` → `J 0 5` (57600) → `B O S` → `E B` → `Y Q` → `O` → Stickmodul `SRMV04.04`. Lesen: `R` + 6 Hexziffern (Echo je Zeichen) → 64 Hexziffern → `O`; Blöcke: `N` + 6 Hexziffern → 256 Byte binär → `O` (~1,6 KB/s). Verwaltungsblock 0xFFFD24, Befehlsfeld 0xFFFED0. **Kein** 0xFFFEE0 und **kein** 115200 bei der 185 (nur bei Geräten, die direkt „SRMV“ melden; Code 0x10005980, 19.11.2026). Speicherziel **0xA0 = Maschinenspeicher, 0x50 = Karte** (am Gerät bestätigt 19.11.2026, v3.3: 51 Designs gelesen, 27 je Bank). Nach 0x100 bleibt die Maschine auf 57600; Neuverbindung ohne Aus/Ein möglich (Synchronisation bei 57600). Schreiben `W`+Adresse+Daten+`?` funktioniert. **0x400** lädt ein Design in den Puffer DATEN (Status 0x0002, 19.11.2026); der Puffer ist nach Aus/Ein leer → direkt danach lesen. Puffer = Kopf 6N+8 + Stichdaten (**EXP-Kodierung**, Ende `80 81`) + BE-Container (Name, Farbtabelle n × 0x86; Entschleierung bestätigt). **Länge 2 (Anhang) wird bei 0x400 nicht zurückgesetzt** – Container prüfen (u32 LE bei +2 = Länge 2). Kapazität max. ~145 KB pro Design.
- **INF/BMP/EmbF5 (USB-Stick) gibt es in dieser Software-Version nicht.**

## 4. Regeln für die Arbeit in diesem Projekt
1. **Niemals Schreib-, Lösch-, Formatier- oder Befehlsfeld-Zugriffe (0xFFFED0, AUSWAHL) an die Maschine senden, ohne dass der Nutzer ausdrücklich zugestimmt hat.** v1/v2 sind reine Lesetests. v3 sendet mit Zustimmung des Nutzers (18.11.2026) nur die Listen-Befehle 0x50/0xA0, 0x30, 0x20, 0x60, 0xC0, 0x100 und schreibt nur ins Auswahlfeld. v3.1/v3.2 schreiben zusätzlich 0x01 nach 0xFFFEE0 (Transfer anfordern, wie Original) – vom Nutzer durch Ausführen + J/N-Abfrage freizugeben. Befehle 0x200/0x800/0x80/0x08 und Datenbereich-Schreiben sind weiterhin tabu ohne neue Zustimmung. Nach jedem Test: Maschine aus/ein empfohlen (bleibt sonst im PC-Modus auf 57600). v3.3 hat gezeigt: Ein neuer Durchlauf klappt auch **ohne** Aus/Ein (Synchronisation bei 57600); ob das Display nach 0x100 normal ist, steht noch aus (`ARCHITECTURE.md` §6).
1a. **Freigabe für selbstständige Gerätetests (Nutzer-Zustimmung 18.11.2026):** Eine Claude-Code-Sitzung auf dem Windows-PC an der Maschine darf **ohne Einzelrückfrage** senden:
   - alles Lesende (`R`, `N`, Firmware-/Verwaltungs-/INFO-Bereiche), Verbindungsaufbau (`T r M E`, `R F ?`, `Y Q`), Baudwechsel (`J` + Code, `E B`);
   - Transfer anfordern: `0x01` → 0xFFFEE0;
   - Befehlsfeld 0xFFFED0 **nur** 0x50, 0xA0, 0x30, 0x20, 0x60, 0xC0, 0x100 und **0x400 zum Lesen eines Designs** (Freigabe des Nutzers 19.11.2026: „für Lesetests generell“); Schreiben **nur** in AUSWAHL +0 / +5.
   **Weiterhin nur nach ausdrücklicher Rückfrage beim Nutzer:** 0x200 (Speichern), 0x800 (Löschen), 0x80 (Formatieren), 0x08 (Sticken), 0x10, 0x40, jedes Schreiben in DATEN/ICON/INFO/Farbtabelle oder andere Adressen, `C`, `P S`, `X` (Reset), `L`/`H`/`G` (Firmware/BIOS), Binärprotokoll `w`/`a`/`d`/`p`.
   Physisches (Nadel hoch, Stickmodus, Aus/Ein, Display ansehen) kann nur die Person vor Ort – dann gezielt darum bitten. Jeden Test mit Mitschnitt nach `Arbeitsdaten/geraetetests/` legen, Befund in `Analyse_Teil2/07_befund_am_geraet.md`, danach committen + pushen.
2. Original-CD-Dateien in diesem Ordner (Setup/, ADX/, HASP/, IE6/, Acrobat/, instoem/ …) **nicht verändern**.
3. Neue Erkenntnisse in die passende Datei aus Abschnitt 2 eintragen (mit Fundstelle + Sicherheit) und hier die Tabelle ergänzen; Rohdaten nach `Arbeitsdaten/`.
4. Werkzeuge: maßgeblich ist `Arbeitsdaten/werkzeuge/`. Kopien in `Analyse/code/` und `Analyse_Teil2/code/` sind für die Lesbarkeit der Berichte da und müssen identisch bleiben.
5. **Speicherplatz auf dem Mac ist fast voll** (~1,8 GB frei): große Zwischenstände nur im Scratchpad, nicht im Projekt. **Windows-PC:** C: ~15 GB frei, **F: ~113 GB frei** → Rust, Node und alle Build-Ausgaben auf F:\Entwicklung (`ARCHITECTURE.md` §5).
6. Testprogramme für Windows: PowerShell 5.1-kompatibel, `.ps1` als UTF-8 **mit BOM** und CRLF; vorher mit Simulation (`-Simulation`) testen.

## 4a. Versionierung
- **GitHub (privat):** https://github.com/SimonP1P/artista-stickprogramm – Branch `main`. Ersetzt das alte Repo `Stickprojekt` (Nutzer will neu anfangen, 18.11.2026).
- **Alles** wird versioniert, auch die Original-CD (Wunsch des Nutzers); `Setup/English/setup.z` (142 MB) über **Git LFS**.
- Nach jedem abgeschlossenen Arbeitsschritt committen und pushen (Wunsch des Nutzers). Commit-Nachrichten auf Deutsch.

- **Windows-PC an der Maschine:** Repo dort klonen mit `$env:GIT_LFS_SKIP_SMUDGE=1; git clone https://github.com/SimonP1P/artista-stickprogramm` (ohne 142-MB-setup.z), dann `claude` im Repo starten. Testprogramme direkt mit `powershell -ExecutionPolicy Bypass -File …ps1 -Port COM3 -OhneNachfrage` aufrufen (die J/N-Abfrage ist durch Regel 1a abgedeckt). Vor Start nach PowerShell 5.1 prüfen; Pfade mit Leerzeichen quoten.

## 5. Reproduzieren / Umgebung
```sh
python3 -m venv venv && venv/bin/pip install -r Arbeitsdaten/werkzeuge/requirements.txt   # pefile capstone olefile
cc -O2 -o blast Arbeitsdaten/werkzeuge/blast.c
python3 Arbeitsdaten/werkzeuge/unz.py Setup/English/setup.z ZIEL ./blast          # Software entpacken (193 MB)
unshield -d ZIEL_ADX x ADX/data1.cab                                              # ADX entpacken
python3 Arbeitsdaten/werkzeuge/dfl_decrypt.py ZIEL/Main/PROGRAM/RES/DEFAULTS.DFL out.bin
cc -O2 -I Arbeitsdaten/werkzeuge -o twofish Arbeitsdaten/werkzeuge/twofish_cfb1.c
venv/bin/python Arbeitsdaten/werkzeuge/rdis.py ZIEL_ADX/App_Executables/BEAGLink.dll full > beaglink.asm
```
- Weitere Aufrufe: `Analyse/code/11_werkzeuge.md`. Wichtig: das Skript heißt `rdis.py`, nicht `dis.py` (kollidiert mit Python-Modul).
- **Windows-PC:** Python nur über `py` (3.14); Umgebung `py -m venv venv`, Programm `venv\Scripts\python -m pip install -e stickprogramm[test]`, Tests `venv\Scripts\python -m pytest stickprogramm`. SVG-Vorschau lässt sich mit Edge headless als PNG ansehen (`msedge --headless=new --screenshot=… file:///….svg`).
- **Windows-PC, Rust/Tauri (eingerichtet 19.11.2026):** Rust + Node unter `F:\Entwicklung`, Build-Ausgaben `F:\Entwicklung\target`; Programm bauen mit `cd app; npm run tauri build -- --debug --no-bundle` (Details `ARCHITECTURE.md` §5, `app/README.md`). In Claude-Code-Shells ggf. PATH/Variablen für den Prozess setzen: `$env:Path = "F:\Entwicklung\cargo\bin;F:\Entwicklung\node;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;$env:Path"; $env:RUSTUP_HOME='F:\Entwicklung\rustup'; $env:CARGO_HOME='F:\Entwicklung\cargo'; $env:CARGO_TARGET_DIR='F:\Entwicklung\target'`.
- **Original-Software auf dem Windows-PC entpackt:** `F:\Entwicklung\artista_ziel` (651 Dateien, 7 bekannte defekte), Werkzeuge `F:\Entwicklung\werkzeuge\blast.exe` (mit `Arbeitsdaten/werkzeuge/binaer_windows.h` gebaut, sonst „blast err 2“) und `twofish.exe`; `setup.z` per `git lfs pull --include Setup/English/setup.z`. Musterdesigns: `F:\Entwicklung\artista_ziel\Main\Samples` (287 ART).
- Vorhandene Mac-Werkzeuge: `unshield`, `pdftotext`, `deark`, `cc`. **Nicht** installiert: Wine, PowerShell (bei Bedarf portable PowerShell 7.4.6 osx-arm64 ins Scratchpad laden).
- **Bekannter Schaden:** Die CD-Kopie hat 14 defekte 64-KB-Blöcke in `setup.z` → ARTISTA.HLP, qrg_enu.pdf, Quilter.hlp, portfolio.exe, vcl60.bpl unbrauchbar (Details `Analyse/code/01_setup_entpacken.md`).

## 6. Stand und Roadmap
| Phase | Inhalt | Stand |
|---|---|---|
| 0 Grundlagen | Software-Analyse, Formate, Werte, Protokoll, Lesetest am Gerät | ✅ erledigt, verifiziert (`VERIFIKATION.md`) |
| 1 Prototyp Stichberechnung | Lauf, Satin mit Auto-Spacing, Tatami, Unterlage, Zugausgleich → EXP + Vorschau, in **Python** | ✅ 19.11.2026: `stickprogramm/` (Lauf, Dreifach, Satin, Tatami mit Löchern, 5 Unterlagen, 8 Stoffe, Rahmen, Vernähen, Auto Jump, EXP lesen/schreiben, SVG), 47 Tests. Bleibt als **Referenz und Prüforakel** für den Rust-Kern (`ARCHITECTURE.md` §4); nicht an der Maschine gestickt |
| 2 Übertragung | Weg zur artista 185: Designliste lesen → ein echtes Design auslesen → Designcontainer verstehen → zuerst „sticken ohne speichern“; Schreiben nur nach Zustimmung. Verlauf: v3 Listenbefehle mit 0x1005 abgelehnt → v3.1/v3.2 (0xFFFEE0, 115200) laut Code der falsche Weg für die 185 → **v3.3 am Gerät erfolgreich (19.11.2026):** Ziel 0xA0 = Maschinenspeicher, **Designliste mit 51 Designs gelesen**; 0x50 = Karte (keine gesteckt); zweiter Durchlauf **ohne** Aus/Ein klappt. **v3.5/v3.6: alle 51 Designs der Maschine vollständig gelesen und entschlüsselt (19.11.2026)**, `Arbeitsdaten/maschinendesigns/`. Offen: Display nach 0x100 ansehen; **Senden (Schreiben/Sticken) – geplant für Sa 21.11.2026, nur nach Zustimmung**. Details `Analyse_Teil2/07_befund_am_geraet.md` | 🟡 Liste lesen ✅, Design lesen ✅, senden offen |
| 3 Grundgerüst neue Architektur | Entwicklungsumgebung auf F: (Rust, Node, MSVC Build Tools); Tauri 2 + Svelte + Rust-Workspace (`ARCHITECTURE.md` §3); Rechenkern in Rust **neu** (Lauf, Dreifach, Satin, Tatami, Unterlagen, Nachbearbeitung, EXP) mit den Prüffällen des Prototyps und Vergleich gegen Python; erste Oberfläche: Rahmen wählen, Design (JSON) laden, Vorschau, EXP speichern | 🟡 begonnen 19.11.2026: Umgebung auf F: eingerichtet, Tauri-Gerüst `app/` baut und startet; **Rust-Kern `kern/` fertig übertragen** (Lauf, Dreifach, Satin, Tatami, Unterlagen, Nachbearbeitung, Rahmen, JSON, EXP, SVG, CLI `stick`) – EXP byte-gleich mit Python für 3 Vergleichsdesigns, 26 Tests, clippy sauber; **erste Oberfläche fertig**: Rahmen/Stoff wählen, Design-JSON anpassen, Stichvorschau, Statistik, EXP speichern (`app/README.md`) |
| 4 Alle Sticharten & Funktionen | voller Umfang wie das Vorbild: Wege durch Flächen statt Sprünge, Start-/Endpunkte, Fancy/Muster, Applikation, Schrift (zuerst Original-Alphabete der CD, dazu TTF), Vektorisierung (Potrace/VTracer), Knockout/Überlappung, Satin-Auto-Split; **eigene Festlegungen durch Original-Verhalten ersetzen** (weitere Analyse der Vorlagen/des Codes), Probestickereien – auch feine Arbeiten | 🟡 begonnen 19.11.2026: Objektteil der ART-Muster entschlüsselt und mit den echten Stichen verglichen (`Analyse_Teil3/03`–`08`); im Kern ersetzt: Satin-Abstand (gleiche Seite, außen im Bogen), Tatami-Abstand (Hinreihen), Auto Underlay, Edge Walk/Mittellauf, Lauf-Ecken, **Laufwege in Flächen statt Sprüngen** (Referenz-Stufen 3–9); **Schrift mit den 50 Original-Alphabeten** (ESB entschlüsselt, Text im Programm, erster Vergleich mit dem Original aus der Maschine; offen: Schrift-Unterlage, Kurzstiche) |
| 5 Maschine im Programm & Komfort | serieller Port aus Rust (`stick-maschine`), Ablauf „Rahmen → anpassen → senden“, Designcontainer schreiben, Garnfarbkarten, Bedienkomfort; Mac-Programm (Mac oder GitHub Actions) | ⬜ |
