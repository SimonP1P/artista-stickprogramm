# Roadmap – Stickprogramm für die BERNINA artista 185

**Stand: 20.11.2026.** Diese Datei ist die maßgebliche Roadmap. `CLAUDE.md` §6 fasst sie in einer Tabelle zusammen,
`ARCHITECTURE.md` beschreibt den technischen Aufbau. Nach jedem abgeschlossenen Schritt hier den Stand nachtragen
(✅ erledigt · 🟡 begonnen/teilweise · ⬜ offen · 📅 geplant mit Datum).

Ziel (Nutzer, 18.11.2026): eigene Digitalisier-Software nach dem Vorbild BERNINA artista V4 (Wilcom) –
**Rahmen wählen → Design anpassen → an die Maschine senden**, voller Funktionsumfang, nur für die artista 185.

## Wo wir stehen (Kurzfassung)
| Phase | Inhalt | Stand |
|---|---|---|
| 0 Grundlagen | Software-Analyse, Formate, Werte, Protokoll | ✅ |
| 1 Prototyp | Stichberechnung in Python (Referenz für den Rust-Kern) | ✅ |
| 2 Übertragung | **Lesen ✅** · **Senden ✅ – eigenes Design am 19.11.2026 gestickt** · Farben/Icon wie das Original **vorbereitet** (v4.1, Gerätetest offen) · offen: dauerhaft speichern | 🟡 fast fertig |
| 3 Grundgerüst | Tauri + Svelte + Rust-Kern, erste Oberfläche | ✅ |
| 4 Alle Sticharten | Original-Regeln nachbauen: Satin, Tatami, Lauf, Unterlagen, Laufwege, Schrift, Applikation, Füllmuster, Aussparen, Blanket ✅ · Restliste **Phase 4b** aus der Bestandsaufnahme (`Analyse_Teil3/15`) | 🟡 |
| 5 Maschine im Programm | serieller Port aus Rust ✅, „Senden“ in der Oberfläche ✅ (Schreiben am Gerät offen), Designs aus der Maschine öffnen ✅, Garnfarbkarten ✅, Mac-Programm | 🟡 |

**Stand 20.11.2026:** Am 19.11. wurde das erste Design aus dem eigenen Rechenkern auf der artista 185 gestickt. Am 20.11.
(ohne Maschine) wurde geklärt, warum das Display „no color“ zeigte: Ab Firmware 3.09 schickt das Original die Farben als
eigene **Garnfarbtabelle** (Zeiger 0x24048), nicht im Anhang. Farbtabelle und Icon sind jetzt im Rust-Kern und im
**Testprogramm v4.1** wie das Original umgesetzt (Simulation ok). Weiter geht es mit
1. **Gerätetest v4.1:** ✅ Lesen (20.11.: M = 60, G = 6000; Tabelle der Maschine für Nr. 21 byte-gleich mit unserer) ·
   ✅ Schreiben von ICON + Tabelle (20.11., v4.1 und Rust) · ⬜ Display ansehen → Liste „Vor Ort“ unten. Danach Speichern (0x10/0x200),
2. Phase 5: ✅ **„Senden“ direkt aus dem Programm** (20.11.: `stick-maschine`, Oberfläche Schritt 4; am Gerät lesend
   erprobt, Schreiben aus dem Programm → „Vor Ort“ V1); ✅ Garnfarbkarten; ✅ Designs aus der Maschine öffnen.
   Weiter: Design mit der Maus anpassen, Garn je Farbe wählen, Mac-Programm,
3. Phase 4 weiter: ✅ Schrift „Unterlagen zuerst“ (20.11.); weiter Auto Jump bei Schrift, Kurzstiche bei Tatami, Auto-Split …

## Merkliste für den Nutzer (Stand 20.11.2026)
| # | Aufgabe | Wann | Stand |
|---|---|---|---|
| M1 | **Parameterkatalog melden:** Sobald alle Funktionen entwickelt sind, dem Nutzer sagen, **welche Parameter man verändern kann und könnte** – was einstellbar sein **kann** und was einstellbar sein **sollte**, getrennt nach **einfachem Modus** und **Expertenmodus** der App. Als eine Liste für Designer und Backend: maschinenlesbar aus dem Kern (z. B. `kern/parameter.json`) plus lesbare Fassung (`PARAMETER.md`); je Parameter Name, Gruppe (Stoff, Satin, Tatami, Füllmuster, Unterlage, Zugausgleich, Kurzstiche, Laufwege, Verbinder/Vernähen, Schrift, Applikation, Bild → Umrisse, Maschine …), gilt für (Design/Objektart), Einheit, Standard, Bereich, Quelle (Original-Fundstelle oder eigene Festlegung), Modus (einfach/experte/intern), Wirkung. Der Nutzer liefert dazu noch seine Parameterwünsche. | wenn Phase 4 fertig | ⬜ |
| M2 | **Bild → Umrisse maßstabsfrei machen:** Umrisse ohne feste Breite liefern; Stichart (EasyStitch nach Breite in mm), Wegfall kleiner Details und Abstände erst beim Berechnen nach der endgültigen Größe entscheiden (heute legt `stick bild --breite` die Größe vorab fest) | mit dem Ausbau von Punkt 8 | ⬜ |
| M3 | Neue UI-Mockups ansehen und unter `UI_Mockups/` archivieren (v1/v2 liegen dort, Hinweise in `UI_Mockups/README.md`); App erst ändern, wenn die Oberfläche feststeht | laufend | 🟡 |

## Vor Ort – wartet auf eine Person an der Maschine (Stand 20.11.2026)
Diese Punkte gehen nur, wenn jemand an der artista 185 ist (Display ansehen, Rahmen/Stoff, Start-Taste, Aus/Ein).
Vorher jeweils die Zustimmung des Nutzers einholen (Regel 1a). Testprogramm: `Testprogramm_artista185_Windows_v4_1/`.
| # | Aufgabe | Braucht vor Ort | Stand |
|---|---|---|---|
| V1 | ✅ **20.11.2026 erledigt** (v4.1 Testquadrat und Rust/App-Weg Prüfmuster: alles zurückgelesen gleich, Tabelle gültig) – **Schreibtest v4.1 ohne Sticken:** `START_SCHREIBEN.bat` (Testquadrat: DATEN + ICON + INFO + Garnfarbtabelle, zurücklesen) – oder im Programm Schritt 4 „Senden …“ → „Jetzt schreiben“ (ohne Häkchen „sticken“) | niemanden (kein 0x08) – aber Freigabe des Nutzers 20.11. | ✅ |
| V2 | **Display prüfen:** `START_STICKEN.bat Nr21_puffer.bin` – zeigt das Display Garnfarben (Isacord 40 rich brown / brass / light beige) statt „no color“? Foto | Person: Display ansehen, Nadel oben, Hände weg | ⬜ |
| V3 | Testquadrat mit Icon + Farbtabelle sticken (`START_STICKEN.bat`) – Farbe und Icon am Display? | Person: Rahmen, Stoff + **Vlies**, Stickgarn, Start-Taste | ⬜ |
| V4 | **Prüfmuster** (`stickprogramm/beispiele/pruefmuster.json`: Satin 1/2/3/4/6 mm, Satinkeil, Tatami, Lauf, Dreifach, Schrift „Abc“ Swiss Block; Woven Medium Weight; 82 × 110 mm) auf festem Stoff mit Vlies und Stickgarn sticken, fotografieren – aus dem Programm oder `START_STICKEN.bat Pruefmuster_puffer.bin` | Person, Material | ⬜ (vorbereitet 20.11.) |
| V5 | Dauerhaft speichern (0x10 → schreiben → 0x200), danach in der Designliste und am Display prüfen | Person: Display, ggf. löschen am Gerät | ⬜ (eigene Zustimmung nötig) |
| V7 | **Platzierung prüfen:** ein Design außerhalb der Rahmenmitte mit `"start": "designmitte"` senden – landet es an der gezeichneten Stelle oder zentriert? Entscheidet den Standard für `start`/`ende` | Person: Display/Rahmen ansehen | ⬜ |
| V6 | **Prüfmuster 2 – neue Sticharten** (`stickprogramm/beispiele/pruefmuster2.json`: Füllmuster Raute + Herz, Tatami-Kreis mit ausgesparter Schrift „Ab“, Applikation „Trim in Place“ mit Blanket, Blanket-Linie; 78 × 82 mm, 8 Farben) sticken und fotografieren: `START_STICKEN.bat Pruefmuster2_puffer.bin` – bei der Applikation nach der Platzierungslinie Stoff auflegen, nach der Schnittlinie zuschneiden | Person, Material (Stoff + Vlies + Applikationsstoff) | ⬜ (vorbereitet 20.11.) |

## Neue Erkenntnisse, die die Roadmap ändern (19.11.2026)
1. **Das Lesen ist fertig und hat das Format des Maschinenpuffers vollständig gezeigt** (Kopf 6N+8, Stichdaten in
   EXP-Kodierung mit `80 81`, BE-Container mit Name und Farbtabelle, Icon). Daraus folgt: **Der Container für das
   Senden kann vor Samstag ohne Maschine gebaut und geprüft werden** – Prüfung: die 51 gelesenen Designs aus ihren
   Teilen neu zusammensetzen, Ergebnis muss **byte-gleich** sein. Das senkt das Risiko am Samstag stark.
   → neuer Schritt 2b (vorher in Phase 5). **Erledigt am 19.11.2026:** Rundweg byte-gleich für alle 51 Designs.
2. **Senden in Stufen:** zuerst ein Design, das die Maschine selbst geliefert hat (bekannt gute Bytes), nur in den
   Transferpuffer und „sticken ohne speichern“ (0x08); erst danach ein eigenes Design; Speichern (0x10/0x200) zuletzt.
3. **Die 51 Maschinendesigns sind Vergleichsmaterial:** 7 Zeichensätze (Drifter, Lisa, Blackboard, Swiss Block,
   Zurich, Alice, Bamboo bei 50 %) und 34 Motive mit echten Stichen – damit lässt sich die eigene Stichberechnung
   prüfen, ohne zu sticken.
4. **Die Regeln des Originals hängen an Stichart und Effekt, nicht am Buchstaben** (`Analyse_Teil3/10`): Kurzstiche,
   Unterlage und Laufwege gelten auch für Bilder und Flächen. Phase 4 wird deshalb nach **Regeln** abgearbeitet, nicht
   nach Objektarten.
5. **Die Maschinen-Zeichensätze wurden von Bernina anders eingestellt als der V4-Stil „Schrift“** (fester Abstand
   0,40 mm, kein Zugausgleich, Unterlage je Alphabet). Der Kern folgt dem V4-Stil; ein Schalter „wie Maschinen-Zeichensatz“
   wäre möglich (Entscheidung des Nutzers, offen).
6. **Farben gehen ab Firmware 3.09 nicht im Anhang, sondern als Garnfarbtabelle** an Zeiger 0x24048 (20.11.2026, Code
   0x10003a5d ff.). Der Anhang enthält nur den Kopf. Das erklärt „no color“ beim ersten Test und ergänzt Phase 2 um
   Schritt 2c-2 (Gerätetest v4.1). **Erledigt im Kern und Testprogramm v4.1**, am Gerät offen.
7. **Die Icons der Maschine sind die Stichlinien**, in 72 × 62 eingepasst (96,9 % der Punkte von 43 Motiven getroffen).
   Damit kann jedes eigene Design ein passendes Icon mitschicken; wie V4 selbst das Icon zeichnet, bleibt offen.
8. **Probestickerei 19.11.2026** auf Spannbettlaken ohne Vlies: Geometrie stimmt, Satin ohne Vlies/Unterlage
   zusammengezogen – für Qualitätsvergleiche mit dem Original braucht es festen Stoff, Vlies und Stickgarn (Phase 4,
   Punkt 10).

## Phase 2 – Übertragung zur artista 185
| Schritt | Inhalt | Stand |
|---|---|---|
| 2a Lesen | Verbindung, Stickmodul, Designliste (v3.3), ein Design (v3.5), alle 51 Designs (v3.6), Container entschleiert | ✅ 19.11.2026 (`Analyse_Teil2/07`, `Arbeitsdaten/maschinendesigns/`) |
| 2b Senden vorbereiten (ohne Maschine) | ✅ Puffer + BE-Container + Kopf + Farbtabelle + INFO-Felder im Rust-Kern (`stick-formate/src/artista.rs`), **Rundweg mit allen 51 Designs byte-gleich**, Flags-Formel 38/38 bestätigt, CLI `stick puffer` (JSON/EXP/gelesenes Design → Puffer) · ✅ 20.11.: **Garnfarbtabelle** (Schritt 12) und **Icon** (Schritt 8, aus den Stichen, Regel aus 43 Maschinen-Icons) wie das Original, Anhang nur Kopf; `stick puffer` schreibt `_farben.bin`/`_icon.bin`; **Testprogramm v4.1** (Selbsttest ok) · ✅ Testprogramm v4 (`Testprogramm_artista185_Windows_v4/`): `W` bis zur 256-Byte-Grenze, dann `P S`-Seiten, Zurücklesen + Vergleich, INFO-Felder, Befehl 0x08; Trockenlauf ohne `-Schreiben`, J/N vor jedem Schreibschritt; Selbsttest (Simulation, auch mit Störung) ok; mitgeliefert Nr47_puffer.bin und Testquadrat_puffer.bin | ✅ (am Gerät offen) |
| 2c Senden am Gerät | ✅ **19.11.2026: Nr. 47 vom PC gesendet** (Abschluss 0x100 + `T r M E`) · ✅ **eigenes Testquadrat aus dem Rechenkern gesendet und gestickt** (`P S` am Gerät ok; nach Rahmenwechsel Layout 1 «check») · offen: Farbtabelle (Schritt 12) + Icon wie das Original, dauerhaft speichern (0x10/0x200) · Stufe 1: gelesenes Maschinendesign (Kandidat **Nr. 47**, 328 Byte) zurück in den Puffer DATEN + 0x08 (sticken ohne speichern) · Stufe 2: eigenes kleines Design (Quadrat/Schrift) + 0x08 · Stufe 2b: **v4.1 mit Icon + Garnfarbtabelle** (Trockenlauf liest Grenzen und vorhandene Tabelle; dann Nr. 21 mit 3 Isacord-Farben, Testquadrat) ⬜ · Stufe 3: speichern (0x10 → schreiben → 0x200). **Jede Stufe nur nach ausdrücklicher Zustimmung** (Regel 1a), Person vor Ort an der Maschine | ✅ Stufe 1–2 (19.11.) · ⬜ 2b, 3 |
| 2d offen | ✅ Display nach 0x100 (+ `T r M E`) wieder an; ✅ Inhalt des Anhangs beim Senden (nur Kopf, 20.11.); Bedeutung der Spalte „Farben?“ der Liste (INFO+N+1); welche Kopf-Felder die V4-Software setzt; Icons vollständig lesen (0xAE + 558 Byte); bleibt die Lage im Rahmen erhalten, wenn der Ursprung nicht in der Designmitte liegt? | ⬜ |

## Phase 4 – Sticharten und Regeln nach dem Original
Einzelheiten je Regel: `Analyse_Teil3/10_regeln_original.md`.

**Erledigt (im Rust-Kern):**
- ✅ Satin: Abstand auf derselben Seite (außen im Bogen), Auto Spacing mit Tabelle × Faktor, Zugausgleich
- ✅ Tatami: Abstand zwischen Hinreihen, Versatz 0,25, Mindeststich am Rand
- ✅ Lauf an jedem Eckpunkt, Dreifach
- ✅ Unterlagen: Mittellauf, Edge Walk, Zickzack, Doppelzickzack, Step; Auto Underlay
- ✅ Laufwege statt Sprünge in Flächen
- ✅ Vernähen, Mindest-/Höchststich, Auto Jump
- ✅ **Schrift mit den 50 Original-Alphabeten** (ESB entschlüsselt), Stil „Schrift“: Auto Spacing × 0,9, Kurzstiche,
  Auto-Unterlage Zickzack, Buchstabe als Gruppe mit Laufweg statt Sprung
- ✅ Kurzstiche bei Satin (MOD1 `ShortStitchesEffect`)

**Offen, in dieser Reihenfolge:**
1. 🟡 Schrift: ✅ 20.11.2026 erst alle Unterlagen einer Säulengruppe (rückwärts), dann die Säulen – gemessen an den
   Schrift-Musterdesigns (`Analyse_Teil3/11`), im Kern umgesetzt · ⬜ Reihenfolge, Start und Ende der Säulen wie das
   Original („Closest Join“) – braucht Text/Alphabet der LetterText-Objekte
2. ⬜ Schrift: Auto Jump aus, Breite des Leerzeichens, Zeilenabstand (aus dem Code)
3. ⬜ Kurzstiche bei Tatami-Flächen (niedrige Priorität: Reihenenden an der Kante liegen mindestens einen
   Reihenabstand auseinander, die Schwelle 0,5 × Abstand greift dort kaum – begründet, nicht gemessen); Gruppenbildung
   der Satin-Kurzstiche (MOD1 0x10186ad0) genau übertragen
4. ✅ 20.11.2026 Unterlagenart für Flächen: Step wie die Mehrheit im Original (Messung); Satin-Auto-Split: im Original
   nicht wirksam (531 Querstiche 9–16 mm ungeteilt) – nichts nachzubauen (`Analyse_Teil3/10` §3/§5)
5. 🟡 ✅ 20.11.2026 Designende wie „Start und Ende“ (Auto End: Mitte – Sprung zurück zur Rahmenmitte; Designs der
   Maschine: 34 von 51 enden auf 0,0) · ✅ 20.11.2026 Start/Ende des Designs wie „Start and End“ (Musterdesigns: Auto Start/End = Designmitte; `"start"`/`"ende"`,
   `Analyse_Teil3/10` §7) · Ein-/Ausstich **je Objekt** setzt im Original der Anwender (Reshape; „closest join“ nur beim Auto Digitizer),
   der Kern nimmt den nächsten Einstieg zum vorigen Ende (eigene Festlegung) · ⬜ Reihenfolge der Objekte
6. 🟡 ✅ 20.11.2026 **Applikation** im Kern (`Analyse_Teil3/12`): Platzierung → Stopp → Heften → Stopp → Abdecken,
   Stopps als „Garne“ der Karte `Applique.tch` (Anweisung im Display); Werte aus „Easy Appliqué“ bzw. Handbuch ·
   ✅ 20.11.2026 Schnittlinie („Trim in Place“) und **Blanket**-Abdeckung (ohne Heftstich), Blanket auch als Kontur-
   Stichart 0x2727 (`"art": "blanket"`) · ⬜ Teil-Applikation, Versatz innen/außen, `AutoAppliqueValues` ·
   ✅ 20.11.2026 **Füllmuster (Program Split / Fancy Fill)**: 242 Muster aus der Vorlage
   entschlüsselt (`Analyse_Teil3/13`), Füllung wie im Sampler gemessen (Reihen 0,20 mm, Einstiche auf den Musterlinien),
   `"art": "muster"`, CLI `stick muster` · ⬜ Motivfüllung/Motivlinien
7. ✅ 20.11.2026 Computer-Schriften (TrueType/OpenType, auch .ttc) im Kern: alle installierten Schriften (Windows/Mac),
   Höhe = Großbuchstaben, je Buchstabe **Tatami-Fläche** mit Löchern (eigene Festlegung – Stichart des Originals für
   TrueType nicht belegt) oder Lauf-/Dreifach-Umriss; JSON `"art": "ttf"`, CLI `stick schriften`. Satin entlang der
   Striche später (braucht Säulenzerlegung, zusammen mit Punkt 8). Im Programm: kommt mit der neuen Oberfläche
8. 🟡 Bild → Umrisse – ✅ 20.11.2026 erste Stufe `stick-vektor` / CLI `stick bild`: Farben reduzieren (k-Means),
   Hintergrund weg, Flächen mit Löchern verfolgen (`imageproc`, Suzuki–Abe), vereinfachen, als Tatami-Flächen nach
   Farbe geordnet (alles **eigene Festlegung**, Auto Digitizer des Originals nicht analysiert); an „Apples-pocket.bmp“
   der CD erprobt · ⬜ dünne Linien als Lauf/Satin (EasyStitch-Schwellen), schmale Flächen als Satin, Strichbilder
9. ✅ 20.11.2026 Knockout/Aussparen wie „Remove Overlay Stitching“/„Lettering Remove Overlay“: Loch = Fläche des
   oberen Objekts (an den Musterdesigns exakt gemessen), `"aussparen"`/`"ueberlappung"` an Tatami/Muster
   (`Analyse_Teil3/14`, `stick-kern::aussparen`)
10. 🟡 Probestickereien an der Maschine: erste am 19.11.2026 (Testquadrat, Bettlaken ohne Vlies – Geometrie ok); nächste
    mit festem Stoff, Vlies, Stickgarn: Prüfmuster Satin in mehreren Breiten, Tatami, Kreis, Schrift; auch feine Arbeiten

## Phase 4b – Restliste aus der Bestandsaufnahme (20.11.2026, `Analyse_Teil3/15_funktionsinventur.md`)
Alle Funktionen des Originals (37 Handbuchkapitel, Werkzeuge, Sticharten) gegen den Kern abgeglichen. Reihenfolge =
Arbeitsreihenfolge (erst, was jede Oberfläche braucht, dann Schrift, dann weitere Sticharten und Effekte). Werte und
Abläufe jeweils aus dem Original (Vorlage, Stile, Code, Messung), sonst als eigene Festlegung markiert.
Wenn alles erledigt ist: Parameterkatalog (Merkliste M1).

| # | Block | Inhalt | Stand |
|---|---|---|---|
| 4b.1 | **Transformationen** | Skalieren, Drehen, Spiegeln, Schrägstellen je Objekt und für das ganze Design – Stiche immer neu berechnen (Kap. 15); Oval als Form | ✅ 20.11. `"transform"`, `stick-kern::transformation` |
| 4b.2 | **Schrift vollständig** | ✅ 20.11. mehrere Zeilen, Zeilenabstand 0,6, Wortabstand 0,25, Standardhöhe 10 mm (Vorlage 0x3e82) · offen: Säulenreihenfolge (🟡), Auto Jump aus, Unterlage bei kleiner Schrift; Kursiv/Fett; Grundlinien Kreis/Bogen/senkrecht/eigene; Buchstaben einzeln (Abstand, Lage, Farbe, Winkel); Stichart je Schrift (Satin/Step/Fancy); Monogramme; Elastic Lettering (Kap. 18–21) | 🟡 |
| 4b.3 | **Linien** | Satin-Kontur mit Versatz, Double Run (Verstärken) | ⬜ |
| 4b.4 | **Flächen** | Step-Muster (Nr. 1 …), mehrere Stichwinkel (Turning Fill), Zickzack-Füllung, Tatami-Kurzstiche | ⬜ |
| 4b.5 | **Reihenfolge** | nach Farbe, an Anfang/Ende, automatisch – mit Prüfung, ob sich Objekte überdecken (Kap. 14); Ein-/Ausstich je Objekt setzbar | ⬜ |
| 4b.6 | **Knopflöcher** | einfügen, Typ, Länge, Dichte, mit Objekten verbinden (Kap. 23, Klasse 0x95) | ⬜ |
| 4b.7 | **Motive** | Pattern Run (Motivlinie), Pattern Fill (Motivfüllung), eigene Muster/Sets (Kap. 25) | ⬜ |
| 4b.8 | **Effekte** | erhöhter Satin (Loft), Textured Edge, Wave Effect, Gradient/Open Fill, Color Blending (Kap. 22–24) | ⬜ |
| 4b.9 | **Craft Stitches** | Blackwork Run/Border/Fill, Backstitch, Stemstitch, Candlewicking, Lacework, Stippling (Kap. 26) | ⬜ |
| 4b.10 | **Applikation Rest** | Versatz innen/außen, AutoAppliqueValues, Partial Appliqué (Kap. 27) | ⬜ |
| 4b.11 | **Automatik** | Bild → Umrisse ausbauen (Linien, Satin, maßstabsfrei = M2), Magic Wand, PhotoSnap (Kap. 11–13) | 🟡 |
| 4b.12 | **Anordnen** | Ausrichten, Gruppieren, Mirror-Merge, Kranz, Kaleidoskop (Kap. 15) | ⬜ |
| 4b.13 | **Dateien/Rahmen** | ART-Designs mit Objekten öffnen (Kern), fremde Stickdateien, eigene Rahmen, welche Rahmen die 185 annimmt (Kap. 6, 28) | ⬜ |
| 4b.14 | **Vorlagen** | Stile/Vorlagen speichern wie NORMAL.AMT (Kap. 9) – zusammen mit dem Parameterkatalog | ⬜ |
| 4b.15 | **Kreuzstich** | eigenes Modul (Kap. 34–37) | ⬜ |
Nicht nötig: Designkarte/Magic Box, Scanner, Quilter (eigenes Programm), nur in Strings vorhandene Sticharten (Coil,
Contour, Trapunto …). Reine Oberflächenaufgaben (Zoom, Auswahl, Stichvorschau, Druck, Portfolio) → neue Oberfläche.

## Phase 5 – Maschine im Programm und Komfort
**20.11.2026:** Die Oberfläche wird komplett neu gestaltet (Mockup von Claude Design, Zugriff auf das Repository);
Anforderungen in `app/ANFORDERUNGEN.md`. Bis zum neuen Design keine Änderungen an der App, neue Funktionen nur im Kern.
- ✅ 20.11.2026 `stick-maschine` (Rust, `kern/stick-maschine`): Protokoll der Testprogramme v3.6/v4.1 mit Simulation –
  verbinden, Liste lesen, Design lesen, Garnfarbbereich, planen, senden; CLI `stick maschine …`; **am Gerät lesend
  erprobt** (prüfen 3,8 s, Liste, Nr. 47 mit vollständigem Icon)
- ✅ 20.11.2026 Oberfläche Schritt 4 „An die Maschine“: Anschluss/Simulation, Maschine prüfen, Senden mit Plan-Dialog
  und Bestätigung, Sticken nur mit Häkchen „Person an der Maschine“; am Gerät geprüft und geplant (lesend) · ⬜ Schreiben
  aus dem Programm am Gerät (Liste „Vor Ort“ V1)
- ✅ 20.11.2026 Designs aus der Maschine öffnen: Liste, lesen (0x400), Vorschau mit den Garnfarben der Maschine, als EXP
  speichern – am Gerät erprobt (Nr. 21 mit Isacord rich brown/brass/light beige)
- ✅ 20.11.2026 Garnfarbkarten: 32 Karten der CD, Auswahl im Programm, jede Farbe → nächstes Garn (CIEDE2000, eigene
  Festlegung) → Marke/Nummer/Name in der Garnfarbtabelle; Standard `Default.tch` wie die Maschine („Default Blue“)
- ⬜ Design in der Oberfläche anpassen (verschieben, drehen, skalieren mit der Maus); Garn je Farbe von Hand wählen
- ⬜ Mac-Programm (GitHub Actions, macOS-Runner)

## Erledigte Phasen (Kurzbeleg)
- **Phase 0** – Analyse der Original-Software, Formate, Werte, Protokoll, Lesetest am Gerät: `Analyse/ANALYSE.md`,
  `VERIFIKATION.md`.
- **Phase 1** – Python-Prototyp (Lauf, Dreifach, Satin, Tatami, 5 Unterlagen, 8 Stoffe, Rahmen, EXP, SVG; 47 Tests):
  `stickprogramm/`. Bleibt Referenz und Prüforakel.
- **Phase 3** – Entwicklungsumgebung auf F:, Tauri 2 + Svelte, Rust-Kern (`kern/`, EXP byte-gleich mit Python),
  Oberfläche mit Rahmen, Stoff, Design-JSON, Schrift, Vorschau, Statistik, EXP speichern: `kern/README.md`,
  `app/README.md`.
