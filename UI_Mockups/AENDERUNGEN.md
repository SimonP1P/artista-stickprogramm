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

## 21.11.2026 – In Knoten umwandeln (W5-Rest, Commit COMMIT)
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

**Abgeglichen bis:** `3a464f6` (21.11.2026) – die Oberfläche v3 (`app/`, Etappen E0–E6) setzt alle Einträge bis hier um.
Offen geblieben: W9 „Buchstabe je Stich“ (Kern) und das Umwandeln „Form/Transformation → Punkte“ für das Knoten-Werkzeug (W5, Kern); siehe `ROADMAP.md` Phase 5.
