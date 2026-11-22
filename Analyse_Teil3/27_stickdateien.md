# 27 – Fremde Stickdateien, eigene Rahmen, ART mit Objekten

Stand 22.11.2026 (Nachtauftrag N7 und N11, ROADMAP 4b.13).

Quellen:
- Handbuch Kap. 28 „Processing Files of Different Formats“ (gedruckte S. 255–257) und Kap. 30 (Formatliste Magic
  Box);
- Strings von `gnfilter.dll` (Lese-/Schreibfilter des Originals, aus `setup.z`);
- Garnkarten `Thread Colors - *.tch` des Originals.

Kern:
- `stick-formate::stickdatei` (Leser);
- `stick-kern::stichdaten` (Objektart `stiche`);
- Befehle `stickdatei_lesen`, `stickdatei_endungen` (`stick-api/src/stickdateien.rs`).

## 1. Was das Original kann — Sicherheit: hoch (Handbuch, Strings)
- **Formate:** `gnfilter.dll` nennt `*.xxx`, `*.csd`, `*.sew`, `*.pes`, `*.hus`, `*.dst`, `*.exp`, `*.pcs` (STR
  0x52d5c ff.), dazu PEC („#PEC0001LA:“). Das Handbuch Kap. 30 listet dieselben (CSD, DST, EXP, HUS, PCM, PCS,
  PES/PEC, XXX, SEW).
  - **JEF und VP3 kennt die Version 4 nicht.** Sie kommen auf Wunsch des Nutzers dazu (heute verbreitete Formate,
    Entscheidung 22.11.2026).
- **Öffnen mit oder ohne Objektumwandlung** (S. 255–257):
  - „By default, stitch files are converted upon opening“ – „converted to a Grade C ART file“; nicht erkannte Teile
    werden „an object with general property only“.
  - Ohne Umwandlung: „The design opens as a collection of stitches with only general properties“ (Grade D). So bleibt
    das Design unverändert, einzelne Stiche lassen sich bearbeiten, „minor scaling and transforming actions“ gehen.
  - Empfehlung: Ohne Umwandlung „not … scale the design by more than ±5 %“.
- **PES speichern** mit Rahmenwahl (S. 257): kein Ziel. Nutzer: Schreiben bleibt EXP und Senden an die 185.

## 2. Umsetzung: Grade D — Entscheidung des Nutzers „nur Stiche, ohne Objekte“
**Objektart `stiche`** (`format.rs` `SticheJson`, `stichdaten.rs`):
- je Farbblock der Datei ein Objekt mit den Einstichen in Zügen; ein Zug endet an einem Sprung oder Fadenschnitt;
- gespeichert als Base64 (i16 in 0,1 mm, Zug-Trenner −32768; eigene Festlegung);
- die Mitte des Designs liegt auf der Rahmenmitte.

Berechnen:
- Die Züge werden unverändert Stiche.
- **Kein zusätzliches Vernähen**, weil die Datei ihr eigenes hat: Verbinder des Teils mit `vernaeh_stiche = 0`.
- Es gelten die Bernina-Regeln wie für jedes Design: Mindeststich 0,3 mm, Auto Jump über 12,7 mm (`bernina_regeln`).
  Sie entsprechen dem Maschinenformat des Originals.

Transformationen bewegen die Einstiche, Effekte wie die Welle verformen sie.
- Ab ±5 % Größenänderung (Singulärwerte der Abbildung aus Objekt, Gruppe und Design) meldet `design_berechnen` den
  Hinweis `stichdaten_skaliert` wie im Handbuch.
- Form für die Oberfläche: konvexe Hülle der Einstiche (Auswahl, Griffe).

Rahmen beim Öffnen: der kleinste der 185, in den das Rechteck des Designs passt. Der Freiarm-Rahmen kommt nur, wenn
nichts anderes passt (eigene Festlegung). Passt nichts, gilt der größte Rahmen mit Hinweis.

## 3. Formate (Aufbau: allgemein bekannte Formatbeschreibungen; Prüfung mit selbst gebauten Dateien)
| Format | Aufbau | Farben | Sicherheit |
|---|---|---|---|
| DST (Tajima) | 512 Byte Kopf (`LA:` Name), Datensätze 3 Byte mit Bits ±1/3/9/27/81 je Achse, Byte 3: `F3` Ende, `C3` Farbwechsel (Stopp), `83` Sprung; 0,1 mm, +y oben | keine → Standardfolge `Default.tch` | hoch (Kodierung für alle Werte −121 … 121 geprüft) |
| PES / PEC (Brother) | `#PES` + Version, Verweis (u32 bei 8) auf den PEC-Teil; PEC: `LA:` Name, bei 48 Zahl der Farbwechsel, ab 49 Farbnummern, ab 0x210 Stiche: 7-Bit-Werte oder 12-Bit mit Kennbits (0x80 lang, 0x20 Schnitt, 0x10 Sprung), `FE B0 xx` Farbwechsel, `FF 00` Ende; +y unten | PEC-Nummer 1–64 → Name (Reihenfolge des PEC-Formats) → RGB aus **`Thread Colors - PES.tch` des Originals** (alle 64 Namen dort vorhanden) | hoch (Aufbau), Farben hoch |
| JEF (Janome) | u32 Anfang der Stiche, u32 bei 24 Zahl der Farben, ab 116 Farbnummern (u32); Stiche 2 Byte (+y oben), `80 01 dx dy` Farbwechsel, `80 02 dx dy` Sprung, `80 10` Ende | Janome-Nummer 1–30 → Name; RGB aus **`Thread Colors - SEW.tch` des Originals** (Janome) | Aufbau hoch, Farbnummern mittel (ab 31: Standardfolge mit Hinweis) |
| VP3 (Viking/Pfaff) | `%vsm%`, Texte mit 16-Bit-Länge, Designmitte (1/1000 mm), Farbblöcke `00 05 00` + Länge + Anfang + Garn (RGB, Nummer, Name, Marke) + Stiche (2 Byte, `80 01` langer Stich mit 16 Bit, `80 03` Schnitt) | RGB aus der Datei | **niedrig: ohne echte Beispieldatei** – Lesefehler melden statt raten |
| EXP | wie bisher (`stick-formate::exp`) | keine → Standardfolge | hoch |
| HUS (Viking) | Stiche, x und y getrennt **komprimiert** (eigenes Verfahren) | `Thread Colors - HUS.tch` (29 Farben) | **offen** – ohne Beispieldatei nicht nachprüfbar; Frage an den Nutzer |

Eine freie Formatbibliothek (pyembroidery) als Gegenprobe ließ sich in dieser Sitzung nicht installieren (Freigabe
fehlt). Mit je einer echten Datei pro Format lassen sich die Leser endgültig prüfen.

## 4. Offen (Stickdateien)
- HUS lesen (Dekompression) – braucht eine Beispieldatei.
- VP3 an einer echten Datei prüfen; JEF-Farbnummern über 30.
- Objektumwandlung (Grade C) – vom Nutzer nicht gewünscht (Entscheidung 22.11.2026).
- Einzelne Stiche bearbeiten (Stitch Edit) – nicht umgesetzt.

## 5. Eigene Rahmen (N7b) — Handbuch Kap. 6 S. 49–52 (Sicherheit hoch für die Bedienung)
Original: Options > Hoop, „New Hoop“, Format **Rectangle** (Height, Width), **Oval** (Rectangle Height, Total Height,
Width), **Sliding** (Sewing Field, Total Height, Width) und **3-Position** (Höhen je Position); „Save Hoop As“ mit Namen.
Rechteck: gestrichelte schwarze Linie; Oval: schwarzer gestrichelter Umriss.

Umsetzung:
- **Rechteck und Oval.** Sliding und 3-Position sind Rahmen anderer Maschinen und für die 185 kein Ziel.
- Oval im Kern (`Rahmen::eigener`): gerader Mittelteil `gerade` mm hoch, oben und unten je eine Halbellipse bis zur
  Gesamthöhe; `gerade` = 0 ergibt eine Ellipse. Die genaue Rundung des Originals steht nicht im Handbuch (eigene
  Festlegung, mittel). Der Oval-Rahmen der 185 selbst kommt weiter aus `hooplibr.dsc`.
- Maße 10 … 1000 mm (eigene Festlegung). Gestalten geht damit auch größer als der MEGA-Rahmen.
- Ablage `<Datenordner>/rahmen.json` (eigene Festlegung; das Original schreibt `hooplibr.dsc`). Ein Design mit eigenem
  Rahmen trägt dessen Maße im Feld `eigener_rahmen` mit und lässt sich darum auch ohne die Ablage öffnen.
- **Senden (Entscheidung des Nutzers 22.11.2026):**
  - Die artista 185 kennt eigene Rahmen nicht. Das Handbuch sagt sinngemäß schon zum gedrehten Rahmen: „When you
    send the design to the machine, however, it does not recognize the hoop in the rotated position“ (S. 49).
  - Deshalb sucht das Programm den kleinsten Rahmen der 185, in den alle Einstiche passen (Freiarm zuletzt), und nennt
    ihn im Plan: „bitte diesen Rahmen einspannen“.
  - Passt keiner, wird nicht gesendet (Fehlercode `ausserhalb`).
  - Gesendet werden dieselben Daten wie sonst: die Maschine bekommt Stiche relativ zur Mitte, keinen Rahmen.
- Oberfläche (Modus E):
  - Assistent Schritt 1: eigene Rahmen in der Liste, Knopf „Eigener Rahmen …“;
  - Inspektor, Reiter Design: Auswahl mit den Gruppen „artista 185“ und „Eigene Rahmen“, Knopf „Eigene …“;
  - Dialog mit Vorschau neben dem MEGA-Rahmen.

## 6. ART-Designs des Originals mit Objekten öffnen (N11, ROADMAP 4b.13c)
Ziel: die 287 Musterdesigns der CD (und eigene ART-Designs aus der artista V4) im Programm öffnen, mit den
**Objekten des Originals** – je Objekt seine echten Stiche, seine Garnfarbe und seine Art.

Kern:
- `stick-formate::art` (Leser);
- angebunden an `stickdatei_lesen` (Endung `.art`, Liste `ENDUNGEN`);
- Test mit `kern/testdaten/art/Seal.ART` (Kopie aus `Samples/` der CD).

### 6.1 Entschlüsseln — Sicherheit: hoch (`Analyse/code/05`, Werkzeuge `art_lesen.py`, `art_decrypt.py`)
- OLE-Datei.
  - **Neuere ART:** Stream `DesignDocument` = Twofish-128-CFB1. Schlüssel „1d43f2ae“ + Format-Schlüssel 0x01c0d932
    (Format-ID 0xD0) + zweimal 0, IV `a885f60d87e90b71c20a81bcc3498af1`.
  - **Ältere ART:** Stream `Contents` = Substitution mit demselben Format-Schlüssel, wie bei ESB.
  - Danach jeweils u32 Länge + zlib.
- **Twofish jetzt im Rust-Kern:**
  - Referenz-Aufbau (q0/q1 aus den Nibble-Tabellen, MDS, RS, Schlüssel-S-Boxen vorab als Tabellen);
  - geprüft mit dem Testvektor der Spezifikation (Schlüssel 0, Klartext 0 → `9F589F5C…`);
  - CFB1 wie `twofish_cfb1.c`: je Bit ein Block.

### 6.2 Datenblock — Sicherheit: hoch (Berichte 02/03)
- Kopf: 29 × u32 (Magic 0x193866de, Prüfsumme = Summe der ersten 28).
- Abschnitt 1: Stichpunkte (h[5] Stück ab h[6]; x in 1/180 mm, **y in 1/360 mm**).
- Abschnitt 2: Segmente (h[8] Stück ab h[9]; Art und Stichzahl).
- Objektteil ab h[14]: Datensätze im gepackten Rahmen.

### 6.3 Neu gelesen: Objekt → Stiche, Objekt → Garn
**Objekt → Segmente** — Sicherheit: hoch (Maschinencode + alle Musterdesigns):
- Jedes Objekt der Objektliste (nach Datensatz 0x0a) hat einen Datensatz der Klasse **0x03**. Leser IMPORT.DLL
  **0x10017180**:
  - 4 gepackte Wörter: Ein- und Ausstich;
  - gepackte Anzahl n, dann n gepackte Wörter = **Nummern der Stichsegmente**;
  - gepackte Anzahl der Farbfolgen, u16;
  - je Farbfolge 2 Byte, ab Dateiversion 0x2b ein **i32 = Garnnummer**, dann weitere Werte (nicht gebraucht).
- **Segmentnummer + 1 = Segment in Abschnitt 2.**
  - Mit +1 liegt jedes Objekt genau zwischen zwei Verbinder-Segmenten (Art 0).
  - Geprüft an 286 lesbaren Musterdesigns: In 282 gehört **jedes Stich-Segment (Art 1) nach dem ersten Objekt zu
    genau einem Objekt**.
  - Ausnahme ist Segment 0 mit 2 Stichen vor dem ersten Objekt (in allen 286). Diese Stiche liegen auf dem ersten
    Einstich (Länge ≈ 0) und markieren den Anfang. Sie fallen weg.
- Die übrigen 4 Designs (Butterfly1, Duck1, Tulip1, Tulip2 – Auto-Digitalisierung) haben nur das AutoStartEnd-Objekt
  0x71. Sie öffnen als ein Objekt mit allen Stichen und Hinweis.
- Das AutoStartEnd-Objekt (0x71) trägt nur den Verbinder zum Anfangs-/Endpunkt und wird übersprungen.

**Garnbibliothek** — Sicherheit: hoch:
- Klasse **0x41**, Leser IMPORT.DLL **0x10039e60**;
- u16 Anzahl, je Garn: R, G, B, 0, drei Längen, dann Nummer, Marke und Name hintereinander, 0;
- in allen 283 Designs mit Garnbibliothek liegt jede Garnnummer aus 0x03 innerhalb der Bibliothek.

**Art des Objekts:**
- Es zählt die letzte Objektklasse vor dem Datensatz 0x03 (Klassen und Namen wie `art_design.py`).
- Füllungen wie MotifFill 0x3d gehören zum Objekt und zählen nicht.
- Deutsche Namen:

| Klasse | Name |
|---|---|
| 0x05 | Linie |
| 0x06, 0x07 | Fläche |
| 0x0c | Satinsäule |
| 0x36 | Ellipse |
| 0x37 | Ring |
| 0x38 | Stern |
| 0x2f | Buchstabe |
| 0x30, 0x7b | Text |
| 0x7a | Motivlinie |
| 0x72, 0x73, 0x93 | Applikation |
| 0x95 | Knopfloch |
| 0x08, 0x49 | Gruppe |
| 0x0b | Manuelle Stiche |

### 6.4 Umsetzung (eigene Festlegungen markiert)
- Je Objekt des Originals ein Objekt `stiche` (wie Grade D, §2):
  - Name „Objektart · Marke Nummer Garnname“, z. B. „Fläche · Sulky 1176 med. dk. Avocado“;
  - Farbe = RGB des Garns; ohne Garn die Standardfolge.
- Stiche der Segmente mit Art 1 werden gestickt.
- **Segmente mit Art 0 (Verbinder) werden Sprünge** und teilen den Zug. Eigene Festlegung, Sicherheit mittel: Ihre
  Stichlängen häufen sich bei 6,5–7,0 mm, also bei der Sprunglänge 0x2712 der Stile.
- Ein Stich-Segment ohne Objekt gehört zum Objekt davor. Das kam in den Musterdesigns nicht vor.
- Die Designmitte liegt auf 0,0. Rahmen und Hinweise wie bei den anderen Stickdateien.
- Die Stiche werden **nicht neu berechnet**, sondern sind die des Originals. So lässt sich jedes Original-Musterdesign
  unverändert sticken und senden.

### 6.5 Geprüft
- Alle 287 Musterdesigns öffnen ohne Fehler (0,01–1,4 s).
- 5 HT-Designs sind größer als jeder Rahmen der 185; sie öffnen mit Hinweis.
- Rose (36 Objekte, 9 Farben) und Seal (9 Objekte) als Vorschau und in der App angesehen: Lage, Ausrichtung und
  Garnfarben sind stimmig.
  - Ein Bild-für-Bild-Vergleich mit der Vorschau des Originals steht aus.
  - „Sampler Alphabets 1“ öffnet mit 6 Textobjekten; das Bild ist nicht angesehen.

### 6.6 Offen
- **Objekte als bearbeitbare Objekte** (Grade A): Umrisse und Werte als `lauf`/`satin`/`tatami`/`text` umwandeln
  statt fester Stiche. Die Bausteine dafür gibt es (Bericht 03: Formen als Bézier-Knoten, Satin-/Lauf-/Tatami-Parameter;
  Werkzeuge `art_objekte.py`, `art_design.py`). Die Zuordnung aller 131 Klassen ist aber eine eigene, große Aufgabe.
  Steht als Frage an den Nutzer in der ROADMAP.
- Weitere Werte je Farbfolge im Datensatz 0x03 (nach der Garnnummer) sind nicht entschlüsselt.
