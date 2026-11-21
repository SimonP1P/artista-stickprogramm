# 04 – Messungen an den Original-Stichen: Satinabstand und Unterlagen (19.11.2026)

**Ergebnis:**
- Der Objektteil ([03](03_art_objekte.md)) und die Stiche ([02](02_art_stichdaten.md)) lassen sich **Objekt für Objekt
  und Stichfunktion für Stichfunktion** zuordnen. Damit sind die Einstellungen und das daraus berechnete Ergebnis des
  Originals direkt vergleichbar.
- Zwei Fehler im bisherigen Stand sind gefunden und im Rust-Kern behoben:
  1. **Satinabstand** = Weg zwischen zwei Einstichen **auf derselben Seite**. Der Kern war bisher halb so dicht.
  2. **0x2746 = Mittellauf (Center Run), 0x2747 = Edge Walk.** Die beiden waren bisher vertauscht.
  3. (21.11.2026, §5) **Satin sticht „gerade quer + schräg vor“**: A und B einer Stelle liegen auf derselben
     Stichlinie, der Rückstich rückt um den ganzen Abstand vor. Der Kern stickte bisher symmetrischen Zickzack.

Werkzeug: `Arbeitsdaten/werkzeuge/art_messen.py` (`zuordnung`, `satin --alle`, `unterlagen --alle`). Grundlage sind die
285 ART-Musterdesigns unter `Main/Samples/` der entpackten Software (`F:\Entwicklung\artista_ziel`).
Rohergebnis: `Arbeitsdaten/messungen/satin_abstand_musterdesigns.txt`.

## 1. Zuordnung Objekt ↔ Stiche — Sicherheit: hoch
- Stichsegmente ([02](02_art_stichdaten.md) §3) der **Art 0** beginnen eine Stichfunktion. Ihr Feld +0x3c zählt die
  Funktionen im Objekt (0, 1, 2 …). Fällt es auf 0 zurück, beginnt das nächste Objekt.
- **Art-1**-Segmente enthalten die Stiche der laufenden Funktion.
- Im Objektteil steht hinter dem Eigenschaftsbaum jedes Objekts die **Liste der Stichfunktionen** als Verweise, in
  Stickreihenfolge, z. B. Flower 001 Objekt 0: 3 × Lauf (0x2746), 3 × Lauf (0x2747), Satin.
- Prüfung über alle Musterdesigns:
  - Bei 206 von 285 Designs ist die Objektzahl gleich.
  - In diesen passt bei **19 226 von 22 068 Objekten** die Zahl der Funktionen genau zur Zahl der Stichblöcke.
  - Abweichungen gibt es vor allem bei Auto-Digitalisier-Designs (Bitmap-Objekt 0x5d liest Rohdaten außerhalb des
    Rahmens) und bei den letzten Objekten einiger Quilt-Designs.

## 2. Edge Walk und Mittellauf — Sicherheit: hoch (drei unabhängige Belege)
| Beleg | 0x2746 | 0x2747 |
|---|---|---|
| Dialogcode DIALOG.DLL 0x100190f5–0x10019147: Kennung → Text-Nr. | (2598, erschlossen: einzige freie Nummer) | **2599** |
| Texte: RESENU.DLL / **BERESENU.DLL** (Bernina-Fassung) | 2598 „Center Run“ | 2599 **„Edge Walk“** |
| Lage der Laufpunkte in den Musterdesigns (25 254 Punkte) | 82 % näher an der Säulenmitte | 85 % näher am Rand |

Gegenprobe aus demselben Dialogcode: 0x2744 → 2600 „Zigzag“, 0x2745 → 2601 „Double Zigzag“, 0x275a → 2603 „Step“ –
alle wie bisher angenommen. Die frühere Zuordnung (Edge Walk = 0x2746, Sicherheit mittel, aus Handbuch-Beispielwerten
geschlossen) war vertauscht.

**Damit gilt:**
- **Mittellauf:** 2,2 mm, Mindeststich 0,728 mm, Sehnenabstand 0,061 mm.
- **Edge Walk:** 2,2 mm, Mindeststich 0,40 mm, Sehnenabstand 0,072 mm.

Korrigiert in [01](01_stile_und_verbinder.md) §2, `Analyse/code/06`, `Analyse/code/10` und im Kern (`einstellungen.rs`,
Referenz-Stufe 4).

**Ablauf am Beispiel** (Flower 001, Objekt 6, Satinsäule ≈ 20 mm lang):
- **Mittellauf** in drei Stücken: kurzer Anlauf (2 Punkte), hin entlang der Mitte (14), zurück (14).
- **Edge Walk** in drei Stücken: Anlauf (2), einmal um den Rand herum (44), Übergang zum Satinanfang (2).
- Dann **Satin** mit 156 Einstichen.

Hier sind beide Unterlagen aktiv. Woran das Original ein- und ausgeschaltete Unterlagen unterscheidet: [05](05_effekte_und_unterlagen.md).

## 3. Satinabstand — Sicherheit: hoch (Handbuch + Messung)
**Handbuch** (PDF-S. 68, „Adjusting Satin outline stitch spacing“): „Stitch spacing is the distance in millimeters
between two needle penetrations **on the same side** of a column.“

**Messung:**
- Je Satinobjekt wird der gespeicherte Abstand (Klasse 0x15, 1/180 mm) mit dem Weg zwischen den Mitten zweier
  gleichgerichteter Stiche verglichen (Stich i → Stich i+2).
- Ausgewertet werden nur gleichmäßige Stellen (Breitenänderung < 0,15 mm), insgesamt **184 000 Stichpaare**.

| gespeichert | Breite 1 mm | 2 mm | 3 mm | 4 mm | 5 mm | 6 mm |
|---|---|---|---|---|---|---|
| 63 = 0,35 mm | 0,336 | 0,345 | 0,345 | 0,356 | 0,348 | – |
| 72 = 0,40 mm | 0,394 | 0,386 | 0,385 | 0,398 | 0,389 | 0,388 |
| 81 = 0,45 mm | 0,432 | 0,447 | 0,453 | 0,453 | 0,439 | – |
| 144 = 0,80 mm | 0,800 | 0,811 | 0,789 | – | – | – |
(Median in mm; vollständige Liste im Rohergebnis.)

**Folgerungen:**
1. Der Abstand zählt **Zick zu Zick**, also auf derselben Seite. Jeder einzelne Einstich rückt um den halben Abstand
   vor.
   - Bisher rückte der Kern je Einstich um den ganzen Abstand vor. Damit war er halb so dicht wie das Original.
   - Behoben im Kern (`satin.rs`, Referenz-Stufe 3), siehe `kern/testdaten/referenz/README.md`.
   - Die Zickzack-Unterlage rechnete schon richtig (Einstich alle Länge/2).
2. Der gemessene Abstand ist **über alle Breiten gleich**, von 0,5 bis 6 mm. In den Musterdesigns wirkt also **kein
   Auto Spacing**, obwohl das Byte nach dem Auto-Split in Klasse 0x16 (Objekt +0x94) überall 1 ist.
   - Dieses Byte ist demnach nicht der Auto-Spacing-Schalter.
   - Die Auto-Spacing-Tabelle (`Analyse/code/10` §A) bleibt richtig. Wann das Original sie anwendet: [05](05_effekte_und_unterlagen.md).
3. Der Median auf der Mitte liegt 2–4 % unter dem Sollwert, weil das Original den Abstand auf der Außenseite einhält
   (§4).

## 4. Satin in Bögen: Abstand auf der Außenseite — Sicherheit: hoch
Gleiche Auswertung, nur Objekte mit festem Abstand (ohne Auto-Spacing-Effekt, [05](05_effekte_und_unterlagen.md)) und
Stiche ab 1 mm Breite. Je Stichpaar werden die Wege zwischen Einstichen derselben Seite verglichen, außen und innen,
dazu der Weg auf der Mitte. Eingeteilt nach Krümmung, gemessen als Verhältnis außen/innen.
Rohergebnis: `Arbeitsdaten/messungen/boegen_musterdesigns.txt` (`art_messen.py boegen --alle`).

| Krümmung (außen/innen) | Stichpaare | **außen** / Abstand | Mitte / Abstand | innen / Abstand |
|---|---|---|---|---|
| gerade (< 1,1) | 38 013 | **1,000** | 0,987 | 0,974 |
| leicht (1,1–1,5) | 28 736 | **1,034** | 0,945 | 0,861 |
| stark (1,5–3) | 8 877 | **1,077** | 0,829 | 0,603 |
| sehr stark (> 3) | 1 531 | **1,029** | 0,641 | 0,259 |

**Folgerung:** Das Original hält den Abstand auf der **längeren Seite** ein, innen wird es dichter. Der Kern nimmt seit
Referenz-Stufe 6 den Fortschritt auf der längeren Seite (`satin.rs`); vorher war es das Mittel beider Seiten, eine
eigene Festlegung.

## 5. Einstichmuster: gerade quer + schräg vor — Sicherheit: hoch (21.11.2026)
**Frage:** Rückt jeder Einstich um den halben Abstand vor (symmetrischer Zickzack, so rechnete der Kern bis
21.11.2026), oder liegen die Einstiche beider Seiten an derselben Stelle (ein Stich quer, einer schräg)? Aus den
Stichen allein ist das nicht zu trennen – bei schrägen Stichlinien ist auch der „quere“ Stich schräg. Deshalb gemessen
**gegen die gespeicherten Stichlinien** der Säulen (EmbroideryPatch 0x06, Umrissstellen, [17](17_auto_satin.md)).

**Verfahren** (`Arbeitsdaten/werkzeuge/art_satinmuster.py`, Rohergebnis `Arbeitsdaten/messungen/satinmuster_musterdesigns.txt`):
je Paar benachbarter Stichlinien, die fast parallel sind (< 3°, Länge ≥ 1,5 mm, Längenunterschied ≤ 0,3 mm), die
Einstiche dazwischen auf die Säulenrichtung (senkrecht zur Stichlinie) projizieren; für drei aufeinanderfolgende
Einstiche a, b, c: r = (t_b − t_a) / (t_c − t_a). Symmetrischer Zickzack → r ≈ 0,5; gerade quer + schräg → r ≈ 0 oder 1.

**Ergebnis:** 3869 Säulen mit Stichlinien, 125 mit genug geraden Abschnitten. Verteilung von r: **1126 bei 0,0 und 1134
bei 1,0**, bei 0,5 nur **12**; je Säule Median von min(r, 1 − r) = **0,066** (Quartile 0,02–0,11). Dazu die
Knopflöcher im Muster `Samples/Reference/Buttonholes.ART` ([19](19_knopfloecher.md)): gerade Raupen mit waagrechten
Stichlinien, Einstiche (−2,71 | 19,71), (−0,37 | 19,71), (−2,71 | 19,21) … – jeder zweite Stich exakt quer.

**Folgerung:** An jeder Stelle stickt das Original A und B auf derselben Stichlinie, dann schräg zur nächsten Stelle
(ganzer Abstand). Der Abstand auf derselben Seite (§3) bleibt gleich, nur die Lage der Einstiche der Gegenseite ändert
sich. Im Kern seit Referenz-Stufe 11 (`satin.rs`, `satin_quer`). **Nicht** geändert (nicht gemessen): Step-Reihen in
Säulen (Schrift „Step“, Reihen parallel wie Tatami) und der Zickzack-Deckstich 0x2724. Wie das Original das Säulenende
abschließt, ist nicht gemessen (Kern: letzte Stelle auf die Endkante, **eigene Festlegung**).

## 6. Offen
1. ~~**Auto Spacing**~~ – geklärt in [05](05_effekte_und_unterlagen.md): Effekt 0x21 am Satin. (Frage war: Welcher Wert schaltet es ein? Ist es in „Software Defaults“ ein? Suchen: Lesefunktion von Klasse
   0x16/0x15 gegen die Stil-Kennungen 0x2734/0x27ab.
2. ~~**Unterlagen ein/aus**~~ – geklärt in [05](05_effekte_und_unterlagen.md) (im Objekt stehen nur aktive; Voreinstellung über Stil-Flags). In Flower stehen Edge Walk und Mittellauf in jedem Objekt und werden beide gestickt. Wie ist
   „ausgeschaltet“ gespeichert? Stile: Flag-Bit 0x0001 ([01](01_stile_und_verbinder.md) §5).
3. ~~**Satin in Bögen**~~ – geklärt, §4: auf der Außenseite.
4. ~~**Tatami**~~ – Reihenabstand und Versatz gemessen in [06](06_messungen_tatami.md).
