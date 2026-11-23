# Schnittstelle Kern ↔ Oberfläche

> ERZEUGT aus dem Rust-Kern (stick-kern/src/format.rs, stick-api) – nicht von Hand ändern; neu erzeugen: stick api exportieren

**Design-Format Fassung 1.** Maschinenlesbar: `kern/schnittstelle/design.schema.json` (Design), `kern/schnittstelle/befehle.schema.json` (Befehle), `kern/schnittstelle/kern.ts` (TypeScript-Typen + Aufruf). Was jede Funktion für den Anwender bedeutet und welcher Modus (einfach/experte): `UI_Mockups/FUNKTIONEN.md`. Alle Einzelwerte (`einstellungen`) mit Standard je Stoff: `PARAMETER.md`.

Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. Objektnummern in Befehlen und Ergebnissen ab 0.

## 1. Design (oberste Ebene)

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `eigener_rahmen` | EigenerRahmenJson |  | – | **Eigener Rahmen** (Rechteck/Oval nach eigenen Maßen): ist das Feld gesetzt, gilt es statt `rahmen` |
| `einstellungen` | EinstellungenJson |  | – | Einzelwerte des Stoffs/Stils überschreiben, z. B. {"satin": {"zugausgleich": 0.3}} (Parameterkatalog) |
| `ende` | Ende |  | – | Endpunkt (Standard: Sprung zurück zur Rahmenmitte) |
| `garnkarte` | string |  | `"Default"` | Garnkarte (Befehl `garnkarten_liste`): jede Farbe → nächstes Garn; „keine“ = ohne (Standard „Default“ wie die Maschine) |
| `gruppen` | Record<string, GruppeJson> |  | – | **Gruppen** („Group“ im Original): Schlüssel → Name und Transformation. Objekte nennen ihre Gruppe im Feld `gruppe`. Reihenfolge der Transformationen: je Objekt → je Gruppe (um die Mitte ihrer Objekte) → ganzes Design |
| `kuerzeste_verbindung` | boolean |  | `false` | Closest Join: jedes Objekt ohne eigenen Einstich beginnt nahe dem vorigen Ende (nicht bei Schrift) |
| `name` | string |  | `"Design"` | Name des Designs (erscheint in der Designliste der Maschine) |
| `objekte` | ObjektJson[] |  | – | Stickobjekte in Stickreihenfolge |
| `rahmen` | string |  | `"artista 130 x 100"` | Stickrahmen (Befehl `rahmen_liste`); bei einem eigenen Rahmen sein Name (gilt dann `eigener_rahmen`) |
| `start` | Start |  | – | Startpunkt (Standard: Rahmenmitte) |
| `stoff` | "0 Software Defaults" \| "Knit Light Weight" \| "Knit Medium Weight" \| "Knit Heavy Weight" \| "Terry Medium Weight" \| "Terry Heavy Weight" \| "Woven Light Weight" \| "Woven Medium Weight" \| "Woven Heavy Weight" |  | `"0 Software Defaults"` | Stoff: ändert Dichte, Unterlage, Zugausgleich (8 Stoffe + Software Defaults) |
| `transform` | Transformation |  | – | Transformation des ganzen Designs (um seine Mitte) |
| `version` | number |  | – | Fassung des Formats (ohne Angabe: aktuelle Fassung) |
| `vorlage` | string |  | – | Vorlage, aus der das Design entstand (Handbuch Kap. 9: „the template from which the design was created“) – nur zur Anzeige und als Vorschlag bei „Werte in Vorlage speichern“; die Werte selbst stehen in `einstellungen` |

## 2. Objekte (`objekte`, Feld `art`)

### Felder aller Objekte

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `ausstich` | Punkt |  | – | Ausstich („exit point“) [x, y] in mm: das Objekt endet dort |
| `auto_jump` | AutoJumpJson |  | – | Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) |
| `doppellauf` | boolean |  | `false` | Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder |
| `effekte` | EffekteJson |  | – | **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei Flächen), sonst ohne Wirkung. An = Feld vorhanden |
| `einstellungen` | EinstellungenJson |  | – | **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}. Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`) |
| `einstich` | Punkt |  | – | Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort |
| `farbe` | string |  | `"#000000"` | Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet) |
| `garn` | string |  | – | **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B. „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte) |
| `gruppe` | string |  | – | **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element) |
| `kuerzeste` | boolean |  | `false` | **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116) |
| `name` | string |  | `""` | Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text |
| `step_muster` | number |  | – | Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) |
| `transform` | Transformation |  | – | Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet |
| `umrandung` | UmrandungJson |  | – | **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz, 0,3 mm nach innen |

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
| `unterlage` | UnterlageJson |  | – | Unterlage: `"auto"` (Standard, Stil „Schrift“ wie im Original: Zickzack unter den Satinsäulen, Step unter Flächen), `"keine"` oder Arten in Stickfolge für Satin/auto, z. B. `["mittellauf", "zickzack"]` (Tatami: jede Liste = an). `true`/`false` (alte Schreibweise) = auto/keine. |
| `winkel` | number |  | – | Stichwinkel (Tatami, Muster) |
| `wortabstand` | number |  | – | Wortabstand in % der Höhe (Vorlage 25) |
| `zeilenabstand` | number |  | – | Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text |

### `monogramm` – Monogramm

Monogramm (Handbuch Kap. 21): 2 oder 3 Initialen mit den Alphabeten Diamond2/Diamond3, Rahmen wahlweise. Wird im Kern zu einem Text – alle Schrift-Einstellungen gelten. **Mit `vorlage`** (eigene Erweiterung): Rahmen aus dem Katalog der Monogramm-Rahmen, Initialen in einer Computer-Schrift oder einem Alphabet (`monogramm.rs`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) |
| `alphabet` | string |  | – | Original-Alphabet der Initialen (nur mit `vorlage`, statt `schrift`) |
| `ausrichtung` | Ausrichtung |  | `"mitte"` |  |
| `breite` | number |  | `100.0` | Breite in % der Höhe |
| `buchstaben` | BuchstabeJson[] |  | – | einzelne Buchstaben: Farbe, Transformation, Stichwinkel |
| `elastisch` | HuelleJson |  | – | Verzerren (Elastic Lettering) |
| `farben` | string[] |  | `[]` | Farben für die Farbwechsel `^` im Text, in Reihenfolge |
| `groesse` | number |  | – | Größe des Füllmusters in % (Stichart „muster“, Standard 100) |
| `grundlinie` | GrundlinieJson |  | – | Grundlinie (ohne Angabe: gerade waagrecht) |
| `hoehe` | number |  | `18.0` | Höhe in mm (18 = kleinste empfohlene Höhe von Diamond2/3) |
| `initialen` | string | ja | – | 2 oder 3 Buchstaben A–Z (Diamond); mit `vorlage` 1–3 Zeichen |
| `kursiv` | number |  | `0.0` | Kursiv in Grad (−45 … 45) |
| `muster` | string |  | – | Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) |
| `position` | Punkt |  | – | Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) |
| `rahmen` | MonogrammRahmen |  | – |  |
| `rahmen_farbe` | string |  | – | Farbe des Rahmens (nur mit Rahmen) |
| `rahmen_farben` | string[] |  | `[]` | Farben des Rahmens statt der Farben der Vorlage: 1., 2., 3. Farbe der Vorlage (nur mit `vorlage`; fehlende bleiben, wie sie sind) |
| `rahmen_groesse` | number |  | – | Größe des Rahmens in mm: längere Seite (nur mit `vorlage`; ohne Angabe wie gezeichnet, 70–88 mm). Die Initialen wachsen mit |
| `schrift` | string |  | – | Computer-Schrift der Initialen (nur mit `vorlage`; ohne Angabe: Schriftvorschlag der Vorlage, Google Fonts) |
| `stichart` | SchriftStichart |  | – |  |
| `unterlage` | UnterlageJson |  | – |  |
| `vorlage` | string |  | – | **Rahmen aus dem Katalog** (eigene Erweiterung): Name eines Monogramm-Rahmens (Befehl `monogramm_rahmen`, z. B. „Doppeloval“). Dann stehen die Initialen in der Computer-Schrift `schrift` (ohne Angabe: Schrift der Vorlage) oder im Alphabet `alphabet`, Höhen nach der Vorlage, mittig im Rahmen und so weit verkleinert, dass rundum 2 mm frei bleiben. `rahmen`, `rahmen_farbe` und `hoehe` gelten dann nicht |
| `winkel` | number |  | – |  |
| `wortabstand` | number |  | – | Wortabstand in % der Höhe (Vorlage 25) |
| `zeilenabstand` | number |  | – | Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text |

### `applikation` – Applikation

Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`). Ohne Angabe gelten die Vorgaben von „Auto Appliqué“ im Original (`AutoAppliqueValues`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abdeckung` | Abdeckung |  | – |  |
| `abstand` | number |  | – | Abstand des Abdeckstichs in mm (0,2 … 10; Original Satin 0,5, Blanket 2,0) |
| `breite` | number |  | – | Breite des Abdeckstichs in mm (0,5 … 10; Original 5,0) |
| `heft_abstand` | number |  | – | Abstand des Heftstichs in mm (0,5 … 12,7; Original 5,0) |
| `heft_breite` | number |  | – | Breite des Zickzack-Heftstichs in mm (0,5 … 10; Original 4,0) |
| `herausfahren` | boolean |  | `true` | Herausfahren (Frame Out des Originals, `Applique::GetJumpoutPoint`): vor jedem Halt der Applikation (nach der Platzierungslinie, bei „Zuschneiden“ nach der Schnittlinie, nach dem Heften) fährt der Rahmen ohne Einstich 12 mm über die Mitte der Oberkante – Stoff auflegen und zuschneiden geht leichter –, danach springt er zurück. Höchstens bis zum Rand des Stickfelds. Standard an wie im Original (dort an, sobald ein Halt eingestellt ist – die Vorgabe); `false` = der Rahmen hält am Ende der Lage |
| `loecher` | PunktAngabe[] |  | – |  |
| `platz_laenge` | number |  | – | Stichlänge der Platzierungslinie in mm (0,5 … 12,7; Original 1,5) |
| `stil` | ApplikationStil |  | – |  |
| `teilabdeckung` | boolean |  | `false` | Teil-Abdeckung (Partial Appliqué): kein Abdeckstich, wo eine spätere Applikation im Design den Rand überdeckt – überlappende Stoffteile ohne doppelte Ränder |
| `umriss` | PunktAngabe | ja | – |  |
| `versatz` | number |  | – | Versatz von Abdeck- und Heftstich gegen die Platzierungslinie in % (Schieberegler „Inside ↔ Outside“ des Originals): −100 = ganz außen, 0 = mittig (Standard), 100 = ganz im Stoff. Bei Blanket ohne Wirkung (immer innen). |

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
| `motiv` | string |  | `"K0001a"` | Motiv im Satz, z. B. „705“ (Standard „K0001a“ wie das Original) |
| `punkte` | PunktAngabe | ja | – |  |
| `satz` | string |  | `"NP004a"` | Motivsatz, z. B. „Heirloom“ (Befehl `motive_liste`; Standard „NP004a“ wie das Original, `<PRESET_STANDARD_MOTIF_RUN>`) |
| `spiegeln` | boolean |  | `false` | Motive auf die andere Seite der Linie spiegeln |

### `motivfuellung` – Motivfüllung (Motiv aus einem Motivsatz in Reihen über die Fläche)

Motivfüllung (Handbuch Kap. 25 „Pattern Fill“): ein Motiv aus einem Motivsatz wird in parallelen Reihen über die Fläche wiederholt. Motive, die mehr als zur Hälfte über den Rand ragen, werden am Rand abgeschnitten.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `groesse` | number |  | – | Länge eines Motivs (Grundlinie) in mm; ohne Angabe: Größe im Motivsatz |
| `loecher` | PunktAngabe[] |  | – |  |
| `motiv` | string |  | `"K0001a"` | Motiv im Satz (Standard „K0001a“ wie das Original) |
| `reihenabstand` | number |  | – | Abstand der Reihen in mm („row spacing“); ohne Angabe: Höhe des Motivs |
| `satz` | string |  | `"NP004a"` | Motivsatz (Befehl `motive_liste`; Standard „NP004a“ wie das Original, `<PRESET_STANDARD_MOTIF_FILL>`) |
| `spaltenabstand` | number |  | – | Abstand der Motive in einer Reihe in mm („column spacing“); ohne Angabe: Breite des Motivs |
| `umriss` | PunktAngabe | ja | – |  |
| `versatz` | number |  | `0.0` | Reihenversatz in mm („row offset“): jede Reihe um so viel weiter verschoben als die vorige (0 = Spalten genau untereinander; halber Spaltenabstand = versetzt wie Ziegel) |
| `winkel` | number |  | `0.0` | Drehung der Reihen in Grad (der Stichwinkel wirkt hier nicht, Handbuch S. 227) |

### `rueckstich` – Rückstich (Backstitch)

Rückstich (Backstitch, Craft Stitch 0x2850): Stiche vor und zurück mit Überlappung, wie von Hand gestickt – eine feine, durchgehende Linie. Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `geschlossen` | boolean |  | `false` | Linie schließen (zurück zum ersten Punkt) |
| `laenge` | number |  | – | Stichlänge in mm (0,5 … 12,7; Original 3,0) |
| `punkte` | PunktAngabe | ja | – |  |
| `staerke` | number |  | – | Fadenstärke in mm = Abstand der Hin- und Rückstiche (0 … 1; Original 0,2) |
| `striche` | number |  | – | Striche je Stich: 3 oder 5 (Original 3) |
| `ueberlappung` | number |  | – | Überlappung in % des Stichs, um die jeder Rückstich in den vorigen greift (0 … 100; Original 50) |

### `stielstich` – Stielstich (Stemstitch)

Stielstich (Stemstitch, Craft Stitch 0x2855): schräge, sich überlappende Stiche zu einer kordelartigen Linie. Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Abstand der Stiche in mm, quer zu den Stichen gemessen (0,2 … 10; Original 1,5) |
| `breite` | number |  | – | Linienstärke in mm = Breite der Kordel (0,5 … 10; Original 2,0) |
| `dreifach` | boolean |  | – | dreifach („Triple“: drei Fäden nebeneinander) statt einfach (Original: dreifach) |
| `geschlossen` | boolean |  | `false` | Linie schließen (zurück zum ersten Punkt) |
| `punkte` | PunktAngabe | ja | – |  |
| `staerke` | number |  | – | Fadenstärke in mm = Abstand der drei Fäden bei dreifach (0 … 1; Original 0,2) |
| `winkel` | number |  | – | Winkel der Stiche zur Linie in Grad (10 … 90; Original 50) |

### `stippling` – Stippling (mäandernde Laufstichfüllung)

Stippling (Craft Stitch „Stipple“): die Fläche wird mit einer mäandernden Laufstichlinie gefüllt, die sich nicht kreuzt – Verfahren des Originals (`StippleFillInterface`: Wabengitter, zufälliger Spannbaum, Linie um den Baum).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Schlaufenabstand in mm = Kantenlänge des Wabengitters, um das die Linie läuft (0,5 … 20; Original 3,0) |
| `laenge` | number |  | – | Stichlänge in mm (0,5 … 12,7; Original 2,0) |
| `loecher` | PunktAngabe[] |  | – |  |
| `umriss` | PunktAngabe | ja | – |  |

### `kreuzstich` – Kreuzstich auf einem Raster (Fläche und/oder einzelne Zellen)

**Kreuzstich** (Handbuch Kap. 34–37, Programmteil „Cross Stitch“ des Originals): Kreuze auf einem festen Raster aus Quadraten. Gefüllt werden die Zellen, deren Mitte in der Fläche liegt, dazu einzelne Zellen aus `zellen`. Das Raster liegt fest am Design (Zelle [i, j] = i·groesse … (i+1)·groesse); Verschieben und Drehen bewegen die Fläche, die Kreuze bleiben auf dem Raster.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `ecke` | KreuzEcke |  | `"unten_links"` | Viertel bzw. Seite für Viertel-, Dreiviertel-, Mini- und längliche Kreuze |
| `faeden` | number |  | `2` | Fäden („Threads“) 2, 4 oder 6 = jedes Kreuz einmal, zweimal, dreimal gestickt |
| `fuellen` | boolean |  | `true` | Fläche füllen („Fill“); aus = nur der Umriss |
| `groesse` | number |  | `2.54` | Kreuzgröße = Rasterweite (Kantenlänge eines Kreuzes) in mm, 1 … 8; Standard 2,54 = 10 Kreuze je Zoll („Fabric Count“ des Originals) |
| `kontur` | KreuzKontur |  | `"keine"` | Kontur („Outline“): keine, Linie auf dem Raster, Kreuze entlang des Umrisses |
| `kreuz` | KreuzArt |  | `"voll"` | Art des Kreuzes (Standard voll) |
| `loecher` | PunktAngabe[] |  | – |  |
| `richtung` | KreuzRichtung |  | `"steigend"` | Halbkreuz: „/“ oder „\“ |
| `umriss` | PunktAngabe |  | – | Fläche (ohne Fläche: nur die Zellen aus `zellen`) |
| `zellen` | number[][] |  | `[]` | einzelne Zellen [Spalte, Reihe] zusätzlich zur Fläche (Spalte i = x von i·groesse bis (i+1)·groesse) |

### `blackwork` – Blackwork Run: mehrere Linien als ein automatisch geordneter Zug

Blackwork Run (Handbuch S. 131): mehrere sich überlappende Linien als **ein** Objekt; die Reihenfolge ergibt sich automatisch (ein Zug je zusammenhängendem Netz, Abschnitte nur doppelt, wo es nötig ist, ein Ein- und ein Ausstich). Jede Linie bleibt einzeln formbar. Werte der Stichart wie bei `lauf`, `rueckstich`, `stielstich`.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand` | number |  | – | Stielstich: Abstand in mm (1,5) |
| `breite` | number |  | – | Stielstich: Linienstärke in mm (2,0) |
| `dreifach` | boolean |  | – | Stielstich: dreifach (ja) |
| `laenge` | number |  | – | Stichlänge in mm (Lauf/Dreifach: Stil 2,5; Rückstich 3,0) |
| `linien` | PunktAngabe[] | ja | – | die Linien (mindestens eine) |
| `staerke` | number |  | – | Rückstich/Stielstich: Fadenstärke in mm (0,2) |
| `stich` | BlackworkStich |  | `"lauf"` | Stichart aller Linien (Standard Laufstich) |
| `striche` | number |  | – | Rückstich: 3 oder 5 Striche (3) |
| `ueberlappung` | number |  | – | Rückstich: Überlappung in % (50) |
| `winkel` | number |  | – | Stielstich: Winkel in Grad (50) |

### `photosnap` – PhotoSnap: Foto als Reihen von Zickzackstichen mit wechselndem Abstand (Befehl `photosnap` legt es an)

PhotoSnap (Handbuch S. 119–120): Foto als Reihen von Zickzackstichen, deren Abstand der Helligkeit folgt („line printer“). Bildmitte = Mitte des Objekts; Lage und Größe über `transform`.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `abstand_max` | number |  | – | größter Abstand in mm an den hellsten Stellen (1 … 12; Standard 10 – Stil 0x2828) |
| `abstand_min` | number |  | – | kleinster Abstand in mm an den dunkelsten Stellen (0,4 … 5; Standard 0,8 – Stil 0x2828) |
| `aufloesung` | PhotoAufloesung |  | – | Auflösung = Breite der Reihen (Standard mittel 2,0 mm) |
| `hintergrund` | PhotoHintergrund |  | – | Stoff hell (Standard) oder dunkel |
| `lagen` | PhotoLagen |  | – | Farblagen gekreuzt (Standard) oder versetzt |
| `raster` | PhotoRasterJson | ja | – |  |
| `winkel` | number |  | – | Winkel der Reihen in Grad (Standard 0 wie im Original; das Handbuch-Beispiel zeigt 45°) |

### `stiche` – Stichdaten aus einer fremden Stickdatei (feste Einstiche, Befehl `stickdatei_lesen` legt sie an)

**Stichdaten** aus einer fremden Stickdatei (DST, PES, JEF, VP3, EXP; Befehl `stickdatei_lesen`): feste Einstiche wie „Grade D“ im Original – werden nicht neu berechnet und nicht zusätzlich vernäht. Transformationen bewegen die Einstiche; Skalieren ändert die Dichte, nicht die Stichzahl (Hinweis ab ±5 % wie im Handbuch S. 256).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `quelle` | string |  | – | Herkunft, z. B. „rose.pes (PES)“ – nur zur Anzeige |
| `stiche` | string | ja | – | Einstiche, Base64 (vom Befehl `stickdatei_lesen` erzeugt – nicht von Hand bearbeiten): Datensätze x, y als i16 in 0,1 mm, +y oben; (−32768, 0) beginnt einen neuen Zug (Sprung) |

## 3. Auswahlwerte

**Abdeckung** – Abdeckstich der Applikation

- `"satin"` – Satin mit Heftstich (Standard)
- `"blanket"` – Blanket (Langette) ohne Heftstich

**ApplikationStil** – Arbeitsweise der Applikation

- `"vorgeschnitten"` – Stoff vorgeschnitten („Pre-cut“, Standard)
- `"zuschneiden"` – im Rahmen zuschneiden („Trim in Place“, mit Schnittlinie)

**BlackworkStich** – Stichart eines Blackwork-Zugs

- `"lauf"` – Laufstich (Standard)
- `"dreifach"` – Dreifachstich
- `"rueckstich"` – Rückstich
- `"stielstich"` – Stielstich

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

**KreuzArt** – Art des Kreuzes (Liste „FillStyle“ des Kreuzstich-Programms des Originals)

- `"voll"` – Full Cross: zwei Diagonalen
- `"dreiviertel"` – 3/4 Cross: Viertel zur Mitte + langer Schenkel
- `"halb"` – 1/2 Cross: eine Diagonale (Feld `richtung`)
- `"viertel"` – 1/4 Cross: von der Ecke zur Mitte (`ecke`)
- `"mini"` – Mini Cross: kleines Kreuz im Viertel (`ecke`)
- `"aufrecht"` – Upright Cross: senkrecht + waagrecht („+“)
- `"doppelt"` – Double Cross: Kreuz + aufrechtes Kreuz (Stern)
- `"laengs"` – Elongated Vertical Cross: Kreuz in der linken oder rechten Hälfte (Seite von `ecke`)
- `"quer"` – Elongated Horizontal Cross: Kreuz in der unteren oder oberen Hälfte (Seite von `ecke`)
- `"knoten"` – French Knot: kleiner Knoten in der Zellmitte

**KreuzEcke** – Ecke bzw. Viertel einer Zelle („Select the quadrants to be filled“)

- `"unten_links"`
- `"unten_rechts"`
- `"oben_links"`
- `"oben_rechts"`

**KreuzKontur** – Kontur eines Kreuzstich-Objekts (Liste „OutlineStyle“ des Originals)

- `"keine"` – kein Umriss
- `"linie"` – Single Line: Laufstich auf den Rasterlinien um die Kreuze
- `"kreuz"` – Current Cross Type: Kreuze auf allen Zellen, durch die der Umriss läuft

**KreuzRichtung** – Richtung des Halbkreuzes

- `"steigend"` – „/“
- `"fallend"` – „\“

**MonogrammRahmen** – Rahmen um ein Monogramm

- `"raute"` – Raute
- `"zier"` – Zierwinkel
- `"keine"` – kein Rahmen (Standard)

**PhotoAufloesung** – Auflösung von PhotoSnap = Breite der Stichreihen

- `"grob"` – grob (Reihen 3,0 mm)
- `"mittel"` – mittel (Reihen 2,0 mm, Standard)
- `"fein"` – fein (Reihen 1,0 mm)

**PhotoHintergrund** – Stoff hinter PhotoSnap

- `"hell"` – heller Stoff: dunkle Stellen bekommen viel Faden (Standard)
- `"dunkel"` – dunkler Stoff: helle Stellen bekommen viel Faden

**PhotoLagen** – Lage der Farbschichten von PhotoSnap zueinander (nur mit mehreren Lagen)

- `"gekreuzt"` – je Lage 45° weiter gedreht: 0°, 45°, 90°, 135° (Standard)
- `"versetzt"` – gleiche Richtung, Reihen je Lage 0,4 mm versetzt

**Profil** – Profil des Verlaufs (Dialog „Gradient Fill“: Linear, Dec-Linear, Convex, Concave; „Color Blending“ zusätzlich Inc_Exp/Dec-Exp und Convex-Concave/Concave-Convex). Abstand vom kleinsten zum größten Wert über die Fläche

- `"linear"` – vom kleinsten zum größten Abstand, gleichmäßig („Linear“, Datensatz 1)
- `"abnehmend"` – vom größten zum kleinsten Abstand („Dec-Linear“, Datensatz 2)
- `"konvex"` – zunehmend, erst schnell, dann langsam („Convex“, Datensatz 3; Kurvenform eigene Festlegung)
- `"konkav"` – abnehmend, erst schnell, dann langsam („Concave“, Datensatz 4; Kurvenform eigene Festlegung)
- `"exp_zunehmend"` – zunehmend, erst langsam, dann schnell („Inc_Exp“, nur Farben mischen; Kurvenform eigene Festlegung)
- `"exp_abnehmend"` – abnehmend, erst langsam, dann schnell („Dec-Exp“; Kurvenform eigene Festlegung)
- `"konvex_konkav"` – S-Kurve zunehmend („Convex - Concave“; Kurvenform eigene Festlegung)
- `"konkav_konvex"` – S-Kurve abnehmend („Concave - Convex“; Kurvenform eigene Festlegung)

**RahmenForm** – Form eines eigenen Rahmens (Handbuch Kap. 6: Format Rectangle / Oval)

- `"rechteck"` – Rechteck Breite × Höhe
- `"oval"` – Oval: gerader Mittelteil (`gerade`), oben und unten rund bis zur Höhe

**RandSeite** – Seite des ausgefransten Rands, in Stichrichtung gesehen (Dialog „Textured Edge“: Left, Both, Right)

- `"links"` – Anfang der Reihen (bei Satin: Seite A)
- `"rechts"` – Ende der Reihen (bei Satin: Seite B; Standard wie das Original: Seite 2)
- `"beide"` – beide Seiten

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
- `"auto"` – Satinsäulen; würde ein Satinstich nach Größe/Transformation länger als 8 mm, der ganze Text Step – wie Stichart „auto“ der Texte (eigene Erweiterung, `Analyse_Teil3/10` §8)
- `"muster"` – Füllmuster (Feld `muster`)

**UmrandungArt** – Stichart der Umrandung

- `"lauf"` – Laufstich (dünne Linie)
- `"dreifach"` – Dreifachstich (kräftige Linie, Standard)
- `"satin"` – Satinkante

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

### EffekteJson

Effekte eines Objekts (`stick_kern::effekte`, Bericht `Analyse_Teil3/23_effekte.md`).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `rand` | RandJson |  | – | **Ausgefranster Rand** (Textured Edge): Reihen- bzw. Stichenden streuen um den Rand (Handbuch S. 213–214) |
| `verlauf` | VerlaufJson |  | – | **Verlauf** (Gradient Fill): Abstand wechselt quer über die Fläche bzw. entlang der Säule zwischen `min` und `max`; der Abstand des Objekts gilt dann nicht (Handbuch S. 218) |
| `welle` | PunktAngabe |  | – | **Welle** (Wave Effect): Leitlinie [[x, y], …] in mm quer durch die Fläche, die Reihen folgen ihr (nur `tatami` und `muster`; Handbuch S. 217–218: Stichlänge ≤ 4 mm empfohlen) |

### EigenerRahmenJson

**Eigener Rahmen** (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“): Stickfeld nach eigenen Maßen. Zum Gestalten frei; beim Senden prüft das Programm, in welchen Rahmen der artista 185 die Stiche passen, und nennt ihn (passt keiner, wird nicht gesendet). Die Liste der eigenen Rahmen verwalten `rahmen_speichern`/`rahmen_loeschen`.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `breite` | number | ja | – | Breite des Stickfelds in mm (10 … 1000) |
| `form` | RahmenForm |  | `"rechteck"` |  |
| `gerade` | number |  | `0.0` | Oval: Höhe des geraden Mittelteils in mm („Rectangle Height“; 0 = Ellipse) |
| `hoehe` | number | ja | – | Höhe des Stickfelds in mm (10 … 1000; Oval: Gesamthöhe „Total Height“) |
| `name` | string | ja | – | Name (Rahmenliste; nicht der Name eines Rahmens der 185) |

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

### PhotoLageJson

Eine Farblage von PhotoSnap.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `farbe` | string | ja | – | Garnfarbe der Lage „#rrggbb“ (bei nur einer Lage gilt `farbe` des Objekts) |
| `werte` | string | ja | – | Deckung je Bildpunkt als Base64 (ein Byte je Punkt, zeilenweise von oben; 0 = kein Faden, 255 = voll – für hellen Stoff) |

### PhotoRasterJson

Bildraster von PhotoSnap (vom Befehl `photosnap` erzeugt, nicht von Hand).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `breite` | number | ja | – | Bildpunkte waagrecht |
| `hoehe` | number | ja | – | Bildpunkte senkrecht |
| `lagen` | PhotoLageJson[] | ja | – | Farblagen in Stickreihenfolge: eine (Graustufen) oder vier (Cyan, Magenta, Gelb, Schwarz) |
| `mm` | number | ja | – | Größe eines Bildpunkts in mm |

### RandJson

Ausgefranster Rand (Textured Edge, ART-Datensatz 0x32: Seite, Spanne, Texture). Standard wie der Effekt-Datensatz im Stil der Musterdesigns (Seite 2, Spanne 1,5 mm).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `glaette` | number |  | `0.1` | Glätte 0 … 1 („Texture“-Regler: 0 = rau, jede Reihe frei; 1 = glatt) |
| `seite` | RandSeite |  | `"rechts"` | Seite in Stichrichtung (links = Anfang der Reihen bzw. Satin-Seite A) |
| `spanne` | number |  | `1.5` | Spanne in mm („Span“): Enden streuen bis so weit nach innen und außen (0,1 … 10) |

### Transformation

Transformation in der JSON-Form (`"transform"` am Objekt oder am Design).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `drehen` | number |  | `0.0` | Drehung in Grad gegen den Uhrzeiger |
| `schraeg` | number |  | `0.0` | Schrägstellen in Grad (x verschiebt sich mit y, wie Skew im Original) |
| `skalieren` | Skalierung |  | – | Skalierung in %: eine Zahl (beide Richtungen) oder [x, y] |
| `spiegeln` | Spiegeln |  | – |  |
| `verschieben` | Punkt |  | – | Verschieben in mm |

### UmrandungJson

**Umrandung** – eigene Erweiterung (Wunsch Nutzer 21.11.2026; im Original legt man dafür von Hand eine zweite Linie an, Handbuch S. 114): eine Linie um die Fläche des Objekts, gestickt gleich nach dem Objekt in eigener Farbe. Bei Text läuft sie um jeden Buchstaben, bei mehreren Teilen um jeden Teil (Löcher bekommen auch eine Linie). Sie liegt um `einzug` nach innen, damit nach dem Zusammenziehen des Stoffs keine Lücke zwischen Füllung und Umrandung bleibt. Wirkt bei Objekten mit Fläche (Flächen, Satin, Satinlinie, Text, Monogramm, Applikation); offene Linien haben keine.

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `art` | UmrandungArt |  | `"dreifach"` | Stichart der Linie (Standard Dreifach) |
| `aussen` | boolean |  | `false` | Satinkante **außen** um die Fläche (nur bei `art` satin): die Innenkante liegt `einzug` mm in der Fläche, die Kante wächst nach außen, die Farbe des Objekts bleibt ganz sichtbar (dicke Umrandung im einfachen Modus). Ohne: Mitte der Kante `einzug` mm in der Fläche (eigene Festlegung, Wunsch Nutzer 21.11.2026) |
| `breite` | number |  | `2.0` | Breite der Satinkante in mm, 0,5 … 10 (nur bei `art` satin; Standard 2 wie die Satinlinie). Die Kante ist ein Band um die Fläche, in Satinsäulen zerlegt wie „Satin automatisch“ (`satinflaeche`) |
| `breite_anteil` | number |  | – | Breite der Satinkante in **Prozent der Buchstabenhöhe**, 1 … 30 (nur Text und Monogramm, gilt statt `breite`; Größe/Skalieren zählt mit). Einfacher Modus: 6 → 3 mm bei 50 mm hohen Buchstaben; unter 1 mm wird daraus eine kräftige Linie (Dreifach, Stichlänge 12 % der Höhe, 1–2,5 mm) wie bei EasyStitch (Wunsch Nutzer 21.11.2026, eigene Festlegung) |
| `einzug` | number |  | `0.3` | Versatz der Linie nach innen in mm, 0 … 3 (Standard 0,3 – eigene Festlegung, am Kalibrierblatt V12 prüfen); bei `aussen`: wie weit die Innenkante der Satinkante in der Fläche liegt |
| `farbe` | string |  | `"#000000"` | Garnfarbe „#rrggbb“ (Standard schwarz) |
| `laenge` | number |  | – | Stichlänge in mm bei Lauf/Dreifach, 0,3 … 12,7 (ohne Angabe: Stil) |

### VerlaufJson

Verlauf (Gradient Fill, ART-Datensatz 0x34). Standard wie der Effekt-Datensatz im Stil der Musterdesigns (0,4 / 2,0 mm, Profil 1).

| Feld | Typ | Pflicht | Standard | Beschreibung |
|---|---|---|---|---|
| `max` | number |  | `2.0` | größter Abstand in mm (offenste Stelle, „Maximum Spacing“; 0,1 … 10) |
| `min` | number |  | `0.4` | kleinster Abstand in mm (dichteste Stelle; 0,1 … 10) |
| `profil` | Profil |  | `"linear"` | Profil (Dialog „Gradient Fill“ und „Color Blending“) |

## 5. Befehle

Aufruf aus der Oberfläche: `kern("name", eingabe)` aus `kern.ts` (ein Tauri-Befehl `kern`, Einbindung siehe `kern/stick-api/src/lib.rs`); zum Ausprobieren ohne Oberfläche: `stick api NAME '{…}'`. Fehler: `{meldung, objekt}`.

| Befehl | Eingabe | Ausgabe | Beschreibung |
|---|---|---|---|
| `alphabet_vorschau` | AlphabetVorschauEingabe | AlphabetVorschau | Vorschau eines Original-Alphabets: Umrisse eines Probetexts und Zeichenvorrat |
| `alphabete_liste` | Leer | AlphabetInfo[] | Original-Alphabete (50) |
| `ausrichten` | AusrichtenEingabe | AusrichtenErgebnis | Ausrichten (links, rechts, oben, unten, Mitten) nach dem zuletzt gewählten Objekt – Gruppen als Ganzes |
| `beispiel_design` | Leer | DesignWert | Beispieldesign mit allen Sticharten der ersten Ausbaustufe |
| `bild_umwandeln` | BildEingabe | BildErgebnis | Bild in Stickflächen umwandeln (neues Design) |
| `bild_vorschau` | BildEingabe | BildVorschau | Vorschau der Bildumwandlung: vereinfachtes Bild (PNG) und Farbliste zum Weglassen – vor dem Umwandeln |
| `blackwork_lauf` | BlackworkEingabe | BlackworkErgebnis | Blackwork Run: gewählte Linien zu einem automatisch geordneten Objekt zusammenfassen (Stichart des ersten) |
| `craft_vorlagen` | Leer | CraftVorlage[] | Vorlagen der Craft-Stitch-Leiste (Rückstich, Stielstich, Blackwork, Candlewicking, Lacework, Stippling) mit den Werten des Originals |
| `datei_lesen` | Pfad | string | Textdatei lesen |
| `design_berechnen` | DesignEingabe | Ergebnis | Stiche berechnen – bei jeder Änderung am Design aufrufen (Stiche werden nie skaliert, immer neu berechnet) |
| `design_einfuegen` | EinfuegenEingabe | EinfuegenErgebnis | Design einfügen wie „Insert Design“: Objekte anhängen, auf Wunsch als eine Gruppe (Bild, SVG, Datei als Element) |
| `design_formen` | DesignEingabe | ObjektForm[] | Formen je Objekt (Umrisse, Linien, Satinseiten, Ein-/Ausstich) zum Zeichnen und Bearbeiten mit der Maus |
| `design_pruefen` | DesignEingabe | Pruefung | Design prüfen, ohne Stiche zu berechnen: Fehler (mit Objektnummer) und Hinweise |
| `design_schema` | Leer | unknown | JSON-Schema des Design-Formats (Felder, Standardwerte, Auswahlwerte, Beschreibungen) |
| `exp_speichern` | ExpEingabe | number | Stiche als EXP-Datei speichern (Ergebnis: Zahl der Bytes) |
| `farben_mischen` | FarbenMischenEingabe | FarbenMischenErgebnis | Farben mischen (Color Blending): Objekt in zwei Lagen mit gegenläufigem Verlauf teilen, obere Lage in der zweiten Farbe, beide gruppiert |
| `fuellmuster_liste` | Leer | MusterInfo[] | Füllmuster (242) mit Linien einer Kachel für die Vorschau |
| `garnkarte_garne` | GarnkartenName | GarnInfo[] | Garne einer Garnkarte (für die Auswahl von Hand) |
| `garnkarten_liste` | Leer | string[] | Namen der Garnkarten (32 der CD) |
| `gelesen_exp_speichern` | ExpZiel | number | Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern |
| `google_schrift_laden` | GoogleLadenEingabe | SchriftInfo | Google-Schrift herunterladen (Internet) in die Ablage des Programms – danach wie eine installierte Schrift |
| `google_schriften_liste` | Leer | GoogleKatalog | Katalog der Google Fonts (1 833 Schriften, freie Lizenzen) mit Stilen zum Filtern und „geladen“ |
| `in_knoten_umwandeln` | ObjektEingabe | ObjektWert | Objekt in Knoten umwandeln (Formen → Punkte/Bézier, Transformation angewandt) – für das Knoten-Werkzeug |
| `kranz` | KranzEingabe | KopienErgebnis | Kranz (Wreath): Kopien der Auswahl rund um einen Mittelpunkt, mit Kaleidoskop jede zweite gespiegelt |
| `kreuzstich_bild` | KreuzstichBildEingabe | KreuzstichBildErgebnis | Bild → Kreuzstich: neues Design, je Farbe ein Kreuzstich-Objekt mit seinen Zellen |
| `kreuzstich_vorschau` | KreuzstichBildEingabe | BildVorschau | Bild → Kreuzstich wie „Auto Stitch Image“: Vorschau auf dem Raster (ein Bildpunkt je Kreuz) und Farbliste |
| `maschine_abbrechen` | Leer | boolean | Laufende Maschinen-Aktion abbrechen (beim Warten auf „Start“: nicht sticken, PC-Modus verlassen) |
| `maschine_lesen` | LeseEingabe | GelesenInfo | Design aus der Maschine lesen (Befehl 0x400, nur lesen) zum Ansehen |
| `maschine_liste` | Anschluss | ListenInfo | Designliste der Maschine – nur lesen |
| `maschine_planen` | SendeEingabe | SendePlanInfo | Senden planen – nur lesen: Prüfungen wie das Original, geplante Schreibzugriffe zum Bestätigen |
| `maschine_ports` | Leer | string[] | Maschine: serielle Ports (COM…) |
| `maschine_pruefen` | Anschluss | MaschinenInfo | Maschine prüfen – nur lesen: Firmware, Garnfarbtabelle; PC-Modus wird verlassen |
| `maschine_senden` | SendeEingabe | SendeInfo | **Schreibt in die Maschine** (nur nach bestätigtem Plan): DATEN, ICON, INFO, Garnfarbtabelle; mit `wahl.sticken` Befehl 0x08 (Rahmen fährt) |
| `maschine_starten` | Leer | boolean | „Start“ nach dem Übertragen (Senden mit `wahl.auf_start_warten`): Befehl 0x08, der Rahmen fährt in die Startlage |
| `monogramm_rahmen` | Leer | MonogrammKatalog | Monogramm-Rahmen aus dem Katalog (47 in 8 Kategorien, eigene Erweiterung) mit Zeichnung, Farben und Schriftvorschlag |
| `motiv_loeschen` | MotivLoeschenEingabe | MotivsatzInfo[] | Motiv aus einem eigenen Motivsatz löschen (Motive der CD bleiben); liefert die Motivsätze danach |
| `motiv_speichern` | MotivSpeichernEingabe | MotivsatzInfo | Ausgewählte Objekte als Motiv in einen eigenen Motivsatz speichern (Create Pattern; Ablage im Datenordner) |
| `motive_liste` | Leer | MotivsatzInfo[] | Motivsätze (14 der CD und eigene) für Motivlinie und Motivfüllung |
| `motivsatz_motive` | MotivsatzName | MotivInfo[] | Motive eines Motivsatzes mit Maßen und Linien für die Vorschau |
| `objekt_werte` | ObjektEingabe | WirksamerWert[] | Wirksame Werte eines Objekts: für leere Felder der Wert, mit dem der Kern rechnet (Stil/Stoff, Alphabet, Füllmuster, Motivsatz, Vorlage) – Anzeige im Inspektor statt „Stil“ |
| `parameter_katalog` | Leer | Parameter[] | Parameterkatalog: jeder einstellbare Wert mit Standard, Bereich, Quelle, Modus einfach/experte/automatisch |
| `photosnap` | PhotosnapEingabe | PhotosnapErgebnis | PhotoSnap: Foto als Reihen von Zickzackstichen (einfarbig oder vier Farblagen) – neues Design mit einem Objekt |
| `rahmen_liste` | Leer | RahmenInfo[] | Stickrahmen der artista 185 mit Stickfeld und Umriss, danach die eigenen Rahmen (Feld `eigen`) |
| `rahmen_loeschen` | RahmenLoeschenEingabe | RahmenInfo[] | Eigenen Rahmen löschen; Ergebnis: neue Rahmenliste |
| `rahmen_speichern` | RahmenSpeichernEingabe | RahmenInfo[] | Eigenen Rahmen speichern (Rechteck/Oval, Handbuch Kap. 6) – gleicher Name ersetzt; Ergebnis: neue Rahmenliste |
| `reihenfolge_aendern` | ReihenfolgeEingabe | ReihenfolgeErgebnis | Objekte umordnen wie Menü „Arrange“: wie ausgewählt, an Anfang/Ende, vor/nach Nummer, nach Farbe |
| `saeulen_vorschau` | SaeulenEingabe | SaeuleInfo[] | Fläche in Satinsäulen zerlegen (Vorschau der automatischen Stichrichtung) |
| `schrift_stile` | Leer | SchriftStile | Stile der Schriftauswahl (Gotisch, Schreibschrift, Modern … mit Erklärung) in Anzeigereihenfolge |
| `schrift_vorschau` | SchriftVorschauEingabe | SchriftVorschau | Vorschau einer Computer-Schrift (Umrisse eines Probetexts) mit „stickbar ab … mm“ |
| `schrift_wechseln` | SchriftWechselEingabe | ObjektWert | Schrift eines Texts wechseln: Original-Alphabet (`text`) ↔ Computer-Schrift (`ttf`), Text und Satz bleiben |
| `schriften_liste` | SchriftenListeEingabe | SchriftInfo[] | installierte Computer-Schriften (TrueType/OpenType) und heruntergeladene Google Fonts |
| `spiegeln_kopieren` | SpiegelnKopierenEingabe | KopienErgebnis | Mirror-Merge: gespiegelte Kopie(n) der Auswahl an einer Linie durch den Ankerpunkt, auf Wunsch zusammengeführt |
| `step_muster_liste` | Leer | StepMusterInfo[] | Step-Muster 1–20 (Tabelle des Originals) mit Einstichen einer Probefläche für Vorschau-Kacheln |
| `stichart_aendern` | StichartEingabe | StichartErgebnis | Stichart wechseln (Umriss: lauf, dreifach, satinlinie, blanket, motivlinie, rueckstich, stielstich · Füllung: tatami, satinflaeche, muster, motivfuellung, stippling) – Form bleibt |
| `stickdatei_endungen` | Leer | string[] | Endungen der Stickdateien, die `stickdatei_lesen` kennt (für den Dateidialog) |
| `stickdatei_lesen` | StickdateiEingabe | StickdateiErgebnis | Fremde Stickdatei öffnen (DST, PES, PEC, JEF, VP3, EXP): neues Design mit festen Stichen je Farbe (Objektart `stiche`, wie „Grade D“ im Original), kleinster passender Rahmen der 185 |
| `stoffe_liste` | Leer | StoffInfo[] | Stoffe (8 + Software Defaults) mit den wichtigsten Werten |
| `svg_lesen` | SvgEingabe | SvgErgebnis | SVG-Datei einlesen (z. B. aus Inkscape): Stichart nach Breite wie EasyStitch, neues Design |
| `text_speichern` | TextDatei | null | Textdatei schreiben |
| `ueberdeckungen` | DesignEingabe | number[][] | Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken |
| `vorlage_loeschen` | VorlageName | VorlageInfo[] | Vorlage löschen (Normal → zurück zur Werkseinstellung); Ergebnis: neue Liste |
| `vorlage_neu` | VorlageName | DesignWert | Neues Design aus einer Vorlage (Kopie mit Name „Design“) |
| `vorlage_speichern` | VorlageSpeichernEingabe | VorlageInfo[] | Design als Vorlage speichern (`umfang` ganz) oder nur seine Werte in eine Vorlage (`werte`); Ergebnis: neue Liste |
| `vorlagen_liste` | Leer | VorlageInfo[] | Vorlagen (Handbuch Kap. 9): Normal zuerst (ohne Datei = Werkseinstellung) |
| `welle_vorschlag` | ObjektEingabe | unknown | Leitlinie für den Wave-Effekt vorschlagen (Objektkoordinaten, für `effekte.welle`) |
| `zauberstab` | ZauberstabEingabe | ZauberstabErgebnis | Zauberstab (Magic Wand): angeklickte Flächen des vorbereiteten Bildes als Fläche oder Umriss (neues Design) |

**Ereignis** `maschine_fortschritt` (Typ `Fortschritt`: `was`, `erledigt`, `gesamt`, `wiederholungen`): Fortschritt von `maschine_senden` und `maschine_lesen`, von der App gesendet. Fehler der Maschine tragen zusätzlich `code` (`FehlerCode`). **Sicherheit:** nur `maschine_senden` schreibt in die Maschine – erst nach `maschine_planen` und ausdrücklicher Bestätigung; `wahl.sticken` nur nach dem Gefahrendialog.
