# CLAUDE.md – Projekt „Bernina artista“

> **Diese Datei zuerst lesen.** Sie sagt, wo welches Wissen liegt. Alle Fakten zu Formaten, Werten und Protokollen stammen aus der Analyse der Original-Software und aus Tests an der echten Maschine. **Nicht aus dem Gedächtnis antworten, sondern die hier genannten Dateien lesen und als Quelle angeben.**

## 1. Worum es geht
- **Ausgangsmaterial:** Installations-CD „BERNINA artista Version 4“ (Wilcom-Software, © 1997–2003) in diesem Ordner, dazu eine **BERNINA artista 185** mit Stickmodul beim Nutzer zu Hause (Windows-PC, Original-PC-Kabel an **COM3**). Der Nutzer arbeitet derzeit aus der Ferne (Fernwartung, Person vor Ort schaltet die Maschine).
- **Projektziel (vom Nutzer festgelegt am 18.11.2026):** **Eigene Digitalisier-Software, die Stickbahnen erzeugt – nach dem Vorbild der analysierten BERNINA artista V4 (Wilcom).** Ablauf für den Anwender: **Stickrahmen auswählen → Design anpassen → los geht’s** (an die Maschine senden). **Umfang: alles sticken können** – voller Funktionsumfang wie das Vorbild (alle Sticharten: Lauf, Dreifach, Satin, Tatami/Step, Fancy/Muster, Applikation, Schrift, Unterlagen, Stoffe …). Feine/filigrane Stickerei (schmale Satinbahnen, kleine Schrift, 60er Garn) ist **ein** Anwendungsfall unter vielen, **kein** Schwerpunkt (Korrektur des Nutzers, 18.11.2026).
- **Zielmaschine:** **nur die BERNINA artista 185** (Stickmodul, serielles Kabel). Andere Maschinen (200/730, USB-Stick, INF/BMP) sind **kein** Ziel.
- **Plattform:** **Windows und Mac.** (Die Maschine hängt am Windows-PC; entwickelt wird auch auf dem Mac.)
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
| Testprogramme für Windows | `Testprogramm_artista185_Windows/` (v1 Lesetest), `Testprogramm_artista185_Windows_v2/` (v2 + Stickmodul), `Testprogramm_artista185_Windows_v3/` (v3 Designliste – am Gerät gescheitert, Transfer fehlte), `Testprogramm_artista185_Windows_v3_1/` (v3.1 – Transfer nicht bestätigt), `Testprogramm_artista185_Windows_v3_2/` (**v3.2, aktuell**: 115200 Baud vor Transfer, 30 s), ZIPs daneben; Quellen in `Arbeitsdaten/testprogramm_quellen/` |
| Verifikationsprotokoll | `VERIFIKATION.md` |
| Übersicht Rohdaten | `Arbeitsdaten/README.md` |

**Quellen-Hierarchie bei Widersprüchen:** (1) Gerätetest > (2) Maschinencode/entschlüsselte Daten > (3) Handbuch > (4) Ableitung/Vermutung. Die Berichte nennen zu jeder Aussage Fundstelle und Sicherheit (hoch/mittel/niedrig). Diese Kennzeichnung beibehalten.

## 3. Kernfakten (Kurzreferenz – Details in den Dateien oben)
- **Einheit intern:** 1 mm = 180 Einheiten. Stiche im Bernina-Format **min 0,3 mm, max 12,7 mm**; Auto Jump 12,7 → Sprünge zu 7 mm. Maschinentyp **17**.
- **Software Defaults:** Satin 0,40 mm · Tatami 0,40 mm / 4,0 mm · Kontur-Lauf 2,5 mm · Zugausgleich 0,20 mm · Edge Walk 2,2 mm.
- **Auto-Spacing Satin:** 0,5 mm Breite → 0,51 mm Abstand … 4 mm → 0,40 … 8 mm → 0,34; mal Stoff-Faktor (0,95–1,13), Schrift ×0,9.
- **EasyStitch:** Linie ≤ 0,3 mm Lauf, ≤ 2 mm kräftige Linie; Fläche ≤ 8 mm Satin, ≤ 12 mm Tatami 1, darüber Tatami 2.
- **EXP:** 2-Byte-Datensätze (dx, dy, signed, 0,1 mm, +y oben); `80 01` Farbwechsel, `80 02/04` Sprung, `80 82` langer Sprung.
- **ART/AMT/ESB:** OLE; `Contents` Substitution (Key 0xD0 ART / 0xD1 AMT / 0xD2 ESB), `DesignDocument` Twofish-128-CFB1, Schlüssel `"1d43f2ae%08x%08x%08x"`, IV `a885f60d87e90b71c20a81bcc3498af1`.
- **artista 185 (am Gerät bestätigt):** COM3, 19200 8N1 → `T r M E` → Nähmaschine `NMMV04.04` → `J 0 5` (57600) → `B O S` → `E B` → `Y Q` → `O` → Stickmodul `SRMV04.04`. Lesen: `R` + 6 Hexziffern (Echo je Zeichen) → 64 Hexziffern → `O`. Verwaltungsblock 0xFFFD24, Befehlsfeld 0xFFFED0. Vor Befehlen Transfer anfordern: 0x01 → 0xFFFEE0, warten auf Bit 0x02. Schreiben `W`+Adresse+Daten+`?` funktioniert. Kapazität max. ~145 KB pro Design.
- **INF/BMP/EmbF5 (USB-Stick) gibt es in dieser Software-Version nicht.**

## 4. Regeln für die Arbeit in diesem Projekt
1. **Niemals Schreib-, Lösch-, Formatier- oder Befehlsfeld-Zugriffe (0xFFFED0, AUSWAHL) an die Maschine senden, ohne dass der Nutzer ausdrücklich zugestimmt hat.** v1/v2 sind reine Lesetests. v3 sendet mit Zustimmung des Nutzers (18.11.2026) nur die Listen-Befehle 0x50/0xA0, 0x30, 0x20, 0x60, 0xC0, 0x100 und schreibt nur ins Auswahlfeld. v3.1/v3.2 schreiben zusätzlich 0x01 nach 0xFFFEE0 (Transfer anfordern, wie Original) – vom Nutzer durch Ausführen + J/N-Abfrage freizugeben. Befehle 0x200/0x800/0x80/0x08 und Datenbereich-Schreiben sind weiterhin tabu ohne neue Zustimmung. Nach jedem Test: Maschine aus/ein (bleibt sonst im PC-Modus auf 57600).
2. Original-CD-Dateien in diesem Ordner (Setup/, ADX/, HASP/, IE6/, Acrobat/, instoem/ …) **nicht verändern**.
3. Neue Erkenntnisse in die passende Datei aus Abschnitt 2 eintragen (mit Fundstelle + Sicherheit) und hier die Tabelle ergänzen; Rohdaten nach `Arbeitsdaten/`.
4. Werkzeuge: maßgeblich ist `Arbeitsdaten/werkzeuge/`. Kopien in `Analyse/code/` und `Analyse_Teil2/code/` sind für die Lesbarkeit der Berichte da und müssen identisch bleiben.
5. **Speicherplatz auf dem Mac ist fast voll** (~1,8 GB frei): große Zwischenstände nur im Scratchpad, nicht im Projekt.
6. Testprogramme für Windows: PowerShell 5.1-kompatibel, `.ps1` als UTF-8 **mit BOM** und CRLF; vorher mit Simulation (`-Simulation`) testen.

## 4a. Versionierung
- **GitHub (privat):** https://github.com/SimonP1P/artista-stickprogramm – Branch `main`. Ersetzt das alte Repo `Stickprojekt` (Nutzer will neu anfangen, 18.11.2026).
- **Alles** wird versioniert, auch die Original-CD (Wunsch des Nutzers); `Setup/English/setup.z` (142 MB) über **Git LFS**.
- Nach jedem abgeschlossenen Arbeitsschritt committen und pushen (Wunsch des Nutzers). Commit-Nachrichten auf Deutsch.

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
- Vorhandene Mac-Werkzeuge: `unshield`, `pdftotext`, `deark`, `cc`. **Nicht** installiert: Wine, PowerShell (bei Bedarf portable PowerShell 7.4.6 osx-arm64 ins Scratchpad laden).
- **Bekannter Schaden:** Die CD-Kopie hat 14 defekte 64-KB-Blöcke in `setup.z` → ARTISTA.HLP, qrg_enu.pdf, Quilter.hlp, portfolio.exe, vcl60.bpl unbrauchbar (Details `Analyse/code/01_setup_entpacken.md`).

## 6. Stand und Roadmap
| Phase | Inhalt | Stand |
|---|---|---|
| 0 Grundlagen | Software-Analyse, Formate, Werte, Protokoll, Lesetest am Gerät | ✅ erledigt, verifiziert (`VERIFIKATION.md`) |
| 1 Stickdatei erzeugen | Grundlegende Stichberechnung (Lauf, Satin mit Auto-Spacing, Tatami, Unterlage, Zugausgleich) → EXP + Vorschau | ⬜ offen |
| 2 Übertragung | Weg zur artista 185 festlegen (Kabel/Karte); Schreiben erst nach Zustimmung, zuerst „sticken ohne speichern“; dafür hilfreich: ein echtes Design aus der Maschine lesen (v3 am Gerät: `W`-Schreiben + Befehl 0x50 ok, Listenbefehle abgelehnt, weil Transfer-Anforderung 0xFFFEE0 fehlte → v3.1: 0x01 in 0xFFFEE0 angenommen, Bit 0x02 kam nicht → v3.2 (115200 Baud wie Original, Nadel oben) ausgeliefert 18.11.2026, Test ausstehend) | 🟡 Ablauf bekannt, nicht getestet |
| 3 Alle Sticharten & Funktionen | vollständiger Umfang wie das Vorbild (Lauf, Dreifach, Satin, Tatami/Step, Fancy/Muster, Applikation, Schrift, Unterlagen, Stoffe), Probestickereien zur Kalibrierung – auch feine Arbeiten | ⬜ |
| 4 Komfort | Oberfläche (Windows + Mac), Rahmenauswahl, Schriften | ⬜ |
