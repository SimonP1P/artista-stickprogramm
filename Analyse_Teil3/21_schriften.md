# 21 – Schriften: Auswahl, Computer-Schriften, Google Fonts, „stickbar ab … mm“

Stand 21.11.2026. Wunsch des Nutzers (21.11.2026): mehr und modernere Schriften, Suche, Favoriten, Einordnung nach Stil
(gotisch, geschwungen, modern …). Kern `kern/stick-kern/src/stickbarkeit.rs`, `kern/stick-formate/src/google_fonts.rs`,
`kern/stick-formate/src/ttf.rs`, Befehle `kern/stick-api/src/schriften.rs`; Oberfläche
`app/src/lib/dialoge/SchriftDialog.svelte`. Katalog-Werkzeug `Arbeitsdaten/werkzeuge/google_fonts_katalog.py`.

## 1. Was das Original kann — Sicherheit: hoch (Handbuch)
- 50 eigene Alphabete (ESB, Bericht 09) mit empfohlener Mindest-/Höchsthöhe (`Analyse/daten/alphabets.md`).
- **Alle installierten TrueType-Schriften** stehen in der Alphabet-Liste und werden beim Platzieren umgewandelt
  (Handbuch S. 190–191, PDF-S. 192–193: „all of the TrueType fonts installed on your computer are available“).
- Eine Bewertung, welche Schrift sich eignet, gibt es **nicht** – nur der Hinweis „Not all TrueType fonts are suitable
  for including in artista designs. Experiment with the ones you like and try different stitch settings.“ (S. 191).
- Keine Online-Schriften, keine Stil-Einordnung, keine Favoriten.

Alles Folgende ist deshalb **eigene Erweiterung** (Grundsatz `ARCHITECTURE.md` §1a: markiert, Werte am Original geeicht,
wo möglich).

## 2. Computer-Schriften lesen — eigene Festlegung
- Ordner: System (Windows `C:\Windows\Fonts` + Benutzerschriften, Mac `/System/Library/Fonts`, `/Library/Fonts`,
  `~/Library/Fonts`) und die **Ablage** des Programms (heruntergeladene Google Fonts, Datenordner `…/schriften`).
- Aufgenommen wird jede Schrift einer Datei (`.ttf/.otf/.ttc/.otc`, Sammlungen je Schnitt) mit den Buchstaben
  **A, Z, a, z**. Weggelassen werden: Dateien über 40 MB (Emoji/CJK), interne macOS-Schriften (Name beginnt mit „.“),
  Schriften ohne lateinische Buchstaben (arabisch, Braille, Symbole) – am Mac 393 von 653 Schnitten.
- Namen: Englisch vor anderen Sprachen; Unicode-Einträge (Windows-Plattform) oder **MacRoman** (Mac-Plattform,
  Kodierung 0). Ohne MacRoman hatten 30 Apple-Schriften (Helvetica Neue, Menlo, Apple Chancery …) keinen Namen.
- Das Verzeichnis wird einmal gelesen und im Kontext gehalten (Mac ~1,5 s); `schriften_liste {neu_einlesen: true}`
  liest neu (z. B. nach dem Installieren einer Schrift).

## 3. Google Fonts — eigene Festlegung (Quellen öffentlich)
- **Katalog** im Programm (`kern/stick-formate/daten/google_fonts.json`, 1 833 Familien, 355 KB), erzeugt mit
  `google_fonts_katalog.py` aus:
  - `https://fonts.google.com/metadata/fonts` – Familie, Kategorie, Schriftsysteme, Beliebtheit (Sortierung);
  - `https://raw.githubusercontent.com/google/fonts/main/tags/all/families.csv` – Stil-Tags von Google mit Gewicht;
  - Dateiliste von github.com/google/fonts (Ordner `ofl/`, `apache/`, `ufl/`).
- Aufgenommen: Familien mit lateinischer Schrift und Ordner im Repo; je Familie **eine** aufrechte Datei („-Regular“ vor
  variabler Schrift vor erster aufrechter Datei).
- **Stile** = eigene Zuordnung der Google-Tags zu 19 deutschen Filtern (Tabelle `STILE` im Werkzeug): Gotisch
  (Blackletter, Medieval), Schreibschrift (Formal), Handschrift (Handwritten, Informal, Upright Script; Kategorie
  Handwriting ohne Script-Tag), Pinsel, Modern, Klassisch, Elegant, Verspielt, Kindlich, Vintage, Kräftig, Rund, Technik,
  Schablone, Pixel, Verziert, Weihnachten, Valentinstag, Halloween. Tags der Gruppe „Expressive“ zählen ab Gewicht 80,
  alle anderen ab 50; bei variablen Schriften gilt die Achsenlage der Standarddatei (sonst landeten Roboto und Open Sans
  über ihre fette Lage bei „Kräftig“).
- **Herunterladen** (`google_schrift_laden`): eine Datei aus `https://raw.githubusercontent.com/google/fonts/main/<pfad>`,
  höchstens 50 MB, Prüfung mit dem Schriftleser, Ablage im Datenordner des Programms (nicht im System installiert).
  Danach ohne Internet nutzbar; im Design steht die Familie als `ttf.schrift`. Lizenzen: SIL OFL, Apache 2.0, Ubuntu
  Font Licence – alle erlauben Gesticktes auch zum Verkauf.
- **Datenschutz:** An Google/GitHub gehen nur der Name der Schrift und der feste Probetext „Stickerei“ (Vorschau als
  Web-Schrift vor dem Herunterladen) – nie der Text des Anwenders.
- Fehlt eine Schrift eines Designs auf einem anderen Rechner und steht sie im Katalog, nennt die Fehlermeldung das
  Herunterladen über „Schrift wählen → Google Fonts“.

## 4. „stickbar ab … mm“ — eigene Erweiterung, am Original geeicht (Sicherheit: mittel)
**Frage:** Ab welcher Höhe sind die dünnsten Striche einer Schrift breit genug zum Sticken?

**Messung** (`stickbarkeit::strichstaerken`): Probetext „AHMOBEGRSaegmnost“, jedes Zeichen gerastert bei 300 Pixel
Bezugshöhe (TTF: Versalhöhe, ESB: 50 mm), gefüllt nach Nonzero (TTF) bzw. Gerade-Ungerade (Satinsäulen der ESB).
Abstandstransformation (Felzenszwalb), Kreismittelpunkte nur auf der Mittelachse (Pixel, deren Kreis nicht in dem eines
Nachbarn liegt); Strichdicke = 2r − 0,5 Pixel; nach Länge gewichtet (je Mittelachsenpunkt 1/Dicke). **dünn** = 10-%-,
**dick** = 90-%-Quantil, in Bruchteilen der Bezugshöhe.

**Eichung am Original:** Für 47 der 50 Alphabete gibt das Handbuch eine kleinste empfohlene Höhe an. Dünnster Strich bei
dieser Höhe (`stick stickbarkeit --kalibrieren --alphabete app/src-tauri/alphabete`):

| Alphabet | dünn | min. Höhe | Strich bei min. |
|---|---|---|---|
| Alice | 0,081 | 8 mm | 0,65 mm |
| Chicago | 0,130 | 6 mm | 0,78 mm |
| Drifter | 0,136 | 6 mm | 0,81 mm |
| Swiss Block | 0,112 | 6 mm | 0,67 mm |
| Castle | 0,082 | 10 mm | 0,82 mm |
| Olde English | 0,045 | 13 mm | 0,58 mm |
| Island | 0,098 | 8 mm | 0,79 mm |
| Victoria | 0,091 | 10 mm | 0,91 mm |
| … (vollständige Liste über den Aufruf) | | | |

**Median 0,78 mm, Viertel 0,59–0,92 mm** → Grenze `MIN_STRICH_MM = 0,8`. Ausreißer: Roman-Small 0,18 mm (Kleinbuchstaben
für 4 mm), Applikations-Alphabete (Easy Appliqué, Tabloid, Varsity Applique) 6–8 mm, weil dort der Stoff die Fläche
füllt. Zurich hat keine Angabe.

**Regel:** `stickbar_ab = aufrunden(0,8 mm / dünn)` in ganzen mm.

Beispiele Mac-Systemschriften: Impact 5 mm, Arial Black 5 mm, Arial 8 mm, Georgia 14 mm, Times New Roman 21 mm,
Apple Chancery 24 mm, Didot Italic 44 mm, Zapfino 47 mm, Snell Roundhand 61 mm; Google „Pirata One“ 10 mm.

**Anzeige:** an jeder Kachel der Computer-Schriften; liegt die Höhe des Texts darunter, orange („· jetzt 20 mm“); im
einfachen Modus am Knopf „Schrift“ und als Warnung unter der Höhe. Für Original-Alphabete gilt weiter die empfohlene
Höhe des Handbuchs.

**Offen:** Prüfung am Gerät (feine Schrift knapp über/unter der Grenze sticken, Stichart Satin und Step). Gemessen ist
nur die Form; Garnstärke, Stoff und Stichart verschieben die Grenze.

## 5. Schrift wechseln — eigene Festlegung
`schrift_wechseln {design, objekt, alphabet | schrift}`: Text mit Original-Alphabet (`text`) ↔ Computer-Schrift (`ttf`).
Es bleiben die gemeinsamen Felder (Lage, Farbe, Garn, Transformation …), die Schrift-Optionen (Höhe, Breite, Abstände,
Ausrichtung, Grundlinie …) und der Text; Felder, die es in der anderen Art nicht gibt, entfallen. Die Oberfläche setzt
für einen neuen Original-Text `stichart: "auto"` wie bei jedem neuen Text (Bericht 10 §8).

## 6. Noch offen (ROADMAP)
- Original-Alphabete und installierte Schriften nach Stil einordnen (Wunsch Nutzer, Schritt 4): Google-Einordnung für
  Schriften, die auch bei Google stehen, Liste für die Windows-Standardschriften, eigene Stile des Anwenders.
