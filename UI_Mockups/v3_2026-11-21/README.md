# UI-Mockups – Stand final (21.11.2026)

Entwurf **v3** der Oberfläche für das Stickprogramm (BERNINA artista 185), überarbeitet nach dem Abgleich mit dem Rechenkern.
Grundlage: `SCHNITTSTELLE.md` (maßgeblich), `UI_Mockups/FUNKTIONEN.md`, `PARAMETER.md`, `app/ANFORDERUNGEN.md`, `UI_Mockups/README.md` – gelesen am 21.11.2026 gegen 0:30 Uhr (Branch `main`). Spätere Änderungen an diesen Dateien sind nicht berücksichtigt.

Entstanden mit Claude Design (Projekt „Sticker-Software UI-Mockups“). Die Mockups sind **nicht** die App – `app/` bleibt unverändert, bis die neue Oberfläche umgesetzt wird.

## Dateien

| Datei | Inhalt |
|---|---|
| `UEBERGABE.md` | **Übergabe für die Entwicklung**: Abgleich mit v1/v2 (fehlt / veraltet / entfernt), Bildschirme und Komponenten, Zuordnung jedes Bedienelements zu Design-Feld bzw. Kern-Befehl, Wünsche an den Kern, Design-Tokens und Icons, Antworten auf die offenen Fragen (ANFORDERUNGEN §7), Umsetzung der Sicherheitsregeln (§4). |
| `Stickprogramm v3 Zustaende.dc.html` | **Einstieg**: alle 34 Bildschirme und Zustände nebeneinander (Gruppen A Einfach · B Experte · C Maschine · D Mac/Dunkel), je mit Beschreibung und „Groß öffnen“. |
| `Stickprogramm v3 Kern.dc.html` | Der klickbare Entwurf (Fenster 1280 × 800). Zustand über `?zustand=…` in der Adresse, z. B. `Stickprogramm%20v3%20Kern.dc.html?zustand=senden-plan`. Einfach/Experte im Kopf umschaltbar. |
| `Stickprogramm v3 Vorentwurf.dc.html` | Der erste v3-Stand vor dem Abgleich mit dem Kern (nur zum Vergleich, überholt). Braucht `stich.js` und `image-slot.js`. |
| `support.js` | Laufzeit der `.dc.html`-Seiten (Claude Design). |
| `stich.js`, `image-slot.js` | Hilfsskripte des Vorentwurfs. |
| `_ds/industry-…/` | Designsystem „Industry“ (Tokens und Komponenten-CSS), von den Seiten eingebunden. |

v1 und v2 liegen unverändert in `../v1_2026-11-20/`.

## Öffnen

Den Ordner vollständig lassen. Die Seiten laden React aus dem Internet (unpkg) und die Schrift Barlow von Google Fonts – also mit Internetverbindung öffnen. Am zuverlässigsten über einen kleinen lokalen Webserver:

```
cd UI_Mockups/2026-11-21_final
python3 -m http.server 8000
```

dann im Browser `http://localhost:8000/Stickprogramm%20v3%20Zustaende.dc.html`.

## Was der Entwurf zeigt

- **Einfacher Modus** (nur E-Funktionen): Assistent mit 4 Schritten – Rahmen & Stoff · Gestalten (Text, Monogramm, Bild) · Prüfen & Garn · Senden (Mac: Speichern). „Namen auf ein Handtuch“ in wenigen Klicks.
- **Expertenmodus** (E + X): Objektliste wie die „Color Film“ (Nr, Farbe, Art, Name, Garn; Ziehen, Menü Anordnen, Warnung bei Überdeckung), Werkzeugleiste mit allen Objektarten, Stichvorschau mit Auswahl per Klick auf die Stiche, Formansicht mit Griffen und Ein-/Ausstich, Inspektor mit den Reitern Objekt · Design · Einzelwerte (Parameterkatalog, Werte je Objekt) · JSON. X-Felder tragen die Marke „X“.
- **Maschine**: Senden in zwei Stufen (Plan nur lesen → ausdrücklich bestätigen → Übertragen mit Fortschritt je Teil → Ergebnis), Sticken nur über einen eigenen Dialog („Rahmen bewegt sich“, drei Häkchen), Simulation mit violettem Streifenband, Fehlerzustände, Anleitung nach Rahmenwechsel (Layout 1 → «check»), Designs aus der Maschine.
- **Kommt noch** (FUNKTIONEN §7): sichtbar im Menü Einfügen und hinter dem Stern-Knopf der Werkzeugleiste, deaktiviert und gekennzeichnet.

Die Daten im Entwurf stammen aus dem Repo (14 Rahmen, 9 Stoffe mit Werten, 50 Alphabete mit empfohlener Höhe, echte Garne aus `Isacord 40 wt-numerical.tch` und `Default.tch`, 12 echte Füllmuster-Kacheln, Parameterkatalog, Maschinen-Designs).

**Wichtig:** Die Stichbilder zeichnet ein kleines Darstellungsskript im Entwurf (Ersatzschrift statt der Original-Alphabete, vereinfachte Tatami-/Satin-Stiche). Im Programm kommt jeder Stich aus `design_berechnen`.

## Zustände (`?zustand=`)

| Gruppe | Zustände |
|---|---|
| Einfach | `einfach-leer`, `einfach-rahmen`, `einfach-text`, `einfach-alphabet`, `einfach-hinweis`, `einfach-pruefen`, `einfach-garn`, `einfach-senden` |
| Experte | `experte-leer`, `experte-auswahl`, `experte-form`, `experte-text`, `experte-werte`, `experte-design`, `experte-muster`, `experte-step`, `experte-anordnen`, `experte-farbe`, `experte-warnung`, `experte-fehler`, `experte-abspielen`, `experte-einfuegen`, `experte-bild`, `experte-json` |
| Maschine | `senden-anschluss`, `senden-plan`, `senden-fortschritt`, `senden-fertig`, `sticken-bestaetigen`, `simulation`, `maschine-fehler`, `aus-maschine` |
| Plattform | `mac`, `dunkel` |

## Nächste Schritte

1. Wünsche an den Kern (UEBERGABE.md §4) prüfen – vor allem empfohlene Alphabet-Höhe und Vorschaubilder, Hinweise mit Objektnummer, Feld für „Garn von Hand“, Marke in den Isacord-Garnkarten.
2. Maschinen-Befehle nach `stick-api` übernehmen (mit Abbrechen und Fehlercodes).
3. Umsetzung in Svelte nach der Zuordnungstabelle (UEBERGABE.md §3).
