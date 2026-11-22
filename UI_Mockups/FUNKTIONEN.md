# Funktionsübersicht für Gestaltung und Oberfläche

**Für wen:** Designer (Claude Design) und Frontend-Entwickler. **Was:** jede Funktion, die der Rechenkern kann – was
der Anwender einstellt, Standardwert, wo es im Design-JSON steht. Technische Feldliste (aus dem Code erzeugt, immer
aktuell): **`SCHNITTSTELLE.md`**, maschinenlesbar `kern/schnittstelle/`. Was noch kommt: `ROADMAP.md` Phase 4b. Vorbild-Funktionen:
`Analyse_Teil3/15_funktionsinventur.md`. **Stand: 21.11.2026** (Wünsche der Oberfläche v3: Hinweise mit Objektnummer, Farbblöcke, Garn von Hand, Alphabet-Vorschau, Step-Muster-Kacheln; 4b.6 Knopflöcher; 4b.17 SVG einlesen; 4b.16 Werte je Objekt; 4b.0: Befehle über `stick-api`, erzeugte Feldliste `SCHNITTSTELLE.md`) – wird bei jeder neuen Funktion ergänzt.

**Modus:** **E** = im einfachen Modus sichtbar, **X** = nur Expertenmodus, **A** = automatisch (Anwender sieht nur
das Ergebnis). **Alle einzelnen Werte** (Stichabstand, Zugausgleich, Unterlagen, Vernähen …) mit Standard, Bereich,
Quelle und Modus stehen im **Parameterkatalog `PARAMETER.md`** (maschinenlesbar `kern/stick-kern/daten/parameter.json`).

## 1. Wie Oberfläche und Kern zusammenarbeiten
- Die Oberfläche hält **ein Design als JSON** (Formen in mm, Ursprung Rahmenmitte, +y oben) und schickt es bei jeder
  Änderung an den Befehl `design_berechnen`. Zurück kommen alle Stiche, **je Stich die Objektnummer** (Auswahl und
  Hervorheben), Farben, Garne, Statistik (Stiche, Sprünge, Größe), fehlende Zeichen, die Zahl der Einstiche außerhalb
  des Rahmens (dazu `ausserhalb_stiche` zum Markieren), `begrenzung` (für Einpassen/Mittig), Datengröße
  (`daten_bytes` gegen `daten_grenze` 145 426 Byte der artista 185), je Farbblock `bloecke` (Objekte, Garn von Hand,
  Lagen einer Applikation für die Anleitung an den Stopps) und Hinweise **mit Objektnummer** (`{meldung, objekt, art}`:
  Feld nicht gelesen, fehlende Zeichen, außerhalb; Schrift `schrift_step` = Info, `lange_satinstiche` = Warnung). Stickzeit liefert der Kern nicht (keine Quelle) – nur als
  gekennzeichnete Schätzung anzeigen.
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
| **Garn von Hand** | Garn aus der Karte wählen (Liste `garnkarte_garne`); setzt `garn` = Nummer und `farbe` = Farbe des Garns; „von Hand“ ist im Ergebnis erkennbar (`bloecke[i].von_hand`). Ohne Angabe: nächstes Garn zur Farbe | `garn` am Objekt | E |
| Ganzes Design skalieren, drehen, spiegeln, schräg, verschieben | Prozent / Grad / Richtung / mm | `transform` oben | E |
| Start und Ende | Rahmenmitte, Designmitte, unten Mitte, erster/letzter Stich | `start`, `ende` | X |
| **SVG einlesen** (z. B. aus Inkscape) | Datei wählen; Breite in mm (Standard: Größe aus der SVG), Rahmen; „Überdeckung aussparen“ (an: was in der SVG oben liegt, spart darunterliegende Flächen aus), „kürzeste Verbindung“ (an). Ergebnis: neues Design, Stichart je Form nach Breite wie EasyStitch des Originals (Linie bis 2 mm Laufstich, Fläche bis 8 mm Satinfläche, darüber Tatami), danach jedes Objekt normal änderbar. Hinweise anzeigen (Text nicht umgewandelt, Verläufe, zu groß für den Rahmen) | Befehl `svg_lesen` → Design | E (Optionen X) |
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
| Satinfläche (Satin mit automatischer Stichrichtung) | Umriss, Löcher – das Programm zerlegt die Fläche in Satinsäulen (an Abzweigungen geteilt, Stiche quer zum Strich, Enden spitz/flach wie die Form); Unterlage (auto), Satin/Zickzack; Teilen am Knick (aus, bei Schrift 87°; mit Teilung überlappen die Säulen an den Schnitten 0,3 mm) | `satinflaeche` | E (Knick X) |
| Füllmuster (Fancy) | Umriss, Muster (242, Liste `kern/stick-kern/daten/fuellmuster.json`), Winkel, Größe (100 %) | `muster` | E |
| Blanket (Langette) | Linie, Abstand (2 mm), Breite (3 mm), mittig/seitlich | `blanket` | E (Werte X) |
| Applikation | Umriss; Abdeckung Satin/Blanket; vorgeschnitten / im Rahmen zuschneiden; Breiten (2,5/3,0 mm) | `applikation` | E (Werte X) |
| Knopfloch | Werkzeug „Knopfloch einfügen“: Klick setzt die **Mitte des Schlitzes**; Typ (9: Standard, Standard schmal, Rund, Stretch/dehnbar, Heirloom, rundes Ende, Auge eckig/rund/spitz – Vorschaubilder sinnvoll), Schlitzlänge (20 mm, 3–60), Winkel (90° = senkrecht; Griffe zum Drehen/Ziehen wie „Reshape“), Raupenabstand (ohne Angabe wie die Vorlage: 0,5 mm, Heirloom 1,5 mm; 0,2–2,0). **Einarbeiten** in eine Fläche darunter („Merge Buttonhole“): Name des Knopflochs in `aussparen` der Fläche (Fläche wird bis zur Raupenmitte ausgespart). Braucht den Alphabet-Ordner (`ButtonHoles.ESB`) | `knopfloch` (`typ`, `position`, `laenge`, `winkel`, `abstand`) | E (Abstand X) |
| Motivlinie | Werkzeug „Motivlinie“ (Handbuch Kap. 25 „Pattern Run“): Linie zeichnen wie beim Lauf (offen/geschlossen, Kreis, Bézier); **Motivsatz** (14, `motive_liste`) und **Motiv** wählen – Auswahl mit Vorschaubildern aus `motivsatz_motive` (Linien in mm, Grundlinie von [0,0] nach [laenge,0]); Größe = Länge eines Motivs entlang der Linie (ohne Angabe wie im Satz), Abstand (ohne Angabe = Größe), Spiegeln. Anzahl und Verteilung macht der Kern wie das Original (gerundet, gleichmäßig verteilt) | `motivlinie` (`punkte`, `geschlossen`, `satz`, `motiv`, `groesse`, `abstand`, `spiegeln`) | E (Abstand X) |

## 4. Schrift
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| **Schrift wählen** (Dialog, beide Modi) | Reiter **Stick-Alphabete** (50) · **Auf dem Computer** (installierte + heruntergeladene Schriften mit lateinischen Buchstaben) · **Google Fonts** (1 833, Katalog im Programm) · **★ Favoriten** (Stern an jeder Kachel, `localStorage` „schrift_favoriten“, Kennung `a:`/`s:`/`g:` + Name); Suche (Name, Stil), **Stil-Filter in allen Reitern** mit Anzahl (`schrift_stile`: Gotisch, Schreibschrift, Handschrift, Pinsel, Modern, Klassisch, Elegant, Verspielt, Kindlich, Vintage, Kräftig, Rund, Technik, Schablone, Pixel, Verziert, Weihnachten, Valentinstag, Halloween, Applikation, Monogramm, Großbuchstaben, Andere Schriften; Stile je Schrift in `alphabete_liste`/`schriften_liste`/`google_schriften_liste` → `stile`, eigene Einordnung), **eigene Stile** (Etikett an der Kachel: Stile setzen/entfernen, neuen Stil anlegen, „Vorgabe“; `localStorage` „schrift_stile_eigen“), **Vorschautext** (Standard: erste Zeile des Texts). Wechsel Alphabet ↔ Computer-Schrift über `schrift_wechseln` (Text, Höhe, Satz, Unterlage bleiben; Stichart auto ↔ auto, Satin ↔ Satin, Step ↔ Tatami, sonst `auto`) | `text.alphabet` / `ttf.schrift` | E |
| Original-Alphabet | 50 Alphabete mit empfohlener Höhe (Handbuch, `alphabete_liste`: `min_hoehe`/`max_hoehe`) und Vorschau (`alphabet_vorschau`: Umrisse eines Probetexts, Zeichenvorrat) | `text` + `alphabet` | E |
| Computer-Schrift (TTF/OTF) | Schriftname; Stichart Tatami/Lauf/Dreifach/Muster/**Satin** (Säulen wie die TrueType-Umwandlung des Originals)/**auto** (wie bei Texten, Step dann je Buchstabe als eine Tatami-Fläche; neue Computer-Schriften der Oberfläche bekommen `auto`); **Unterlage** wie Texte (Satin/auto: Stil „Schrift“ = Zickzack oder die gewählten Arten; Tatami: Step-Unterlage, aus nur mit `"keine"`). **„stickbar ab … mm“** (`schrift_vorschau.stickbarkeit`, eigene Messung: dünnster Strich ≥ 0,8 mm wie bei den Original-Alphabeten an ihrer kleinsten empfohlenen Höhe) an der Kachel, im einfachen Modus am Schrift-Knopf und als Warnung unter der Höhe | `ttf` + `schrift` | E (Stichart X) |
| Google Fonts laden | Klick auf eine Google-Kachel lädt **eine** Datei (aufrecht, Regular) aus github.com/google/fonts in den Datenordner des Programms (`…/schriften`), danach ohne Internet. Vorschau vor dem Laden: Web-Schrift von fonts.googleapis.com mit festem Wort „Stickerei“ – an Google geht nur der Schriftname, nie der Text des Anwenders. Freie Lizenzen (OFL/Apache/UFL) | `google_schrift_laden` → `ttf.schrift` = Familie | E |
| Höhe, Breite, Buchstabenabstand | Höhe (10 mm), Breite (100 %), Abstand (aus dem Alphabet) | `hoehe`, `breite`, `abstand` | E (Abstand X) |
| Ausrichtung | links, mitte, rechts, Blocksatz (nur mit fester Grundlinienlänge) | `ausrichtung` | E |
| Mehrere Zeilen | Zeilenumbruch; Zeilenabstand (60 %), Wortabstand (25 %) | `text` mit `\n`, `zeilenabstand`, `wortabstand` | E (Abstände X) |
| Kursiv | −45 … 45° (0) | `kursiv` | E |
| Grundlinie | gerade (Winkel, Länge), Kreis oben/unten, Oval, Bogen, senkrecht, eigene Kurve, Wappen (oben–Mitte–unten); Text über/auf/unter der Linie | `grundlinie` | E (Feinwerte X) |
| Farbwechsel im Text | `^` zwischen Buchstaben, Farbliste | `^` im Text, `farben` | E |
| Einzelne Buchstaben | Farbe, verschieben, drehen, skalieren; neue Stichwinkel (Winkellinien ziehen). Auswahl per Klick auf den Buchstaben: `design_berechnen` liefert je Stich `buchstabe` (Nr. wie `buchstaben[].nr`, 0 = kein Buchstabe) | `buchstaben` | X |
| Stichart der Schrift | Satin (Standard des Originals), Step, Füllmuster, **automatisch** (`auto`: Satin, bis ein Satinstich nach Größe/Transformation länger als 8 mm würde, dann der ganze Text Step – eigene Erweiterung). Neue Texte, Computer-Schriften und Monogramme der Oberfläche bekommen `auto`. Einfacher Modus: bei Hinweis `schrift_step` grüne Info „wird mit Step gestickt“, bei `lange_satinstiche` Warnung; jeweils Schalter **„Trotzdem Satin“** (an = `satin`, aus = `auto`) | `stichart` (auch am `monogramm`) | E (Schalter) / X (Auswahl) |
| Monogramm | 2 oder 3 Initialen, Rahmen Raute / Zier / keiner, Rahmenfarbe; Höhe (18 mm) | `monogramm` | E |
| Verzerren (Elastic) | gerade (Wimpel/schräg) oder gebogen (Bogen/Bauch), Griffe oben/unten | `elastisch` | E |
| Unterlage (Stabilisieren) | **Neue Texte, Computer-Schriften und Monogramme: Mittellauf + Zickzack vorgewählt** (`["mittellauf", "zickzack"]`, Entscheidung Nutzer 21.11.2026; Original: nur Zickzack = `"auto"`). Einfacher Modus: ein Häkchen **„Stabilisieren“** unter der Höhe (an = beide, aus = `"keine"`; Handbuch: bei kleiner Schrift abschalten). Experte: zwei Häkchen **Mittellauf** und **Zickzack** im Inspektor (keins = `"keine"`; andere Arten aus dem JSON bleiben erhalten). Ohne Angabe bzw. `"auto"` = Stil „Schrift“ (Zickzack); `true`/`false` (alte TTF-Dateien) = auto/keine | `unterlage` (Text, `ttf`, `monogramm`) | E (ein Häkchen) / X (zwei Häkchen, Arten) |

## 5. Bearbeiten (für jedes Objekt)
| Funktion | Anwender stellt ein (Standard) | JSON | Modus |
|---|---|---|---|
| Skalieren, drehen, spiegeln, schräg, verschieben | Prozent / Grad / Richtung / mm | `transform` | E |
| **Auswählen** (Handbuch S. 41–42) | Klick (Umschalt/Strg: dazu/weg), **Strg+A** alle Objekte („Select All“), **Auswahlrechteck**: auf freier Fläche der Vorschau ziehen – gewählt wird, was ganz im Rechteck liegt; Teil einer Gruppe ganz drin → ganze Gruppe; Umschalt/Strg: dazu. Nicht gewähltes Objekt anfassen = wählen und verschieben. Ansicht verschieben: Leertaste + Ziehen oder mittlere Maustaste. Esc hebt die Auswahl auf | – (nur Oberfläche, Begrenzung aus `design_formen`) | E |
| **Gruppe** („Group“, Handbuch S. 134) | Objekte zu einem Element zusammenfassen: gemeinsam wählen, verschieben, skalieren, drehen (um die Mitte der Gruppe); Gruppieren mit Strg+G (beide Modi; einfacher Modus: Knopf „Gruppieren“, wenn mehrere Elemente gewählt sind), „Gruppe auflösen“ (Strg+Umschalt+G; einfacher Modus bei selbst gruppierten Elementen). Gruppen in Gruppen gibt es nicht – beim Gruppieren lösen sich alte Gruppen der Auswahl auf (reine Verschiebung bleibt erhalten). Bild/SVG/Datei werden beim Einfügen gleich gruppiert. Einfacher Modus: eine Gruppe = ein Element in der Elementliste | `gruppe` am Objekt (Schlüssel), `gruppen` am Design: `{name, herkunft (gruppiert/bild/svg), transform}` | E |
| Aussparen (Knockout) | Füllfläche spart darüberliegende Objekte aus, Überlappung (0 mm) | `aussparen`, `ueberlappung` | E (Überlappung X) |
| **Werte je Objekt** (Objekteigenschaften) | jeder Stil-Wert nur für dieses Objekt: Stichabstand, Stichlänge, Mindeststich am Rand, Zugausgleich, Auto Spacing, Kurzstiche, Unterlage-Werte, Lauf fest/variabel, Vernähen/Verbinder – Aufbau wie `einstellungen` des Designs (ohne Stoff). Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt. Anzeige: Eigenschaften-Fenster des Objekts, Standard = Wert des Designs/Stoffs, geänderte Werte markieren und einzeln zurücksetzen | `einstellungen` am Objekt (Pfade wie `PARAMETER.md`, Ebene „beide“) | X |
| Verstärken (Double Run) | an/aus – gleiche Stiche rückwärts noch einmal | `doppellauf` | X |
| **Umrandung** (eigene Erweiterung, Nutzer 21.11.2026; im Original eine zweite Linie von Hand, Handbuch S. 114) | an/aus – Linie um die Fläche, bei Text um jeden Buchstaben, bei Gruppen (Bild) um jede Fläche; Löcher bekommen auch eine Linie. Gestickt gleich nach dem Objekt in eigener Farbe, **0,3 mm nach innen** versetzt (keine Lücke zur Füllung). Einfacher Modus: Häkchen „Umrandung“ (unter „Stabilisieren“) + Garn am Element (Text, Monogramm, Fläche, Bild) – setzt eine **dicke Satinkante außen**: Text **6 % der Buchstabenhöhe** (50 mm → 3 mm, mindestens 1 mm, wächst beim Vergrößern mit), Flächen 2 mm; die Farbe des Objekts bleibt ganz sichtbar, benachbarte Buchstaben behalten je ihre Kante. Experte: Abschnitt „Umrandung“ mit Stichart (Lauf, **Dreifach** = Standard des Felds, Satinkante), Garn, Lage der Satinkante (auf der Kante / außen), nach innen bzw. „in der Fläche“ (0–3 mm), Breite (Text: % der Höhe oder fest in mm; sonst mm, Standard 2), Stichlänge. Nur Objekte mit Fläche (`tatami`, `satinflaeche`, `muster`, `satin`, `satinlinie`, `blanket`, `text`, `ttf`, `monogramm`, `applikation`). Garn von Hand des Objekts gilt nicht für die Umrandung | `umrandung: {art, farbe, einzug, breite, breite_anteil, aussen, laenge}` (an = Feld vorhanden, `{}` = Standard) | E (an/aus, Garn) / X (Rest) |
| Auto Jump | an/aus, Höchststich (12,7 mm), Sprunglänge (7 mm); kleiner Höchststich = „Satin mit mehr Loft“ | `auto_jump` | X |
| Reihenfolge der Objekte | Reihenfolge im JSON = Stickreihenfolge; Liste wie die „Color Film“ des Originals (Nr, Farbe, Art, Name), Ziehen zum Umordnen | Reihenfolge `objekte` | E |
| Umordnen (Menü „Anordnen“) | **nach Farbe** (Farbfolge mit Hoch/Runter wählbar, weniger Farbwechsel), **an den Anfang / ans Ende**, **wie ausgewählt** (Objekte in Klickfolge), **vor/nach Objekt Nr.** – auf die Auswahl oder alles; Ergebnis = neues JSON | Kern `stick_kern::reihenfolge::ausfuehren` | E |
| Überdeckung beachten | Häkchen beim Sortieren nach Farbe (aus wie im Original): Objekte, die sich überdecken, behalten ihre Lage; **Warnung**, wenn ein Umordnen ein Objekt unter eine Fläche schiebt, über der es lag (z. B. Kontur unter die Füllung) | – (Werkzeug) | E |
| Einstich / Ausstich | zwei Griffe am Objekt („entry/exit point“): Objekt beginnt bzw. endet dort (Linien drehen sich um, geschlossene Formen beginnen am nächsten Punkt, Flächen beginnen am Einstich) | `einstich`, `ausstich` ([x, y] mm) | X |

## 6. Befehle (`kern/stick-api`, Ein-/Ausgabe in `SCHNITTSTELLE.md` §5)
| Befehl | Wofür in der Oberfläche |
|---|---|
| `rahmen_liste`, `stoffe_liste`, `alphabete_liste`, `garnkarten_liste` | Auswahllisten Schritt 1 (Rahmen, Stoff) und Schrift/Garn; Alphabete mit empfohlener Höhe (Band am Höhenregler) |
| `alphabet_vorschau` | Bild eines Alphabets im Auswahldialog (Umrisse von „Abc“ oder eigenem Probetext) und Zeichenvorrat |
| `motive_liste` | Auswahl des Motivsatzes (14 der CD) |
| `motivsatz_motive` | Auswahl des Motivs mit Vorschaubild (Linien je Motiv, Länge, Höhe) |
| `step_muster_liste` | Kacheln der Step-Muster 1–20 (Einstiche einer Probefläche, vom Kern berechnet) |
| `garnkarte_garne` | Garn je Farbe von Hand wählen (alle Garne einer Karte mit Farbe, Nummer, Name) |
| `fuellmuster_liste` | Auswahl der 242 Füllmuster mit Vorschau-Kacheln (Linien einer Kachel + Gitter A/B) |
| `schriften_liste` | Reiter „Auf dem Computer“: installierte TrueType/OpenType mit lateinischen Buchstaben + heruntergeladene Google Fonts (`heruntergeladen`); `neu_einlesen` nach dem Installieren einer Schrift |
| `schrift_stile` | Stil-Filter der Schriftauswahl: Name und Erklärung (Tooltip) in Anzeigereihenfolge; die Listen tragen je Schrift `stile` |
| `google_schriften_liste` | Reiter „Google Fonts“: Katalog (Familie, Kategorie, Stile, Größe, Lizenz, `geladen`) und die Stil-Filter in fester Reihenfolge |
| `google_schrift_laden` | Klick auf eine Google-Kachel: Datei herunterladen (Internet), prüfen, ablegen → `SchriftInfo` (Name für `ttf.schrift`) |
| `schrift_vorschau` | Kachel einer Computer-Schrift: Umrisse des Probetexts (gefüllt zeichnen, Nonzero), fehlende Zeichen, `stickbarkeit` (`stickbar_ab`, `duenn`, `dick` in mm) |
| `schrift_wechseln` | Wahl im Dialog „Schrift wählen“: Text ↔ Computer-Schrift umstellen (gemeinsame Felder, Unterlage und Stichwinkel bleiben; Stichart auto/Satin/Muster bleibt, Step ↔ Tatami, sonst auto) → neues Objekt |
| `parameter_katalog` | Formulare für Einstellungen und Expertenmodus (Standard, Bereich, Einheit, Modus) |
| `beispiel_design` | Beispiel zum Ausprobieren |
| `design_schema` | Design-Format zur Laufzeit (dasselbe wie `design.schema.json`) |
| `design_pruefen` | schnelle Prüfung beim Tippen (Fehler mit Objektnummer, Hinweise), ohne Stiche |
| `design_berechnen` | Stichvorschau, Kennzahlen, Garne – bei jeder Änderung |
| `design_formen` | Formansicht und Griffe zum Bearbeiten mit der Maus |
| `in_knoten_umwandeln` | Knoten-Werkzeug: „In Knoten umwandeln“ – Kreis/Oval/Bogen werden Bézierstücke (Knoten mit Anfassern wie im Original), Rechteck wird Eckpunkte, `transform` wird auf die Knoten angewandt (Stichwinkel, Satinbreiten mitgeführt); Ergebnis ersetzt das Objekt. Modus experte. Text/Monogramm/Knopfloch: Fehler (keine Knoten) |
| `exp_speichern` | Datei → Stickdatei speichern |
| `reihenfolge_aendern` | Menü „Anordnen“: nach Farbe, an Anfang/Ende, wie ausgewählt, vor/nach Nummer – mit Warnungen bei Überdeckung |
| `ueberdeckungen` | Anzeige, welche Objekte sich überdecken (z. B. in der Objektliste) |
| `saeulen_vorschau` | Vorschau der automatischen Satin-Stichrichtung einer Fläche (optional) |
| `stichart_aendern` | **Stichart wechseln** wie im Original („Outline/Fill“ + Stichart-Liste, Handbuch S. 63/64: jederzeit änderbar): Umriss `lauf`, `dreifach`, `satinlinie`, `blanket`, `motivlinie` · Füllung `tatami`, `satinflaeche`, `muster`. Form, Farbe, Name, Garn, Transformation, Gruppe bleiben; offene Linie → Füllung: Fehler; Füllung → Linie: Löcher entfallen (Hinweis). Oberfläche: Inspektor oben „Stichart“, Werkzeugleiste „Neue Formen: Füllung/Umriss + Stichart“. Modus E |
| `design_einfuegen` | „Insert Design“: ein Design (Ergebnis von `bild_umwandeln`/`svg_lesen` oder eine Datei) ins aktuelle einfügen, auf Wunsch als **eine Gruppe** (`gruppe: {name, herkunft}`); doppelte Namen werden „Name (2)“, `aussparen` zieht mit; Transformation des eingefügten Designs wird die der Gruppe. Ergebnis: Design, Nummern der neuen Objekte, Gruppenschlüssel. Einfacher Modus: „+ Element hinzufügen“ → Bild/SVG/Design öffnen. Modus E |
| `bild_vorschau` | **Vorschau im Dialog „Bild umwandeln“**, bevor Stiche entstehen: vereinfachtes Bild (PNG als data-URL) und **Farbliste** (Farbe, Anteil %, Garn der Karte, `weglassen` = Vorschlag). Anwender stellt ein: **Farben** (Standard 8), **Vereinfachen** 0–5 (Standard 2, glättet Fotorauschen, Kanten bleiben), **Garnfarben** an/aus (`garnkarte` + `garnfarben`; Garnkarte des Designs; bei „Default“ – nur 19 Grundfarben – aus; aus = Farben des Bildes, Liste nennt das nächste Garn mit „≈“), je Farbe **sticken/weglassen** wie „Omitted Colors“ im Original (Vorschlag: weißer Hintergrund – Haken setzen = Weiß wird gestickt). Weggelassene Farben blendet die Oberfläche im Bild aus. Modus E; kleinste Fläche, Glättung, Stichart X |
| `bild_umwandeln` | Bild → Stickflächen wie der Auto Digitizer (gleiche Eingabe wie `bild_vorschau`, dazu `weglassen` als „#rrggbb“-Liste, `fuellung` „step“ (Standard, sauber bei Fotos) oder „auto“ (bis 8 mm Satin, für Logos)); kleine Flächen gehen in der Nachbarfarbe auf (keine Löcher); Zugausgleich 0,40 mm und Unterlage an wie im Handbuch S. 116; mit Garnkarte bekommen die Objekte `garn`. Ergebnis: Design, Zahl der Flächen/Farben, `klassen` je Objektart |
| `svg_lesen` | SVG-Datei (Inkscape) → neues Design; Zählung je Stichart und Hinweise für die Anzeige |
| `datei_lesen`, `text_speichern` | Design öffnen/speichern |
| `maschine_ports`, `maschine_pruefen` | Anschluss wählen (COM-Port oder Simulation), Maschine prüfen (nur lesen: Firmware, Garnfarbtabelle) |
| `maschine_planen` | Senden Stufe 1: nur lesen, Prüfungen wie das Original, Liste der Schreibzugriffe mit Größe, Garne an der Maschine – zum Bestätigen |
| `maschine_senden` | Senden Stufe 2 (**schreibt**, nur nach Bestätigung); mit `wahl.sticken` Befehl 0x08 (Rahmen fährt – nur nach Gefahrendialog „Person an der Maschine“); Fortschritt als Ereignis `maschine_fortschritt` `{was, erledigt, gesamt, wiederholungen}` |
| `maschine_abbrechen` | „Abbrechen“ während des Sendens/Lesens (PC-Modus wird trotzdem verlassen) |
| `maschine_liste`, `maschine_lesen`, `gelesen_exp_speichern` | „Designs aus der Maschine“: Liste, eines lesen (Befehl 0x400, nur lesen) und ansehen, als EXP speichern |
| Fehler der Maschine | `{meldung, code}` mit `code` = `keine_verbindung`, `stickmodul`, `antwortet_nicht`, `zurueckgelesen_weicht_ab`, `farbtabelle`, `zu_gross`, `ausserhalb`, `abgebrochen`, `belegt` – je Code Zustand und nächste Schritte zeigen |

Ausprobieren ohne Oberfläche: `stick api liste`, `stick api design_berechnen DATEI.json` (Eingabe wie in
`SCHNITTSTELLE.md`).

## 7. Kommt noch (Platz vorsehen, ROADMAP Phase 4b)
Motivlinien und Motivfüllungen · Effekte (ausgefranster Rand, Welle, Farbverlauf,
Farben mischen) · Craft Stitches (Blackwork, Rückstich, Stielstich, Candlewicking, Stippling) · Applikation-Feinheiten ·
Magic Wand, PhotoSnap · Ausrichten, Gruppieren, Kranz, Kaleidoskop · eigene Rahmen, fremde Stickdateien ·
Vorlagen/Stile speichern · Kreuzstich.
