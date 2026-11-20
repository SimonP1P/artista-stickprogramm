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

## 4. Text und Buchstaben im Design — Sicherheit: hoch (20.11.2026)
Der Text steht nicht im LetterText-Objekt selbst, sondern in den folgenden Objekten: **je Buchstabe ein LetterObj**
(Klasse 0x2f) mit **0x3a = Zeichen (UTF-16, Länge 1)**, **0x2e = Grundlinie** (gepackte Wörter wie im Alphabet) und
0x2f (Lage/Maßstab, doubles), danach die **Säulen des Buchstabens als eigene Objekte** (EmbroideryBoundGroup,
EmbroideryPatch mit Umriss 0x01 und **Winkellinien 0x06** im selben Format wie im ESB, Einheit 1/180 mm in x und y) –
also Kopien der Glyphen des Alphabets. Die Stiche hängen am LetterText (Satin 0x16, Zickzack 0x17, Lauf 0x18).
Die 0x80-Datensätze sind **nicht** je Buchstabe („Children“: 5 Datensätze, 8 Buchstaben).
Hinweis Werkzeug: `art_design.objekte` bricht am Zeichen-Datensatz 0x3a eines LetterObj ab (Ende-Erkennung der
Objektliste) – `art_saeulenfolge.py` liest ohne diesen Abbruch.

## 5. Reihenfolge, Richtung und Stücke der Säulen — Sicherheit: hoch (Messung), Verfahren des Originals offen
Werkzeug `Arbeitsdaten/werkzeuge/art_saeulenfolge.py`, Ergebnis `Arbeitsdaten/messungen/saeulenfolge_musterdesigns.txt`:
jedes gestickte Satin-Stück (0x16) des LetterText wird der Säule zugeordnet, in deren Umriss seine Einstiche liegen;
Lage entlang der Säule über die Mitten der Winkellinien (0 = erste, 1 = letzte Winkellinie).

| Messgröße (alle Musterdesigns) | Wert |
|---|---|
| Satin-Stücke zugeordnet / ohne Säule | **4558** / 50 |
| Stücke vorwärts / rückwärts (gegen die Winkellinien-Folge) | **2849 / 1709** (62 % / 38 %) |
| Buchstaben mit ≥ 2 Säulen: Reihenfolge und Richtung wie im Alphabet („As Digitized“) | **150 von 938** |
| … andere Reihenfolge und/oder teils rückwärts | 774 + 11 + 3 |
| Säule in 1 / 2 / 3 / 4 Stücken | 1577 / 1066 / 268 / 11 |
| erste Stich einer Säule am Anfang / Ende / mitten in der Säule | 1832 / 636 / 454 |
| Einstieg am näheren / ferneren Ende (zum vorigen Satin-Stück, Unterschied > 1 mm) | 1381 / 680 |

**Ergebnis:** Das Original stickt Schrift **nicht** in der Reihenfolge des Alphabets, sondern wählt Reihenfolge und
Richtung der Säulen selbst (Dialog „Sequence“: Join „As Digitized“ / „Closest Join“ / „Bottom Join“, STR 2141–2143;
Voreinstellung damit offensichtlich nicht „As Digitized“) und **teilt Säulen in Stücke** – bei 47 % der Säulen, meist
dort, wo eine andere Säule ansetzt (Beispiel „h“ in Swiss Block: Bogen vom Ende bis zur Mitte, Stammkopf, Rest des
Bogens, Stamm rückwärts). Das ist ein Verzweigen wie bei „Branching“; die Regel dafür ist aus den Stichen allein nicht
eindeutig (Einstieg nur zu 67 % am näheren Ende, wobei die Unterlagen und Laufwege dazwischen nicht berücksichtigt sind).

**Kern (20.11.2026):** `gruppe_wege` darf Satinsäulen jetzt **auch rückwärts** sticken und wählt Reihenfolge und
Richtung gemeinsam (erst wenigste Sprünge, dann kürzeste Laufwege; bis 7 Teile vollständig, sonst nächster Anfang) –
Nachbau von „Closest Join“ **ohne** das Teilen von Säulen. Bewertung = eigene Festlegung. Testtext (Swiss Block,
Drifter, Zurich, Alice): 88 statt 98 Sprünge, 13 635 statt 13 752 Stiche.

## 6. Offen
- **Säulen in Stücken (Verzweigen)** wie das Original: Code des Lettering-Sequencing (MOD1/„Closest Join“) lesen oder
  aus den Stücken der Musterdesigns die Teilungsregel ableiten (Teilungspunkte gegen die Ansatzstellen anderer Säulen).
- Wegführung der Laufwege zwischen den Unterlagen (Original: kurze Läufe 1,8 mm, teils über die Säule zurück).
