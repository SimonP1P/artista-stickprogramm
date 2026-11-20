# 15 – Bestandsaufnahme: Funktionen des Originals gegen den eigenen Rechenkern (20.11.2026)

**Zweck:** Vollständige Liste, was BERNINA artista V4 kann, und was davon der eigene Kern schon kann – Grundlage für die
Restliste der Roadmap (Phase 4b) und später für den Parameterkatalog (ROADMAP Merkliste M1).
**Quellen:** Handbuch (37 Kapitel, Abläufe „To …“ je Kapitel aus `Arbeitsdaten/texte/man_raw_pg.txt`), Werkzeuge je
Leiste („Use X (… toolbar)“), Stichart-Namen der Stringtabelle (`Arbeitsdaten/ressourcen/BERESENU.DLL.txt` STR 2234 ff.,
32649 ff.), Objektklassen (`Analyse_Teil3/03`). Sicherheit: hoch für „gibt es im Original“ (Handbuch), Einträge nur aus
Strings sind als „nur Strings“ markiert (vermutlich Wilcom-Kern, in artista V4 nicht bedienbar).

**Zeichen:** ✅ im Kern · 🟡 teilweise · ⬜ fehlt · 🖥 Aufgabe der Oberfläche (kein Rechenkern nötig oder nur Hilfsfunktion)
· ➖ für dieses Projekt nicht nötig (Begründung).

## A. Sticharten für Linien (Kap. 8, 26)
| Funktion (Original) | Stand | Bemerkung |
|---|---|---|
| Single (Lauf), Triple (Dreifach) | ✅ | `lauf`, `dreifach` |
| Satin outline (Satinlinie), Abstand, Auto Spacing | ✅ | `satinlinie` |
| Satin outline **Versatz** (Center/Offset) | ✅ | 20.11. `versatz` mitte/seite (Seite links: eigene Festlegung) |
| Blanket, Abstand, Breite, Versatz | ✅ | `blanket` (Form eigene Festlegung) |
| Double Run (Verstärken, rückwärts doppelt) | ✅ | 20.11. `doppellauf` an jedem Objekt (Kap. 23) |
| Pattern Run (Motivlinie), Muster/Skalierung/Abstand, eigene Muster | ⬜ | Kap. 25, „Motif Run“ |
| Blackwork Run / Blackwork Border | ⬜ | Kap. 26 |
| Backstitch, Stemstitch (Rück-, Stielstich) | ⬜ | Kap. 26, Klassen 0x97/0x96 |
| Candlewicking Border | ⬜ | Kap. 26 |

## B. Sticharten für Flächen (Kap. 8, 24–26)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Satin-Füllung, Säule variabler Breite, Auto Spacing | ✅ | `satin` |
| Step (Tatami), Länge, Abstand, Versatz | ✅ | `tatami` |
| **Step-Muster** (Nr. 1 Standard, weitere Vorgaben) | ⬜ | Kap. 8 „Selecting Step patterns“ |
| Mehrere Stichwinkel (Turning/Complex Fill) | ⬜ | Kap. 16 „set multiple stitch angles“ |
| Zickzack-Füllung („Zigzag“, Stil 0x2724) | ⬜ | Stichartliste STR 2236 |
| Fancy Fill (Program Split), Abstand, Winkel | ✅ | `muster`, 242 Muster |
| Pattern Fill (Motivfüllung), Layout am Bildschirm, eigene Sets | ⬜ | Kap. 25, „Motif Fill“ 0x3d |
| Blackwork Fill, Candlewicking Fill, Lacework, Stippling | ⬜ | Kap. 26 |
| Contour, Coil, Island Coil, Straight, Perpendicular Fill, Trapunto | ➖ | nur Strings, im Handbuch nicht beschrieben |

## C. Effekte (Kap. 22–24)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Unterlagen (Mittellauf, Edge Walk, Zickzack, Doppel-Zickzack, Step), Ränder | ✅ | Auto Underlay wie Stil |
| Zugausgleich (auto, Werte) | ✅ | |
| Kurzstiche (Satin) | ✅ · ⬜ Tatami | |
| Auto Jump, Mindeststich, Vernähen | ✅ | |
| „Säule mit mehr Loft“ (erhöhter Satin, Kap. 22) | ✅ | laut Handbuch Auto Jump mit kleinem Höchststich (z. B. 6 mm) + Unterlage: `"auto_jump": {"max": 6}` |
| Textured Edge (ausgefranster Rand) | ⬜ | Kap. 23/24 |
| Wave Effect (Füllung entlang einer Kurve) | ⬜ | Kap. 24/26 |
| Gradient Fill, Open Fill | ⬜ | Kap. 24 |
| Color Blending (zwei Farben ineinander) | ⬜ | Kap. 24 |

## D. Schrift (Kap. 18–21)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Original-Alphabete (ESB), Höhe, Breite, Abstand, Ausrichtung | ✅ | 50 Alphabete |
| Computer-Schriften (TTF/OTF) | ✅ | Stichart eigene Festlegung |
| Unterlagen zuerst je Säulengruppe | ✅ | Bericht 11 |
| Säulenreihenfolge/Start/Ende wie Original | 🟡 | Analyse läuft (Bericht 11 §4) |
| Mehrere Zeilen, Zeilenabstand, Leerzeichenbreite | ✅ | Vorlage 0x3e82 (20.11.) |
| Kursiv, Fett | ✅ | Kursiv ±45° (20.11.); Fett = Zugausgleich 0,4–0,6 mm laut Handbuch |
| Grundlinien: waagrecht, senkrecht, **Kreis/Bogen**, mehrere, eigene Kurve | ✅ | 20.11.: alle Arten des Dialogs „Layout“ (Kreis CW/CCW, Oval, Tilt, Arc Angle, Above/Through/Below, gerade mit Winkel/Länge, senkrecht, Any Shape, Multiple), Ausrichtung Full |
| Abstand/Position/Form einzelner Buchstaben, Farbe je Buchstabe | 🟡 | 20.11.: Farbe (`^`, je Buchstabe), verschieben/drehen/skalieren ✅; Umriss umformen ⬜ |
| Stichart je Schrift (Satin/Step/Fancy), Stichwinkel je Buchstabe | 🟡 | 20.11.: `stichart` satin/step/muster ✅ (alle 56 Schriftobjekte mit Stichen in den Musterdesigns sind Satin – Step/Fancy nicht messbar, Verteilung eigene Festlegung); Stichwinkel je Buchstabe ✅ 20.11. (`winkellinien`: Säulen behalten Anfang/Ende, dazwischen die neuen Linien; Füllungen nehmen die Linienrichtung) |
| Auto Jump bei Schrift aus, Unterlage bei kleiner Schrift aus | 🟡 | Unterlage aus: `"unterlage": []` (Handbuch: Anwender schaltet ab); Auto Jump ✅ 20.11. je Objekt (`auto_jump`, Schrift aus, eigene Werte) |
| Elastic Lettering (Verzerren: Bogen, Welle …) | ✅ | 20.11.: `elastisch` gerade (Straight) / gebogen (Curved); Kantenform linear/Parabel eigene Festlegung |
| Monogramme | ✅ | 20.11.: `monogramm` → Diamond2/Diamond3 (Belegung `Analyse_Teil3/09` §7) |
| Symbole/Zeichentabelle | 🟡 | Zeichen der Alphabete; Auswahl 🖥 |
| Schrift spart aus (Lettering Remove Overlay) | ✅ | `aussparen` |

## E. Formen und Bearbeitung (Kap. 7, 14–17)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Offene/geschlossene Kurve, Kreis/Oval, Rechteck, Bézier | ✅ | Formen im Design-JSON (`oval` seit 20.11.) |
| Löcher (Add Holes), Aussparen/Wiederherstellen | ✅ | |
| **Skalieren, Drehen, Spiegeln, Schrägstellen** je Objekt und Design (Stiche neu) | ✅ | `"transform"` (20.11.) |
| Ausrichten, Gruppieren | ⬜ | Kap. 15 |
| Mirror-Merge, Wreath (Kranz), Kaleidoscope | ⬜ | Kap. 15 |
| Stichwinkel ändern, Ein-/Ausstich je Objekt setzen | 🟡 | Winkel ✅, Ein-/Ausstich ⬜ |
| Reihenfolge: nach Farbe, an Anfang/Ende, automatisch | ⬜ | Kap. 14 (mit Überdeckungsprüfung) |
| Stiche einzeln einfügen/verschieben/löschen | ⬜ 🖥 | Kap. 17 |
| Start und Ende des Designs | ✅ | `start`/`ende` |
| Knopflöcher (einfügen, Typ, Länge, Dichte, verbinden) | ⬜ | Kap. 23, Klasse 0x95 |

## F. Applikation (Kap. 27)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Auto Appliqué: Platzierung, Schnittlinie, Heften, Abdecken Satin/Blanket | ✅ | |
| Versatz innen/außen, Vorgabewerte AutoAppliqueValues | ⬜ | |
| Partial Appliqué (Teil-Abdeckung) | ⬜ | |
| Applikationsvorlage drucken | ⬜ 🖥 | Schnittmuster 1:1 |

## G. Automatisches Digitalisieren (Kap. 11–13)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Bild laden, zuschneiden, Farben reduzieren | 🟡 | `stick-vektor` |
| Auto Digitizer (Bild → Objekte mit Stichart) | 🟡 | Flächen ✅, Linien/Satin ⬜, maßstabsfrei ⬜ (M2) |
| Magic Wand (Fläche/Kontur per Klick) | ⬜ | Kap. 13 |
| PhotoSnap (Foto → Stiche) | ⬜ | Kap. 13 |
| Scanner | ➖ | heute Bilddatei |

## H. Rahmen, Farben, Stoffe (Kap. 6, 10, 22)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Rahmenliste, Ausrichtung | ✅ | welche Rahmen die 185 annimmt: prüfen |
| Eigene Rahmen (Rechteck, Oval, Sliding, 3-Position) | ⬜ | Kap. 6 |
| Garnkarten, Garn suchen/zuordnen | ✅ | CIEDE2000 |
| Eigene Garnkarten, Farben mischen | ⬜ 🖥 | |
| Stoffe (8 + Defaults) | ✅ | |
| Vorlagen/Stile speichern (Templates, NORMAL) | ⬜ | Kap. 9 – hängt am Parameterkatalog |

## I. Dateien und Maschine (Kap. 28–30)
| Funktion | Stand | Bemerkung |
|---|---|---|
| An die Maschine senden, aus der Maschine lesen | ✅ | seriell, artista 185 |
| EXP schreiben/lesen | ✅ | |
| ART-Designs öffnen (Objekte) | 🟡 | Werkzeuge in Python, nicht im Kern |
| Fremde Stickdateien mit Objekterkennung öffnen, PES speichern | ⬜ | Kap. 28 |
| Designkarte, Magic Box | ➖ | Ziel ist nur das Kabel an der 185 |
| Druck (Arbeitsblatt) | ⬜ 🖥 | |

## J. Eigene Programme
| Funktion | Stand | Bemerkung |
|---|---|---|
| Cross Stitch (Kap. 34–37: Raster, Auto Stitch, Magic Wand, Buchstaben, Ränder) | ⬜ | eigenes Modul, großer Umfang |
| Portfolio (Dateiverwaltung, Kataloge) | 🖥 | Projekte in der Oberfläche |
| Quilter | ➖ | eigenes Programm, Hilfe auf der CD defekt; erst bei Bedarf |
| Ansicht, Zoom, Raster, Messen, Auswahl, Rückgängig, Stichvorschau | 🖥 | Oberfläche |
