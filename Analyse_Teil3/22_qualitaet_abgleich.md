# 22 – Abgleich mit allgemeinen Stickregeln: Stand, Lücken, Plan (21.11.2026)

Anlass: Prüfauftrag des Nutzers mit vier Themen aus allgemeinem Stickwissen (Zug-/Schubausgleich, Wegplanung,
variable Stichwinkel, Qualitätsregeln). Je Thema: was der Kern schon kann, was fehlt, was sich lohnt.

**Maßstab (CLAUDE.md §1, ARCHITECTURE.md §1a):** Werte und Abläufe kommen aus dem analysierten Original. Die BERNINA
artista V4 **ist** Wilcom-Software – die Analyse von Code, Stilen und 287 Musterdesigns ist damit die genaueste
Wilcom-Quelle, die wir haben. Allgemeine Richtwerte aus dem Prüfauftrag (z. B. „Vernähen 3–4 Stiche 0,3–0,5 mm“,
„Satin ab 6,5 mm teilen“) gelten hier nur als Vergleich. Wo sie vom Original abweichen, gilt das Original; wo das
Original nichts hat, wäre es eine **eigene Erweiterung** (markiert, abschaltbar, am Gerät geprüft). Die Links im
Prüfauftrag zeigen nur auf Startseiten (wilcom.com, inkstitch.org, google.com …) und wurden nicht ausgewertet.

Quellen-Hierarchie: Gerätetest > Code/entschlüsselte Daten > Handbuch > Ableitung.

---

## 1. Zug- und Schubausgleich (Pull/Push Compensation)

### Stand – Sicherheit: hoch
| Was | Wert im Kern | Beleg |
|---|---|---|
| Zugausgleich Satin, Tatami, Unterlage (Stich an beiden Enden verlängert) | 0,20 mm | Handbuch S. 199–201, Stil 0x273d, AMT 0x2743 |
| je Stoff (8 Stoffe) | 0,20–0,25 mm | Stile NORMAL.AMT (`PARAMETER.md`) |
| Auto Digitizer / EasyStitch / SVG / Bild | 0,40 mm | Handbuch S. 116 „Use 0.4 when using automatic digitizing“, Stile `<PRESET_EASYSTITCH>` |
| Überlappung zwischen Satinsäulen (TrueType, Satinfläche mit Knick) | 0,30 mm | TurningConversionValues „T“ (Bericht 17 §6.2) |
| je Objekt einstellbar | ja | `einstellungen.satin/tatami.zugausgleich` |

Das Handbuch (PDF-S. 200–201) beschreibt den Zugausgleich der V4 als **festen Wert in mm**, abhängig nur vom Stoff
(Richttabelle: Drill/Baumwolle 0,1 · T-Shirt 0,2 · Fleece/Strick 0,25–0,3 · Schrift 0,2–0,3 mm). Einen Ausgleich nach
Säulenbreite oder Dichte gibt es in der V4 nicht; einen **Schubausgleich (Push) gibt es gar nicht** (Strings: nur
„Pull Compensation Effect“, „Shortening Effect“ = Kurzstiche, BERESENU STR 32699/32700).

### Lücken
- **Breitenabhängiger Zugausgleich** (Prüfauftrag: +0,15 … +0,3 mm je nach Breite): nicht im Original → wäre eigene
  Erweiterung.
- **Schubausgleich:** nicht im Original. Laut Handbuch werden Formen „schmaler **und länger**“ – das „länger“ gleicht
  niemand aus. Bei Kreisen aus Tatami mit Laufstich-Umrandung kann das sichtbar werden.
- **Wie stark die artista 185 tatsächlich verzieht, ist nicht gemessen.** Ohne Messung wäre jede Formel geraten.

### Vorschlag (erst messen, dann rechnen)
Formel mit Kalibrierwerten, die bei 0 genau das Original ergibt:

```
zugausgleich(b, s) = zg_stoff + k_b · (b − 4 mm) + k_s · (0,40 mm / s − 1)      (b = Säulenbreite, s = Abstand)
schubausgleich      = k_p · zg_stoff   (Form in Stichrichtung quer um diesen Betrag an beiden Enden kürzen)
```
`k_b`, `k_s`, `k_p` kommen aus einem **Kalibrierblatt am Gerät** (Satinsäulen 1/2/4/8 mm in 0° und 90°, Tatami-Kreis
Ø 30 mm mit Laufstich-Umriss, Quadrat 30 mm; auf Webware und Strick; mit dem Lineal nachmessen). Der allgemeine Richtwert
„0,15 mm bei schmalen bis 0,3 mm bei breiten Säulen“ entspräche etwa `k_b` ≈ 0,02 mm je mm Breite – unbelegt.
Standard bliebe 0 (= Original), einschalten je Stoff im Expertenmodus.

---

## 2. Wegplanung, Sprünge, Ein-/Ausstich

### Stand – Sicherheit: hoch (Messung), Wegführung mittel
| Was | Kern | Beleg |
|---|---|---|
| Reihenfolge wie digitalisiert; Umordnen wie Menü *Arrange* (wie ausgewählt, Anfang/Ende, nach Farbe, nach Nummer) | ✅ `reihenfolge.rs` | Bericht 16 §1–3 |
| Prüfung „Objekt verdeckt ein späteres“ | ✅ eigene Erweiterung | Bericht 16 |
| Closest Join (Aus-/Einstich auf die nächsten Umrisspunkte) | ✅ `kuerzeste_verbindung`; **an** beim SVG-Import, sonst aus | MOD1 ResequenceByClosestJoin → GEOMETRY FindExitEntry (Bericht 16 §2) |
| Teilbereiche einer Fläche mit Laufwegen statt Sprüngen (1,8 mm, halb am Rand) | ✅ | Bericht 08 |
| Buchstabe als Gruppe: beste Reihenfolge und Richtung der Säulen (bis 7 Teile alle Möglichkeiten), Laufweg nur wo später überstickt | ✅ | Bericht 10 §4, 11 |
| Je Säulengruppe erst alle Unterlagen rückwärts, dann die Säulen – die Unterlage dient als Laufweg zurück | ✅ | Bericht 11 (85/85) |
| Vernähen nur bei Verbindern > 3 mm, Auto Jump > 12,7 mm in 7-mm-Sprünge | ✅ | Bericht 01, DEFAULTS.DFL |

Die Idee aus dem Prüfauftrag („Unterlage als unsichtbarer Laufweg“) setzt das Original schon um: Unterlagen einer
Gruppe rückwärts, dann die Säulen vorwärts.

### Lücken
- **Sprünge zwischen Objekten und zwischen Buchstaben.** Beispiel „Anna“ + „Sky“ (Computer-Schrift): 8 Sprünge, vier
  Original-Alphabete mit 24 Buchstaben: 38 Sprünge. Die artista 185 **schneidet nicht** (DFL `trims_for_jump = 0`) –
  jeder Sprung ist ein Faden, der von Hand abgeschnitten werden muss. Das ist der größte praktische Qualitätsverlust.
  Das Original kennt dafür im Dialog „Connectors“ den **Verbindertyp „Run“** (Jump/Run) – im Kern **nicht umgesetzt**,
  Tag und Wegführung noch nicht analysiert.
- **Bild umwandeln** (`stick-vektor`, Auto Digitizer) setzt `kuerzeste_verbindung` nicht – laut Handbuch nutzt gerade
  der Auto Digitizer Closest Join.
- **Blackwork Run** (Linien eines Netzes in einem Zug, jede Kante höchstens zweimal = Euler-Weg): im Original vorhanden
  (Handbuch S. 133 „Automatic sequencing“), im Kern offen (ROADMAP 4b.9).
- **Säulen teilen** wie das Original (47 % der Säulen in Stücken, Bericht 11 §5): offen.
- **Automatisches Umordnen über Objekte hinweg (TSP):** macht das Original nicht (stickt wie digitalisiert, Bericht 16).
  Die Reihenfolge bestimmt, was oben liegt – ein Umordnen ohne Rücksicht darauf zerstört Designs. Möglich nur als
  eigene Erweiterung **innerhalb einer Farbe und nur zwischen Objekten, die sich nicht überdecken** (die
  Überdeckungsprüfung gibt es schon).

---

## 3. Variable Stichwinkel, Kurven, Dichte in engen Bögen

### Stand – Sicherheit: hoch
| Was | Kern | Beleg |
|---|---|---|
| Satin dreht mit der Form (Säulen-Zerlegung über Triangulation = Näherung der Mittelachse) | ✅ `saeulen.rs` (EZSTITCH-Verfahren) | Bericht 17; 54/54 Säulen wie das Original, Median 4,4° |
| Tatami mit mehreren Stichwinkeln (Winkellinien) | ✅ `winkellinien` | Bericht 06 §6 (83 % < 10°) |
| Abstand in Bögen auf der **Außenseite** gemessen → innen dichter | ✅ | 184 000 Stiche, Bericht 04 |
| **Kurzstiche** an Innenkurven (Schwelle 0,5 × Abstand, Gruppen bis 5, Faktoren 0,70–0,90, Zufall ±0,04) | ✅ Satin, Step mit Winkellinien | MOD1 ShortStitchesEffect (Bericht 10 §2) |
| Auto Spacing (schmale Säulen weiter, breite enger) | ✅ | Bericht 05 |
| Füllmuster (242), Motive, Knopflöcher | ✅ | Berichte 13, 19, 20 |

### Lücken
- **Wave Effect** (Füllung folgt einer Kurve): im Original (Handbuch Kap. 24/26), im Kern offen.
- Contour/Coil/Spiral-Füllungen: in der V4 nur als Strings, im Handbuch nicht beschrieben → kein Ziel, höchstens eigene
  Erweiterung.
- Kurzstiche in gewöhnlichen Tatami-Flächen ohne Winkellinien: nicht nötig (gerade Reihen haben keine Innenkurven).

---

## 4. Qualitätsregeln

| Regel aus dem Prüfauftrag | Original (V4) | Kern | Bewertung |
|---|---|---|---|
| **Unterlage** automatisch | Auto Underlay im Stil „Schrift“ (Zickzack); je Stoff; je Objekt 5 Arten (Mittellauf, Edge Walk 2,2 mm, Zickzack, Doppelzickzack, Step) | ✅ alle Arten, Auto bei Schrift und Computer-Schrift, Vorwahl Mittellauf + Zickzack (eigene Festlegung) | erfüllt |
| **Vernähen** 3–4 Mikrostiche 0,3–0,5 mm | 2 Schritte vor und zurück, ≤ 1,0 mm, nur nach Farbwechsel/vor langem Verbinder (> 3 mm) | ✅ wie das Original | Original weicht vom Richtwert ab; ob es auf der 185 hält: Gerätetest |
| **Auto-Split** > 6,5 mm mit Zufallsversatz | Auto Split 10 mm im Stil, **aber nicht wirksam** (531 Satinstiche 9–16 mm ungeteilt, Bericht 10 §5) | Höchststich 12,7 mm, Auto Jump; Schrift `auto` → Step ab 8 mm (eigene Erweiterung); Hinweis `lange_satinstiche` | für Schrift erfüllt; für Satin-Formen nur Hinweis |
| **Winkelschutz** (kein exakt 0°/90° bei Tatami) | keine Regel; Versatz 0,25 je Reihe verhindert Einstichlinien | Standard 45° (eigene Festlegung), Versatz wie Original | kein Handlungsbedarf, höchstens Hinweis |
| Mindeststich (gegen Fadenriss) | 0,3 mm | ✅ | erfüllt |

---

## 5. Was bringt am meisten? (Vorschlag, Entscheidung beim Nutzer)

| # | Maßnahme | Nutzen | Aufwand | Grundlage |
|---|---|---|---|---|
| 1 | **Kalibrierblatt am Gerät** (siehe §1) auf 2 Stoffen sticken und messen: Verzug, Lücken Umriss/Füllung, halten die Vernähstiche | entscheidet, ob §1 überhaupt nötig ist | klein (Design vorbereitet: `stickprogramm/beispiele/kalibrierblatt.json`, ROADMAP Vor Ort V12/V13; Person vor Ort) | Gerätetest = höchste Quelle |
| 2 | **Bild umwandeln mit Closest Join** | kürzere Verbindungen, weniger Fäden | sehr klein | Handbuch (Auto Digitizer), Code FindExitEntry |
| 3 | **Verbindertyp „Run“** analysieren (Tag im Dialog „Connectors“, Wegführung in MOD1) und umsetzen, zuerst zwischen Buchstaben | weniger Sprungfäden – auf der 185 der größte Gewinn, weil sie nicht schneidet | mittel | Original-Dialog vorhanden |
| 4 | **Blackwork Run** (Euler-Weg über Liniennetze, 4b.9) | Linienzeichnungen in einem Zug | mittel | Handbuch S. 133 |
| 5 | Zug-/Schubausgleich nach §1 – nur wenn #1 Verzug zeigt | Formtreue auf Strick | mittel | eigene Erweiterung, kalibriert |
| 6 | Wave Effect | gestalterisch | mittel–groß | Handbuch Kap. 24/26 |
| 7 | Säulen teilen wie das Original | etwas weniger Wege in Buchstaben | groß | Bericht 11 §5 |
| 8 | Satin teilen mit Zufallsversatz, Winkelschutz | gering (Schrift schon über `auto`) | klein | eigene Erweiterung – nicht empfohlen |

Empfehlung: **1 und 2 sofort** (klein, sicher), dann **3** (größter Gewinn an der Maschine), danach 4. Punkt 5 nur nach
Messung. TSP-Umordnen über Objekte hinweg nicht automatisch – höchstens als Knopf „Wege verkürzen (gleiche Farbe, nur
ohne Überdeckung)“.
