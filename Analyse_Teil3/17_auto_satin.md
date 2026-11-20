# 17 – Satin-Stichrichtung automatisch (20.11.2026, ROADMAP 4b.17 Baustein, auch für 4b.11)

**Ergebnis:** Das Original zerlegt gezeichnete oder aus Bildern gewonnene Formen selbst in **Satinsäulen mit
Stichlinien**. Das Verfahren steckt in EZSTITCH.DLL: Rand abtasten, triangulieren, bedeutende Dreiecke finden, an
Abzweigungen schneiden, je Teil eine Säule. Die Ergebnisse liegen in den Musterdesigns als **EmbroideryPatch mit
Stichlinien** vor (Format entschlüsselt). Der Kern baut das Verfahren in Aufbau und Werten nach (`stick_kern::saeulen`,
JSON `satinflaeche`, TrueType-Schrift `"stichart": "satin"`, CLI `stick saeulen`) und trifft die 54 automatisch
erzeugten Säulen der Musterdesigns: **54/54 eine Säule wie das Original, Stichrichtung im Median 4,4° daneben, 78 %
unter 10°**. Werkzeug `Arbeitsdaten/werkzeuge/art_autosatin.py`.

## 1. Gespeicherte Form: EmbroideryPatch mit Stichlinien – Sicherheit: hoch
Lesefunktion IMPORT.DLL 0x10018970 (Klasse 0x06): u16 Anzahl, je Stichlinie zwei Stellen auf dem Umriss
(`ReadBytePacked` Segment + float t, zweimal), danach der geschlossene Umriss (GeoCurve). MOD1
`EmbroideryPatch::SplitSides(GeoCurve, StitchLines, upper&, lower&, CurveIndex-Listen)` macht daraus die zwei Seiten.
Von Hand digitalisierte Säulen haben stattdessen zwei Seitenkurven (Bericht 03 §5). Beispiel Duck2 #11: 216 Knoten,
42 Stichlinien `0:0.00–214:1.00` (Spitze am Anfang), `7:0.00–210:1.00` …; alle Linien enden an Knoten.

## 2. Verfahren im Code – Sicherheit: hoch (Aufbau, Werte), offen (Einzelregeln)
Zwei Wege führen in dieselbe Zerlegung (EZSTITCH, Klassen aus den Exporten):
- **Auto Digitizer** (Bild → Stiche): `EasyStitchAlgorithm::DecomposedHomeProduct` 0x100500f0 (artista = Home Product)
  mit Stilen `DecompositionStyle::InitForStrokes`, `InitForBlobs`, `InitForHPAutoSelect` (Werte: 72 = 0,4 mm,
  180 = 1 mm, 2160 = 12 mm; intern 10 / 0,1 / 1,0 / 0,6), `InitForHPSatinLines`.
- **Turning-Umwandlung** (Vektorformen, TrueType): MOD1 `EmbroideryBoundGroup::Create(Kurven, TurningConversionValues)`
  → EZSTITCH `StAnglesWithOverlaps::FoundPatches` 0x1005f720 → `AutoFontDecomposition::Generate` 0x100200a0 →
  `AF_Patch_Inter` (zwei Seiten + Stichstellen) → MOD1 `EmbroideryPatch::CreatePatch`.

`Generate` (0x100200a0): `ShapeSampleManager(Kurven, 180, 9, 0,1, …, 4,0)` = Rand abtasten mit **„low precision“
180 (1 mm), „high precision“ 9 (0,05 mm), „proportional precision“ 0,1** (Getter `GetLowPrecision`,
`GetHighPrecision`, `GetProportionalPrecision`; `RefineToPrecision`, `ProportionalRefinement`,
`IncreasePrecisionNearSignificantTriangles`); Triangulation in einer Quad-Edge-Struktur (`QuadTree`,
`EdgeIsInterior`); danach mehrere Stufen mit Kosinus-Schwellen (cos Break Angle, −0,9, 2,5, 0,4, 0,98, 0,95, 0,97,
0,01 an 0x10087bb0; cos 150° an 0x1008a130; ±0,9 an 0x1002c9e0/0x10029240), die **Schnitte** setzen
(`ShapeSample::AddCut`, `AddStarShapedCuts` = sternförmig an Abzweigungen); `SignificantTriangles` prüft Ecken,
Serifen und Fehlstellen (`TriangleIsOnCorner/OnSerif/OnDefect`, `GetMaxDefectSize`, `GetMaxSerifProportion`,
`CornerMeasureAtVertex`). Teile: `DecompositionUnit_StrokeLike` (obere/untere Seite + Stichstellen,
`GetStitchLine`), `_Blobby`, `_Line`.

**TurningConversionValues** (Standard, MOD1 0x101f5bc0; Dialog „Conversion Values“ RESENU DLG): Turning Strokes an,
**Break Angle 87°** (an), Match Ends an, „T“-Abzweigungen 54 = 0,3 mm, Separate Serifs an, **Corner Detection Angle
120°**, Corner Fraction 0,6 (an), weitere Felder 1 / 90 / 9000 (Bedeutung offen).

Die Einzelregeln (welches Dreieck „bedeutend“ ist, wo genau geschnitten wird) stehen in sehr umfangreichen
Funktionen und sind **nicht** übertragen; der Kern setzt sie als eigene Festlegung (§4) und prüft am Ergebnis (§3).

## 3. Messung an den Musterdesigns – Sicherheit: hoch
`art_autosatin.py messen`: 54 EmbroideryPatch mit Stichlinien in `Samples/AutoDigitizing` (Butterfly2 36, Duck2 18;
dicke Linien der Bilder, 0,3–0,9 mm breit).
- Benachbarte Stichlinien: Abstand der Mitten Median **1,85 mm = 4,2 × Breite** (10 %: 1,4×, 90 %: 11×),
  Winkeländerung Median **15,6°** (10 %: 5,6°, 90 %: 44°) – Linien nach Krümmung, nicht nach festem Abstand.
- Der Auto Digitizer **teilt Säulen nicht an Knicken**: Der Umriss eines Entenfußes (Duck2 #11) ist **eine** Säule,
  auch durch Kehren von 150°; an der Spitze eine Stichlinie von der äußeren Spitze zur inneren Ecke. Geteilt wird nur
  an Abzweigungen der Linienzeichnung. Der Knickwinkel 87° gilt für die Turning-Umwandlung (TrueType).

## 4. Umsetzung im Kern (`stick_kern::saeulen`)
1. Rand abtasten: höchstens 1 mm, verfeinert auf 0,1 × örtliche Breite, nicht unter 0,05 mm (Werte wie §2). Örtliche
   Breite = kürzester senkrechter Abstand über eine **Quersehne** (innere Kante, deren Endpunkte auf dem Rand weiter
   als 1,5 × Sehne auseinander liegen – sonst verfeinern „Ohren“ am Außenbogen den Rand immer weiter).
2. Delaunay-Triangulation mit dem Rand als Zwangskanten (freie Bibliothek `spade`, neutrale Technik); innen/außen
   durch Flutung über das Netz (jede Randkante wechselt die Seite); hauchdünne Dreiecke aus fast geraden Randpunkten
   entfallen (ihre lange Kante wird Rand).
3. Dreiecke nach Zahl innerer Kanten: 1 = Ende, 2 = Säulenstück, 3 = Abzweigung. Ketten zwischen Enden/Abzweigungen
   sind die Säulen; Ringe (z. B. „O“) werden an der schmalsten Sehne geöffnet.
4. Kurze Zweige abschneiden (**eigene Festlegung** für Defect/Serif/Corner): kürzer als die Breite an der Abzweigung,
   oder Ende in einer Ecke mit Innenwinkel θ < 120° und kürzer als 1,2 × w / (2·sin(θ/2)) (Spitze unten am „V“).
5. Abzweigung: Sternschnitt zum Schwerpunkt des Dreiecks (wie `AddStarShapedCuts`). Formende: die Säule beginnt bei
   der ersten Sehne mit voller Breite (≥ 0,75 × Breite, mindestens 70° quer zur Mittellinie); die Kappe folgt dem Rand,
   der weiteste Teil (bis 0,2 × Breite darunter) wird der letzte Stich – Spitze, Kante oder Bogen (**eigene
   Festlegung**).
6. Stichlinien = innere Kanten, ausgewählt nach Douglas-Peucker auf der Mittellinie mit **0,3 × Breite** (eingestellt
   an §3: 4,3 × Breite Abstand, Original 4,2); Randecken < 120° erzwingen eine Linie (mittlere Sehne des Fächers).
7. Teilung am Knick nur, wenn eingestellt (`knickwinkel`; 0 = nie wie der Auto Digitizer, 87° = TrueType); gemessen
   zwischen gewählten Linien, die mindestens eine Breite entfernt liegen.
8. Seiten A/B: zwischen den Stichlinien nach relativer Bogenlänge (wie die Wendefüllung), je Randpunkt und
   mindestens alle 1 mm ein Paar; Splitter (unter 2 % der Fläche und unter 0,5 mm²) entfallen.

Verwendung: JSON `{"art": "satinflaeche", "umriss", "loecher", "unterlage", "abstand", "faktor", "kurzstiche",
"stichart", "knickwinkel"}` → Gruppe aus Satinsäulen, verbunden wie ein Buchstabe (Laufweg 1,8 mm, Lauf 0x271c der
Füllung); TrueType-Schrift `"stichart": "satin"` → Säulen mit Break Angle 87°, Schriftwerte (Auto Spacing × 0,9,
Kurzstiche). CLI `stick saeulen FORMEN.json [--svg]` zeigt die Zerlegung.

## 5. Prüfung gegen das Original – Sicherheit: hoch
`art_autosatin.py export` → `stick saeulen` → `art_autosatin.py vergleich`: dieselben 54 Umrisse durch den Kern.
| Größe | Original | Kern |
|---|---|---|
| Säulen je Form | 1 | **1 (54/54)** |
| Abstand benachbarter Stichlinien / Breite (Median) | 4,21 | 4,30 |
| Stichrichtung an den Original-Linien: Abweichung Median / 75 % / 90 % | – | 4,4° / 8,5° / 24,7° |
| Anteil unter 10° | – | 78 % |
Die größeren Abweichungen liegen in Kehren und an den Enden (Fächer anders verteilt). Bildprobe (Y, Ring, Welle,
„Sa“ Arial in Satin): Stiche quer zu jedem Strich, Abzweigungen gedeckt.

## 6. Offen
1. Einzelregeln von `SignificantTriangles` und den Schnittstufen (Kosinus-Schwellen §2) genau übertragen – nur, wenn
   die Ergebnisse an Formen abweichen, die die 54 Säulen nicht abdecken (breite Abzweigungen, Serifen der Schriften).
2. Überlappung an Abzweigungen („Create Overlaps“, „T“-Abzweigungen 0,3 mm) – derzeit stoßen die Säulen am
   Sternschnitt aneinander.
3. Wahl der Stichart nach Breite (EasyStitch: Fläche ≤ 8 mm Satin) beim SVG-Import (`saeulen::groesste_breite`).
