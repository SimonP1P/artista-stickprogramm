# Roadmap – Stickprogramm für die BERNINA artista 185

**Stand: 21.11.2026.** Diese Datei ist die maßgebliche Roadmap. `CLAUDE.md` §6 fasst sie in einer Tabelle zusammen,
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
| 4 Alle Sticharten | Grundstock ✅ (Satin, Tatami, Lauf, Unterlagen, Laufwege, Schrift, Applikation, Füllmuster, Aussparen, Blanket) · **Phase 4b**: **4b.0 Schnittstelle für die Oberfläche ✅** (`SCHNITTSTELLE.md`, `kern.ts`, `stick-api`), **4b.16 Werte je Objekt ✅**, 4b.1–4b.6 ✅, **4b.7 Motivlinie ✅** (Motivfüllung offen), 4b.17 SVG einlesen ✅, 4b.8/4b.11 begonnen, 4b.8–4b.15 offen · Parameterkatalog `PARAMETER.md` (167 Parameter) | 🟡 |
| 5 Maschine im Programm + neue Oberfläche | serieller Port aus Rust ✅, **Oberfläche v3 (E0–E6) ✅ 21.11.** (**am Gerät gesendet und gestickt 21.11.**, Vor Ort V10), Designs aus der Maschine ✅, Garnfarbkarten ✅, Mac-Programm (Bau auf dem Mac ✅, läuft im Fenster ✅), **Elemente + Gruppen** ✅ 21.11., Senden auch am Mac | 🟡 |

## Übergabe für die nächste Sitzung (Stand 21.11.2026, spät)
**Hier weitermachen.** Alles ist committet und gepusht (`main`).

**Zuletzt erledigt (20.11.):** Phase 4b.1–4b.4 (Transformationen, Schrift vollständig, Linien, Flächen), Messungen an
den Musterdesigns (Säulenreihenfolge der Schrift, Step-Muster, Wendefüllung), Korrektur Step-Versatz der Stoffe
(0,25/0,75), **Parameterkatalog** (`PARAMETER.md`, 145 Parameter, Modus einfach/experte/automatisch),
**Funktionsübersicht für Designer/Frontend** (`UI_Mockups/FUNKTIONEN.md`), **4b.5 Reihenfolge** (Umordnen wie Menü
Arrange, Überdeckungsprüfung als eigene Erweiterung, Ein-/Ausstich je Objekt, Closest Join; Bericht
`Analyse_Teil3/16`, CLI `stick reihenfolge`), **4b.17 Baustein „Satin-Stichrichtung automatisch“** (`stick_kern::saeulen`,
JSON `satinflaeche`, TrueType in Satin, CLI `stick saeulen`; Bericht `Analyse_Teil3/17`).

**Erledigt 20.11. (spät): 4b.0 Schnittstelle für die Oberfläche** – die Oberfläche kann jetzt beginnen: Feldliste
`SCHNITTSTELLE.md`, TypeScript `kern/schnittstelle/kern.ts`, Befehle in `kern/stick-api` (ein Tauri-Befehl `kern`).
Pflege nach **CLAUDE.md §4 Regel 7** (Wächter-Tests erzwingen sie). Der Designentwurf des Nutzers (Claude Design) ist
teils veraltet – mit `SCHNITTSTELLE.md` + `UI_Mockups/FUNKTIONEN.md` abgleichen.

**Erledigt 21.11.: 4b.16 Werte je Objekt** – `"einstellungen"` an jedem Objekt (wie die Objekteigenschaften des
Originals, auch Verbinder je Objekt); Katalog-Ebene „beide“ für alle Stil-Werte.

**Erledigt 21.11.: 4b.17 SVG einlesen** – `stick svg`, Befehl `svg_lesen`: Stichart nach Breite wie EasyStitch des
Originals (Vorlagen ↔ Stile `<PRESET_EASYSTITCH>` über die Flags belegt; Lauf 2,0 mm, Tatami 0,45 mm, Zugausgleich der
Flächen 0,40 mm bestätigt), Dicke = größter Innenkreis (`saeulen::dicke`, eigene Festlegung), Überdeckung aussparen,
Closest Join; Bericht `Analyse_Teil3/18`. Offen dort §5 (Thin/Thick Lines, Breitenmaß des Originals).

**Erledigt 21.11. (spät): Satin „gerade quer + schräg vor“** – gegen die gespeicherten Stichlinien von 125 Säulen
gemessen (`Analyse_Teil3/04` §5, `art_satinmuster.py`): A und B einer Stelle liegen auf derselben Stichlinie. Im Kern für
alle Satinsäulen (Referenz-Stufe 11); Step-Reihen in Säulen und Zickzack 0x2724 unverändert (nicht gemessen).

**Erledigt 21.11. (spät): 4b.6 Knopflöcher** – 9 Typen aus `ButtonHoles.ESB`, Verfahren und Standardwerte aus MOD1,
gegen das Musterdesign `Buttonholes.ART` gemessen; JSON `"art": "knopfloch"`, CLI `stick knopfloecher`, Beispiel
`stickprogramm/beispiele/knopfloecher.json`; Bericht `Analyse_Teil3/19`. Offen dort §6 (Merge im Code bestätigen,
Vernähen je Teil).

**Neue Oberfläche (21.11.) ✅:** Entwurf v3 (`UI_Mockups/v3_2026-11-21/`) in `app/` umgesetzt, Etappen **E0–E6**
(Tabelle in Phase 5, Stand je Etappe und Bildschirmfotos in `app/README.md`). Gebaut und geprüft auf dem **Mac** (Rust
dort seit 21.11. in `~/.cargo`, `CARGO_TARGET_DIR=kern/target`); Bildschirmfotos ohne Fenster mit
`app/werkzeuge/bildschirmfoto.py` (das Terminal darf auf dem Mac keine Bildschirmaufnahme). **Maschine am Gerät
erprobt** – Vor Ort **V10** ✅ 21.11. (gesendet und gestickt, Bericht Nutzer). Offene Kern-Schritte der Oberfläche:
W12 (Phase 5); **W5-Rest ✅ 21.11.** (Befehl `in_knoten_umwandeln`, Knoten mit Bézier-Anfassern), **W9 ✅ 21.11.**
(Buchstabe je Stich, einzelne Buchstaben per Klick). **Schriftauswahl mit Google Fonts und „stickbar ab“ ✅ 21.11.**
(Phase 5, `Analyse_Teil3/21`) mit Stil-Einordnung aller Schriften und eigenen Stilen.
Merkliste M1: Aufteilung des Parameterkatalogs nach einfach/experte **vom Nutzer bestätigt (21.11.)**.

**Erledigt 21.11.: 4b.7 Motivlinie (Pattern Run)** – 14 Motivsätze der CD (`userletw/`, Ressource
`app/src-tauri/motive/`), Regel des Originals an 156 Motivlinien der Musterdesigns gemessen (Anzahl = Linienlänge /
Größe gerundet, gleichmäßig verteilt, Größe bleibt); JSON `"art": "motivlinie"`, Befehle `motive_liste`,
`motivsatz_motive`; Oberfläche: Einfügen → Motivlinie, Inspektor „Motiv“ mit Auswahl-Dialog (Vorschaubilder);
Bericht `Analyse_Teil3/20_motive.md`.

**Nächster Schritt:** **4b.7 Motivfüllung** (Pattern Fill; Raster nicht messbar → nach Handbuch, eigene Festlegung
kennzeichnen, Bericht 20 §4), danach 4b.8 Effekte … Dabei jeweils CLAUDE.md §4 Regel 7 (Schnittstelle) beachten.

**Hinweise aus der letzten Sitzung (20.11.):**
- **Freigabe des Nutzers:** Programmcode des Originals darf bei Bedarf entschlüsselt werden (ARTISTA.EXE ist mit
  HASP verschlüsselt, Entropie 8,0 – die Menübefehle wie „Sequence By Color“ stehen nur dort). Bisher war es nicht
  nötig: EZSTITCH.DLL, MOD1.DLL, GEOMETRY.DLL, IMPORT.DLL sind lesbar.
- Offen aus 4b.5 (Bericht 16 §5): Platzierung von „Sequence By Color“ bei Teilauswahl, Ausstich in Flächen wie das
  Original, Bedeutung „Auto Sequence“.
- Offen aus 4b.17-Baustein (Bericht 17 §6): Einzelregeln `SignificantTriangles`/Schnittstufen, Überlappung an
  Abzweigungen („Create Overlaps“, „T“-Abzweigungen 0,3 mm). Prüfdaten: `art_autosatin.py export/vergleich`
  (54 Säulen aus `Samples/AutoDigitizing`, Vergleich mit `stick saeulen`).

**Arbeitsweise (vom Nutzer festgelegt):**
- Werte und Abläufe aus dem Original: Stile `Analyse/daten/entschluesselt/styles.txt`, Code (DIALOG.DLL, MOD1 –
  `Arbeitsdaten/werkzeuge/rdis.py`, capstone/pefile im `venv`), Handbuch `Arbeitsdaten/texte/man_raw_pg.txt`,
  **Messung an den 287 Musterdesigns** (`F:/Entwicklung/artista_ziel/Main/Samples`, Werkzeuge `Arbeitsdaten/werkzeuge/art_*.py`,
  Python mit `venv/Scripts/python.exe`). Eigene Festlegungen nur als markierte Notlösung.
- **Oberfläche (21.11.2026):** Die Sperre „App nicht ändern“ ist **aufgehoben** – der Entwurf v3
  (`UI_Mockups/v3_2026-11-21/UEBERGABE.md`) wird in `app/` umgesetzt. Die Oberfläche rechnet keine Geometrie und keine
  Stiche; sie ruft den Kern nur über `kern(…)` aus `kern/schnittstelle/kern.ts` (per Alias eingebunden). Fehlt ein
  Feld/Befehl: im Kern ergänzen (Regel 7). Maschinen-Abläufe der alten Oberfläche bleiben erhalten.
- Nach jedem Schritt: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` (Umgebung
  CLAUDE.md §5), App mit `cargo check` in `app/src-tauri`; Doku pflegen:
  1. `stickprogramm/README.md` „Design beschreiben (JSON)“ (neue Felder),
  2. `UI_Mockups/FUNKTIONEN.md` (Funktion, Einstellung, Standard, JSON, Modus),
  3. **Parameterkatalog:** neue Werte in `Arbeitsdaten/werkzeuge/parameter_katalog.py` eintragen →
     `py Arbeitsdaten/werkzeuge/parameter_katalog.py` → `stick parameter --markdown > PARAMETER.md`
     (Tests `stick_kern::parameter` schlagen fehl, wenn ein Design-Wert fehlt oder ein Standard nicht stimmt),
  4. `Analyse_Teil3/15_funktionsinventur.md`, diese Roadmap (Phase-4b-Tabelle), Fundstellen in `Analyse_Teil3/…`,
  5. **Schnittstelle** (CLAUDE.md §4 Regel 7): Felder nur in `stick-kern/src/format.rs`, Befehle nur in `stick-api`,
     dann `stick api exportieren` und `SCHNITTSTELLE.md` + `kern/schnittstelle/*` mitcommitten; Eintrag in
     `UI_Mockups/AENDERUNGEN.md` (für den UI-Entwickler),
  6. ändert sich die Stichberechnung der Referenz-Designs: `kern/testdaten/referenz/` ersetzen und im dortigen
     `README.md` als neue Stufe eintragen.
- Commit + Push nach jedem Schritt, Nachricht auf Deutsch (Trailer: Co-Authored-By / Claude-Session).
- Dem Nutzer zu Beginn jeder Runde sagen, wo wir in der Roadmap stehen.
- Praxis: Patches als Python-Datei schreiben (Heredocs mit Backslashes brechen unter Windows); Bildprobe einer
  Stickdatei mit `stick erzeugen … --alphabete F:/Entwicklung/artista_ziel/Main/PROGRAM/wletter` und Edge headless.

**Maschine:** Senden/Sticken funktioniert (19.11.). Offene Gerätetests stehen in „Vor Ort“ (Person an der Maschine nötig);
Schreib-/Löschbefehle nur nach ausdrücklicher Zustimmung (CLAUDE.md §4 Regel 1/1a).

## Merkliste für den Nutzer (Stand 20.11.2026)
| # | Aufgabe | Wann | Stand |
|---|---|---|---|
| M1 | **Parameterkatalog melden:** Sobald alle Funktionen entwickelt sind, dem Nutzer sagen, **welche Parameter man verändern kann und könnte** – was einstellbar sein **kann** und was einstellbar sein **sollte**, getrennt nach **einfachem Modus** und **Expertenmodus** der App. Als eine Liste für Designer und Backend: maschinenlesbar aus dem Kern (z. B. `kern/parameter.json`) plus lesbare Fassung (`PARAMETER.md`); je Parameter Name, Gruppe (Stoff, Satin, Tatami, Füllmuster, Unterlage, Zugausgleich, Kurzstiche, Laufwege, Verbinder/Vernähen, Schrift, Applikation, Bild → Umrisse, Maschine …), gilt für (Design/Objektart), Einheit, Standard, Bereich, Quelle (Original-Fundstelle oder eigene Festlegung), Modus (einfach/experte/intern), Wirkung. Der Nutzer liefert dazu noch seine Parameterwünsche. | wenn Phase 4 fertig | ✅ **Aufteilung einfach/experte/automatisch vom Nutzer bestätigt 21.11.** („passt so“; 162 Parameter: 38 einfach, 78 experte, 46 automatisch); weiter mit jeder neuen Funktion ergänzen · 🟡 **angelegt 20.11.** (Wunsch Nutzer: schon jetzt): `PARAMETER.md`, `kern/stick-kern/daten/parameter.json` (132 Parameter: 31 einfach, 70 experte, 31 automatisch), CLI `stick parameter`; Tests halten ihn aktuell; wird mit jeder Funktion ergänzt; offen: Parameterwünsche des Nutzers, je Objekt überschreibbare Werte nachrüsten |
| M2 | **Bild → Umrisse maßstabsfrei machen:** Umrisse ohne feste Breite liefern; Stichart (EasyStitch nach Breite in mm), Wegfall kleiner Details und Abstände erst beim Berechnen nach der endgültigen Größe entscheiden (heute legt `stick bild --breite` die Größe vorab fest) | mit dem Ausbau von Punkt 8 | ⬜ |
| M3 | Neue UI-Mockups ansehen und unter `UI_Mockups/` archivieren (v1/v2 liegen dort, Hinweise in `UI_Mockups/README.md`); App erst ändern, wenn die Oberfläche feststeht | laufend | ✅ 21.11.: v3 unter `UI_Mockups/v3_2026-11-21/`, verbindlich, Sperre aufgehoben |

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
| V8 | **SVG-Probe:** `stickprogramm/beispiele/blume.svg` mit `stick svg` einlesen (Blume aus Inkscape: Tatami mit Aussparung, Satinflächen, Laufstiche, Ring mit Loch; 90 × 84 mm, Rahmen 130 × 100) und sticken – passen Flächen, Satin der Blätter, Aussparungen ohne Lücken? Foto | Person, Material (Stoff + Vlies) | ⬜ (vorbereitet 21.11.) |
| V9 | **Knopflöcher:** `stickprogramm/beispiele/knopfloecher.json` (alle 9 Typen + 2 eingearbeitete in einer Fläche; mit `--alphabete …/wletter` erzeugen) auf festem Stoff mit Vlies sticken, Schlitz aufschneiden – Raupen dicht genug, Riegel fest, Fläche am Schlitz sauber? Foto | Person, Material (Stoff + Vlies) | ⬜ (vorbereitet 21.11.) |
| V10 | **Neue Oberfläche am Gerät** (nach Zustimmung): Windows-PC, App bauen, Anschluss COM3 – 1. „Maschine prüfen“ (nur lesen), 2. „Designs aus der Maschine“ Liste + ein Design lesen (nur lesen), 3. Senden ohne Sticken (Plan → Bestätigen), 4. „Abbrechen“ während des Sendens probieren, 5. erst dann mit Person an der Maschine „Jetzt sticken …“ | Person: Display, Rahmen, Start | ✅ 21.11. (Bericht Nutzer: COM3 gefunden, eingestellt, gesendet, **sauber gestickt**; Einzelschritte 1–4 nicht gemeldet; `Analyse_Teil2/07`) |
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
   der CD erprobt · ✅ 21.11.2026 zweite Stufe (Nutzer-Test mit einem Foto: „noch nicht brauchbar“, leere Stellen):
   Vorbereitung nach Handbuch Kap. 12 (vereinfachen = Kuwahara, k-Means im Lab-Raum, Garnfarben, kleine Flächen der
   Nachbarfarbe zuschlagen statt weglassen), Farben weglassen wie „Omitted Colors“ (Vorschlag nur weißer Hintergrund),
   „Fills: Step/Auto“ (Standard Step – Satin franst bei Fotos aus und rechnet 10× länger), Zugausgleich 0,4 mm und
   Unterlage (Handbuch S. 116), Befehl `bild_vorschau` + Dialog mit Vorschau · ⬜ dünne Linien als Lauf/Satin
   („Details“), PhotoSnap für Fotos (Original: Satinreihen mit wechselndem Abstand – erst analysieren), Strichbilder
9. ✅ 20.11.2026 Knockout/Aussparen wie „Remove Overlay Stitching“/„Lettering Remove Overlay“: Loch = Fläche des
   oberen Objekts (an den Musterdesigns exakt gemessen), `"aussparen"`/`"ueberlappung"` an Tatami/Muster
   (`Analyse_Teil3/14`, `stick-kern::aussparen`)
10. 🟡 Probestickereien an der Maschine: erste am 19.11.2026 (Testquadrat, Bettlaken ohne Vlies – Geometrie ok); nächste
    mit festem Stoff, Vlies, Stickgarn: Prüfmuster Satin in mehreren Breiten, Tatami, Kreis, Schrift; auch feine Arbeiten

## Phase 4b – Restliste aus der Bestandsaufnahme (20.11.2026, `Analyse_Teil3/15_funktionsinventur.md`)
Alle Funktionen des Originals (37 Handbuchkapitel, Werkzeuge, Sticharten) gegen den Kern abgeglichen. Reihenfolge =
Arbeitsreihenfolge (erst, was jede Oberfläche braucht, dann Schrift, dann weitere Sticharten und Effekte). Die Tabelle steht in Arbeitsreihenfolge (4b.16/4b.17 nachträglich vorgezogen). Werte und
Abläufe jeweils aus dem Original (Vorlage, Stile, Code, Messung), sonst als eigene Festlegung markiert.
Parameterkatalog (Merkliste M1) seit 20.11. angelegt (`PARAMETER.md`) und mit jeder Funktion ergänzt; wenn alles erledigt ist,
dem Nutzer zur Abstimmung vorlegen.

| # | Block | Inhalt | Stand |
|---|---|---|---|
| 4b.0 | **Schnittstelle für die Oberfläche** (Wunsch Nutzer 20.11.: Oberfläche jetzt beginnen und mit jeder Kern-Funktion synchron erweitern) | **Ziel:** Die Oberfläche kann jetzt gebaut werden; jede neue Kern-Funktion erscheint **automatisch** mit Typen, Standardwerten und Doku. 1. **Design-Format als Schema aus Rust:** alle Felder des Design-JSON in festen Strukturen (heute teils „roh“ gelesen: `transform`, `einstich`, `farben`, `buchstaben` …; `garnkarte` liest nur die App), Auswahlwerte als Aufzählungen, `"version"` mit Umwandlung alter Fassungen; JSON-Schema (`schemars`) mit Standardwerten und Beschreibungen aus den Doc-Kommentaren · 2. **Befehlsschicht `kern/stick-api`:** alle Befehle der Oberfläche an einer Stelle (Ein-/Ausgabetypen, ein Verteiler `ausfuehren(befehl, daten)`) – die App bindet nur noch einen Befehl ein, neue Kern-Befehle brauchen keine App-Änderung; CLI `stick api` · 3. **Ergebnis für die Oberfläche:** Objektnummer je Stich (Auswahl/Hervorheben), **Formen je Objekt** (aufgelöste Umrisse, Linien, Griffe für Ein-/Ausstich) zum Bearbeiten mit der Maus, Hinweise (unbekannte Felder), Fehler mit Objektnummer · 4. **Fehlende Befehle** (`UI_Mockups/FUNKTIONEN.md` §6): Parameterkatalog, Füllmuster mit Vorschau, Computer-Schriften, Bild → Umrisse, Reihenfolge/Überdeckungen, Säulen-Vorschau · 5. **Erzeugte Dateien + Wächter:** `kern/schnittstelle/` (`design.schema.json`, `befehle.json`, `kern.ts` mit TypeScript-Typen und Aufrufen) und `SCHNITTSTELLE.md` (lesbare Feld- und Befehlsliste) werden aus dem Code erzeugt; Tests schlagen fehl, wenn sie veraltet sind, wenn `FUNKTIONEN.md` eine Objektart/einen Befehl nicht nennt oder der Parameterkatalog ein Feld nennt, das es nicht gibt; Pflege-Regel in `CLAUDE.md` §4 | ✅ 20.11.: `stick_kern::format` (alle Felder in festen Strukturen, Auswahlwerte, `version`, `garnkarte`, Monogramm im Format, Fehler deutsch + Objektnummer; Stiche unverändert, Referenzen byte-gleich) · `StichFolge::objekt` + `Design::formen()` · `kern/stick-api` mit 20 Befehlen (Listen inkl. Garne einer Karte, Füllmuster mit Kacheln, Schriften, Parameterkatalog; Design prüfen/berechnen/Formen/EXP; Reihenfolge/Überdeckungen; Säulen-Vorschau; Bild umwandeln; Dateien), CLI `stick api` · erzeugt: `SCHNITTSTELLE.md`, `kern/schnittstelle/` (`kern.ts` mit `tsc --strict` geprüft) · 3 Wächter-Tests · Regel 7 in `CLAUDE.md` §4. **Offen:** App bindet `kern` ein, sobald die neue Oberfläche entsteht (Code-Schnipsel im Kopf von `stick-api/src/lib.rs`); Maschinen-Befehle nach `stick-api` umziehen |
| 4b.1 | **Transformationen** | Skalieren, Drehen, Spiegeln, Schrägstellen je Objekt und für das ganze Design – Stiche immer neu berechnen (Kap. 15); Oval als Form | ✅ 20.11. `"transform"`, `stick-kern::transformation` |
| 4b.2 | **Schrift vollständig** | ✅ 20.11. mehrere Zeilen, Zeilenabstand 0,6, Wortabstand 0,25, Standardhöhe 10 mm (Vorlage 0x3e82); ✅ Kursiv ±45°; ✅ **Grundlinien wie Dialog „Layout“**: Kreis CW/CCW mit Oval (Radius X/Y), Tilt, Arc Angle, Position über/durch/unter (CCW standardmäßig unter = außen – Korrektur zum ersten Stand), gerade mit Winkel/Länge, senkrecht, eigene Kurve, mehrfach (oben – Mitte – unten); Ausrichtung „Full“ (`block`), Text wird bei fester Länge eingepasst; ✅ Farbwechsel `^` + `farben`; ✅ Buchstaben einzeln (Farbe, verschieben, drehen, skalieren); ✅ Stichart je Schrift: Step (Reihen folgen den Stichwinkeln der Säulen) und Füllmuster (Fancy, über die Buchstabenfläche) – Verteilung eigene Festlegung, keine Messung möglich (keine solche Schrift in den Musterdesigns); ✅ Monogramme (`monogramm`: Diamond2/3, Raute/Zier, Belegung aus den Glyphen); ✅ Elastic Lettering gerade/gebogen (`elastisch`, Kantenform eigene Festlegung); Fett = mehr Zugausgleich (über `einstellungen`); Unterlage bei kleiner Schrift = `"unterlage": []` · ✅ Auto Jump je Objekt (`auto_jump`, Schrift standardmäßig aus wie Stil Schrift, eigene Werte = Loft) ✅ Stichwinkel je Buchstabe (`winkellinien`) 🟡 Säulenreihenfolge: gemessen (Original wählt Reihenfolge und Richtung, 38 % rückwärts, teilt 47 % der Säulen – Bericht 11 §5), Kern wählt jetzt Reihenfolge **und Richtung** ✅ · **eventuell später:** Säulen in Stücke teilen wie das Original (47 % der Säulen; Aufwand hoch – Code-Analyse des Lettering-Sequencing –, Nutzen gering: ändert nur Wege innerhalb eines Buchstabens, nicht das Aussehen; Entscheidung Nutzer 20.11.); Umriss einzelner Buchstaben umformen = Aufgabe der Oberfläche (Kap. 18–21) | ✅ (Rest: eventuell später) |
| 4b.3 | **Linien** | Satin-Kontur mit Versatz, Double Run (Verstärken) | ✅ 20.11. `satinlinie` `"versatz": "seite"` (Seite links wie Blanket, eigene Festlegung), `"doppellauf": true` an jedem Objekt (Stiche rückwärts, gleiche Farbe, ohne Verbinder – Handbuch Kap. 23) |
| 4b.4 | **Flächen** | Step-Muster (Nr. 1 …), mehrere Stichwinkel (Turning Fill), Zickzack-Füllung, Tatami-Kurzstiche | ✅ 20.11. Step-Muster 1–20 (Tabelle DIALOG.DLL, Reihen abwechselnd Versatz 1/2, gemessen) · ✅ mehrere Stichwinkel (`winkellinien`, Step längs der Linien wie im Original gemessen; ohne Löcher) · ✅ Zickzack-Deckstich (`stichart: zickzack`, 0x2724/0x2725) · ✅ Korrektur: Deck-Step der Stoffe Versatz 0,25/0,75 (Referenz `vergleich_stoff` ersetzt) · ✅ Kurzstiche in Step-Flächen mit Winkellinien und Step-Schrift (Effekt im allgemeinen Füllgenerator; 770 solcher Flächen der Musterdesigns mit Kurzstichen) |
| 4b.5 | **Reihenfolge** | nach Farbe, an Anfang/Ende, automatisch – mit Prüfung, ob sich Objekte überdecken (Kap. 14); Ein-/Ausstich je Objekt setzbar | ✅ 20.11. `stick_kern::reihenfolge` + CLI `stick reihenfolge`: wie ausgewählt (MOD1 Resequence: Block an die Stelle des ersten ausgewählten Objekts), an Anfang/Ende, nach Nummer, nach Farbe (Farbfolge wählbar); **Überdeckungsprüfung = eigene Erweiterung** (Original prüft nicht): Warnung, wenn ein Objekt unter eine Fläche gerät, und Sortieren nach Farbe, das Überdeckungen beachtet (`--ueberdeckung`) · ✅ Ein-/Ausstich je Objekt (`einstich`/`ausstich`) und Closest Join (`kuerzeste_verbindung`, MOD1/GEOMETRY FindExitEntry) · gemessen: Original stickt ohne Eingriff wie digitalisiert (Bericht 16) · „automatisch“ im Handbuch Kap. 14 = Blackwork Run → 4b.9 |
| 4b.16 | **Werte je Objekt** | Wie die Objekteigenschaften im Original: Stil-Werte am einzelnen Objekt überschreiben – vor allem Tatami-Abstand, Stichlänge, Mindeststich am Rand, Zugausgleich (Satin/Tatami), Unterlage-Werte, Lauf-Varianten; Grundlage für den Expertenmodus je Objekt. Heute nur Design-weit (`einstellungen`), Ebene im Katalog „design“ → „beide“ (Wunsch Nutzer 20.11.) | ✅ 21.11.: `"einstellungen"` an jedem Objekt, Aufbau wie am Design ohne `stoff` (Stoff → Design → Objekt → Felder direkt am Objekt); gilt für Lauf, Satin, Tatami, Unterlagen **und Verbinder** (Vernähen je Objekt, `wege_verbinden_je_teil`); Beleg: jedes ART-Objekt trägt eine eigene Kopie des Stil-Baums (`Analyse_Teil3/03` §4, hoch); `Design::einstellungen_fuer(nr)`; Katalog: 66 Parameter Ebene „beide“ mit `json_objekt` = `einstellungen.…`; Schema-Typ `EinstellungenJson` |
| 4b.17 | **SVG-Import** (Erweiterung, Wunsch Nutzer 20.11.: in Inkscape zeichnen → laden → sticken) | SVG lesen (freie Bibliothek `usvg`: Gruppen, Verschiebungen, Stile, Formen → Pfade); gefüllte Pfade → Flächen mit Löchern (Füllregel), Konturen → Lauf/Dreifach/Satinlinie nach Strichbreite; Stichart nach Breite wie EasyStitch des Originals (Linie ≤ 0,3 mm Lauf, ≤ 2 mm kräftige Linie; Fläche ≤ 8 mm Satin, ≤ 12 mm Tatami 1, darüber Tatami 2); Zeichenreihenfolge = Stickreihenfolge, Überdeckungen aussparen, Farben → Garnkarte, Maßstab in mm (viewBox/width). Hinweise: Text in Pfade umwandeln, Verläufe/Transparenz → Hauptfarbe; schmale Flächen als Satin brauchen eine berechnete Stichrichtung (s. u.). CLI `stick svg`, später Befehl für die App. **Baustein „Satin-Stichrichtung automatisch“ (gehört dazu, zuerst bauen):** für eine beliebige schmale Form Mittellinie und Winkellinien selbst bestimmen (bei den Alphabeten liefert das Original sie mit), daraus Seiten A/B wie bei den Säulen; Verzweigungen (z. B. „Y“-Form) in mehrere Säulen teilen. Vorbild im Original: Auto Digitizer / EasyStitch „Satin“ für Flächen ≤ 8 mm – Verfahren dort nachsehen (Code, Musterdesigns aus „AutoDigitizing“). Wird auch für 4b.11 (Bild → Umrisse mit Satin) gebraucht | 🟡 Baustein ✅ 20.11.: `stick_kern::saeulen` nach EZSTITCH (Abtastung 1 mm/0,1 × Breite/0,05 mm, Triangulation, Sternschnitt an Abzweigungen, Stichlinien nach Krümmung), EmbroideryPatch-Stichlinien entschlüsselt, an 54 Säulen des Auto Digitizers geprüft (54/54 eine Säule, Richtung Median 4,4°), JSON `satinflaeche`, TrueType `"stichart": "satin"` (Break Angle 87°), CLI `stick saeulen` (Bericht 17) · ✅ 21.11. **SVG lesen**: `stick-vektor::svg` (`usvg`), Stichart nach Breite `stick-vektor::easystitch` mit den Werten der EasyStitch-Stile, `saeulen::dicke`, Überdeckung aussparen, Closest Join, CLI `stick svg`, Befehl `svg_lesen` (Bericht 18) |
| 4b.6 | **Knopflöcher** | einfügen, Typ, Länge, Dichte, mit Objekten verbinden (Kap. 23, Klasse 0x95) | ✅ 21.11.: 9 Typen = Zeichen von `ButtonHoles.ESB` (Teile `<L>`/`<U>`/`<B>`, Umrisse E/I/M), Länge/Raupenabstand/Winkel und Standardwerte (20 mm, 0,5 mm, 90°) aus MOD1, gegen `Samples/Reference/Buttonholes.ART` gemessen (Median 0,04–0,15 mm); `stick_kern::knopfloch`, `esb::knopfloecher_laden`, `"art": "knopfloch"`, CLI `stick knopfloecher`; Einarbeiten über `aussparen` mit dem Mittelumriss (Sicherheit niedrig, Bericht 19 §5). Dabei gefunden und für alle Satinsäulen umgesetzt: **Satin gerade quer + schräg vor** (`Analyse_Teil3/04` §5, Referenz-Stufe 11) |
| 4b.7 | **Motive** | Pattern Run (Motivlinie) ✅ 21.11. (`Analyse_Teil3/20`), Pattern Fill (Motivfüllung) ⬜, eigene Muster/Sets (Kap. 25) ⬜ | 🟡 |
| 4b.8 | **Effekte** | erhöhter Satin (Loft), Textured Edge, Wave Effect, Gradient/Open Fill, Color Blending (Kap. 22–24) | 🟡 Loft ✅ (= Auto Jump mit kleinem Höchststich, Handbuch) · Rest ⬜ |
| 4b.9 | **Craft Stitches** | Blackwork Run/Border/Fill, Backstitch, Stemstitch, Candlewicking, Lacework, Stippling (Kap. 26) | ⬜ |
| 4b.10 | **Applikation Rest** | Versatz innen/außen, AutoAppliqueValues, Partial Appliqué (Kap. 27) | ⬜ |
| 4b.11 | **Automatik** | Bild → Umrisse ausbauen (Linien, Satin – mit dem Baustein „Satin-Stichrichtung automatisch“ aus 4b.17 –, maßstabsfrei = M2), Magic Wand, PhotoSnap (Kap. 11–13) | 🟡 |
| 4b.12 | **Anordnen** | Ausrichten, Gruppieren, Mirror-Merge, Kranz, Kaleidoskop (Kap. 15) | ⬜ |
| 4b.13 | **Dateien/Rahmen** | ART-Designs mit Objekten öffnen (Kern), fremde Stickdateien, eigene Rahmen, welche Rahmen die 185 annimmt (Kap. 6, 28) | ⬜ |
| 4b.14 | **Vorlagen** | Stile/Vorlagen speichern wie NORMAL.AMT (Kap. 9) – zusammen mit dem Parameterkatalog | ⬜ |
| 4b.15 | **Kreuzstich** | eigenes Modul (Kap. 34–37) | ⬜ |
Nicht nötig: Designkarte/Magic Box, Scanner, Quilter (eigenes Programm), nur in Strings vorhandene Sticharten (Coil,
Contour, Trapunto …). Reine Oberflächenaufgaben (Zoom, Auswahl, Stichvorschau, Druck, Portfolio) → neue Oberfläche.

## Phase 5 – Maschine im Programm und Komfort
**20.11.2026:** Die Oberfläche wird komplett neu gestaltet (Mockup von Claude Design, Zugriff auf das Repository);
Anforderungen in `app/ANFORDERUNGEN.md`. **21.11.2026: Entwurf v3 ist fertig und verbindlich, die Sperre für die App
ist aufgehoben** (Nutzer); Umsetzung in Etappen, Übergabe `UI_Mockups/v3_2026-11-21/UEBERGABE.md`.

**Neue Oberfläche v3 – Etappen (vom Nutzer freigegeben 21.11.2026; gebaut auf dem Mac, Rust dort eingerichtet):**
| Etappe | Inhalt | Stand |
|---|---|---|
| E0 | Kern-Wünsche (UEBERGABE §4): Hinweise mit Objektnummer (W2), Begrenzung (W6), Datengröße gegen 145 426 Byte (W8), Garn von Hand `garn` (W3), Farbblöcke mit Applikationslagen (W11), Alphabete mit empfohlener Höhe + `alphabet_vorschau` (W1), `step_muster_liste` (W7); W4 war schon erfüllt | ✅ 21.11. (0d56a68) |
| E1 | Grundgerüst: alte Oberfläche archivieren, Tauri-Befehl `kern`, `kern.ts` per Alias, Tokens/Barlow offline, Fensteraufbau, Design-Zustand mit Rückgängig, Berechnung entprellt, Stichvorschau mit Auswahl | ✅ 21.11. (`app/README.md`; Bildschirmfotos `app/bildschirmfotos/e1_*`; Werkzeug `app/werkzeuge/bildschirmfoto.py`, weil das Terminal auf dem Mac keine Bildschirmaufnahme darf) |
| E2 | Einfacher Modus: Assistent 4 Schritte (Rahmen & Stoff · Gestalten · Prüfen & Garn · Speichern/Senden), Dialoge Alphabet, Garn | ✅ 21.11. (dazu Bild umwandeln, SVG einlesen; Bilder `app/bildschirmfotos/e2_*`) |
| E3 | Maschine: Befehle nach `stick-api` (W10: Abbrechen, Wiederholungen, Fehlercodes; Garn von Hand in der Garnfarbtabelle), Senden in 4 Stufen, Sticken-Dialog, Simulation, Fehlerzustände, Anleitung, „Aus der Maschine“ – nur Simulation | ✅ 21.11. (Simulation; am Gerät offen → „Vor Ort“ V10) |
| E4 | Experte I: Objektliste (Color Film), Anordnen mit Überdeckungswarnung, Werkzeugleiste/Einfügen (alle Objektarten inkl. Knopfloch, SVG), Inspektor Objekt/Design | ✅ 21.11. (Inspektor aus dem Schema des Kerns) |
| E5 | Experte II: Griffe (verschieben, skalieren, drehen), Ein-/Ausstich, Einzelwerte, JSON, Abspielen, Füllmuster-/Step-Dialog, einzelne Buchstaben (W9) | ✅ 21.11.; **W9 ✅ 21.11. (nachgeholt):** Kern liefert je Stich die Buchstabennummer (`Ergebnis.buchstabe`, aus `text_setzen_nr` durch Transformation und Aussparen gereicht); Oberfläche: Text wählen, Buchstaben anklicken → Inspektor „Einzelne Buchstaben“ (Farbe, Größe, Drehen, Verschieben, Zurücksetzen) |
| E6 | Formansicht mit Knoten (W5), Dunkel/Mac, ANFORDERUNGEN.md, Parameterkatalog nach Modus vorlegen (M1), „Abgeglichen bis“ | ✅ 21.11. (Knoten für Punktlisten; W5-Rest ✅ 21.11.: Befehl `in_knoten_umwandeln` – Kreis/Oval/Bogen → Bézier-Knoten mit Anfassern wie im Original, Transformation auf die Knoten angewandt, Knoten-Werkzeug zieht Knoten und Anfasser) |

**Erster Test des Nutzers auf dem Mac (21.11.2026) – umgesetzt:** weißes Fenster behoben (WebKit + Hash-Routing,
`app/src/app.html`); Entf/⌫ löscht auch im einfachen Modus; Griffe springen nach dem Loslassen nicht mehr zurück;
grünes Höhenband entfernt; **Elemente** in Schritt 2 (Kacheln → Element bearbeiten, „Elemente (n)“ mit Liste,
„+ Hinzufügen“ öffnet die Kacheln; Bild/SVG/Design werden als **Gruppe** eingefügt statt das Design zu ersetzen –
Kern: `gruppe`/`gruppen` wie „Group“ des Originals, Befehl `design_einfuegen` wie „Insert Design“); Gruppieren/Auflösen
im Expertenmodus (Strg+G); **Senden auch am Mac** (USB-Seriell-Adapter, Ports `/dev/cu.*`). Bild „nur schwarz-weiß“:
war das Bild des Nutzers. **Bild umwandeln verbessert** (21.11., Phase 4 Punkt 8): Dialog mit Vorschau und Farbliste.
**Schrift „automatisch“** (21.11., Wunsch Nutzer): `stichart: "auto"` – Satin bis 8 mm Stichlänge, darüber Step für den
ganzen Text (eigene Erweiterung); einfacher Modus mit Schalter „Trotzdem Satin“. **Am Gerät (V10, 21.11.):** die
neue Oberfläche hat an der artista 185 gesendet und gestickt (Bericht Nutzer). Den Hinweis am Ende des Stickens zeigt
die Maschine selbst (Firmware, nicht änderbar) – Wunsch „100 Zufallssprüche“ daher im Ergebnis des Sende-Dialogs
(`app/src/lib/sprueche.ts`).
**Schriften (21.11., Wunsch Nutzer „mehr und modernere Schriften, suchen, Favoriten, nach Stil“):** Dialog **„Schrift
wählen“** für beide Modi – Stick-Alphabete · Computer-Schriften (installiert, nur mit lateinischen Buchstaben; Namen auch
aus MacRoman) · **Google Fonts** (1 833, Katalog im Programm, Herunterladen per Klick in den Datenordner, danach ohne
Internet) · ★ Favoriten; Suche, 19 Stil-Filter (Gotisch, Schreibschrift, Modern …), Vorschau mit eigenem Text, gefüllte
Buchstaben; **„stickbar ab … mm“** je Computer-Schrift (dünnster Strich ≥ 0,8 mm, geeicht an den 47 Original-Alphabeten
mit empfohlener Höhe – eigene Erweiterung); Wechsel Alphabet ↔ Computer-Schrift (`schrift_wechseln`). Bericht
`Analyse_Teil3/21_schriften.md`. **Stil-Einordnung ✅ (Punkt 4):** 23 Stile in allen Reitern mit Anzahl; 50
Original-Alphabete und 173 Standardschriften (Windows, Office, macOS) nach dem Aussehen eingeordnet, sonst die
Einordnung von Google; eigene Stile des Anwenders (Etikett an der Kachel).

**Werkzeuge nach dem Original (21.11.2026, Wunsch Nutzer):** Formen-Werkzeuge + Stichart getrennt (Füllung/Umriss,
Handbuch S. 58/63/64), Stichart jederzeit wechseln (Kern `stichart_aendern`), „Umrandung dazu“ (eigene Erweiterung),
schnelle Tooltips, neue Symbole ✅. **Nächster Oberflächen-Schritt:** Formen mit der Maus zeichnen (Punkt für Punkt,
links = Ecke, rechts = Kurve wie im Original, S. 58) statt Probeform in der Mitte.

**Offen nach E6 (Kern-Schritte):** ~~W9~~ ✅ 21.11. · ~~W5-Rest~~ ✅ 21.11. (`stick-kern::umwandeln`) · W12 Rahmen von der Maschine lesen (Analyse, Gerätetest nur nach Zustimmung) · Neue Oberfläche am Gerät (Vor Ort V10).
Später: W12 (eingesetzten Rahmen von der Maschine lesen) – erst Analyse, Gerätetest nur nach Zustimmung.
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
