# 07 – Laufstich an Ecken, Zickzack-Unterlage, Reihenfolge der Unterlage (19.11.2026)

Drei weitere „eigene Festlegungen“ des Prototyps (`stickprogramm/README.md`) sind an den Original-Stichen geprüft.
Werkzeug: `Arbeitsdaten/werkzeuge/art_messen.py`. Zuordnung Objekt ↔ Stiche wie in [04](04_messungen_satin.md) §1.

## 1. Laufstich: Einstich an jedem Eckpunkt — Sicherheit: hoch
Formen bestehen im Original aus Knoten, die beim Digitalisieren als **Eckpunkt** oder **Kurvenpunkt** gesetzt werden:
Kennbyte Bit 2 in `SerializeKnots`, [03](03_art_objekte.md) §5.

**Messung:**
- Ausgewertet werden alle Linienobjekte (EmbroideryCurve) mit genau einem Lauf: 76 245 innere Knoten.
- Geprüft wird, ob ein Einstich auf dem Knoten liegt (< 0,05 mm).
- Aufgeteilt nach Knotenart und Knickwinkel.
- Rohergebnis: `Arbeitsdaten/messungen/ecken_musterdesigns.txt` (`art_messen.py ecken --alle`).

| Knoten | Knick 0–30° | 30–60° | 60–90° | 90–120° | 120–150° | 150–180° |
|---|---|---|---|---|---|---|
| **Eckpunkt** | **100 %** (32 819) | 99 % | 100 % | 99 % | 99 % | 100 % |
| Kurvenpunkt | 6 % (9 236) | – | – | – | – | – |

**Folgerung:**
- Das Original sticht an **jedem Eckpunkt** ein, unabhängig vom Winkel, auch bei fast geraden Knicken.
- An Kurvenpunkten liegt ein Einstich nur zufällig.
- Bit 2 des Knoten-Kennbytes ist damit als „Eckpunkt“ belegt.

**Kern (Referenz-Stufe 8):** `laufstich_mit_ecken`.
- **Eckpunkte:** Punktlisten und Rechteckecken aus dem Design-JSON, dazu die Enden von Kreis, Bogen und Bézier.
- **Kurvenpunkte:** die übrigen Punkte von Kreis, Bogen und Bézier.
- Die Nahtstelle beim Verbinden mehrerer Teile zählt als Ecke (eigene Festlegung).
- Die frühere Regel „Einstich ab 30° Knick“ gilt nur noch für berechnete Linien (Unterlagen). Wie das Original dort
  vorgeht, ist nicht gemessen.

## 2. Zickzack-Unterlage: Länge = Weg zwischen Einstichen derselben Seite — Sicherheit: mittel-hoch
- Im Design hat die Zickzack-Unterlage eine Stichart 0x17 (Kennung 0x2724) mit long 540 (3,0 mm) und Basis 0x15
  (Abstand 3,0 bzw. 5,0 mm).
- Gemessen wird der Weg zwischen Einstichen derselben Seite:
  - bei 3,0 mm: Median **2,90 mm** (156 Stiche);
  - bei 5,0 mm: 4,44 mm (1 029 Stiche, kurze, gebogene Säulen).
- Das entspricht der Deutung des Kerns (Einstich alle Länge/2), die damit von „niedrig“ auf mittel-hoch steigt.

## 3. Reihenfolge: Unterlage endet am Säulenanfang — Sicherheit: hoch
- Bei allen 1 726 Satinobjekten mit Unterlage liegt der letzte Unterlagenstich **genau** auf dem ersten Satinstich
  (Median und 90-%-Wert 0,0 mm).
- Mittellauf: Anlauf, hin, zurück. Edge Walk: einmal um den Rand ([04](04_messungen_satin.md) §2).
- Der Kern macht es ebenso, die Festlegung ist damit bestätigt.
