# 08 – Rahmenbibliothek `RES/hooplibr.dsc`

**Dateien:** [`hoops.py`](hoops.py), Original [`../daten/hooplibr.dsc`](../daten/hooplibr.dsc), Ergebnis [`../daten/hoops_table.md`](../daten/hoops_table.md)
**Sicherheit:** hoch (Textdatei, direkt gelesen); die Zuordnung der Rahmen zu den Bernina-Produktnamen stammt aus den Bezeichnungen in der Datei

## Format (Text, CRLF)
```
MACHINE 17                      ← Maschinentyp (17 = Bernina, wie in DEFAULTS.DFL)
POSITION  AUTOCENTER            ← Standard: Rahmen folgt der Designmitte
;-----------------------------  ← Kommentar/Trenner
HOOP RECTANGLE (B,H) Name       ← neuer Rahmen; (B,H) = Stickfeld Breite × Höhe in mm
SECTION PEN (0,DOTTED[,r,g,b])  ← Stift 0 gestrichelt = Stickfeld (optional mit Farbe, z. B. rote Achsen)
SECTION PEN (1,SOLID)           ← Stift 1 durchgezogen = Kontur des physischen Rahmens
LINE (x1, y1) (x2, y2)          ← Strecke
ARC  (x1, y1) (x2, y2) (x3, y3) ← Kreisbogen durch drei Punkte
```
Koordinaten in **mm**, Ursprung = Rahmenmitte, x nach rechts, y nach oben. Die Datei enthält Tippfehler, die `hoops.py` toleriert: `-101,23` mit Dezimalkomma und `(79.6 -72.2)` ohne Komma.

## Form des Oval-Rahmens 145 × 255 (aus den gestrichelten Linien)
```
Seiten gerade:   x = ±72,5  für  y = −60 … +60
Übergang:        Linie von (±72,5, ±60) nach (±65, ±90)
Abschluss:       Bogen (−65, 90) → (0, 127,5) → (65, 90)   (oben und unten)
```
Das Stickfeld ist also kein echtes Oval, sondern ein Rechteck mit schrägen Ecken und Bogenabschluss: 145 mm breit, 255 mm hoch.

## Ergebnis
Siehe [`../daten/hoops_table.md`](../daten/hoops_table.md). „Kopf (B×H)“ = Angabe in der HOOP-Zeile; „Stickfeld X/Y“ = Ausdehnung der gestrichelten Linien. Beim MEGA-Rahmen reicht X bis +130, weil die gestrichelte Sektion zusätzlich Markierungen enthält; maßgeblich ist 150 × 400.
