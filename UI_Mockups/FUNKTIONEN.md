# Funktionsübersicht für Gestaltung und Oberfläche

**Für wen:** Designer (Claude Design) und Frontend-Entwickler. **Was:** jede Funktion, die der Rechenkern kann – was
der Anwender einstellt, Standardwert, wo es im Design-JSON steht. Technische Feldliste (aus dem Code erzeugt, immer
aktuell): **`SCHNITTSTELLE.md`**, maschinenlesbar `kern/schnittstelle/`. Was noch kommt: `ROADMAP.md` Phase 4b. Vorbild-Funktionen:
`Analyse_Teil3/15_funktionsinventur.md`. **Stand: 21.11.2026** (4b.16 Werte je Objekt; 4b.0: Befehle über `stick-api`, erzeugte Feldliste `SCHNITTSTELLE.md`) – wird bei jeder neuen Funktion ergänzt.

**Modus:** **E** = im einfachen Modus sichtbar, **X** = nur Expertenmodus, **A** = automatisch (Anwender sieht nur
das Ergebnis). **Alle einzelnen Werte** (Stichabstand, Zugausgleich, Unterlagen, Vernähen …) mit Standard, Bereich,
Quelle und Modus stehen im **Parameterkatalog `PARAMETER.md`** (maschinenlesbar `kern/stick-kern/daten/parameter.json`).

## 1. Wie Oberfläche und Kern zusammenarbeiten
- Die Oberfläche hält **ein Design als JSON** (Formen in mm, Ursprung Rahmenmitte, +y oben) und schickt es bei jeder
  Änderung an den Befehl `design_berechnen`. Zurück kommen alle Stiche, **je Stich die Objektnummer** (Auswahl und
  Hervorheben), Farben, Garne, Statistik (Stiche, Sprünge, Größe), fehlende Zeichen, die Zahl der Einstiche außerhalb
  des Rahmens und Hinweise (z. B. Felder, die der Kern nicht liest).
- Zum Bearbeiten mit der Maus liefert `design_formen` je Objekt die fertigen Umrisse, Löcher, Linien, Satinseiten,
  Winkellinien, das umschließende Rechteck und Ein-/Ausstich – die Oberfläche rechnet keine Geometrie selbst.
- **Stiche werden immer neu berechnet** (nie skaliert): Größe, Drehung, Stoff usw. ändern das JSON, nicht die Stiche.
- **Alle Befehle laufen über einen Tauri-Befehl `kern`** (`kern/stick-api`): `kern("design_berechnen", {design})`
  aus `kern/schnittstelle/kern.ts`. Neue Kern-Befehle brauchen keine Änderung an der App.
- **Feld- und Befehlsliste werden aus dem Code erzeugt** (nicht von Hand): `SCHNITTSTELLE.md` (lesbar),
  `kern/schnittstelle/design.schema.json` (Design-Format mit Standardwerten, Auswahlwerten, Beschreibungen),
  `befehle.schema.json`, `kern.ts` (TypeScript-Typen). Auswahllisten (Stoffe, Rahmen, Stichart …) kommen aus dem
  Schema bzw. den Listen-Befehlen. Diese Datei hier sagt, **was** eine Funktion für den Anwender ist und in welchem
  **Modus** sie erscheint.
- Fehler kommen als `{meldung, objekt}` – `objekt` (ab 0) zeigt, welches Objekt die Oberfläche markieren soll.
- Objektnummern in Befehlen und Ergebnissen sind **ab 0** (Stelle in `objekte`); in der Objektliste für den Anwender ab 1.
- Jede neue Kern-Funktion ist sofort nutzbar, sobald die Oberfläche das Feld ins JSON schreibt.

## 2. Rahmen, Stoff, Garn, Design als Ganzes
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Rahmen | Auswahl aus der Liste (artista 130 × 100) | `rahmen` | E |
| Stoff | 8 Stoffe + Software Defaults; ändert Dichte, Unterlage, Zugausgleich | `stoff` | E |
| Garnkarte | Auswahl (Default), Farben werden dem nächsten Garn zugeordnet | `garnkarte` | E |
| Ganzes Design skalieren, drehen, spiegeln, schräg, verschieben | Prozent / Grad / Richtung / mm | `transform` oben | E |
| Start und Ende | Rahmenmitte, Designmitte, unten Mitte, erster/letzter Stich | `start`, `ende` | X |
| Kürzeste Verbindung (Closest Join) | an/aus (aus = wie gezeichnet): jedes Objekt beginnt nahe dem vorigen Ende, Linien werden dafür umgedreht, geschlossene Formen gedreht – weniger Sprünge | `kuerzeste_verbindung` | X |
| Einzelwerte überschreiben (ganzes Design) | z. B. Satin-Zugausgleich; Werte im Parameterkatalog | `einstellungen` | X |

## 3. Objekte (Sticharten)
Formen: Punktliste oder `kreis`, `oval`, `rechteck`, `bogen`, `bezier`, `verbinden`. Alle Objekte: `farbe`, `name`,
`einstich`/`ausstich` (Abschnitt 5).

| Objektart | Anwender stellt ein (Standard) | JSON `art` | Modus |
|---|---|---|---|
| Laufstich | Linie, Stichlänge (2,5 mm), offen/geschlossen | `lauf` | E (Länge X) |
| Dreifachstich | wie Lauf | `dreifach` | E |
| Satinsäule | zwei Seiten; Stichart Satin oder Zickzack (offen); Unterlage (auto), fester Abstand statt Auto Spacing, Kurzstiche | `satin` (+ `stichart`) | E (Rest X) |
| Satinlinie (Kontur) | Linie, Breite (2 mm), Lage mittig / zur Seite, Satin oder Zickzack | `satinlinie` (+ `stichart`) | E (Lage X) |
| Fläche Tatami (Step) | Umriss, Löcher, Stichwinkel (45°), Unterlage (an); Step-Muster 1–20 (Struktur der Fläche, Vorschau-Kacheln sinnvoll) | `tatami`, `step_muster` | E (Winkel E, Muster E) |
| Fläche mit mehreren Stichwinkeln | Winkellinien quer über die Fläche ziehen (Werkzeug „Stichwinkel“), Stiche drehen mit; ohne Löcher; Kurzstiche an Innenkurven | `tatami` + `winkellinien`, `kurzstiche` | X |
| Satinfläche (Satin mit automatischer Stichrichtung) | Umriss, Löcher – das Programm zerlegt die Fläche in Satinsäulen (an Abzweigungen geteilt, Stiche quer zum Strich, Enden spitz/flach wie die Form); Unterlage (auto), Satin/Zickzack; Teilen am Knick (aus, bei Schrift 87°) | `satinflaeche` | E (Knick X) |
| Füllmuster (Fancy) | Umriss, Muster (242, Liste `kern/stick-kern/daten/fuellmuster.json`), Winkel, Größe (100 %) | `muster` | E |
| Blanket (Langette) | Linie, Abstand (2 mm), Breite (3 mm), mittig/seitlich | `blanket` | E (Werte X) |
| Applikation | Umriss; Abdeckung Satin/Blanket; vorgeschnitten / im Rahmen zuschneiden; Breiten (2,5/3,0 mm) | `applikation` | E (Werte X) |

## 4. Schrift
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Original-Alphabet | 50 Alphabete (`Analyse/daten/alphabets.md`, Bilder `Arbeitsdaten/alphabete/`) | `text` + `alphabet` | E |
| Computer-Schrift (TTF/OTF) | Schriftname; Stichart Tatami/Lauf/Dreifach/Muster/**Satin** (Säulen wie die TrueType-Umwandlung des Originals) | `ttf` + `schrift` | E (Stichart X) |
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
| **Werte je Objekt** (Objekteigenschaften) | jeder Stil-Wert nur für dieses Objekt: Stichabstand, Stichlänge, Mindeststich am Rand, Zugausgleich, Auto Spacing, Kurzstiche, Unterlage-Werte, Lauf fest/variabel, Vernähen/Verbinder – Aufbau wie `einstellungen` des Designs (ohne Stoff). Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt. Anzeige: Eigenschaften-Fenster des Objekts, Standard = Wert des Designs/Stoffs, geänderte Werte markieren und einzeln zurücksetzen | `einstellungen` am Objekt (Pfade wie `PARAMETER.md`, Ebene „beide“) | X |
| Verstärken (Double Run) | an/aus – gleiche Stiche rückwärts noch einmal | `doppellauf` | X |
| Auto Jump | an/aus, Höchststich (12,7 mm), Sprunglänge (7 mm); kleiner Höchststich = „Satin mit mehr Loft“ | `auto_jump` | X |
| Reihenfolge der Objekte | Reihenfolge im JSON = Stickreihenfolge; Liste wie die „Color Film“ des Originals (Nr, Farbe, Art, Name), Ziehen zum Umordnen | Reihenfolge `objekte` | E |
| Umordnen (Menü „Anordnen“) | **nach Farbe** (Farbfolge mit Hoch/Runter wählbar, weniger Farbwechsel), **an den Anfang / ans Ende**, **wie ausgewählt** (Objekte in Klickfolge), **vor/nach Objekt Nr.** – auf die Auswahl oder alles; Ergebnis = neues JSON | Kern `stick_kern::reihenfolge::ausfuehren` | E |
| Überdeckung beachten | Häkchen beim Sortieren nach Farbe (aus wie im Original): Objekte, die sich überdecken, behalten ihre Lage; **Warnung**, wenn ein Umordnen ein Objekt unter eine Fläche schiebt, über der es lag (z. B. Kontur unter die Füllung) | – (Werkzeug) | E |
| Einstich / Ausstich | zwei Griffe am Objekt („entry/exit point“): Objekt beginnt bzw. endet dort (Linien drehen sich um, geschlossene Formen beginnen am nächsten Punkt, Flächen beginnen am Einstich) | `einstich`, `ausstich` ([x, y] mm) | X |

## 6. Befehle (`kern/stick-api`, Ein-/Ausgabe in `SCHNITTSTELLE.md` §5)
| Befehl | Wofür in der Oberfläche |
|---|---|
| `rahmen_liste`, `stoffe_liste`, `alphabete_liste`, `garnkarten_liste` | Auswahllisten Schritt 1 (Rahmen, Stoff) und Schrift/Garn |
| `garnkarte_garne` | Garn je Farbe von Hand wählen (alle Garne einer Karte mit Farbe, Nummer, Name) |
| `fuellmuster_liste` | Auswahl der 242 Füllmuster mit Vorschau-Kacheln (Linien einer Kachel + Gitter A/B) |
| `schriften_liste` | Auswahl der Computer-Schriften (installierte TrueType/OpenType) |
| `parameter_katalog` | Formulare für Einstellungen und Expertenmodus (Standard, Bereich, Einheit, Modus) |
| `beispiel_design` | Beispiel zum Ausprobieren |
| `design_schema` | Design-Format zur Laufzeit (dasselbe wie `design.schema.json`) |
| `design_pruefen` | schnelle Prüfung beim Tippen (Fehler mit Objektnummer, Hinweise), ohne Stiche |
| `design_berechnen` | Stichvorschau, Kennzahlen, Garne – bei jeder Änderung |
| `design_formen` | Formansicht und Griffe zum Bearbeiten mit der Maus |
| `exp_speichern` | Datei → Stickdatei speichern |
| `reihenfolge_aendern` | Menü „Anordnen“: nach Farbe, an Anfang/Ende, wie ausgewählt, vor/nach Nummer – mit Warnungen bei Überdeckung |
| `ueberdeckungen` | Anzeige, welche Objekte sich überdecken (z. B. in der Objektliste) |
| `saeulen_vorschau` | Vorschau der automatischen Satin-Stichrichtung einer Fläche (optional) |
| `bild_umwandeln` | Bild → Umrisse (Auto-Digitalisieren): neues Design aus einem Bild |
| `datei_lesen`, `text_speichern` | Design öffnen/speichern |
| Maschine: `maschine_ports`, `maschine_pruefen`, `maschine_planen`, `maschine_senden`, `maschine_liste`, `maschine_lesen`, `gelesen_exp_speichern` | noch als eigene Befehle in der App (`app/src-tauri/src/lib.rs`), kommen später in `stick-api` |

Ausprobieren ohne Oberfläche: `stick api liste`, `stick api design_berechnen DATEI.json` (Eingabe wie in
`SCHNITTSTELLE.md`).

## 7. Kommt noch (Platz vorsehen, ROADMAP Phase 4b)
**SVG-Import** (vorgezogen): Design aus Inkscape laden – Flächen, Konturen, Farben und Reihenfolge werden
übernommen, Stichart je Form automatisch nach Breite (schmale Flächen als `satinflaeche`) und in der App änderbar · Knopflöcher · Motivlinien und Motivfüllungen · Effekte (ausgefranster Rand, Welle, Farbverlauf,
Farben mischen) · Craft Stitches (Blackwork, Rückstich, Stielstich, Candlewicking, Stippling) · Applikation-Feinheiten ·
Magic Wand, PhotoSnap · Ausrichten, Gruppieren, Kranz, Kaleidoskop · eigene Rahmen, fremde Stickdateien ·
Vorlagen/Stile speichern · Kreuzstich.
