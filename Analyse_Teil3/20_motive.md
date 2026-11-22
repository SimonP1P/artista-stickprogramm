# 20 – Motive: Motivsätze, Motivlinie (Pattern Run), Motivfüllung

Stand 21.11.2026. Handbuch Kap. 25 „Pattern Runs and Fills“ (PDF-S. 225–233). Werkzeug
`Arbeitsdaten/werkzeuge/art_motive.py`, Rohdaten `Arbeitsdaten/messungen/motivlinien_musterdesigns.txt`.

## 1. Motivsätze — Sicherheit: hoch (gelesen)
- Ordner `Main/PROGRAM/userletw/` der CD: **14 Motivsätze** (ESB, gleicher Aufbau wie die Alphabete, Bericht 09):
  BW004a (Blackwork, 84), Children (9), Corners (21), cross (18), CW004a (Candlewicking, 5), Embellishment (15),
  Heirloom (21), Home Dec (32), LW004a (Lacework, 6), Misc (14), nature (17), NP004 (36), Quilting (30),
  Satin-Quilt (12). Handbuch S. 230: „Pattern sets are stored in the C:\BERNINA artista\Userletw folder.“
- Streams heißen nach dem Motivnamen als UTF-16 hex (`003700300039` = „709“); CW004a hat Klartextnamen mit
  vorangestelltem U+FFFF („Knot-3mm“). Der Name steht in 0x3a (ganze Zeichenkette, nicht nur ein Zeichen).
- Je Motiv: **Grundlinie 0x2e** = die zwei Bezugspunkte (Handbuch S. 231: „Reference points set the standard orientation
  of the pattern … also set the spacing between each repetition“), Objekte meist EmbroideryGroup 0x49 + EmbroideryCurve
  0x05 mit Lauf; Satin-Quilt mit Satinsäulen.
- Kern: `stick-formate::esb::motive_laden`, Programm-Ressource `app/src-tauri/motive/` (Kopie von `userletw/`).

## 2. Stile — Sicherheit: mittel (Werte gelesen, Zuordnung teils erschlossen)
NORMAL.AMT `<PRESET_STANDARD_MOTIF_RUN>`, `<PRESET_MOTIF_RUN_1/2>`, `<PRESET_STANDARD_MOTIF_FILL>`, `<PRESET_MOTIF_FILL_1…3>`
(`Analyse/daten/entschluesselt/styles.txt`):
> **Korrektur 22.11.2026 (Bericht [24](24_craft_stitches.md) §1):** Die beiden Sticharten waren hier vertauscht.
> Richtig: **0x2798 = Motivlinie**, **0x27ee = Motivfüllung**. Beleg: Jeder Motivstil setzt seine eigenen Werte (Flag 0)
> unter der gemeinten Stichart; `Children`/`901` unter 0x2798 der Füllstile ist nur geerbt (Flag 0x2000). House2.ART
> trägt am MotifFill-Objekt 0x27ee. Standard beider Arten: **NP004a / K0001a**.

- Motivlinie = Stichart **0x2798**: Satz 0x27b0, Motiv 0x27b1, 0x27b6/0x27b9 (z. B. 1440), Abstand 0x27df
  (`<PRESET_STANDARD_MOTIF_RUN>`: NP004a/K0001a, 2146 = 11,92 mm), Laufweg 0x271d.
- Motivfüllung = Stichart **0x27ee**: Motiv `0x27ef/0x2801` (Satz 0x27b0, Name 0x27b1), Größe 0x3ab8/0x3aba,
  Spalten-/Reihenabstand 0x27b6/0x27b9 (`<PRESET_STANDARD_MOTIF_FILL>`: NP004a/K0001a, 2158/2158; die Werte
  „70084s“ 1638 × 990 mit Abstand 0 stehen in den Linienstilen und gelten dort nicht).
- Auto-Spacing-Faktor im Motivstil 0x2736 = 1,0 (Kern `MOTIV_FAKTOR`).

## 3. Motivlinie im Design — Sicherheit: hoch (gemessen, 156 Linien)
Objekt **MotifText 0x7a** (wie LetterText 0x7b): Linie = GeoCurve 0x01 direkt davor; dann **Vorlage** (LetterObj 0x2f,
Grundlinie 0x2e, Name 0x3a, Form), Datensatz **0x7c** (3 Byte, double **Größe** in 1/180 mm, double 0,25, u32 30, …,
Byte 56 = Kopien + 1), danach **je Kopie ein LetterObj 0x2f** (gepackte Wörter: Oberpunkt x/y, Grundlinie Anfang x/y,
Ende x/y; Floats: Seitenverhältnis, Lage als Anteil der Linie) mit den kopierten Formen. Stichfunktionen: je Kopie Lauf,
dann Verbinder 0x14.

Messung (`art_motive.py`, alle 22 Sampler „Pattern Outlines“, 175 Motivlinien):

| Befund | Wert |
|---|---|
| Kopien behalten ihre Größe (Grundlinienlänge der Kopie = Größe) | alle |
| Anzahl n = round(Linienlänge / Größe), Anfang Kopie i = i · L / n | **156 von 156** (ohne Ecken) |
| … davon Größenfeld 0x7c ≠ Länge der Kopie (Cross 2: b302–b306), Regel gilt mit der Kopienlänge | 4 |
| Beispiel Heirloom 705: L = 22 148, Größe 2038, L/Größe = 10,87 → 11 Kopien, Schritt 2013 (= L/11, überlappend) | |
| Beispiel Children 929: Größe 1802, L/Größe = 12,29 → 12 Kopien, Schritt 1846 (= L/12, mit Lücke) | |
| Eckmotive (Corners, 19): Linie kürzer als ein halbes Motiv → 1 Kopie, Lage anders (offen) | 19 |

Das ist die Regel des Handbuchs (S. 225): „If the space remaining … is less than half a pattern, the extra space is spread
evenly … If the space is larger than half a pattern, BERNINA artista inserts an extra pattern, and slightly overlaps the
patterns.“ Die letzte Kopie ragt dabei um Größe − L/n über das Linienende (gerade verlängert).

**Umsetzung** (`stick-kern/src/motiv.rs`, JSON `"art": "motivlinie"`): Größe ohne Angabe = Grundlinie im Motivsatz;
Abstand ohne Angabe = Größe. Drehung jeder Kopie = Richtung der Sehne Anfang → Ende auf der Linie (bei geraden Linien
gemessen; Kurven: **eigene Festlegung**, die Musterdesigns haben fast nur gerade Linien). Geschlossene Linie: ringsum.
Spiegeln = andere Seite der Linie (Handbuch „orientation“; **eigene Festlegung** für das Feld). Läufe der Motive stechen
auf jeden Knoten (wie Bericht 07). Alle Teile aller Kopien werden ohne Sprünge nacheinander gestickt (Kette).

## 4. Offen
1. **Abstand ≠ Größe**: Feld „Spacing“ des Dialogs – in den Musterdesigns immer Standard; Deutung als Wiederholabstand ist
   mittel.
2. **Eckmotive** (Corners): Lage bei zu kurzer Linie.
3. **Motivfüllung (Pattern Fill)**: Objekt MotifFill 0x3d (Datensatz 0x3f mit Abständen, z. B. `0000 a005 a005` =
   1440/1440). Das einzige Musterdesign mit einer Motivfüllung (House2) hat ein MotifFill-Objekt mit 0x27ee; die
   Einträge 0x2798 „MOTIFS“/„21“ sind Motivlinien (korrigiert 22.11.2026). Der Satz „MOTIFS“ fehlt auf der CD →
   Raster nicht messbar. **Umgesetzt 22.11.2026** (§5).
4. Sonderarten BW004a/CW004a/LW004a (Blackwork, Candlewicking, Lacework) haben eigene Stil-Teile (0x2828 … 0x2855) –
   Teil von „Craft Stitches“ → Bericht [24](24_craft_stitches.md) (umgesetzt 22.11.2026, N3).

## 5. Motivfüllung im Kern (22.11.2026, Nachtauftrag N1)
> **Überholt (22.11.2026, N3):** Die folgenden Werte sind die *geerbten* Werte der Linien-Stichart 0x2798. Der Standard
> der Motivfüllung ist **NP004a / K0001a**, Größe 0x3ab8 = 2158, Raster 2158/2158 (Bericht 24 §1); so im Kern.

**Standardwerte (alt, N1).** `<PRESET_STANDARD_MOTIF_FILL>` 0x2798: Satz 0x27b0 = „Children“,
Motiv 0x27b1 = „901“, **0x27df = 2786** = Länge der Grundlinie von Children 901 (15,48 mm, `motivsatz_motive`),
**0x27b9 = 2781** ≈ Breite des Motivkastens (Kern: 15,45 mm), **0x27b6 = 1000** ≈ Höhe des Motivkastens (5,55 mm);
Leitmotiv-Matrix 0x3abe = (1, 0, 0, 1, 0, 0) = keine Drehung/Scherung/Verschiebung. Gedeutet: Spaltenabstand =
Breite, Reihenabstand = Höhe des Motivs (Motive Kante an Kante) – mittel (ein Beispiel, Zuordnung 0x27b6/0x27b9 über
die Zahlenwerte).

**Regeln — Handbuch S. 227–229, Umsetzung `stick-kern/src/motiv.rs` (`motivfuellung_objekte`):**
- Motive in parallelen Reihen; Spalten-/Reihenabstand, Reihenversatz (jede Reihe um den Versatz weiter – „Changing the
  offset distorts the rows“), Größe, Drehung der Reihen („The stitch angle has no effect on pattern layout“).
- „If more than half a pattern is outside the boundary, it will be clipped along the boundary“: Anteil nach der
  Länge der Motivlinien; mehr als die Hälfte draußen → am Rand abgeschnitten, sonst bleibt das Motiv ganz (Deutung,
  mittel). Motive ganz draußen entfallen.
- **Eigene Festlegungen:** Raster mittig im Flächenkasten (im Original über die Leitmotive verschiebbar); Reihen von
  oben nach unten, in der Reihe links → rechts (Motive sind „from left to right“ digitalisiert, S. 230); Motive mit
  Lücke ≤ 1 mm ohne Sprung verbunden, sonst Sprung.

**Eigene Motive (Create Pattern, S. 230–231):** Objekte (Lauf, Satinsäule, Fläche) werden in Knoten umgewandelt und als
eigener Satz im Datenordner gespeichert (`stick-api/src/motive.rs`, JSON statt ESB – eigene Erweiterung); Bezugspunkte
ohne Angabe linke/rechte Kante auf halber Höhe (eigene Festlegung; im Original angeklickt). Flächen werden im Motiv
Step-Flächen, Dreifach wird Lauf (eigene Festlegung).
