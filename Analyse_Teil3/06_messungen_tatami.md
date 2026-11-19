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

## 5. Offen
1. Bedeutung von 0x271b (0 / 0,1 / 0,75) und das Raster der älteren Muster mit Versatz (0, 0).
2. Laufwege innerhalb der Fläche statt Sprüngen (Stil-Eintrag 0x271c unter jeder Füllart: 1,8 mm, variabel,
   [01](01_stile_und_verbinder.md) §4) – an den Tatami-Stichblöcken der Muster messbar (Stiche zwischen Teilbereichen).
3. Startpunkt/Endpunkt und Reihenfolge der Teilbereiche.
