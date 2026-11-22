# 28 – Vorlagen (Templates) und Werte: Standard, aktuell, je Objekt

Stand 22.11.2026 (Nachtauftrag N8, ROADMAP 4b.14). Handbuch Kap. 9 „Object Properties and Templates“ (gedruckte
S. 64–70; Text `Arbeitsdaten/texte/man_raw_pg.txt` ab Zeile 4463). Kern: `stick-api/src/vorlagen.rs`.

## 1. Original — Sicherheit: hoch (Handbuch)
**Drei Wertesätze:**
- „BERNINA artista stores three sets of object properties: default, current and existing.“
  - **default** = Werte der Vorlage. Sie sind die Startwerte jedes neuen Designs aus dieser Vorlage.
  - **current** = Werte für neue Objekte im Design. „Unless you change these settings, they remain the same as the
    template defaults.“
  - **existing** = Werte, die jedes Objekt selbst trägt. Das bestätigen auch die ART-Dateien: jedes Objekt hat eine
    eigene Kopie des Stil-Baums (`Analyse_Teil3/03` §4).
- Übergänge:
  - „You can make the properties of a selected object into new current properties or new defaults.“
  - „You can also apply current properties to existing objects.“
  - „Save“ im Dialog Object Properties speichert die Werte in die Vorlage, aus der das Design entstand: „Only the
    current object properties — not the objects or other settings in the design — are saved to the template.“

**Vorlagen (AMT):**
- „Templates are special files used to store styles and default settings.“ Eine Vorlage kann auch Objekte und
  Mustertext enthalten („a template may include standard objects and sample lettering“).
- Speichern mit „Save As … Design Templates (AMT)“ im Ordner `artista\template`.
- „New“ zeigt die Liste der Vorlagen. „Each time you create a new design from a template, BERNINA artista opens a
  duplicate“: die Vorlage selbst wird nie versehentlich überschrieben.
- **NORMAL** ist die Standardvorlage. „Revert“ stellt die Werkskopie wieder her.

**Stile:** Die Wilcom-Dialoge „Organize Styles“, „New Object Style“ (Style Name, Based On Style), „Use Object Style“
gibt es in `RESENU.DLL` (Zeilen 1315 ff., 3749 ff.). In `BERESENU.DLL` und im artista-Handbuch kommen sie nicht vor.
Die artista-Oberfläche bietet Stile also nicht als eigene Funktion an. Die 22 Stile von NORMAL.AMT
(`Analyse/daten/entschluesselt/styles.txt`) sind die Werte, die je Stichart und Stoff gelten. Im Kern sind sie
Standard → Stoff → `einstellungen` (Parameterkatalog).

## 2. Umsetzung
| Original | Programm |
|---|---|
| default (NORMAL.AMT bzw. Vorlage) | Vorlage `<Datenordner>/vorlagen/<Name>.json` (Design-JSON); ohne Datei: Werkseinstellung = Standardwerte des Kerns |
| current | `einstellungen` des Designs |
| existing | `einstellungen` am Objekt (und die Felder des Objekts) |
| New (mit Vorlage NORMAL) | Datei → „Neu“ = Kopie der Vorlage **Normal**; Rahmen und Stoff bleiben wie im bisherigen Design (eigene Festlegung, wie bisher bei „Neu“) |
| New … (Vorlagenliste) | Datei → „Neu aus Vorlage …“: Werte, Stoff, Rahmen, Garnkarte und Objekte der Vorlage |
| Save As … Templates (AMT) | Datei → „Als Vorlage speichern …“ (`umfang: "ganz"`, Objekte auf Wunsch) |
| Object Properties → Save | Einzelwerte, Ebene Design → „In Vorlage speichern …“ (`umfang: "werte"`; Objekte und Rest der Vorlage bleiben) |
| make selected object's properties current | Einzelwerte, Ebene Objekt → „Als Werte des Designs übernehmen“ (Objektwerte → Design, am Objekt gelöscht) |
| apply current properties to existing objects | „Designwerte auf Auswahl anwenden“ (eigene Werte der gewählten Objekte löschen) |
| Revert (Templates) | „Neu aus Vorlage …“ → bei Normal „Werkseinstellung“ (Datei löschen) |

Eigene Festlegungen:
- JSON statt AMT. Das Design merkt sich seine Vorlage im Feld `vorlage`: es ist Vorschlag beim Speichern der Werte,
  die Werte selbst stehen in `einstellungen`.
- Eine Vorlage wird nur gespeichert, wenn sie sich als Design lesen lässt.
- Gleicher Name ersetzt eine Vorlage; die Oberfläche warnt vorher.
- Stile als eigene Funktion entfallen, weil die artista sie nicht anbietet (§1). Ihre Wirkung, Werte für eine Stoffart
  oder Designart festzuhalten, übernehmen die Vorlagen.

## 3. Offen
- Hintergrundfarbe in der Vorlage (Handbuch: „Background color can be saved in the template“): das Programm hat noch
  keine Hintergrundfarbe.
