# 24 – Craft Stitches: Rückstich, Stielstich, Stippling, Blackwork, Candlewicking, Lacework

Stand 22.11.2026 (Nachtauftrag N3, ROADMAP 4b.9; Stippling wie das Original: N13). Handbuch Kap. 26 „Craft
Stitches“ (gedruckte S. 235–243) und S. 131–132 („Blackwork Run“). Quellen: NORMAL.AMT (`Analyse/daten/entschluesselt/styles.txt`), MOD1.DLL (Disassembly),
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

## 4. Stippling — Sicherheit: hoch (Verfahren gelesen), mittel (Folge der Schleifen, Randabstand)
Handbuch S. 242: „a fill made of run stitches which meander around within a border“. Dialog „Stipple“ (BERESENU DLG
1450): Stichlänge und Schlaufenabstand. **Seit N13 (22.11.2026) wie das Original nachgebaut** (vorher eigener Mäander
auf einem Quadratraster).

### 4.1 Wo es steht
- `StippleFillInterface::CreateStippleFillPolygons` MOD1 0x101a4590: Erzeuger 0x1019fbb0 (Konstruktor 0x101a0100,
  Lauf 0x101a2a70), jede Kurve `GeoCurve::AddToPolyline` mit 18 Einheiten Toleranz, dann Verbinden 0x101a3250.
- Folgen: `StippleFill_TrapuntoSequence` 0x101a47b0/0x101a4b00 und `StippleFill_MedialAxisSequence`
  0x101a4970/0x101a4c30.
- **Werte** (`DesignDocument::GetCurrentStippleFillValues` +0x138; der Erzeuger kopiert 6 Zahlen nach +0x10):

| Wert | Vorgabe | Bereich | Verwendung |
|---|---|---|---|
| Stichlänge | 360 = 2,0 mm | 1 … 1,8 · 10⁶ | Laufstich |
| Schlaufenabstand | 540 = 3,0 mm | 180 … 18 000 | Gitter, Verbinden |
| Randabstand | 0 | ≥ 0 | Gitterpunkte und Kanten |
| Zufall | 0,125 | 0 … 0,5 | Versatz der Schlaufenpunkte |
| Kurve | 1,0 | 0 … 1 | > 0: Bézierkurven, sonst Linienzug |
| Randgewicht | 0 | 0 … 1 | Kantengewicht nach Randnähe (0 = aus) |

- **Niemand ruft die Funktionen auf.** In keiner Programmdatei gibt es einen Aufruf oder Verweis auf die vier
  Folge-Funktionen: geprüft wurden alle `call`/`jmp`, Zeigertabellen, Importe nach Name und Ordinal sowie
  Funktionsnamen-Strings. Auch der Menübefehl „Stipple“ (STR 33592 = 0x8338) hat keinen Eintrag in einer Nachrichtentabelle.
  - Der Dialog 1450 wird nur über die allgemeine Dialog-Fabrik von DIALOG.DLL (0x1003ecd9, Schlüssel = Dialognummer)
    erzeugt.
  - Wahrscheinlich war die Funktion in dieser Version noch nicht freigeschaltet. Das Verfahren selbst ist aber
    vollständig vorhanden.

### 4.2 Verfahren (Sicherheit hoch, Adressen MOD1)
1. **Wabengitter** (0x101a0800):
   - Begrenzung der Fläche, um 1,0625 · Randabstand verkleinert.
   - Entlang y Raster s/2 (Anzahl = ⌈h/(s/2)⌉ + 1, erhöht bis ≡ 2 mod 3); entlang x Raster s·√3/2 (Anzahl
     ⌈w/(s·√3/2)⌉ + 1, ungerade). Das Raster ist genau über die Begrenzung gestreckt.
   - Behalten werden (Spalte c, Zeile r) mit c gerade und r mod 6 ∈ {0, 4} oder c ungerade und r mod 6 ∈ {1, 3}.
     Das ergibt ein Sechseckgitter mit Kantenlänge ≈ s und senkrechten Kanten.
   - Ein Punkt zählt, wenn er in der Fläche liegt und mindestens den Randabstand vom Rand hat (0x101a04e0).
2. **Kanten** zwischen Punkten mit Abstand² < 1,1 · s².
   - Die Strecke muss in der Fläche bleiben (0x101a05a0): Beide Enden liegen innen und haben den Randabstand.
   - Ist die Summe der Randabstände + 18 Einheiten kleiner als die Länge, wird die Strecke halbiert und jede Hälfte
     neu geprüft.
3. **Zufälliger Spannbaum** (0x101a1210):
   - Kantengewicht = Größe/8 · (0,9 + `rand()`/327 670) (Funktor 0x101a10c0, Tabelle 0x102410a8).
   - Bei Randgewicht > 0 kommt ein Randanteil hinzu (nicht nachgebaut, Vorgabe 0).
   - Dazu der kleinste Spannbaum aus der Graphen-Bibliothek.
4. **Rundgang** (0x101a1de0 → 0x101a1a40):
   - Je zusammenhängendem Teil beginnt er am Gitterpunkt, der dem Startpunkt am nächsten liegt.
   - Tiefensuche: Punkt, je Kind (Teilbaum, Punkt).
   - Die Kinder werden nach dem Winkel zur Richtung des Elternpunkts sortiert (Vergleich 0x101a3e10, an der Wurzel
     ab +x), und zwar **im Uhrzeigersinn**.
   - 0x101a1e20 entfernt doppelte Punkte hintereinander und führende Einträge gleich dem letzten.
5. **Schlaufenpunkte** (0x101a1f40). Mit A = 0,4 · (Pᵢ − Pᵢ₋₁), B = 0,4 · (Pᵢ₊₁ − Pᵢ), Kreuzprodukt A × B, Schwelle
   0,125 · |A|²:
   - Linkskurve: P + B − A (innere Ecke);
   - Rechtskurve: P − B − A, P − B + A, P + A (äußere Ecke). Der erste Punkt liegt 0,6 · |a| statt 0,2 · |a| vor der
     Ecke – so steht es im Code, übernommen;
   - Sackgasse: halbes Sechseck P + N − A/2, P + N + A/2, P + A, P + A/2 − N, P − A/2 − N mit N = √3/2 · A um +90°
     gedreht.
   Die Linie liegt links vom Baum (Abstand ≈ 0,35 · s) und umläuft ihn im Uhrzeigersinn.
6. **Zufall** (0x101a2340, Zufallszahl 0x101a2540 = lo + `rand()` · (hi − lo)/32 767):
   - Je Punkt r₁, r₂ ∈ [−0,125; 0,125].
   - Der Punkt wird um (−(r₁aᵧ + r₂bᵧ), r₁aₓ + r₂bₓ) verschoben; a und b sind die Nachbarstrecken des schon
     verschobenen Zugs.
7. **Glätten** (0x101a2570):
   - Je Punkt eine Bézierkurve von Streckenmitte zu Streckenmitte. Stützpunkte bei 0,8 der Strecke davor und 0,2 der
     Strecke danach (0x10048030 = lineare Interpolation).
   - An jeder Naht wird die Tangente gemittelt: T = (S₂ − C₁)/2, C₁ = E − T, S₂ = E + T.
   - `GeoCurve::Join`, Linienzug mit 0,1 mm Toleranz.
8. **Verbinden** (0x101a3250, Aufruf 0x101a470f):
   - Gesucht wird das Schleifenpaar mit dem kleinsten Abstand (`Distance`).
   - Ist er ≤ 1,125 · s, werden beide Schleifen an dieser Stelle um ± 0,0625 · s geöffnet (`MovePolygonPosition`) und
     über Kreuz verbunden (0x101a30e0). Das wiederholt sich.
   - Die Trapunto-Folge (0x101a47b0) verbindet danach alle übrigen Schleifen ohne Lücke (0x101a3670, Abstand
     unbegrenzt).
   - Die Mittelachsen-Folge (0x101a4c30 → 0x101a7a50) führt Laufwege mit Segmentarten. Nicht gelesen.

### 4.3 Kern (`sticharten/stippling.rs`)
Schritte 1–8 wie oben, mit den Werten des Originals. Geprüft mit Tests:
- Gitter und Kanten;
- Rundgang im Uhrzeigersinn, Schleife mit negativer Fläche;
- keine Kreuzung im Rechteck und mit Loch;
- Verbinden zweier Kreise ohne Kreuzung;
- gleich bei jeder Berechnung.

**Eigene Festlegungen (markiert):**
- **Randabstand 0,5 · s statt 0.** Mit 0 ragen die Schlaufen bis 0,4 · s über den Umriss. Wie die Oberfläche des
  Originals den Wert gesetzt hätte, ist nicht belegt (§4.1). Sicherheit niedrig.
- **Zufallszahlen** wie `rand()` von MSVC (LCG 214013/2531011) mit Startwert 1 je Objekt: bei jeder Berechnung
  gleich.
- **Reihenfolge der Zufallszahlen:** ein Gewicht je Kante in Gitterreihenfolge, dann je Punkt r₁, r₂.
- **Übrige Schleifen** (weiter als 1,125 · s auseinander) werden wie bisher nacheinander gestickt: die nächste zuerst,
  Beginn am nächsten Punkt, dazwischen der Verbinder. Das Original würde sie mit der Trapunto-Folge ohne Lücke
  verbinden, also mit einer doppelten Strecke quer über den Stoff, oder mit der Mittelachsen-Folge.
- **Laufstich** mit der Stichlänge über die ganze Linie.

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
- Stippling: Randabstand 0,5 · s, Zufallszahlen und Folge der übrigen Schleifen (§4.3); Blackwork-Reihenfolge (§5).
- Fangweite 0,5 mm beim Blackwork.
- Craft-Vorlagen mit fester Startform (Linie bzw. Kreis in der Rahmenmitte).

## 8. Offen
- Stippling: Mittelachsen-Folge (0x101a4c30 → 0x101a7a50) nicht gelesen; Randgewicht (Wert 6) nicht nachgebaut; wie das
  Original den Randabstand setzt, ist unbekannt (Funktionen werden nicht aufgerufen, §4.1).
- Stippling mit anderen Sticharten als Laufstich – im Original nicht geprüft.
- „Double Run“ (Doppellauf als eigene Stichart) – im Kern nur als Folge des Blackwork-Wegs.
- Stilteil 0x2830 unbekannt (möglicherweise Kreuzstich, ROADMAP 4b.15). **0x2828 = PhotoSnap-Stichart** (N5, [25](25_automatik.md) §3.1).
- Stielstich: Zuordnung Breite/Abstand am Gerät prüfen (Kalibrierblatt).
