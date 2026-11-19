# 02 – Echte Stiche des Originals aus den ART-Musterdesigns (19.11.2026)

**Ergebnis:** Die vom Original berechneten Stiche lassen sich aus den 287 Musterdesigns der CD lesen. Damit gibt es
einen Maßstab für unsere Stichberechnung ohne Dongle. Werkzeug: `Arbeitsdaten/werkzeuge/art_stiche.py`
(nutzt `art_lesen.py`, `art_decrypt.py`, `twofish_cfb1.c`). Original-Software entpackt unter `F:\Entwicklung\artista_ziel`.

## 1. Wo die Stiche stehen — Sicherheit: hoch
- Neuere ART (94 Stück, AUX_INFO `03…`): Stream `DesignDocument` (Twofish, `Analyse/code/05`). `Contents` ist dort nur ein
  kurzer Stummel (Kopf mit 3 Einträgen).
- Ältere ART (193 Stück): Stream `Contents` (Substitution) enthält denselben Datenblock.
- Der Datenblock beginnt mit einem **Kopf von 0x74 Byte** (Leser IMPORT.DLL **0x10078b10**): Magic `0x193866de`,
  0x74, Gesamtgröße, zweimal ein Formatstand (hier 0x3793576/7, als „Version“ an die Abschnittsleser übergeben),
  dann Abschnitte, an 0x70 eine **Prüfsumme = Summe der ersten 28 DWORDs**.
- Aufrufer IMPORT.DLL **0x100777d0** (Design laden): Abschnitt 1 = **Stichpunkte** (Kopf +0x14 Anzahl, +0x18 Offset,
  +0x1c Größe), Abschnitt 2 = **Stichreihenfolge/Segmente** (+0x20 Anzahl, +0x24 Offset, +0x28 Größe); danach Maschine
  (`EmbroideryMachine::Init`), weitere Abschnitte und die Objekte. Beleg Anzahl: Design-Eigenschaft „Stiche“ = Kopfwert − 3
  (Flower 001: 19 720 / 19 723; HT116: 192 / 195).

## 2. Abschnitt 1: Stichpunkte (IMPORT.DLL 0x10037330) — Sicherheit: hoch
Je Punkt ein Byte: **unteres Halbbyte = x-Code, oberes = y-Code**. Ist Bit 3 eines Codes gesetzt, folgt ein gepacktes
Byte (WESSYS `ReadBytePacked` 0x10029de0 / `X_ReadBytePacked` 0x1002a140: 7 Bit je Byte, Bit 7 = Fortsetzung),
dessen Wert um 4 Bit verschoben angehängt wird. Jede Achse hat Zustand (Position, Schritt, Betrag, Vorzeichen):
- **Code < 8:** ZigZag-Wert d = c>>1 (bei c&1: ~d); **Schritt = alter Schritt + d** (zweite Differenz), Position += Schritt.
- **Code ≥ 8:** Bit 3 entfernen → c′; Betragsänderung m = c′>>2 (bei c′&2: ~m); Betrag += m; c′&1 kehrt das Vorzeichen um;
  Schritt = Betrag bzw. ~Betrag; Position += Schritt.

Prüfung: Für HT116 (195 Punkte) und Flower 001 (19 723) endet das Lesen **exakt am Abschnittsende**; die gezeichneten
Punkte ergeben die Designs (Bilder im Scratchpad der Sitzung).

**Einheiten:** x in 1/180 mm, **y doppelt (1/360 mm)**. Belege: (1) der Vernäh-Generator des Originals halbiert y beim
Lesen der Stichpunkte (`sar ecx, 1`, MOD1.DLL 0x101ca796/0x101ca863); (2) die Segmentpunkte in Abschnitt 2 haben halbe
y-Werte; (3) Vorschaubild `DESIGN_ICON` (100×100-BMP) passt nur halbiert; (4) Quiltdesigns HT1xx haben dann
Laufstiche von genau 2,0 mm; (5) alle 14 geprüften Designs aus „Samples 4“ passen nur halbiert in ihren Rahmen
(Borten sonst bis 375 mm im 255-mm-Oval).

## 3. Abschnitt 2: Segmente (IMPORT.DLL 0x10036640, MOD1 `StitchSegment`/`StitchOrder`) — Sicherheit: hoch (Format), mittel (Bedeutung)
Je Segment: gepacktes Byte (Bit 0 = Segmentart, Rest = Stichanzahl n), 4 ZigZag-Wörter (WESSYS `ReadWordPacked`
0x10029ed0 / `X_ReadWordPacked` 0x1002a1c0; x- und y-Werte jeweils als Differenz zum vorigen Wert derselben Achse),
gepacktes Feld (+0x3c, 16 Bit), Anzahl k, dann k Attribute (3 Byte, ab Version 6 ein weiteres Byte, ab Version 0x14
ein 8-Byte-Schlüssel, über den ein Objekt nachgeschlagen wird). Prüfung: Lesen endet exakt am Abschnittsende.
Bedeutung (an HT116 abgelesen): **Art 1 = Ankerpunkt** (Koordinate = Punkt am Ende, n = Stiche seit dem vorigen Anker),
**Art 0 = Begrenzung** (x0, y0, x1, y1) der Stichstrecke dazwischen. Das Feld +0x3c zählt in Flower 001 hoch (0…7 …),
vermutlich Farbe/Objektnummer – offen.

## 4. Vernähen (Tie In / Tie Off) im Original — Sicherheit: hoch
Gemessen an allen 94 neueren Musterdesigns und am Code (MOD1.DLL `PhysicalMachine::ExpandTieInStandard` 0x10131850,
`ExpandTieOffStandard` 0x10131140, `TieOffGenerator::Generate` 0x101ca200 / `GetStitchPoints` 0x101ca750,
`TieParameters(Typ, Schalter, Länge1, Länge2)` 0x10055250):
- **Tie Off:** vom Endpunkt E rückwärts entlang des letzten Stichs: E − t, E − 2t, E − t, E (danach ein doppelter
  Endpunkt). **t = min(1,0 mm, letzter Stich / 2)**: letzter Stich 1,8 → 0,9 (26 Designs), 2,0 → 1,0, 1,3 → 0,65,
  13 mm → 1,0.
- **Tie In:** spiegelbildlich am Anfang S entlang des ersten Stichs: S, S + t, S + 2t, S + t, S, dann weiter
  (Blackwork-Sampler 03/07/08/09).
- Damit sind die Stilwerte bestätigt: **0x2730/0x2731 = 1,0 mm = größte Vernäh-Schrittlänge**, **0x272f/0x275d = 2 =
  Anzahl der Schritte** je Richtung.
- Sonderfall im Code: Bei sehr kurzen Stichen sammelt `GetStitchPoints` bis zu 4 Punkte über mehrere frühere Stiche
  (höchstens 40 Schritte), mit einer **Knickgrenze von 30°** (0.523599). Noch nicht nachgebaut.
- Quiltdesigns (HT1xx) haben kein Vernähen (eigene Verbinder-Einstellungen im Design).

**Umgesetzt im Rust-Kern** (`kern/stick-kern/src/nachbearbeitung.rs`), Referenzänderung in `kern/testdaten/referenz/README.md`.

## 5. Nächste Schritte
1. Objekt-Abschnitte des Datenblocks dekodieren (IMPORT.DLL 0x1002e290, 0x1002f5b0, 0x1002ffb0, 0x10079170) → Objekte
   mit ihren Eigenschaften (Stichart, Abstand, Unterlage) den Stichen zuordnen (Schlüssel in den Segment-Attributen).
2. Damit an echten Designs messen: Satin-Abstand je Breite (Auto Spacing), Tatami-Versatz 0x2719/0x271a,
   Unterlagenwahl je Objekt, Zickzack-Unterlage, Laufwege in Flächen.
3. Vergleichsdesigns: `Reference/Sampler Stitch Angles`, `Sampler Step stitch pattern 1–20`, Fancy-Füllungen.
