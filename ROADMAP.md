# Roadmap – Stickprogramm für die BERNINA artista 185

**Stand: 19.11.2026.** Diese Datei ist die maßgebliche Roadmap. `CLAUDE.md` §6 fasst sie in einer Tabelle zusammen,
`ARCHITECTURE.md` beschreibt den technischen Aufbau. Nach jedem abgeschlossenen Schritt hier den Stand nachtragen
(✅ erledigt · 🟡 begonnen/teilweise · ⬜ offen · 📅 geplant mit Datum).

Ziel (Nutzer, 18.11.2026): eigene Digitalisier-Software nach dem Vorbild BERNINA artista V4 (Wilcom) –
**Rahmen wählen → Design anpassen → an die Maschine senden**, voller Funktionsumfang, nur für die artista 185.

## Wo wir stehen (Kurzfassung)
| Phase | Inhalt | Stand |
|---|---|---|
| 0 Grundlagen | Software-Analyse, Formate, Werte, Protokoll | ✅ |
| 1 Prototyp | Stichberechnung in Python (Referenz für den Rust-Kern) | ✅ |
| 2 Übertragung | **Lesen ✅** (Liste + alle 51 Designs) · **Senden vorbereiten 🟡** · **Senden am Gerät 📅 Sa 21.11.2026** | 🟡 |
| 3 Grundgerüst | Tauri + Svelte + Rust-Kern, erste Oberfläche | ✅ |
| 4 Alle Sticharten | Original-Regeln nachbauen: Satin, Tatami, Lauf, Unterlagen, Laufwege, **Schrift** ✅ · Rest offen | 🟡 8 Punkte erledigt, 10 offen |
| 5 Maschine im Programm | serieller Port aus Rust, „Senden“ in der Oberfläche, Mac-Programm | ⬜ |

**Wir sind in Phase 4 (Sticharten nach dem Original) und bereiten parallel das Senden (Phase 2) für Samstag vor.**

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

## Phase 2 – Übertragung zur artista 185
| Schritt | Inhalt | Stand |
|---|---|---|
| 2a Lesen | Verbindung, Stickmodul, Designliste (v3.3), ein Design (v3.5), alle 51 Designs (v3.6), Container entschleiert | ✅ 19.11.2026 (`Analyse_Teil2/07`, `Arbeitsdaten/maschinendesigns/`) |
| 2b Senden vorbereiten (ohne Maschine) | ✅ Puffer + BE-Container + Kopf + Farbtabelle + INFO-Felder im Rust-Kern (`stick-formate/src/artista.rs`), **Rundweg mit allen 51 Designs byte-gleich**, Flags-Formel 38/38 bestätigt, CLI `stick puffer` (JSON/EXP/gelesenes Design → Puffer) · ⬜ Icon aus den Stichen erzeugen · ⬜ Testprogramm v4 (PowerShell): `W` bis zur 256-Byte-Grenze, dann `P S`-Seiten, INFO-Felder, Befehl 0x08; `-Simulation`, Mitschnitt, J/N-Abfrage vor jedem Schreibbefehl | 🟡 vor Sa 21.11. |
| 2c Senden am Gerät | Stufe 1: gelesenes Maschinendesign (Kandidat **Nr. 47**, 328 Byte) zurück in den Puffer DATEN + 0x08 (sticken ohne speichern) · Stufe 2: eigenes kleines Design (Quadrat/Schrift) + 0x08 · Stufe 3: speichern (0x10 → schreiben → 0x200). **Jede Stufe nur nach ausdrücklicher Zustimmung** (Regel 1a), Person vor Ort an der Maschine | 📅 Sa 21.11.2026 |
| 2d offen | Display nach 0x100 ansehen; Bedeutung der Spalte „Farben?“ der Liste (INFO+N+1); welche Kopf-Variante die V4-Software schreibt; Icons vollständig lesen (0xAE + 558 Byte); bleibt die Lage im Rahmen erhalten, wenn der Ursprung nicht in der Designmitte liegt? | ⬜ |

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
10. ⬜ Probestickereien an der Maschine (braucht Phase 2c), auch feine Arbeiten

## Phase 5 – Maschine im Programm und Komfort
- ⬜ `stick-maschine` (Rust): Protokoll der Testprogramme v3.6/v4 mit Simulation – verbinden, Liste lesen, Design
  lesen, senden
- ⬜ Oberfläche Schritt 4 „Senden“ (mit Rückfrage vor jedem Schreiben), Designs aus der Maschine öffnen
- ⬜ Design in der Oberfläche anpassen (verschieben, drehen, skalieren mit der Maus), Garnfarbkarten
- ⬜ Mac-Programm (GitHub Actions, macOS-Runner)

## Erledigte Phasen (Kurzbeleg)
- **Phase 0** – Analyse der Original-Software, Formate, Werte, Protokoll, Lesetest am Gerät: `Analyse/ANALYSE.md`,
  `VERIFIKATION.md`.
- **Phase 1** – Python-Prototyp (Lauf, Dreifach, Satin, Tatami, 5 Unterlagen, 8 Stoffe, Rahmen, EXP, SVG; 47 Tests):
  `stickprogramm/`. Bleibt Referenz und Prüforakel.
- **Phase 3** – Entwicklungsumgebung auf F:, Tauri 2 + Svelte, Rust-Kern (`kern/`, EXP byte-gleich mit Python),
  Oberfläche mit Rahmen, Stoff, Design-JSON, Schrift, Vorschau, Statistik, EXP speichern: `kern/README.md`,
  `app/README.md`.
