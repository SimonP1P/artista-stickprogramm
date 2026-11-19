# 08 – Laufwege in Flächen statt Sprüngen (19.11.2026)

**Ergebnis:**
- Das Original verbindet die Teilbereiche einer Tatami-Fläche fast immer mit **Laufstichen**, nicht mit Sprüngen.
- Die Laufwege entsprechen dem Lauf 0x271c unter jeder Füllart (1,8 mm, variabel; [01](01_stile_und_verbinder.md) §4).
- Etwa die Hälfte führt am Rand entlang, der Rest durchs Innere.
- Der Kern verbindet die Teilbereiche jetzt mit Laufwegen am Rand, statt zu springen (Referenz-Stufe 9). Das ist für die
  artista 185 wichtig, weil sie keinen Faden schneidet: Jeder Sprung hinterlässt einen Faden, der von Hand abgeschnitten
  werden muss.

Werkzeug: `Arbeitsdaten/werkzeuge/art_messen.py laufwege --alle`. Rohergebnis:
`Arbeitsdaten/messungen/laufwege_musterdesigns.txt`.

## 1. Messung — Sicherheit: hoch (Sprünge), mittel (Rand/Innen)
Ausgewertet sind Tatami-Objekte mit genau einer Tatami-Funktion; die Zuordnung wie in [04](04_messungen_satin.md) §1.

| Befund | Wert |
|---|---|
| Tatami-Blöcke | 2 231 |
| davon mit mindestens einem Stich > 1,3 × Stichlänge (Sprung oder Verbindung) | **191 (9 %)** |
| Laufwege (≥ 3 aufeinanderfolgende Stiche quer zur Reihenrichtung, 0,4–2,2 mm) | 642 |
| … Median-Abstand zum Umriss < 0,6 mm (**am Rand**) | 356 (55 %) |
| … weiter innen (**durch die Fläche**, später überstickt) | 286 (45 %) |
| Stichlänge in Laufwegen | Median **1,40 mm**, Quartile 0,76–1,50 mm |

**Deutung:**
- Die Stichlängen passen zum variablen Laufweg mit 1,8 mm: in Kurven kürzer (Sehnenabstand 0,089 mm), nie unter
  0,40 mm.
- Wann das Original am Rand und wann innen läuft, ist offen. Wahrscheinlich nimmt es den kürzeren Weg, der später
  überstickt wird.

## 2. Umsetzung im Kern (Referenz-Stufe 9)
- `tatami.rs`, Funktion `laufweg`:
  - Ende des einen und Anfang des nächsten Teilbereichs werden auf den nächstgelegenen Rand projiziert (Außenumriss oder
    Loch, Toleranz max(1 mm, 3 × Zugausgleich)).
  - Gelaufen wird in der kürzeren Richtung des Rings, mit `TatamiEinstellung::laufweg_laenge` (1,8 mm) und
    `laufweg` (Mindeststich 0,40 mm, Sehnenabstand 0,089 mm), variabel.
  - Liegen beide Punkte nicht am selben Rand, bleibt es beim Sprung.
- Gilt auch für die Tatami-Unterlage (Step).
- Test `u_form_mehrere_bereiche`: U-Form ergibt einen Weg ohne Sprung; alle Stichmitten liegen in der Fläche oder auf dem
  Rand.
- Wirkung auf die Vergleichsdesigns: Sprünge 23 → 22 (`vergleich_beispiel`) und 255 → 245 (`vergleich_unterlagen`; die
  übrigen stammen aus der absichtlich zu breiten Satinsäule).

## 3. Offen
1. Regel Rand ↔ Innen (Laufweg durch Bereiche, die später gestickt und damit überdeckt werden).
2. Reihenfolge der Teilbereiche und Start/Ende der Fläche (Original: Ein- und Ausstichpunkt je Objekt, Handbuch
   „entry/exit point“).
