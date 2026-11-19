# 03 – Objektteil der ART-Designs (19.11.2026)

**Ergebnis:** Die Objekte der Musterdesigns lassen sich lesen: Objektklasse, Form (Bézier-Knoten), Stichart mit
Parametern, Unterlagen, Verbinder. Werkzeuge: `Arbeitsdaten/werkzeuge/art_objekte.py` (Datensatzrahmen,
Bézier-Knoten), `art_design.py` (Objekte mit Parametern). Prüfung: Flower 001 ergibt **210 Objekte** = Objektzahl in den
Design-Eigenschaften (Eigenschaft 7; HT116: 2 = 2); alle Datensätze lesen sich bis exakt ans Abschnittsende.

## 1. Lage im Datenblock — Sicherheit: hoch
Designlader IMPORT.DLL 0x100777d0: Objektabschnitt ab Kopf+0x38 (Offset) mit Länge Kopf+0x3c, gelesen über einen
Datensatzleser (0x100d9a60). Reihenfolge der Hauptteile: **0x29** eingebetteter Stil (Leser 0x10037a40) → **0x0a**
Objektliste (0x10032aa0 → 0x10033630) → **0x41**/**0x85** Garnbibliothek (Version ≤ 0x46 bzw. neuer) → **0x43** → **0x3a** …

## 2. Datensatzrahmen — Sicherheit: hoch (IMPORT.DLL 0x100d9b90 / 0x100d9bf0)
`gepackt u`: Klasse = u >> 1, Bit 0 = Verweis. Klasse 0 = Ende einer Ebene. Verweis: `gepackt v` = Nummer eines
früheren Datensatzes. Sonst `gepackt v`: Länge = v >> 2, Bit 0 = **a** (Datensatz wird für Verweise nummeriert,
0x1002d6c0 → 0x100e6850), Bit 1 = b; danach Länge Byte Nutzdaten. Verweisnummern zählen die a-Datensätze in
Lesereihenfolge ab 1 (geprüft an HT116: „0x18 → Verweis 7“ = siebter a-Datensatz, eine 0x18).

## 3. Klassen — Sicherheit: hoch (Tabellen), mittel (Namen ohne Konstruktor)
Verteiler 0x1002d6c0: Fabrik `[0x10191e48 + 4·Klasse]`, Leser `[0x101920c0 + 4·Klasse]`, Klassen 1–0x9d. Beide Tabellen
werden zur Laufzeit gefüllt: Leser in 0x10049da0 ff. (131 Einträge), Fabriken einzeln (`mov [Slot], Funktion`, 103).
Namen aus den Konstruktoraufrufen der Fabriken, Stil-Kennung aus `mov word [obj+0xc], Kennung` der Fabriken:

| Klasse | Name / Kennung | Klasse | Name / Kennung |
|---|---|---|---|
| 0x01 | GeoCurve (Form) | 0x14 | Verbinder 0x272a |
| 0x05 | EmbroideryCurve (Linie) | 0x16 | **Satin 0x2721** |
| 0x06 | EmbroideryPatch (Fläche/Säule) | 0x17 | Zickzack 0x2724 |
| 0x07 | ComplexFillGroup | 0x18 / 0x19 | **Lauf 0x271c / Dreifach 0x271f** |
| 0x0b | ManualObject | 0x1a | 0x2727 |
| 0x0c | InputCCurve (Satinsäule-Eingabe) | 0x1b | **Tatami 0x2714** |
| 0x2f / 0x7a / 0x7b | LetterObj / MotifText / LetterText | 0x1d / 0x1e / 0x1f / 0x20 / 0x65 | Unterlagen 0x2744 / 0x2745 / **0x2746 Edge Walk** / **0x2747 Mittellauf** / 0x275a Tatami |
| 0x36 / 0x37 / 0x38 | Ellipse / Ring / Stern | 0x33 | 0x277e |
| 0x3d / 0x91 | MotifFill | 0x0d | Kennung (short) |
| 0x49 / 0x95 | EmbroideryGroup / ButtonHole | 0x0e | Listenknoten: Anzahl (Byte) + Basis 0x0d |
| 0x93 / 0x72 / 0x73 | Applique / AutoAppliqueGroup / AppliqueCurve | 0x10 | Unterlagen-Basis |
| 0x74 | EdgeZigZagFill | 0x12 / 0x15 | gemeinsame Stichparameter / Abstand |

Vollständige Listen: `F:\Entwicklung\art_analyse\{leser_tabelle,fabrik_namen,klassen_tags}.txt` (Arbeitsstand).

## 4. Vererbung im Strom — Sicherheit: hoch
Abgeleitete Klassen lesen ihre Felder, dann den **nächsten** Datensatz als Basisklasse. Beispiele (Feldfolge aus den
Lesefunktionen, Länge an den Daten geprüft):
- **Satin** 0x16: long Auto-Split (1800 = 10 mm, = 0x2722), Byte Auto Spacing → 0x15: long **Abstand** (z. B. 72/76/90)
  → 0x12: long Höchststich (0x2711), long Sprunglänge (0x2712), long (Bedeutung offen), 2 Byte → 0x0d: Kennung 0x2721.
- **Lauf** 0x18 (14 Byte): long **Länge** (0x271d), Byte variabel (0x271e), long Sehnenabstand (0x2768), Byte
  Durchgänge (0x275e), long Mindeststich (0x2767) → 0x12 → 0x0d 0x271c. (Neuere Version zusätzlich ein double.)
- **Tatami** 0x1b (34 Byte): long Stichlänge (0x2717), long Mindeststich am Rand (0x2718), double ×3 Versatz
  (0x2719/0x271a/0x271b), 2 Byte → 0x15 Abstand (0x2716) → 0x12 → 0x0d 0x2714.
- **Unterlage** (0x1d/0x1f/0x20/0x65) → 0x10: long, double **Winkel** (π/2), long, long, long → 0x0e → 0x0d.
- **Verbinder** 0x14: 5 long, 5 ushort → 0x13 → 0x12 → 0x0d 0x272a (Feldzuordnung offen).

Die Listenknoten 0x0e/0x0f bilden **denselben Baum wie die Stile in NORMAL.AMT** nach: je Objekt 0x2741 → Verbinder
0x272a, Unterlagen 0x2746/0x2747, Füllung 0x2748 (Stichart + Laufweg 0x271c).

## 5. Formen — Sicherheit: hoch (IMPORT.DLL `SerializeKnots` 0x1000fc20)
GeoCurve 0x01: gepackt (Bit 0 = geschlossen), gepackt Anzahl, je Knoten Kennbyte (Bit 0 Anfasser ein, Bit 1 Anfasser
aus, Bit 2 Knotenart) und vorzeichenbehaftete gepackte Wörter (WESSYS `ReadWordPacked`, ZigZag) relativ zum zuletzt
gelesenen Punkt. Einheit 1/180 mm, **y einfach** (nicht verdoppelt wie im Stichteil). Prüfung HT116: Kurvenenden =
Ankerpunkte der Stiche. Ein EmbroideryPatch (Satinsäule) hat zwei Kurven = die zwei Seiten.

## 6. Offen
1. Welche der im Objekt aufgeführten Unterlagen **aktiv** ist (beide Edge Walk und Mittellauf stehen in jedem
   Flower-Objekt) – Schalterbits suchen.
2. Zuordnung Objekt ↔ Stichbereich (Reihenfolge der Objekte = Stickreihenfolge? Segmentattribute).
3. Bedeutung der offenen Felder (0x12 drittes long, 0x10, 0x14).
