# 29 – Kreuzstich

Stand 22.11.2026 (Nachtauftrag N9, ROADMAP 4b.15). Entscheidung des Nutzers (22.11.2026): **Kreuzstich als Objektart im
Programm**, kein eigenes Programm wie im Original.

Quellen:
- Handbuch Kap. 34–37 (gedruckte S. 307–342, Menüs S. 353);
- Programmteil „BERNINA artista Cross Stitch“ des Originals aus `setup.z`: `BLOT.EXE` (C++Builder) und `XSLang.DLL`.
  Ihre Formulare (DFM-Ressourcen „TPF0“) sind mit dem neuen Werkzeug `Arbeitsdaten/werkzeuge/dfm_lesen.py` gelesen
  (`python3 -I dfm_lesen.py DATEI [FORM…]`).
- Beispieldesigns `Samples 3/*.ARX`: gepackt mit TCompress (LZH; Fehler „EInvalidKey“, vermutlich mit Schlüssel) – **nicht
  lesbar**. Die Stichfolge des Originals ließ sich darum nicht messen.

Kern:
- `stick-kern/src/sticharten/kreuzstich.rs` (Zellen, Stichfolge, Kontur);
- Objektart `kreuzstich` (`format.rs` `KreuzstichJson`);
- Befehle `kreuzstich_vorschau`, `kreuzstich_bild` (`stick-api/src/kreuzstich.rs`).

## 1. Was das Original kann — Sicherheit: hoch (Handbuch, Formulare)
- **Raster:** Kreuze liegen auf einem Raster aus Quadraten. Die Größe folgt der Stoffdichte („Fabric Count“, Menü
  Settings, S. 317/318): „Number of stitches per inch“ oder „Number of millimeters per number of stitches“. Beispiele im
  Handbuch: 10 und 15 Kreuze je Zoll.
  - Die Vorgabe im Formular ist ein Platzhalter (10,0). Den Standardwert der ausgelieferten Einstellungen haben wir
  nicht gefunden.
- **Kreuzarten** (Liste `FillStyle` in `BLOT.EXE`, Reihenfolge so übernommen):
  - Full Cross;
  - 3/4 Cross: „quarter stitch followed by a half stitch“, langer Schenkel „/“ oder „\“ (S. 308);
  - 1/2 Cross;
  - 1/4 Cross;
  - Mini Cross;
  - Upright Cross;
  - Double Cross;
  - Elongated Vertical Cross;
  - Elongated Horizontal Cross;
  - French Knot.
- **Teilkreuze:** „Select the quadrants to be filled“ (Viertel wählen), Halbkreuz „/“ oder „\“.
- **Umriss** (Liste `OutlineStyle`): „Single Line“ oder „Current Cross Type“. Füllen und Umriss lassen sich je an- und
  ausschalten.
- **Fäden** (Liste `Threads`, S. 318): 2, 4, 6. Wörtlich: „The value 2 corresponds to a single stitch, 4 is double, while
  6 is triple.“ Für 15 Kreuze je Zoll empfiehlt das Handbuch 2 Fäden.
- **Bild → Kreuzstich** (Kap. 37, S. 338; Formular `TAutoStitchImageForm`):
  - höchstens so viele Farben („Restricted“ 2 … 255);
  - „Map Image To: Current Palette / Specific Thread Chart“;
  - „Omit Background“;
  - Vorschau der Kreuze.
- Weitere Werkzeuge des Originals: Magic Wand auf dem Bild, Buchstaben, Ränder (Kap. 36/37).
  - Im Programm gibt es den Zauberstab schon für Flächen (Bericht 25).
  - Die Rückstich-Linien des Originals gibt es als Objektart `rueckstich` (Bericht 24).
  - Ränder entstehen über die Kontur.

## 2. Umsetzung — Objektart `kreuzstich`
Felder (alle mit Standard):
- `umriss`/`loecher`: Fläche, wahlweise;
- `zellen`: einzelne Zellen [Spalte, Reihe];
- `groesse`: Kreuzgröße in mm, 1 … 8, Standard **2,54 = 10 Kreuze je Zoll** (Handbuch-Beispiel; eigene Festlegung als
  Standard);
- `kreuz`: Kreuzart wie `FillStyle`;
- `ecke`: Viertel bzw. Hälfte, für Teil- und längliche Kreuze;
- `richtung`: Halbkreuz „/“ oder „\“;
- `faeden`: 2/4/6 = ein-, zwei-, dreimal gestickt;
- `fuellen` (an);
- `kontur`: `keine`, `linie` (Single Line) oder `kreuz` (Current Cross Type).

**Raster (eigene Festlegung):**
- Das Raster liegt fest am Design: Zelle [i, j] reicht von i·groesse bis (i+1)·groesse.
- Eine Zelle gehört zur Fläche, wenn ihre Mitte in der Fläche liegt (Gerade-Ungerade-Regel mit Löchern).
- Verschieben, Drehen und Skalieren bewegen die Fläche. Einzelne Zellen werden über ihre Mitte abgebildet und auf die
  nächste Zelle gerundet. Die Kreuze bleiben auf dem Raster, wie auf Stramin.

**Stichfolge (eigene Festlegung; das Verfahren `SQMovementToStitches` in `BLOT.EXE` ist nicht gelesen):**
- **Volle Kreuze:**
  - Reihe für Reihe von unten nach oben. Je zusammenhängendem Lauf hin im Zickzack („/“ und „\“ abwechselnd), zurück die
    jeweils anderen Schenkel.
  - Jede Diagonale liegt genau einmal. Einzige Verbindung ist ein Kantenstich am rechten Ende.
  - Der Lauf endet oben links, also dort, wo die nächste Reihe beginnt.
  - Bei 4/6 Fäden wird der Lauf über die linke Kante wiederholt.
- **Halbkreuze:**
  - Halbkreuze liegen auf Diagonalen hintereinander. Je Diagonale entsteht eine Kette von Knoten zu Knoten.
  - Nachbarketten laufen abwechselnd hinauf und hinunter, verbunden am Rand der Fläche.
- **Aufrechte und doppelte Kreuze:** Reihen in Schlangenlinie.
  - Aufrecht: Mitte links → je Zelle Mitte, oben, unten, Mitte → Mitte rechts. Der senkrechte Strich liegt doppelt,
    weil ein Laufstich aus der Sackgasse zurück muss.
  - Doppelt: erst die vollen Kreuze der Reihe, dann das aufrechte Kreuz.
- **Teilkreuze** (¾, ¼, Mini, länglich):
  - Zelle für Zelle in Schlangenlinie. Anfang und Ende jedes Musters liegen auf dem Zellrand.
  - ¼ und Mini: der kurze Schenkel liegt doppelt (hin und zurück).
- **Übergänge:**
  - nur auf Rasterlinien, die an gestickte Kreuze grenzen: kürzester Weg, höchstens 16 Kreuze weit, wenig Umweg;
  - sonst ein neuer Zug (Sprung mit Vernähen wie jedes Objekt);
  - so liegen die Verbindungen in denselben Löchern wie die Kreuze.
- **Französische Knoten:** je Knoten ein Zug aus sechs Stichen à 0,5 mm um die Zellmitte (Mindeststich 0,3 mm), bei
  4/6 Fäden wiederholt.
- **Kontur:**
  - `linie`: Laufstich auf den Rasterkanten um die gefüllten Zellen, je Rand ein geschlossener Zug (außen gegen den
    Uhrzeiger, Löcher mit).
  - `kreuz`: zusätzlich Kreuze in allen Zellen, durch die der Umriss läuft (abgetastet alle groesse/8).
- Längster Stich = Diagonale (bei 2,54 mm: 3,59 mm). Die Bernina-Regeln (0,3 … 12,7 mm, Auto Jump) gelten wie immer.

Aussparen: Ein Kreuzstich-Objekt bedeckt seine Zellen (Rand auf dem Raster). Andere Flächen können es aussparen, die
Umrandung (eigene Erweiterung) folgt den Zellen.

## 3. Bild → Kreuzstich (`kreuzstich_vorschau`, `kreuzstich_bild`)
Wie „Auto Stitch Image“ des Originals:
- Das Bild wird auf das Raster verkleinert (Breite/Kreuzgröße Spalten, höchstens 500 × 500): **ein Bildpunkt = ein
  Kreuz**.
- Die Farben werden reduziert (2 … 64; das Original erlaubt bis 255) und auf Wunsch auf die Garnkarte gezogen
  („Specific Thread Chart“).
- Weißer Hintergrund ist zum Weglassen vorgeschlagen („Omit Background“); jede Farbe lässt sich ab- und zuschalten.
- Ergebnis: je Farbe ein Objekt `kreuzstich` mit seinen `zellen`, Mitte des Bildes auf der Rahmenmitte, eingefügt als
  Gruppe.
- Kein Glätten, keine Mindestfläche: jedes Kreuz zählt.

Geprüft: 400 × 392 Kreuze (Raster 1 mm, 11 Farben) in unter 0,2 s umgewandelt und berechnet.

## 4. Oberfläche
- Stichart „Kreuzstich“ in der Liste „Füllung“ (neue Formen, Stichart wechseln; Modus E).
- Inspektor (Experte), oben: Kreuz, Ecke/Hälfte (nur bei Teilkreuzen), Richtung (nur Halbkreuz), Fäden, Kreuzgröße,
  Kontur. „Füllen“ steht unter den Feinheiten.
- Datei → **„Kreuzstich aus Bild …“** und Einfügen → „Kreuzstich aus Bild …“. Das ist der vierte Weg im Dialog „Bild
  umwandeln“: Breite, Farben, Kreuzgröße, Kreuz voll/halb, Fäden, Garnfarben, Farbliste zum Weglassen. Vorschau in
  Kreuzen, ohne Glätten (Modus E).

## 5. Offen
- Stichfolge des Originals messen. Dafür ARX entpacken (TCompress-Schlüssel suchen) oder ein Kreuzstich-Design aus
  dem Original vor Ort sticken bzw. senden und zurücklesen.
- Fabric-Count-Standard des Originals (Einstellungsdatei).
- Zellen mit der Maus setzen und löschen (Malwerkzeug wie im Original; bisher über `zellen` im JSON oder „Bild →
  Kreuzstich“).
- Buchstaben in Kreuzstich (Alphabete des Kreuzstich-Programms) und fertige Ränder (Kap. 36/37).
