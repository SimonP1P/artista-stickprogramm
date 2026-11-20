# Rechenkern (Rust)

Stichberechnung für die BERNINA artista 185 nach dem Vorbild BERNINA artista V4. Plan und Grundsatz: `../ARCHITECTURE.md`.

## Crates
| Crate | Inhalt |
|---|---|
| `stick-kern` | Geometrie, Einstellungen (Werte mit Quellen, 8 Stoffe), Sticharten (Lauf, Dreifach, Satin, Tatami), Unterlagen, Nachbearbeitung (Vernähen, Mindeststich 0,3 mm, Auto Jump), Rahmen, Design-Modell mit JSON-Eingabe, **Schrift** (`schrift.rs`: Satinsäulen mit Winkellinien, Textsatz) |
| `stick-formate` | EXP schreiben/lesen (Bernina-Variante), SVG-Vorschau, **Original-Alphabete (ESB) lesen** (`esb.rs`), **Puffer DATEN und BE-Container der artista 185** (`artista.rs`: lesen/schreiben, Verschleierung, Kopf, Farbtabelle, INFO-Felder; Rundweg mit allen 51 Maschinendesigns byte-gleich; **Garnfarbtabelle für Zeiger 0x24048** und **Icon 72 × 62** wie das Original, Anhang ohne Farben; Icon-Regel an 43 Maschinen-Icons geprüft) |
| `stick-maschine` | **Anbindung der artista 185** (Phase 5): serielles Protokoll aus den Testprogrammen v3.6/v4.1 (`protokoll.rs`: `T r M E`, `R`/`N`, `W`/`P S`, Befehlsfeld, Synchronisieren, Baudwechsel, Wiederholung), Abläufe (`maschine.rs`: verbinden, Designliste, Design lesen, Garnfarbbereich, planen, senden, verlassen), echter Port (`port.rs`, Crate `serialport`) und **Simulation** mit virtueller Uhr (`simulation.rs`); 8 Tests gegen die Simulation; am Gerät lesend erprobt (20.11.2026) |
| `stick-cli` | Kommandozeile `stick` (`erzeugen`, `anzeigen`, `alphabete`, `rahmen`, `stoffe`, **`puffer`**: Design-JSON/EXP/gelesenes Design → `*_puffer.bin` für DATEN, `*_farben.bin` (Garnfarbtabelle), `*_icon.bin` (ICON) + `*_puffer.txt` mit INFO-Feldern und Dateinamen; `--farben-im-anhang` = Variante des Gerätetests v4.0); **`maschine`** `ports | pruefen | liste | lesen NR | senden EINGABE [--schreiben [--sticken]]` mit `--port COM3` oder `--simulation`, Mitschnitt nach `--log` |

Später: `stick-vektor`. Schrift steckt in `stick-kern::schrift` + `stick-formate::esb`
(`Analyse_Teil3/09`); Text im Design: `{"art": "text", "text": "Anna", "alphabet": "Drifter", "hoehe": 20}`,
Alphabet-Ordner für die CLI: `--alphabete ORDNER` oder `STICK_ALPHABETE` (`stick alphabete` listet sie).

## Stand (19.11.2026)
**Getreue Übertragung des Python-Prototyps** `../stickprogramm/`. Prüfung: Für die drei Designs in `testdaten/vergleich_*.json`
erzeugte der Rust-Kern **byte-genau dieselben EXP-Dateien** wie Python (damals `testdaten/python/*.exp`,
Test `stick-formate/tests/vergleich_referenz.rs`). Dazu 23 Einzeltests (aus den Python-Tests übernommen).

**Original-Verhalten Stufe 1** (`../Analyse_Teil3/01_stile_und_verbinder.md`): Vernähen nach dem Dialog „Connectors“
(1,0 mm, letztes Objekt ohne Vernähen), variable Unterlagen-Läufe mit Grenzen aus dem Stil. Seitdem sind die Vergleichsdateien
Regressionsreferenzen (`testdaten/referenz/`, Änderungsliste dort in der README).

**Original-Verhalten Stufen 2–9** (gemessen an den echten Stichen der 285 ART-Musterdesigns, `../Analyse_Teil3/02`–`08`):
- **Vernähen:** 2 Schritte vor und zurück.
- **Satin:**
  - Abstand gilt zwischen Einstichen derselben Seite.
  - Er wird auf der längeren Seite gemessen, im Bogen also außen.
- **Unterlagen:**
  - Edge Walk = 0x2747, Mittellauf = 0x2746.
  - Auto Underlay wie im Stil: in Software Defaults aus; eingeschaltet Zickzack.
- **Tatami:**
  - Abstand gilt zwischen Hinreihen, Versatz 0,25 je Reihe.
  - Teilbereiche werden mit einem Laufweg am Rand verbunden statt mit einem Sprung.
- **Laufstich:** Einstich an jedem Eckpunkt der Form.

**Schrift und Kurzstiche (19.11.2026, `../Analyse_Teil3/09`, `10`):**
- `schrift.rs`: Original-Alphabete (ESB, gelesen in `stick-formate/src/esb.rs`), Text im Design; Stil „Schrift“:
  Auto Spacing × 0,9, Kurzstiche an, Auto-Unterlage Zickzack.
- **Kurzstiche** (`sticharten/satin.rs::kurzstiche`, MOD1 `ShortStitchesEffect`): Schwelle 0,5 × Abstand, Gruppen
  bis 5 Stiche, Dreieckstabelle der Faktoren, Zufall ±0,04. Standard aus; je Satin `"kurzstiche": true`.
- **Gruppe** (`design.rs::gruppe_wege`): Teile eines Buchstabens mit Laufweg statt Sprung, wenn verdeckt; Reihenfolge
  mit den wenigsten Sprüngen.

## Bauen und testen (Windows-PC)
Umgebung siehe `../ARCHITECTURE.md` §5 (Build-Ausgaben in `F:\Entwicklung\target`).
```powershell
cd kern
cargo test                                   # alle Tests inkl. Vergleich mit den Referenzen
cargo clippy --all-targets -- -D warnings    # strenge Prüfung
cargo build --release -p stick-cli
F:\Entwicklung\target\release\stick.exe erzeugen testdaten\vergleich_beispiel.json -o ausgabe
F:\Entwicklung\target\release\stick.exe anzeigen ausgabe\vergleich_beispiel.exp --rahmen "artista 130 x 100"
```
Referenzen neu erzeugen (nur bei bewusster Änderung, mit Eintrag in `testdaten/referenz/README.md`):
`F:\Entwicklung\target\release\stick.exe erzeugen testdaten\vergleich_X.json -o testdaten\referenz` (danach die SVG dort löschen).

## Festlegungen für exakte Gleichheit mit Python
- Rundung beim EXP-Export „halb auf gerade“ (`round_ties_even`) wie Pythons `round`.
- Bézier mit `powf` wie Pythons `**`; Rechenreihenfolge der Ausdrücke wie im Prototyp.
- Reihenfolge der Tatami-Teilbereiche wie im Prototyp (geschlossene in Abschlussreihenfolge, dann offene).
