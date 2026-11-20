# 18 – SVG einlesen (Inkscape → Stickdesign)

**Stand 21.11.2026, ROADMAP 4b.17.** Erweiterung über das Original hinaus (Wunsch des Nutzers: in Inkscape zeichnen →
laden → sticken). Das Original liest keine SVG; es hat aber für das automatische Digitalisieren die Regel, **welche
Stichart eine Form nach ihrer Breite bekommt** (EasyStitch). Der Import verwendet diese Regel und die Werte der
EasyStitch-Stile. Das Einlesen selbst ist neutrale Technik (freie Bibliothek `usvg`).

Kern: `kern/stick-vektor/src/svg.rs` (Einlesen), `easystitch.rs` (Stichart nach Breite), `stick-kern::saeulen::dicke`,
`stick-kern::aussparen::flaechen_aus_ringen`. Aufruf: `stick svg DATEI.svg [--breite MM] [--rahmen NAME]
[--ohne-aussparen] [--ohne-closest-join] [-o DESIGN.json]`, Befehl `svg_lesen` (`SCHNITTSTELLE.md`).

## 1. Stichart nach Breite – aus dem Original (Sicherheit hoch, Zuordnung der Vorlagen mittel)
Klassifikationstabelle 0x3f54 in jedem EasyStitch-Stil der Vorlage NORMAL.AMT (`Analyse/code/10` §C):

| Form | Breite bis | Vorlage | Stil (Flags in `styles.txt`) | Ergebnis im Design |
|---|---|---|---|---|
| Linie | 0,3 mm | `<PRESET_RUN_1>` | Thin Lines: Kontur Laufstich, **2,0 mm** | `lauf`, `laenge` 2,0 |
| Linie | 2,0 mm | `<PRESET_RUN_2>` | Thick Lines: wie Thin Lines | `lauf`, `laenge` 2,0 |
| Fläche | 8,0 mm | `<PRESET_SATIN_1>` | Narrow Objects: **Satin**, Zugausgleich **0,40 mm** | `satinflaeche` (Kontur breiter 2 mm: `satinlinie`), `einstellungen.satin.zugausgleich` 0,40 |
| Fläche | 12,0 mm | `<PRESET_TATAMI_1>` | Wide Objects: **Tatami**, Abstand **0,45 mm**, Zugausgleich 0,40 mm | `tatami` mit `einstellungen.tatami` {abstand 0,45, zugausgleich 0,40} |
| Fläche | darüber | `<PRESET_TATAMI_2>` | Other Objects: wie Wide Objects | wie Tatami 1 |

- Die Vorlagen-Namen stehen in der Tabelle, die Stile tragen andere Namen; zugeordnet über die vorgewählte Stichart
  (Flags 0x0001 in der Gruppe Füllung 0x27a6 bzw. Kontur 0x27a8; Gruppe aus = 0x2000) – **mittel**.
- **Thin und Thick Lines** sind in der Vorlage gleich (Laufstich 2,0 mm). Worin sich die „kräftige Linie“ im Original
  unterscheidet (z. B. im Digitalisier-Code), ist nicht gefunden → beide als Laufstich. Offen.
- **Zugausgleich 0,40 mm** (0x273d = 72) der Flächen-Stile: bestätigt die Angabe in `Analyse/code/10` §B und den
  Handbuch-Hinweis „Use 0.4 when using automatic digitizing“. Achtung: `style_keyvalues_all.txt` enthält die Gruppe
  0x273b/0x273d nicht – Vergleiche dort zeigen den Zugausgleich nicht.
- Mindestfläche 1 mm² (0x3f57 = 32 400 in derselben Tabelle): **Annahme**, die Rolle ist nicht bestimmt.
- Werte je Objekt (4b.16) tragen die Stil-Werte; alles Übrige wie der Stoff des Designs.

## 2. Eigene Festlegungen (markiert)
| Punkt | Festlegung | Grund |
|---|---|---|
| **Breite einer Fläche** | Dicke = Durchmesser des größten Innenkreises (`saeulen::dicke`: Umkreismittelpunkte der Delaunay-Dreiecke, Abstand zum Rand) | wie EasyStitch misst, ist nicht analysiert; `groesste_breite` (örtliche Breite der Säulenstücke) unterschätzt runde Flächen (Kreis 14 mm → unter 2 mm, im Test gesehen) |
| Schmale Flächen ≤ 2 mm | Laufstich auf der Mittellinie (Mitte der Säulenpaare aus `saeulen::zerlegen`) | EasyStitch macht Objekte unter 2 mm zu Linien |
| **Überdeckung** | jede Füllfläche spart aus, was in der SVG später gezeichnet ist (exakt, ohne Überlappung); Linien sparen nichts aus; abschaltbar | so sieht die SVG aus; exakt wie „Remove Overlay Stitching“ (Bericht 14) |
| Reihenfolge | Zeichenreihenfolge, je Pfad erst Füllung, dann Kontur; Closest Join an | Auto Digitizer: „closest join“ (Handbuch PDF-S. 207) |
| Konturen > 8 mm | bleiben Satinlinien (Hinweis „besser als Fläche zeichnen“) | selten; Umriss eines Strichs als Fläche wäre aufwendig |
| Verläufe/Muster | Farbe des mittleren Farbstopps (Hinweis) | Stickgarn hat eine Farbe |
| Lage | Mitte der Zeichnung = Rahmenmitte, +y oben | wie `stick bild` |
| Kurven | auf 0,02 mm genau in Linien zerlegt | wie Kreis/Bogen im Kern |

## 3. Technik (neutral)
- `usvg` 0.50 ohne Text/Schriften: löst Gruppen, Transformationen, CSS/`style`, `use`, Grundformen (rect, circle,
  ellipse, polygon …) in Pfade auf; Maßstab 1 px = 25,4/96 mm (Inkscape mit `width="…mm"` + `viewBox` ergibt die
  echte Größe), Strichbreite × Maßstab der Transformation.
- Füllregel `nonzero`/`evenodd` über `i_overlay` (`aussparen::flaechen_aus_ringen`), Löcher werden Löcher.
- Text: Hinweis „in Inkscape in Pfade umwandeln“; Bilder: Hinweis (Bild → Umrisse ist ein eigener Befehl).

## 4. Prüfung
- Tests `stick-vektor::svg` (Stichart nach Breite, Inkscape-mm-Seite, Mittellinie, evenodd-Loch, Überdeckung,
  Breite/Hinweise) und `saeulen::dicke` (Säule, Kreis, Ring).
- Praxisprobe 21.11.2026: Blume wie aus Inkscape (Ebene, `style`, Transformationen, Bézier, Ellipsen gedreht, evenodd-
  Ring) → 12 Objekte: Blütenblätter Tatami (Other Objects) mit ausgesparten Überdeckungen, Blätter Satinfläche, Stiel
  1,5 mm und Rand 0,25 mm Laufstich, Ring mit Loch Satinfläche; 2 777 Stiche, passt in 130 × 100. Vorschau stimmig.
- Offen: Probestickerei (Liste „Vor Ort“), echte Inkscape-Dateien des Nutzers.

## 5. Offen
- Unterschied Thin/Thick Lines im Original (Digitalisier-Code EZSTITCH/MOD1).
- Bedeutung der übrigen Schwellen der Tabelle (5 mm, 0,2 mm, 1 mm, 3/1/12 mm, 0,83 mm²).
- Wie EasyStitch die Breite misst (Code `EasyStitchAlgorithm`, EZSTITCH.DLL) – dann `dicke` ggf. ersetzen.
- `stick bild` (Bild → Umrisse) nutzt die EasyStitch-Einteilung noch nicht (ROADMAP 4b.11/M2).
