# 10 – Algorithmen für schmale Bahnen, Auto Spacing, EasyStitch, Stoffe

**Quelle:** entschlüsselte Vorlage `TEMPLATE/NORMAL.AMT` (Stream `DesignDocument`, siehe [05](05_art_amt_verschluesselung.md)) mit 22 benannten Stilen; Werkzeuge [`style_keyvalues.py`](style_keyvalues.py), [`fabrics.py`](fabrics.py), [`styles_compare.py`](styles_compare.py). Rohdaten: [`../daten/entschluesselt/`](../daten/entschluesselt/)
**Sicherheit:** hoch für die Zahlen (direkt aus der Datei). Bei der Interpretation steht die Sicherheit jeweils dabei. Die Rechenvorschrift (Interpolation) ist nicht im Code verfolgt, sondern aus der Tabellenform geschlossen.

Einheit überall: 1 mm = 180 interne Einheiten.

## A) Satin: automatischer Stichabstand nach Säulenbreite („Auto Spacing“)
Eine Tabelle in jedem Stil (`0x2734/0x27ab/0x2735/0x2776`) ordnet der **Säulenbreite** (`0x2737`) einen **Stichabstand** (`0x2738`) zu. Werte aus „0 Software Defaults“:

| Säulenbreite | Stichabstand | Stiche pro mm Säulenlänge (beide Seiten zusammen) |
|---|---|---|
| ≈ 0 mm (0,01) | 0,54 mm | 1,9 |
| 0,5 mm | 0,51 mm | 2,0 |
| 1,0 mm | 0,49 mm | 2,0 |
| 1,5 mm | 0,47 mm | 2,1 |
| 2,0 mm | 0,46 mm | 2,2 |
| 3,0 mm | 0,42 mm | 2,4 |
| 4,0 mm | **0,40 mm** | 2,5 |
| 5,0 mm | 0,37 mm | 2,7 |
| 6,0 mm | 0,36 mm | 2,8 |
| 7,0 mm | 0,34 mm | 2,9 |
| 8,0 mm | 0,34 mm | 2,9 |

**So wirkt es (Interpretation, Sicherheit mittel-hoch):**
1. Die Software misst die Breite der Satinsäule an jeder Stelle.
2. Sie liest den Abstand aus der Tabelle ab, zwischen den Stützstellen vermutlich linear interpoliert.
3. Der Abstand wird mit dem **Stoff-/Stilfaktor** (`0x2736`) multipliziert. Faktor > 1 = lockerer, < 1 = dichter (vgl. Dialog „Adjust Settings To … %“, Handbuch S. 202).
4. **Je schmaler die Säule, desto größer der Abstand** (0,51 mm bei 0,5 mm Breite statt 0,40 mm bei 4 mm). Das deckt sich mit dem Handbuch: „Where a column is very narrow, stitches need to be less dense because too many needle penetrations can damage the fabric“ (S. 68/72).
5. Der feste Satin-Abstand bei ausgeschaltetem Auto Spacing ist 0,40 mm (`0x2723`), also genau der Tabellenwert bei 4 mm.

**Faktoren je Stil (`0x2736`):**
| Stil | Faktor | Wirkung bei 1 mm breiter Säule (0,49 mm × Faktor) |
|---|---|---|
| 0 Software Defaults | 1,00 | 0,49 mm |
| 1 Knit Light Weight | 1,08 | 0,53 mm |
| 2 Knit Medium Weight | 1,10 | 0,54 mm |
| 3 Knit Heavy Weight | 1,00 | 0,49 mm |
| 4 Terry Medium Weight | 1,05 | 0,51 mm |
| 5 Terry Heavy Weight | 0,95 | 0,47 mm |
| 6 Woven Light Weight | 1,05 | 0,51 mm |
| 7 Woven Medium Weight | 1,08 | 0,53 mm |
| 8 Woven Heavy Weight | 1,13 | 0,55 mm |
| **&lt;PRESET_LETTERING&gt; (Schrift)** | **0,90** | **0,44 mm** (Schrift wird 10 % dichter gestickt) |

Eine zweite Werteliste in der Tabelle (`0x2739`, Defaults: +0,01/+0,03/−0,03/−0,06 mm …; je Stoff ±0,02 mm) ist ein kleiner Korrekturwert. **Bedeutung unklar.**

## B) Satin: weitere Regeln für schmale/lange Säulen
| Regel | Wert | Quelle | Sicherheit |
|---|---|---|---|
| Auto Jump: Satinstiche länger als **12,7 mm** werden in Sprünge zu **7 mm** zerlegt | 12,7 / 7,0 mm | `0x2711/0x2712`, Handbuch S. 203–204 | hoch |
| Satin ab **10 mm** Länge wird geteilt (Auto-Split) | 10,0 mm (`0x2722`) | Struktur | niedrig-mittel |
| Kürzester erlaubter Stich im Bernina-Format | **0,3 mm** | DEFAULTS.DFL | hoch |
| Zugausgleich Unterlagen | **0,20 mm** | `0x2743`, Handbuch | hoch |
| Zugausgleich (Gruppe `0x273b/0x273d`) | Defaults 0,17 · Knit/Woven Light 0,20 · übrige Stoffe 0,25 · Schrift 0,20 · EasyStitch-Flächen **0,40 mm** | Struktur; passt zum Handbuch-Hinweis „Use 0.4 when using automatic digitizing“ | mittel |
| Unterlagenrand | 0,20 mm (`0x2769/0x276a`) | Handbuch S. 199–200 | mittel |

## C) EasyStitch: Stichart nach Objektbreite wählen (Automatisches Digitalisieren)
Jeder EasyStitch-Stil enthält dieselbe Klassifikationstabelle (`0x3f54`):

| Objekt | bis Breite | Stichart (Preset) |
|---|---|---|
| Linie | ≤ **0,3 mm** (54) | `<PRESET_RUN_1>` (Laufstich) |
| Linie | ≤ **2,0 mm** (360) | `<PRESET_RUN_2>` (kräftigere Linie) |
| Fläche | ≤ **8,0 mm** (1440) | `<PRESET_SATIN_1>` (**Satin**) |
| Fläche | ≤ **12,0 mm** (2160) | `<PRESET_TATAMI_1>` |
| Fläche | darüber (500 000) | `<PRESET_TATAMI_2>` |

Weitere Schwellen in derselben Tabelle: Mindestfläche 1 mm² (`0x3f57` = 32 400), 5 mm (`0x3f58`), 0,2 mm (`0x3f5a`), 1 mm (`0x3f63`), 3 mm / 1 mm / 12 mm (`0x3f99/9a/9b`), Fläche 0,83 mm² (`0x3f69` = 27 000). Deren genaue Rolle (Glättung, Mindestgröße, Kurvenabweichung) ist **nicht bestimmt**.

**Daraus folgt (Sicherheit hoch):** Die Software macht Flächen bis **8 mm** Breite zu Satin. Unter 0,3 mm entsteht eine einfache Linie, bis 2 mm Breite eine kräftige Linie statt Satin.

## D) Werksvorgaben „0 Software Defaults“ (alle Stile in `style_keyvalues_all.txt`)
| Stichart | Wert | Software Defaults | Schrift (PRESET_LETTERING) |
|---|---|---|---|
| Satin | Abstand fest | **0,40 mm** | 0,40 mm × 0,9 (Auto) |
| Satin | Auto Jump | ein, 12,7 / 7,0 mm | **aus** |
| Step/Tatami | Reihenabstand | **0,40 mm** | 0,40 mm |
| Step/Tatami | Stichlänge | **4,0 mm** | 4,0 mm |
| Step/Tatami | Mindeststich am Rand | 0,40 mm | **0,50 mm** |
| Step/Tatami | Reihenversatz | 0,25 / 0,25 | 0,25 / 0,25 |
| Laufstich (Kontur, `0x27a8`) | Länge | **2,5 mm**, feste Länge, min. 0,8 mm | 2,5 mm |
| Laufstich in Flächen (Wege) | Länge | 1,8 mm, variabel, min. 0,4 mm, Chord Gap 0,09 mm | 2,5 mm |
| Zickzack-Füllung | | 2,0 / 3,0 mm | 1,5 / 3,0 mm |
| Fancy-Füllung | Abstand / Länge | 0,40 / 4,0 mm | gleich |
| Verbinder | Trim ab | **3,0 mm** (Schrift-Verbinder 2,0 mm) | |
| Verbinder | Vernähen, wenn nächster/vorheriger länger als | 3,0 mm | |

**Unterlagen (`0x27a7`, alle Stile gleich, Zugausgleich jeweils 0,20 mm):**
| Unterlage | Werte |
|---|---|
| Edge Walk (`0x2746`) | Laufstich **2,2 mm** (variabel, min. 0,73 mm) |
| Zickzack (`0x2744`) | Zickzack 5,0 / 3,0 mm (max. Stich 5,0 mm) + Lauf 3,0 mm |
| Doppel-Zickzack (`0x2745`) | Zickzack 7,0 / 3,0 mm + Lauf 3,0 mm |
| Mittellauf (`0x2747`) | Laufstich 2,2 mm |
| Tatami-Unterlage (`0x275a`) | Abstand **3,0 mm**, Länge 4,0 mm + Randlauf 2,5 mm |
Die Zuordnung der Kennungen zu den Namen „Edge Walk / Zickzack / Doppel-Zickzack“ ist aus dem Handbuch (S. 198–199, Beispiellängen 1,5–3,5 / 2,5–5 / 3,5–7 mm) geschlossen. **Sicherheit: mittel.**

## E) Stoffe (Settings → Fabric Settings) — vollständige Liste und Änderungen
Die 8 Stoffe sind vollständige Stile. Gegenüber den anderen Stoffen ändern sich **nur** diese Werte ([`fabrics_diff.txt`](../daten/entschluesselt/fabrics_diff.txt)):

| Stoff | Satin-Abstand fest | Tatami-Abstand | Auto-Spacing-Faktor | Zugausgleich (0x273d) | Verbinder-Trim |
|---|---|---|---|---|---|
| 1 Knit Light Weight | 0,43 mm | 0,45 mm | 1,08 | 0,20 mm | 6,0 mm |
| 2 Knit Medium Weight | 0,44 mm | 0,45 mm | 1,10 | 0,25 mm | 6,0 mm |
| 3 Knit Heavy Weight | 0,38 mm | 0,40 mm | 1,00 | 0,25 mm | 6,0 mm |
| 4 Terry Medium Weight | 0,42 mm | 0,42 mm | 1,05 | 0,25 mm | 6,0 mm |
| 5 Terry Heavy Weight | 0,38 mm | 0,38 mm | 0,95 | 0,25 mm | 6,0 mm |
| 6 Woven Light Weight | 0,42 mm | 0,45 mm | 1,05 | 0,20 mm | 6,0 mm |
| 7 Woven Medium Weight | 0,43 mm | 0,40 mm | 1,08 | 0,25 mm | 6,0 mm |
| 8 Woven Heavy Weight | 0,38 mm | 0,38 mm | 1,13 | 0,25 mm | 6,0 mm |
| *(0 Software Defaults)* | 0,40 mm | 0,40 mm | 1,00 | 0,17 mm | 3,0 mm |

Bei **allen** 8 Stoffen gilt außerdem (gleich für alle, aber anders als in den Software Defaults): Zickzack- und Doppel-Zickzack-Unterlage 3,0/3,0 mm statt 5,0 bzw. 7,0 mm, Tatami-Reihenversatz 0,25/0,75 statt 0,25/0,25, Verbinder-Trim ab 6,0 mm statt 3,0 mm. Die Schaltfläche „Adjust Settings To X %“ skaliert die Abstände zusätzlich (Handbuch S. 202). Mit „Apply to Lettering Objects“ gilt das auch für Schrift.

## F) Was das für kleine, filigrane Stickerei bedeutet
1. **Satin-Mindestbreite:** Die Software kennt keine harte Untergrenze für Satin. EasyStitch macht Objekte **unter 2 mm Breite nicht zu Satin**, sondern zu Linien (≤ 0,3 mm einfacher Lauf, ≤ 2 mm kräftige Linie). Die Auto-Spacing-Tabelle beginnt bei 0,5 mm Breite. **Praktische Untergrenze für Satin laut Software-Logik: etwa 0,5 mm, bei automatischer Umwandlung 2 mm.**
2. Je schmaler die Säule, desto lockerer: 0,51 mm Abstand bei 0,5 mm Breite.
3. Kleine Schrift: Auto-Unterlage abschalten (Handbuch S. 186 und 379: Anniversary ist ab 5 mm stickbar, die Unterlage würde sonst herausschauen). Mindesthöhen je Alphabet: [`../daten/alphabets.md`](../daten/alphabets.md). Am kleinsten sind **Quilting Text und Roman Small mit 4–6 mm**, Prestige ab 5 mm, Chicago/Drifter/Fortress/Nightowl/Swiss Block/Varsity Block/Zurich ab 6 mm.
4. Schrift wird 10 % dichter gesetzt (Faktor 0,9) und ohne Auto Jump gestickt.
5. Laufstich an engen Kurven auf **1,8 mm** verkürzen (Handbuch S. 68); die Software nutzt 1,8 mm selbst für Wege in Flächen.
6. **Feines Garn (60er) und kleine Nadeln:** In der gesamten Software (Handbuch, Garnkarten, Vorlagen) kommt **keine Angabe** dazu vor. Alle Garnkarten sind 40er (Isacord 40, Madeira 40, Mettler, Sulky …) bzw. Quilt-Garne 25/50 wt. Die Garnkarten haben ein Feld `thrd_density` (Wert „A“ oder „B“), das ist aber keine Stärkeangabe.
