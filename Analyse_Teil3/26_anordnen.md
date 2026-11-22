# 26 – Anordnen: Ausrichten, Mirror-Merge, Kranz und Kaleidoskop

Stand 22.11.2026 (Nachtauftrag N6, ROADMAP 4b.12). Handbuch Kap. 15 „Arranging and Transforming Objects“ (gedruckte
S. 133–142).

Quellen:
- DESVIEW.DLL (aus `setup.z`, Disassembly der Exporte `MirrorMergeView*`);
- BERESENU/RESENU (Strings, Menü).

Kern: `stick-kern/src/anordnen.rs`, `Design::bezugspunkte`/`flaechen_eigen` (`design.rs`). Befehle:
`stick-api/src/anordnen.rs`.

## 1. Ausrichten — Sicherheit: hoch (Handbuch, Strings)
Die Arrange-Leiste hat sieben Werkzeuge (BERESENU STR 33233–33239):
- Align Left, Right, Top, Bottom;
- Align Centers Vertical (Mitten senkrecht übereinander), Align Centers Horizontal (Mitten waagrecht
  nebeneinander);
- Align Centers.

Handbuch S. 135:
- „Select the objects you want to align. Select the object you want to align with.“
- „Objects are aligned with the last object selected.“

**Kern:**
- Bezug ist die Einheit des zuletzt gewählten Objekts. Eine Einheit ist ein Objekt ohne Gruppe oder eine ganze Gruppe.
- Rechtecke = Begrenzung der Formen (`design_formen`) in der Anzeige.
- Verschoben wird über `transform.verschieben` des Objekts bzw. der Gruppe, umgerechnet in die Koordinaten vor der
  Transformation des ganzen Designs.

## 2. Mirror-Merge — Sicherheit: hoch (Code)
`MirrorMergeView{Horizontal,Vertical,Cross}::GetReflectionTransforms` (DESVIEW 0x10044100 / 0x100445b0 / 0x10044b90)
bauen je eine `AffineTransform` und rufen `Mirror(p, 0, x, y)` am Ankerpunkt p auf:
- **Horizontal** (x = 1, y = 0): Spiegeln an der senkrechten Linie durch p → eine Kopie links/rechts.
- **Vertical** (x = 0, y = 1): an der waagrechten Linie → eine Kopie oben/unten.
- **Horizontal & Vertical** (Cross): drei Abbildungen (1, 0), (0, 1), (1, 1) → drei Kopien (die dritte über Kreuz =
  180°).

Handbuch S. 141: „Move the reference line to the position you want to add the mirrored object and click to mark the
anchor point.“

## 3. Kranz (Wreath) und Kaleidoskop — Sicherheit: hoch (Code)
`MirrorMergeViewWreath::GetReflectionTransforms` (0x100454b0), Zahl der Punkte n an +0xc4, Kaleidoskop an +0xc8:
- für i = 1 … n−1: `Rotate(Mittelpunkt, −i · 2π/n)` (Konstante 0x1009e838 = 6,283185…). Die Kopien laufen also **im
  Uhrzeigersinn** (+y oben).
- Kaleidoskop und i ungerade: zusätzlich `Mirror(Mittelpunkt, θ, 0, 1)`. θ ist die Richtung der Bezugslinie vom
  Mittelpunkt zum Objekt; gespiegelt wird an dieser Linie, danach gedreht.
- Handbuch S. 141–142:
  - „Because the object is mirrored, Kaleidoscope only works with an even number.“
  - „Enter an even number to activate the Kaleidoscope icon.“

Die Zahl der Punkte ist ein Eingabefeld der Leiste („Wreath Points“, STR 33558). Vorgabe und Grenzen stehen nicht im
gelesenen Code. Im Programm gilt **6** (Handbuch-Beispiel „six-point wreath“), Bereich 2–36 (eigene Festlegung).

## 4. Zusammenführen — Sicherheit: mittel
RESENU STR 4159: „Do you want to merge the overlapped objects?“. Handbuch S. 141/142: „If the mirrored copy overlaps
the original object, you are asked if you want to merge the objects.“ Wie das Original zusammenführt, ist nicht
gelesen. Die aufrufende Stelle liegt in keiner der entpackten DLLs (ARTISTA.EXE, EMBENG, LIBINTF, IMPORT, MOD1
importieren `MirrorMergeView` nicht namentlich).

**Kern (eigene Festlegung):**
- Gilt für Step-, Muster-, Stippling- und Satinflächen ohne Gruppe.
- Kopien, deren Fläche sich mit dem Original oder untereinander um mehr als 0,5 mm² überlappt, werden mit ihm zu
  **einem** Objekt vereinigt (Polygon-Vereinigung, `i_overlay`). Die Form steht danach als Punkte im Objekt;
  Transformation und Winkellinien entfallen, der Stichwinkel des Originals bleibt.
- Die Oberfläche fragt wie das Original erst, wenn der Kern eine Überlappung meldet (`ueberlappt`).

## 5. Umsetzung im Kern
- Kopien werden nicht „eingebrannt“. Sie bekommen die zusammengesetzte Transformation, wie im Original bleibt die Form
  und die Stiche werden neu berechnet.
  - Neue Transformation = Abbildung ∘ alte Transformation um denselben Bezugspunkt.
  - Die lineare Abbildung wird in Drehen · Schrägstellen · Skalieren · Spiegeln zerlegt (QR-Zerlegung, von zwei
    gleichwertigen Formen die mit dem kleineren Drehwinkel).
  - Bezugspunkt eines Objekts = Mitte seiner Teile vor der eigenen Transformation. Bezugspunkt einer Gruppe = Mitte nach
    den Objekt-Transformationen (wie `transformieren`).
- Gruppen werden als Ganzes kopiert (neuer Gruppenschlüssel, Name „… (2)“). Ihre Transformation wird
  zusammengesetzt.
- Anker und Mittelpunkt kommen in Anzeige-Koordinaten. Hat das ganze Design eine Transformation D, rechnet der Kern
  mit D⁻¹ ∘ A ∘ D. Danach gleicht er über `verschieben` des Designs aus, dass sich durch neue Objekte die Mitte des
  Designs (und damit D) verschiebt.
- Einstich/Ausstich (Anzeige-Koordinaten) werden mit abgebildet; Objektnamen werden eindeutig („… (2)“).
- **Reihenfolge (eigene Festlegung):** Die Kopien kommen direkt hinter die Auswahl, Kopie für Kopie (beim Kranz im
  Uhrzeigersinn). Gemessen ist das am Original nicht.

## 6. Oberfläche
- Menü Anordnen (Experte):
  - sieben Ausrichten-Knöpfe (ab zwei Einheiten);
  - „Spiegeln und kopieren (Mirror-Merge) …“;
  - „Kranz / Kaleidoskop …“.
- Der Dialog zeigt die Formen des Ergebnisses: Original kräftig, Kopien hell, Stickfeld gestrichelt.
  - Ein Klick in die Vorschau setzt Ankerpunkt bzw. Mittelpunkt, statt der Bezugslinie des Originals (eigene
    Festlegung).
  - Vorschlag: Spiegellinie an der rechten bzw. unteren Kante; Kranzmitte unter der Auswahl, so weit weg, dass sich
    die Kopien nicht berühren.
- „Vor/Nach Objekt Nr. …“ fragt jetzt in einem eigenen Dialog statt mit der Browser-Abfrage.

## 7. Offen
- Original-Verfahren des Zusammenführens und die Reihenfolge der Kopien (Gerätetest oder weitere DLL).
- Vorgabe und Grenzen von „Wreath Points“.
- Ankerpunkt direkt auf der Leinwand ziehen (wie im Original mit der Maus).
