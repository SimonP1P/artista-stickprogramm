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
Die Vorgabe von Auto Appliqué selbst steht in `AutoAppliqueValues` – gelesen am 22.11.2026, §8.

## 4. Umsetzung im Kern — `kern/stick-kern/src/applikation.rs`
Objekt `"art": "applikation"` (Umriss, Löcher, Farbe): alle Platzierungslinien („Applique Guide“) → alle Heftstiche
(Zickzack, „Place Applique & Tack“) → alle Abdeckstiche (Satin in der gewählten Farbe). Werte seit 22.11.2026 wie
`AutoAppliqueValues` (§8): Platzierung 1,5 mm, Heften 4,0 / 5,0 mm, Abdeckung 5,0 mm / 0,5 mm (vorher Easy Appliqué
2,0 / 3,0 / 2,0 mm und Abdeckung 2,5 mm aus dem Handbuch-Beispiel). Beim Senden bekommen die Lagen die Garne
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
- Frame Out: Wirkung an der 185 (Vor Ort V15); „With Stop“ (0x3f1c/0x3f1f = 1) statt Farbwechsel – die 185 hält an
  jedem Farbwechsel, im Kern immer Farbwechsel.
- Schnitt der Teil-Abdeckung im Original (`AppliqueOverlapProcessor::FormIntersections` 0x10014460) und Richtung
  „clockwise between the start and end points“.
- Felder 0x3f22–0x3f25, 0x3f2c, 0x3f29 (Zählwerte/Stopps des allgemeinen Wilcom-Dialogs DLG 1246, für die 185 ohne
  Bedeutung erkannt).
- Blanket-Ecken und Feld 0x27c5.

## 8. Vorgaben „Auto Appliqué“ (`AutoAppliqueValues`) — Sicherheit: hoch (Code + Vorlage), Zuordnung über die Dialoge
Quellen:
- Konstruktor MOD1 **0x101f3c70**;
- `GetData` 0x101f4060 schreibt das DataItem **0x3f19**. NORMAL.AMT enthält es mit 19 Einträgen (ohne 0x3f2f/30/31);
  die Werte sind gleich dem Konstruktor.
- Zuordnung über die Datenbindung der Dialoge in DIALOG.DLL:
  - BERNINA-Reiter „Appliqué“ (BERESENU DLG 1455): 0x1001cd00 ff., Übernahme 0x1001d246 ff.;
  - allgemeiner Wilcom-Dialog „Auto Applique“ (RESENU DLG 1246): 0x1000da28 ff.

| Kennung | Feld (Struktur) | Bedeutung | Vorgabe |
|---|---|---|---|
| 0x3f1a | +0x08 | Platzierungslinie (Guide Run): Stichlänge | 270 = **1,5 mm** |
| 0x3f1b | +0x0c | Platzierungslinie: Versatz | 0 |
| 0x3f1c | +0x10 | Halt nach der Platzierung (None / With Stop / With Color Change) | 2 = Farbwechsel |
| 0x3f1d | +0x14 | Heftstich: Breite | 720 = **4,0 mm** |
| 0x3f1e | +0x18 | Heftstich: Versatz (vom Schieberegler gesetzt) | 0 |
| 0x3f1f | +0x1c | Halt nach dem Heften | 2 = Farbwechsel |
| 0x3f20 | +0x20 | Heftstich: Stichart („Use Tackdown“) | 0x2724 = **Zickzack** |
| 0x3f21 | +0x24 | Heftstich: Abstand | 900 = **5,0 mm** |
| 0x3f26 | +0x3c | Teil-Abdeckung (Partial Cover) | 0 = aus |
| 0x3f27 | +0x40 | Abdeckstich: Breite | 900 = **5,0 mm** |
| 0x3f28 | +0x44 | Abdeckstich: Versatz (Schieberegler) | 0 = mittig |
| 0x3f2a / 0x3f2b | +0x54 / +0x58 | Frame Out X / Y | 0 / 2160 (12 mm) |
| 0x3f2f | +0x48 | Abdeckstich: Stichart (Liste) | 0x2721 = **Satin** |
| 0x3f30 | +0x4c | Abdeckstich: Abstand | 90 = **0,5 mm** |
| 0x3f31 | +0x04 | Stil: 0 = Trim in Place, 1 = Pre-Cut | 1 = **Pre-Cut** |

**Versatz** (DIALOG.DLL 0x1001ce70 / 0x1001ceb0 / 0x1001d36e):
- Der Schieberegler „Inside ↔ Outside“ reicht von −1 bis +1 in 21 Stufen (SetRange 0x1001d01b).
- Der Dialog speichert **−Reglerwert** in beide Felder, 0x3f28 (Abdecken) und 0x3f1e (Heften). Ein Regler gilt also
  für beide, wie im Handbuch („the cover stitch and tack-down are offset against the placement line using the slider“).
- Links („Inside“) ergibt +1 = Band ganz im Stoff. Bei Abdeckart 0x2727 (Blanket) ist der Regler gesperrt (Handbuch:
  „always 100% inside“).
- Im Kern: `versatz` in % (−100 … 100). Anteil im Stoff = (1 + v)/2. Bei Löchern liegt der Stoff außerhalb des Rings.

**Teil-Abdeckung** (Handbuch S. 248–249):
- Im Original werden die Formen gewählt, dann „Partial Appliqué“; die unteren Lagen bekommen Teil-Abdeckung.
- Kern `teilabdeckung` (eigene Festlegung für den Schnitt): Der Abdeckstich entfällt dort, wo seine Mitte im Stoff
  einer **späteren** Applikation liegt (deren Platzierungsringe nach der Transformation). Jedes Reststück reicht einen
  Punkt unter den fremden Rand.
- Platzierung und Heften bleiben ganz.
- In der Oberfläche: Haken im Inspektor oder Anordnen → „Teil-Abdeckung (Partial Appliqué)“.

## 9. Herausfahren (Frame Out) — Sicherheit: hoch (Code), Wirkung an der 185 offen
Umsetzung N14 (22.11.2026). Quellen MOD1:
- **Punkt:** `Applique::GetJumpoutPoint` 0x1000f2f0 = 0x1000f230 + Versatz (Applique +0xb8/+0xbc = Werte +0x54/+0x58
  = 0x3f2a/0x3f2b = **0 / 2160**).
  - 0x1000f230 nimmt `GetFullExtents` (0x1000f0b0, Vereinigung der Begrenzungen aller Lagen) und daraus
    x = (links + rechts)/2, y = oberer Rand.
  - Also **Mitte der Oberkante + 12 mm nach oben**. Gleich bei `AutoAppliqueGroup::GetJumpoutPoint` 0x1001b520 =
    `GetTopCenter` + Versatz +0xbc/+0xc0.
- **Wann:** `Applique::RegenerateComponents` 0x1000e6c0 baut die Lagen der Reihe nach:

| Baustein | Lage | Bedingung |
|---|---|---|
| 0x10012bb0 | Platzierung | – |
| 0x10012ca0 | Herausfahren | Halt nach der Platzierung (+0x74 = Werte +0x10 = 0x3f1c) ≠ 0 |
| 0x10012d60 | Schnittlinie | Stil |
| 0x10012e50 | Herausfahren | Stil (+0x68 = 0x3f31) = 0 (Trim in Place) und Halt nach der Platzierung ≠ 0 |
| 0x10012f20 | Heften | – |
| 0x10013010 | Herausfahren | Halt nach dem Heften (+0x80 = Werte +0x1c = 0x3f1f) ≠ 0 |
| 0x100130f0 | Abdecken | – |

- **Was:** 0x10011a00 legt ein `ManualObject` (Handstich) mit **einem** Punkt (`AddCuspPoint`) und der Stichart
  **0x2713** an.
  - 0x2713 = Sprung: im Stil steht er nur mit 0x2711/0x2712 = 12,7 / 7,0 mm, den Werten von Auto Jump. Sicherheit:
    mittel.
  - Bei Halt = 1 („With Stop“) kommt eine zweite Funktions-Eigenschaft dazu (`AddFuncProperty`; Stopp, nicht
    gelesen).
  - Der Rahmen fährt also ohne Einstich zum Punkt und hält dort an. Danach springt er zum Anfang der nächsten Lage.
- **Folge im Original:** Die Vorgabe für beide Halte ist 2 (Farbwechsel). Das Original fährt also bei jeder
  Applikation heraus, auch wenn der BERNINA-Dialog (DLG 1455) das Feld nicht zeigt.
- Kein Musterdesign enthält eine Applikation (Objektklassen aller 287 ART: Linie, Fläche, Text, Manuelle Stiche,
  Ellipse, Gruppe, Objekt, Ring). Eine Messung ist deshalb nicht möglich.

**Kern** (`applikation::herausfahr_punkt`, `nachbearbeitung::Zusatz::herausfahren`, Feld `herausfahren`):
- Vor jedem Farbwechsel innerhalb derselben Applikation kommt ein Sprung zum Punkt. Der Farbwechsel steht dann dort,
  danach folgt der Sprung zum nächsten Weg.
- Die Oberkante wird aus allen Stichwegen der Applikation berechnet (mit Abdeckband). Das entspricht `GetFullExtents`
  über alle Lagen.
- **Standard an wie im Original** (Nutzer 22.11.2026; zuerst als Option mit Standard aus umgesetzt), abschaltbar mit
  `"herausfahren": false`. Wie es an der 185 wirkt, zeigt Vor Ort V15.
- **Eigene Festlegung:** Liegt der Punkt außerhalb des Stickfelds, rückt er in 0,5-mm-Schritten bis an die
  Oberkante zurück. Ein Sprung aus dem Feld würde das Design für die Maschine größer machen als den Rahmen.
