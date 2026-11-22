# 25 – Automatisches Digitalisieren: Zauberstab, PhotoSnap, kürzeste Wege

Stand 22.11.2026 (Nachtauftrag N5, ROADMAP 4b.11). Handbuch Kap. 13 „Automatic Digitizing“ (gedruckte S. 113–120).
Quellen:
- MOD1.DLL (Disassembly über die Exporte von `PhotoStitchObject`, `PhotoValues`, `PhotoFill*`);
- NORMAL.AMT (`Analyse/daten/entschluesselt/software_defaults_flat.txt`, `styles.txt`);
- BERESENU (Dialoge DLG 1410–1412).

Kern:
- `stick-vektor` (`bereich_bei`, `photo_raster`, `ordnen`);
- `stick-kern/src/photosnap.rs`;
- Befehle `stick-api/src/automatik.rs`.

## 1. Kürzeste Wege beim Bild umwandeln — Sicherheit: hoch (Verhalten), Verfahren eigene Festlegung
Handbuch S. 116: Auto Digitizer „determines the stitching sequence based on closest join“.

Messung am Kern: Bild mit 36 Punkten in drei Farben, 100 mm breit.
- Closest Join an und aus ergeben **denselben** Sprungweg. Flächen und Gruppen (Satinflächen) beginnen im Kern schon
  immer am Umrisspunkt nächst dem vorigen Ende (`ein_ausstich::start_hinweis`, Bericht 16). Closest Join wirkt nur bei
  Linien, Säulen und Ringen.
- Was beim Bild zählt, ist die **Reihenfolge** der Flächen einer Farbe. Bisher: nächster Nachbar über die Mittelpunkte.
  Neu: dazu **2-opt** (ein Stück des Wegs umdrehen, solange er kürzer wird).
  - Satin/Auto: Sprungweg 579 → 564 mm.
  - Step: Sprungweg 722 → 715 mm.

Umsetzung:
- `bild_umwandeln` und `zauberstab` setzen `kuerzeste_verbindung` (für Zauberstab-Umrisse wirksam).
- `design_einfuegen` überträgt Closest Join des eingefügten Designs als Feld **`kuerzeste`** auf jedes eingefügte
  Objekt, wenn das Ziel es nicht hat. Vorher ging der Schalter beim Einfügen von SVG-Zeichnungen verloren.

## 2. Zauberstab (Magic Wand) — Sicherheit: hoch (Verhalten laut Handbuch), Verfahren eigene Festlegung
Handbuch S. 114–115:
- Vorher wird die Stichart gewählt: Füllstich → Fläche, Konturstich → Umriss.
- Ein Klick in eine Fläche des vorbereiteten Bildes erzeugt das Objekt: „The shape is filled with fill stitches“ bzw.
  „The object is traced with outline stitches“.
- Für Fläche und Umriss braucht es zwei Objekte („you need to create two objects“).

Kern `bereich_bei`:
- die zusammenhängende Fläche gleicher Farbe im vorbereiteten Bild (Farbreduktion wie `bild_umwandeln`), Nachbarn über
  die Kanten (4er-Nachbarschaft);
- Umrisse mit `find_contours`; Inseln anderer Farben werden Löcher;
- Flächen < 1 mm² entfallen (EasyStitch).

Befehl `zauberstab`:
- Klicks als Lage 0 … 1 im Vorschaubild, mit Stichart (Step, Satin, Auto; Lauf, Dreifach, Satinlinie).
- Flächen bekommen die Werte des Auto Digitizers (Tatami-Abstand 0,45, Zugausgleich 0,40, Unterlage).
- Umrisse: je Ring (Umriss und Löcher) eine geschlossene Linie.

**Abweichung Oberfläche (eigene Festlegung):** Im Original klickt man auf das Bild im Design. Hier gibt es (noch) kein
Bild hinter dem Design. Deshalb liegt der Zauberstab im Dialog „Bild umwandeln“; das Ergebnis wird ein Element
(Gruppe).

## 3. PhotoSnap — Sicherheit: siehe je Punkt
Handbuch S. 119–120:
- „PhotoSnap designs consist of rows of Satin stitches with varied spacing settings. The effect resembles the output
  of a line printer.“
- Einstellungen: Resolution (Coarse/Medium/Fine), Background (Light: „applies the Max Width value to the lightest part
  of the image“; Dark), Fill Angle, Fill-Stitch-Reiter.

### 3.1 Gelesen in MOD1 — hoch
**`PhotoValues`** (Konstruktor 0x10122d50, `GetData` 0x101230d0, `SetData` 0x10122f20), DataItem **0x3ee4**:

| Feld | Kennung | Bedeutung (aus der Verwendung) | Vorgabe |
|---|---|---|---|
| +0x0c (Byte) | 0x3ee9 | einfarbig (1) / farbig (0) – `GenerateComponents` prüft +0x7c: 0 → `Split_CMYK`, sonst `ConvertToGrayScale` | 1 |
| +0x0d (Byte) | 0x3ee8 | Hintergrund (an jede Lage weitergegeben) | 1 (NORMAL.AMT) |
| +0x10 / +0x18 (double) | 0x3ee5 / 0x3ee6 | an jede Lage weitergegeben (Zeiger); NORMAL.AMT 360 (2,0 mm) / 0 | Deutung: Auflösung, **niedrig** |
| +0x20 (double) | 0x3eeb | **Winkel** der Reihen (Bogenmaß; `AffineTransform::Rotate`, plus Zusatzwinkel je Lage) | 0 (Konstruktor; nicht in NORMAL.AMT) |
| +0x28 | 0x3eed | Modus der Lagen (siehe unten); NORMAL.AMT 1,22173 – als Ganzzahl gelesen vermutlich 1 | mittel |
| +0x2c | 0x3eea | Stichart | **0x2828** |
| +0x30 (Liste) | 0x3eec | vier Lagenfarben (Laufzeit-Konstanten, nicht gelesen) | – |

**Lagen** (`GenerateComponents` 0x10120aa0, Tabellen 0x101208f0 / 0x10120a40):
- Bei Farbe: Zusatzwinkel je Lage. Für Stichart 0x2828 mit Modus 1 (0x10120890): **0, π/4, π/2, 3π/4** = 0°, 45°, 90°,
  135° (Konstanten 0x1011f4c0 ff.).
- Modus 2 (0x101209e0): gleicher Winkel, Lagen quer versetzt um LongPoint (0, **72 / 144 / 216**) = 0 / 0,4 / 0,8 /
  1,2 mm, gedreht mit dem Winkel.
- Stichart 0x282e, Modus 3: Winkel 0/45/90/135° über 0x10120870.

**Stichart 0x2828** (NORMAL.AMT, Stilteil mit 8 Feldern):
- enthält Motivwerte 0x3ab4 (Größe 720);
- 0x2711 = 2286 (12,7 mm), 0x2712 = 1260 (7,0 mm);
- **0x282a = 144 (0,8 mm)**, **0x282f = 1800 (10 mm)**.
Damit ist der in Bericht 24 §8 offene Stilteil 0x2828 als **PhotoSnap-Stichart** erkannt.

Dialog: BERESENU DLG 1412 (Background Light/Dark, Fill angle, Resolution Coarse/Medium/Fine) und eine zweite Fassung
mit „Character: Color / Monochrome“ und „Layer Order“. Fill-Stitch-Reiter: DLG 1410 („Use PhotoStar for BlackLayer“),
DLG 1411 („Stitch Spacing“).

### 3.2 Umsetzung im Kern (`photosnap.rs`) — Verfahren eigene Festlegung
- Der Befehl `photosnap` verkleinert das Bild auf **0,5 mm je Punkt** und speichert die **Deckung** 0–255 je Punkt als
  Base64 im Objekt (`raster`). So ist das Design ohne die Bilddatei vollständig.
  - Einfarbig: 255 − Helligkeit (0,299 R + 0,587 G + 0,114 B).
  - Farbig: C, M, Y, K nach der üblichen Formel (K = 1 − max(R, G, B)).
  - Durchsichtig: kein Faden.
- Reihen der Breite **grob 3 / mittel 2 / fein 1 mm** quer zum Winkel über das ganze Bild. Mittel = 0x3ee5; grob/fein
  sind eigene Festlegung.
- In der Reihe Einstiche abwechselnd oben/unten.
  - Abstand (gleiche Seite, Bericht 04) = 0,8 mm / Deckung, höchstens 10 mm (0x282a/0x282f, Zuordnung niedrig).
  - Deckung = Mittel aus drei Proben quer zur Reihe.
  - Auch die hellsten Stellen bekommen Stiche im größten Abstand („applies the Max Width value to the lightest part“).
- Reihen abwechselnd hin und zurück.
- Je Lage ein Zug aus festen Einstichen (Laufstich mit allen Punkten als Ecken, höchstens 12,7 mm).
- Dunkler Stoff kehrt die Deckung um.
- Lagenfarben ohne Garnkarte: Druckfarben Cyan #00aeef, Magenta #ec008c, Gelb #fff200, Schwarz (eigene Festlegung).
  Mit Karte: nächstes Garn. Reihenfolge C, M, Y, K.
- Farblagen „gekreuzt“ (Standard, Modus 1) oder „versetzt“ (Modus 2).

Gemessen am Kern: Bild 90 × 58 mm.
- Einfarbig: 3 310 Stiche.
- Farbig: 6 670 Stiche, 4 Farben, 5 Sprünge.

## 4. Eigene Festlegungen (Übersicht)
- Zauberstab im Dialog statt auf einem Bild im Design.
- Reihenfolge 2-opt.
- PhotoSnap:
  - Raster 0,5 mm;
  - Abstandsformel;
  - grob/fein;
  - Druckfarben;
  - Reihenfolge der Lagen.

## 5. Offen
- **Auto Digitizer „Details“** (S. 118): dünne Linien als Satin, Double Run oder Satin Line über die Mittellinie. Im
  Kern werden dünne Flächen mit „Auto“ zu Satinflächen; Mittellinie (Skelett) fehlt.
- Bild als Vorlage hinter dem Design (Zauberstab direkt auf der Leinwand) – braucht ein Bildobjekt im Design (M2
  „maßstabsfrei“ hängt daran).
- PhotoSnap:
  - Bedeutung von 0x3ee5/0x3ee6 genau;
  - Werte der Auflösungsstufen;
  - Lagenfarben (Laufzeit-Konstanten 0x102a6f84 ff.);
  - „Use PhotoStar for BlackLayer“;
  - Stichart 0x282d/0x282e (PhotoFillContour, Motiv-Füllungen `PhotoFillDynamic/Predefined`).
- Original-Verfahren der Reihenfolge beim Auto Digitizer (nicht gelesen).
