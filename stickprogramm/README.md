# Stickprogramm – Python-Prototyp (Phase 1)

> **Prototyp und Referenz.** Das eigentliche Programm entsteht mit Tauri + Svelte + Rust (`../ARCHITECTURE.md`).
> Dieser Prototyp liefert die Werte mit Quellen und die Prüffälle für den Rust-Kern.

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
| `satin` | `seite_a`, `seite_b`, `unterlage`; nur Rust-Kern: `stichart` (`satin` Standard / `zickzack`) | Satinsäule; Punkt *i* von A gehört zu Punkt *i* von B; Zickzack = offener Deckstich, Abstand 2,0 mm (Stoffe 1,0 mm) |
| `satinlinie` | `punkte`, `breite`, `unterlage`, `versatz` (nur Rust-Kern: `mitte` Standard / `seite` = Linie ist die Kante, Säule links davon), `stichart` (nur Rust-Kern: `satin` / `zickzack`) | Satinbahn konstanter Breite um eine Mittellinie |
| `tatami` | `umriss`, `loecher`, `winkel` (45°), `unterlage` (true); nur Rust-Kern: `winkellinien` ([[[x1, y1], [x2, y2]], …] quer über die Fläche, ohne Löcher), dazu `kurzstiche` (Innenkurven) | Füllung mit Tatami-Unterlage; mit Winkellinien Wendefüllung (Stiche folgen den Linien) |
| `text` (nur Rust-Kern) | `text`, `alphabet`, `hoehe`, `breite` (%), `abstand`, `position`, `ausrichtung`, `unterlage` | Original-Alphabet (ESB) |
| `tatami`/`muster`: Aussparen (nur Rust-Kern) | `aussparen` (Liste von Objektnamen), `ueberlappung` (mm, Standard 0) | Knockout wie „Remove Overlay Stitching“ |
| `transform` an jedem Objekt und oben im Design (nur Rust-Kern) | `skalieren` (% oder [x, y]), `drehen` (Grad), `spiegeln` (`links_rechts`/`oben_unten`/`beide`), `schraeg` (Grad), `verschieben` ([dx, dy] mm) | um die Mitte; Stiche werden neu berechnet; Form `oval` (`mitte`, `breite`, `hoehe`) |
| `text`/`ttf`: Zeilen, Abstände, Grundlinien, Farben (nur Rust-Kern) | `\n` im `text` trennt Zeilen; `zeilenabstand` (% der Höhe, Vorlage 60), `wortabstand` (% der Höhe, Vorlage 25); Standardhöhe 10 mm; `kursiv` (Grad, −45 … 45); `ausrichtung` auch `block` (Full: Text füllt die Länge der Grundlinie); `grundlinie`: `{"art": "kreis", "radius": 50, "radius_y", "winkel": 90, "neigung", "bogen", "lage": "oben"/"unten", "stand": "ueber"/"durch"/"unter"}` (`position` = Kreismitte; unten steht der Text standardmäßig außen unter der Linie), `{"art": "gerade", "winkel", "laenge"}`, `{"art": "senkrecht"}` (Zeilen von rechts nach links), `{"art": "kurve", "punkte": Punkte oder Form}`, `{"art": "mehrfach", "radius"}` (Zeile 1 oben, 2 mitten, 3 unten); `^` im Text = Farbwechsel zur nächsten Farbe aus `farben` (Liste); `buchstaben`: `[{"nr": 1, "farbe", "transform", "winkellinien": [[[x1, y1], [x2, y2]], …]}]` (Nr. ohne Leerzeichen/`^`, Transformation um die Buchstabenmitte; Winkellinien = neue Stichwinkel quer über die Säulen, Lage wie der gesetzte Text); `stichart` (`satin` Standard, `step` = Step-Reihen in den Säulen, `muster` mit `muster`, `winkel`, `groesse` = Füllmuster über die Buchstabenfläche; bei `ttf` zusätzlich `muster`); `elastisch` (Elastic Lettering): `{"art": "gerade", "oben": [links, rechts], "unten": [links, rechts]}` oder `{"art": "gebogen", "oben": Mitte oder [links, Mitte, rechts], "unten": …}` (mm, + nach oben) | Werte aus NORMAL.AMT 0x3e82, Dialog „Layout“ (RESENU DLG 1039), Handbuch Kap. 18–19 |
| `muster` (nur Rust-Kern) | `umriss`, `loecher`, `muster` (Name/Nummer, `stick muster`), `winkel`, `groesse` (%), `unterlage` | Füllmuster (Program Split) |
| `applikation` (nur Rust-Kern) | `umriss`, `loecher`, `farbe` (Abdeckung), `breite` (2,5), `abstand`, `heft_breite` (3,0), `heft_abstand` (2,0), `platz_laenge` (2,0), `abdeckung` (`satin`/`blanket`), `stil` (`vorgeschnitten`/`zuschneiden`) | Platzierung → (Schnittlinie) → Stopp → Heften → Stopp → Satin- oder Blanket-Abdeckung |
| `blanket` (nur Rust-Kern) | `punkte`, `geschlossen`, `abstand` (2,0), `breite` (3,0), `versatz` (`mitte`/`seite`) | Blanket (Langettenstich, Stil 0x2727) |
| `monogramm` (nur Rust-Kern) | `initialen` (2 oder 3 Buchstaben A–Z), `rahmen` (`raute`/`zier`/`keine`), `rahmen_farbe`, `hoehe` (18), dazu alle Felder von `text` | Monogramm mit Diamond2/Diamond3 wie im Original (Handbuch Kap. 21, Belegung `Analyse_Teil3/09` §7) |
| `ttf` (nur Rust-Kern) | `text`, `schrift` (Name oder Pfad), `hoehe` (Großbuchstaben), `breite`, `abstand`, `position`, `ausrichtung`, `stichart` (`tatami`/`lauf`/`dreifach`), `winkel`, `unterlage`, `laenge` | Computer-Schrift (TrueType/OpenType) |

Alle Objekte haben `farbe` (`#rrggbb`) und `name`; im Rust-Kern außerdem `"step_muster": 1 … 20` (Tatami-Flächen und Step in Schrift: Versatz der Reihen wie Dialog „Fill Stitch“, Tabelle `Analyse_Teil3/06` §5), `"doppellauf": true` (Double Run: gleiche Stiche rückwärts noch einmal, Handbuch Kap. 23), `"einstich": [x, y]` /
`"ausstich": [x, y]` (Ein-/Ausstich wie die „entry/exit point“-Griffe: offene Linien und Säulen drehen sich um,
geschlossene Formen beginnen und enden am nächsten Punkt, Flächen beginnen am Einstich bzw. gegenüber dem Ausstich;
`Analyse_Teil3/16`) und `"auto_jump"`: `true`/`false` oder `{"max": 6, "sprung": 3}` (Effects-Dialog; Standard an mit 12,7/7,0 mm, bei Schrift aus; aus = lange Stiche in echte Stiche ≤ 12,7 mm geteilt; kleines `max` = Säule mit mehr Loft). Statt einer Punktliste geht eine Form:
`{"kreis": {"mitte", "radius"}}`, `{"rechteck": {"mitte", "breite", "hoehe"}}`, `{"bogen": {"mitte", "radius", "von", "bis"}}`,
`{"bezier": [p0, p1, p2, p3]}`, `{"verbinden": [Form, …]}`.
Satin-Unterlage: `"auto"`, `"keine"` oder Kombination aus `mittellauf`, `edge_walk`, `zickzack`, `doppelzickzack` (z. B. `"edge_walk+zickzack"`).
Oben im Design: `rahmen` (Name aus `stickprogramm rahmen`), `stoff` (einer der 8 Stoffe), `garnkarte` (nur Rust-Kern/Programm: Name einer `.tch`-Karte, Standard `Default`, `keine` = ohne), `kuerzeste_verbindung` (nur Rust-Kern, Standard `false` = wie gezeichnet; `true` = Closest Join: jedes Objekt ohne
eigenen Einstich beginnt am Punkt nächst dem vorigen Ende, außer Schrift), `start`/`ende` (nur Rust-Kern, Dialog „Start and End“: `rahmenmitte` Standard, `designmitte` wie Auto Start/End des Originals, `unten_mitte`, `erster_stich`/`letzter_stich`, Ende auch `start`; `mitte` = `rahmenmitte`), `einstellungen` (einzelne Werte überschreiben, z. B. `{"satin": {"zugausgleich": 0.3}}`).
Die Reihenfolge in `objekte` ist die Stickreihenfolge; umordnen (nach Farbe, an Anfang/Ende, wie ausgewählt, vor/nach
Nr., mit Prüfung auf Überdeckungen) mit `stick reihenfolge DESIGN.json …` (Rust-CLI, `Analyse_Teil3/16`).

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

## Eigene Festlegungen (Notlösungen – werden durch das analysierte Original-Verhalten ersetzt, `../ARCHITECTURE.md` §1a)
| Punkt | Festlegung |
|---|---|
| Satin-Unterlage „auto“ | Breite < 1,5 mm keine · < 3 mm Mittellauf · < 6 mm Edge Walk · ab 6 mm Edge Walk + Zickzack – **im Rust-Kern ersetzt**: Auto Underlay wie im Original (Software Defaults/Stoffe aus, sonst Zickzack, `Analyse_Teil3/05`) |
| Reihenfolge Unterlage | jede Satin-Unterlage endet am Säulenanfang, danach Deckstich in einem Zug – **im Original so gemessen** (`Analyse_Teil3/07` §3) |
| Zickzack-Länge | als Weg entlang der Säule je Zick + Zack gedeutet (Einstich alle Länge/2) – **im Original so gemessen** (`Analyse_Teil3/07` §2) |
| Satin-Fortschritt | Abstand gemessen als Mittel der Wege auf beiden Seiten – **im Rust-Kern ersetzt**: auf der längeren Seite wie im Original (`Analyse_Teil3/04` §4) |
| Satin-Abstand | als Weg zwischen **aufeinanderfolgenden** Einstichen gedeutet – **falsch**: im Original ist es der Weg zwischen zwei Einstichen **auf derselben Seite** (Handbuch, Messung `Analyse_Teil3/04`); **im Rust-Kern ersetzt**, Satin dort doppelt so dicht |
| Tatami-Abstand | als Abstand **benachbarter** Reihen gedeutet – **falsch**: im Original zwischen zwei Hinreihen; **im Rust-Kern ersetzt** (`Analyse_Teil3/06`) |
| Tatami-Versatz | Raster je Reihe um 0,25 × Stichlänge verschoben (im Original so gemessen, `Analyse_Teil3/06`); zwischen Teilbereichen ein **Sprung** (Laufwege durch die Fläche fehlen noch) – **im Rust-Kern ersetzt**: Laufweg am Rand wie im Original (`Analyse_Teil3/08`) |
| Tatami-Winkel | 45°, wenn nicht angegeben |
| Ecken beim Lauf | Einstich an Knicken ≥ 30° – **im Rust-Kern ersetzt**: an jedem Eckpunkt wie im Original (`Analyse_Teil3/07` §1) |
| Vernähen | 0,8 mm vor und zurück am Anfang/Ende, bei Farbwechsel und Verbindern > 3,0 mm – **im Rust-Kern ersetzt** durch das Original (1,0 mm, letztes Objekt ohne Vernähen, `Analyse_Teil3/01`) |
| Start | Rahmenmitte; erster Weg wird per Sprung angefahren. Fadenschnitt gibt es nicht (Bernina-Format: `trims_for_jump = 0`) |

## Noch nicht enthalten (kommt im Rust-Kern, Phase 3–5)
Fancy-/Musterfüllungen, Applikation, Schrift, Satin-Auto-Split, Laufwege statt Sprünge in Flächen, Start-/Endpunkte wählbar,
Garnfarbkarten, Oberfläche. Übertragung an die Maschine: Phase 2 und 5.
