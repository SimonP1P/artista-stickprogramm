# 24 – Craft Stitches: Rückstich, Stielstich, Stippling, Blackwork, Candlewicking, Lacework

Stand 22.11.2026 (Nachtauftrag N3, ROADMAP 4b.9). Handbuch Kap. 26 „Craft Stitches“ (gedruckte S. 235–243) und
S. 131–132 („Blackwork Run“). Quellen: NORMAL.AMT (`Analyse/daten/entschluesselt/styles.txt`), MOD1.DLL (Disassembly),
RESENU/BERESENU (Dialoge, Strings). Kern: `stick-kern/src/sticharten/handstich.rs`, `sticharten/stippling.rs`,
`blackwork.rs`; Befehle `stick-api/src/craft.rs`.

## 1. Werkzeugleiste und Stile — Sicherheit: hoch (gelesen)
Die Craft-Stitch-Leiste hat acht Werkzeuge. Die Zuordnung zu den Stilen von NORMAL.AMT folgt aus den Strings 4211–4215
(Werkzeugnamen) und 4221–4225 (Stilnamen) von RESENU:

| Werkzeug | Stil / Stichart | eigene Werte des Stils (Flag 0) |
|---|---|---|
| Blackwork Fill | `<PRESET_MOTIF_FILL_1>`, Motivfüllung 0x27ee | `_BW004a` / `B0001a`, Größe 0x3ab8 = 1440, Raster 0x27b6/0x27b9 = 1440/1440 (8,0 mm) |
| Candlewicking Fill | `<PRESET_MOTIF_FILL_2>`, 0x27ee | `_CW004a` / `Knot-3mm`, 540, Raster 540/540 (3,0 mm) |
| Lacework | `<PRESET_MOTIF_FILL_3>`, 0x27ee | `_LW004a` / `L0001a`, 720, Raster 720/720 (4,0 mm) |
| Blackwork Border | `<PRESET_MOTIF_RUN_1>`, Motivlinie 0x2798 | `_BW004a` / `B0001a`, 0x27b6/0x27b9 = 1440, Abstand 0x27df = 1440 (8,0 mm) |
| Candlewicking Border | `<PRESET_MOTIF_RUN_2>`, 0x2798 | `_CW004a` / `Knot-3mm`, 540, Abstand 0x27df = 1261 (7,0 mm) |
| Backstitch | Kontur-Stichart 0x2850 | §2 |
| Stemstitch | Kontur-Stichart 0x2855 | §3 |
| Stipple | eigene Füllart | §4 |

Der Unterstrich vor dem Satznamen (`_BW004a`) kennzeichnet die Sätze der Leiste; auf der CD heißen die Ordner `BW004a`,
`CW004a`, `LW004a` (Bericht 20 §1).

**Korrektur Bericht 20 §2/§5.** Bericht 20 hatte die beiden Motiv-Sticharten vertauscht. Richtig ist:
**0x2798 = Motivlinie** (Kontur) und **0x27ee = Motivfüllung**.

Belege:
1. In jedem Motivstil stehen die eigenen Werte (Flag 0) unter der Stichart, die der Stil meint:
   - `<PRESET_MOTIF_FILL_1…3>` unter 0x27ee. Unter 0x2798 steht dort nur der geerbte Wert `Children`/`901` mit Flag
     0x2000 (= nicht gesetzt, [05](05_effekte_und_unterlagen.md) §2).
   - `<PRESET_MOTIF_RUN_1/2>` und `<PRESET_STANDARD_MOTIF_RUN>` unter 0x2798.
2. House2.ART, das einzige Musterdesign mit einem MotifFill-Objekt (0x3d, Raster 0x3f = `0000 a005 a005` =
   1440/1440), trägt die Füll-Stichart **0x27ee**. Die acht Einträge „MOTIFS“/„21“ hängen an 0x2798 – das sind
   Motivlinien, keine Motivfüllungen.

**Standardwerte der Motiv-Sticharten (korrigiert, im Kern seit N3):**
- `<PRESET_STANDARD_MOTIF_RUN>` 0x2798: `NP004a` / `K0001a`, Abstand 0x27df = 2146 (11,92 mm = Grundlinie von K0001a).
- `<PRESET_STANDARD_MOTIF_FILL>` 0x27ee: `NP004a` / `K0001a`, Größe 0x3ab8 = 2158 (12,0 mm), Raster 2158/2158 (Kante an
  Kante).

Vorher stand im Kern Children 901 (Füllung) bzw. Heirloom 705 (Linie, eigene Festlegung). Ein gespeichertes Design ohne
Angabe von `satz`/`motiv` stickt jetzt mit K0001a.

## 2. Rückstich (Backstitch) — Sicherheit: hoch (Code)
`BackStitchFill::AddRunPoint`, MOD1 0x1001d960; die Linie wird vorher von `DoRun` (0x1001d8e0) mit der Stichlänge als
Laufstich geteilt (Ecken bleiben).

Je Laufpunkt P mit Vorgänger Q (D = P − Q, o = Stärke · Normale von D) und Rückpunkt M:
- **3 Striche:** P + o, M − o, P. Liegt Q weiter als 1,33 · Stärke von der Strecke (P + o, M − o) entfernt (Ecke), rückt M
  vorher auf die Mitte zwischen M und Q.
- **5 Striche:** P + o, M + o/2, P − o, M − o/2, P.
- Danach M = P − f · D mit f = Überlappung (beim ersten Stich f / (f + 1)); am Linienende M = P.

Stil `<0 Software Defaults>` 0x2850: Länge 0x2851 = 540 (3,0 mm), Stärke 0x2852 = 36 (0,2 mm), Überlappung 0x2853 =
0,5, Striche 0x2854 = 3. Dialog „Backstitch“ BERESENU DLG 1448.

## 3. Stielstich (Stemstitch) — Sicherheit: mittel
`StemStitchFill`, MOD1 0x1019d330 / 0x1019d4d0 / 0x1019d9e0 / 0x1019d6a0.
- Laufpunkte C im Schritt L = Abstand / sin(a).
- Je Punkt ein schräger Stich über die Linienbreite: s = h · n + (h / tan a) · u (h = halbe Breite, u = Richtung,
  n = Normale); Stich C + s → C − s.
- **Enge Kurven:** Ist der Umkreisradius r (über drei Punkte gemittelt) kleiner als 4 · Breite, wird der Winkel
  steiler: a' = a + (90° − a) · min(1, Breite / r − 0,25).
- **Dreifach** („Triple“): C + s, C − s + o, C + s − o, C − s mit o = Stärke · Normale von s.

Stil 0x2855: Breite 0x2857 = 360 (2,0 mm), Abstand 0x2859 = 270 (1,5 mm), Winkel 0x285a = 0,8727 rad (50°), Art 0x2858 =
3 (dreifach; 2 = einfach), Stärke 0x2856 = 36 (0,2 mm).

**Mittel**, weil Breite ↔ Abstand nur über die Verwendung im Code zugeordnet sind: Breite im Generator, Abstand / sin a als
Schrittweite. Die einfache Form wirkt im Bild wie ein Zickzack mit Verbindern; das ist der gelesene Ablauf.

## 4. Stippling — Sicherheit: niedrig (eigene Festlegung)
Handbuch S. 242: „a fill made of run stitches which meander around within a border“. Dialog „Stipple“ (BERESENU DLG 1450):
Stichlänge und Schlaufenabstand. Das Original rechnet über die Mittelachse der Fläche:
- `StippleFillInterface::StippleFill_MedialAxisSequence`, MOD1 0x101a4970 / 0x101a4c30;
- `CreateStippleFillPolygons` 0x101a4590.

Nicht nachgebaut, der Aufwand ist groß. Die Werte liegen im Dokument (`DesignDocument::GetCurrentStippleFillValues`
+0x138), nicht im Stil.

**Kern (eigene Festlegung):**
1. Raster mit Zellen von 2 · Abstand. Eine Zelle zählt, wenn ihre Mitte in der Fläche liegt und mindestens einen Abstand
   vom Rand entfernt ist.
2. Je zusammenhängendem Teil ein zufälliger Spannbaum (Prim, Park–Miller mit Startwert 20031997). So ist das Ergebnis
   bei jeder Berechnung gleich.
3. Die Linie umläuft den Baum im Viertelzellen-Abstand („Spanning Tree Coverage“). Das ergibt einen geschlossenen Weg
   ohne Kreuzung, Nachbarbahnen liegen einen Abstand auseinander.
4. Die Ecken werden um ±0,15 · Abstand verschoben und zweimal mit Chaikin gerundet, dann folgt Laufstich mit der
   Stichlänge.

Löcher bleiben frei; getrennte Teile werden nach Nähe verbunden. Vorgaben 2,0 / 3,0 mm stammen aus dem ersten
Beispielbild im Handbuch S. 243 (weitere Beispiele 1,0 bzw. 1,5 mm).

## 5. Blackwork Run — Sicherheit: niedrig (Verfahren), hoch (Verhalten laut Handbuch)
Handbuch S. 131–132:
- mehrere sich überlappende Linien werden **ein** Objekt mit einem Ein- und einem Ausstich, „without having to think about
  the most efficient stitching sequence and joins“;
- es gilt die Stichart des ersten Objekts (Einfach-/Dreifachlauf, Stielstich, Rückstich), „Outlines with Pattern Run
  stitch types cannot be used“;
- jede Linie bleibt einzeln formbar.

Der Code des Originals ist nicht gelesen.

**Kern (eigene Festlegung, `blackwork.rs`):**
1. Enden, die bis 0,5 mm an eine Linie heranreichen, werden auf sie gezogen. Die Linien werden an allen Schnittpunkten
   geteilt; Knoten innerhalb von 0,05 mm verschmelzen. So entsteht ein Netz.
2. Für jedes Netz:
   - Anfang ist der Knoten nächst dem vorigen Stich (gibt es Knoten mit ungerader Kantenzahl, der nächste davon).
   - Ende ist der ungerade Knoten, bei dem die übrigen ungeraden Knoten sich am billigsten paaren lassen (gierig, über
     kürzeste Wege). Die Wege dieser Paare werden doppelt gestickt, wie beim Doppellauf der Handarbeit.
   - Der Euler-Weg folgt Hierholzer.
3. Getrennte Netze folgen nach Nähe. An Knoten wird eingestochen.

Befehl `blackwork_lauf`: Formen und Transformation werden vorher in Punkte umgewandelt, geschlossene Linien zum Anfang
zurückgeführt. Das neue Objekt steht an der Stelle des ersten.

## 6. Umsetzung im Programm
- Objektarten `rueckstich`, `stielstich`, `stippling`, `blackwork` (`format.rs`), auch über „Stichart wechseln“.
- Einfügen-Menü „Craft Stitches“ mit den acht Vorlagen (`craft_vorlagen`). Rand/Füllung/Lacework sind Motivlinien bzw.
  Motivfüllungen mit den Werten aus §1.
- Anordnen → „Blackwork Run“.
- Prüfung der Bereiche in `design.rs` (`craft_pruefen`).

## 7. Eigene Festlegungen
- Stippling-Verfahren (§4) und Blackwork-Reihenfolge (§5).
- Fangweite 0,5 mm beim Blackwork.
- Vorgaben Stippling 2,0 / 3,0 mm (Handbuch-Beispiel).
- Craft-Vorlagen mit fester Startform (Linie bzw. Kreis in der Rahmenmitte).

## 8. Offen
- Stippling über die Mittelachse wie das Original (MOD1 0x101a4970 ff.); Werte im Dokument +0x138.
- Stippling mit anderen Sticharten als Laufstich – im Original nicht geprüft.
- „Double Run“ (Doppellauf als eigene Stichart) – im Kern nur als Folge des Blackwork-Wegs.
- Stilteile 0x2828 und 0x2830 unbekannt (0x2830 möglicherweise Kreuzstich, ROADMAP 4b.15).
- Stielstich: Zuordnung Breite/Abstand am Gerät prüfen (Kalibrierblatt).
