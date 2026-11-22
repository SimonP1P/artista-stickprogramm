# Änderungen für die Oberfläche

**Für den UI-Entwickler und den Designer:** Was sich im Kern geändert hat, das die Oberfläche betrifft – neueste
zuerst. Jeder Eintrag nennt, was neu ist, wo es steht und was in der Oberfläche zu tun ist. Die vollständigen Felder
und Befehle stehen immer aktuell in `SCHNITTSTELLE.md` (erzeugt), die Bedeutung für den Anwender in
`UI_Mockups/FUNKTIONEN.md`.

**Abgleich:** Die Oberfläche trägt unten unter „Abgeglichen bis“ den Commit ein, bis zu dem sie alles umgesetzt hat.
Neu ist alles darüber. Technisch prüfen:
`git log --oneline <commit>..HEAD -- kern/schnittstelle SCHNITTSTELLE.md UI_Mockups/FUNKTIONEN.md UI_Mockups/AENDERUNGEN.md`
und `git diff <commit>..HEAD -- kern/schnittstelle/kern.ts` (TypeScript-Typen).

Pflege: Claude Code trägt hier bei jeder Kern-Änderung, die die Oberfläche betrifft, einen Eintrag ein
(`CLAUDE.md` §4 Regel 7).

---

## 22.11.2026 – Zauberstab, PhotoSnap, kürzeste Wege (Nachtauftrag N5, 4b.11; Commit 71d232d)
- **Neue Objektart `photosnap`** (Handbuch S. 119–120): Foto als Reihen von Zickzackstichen. Felder `raster` (vom Befehl
  `photosnap` erzeugt: `breite`, `hoehe`, `mm`, `lagen[{farbe, werte (Base64, Deckung 0–255)}]` – nicht von Hand
  bearbeiten), `aufloesung` grob/mittel/fein, `hintergrund` hell/dunkel, `winkel`, `lagen` gekreuzt/versetzt,
  `abstand_min`/`abstand_max`. Jede Farblage wird ein Farbblock. Größe über `transform`.
- **Neue Befehle:** `zauberstab` (Klicks im Vorschaubild → Design mit einem Objekt je Fläche/Umriss) und `photosnap`
  (Bild → Design mit einem `photosnap`-Objekt).
- **Neues Feld an jedem Objekt:** `kuerzeste` (bool) = Closest Join nur für dieses Objekt. `design_einfuegen` setzt es auf
  die eingefügten Objekte, wenn das eingefügte Design `kuerzeste_verbindung` hat und das Ziel nicht.
- `bild_umwandeln`: Flächen einer Farbe in kürzerer Reihenfolge (2-opt), `kuerzeste_verbindung` an.
- `objekt_werte` liefert für Auswahlfelder von PhotoSnap und Applikation den wirksamen Wert (z. B. `aufloesung` „mittel“).
- **Oberfläche (umgesetzt):** Dialog „Bild umwandeln“ mit drei Wegen oben (Alles umwandeln / Zauberstab / PhotoSnap);
  Zauberstab: Stichart wählen, ins Bild klicken (schraffiert markiert), Liste der Klicks mit Stichart und Entfernen;
  PhotoSnap: einfarbig/4 Farblagen, Stoff, Auflösung, Winkel, Garnfarben. Menüs Datei und Einfügen: „Zauberstab …“,
  „PhotoSnap (Foto) …“. Inspektor PhotoSnap: Auflösung, Stoff, Winkel der Reihen, Farblagen (einfach), Abstände (X).
  Leere Auswahlfelder zeigen „Standard (wirksamer Wert)“.

## 22.11.2026 – Applikation: Vorgaben des Originals, Versatz, Teil-Abdeckung (Nachtauftrag N4, 4b.10; Commit 00823c2)
- **Neue Felder** an `applikation`: `versatz` (%, −100 außen … 0 mittig … 100 innen; gilt für Abdeck- und Heftstich,
  bei Blanket ohne Wirkung) und `teilabdeckung` (bool, Partial Appliqué: kein Abdeckstich, wo eine spätere Applikation
  den Rand überdeckt). Bereichsprüfung mit Objektnummer für alle Zahlenfelder der Applikation.
- **Geänderte Standards (Bedeutung):** Ohne Angabe gelten jetzt die Vorgaben von „Auto Appliqué“ im Original
  (`AutoAppliqueValues`): Abdeckung **5,0 mm** (vorher 2,5), Abstand Satin **0,5 mm** fest (vorher Stoff/Auto Spacing),
  Heftstich **4,0 / 5,0 mm** (vorher 3,0 / 2,0), Platzierung **1,5 mm** (vorher 2,0). Format-Version nicht erhöht –
  Werte ohne Angabe sind Vorgaben, keine geänderte Feldbedeutung; alte Dateien mit Angaben bleiben gleich.
- `objekt_werte` liefert für die Applikation zusätzlich `abstand` (Satin 0,5 / Blanket 2,0) und `versatz`.
- **Oberfläche (umgesetzt):** Inspektor oben Abdeckung, Stil, **Versatz (außen − / innen +)**, **Teil-Abdeckung**;
  Anordnen → „Teil-Abdeckung (Partial Appliqué)“ (ab zwei gewählten Applikationen).

## 22.11.2026 – Craft Stitches (Nachtauftrag N3, 4b.9; Commit d956fd2)
- **Neue Objektarten** (`format.rs`): `rueckstich` (`punkte`, `geschlossen`, `laenge`, `staerke`, `ueberlappung` in %,
  `striche` 3|5), `stielstich` (`punkte`, `geschlossen`, `breite`, `abstand`, `winkel`, `dreifach`, `staerke`),
  `stippling` (`umriss`, `loecher`, `laenge`, `abstand`), `blackwork` (`linien` = Liste von Formen, `stich`
  `lauf`/`dreifach`/`rueckstich`/`stielstich` und deren Werte). Ohne Angabe gelten die Werte des Originals
  (`objekt_werte` liefert sie). Werte außerhalb des Bereichs → Fehler mit Objektnummer.
- `stichart_aendern` kennt `rueckstich`, `stielstich` (Umriss) und `stippling` (Füllung).
- **Neue Befehle** `craft_vorlagen` (Liste der 8 Vorlagen der Craft-Stitch-Leiste: `id`, `name`, `original`, `form`
  „linie“/„flaeche“, `felder`, `beschreibung`) und `blackwork_lauf` (`{design, objekte}` → `{design, objekt,
  hinweise}`).
- **Geänderter Standard (Bedeutung):** `motivlinie` und `motivfuellung` ohne `satz`/`motiv` nehmen jetzt **NP004a /
  K0001a** wie das Original (vorher Children 901 bzw. Heirloom 705; Bericht 20 hatte die Stilteile vertauscht, Bericht
  24 §1). Format-Version nicht erhöht: Die Oberfläche schreibt `satz`/`motiv` bei neuen Objekten nicht mehr aus, ältere
  Dateien mit ausgeschriebenem Satz bleiben gleich; nur Dateien ohne Angabe sticken anders.
- **Oberfläche (umgesetzt):** Einfügen → „Craft Stitches“ (Vorlagen aus `craft_vorlagen`, Linie bzw. Kreis in der
  Rahmenmitte), Stichart-Menüs Umriss/Füllung mit Rückstich, Stielstich, Stippling, Inspektor-Felder (oben Länge/Breite/
  Abstand, Feinheiten unter X), Anordnen → „Blackwork Run“ (ab zwei geeigneten Linien), Knoten-Werkzeug je
  Blackwork-Linie, Icons; „Craft Stitches“ aus „Kommt noch“ entfernt.

## 22.11.2026 – Effekte und Farben mischen (Nachtauftrag N2, 4b.8; Commit 47fd38a)
- **Neues Feld `effekte`** an `tatami`, `muster`, `satin`, `satinlinie`, `satinflaeche` (`format.rs` `EffekteJson`,
  Kern `stick-kern::effekte`): `verlauf` {`profil` (8 Werte, Standard `linear`), `min` 0,4, `max` 2,0 mm} · `rand`
  {`seite` links/rechts/beide (Standard rechts), `spanne` 1,5 mm, `glaette` 0 … 1 (0,1)} · `welle` = Punktangabe
  der Leitlinie (nur `tatami`/`muster`). Leeres Objekt `{}` = Standardwerte. Fehler mit Objektnummer bei Werten außerhalb.
- `in_knoten_umwandeln` wandelt `effekte.welle` mit dem Objekt um (wird zu Bézierstücken).
- **Neue Befehle** `farben_mischen` (`{design, objekt, farbe, profil?, min?, max_unten?, max_oben?}` →
  `{design, objekte: [unten, oben], gruppe}`) und `welle_vorschlag` (`{design, objekt}` → Punktangabe für
  `effekte.welle`).
- **Oberfläche (umgesetzt):** Inspektor-Abschnitt „Effekte“ (Experte; Verlauf, Ausgefranster Rand, Welle, Knopf
  „Zweite Farbe dazu …“), Leitlinie im Knoten-Werkzeug, Dialog „Farben mischen“ (`FarbenMischenDialog`, Garn der
  oberen Lage, Profil mit Gegenstück, Abstände, Vorschau der Lagen); „Effekte“ aus „Kommt noch“ entfernt.

## 22.11.2026 – Motivfüllung und eigene Motive (Nachtauftrag N1, 4b.7; Commit ffb6249)
- **Neue Objektart `motivfuellung`** (`format.rs` `MotivfuellungJson`, Kern `motiv::motivfuellung_objekte`): Felder
  `umriss`, `loecher`, `satz` (Standard „Children“), `motiv` („901“), `groesse`, `spaltenabstand`, `reihenabstand`,
  `versatz`, `winkel`. Standardwerte aus dem Original-Stil, Abschneide-Regel aus dem Handbuch (Bericht 20 §4).
- `stichart_aendern` kennt `motivfuellung` als Füllung; `objekt_werte` liefert Größe, Spalten- und Reihenabstand.
- **Neue Befehle** `motiv_speichern` (`{design, objekte, satz, name, grundlinie?}` → `MotivsatzInfo`) und
  `motiv_loeschen` (`{satz, motiv}` → Liste der Sätze); `motive_liste` hat neu `eigen`. Ablage der eigenen Sätze:
  Datenordner `motive/` (App setzt `motive_ablage`).
- **Oberfläche (umgesetzt):** Füllung „Motive“ in der Stichart-Wahl, Inspektor (Motiv, Motivgröße, Spalten-/
  Reihenabstand, Reihenversatz, Drehung der Reihen), Anordnen → „Als Motiv speichern …“ (`MotivSpeichernDialog`),
  „Motiv wählen“ mit Gruppe „Eigene Motive“ und Löschen (×, Rückfrage im Dialog).

## 22.11.2026 – Formen mit der Maus zeichnen (Nachtauftrag N0; Commit dfc1bd4)
- **Neue Form `pfad`** (`format.rs` `Form::Pfad`, `PfadPunkt`): Referenzpunkte wie beim Digitalisieren im Original –
  `[x, y]` Eckpunkt, `{"kurve": [x, y]}` Kurvenpunkt, `geschlossen` (Ring). Ecke–Ecke = Gerade, sonst Bézier mit
  Catmull-Rom-Tangenten (eigene Festlegung). Überall erlaubt, wo eine Punktangabe steht (`punkte`, `umriss`,
  `seite_a`/`seite_b`, Löcher). `in_knoten_umwandeln` macht daraus Punktlisten und Bézierstücke (Kurvenpunkte bleiben
  glatte Knoten). Kein neuer Befehl.
- **Oberfläche (umgesetzt):** Form-Werkzeuge zeichnen statt Probeformen einzufügen (`src/lib/zeichnen.ts`, Vorschau
  Werkzeug `zeichnen`): Klick Ecke, Rechtsklick Kurve, ⌫ zurück, Enter/„Fertig“, Esc; Kreis/Oval, Rechteck (Strg =
  Quadrat), Säule paarweise. Knoten-Werkzeug: Rechtsklick auf einen Knoten = Ecke ↔ Kurve, Kurvenpunkte rund.

## 21.11.2026 – Wirksame Werte im Inspektor (Wunsch Nutzer; Commit 6f3b47f)
- **Neuer Befehl `objekt_werte`** (`{design, objekt}` → Liste `{feld, wert, text, herkunft}`, `stick_kern::werte`): für
  leere Felder der Wert, mit dem der Kern rechnet – Stoff → Design → Objekt-Einstellungen (Stichwinkel, Stichlänge,
  Faktor, Kurzstiche, Unterlage an/aus, Step-Muster; passt der Versatz zu keinem der 20 Muster, z. B. bei den Stoffen
  0,25/0,75: Text „Versatz …“), Alphabet (Buchstabenabstand = Anteil × Höhe), Füllmuster (Winkel aus dem Namen),
  Motivsatz (Größe, Abstand), Vorlagen (Blanket, Satinlinie, Applikation, Knopfloch, Wort-/Zeilenabstand). Auto
  Spacing als Spanne über die Breiten der Säule („auto 0,40–0,51“). Keine neuen Felder.
- **Oberfläche (umgesetzt):** Inspektor holt die Werte nach jeder Berechnung; leere Felder zeigen sie grau statt
  „Stil“ („–“, wenn ein Feld gerade nicht gilt), „wie Stil (an)“ bei Auswahl an/aus, Step-Muster „wie Stoff/Stil (Nr. 1)“.
  X-Marken entfernt, Erklärung als „?“ beim Darüberfahren (mit „Leer = … (Herkunft)“). Vorschau: Knoten lassen sich
  ziehen (Commit 28cd950), linke Taste verschiebt die Ansicht nicht mehr, Touchpad verschiebt, Mausrad/Pinch zoomt.

## 21.11.2026 – Monogramm-Rahmen aus dem Katalog (Wunsch Nutzer; Commit 57b5d39)
- **Neue Felder am `monogramm`:** `vorlage` (Name eines der 47 Rahmen; mit Vorlage gelten 1–3 Zeichen, Groß-/
  Kleinschreibung bleibt), `schrift` (Computer-Schrift/Google Font der Initialen; ohne Angabe der Vorschlag der
  Vorlage) bzw. `alphabet` (Stick-Alphabet), `rahmen_groesse` (längere Seite in mm, ohne Angabe wie gezeichnet,
  70–88 mm), `rahmen_farben` (Garn je Farbe der Vorlage, leer = Farben der Vorlage). Mit `vorlage` gelten `hoehe`,
  `rahmen`, `rahmen_farbe` und `umrandung` nicht. Ohne `vorlage` alles wie bisher (Diamond2/3).
- **Neuer Befehl `monogramm_rahmen`:** Kategorien und Rahmen (Name, Nr., Kategorie, Beschreibung, Schrift, Versalhöhen,
  Zahl der Initialen, Größe, Farben, SVG für die Kachel).
- **Stiche:** Initialen (Objekt-Buchstaben 1…n), danach die Rahmenteile (Buchstabe 0; Farbe der Initialen zuerst, je
  Farbe kürzester Weg). Fehlt die Schrift: Fehlermeldung wie bei `ttf`.
- **Kern, ohne neue Felder:** Variable Schriften werden in „Regular“ (wght 400) gelesen statt in der Standardlage der
  Datei (oft „Thin“); dazu Einträge „… Bold“ in `schriften_liste`. „stickbar ab“ ändert sich dadurch für viele Google
  Fonts (Montserrat 32 → 10 mm).
- **Oberfläche (umgesetzt):** Dialog „Rahmen wählen“ (`RahmenDialog.svelte`), einfacher Modus und Inspektor (Rahmen,
  Schrift mit „stickbar ab“ und Warnung, Größe, Garn je Rahmenfarbe), Dialog „Schrift wählen“ für die Initialen
  (`arbeit.monogrammSchrift`), Garn-Dialog mit `rahmen: k`.

## 21.11.2026 – Umrandung bei kleiner Schrift (Fehler gemeldet vom Nutzer; Commit 70b62e4)
- **Kern, keine neuen Felder:** Satinkante der Umrandung ist jetzt ein **Band**, das in Satinsäulen zerlegt wird
  (`saeulen::zerlegen` wie `satinflaeche`, Teilung am Knick 87°, Überlappung 0,3 mm) – keine Stichsterne mehr an engen
  Kurven und kleinen Innenräumen. Ergibt `breite_anteil` weniger als 1 mm (Schrift unter 17 mm bei 6 %), wird die
  Umrandung eine **kräftige Linie** (Dreifach, Stichlänge 12 % der Höhe, 1–2,5 mm) statt Satin; die frühere Untergrenze
  1 mm entfällt.
- **Oberfläche (umgesetzt):** Hinweis unter dem Häkchen „Umrandung“, wenn die Schrift dafür zu klein ist (Höhe ×
  Größe des Elements × Größe des Designs).

## 21.11.2026 – Dicke Umrandung im einfachen Modus (Wunsch Nutzer; Commit e714bad)
- **Neue Felder in `umrandung`:** `breite_anteil` (Breite der Satinkante in % der Buchstabenhöhe, 1–30, nur
  Text/Monogramm, gilt statt `breite`; Größe/Skalieren von Objekt, Gruppe und Design zählt mit; mindestens 1 mm) und
  `aussen` (Satinkante außen: Innenkante `einzug` mm in der Fläche, Kante wächst nach außen; ohne: Mitte der Kante
  `einzug` mm innen wie bisher). Bedeutung vorhandener Felder unverändert.
- **Kern:** Haarfugen zwischen Buchstabenteilen werden vor dem Versatz geschlossen (keine Stacheln mehr bei schmal
  gestellter Schrift); jeder Buchstabe/jede Fläche wird einzeln versetzt, außen mit runden Ecken.
- **Oberfläche (umgesetzt):** Häkchen „Umrandung“ steht bei Text und Monogramm **unter „Stabilisieren“** und setzt
  `{art: "satin", aussen: true, breite_anteil: 6}` (Flächen/Bilder: `breite: 2`). Inspektor: Umschalter „Lage“ (auf der
  Kante / außen) und bei Text Breite in „% der Höhe“ (leer = feste Breite in mm).

## 21.11.2026 – Umrandung um Flächen und Buchstaben (Wunsch Nutzer; Commit 793357f)
- **Neues Feld an jedem Objekt:** `umrandung` (`format::UmrandungJson`): `art` lauf | **dreifach** | satin, `farbe`
  (Standard schwarz), `einzug` (Standard 0,3 mm nach innen, eigene Festlegung), `breite` (Satinkante, Standard 2 mm),
  `laenge` (Lauf/Dreifach, ohne Angabe Stil). Feld vorhanden = an; `{}` = alle Standardwerte. Der Kern hängt hinter
  dem Objekt je Ring der vereinigten, nach innen versetzten Fläche eine Linie an (Text: jeder Buchstabe mit Innenräumen;
  Objektnummer je Stich = das Objekt; `buchstabe` = 0). Wirkt nur bei Objekten mit Fläche.
- **Garn von Hand** (`garn`) gilt nur noch für Farbblöcke in der Farbe des Objekts – Umrandung, einzeln gefärbte
  Buchstaben und Applikationslagen bekommen das nächste Garn der Karte (`design_berechnen`, Senden).
- **Oberfläche (umgesetzt):** einfacher Modus Häkchen „Umrandung“ + Garn-Knopf (Text, Monogramm, Element/Gruppe),
  Experte Abschnitt „Umrandung“ im Inspektor; Garn-Dialog mit `umrandung: true` setzt `umrandung.farbe`; Schritt 3
  „Garn ändern“ beim Block der Umrandung ändert die Umrandung. „Umrandung dazu“ (eigene Linie) heißt jetzt „Umrandung
  als eigene Linie“.

## 21.11.2026 – Satin-Fächer an Innenspitzen gleichmäßig (Fehler gemeldet vom Nutzer; Commit 954e683)
- **Nur Kern, keine Feldänderung:** Computer-Schrift in Satin/auto und `satinflaeche`: Fächer an der Innenseite von
  Spitzen und Kehren bleiben Fächer (keine einseitigen „Finger“ mehr innen am „A“, „V“, „W“). Original-Alphabete
  unverändert.
- **Oberfläche:** nichts zu tun.

## 21.11.2026 – Satinsäulen überlappen an Schnitten (Fehler gemeldet vom Nutzer; Commit e332611)
- **Nur Kern, keine Feldänderung:** Computer-Schrift in Satin/auto und `satinflaeche` mit `knickwinkel` > 0: Säulen
  überlappen an Teilungen und Abzweigungen 0,3 mm (Original „Create Overlaps“) – kein Spalt mehr an Spitzen.
- **Oberfläche:** nichts zu tun.

## 21.11.2026 – Computer-Schrift: überlappende Konturen, Step bei „auto“ (Fehler gemeldet vom Nutzer; Commit 59afa4f)
- **Nur Kern, keine Feldänderung:** Ringe einer Computer-Schrift werden nach der Nicht-Null-Regel vereinigt (variable
  Schriften wie Google Sans: das „A“ war bei 25 mm zerrissen, Querbalken fehlte). Schaltet `stichart: "auto"` eine
  Computer-Schrift auf Step, wird jeder Buchstabe eine Tatami-Fläche statt Step je Säulenstück.
- **Oberfläche:** nichts zu tun.

## 21.11.2026 – Unterlage für jede Schrift, Vorwahl Mittellauf + Zickzack (Wunsch Nutzer; Commit b88e5ff)
- **Geändert (Feld):** `ttf.unterlage` nimmt jetzt dieselben Werte wie `text.unterlage` (`"auto"` Standard, `"keine"`,
  Arten-Liste); `true`/`false` aus alten Dateien gelten weiter (auto/keine). Typ `UnterlageJson` hat dafür die Variante
  `boolean`. Computer-Schriften in Satin bekommen jetzt die Auto-Unterlage des Stils „Schrift“ (vorher keine).
- **Neu (Auswahlwert):** `ttf.stichart: "auto"` wie bei Texten (Satin, ab 8 mm Satinstich Step; Hinweise
  `schrift_step`/`lange_satinstiche` jetzt auch für Computer-Schriften).
- **Geändert (Befehl):** `schrift_wechseln` behält `unterlage` und `winkel` und übersetzt die Stichart (auto/Satin/Muster
  bleiben, Step ↔ Tatami, sonst auto).
- **Oberfläche:** umgesetzt – neue Texte, Computer-Schriften und Monogramme mit `unterlage: ["mittellauf", "zickzack"]`
  (neue Computer-Schriften auch `stichart: "auto"`); einfacher Modus Häkchen „Stabilisieren“ unter der Höhe; Experte zwei
  Häkchen „Mittellauf“/„Zickzack“ oben im Inspektor (Text, Computer-Schrift, Monogramm); `arbeit.schriftWechseln` setzt
  die Stichart nicht mehr selbst.

## 21.11.2026 – Stile für alle Schriften, eigene Stile (Wunsch Nutzer, Punkt 4; Commit 3bde3f4)
- **Neu:** Befehl `schrift_stile` → `{quelle, stile: [{name, beschreibung}]}` (23 Stile in Anzeigereihenfolge).
- **Neue Felder:** `alphabete_liste[].stile`, `schriften_liste[].stile` (eigene Einordnung von 50 Alphabeten und 173
  Standardschriften, sonst Google). Google-Katalog: mehr Schriften mit Stil (75 statt 650 ohne).
- **Oberfläche:** umgesetzt – Stil-Filter in allen Reitern mit Anzahl (leere ausgeblendet); Etikett an jeder Kachel
  öffnet die Leiste „Stile für …“ (setzen/entfernen, neuer Stil, eigene löschen, „Vorgabe“); Suche findet auch Stile.

## 21.11.2026 – Schrift wählen: Computer-Schriften, Google Fonts, Favoriten, „stickbar ab“ (Wunsch Nutzer; Commit ab8505d)
- **Neue Befehle:** `google_schriften_liste` (Katalog 1 833 Familien + Stil-Filter), `google_schrift_laden {familie}`
  (Internet; Datei in den Datenordner, danach offline), `schrift_vorschau {schrift, text?, hoehe?}` (Umrisse,
  fehlende Zeichen, `stickbarkeit {stickbar_ab, duenn, dick}`), `schrift_wechseln {design, objekt, alphabet|schrift}`
  (Text ↔ Computer-Schrift, gemeinsame Felder bleiben).
- **Geändert:** `schriften_liste {neu_einlesen?}` liefert jetzt auch heruntergeladene Google Fonts (`heruntergeladen`)
  und nur Schriften mit lateinischen Buchstaben; Namen älterer Apple-Schriften (MacRoman) werden gelesen; interne
  macOS-Schriften („.…“) fehlen. Das Verzeichnis wird zwischengespeichert. Die App setzt die Ablage
  (`Kontext::schriften_ablage`, Datenordner `…/schriften`).
- **Oberfläche:** umgesetzt – Dialog „Schrift wählen“ (`SchriftDialog.svelte`, ersetzt den Alphabet-Dialog) mit Reitern
  Stick-Alphabete · Auf dem Computer · Google Fonts · ★ Favoriten, Suche, Stil-Filtern, Vorschautext, gefüllten
  Buchstaben, Stern (`localStorage` „schrift_favoriten“), „stickbar ab … mm“ (orange, wenn die Höhe kleiner ist).
  Einfacher Modus: Knopf „Schrift“ auch für Computer-Schriften, Warnung unter der Höhe; Inspektor: Knopf statt
  Auswahlliste. Google-Vorschau vor dem Laden nur mit dem festen Wort „Stickerei“ (Datenschutz).

## 21.11.2026 – Schrift: Stichart „automatisch“ (Wunsch Nutzer: große Schrift nicht als lockerer Satin; Commit db84764)
- **Neu:** `stichart: "auto"` an `text` und `monogramm`: Satin, solange kein Satinstich (nach Höhe, Transformation,
  Verzerrung) länger als **8 mm** wird, sonst der ganze Text **Step** (eigene Erweiterung; Grenze wie EasyStitch,
  Handbuch S. 118). Ohne Angabe bleibt Satin wie im Original.
- **Neue Hinweisarten** in `design_berechnen.hinweise`: `schrift_step` (Info: „auto“ hat Step gewählt) und
  `lange_satinstiche` (Warnung: Satin mit Stichen über 8 mm).
- **Oberfläche:** umgesetzt – neue Texte/Monogramme mit `stichart: "auto"`; einfacher Modus unter der Höhe Info bzw.
  Warnung und Schalter „Trotzdem Satin“; Statusleiste zeigt `schrift_step` als Info (grün). Expertenmodus: Auswahl
  „auto“ im Inspektor (aus dem Schema).

## 21.11.2026 – Bild umwandeln verbessert (Wunsch Nutzer: Foto „noch nicht brauchbar“, leere Stellen; Commit 19c8b6e)
- **Neu:** Befehl `bild_vorschau` (gleiche Eingabe wie `bild_umwandeln`) → vereinfachtes Bild als PNG-data-URL und
  Farbliste `{farbe, anteil, weglassen, garn}`.
- **`bild_umwandeln` neu/geändert:** `vereinfachen` 0–5 (Standard 2), `weglassen` („#rrggbb“-Liste, ohne Angabe
  Vorschlag weißer Hintergrund, `[]` = alles sticken), `garnkarte` + `garnfarben`, `fuellung` „step“ (Standard) /
  „auto“; `farben` zählt jetzt **alle** Farben (Standard 8); kleine Flächen gehen in der Nachbarfarbe auf; Ergebnis
  hat `klassen`. Objekte: Zugausgleich 0,40 mm, Unterlage an (Handbuch S. 116). `mit_hintergrund` veraltet.
- **Oberfläche:** umgesetzt – Dialog „Bild umwandeln“ zweispaltig: links das vereinfachte Bild (weggelassene Farben
  kariert), rechts Breite, Farben, Vereinfachen, Garnfarben, Farbliste mit Haken; Experte: Stichart, kleinste Fläche,
  Glättung. Programm-Bau: Rechenkern auch im Debug-Bau optimiert (`app/src-tauri/Cargo.toml`).

## 21.11.2026 – Stichart wechseln, Werkzeugleiste nach Formen (Wunsch Nutzer, Commit 49eb027)
- **Neu:** Befehl `stichart_aendern {design, objekt, art}` → `{objekt, hinweise}`: Umriss `lauf`, `dreifach`,
  `satinlinie`, `blanket`, `motivlinie` ↔ Füllung `tatami`, `satinflaeche`, `muster`; Form und gemeinsame Felder bleiben
  (Handbuch S. 63/64 „You can change it at any stage“). Offene Linie → Füllung: Fehler.
- **Oberfläche:** umgesetzt – Werkzeugleiste mit **Formen** (Linie, Kreis/Oval, Rechteck, freie Form, Säule – wie die
  Design-Leiste des Originals, S. 58), Text/Monogramm, Applikation/Knopfloch; darüber **Füllung/Umriss** mit
  Stichart-Menü (Vorwahl für neue Formen, gilt auch für die Auswahl); Inspektor oben „Stichart“ und „Umrandung dazu“
  (eigene Erweiterung: Linie gleicher Form, gruppiert); Menü Einfügen nach Formen/Schrift/Besondere; schnelle Tooltips
  (`Tooltip.svelte`, 0,18 s); neue Symbole (Formen, Sticharten, E/A).

## 21.11.2026 – Gruppen und „Design einfügen“ (Test des Nutzers, Commit 08b469a)
- **Neu:** Feld `gruppe` an jedem Objekt (Schlüssel) und `gruppen` am Design `{name, herkunft: gruppiert|bild|svg,
  transform}` – „Group“ des Originals (Handbuch S. 134, ART `EmbroideryGroup`). Transformationen: je Objekt → je Gruppe
  (um die Mitte ihrer Objekte) → ganzes Design. Befehl `design_einfuegen {design, einfuegen, gruppe?}` wie „Insert
  Design“: hängt Objekte an, auf Wunsch als eine Gruppe; doppelte Namen → „Name (2)“, `aussparen` zieht mit.
- **Oberfläche:** umgesetzt – Klick wählt die ganze Gruppe (Objektliste: einzeln), ein Rahmen mit Griffen wirkt auf
  `gruppen[k].transform`, Pfeiltasten ebenso; Anordnen → Gruppieren/Gruppe auflösen (Strg+G / Strg+Umschalt+G);
  Bild/SVG-Dialog und „Design einfügen“ fügen als Gruppe ein (Häkchen „Design ersetzen“ für das alte Verhalten);
  einfacher Modus Schritt 2 mit **Elementen** (Liste, Hinzufügen, Bearbeiten: Name, Größe, Drehen, Mittig, Garn je
  Farbe, Löschen).

## 21.11.2026 – Motivlinie (4b.7, Commit 205e6be)
- **Neu:** Objektart `motivlinie` (`punkte`, `geschlossen`, `satz`, `motiv`, `groesse`, `abstand`, `spiegeln`) –
  Motiv aus einem Motivsatz entlang einer Linie, Anzahl/Verteilung wie das Original. Befehle `motive_liste` (14 Sätze)
  und `motivsatz_motive {satz}` (je Motiv Name, Länge, Höhe, Linien in mm für Vorschaubilder).
- **Programm:** Ressource `app/src-tauri/motive/` (14 ESB), in `tauri.conf.json` gebündelt; `STICK_MOTIVE` überschreibt.
- **Oberfläche:** umgesetzt – Einfügen → Motivlinie, Inspektor-Zeile „Motiv“ öffnet `MotivDialog` (Satz wählen,
  Kacheln), Felder Motivgröße/Spiegeln/Geschlossen oben, Abstand bei den Feinheiten. Knoten-Werkzeug wie beim Lauf.

## 21.11.2026 – Buchstabe je Stich (W9, Commit a393218)
- **Neu:** `design_berechnen` liefert `buchstabe` – je Stich die Nummer des Buchstabens bei Text/Computer-Schrift/
  Monogramm (1 = erster ohne Leerraum und `^`, wie `buchstaben[].nr`), 0 = kein Buchstabe.
- **Oberfläche:** umgesetzt – gewählten Text in der Vorschau noch einmal auf einen Buchstaben klicken → Buchstabe
  hervorgehoben, Inspektor-Abschnitt „Einzelne Buchstaben“ (Farbe, Größe, Drehen, Verschieben, Zurücksetzen; Liste
  der geänderten). Garn eines Objekts = Farbblock mit der Objektfarbe (`arbeit.garnVon`).

## 21.11.2026 – In Knoten umwandeln (W5-Rest, Commit 6d1c625)
- **Neu:** Befehl `in_knoten_umwandeln {design, objekt}` → neues Objekt-JSON: Kreis/Oval/Bogen werden
  `{"verbinden": [{"bezier": [p0, p1, p2, p3]}, …]}` (Knoten mit Anfassern wie im Original), Rechteck wird Eckpunkte,
  `transform` wird auf die Knoten angewandt und entfällt (Stichwinkel `winkel`, Satin-/Blanket-/Applikationsbreiten
  werden mitgeführt). Text, Monogramm, Knopfloch: Fehler mit `objekt`.
- **Kern:** Zwei Bézierstücke, die ohne Knick ineinander übergehen, bilden beim Lauf einen Kurvenknoten (keine Ecke) –
  umgewandelte Kreise sticken wie vorher.
- **Oberfläche:** umgesetzt – Knoten-Werkzeug zeigt Bézier-Knoten (□) und Anfasser (○), Knopf „In Knoten umwandeln“
  (`src/lib/knoten.ts`, `arbeit.inKnoten`).

## 21.11.2026 – Maschine in `stick-api` (Etappe E3, W10, Commit 3a464f6)
- **Neu:** Befehle `maschine_ports`, `maschine_pruefen`, `maschine_planen`, `maschine_senden`, `maschine_abbrechen`,
  `maschine_liste`, `maschine_lesen`, `gelesen_exp_speichern` über `kern(…)` (vorher eigene Tauri-Befehle der App).
- **Fehler** haben bei der Maschine `code` (`keine_verbindung`, `stickmodul`, `antwortet_nicht`,
  `zurueckgelesen_weicht_ab`, `farbtabelle`, `zu_gross`, `ausserhalb`, `abgebrochen`, `belegt`).
- **Ereignis** `maschine_fortschritt` jetzt mit `wiederholungen` (Typ `Fortschritt` in `kern.ts`).
- `maschine_planen` liefert Größen der Teile (`bytes_daten`, `bytes_icon`, `bytes_farben`) und die Garne an der Maschine
  (Garn von Hand berücksichtigt).
- **Oberfläche:** umgesetzt in E3 (Senden in Stufen, Abbrechen, Fehlerzustände, Gefahrendialog).

## 21.11.2026 – Wünsche der Oberfläche v3 (UEBERGABE §4, Etappe E0, Commit 0d56a68)
- **Geändert:** `hinweise` in `design_berechnen` und `design_pruefen` sind jetzt Objekte `{meldung, objekt, art}`
  (`art`: `feld` | `fehlende_zeichen` | `ausserhalb`) statt Text – Hinweis am Objekt markieren.
- **Neu im Ergebnis:** `begrenzung` [x0, y0, x1, y1] (Einpassen, Mittig), `ausserhalb_stiche` (Stichnummern zum
  Markieren), `daten_bytes` / `daten_grenze` (Datengröße gegen 145 426 Byte), `bloecke` je Farbblock (`objekt`,
  `objekte`, `von_hand`, `applikation` = Lagen `platzierung`/`schnittlinie`/`heften`/`abdecken` für die Anleitung
  „Stoff auflegen“, „zuschneiden“).
- **Neues Feld** `garn` an jedem Objekt: Garn von Hand (Nummer der Garnkarte); Oberfläche setzt dazu `farbe`.
- **Neue Befehle:** `alphabet_vorschau` (Umrisse eines Probetexts, Zeichenvorrat), `step_muster_liste` (Kacheln der
  Step-Muster 1–20). `alphabete_liste` liefert `min_hoehe`/`max_hoehe` (Handbuch; Zurich ohne Angabe).
- Wunsch 4 (Marke der Isacord-Karten) war schon erfüllt: `marke` = Marke der ersten Zeile („Isacord 40“).

## 21.11.2026 – Knopflöcher (ROADMAP 4b.6, Commit 7de0a07)
- **Neu:** Objektart `knopfloch` mit `typ` (9 Werte: `standard`, `standard_schmal`, `rund`, `dehnbar`, `heirloom`,
  `rundes_ende`, `auge_eckig`, `auge_rund`, `auge_spitz`), `position` (Mitte des Schlitzes), `laenge` (20 mm, 3–60),
  `winkel` (90° = senkrecht), `abstand` (Raupenabstand, ohne Angabe wie die Vorlage) – Felder in `SCHNITTSTELLE.md`.
- **Einarbeiten** („Merge Buttonhole“): Name des Knopflochs in `aussparen` einer Tatami-/Musterfläche.
- Braucht den Alphabet-Ordner (lädt `ButtonHoles.ESB` wie die Alphabete) – keine neuen Befehle.
- `design_formen`: Knopfloch liefert die Formen seiner Teile und den Mittelumriss als `loch`.
- **Oberfläche:** Werkzeug „Knopfloch einfügen“ (Klick = Schlitzmitte), Eigenschaften Typ (mit Vorschaubildern),
  Länge, Winkel (Griffe zum Drehen/Strecken), Raupenabstand im Expertenmodus; „In Fläche einarbeiten“ setzt
  `aussparen` an der Fläche darunter. Beschreibung: `FUNKTIONEN.md` §3.

## 21.11.2026 – Satin sticht „gerade quer + schräg vor“ (Messung, Commit ccd9c68)
- **Geändert:** nur die berechneten Stiche jeder Satinsäule (A und B einer Stelle auf derselben Stichlinie, wie im
  Original gemessen, `Analyse_Teil3/04` §5). Keine neuen Felder oder Befehle.
- **Oberfläche:** nichts zu tun; Stichvorschau und Stichzahl ändern sich minimal.

## 21.11.2026 – SVG einlesen (ROADMAP 4b.17, Commit a4ec447)
- **Neu:** Befehl `svg_lesen` – SVG-Datei (z. B. aus Inkscape) → neues Design. Eingabe: `pfad`, optional `breite`
  (mm), `rahmen`, `aussparen` (Standard an), `kuerzeste_verbindung` (Standard an). Ausgabe: `design`, `breite`,
  `hoehe`, `klassen` (Zahl der Objekte je Stichart), `hinweise`.
- **Oberfläche:** Menüpunkt „SVG öffnen/importieren“ (Dateiauswahl, Breite, Rahmen; Optionen im Expertenmodus),
  danach Hinweise anzeigen (Text nicht in Pfade umgewandelt, Verläufe, zu groß für den Rahmen) und das Design wie
  jedes andere bearbeiten. Beschreibung: `FUNKTIONEN.md` §2 „SVG einlesen“.
- Erzeugte Objekte nutzen `einstellungen` am Objekt (Werte der EasyStitch-Stile) – im Eigenschaften-Fenster als
  „geändert gegenüber Design“ sichtbar.

## 21.11.2026 – Werte je Objekt (ROADMAP 4b.16, Commit 61b37bf)
- **Neu:** Feld `einstellungen` an **jedem Objekt** (Aufbau wie `einstellungen` des Designs, ohne `stoff`).
  Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt.
- **Oberfläche:** Eigenschaften-Fenster des Objekts (Expertenmodus): Werte aus dem Parameterkatalog mit Ebene
  „beide“ (`parameter_katalog`, Feld `json_objekt` = `einstellungen.…`); Standard = Wert des Designs/Stoffs, geänderte
  Werte markieren und einzeln zurücksetzen. Typ `EinstellungenJson` in `kern.ts`.

## 20.11.2026 – Schnittstelle für die Oberfläche (ROADMAP 4b.0, Commits 829a28d … 22f207c)
- **Grundlage:** alle Befehle über **einen** Tauri-Befehl `kern` (`kern/stick-api`, Einbindung im Kopf von
  `kern/stick-api/src/lib.rs`), Aufruf `kern("name", eingabe)` aus `kern/schnittstelle/kern.ts`.
- Ergebnis von `design_berechnen`: `objekt` je Stich (Auswahl/Hervorheben), `hinweise`; `design_formen`: Umrisse,
  Linien, Satinseiten, Ein-/Ausstich je Objekt zum Bearbeiten mit der Maus; Fehler als `{meldung, objekt}`.
- Neue Befehle: `garnkarte_garne`, `fuellmuster_liste`, `schriften_liste`, `parameter_katalog`, `design_schema`,
  `design_pruefen`, `design_formen`, `reihenfolge_aendern`, `ueberdeckungen`, `saeulen_vorschau`, `bild_umwandeln`.
- Design-JSON hat `version`; Auswahlwerte stehen im Schema (Listen für Auswahlfelder).

---

**Abgeglichen bis:** `e332611` (21.11.2026) – die Oberfläche v3 (`app/`, Etappen E0–E6) setzt alle Einträge bis hier um.
Offen geblieben: W9 „Buchstabe je Stich“ (Kern) und das Umwandeln „Form/Transformation → Punkte“ für das Knoten-Werkzeug (W5, Kern); siehe `ROADMAP.md` Phase 5.
