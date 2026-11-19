# 05 – Effekte (Auto Spacing …) und Auto-Unterlage: wann das Original sie anwendet (19.11.2026)

**Ergebnis:**
- **Auto Spacing ist ein „Effekt“-Objekt**, das am Satin hängt (Klasse 0x21 mit der Tabelle, Faktor in 0x22). Hängt es
  dort, folgt der Abstand der Tabelle × Faktor, und der feste Abstand wird ignoriert. Fehlt es, gilt der feste Abstand.
- In den Stilen steht für jeden Effekt und jede Unterlage ein Flagwort. **Bit 0x2000 = aus**, **Bit 0x0001 = vorgewählte
  Art einer Liste**.
- Daraus folgt:
  - Auto Spacing ist **an** in Software Defaults, den 8 Stoffen und bei Schrift.
  - Die **Auto-Unterlage ist aus** in Software Defaults und den Stoffen, **an** bei Schrift. Die vorgewählte Art ist
    immer **Zickzack**.

Umgesetzt im Rust-Kern (Referenz-Stufe 5). Werkzeuge: `art_design.py`, `art_messen.py`. Musterdesigns wie in
[04](04_messungen_satin.md).

## 1. Effekte im Design — Sicherheit: hoch (Datenformat), mittel (Namen außer 0x21/0x23)
Im ART-Objektteil folgen die Effekte einer Stichart **hinter** deren Kennung 0x0d, als eigene Datensätze (mit Bit a,
spätere Objekte verweisen darauf). Beispiel: der eingebettete Stil jedes Musterdesigns, Satin.

| Klasse | Nutzdaten | Bedeutung |
|---|---|---|
| **0x21** | 11 × float Breite, 11 × float Abstand (1/180 mm), dahinter 7 float | **AutoSpacingEffect**: Breiten 0,01 / 0,5 / 1 / 1,5 / 2 / 3 / 4 / 5 / 6 / 7 / 8 mm → Abstände 0,54 / 0,51 / 0,49 / 0,47 / 0,46 / 0,42 / 0,40 / 0,37 / 0,36 / 0,34 / 0,34 mm – identisch mit der Stil-Tabelle 0x2776 (`Analyse/code/10` §A); in allen 66 Objekten mit Effekt gleich |
| **0x22** | float | **Abstandsfaktor** (0x2736): 1,0; bei Schrift-Objekten **0,9** |
| **0x23** | float 0,5, Byte 1, long 5, long 15, 15 × float | **ShortStitchesEffect** (Fabrik ruft MOD1 `ShortStitchesEffect::init`); entspricht Stil-Eintrag 0x273a (0x2819 = 0,5, 0x281a = 1, 0x281b = 5, Liste 0x281c mit 15 Werten) |
| 0x24 | float 30,0, Byte | offen (Winkel 30°?) |
| 0x26 | leer | offen |
| 0x6a | short, short, double 1,309 (75°), long 90, double 0,785 (45°), long 1, long 1440 (8 mm) | offen (Ecken/Teilung?) |

**Messung** (`art_messen.py effekte --alle`, Rohergebnis `Arbeitsdaten/messungen/effekte_musterdesigns.txt`, Auswertung wie [04](04_messungen_satin.md) §3, je Stich Verhältnis gemessen/erwartet):
| Satinobjekte | Stiche | Verhältnis zu Tabelle(Breite) × Faktor | Verhältnis zum festen Abstand |
|---|---|---|---|
| **mit** 0x21 | 20 012 | **0,99** (Quartile 0,88–1,05) | 0,08 (fester Wert steht auf 5,0 mm = ungenutzt) |
| ohne 0x21 | 166 330 | – | **0,97** (Quartile 0,92–1,00) |

Ob die Breite als Stichlänge oder senkrecht zur Säule gemessen wird, lässt sich so nicht trennen (0,987 gegenüber
0,982). Der Kern nimmt den Abstand der zugeordneten Seitenpunkte.

In den ART-Objekten heißt **vorhanden = aktiv**. Flower und die meisten älteren Muster haben keinen 0x21, also festen
Abstand. Schrift- und Sampler-Designs haben ihn.

## 2. Flags in den Stilen — Sicherheit: mittel-hoch
Effektliste 0x27ab und Unterlagenliste 0x27a7 je Stil (`Analyse/daten/entschluesselt/styles.txt`, Flagwort `f`):

| Stil | 0x2735 Auto Spacing | 0x273a Kurze Stiche | 0x273b Zugausgleich | Unterlagen 0x27a7 |
|---|---|---|---|---|
| 0 Software Defaults | 0x0001 **an** | 0x2001 aus | 0x0000 an | alle 0x2000, Zickzack 0x2001 → **aus**, Zickzack vorgewählt |
| 1–8 Stoffe | 0x0001 **an** | 0x2000 aus | 0x0000 an | wie Software Defaults → **aus** |
| Schrift | 0x0001 **an** | 0x0001 **an** | 0x0000 an | alle 0x0000, Zickzack 0x0001 → **an**, Zickzack |
| EasyStitch Narrow Objects | 0x0001 an | 0x2001 aus | 0x0000 an | aus, Zickzack |
| EasyStitch übrige | 0x2001 aus | 0x2001 aus | 0x0000 an | aus, Zickzack |
| Motiv-Stile | 0x2001 aus | 0x2001 aus | 0x2001 aus | 0x2002, Step 0x2003 → aus, Step vorgewählt |

**Deutung:**
- **Bit 0x2000 = aus.**
  - Bei den Effekten passt das überall: Kurze Stiche sind nur bei Schrift an. Zugausgleich ist bei Motiven aus, weil
    Motive ohne Zugausgleich gestickt werden.
  - In der Unterlagenliste tragen entweder alle Einträge das Bit oder keiner. Es ist also der Listen-Schalter
    „Underlay“.
- **Bit 0x0001** markiert die vorgewählte Art. In der Füllartenliste 0x27a6 gibt es kein Bit 0x2000, dort trägt genau
  die Vorwahl 0x0001 (Satin; Tatami bei EasyStitch Wide/Thin).

**Belege für die Unterlage:**
- **Handbuch** (PDF-S. 197–198): Unterlage kommt über das Werkzeug **„Auto Underlay“**: „Click the Auto Underlay icon
  … Click the Auto Underlay icon again to turn off the effect“. Im Dialog Effects > Others gibt es das Kästchen
  „Underlay“ und die Auswahl der Art.
- **Schrift** hat die Unterlage voreingestellt („Switching off auto underlay for small lettering … Click the Underlay
  icon. The underlay is removed.“, PDF-S. 186; Anhang Alphabete PDF-S. 379).
- **MOD1:**
  - `DesignDocument::SetUnderlayOn` (0x100660b0 → 0x101b8450) setzt die aktuelle Unterlage (+0xC, leer = aus) auf die
    vorgewählte (+0x10).
  - `Get/SetCurrentUnderlayType` (0x10066080/0x10066090) lesen bzw. setzen die Art.
  - `IsSecondUnderlayAllowed` (zweite Unterlage möglich, vgl. Flower: Mittellauf + Edge Walk).

## 3. Unterlagen in den Objekten — Sicherheit: hoch
Im Eigenschaftsbaum eines ART-Objekts stehen **nur die aktiven Unterlagen**, und genau diese werden gestickt
(Musterdesigns, Objekte mit passender Zuordnung):

| Objekte | Unterlagen im Baum = gestickt |
|---|---|
| 3 529 Satin | keine |
| 2 215 Tatami | keine |
| 1 366 Satin | Mittellauf |
| 195 Satin | Edge Walk + Mittellauf |
| 65 Tatami | Step |
| 61 Satin | Edge Walk |
| 25 Satin / 15 Tatami | Zickzack |
| 15 Tatami | Edge Walk + Step |

Ausnahmen: 16 sehr kurze Objekte mit Unterlage im Baum, aber ohne gestickten Lauf.

Die Musterdesigns zeigen damit, was die Digitalisierer gewählt haben, nicht die Voreinstellung. Auto-digitalisierte
Designs (EasyStitch) haben bei Tatami meist Step, bei Satin teils Zickzack, teils keine.

## 4. Umsetzung im Kern (Referenz-Stufe 5)
- `UnterlageEinstellung`:
  - `auto_an` (Standard **false**) und `auto_art` (Standard **["zickzack"]**).
  - Satin mit `"unterlage": "auto"` (Standard) folgt diesem Schalter.
  - Tatami ohne Angabe ebenso. Ist er an, kommt die Tatami-Unterlage „Step“; welche Art das Original für Flächen
    nimmt, wenn Zickzack vorgewählt ist, ist offen (eigene Festlegung).
- Die frühere eigene Breitenregel (< 1,5 mm keine … ab 6 mm Edge Walk + Zickzack) ist entfernt.
- Auto Spacing war im Kern schon Standard (`auto_spacing: true`) und entspricht den Stilen.

## 5. Offen
1. Bedeutung von 0x24, 0x26, 0x6a und der 7 float hinter der Tabelle in 0x21.
2. ~~Algorithmus der kurzen Stiche~~ → gefunden (MOD1 0x10186ed0) und umgesetzt, siehe [10](10_regeln_original.md) §2;
   der Effekt hängt auch an Bild-Objekten (Satin und Tatami), voreingestellt ist er nur bei Schrift.
3. Unterlagenart für Flächen, wenn Auto Underlay an ist (`IsUnderlayTypeApplicable`, MOD1 0x100938a0).
4. Bit 0x0002 (nur Motiv-Stile).
