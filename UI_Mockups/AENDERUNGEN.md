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

## 21.11.2026 – Satin sticht „gerade quer + schräg vor“ (Messung, Commit 9cf2b99)
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

**Abgeglichen bis:** – (die neue Oberfläche ist noch nicht gebaut; beim ersten Bau den dann aktuellen Commit eintragen)
