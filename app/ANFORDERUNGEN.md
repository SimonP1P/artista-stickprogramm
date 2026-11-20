# Anforderungen an das Programm – Grundlage für die neue Oberfläche (UX/UI)

**Stand: 20.11.2026.** Die Oberfläche wird **komplett neu gestaltet** (Mockup durch Claude Design). Dieses Dokument
beschreibt, **was** das Programm können muss – nicht, wie es aussieht. Die heutige Oberfläche in `app/src/` ist ein
funktionaler Prototyp; Aufbau, Anordnung und Optik sind für das neue Design **nicht** bindend. Die Rechenlogik
(Rust-Kern `kern/`, Tauri-Befehle `app/src-tauri/src/lib.rs`) bleibt und wird bei Bedarf erweitert.

Kennzeichnung: **M** = muss (gibt es schon im Kern/Programm) · **S** = soll (bald) · **P** = geplant (Roadmap
Phase 4/5, das Design soll Platz dafür vorsehen).

---

## 1. Worum es geht
- **Eigenes Stickprogramm** für **eine** Maschine: **BERNINA artista 185** mit Stickmodul, angeschlossen über das
  serielle Original-Kabel an einem Windows-PC (COM3). Vorbild ist die Original-Software „BERNINA artista V4“
  (Wilcom, 2003) – Stichberechnung und Werte stammen aus deren Analyse.
- **Ablauf für den Anwender:** **Rahmen wählen → Design anpassen → an die Maschine senden.**
- **Nur für den Nutzer selbst** (privat, kein Verkauf), **Deutsch**, **offline**, **Windows und Mac** (Mac ohne
  Maschine: entwerfen, speichern, exportieren). Technik: Tauri 2 + Svelte/TypeScript + Rust.
- Anwendungsfälle: Schrift (Namen, Monogramme), Motive aus Formen, Flächen, Linien; später Computer-Schriften,
  Applikationen, Bilder. Feine Arbeiten sind ein Fall unter vielen.

## 2. Hauptabläufe (User Journeys)

### A. Schrift sticken (häufigster Fall) – M
1. Rahmen wählen (z. B. mittlerer Rahmen 130 × 100).
2. Stoff wählen (beeinflusst Dichte und Zugausgleich).
3. Text eingeben, Alphabet wählen (50 Original-Alphabete), Höhe/Breite/Ausrichtung einstellen – **Vorschau
   aktualisiert sich beim Tippen**.
4. Hinweise sehen: empfohlene Höhe des Alphabets, fehlende Zeichen, passt in den Rahmen?
5. Garnfarbe wählen (aus Garnkarte, z. B. Isacord 40).
6. An die Maschine senden (siehe D).

### B. Design aus Formen – M (heute nur über JSON), S (mit der Maus)
Objekte anlegen und bearbeiten: Laufstich, Dreifachstich, Satinsäule, Satinbahn, Tatami-Fläche (mit Löchern), Text.
Formen: Punktlisten, Kreis, Rechteck, Bogen, Bézier, verbundene Formen. Heute nur im JSON-Editor – **das neue
Design braucht eine grafische Bearbeitung** (Auswählen, Verschieben, Drehen, Skalieren, Knoten bearbeiten,
Reihenfolge, Farbe je Objekt). Ein JSON-/Experten-Bereich darf bleiben (S), muss aber nicht im Vordergrund stehen.

### C. Dateien – M
Design öffnen/speichern (eigenes JSON-Format), **EXP exportieren** (Stickdatei), Beispieldesign laden, neues Design.

### D. An die Maschine senden – M
1. Anschluss wählen (COM-Port oder **Simulation** zum Ausprobieren ohne Maschine).
2. Optional „Maschine prüfen“ (nur lesen): Firmware, aktuelle Garnfarben in der Maschine.
3. **Senden** in zwei Stufen:
   - **Planen (nur lesen):** Programm verbindet sich, prüft Design und Maschine und zeigt **alle geplanten
     Schreibzugriffe** (Design-Daten, Vorschaubild/Icon, Name/Infos, Garnfarbtabelle) und Hinweise.
   - **Bestätigen → Schreiben:** mit Fortschrittsanzeige (je Teil: geschrieben X von Y Byte, zurücklesen), danach
     Ergebnisliste („geschrieben und zurückgelesen – stimmt“).
4. Optional **danach sticken** (Befehl „sticken ohne speichern“): nur mit ausdrücklicher Bestätigung **„Es ist jemand
   an der Maschine – Rahmen eingesetzt, Nadel oben, Hände weg“** (der Rahmen fährt sofort).
5. Abschluss: PC-Modus wird immer verlassen; Ergebnis anzeigen; Mitschnitt (Log) gespeichert.
6. Hinweise für die Person an der Maschine (siehe §4.3).

### E. Designs aus der Maschine – M
Liste der gespeicherten Designs (51 Stück, mit Nummer, Name, Merkmalen) → eines lesen → **Ansicht** mit den
Garnfarben, die die Maschine liefert → als EXP sichern. Die Ansicht ist nur zum Ansehen (kein Bearbeiten).

### F. Geplante Funktionen (Roadmap Phase 4, in dieser Reihenfolge) – P
1. **Computer-Schriften (TrueType/OpenType)** wie Alphabete benutzen (Schrift auswählen, Text, Höhe; Stichart
   Satin/Tatami je nach Strichbreite).
2. **Applikation** (Stoffteil aufnähen): Platzierungslinie → Stopp (Stoff auflegen) → Heftstich → Stopp
   (Zuschneiden) → Abdeckstich (Satin). Braucht im Ablauf **Anleitungsschritte an den Farbstopps**.
3. **Muster-/Motivfüllungen** (Fancy Fill, Motivlinien): Muster auswählen, Größe, Abstand.
4. **Bild → Umrisse**: Bild laden (PNG/JPG/BMP), Farben reduzieren, automatisch in Objekte umwandeln, nachbearbeiten.
5. **Aussparung (Knockout)**: Überlappungen nicht doppelt sticken.
6. **Dauerhaft in der Maschine speichern** (statt nur sticken) – eigene, ausdrücklich bestätigte Aktion.
7. **Garn je Farbe von Hand wählen** (statt automatischer Zuordnung).

## 3. Funktionen im Detail

### 3.1 Rahmen – M
14 Einträge (Stickfeld Breite × Höhe in mm, Ursprung = Rahmenmitte, +y nach oben):

| Rahmen | Stickfeld | Hinweis |
|---|---|---|
| artista 255 x 145 Oval | 145 × 255 | großer Ovalrahmen (oval) |
| artista 200 x 155 | 155 × 200 | großer Rahmen |
| artista 130 x 100 | 100 × 130 | mittlerer Rahmen (Standard) |
| artista 50 x 72 Free Arm / Normal | 72 × 50 | eine Seite gerundet |
| artista 40 x 60 | 60 × 40 | kleiner Rahmen |
| artista 400 x 150 MEGA | 150 × 400 | Mega-Rahmen, mehrere Positionen (P: Aufteilen) |
| je „Rotated“ | um 90° gedreht | |

Darstellung: Rahmenumriss/Stickfeld in der Vorschau; Design außerhalb → deutliche Warnung (Senden gesperrt).

### 3.2 Stoff – M
8 Stoffe + „0 Software Defaults“ (Werte aus dem Original): Knit/Woven/Terry je Light/Medium/Heavy Weight. Ändern
Stichabstand (Satin/Tatami), Auto-Spacing-Faktor und Zugausgleich. **S:** kurze Erklärung/Empfehlung je Stoff
(z. B. „dehnbarer Stoff → Knit, Vlies verwenden“) – Erfahrung vom ersten Sticktest: ohne Vlies/Stickgarn
zieht sich Satin zusammen.

### 3.3 Farben und Garne – M
- Jede Farbe eines Designs ist ein **Farbstopp** an der Maschine (Reihenfolge = Stickreihenfolge).
- **Garnkarte** wählen (32 Karten der CD: Isacord 40, Madeira, Mettler, Sulky, Robison-Anton, DMC, Default …);
  jede Farbe bekommt das **nächste Garn** (Marke, Nummer, Name, Garnfarbe). Diese Angaben gehen an die Maschine
  (Display). Standard-Karte „Default“ (20 Grundfarben, wie die Maschine selbst).
- Anzeige je Farbe: Bildschirmfarbe, zugeordnetes Garn (Farbfeld + „Isacord 40 1311 rich brown“).
- **S:** Garn je Farbe von Hand ändern (Suche in der Karte nach Nummer/Name, Farbfelder); Farben umsortieren.
- Grenze der Maschine: **höchstens 60 Farben** je Design.

### 3.4 Objekte und Sticharten – M (Kern), S (grafisch bearbeiten)
| Art | Parameter | Hinweis |
|---|---|---|
| Laufstich | Punkte/Form, Stichlänge (Standard 2,5 mm), offen/geschlossen | |
| Dreifachstich | wie Lauf | kräftige Linie |
| Satinsäule | Seite A, Seite B, Unterlage, fester Abstand oder Auto Spacing, Faktor, Kurzstiche | Breite bis ~12 mm sinnvoll |
| Satinbahn | Mittellinie + Breite, Unterlage | einfache Linie mit Breite |
| Tatami-Fläche | Umriss, Löcher, Winkel (Standard 45°), Unterlage an/aus | große Flächen |
| Text | Text, Alphabet, Höhe (1–200 mm), Breite %, Buchstabenabstand, Position, Ausrichtung links/Mitte/rechts, Unterlage | siehe 3.5 |

Unterlagen: Auto, keine, Mittellauf, Edge Walk, Zickzack, Doppel-Zickzack (kombinierbar). Alle Objekte: Name, Farbe.
Formen: Punktliste, Kreis, Rechteck, Bogen, Bézier, verbundene Formen. Design-weite Einstellungen können einzelne
Werte überschreiben (Expertenbereich).

**S – Bearbeiten mit der Maus:** Objekt auswählen (auch in einer Objektliste), verschieben, drehen, skalieren,
spiegeln, duplizieren, löschen; Knoten bearbeiten; Reihenfolge ändern (= Stickreihenfolge); Rückgängig/Wiederholen;
Raster/Fangen; Maße numerisch eingeben.

### 3.5 Schrift – M
- **50 Original-Alphabete** (Alice, Anniversary, Bamboo, Blackboard, Border CAPS, Castle, Chicago, Drifter, Swiss
  Block, Zurich …). Je Alphabet **empfohlene Höhe** (z. B. Drifter 6–40 mm) – Warnung außerhalb.
- Eingaben: Text, Alphabet, Höhe, Breite %, Abstand, Position, Ausrichtung, Unterlage („Auto“ = Zickzack wie
  Original; „keine“ für kleine Schrift).
- Mehrere Textobjekte je Design; **Zeichen, die das Alphabet nicht hat**, werden gemeldet.
- **S:** Alphabet-Vorschau (Musterzeile je Alphabet) in der Auswahl; mehrzeiliger Text; Text auf Bogen (P).
- **P → im Kern fertig (20.11.2026):** Computer-Schriften (TTF/OTF, alle installierten Schriften) in derselben
  Auswahl, klar getrennt von den Original-Alphabeten. Eingaben: Schrift, Text, Höhe (Großbuchstaben), Breite %,
  Abstand, Position, Ausrichtung, **Stichart** (Fläche/Tatami mit Winkel, Umriss Lauf, Umriss Dreifach). Das Programm
  braucht dafür noch einen Befehl „Schriften auflisten“ und das Laden in `design_berechnen` (Kern: `stick_formate::ttf`).

### 3.6 Vorschau – M
- Stiche in Garnfarben, Rahmen/Stickfeld, Ursprung.
- Schalter: **Sprünge** zeigen, **Einstiche** zeigen; Zoom (Mausrad), Verschieben (Ziehen), Einpassen.
- Neuberechnung beim Bearbeiten (heute 250 ms nach der letzten Änderung; Berechnung dauert typ. < 100 ms).
- **S:** realistische Garnansicht (Fadenstärke, Glanz) umschaltbar; Stick-Simulation (Abspielen der Stichfolge mit
  Farbstopps); Anzeige der Farbstopp-Reihenfolge; Maßband/Lineal.

### 3.7 Kennzahlen und Prüfungen – M
Stiche, Sprünge, Farbwechsel, Größe (B × H mm), Stichlänge min–max, **Einstiche außerhalb des Stickfelds**, fehlende
Zeichen. **S:** geschätzte Stickzeit (Original: ≈ 0,4 s je Stich), Garnverbrauch, Datengröße gegen die Grenze der
Maschine (**≈ 145 KB** je Design, darüber Fehler „Design is too big“).

### 3.8 Dateien – M
| Aktion | Format |
|---|---|
| Neu / Beispiel | – |
| Öffnen / Speichern | Design-JSON (eigenes Format, `stickprogramm/README.md`) |
| Exportieren | EXP (Bernina-Stickdatei) |
| Aus der Maschine gelesenes Design sichern | EXP |
| P | Import EXP als Vorlage; Bild laden (Bild → Umrisse) |

### 3.9 Maschine – M
| Funktion | Dauer (gemessen) | Schreibt? |
|---|---|---|
| Anschluss wählen (Liste der COM-Ports + Simulation) | – | nein |
| Maschine prüfen (Firmware 04.04, Garnfarbtabelle) | ≈ 4 s | nein |
| Designliste lesen (51 Designs) | ≈ 13 s | nein |
| Design lesen (Ansicht) | ≈ 1,5 KB/s (10 s bis ~1,5 min) | nein |
| Senden planen | ≈ 4 s | nein |
| Senden (Design, Icon, Infos, Garnfarben, alles zurückgelesen) | Testquadrat ≈ 25 s, 6,8 KB ≈ 36 s | **ja** |
| danach sticken (Befehl 0x08) | ≈ 1 s | **ja, Rahmen fährt** |
| P: dauerhaft speichern, löschen | – | **ja** |

Während eines Vorgangs: Fortschritt (Teil, Byte, Zeit), andere Maschinen-Aktionen gesperrt, Bearbeiten erlaubt.
Nach jedem Vorgang: Ergebnis + „PC-Modus verlassen: bestätigt / nicht bestätigt (Maschine aus/ein)“, Pfad des
Mitschnitts (für Fehleranalyse).

## 4. Regeln, die das Design sichtbar machen muss
### 4.1 Sicherheit – M
- **Nichts wird in die Maschine geschrieben ohne ausdrückliche Bestätigung**; vor dem Schreiben eine Übersicht aller
  Schreibzugriffe (Plan).
- **Sticken** nur mit eigener Bestätigung „Person an der Maschine“ – deutlich als „Rahmen bewegt sich“ markiert.
- Senden gesperrt, wenn das Design nicht in den Rahmen passt, fehlerhaft ist oder gerade eine Maschine-Aktion läuft.
- Es wird **nichts dauerhaft** gespeichert (bis P6); das soll der Nutzer wissen.
- Simulation klar als solche erkennbar (kein Verwechseln mit der echten Maschine).

### 4.2 Fehler und Zustände – M
Keine Verbindung (Kabel/Port/Maschine aus) · Stickmodul nicht erreicht · Maschine antwortet nicht mehr (Hinweis
„Maschine aus- und einschalten“) · Übertragungsfehler mit automatischer Wiederholung (anzeigen, nicht verstecken) ·
zurückgelesen weicht ab (Senden abgebrochen, nichts gestickt) · Garnfarbtabelle passt nicht (Hinweis, Senden ohne
Farben) · Design zu groß · Design außerhalb des Rahmens · Alphabet/Garnkarte fehlt.

### 4.3 Hinweise für die Person an der Maschine – S
Kurze Anleitung im Programm, z. B. als Schritt vor/nach dem Senden:
- Nach einem **Rahmenwechsel**: Nadel und Nähfuß hoch → Display „Layout 1“ → **«check»** (Rahmen fährt und erkennt
  den Rahmen) → «Mustermitte anzeigen» → «ok» → Nähfuß senken → Start.
- Material: fester Stoff oder passendes Vlies (Abreißvlies, bei dehnbarem Stoff Schneide-/Bügelvlies), Stickgarn
  40 + Stick-Unterfaden.
- Die artista 185 schneidet Verbindungsfäden **nicht** selbst ab → nach dem Sticken von Hand schneiden.
- Farbstopps: Garn wechseln, wenn die Maschine anhält (Farbliste mit Garnnamen anzeigen).

## 5. Was der Rechenkern heute liefert (Tauri-Befehle)
Alle in `app/src-tauri/src/lib.rs`, Typen in `app/src/lib/typen.ts`. Erweiterbar, wenn das Design mehr braucht.

| Befehl | Eingabe | Ausgabe |
|---|---|---|
| `rahmen_liste` | – | Rahmen: Name, Breite, Höhe, Umriss (Polygon), Hinweis |
| `stoffe_liste` | – | Namen der Stoffe |
| `alphabete_liste` | – | Alphabete: Name, Datei |
| `garnkarten_liste` | – | Namen der Garnkarten |
| `beispiel_design` | – | Design-JSON |
| `design_berechnen` | Design-JSON | Ergebnis: Name, Rahmen, Stoff, **Stiche** (x, y, Art Stich/Sprung/Farbwechsel), Farben, **Garne je Farbe**, Statistik, Zahl außerhalb, fehlende Zeichen |
| `datei_lesen` / `text_speichern` / `exp_speichern` | Pfad (+ Text/JSON) | Text bzw. Byte-Zahl |
| `maschine_ports` | – | COM-Ports |
| `maschine_pruefen` | Port, Simulation | Sitzung (Firmware, Baud, PC-Modus verlassen, Mitschnitt) + Garnfarbbereich der Maschine |
| `maschine_planen` | Design, Port, Simulation, Wahl (Icon, Farben, Sticken) | geplante Schreibzugriffe, Hinweise, Garnfarbbereich |
| `maschine_senden` | wie planen | Ergebnisliste, gestickt ja/nein; **Fortschritt als Ereignis** `maschine_fortschritt` (was, erledigt, gesamt) |
| `maschine_liste` | Port, Simulation | Designs der Maschine: Nr., Name, Flags |
| `maschine_lesen` | Port, Simulation, Nr. | Ergebnis wie `design_berechnen` + Garne der Maschine |
| `gelesen_exp_speichern` | Pfad | Byte-Zahl |

Weitere vorhandene Daten für das Design: Icons der Maschine (72 × 62 Punkte, aus den Stichen gezeichnet),
Garnfarben aller Karten (RGB), Rahmenformen, empfohlene Alphabet-Höhen (`Analyse/daten/alphabets.md`).

## 6. Nicht-funktionale Anforderungen
- **Sprache Deutsch**, Maße in **mm** mit Komma (deutsches Zahlenformat).
- **Windows 10/11** (mit Maschine) und **macOS** (ohne Maschine); Fenster ab etwa 1280 × 800 nutzbar.
- Reaktionsschnell: Vorschau beim Tippen; lange Maschinenvorgänge im Hintergrund mit Fortschritt.
- Offline, keine Konten, keine Telemetrie.
- Hell/Dunkel: S. Barrierearm: Tastatur bedienbar, ausreichende Kontraste, Farben nicht als einziges Merkmal
  (Garnfarben immer mit Namen/Nummer).
- Die Stickvorschau ist das Herzstück und braucht die größte Fläche.

## 7. Offene Fragen für das Design
1. Ein Fenster mit Bereichen oder geführter Ablauf (Schritte 1–4) – oder beides (Assistent für Einsteiger, freie
   Ansicht für Fortgeschrittene)?
2. Wo leben Objektliste, Eigenschaften und Garnliste (Seitenleisten, Paletten, Inspektor)?
3. Wie wird das Senden mit Plan, Bestätigung, Fortschritt und „Person an der Maschine“ dargestellt (Dialog,
   eigene Seite, Seitenleiste)?
4. Wie werden die geplanten Funktionen (TTF, Applikation mit Anleitungsschritten, Bild → Umrisse) eingebunden, ohne
   dass die Oberfläche überladen wirkt?
5. Was bleibt vom JSON-Editor (Expertenmodus)?

## 8. Quellen
`ROADMAP.md` (Phasen, Liste „Vor Ort“), `ARCHITECTURE.md`, `app/README.md`, `stickprogramm/README.md` (Design-Format),
`kern/README.md`, `Analyse_Teil2/07_befund_am_geraet.md` (Gerätetests), `Analyse/ANALYSE.md` (Werte des Originals),
`Analyse/daten/alphabets.md`, `Analyse/daten/garnkarten/`.
