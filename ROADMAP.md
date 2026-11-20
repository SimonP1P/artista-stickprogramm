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
| 4 Alle Sticharten | Original-Regeln nachbauen: Satin, Tatami, Lauf, Unterlagen, Laufwege, **Schrift** ✅ · Rest offen | 🟡 8 Punkte erledigt, 10 offen |
| 5 Maschine im Programm | serieller Port aus Rust ✅, „Senden“ in der Oberfläche ✅ (Schreiben am Gerät offen), Designs aus der Maschine öffnen, Garnfarbkarten, Mac-Programm | 🟡 |

**Stand 20.11.2026:** Am 19.11. wurde das erste Design aus dem eigenen Rechenkern auf der artista 185 gestickt. Am 20.11.
(ohne Maschine) wurde geklärt, warum das Display „no color“ zeigte: Ab Firmware 3.09 schickt das Original die Farben als
eigene **Garnfarbtabelle** (Zeiger 0x24048), nicht im Anhang. Farbtabelle und Icon sind jetzt im Rust-Kern und im
**Testprogramm v4.1** wie das Original umgesetzt (Simulation ok). Weiter geht es mit
1. **Gerätetest v4.1:** ✅ Lesen (20.11.: M = 60, G = 6000; Tabelle der Maschine für Nr. 21 byte-gleich mit unserer) ·
   ⬜ Schreiben von ICON + Tabelle und Display ansehen → Liste „Vor Ort“ unten. Danach Speichern (0x10/0x200),
2. Phase 5: ✅ **„Senden“ direkt aus dem Programm** (20.11.: `stick-maschine`, Oberfläche Schritt 4; am Gerät lesend
   erprobt, Schreiben aus dem Programm → „Vor Ort“ V1). Weiter: Garnangaben (Marke, Nummer, Name) aus den
   Garnfarbkarten, damit eigene Designs echte Garnfarben mitschicken; Designs aus der Maschine öffnen,
3. Phase 4 weiter: Schrift-Reihenfolge, Kurzstiche bei Tatami, Auto-Split …

## Vor Ort – wartet auf eine Person an der Maschine (Stand 20.11.2026)
Diese Punkte gehen nur, wenn jemand an der artista 185 ist (Display ansehen, Rahmen/Stoff, Start-Taste, Aus/Ein).
Vorher jeweils die Zustimmung des Nutzers einholen (Regel 1a). Testprogramm: `Testprogramm_artista185_Windows_v4_1/`.
| # | Aufgabe | Braucht vor Ort | Stand |
|---|---|---|---|
| V1 | **Schreibtest v4.1 ohne Sticken:** `START_SCHREIBEN.bat` (Testquadrat: DATEN + ICON + INFO + Garnfarbtabelle, zurücklesen) – oder im Programm Schritt 4 „Senden …“ → „Jetzt schreiben“ (ohne Häkchen „sticken“) | niemanden (kein 0x08) – aber Freigabe: der Nutzer startet es selbst oder gibt Claude Code die Berechtigung (am 20.11. von der Rechteprüfung abgelehnt) | ⬜ |
| V2 | **Display prüfen:** `START_STICKEN.bat Nr21_puffer.bin` – zeigt das Display Garnfarben (Isacord 40 rich brown / brass / light beige) statt „no color“? Foto | Person: Display ansehen, Nadel oben, Hände weg | ⬜ |
| V3 | Testquadrat mit Icon + Farbtabelle sticken (`START_STICKEN.bat`) – Farbe und Icon am Display? | Person: Rahmen, Stoff + **Vlies**, Stickgarn, Start-Taste | ⬜ |
| V4 | **Prüfmuster** (`stickprogramm/beispiele/pruefmuster.json`: Satin 1/2/3/4/6 mm, Satinkeil, Tatami, Lauf, Dreifach, Schrift „Abc“ Swiss Block; Woven Medium Weight; 82 × 110 mm) auf festem Stoff mit Vlies und Stickgarn sticken, fotografieren – aus dem Programm oder `START_STICKEN.bat Pruefmuster_puffer.bin` | Person, Material | ⬜ (vorbereitet 20.11.) |
| V5 | Dauerhaft speichern (0x10 → schreiben → 0x200), danach in der Designliste und am Display prüfen | Person: Display, ggf. löschen am Gerät | ⬜ (eigene Zustimmung nötig) |

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
1. ⬜ Schrift: erst alle Unterlagen eines Buchstabens, dann die Säulen; Reihenfolge, Start und Ende der Säulen wie
   das Original („Closest Join“ / „As Digitized“)
2. ⬜ Schrift: Auto Jump aus, Breite des Leerzeichens, Zeilenabstand (aus dem Code)
3. ⬜ Kurzstiche bei Tatami-Flächen; Gruppenbildung (MOD1 0x10186ad0) genau übertragen
4. ⬜ Unterlagenart für Flächengruppen (`IsUnderlayTypeApplicable`); Satin-Auto-Split ab 10 mm
5. ⬜ Start-/Endpunkte und Reihenfolge der Objekte im ganzen Design
6. ⬜ Fancy-/Musterfüllungen, Applikation
7. ⬜ Computer-Schriften (TTF) als Umrisse
8. ⬜ Bild → Umrisse (Vektorisierung: Potrace/VTracer)
9. ⬜ Knockout/Überlappung
10. 🟡 Probestickereien an der Maschine: erste am 19.11.2026 (Testquadrat, Bettlaken ohne Vlies – Geometrie ok); nächste
    mit festem Stoff, Vlies, Stickgarn: Prüfmuster Satin in mehreren Breiten, Tatami, Kreis, Schrift; auch feine Arbeiten

## Phase 5 – Maschine im Programm und Komfort
- ✅ 20.11.2026 `stick-maschine` (Rust, `kern/stick-maschine`): Protokoll der Testprogramme v3.6/v4.1 mit Simulation –
  verbinden, Liste lesen, Design lesen, Garnfarbbereich, planen, senden; CLI `stick maschine …`; **am Gerät lesend
  erprobt** (prüfen 3,8 s, Liste, Nr. 47 mit vollständigem Icon)
- ✅ 20.11.2026 Oberfläche Schritt 4 „An die Maschine“: Anschluss/Simulation, Maschine prüfen, Senden mit Plan-Dialog
  und Bestätigung, Sticken nur mit Häkchen „Person an der Maschine“; am Gerät geprüft und geplant (lesend) · ⬜ Schreiben
  aus dem Programm am Gerät (Liste „Vor Ort“ V1)
- ⬜ Designs aus der Maschine öffnen (Liste anzeigen, lesen, Vorschau)
- ⬜ Design in der Oberfläche anpassen (verschieben, drehen, skalieren mit der Maus), Garnfarbkarten (Marke, Nummer und
  Name je Farbe → Garnfarbtabelle an die Maschine)
- ⬜ Mac-Programm (GitHub Actions, macOS-Runner)

## Erledigte Phasen (Kurzbeleg)
- **Phase 0** – Analyse der Original-Software, Formate, Werte, Protokoll, Lesetest am Gerät: `Analyse/ANALYSE.md`,
  `VERIFIKATION.md`.
- **Phase 1** – Python-Prototyp (Lauf, Dreifach, Satin, Tatami, 5 Unterlagen, 8 Stoffe, Rahmen, EXP, SVG; 47 Tests):
  `stickprogramm/`. Bleibt Referenz und Prüforakel.
- **Phase 3** – Entwicklungsumgebung auf F:, Tauri 2 + Svelte, Rust-Kern (`kern/`, EXP byte-gleich mit Python),
  Oberfläche mit Rahmen, Stoff, Design-JSON, Schrift, Vorschau, Statistik, EXP speichern: `kern/README.md`,
  `app/README.md`.
