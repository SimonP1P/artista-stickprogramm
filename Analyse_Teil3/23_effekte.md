# 23 – Effekte: Verlauf, ausgefranster Rand, Welle, Farben mischen

Stand 22.11.2026 (Nachtauftrag N2, ROADMAP 4b.8). Handbuch Kap. 24 „Artistic Stitch Effects“ (gedruckte S. 213–222).
Werkzeuge `Arbeitsdaten/werkzeuge/import_klassen.py` (Klassen in IMPORT.DLL), `art_effekte_objekte.py` (Objekte eines
Musterdesigns mit Formen und Stichblöcken als JSON), `art_effekte_verlauf.py`, `art_effekte_rand.py` (Messungen).
Vorher bekannt: Auto Spacing 0x21, Faktor 0x22, Kurzstiche 0x23 ([05](05_effekte_und_unterlagen.md) §1).

## 1. Klassen der Effekte — Sicherheit: hoch (Klassennamen aus IMPORT.DLL)
Jede Datensatz-Klasse hat in IMPORT.DLL eine Vtabelle mit 4 Einträgen. Eintrag 3 springt über einen `jmp [IAT]`-Thunk
nach `…::CreateData` der Klasse. Der importierte Name nennt so die Klasse (`import_klassen.py`).

| Klasse | Name im Original | Bedeutung |
|---|---|---|
| 0x32 | JaggedEdgeEffect | **Textured Edge** (ausgefranster Rand) |
| 0x34 | GradientSpacingEffect | **Gradient Fill** (Verlauf) |
| 0x6b / 0x6d | CurveFillEffect | **Wave Effect** (Welle) |
| 0x6e / 0x6c | OneCurveD2Wrap / TwoCurvesD2Wrap | Leitlinie(n) der Welle |
| 0x6a | SmartCornerEffect | Ecken (in [05](05_effekte_und_unterlagen.md) noch „offen“) |
| 0x8b | ColumnWidthEffect | Säulenbreite |
| 0x8c / 0x24 | PullCompensationEffect / …Old | Zugausgleich (0x24 = alte Form, in [05] „Winkel 30°?“) |
| 0x25 | FractionalSpacingEffect | Abstand in Bruchteilen |
| 0x31 / 0x3b | SplitEffect | Teilung |
| 0x68 | FlexiProgramSplitEffect | Füllmuster-Teilung |
| 0x5a | CompoundChenilleEffect | Chenille (nicht für die artista 185) |
| 0x5f | D3Effect | 3D-Effekt |
| 0x3e | OFDelayedRun | verzögerter Lauf |

## 2. Datensätze und Stilwerte — Sicherheit: hoch (gelesen), mittel (Zuordnung im Stil)
- **0x34 GradientSpacingEffect:** u16 kleinster Abstand, u16 größter Abstand (1/180 mm), Byte Profil 1–4. Eingebettet im
  Stil der Musterdesigns: `48 00 68 01 01` = 72 / 360 / Profil 1 = **0,4 / 2,0 mm, Profil 1**.
- **0x32 JaggedEdgeEffect:** Byte Seite, float Spanne (1/180 mm), float Textur. Vorgabe: Seite 2, 270 = **1,5 mm**, **0,1**.
- **Stil `<0 Software Defaults>`** (`Analyse/daten/entschluesselt/styles.txt`):
  - 0x2788 mit 0x2789 = 72, 0x278a = 360, 0x278b = 2 → Gradient-Vorgabe (0,4 / 2,0 mm);
  - 0x27cf mit 0x27d7 = 360, 0x27d8 = 396, 0x27d9 = 1, 0x27da = 1 → **Color Blending**: größter Abstand unten
    **2,0 mm**, oben **2,2 mm**, Profile 1/1 (mittel: Zuordnung erschlossen);
  - 0x277a mit 0x277d = 540, 0x277c = 2,0, 0x277b = 2 → vermutlich die Vorgabe für Textured Edge (niedrig).
- **Dialoge (RESENU, hoch):**
  - DLG 1038 „Accordion“ = Gradient Fill: 8 Profilknöpfe Linear, Dec-Linear, Inc_Exp, Dec-Exp, Convex, Concave,
    Convex-Concave, Concave-Convex; Felder Maximum Spacing und Minimum Spacing.
  - DLG 1401 „Color Gradient“: nur Linear/Dec-Linear; „**Minimum Spacing is preset to 0.4mm**“.
  - DLG 1403 „Color Blending“: unten Linear, Inc_Exp, Convex, Convex-Concave; oben **jeweils das Gegenstück**
    Dec-Linear, Dec-Exp, Concave, Concave-Convex; Feld Maximum Spacing.
- **Handbuch:**
  - Gradient Fill: „Four Gradient Fill effects are available“; „the current spacing settings are ignored“ (S. 218).
  - Wave Effect: „Stitches follow the line but maintain uniform density and needle penetration patterns“; nur
    Closed-Curve-Objekte mit Step, Fancy oder Craft-Sticharten; „Use smaller stitch lengths — e.g. 4.00 mm or less“
    (S. 217).
  - Textured Edge: Seite links/beide/rechts, Texture-Regler (links rau, rechts glatt), Span-Regler (S. 214).
  - Color Blending: „duplicating the object, then applying Gradient Fill and Travel on Edges to both layers, which are
    then grouped“; gilt für Satin-, Step- und Fancy-Flächen sowie Satin- und Blanket-Umrisse; Tipp: Unterlage
    abschalten, sonst scheint sie durch (S. 220–222).

## 3. Verlauf gemessen — Sicherheit: mittel
`art_effekte_verlauf.py` an Step-Flächen mit 0x34. Gemessen wurde der Abstand der Reihen quer zur Stichrichtung gegen
die Lage in der Fläche:
- Tomatos.ART #2, Profil 4: Reihenabstand fällt von 0,68 auf 0,32 mm (Nachbarreihen; zwischen Hinreihen doppelt).
- Tomatos.ART #3, Profil 3: Reihenabstand steigt von 0,29 auf 0,65 mm.
- AIRLINER.ART #4, Profil 2: Abstand fällt von 2,0 auf 1,0 mm.

In allen drei Fällen ändert sich der Abstand **linear** über die Fläche. Zwischen Hinreihen gilt der volle Abstand,
zwischen Nachbarreihen der halbe, wie beim Tatami ([06](06_messungen_tatami.md)). Nicht bestimmt ist:
- an welcher Seite der Fläche das Profil beginnt;
- welche Profilnummer zu welchem Knopf gehört;
- wie die gekrümmten Profile genau verlaufen.

## 4. Ausgefranster Rand gemessen — Sicherheit: mittel
Tomatos.ART #3 (Spanne 0,75 mm), `art_effekte_rand.py`:
- Die Reihenenden einer Seite liegen zwischen **−0,66 und +0,43 mm** um den Umriss.
- Die der anderen Seite liegen **genau auf dem Umriss**.

Seite 2 heißt also „eine Seite“. Nach der Reihenfolge im Dialog (Left/Both/Right) ist das **rechts**; die Spanne wirkt
nach innen und außen.

## 5. Umsetzung im Kern
- `stick-kern::effekte`: `Profil`, `Verlauf`, `Rand`, `Welle`, `welle_vorschlag`. Feld `effekte` am Objekt
  (`format.rs` `EffekteJson`), gültig für `tatami`, `muster`, `satin`, `satinlinie`, `satinflaeche`; Welle nur für
  `tatami`/`muster`.
- **Verlauf:**
  - Tatami/Muster: Die Reihen liegen nicht mehr in festem Abstand. Die nächste Reihe folgt im halben Abstand der
    Stelle (`Verlauf::reihen`).
  - Satin: Der Abstand der Einstiche richtet sich nach der Lage entlang der Säule.
  - Der Abstand des Objekts gilt dann nicht (Handbuch).
- **Rand:** Je Reihe werden die Enden auf der gewählten Seite um einen Wert in [−Spanne, +Spanne] versetzt. Bei Satin
  wird der Einstich auf der Seite zum Partner hin bzw. von ihm weg geschoben. Die Breite bleibt ≥ 0,3 mm.
- **Welle:**
  1. Die Fläche wird mit der Leitlinie verzogen (y′ = y − g(x) im Stichwinkel-System).
  2. Sie wird normal gefüllt.
  3. Die Einstiche werden zurückverzogen.

  Die Reihen folgen so der Linie, Dichte und Einstichmuster bleiben gleich (Handbuch). Die Stiche selbst bleiben gerade.
  Das erklärt den Tipp „Stichlänge ≤ 4 mm“. Die Leitlinie wird mit dem Objekt umgewandelt (`in_knoten_umwandeln`) und
  transformiert.
- **Farben mischen** (`stick-api` `farben_mischen`):
  - Das Objekt wird kopiert. Unten bleibt Profil P mit Abstand 0,4 → 2,0 mm.
  - Oben kommt das Gegenstück mit 0,4 → 2,2 mm in der zweiten Farbe, ohne Unterlage.
  - Beide Lagen kommen in eine Gruppe. Eine vorhandene Gruppe bleibt.
- **Travel on Edges:** Der Kern verbindet Reihen und Bereiche ohnehin am Rand
  ([08](08_laufwege_flaechen.md)), ohne Laufwege quer durch die Fläche. Deshalb gibt es keinen eigenen Schalter.

## 6. Eigene Festlegungen (im Kern markiert)
- Formeln der Profile: konvex t(2−t), konkav (1−t)², exponentiell (e^{3t}−1)/(e^3−1), S-Kurve t²(3−2t). Gegenstück
  wie der Dialog (Linear ↔ Dec-Linear …).
- Anfang des Verlaufs:
  - Fläche: Seite, an der die Reihen beginnen (unten im Stichwinkel-System);
  - Satin: Anfang der Säule.
- Rand: Zufallszahlen Park-Miller mit festem Startwert, damit die Stiche bei jeder Berechnung gleich sind. Glätte
  g mischt den neuen Wert mit dem der Vorreihe (g · alt + (1 − g) · neu).
- Welle: Vorschlag der Leitlinie quer durch die Fläche in Stichrichtung, 1 mm über den Rand, Bogen von 15 % der
  Flächenhöhe (`CurveFillEffect::CreateDefaultShape` nicht gelesen).
- Farben mischen: Die obere Lage bekommt **automatisch** keine Unterlage (Handbuch: Tipp zum Abschalten).
- Auswahl der Profile im Inspektor: alle 8 Profile des Gradient-Dialogs; im Dialog „Farben mischen“ wählt man das
  Profil der unteren Lage, die obere bekommt das Gegenstück.

## 7. Offen
- Profilnummern 1–4 der ART-Datensätze ↔ Knöpfe; Richtung des Verlaufs am Gerät prüfen (Kalibrierblatt). Wichtig erst
  für ART mit Objekten (ROADMAP 4b.13).
- Form der Vorgabe-Leitlinie im Original, TwoCurvesD2Wrap (zwei Leitlinien) – nicht umgesetzt.
- Textured Edge und Verlauf auf Satin im Original nicht gemessen (Kern: Regel der Flächen sinngemäß übertragen).
