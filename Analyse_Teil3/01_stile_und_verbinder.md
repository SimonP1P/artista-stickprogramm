# 01 – Stile, Verbinder und Unterlagen: was das Original wirklich vorgibt (19.11.2026)

Ziel: die „eigenen Festlegungen“ des Prototyps (`stickprogramm/README.md`) durch das Verhalten des Originals ersetzen
(Grundsatz `ARCHITECTURE.md` §1a). **Quellen:** entschlüsselte Stile aus NORMAL.AMT (`Analyse/daten/entschluesselt/styles.txt`,
22 Stile), Dialogcode DIALOG.DLL (`Arbeitsdaten/disassembly/DIALOG.DLL.asm`), Dialogtexte RESENU.DLL
(`Arbeitsdaten/ressourcen/RESENU.DLL.txt`), Original-Software entpackt nach `F:\Entwicklung\artista_ziel` (Windows-PC).
Einheiten: 1 mm = 180.

## 1. Verbinder und Vernähen (Tie In / Tie Off) — Sicherheit: hoch
Dialog **„Connectors“** (RESENU DLG 1058): Gruppen *Connector Type and Values* (Jump/Run), *Trim After Object*
(Off / Always Trim / Trim If Next Connector Longer Than), *Tie In Before Object* (Off / Always Tie In / Tie In If:
After Trim / Color Change, Previous Connector Longer Than) und *Tie Off After Object* (Off / Always Tie Off / Tie Off If:
Before Trim / Color Change, Always Tie Off Last Object, Next Connector Longer Than) sowie *Values…* und *Method* (Tatami/Satin).

Die Bindung Tag ↔ Bedienelement steht im Dialogcode **DIALOG.DLL 0x10035e89–0x10036032**:
- `0x10043ed0(member, …, tag)` bindet Eingabefelder: 0x272e → Trim-Länge, **0x2754 → „Next Connector Longer Than“**,
  **0x2755 → „Previous Connector Longer Than“**.
- `0x100362a0(tagImmer, tagBedingung1, tagBedingung2, Always, If, Off)` bildet die drei Optionsfelder je Gruppe:
  Tie In (0x2753, 0x2750, 0x272d; Steuerelemente 1414/1416/1415), Tie Off (0x2752, 0x2733, 0x272c; 1423/1425/1424),
  Trim (0x2751, –, 0x272b; 1431/1433/1432).
- `0x10036240(tag, member, Steuerelement)` bindet Kontrollkästchen: 0x2750 ↔ 1419 „After Trim / Color Change“,
  0x272d ↔ 1417 „Previous Connector Longer Than“, 0x2733 ↔ 1429 „Before Trim / Color Change“,
  0x276c ↔ 1964 „Always Tie Off Last Object“, 0x272c ↔ 1426 „Next Connector Longer Than“.

| Tag | Bedeutung | Wert (alle 22 Stile gleich, Ausnahme vermerkt) |
|---|---|---|
| 0x2753 | Always Tie In | 0 |
| 0x2750 | Tie In If: After Trim / Color Change | **1** |
| 0x272d | Tie In If: Previous Connector Longer Than | **1** |
| 0x2755 | … Länge | **3,0 mm** |
| 0x2752 | Always Tie Off | 0 |
| 0x2733 | Tie Off If: Before Trim / Color Change | **1** |
| 0x272c | Tie Off If: Next Connector Longer Than | **1** |
| 0x2754 | … Länge | **3,0 mm** |
| 0x276c | Always Tie Off Last Object | **0** |
| 0x2751 | Always Trim | 0 |
| 0x272b | Trim If Next Connector Longer Than | 1 (Software Defaults, Schrift) bzw. 0 (8 Stoffe) |
| 0x272e | … Länge | 3,0 mm (Software Defaults) bzw. 6,0 mm (Stoffe) |
| 0x2730 / 0x2731 | 1,0 mm – **vermutlich Länge der Tie-In-/Tie-Off-Stiche** („Values…“, Dialog nicht in DIALOG.DLL) | mittel |
| 0x272f / 0x275d | 2 – **vermutlich Anzahl bzw. Art der Vernähstiche** | mittel |
| 0x2732 | unbekannt (Software Defaults 0, Schrift 1) | – |

**Folgerungen:** Vernäht wird am Objektanfang nach Trim/Farbwechsel oder wenn der vorige Verbinder > 3 mm ist, am
Objektende vor Trim/Farbwechsel oder wenn der nächste Verbinder > 3 mm ist. **Das letzte Objekt wird nicht vernäht.**
Die artista 185 schneidet nicht (DFL `trims_for_jump = 0`); „Trim“ zählt nur als Bedingung fürs Vernähen.
Der Designanfang wird wie „nach Trim“ behandelt (Annahme, mittel: es gibt keine Option „Tie In First Object“, wohl
aber „Always Tie Off Last Object“).

## 2. Läufe der Unterlagen — Sicherheit: hoch (Werte), hoch (Bedeutung „variabel“ über Dialog 1819)
Alle Unterlagen-Läufe haben „Variable Run Length“ = 1 (0x271e) mit eigenem Mindeststich (0x2767) und Sehnenabstand (0x2768):

| Unterlage | Lauf | Mindeststich | Sehnenabstand |
|---|---|---|---|
| Edge Walk (0x2746) | 2,2 mm | 0,728 mm (131) | 0,061 mm (11) |
| Zickzack (0x2744), Rücklauf | 3,0 mm | 0,40 mm (72) | 0,20 mm (36) |
| Doppel-Zickzack (0x2745), Lauf | 3,0 mm | 0,994 mm (179) | 0,089 mm (16) |
| Mittellauf (0x2747) | 2,2 mm | 0,40 mm (72) | 0,072 mm (13) |
| Tatami-Unterlage (0x275a), Randlauf | 2,5 mm | 0,40 mm (72) | 0,072 mm (13) |

Zum Vergleich der **Kontur-Lauf** (0x27a8 → 0x271c): 2,5 mm, **feste Länge** (0x271e = 0), Mindeststich 0,80 mm (144).

## 3. Dreifachstich — Sicherheit: hoch (Werte), mittel (0x275e)
Die Konturliste 0x27a8 enthält neben dem Lauf (0x271c) einen eigenen Eintrag **0x271f**: 2,5 mm, feste Länge,
Mindeststich 0,80 mm, **0x275e = 3** (beim einfachen Lauf 1) → 0x275e ist sehr wahrscheinlich die Zahl der Durchgänge.
Damit ist die bisherige Annahme „Dreifach = Lauflänge“ (ANALYSE.md §5, mittel) bestätigt. Dritter Eintrag: Motivlauf 0x2798.

## 4. Laufwege in Füllungen — Sicherheit: hoch (Werte)
Jede Füllart in 0x27a6 (Satin 0x2721, Tatami 0x2714, Zickzack 0x2724, 0x2727, 0x277e, Fancy 0x278c, …) hat einen
zweiten Eintrag 0x271c: **1,8 mm, variabel, Mindeststich 0,40 mm, Sehnenabstand 0,089 mm** – die Wege innerhalb einer
Fläche (bisher Sprünge im Prototyp; umzusetzen in Phase 4).

## 5. Auswahl der Unterlage — Sicherheit: mittel (Deutung), offen (Regel)
In jeder Liste eines Stils trägt **genau ein Eintrag das Flag-Bit 0x0001**: Unterlagen 0x27a7 → **Zickzack** (alle
Stoff-, Standard-, Schrift- und EasyStitch-Stile) bzw. **Tatami-Unterlage** (alle Motiv-Stile); Füllarten 0x27a6 → Satin;
Konturen 0x27a8 → Lauf. Das spricht für „ausgewählter Eintrag“ statt „ausgeschaltet“ (so bisher in
`Analyse/code/06_dataitem_und_tags.md`). Wie das Original daraus die Unterlage **je Objektart und Breite** wählt, ist
noch nicht belegt → die Auto-Unterlage des Kerns bleibt eine markierte eigene Festlegung.

## 6. Musterdesigns: echte Stiche des Originals — nächster Schritt
Die 287 ART-Musterdesigns (`Main/Samples/`, u. a. `Reference/Sampler Stitch Angles`, `Step stitch pattern 1–20`,
150 Fancy-Füllungen, Alphabete) beginnen in `Contents` und `DesignDocument` mit einem Kopf (Magic 0x193866de,
Gesamtgröße, Abschnitte mit Anzahl/Offset/Größe). Der erste Abschnitt ist ein gepackter Datenstrom mit etwa 3 Byte je
Eintrag (z. B. Flower 001: 19 723 Einträge in 59 820 Byte); Muster wie `bd 1e 35 / db 1a 45 / bd 1e 37` passen zu
Satinstichen. Dahinter folgen die Objekte als DataItems (0x2721, 0x2714, 0x271c, 0x272a …). **Wird der Stichstrom
entschlüsselt, liegen die vom Original berechneten Stiche zu bekannten Objekten vor** – der genaueste Maßstab ohne Dongle.
Werkzeug: `Arbeitsdaten/werkzeuge/art_lesen.py`.

## 7. Umgesetzt im Rust-Kern (Referenzen `kern/testdaten/referenz/`)
- Vernähen nach Abschnitt 1 (`VerbinderEinstellung`), Stichlänge 1,0 mm, 2 Stiche, letztes Objekt ohne Vernähen.
- Unterlagen-Läufe variabel mit den Grenzen aus Abschnitt 2.
Offen: Auswahl der Unterlage (5), Laufwege (4), Tatami-Versatzpaar 0x2719/0x271a, Zickzack-Deutung, Satin-Fortschritt –
am besten über die Stichdaten der Musterdesigns (6).
