# 10 – Regeln des Originals: Übersicht und Stand im Rechenkern (19.11.2026)

Diese Datei fasst zusammen, **nach welchen Regeln die BERNINA artista V4 (Wilcom) Stiche erzeugt** – aus Code (BEAGLink,
IMPORT, MOD1), Handbuch, Vorlagen/Stilen und den Messungen an echten Stichen (287 ART-Musterdesigns, 51 Designs aus dem
Maschinenspeicher). Je Regel: wo sie gilt, Beleg, Sicherheit und ob der Rust-Kern sie umsetzt (✅ umgesetzt, 🟡 teilweise
oder mit eigener Festlegung, ⬜ offen). Einzelheiten stehen in den verlinkten Berichten.

## 1. Gilt das auch für Bilder und Flächen? (Frage des Nutzers, 19.11.2026)
Ja – die Regeln hängen **nicht am Buchstaben**, sondern an der **Stichart und den Effekten des Objekts**. Was Schrift
besonders macht, ist nur ihr **Stil**: dort sind Kurzstiche und Auto-Unterlage voreingestellt (Analyse_Teil3/05 §2).
Zählung an allen Objekten der Musterdesigns (Werkzeug `Arbeitsdaten/werkzeuge/effekte_scan.py`, Rohergebnis `Arbeitsdaten/messungen/effekte_je_objektart.txt`):

| Objekte in den Musterdesigns | Kurzstiche an (0,5 / Zufall) | an (0,2) | aus | Unterlage im Objekt |
|---|---|---|---|---|
| Satinsäulen in Bildern (EmbroideryPatch) | 1 936 | 416 | 1 337 (+139 mit Schwelle 0) | Mittellauf 676, Zickzack 99, … |
| **Tatami-Flächen** in Bildern (EmbroideryPatch) | 770 | 7 | 35 (+12) | Step 54, … |
| Flächengruppen Tatami (ComplexFill) | 453 | 22 | 224 (+222) | Step 744, Edge Walk 56 |
| Flächengruppen Satin | 147 | – | 1 | Step 33, Zickzack 15 |
| Schrift (LetterText, Säulen in Schrift-Designs) | **alle** | – | – | Zickzack 50/101, Mittellauf 91 |

- **Kurzstiche:** der Effekt sitzt im allgemeinen Füllgenerator (MOD1 `FillStyle`/`StraightFill`, Aufruf 0x100a8595) –
  er wirkt bei **jeder Satin- und Füllfläche**, an der er hängt. Bei Bildern haben die Digitalisierer ihn je Objekt
  eingeschaltet; voreingestellt ist er nur im Stil „Schrift“.
- **Auto-Unterlage:** voreingestellt nur bei Schrift (Zickzack). In Bildern wählt der Digitalisierer je Objekt
  (Bericht 05 §3: 1 366 Satin mit Mittellauf, 195 mit Edge Walk + Mittellauf, 65 Tatami mit Step …).
- **Laufwege statt Sprünge:** bei Flächen (Teilbereiche einer Tatami-Fläche, Bericht 08) **und** in Buchstaben (§4).

## 2. Kurzstiche (Short Stitches) — Sicherheit: hoch (Code), Gruppenbildung vereinfacht
MOD1 `ShortStitchesEffect` (Konstruktor 0x101867d0, `init` 0x10186a10), Stil-Eintrag 0x273a:

| Feld | Stil-Tag | Wert | Bedeutung (Code) |
|---|---|---|---|
| +0x0C | 0x2819 | **0,5** | Schwelle: Stelle ist zu dicht, wenn zwei Nachbarstiche auf einer Seite näher als 0,5 × Abstand zusammenkommen (Aufrufer 0x100a858a: `fld [Effekt+0xC]; fimul Abstand`) |
| +0x10 | 0x281a | **1** | Zufall an: Faktor + (−0,04 / 0 / +0,04) (0x10186a60, Tabelle 0x10240660), Zufallsgenerator `ACMRandomNumber` mit festem Start 0x66ec8f |
| +0x14 | 0x281b | **5** | höchstens 5 Stiche je Gruppe |
| +0x18 | 0x281c | 15 Werte | **Dreieckstabelle** der Kürzungsfaktoren: Gruppe aus n Stichen → n Faktoren: n=1: 0,80 · n=2: 0,85 0,72 · n=3: 0,70 0,90 0,70 · n=4: 0,70 0,90 0,80 0,70 · n=5: 0,70 0,87 0,65 0,83 0,70 |

Ablauf (0x10186ed0): für Seite 0, dann Seite 1 der Säule: Abstand des Einstichs zum Nachbarstich (kleinster Abstand
Punkt ↔ Stichlinie, 0x10186db0 mit `LineSegment::Distance`) < Schwelle → Gruppe bilden (0x10186ad0, Toleranzen
0,9/1,1/1,33, bis 5 Stiche) → jeden Stich der Gruppe auf Faktor × Länge kürzen (Einstich rückt zur Gegenseite).
Gemessen am Maschinen-„A“ (Drifter): oben innen wechseln die Einstiche zwischen y = 20,1 und 20,9 mm – Gruppen aus
einem Stich, Faktor ≈ 0,8.
**Kern:** ✅ `sticharten/satin.rs::kurzstiche` (Gruppe vereinfacht: folgende Stiche näher als 0,9 × Abstand; Zufall
Park–Miller als Annahme für `ACMRandomNumber`). Standard aus, Schrift an, je Satin-Objekt `"kurzstiche": true`.
⬜ bei Tatami/Füllflächen noch nicht (Wirkung dort nicht gemessen).

## 3. Unterlage — Sicherheit: hoch (Code, Stile, Handbuch)
- Arten und Werte: Mittellauf 0x2746, Edge Walk 0x2747 (2,2 mm), Zickzack 0x2744 (5,0 mm + Rücklauf 3,0 mm),
  Doppelzickzack 0x2745 (7,0 mm), Step/Tatami 0x275a (3,0 mm) – Berichte 01, 04, 07. ✅
- Unterlage endet am Säulenanfang (07 §3). ✅
- **Auto Underlay** (Stil-Flag, Bericht 05 §2): Software Defaults und Stoffe **aus**, **Schrift an**, vorgewählte Art
  **Zickzack**. Handbuch PDF-S. 186/379: Schrift hat Auto-Unterlage, bei kleiner Schrift abschalten (z. B. Anniversary).
  ✅ (Schrift: neu)
- **Art je Objekt** (MOD1 `IsUnderlayTypeApplicable` → 0x101ffb10): Satinsäulen, Schrift, Ellipse/Ring/Stern: jede Art;
  **Flächengruppen (ComplexFill 7): nur Step, Zickzack, Edge Walk – sonst Zickzack**; Linien (0x05): keine. ✅ Kern:
  Flächen nehmen Step – gemessen die häufigste Unterlage der Tatami-Flächen im Original (ComplexFillGroup/Tatami: Step
  744, Edge Walk 56, Mittellauf 8, Zickzack 8; `messungen/effekte_je_objektart.txt`)
- Die Art hängt **nicht** von der Säulenbreite ab: Zeichensätze der Maschine (von Bernina erstellt) haben Mittellauf
  (Drifter 4,1 mm, Lisa 2,0 mm, Blackboard, Swiss Block, Zurich) oder Zickzack (Alice 2,3 mm, Bamboo 4,3 mm) – Wahl je
  Alphabet beim Erstellen, keine Regel in Code oder Dateien.

## 4. Laufwege und Reihenfolge — Sicherheit: hoch (Messung), Wegführung mittel
- **Flächen:** Teilbereiche einer Tatami-Fläche mit Laufstichen (1,8 mm, variabel) verbinden, etwa die Hälfte am Rand
  (Bericht 08). ✅
- **Buchstaben** (Maschine, Drifter „A“): erst der Querbalken, dann **Laufweg im linken Bein** (etwa auf seiner Mitte:
  (2,9|4,4) → (3,5|2,4) → (4,2|0,4)) unter dem später gestickten Bogen, dann der Bogen – kein Sprung im Buchstaben.
  Laufweg-Länge im Alphabet: Lauf 0x271c in der Füllung (540 = 3,0 mm). **Kern:** ✅ Buchstabe = Gruppe; Reihenfolge
  mit den wenigsten Sprüngen (bis 7 Teile alle Reihenfolgen), Laufweg gerade oder über die Mittellinie der nächsten
  Säule, nur wenn verdeckt (eigene Festlegung der Wegführung). Ergebnis „A“: wie das Original ohne Sprung.
- ✅ Das Original stickt je **Gruppe sich berührender Säulen zuerst alle Unterlagen, in umgekehrter Reihenfolge der
  Säulen**, dann die Säulen (Schrift-Musterdesigns: 1 Unterlage 85/85, mehrere 43/62, Rest Messgrenzen – Bericht 11).
  **Kern:** ✅ `gruppe_wege`. 🟡 Säulenreihenfolge: Das Original wählt Reihenfolge **und Richtung** selbst (38 % der
  Satin-Stücke rückwärts, nur 150/938 Buchstaben wie im Alphabet) und teilt 47 % der Säulen in Stücke (Bericht 11 §5).
  Kern: Reihenfolge + Richtung mit wenigsten Sprüngen/kürzesten Wegen ✅ (20.11.); Teilen der Säulen ⬜.

## 5. Satin — Sicherheit: hoch
| Regel | Beleg | Kern |
|---|---|---|
| Abstand = Weg zwischen Einstichen **derselben Seite**, gemessen auf der **längeren Seite** (außen im Bogen) | Handbuch S. 68, 184 000 Stiche (04) | ✅ |
| Auto Spacing: Tabelle (0,5 mm → 0,51 … 8 mm → 0,34) × Faktor, wenn Effekt 0x21 am Objekt; sonst fester Abstand | Code + 20 012 Stiche (05) | ✅ |
| Faktor: Stoffe 0,95–1,13; **Schrift 0,9** (Musterdesigns 0,9/0,87) | Stile, 05 | ✅ (Schrift neu) |
| Zugausgleich 0,2 mm (Stoffe 0,2–0,25; Handbuch Schrift 0,2–0,3) | Handbuch S. 199/201 | ✅ |
| Auto-Split ab 10 mm (0x2722 = 1800) – **im Original nicht wirksam**: 531 Satin-Querstiche 9–16 mm in 104 Säulen der Musterdesigns ungeteilt; mehrteilige gerade Züge sind gleich geteilte Laufwege (1/k) (`art_split.py`, `messungen/satin_split_musterdesigns.txt`, 20.11.2026) | Stil, Objekte, Messung | ✅ nichts nachzubauen |
| Kurzstiche | §2 | ✅ (Satin) |

## 6. Tatami, Lauf, Dreifach — Sicherheit: hoch
- Tatami: Abstand zwischen Hinreihen 0,40 mm (Nachbarreihen beim halben Abstand), Stichlänge 4,0 mm, Versatz 0,25 je
  Reihe, Mindeststich am Rand 0,40 mm (06). ✅
- Lauf: Einstich an **jedem Eckpunkt** der Form (07 §1); Länge 2,5 mm (Kontur), variabel mit Sehnenabstand bei
  Unterlagen-Läufen (01 §2). ✅
- Dreifach: Hin-Rück-Hin je Stich (01 §3). ✅

## 7. Verbinder, Vernähen, Maschinenregeln — Sicherheit: hoch
- Vernähen (Tie In/Off) nach Dialog „Connectors“: Tie In nach Trim/Farbwechsel oder wenn der vorige Verbinder > 3,0 mm;
  Tie Off vor Trim/Farbwechsel oder wenn der nächste > 3,0 mm; letztes Objekt immer; 2 Schritte vor und zurück (01, 02). ✅
- Mindeststich 0,3 mm, Höchststich 12,7 mm, Auto Jump: längere Stiche in Sprünge zu 7,0 mm (DEFAULTS.DFL, AMT). ✅
- ✅ Auto Jump ist bei Schrift laut Stil **aus** (ANALYSE.md §5) – im Kern je Objekt seit 20.11.2026 (`auto_jump`, Standard je Objektart; aus = lange Stiche in gleich lange echte Stiche geteilt, eigene Festlegung der Teilung).
- Trim ab 3,0 mm (Stoffe 6,0 mm); die artista 185 schneidet nicht selbst, jeder Sprung bleibt als Faden (08).
- **Designende** („Start und Ende“, Handbuch PDF-S. 205): Auto End – Mitte; Designs der Maschine enden zu 34/51 auf
  0,0 (Zeichensätze auf der Grundlinie). ✅ Kern: Sprung zurück zum Ursprung, abschaltbar (`"ende": "letzter_stich"`).
- **Start und Ende des Designs** (Dialog „Start and End“, Menü Arrange; BERESENU DLG „Start and End“: First Stitch /
  Auto Start at / Digitize; Last Stitch / Auto End at / Digitize; Auswahl Mitte, unten Mitte, zurück zum Start).
  Gemessen an den 287 Musterdesigns (20.11.2026, Skript wie `art_aussparen.py`, Klasse 0x71 = AutoStartEndObject,
  Inhalt in allen 143 Designs gleich `93 4e 00`): **mit 0x71 beginnen und enden 142/143 Designs in der Mitte des
  umschließenden Rechtecks** (90 davon liegen im Ursprung); ohne 0x71 liegen Start/Ende bei 120/144 woanders (erster/
  letzter Stich). Sicherheit hoch (Messung). ✅ Kern: `"start"`/`"ende"` = `rahmenmitte` (Standard, wie bisher),
  `designmitte`, `unten_mitte`, `erster_stich`/`letzter_stich`, Ende auch `start`. Standard bleibt die Rahmenmitte, bis
  am Gerät geprüft ist, wie die artista 185 ein Design platziert, dessen Start nicht in der Rahmenmitte liegt (Vor Ort V7).
  Ein Start/Ende **je Objekt** („closest join“) gibt es im Handbuch nur beim Auto Digitizer; Ein-/Ausstich je Objekt
  setzt sonst der Anwender (Reshape). **Bericht 16 (20.11.2026):** Closest Join = MOD1 ResequenceByClosestJoin →
  GEOMETRY FindExitEntry (exakt nächste Umrisspunkte, hoch); ohne Eingriff stickt das Original wie digitalisiert
  (Linien 75 % vom ersten Knoten, geschlossene 88 % ab dem ersten Knoten, Flächen 69 %); Verbindungen zwischen
  getrennten Objekten der Musterdesigns liegen zu 96 % auf dem kürzesten Weg. ✅ Kern: `einstich`/`ausstich` je
  Objekt, `kuerzeste_verbindung` (Standard aus); Flächen beginnen wie bisher am Punkt nächst dem vorigen Ende.

## 8. Schrift — Sicherheit: siehe Bericht 09
Original-Alphabete (ESB), Referenzhöhe 50 mm (mittel-hoch), Buchstabenabstand = Anteil × Höhe aus dem Alphabet-Kopf,
Leerzeichen 0,3 × Höhe (**eigene Festlegung**, offen), Stil „Schrift“ wie §2/§3/§5. Unterschied zu den
Maschinen-Zeichensätzen: Bernina hat diese mit **festem Abstand 0,40 mm** (gemessen 0,41 mm, Verhältnis zur
Auto-Spacing-Tabelle 1,06), ohne Zugausgleich und mit eigener Unterlagenwahl gestickt; die V4-Software (und der Kern)
nimmt Auto Spacing × 0,9 und Zickzack. ✅ / offen: Unterlagen-zuerst-Reihenfolge, Leerzeichen, Auto Jump aus.

**Lange Satinstiche in Schrift (21.11.2026):** Das Original stickt Schrift als Satin und wechselt **nicht selbst** zu Step
(Handbuch Kap. 20: Step/Special Satin „for large, irregular lettering shapes“ – von Hand); ein wirksames Auto Split gibt
es nicht (§5). Gemessen mit dem Kern (Text „AHMOW“, Stichart Satin, je Alphabet bei der **empfohlenen Maximalhöhe**,
`alphabets.md`): längster Satinstich bei 45 der 49 Alphabete mit Maximalhöhe **über 8 mm**, meist 9–16 mm (Swiss Block 60 mm → 11,8 mm,
Drifter 40 mm → 13,0 mm, Lisa 75 mm → 16,2 mm, Victoria 50 mm → 23,6 mm); nur Creative Cross, Hiragana, Quilting Text,
Roman-Small bleiben darunter. Das Original nimmt bis zur Maximalhöhe also lange Satinstiche hin. **Eigene Erweiterung**
`stichart: "auto"` (Kern `schrift::AUTO_STEP_AB_MM` = 8 mm wie EasyStitch, Handbuch S. 118 „Satin nicht über 7 mm“):
ganzer Text Step, sobald ein Satinstich nach Größe und Transformation länger wird – strenger als das Original;
„Trotzdem Satin“ (= `satin`) gibt das Verhalten des Originals. Sicherheit: Messung hoch, Grenze eigene Festlegung.

## 9. Offene Regeln (Reihenfolge der Bearbeitung)
1. Buchstaben: ✅ erst alle Unterlagen (rückwärts), dann Säulen (Bericht 11); ⬜ Säulenreihenfolge und Start/Ende wie das
   Original („closest join“) – braucht Text/Alphabet der LetterText-Objekte.
2. ✅ (20.11.) Kurzstiche in Step-Flächen mit Winkellinien und Step-Schrift umgesetzt (gerade Tatami-Reihen: keine Innenkurven); offen: an den 770 Step-Flächen der Musterdesigns nachmessen, Gruppenbildung 0x10186ad0 genau übertragen.
3. ✅ Unterlagenart für Flächen: Step (Messung); ✅ Auto-Split: im Original nicht wirksam (Messung, 20.11.2026).
4. Auto Jump je Stil (Schrift aus); Leerzeichen und Zeilenabstand der Schrift aus dem Code.
