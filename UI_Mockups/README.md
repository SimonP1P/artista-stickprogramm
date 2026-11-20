# UI-Mockups (Claude Design) – Archiv

Die Mockups der neuen Oberfläche entstehen mit Claude Design (Nutzer). Hier liegen die Stände zum Nachschlagen; sie
sind **nicht** die App (`app/` bleibt unverändert, bis die neue Oberfläche feststeht).

| Stand | Ordner | Inhalt |
|---|---|---|
| v1 + v2, 20.11.2026 | `v1_2026-11-20/` | `Stickprogramm.dc.html` (v1: 6 Schritte Motiv → Senden), `Stickprogramm v2.dc.html` (v2: Gestalten, Rahmen, Stoff & Reihenfolge, Vorschau, Senden; Einfach/Erweitert), `Vergleich Designsysteme.dc.html`; Bildschirmfotos in `screenshots/` |

Öffnen: die `.dc.html` im Browser (Ordner muss vollständig sein, `support.js` und `_ds/` werden nachgeladen).

## Unterlagen für Designer und Frontend-Entwickler (werden bei jeder neuen Funktion aktualisiert)
1. **`UI_Mockups/FUNKTIONEN.md`** – jede Funktion des Kerns: was der Anwender einstellt, Standardwert, JSON-Feld,
   Vorschlag einfach/experte; fehlende App-Befehle; was noch kommt.
2. `UI_Mockups/README.md` (diese Datei) – Hinweise aus der Analyse zu Stoffen, Sticharten, Reihenfolge, Rahmen,
   Expertenmodus.
3. `Analyse_Teil3/15_funktionsinventur.md` – alle Funktionen des Originals und was der Kern schon kann.
4. `ROADMAP.md`, Abschnitt Phase 4b – was noch kommt.
5. `stickprogramm/README.md`, Abschnitt „Design beschreiben (JSON)“ – alle Objekte und Felder im Detail.
6. Auswahllisten: `Analyse/daten/hoops_table.md` (Rahmen), `Analyse/daten/alphabets.md` + `Arbeitsdaten/alphabete/`
   (Schriften), `Analyse/daten/garnkarten/UEBERSICHT.md` (Garnkarten), `kern/stick-kern/daten/fuellmuster.json`
   (242 Füllmuster).
7. Frontend: `app/src-tauri/src/lib.rs` – die Befehle zwischen Oberfläche und Kern.

## Hinweise aus der Analyse für die nächsten Stände (Claude Code, 20.11.2026)
Fakten aus dem Original und dem Rechenkern, die die Gestaltung betreffen (Quellen in Klammern):
1. **Stoffe:** Das Original kennt 8 Stoffe plus „Software Defaults“: Knit Light/Medium/Heavy, Terry Medium/Heavy,
   Woven Light/Medium/Heavy (`kern/stick-kern/src/einstellungen.rs`, `Analyse/code/10` §E). Sie ändern Dichte,
   Unterlage und Zugausgleich. Die Kacheln im Mockup (Baumwolle, Jersey, Fleece, Leder …) müssen auf diese 8 abgebildet
   werden; Vlies-Empfehlungen stammen nicht aus dem Original (als eigene Hinweise kennzeichnen oder belegen).
2. **Sticharten:** „Füllstich“ und „Stepp“ sind im Original dasselbe (Tatami = Step-Füllung). Wirklich verschieden sind:
   Satin, Tatami, Füllmuster (242 Muster), Lauf/Dreifach (Linien), Blanket, Applikation. Im einfachen Modus sollte
   „Automatisch“ der Standard sein – das Original wählt nach der Breite in mm (EasyStitch: Fläche ≤ 8 mm Satin,
   ≤ 12 mm Tatami 1, darüber Tatami 2).
3. **Farbreihenfolge:** Umstellen ändert, was oben liegt (Überlappungen, Aussparungen). Zusammenlegen gleicher Farben
   geht nur gefahrlos, wenn sich die Objekte dazwischen nicht überdecken – die Oberfläche braucht dafür eine Prüfung
   aus dem Kern. Passt inhaltlich eher zu Vorschau/Reihenfolge als zu Stoff.
4. **Größe zuerst in mm, Stiche zuletzt:** Formen (auch mit Löchern) sind maßstabsfrei; Stiche werden bei jeder
   Änderung neu berechnet, nie skaliert. Entscheidungen wie Satin/Tatami, Satinabstand und Wegfall kleiner Details
   hängen aber von der endgültigen Größe ab → erst beim Berechnen treffen.
5. **Rahmen:** Der Rahmen ist ein festes Teil – er skaliert nicht mit, das Design wird relativ zu ihm skaliert.
   Welche Rahmen die artista 185 tatsächlich annimmt, ist noch zu prüfen (Liste der Software: `Analyse/daten/hoops_table.md`).
6. **Stickzeit:** Für eine Zeitangabe fehlt bisher eine Quelle (Stiche pro Minute der artista 185) – als Schätzung
   kennzeichnen.
7. **Expertenmodus:** Die einstellbaren Parameter kommen aus einem Parameterkatalog des Kerns (ROADMAP „Merkliste“),
   gruppiert nach Stichart/Bereich, je Parameter Einheit, Standard, Bereich, Quelle und Modus (einfach/experte).
