# 12 – Applikation im Original (20.11.2026)

**Frage:** Wie erzeugt die Original-Software eine Applikation (Stoffteil aufnähen), mit welchen Lagen und Werten?
**Quellen:** Handbuch Kap. 27 „Digitizing for Appliqué“ (PDF-S. 245–250), Garnkarte `Applique.tch`, Original-Alphabet
„Easy Appliqué.esb“ (gelesen mit `Arbeitsdaten/werkzeuge/esb_lesen.py` / `esb_satin.py`), Symbole in MOD1.DLL.

## 1. Lagen und Stopps — Sicherheit: hoch (Handbuch)
Auto Appliqué erzeugt je Applikation bis zu vier Lagen: **Platzierungslinie**, **Schnittlinie** (nur bei „Trim in
Place“; bei „Pre-cut“ keine), **Heftstich** (Zickzack) und **Abdeckstich** (Satin oder Blanket). Die Maschine hält
zwischen den Lagen an: Stoff auflegen nach der Platzierungslinie, zuschneiden nach Schnitt-/Heftlinie, dann Abdecken.
Einstellungen (Objekt-Eigenschaften, Reiter Appliqué): Abdeckart Satin/Blanket, Breite, Abstand, Heftstich-Breite und
-Abstand, Versatz von Abdeck- und Heftstich gegen die Platzierungslinie (Schieberegler, innen/außen; Blanket immer
innen), Stil „Trim in Place“/„Pre-cut“. Teil-Applikation („Partial Appliqué“) für überlappende Stoffteile.

## 2. Die Stopps als „Garne“ — Sicherheit: hoch
Die Original-Garnkarte `Applique.tch` enthält genau drei Einträge: 1 „Applique Guide“ (255,60,172), 2 „Place
Applique & Tack“ (26,140,103), 3 „Trim Applique & Cover“ (85,0,170). Als Farben der Lagen erzeugen sie die Stopps und
sagen an der Maschine (Display, Garnfarbtabelle), was als Nächstes zu tun ist.

## 3. Werte aus „Easy Appliqué“ — Sicherheit: mittel (ein Alphabet, nicht die Auto-Appliqué-Vorgabe)
Buchstabe „A“ (Referenzgröße des Alphabets): je Umriss (außen und Loch)
| Lage | Objekt | Wert |
|---|---|---|
| Platzierung | EmbroideryCurve, Lauf 0x271c | Stichlänge 360 = **2,0 mm** (variabel) |
| Heften | EmbroideryCurve, Zickzack 0x2724 + Abstand 0x15 | Breite 540 = **3,0 mm**, Abstand 360 = **2,0 mm** |
| Abdecken | EmbroideryPatch (Säule mit Winkellinien entlang des Umrisses) | Breite **3,0 mm** bzw. **4,2 mm** (zweite Lage) |
Die Vorgabe von Auto Appliqué selbst steht in `AutoAppliqueValues` (MOD1: `DesignDocument::GetCurrentAppliqueValues`)
und ist noch nicht gelesen.

## 4. Umsetzung im Kern — `kern/stick-kern/src/applikation.rs`
Objekt `"art": "applikation"` (Umriss, Löcher, Farbe): alle Platzierungslinien (Lauf 2,0 mm, „Applique Guide“) →
alle Heftstiche (Zickzack 3,0 mm / 2,0 mm, „Place Applique & Tack“) → alle Abdeckstiche (Satin **2,5 mm** mittig auf
dem Umriss – Handbuch-Beispiel, eigene Festlegung – in der gewählten Farbe). Beim Senden bekommen die Lagen die Garne
der Karte `Applique.tch` (genaue Farbgleichheit), die Abdeckung das nächste Garn der gewählten Karte.

## 5. Schnittlinie und Blanket (20.11.2026)
- **Trim in Place** (Handbuch S. 246/248, hoch): Schnittlinie nach der Platzierungslinie; „Place the fabric patch after
  the placement line has been stitched, then trim after the cutting line has been stitched“. **Pre-cut**: keine
  Schnittlinie. Im Kern `"stil": "zuschneiden"`: Platzierung („Applique Guide“) → Schnittlinie Lauf 2,0 mm („Place
  Applique & Tack“) → Heften („Trim Applique & Cover“) → Abdecken (Stichlänge der Schnittlinie und Zuordnung der
  Stopps: **eigene Festlegung**). Standard bleibt Pre-cut (Vorgabe des Originals noch nicht gelesen).
- **Blanket-Abdeckung** (Handbuch S. 247, hoch): „Tacking is not used with blanket cover stitch“ – ohne Heftstich;
  Zähne nach innen (Schieberegler, Blanket immer innen). Im Kern `"abdeckung": "blanket"`.

## 6. Blanket als Stichart — Sicherheit: mittel
Kontur-Stichart **0x2727** (DIALOG.DLL 0x1000ecca: Liste der Kontur-Sticharten {0x2721 Satin, 0x2727}; Dialog
„Outline Stitch“ mit „Stitch Spacing“ und „Blanket Width“, Center/Offset). Vorlage NORMAL.AMT, Füllung 0x2748#4:
**0x2728 = 360 (2,0 mm)**, **0x2729 = 540 (3,0 mm)**, 0x27c5 = 1 (offen). Zuordnung Abstand/Breite nach dem gleich
gebauten Zickzack 0x2724 (0x2725 Abstand 2,0 / 0x2726 Breite 3,0; im Alphabet „Easy Appliqué“ Breite 540, Abstand
360). Form nach Handbuch S. 70–71 („Kamm“): je Zahn Rücken → Spitze → Rücken, dann um den Abstand weiter (**eigene
Festlegung**; kein Musterdesign enthält Blanket – die 0x1a-Datensätze in Butterfly1 gehören zu eingebetteten Stilen).
Kern: `sticharten/blanket.rs`, Objekt `"art": "blanket"` (punkte, geschlossen, abstand, breite, versatz mitte/seite).

## 7. Offen
Versatz innen/außen (Schieberegler), Teil-Applikation, Reihenfolge mehrerer Applikationen (Original:
`AppliqueOverlapProcessor`), Vorgabewerte aus `AutoAppliqueValues`, Blanket-Ecken und Feld 0x27c5.
