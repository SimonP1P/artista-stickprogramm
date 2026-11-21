# 19 – Knopflöcher im Original (21.11.2026, ROADMAP 4b.6)

**Ergebnis:**
- Die 9 Knopfloch-Typen sind **Zeichen der Datei `wletter/ButtonHoles.ESB`** (gleiches Format wie die Alphabete,
  [09](09_esb_alphabete.md)). Jedes Teil einer Vorlage trägt einen Namen: `<L>` unteres Ende, `<U>` oberes Ende,
  `<B>` Raupe; mit Zusatz `E`/`I`/`M` ist es kein Stichteil, sondern ein Umriss (außen, innen = Schlitz, Mitte).
- **Länge, Raupenabstand, Winkel** setzt MOD1 nach festen Regeln (Code gelesen, hoch); **Standardwerte** aus dem Code:
  Typ Standard, Schlitz **20,0 mm**, Raupenabstand **0,5 mm**, Winkel **90°**.
- Im Kern nachgebaut (`stick-kern/src/knopfloch.rs`, Leser `stick-formate::esb::knopfloecher_laden`, JSON
  `"art": "knopfloch"`, CLI `stick knopfloecher`). Gegen das Musterdesign `Samples/Reference/Buttonholes.ART`
  (alle 9 Typen, 20 mm) gemessen: Einstiche im Median **0,04–0,15 mm** vom Original (EXP rundet auf 0,1 mm).
- Nebenbei gefunden: Satin sticht im Original „gerade quer + schräg vor“ ([04](04_messungen_satin.md) §5) – an den
  Raupen am deutlichsten zu sehen; im Kern für alle Satinsäulen umgesetzt (Referenz-Stufe 11).

Werkzeuge: `Arbeitsdaten/werkzeuge/art_knopfloch.py` (Lage der 9 Knopflöcher aus dem Muster → Design-JSON),
`art_knopfloch_vergleich.py` (Kern gegen Original), `stick knopfloecher ORDNER` (gelesene Vorlagen). Rohdaten:
`Arbeitsdaten/messungen/knopfloch_vorlagen.txt`, `knopfloecher_vergleich.txt`.

## 1. Typen und Standardwerte — Sicherheit: hoch (Code)
| JSON `typ` | Dialog (BERESENU) | Typnummer | Vorlage in ButtonHoles.ESB | Schlitz der Vorlage |
|---|---|---|---|---|
| `standard` | Standard | 0x3ff9 | Standard | 19,36 mm |
| `standard_schmal` | Standard Narrow | 0x3ffa | StandardNarrow | 19,61 mm |
| `rund` | Round | 0x3ffb | Round | 22,79 mm |
| `dehnbar` | Stretch | 0x4000 | Stretch | 19,58 mm |
| `heirloom` | Heirloom | 0x4001 | Heirloom | 19,42 mm |
| `rundes_ende` | Round End | 0x3ffc | RoundEnd | 21,01 mm |
| `auge_eckig` | Keyhole Square | 0x3ffd | KeyholeSquare | 22,06 mm |
| `auge_rund` | Keyhole Round | 0x3ffe | KeyholeRound | 24,13 mm |
| `auge_spitz` | Keyhole Point | 0x3fff | KeyholePoint | 23,57 mm |

- Typnummer → Vorlagenname: MOD1 0x10027b40 (Strings 0x1029f558 ff.).
- Werte `ButtonHoleValues` (MOD1 GetData/SetData 0x10028360/0x100282b0): Kennung 0x3ff4 (Satz), 0x3ff5 Typ, 0x3ff6
  Schlitzlänge (long), 0x3ff7 Raupenabstand (long), 0x3ff8 Winkel (double). `AllButtonHolesValues` 0x3ff2 (aktueller
  Typ, Winkel, Liste 0x3ff3).
- Standard (`AllButtonHolesValues`-Konstruktor 0x10028060): aktueller Typ 0x3ff9, Winkel 0x3ff921fb54442d18 = π/2;
  für alle 9 Typen Schlitz 0xe10 = 3600 = 20,0 mm, Raupenabstand 0x5a = 90 = 0,5 mm. In NORMAL.AMT steht nichts davon.
- Dialog „Buttonhole“: Buttonhole Type, Slit Length, Bead Stitch Spacing, Angle (BERESENU 0x208f2e ff.); Handbuch
  Kap. 23 (PDF-S. 210–214): Schlitz 10/15/20 mm, Abstand 0,3/0,4/1,0 mm als Beispiele. Grenzen der Felder nicht
  gelesen → Kern: Schlitz 3–60 mm, Abstand 0,2–2,0 mm (**eigene Festlegung**).

## 2. Aufbau der Vorlagen — Sicherheit: hoch (Daten), mittel (Deutung einzelner Klassen)
Zeichen-Stream wie bei den Alphabeten (Kopf 0x24, Datensätze im ART-Rahmen). Objekte:
- **ManualObject 0x0b** – Handstiche: jeder Knoten ein Einstich (Vernähen am Anfang/Ende, Riegel).
- **EmbroideryCurve 0x05** mit Lauf 0x271c – Laufweg längs der Raupe, Zickzack bei Stretch (4,0 mm, Stiche nur an den
  Knoten). Die Stichlänge ist der **letzte** Lauf hinter einer Füllliste 0x0d = 0x2748; Verweise auf Verbinder ziehen
  fremde Listen mit (2,5 mm).
- **InputCCurve 0x0c** (Basisklasse 0x05 direkt danach) mit Spine (GeoCurve) und eingebettetem **EmbroideryPatch 0x06**:
  Winkellinien (Knoten a der Schiene 1 ↔ Knoten c der Schiene 2) und **zwei Schienen** (Datensatz 0x27 = 2, zwei
  GeoCurves) → Satinsäule. Keyhole Point hat außerdem **eigenständige** 0x06-Säulen (Spitze oben).
- Stichart der Säulen: Satin 0x16 mit Abstand 0x15 = 90 (0,5 mm), ShortStitchesEffect 0x23, **0x8c = 31,0** (float,
  Einheiten) = Zugausgleich 0,17 mm je Seite (am Muster gemessen: Raupe 2,0 mm zwischen den Schienen, Stich 2,34 mm).
  Heirloom: **Blanket 0x1a** (0x2727) mit Abstand 0x15 = 270 = **1,5 mm**, Rücken an der Schlitzkante.
- Name im Datensatz **0x28** (ASCII). MOD1 `ButtonHole::ButtonHole(int, AlphaCharacter)` 0x100257b0 sortiert die
  Objekte nach Namen: `<UE>/<BE>/<LE>` → Außenumriss (+0xbc), `<UI>/<BI>/<LI>` → Innenumriss (+0x13c),
  `<UM>/<BM>/<LM>` → Mittelumriss (+0x1bc), alles andere → Stichteile (+0x54). Prüfungen 0x10026eb0/0x10026fd0/
  0x100270f0; Lage: `<B>`-Gruppe 0x10026bc0, `<U>`-Gruppe 0x10026d40, sonst unten.

## 3. Länge, Abstand, Winkel — Sicherheit: hoch (Code + Messung)
`ButtonHole::SetValues` 0x10027480:
1. Raupenabstand: an jedem `<B…>`-Teil den Abstand (Parameter 0x15, Objekt +0x58) setzen – **nur Raupen**, die Enden
   behalten den Abstand der Vorlage.
2. Winkel: weicht er um mehr als 0,01 rad ab, um die Schlitzmitte drehen.
3. Länge (0x10026680): Vorlage so drehen, dass der Schlitz längs der x-Achse liegt; Δ = neue − alte Länge; je Teil
   (0x10026970): `<B>` – umschließendes Rechteck in Schlitzrichtung um Δ/2 nach beiden Seiten aufziehen (affin),
   `<U>` – um +Δ/2 verschieben, sonst um −Δ/2; Schlitzpunkte ± Δ/2; zurückdrehen.
   Bezogen auf den unteren Schlitzpunkt: unten bleibt, oben +Δ, Raupe gestreckt (oberes Ende +Δ).
- `GetValues` 0x10027210: Länge auf 10 Einheiten gerundet, Winkel auf 3 Dezimalen (sprintf „%5.3f“).
- Einfügen (`ReadButtonHole` 0x1001f6d0): Vorlage laden, unteren Schlitzpunkt auf den Klickpunkt legen.
  Kern: `position` = **Schlitzmitte** (eigene Festlegung, für die Oberfläche handlicher).

**Messung** am Musterdesign (Datensatz 0x95 = Typ, unterer und oberer Schlitzpunkt; alle 9 mit 20,0 mm, senkrecht):
- Standard: Laufweg links von y −0,32 bis 20,27 (Vorlage 19,63 + Δ 0,64 ✓), Riegel oben bei 20,21–20,30
  (Vorlage 19,57–19,66 + Δ ✓), Riegel unten unverändert ✓, Raupenabstand 0,48–0,50 mm auf derselben Seite ✓.
- Gespeicherter Raupenabstand je Typ = der der Vorlage (Standard 0,5, Heirloom 1,5): das Muster wurde ohne geänderten
  Abstand angelegt. Kern: `abstand` ohne Angabe = Vorlage.
- Raupen beginnen an dem Ende, wo der vorige Teil endet (Standard: Laufweg endet oben → Raupe oben nach unten;
  Round: von unten); Kern wählt die Richtung jeder Raupe danach.

## 4. Vergleich Kern ↔ Original — Sicherheit: hoch
Design aus den 9 Lagen des Musters (`art_knopfloch.py`), mit dem Kern berechnet; je Knopfloch Abstand jedes Einstichs
zum nächsten Einstich des Originals (`knopfloecher_vergleich.txt`):

| Typ | Einstiche Kern / Original | Median Kern→Orig | 90 % | Median Orig→Kern | 90 % |
|---|---|---|---|---|---|
| standard | 215 / 246 | 0,10 | 0,22 | 0,09 | 0,23 |
| standard_schmal | 215 / 230 | 0,09 | 0,22 | 0,09 | 0,22 |
| rund | 253 / 300 | 0,15 | 0,25 | 0,15 | 0,27 |
| rundes_ende | 237 / 271 | 0,07 | 0,20 | 0,08 | 0,30 |
| auge_eckig | 244 / 295 | 0,08 | 0,24 | 0,09 | 0,39 |
| auge_rund | 261 / 335 | 0,09 | 0,24 | 0,11 | 0,36 |
| auge_spitz | 341 / 299 | 0,10 | 0,39 | 0,08 | 0,24 |
| dehnbar | 106 / 117 | 0,05 | 0,06 | 0,05 | 0,06 |
| heirloom | 131 / 155 | 0,04 | 0,08 | 0,05 | 0,10 |

Unterschiede: Das Original vernäht einzelne Teile (Verbinder je Teil, 1-mm-Vernähstiche am Raupenende und Laufanfang)
und stickt die Mittellauf-Unterlage der gebogenen Enden; der Kern verbindet die Teile ohne Zwischen-Vernähen zu einem
Zug (die Vorlage bringt eigene Vernähstiche am Anfang und Ende mit) und setzt nur die Unterlagen, die die Vorlage nennt.

## 5. Einarbeiten („Merge Buttonhole“) — Sicherheit: niedrig
Handbuch: „remove parts of objects that a buttonhole object overlaps … boundaries line up along buttonhole slit“.
Der Befehl selbst steht im verschlüsselten ARTISTA.EXE. `GetMidOutline` wird nur von MOD1 exportiert, DESVIEW nutzt
`GetExtOutline`/`GetIntOutline` (Zeichnen); naheliegend ist darum der **mittlere Umriss** als Loch – so endet die
Fläche unter der Raupe und wird vom Schlitz nicht zerschnitten. Kern: Name des Knopflochs in `aussparen` einer
Tatami-/Musterfläche → Loch = Mittelumriss (`ObjektArt::Kette.loch`). Prüfen, falls ARTISTA.EXE entschlüsselt wird.

## 6. Offen
1. Merge Buttonhole im Code bestätigen (§5); Zerteilen von Objekten („may result in objects being split“).
2. Bedeutung der 24 Byte der InputCCurve (zwei gepackte Punkte: Schienenanfang und ein Punkt 2 mm daneben, double 0,5).
3. Vernähen je Teil wie das Original (Verbinder 0x14 der Vorlagenteile).
4. Feldgrenzen des Dialogs (Schlitzlänge, Abstand).
