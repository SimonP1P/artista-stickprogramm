# Schnittstelle Kern ↔ Oberfläche

> ERZEUGT aus dem Rust-Kern (stick-kern/src/format.rs, stick-api) – nicht von Hand ändern; neu erzeugen: stick api exportieren

**Design-Format Fassung 1.** Maschinenlesbar: `kern/schnittstelle/design.schema.json` (Design), `kern/schnittstelle/befehle.schema.json` (Befehle), `kern/schnittstelle/kern.ts` (TypeScript-Typen + Aufruf). Was jede Funktion für den Anwender bedeutet und welcher Modus (einfach/experte): `UI_Mockups/FUNKTIONEN.md`. Alle Einzelwerte (`einstellungen`) mit Standard je Stoff: `PARAMETER.md`.

Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. Objektnummern in Befehlen und Ergebnissen ab 0.

## 1. Design (oberste Ebene)

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `einstellungen` | EinstellungenJson |  | – | Einzelwerte des Stoffs/Stils überschreiben, z. B. {"satin": {"zugausgleich": 0.3}} (Parameterkatalog) |
| `ende` | Ende |  | – | Endpunkt (Standard: Sprung zurück zur Rahmenmitte) |
| `garnkarte` | string |  | `"Default"` | Garnkarte (Befehl `garnkarten_liste`): jede Farbe → nächstes Garn; „keine“ = ohne (Standard „Default“ wie die Maschine) |
| `gruppen` | Record<string, GruppeJson> |  | – | **Gruppen** („Group“ im Original): Schlüssel → Name und Transformation. Objekte nennen ihre Gruppe im Feld `gruppe`. Reihenfolge der Transformationen: je Objekt → je Gruppe (um die Mitte ihrer Objekte) → ganzes Design |
| `kuerzeste_verbindung` | boolean |  | `false` | Closest Join: jedes Objekt ohne eigenen Einstich beginnt nahe dem vorigen Ende (nicht bei Schrift) |
| `name` | string |  | `"Design"` | Name des Designs (erscheint in der Designliste der Maschine) |
| `objekte` | ObjektJson[] |  | – | Stickobjekte in Stickreihenfolge |
| `rahmen` | "artista 255 x 145 Oval" \| "artista 255 x 145 Oval Rotated" \| "artista 200 x 155" \| "artista 200 x 155 Rotated" \| "artista 130 x 100" \| "artista 130 x 100 Rotated" \| "artista 50 x 72 Free Arm" \| "artista 50 x 72 Free Arm Rotated" \| "artista 50 x 72 Normal" \| "artista 50 x 72 Normal Rotated" \| "artista 40 x 60" \| "artista 40 x 60 Rotated" \| "artista 400 x 150 MEGA" \| "artista 400 x 150 MEGA Rotated" |  | `"artista 130 x 100"` | Stickrahmen (Befehl `rahmen_liste`) |
| `start` | Start |  | – | Startpunkt (Standard: Rahmenmitte) |
| `stoff` | "0 Software Defaults" \| "Knit Light Weight" \| "Knit Medium Weight" \| "Knit Heavy Weight" \| "Terry Medium Weight" \| "Terry Heavy Weight" \| "Woven Light Weight" \| "Woven Medium Weight" \| "Woven Heavy Weight" |  | `"0 Software Defaults"` | Stoff: ändert Dichte, Unterlage, Zugausgleich (8 Stoffe + Software Defaults) |
| `transform` | Transformation |  | – | Transformation des ganzen Designs (um seine Mitte) |
| `version` | number |  | – | Fassung des Formats (ohne Angabe: aktuelle Fassung) |

## 2. Objekte (`objekte`, Feld `art`)

### Felder aller Objekte

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `ausstich` | Punkt |  | – | Ausstich („exit point“) [x, y] in mm: das Objekt endet dort |
| `auto_jump` | AutoJumpJson |  | – | Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) |
| `doppellauf` | boolean |  | `false` | Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder |
| `einstellungen` | EinstellungenJson |  | – | **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}. Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`) |
| `einstich` | Punkt |  | – | Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort |
| `farbe` | string |  | `"#000000"` | Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet) |
| `garn` | string |  | – | **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B. „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte) |
| `gruppe` | string |  | – | **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element) |
| `name` | string |  | `""` | Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text |
| `step_muster` | number |  | – | Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) |
| `transform` | Transformation |  | – | Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet |

### `lauf` – Laufstich

Laufstich bzw. Dreifachstich entlang einer Linie.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `dreifach` | boolean |  | – | Dreifachstich (bei art „dreifach“ immer) |
| `geschlossen` | boolean |  | `false` | Linie schließen (zurück zum ersten Punkt) |
| `laenge` | number |  | – | Stichlänge in mm (ohne Angabe: Stil, 2,5 mm) |
| `punkte` | PunktAngabe | ja | – |  |

### `dreifach` – Dreifachstich

Laufstich bzw. Dreifachstich entlang einer Linie.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `dreifach` | boolean |  | – | Dreifachstich (bei art „dreifach“ immer) |
| `geschlossen` | boolean |  | `false` | Linie schließen (zurück zum ersten Punkt) |
| `laenge` | number |  | – | Stichlänge in mm (ohne Angabe: Stil, 2,5 mm) |
| `punkte` | PunktAngabe | ja | – |  |

### `satin` – Satinsäule

Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | fester Satinabstand in mm statt Auto Spacing (wie im Alphabet: 0x15) |
| `faktor` | number |  | – | Auto-Spacing-Faktor des Objekts (Stil: 0x2736; Schrift 0,9) |
| `kurzstiche` | boolean |  | – | Kurzstiche an Innenkurven (Effekt 0x273a; Software Defaults: aus) |
| `seite_a` | PunktAngabe | ja | – |  |
| `seite_b` | PunktAngabe | ja | – |  |
| `stichart` | SatinStichart |  | – |  |
| `unterlage` | UnterlageJson |  | – |  |

### `satinlinie` – Satinlinie (Kontur)

Satinbahn konstanter Breite entlang einer Linie.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `breite` | number |  | – | Breite in mm (Standard 2) |
| `punkte` | PunktAngabe | ja | – |  |
| `stichart` | SatinStichart |  | – |  |
| `unterlage` | UnterlageJson |  | – |  |
| `versatz` | Versatz |  | – |  |

### `blanket` – Blanket (Langette)

Blanket (Langettenstich, 0x2727) entlang einer Linie.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Abstand der Zähne in mm (Vorlage 2,0) |
| `breite` | number |  | – | Breite in mm (Vorlage 3,0) |
| `geschlossen` | boolean |  | `false` |  |
| `punkte` | PunktAngabe | ja | – |  |
| `versatz` | Versatz |  | – |  |

### `tatami` – Fläche Tatami (Step)

Tatami-Fläche (Step) mit Löchern; mit `winkellinien` Fläche mit mehreren Stichwinkeln (Turning).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `aussparen` | string[] |  | `[]` | Namen der Objekte, deren Fläche ausgespart wird (Knockout) |
| `kurzstiche` | boolean |  | – | Kurzstiche an Innenkurven (nur mit Winkellinien wirksam; ohne Angabe: Stil) |
| `loecher` | PunktAngabe[] |  | – |  |
| `ueberlappung` | number |  | `0.0` | Überlappung der Aussparung in mm (Standard 0 = exakt wie das Original) |
| `umriss` | PunktAngabe | ja | – |  |
| `unterlage` | boolean |  | – | Unterlage an/aus (ohne Angabe: Auto Underlay des Stils) |
| `winkel` | number |  | – | Stichwinkel in Grad (ohne Angabe: Stil, 45°) |
| `winkellinien` | Punkt[][] |  | `[]` | mehrere Stichwinkel: Strecken [[x1, y1], [x2, y2]], die den Umriss je zweimal schneiden (ohne Löcher) |

### `satinflaeche` – Satinfläche mit automatischer Stichrichtung

Fläche als Satinsäulen mit automatischer Stichrichtung (`saeulen.rs`, ROADMAP 4b.17).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | fester Satinabstand in mm statt Auto Spacing |
| `faktor` | number |  | – | Auto-Spacing-Faktor |
| `knickwinkel` | number |  | – | Säulen an Knicken teilen: Innenwinkel in Grad, 0 … 180 (0 = nie wie der Auto Digitizer; 87 = TrueType) |
| `kurzstiche` | boolean |  | – | Kurzstiche an Innenkurven |
| `loecher` | PunktAngabe[] |  | – |  |
| `stichart` | SatinStichart |  | – |  |
| `umriss` | PunktAngabe | ja | – |  |
| `unterlage` | UnterlageJson |  | – |  |

### `text` – Text mit Original-Alphabet

Text mit einem Original-Alphabet (50 Alphabete, Befehl `alphabete_liste`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) |
| `alphabet` | string | ja | – | Name des Alphabets („Drifter“) |
| `ausrichtung` | Ausrichtung |  | `"mitte"` |  |
| `breite` | number |  | `100.0` | Breite in % der Höhe |
| `buchstaben` | BuchstabeJson[] |  | – | einzelne Buchstaben: Farbe, Transformation, Stichwinkel |
| `elastisch` | HuelleJson |  | – | Verzerren (Elastic Lettering) |
| `farben` | string[] |  | `[]` | Farben für die Farbwechsel `^` im Text, in Reihenfolge |
| `groesse` | number |  | – | Größe des Füllmusters in % (Stichart „muster“, Standard 100) |
| `grundlinie` | GrundlinieJson |  | – | Grundlinie (ohne Angabe: gerade waagrecht) |
| `hoehe` | number |  | `10.0` | Buchstabenhöhe in mm (Original 1–200 mm; Vorlage 10) |
| `kursiv` | number |  | `0.0` | Kursiv in Grad (−45 … 45) |
| `muster` | string |  | – | Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) |
| `position` | Punkt |  | – | Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) |
| `stichart` | SchriftStichart |  | – |  |
| `text` | string | ja | – | Text; `\n` = neue Zeile, `^` = Farbwechsel (Farben in `farben`) |
| `unterlage` | UnterlageJson |  | – | Unterlage (auto = wie Stil Schrift: Zickzack) |
| `winkel` | number |  | – | Stichwinkel der Musterfüllung (Stichart „muster“) |
| `wortabstand` | number |  | – | Wortabstand in % der Höhe (Vorlage 25) |
| `zeilenabstand` | number |  | – | Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text |

### `ttf` – Text mit Computer-Schrift

Text mit einer Computer-Schrift (TrueType/OpenType, Befehl `schriften_liste`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) |
| `ausrichtung` | Ausrichtung |  | `"mitte"` |  |
| `breite` | number |  | `100.0` | Breite in % der Höhe |
| `buchstaben` | BuchstabeJson[] |  | – | einzelne Buchstaben: Farbe, Transformation, Stichwinkel |
| `elastisch` | HuelleJson |  | – | Verzerren (Elastic Lettering) |
| `farben` | string[] |  | `[]` | Farben für die Farbwechsel `^` im Text, in Reihenfolge |
| `groesse` | number |  | – | Größe des Füllmusters in % (Stichart „muster“, Standard 100) |
| `grundlinie` | GrundlinieJson |  | – | Grundlinie (ohne Angabe: gerade waagrecht) |
| `hoehe` | number |  | `10.0` | Höhe der Großbuchstaben in mm |
| `kursiv` | number |  | `0.0` | Kursiv in Grad (−45 … 45) |
| `laenge` | number |  | – | Stichlänge bei Lauf/Dreifach in mm |
| `muster` | string |  | – | Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) |
| `position` | Punkt |  | – | Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) |
| `schrift` | string | ja | – | Name der Schrift („Arial“, „Arial Bold“) oder Pfad einer .ttf/.otf-Datei |
| `stichart` | TtfStichart |  | `"tatami"` |  |
| `text` | string | ja | – | Text; `\n` = neue Zeile, `^` = Farbwechsel |
| `unterlage` | boolean |  | – | Unterlage an/aus (Tatami, Satin; ohne Angabe: Stil) |
| `winkel` | number |  | – | Stichwinkel (Tatami, Muster) |
| `wortabstand` | number |  | – | Wortabstand in % der Höhe (Vorlage 25) |
| `zeilenabstand` | number |  | – | Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text |

### `monogramm` – Monogramm

Monogramm (Handbuch Kap. 21): 2 oder 3 Initialen mit den Alphabeten Diamond2/Diamond3, Rahmen wahlweise. Wird im Kern zu einem Text – alle Schrift-Einstellungen gelten.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) |
| `ausrichtung` | Ausrichtung |  | `"mitte"` |  |
| `breite` | number |  | `100.0` | Breite in % der Höhe |
| `buchstaben` | BuchstabeJson[] |  | – | einzelne Buchstaben: Farbe, Transformation, Stichwinkel |
| `elastisch` | HuelleJson |  | – | Verzerren (Elastic Lettering) |
| `farben` | string[] |  | `[]` | Farben für die Farbwechsel `^` im Text, in Reihenfolge |
| `groesse` | number |  | – | Größe des Füllmusters in % (Stichart „muster“, Standard 100) |
| `grundlinie` | GrundlinieJson |  | – | Grundlinie (ohne Angabe: gerade waagrecht) |
| `hoehe` | number |  | `18.0` | Höhe in mm (18 = kleinste empfohlene Höhe von Diamond2/3) |
| `initialen` | string | ja | – | 2 oder 3 Buchstaben A–Z |
| `kursiv` | number |  | `0.0` | Kursiv in Grad (−45 … 45) |
| `muster` | string |  | – | Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) |
| `position` | Punkt |  | – | Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) |
| `rahmen` | MonogrammRahmen |  | – |  |
| `rahmen_farbe` | string |  | – | Farbe des Rahmens (nur mit Rahmen) |
| `stichart` | SchriftStichart |  | – |  |
| `unterlage` | UnterlageJson |  | – |  |
| `winkel` | number |  | – |  |
| `wortabstand` | number |  | – | Wortabstand in % der Höhe (Vorlage 25) |
| `zeilenabstand` | number |  | – | Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text |

### `applikation` – Applikation

Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abdeckung` | Abdeckung |  | – |  |
| `abstand` | number |  | – | fester Satinabstand des Abdeckstichs in mm |
| `breite` | number |  | – | Breite des Abdeck-Satins in mm (Standard 2,5) |
| `heft_abstand` | number |  | – | Abstand des Heftstichs in mm |
| `heft_breite` | number |  | – | Breite des Heftstichs in mm |
| `loecher` | PunktAngabe[] |  | – |  |
| `platz_laenge` | number |  | – | Stichlänge der Platzierungslinie in mm |
| `stil` | ApplikationStil |  | – |  |
| `umriss` | PunktAngabe | ja | – |  |

### `muster` – Fläche mit Füllmuster

Fläche mit Füllmuster (Program Split / Fancy Fill; 242 Muster, Befehl `fuellmuster_liste`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `aussparen` | string[] |  | `[]` | Namen der Objekte, deren Fläche ausgespart wird (Knockout) |
| `groesse` | number |  | `100.0` | Größe des Musters in % |
| `loecher` | PunktAngabe[] |  | – |  |
| `muster` | string | ja | – | Name des Musters („001 Diamond (90)“) oder seine Nummer („001“) |
| `ueberlappung` | number |  | `0.0` | Überlappung der Aussparung in mm |
| `umriss` | PunktAngabe | ja | – |  |
| `unterlage` | boolean |  | – | Unterlage an/aus (ohne Angabe: Stil) |
| `winkel` | number |  | – | Stichwinkel (ohne Angabe: Winkel aus dem Musternamen, sonst der Füllung) |

### `knopfloch` – Knopfloch (9 Typen; in Flächen einarbeiten: Namen des Knopflochs in `aussparen` der Fläche)

Knopfloch (Handbuch Kap. 23 „Adding buttonholes“): Vorlage aus `ButtonHoles.ESB`, auf Schlitzlänge gestreckt.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Raupenabstand in mm, 0,2 … 2,0 („Bead Stitch Spacing“); gilt nur für die Raupen, nicht die Enden. Ohne Angabe wie die Vorlage (0,5 mm; Heirloom 1,5 mm) |
| `laenge` | number |  | `20.0` | Schlitzlänge in mm, 3 … 60 („Slit Length“, Standard 20 wie das Original) |
| `position` | Punkt |  | – | Mitte des Schlitzes [x, y] in mm (ohne Angabe: Rahmenmitte) |
| `typ` | KnopflochTyp |  | `"standard"` | Typ (Standard „standard“ wie das Original) |
| `winkel` | number |  | `90.0` | Richtung des Schlitzes in Grad („Angle“; 90 = senkrecht, Standard wie das Original) |

### `motivlinie` – Motivlinie (Motiv aus einem Motivsatz entlang einer Linie)

Motivlinie (Handbuch Kap. 25 „Pattern Run“): ein Motiv aus einem Motivsatz (`motive_liste`) wird entlang der Linie wiederholt. Anzahl = Linienlänge / Abstand gerundet, die Motive werden gleichmäßig verteilt (wie das Original).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Wiederholabstand in mm („Spacing“); ohne Angabe = Größe (Motive stoßen aneinander) |
| `geschlossen` | boolean |  | `false` | Linie schließen (Motive ringsum) |
| `groesse` | number |  | – | Länge eines Motivs entlang der Linie in mm („Size“); ohne Angabe: Größe im Motivsatz |
| `motiv` | string |  | `"705"` | Motiv im Satz, z. B. „705“ (Standard „705“ – eigene Festlegung) |
| `punkte` | PunktAngabe | ja | – |  |
| `satz` | string |  | `"Heirloom"` | Motivsatz, z. B. „Heirloom“ (Befehl `motive_liste`; Standard „Heirloom“ – eigene Festlegung) |
| `spiegeln` | boolean |  | `false` | Motive auf die andere Seite der Linie spiegeln |

## 3. Auswahlwerte

**Abdeckung** – Abdeckstich der Applikation

- `"satin"` – Satin mit Heftstich (Standard)
- `"blanket"` – Blanket (Langette) ohne Heftstich

**ApplikationStil** – Arbeitsweise der Applikation

- `"vorgeschnitten"` – Stoff vorgeschnitten („Pre-cut“, Standard)
- `"zuschneiden"` – im Rahmen zuschneiden („Trim in Place“, mit Schnittlinie)

**Ende** – Endpunkt des Designs (Dialog „Start and End“)

- `"rahmenmitte"` – Sprung zurück zur Rahmenmitte (Standard; 34 von 51 Maschinendesigns enden auf 0,0)
- `"designmitte"` – Mitte des Designs
- `"unten_mitte"` – Mitte der Unterkante
- `"letzter_stich"` – letzter Stich (kein Sprung)
- `"start"` – zurück zum Startpunkt

**GrundlinieArt** – Art der Grundlinie (Dialog „Layout“)

- `"kreis"` – Kreis CW/CCW, auch Oval (radius_y) und Bogen (bogen)
- `"gerade"` – gerade mit Winkel und optional fester Länge
- `"senkrecht"` – senkrecht
- `"kurve"` – eigene Kurve (Feld `punkte`)
- `"mehrfach"` – mehrfach: oben – Mitte – unten (höchstens drei Zeilen)

**GrundlinieLage** – Lage auf der Kreis-Grundlinie

- `"oben"` – oben, im Uhrzeiger (Circle CW)
- `"unten"` – unten, gegen den Uhrzeiger (Circle CCW)

**GrundlinieStand** – Stand der Buchstaben zur Grundlinie (Position Above/Through/Below)

- `"ueber"`
- `"durch"`
- `"unter"`

**GruppenHerkunft** – Woher eine Gruppe stammt (nur für die Anzeige in der Oberfläche)

- `"gruppiert"` – von Hand gruppiert („Group“, Standard)
- `"bild"` – Bild umwandeln (`bild_umwandeln`)
- `"svg"` – SVG einlesen (`svg_lesen`)

**HuelleArt** – Form der Verzerrung (Elastic Lettering)

- `"gerade"` – gerade Kanten (Wimpel, schräg): oben/unten = [links, rechts]
- `"gebogen"` – gebogene Kanten (Bogen, Bauch): oben/unten = Mitte oder [links, Mitte, rechts]

**KnopflochTyp** – Knopfloch-Typ (Dialog „Buttonhole Type“; Vorlagen aus `ButtonHoles.ESB`)

- `"standard"` – Standard (zwei Raupen, Riegel oben und unten)
- `"standard_schmal"` – Standard Narrow (schmale Raupen)
- `"rund"` – Round (beide Enden rund)
- `"dehnbar"` – Stretch (dehnbar, Zickzack-Läufe statt Satin)
- `"heirloom"` – Heirloom (Raupen mit Kreuzstichen)
- `"rundes_ende"` – Round End (ein Ende rund)
- `"auge_eckig"` – Keyhole Square (Augenknopfloch, eckiges Auge)
- `"auge_rund"` – Keyhole Round (Augenknopfloch, rundes Auge)
- `"auge_spitz"` – Keyhole Point (Augenknopfloch, spitzes Auge)

**MonogrammRahmen** – Rahmen um ein Monogramm

- `"raute"` – Raute
- `"zier"` – Zierwinkel
- `"keine"` – kein Rahmen (Standard)

**SatinStichart** – Stichart einer Satinsäule, Satinlinie oder Satinfläche

- `"satin"` – Satin (Standard)
- `"zickzack"` – Zickzack (offen, fester Abstand; Stichart „Zigzag“ 0x2724)

**SchriftStichart** – Stichart einer Schrift mit Original-Alphabet

- `"satin"` – Satin (Standard, wie das Alphabet)
- `"auto"` – automatisch: Satin, solange kein Satinstich länger als 8 mm wird, sonst der ganze Text Step (eigene Erweiterung – das Original wechselt bei Schrift nicht selbst; Grenze wie EasyStitch, Handbuch S. 118)
- `"step"` – Step: Reihen folgen den Stichwinkeln der Säulen
- `"muster"` – Füllmuster über die Buchstabenfläche (Feld `muster`)

**Spiegeln** – Spiegeln (Mirror)

- `"links_rechts"` – waagrecht spiegeln (Mirror Horizontal)
- `"oben_unten"` – senkrecht spiegeln (Mirror Vertical)
- `"beide"` – beides

**Start** – Startpunkt des Designs (Dialog „Start and End“)

- `"rahmenmitte"` – Rahmenmitte = Ursprung (Standard)
- `"designmitte"` – Mitte des Designs (umschließendes Rechteck) – wie „Auto Start“ des Originals
- `"unten_mitte"` – Mitte der Unterkante
- `"erster_stich"` – erster Stich (kein Verbinder)

**TtfStichart** – Stichart einer Computer-Schrift

- `"tatami"` – Tatami-Fläche mit Löchern (Standard, eigene Festlegung)
- `"lauf"` – Umriss als Laufstich
- `"dreifach"` – Umriss als Dreifachstich
- `"satin"` – Satinsäulen wie die TrueType-Umwandlung des Originals
- `"muster"` – Füllmuster (Feld `muster`)

**UnterlageArt** – Art einer Unterlage (in Stickfolge)

- `"mittellauf"` – Mittellauf (0x2746)
- `"edge_walk"` – Edge Walk entlang der Kanten (0x2747)
- `"zickzack"` – Zickzack
- `"doppelzickzack"` – doppelter Zickzack

**UnterlageWahl** – Unterlage automatisch oder keine

- `"auto"` – wie „Auto Underlay“ des Stils (Standard)
- `"keine"` – keine Unterlage (z. B. kleine Schrift)

**Versatz** – Lage einer Linie zur gezeichneten Linie

- `"mitte"` – mittig auf der Linie („Center“, Standard)
- `"seite"` – zur Seite: Linie = Kante, Stiche links davon („Offset“)

## 4. Weitere Typen des Formats

### AutoJumpWerte



| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `max` | number |  | – | Höchststich in mm, 0,3 … 12,7 (Standard 12,7); länger = Sprung |
| `sprung` | number |  | – | Länge der Sprünge in mm, 0,3 … 12,7 (Standard 7,0) |

### BuchstabeJson

Einzelner Buchstabe eines Texts (`"buchstaben"`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `farbe` | string |  | – | eigene Farbe |
| `nr` | number | ja | – | Nummer des Buchstabens (1 = erster, ohne Leerzeichen und Zeilenumbrüche) |
| `transform` | Transformation |  | – | verschieben, drehen, skalieren … um die Mitte des Buchstabens |
| `winkellinien` | Punkt[][] |  | `[]` | neue Stichwinkel: Strecken [[x1, y1], [x2, y2]] quer über die Säulen |

### EinstellungenJson

Einzelwerte des Stils überschreiben (Parameterkatalog `PARAMETER.md`); jedes Feld darf fehlen; `stoff` nur am Design

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `lauf` | Objekt |  | – |  |
| `satin` | Objekt |  | – |  |
| `stoff` | string |  | `"0 Software Defaults"` |  |
| `tatami` | Objekt |  | – |  |
| `unterlage` | Objekt |  | – |  |
| `verbinder` | Objekt |  | – | Verbinder (Connectors) – Stil-Container AMT 0x272a, in allen 22 Stilen gleich. Zuordnung Tag ↔ Dialogfeld „Connectors“ aus DIALOG.DLL 0x10035e89–0x10036032 (hoch), Analyse_Teil3/01. |

### GrundlinieJson

Grundlinie eines Texts (`schrift::Grundlinie`; Dialog „Layout“ des Originals). Beim Kreis ist `position` des Texts die Kreismitte.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `art` | GrundlinieArt | ja | – |  |
| `bogen` | number |  | – | Arc Angle in Grad (nur ein Bogen; Text wird eingepasst) |
| `laenge` | number |  | – | Length der geraden Grundlinie in mm (Text wird eingepasst) |
| `lage` | GrundlinieLage |  | – | oben (Circle CW) \| unten (Circle CCW) |
| `neigung` | number |  | `0.0` | Tilt des Ovals in Grad |
| `punkte` | PunktAngabe |  | – | Punkte oder Form der eigenen Kurve (art = kurve) |
| `radius` | number |  | – | Radius X in mm (Vorlage 50) |
| `radius_y` | number |  | – | Radius Y in mm (Oval) |
| `stand` | GrundlinieStand |  | – | ueber \| durch \| unter (Standard: über der CW-, unter der CCW-Grundlinie) |
| `winkel` | number |  | – | Kreis: Justif. Angle (Grad); gerade: Line Angle |

### GruppeJson

Eine Gruppe von Objekten (Feld `gruppen` des Designs).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `herkunft` | GruppenHerkunft |  | – | Herkunft (nur Anzeige) |
| `name` | string |  | `""` | Name der Gruppe (Elementliste der Oberfläche), z. B. der Dateiname des Bilds |
| `transform` | Transformation |  | – | Verschieben, Skalieren, Drehen … der ganzen Gruppe um die Mitte ihrer Objekte – Stiche werden neu berechnet |

### HuelleJson

Elastic Lettering: `{"art": "gerade", "oben": [links, rechts], "unten": [links, rechts]}` oder `{"art": "gebogen", "oben": mitte \| [links, mitte, rechts], "unten": …}` (mm, + nach oben).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `art` | HuelleArt | ja | – |  |
| `oben` | Griffe |  | – | Verschiebung der oberen Kante in mm |
| `unten` | Griffe |  | – | Verschiebung der unteren Kante in mm |

### Transformation

Transformation in der JSON-Form (`"transform"` am Objekt oder am Design).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `drehen` | number |  | `0.0` | Drehung in Grad gegen den Uhrzeiger |
| `schraeg` | number |  | `0.0` | Schrägstellen in Grad (x verschiebt sich mit y, wie Skew im Original) |
| `skalieren` | Skalierung |  | – | Skalierung in %: eine Zahl (beide Richtungen) oder [x, y] |
| `spiegeln` | Spiegeln |  | – |  |
| `verschieben` | Punkt |  | – | Verschieben in mm |

## 5. Befehle

Aufruf aus der Oberfläche: `kern("name", eingabe)` aus `kern.ts` (ein Tauri-Befehl `kern`, Einbindung siehe `kern/stick-api/src/lib.rs`); zum Ausprobieren ohne Oberfläche: `stick api NAME '{…}'`. Fehler: `{meldung, objekt}`.

| Befehl | Eingabe | Ausgabe | Beschreibung |
|---|---|---|---|
| `alphabet_vorschau` | AlphabetVorschauEingabe | AlphabetVorschau | Vorschau eines Original-Alphabets: Umrisse eines Probetexts und Zeichenvorrat |
| `alphabete_liste` | Leer | AlphabetInfo[] | Original-Alphabete (50) |
| `beispiel_design` | Leer | DesignWert | Beispieldesign mit allen Sticharten der ersten Ausbaustufe |
| `bild_umwandeln` | BildEingabe | BildErgebnis | Bild in Stickflächen umwandeln (neues Design) |
| `bild_vorschau` | BildEingabe | BildVorschau | Vorschau der Bildumwandlung: vereinfachtes Bild (PNG) und Farbliste zum Weglassen – vor dem Umwandeln |
| `datei_lesen` | Pfad | string | Textdatei lesen |
| `design_berechnen` | DesignEingabe | Ergebnis | Stiche berechnen – bei jeder Änderung am Design aufrufen (Stiche werden nie skaliert, immer neu berechnet) |
| `design_einfuegen` | EinfuegenEingabe | EinfuegenErgebnis | Design einfügen wie „Insert Design“: Objekte anhängen, auf Wunsch als eine Gruppe (Bild, SVG, Datei als Element) |
| `design_formen` | DesignEingabe | ObjektForm[] | Formen je Objekt (Umrisse, Linien, Satinseiten, Ein-/Ausstich) zum Zeichnen und Bearbeiten mit der Maus |
| `design_pruefen` | DesignEingabe | Pruefung | Design prüfen, ohne Stiche zu berechnen: Fehler (mit Objektnummer) und Hinweise |
| `design_schema` | Leer | unknown | JSON-Schema des Design-Formats (Felder, Standardwerte, Auswahlwerte, Beschreibungen) |
| `exp_speichern` | ExpEingabe | number | Stiche als EXP-Datei speichern (Ergebnis: Zahl der Bytes) |
| `fuellmuster_liste` | Leer | MusterInfo[] | Füllmuster (242) mit Linien einer Kachel für die Vorschau |
| `garnkarte_garne` | GarnkartenName | GarnInfo[] | Garne einer Garnkarte (für die Auswahl von Hand) |
| `garnkarten_liste` | Leer | string[] | Namen der Garnkarten (32 der CD) |
| `gelesen_exp_speichern` | ExpZiel | number | Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern |
| `in_knoten_umwandeln` | ObjektEingabe | ObjektWert | Objekt in Knoten umwandeln (Formen → Punkte/Bézier, Transformation angewandt) – für das Knoten-Werkzeug |
| `maschine_abbrechen` | Leer | boolean | Laufende Maschinen-Aktion abbrechen |
| `maschine_lesen` | LeseEingabe | GelesenInfo | Design aus der Maschine lesen (Befehl 0x400, nur lesen) zum Ansehen |
| `maschine_liste` | Anschluss | ListenInfo | Designliste der Maschine – nur lesen |
| `maschine_planen` | SendeEingabe | SendePlanInfo | Senden planen – nur lesen: Prüfungen wie das Original, geplante Schreibzugriffe zum Bestätigen |
| `maschine_ports` | Leer | string[] | Maschine: serielle Ports (COM…) |
| `maschine_pruefen` | Anschluss | MaschinenInfo | Maschine prüfen – nur lesen: Firmware, Garnfarbtabelle; PC-Modus wird verlassen |
| `maschine_senden` | SendeEingabe | SendeInfo | **Schreibt in die Maschine** (nur nach bestätigtem Plan): DATEN, ICON, INFO, Garnfarbtabelle; mit `wahl.sticken` Befehl 0x08 (Rahmen fährt) |
| `motive_liste` | Leer | MotivsatzInfo[] | Motivsätze (14 der CD) für die Motivlinie |
| `motivsatz_motive` | MotivsatzName | MotivInfo[] | Motive eines Motivsatzes mit Maßen und Linien für die Vorschau |
| `parameter_katalog` | Leer | Parameter[] | Parameterkatalog: jeder einstellbare Wert mit Standard, Bereich, Quelle, Modus einfach/experte/automatisch |
| `rahmen_liste` | Leer | RahmenInfo[] | Stickrahmen der artista 185 mit Stickfeld und Umriss |
| `reihenfolge_aendern` | ReihenfolgeEingabe | ReihenfolgeErgebnis | Objekte umordnen wie Menü „Arrange“: wie ausgewählt, an Anfang/Ende, vor/nach Nummer, nach Farbe |
| `saeulen_vorschau` | SaeulenEingabe | SaeuleInfo[] | Fläche in Satinsäulen zerlegen (Vorschau der automatischen Stichrichtung) |
| `schriften_liste` | Leer | SchriftInfo[] | installierte Computer-Schriften (TrueType/OpenType) |
| `step_muster_liste` | Leer | StepMusterInfo[] | Step-Muster 1–20 (Tabelle des Originals) mit Einstichen einer Probefläche für Vorschau-Kacheln |
| `stichart_aendern` | StichartEingabe | StichartErgebnis | Stichart wechseln (Umriss: lauf, dreifach, satinlinie, blanket, motivlinie · Füllung: tatami, satinflaeche, muster) – Form bleibt |
| `stoffe_liste` | Leer | StoffInfo[] | Stoffe (8 + Software Defaults) mit den wichtigsten Werten |
| `svg_lesen` | SvgEingabe | SvgErgebnis | SVG-Datei einlesen (z. B. aus Inkscape): Stichart nach Breite wie EasyStitch, neues Design |
| `text_speichern` | TextDatei | null | Textdatei schreiben |
| `ueberdeckungen` | DesignEingabe | number[][] | Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken |

**Ereignis** `maschine_fortschritt` (Typ `Fortschritt`: `was`, `erledigt`, `gesamt`, `wiederholungen`): Fortschritt von `maschine_senden` und `maschine_lesen`, von der App gesendet. Fehler der Maschine tragen zusätzlich `code` (`FehlerCode`). **Sicherheit:** nur `maschine_senden` schreibt in die Maschine – erst nach `maschine_planen` und ausdrücklicher Bestätigung; `wahl.sticken` nur nach dem Gefahrendialog.
