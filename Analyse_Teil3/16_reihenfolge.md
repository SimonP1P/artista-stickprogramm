# 16 – Reihenfolge der Objekte, Ein- und Ausstich (20.11.2026, ROADMAP 4b.5)

**Ergebnis:** Das Original ordnet Objekte nur auf Anweisung um (Menü *Arrange*): wie ausgewählt, an den Anfang, ans
Ende, nach Farbe, vor/nach einer Nummer. Eine **Prüfung auf Überdeckungen gibt es im Original nicht**. Die Prüfung im
Kern ist eine eigene Erweiterung. Ein- und Ausstich hat jedes Objekt; ohne Eingriff stickt das Original **wie
digitalisiert** (gemessen). „Closest Join“ legt Aus- und Einstich zweier Objekte auf die nächsten Punkte ihrer Umrisse
(Code). Kern: `stick_kern::reihenfolge`, `stick_kern::ein_ausstich`, CLI `stick reihenfolge`; Messwerkzeug
`Arbeitsdaten/werkzeuge/art_reihenfolge.py`.

## 1. Befehle des Originals – Sicherheit: hoch (Handbuch, Ressourcen)
Handbuch Kap. 14 „Resequencing embroidery objects“ (PDF-S. 130–133, `man_raw_pg.txt` Zeile 7734 ff.); Menü-IDs aus
BERESENU.DLL (Menü 5/129 *Arrange*, Kontextmenü 700 der Color Film), gelesen mit pefile:

| Befehl | ID | Wirkung laut Handbuch |
|---|---|---|
| Sequence As Selected | 0x8163 | Objekte in der Reihenfolge, in der sie mit Strg angeklickt wurden |
| Sequence to Start / to End | 0x81A9 / 0x81AA | ausgewählte Objekte an den Anfang bzw. ans Ende |
| Sequence By Color… | 0x81AB | Dialog mit den Farben der ausgewählten Objekte, *Move Up/Down*, OK – „reduces the number of color changes“ |
| Color Film | 0x826F | Liste der Farbblöcke, Umordnen durch Ziehen |
| Resequence by Number… | 0x824C | DLG 1280: *Before/After the object #* |
| Auto Sequence | 0x8270 | nur im Kontextmenü der Color Film; im Handbuch nicht beschrieben |
| Start and End… | 0x8351 | Start/Ende des **Designs** (schon im Kern, Bericht 10 §6) |

„Automatic sequencing“ im Handbuch (S. 133) ist **Blackwork Run** (Craft Stitches): überlappende Linien werden zu
einem Objekt mit einem Ein- und Ausstich verbunden → ROADMAP 4b.9, nicht Teil von 4b.5.

## 2. Code – Sicherheit: hoch (MOD1/GEOMETRY), offen (ARTISTA.EXE)
Die Menübefehle stehen in **ARTISTA.EXE**, und das ist verschlüsselt (HASP-Hülle: `.text` Entropie 8,0, Abschnitt
`.protect`, nur ein Import aus MOD1). Lesbar sind die Bausteine in MOD1.DLL und GEOMETRY.DLL:

- **`DesignDocument::Resequence(SharedMObjectList&, bool, bool, bool)`** MOD1 0x1006db20: braucht ≥ 2 Objekte; sucht
  das erste Objekt der Liste im Design (`ObjectDesign::IsIn`), kopiert die Liste **in Listenreihenfolge**
  (`CopySelection`), wahlweise `UngroupObjects` und **`ResequenceByClosestJoin`** (zweiter bzw. dritter Schalter),
  fügt die Kopie an der Stelle des ersten Objekts ein (`PasteDocument`) und löscht die Originale (`DeleteObject`).
  → „Sequence As Selected“: Block an der Stelle des **ersten ausgewählten** Objekts.
- **`DesignDocument::ResequenceByClosestJoin`** MOD1 0x1006d0c0: ordnet **nicht** um. Für jedes Paar
  aufeinanderfolgender Objekte: Umrisse beider holen (virtuelle Funktion +0x30, `DesignCurveShrPtrList::Convert`),
  **`FindExitEntry`** → `ChangeExitPointTransaction` (erstes Objekt) und `ChangeEntryPointTransaction` (zweites),
  danach `ProcessObject` (Stiche neu). Hat nur eines einen Umriss: `Distance(Umriss, Punkt)` zum End- bzw. Startpunkt
  des anderen → nur dessen Ein- bzw. Ausstich. Aufrufer: Resequence (Schalter) und 0x1005a660 (Paare, deren Nummern
  nicht aufeinander folgen – nach der automatischen Zerlegung).
- **`FindExitEntry(GeoRect, GeoRect, CurveList*, CurveList*, LongPoint&, LongPoint&)`** GEOMETRY 0x10056544: ruft
  `Distance(CurveList, CurveList, Pos&, Pos&, DBL_MAX)` → **exakt kleinster Abstand** der beiden Umrisse; Ausstich =
  Punkt auf A, Einstich = Punkt auf B (gerundet auf ganze Einheiten 1/180 mm).
- Eine Funktion, die Überdeckungen prüft oder Objekte automatisch nach Farbe ordnet, gibt es in den lesbaren DLLs
  nicht (Suche nach Exporten/Strings: `Resequence`, `Sequence`, `Overlap` – nur Applikation `AppliqueOverlapProcessor`
  und Konvertierungswerte u. a.). Wie „Sequence By Color“ den Block platziert, ist im verschlüsselten Teil.

## 3. Messung an den 287 Musterdesigns – Sicherheit: mittel-hoch
Werkzeug `art_reihenfolge.py` (Objekte ↔ Stichblöcke wie `art_messen.py`). Der Einstich selbst steht nicht in den
Stichdaten (Ende des Verbinders); der erste Stich liegt eine Stichlänge weiter → Toleranz 3 mm. Ausstich = letzter
Stich ohne Vernähstich.

| Frage | Ergebnis |
|---|---|
| offene Laufstich-Linien (9 650) | **75 % vom ersten zum letzten Knoten**, 13 % rückwärts (darunter Double-Run-Kopien, die rückwärts gestickt werden – Beispiel HT102), 12 % anders |
| geschlossene Linien (153) | Einstich **am ersten Knoten 88 %**, Ausstich = Einstich (ganze Runde) **80 %** |
| Flächen mit Tatami (3 521) | Einstich **am ersten Knoten 69 %**, auf dem Rand 61 %; Ausstich fast nie am Einstich (7 %), 60 % im Inneren |
| Closest Join zwischen getrennten Objekten | siehe §3a |

**Folgerung:** Ohne Eingriff gilt „wie digitalisiert“ – der erste gezeichnete Punkt ist der Einstich, offene Linien
laufen in Zeichenrichtung, geschlossene schließen am Anfang. Davon abweichende Ein-/Ausstiche hat der Digitalisierer
gesetzt (Reshape) oder Closest Join/Auto Digitizer.

### 3a. Closest Join in den Musterdesigns
Je zwei aufeinanderfolgende Objekte (ohne Schrift): Abstand Ausstich → Einstich gegen den kleinsten Abstand der beiden
Umrisse. Gewertet nur Paare, deren Stiche zu ihren Formen passen (Ausstich ≤ 0,5 mm vom eigenen Umriss, Einstich ≤ 3 mm)
– bei 11 974 Paaren liegt der Ausstich im Inneren (Flächen) oder Form und Stiche gehören zu verschiedenen Lagen.

| Paare | Anzahl |
|---|---|
| Umrisse berühren sich (< 0,3 mm) | 8 422 |
| getrennt (≥ 0,3 mm), gewertet | 474 |
| davon Verbindung ≤ kleinster Abstand + 3 mm | **453 (96 %)** |
| davon ≤ 2 × kleinster Abstand + 3 mm | 13 |
| davon deutlich länger | 8 |

Verbindung minus kleinster Abstand: Median **0,96 mm** (10 %: 0,06, 90 %: 2,02) – weniger als eine Stichlänge.
**Folgerung:** Zwischen getrennten Objekten liegen Aus- und Einstich in den Musterdesigns fast immer auf dem kürzesten
Weg – so digitalisiert oder mit Closest Join gesetzt. Im Kern entspricht dem `kuerzeste_verbindung: true`; Standard
bleibt „wie gezeichnet“ wie im Original. Für Importe ohne gezeichnete Reihenfolge (SVG, Bild → Umrisse) bietet sich
Closest Join an, wie beim Auto Digitizer (Handbuch Kap. 13).

## 4. Umsetzung im Kern
**Umordnen** (`stick_kern::reihenfolge`, arbeitet auf dem Design-JSON, Bezüge zwischen Objekten laufen über Namen):

| Kern | Original | Festlegung |
|---|---|---|
| `wie_ausgewaehlt` | Sequence As Selected | Block in Auswahlfolge an die Stelle des ersten ausgewählten Objekts (MOD1 Resequence, hoch) |
| `an_anfang`, `ans_ende` | Sequence to Start/End | mehrere Objekte behalten ihre Reihenfolge untereinander (eigene Festlegung, Handbuch beschreibt eines) |
| `nach_nummer` | Resequence by Number | vor/nach Objekt Nr. (Reihenfolge der Auswahl untereinander bleibt, eigene Festlegung) |
| `nach_farbe` | Sequence By Color | Farbfolge = gewünschte Farben, übrige in der Reihenfolge ihres ersten Auftretens; innerhalb einer Farbe bleibt die Reihenfolge; Block an der Stelle des ersten ausgewählten Objekts (**eigene Festlegung** wie Resequence – Befehl verschlüsselt) |
| `ueberdeckungen`, `vertauscht` | – | **eigene Erweiterung**: Paare, deren belegte Flächen sich überdecken (≥ 0,5 mm²; Laufstich als Band 0,5 mm; Texte mit Buchstaben, Aussparen berücksichtigt); Warnung, wenn ein Objekt nach dem Umordnen unter einer Fläche liegt, über der es lag |
| `nach_farbe(…, ueberdeckung)` | – | **eigene Erweiterung**: Objekte, die sich überdecken, behalten ihre Reihenfolge; sonst wird die laufende Farbe fortgesetzt, dann die nächste nach Rang (topologische Sortierung) |

**Ein-/Ausstich** (`stick_kern::ein_ausstich`, JSON `einstich`/`ausstich` am Objekt, `kuerzeste_verbindung` am
Design): ohne Angabe wie bisher (wie gezeichnet; Flächen beginnen am Punkt nächst dem vorigen Ende – die JSON-Formen
haben keinen gezeichneten ersten Punkt, eigene Festlegung). Mit Angabe (Umsetzung je Objektart **eigene Festlegung**,
weil der Kern keinen frei gelegten Ausstich in Flächen kennt):
- offene Linie, Satinsäule, mittige Satinlinie/Blanket: Richtung so, dass Anfang nahe Einstich und Ende nahe
  Ausstich (Ein-/Ausstich liegen wie im Original an den Enden);
- geschlossene Linie/Satinlinie/Blanket: Beginn und Ende am Umrisspunkt nächst dem Einstich (sonst Ausstich), dort
  ein neuer Kurvenknoten – entspricht 80 % „Ausstich = Einstich“ der Messung;
- Flächen, Buchstaben: Beginn nächst dem Einstich; nur Ausstich: Beginn am Umrisspunkt, der am weitesten entfernt
  ist (die Reihen enden auf der Seite des Ausstichs);
- seitlich versetzte Satinlinie/Blanket werden nicht umgedreht (Richtung bestimmt die Seite).

**Closest Join** (`kuerzeste_verbindung: true`): jedes Objekt ohne eigenen Einstich beginnt am Punkt nächst dem
vorigen Ende (Linien umgedreht, geschlossene Formen gedreht, Flächen wie bisher); Schrift ausgenommen (Reihenfolge
der Buchstaben und Säulen nach Bericht 11). Unterschied zum Original: FindExitEntry legt auch den **Ausstich** des
vorigen Objekts zum nächsten hin; der Kern wählt nur den Einstich (gierig) – für offene Linien ist das gleich, für
geschlossene Formen bliebe sonst ein doppelt gestickter Bogen.

## 5. Offen
1. Wie „Sequence By Color“ den Block platziert, wenn nur ein Teil ausgewählt ist (verschlüsselt; Kern: Stelle des
   ersten ausgewählten Objekts).
2. Ausstich in Flächen wie das Original (Reihen so gelegt, dass die Fläche am Ausstich endet) – Verfahren im Füll-
   generator suchen (MOD1, Fill-Sequencing), wenn nötig.
3. „Auto Sequence“ (0x8270) der Color Film: Bedeutung unklar (vermutlich Resequence mit Closest Join).
