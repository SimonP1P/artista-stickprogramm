# 13 – Füllmuster („Program Split“, Fancy Fill) im Original (20.11.2026)

**Quellen:** Vorlage NORMAL.AMT (entschlüsselter Stream `Analyse/daten/entschluesselt/NORMAL.AMT.DesignDocument.bin`),
gestickte Sampler „Sampler Fancy fills - 001-012 … 140-150.ART“, Symbole MOD1 (`SplitPattern`, `ProgramSplitFill`).
Werkzeuge: `Arbeitsdaten/werkzeuge/muster_lesen.py` (Bibliothek), `art_messen.py` (Sampler).

## 1. Wie die Füllung entsteht — Sicherheit: hoch (Messung)
Eine Musterfüllung ist eine Tatami-Fläche, deren Reihen **an den Kreuzungen mit den gekachelten Musterlinien
gestochen** werden; das Muster zeichnet sich aus den Einstichen ab. Gemessen im Sampler 001–012: Reihenabstand
**0,20 mm** (= Nachbarreihe von Tatami, Abstand 0,40 zwischen Hinreihen), kürzester Stich **0,40 mm** (Mindeststich am
Rand), lange Stiche innen bis etwa 7–9 mm. Stichwinkel z. B. 90° (Reihen senkrecht).

## 2. Musterbibliothek — Sicherheit: hoch (Format), geprüft an 4 Mustern
Gepackter Teil der Vorlage ab ≈ 0x82c00, Datensätze wie die ART-Objekte: je Muster Stilwerte, **GeoCurve (0x01)** mit
Bézier-Knoten = Linienzüge, **Name (0x28)** („001 Diamond (90)“ – Zahl in Klammern = Stichwinkel), Matrix (0x63),
**Gitter (0x3f)**: gepackte Wörter **Bx, By, Ax, Ay, Ursprung x, y** (1/180 mm), das Muster wiederholt sich um
i·A + j·B. Geprüft gegen den lesbaren DataItem-Teil der Vorlage (Gitter 0x3aab/0x3ab6/0x3ab7/0x3aba/0x3abb) bei
„027 Heart2“, „06 Brik“, „10 Shel“, „144 Dynasty“ (Ausdehnung und Gitter gleich). **242 Muster** (001 Diamond …
150, mit Varianten je Winkel). Daten: `kern/stick-kern/daten/fuellmuster.json` (mm, relativ zur Musterecke).

## 3. Umsetzung im Kern
`stick-kern::muster` (Muster laden, `Teilung::schnitte`), `sticharten::tatami::tatami_mit_teilung`; Objekt
`"art": "muster"` (Umriss, Löcher, Muster, Winkel – ohne Angabe aus dem Namen –, Größe %, Unterlage). Lücken über
7,0 mm werden gleichmäßig geteilt (**eigene Festlegung**, Sprunglänge der Vorlage 0x2712 = 1260). CLI `stick muster`.

## 4. Offen
Lage des Gitters zum Design (Ursprung der Kachelung; Original: Gitter-Ursprung im Muster, „Ursprung x/y“), genaue
Regel für lange Lücken (Werte 0x27c6–0x27cd im Stil 0x279a), Motivfüllung (MotifFill 0x3d) und Motivlinien.
