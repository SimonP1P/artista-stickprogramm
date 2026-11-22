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
| Objekteigenschaften: Stil-Werte je Objekt (Abstand, Länge, Zugausgleich, Unterlage, Verbinder …) | ✅ | 21.11. `einstellungen` am Objekt (ROADMAP 4b.16); Beleg: jedes ART-Objekt trägt eine eigene Kopie des Stil-Baums (`Analyse_Teil3/03` §4, hoch) |
| Pattern Run (Motivlinie), Muster/Skalierung/Abstand, eigene Muster | ⬜ | Kap. 25, „Motif Run“ |
| Blackwork Run / Blackwork Border | ✅ 22.11. `blackwork` (Netz, Euler-Weg – eigene Festlegung), Rand = Motivlinie `_BW004a` | Kap. 26, Bericht 24 §1/§5 |
| Backstitch, Stemstitch (Rück-, Stielstich) | ✅ 22.11. `rueckstich` (MOD1 0x1001d960, hoch), `stielstich` (mittel) | Kap. 26, Klassen 0x97/0x96, Bericht 24 §2/§3 |
| Candlewicking Border | ✅ 22.11. Motivlinie `_CW004a` Knot-3mm, Abstand 7,0 mm | Kap. 26, Bericht 24 §1 |

## B. Sticharten für Flächen (Kap. 8, 24–26)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Satin-Füllung, Säule variabler Breite, Auto Spacing | ✅ | `satin` |
| Step (Tatami), Länge, Abstand, Versatz | ✅ | `tatami` |
| **Step-Muster** (Nr. 1 Standard, weitere Vorgaben) | ✅ | 20.11. `step_muster` 1–20 aus DIALOG.DLL (Bericht 06 §5) |
| Mehrere Stichwinkel (Turning/Complex Fill) | ✅ | 20.11. `winkellinien` an `tatami` (Step längs der Linien, gemessen 83 % < 10°; ohne Löcher) |
| Zickzack-Füllung („Zigzag“, Stil 0x2724) | ✅ | 20.11. `stichart: zickzack` an `satin`/`satinlinie`, Abstand 0x2725 (2,0 mm, Stoffe 1,0 mm) |
| Fancy Fill (Program Split), Abstand, Winkel | ✅ | `muster`, 242 Muster |
| Pattern Fill (Motivfüllung), Layout am Bildschirm, eigene Sets | ⬜ | Kap. 25, „Motif Fill“ 0x3d |
| Blackwork Fill, Candlewicking Fill, Lacework, Stippling | ✅ 22.11. Motivfüllung mit den Werten der Stile; `stippling` eigene Festlegung | Kap. 26, Bericht 24 §1/§4 |
| Contour, Coil, Island Coil, Straight, Perpendicular Fill, Trapunto | ➖ | nur Strings, im Handbuch nicht beschrieben |

## C. Effekte (Kap. 22–24)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Unterlagen (Mittellauf, Edge Walk, Zickzack, Doppel-Zickzack, Step), Ränder | ✅ | Auto Underlay wie Stil |
| Zugausgleich (auto, Werte) | ✅ | |
| Kurzstiche (Satin) | ✅ | Satin; seit 20.11. auch Step-Flächen mit Winkellinien und Step-Schrift (gerade Tatami-Reihen haben keine Innenkurven) |
| Auto Jump, Mindeststich, Vernähen | ✅ | |
| „Säule mit mehr Loft“ (erhöhter Satin, Kap. 22) | ✅ | laut Handbuch Auto Jump mit kleinem Höchststich (z. B. 6 mm) + Unterlage: `"auto_jump": {"max": 6}` |
| Textured Edge (ausgefranster Rand) | ✅ 22.11. `effekte.rand` | Kap. 23/24, Bericht 23 |
| Wave Effect (Füllung entlang einer Kurve) | ✅ 22.11. `effekte.welle` | Kap. 24/26, Bericht 23 |
| Gradient Fill, Open Fill | ✅ 22.11. `effekte.verlauf`; Open Fill = Laufwege am Rand | Kap. 24, Bericht 23 |
| Color Blending (zwei Farben ineinander) | ✅ 22.11. Befehl `farben_mischen` | Kap. 24, Bericht 23 |

## D. Schrift (Kap. 18–21)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Original-Alphabete (ESB), Höhe, Breite, Abstand, Ausrichtung | ✅ | 50 Alphabete |
| Computer-Schriften (TTF/OTF) | ✅ | Stichart Tatami/Lauf eigene Festlegung; **Satin wie die TrueType-Umwandlung** ✅ 20.11. (Bericht 17) |
| Unterlagen zuerst je Säulengruppe | ✅ | Bericht 11 |
| Säulenreihenfolge/Start/Ende wie Original | 🟡 | 20.11. gemessen (Bericht 11 §5): Reihenfolge + Richtung im Kern ✅, Teilen der Säulen in Stücke ⬜ |
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
| Stichwinkel ändern, Ein-/Ausstich je Objekt setzen | ✅ | Winkel ✅; Ein-/Ausstich ✅ 20.11. (`einstich`/`ausstich`, Closest Join `kuerzeste_verbindung`; Umsetzung je Objektart eigene Festlegung, Bericht 16) |
| Reihenfolge: wie ausgewählt, an Anfang/Ende, nach Farbe, nach Nummer | ✅ | 20.11. `stick_kern::reihenfolge`, CLI `stick reihenfolge` (Kap. 14; Resequence MOD1 0x1006db20, Bericht 16) |
| Prüfung auf Überdeckungen beim Umordnen | ✅ | 20.11. **eigene Erweiterung** (Original prüft nicht): Warnung + Sortieren nach Farbe mit `--ueberdeckung` |
| Automatic sequencing (Blackwork Run: überlappende Linien zu einem Objekt) | ✅ 22.11. `blackwork_lauf` | Kap. 14 „Automatic sequencing“ = Craft Stitch Blackwork Run → ROADMAP 4b.9 |
| Stiche einzeln einfügen/verschieben/löschen | ⬜ 🖥 | Kap. 17 |
| Start und Ende des Designs | ✅ | `start`/`ende` |
| Knopflöcher (einfügen, Typ, Länge, Dichte, verbinden) | ✅ | 21.11. 9 Typen aus ButtonHoles.ESB, Länge/Abstand/Winkel wie MOD1, Einarbeiten über `aussparen` (Mittelumriss, niedrig); `"art": "knopfloch"`, Bericht 19 |

## F. Applikation (Kap. 27)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Auto Appliqué: Platzierung, Schnittlinie, Heften, Abdecken Satin/Blanket | ✅ | |
| Versatz innen/außen, Vorgabewerte AutoAppliqueValues | ✅ 22.11. `versatz`, Vorgaben aus MOD1 0x101f3c70 | Bericht 12 §8 |
| Partial Appliqué (Teil-Abdeckung) | ✅ 22.11. `teilabdeckung` (Schnitt eigene Festlegung) | Bericht 12 §8 |
| Applikationsvorlage drucken | ⬜ 🖥 | Schnittmuster 1:1 |

## G. Automatisches Digitalisieren (Kap. 11–13)
| Funktion | Stand | Bemerkung |
|---|---|---|
| Bild laden, zuschneiden, Farben reduzieren | 🟡 | `stick-vektor` |
| Auto Digitizer (Bild → Objekte mit Stichart) | 🟡 | Flächen ✅, Satin-Zerlegung schmaler Flächen ✅ 20.11. (`satinflaeche`, Bericht 17); 21.11.: Vorbereitung (vereinfachen, Lab-Farben, Garnfarben, kleine Flächen zuschlagen), „Omitted Colors“, „Fills: Auto/Step“, Zugausgleich 0,4 + Unterlage (Handbuch S. 116–118) ✅ – Verfahren eigene Festlegung; Linien/„Details“ ⬜, maßstabsfrei ⬜ (M2) |
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

## K. Erweiterungen über das Original hinaus (Wünsche des Nutzers)
| Funktion | Stand | Bemerkung |
|---|---|---|
| SVG-Import (Inkscape → Stickdesign) | ✅ | 21.11. ROADMAP 4b.17 (Bericht 18): `stick svg`, Befehl `svg_lesen`; Stichart nach Breite wie EasyStitch des Originals (Stile und Werte aus NORMAL.AMT), schmale Flächen als `satinflaeche` (Bericht 17) |
| Parameterkatalog einfach/experte/automatisch | ✅ | `PARAMETER.md` (20.11.2026) |
