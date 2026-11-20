# Funktionsübersicht für Gestaltung und Oberfläche

**Für wen:** Designer (Claude Design) und Frontend-Entwickler. **Was:** jede Funktion, die der Rechenkern kann – was
der Anwender einstellt, Standardwert, wo es im Design-JSON steht. Technische Feldliste:
`stickprogramm/README.md` „Design beschreiben (JSON)“. Was noch kommt: `ROADMAP.md` Phase 4b. Vorbild-Funktionen:
`Analyse_Teil3/15_funktionsinventur.md`. **Stand: 20.11.2026** – wird bei jeder neuen Funktion ergänzt.

**Modus** (Vorschlag, endgültig mit dem Parameterkatalog, ROADMAP M1): **E** = im einfachen Modus sichtbar,
**X** = nur Expertenmodus, **A** = automatisch (Anwender sieht nur das Ergebnis).

## 1. Wie Oberfläche und Kern zusammenarbeiten
- Die Oberfläche hält **ein Design als JSON** (Formen in mm, Ursprung Rahmenmitte, +y oben) und schickt es bei jeder
  Änderung an den Befehl `design_berechnen` (`app/src-tauri/src/lib.rs`). Zurück kommen alle Stiche, Farben, Garne,
  Statistik (Stiche, Sprünge, Größe), fehlende Zeichen und die Zahl der Einstiche außerhalb des Rahmens.
- **Stiche werden immer neu berechnet** (nie skaliert): Größe, Drehung, Stoff usw. ändern das JSON, nicht die Stiche.
- Jede neue Kern-Funktion ist damit sofort nutzbar, sobald die Oberfläche das Feld ins JSON schreibt.

## 2. Rahmen, Stoff, Garn, Design als Ganzes
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Rahmen | Auswahl aus der Liste (artista 130 × 100) | `rahmen` | E |
| Stoff | 8 Stoffe + Software Defaults; ändert Dichte, Unterlage, Zugausgleich | `stoff` | E |
| Garnkarte | Auswahl (Default), Farben werden dem nächsten Garn zugeordnet | `garnkarte` | E |
| Ganzes Design skalieren, drehen, spiegeln, schräg, verschieben | Prozent / Grad / Richtung / mm | `transform` oben | E |
| Start und Ende | Rahmenmitte, Designmitte, unten Mitte, erster/letzter Stich | `start`, `ende` | X |
| Einzelwerte überschreiben | z. B. Satin-Zugausgleich | `einstellungen` | X |

## 3. Objekte (Sticharten)
Formen: Punktliste oder `kreis`, `oval`, `rechteck`, `bogen`, `bezier`, `verbinden`. Alle Objekte: `farbe`, `name`.

| Objektart | Anwender stellt ein (Standard) | JSON `art` | Modus |
|---|---|---|---|
| Laufstich | Linie, Stichlänge (2,5 mm), offen/geschlossen | `lauf` | E (Länge X) |
| Dreifachstich | wie Lauf | `dreifach` | E |
| Satinsäule | zwei Seiten; Stichart Satin oder Zickzack (offen); Unterlage (auto), fester Abstand statt Auto Spacing, Kurzstiche | `satin` (+ `stichart`) | E (Rest X) |
| Satinlinie (Kontur) | Linie, Breite (2 mm), Lage mittig / zur Seite, Satin oder Zickzack | `satinlinie` (+ `stichart`) | E (Lage X) |
| Fläche Tatami (Step) | Umriss, Löcher, Stichwinkel (45°), Unterlage (an); Step-Muster 1–20 (Struktur der Fläche, Vorschau-Kacheln sinnvoll) | `tatami`, `step_muster` | E (Winkel E, Muster E) |
| Fläche mit mehreren Stichwinkeln | Winkellinien quer über die Fläche ziehen (Werkzeug „Stichwinkel“), Stiche drehen mit; ohne Löcher | `tatami` + `winkellinien` | X |
| Füllmuster (Fancy) | Umriss, Muster (242, Liste `kern/stick-kern/daten/fuellmuster.json`), Winkel, Größe (100 %) | `muster` | E |
| Blanket (Langette) | Linie, Abstand (2 mm), Breite (3 mm), mittig/seitlich | `blanket` | E (Werte X) |
| Applikation | Umriss; Abdeckung Satin/Blanket; vorgeschnitten / im Rahmen zuschneiden; Breiten (2,5/3,0 mm) | `applikation` | E (Werte X) |

## 4. Schrift
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Original-Alphabet | 50 Alphabete (`Analyse/daten/alphabets.md`, Bilder `Arbeitsdaten/alphabete/`) | `text` + `alphabet` | E |
| Computer-Schrift (TTF/OTF) | Schriftname; Stichart Tatami/Lauf/Dreifach/Muster | `ttf` + `schrift` | E (Stichart X) |
| Höhe, Breite, Buchstabenabstand | Höhe (10 mm), Breite (100 %), Abstand (aus dem Alphabet) | `hoehe`, `breite`, `abstand` | E (Abstand X) |
| Ausrichtung | links, mitte, rechts, Blocksatz (nur mit fester Grundlinienlänge) | `ausrichtung` | E |
| Mehrere Zeilen | Zeilenumbruch; Zeilenabstand (60 %), Wortabstand (25 %) | `text` mit `\n`, `zeilenabstand`, `wortabstand` | E (Abstände X) |
| Kursiv | −45 … 45° (0) | `kursiv` | E |
| Grundlinie | gerade (Winkel, Länge), Kreis oben/unten, Oval, Bogen, senkrecht, eigene Kurve, Wappen (oben–Mitte–unten); Text über/auf/unter der Linie | `grundlinie` | E (Feinwerte X) |
| Farbwechsel im Text | `^` zwischen Buchstaben, Farbliste | `^` im Text, `farben` | E |
| Einzelne Buchstaben | Farbe, verschieben, drehen, skalieren; neue Stichwinkel (Winkellinien ziehen) | `buchstaben` | X |
| Stichart der Schrift | Satin (Standard), Step, Füllmuster | `stichart` | X |
| Monogramm | 2 oder 3 Initialen, Rahmen Raute / Zier / keiner, Rahmenfarbe; Höhe (18 mm) | `monogramm` | E |
| Verzerren (Elastic) | gerade (Wimpel/schräg) oder gebogen (Bogen/Bauch), Griffe oben/unten | `elastisch` | E |
| Unterlage | automatisch / aus (kleine Schrift) / Arten | `unterlage` | X |

## 5. Bearbeiten (für jedes Objekt)
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Skalieren, drehen, spiegeln, schräg, verschieben | Prozent / Grad / Richtung / mm | `transform` | E |
| Aussparen (Knockout) | Füllfläche spart darüberliegende Objekte aus, Überlappung (0 mm) | `aussparen`, `ueberlappung` | E (Überlappung X) |
| Verstärken (Double Run) | an/aus – gleiche Stiche rückwärts noch einmal | `doppellauf` | X |
| Auto Jump | an/aus, Höchststich (12,7 mm), Sprunglänge (7 mm); kleiner Höchststich = „Satin mit mehr Loft“ | `auto_jump` | X |
| Reihenfolge der Objekte | Reihenfolge im JSON = Stickreihenfolge | Reihenfolge `objekte` | E |

## 6. Weitere Werkzeuge (noch ohne Befehl in der App)
| Funktion | Heute | Für die neue Oberfläche nötig |
|---|---|---|
| Liste der Füllmuster | CLI `stick muster`, Datei `kern/stick-kern/daten/fuellmuster.json` | Befehl „Musterliste“ (Name, Nummer, Vorschau) |
| Computer-Schriften (TTF) laden | CLI `stick schriften`; die App lädt TTF noch nicht | Befehl „Schriftenliste“ + Laden der Schrift beim Berechnen |
| Bild → Umrisse (Auto-Digitalisieren) | CLI `stick bild` (Farben, Breite, Glättung) | Befehl „Bild umwandeln“ → Objekte ins Design |
| An die Maschine senden / aus der Maschine lesen | Befehle `maschine_*` (vorhanden) | – |

Vorhandene Befehle (`app/src-tauri/src/lib.rs`): `rahmen_liste`, `stoffe_liste`, `alphabete_liste`,
`garnkarten_liste`, `beispiel_design`, `design_berechnen`, `datei_lesen`, `text_speichern`, `exp_speichern`,
`maschine_ports`, `maschine_pruefen`, `maschine_planen`, `maschine_senden`, `maschine_liste`, `maschine_lesen`,
`gelesen_exp_speichern`.

## 7. Kommt noch (Platz vorsehen, ROADMAP Phase 4b)
Kurzstiche in Flächen · Reihenfolge nach Farbe/automatisch mit
Überdeckungsprüfung · Knopflöcher · Motivlinien und Motivfüllungen · Effekte (ausgefranster Rand, Welle, Farbverlauf,
Farben mischen) · Craft Stitches (Blackwork, Rückstich, Stielstich, Candlewicking, Stippling) · Applikation-Feinheiten ·
Magic Wand, PhotoSnap · Ausrichten, Gruppieren, Kranz, Kaleidoskop · eigene Rahmen, fremde Stickdateien ·
Vorlagen/Stile speichern · Kreuzstich.
