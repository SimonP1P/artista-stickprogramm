# 11 – Schrift: Reihenfolge von Unterlagen und Säulen im Original (20.11.2026)

**Frage:** In welcher Reihenfolge stickt die Original-Software die Unterlagen und Satinsäulen eines Buchstabens?
**Quelle:** die vom Original berechneten Stiche der ART-Musterdesigns (`Analyse_Teil3/02`), Zuordnung Stiche →
Objekt → Stichfunktion wie in `Analyse_Teil3/04` (`art_messen.py zuordnung`). Werkzeug:
`Arbeitsdaten/werkzeuge/art_schrift.py`, Ergebnis `Arbeitsdaten/messungen/schrift_reihenfolge_musterdesigns.txt`.

## 1. Was in den Schrift-Objekten steht — Sicherheit: hoch
LetterText-Objekte (Klasse 0x7b) bestehen aus einer Folge von Stichfunktionen: **Satin 0x16** (Säule), **Zickzack
0x17** (hier die Unterlage der Säulen) und **Lauf 0x18** (Stil 0x271c: 1,8 mm, variabel – Laufwege und kurze
Verbindungen). Beispiel „Sampler - Pattern Outlines - Children 1“, Funktionen 6–20 (vermutlich „h“):

| # | Art | von → bis (mm) | Deutung |
|---|---|---|---|
| 6–7 | Lauf | (−29,0│73,3) → (−20,2│80,9) | Laufweg zum Buchstaben |
| 8 | Zickzack | (−20,2│80,9) → (−15,4│71,9) | Unterlage des **Bogens** (zweite Säule) |
| 9–10 | Lauf | → (−23,6│73,9) | zurück zum Stamm |
| 11 | Zickzack | (−23,6│73,9) → (−21,7│86,5) | Unterlage des **Stamms**, endet oben am Säulenanfang |
| 13–17 | Satin | Stamm (oben → unten, in Stücken) | erste Säule |
| 18 | Satin | (−20,9│79,8) → (−15,4│71,9) | Bogen |

Beim „i“ (Funktionen 21–32) dagegen: Unterlage + Satin des Punktes, Laufweg zum Stamm, Unterlage + Satin des
Stamms – Punkt und Stamm berühren sich nicht.

## 2. Regel — Sicherheit: hoch (1 Unterlage), mittel-hoch (mehrere)
Säulen, die sich berühren, bilden eine **Gruppe**. Je Gruppe stickt das Original **erst alle Unterlagen, in
umgekehrter Reihenfolge der Säulen** (die letzte Unterlage endet am Anfang der ersten Säule), **dann die Säulen** in
ihrer Reihenfolge. Getrennte Teile eines Buchstabens kommen nacheinander, jeweils Unterlage(n) und Säule(n).

| Gruppen in allen LetterText-Objekten mit Unterlage | Anzahl |
|---|---|
| 1 Unterlage – vor den Säulen der Gruppe | **85 von 85** |
| mehrere Unterlagen – alle zuerst, umgekehrt zur Säulenfolge | **43** (2 Unterlagen: 38, 3: 5) |
| mehrere Unterlagen – anders | 19 |

Die 19 anderen Fälle sind nach Stichprobe **Grenzen der Messung**: Die Gruppenbildung (Abstand < 0,6 mm) fasst auch
Säulen zusammen, die nur nahe beieinander liegen, oder benachbarte Buchstaben (ein Fall mit 13 „Unterlagen“ in
„Sampler Alphabets 1“); bei „Heirloom 1“ folgen auf die Unterlage der ersten Säule zwei Satins, dann erst die nächste
Unterlage – passt zu getrennten Teilen.

## 3. Umsetzung im Kern
`kern/stick-kern/src/design.rs`, `gruppe_wege`: Satinsäulen liefern Unterlage und Decklage getrennt
(`satin_getrennt`); Teile, deren Umrisse sich überlappen, kreuzen oder näher als 0,15 mm liegen, bilden eine Gruppe;
je Gruppe erst die Unterlagen rückwärts, dann die Decklagen; dazwischen Laufwege, wenn sie unter noch ausstehenden
Teilen liegen (sonst Sprung). Test `gruppe_unterlagen_zuerst`. Text „ABC Anna“ (Drifter): Sprünge unverändert 9,
242 Stiche mehr (Laufwege zwischen Unterlagen und Säulen, wie im Original).

## 4. Offen
- **Reihenfolge, Start und Ende der Säulen** wie das Original („Closest Join“): Der Kern wählt die Folge mit den
  wenigsten Sprüngen (eigene Festlegung). Zum Messen müsste man Text und Alphabet der LetterText-Objekte kennen; sie
  stehen nicht als lesbarer Text im Datenblock (Feldaufbau der Klasse 0x7b noch nicht entschlüsselt).
- Wegführung der Laufwege zwischen den Unterlagen (Original: kurze Läufe 1,8 mm, teils über die Säule zurück).
