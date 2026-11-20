# 09 – Die Original-Alphabete (ESB) (19.11.2026)

**Ergebnis:** Alle **51 Dateien** in `Main/PROGRAM/wletter/` (**50 Alphabete** + ButtonHoles) lassen sich vollständig lesen: je Zeichen Grundlinie,
Umrisse (Bézier-Knoten), Winkellinien der Satinsäulen, Stichart mit Parametern, Verbinder. Werkzeug:
`Arbeitsdaten/werkzeuge/esb_lesen.py` (`--png` zeichnet alle Zeichen mit Umriss, Winkellinien und Grundlinie).
Zum Vergleich gibt es die **echten Stiche des Originals** für 7 Alphabete: die Zeichensätze Nr. 1–7 aus dem
Maschinenspeicher (`Arbeitsdaten/maschinendesigns/`, Analyse_Teil2/07 Test v3.6).

## 1. Datei — Sicherheit: hoch (IMPORT.DLL, Klasse `AlphabetFile`)
OLE-Datei mit den Streams
- `\x06HEADER` (unverschlüsselt, Leser 0x100040f0 / `AlphabetFile::GetHeader` 0x10003cf0):
  u32 **0x0d0a1aed**, u32 Länge (0xb7), Byte Version (1, ≤ 4), Byte Unterversion (3), [u32 bei Unterversion ≥ 2],
  **Name** UTF-16 (0xa2 Byte), **float** (0,0), **float Buchstabenabstand** (Drifter/Alice 0,10, Zurich 0,08), 11 Byte
  (0), [Version ≥ 4: 32 Byte], u16 (0x5002, Bedeutung offen), [Version ≥ 4: u32 + 8 Byte].
- `AUX_INFO` (5 Byte `01 00 00 00 00`).
- **ein Stream je Zeichen**, Name = Zeichencode hex (`0041` = A, `00E9` = é), Substitution Format-ID 0xD2 + zlib
  (`art_decrypt.try_all`, Analyse/code/05). Kein Leerzeichen-Stream.

## 2. Zeichen-Stream — Sicherheit: hoch (IMPORT.DLL 0x100c6230)
- 0x24 Byte Kopf: u32 0x0d0a1a0d, u32 0xeaae8356, u32 0x24, u32 6 (≤ 0x3793578), u32 0x11, u32 0x24, u32 Länge,
  u32 0, u32 **Prüfsumme = Summe der ersten 8**.
- Danach Datensätze im Rahmen der ART-Objekte ([03](03_art_objekte.md) §2), gelesen mit 0x100d9a60/0x100d9b90:
  - **0x2e Buchstabenkopf** (0x1000b330): 4 vorzeichenbehaftete gepackte Wörter = **Grundlinie x1, y1, x2, y2**
    (1/180 mm, Koordinaten des Digitalisier-Blatts; Vorschub = x2 − x1).
  - **0x3a** Zeichen (u32 Anzahl + UTF-16).
  - die Objekte: meist **0x08 EmbroideryBoundGroup** (Buchstabe) mit **0x06 EmbroideryPatch** (Satinsäulen); daneben
    0x05 EmbroideryCurve (Linien), 0x0c InputCCurve (Säule mit zwei Kanten, Applikations-Alphabete), 0x0b ManualObject.
- **0x06 EmbroideryPatch** (Serialisierer 0x10018970): u16 n, dann n × (gepackt **a**, float **x**, gepackt **c**,
  float **y**) = **Winkellinien**: von Segment a beim Anteil x zu Segment c beim Anteil y des Umrisses; danach der
  Umriss (0x01 GeoCurve, geschlossen). Beispiel „A“ (Drifter), Umriss mit 14 Knoten: (0, 0,0 → 12, 1,0) … (5 → 7, 1,0),
  (6, 0,0 → 7, 0,0), d. h. Knoten 0↔13, 1↔12 … 6↔7: die Säule läuft auf der einen Seite über die Knoten 0…6, auf der
  anderen rückwärts über 13…7.
- Stichart wie bei ART-Objekten über den Baum 0x2741 → Verbinder 0x272a (0x14: 1080, 540, 1080, 540, 180, …) und
  Füllung 0x2748 → **Satin 0x2721** (0x16 Auto-Split 1800, 0x15 Abstand **72 = 0,40 mm**) + **Laufweg 0x271c**
  (0x18 Länge 540 = 3 mm). Die zweite Säule verweist auf dieselben Datensätze (Verweis 2/3).

## 3. Inhalt der 50 Alphabete — Sicherheit: hoch (Zählung mit `esb_lesen.py`)
- 9 bis 162 Zeichen je Alphabet (Drifter 141: Satzzeichen, Ziffern, A–Z, a–z, Umlaute/Akzente, Æ, Ø, ß …).
- Fast überall **Satinsäulen (EmbroideryPatch) + Laufweg**; Unterlagen nur in einzelnen Alphabeten (Mittellauf:
  Blackboard, Swiss Block, Varsity Block, Tabloid; Zickzack: Castle, Greek Ionic, Hebrew 2, Jikharev, Secret Garden;
  Rand: Island, Tabloid); Tatami nur in Anniversary (5 Objekte); Laufstich-Alphabete: Creative Cross, Quilting Text
  (ManualObject/Lauf); Säulen mit zwei Kanten (InputCCurve): Easy Appliqué, Varsity Applique, Tabloid, ButtonHoles.
- **Großbuchstaben ≈ 50 mm hoch digitalisiert** (H: 47–56 mm, Zurich 61,5 mm).

## 4. Maßstab und Abstand — Sicherheit: mittel-hoch (Messung), Code offen
- Die 7 Zeichensätze im Maschinenspeicher sind **genau** diese Alphabete im **Maßstab 0,500** (Höhen 25,0/50,0,
  25,3/50,8, 26,1/52,1 …; mittlere Abweichung der Maße A–J 0,07–0,33 mm): Nr. 1 Drifter, 2 Lisa, 3 Blackboard,
  4 Swiss Block, 5 Zurich, 6 Alice, 7 Bamboo. Der Maßstab ist auch bei Zurich (H 61,5 mm) 0,500 → die
  **Referenzhöhe ist für alle Alphabete gleich**, vermutlich **50 mm = 9000 Einheiten** („Buchstabenhöhe 25 mm“ ⇒ 0,5).
  Im Code noch nicht gefunden (Kandidaten: Nutzer von `AlphabetFile::GetHeader`, `TrueTypeFont::GetRefHeight`).
- **Buchstabenabstand** „automatically as a percentage of letter height“ (Handbuch PDF-S. 168) = float 2 im Kopf
  (Drifter 10 %). In den Zeichensatz-Designs der Maschine stehen die Buchstaben ohne Zwischenraum (B beginnt bei
  x = 19,1 mm, wo A endet) – dort also Abstand 0 bzw. nur der Vorschub.
- Buchstabenhöhe im Original 1–200 mm (Handbuch S. 165), Breite in % der Höhe (100 % = Original).

## 5. Umsetzung im Programm (19.11.2026)
- `kern/stick-formate/src/esb.rs` liest die Alphabete (Test mit DRIFTER.ESB von der CD: 141 Zeichen, „A“ = 2 Säulen mit
  14/22 Knoten, 7 Winkellinien, Satin 0,40 mm); `kern/stick-kern/src/schrift.rs` setzt Texte: Maßstab Höhe/50 mm,
  Vorschub x2 − x1, Buchstabenabstand = Anteil aus dem Kopf × Höhe, Leerzeichen 0,3 × Höhe (**eigene Festlegung**),
  Satinsäule → Seiten A/B über die Winkellinien (Abtastung 1 Punkt/mm: eigene Festlegung), fester Abstand aus 0x15.
- Design-JSON: `{"art": "text", "text": …, "alphabet": "Drifter", "hoehe": 20, "breite": 100, "abstand": …,
  "position": [x, y], "ausrichtung": "mitte"}`. Programm: Bereich „Schrift“ (Text, Alphabet, Höhe, Breite, Farbe),
  Alphabete als Ressource `app/src-tauri/alphabete/`.
- **Stufe 2 (19.11.2026, [10](10_regeln_original.md)):** Stil „Schrift“ wie im Original – Auto Spacing (Effekt 0x21 am
  Zeichen) × 0,9, Kurzstiche an, Auto-Unterlage Zickzack (Unterlage aus dem Alphabet hat Vorrang, Text-Option
  `unterlage`), Buchstabe als Gruppe mit Laufweg (3,0 mm aus 0x271c) statt Sprung. „A“ jetzt wie das Original ohne
  Sprung. Die Zeichensätze der Maschine weichen vom Stil ab (fester Abstand 0,40 mm, kein Zugausgleich, Mittellauf
  bzw. Zickzack je Alphabet – von Bernina so erstellt).
- **Erster Vergleich** „A“ Drifter 25 mm mit dem Original aus der Maschine (Zeichensatz Nr. 1): 373 statt 397 Stiche,
  gleiche Stichrichtungen und Reihenfolge der Säulen (Querbalken zuerst). Es fehlen noch: (a) **Mittellauf als Unterlage**
  unter beiden Säulen (im Alphabet nicht enthalten → automatische Schrift-Unterlage des Originals, `let_underlay`),
  (b) **Kurzstiche** an der Innenkurve (Einstiche innen abwechselnd kürzer; Kandidaten: Datensätze 0x21–0x26 der
  Satin-Stichart, z. B. 0x24 = 30,0), (c) Laufweg innerhalb des Buchstabens statt Sprung, Vernähen 2 × 1,0 mm.
- `Main/PROGRAM/userletw/` (14 ESB) enthält **keine Alphabete**, sondern Motiv-Sammlungen (Children, Corners, Heirloom,
  Quilting …) für Motivfüllung/-linie – Streams nicht nach Zeichencodes benannt; später für „Fancy/Muster“ analysieren.

## 6. Offen
1. Referenzhöhe und Buchstabenabstand im Code bestätigen (Lettering-Modul: wer liest `AlphabetHeader` +0x10).
2. ✅ 20.11.2026 Wortabstand und Zeilenabstand: **Vorlage NORMAL.AMT, Schriftvorgaben 0x3e82**: 0x3e84 = 0,1
   (Buchstabenabstand), **0x3e85 = 0,25 (Wortabstand)**, **0x3f2d = 0,6 (Zeilenabstand)**, je Anteil der Höhe; 0x3e88 =
   1800 (Höhe 10 mm), 0x3e89 = 1,0 (Breite), 0x3e8a = 0 (Kursiv), 0x3f03 = π/2, 0x3e86 = 2, Grundlinie 0x3e8b/0x3e8c =
   9000 (Radius X/Y 50 mm?), 0x3e90 = 18000 (Länge 100 mm?), 0x3e92 = 5 (Art?). Zuordnung über den Lettering-Dialog
   (RESENU: „Letter/Word/Line Spacing … % of Height“, Grundlinie: Length, Line Angle, Radius X/Y, Justif. Angle, Tilt,
   Arc Angle, Above/Through/Below) und die Datensätze 0x7c der 820 Schriftobjekte in den Musterdesigns (Aufbau: 3 Byte,
   double Buchstabenabstand 0,1/0,2, double Wortabstand 0,25/0,5, u32, double Höhe, double Breite 1,0, double 0,
   double π/2, …, double Zeilenabstand 0,6/0,5). Sicherheit mittel (Feldnamen erschlossen, nicht aus dem Code).
3. Automatische Unterlage für Schrift (`let_underlay`, Tag 0x197f ff.) – Vergleich mit Blackboard/Swiss Block (Unterlage
   im Alphabet) und Drifter/Lisa/Zurich/Alice/Bamboo (ohne) im Maschinenspeicher.
4. Stichbildung der Satinsäule mit Winkellinien (MOD1.DLL) gegen die echten Stiche messen.
