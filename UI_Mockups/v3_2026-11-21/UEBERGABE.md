# Übergabe: Oberfläche Stickprogramm v3

**Stand 21.11.2026** · Grundlage: `SCHNITTSTELLE.md` (maßgeblich), `UI_Mockups/FUNKTIONEN.md`, `PARAMETER.md`, `app/ANFORDERUNGEN.md`, `UI_Mockups/README.md` (Branch `main`).
Zielplattform: Tauri 2 + Svelte 5, Rust-Kern über **einen** Tauri-Befehl `kern(name, eingabe)` (`kern/schnittstelle/kern.ts`).

## 0. Dateien im Entwurf

| Datei | Inhalt |
|---|---|
| `Stickprogramm v3 Kern.dc.html` | Der klickbare Entwurf (1280 × 800). Zustand über `?zustand=…` oder den Tweak „Zustand“. Einfach/Experte im Kopf umschaltbar. |
| `Stickprogramm v3 Zustaende.dc.html` | Übersicht: alle 34 Zustände nebeneinander (Gruppen A–D), je mit „Groß öffnen“. |
| `UEBERGABE.md` | Diese Datei. |
| `Stickprogramm v3.dc.html` | Vorentwurf v3 (vor diesem Abgleich) – nur noch zum Vergleich. |

**Wichtig:** Die Stichbilder im Entwurf rechnet ein kleines Darstellungsskript (Ersatzschrift für die Alphabete, vereinfachte Tatami/Satin). Im Programm kommt jeder Stich aus `design_berechnen`; die Oberfläche rechnet keine Geometrie selbst.

Alle Daten im Entwurf stammen aus dem Repo: 14 Rahmen, 9 Stoffe mit Werten, 50 Alphabete mit empfohlener Höhe (`Analyse/daten/alphabets.md`), echte Garne aus `Isacord 40 wt-numerical.tch` und `Default.tch`, 12 echte Füllmuster-Kacheln aus `fuellmuster.json`, Maschinen-Designs aus `Arbeitsdaten/maschinendesigns/UEBERSICHT.md`, Parameter aus `PARAMETER.md`.

---

## 1. Abgleich mit dem bisherigen Entwurf (v1, v2 und Vorentwurf v3) — Liste c)

### 1a. Entfernt – zeigte etwas, das der Kern nicht kann

| Bisher | Warum entfernt | In v3 stattdessen |
|---|---|---|
| Kunde, Stückzahl, „Projekte“-Liste mit Status (Entwurf/Gesendet/Gestickt) | kein Feld im Design-Format, kein Befehl | Datei › Öffnen/Speichern (`datei_lesen`, `text_speichern`) |
| Motiv-Bibliothek (Wappen, Blüte, Krone, Pfote …) | Motive = FUNKTIONEN §7 | Kachel „Motiv aus Sammlung – kommt noch“ |
| Eigene Schriftnamen (Block, Schreibschrift, Antiqua, Fraktur …) über Systemschriften | Kern kennt 50 Original-Alphabete + Computer-Schriften | `text.alphabet` (Liste `alphabete_liste`), `ttf.schrift` (`schriften_liste`) |
| Garnsystem „140 Farben (Platzhalter)“ mit erfundenen Nummern | Kern: 32 Garnkarten | `garnkarte` + `garnkarte_garne`, Anzeige immer Marke · Nummer · Name |
| Deutsche Stoffnamen als Werte, Vlies je Stoff als Kernwert | `stoff` hat 9 feste englische Werte; Vlies stammt nicht aus dem Original | Originalname + deutsche Erklärung; Vlies als „eigener Hinweis“ gekennzeichnet |
| Schalter „Gleiche Farben zusammen“ (merge) | Zusammenlegen ist kein Feld, sondern ein Umordnen | Anordnen › Nach Farbe sortieren (`reihenfolge_aendern` `nach_farbe`, mit Überdeckungsprüfung) |
| Stichart „Automatisch“ je Teilfläche, Farbe je Teilfläche („Fläche anklicken“) | Kern: eine Stichart und eine Farbe je Objekt | Objekt wählen → Art und `farbe` des Objekts |
| Auto-Split, Unterlage „Step“, „Versatzmuster Nr. 1–3“, „Muster Nr.“ mit Abstand, Schalter „Vernähen“, „Mindeststich“ | nicht so im Kern | `auto_jump.max/sprung`, `unterlage` (auto/keine/mittellauf/edge_walk/zickzack/doppelzickzack), `step_muster` 1–20, `muster` + `groesse` + `winkel`, `einstellungen.verbinder.*`; kürzester Stich ist automatisch |
| Bild-Import „Kanten glätten“, „Stichart je Farbe“ | `bild_umwandeln` liefert Tatami-Flächen | Breite, Farben (E); Kleinste Fläche, Glättung, Hintergrund (X); Stichart nach Breite = „kommt noch“ (SVG-Import §7) |
| Stickablauf-Checkliste „Haken setzen, wenn Farbe fertig“ | Maschine meldet keinen Fortschritt beim Sticken | Farbfolge mit Garnnamen im Ergebnis des Sendens |
| Garnglanz-Schalter | kein Kernwert (rein Darstellung, ANFORDERUNGEN 3.6 „S“) | weggelassen; später als Ansichtsoption möglich |

### 1b. Fehlte – jetzt im Entwurf

- **Objektliste wie die Color Film** (Nr, Farbe, Art, Name, Garn), Ziehen zum Umordnen, Menü **Anordnen** (nach Farbe, an Anfang/Ende, vor/nach, wie ausgewählt, vor/nach Nr.), **Warnung bei Überdeckung**, Anzeige „über Nr. …“ aus `ueberdeckungen`.
- **Auswahl über die Stiche** (`Ergebnis.objekt` je Stich), gewähltes Objekt hervorgehoben, andere gedimmt.
- **Formansicht mit Griffen** (`design_formen`), Werkzeuge Knoten, Stichwinkel (`winkellinien`), **Einstich/Ausstich**.
- Alle Objektarten: `lauf`, `dreifach`, `satin`, `satinlinie`, `blanket`, `tatami` (+ `winkellinien`, `kurzstiche`), `satinflaeche` (+ `knickwinkel`), `muster`, `applikation`, `text`, `ttf`, `monogramm`; Formen `kreis`, `rechteck`, `oval`, `bogen`, `bezier`, `verbinden`.
- Schrift: Grundlinie (gerade, Kreis/Bogen, senkrecht, Kurve, Wappen), Kursiv, Verzerren (`elastisch`), Farbwechsel `^` + `farben`, einzelne Buchstaben, Stichart, Unterlage, Abstände.
- **Aussparen** (`aussparen`, `ueberlappung`), **Verstärken** (`doppellauf`), **Auto Jump**, **Start/Ende**, **Kürzeste Verbindung**.
- **Einzelwerte / Werte je Objekt** aus dem Parameterkatalog mit Herkunft (Standard / Stoff / Design / geändert) und Zurücksetzen.
- **Garnkarte wählen** und **Garn je Farbe von Hand**.
- **Fehler mit Objektnummer** (Banner, Liste, Vorschau, Inspektor) und **Hinweise** unaufdringlich in der Statusleiste.
- Auswahllisten mit Vorschau: Alphabete (mit empfohlener Höhe), Füllmuster (echte Kacheln), Step-Muster 1–20.
- **JSON-Ansicht** (Expertenmodus) mit `design_pruefen`.
- Senden in zwei Stufen mit **Plan-Übersicht**, eigener **Sticken-Dialog**, **Fehlerzustände**, **Anleitung nach Rahmenwechsel**, Simulation unverwechselbar, „Designs aus der Maschine“, **Mac ohne Maschine**.
- „Stickfolge abspielen“ klar getrennt von der Maschinen-**Simulation**.
- Platz für alles aus FUNKTIONEN §7, als „kommt noch“ gekennzeichnet.

### 1c. Veraltet – korrigiert

| Bisher | Jetzt |
|---|---|
| Befehlsliste aus ANFORDERUNGEN §5 (einzelne Tauri-Befehle) | ein Befehl `kern(name, eingabe)` laut SCHNITTSTELLE §5; nur die Maschinen-Befehle stehen noch in `app/src-tauri/src/lib.rs` |
| Senden + Sticken in einem Dialog mit einem Häkchen | Anschluss → Plan (nur lesen) → Bestätigen → Übertragen → Ergebnis; Sticken nur über eigenen Dialog mit drei Bestätigungen |
| 6 Rahmen, Freiarm als Oval, „gedreht“ als eigener Schalter ohne Feldwert | 7 Rahmen × normal/„Rotated“ = 14 Werte von `rahmen`; Free Arm/Normal mit einer gerundeten Seite |
| „Werte je Objekt“ in der Aufgabe als geplant genannt | ist im Kern (`einstellungen` am Objekt, 4b.16) → voll ausgestaltet, nicht „kommt noch“ |
| Stoff ändert „Dichte und Zugausgleich“ | auch Unterlage-Abstände und „Trim ab“ (PARAMETER.md „Werte je Stoff“) – sichtbar als Herkunft „Stoff“ |
| Garnfarben als reine Farbfelder | immer Marke · Nummer · Name; Farbe nie das einzige Merkmal |

---

## 2. Bildschirme, Bereiche, Komponenten — a)

**Ein Fenster** (ab 1280 × 800), Kopf 48 px, Statusleiste 30 px.

| Bereich | Einfach | Experte | Komponenten |
|---|---|---|---|
| Kopf | Datei, Rückgängig/Wiederholen, Design-Name, Modus, Maschinen-Knopf, Senden | + Einfügen, Anordnen | `AppHeader`, `MenuButton`, `Menu`, `ModusSchalter`, `MaschinenChip`, `SendenKnopf` |
| Links | **Assistent** 360 px: Schritte 1 Rahmen & Stoff · 2 Gestalten · 3 Prüfen & Garn · 4 Senden (Mac: Speichern) | **Objektliste** 268 px (Color Film) | `Stepper`, `RahmenKarte`, `StoffOption`, `StartKachel`, `HoeheRegler` (mit Band), `GarnKnopf`, `FarbstoppListe`, `Kennzahlen`, `ObjektZeile` (ziehbar) |
| Mitte | **Stichvorschau** (größte Fläche): Ansicht Stiche/Abspielen | + Form, Werkzeugleiste 44 px | `Vorschau` (SVG/Canvas, Zoom, Verschieben), `Werkzeugleiste`, `Auswahlrahmen` (8 Griffe + Drehgriff + Maß), `FormGriffe`, `EinAusGriffe`, `FehlerBanner`, `Abspielleiste` mit Farbstopp-Marken |
| Rechts | – | **Inspektor** 340 px: Objekt · Design · Einzelwerte · JSON | `Abschnitt` (aufklappbar), `Feldzeile` (Zahl/Text/Auswahl/Schalter/Segment/Wähler/Info), `HerkunftsMarke`, `ZuruecksetzenKnopf`, `XMarke` |
| Unten | Statusleiste: Stiche, Sprünge, Farbwechsel, Größe, Stickzeit*, Daten/145 KB, Hinweise | gleich | `Statusleiste`, `HinweisChip` + `HinweisListe` |
| Dialoge | Alphabet, Garn, Bild, Senden, Sticken | + Füllmuster, Step-Muster, Nach Farbe sortieren, Warnung Überdeckung, Aus der Maschine | `Dialog` (Kopf/Inhalt/Fuß), `KachelRaster`, `GarnZeile`, `SendenSchritte`, `TeilFortschritt`, `Gefahrendialog` |

Gemeinsame Regeln: Feldzeile = Beschriftung links (118 px), Wert rechts, Einheit dahinter, Hilfe/Fehler darunter. Zahlen mit Komma, `inputmode="decimal"`, Übernahme bei Änderung. X-Felder tragen im Expertenmodus die Marke **X** (nur zur Orientierung, kann später entfallen).

**Tastatur:** Tab-Reihenfolge Kopf → links → Vorschau → rechts; Esc schließt Menü/Dialog bzw. hebt Auswahl auf; Strg+Z / Strg+Y; Entf löscht; Pfeile verschieben 0,5 mm (Umschalt: 5 mm); Alt+↑/↓ = eins nach vorn/hinten; Strg+S/O/E/N; Menüs mit Pfeiltasten.

---

## 3. Zuordnung Bedienelement → Feld / Befehl — b)

Pfade wie in `SCHNITTSTELLE.md`; `objekte[i]` = gewähltes Objekt (ab 0; Anzeige ab 1). Jede Änderung → neues Design-JSON → `design_pruefen` beim Tippen, `design_berechnen` danach (Stiche nie skalieren).

### Kopf, Datei, Maschine
| Bedienelement | Feld / Befehl |
|---|---|
| Datei › Neu | leeres `DesignJson` (Standardwerte aus `design_schema`) |
| Datei › Öffnen / Speichern | `datei_lesen` / `text_speichern` (Design-JSON) |
| Datei › Beispiel laden | `beispiel_design` |
| Datei › Stickdatei exportieren (EXP), Mac: „EXP exportieren“ | `exp_speichern {design, pfad}` |
| Datei › Bild umwandeln | `bild_umwandeln {pfad, breite, farben, min_flaeche, glaettung, mit_hintergrund, rahmen}` → neues Design |
| Datei › Designs aus der Maschine | `maschine_liste`, `maschine_lesen`, `gelesen_exp_speichern` (noch App-Befehle) |
| Design-Name (Kopf) | `name` |
| Rückgängig / Wiederholen | Oberfläche: Historie der Design-JSONs |
| Modus Einfach / Experte | Oberfläche; welche Felder: `Parameter.modus` (einfach/experte) bzw. FUNKTIONEN E/X |
| Maschinen-Knopf | `maschine_ports`, `maschine_pruefen` |
| Senden … | `maschine_planen` → `maschine_senden` (+ Ereignis `maschine_fortschritt`) |

### Assistent (einfacher Modus)
| Bedienelement | Feld / Befehl |
|---|---|
| Rahmen-Karten | `rahmen` (Liste `rahmen_liste`: name, breite, hoehe, form, hinweis) |
| „Rahmen quer einspannen“ | `rahmen` mit/ohne Endung „ Rotated“ |
| Stoff-Auswahl | `stoff`, Liste aus `stoffe_liste` (Werte: satin_abstand_fest, tatami_abstand, zugausgleich) |
| Startkachel Text | neues Objekt `{art:"text", alphabet, text, hoehe, farbe}` |
| Startkachel Monogramm | `{art:"monogramm", initialen, hoehe (18), rahmen, rahmen_farbe}` |
| Text | `objekte[i].text` (`\n` = Zeile, `^` = Farbwechsel), `name` = erste Zeile |
| Alphabet | `objekte[i].alphabet` (Dialog, `alphabete_liste`) |
| Höhe (Regler + Zahl, grünes Band) | `objekte[i].hoehe`; Band = empfohlene Höhe (siehe Wunsch 1) |
| Ausrichtung | `objekte[i].ausrichtung` links/mitte/rechts (block nur Experte, braucht `grundlinie.laenge`) |
| Form gerade / Bogen oben / Bogen unten | `grundlinie` entfällt / `{art:"kreis", lage:"oben"\|"unten", radius}` |
| Garnfarbe | `objekte[i].farbe` (Wahl aus `garnkarte_garne`) |
| Größe des Designs / Drehen | `transform.skalieren` / `transform.drehen` |
| Mittig | `transform.verschieben = [0,0]` |
| Einpassen | `transform.skalieren` aus `statistik.breite/hoehe` und Stickfeld (Wunsch 6: Begrenzung) |
| Hinweis „passt nicht“ | `Ergebnis.ausserhalb > 0` → Senden gesperrt |
| Hinweis fehlende Zeichen | `Ergebnis.fehlende_zeichen` |
| Farbstopps (Schritt 3) | `Ergebnis.farben[k]` + `Ergebnis.garne[k]` (marke, nummer, name) |
| Garn ändern | `garnkarte_garne {name: garnkarte}` → `farbe` aller Objekte des Blocks = `GarnInfo.rgb` (Wunsch 3) |
| Kennzahlen | `Ergebnis.statistik` (stiche, farbwechsel, breite, hoehe); Stickzeit = Schätzung 0,4 s/Stich |
| Schritt 4 Anschluss / Prüfen / Plan | `maschine_ports` / `maschine_pruefen` / `maschine_planen` |

### Objektliste und Anordnen
| Bedienelement | Feld / Befehl |
|---|---|
| Zeilen, Nr | `objekte[]` in Stickreihenfolge, Nr = Index + 1 |
| Farbfeld, Garn, Art, Name | `farbe` + `Ergebnis.garne[block]`, `art`, `name` |
| Marke „über 2“ | `ueberdeckungen` → Paare [früher, später] |
| Marke „Fehler“ | `Pruefung.fehler.objekt` bzw. `Fehler.objekt` |
| Ziehen | `reihenfolge_aendern {befehl:{art:"vor"\|"nach", auswahl:[i], ziel:j}, design}` |
| Anordnen › An den Anfang / Ans Ende | `art:"an_anfang"` / `"ans_ende"` mit `auswahl` |
| Eins nach vorn / hinten, Vor/Nach Objekt Nr. | `art:"vor"` / `"nach"` mit `ziel` |
| Wie ausgewählt | `art:"wie_ausgewaehlt"`, `auswahl` in Klickfolge |
| Nach Farbe sortieren (Dialog) | `art:"nach_farbe", farben:[…], ueberdeckung:bool`; Anzeige `farbwechsel_vorher → farbwechsel_nachher` |
| Warnung Überdeckung | `ReihenfolgeErgebnis.vertauscht` [[oben, unten]] → „Rückgängig“ = altes Design |
| Duplizieren / Löschen | Oberfläche auf `objekte[]` |

### Vorschau und Werkzeuge
| Bedienelement | Feld / Befehl |
|---|---|
| Stiche | `Ergebnis.stiche` (x, y, art stich/sprung/farbwechsel), Farben aus `Ergebnis.farben` |
| Klick auf Stiche = Auswahl | `Ergebnis.objekt[stichnummer]` |
| Auswahlrahmen, Maß | `ObjektForm.begrenzung` |
| Ziehen / Ecken / Drehgriff | `objekte[i].transform.verschieben` / `.skalieren` / `.drehen` |
| Ansicht Form, Knoten | `design_formen` → `linien` (umriss, loch, linie, seite_a, seite_b, winkellinie); Ändern schreibt `umriss`/`loecher`/`punkte`/`seite_a`/`seite_b` (Wunsch 5) |
| Werkzeug Stichwinkel | `tatami.winkellinien`, `buchstaben[].winkellinien` |
| Werkzeug Ein-/Ausstich, Griffe „E“/„A“ | `einstich` / `ausstich` [x, y] |
| Zeichenwerkzeuge | neue Objekte `art` lauf, dreifach, satinlinie, satin (seite_a/seite_b), tatami, satinflaeche, muster, blanket, applikation, text, monogramm; Formen kreis, rechteck/oval, bogen/bezier |
| Rahmen, Stickfeld | `Ergebnis.rahmen.form` (bzw. `rahmen_liste`) |
| Sprünge / Einstiche / Raster, Zoom | Oberfläche |
| Abspielen | Oberfläche über `Ergebnis.stiche`; Marken = Farbwechsel |
| Kreuze außerhalb | Oberfläche (Stich gegen `rahmen.form`), Zahl = `Ergebnis.ausserhalb` |
| „↻ 34 ms“ | Dauer des letzten `design_berechnen` |
| Fehlerbanner, „Zum Objekt“ | `Fehler {meldung, objekt}` → Auswahl `objekt` |
| Hinweise (Statusleiste) | `Ergebnis.hinweise`, `Pruefung.hinweise`, `fehlende_zeichen`, `ausserhalb` |
| Satinflächen-Säulen (optional) | `saeulen_vorschau` |

### Inspektor › Objekt (E = immer, X = nur Experte)
| Feld | Pfad |
|---|---|
| Name | `name` |
| Garn | `farbe` → `Ergebnis.garne` (Dialog `garnkarte_garne`) |
| Text / Initialen | `text` / `initialen` |
| Alphabet / Computer-Schrift | `alphabet` (`alphabete_liste`) / `schrift` (`schriften_liste`) |
| Höhe, Ausrichtung, Kursiv | `hoehe`, `ausrichtung`, `kursiv` |
| Grundlinie | `grundlinie.art` (+ X: `radius`, `radius_y`, `lage`, `winkel`, `neigung`, `bogen`, `laenge`, `stand`, `punkte`) |
| Verzerren | `elastisch {art, oben, unten}` |
| X: Breite, Buchstaben-/Wort-/Zeilenabstand | `breite`, `abstand`, `wortabstand`, `zeilenabstand` |
| X: Stichart (Schrift) | `stichart` (text: satin/step/muster; ttf: tatami/lauf/dreifach/satin/muster) |
| X: Unterlage (Säule/Schrift) | `unterlage` (auto/keine/Arten) |
| X: Einzelne Buchstaben | `buchstaben[] {nr, farbe, transform, winkellinien}` |
| Stichwinkel | `winkel` (tatami, muster, ttf) |
| Step-Muster | `step_muster` 1–20 |
| Füllmuster | `muster` (Name „001 Diamond (90)“ oder Nummer), X: `groesse` |
| Breite (Satinlinie, Blanket) | `breite` |
| Stichart Satin/Zickzack | `stichart` (SatinStichart) |
| Geschlossen | `geschlossen` |
| Abdeckung, Stoff zuschneiden | `abdeckung`, `stil` |
| Aussparen | `aussparen` (Objektnamen), X: `ueberlappung` |
| X: Unterlage an/aus (Fläche) | `unterlage` (bool) |
| X: Mehrere Stichwinkel, Kurzstiche | `winkellinien`, `kurzstiche` |
| X: Lage | `versatz` mitte/seite |
| X: Säulen am Knick teilen | `knickwinkel` |
| X: Fester Abstand, Faktor | `abstand`, `faktor` |
| X: Stichlänge (Lauf) | `laenge` |
| X: Applikation Feinheiten | `breite`, `abstand`, `heft_breite`, `heft_abstand`, `platz_laenge` |
| Größe, Drehen, Spiegeln, Verschieben; X: Schrägstellen | `transform.skalieren`, `.drehen`, `.spiegeln`, `.verschieben`; `.schraeg` |
| X: Einstich, Ausstich | `einstich`, `ausstich` |
| X: Verstärken | `doppellauf` |
| X: Auto Jump | `auto_jump.max`, `auto_jump.sprung` |
| X: Werte je Objekt | `objekte[i].einstellungen` (→ Reiter Einzelwerte) |

### Inspektor › Design, Einzelwerte, JSON
| Bedienelement | Feld / Befehl |
|---|---|
| Rahmen, Stoff, Garnkarte | `rahmen`, `stoff`, `garnkarte` (`garnkarten_liste`) |
| Ganzes Design | `transform.*` am Design |
| X: Startpunkt, Endpunkt | `start`, `ende` |
| X: Kürzeste Verbindung | `kuerzeste_verbindung` |
| Kennzahlen | `Ergebnis.statistik` (+ min_stich, max_stich), `ausserhalb` |
| Farbstopps und Garne | `Ergebnis.farben` + `Ergebnis.garne` |
| Einzelwerte: Liste | `parameter_katalog` (gruppe, name, einheit, standard, bereich, stoff, modus, ebene); angezeigt: `modus = experte`, `ebene ∈ {beide, design}` |
| Ebene „Ganzes Design“ / „Objekt“ | `einstellungen.<pfad>` am Design / `objekte[i].einstellungen.<pfad>` (`json_design`) |
| Herkunft Standard / Stoff / Design / geändert | `standard`; Wert je Stoff (`stoff: true`); Wert im Design; eigener Wert |
| Zurücksetzen | Schlüssel löschen |
| JSON | `objekte[i]` als Text; Prüfen mit `design_pruefen`, „Übernehmen“ ersetzt das Objekt |

### Dialoge
| Dialog | Daten |
|---|---|
| Alphabet wählen | `alphabete_liste` (+ empfohlene Höhe, Bild: Wunsch 1) |
| Füllmuster wählen | `fuellmuster_liste` (name, nummer, winkel, a, b, zuege) → Kachel = Linien wiederholt um i·A + j·B |
| Step-Muster | `step_muster` (Kacheln: Wunsch 4) |
| Garn von Hand | `garnkarte_garne` |
| Bild umwandeln | `BildEingabe` → `BildErgebnis` (breite_px, hoehe_px, farben, flaechen, design) |
| Senden 1 Anschluss | `maschine_ports`, `maschine_pruefen` (Firmware, Garnfarbtabelle) |
| Senden 2 Plan | `maschine_planen {design, port, simulation, wahl: icon, farben, sticken}` → geplante Schreibzugriffe, Hinweise |
| Senden 3 Übertragen | `maschine_senden` + Ereignis `maschine_fortschritt {was, erledigt, gesamt}` |
| Senden 4 Ergebnis | Ergebnisliste, „PC-Modus verlassen“, Pfad des Mitschnitts |
| Sticken bestätigen | `maschine_senden` mit Wahl „sticken“ (Befehl 0x08) – nur nach drei Häkchen |
| Aus der Maschine | `maschine_liste` (Nr., Name, Flags), `maschine_lesen`, `gelesen_exp_speichern` |

---

## 4. Wünsche an den Kern — d)

1. **`alphabete_liste` erweitern:** empfohlene Höhe (min/max), Zeichenvorrat, Vorschau (Bild-Pfad oder Linien wie bei `fuellmuster_liste`). Heute nur `name`, `datei`; die Höhen stehen nur in `Analyse/daten/alphabets.md`, Bilder in `Arbeitsdaten/alphabete/` sind nicht im Repo (nur `Drifter_umrisse.png`).
2. **Hinweise mit Objektnummer:** `hinweise` als `{meldung, objekt?, art}` statt `string[]`; `fehlende_zeichen` je Objekt; Einstiche außerhalb je Objekt (oder Liste der Stichnummern), damit die Oberfläche markieren kann, ohne selbst zu rechnen.
3. **Garn von Hand im Design:** Feld z. B. `garn: {nummer}` am Objekt oder `garne` je Farbblock. Heute geht es nur über `farbe = GarnInfo.rgb` – mehrdeutig, wenn zwei Garne einer Karte dieselbe Farbe haben, und die Wahl ist nicht als „von Hand“ erkennbar.
4. **`GarnInfo.marke` bereinigen:** Die Isacord-Karten liefern in Feld 2 laufende Werte („Isacord 40“, „Isacord 41“ … „Isacord 354“). Vorschlag: Marke normalisieren oder zusätzlich `karte` liefern. Der Entwurf zeigt „Isacord 40“.
5. **Formen bearbeiten:** `design_formen` liefert Punkte, aber nicht, zu welchem JSON-Pfad ein Punkt gehört (bei `kreis`, `bezier`, `verbinden` entstehen Punkte erst im Kern) und keine Knotenart. Für das Knoten-Werkzeug: Pfad je Punkt oder Befehl „Punkt setzen/einfügen/löschen“.
6. **Begrenzung im Ergebnis:** `Ergebnis.begrenzung [x0,y0,x1,y1]` (heute nur Breite/Höhe) für Einpassen und Mittig.
7. **Step-Muster-Kacheln:** Versatzfolgen der 20 Step-Muster für die Vorschau (der Entwurf zeigt Platzhalter).
8. **Kennzahlen:** Datengröße in Byte (Grenze ≈ 145 KB), Stickzeit mit belegter Quelle, optional Garnverbrauch.
9. **Buchstabe je Stich:** für „einzelne Buchstaben“ (`buchstaben[nr]`) per Klick: zu jedem Stich die Buchstabennummer (wie `objekt`).
10. **Maschinen-Befehle in `stick-api`** mit Schema (`maschine_ports … gelesen_exp_speichern`); dazu **Abbrechen** während des Sendens, Fortschritt mit **Wiederholungen** (Block, Versuch), Fehler als **Code** (keine_verbindung, stickmodul, antwortet_nicht, zurueckgelesen_weicht_ab, farbtabelle, zu_gross, ausserhalb).
11. **Applikation an den Farbstopps:** im Ergebnis je Farbblock die Art (Platzierung, Heften, Abdecken), damit Senden-Ergebnis und Abspielen Anleitungsschritte zeigen können („Stoff auflegen“, „zuschneiden“).
12. **Rahmen der Maschine:** falls `maschine_pruefen` den eingesetzten Rahmen lesen kann, mitliefern – dann erscheint die Anleitung nach Rahmenwechsel nur, wenn sie nötig ist (sonst merkt sich die Oberfläche den zuletzt gesendeten Rahmen).

---

## 5. Design-Tokens und Icons — e)

Basis ist das Designsystem **Industry** (bereits in v2 gewählt), `project/_ds/industry-…/styles.css`. **Offline:** Barlow und Barlow Condensed in die App einbetten (keine Google-Fonts-Anfrage).

| Token | Hell | Dunkel | Verwendung |
|---|---|---|---|
| `--color-bg` | #f2f2f3 | #16191c | Fenster, Dialoge |
| `--panel` | #f7f7f8 | #1b1e22 | Seitenleisten |
| `--color-surface` | #e9e9ea | #1f2327 | Eingabefelder |
| `--color-text` | #1d1f20 | #e6e8ea | Text |
| `--muted` | #55585b (6,4 : 1) | #a9adb1 | Beschriftungen, Hilfe |
| `--color-divider` | Text 16 % | Text 18 % | Linien |
| `--color-accent` | #5980a6 | #7fa6cc | Auswahlrahmen, Linien, Fortschritt (nur Flächen/Rahmen, 3,7 : 1) |
| `--color-accent-700` | #416180 (5,8 : 1) | #b5d9fd | Primärknopf und gewähltes Segment (Fläche mit hellem Text – Industry-Standard #5980a6 hätte nur 3,7 : 1), Akzent als Text, Stickfeld-Linie |
| `--color-accent-100` | #eef6ff | #1f3346 | gewählte Zeile/Karte |
| `--ok` / `--ok-bg` | #2b6a3b / #e6f2e9 | #93d6a3 / #15301d | passt, gesendet |
| `--warn` / `--warn-bg` | #8a4300 / #fff3df | #f3b866 / #35270f | Hinweise |
| `--err` / `--err-bg` | #b42318 / #fdeceb | #ff9b91 / #3a1714 | Fehler, außerhalb |
| `--sim` / `--sim-bg` | #5b3d91 / #efe9f8 | #a58be0 / #2a2140 | **nur** Simulation (Streifenband) |
| `--danger` | #9b1c1c | gleich | „Jetzt sticken“, Gefahrendialog (+ Warnstreifen #1d1f20/#e3a600) |
| Vorschau | Grund #dcdde0, Stoff #fbfaf7, Rahmen #c9cbd0 | #0f1113, #2a2d31, #30343a | Stichvorschau |

- **Schrift:** Überschriften Barlow Condensed 600 (Dialog 21 px, Assistent 25 px, Abschnitt 15 px), Text Barlow 13,5 px / 1,4; klein 12 px; Zahlen `tabular-nums`.
- **Abstände:** Raster 4 px; Felder 4/8 px, Abschnitte 14 px Rand, Dialoge 16 px. Ecken 0 (Industry), Blueprint-Ecken nur am Primärknopf.
- **Maße:** Kopf 48, Assistent 360, Objektliste 268, Werkzeugleiste 44, Inspektor 340, Statusleiste 30, Feldzeile 30 hoch, Beschriftungsspalte 118.
- **Fokus:** 2 px Akzent, 2 px Abstand (`:focus-visible`). Farbe nie allein: Fehler/Hinweise immer mit Symbol und Text, Garne mit Nummer und Name, Ein-/Ausstich mit „E“/„A“.
- **Icons:** Lucide, Strich 1,5 – mouse-pointer, pen-tool (Knoten), triangle-right/Winkel, log-in (Ein/Aus), type (Text), Monogramm „M“, minus/equal (Lauf/Dreifach), Zickzack (Satinlinie), Säule, square-dashed hatch (Tatami), star (Satinfläche), grid (Füllmuster), Kamm (Blanket), hexagon (Applikation), circle, square, spline (Bézier), image, sparkles („kommt noch“), send, download, folder-open, undo-2, redo-2, zoom-in/out, maximize, play/pause, grip-vertical, copy, trash-2, rotate-ccw (Zurücksetzen), alert-triangle, x-circle, check, chevron-down/right, arrow-up/down, monitor (Simulation). Eigene Icons für Satinsäule, Blanket, Applikation, Monogramm (Pfade im Entwurf, Konstante `ART`).

---

## 6. Antworten auf die offenen Fragen (ANFORDERUNGEN §7)

1. **Ein Fenster oder geführter Ablauf?** → **Beides in einem Fenster.** Einfacher Modus = Assistent links (4 Schritte), Expertenmodus = freie Arbeitsfläche; Umschalter im Kopf, jederzeit ohne Verlust, weil beide dasselbe Design-JSON bearbeiten. *Grund:* Der häufigste Fall (Name auf Handtuch) braucht 4 Schritte, Digitalisieren braucht Werkzeuge und Inspektor – die Vorschau bleibt in beiden die größte Fläche.
2. **Wo leben Objektliste, Eigenschaften, Garnliste?** → Objektliste **links** (wie Color Film), Inspektor **rechts** mit Reitern Objekt · Design · Einzelwerte · JSON; Garnliste im Reiter Design und im Assistent-Schritt 3. *Grund:* Reihenfolge lesen und ziehen gehört zur Liste, Werte zum gewählten Objekt; feste Seitenleisten statt schwebender Paletten (Tastatur, 1280 px).
3. **Darstellung des Sendens?** → **Modaler Dialog in 4 Schritten** (Anschluss → Plan prüfen → Übertragen → Ergebnis); während der Übertragung „Im Hintergrund weiter“ – Fortschritt dann im Maschinen-Knopf, Bearbeiten erlaubt, Maschinen-Aktionen gesperrt. Sticken ist ein **eigener Gefahrendialog** mit drei Häkchen. *Grund:* Schreiben in die Maschine ist der einzige riskante Vorgang – er soll nicht beiläufig in einer Seitenleiste passieren, aber das Arbeiten nicht blockieren.
4. **Geplante Funktionen ohne Überladen?** → Alle Objektarten im Menü **Einfügen** und in der Werkzeugleiste (Experte); im Assistenten nur Startkacheln Text, Monogramm, Bild, Öffnen. Computer-Schriften als zweite Quelle der Schrift (Experte), Applikation als Objektart mit Anleitung an den Farbstopps, Bild → Umrisse als Dialog. Alles aus FUNKTIONEN §7 gesammelt in der rechten Spalte von „Einfügen“ und hinter dem Stern-Knopf der Werkzeugleiste, deutlich „kommt noch“ und deaktiviert. *Grund:* ein Ort für Neues, keine verstreuten toten Knöpfe.
5. **Was bleibt vom JSON-Editor?** → Reiter **JSON** im Inspektor (nur Experte), zeigt das gewählte Objekt (Punktlisten gekürzt), prüft beim Tippen mit `design_pruefen`, „Übernehmen“ setzt es. Das ganze Design über Datei öffnen/speichern. *Grund:* Jede neue Kern-Funktion ist sofort nutzbar, bevor sie ein eigenes Bedienelement hat (FUNKTIONEN §1).

---

## 7. Sicherheitsregeln (ANFORDERUNGEN §4) im Entwurf

| Regel | Umsetzung |
|---|---|
| Nichts schreiben ohne Bestätigung, vorher Plan | Schritt „Plan prüfen“ listet alle Schreibzugriffe mit Größe; „Bestätigen und schreiben“ ist der einzige Weg |
| Sticken nur mit eigener Bestätigung | eigener Dialog, rotes Warnband, drei Häkchen, Knopf „Sticken starten – Rahmen fährt“ erst dann aktiv |
| Senden gesperrt bei: außerhalb, Fehler, laufender Aktion | Senden-Knopf deaktiviert mit Grund (Titel und Text im Schritt 4) |
| Nichts dauerhaft gespeichert | Hinweis im Plan und in Schritt 4 |
| Simulation unverwechselbar | violettes Streifenband oben und im Dialog, Maschinen-Knopf „Simulation“ |
| Fehlerzustände | Dialog-Zustand „Maschine antwortet nicht“ mit Teilergebnis und nächsten Schritten; Wiederholungen sichtbar |
| Hinweise für die Person an der Maschine | Anleitung nach Rahmenwechsel (Layout 1 → «check» → «Mustermitte anzeigen» → «ok»), Material, Fäden selbst schneiden, Farbfolge mit Garnnamen |

---

## 8. Zustände im Entwurf (`?zustand=`)

Einfach: `einfach-leer`, `einfach-rahmen`, `einfach-text`, `einfach-alphabet`, `einfach-hinweis`, `einfach-pruefen`, `einfach-garn`, `einfach-senden` ·
Experte: `experte-leer`, `experte-auswahl`, `experte-form`, `experte-text`, `experte-werte`, `experte-design`, `experte-muster`, `experte-step`, `experte-anordnen`, `experte-farbe`, `experte-warnung`, `experte-fehler`, `experte-abspielen`, `experte-einfuegen`, `experte-bild`, `experte-json` ·
Maschine: `senden-anschluss`, `senden-plan`, `senden-fortschritt`, `senden-fertig`, `sticken-bestaetigen`, `simulation`, `maschine-fehler`, `aus-maschine` ·
Plattform: `mac`, `dunkel`.
