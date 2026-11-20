# 14 – Aussparen (Knockout) im Original (20.11.2026)

**Quellen:** Handbuch PDF-S. 208–210 („Cutting holes and removing overlays“, „Remove Overlay Stitching“, „Lettering
Remove Overlay“, „Include Overlay Stitching“), MOD1-Strings (Menütexte 0x2032d8 ff., `ComplexFillGroup::ContainHoles`),
Messung an den 287 Musterdesigns mit `Arbeitsdaten/werkzeuge/art_aussparen.py`
(Ergebnis `Arbeitsdaten/messungen/aussparen_musterdesigns.txt`).

## 1. Was das Original anbietet — Sicherheit: hoch (Handbuch)
- **Remove Overlay Stitching** (Menü Arrange): erst das Objekt wählen, das das Loch bildet (oben), dann mit Strg das
  Objekt, aus dem ausgespart wird. Das untere Objekt **behält seine Form**, die überdeckte Lage entfällt.
  **Include Overlay Stitching** stellt sie wieder her (die Aussparung wird also am Objekt gemerkt, nicht die Form
  geändert).
- **Lettering Remove Overlay**: Schrift spart ihre Buchstaben aus einer Fläche aus.
- **Add Holes**: Löcher von Hand digitalisieren.
- Nur für Flächen aus „Closed Curve“/„Rectangle“ mit **einem** Stichwinkel (nicht Block-Digitalisieren/Stitch Angle).
- Flächen mit Löchern sind im ART-Format **ComplexFillGroup** (Klasse 0x07, `Analyse_Teil3/03`).

## 2. Messung an den Musterdesigns — Sicherheit: mittel
360 Löcher in Flächenobjekten, 99 davon entsprechen dem Umriss eines anderen Objekts (mittlerer Abstand < 1 mm):
- **7 Löcher liegen exakt** auf dem Umriss des anderen Objekts (≤ 0,05 mm; Butterfly2, Duck2, Birdhouse – Center,
  Teddy Balloon) – so sieht ein ausgespartes Loch aus (Loch = Fläche des oberen Objekts, ohne Zugabe). In Duck2 und
  Birdhouse liegt das schneidende Objekt **darüber** (später gestickt, Satinsäule/geschlossene Linie).
- 92 von Hand digitalisierte Löcher: Abstand mit Vorzeichen Median **−0,15 mm** (Quartile −0,31 / −0,01 mm), 76 %
  kleiner als das Objekt → die Fläche läuft etwas unter das Nachbarobjekt (Überlappung gegen Lücken).
- In 70 von 99 Fällen liegt das Objekt mit dem Loch **nach** dem anderen (Hintergrund um ein schon gesticktes Detail).

## 3. Umsetzung im Kern
`stick-kern::aussparen`: Fläche eines Objekts (Satinsäule = Vierecke zwischen den Stichpaaren, Satinlinie, Tatami/
Muster mit Löchern, geschlossener Lauf, Gruppen/Buchstaben), Differenz mit `i_overlay` (Nicht-Null-Regel). Am
Tatami- und Musterobjekt: `"aussparen": ["Name", …]` (Objekte nach Namen, auch Text/TTF-Text/Applikation),
`"ueberlappung"` in mm (Standard **0 = exakt wie die Funktion des Originals**; die Musterdesigns zeigen bei Hand-
Löchern etwa 0,15 mm). Zerfällt eine Fläche, folgen die Teile als eigene Objekte gleicher Farbe (größter zuerst;
**eigene Festlegung**), Teile unter 0,5 mm² entfallen (**eigene Festlegung**).

## 4. Offen
Wo das Original die Aussparung im ART-Objekt speichert (für „Include Overlay Stitching“), Verhalten bei mehreren
Stichwinkeln, Reihenfolge der Teile im Original.
