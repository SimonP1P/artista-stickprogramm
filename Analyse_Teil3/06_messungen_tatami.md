# 06 – Tatami (Step): Reihenabstand und Versatz wie im Original (19.11.2026)

**Ergebnis:**
- Der Tatami-Abstand (0x2716, Software Defaults 0,40 mm) ist der Weg zwischen zwei **Hinreihen**. Benachbarte Reihen
  (hin/zurück) liegen also beim **halben Abstand**. Belegt durch Handbuch und Messung.
  - Der Kern hatte die Reihen bisher beim ganzen Abstand gelegt, Tatami war damit halb so dicht wie im Original.
  - Behoben (Referenz-Stufe 7).
- Der Versatz 0x2719/0x271a = 0,25/0,25 verschiebt das Einstichraster **je Reihe um 0,25 Stichlänge**: Nachbarreihe
  0,25, übernächste Reihe 0,50. So rechnet der Kern schon. Die bisherige Deutung (mittel) ist damit bestätigt (hoch).

Werkzeug: `Arbeitsdaten/werkzeuge/art_messen.py tatami --alle`. Rohergebnis:
`Arbeitsdaten/messungen/tatami_musterdesigns.txt`. Grundlage: Zuordnung Objekt ↔ Stiche wie in
[04](04_messungen_satin.md) §1.

## 1. Tatami-Parameter im Design (Klasse 0x1b, 34 Byte) — Sicherheit: hoch
long Stichlänge (0x2717), long Mindeststich am Rand (0x2718), 3 × double Versatz (0x2719, 0x271a, 0x271b), 2 Byte.
Basis 0x15: long Abstand (0x2716). Häufige Werte in den Musterdesigns: Stichlänge 3,8 / 4,0 / 5,0 mm, Abstand 0,40 mm,
Versatz (0,25, 0,25, 0,1) oder (0, 0, 0). Software Defaults: 4,0 mm / 0,40 mm / 0,25 / 0,25 / 0.

## 2. Reihenabstand — Sicherheit: hoch (Handbuch + Messung)
**Handbuch** PDF-S. 74, „Adjusting Step fill spacing“: „The spacing setting is the distance between two **forward
rows**.“ Ebenso: „This value is the distance between each forward row of stitching.“

**Messung:**
- Je Tatami-Stichblock wird die Reihenrichtung aus den Stichen voller Stichlänge bestimmt. Die Reihen werden waagerecht
  gedreht und als Folge gleichgerichteter Stiche (± 4°) erkannt.
- Je Reihe kommt eine Ausgleichsgerade dazu; gemessen wird ihr Abstand in der gemeinsamen Mitte.

| Reihenpaar | Anzahl | gemessen / eingestellter Abstand |
|---|---|---|
| Nachbarreihe (hin → zurück) | 10 256 | **0,490** (Quartile 0,47–0,56) |
| übernächste Reihe (hin → hin) | 10 256 | **0,980** (Quartile 0,96–1,05) |

Das gilt gleich für alle Abstände in den Mustern (0,30 / 0,40 / 0,42 / 0,44 / 0,45 / 0,65 mm, einzeln ausgewertet:
0,46–0,49).

## 3. Versatz der Einstiche — Sicherheit: hoch für 0,25/0,25
Phase der inneren Einstiche einer Reihe gegen das Raster der Stichlänge (Kreismittel), verglichen mit der
Nachbarreihe und der übernächsten Reihe:

| Versatz im Objekt | Nachbarreihe | übernächste Reihe |
|---|---|---|
| (0,25, 0,25) – 1 229 Reihen | **0,24–0,27: 65 %** | **0,47–0,50: 75 %** |
| (0, 0) – 8 469 Reihen | 0,25: 15 %, sonst gestreut | 0,50: 12 %, sonst gestreut |

Bei (0,25, 0,25) verschiebt sich das Raster also je Reihe um ¼ Stichlänge, nach vier Reihen wiederholt es sich.
Die älteren Muster mit (0, 0) zeigen kein festes Raster. Ob dort der dritte Wert (0x271b) oder eine andere Einstellung
wirkt, ist offen.

## 4. Umsetzung im Kern (Referenz-Stufe 7)
- `TatamiEinstellung::reihenabstand()` = Abstand / 2; `tatami.rs` legt die Reihen in diesem Abstand.
- Die Tatami-Unterlage (Step, Abstand 3,0 mm) läuft über dieselbe Funktion: Nachbarreihen 1,5 mm.
- Tests:
  - `reihen_beim_halben_abstand`: Rechteck, Nachbarreihen 0,20 mm.
  - `rechteck_reihen_und_grenzen`: 10 mm Höhe ergibt 50 statt 25 Reihen.

## 5. Step-Muster 1–20 (20.11.2026) — Sicherheit: hoch
**Dialog** „Fill Stitch“ (BERESENU DLG 1190): Gruppe „Step pattern“ mit Knöpfen 1–20. **Code** DIALOG.DLL
0x1006f0f0 ff.: liest 0x2719, 0x271a, 0x271b des Objekts und sucht sie in drei Tabellen zu je 25 doubles (0x1017fdb0,
0x1017fe78, 0x1017ff40); bei Treffer wird die Musternummer markiert, sonst gilt das Muster als eigenes. Die
Stichlänge gehört **nicht** zum Muster.

| Nr. | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x2719 | 0,25 | 0,33 | 0,5 | 0,1 | 0,2 | 0,1 | 0 | 0 | 0,3 | 0,2 | 0,5 | 0,5 | 0,9 | 0,6 | 0,9 | 1,0 | 0,15 | 0 | 0,5 | 0,3 |
| 0x271a | 0,25 | 0,33 | 0,5 | 0,1 | 0,4 | 0,4 | 0,25 | 0,5 | 0,6 | 0,6 | 0,25 | 0,15 | 0,15 | 0,6 | 0,9 | 0,25 | 0,15 | 0 | 0 | 0,3 |

0x271b ist in allen Voreinstellungen 0. (Die Tabellen haben 25 Einträge; 21–25: 0,6/0,3 · 0,6/0,5 · 0,7/0 · 1,0/0,2 ·
0/0, im artista-Dialog nicht wählbar.)

**Messung** an „Sampler Step stitch pattern 1-10/11-20“ (`Arbeitsdaten/werkzeuge/art_stepmuster.py`; Reihen nach Lage
sortiert, Phase der Einstiche je Reihe): Das Raster verschiebt sich **abwechselnd um 0x2719 und 0x271a** – z. B.
Muster 6 (0,1/0,4): Schritte 0,1, 0,4, 0,1, 0,4 …; Muster 9 (0,3/0,6): 0,3, 0,6 …; bei gleichen Werten wie bisher
gleichmäßig. Die Muster 1–10 der Designs stimmen genau mit der Tabelle (Flächen in Schlangenlinie: Fläche 4 trägt
Muster 6, Fläche 6 Muster 4 – an der Beschriftung erkannt). Die Flächen in „11-20“ tragen **andere** Werte als die
Tabelle (Stichlänge 3,5/4,5 mm, 0x271b = 0,3 … 0,5) – Designs eines anderen Stands oder eigene Werte. Dort zeigt sich
die Wirkung von **0x271b**: die Einstiche einer Reihe liegen nicht mehr auf dem Raster, die Stiche streuen (0x271b = 0,5:
0,6 … 1,4 × Länge; 0,3: 0,77 … 1,23) – als Zufallsversatz ± 0x271b/2 × Länge nachgebaut (Verteilung eigene
Festlegung).

**Stoffe:** Der Deck-Step der 8 Stoffe hat 0,25 / 0,75 (Stil-Liste 0x27a6; Software Defaults 0,25 / 0,25) – seit 20.11.2026 im Kern (Referenz `vergleich_stoff` ersetzt, Stufe 10).
**Kern:** `TatamiEinstellung` mit `versatz2` (0x271a) und `zufall` (0x271b), `phase(k)`, `STEP_MUSTER`; JSON
`"step_muster": 1 … 20` je Objekt (auch für Step in Schrift). Test `step_muster_versatz`.

## 6. Mehrere Stichwinkel (Turning) — Sicherheit: hoch (Messung)
Handbuch Kap. 16 „Setting multiple stitch angles“: Winkellinien, die zwei Seiten des Objekts schneiden, „to create a
turning fill effect“. Im Original ist das ein **EmbroideryPatch mit Winkellinien** (0x06) – in den Musterdesigns
2401 Objekte mit Step-Füllung (0x1b). Messung `Arbeitsdaten/werkzeuge/art_wendung.py` an 415 davon: Richtung jedes
Step-Stichs voller Länge gegen die nächste Winkellinie – **83 % innerhalb 10°, 93 % innerhalb 20°**. Die Step-Stiche
laufen also längs der Winkellinien (wie Satin, in Step-Stiche geteilt), nicht quer.
**Kern:** `sticharten::wendung::wende_seiten` (Seiten A/B aus Umriss und Linien wie bei den Alphabet-Säulen; Kappen vor
der ersten/nach der letzten Linie laufen in eine Spitze aus – eigene Festlegung) + `satin::step_reihen`; JSON
`tatami` mit `"winkellinien"` (ohne Löcher). Test `wendefuellung_im_design`, `rechteck_mit_winkellinien`.

## 7. Offen
1. Das Raster der älteren Muster mit Versatz (0, 0) (= Step-Muster 18).
2. Laufwege innerhalb der Fläche statt Sprüngen (Stil-Eintrag 0x271c unter jeder Füllart: 1,8 mm, variabel,
   [01](01_stile_und_verbinder.md) §4) – an den Tatami-Stichblöcken der Muster messbar (Stiche zwischen Teilbereichen).
