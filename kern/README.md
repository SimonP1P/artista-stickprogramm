# Rechenkern (Rust)

Stichberechnung für die BERNINA artista 185 nach dem Vorbild BERNINA artista V4. Plan und Grundsatz: `../ARCHITECTURE.md`.

## Crates
| Crate | Inhalt |
|---|---|
| `stick-kern` | Geometrie, Einstellungen (Werte mit Quellen, 8 Stoffe), Sticharten (Lauf, Dreifach, Satin, Tatami), Unterlagen, Nachbearbeitung (Vernähen, Mindeststich 0,3 mm, Auto Jump), Rahmen, Design-Modell mit JSON-Eingabe |
| `stick-formate` | EXP schreiben/lesen (Bernina-Variante), SVG-Vorschau |
| `stick-cli` | Kommandozeile `stick` |

Später: `stick-maschine` (serielles Protokoll), `stick-vektor`, `stick-schrift`.

## Stand (19.11.2026)
**Getreue Übertragung des Python-Prototyps** `../stickprogramm/`. Prüfung: Für die drei Designs in `testdaten/vergleich_*.json`
erzeugt der Rust-Kern **byte-genau dieselben EXP-Dateien** wie Python (`testdaten/python/*.exp`,
Test `stick-formate/tests/vergleich_python.rs`). Dazu 23 Einzeltests (aus den Python-Tests übernommen).

Nächster Schritt laut Grundsatz §1a: die eigenen Festlegungen des Prototyps durch das analysierte Original-Verhalten ersetzen.
Jede bewusste Abweichung vom Prototyp ersetzt die betroffene Referenz in `testdaten/python/` und wird dokumentiert.

## Bauen und testen (Windows-PC)
Umgebung siehe `../ARCHITECTURE.md` §5 (Build-Ausgaben in `F:\Entwicklung\target`).
```powershell
cd kern
cargo test                                   # alle Tests inkl. Vergleich mit Python
cargo clippy --all-targets -- -D warnings    # strenge Prüfung
cargo build --release -p stick-cli
F:\Entwicklung\target\release\stick.exe erzeugen testdaten\vergleich_beispiel.json -o ausgabe
F:\Entwicklung\target\release\stick.exe anzeigen ausgabe\vergleich_beispiel.exp --rahmen "artista 130 x 100"
```
Referenzen neu erzeugen (nur bei bewusster Änderung des Prototyps):
`venv\Scripts\stickprogramm erzeugen kern\testdaten\vergleich_X.json -o kern\testdaten\python` (danach die SVG dort löschen).

## Festlegungen für exakte Gleichheit mit Python
- Rundung beim EXP-Export „halb auf gerade“ (`round_ties_even`) wie Pythons `round`.
- Bézier mit `powf` wie Pythons `**`; Rechenreihenfolge der Ausdrücke wie im Prototyp.
- Reihenfolge der Tatami-Teilbereiche wie im Prototyp (geschlossene in Abschlussreihenfolge, dann offene).
