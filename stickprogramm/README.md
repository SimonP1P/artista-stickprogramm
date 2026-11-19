# Stickprogramm (Phase 1: Stichberechnung)

Eigene Digitalisier-Software für die **BERNINA artista 185**, nach dem Vorbild BERNINA artista V4 (Wilcom).
Phase 1 berechnet aus Formen die Stiche und schreibt **EXP** (Maschinenformat) und eine **SVG-Vorschau**.
Reines Python ohne Zusatzpakete, läuft auf Windows und Mac (Python ≥ 3.10).

## Einrichten und starten
```sh
# Windows (Python über den Launcher „py“)
py -m venv venv
venv\Scripts\python -m pip install -e stickprogramm[test]
venv\Scripts\stickprogramm beispiel -o ausgabe

# Mac
python3 -m venv venv
venv/bin/pip install -e "stickprogramm[test]"
venv/bin/stickprogramm beispiel -o ausgabe
```
Befehle:
| Befehl | Wirkung |
|---|---|
| `stickprogramm beispiel -o ORDNER` | Beispiel mit allen Sticharten (siehe `beispiele/`) |
| `stickprogramm erzeugen DESIGN.json -o ORDNER` | Design berechnen → `.exp` + `.svg`, Statistik, Warnung wenn außerhalb des Stickfelds |
| `stickprogramm anzeigen DATEI.exp [--rahmen NAME]` | vorhandene EXP-Datei als SVG darstellen |
| `stickprogramm rahmen` / `stoffe` | Rahmen bzw. Stoffe mit Werten auflisten |

Tests: `venv\Scripts\python -m pytest stickprogramm` (Windows) bzw. `venv/bin/python -m pytest stickprogramm`.

## Design beschreiben (JSON)
Koordinaten in **mm**, Ursprung = **Rahmenmitte**, **+y nach oben**. Beispiel: [`beispiele/beispiel.json`](beispiele/beispiel.json).
| Objekt (`art`) | Felder | Bedeutung |
|---|---|---|
| `lauf` | `punkte`, `laenge` (2,5), `geschlossen` | Laufstich |
| `dreifach` | wie `lauf` | Dreifachstich (vor–zurück–vor) |
| `satin` | `seite_a`, `seite_b`, `unterlage` | Satinsäule; Punkt *i* von A gehört zu Punkt *i* von B |
| `satinlinie` | `punkte`, `breite`, `unterlage` | Satinbahn konstanter Breite um eine Mittellinie |
| `tatami` | `umriss`, `loecher`, `winkel` (45°), `unterlage` (true) | Füllung mit Tatami-Unterlage |

Alle Objekte haben `farbe` (`#rrggbb`) und `name`. Statt einer Punktliste geht eine Form:
`{"kreis": {"mitte", "radius"}}`, `{"rechteck": {"mitte", "breite", "hoehe"}}`, `{"bogen": {"mitte", "radius", "von", "bis"}}`,
`{"bezier": [p0, p1, p2, p3]}`, `{"verbinden": [Form, …]}`.
Satin-Unterlage: `"auto"`, `"keine"` oder Kombination aus `mittellauf`, `edge_walk`, `zickzack`, `doppelzickzack` (z. B. `"edge_walk+zickzack"`).
Oben im Design: `rahmen` (Name aus `stickprogramm rahmen`), `stoff` (einer der 8 Stoffe), `einstellungen` (einzelne Werte überschreiben, z. B. `{"satin": {"zugausgleich": 0.3}}`).

## Aufbau (`src/stickprogramm/`)
| Datei | Inhalt |
|---|---|
| `einstellungen.py` | alle Standardwerte mit Quelle: Auto-Spacing-Tabelle, Satin, Tatami, Lauf, Unterlagen, Verbinder, 8 Stoffe |
| `geometrie.py` | Punkte, Linien, Polygone, Versatz, Grundformen |
| `sticharten/lauf.py` | Laufstich (feste/variable Länge, Ecken), Dreifachstich |
| `sticharten/satin.py` | Satinsäule mit Auto Spacing und Zugausgleich |
| `sticharten/tatami.py` | Tatami-Füllung mit Löchern, Reihenversatz, Bereichszerlegung |
| `sticharten/unterlagen.py` | Mittellauf, Edge Walk, Zickzack, Doppel-Zickzack, Tatami-Unterlage |
| `nachbearbeitung.py` | Verbinden, Vernähen, Mindeststich 0,3 mm, Auto Jump (> 12,7 mm → Sprünge ≤ 7 mm) |
| `design.py` | Objekte und Design → Stichfolge |
| `exp.py` | EXP schreiben und lesen (wie der BERNINA-Leser) |
| `rahmen.py` | artista-Rahmen inkl. echter Form des Ovalrahmens 145 × 255 |
| `vorschau.py` | SVG-Vorschau |

## Woher die Werte kommen
Zahlen aus der Analyse der Original-Software (`Analyse/ANALYSE.md` §5, `Analyse/code/10_algorithmen_schmale_bahnen.md`):
Satin Auto Spacing 0,54 … 0,34 mm je Breite × Stofffaktor · Satin fest 0,40 · Tatami 0,40 / 4,0 mm, Rand 0,40, Versatz 0,25 ·
Lauf 2,5 mm (min. 0,8, Sehnenabstand 0,09) · Zugausgleich 0,20 · Unterlagen Edge Walk/Mittellauf 2,2, Zickzack 5,0 (Stoffe 3,0),
Doppel-Zickzack 7,0 (Stoffe 3,0), Tatami-Unterlage 3,0/4,0 mm 90° versetzt, Randlauf 2,5, Unterlagenrand 0,20 ·
Mindeststich 0,3 mm, Höchststich 12,7 mm, Auto Jump 7,0 mm · Vernähen bei Verbindern > 3,0 mm. EXP nach `Analyse/code/07_exp_format.md`.

## Eigene Festlegungen (nicht aus dem Original – in Phase 3 an Probestickereien prüfen)
| Punkt | Festlegung |
|---|---|
| Satin-Unterlage „auto“ | Breite < 1,5 mm keine · < 3 mm Mittellauf · < 6 mm Edge Walk · ab 6 mm Edge Walk + Zickzack |
| Reihenfolge Unterlage | jede Satin-Unterlage endet am Säulenanfang, danach Deckstich in einem Zug |
| Zickzack-Länge | als Weg entlang der Säule je Zick + Zack gedeutet (Einstich alle Länge/2) |
| Satin-Fortschritt | Abstand gemessen als Mittel der Wege auf beiden Seiten |
| Tatami-Versatz | Raster je Reihe um 0,25 × Stichlänge verschoben; zwischen Teilbereichen ein **Sprung** (Laufwege durch die Fläche fehlen noch) |
| Tatami-Winkel | 45°, wenn nicht angegeben |
| Ecken beim Lauf | Einstich an Knicken ≥ 30° |
| Vernähen | 0,8 mm vor und zurück am Anfang/Ende, bei Farbwechsel und Verbindern > 3,0 mm |
| Start | Rahmenmitte; erster Weg wird per Sprung angefahren. Fadenschnitt gibt es nicht (Bernina-Format: `trims_for_jump = 0`) |

## Noch nicht enthalten (Phase 3/4)
Fancy-/Musterfüllungen, Applikation, Schrift, Satin-Auto-Split, Laufwege statt Sprünge in Flächen, Start-/Endpunkte wählbar,
Garnfarbkarten, Oberfläche. Übertragung an die Maschine: Phase 2.
