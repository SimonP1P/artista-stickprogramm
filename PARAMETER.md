# Parameterkatalog

Stand 22.11.2026 · **256 Parameter**: 82 einfach, 113 experte, 61 automatisch. Erzeugt mit `stick parameter --markdown` aus `kern/stick-kern/daten/parameter.json` (Quelle: `Arbeitsdaten/werkzeuge/parameter_katalog.py`) – **nicht von Hand ändern**. Die Tests im Kern prüfen, dass jeder Wert unter `einstellungen` hier steht und die Standards stimmen.

**Modus:** *einfach* = im einfachen Modus sichtbar · *experte* = nur im Expertenmodus · *automatisch* = nicht anzeigen (fester oder abgeleiteter Wert, nur dokumentiert).  
**Ebene:** *design* = gilt für das ganze Design · *objekt* = am einzelnen Objekt · *beide* = Design-Standard, am Objekt überschreibbar · *werkzeug* = Bild → Umrisse.  
**JSON:** Pfad im Design-JSON bzw. `Objekt: feld` am Objekt. **Standard** = „0 Software Defaults“; *Stoff* = ändert sich mit dem Stoff (Tabelle am Ende). Bereiche mit „Vorschlag“ in der Quelle sind eigene Festlegungen.

## Design

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Rahmen** | Stickrahmen; legt das Stickfeld fest |  | `rahmen` | artista 130 x 100 |  |  | design | einfach | Rahmenliste der Software (Analyse/daten/hoops_table.md); Befehl rahmen_liste |
| **Eigener Rahmen: Breite** | Stickfeld eines eigenen Rahmens (Rechteck/Oval) |  | `eigener_rahmen.breite` | – | 10 … 1000 |  | design | einfach | Handbuch Kap. 6 S. 49–51 (Creating custom hoops); Bereich eigene Festlegung |
| **Eigener Rahmen: Höhe** | Oval: Gesamthöhe („Total Height“) |  | `eigener_rahmen.hoehe` | – | 10 … 1000 |  | design | einfach | Handbuch Kap. 6 S. 49–51; Bereich eigene Festlegung |
| **Eigener Rahmen: gerader Teil** | Oval: Höhe des geraden Mittelteils („Rectangle Height“), 0 = Ellipse |  | `eigener_rahmen.gerade` | 0 mm |  |  | design | einfach | Handbuch Kap. 6 S. 50 |
| **Stoff** | ändert Satin-/Step-Abstand, Zugausgleich, Unterlagen, Step-Versatz, Trim |  | `stoff` | 0 Software Defaults | 0 Software Defaults, Knit Light Weight, Knit Medium Weight, Knit Heavy Weight, Terry Medium Weight, Terry Heavy Weight, Woven Light Weight, Woven Medium Weight, Woven Heavy Weight |  | design | einfach | Stile in NORMAL.AMT (Analyse/code/10 §E) |
| **Garnkarte** | Farben werden dem nächsten Garn der Karte zugeordnet |  | `garnkarte` | Default |  |  | design | einfach | Garnkarten .tch; die Maschine nutzt Default.tch; Befehl garnkarten_liste |
| **Garn von Hand** | Garn der Karte für dieses Objekt (statt nächstes Garn zur Farbe) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `garn` | – |  |  | objekt | einfach | Feldform eigene Festlegung; Garnauswahl aus der Karte wie im Original (Handbuch Kap. 10) |
| **Startpunkt** | wo der erste Stich beginnt |  | `start` | rahmenmitte | rahmenmitte, designmitte, unten_mitte, erster_stich |  | design | experte | Dialog „Start and End“ (Handbuch PDF-S. 205); Standard bis Gerätetest V7 eigene Festlegung |
| **Endpunkt** | wohin der Rahmen nach dem letzten Stich fährt |  | `ende` | rahmenmitte | rahmenmitte, designmitte, unten_mitte, start, letzter_stich |  | design | experte | Dialog „Start and End“; 34 von 51 Maschinendesigns enden auf 0,0 |

## Größe und Lage

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Größe** | skaliert um die Mitte (eine Zahl oder [x, y]) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `transform.skalieren` · Objekt: `transform.skalieren` | 100 % | 10 … 500 |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert; Bereich Vorschlag (eigene Festlegung) |
| **Drehen** | dreht um die Mitte, gegen den Uhrzeiger | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `transform.drehen` · Objekt: `transform.drehen` | 0 ° | -360 … 360 |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |
| **Spiegeln** | spiegelt um die Mitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `transform.spiegeln` · Objekt: `transform.spiegeln` | – | links_rechts, oben_unten, beide |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |
| **Schrägstellen** | verschiebt x mit der Höhe (Skew) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `transform.schraeg` · Objekt: `transform.schraeg` | 0 ° | -60 … 60 |  | beide | experte | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert; Bereich Vorschlag (eigene Festlegung) |
| **Verschieben** | Lage im Rahmen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `transform.verschieben` · Objekt: `transform.verschieben` | [0; 0] mm [x, y] |  |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |

## Satin

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Auto Spacing** | Stichabstand automatisch nach der Säulenbreite (schmal = lockerer) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.auto_spacing` · Objekt: `einstellungen.satin.auto_spacing` | an |  |  | beide | experte | Effekt Auto Spacing 0x21, Tabelle nach Breite (Analyse/code/10, hoch); am Objekt aus, wenn „abstand“ gesetzt |
| **Stichabstand (fest)** | Weg zwischen zwei Einstichen derselben Seite, wenn Auto Spacing aus | satin | `einstellungen.satin.abstand_fest` · Objekt: `abstand` | 0,4 mm | 0,2 … 2 | ja | beide | experte | AMT 0x2723 (hoch); Bereich Vorschlag (eigene Festlegung) |
| **Auto-Spacing-Faktor** | Faktor auf die Tabelle (Stoff, Schrift 0,9) | satin | `einstellungen.satin.faktor` · Objekt: `faktor` | 1 Faktor | 0,5 … 2 | ja | beide | experte | AMT 0x2736 (hoch) |
| **Zugausgleich** | verlängert jeden Stich an beiden Enden („Fett“ = mehr Zugausgleich) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.zugausgleich` · Objekt: `einstellungen.satin.zugausgleich` | 0,2 mm | 0 … 1 | ja | beide | experte | Handbuch S. 199 (hoch); Stile 0x273d |
| **Zickzack-Abstand** | Abstand des Zickzack-Deckstichs (Stichart zickzack) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.zickzack_abstand` · Objekt: `einstellungen.satin.zickzack_abstand` | 2 mm | 0,5 … 6 | ja | beide | experte | Stichart Zigzag 0x2724/0x2725 (Stil-Liste 0x27a6, hoch) |
| **Kurzstiche** | kürzt Stiche an Innenkurven, damit es dort nicht zu dicht wird | satin, tatami (mit Winkellinien) | `einstellungen.satin.kurzstiche.an` · Objekt: `kurzstiche` | aus |  |  | beide | experte | Effekt 0x273a (MOD1 ShortStitchesEffect); Stil aus, Schrift an |
| **Kurzstiche: Schwelle** | ab welcher Dichte gekürzt wird (× Abstand) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.kurzstiche.schwelle` · Objekt: `einstellungen.satin.kurzstiche.schwelle` | 0,5 Faktor |  |  | beide | automatisch | 0x2819 (hoch) |
| **Kurzstiche: Zufall** | Kürzung zufällig ±0,04 verändert | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.kurzstiche.zufall` · Objekt: `einstellungen.satin.kurzstiche.zufall` | an |  |  | beide | automatisch | 0x281a (hoch) |
| **Kurzstiche: Gruppe** | höchstens so viele Stiche je Gruppe | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.kurzstiche.max_gruppe` · Objekt: `einstellungen.satin.kurzstiche.max_gruppe` | 5 Anzahl |  |  | beide | automatisch | 0x281b (hoch) |
| **Kurzstiche: Faktoren** | Kürzungsfaktoren als Dreieckstabelle | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.satin.kurzstiche.faktoren` · Objekt: `einstellungen.satin.kurzstiche.faktoren` | [0,8; 0,85; 0,72; 0,7; 0,9; 0,7; 0,7; 0,9; 0,8; 0,7; 0,7; 0,87; 0,65; 0,83; 0,7] Liste |  |  | beide | automatisch | 0x281c (hoch) |
| **Stichart Säule/Linie** | Satin (dicht) oder Zickzack (offen) | satin, satinlinie | Objekt: `stichart` | satin | satin, zickzack |  | objekt | experte | Stichartliste STR 2234/2236 |
| **Breite der Satinlinie** | Breite der Bahn um die Linie | satinlinie | Objekt: `breite` | 2 mm | 0,5 … 12 |  | objekt | einfach | Standard Vorschlag (eigene Festlegung) |
| **Lage der Satinlinie** | mittig um die Linie oder zur Seite (Linie = Kante) | satinlinie | Objekt: `versatz` | mitte | mitte, seite |  | objekt | experte | Dialog „Outline Stitch“ Center/Offset; Seite links Vorschlag (eigene Festlegung) |

## Tatami (Step)

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Stichabstand** | Weg zwischen zwei Hinreihen (Nachbarreihen halber Abstand) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.abstand` · Objekt: `einstellungen.tatami.abstand` | 0,4 mm | 0,2 … 2 | ja | beide | experte | AMT 0x2716 (hoch; Handbuch PDF-S. 74) |
| **Stichlänge** | Länge der Step-Stiche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.laenge` · Objekt: `einstellungen.tatami.laenge` | 4 mm | 1 … 12,7 |  | beide | experte | AMT 0x2717 (hoch) |
| **Mindeststich am Rand** | Einstiche näher am Rand entfallen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.min_rand` · Objekt: `einstellungen.tatami.min_rand` | 0,4 mm | 0,1 … 2 |  | beide | experte | AMT 0x2718 (hoch) |
| **Reihenversatz 1** | Verschiebung des Rasters (über Step-Muster gewählt) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.versatz` · Objekt: `einstellungen.tatami.versatz` | 0,25 × Stichlänge |  |  | beide | automatisch | AMT 0x2719 (hoch) |
| **Reihenversatz 2** | zweiter Versatz, abwechselnd mit dem ersten (null = wie Versatz 1) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.versatz2` · Objekt: `einstellungen.tatami.versatz2` | – |  | ja | beide | automatisch | AMT 0x271a (hoch); Stoffe 0,75 |
| **Zufallsversatz** | streut die Einstiche einer Reihe | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.zufall` · Objekt: `einstellungen.tatami.zufall` | 0 × Stichlänge |  |  | beide | automatisch | AMT 0x271b; Verteilung Vorschlag (eigene Festlegung) |
| **Stichwinkel** | Richtung der Stichreihen | tatami, muster, ttf | `einstellungen.tatami.winkel` · Objekt: `winkel` | 45 ° | 0 … 180 |  | beide | einfach | Handbuch Kap. 16 (Stitch Angle); Standard 45° Vorschlag (eigene Festlegung) |
| **Zugausgleich** | verlängert die Reihen an beiden Enden | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.zugausgleich` · Objekt: `einstellungen.tatami.zugausgleich` | 0,2 mm | 0 … 1 | ja | beide | experte | Handbuch S. 199 (hoch) |
| **Laufweg in der Fläche** | Stichlänge der Wege zwischen Teilbereichen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.laufweg_laenge` · Objekt: `einstellungen.tatami.laufweg_laenge` | 1,8 mm |  |  | beide | automatisch | Lauf 0x271c (hoch) |
| **Laufweg: Mindeststich** | variabler Lauf: kleinster Stich | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.laufweg.min_laenge` · Objekt: `einstellungen.tatami.laufweg.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **Laufweg: Sehnenabstand** | variabler Lauf: Abweichung von der Kurve | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.tatami.laufweg.sehnenabstand` · Objekt: `einstellungen.tatami.laufweg.sehnenabstand` | 0,089 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **Step-Muster** | Struktur der Fläche (1 = gleichmäßig) | tatami, text (Step) | Objekt: `step_muster` | 1 Nummer | 1 … 20 |  | objekt | einfach | Dialog „Fill Stitch“, Tabelle DIALOG.DLL (hoch) |
| **Mehrere Stichwinkel** | Winkellinien quer über die Fläche, die Stiche drehen mit | tatami | Objekt: `winkellinien` | [] Linien |  |  | objekt | experte | Handbuch Kap. 16; gemessen (Analyse_Teil3/06 §6) |
| **Unterlage der Fläche** | Step-Unterlage an/aus (ohne Angabe: Stil) | tatami, muster | Objekt: `unterlage` | – |  |  | objekt | experte | Auto Underlay des Stils |

## Füllmuster

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Füllmuster** | eines der 242 Muster (Program Split) | muster, text | Objekt: `muster` | – |  |  | objekt | einfach | Bibliothek in NORMAL.AMT (Analyse_Teil3/13); Liste fuellmuster.json |
| **Mustergröße** | skaliert die Musterkachel | muster, text | Objekt: `groesse` | 100 % | 25 … 400 |  | objekt | experte | Analyse_Teil3/13; Bereich Vorschlag (eigene Festlegung) |

## Lauf

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Stichlänge** | Länge der Laufstiche | lauf, dreifach, ttf | `einstellungen.lauf.laenge` · Objekt: `laenge` | 2,5 mm | 0,5 … 12,7 |  | beide | experte | AMT 0x27a8 (hoch) |
| **Mindeststich** | kleinster Stich (variable Länge) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.lauf.min_laenge` · Objekt: `einstellungen.lauf.min_laenge` | 0,8 mm |  |  | beide | automatisch | AMT 0x27a8 (hoch) |
| **Variable Stichlänge** | in Kurven kürzere Stiche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.lauf.variabel` · Objekt: `einstellungen.lauf.variabel` | aus |  |  | beide | experte | Run-Dialog: feste Länge Standard (hoch) |
| **Sehnenabstand** | erlaubte Abweichung von der Kurve (variable Länge) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.lauf.sehnenabstand` · Objekt: `einstellungen.lauf.sehnenabstand` | 0,09 mm |  |  | beide | automatisch | Chord Gap (hoch) |
| **Eckwinkel** | ab diesem Knick Einstich an der Ecke (nur berechnete Linien) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.lauf.ecke_grad` · Objekt: `einstellungen.lauf.ecke_grad` | 30 ° |  |  | beide | automatisch | Vorschlag (eigene Festlegung) |

## Unterlage

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Unterlage automatisch** | Auto Underlay an/aus (Schrift: an) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.auto_an` · Objekt: `einstellungen.unterlage.auto_an` | aus |  |  | beide | einfach | Stil-Flags 0x2000 (Analyse_Teil3/05, mittel-hoch) |
| **Art der Auto-Unterlage** | vorgewählte Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.auto_art` · Objekt: `einstellungen.unterlage.auto_art` | [zickzack] | mittellauf, edge_walk, zickzack, doppelzickzack |  | beide | experte | Stil-Bit 0x0001 = Zickzack |
| **Unterlage der Säule/Schrift** | auto, keine oder Arten (z. B. edge_walk+zickzack). Schrift (auch Computer-Schrift, Monogramm): auto = Stil „Schrift“ (Zickzack); die Oberfläche wählt für neue Texte Mittellauf + Zickzack vor (Entscheidung Nutzer 21.11.2026); einfacher Modus ein Häkchen „Stabilisieren“ (an = beide, aus = keine), Experte zwei Häkchen | satin, satinlinie, text, ttf, monogramm | Objekt: `unterlage` | auto | auto, keine, mittellauf, edge_walk, zickzack, doppelzickzack |  | objekt | einfach | Handbuch Kap. 22, S. 186 (kleine Schrift: aus); Stil Schrift 0x27a7 (Analyse_Teil3/05) |
| **Abstand zum Rand** | Unterlage bleibt so weit innen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.rand` · Objekt: `einstellungen.unterlage.rand` | 0,2 mm | 0 … 2 |  | beide | experte | AMT 0x2769/0x276a (mittel) |
| **Zugausgleich der Unterlage** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.zugausgleich` · Objekt: `einstellungen.unterlage.zugausgleich` | 0,2 mm |  |  | beide | experte | AMT 0x2743 (hoch) |
| **Edge Walk: Stichlänge** | Lauf am Rand der Säule | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.edge_walk_laenge` · Objekt: `einstellungen.unterlage.edge_walk_laenge` | 2,2 mm |  |  | beide | experte | AMT 0x2747 (hoch) |
| **Mittellauf: Stichlänge** | Lauf auf der Säulenmitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.mittellauf_laenge` · Objekt: `einstellungen.unterlage.mittellauf_laenge` | 2,2 mm |  |  | beide | experte | AMT 0x2746 (hoch) |
| **Zickzack-Unterlage: Abstand** | Weg zwischen Einstichen derselben Seite | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.zickzack_laenge` · Objekt: `einstellungen.unterlage.zickzack_laenge` | 5 mm |  | ja | beide | experte | AMT 0x2744/0x2725 (mittel) |
| **Doppel-Zickzack: Abstand** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.doppelzickzack_laenge` · Objekt: `einstellungen.unterlage.doppelzickzack_laenge` | 7 mm |  | ja | beide | experte | AMT 0x2745/0x2725 (mittel) |
| **Zickzack: Rücklauf** | Lauf zurück zur Säulenmitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.rueck_lauf_laenge` · Objekt: `einstellungen.unterlage.rueck_lauf_laenge` | 3 mm |  |  | beide | automatisch | AMT 0x2744/0x271d (mittel) |
| **Step-Unterlage: Abstand** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_abstand` · Objekt: `einstellungen.unterlage.tatami_abstand` | 3 mm |  |  | beide | experte | AMT 0x275a (mittel) |
| **Step-Unterlage: Stichlänge** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_laenge` · Objekt: `einstellungen.unterlage.tatami_laenge` | 4 mm |  |  | beide | experte | AMT 0x275a (mittel) |
| **Step-Unterlage: Randlauf** | Stichlänge des Laufs um die Fläche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_randlauf` · Objekt: `einstellungen.unterlage.tatami_randlauf` | 2,5 mm |  |  | beide | automatisch | AMT 0x275a (mittel) |
| **Step-Unterlage: Winkel** | gegen die Deckrichtung gedreht | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_winkel_versatz` · Objekt: `einstellungen.unterlage.tatami_winkel_versatz` | 90 ° |  |  | beide | automatisch | AMT 0x2759 |
| **edge_walk_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.edge_walk_lauf.min_laenge` · Objekt: `einstellungen.unterlage.edge_walk_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **edge_walk_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.edge_walk_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.edge_walk_lauf.sehnenabstand` | 0,072 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **mittellauf_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.mittellauf_lauf.min_laenge` · Objekt: `einstellungen.unterlage.mittellauf_lauf.min_laenge` | 0,728 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **mittellauf_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.mittellauf_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.mittellauf_lauf.sehnenabstand` | 0,061 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **zickzack_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.zickzack_lauf.min_laenge` · Objekt: `einstellungen.unterlage.zickzack_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **zickzack_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.zickzack_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.zickzack_lauf.sehnenabstand` | 0,2 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **doppelzickzack_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.doppelzickzack_lauf.min_laenge` · Objekt: `einstellungen.unterlage.doppelzickzack_lauf.min_laenge` | 0,994 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **doppelzickzack_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.doppelzickzack_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.doppelzickzack_lauf.sehnenabstand` | 0,089 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **tatami_randlauf_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_randlauf_lauf.min_laenge` · Objekt: `einstellungen.unterlage.tatami_randlauf_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **tatami_randlauf_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.unterlage.tatami_randlauf_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.tatami_randlauf_lauf.sehnenabstand` | 0,072 mm |  |  | beide | automatisch | 0x2768 (hoch) |

## Verbinder und Vernähen

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Immer vernähen (Anfang)** | Tie In immer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.anfang_immer` · Objekt: `einstellungen.verbinder.anfang_immer` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2753 |
| **Vernähen nach Trim/Farbwechsel** | Tie In If After Trim | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.anfang_nach_trim_farbwechsel` · Objekt: `einstellungen.verbinder.anfang_nach_trim_farbwechsel` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2750 |
| **Vernähen nach langem Verbinder** | Tie In If Previous Connector Longer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.anfang_bei_langem_verbinder` · Objekt: `einstellungen.verbinder.anfang_bei_langem_verbinder` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272d |
| **… Verbinder länger als** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.anfang_verbinder_ab` · Objekt: `einstellungen.verbinder.anfang_verbinder_ab` | 3 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2755 |
| **Immer vernähen (Ende)** | Tie Off immer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.ende_immer` · Objekt: `einstellungen.verbinder.ende_immer` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2752 |
| **Vernähen vor Trim/Farbwechsel** | Tie Off If Before Trim | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.ende_vor_trim_farbwechsel` · Objekt: `einstellungen.verbinder.ende_vor_trim_farbwechsel` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2733 |
| **Vernähen vor langem Verbinder** | Tie Off If Next Connector Longer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.ende_bei_langem_verbinder` · Objekt: `einstellungen.verbinder.ende_bei_langem_verbinder` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272c |
| **… Verbinder länger als** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.ende_verbinder_ab` · Objekt: `einstellungen.verbinder.ende_verbinder_ab` | 3 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2754 |
| **Letztes Objekt vernähen** | Always Tie Off Last Object | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.letztes_objekt_vernaehen` · Objekt: `einstellungen.verbinder.letztes_objekt_vernaehen` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x276c |
| **Trim ab** | ab dieser Verbinderlänge gilt ein Sprung als Trim (die 185 schneidet nicht) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.trim_ab` · Objekt: `einstellungen.verbinder.trim_ab` | 3 mm |  | ja | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272e |
| **Länge der Vernähstiche** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.vernaeh_stichlaenge` · Objekt: `einstellungen.verbinder.vernaeh_stichlaenge` | 1 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2730/0x2731 |
| **Zahl der Vernähstiche** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.vernaeh_stiche` · Objekt: `einstellungen.verbinder.vernaeh_stiche` | 2 Anzahl |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272f/0x275d |
| **Verbinder zum nächsten Objekt** | Sprung wie das Original; Lauf = gerader Laufstich („Run“, kein Vernähen davor, kein Faden zum Abschneiden); verdeckt = Lauf nur, wo später gestickte Flächen ihn bedecken (eigene Erweiterung) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.art` · Objekt: `einstellungen.verbinder.art` | sprung | sprung, lauf, verdeckt |  | beide | einfach | Dialog „Connectors“: Knöpfe Jump/Run = Tag 0x2805 (DIALOG.DLL 0x10036450, hoch), Wert 0 in allen 22 Stilen; „Connector After Object“ 0x2803 |
| **Lauf-Verbinder: Stichlänge** | Stichlänge des Laufs zum nächsten Objekt | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.lauf_laenge` · Objekt: `einstellungen.verbinder.lauf_laenge` | 12,7 mm | 0,3 … 12,7 |  | beide | experte | 0x2803 → 0x2711 = 2286 (12,7 mm), Feld neben „Run“ (DIALOG.DLL 0x100360a0, hoch) |
| **Verbinder im Objekt** | zwischen Teilen eines Objekts, z. B. Buchstaben eines Textes; Werte wie „Verbinder zum nächsten Objekt“ | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.innen_art` · Objekt: `einstellungen.verbinder.innen_art` | sprung | sprung, lauf, verdeckt |  | beide | experte | Dialog „Connectors“: Knöpfe Jump/Run = Tag 0x2805 (DIALOG.DLL 0x10036450, hoch), Wert 0 in allen 22 Stilen; „Connector Inside Object“ 0x2804 |
| **Lauf-Verbinder im Objekt: Stichlänge** | Stichlänge des Laufs im Objekt | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | `einstellungen.verbinder.innen_lauf_laenge` · Objekt: `einstellungen.verbinder.innen_lauf_laenge` | 1,8 mm | 0,3 … 12,7 |  | beide | experte | 0x2804 → 0x2711 = 324 (1,8 mm) (DIALOG.DLL 0x100360a0, hoch) |
| **Verdeckt: Enden und Einzug** | Enden je 0,5 mm frei, bedeckte Flächen 0,3 mm verkleinert, Prüfpunkte alle 0,5 mm |  |  | 0,5 mm |  |  | design | automatisch | Vorschlag (eigene Festlegung) |

## Auto Jump und Maschine

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Auto Jump** | lange Stiche als Sprünge statt geteilt (Standard an, Schrift aus) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `auto_jump` | an |  |  | objekt | experte | Effects-Dialog; AMT 0x2758 (hoch) |
| **Auto Jump: Höchststich** | darüber Sprünge; klein (z. B. 6 mm) = Satin mit mehr Loft | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `auto_jump.max` | 12,7 mm | 0,3 … 12,7 |  | objekt | experte | AMT 0x2711 (hoch) |
| **Auto Jump: Sprunglänge** | Länge der Teilsprünge | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `auto_jump.sprung` | 7 mm | 0,3 … 12,7 |  | objekt | experte | AMT 0x2712 (hoch) |
| **Kürzester Stich** | kürzere Stiche werden entfernt |  |  | 0,3 mm |  |  | design | automatisch | DEFAULTS.DFL (hoch) |
| **Längster Stich** | Maschinengrenze artista 185 |  |  | 12,7 mm |  |  | design | automatisch | DEFAULTS.DFL (hoch) |

## Bearbeiten

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Verstärken (Double Run)** | gleiche Stiche rückwärts noch einmal | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `doppellauf` | aus |  |  | objekt | experte | Handbuch Kap. 23 |
| **Umrandung** | Linie um die Fläche (Text: um jeden Buchstaben), gestickt nach dem Objekt; das Häkchen im einfachen Modus setzt eine dicke Satinkante außen (Text 6 % der Buchstabenhöhe, Flächen 2 mm) | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung` | aus |  |  | objekt | einfach | eigene Erweiterung (Nutzer 21.11.2026); im Original eine zweite Linie von Hand (Handbuch S. 114) |
| **Umrandung: Garn** | Farbe der Umrandung (eigener Farbblock) | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.farbe` | #000000 #rrggbb |  |  | objekt | einfach | eigene Erweiterung (Nutzer 21.11.2026); im Original eine zweite Linie von Hand (Handbuch S. 114); Vorschlag (eigene Festlegung) |
| **Umrandung: Stichart** | Lauf (dünn), Dreifach (kräftig), Satinkante | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.art` | dreifach | lauf, dreifach, satin |  | objekt | experte | eigene Erweiterung (Nutzer 21.11.2026); im Original eine zweite Linie von Hand (Handbuch S. 114); Vorschlag (eigene Festlegung) |
| **Umrandung: nach innen** | Versatz der Linie in die Fläche, damit nach dem Sticken keine Lücke zur Füllung bleibt | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.einzug` | 0,3 mm | 0 … 3 |  | objekt | experte | Vorschlag (eigene Festlegung) – am Kalibrierblatt (Vor Ort V12) prüfen |
| **Umrandung: Breite der Satinkante** | nur bei Stichart Satinkante | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.breite` | 2 mm | 0,5 … 10 |  | objekt | experte | wie Satinlinie (Standard 2 mm) |
| **Umrandung: Breite in % der Buchstabenhöhe** | nur Text/Monogramm mit Satinkante, gilt statt der Breite in mm; Größe/Skalieren zählt mit; unter 1 mm wird daraus eine kräftige Linie (Dreifach, Stichlänge 12 % der Höhe) | text, ttf, monogramm | Objekt: `umrandung.breite_anteil` | – | 1 … 30 |  | objekt | experte | Vorschlag (eigene Festlegung) – Wunsch Nutzer 21.11.2026 (50 mm → 3 mm = 6 %, im einfachen Modus gesetzt) |
| **Umrandung: Satinkante außen** | Innenkante liegt „nach innen“ mm in der Fläche, die Kante wächst nach außen, die Farbe bleibt ganz sichtbar; sonst liegt die Mitte der Kante auf dem Rand | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.aussen` | aus |  |  | objekt | experte | Vorschlag (eigene Festlegung) (im einfachen Modus an) |
| **Umrandung: Stichlänge** | Lauf/Dreifach; ohne Angabe Stil | tatami, satinflaeche, muster, satin, satinlinie, blanket, text, ttf, monogramm, applikation | Objekt: `umrandung.laenge` | – | 0,3 … 12,7 |  | objekt | experte | Stil Lauf (Software Defaults 2,5 mm) |
| **Aussparen (Knockout)** | Fläche spart die genannten Objekte aus | tatami, muster | Objekt: `aussparen` | [] Objektnamen |  |  | objekt | einfach | Remove Overlay (Analyse_Teil3/14) |
| **Überlappung beim Aussparen** | Loch bleibt um so viel kleiner | tatami, muster | Objekt: `ueberlappung` | 0 mm | 0 … 3 |  | objekt | experte | Analyse_Teil3/14 |

## Reihenfolge

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Kürzeste Verbindung (Closest Join)** | jedes Objekt ohne eigenen Einstich beginnt am Punkt nächst dem vorigen Ende (Linien umgedreht, geschlossene Formen gedreht; Schrift ausgenommen) |  | `kuerzeste_verbindung` | aus |  |  | design | experte | MOD1 ResequenceByClosestJoin 0x1006d0c0, GEOMETRY FindExitEntry (hoch); im Original Option von Resequence und Auto Digitizer, sonst „wie digitalisiert“ (Messung Analyse_Teil3/16_reihenfolge.md) |
| **Einstich (entry point)** | Objekt beginnt hier bzw. am nächsten Punkt der Form | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `einstich` | – |  |  | objekt | experte | Handbuch Kap. 7/14 (entry point, Reshape); Umsetzung je Objektart eigene Festlegung (Analyse_Teil3/16_reihenfolge.md) |
| **Ausstich (exit point)** | Objekt endet hier bzw. so nah wie die Form erlaubt | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich | Objekt: `ausstich` | – |  |  | objekt | experte | Handbuch Kap. 7/14 (exit point); Flächen: Beginn gegenüber, eigene Festlegung (Analyse_Teil3/16_reihenfolge.md) |
| **Closest Join je Objekt** | wie „Kürzeste Verbindung“, nur für dieses Objekt; setzen Bild umwandeln, Zauberstab und SVG einlesen beim Einfügen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm, knopfloch, motivlinie, motivfuellung, rueckstich, stielstich, stippling, blackwork, kreuzstich, photosnap | Objekt: `kuerzeste` | aus |  |  | objekt | automatisch | Handbuch S. 116 (Auto Digitizer: closest join); je Objekt eigene Festlegung (Analyse_Teil3/25 §1) |
| **Farbreihenfolge beim Sortieren** | gewünschte Folge der Farben (Move Up/Down), übrige Farben in der Reihenfolge ihres ersten Auftretens |  |  | [] Farbliste |  |  | werkzeug | experte | Dialog „Sequence By Color“ (Handbuch Kap. 14); Befehl reihenfolge --nach-farbe |
| **Überdeckung beachten** | beim Sortieren nach Farbe behalten Objekte, die sich überdecken, ihre Reihenfolge |  |  | aus |  |  | werkzeug | einfach | eigene Erweiterung (das Original prüft nicht, Analyse_Teil3/16_reihenfolge.md) |
| **Überdeckung ab** | kleinere gemeinsame Flächen zählen nicht |  |  | 0,5 mm² |  |  | werkzeug | automatisch | eigene Festlegung |
| **Breite einer Linie bei der Prüfung** | Laufstich zählt als Band dieser Breite |  |  | 0,5 mm |  |  | werkzeug | automatisch | eigene Festlegung |

## Anordnen

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Ausrichten** | Kanten oder Mitten der gewählten Objekte an das zuletzt gewählte |  |  | – | links, rechts, oben, unten, mitte_senkrecht, mitte_waagrecht, mitte |  | werkzeug | experte | Arrange-Leiste BERESENU STR 33233–33239; Handbuch S. 135 („aligned with the last object selected“) |
| **Mirror-Merge Richtung** | waagrecht = Kopie links/rechts, senkrecht = oben/unten, beides = drei Kopien |  |  | links_rechts | links_rechts, oben_unten, beide |  | werkzeug | experte | DESVIEW MirrorMergeView*::GetReflectionTransforms (Analyse_Teil3/26) (hoch); Vorgabe eigene Festlegung |
| **Ankerpunkt** | Punkt, durch den die Spiegellinie geht; Vorschlag rechte bzw. untere Kante der Auswahl |  |  | – |  |  | werkzeug | experte | Handbuch S. 141 (Bezugslinie, Klick setzt Anker); Vorschlag eigene Festlegung |
| **Punkte des Kranzes** | Original + Kopien rundherum, je 360°/n gedreht (im Uhrzeigersinn) |  |  | 6 Anzahl | 2 … 36 |  | werkzeug | experte | DESVIEW MirrorMergeView*::GetReflectionTransforms (Analyse_Teil3/26) (Drehung −i·2π/n, hoch); Vorgabe Handbuch-Beispiel „six-point wreath“, Bereich eigene Festlegung |
| **Kaleidoskop** | jede zweite Kopie an der Bezugslinie gespiegelt; nur gerade Zahl |  |  | aus |  |  | werkzeug | experte | DESVIEW MirrorMergeView*::GetReflectionTransforms (Analyse_Teil3/26) (Mirror bei ungeradem i, hoch); Handbuch S. 142 |
| **Mittelpunkt** | Mitte des Kranzes; Vorschlag unter der Auswahl, Kopien berühren sich nicht |  |  | – |  |  | werkzeug | experte | Handbuch S. 141; Vorschlag eigene Festlegung |
| **Überlappende zusammenführen** | Step-, Muster-, Stippling- und Satinflächen, die sich mit ihrer Kopie überlappen, werden ein Objekt |  |  | aus |  |  | werkzeug | experte | RESENU STR 4159 (Rückfrage, hoch); Verfahren eigene Festlegung (Analyse_Teil3/26 §4) |
| **Überlappung ab** | kleinere gemeinsame Flächen zählen nicht als Überlappung |  |  | 0,5 mm² |  |  | werkzeug | automatisch | eigene Festlegung |

## Dateien

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Rahmen beim Öffnen** | kleinster Rahmen der 185, in den das Design passt (Freiarm nur, wenn nichts anderes passt) |  |  | – |  |  | werkzeug | automatisch | eigene Festlegung (Analyse_Teil3/27 §2) |
| **Hinweis beim Skalieren fester Stiche** | ab dieser Größenänderung meldet das Programm, dass sich die Dichte ändert (Stichzahl bleibt) |  |  | 5 % |  |  | werkzeug | automatisch | Handbuch Kap. 28 S. 256 („more than ±5 %“) |
| **Vernähen fester Stiche** | Stichdaten aus Dateien werden nicht zusätzlich vernäht (die Datei hat ihr eigenes) |  |  | aus |  |  | werkzeug | automatisch | eigene Festlegung (Grade D: Stiche unverändert, Handbuch S. 256) |

## Blanket

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Zahnabstand** |  | blanket | Objekt: `abstand` | 2 mm | 0,5 … 10 |  | objekt | experte | Stil 0x2727/0x2728 |
| **Zahnbreite** |  | blanket | Objekt: `breite` | 3 mm | 0,5 … 12 |  | objekt | einfach | Stil 0x2729 |
| **Lage** | mittig oder zur Seite | blanket | Objekt: `versatz` | mitte | mitte, seite |  | objekt | experte | Dialog Center/Offset |

## Applikation

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Abdeckung** | Satin oder Blanket | applikation | Objekt: `abdeckung` | satin | satin, blanket |  | objekt | einfach | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f2f = 0x2721 Satin |
| **Stoff zuschneiden** | vorgeschnitten oder im Rahmen zuschneiden (mit Schnittlinie) | applikation | Objekt: `stil` | vorgeschnitten | vorgeschnitten, zuschneiden |  | objekt | einfach | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f31 = 1 = Pre-Cut |
| **Versatz** | Abdeck- und Heftstich gegen die Platzierungslinie: −100 ganz außen, 0 mittig, 100 ganz im Stoff (bei Blanket immer innen) | applikation | Objekt: `versatz` | 0 % | -100 … 100 |  | objekt | einfach | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f28/0x3f1e = 0, Schieberegler −1 … +1 in 21 Stufen |
| **Teil-Abdeckung** | kein Abdeckstich, wo eine spätere Applikation den Rand überdeckt (Partial Appliqué) | applikation | Objekt: `teilabdeckung` | aus |  |  | objekt | einfach | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f26 = 0; Handbuch S. 248–249; Schnitt eigene Festlegung |
| **Breite der Abdeckung** |  | applikation | Objekt: `breite` | 5 mm | 0,5 … 10 |  | objekt | experte | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f27 = 900 |
| **Abstand der Abdeckung** | Satinabstand bzw. Abstand der Blanket-Zähne | applikation | Objekt: `abstand` | 0,5 mm | 0,2 … 10 |  | objekt | experte | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f30 = 90 (Satin); Blanket ohne Angabe 2,0 (Stil 0x2728, eigene Festlegung) |
| **Heftstich: Breite** | Zickzack | applikation | Objekt: `heft_breite` | 4 mm | 0,5 … 10 |  | objekt | experte | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f1d = 720 |
| **Heftstich: Abstand** |  | applikation | Objekt: `heft_abstand` | 5 mm | 0,5 … 12,7 |  | objekt | experte | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f21 = 900 |
| **Platzierungslinie: Stichlänge** |  | applikation | Objekt: `platz_laenge` | 1,5 mm | 0,5 … 12,7 |  | objekt | experte | AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch): 0x3f1a = 270 |

## Knopfloch

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Knopfloch-Typ** | Vorlage aus ButtonHoles.ESB (9 Typen) | knopfloch | Objekt: `typ` | standard | standard, standard_schmal, rund, dehnbar, heirloom, rundes_ende, auge_eckig, auge_rund, auge_spitz |  | objekt | einfach | MOD1 AllButtonHolesValues 0x10028060 (hoch); Bericht Analyse_Teil3/19, Typ 0x3ff9 |
| **Lage** | Mitte des Schlitzes [x, y] | knopfloch | Objekt: `position` | – |  |  | objekt | einfach | Vorschlag (eigene Festlegung) |
| **Schlitzlänge** | Raupen werden gestreckt, oberes Ende rückt nach (Slit Length) | knopfloch | Objekt: `laenge` | 20 mm | 3 … 60 |  | objekt | einfach | MOD1 AllButtonHolesValues 0x10028060 (hoch); Bericht Analyse_Teil3/19, 3600 = 20 mm; Bereich eigene Festlegung |
| **Winkel** | Richtung des Schlitzes, 90° = senkrecht (Angle) | knopfloch | Objekt: `winkel` | 90 Grad |  |  | objekt | einfach | MOD1 AllButtonHolesValues 0x10028060 (hoch); Bericht Analyse_Teil3/19, π/2 |
| **Raupenabstand** | Dichte der Raupen (Bead Stitch Spacing); Enden bleiben wie die Vorlage; ohne Angabe wie die Vorlage (0,5 mm, Heirloom 1,5 mm) | knopfloch | Objekt: `abstand` | – | 0,2 … 2 |  | objekt | experte | MOD1 AllButtonHolesValues 0x10028060 (hoch); Bericht Analyse_Teil3/19, 90 = 0,5 mm; Musterdesign Buttonholes.ART |

## Motivlinie

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Motivsatz** | einer der 14 Motivsätze der CD (userletw) | motivlinie | Objekt: `satz` | NP004a |  |  | objekt | einfach | Befehl motive_liste; Standard NORMAL.AMT <PRESET_STANDARD_MOTIF_RUN> 0x2798 (Analyse_Teil3/24 §1) |
| **Motiv** | Motiv im Satz (Auswahl mit Vorschau) | motivlinie | Objekt: `motiv` | K0001a |  |  | objekt | einfach | Befehl motivsatz_motive; Standard <PRESET_STANDARD_MOTIF_RUN> (Analyse_Teil3/24 §1) |
| **Größe** | Länge eines Motivs entlang der Linie (Size); Höhe wächst mit | motivlinie | Objekt: `groesse` | – |  |  | objekt | einfach | Musterdesigns „Sampler - Pattern Outlines“ (Analyse_Teil3/20, hoch); Handbuch Kap. 25: Größe = Grundlinie der Kopie, ohne Angabe wie im Motivsatz |
| **Abstand** | Wiederholabstand (Spacing); Anzahl = Linienlänge / Abstand gerundet, Motive gleichmäßig verteilt | motivlinie | Objekt: `abstand` | – |  |  | objekt | experte | Musterdesigns „Sampler - Pattern Outlines“ (Analyse_Teil3/20, hoch); Handbuch Kap. 25: Verteilung gemessen; Abstand ≠ Größe nicht gemessen (mittel) |
| **Spiegeln** | Motive auf die andere Seite der Linie | motivlinie | Objekt: `spiegeln` | aus |  |  | objekt | einfach | Vorschlag (eigene Festlegung) (Handbuch: Orientation) |

## Motivfüllung

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Motivsatz** | Motivsatz der CD oder eigener Satz | motivfuellung | Objekt: `satz` | NP004a |  |  | objekt | einfach | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: Satz NP004a |
| **Motiv** | Motiv im Satz (Auswahl mit Vorschau) | motivfuellung | Objekt: `motiv` | K0001a |  |  | objekt | einfach | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: Motiv K0001a |
| **Motivgröße** | Länge der Grundlinie eines Motivs; Höhe und Abstände wachsen mit | motivfuellung | Objekt: `groesse` | – |  |  | objekt | einfach | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: 0x3ab8 = 2158 (12,0 mm, Motiv K0001a); ohne Angabe wie im Motivsatz |
| **Spaltenabstand** | Abstand der Motive in einer Reihe (column spacing) | motivfuellung | Objekt: `spaltenabstand` | – |  |  | objekt | experte | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: 0x27b9 = 2158 = Kante an Kante; ohne Angabe Breite des Motivs |
| **Reihenabstand** | Abstand der Reihen (row spacing) | motivfuellung | Objekt: `reihenabstand` | – |  |  | objekt | experte | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: 0x27b6 = 2158 = Kante an Kante; ohne Angabe Höhe des Motivs |
| **Reihenversatz** | jede Reihe um so viel weiter verschoben (row offset) | motivfuellung | Objekt: `versatz` | 0 mm |  |  | objekt | experte | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: Leitmotiv-Matrix 0x3abe = Einheitsmatrix (kein Versatz) |
| **Drehung der Reihen** | dreht das Raster (Stichwinkel wirkt nicht) | motivfuellung | Objekt: `winkel` | 0 ° | -180 … 180 |  | objekt | experte | NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229: „The stitch angle has no effect on pattern layout“ |

## Craft Stitches

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Länge** | Länge eines Rückstichs (Abstand der Punkte entlang der Linie) | rueckstich | Objekt: `laenge` | 3 mm | 0,5 … 12,7 |  | objekt | einfach | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2851 = 540 = 3,0 mm (hoch) |
| **Striche** | Fäden je Stich: 3 (Hin, Rück, Hin) oder 5 | rueckstich | Objekt: `striche` | 3 Anzahl | 3 … 5 |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2854 = 3 (hoch) |
| **Stärke** | Seitenversatz der Striche – wie dick die Linie wirkt | rueckstich | Objekt: `staerke` | 0,2 mm | 0 … 1 |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2852 = 36 = 0,2 mm (hoch) |
| **Überlappung** | wie weit ein Stich in den vorigen zurückreicht | rueckstich | Objekt: `ueberlappung` | 50 % | 0 … 100 |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2853 = 0,5 (hoch) |
| **Breite** | Breite der Kordel | stielstich | Objekt: `breite` | 2 mm | 0,5 … 10 |  | objekt | einfach | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2857 = 360 = 2,0 mm (Zuordnung mittel) |
| **Abstand** | Abstand der Stiche entlang der Linie | stielstich | Objekt: `abstand` | 1,5 mm | 0,2 … 10 |  | objekt | einfach | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2859 = 270 = 1,5 mm (mittel) |
| **Stichwinkel** | Neigung der Stiche zur Linie; in engen Kurven steiler | stielstich | Objekt: `winkel` | 50 ° | 10 … 90 |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x285a = 50° (mittel) |
| **Dreifach** | jeder Stich mit Hin-Rück-Hin – Seil-Optik | stielstich | Objekt: `dreifach` | an |  |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2858 = 3 (mittel) |
| **Stärke** | Versatz der Dreifach-Striche | stielstich | Objekt: `staerke` | 0,2 mm | 0 … 1 |  | objekt | experte | NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24): 0x2856 = 36 = 0,2 mm (mittel) |
| **Schlaufenabstand** | Kantenlänge des Wabengitters, um das die mäandernde Linie läuft (Loop Spacing) | stippling | Objekt: `abstand` | 3 mm | 0,5 … 20 |  | objekt | einfach | Dokument StippleFillValues 540 = 3,0 mm (MOD1, hoch); Verfahren StippleFillInterface 0x101a4590 (hoch); Bereich Vorschlag (eigene Festlegung) |
| **Stichlänge** | Länge der Laufstiche | stippling | Objekt: `laenge` | 2 mm | 0,5 … 12,7 |  | objekt | einfach | Dokument StippleFillValues 360 = 2,0 mm (MOD1, hoch) |
| **Stichart (Blackwork)** | Stichart der verbundenen Linien; Werte wie beim Objekt dieser Art | blackwork | Objekt: `stich` | lauf | lauf, dreifach, rueckstich, stielstich |  | objekt | einfach | Handbuch S. 131–132 (Blackwork Run: Einfach-/Dreifachlauf, Stielstich, Rückstich); Reihenfolge Vorschlag (eigene Festlegung) |

## PhotoSnap

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Auflösung** | Breite der Stichreihen: grob 3, mittel 2, fein 1 mm | photosnap | Objekt: `aufloesung` | mittel | grob, mittel, fein |  | objekt | einfach | Handbuch S. 119 (Coarse/Medium/Fine); mittel 2,0 mm = 0x3ee5 = 360 (niedrig); grob/fein Vorschlag (eigene Festlegung) |
| **Stoff** | hell: dunkle Stellen bekommen viel Faden; dunkel: helle Stellen | photosnap | Objekt: `hintergrund` | hell | hell, dunkel |  | objekt | einfach | MOD1 PhotoValues 0x10122d50 / GenerateComponents 0x10120aa0, NORMAL.AMT 0x3ee4 (Analyse_Teil3/25 §3): 0x3ee8 = 1; Handbuch S. 120 Background Light/Dark |
| **Winkel der Reihen** | Richtung der Stichreihen | photosnap | Objekt: `winkel` | 0 ° | -360 … 360 |  | objekt | einfach | MOD1 PhotoValues 0x10122d50 / GenerateComponents 0x10120aa0, NORMAL.AMT 0x3ee4 (Analyse_Teil3/25 §3): 0x3eeb, Vorgabe 0 im Konstruktor (Handbuch-Bild 45°) |
| **Farblagen** | gekreuzt: je Lage 45° mehr; versetzt: gleicher Winkel, Reihen je Lage 0,4 mm verschoben | photosnap | Objekt: `lagen` | gekreuzt | gekreuzt, versetzt |  | objekt | einfach | MOD1 PhotoValues 0x10122d50 / GenerateComponents 0x10120aa0, NORMAL.AMT 0x3ee4 (Analyse_Teil3/25 §3): Modus 1 = 0/45/90/135° (0x10120890), Modus 2 = 72/144/216 (0x101209e0) (hoch) |
| **Abstand dunkel** | Abstand der Zickzackstiche an der dunkelsten Stelle | photosnap | Objekt: `abstand_min` | 0,8 mm | 0,4 … 5 |  | objekt | experte | Stil 0x2828 Feld 0x282a = 144 (Zuordnung niedrig) |
| **Abstand hell** | Abstand an der hellsten Stelle (größter Abstand) | photosnap | Objekt: `abstand_max` | 10 mm | 1 … 12 |  | objekt | experte | Stil 0x2828 Feld 0x282f = 1800 (Zuordnung niedrig); Handbuch „Max Width … lightest part“ |
| **Farbe** | einfarbig (Graustufen) oder vier Farblagen Cyan, Magenta, Gelb, Schwarz |  |  | aus |  |  | werkzeug | einfach | MOD1 PhotoValues 0x10122d50 / GenerateComponents 0x10120aa0, NORMAL.AMT 0x3ee4 (Analyse_Teil3/25 §3): 0x3ee9 = 1 einfarbig; farbig Split_CMYK (hoch) |
| **Rasterpunkt** | Größe eines Bildpunkts im gespeicherten Raster |  |  | 0,5 mm |  |  | werkzeug | automatisch | Vorschlag (eigene Festlegung) |

## Zauberstab

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Stichart des nächsten Klicks** | Fläche Step/Satin/Auto oder Umriss Lauf/Dreifach/Satinlinie |  |  | step | step, satin, auto, lauf, dreifach, satinlinie |  | werkzeug | einfach | Handbuch S. 114–115 (Magic Wand: Stichart vor dem Klick) |

## Kreuzstich

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Kreuz** | Art der Kreuze: voll, ¾, ½, ¼, Mini, aufrecht, doppelt, längs, quer, Knoten | kreuzstich | Objekt: `kreuz` | voll | voll, dreiviertel, halb, viertel, mini, aufrecht, doppelt, laengs, quer, knoten |  | objekt | einfach | Handbuch Kap. 34–37; Formulare BLOT.EXE/XSLang.DLL (Analyse_Teil3/29, hoch): Liste FillStyle |
| **Ecke / Hälfte** | Viertel der Zelle bei ¾, ¼, Mini; Seite bei länglichen Kreuzen | kreuzstich | Objekt: `ecke` | unten_links | unten_links, unten_rechts, oben_links, oben_rechts |  | objekt | einfach | Handbuch Kap. 34–37; Formulare BLOT.EXE/XSLang.DLL (Analyse_Teil3/29, hoch): „Select the quadrants to be filled“; Standard Vorschlag (eigene Festlegung) |
| **Richtung** | Halbkreuz „/“ oder „\“ | kreuzstich | Objekt: `richtung` | steigend | steigend, fallend |  | objekt | einfach | Handbuch S. 308; Standard Vorschlag (eigene Festlegung) |
| **Fäden** | 2 = einmal, 4 = zweimal, 6 = dreimal gestickt | kreuzstich | Objekt: `faeden` | 2 | 2, 4, 6 |  | objekt | einfach | Handbuch S. 318 (Threads-Liste, hoch) |
| **Kreuzgröße** | Kantenlänge eines Kreuzes (Fabric Count): 2,54 mm = 10 je Zoll | kreuzstich | Objekt: `groesse` | 2,54 mm | 1 … 8 |  | objekt | einfach | Handbuch S. 317–318 (Beispiel 10 je Zoll); Standard und Bereich Vorschlag (eigene Festlegung) |
| **Kontur** | keine, Linie auf dem Raster (Single Line) oder Kreuze entlang des Umrisses (Current Cross Type) | kreuzstich | Objekt: `kontur` | keine | keine, linie, kreuz |  | objekt | einfach | Handbuch Kap. 34–37; Formulare BLOT.EXE/XSLang.DLL (Analyse_Teil3/29, hoch): Liste OutlineStyle |
| **Füllen** | aus = nur die Kontur | kreuzstich | Objekt: `fuellen` | an |  |  | objekt | experte | Handbuch Kap. 34–37; Formulare BLOT.EXE/XSLang.DLL (Analyse_Teil3/29, hoch): Fill an/aus |
| **Übergänge** | Verbindungen nur auf Rasterlinien an gestickten Kreuzen, bis 16 Kreuze weit, sonst Sprung |  |  | 16 Kreuze |  |  | werkzeug | automatisch | Vorschlag (eigene Festlegung) (Stichfolge des Originals nicht gemessen, Analyse_Teil3/29 §2) |
| **Französischer Knoten** | sechs Stiche dieser Länge um die Zellmitte |  |  | 0,5 mm |  |  | werkzeug | automatisch | Vorschlag (eigene Festlegung) |
| **Farben (Bild → Kreuzstich)** | höchstens so viele Farben; ein Bildpunkt = ein Kreuz |  |  | 8 Anzahl | 2 … 64 |  | werkzeug | einfach | Formular TAutoStitchImageForm („Restricted“ 2 … 255); Standard und Obergrenze Vorschlag (eigene Festlegung) |

## Effekte

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Verlauf: Profil** | Verlauf des Abstands über die Fläche bzw. entlang der Säule (Gradient Fill) | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.verlauf.profil` | linear | linear, abnehmend, konvex, konkav, exp_zunehmend, exp_abnehmend, konvex_konkav, konkav_konvex |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): GradientSpacingEffect 0x34, Profil-Byte 1–4, Stil 4800680101 = Profil 1; Formeln und Richtung Vorschlag (eigene Festlegung) |
| **Verlauf: Abstand dicht** | kleinster Abstand | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.verlauf.min` | 0,4 mm | 0,1 … 10 |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): Stil 0x48 = 72 = 0,4 mm; Tomatos.ART gemessen (linear) |
| **Verlauf: Abstand offen** | größter Abstand (Maximum Spacing) | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.verlauf.max` | 2 mm | 0,1 … 10 |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): Stil 0x168 = 360 = 2,0 mm (0x2789/0x278a) |
| **Ausgefranster Rand: Seite** | links, beide, rechts (in Stichrichtung; Satin: links = Seite A) | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.rand.seite` | rechts | links, rechts, beide |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): JaggedEdgeEffect 0x32, Seite 2 = rechts (Tomatos.ART: eine Seite gestreut) |
| **Ausgefranster Rand: Spanne** | so weit streuen die Enden (Span) | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.rand.spanne` | 1,5 mm | 0,1 … 10 |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): Stil 270 = 1,5 mm |
| **Ausgefranster Rand: Glätte** | 0 = rau (jede Reihe frei), 1 = glatt (Texture) | tatami, muster, satin, satinlinie, satinflaeche | Objekt: `effekte.rand.glaette` | 0,1 0 … 1 | 0 … 1 |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): Stil 0,1; Zufall/Glättung Vorschlag (eigene Festlegung) |
| **Welle: Leitlinie** | Reihen folgen der Linie (Wave Effect); Einschalten legt eine Linie mit Bogen an (welle_vorschlag) | tatami, muster | Objekt: `effekte.welle` | – |  |  | objekt | experte | IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23): CurveFillEffect 0x6b/0x6d + OneCurveD2Wrap 0x6e; Vorschlag Vorschlag (eigene Festlegung) |

## Schrift

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Alphabet** | eines der 50 Original-Alphabete | text | Objekt: `alphabet` | – |  |  | objekt | einfach | ESB-Alphabete (Analyse/daten/alphabets.md); Befehl alphabete_liste |
| **Computer-Schrift** | TrueType/OpenType-Schrift | ttf | Objekt: `schrift` | – |  |  | objekt | einfach | Handbuch Kap. 21; Stichart Vorschlag (eigene Festlegung) |
| **Höhe** | Buchstabenhöhe | text, ttf, monogramm | Objekt: `hoehe` | 10 mm | 1 … 200 |  | objekt | einfach | Handbuch S. 165; Vorlage 0x3e88 (Monogramm 18 mm) |
| **Breite** | Breite in % der Höhe | text, ttf, monogramm | Objekt: `breite` | 100 % | 10 … 400 |  | objekt | experte | Vorlage 0x3e89; Bereich Vorschlag (eigene Festlegung) |
| **Buchstabenabstand** | ohne Angabe: Wert des Alphabets | text, ttf, monogramm | Objekt: `abstand` | – |  |  | objekt | experte | Alphabet-Kopf (Analyse_Teil3/09) |
| **Wortabstand** | Breite des Leerzeichens | text, ttf, monogramm | Objekt: `wortabstand` | 25 % der Höhe | 0 … 200 |  | objekt | experte | Vorlage 0x3e85 |
| **Zeilenabstand** | Lücke zwischen den Zeilen | text, ttf, monogramm | Objekt: `zeilenabstand` | 60 % der Höhe | 0 … 300 |  | objekt | experte | Vorlage 0x3f2d |
| **Ausrichtung** | links, mitte, rechts, Blocksatz (nur mit fester Grundlinienlänge) | text, ttf, monogramm | Objekt: `ausrichtung` | mitte | links, mitte, rechts, block |  | objekt | einfach | Lettering-Dialog Justification |
| **Kursiv** | Neigung der Buchstaben | text, ttf, monogramm | Objekt: `kursiv` | 0 ° | -45 … 45 |  | objekt | einfach | Lettering-Dialog Italic; Vorlage 0x3e8a |
| **Grundlinie** | gerade, Kreis/Oval/Bogen, senkrecht, eigene Kurve, Wappen (mehrfach) | text, ttf, monogramm | Objekt: `grundlinie.art` | – | kreis, gerade, senkrecht, kurve, mehrfach |  | objekt | einfach | Dialog „Layout“ (DLG 1039) |
| **Radius** | Kreisradius (Oval: Radius X) | text, ttf, monogramm | Objekt: `grundlinie.radius` | 50 mm | 5 … 200 |  | objekt | einfach | Vorlage 0x3e8b (mittel) |
| **Oben/unten am Kreis** | oben im Uhrzeiger, unten gegen den Uhrzeiger | text, ttf, monogramm | Objekt: `grundlinie.lage` | oben | oben, unten |  | objekt | einfach | Circle CW/CCW |
| **Radius Y (Oval)** |  | text, ttf, monogramm | Objekt: `grundlinie.radius_y` | – |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Lage auf dem Kreis / Linienwinkel** |  | text, ttf, monogramm | Objekt: `grundlinie.winkel` | – |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Neigung des Ovals** |  | text, ttf, monogramm | Objekt: `grundlinie.neigung` | 0 ° |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Bogenwinkel** |  | text, ttf, monogramm | Objekt: `grundlinie.bogen` | – |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Länge (gerade)** |  | text, ttf, monogramm | Objekt: `grundlinie.laenge` | – |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Text über/auf/unter der Linie** |  | text, ttf, monogramm | Objekt: `grundlinie.stand` | – | ueber, durch, unter |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Eigene Kurve** |  | text, ttf, monogramm | Objekt: `grundlinie.punkte` | – |  |  | objekt | experte | Dialog „Layout“ (DLG 1039) |
| **Farbwechsel im Text** | ^ im Text wechselt zur nächsten Farbe der Liste | text, ttf, monogramm | Objekt: `farben` | [] Farbliste |  |  | objekt | einfach | Handbuch S. 158 |
| **Einzelne Buchstaben** | Farbe, verschieben, drehen, skalieren, Winkellinien je Buchstabe | text, ttf, monogramm | Objekt: `buchstaben` | [] Liste |  |  | objekt | experte | Handbuch Kap. 19/20 |
| **Stichart der Schrift** | Satin (Alphabet), Step, Füllmuster oder automatisch (Satin, ab 8 mm Stichlänge Step); Computer-Schrift: Tatami, Lauf, Dreifach, Satin (Säulen wie die TrueType-Umwandlung) oder auto. Einfacher Modus: neue Texte „auto“, Schalter „Trotzdem Satin“ | text, monogramm, ttf | Objekt: `stichart` | satin | satin, auto, step, muster, tatami, lauf, dreifach |  | objekt | einfach | Handbuch Kap. 20; Verteilung Vorschlag (eigene Festlegung); ttf satin: EZSTITCH Turning (Analyse_Teil3/17); auto = eigene Erweiterung |
| **Step ab Stichlänge (Stichart auto)** | wird ein Satinstich der Schrift (nach Größe und Transformation) länger, stickt der ganze Text Step; darüber ohne „auto“ ein Hinweis | text, monogramm, ttf |  | 8 mm |  |  | objekt | automatisch | eigene Erweiterung; Grenze wie EasyStitch (NORMAL.AMT 0x3f54, Analyse/code/10 §C), Handbuch S. 118 (Satin nicht über 7 mm); Konstante schrift::AUTO_STEP_AB_MM |
| **Verzerren (Elastic)** | gerade (Wimpel) oder gebogen (Bogen), Griffe oben/unten | text, ttf, monogramm | Objekt: `elastisch` | – |  |  | objekt | einfach | Handbuch Kap. 21; Kantenform Vorschlag (eigene Festlegung) |
| **Initialen** | 2 oder 3 Buchstaben | monogramm | Objekt: `initialen` | – |  |  | objekt | einfach | Diamond2/Diamond3 (Analyse_Teil3/09 §7) |
| **Monogramm-Rahmen** | Raute, Zier oder keiner | monogramm | Objekt: `rahmen` | keine | raute, zier, keine |  | objekt | einfach | Analyse_Teil3/09 §7 |
| **Rahmenfarbe** |  | monogramm | Objekt: `rahmen_farbe` | – |  |  | objekt | einfach | Farbwechsel ^ |
| **Rahmen aus dem Katalog** | einer der 47 Rahmen (Befehl monogramm_rahmen); mit Vorlage 1–3 Zeichen, Schrift der Vorlage, Initialen automatisch eingepasst | monogramm | Objekt: `vorlage` | – |  |  | objekt | einfach | eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026) |
| **Schrift der Initialen** | Computer-Schrift (TTF/OTF, Google Fonts); ohne Angabe der Schriftvorschlag der Vorlage | monogramm | Objekt: `schrift` | – |  |  | objekt | einfach | eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026) |
| **Stick-Alphabet der Initialen** | statt der Computer-Schrift ein Original-Alphabet | monogramm | Objekt: `alphabet` | – |  |  | objekt | einfach | eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026) |
| **Größe des Rahmens** | längere Seite in mm; die Initialen wachsen mit; ohne Angabe wie gezeichnet | monogramm | Objekt: `rahmen_groesse` | – | 10 … 200 |  | objekt | einfach | eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026); Rahmen gezeichnet 70–88 mm |
| **Garne des Rahmens** | je Farbe der Vorlage (1–3) ein Garn; leer = Farben der Vorlage | monogramm | Objekt: `rahmen_farben` | [] Farbliste |  |  | objekt | einfach | eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026) |
| **Lücke zwischen den Initialen** | Anteil der größten Versalhöhe | monogramm |  | 8 % |  |  | objekt | automatisch | eigene Festlegung (monogramm::LUECKE_ANTEIL) |
| **Abstand Initialen – Rahmen** | die Initialen werden verkleinert, bis sie mit diesem Abstand in den Rahmen passen (höchstens auf 30 %) | monogramm |  | 2 mm |  |  | objekt | automatisch | eigene Festlegung (monogramm::ABSTAND, MIN_ANTEIL) |
| **Äußere Initialen bei 3 in einem Rahmen für 2** | Höhe der äußeren Initialen im Verhältnis zur mittleren | monogramm |  | 68 % |  |  | objekt | automatisch | eigene Festlegung (monogramm::SEITE_ANTEIL; Verhältnis der Vorlagen mit 3 Initialen) |
| **Schrift: Auto-Spacing-Faktor** | Schrift 10 % dichter |  |  | 0,9 Faktor |  |  | design | automatisch | Stil „Schrift“ (Analyse/code/10) |
| **Schrift: Unterlage** | Zickzack, wenn das Alphabet keine eigene hat |  |  | zickzack |  |  | design | automatisch | Stil „Schrift“ |
| **Schrift: Laufweg** | Stichlänge der Wege im Buchstaben |  |  | 3 mm |  |  | design | automatisch | Lauf 0x271c im Alphabet |

## Satinfläche (Stichrichtung automatisch)

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Säulen am Knick teilen** | Innenwinkel, unter dem eine Säule am Knick geteilt wird (0 = nie) | satinflaeche | Objekt: `knickwinkel` | 0 ° | 0 … 179 |  | objekt | experte | Auto Digitizer teilt nicht (gemessen, Analyse_Teil3/17_auto_satin.md); TrueType-Umwandlung 87° (Break Angle, MOD1 0x101f5bc0) |
| **Randabstand höchstens** | Randpunkte für die Zerlegung höchstens so weit auseinander |  |  | 1 mm |  |  | design | automatisch | EZSTITCH ShapeSample „low precision“ 180 (AutoFont-Generate) |
| **Randabstand mindestens** | feiner wird nicht abgetastet |  |  | 0,05 mm |  |  | design | automatisch | EZSTITCH ShapeSample „high precision“ 9 |
| **Randabstand je Breite** | Randpunkte im Abstand Anteil × örtliche Breite |  |  | 0,1 Faktor |  |  | design | automatisch | EZSTITCH ShapeSample „proportional precision“ 0,1 |
| **Eckerkennung** | Randecken spitzer als das bekommen eine Stichlinie; Eck-Zweige werden abgeschnitten |  |  | 120 ° |  |  | design | automatisch | Corner Detection Angle (TurningConversionValues, MOD1 0x101f5bc0) |
| **Dichte der Stichlinien** | Abweichung der Mittellinie (× Breite), ab der eine neue Stichlinie kommt |  |  | 0,3 Faktor |  |  | design | automatisch | eigene Festlegung, an 54 Säulen des Auto Digitizers eingestellt (Analyse_Teil3/17_auto_satin.md) |

## Bild → Umrisse

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Breite** | Breite des Ergebnisses (Höhe folgt) |  |  | 80 mm |  |  | werkzeug | einfach | stick-vektor (eigene Festlegung) |
| **Farben** | Zahl der Farben im Bild nach der Reduktion (k-Means im Lab-Raum), auch die weggelassenen |  |  | 8 Anzahl |  |  | werkzeug | einfach | stick-vektor (eigene Festlegung) |
| **Vereinfachen** | kantenerhaltendes Glätten vor der Farbreduktion (Kuwahara); 0 = aus |  |  | 2 Stufe 0–5 |  |  | werkzeug | einfach | Schritt nach Handbuch Kap. 12 „Image clean up“, Verfahren eigene Festlegung |
| **Farben weglassen** | Farben, die nicht gestickt werden; ohne Wahl: fast weiße häufigste Randfarbe |  |  | weißer Hintergrund Farbliste |  |  | werkzeug | einfach | Auto Digitizer „Omitted Colors“, „white background omitted“ (Handbuch S. 117) |
| **Garnfarben** | Farben auf die nächsten Garne der Garnkarte des Designs ziehen (CIEDE2000); Oberfläche: aus bei der Karte „Default“ (nur 19 Grundfarben) |  |  | an |  |  | werkzeug | einfach | eigene Festlegung (Zuordnung wie `garnkarte`) |
| **Stichart der Flächen** | „step“ alles Tatami, „auto“ wie EasyStitch (bis 8 mm Satin) |  |  | step |  |  | werkzeug | experte | Auto Digitizer „Fills: Auto/Step“ (Handbuch S. 118); Standard Step eigene Festlegung |
| **Kleinste Fläche** | kleinere Flächen gehen in der Nachbarfarbe auf (keine Löcher) |  |  | 4 mm² |  |  | werkzeug | experte | stick-vektor (eigene Festlegung) |
| **Glättung** | Toleranz der Vereinfachung der Umrisse |  |  | 0,2 mm |  |  | werkzeug | experte | stick-vektor (eigene Festlegung) |
| **Zugausgleich** | für alle umgewandelten Flächen (fest) |  |  | 0,4 mm |  |  | werkzeug | automatisch | Handbuch S. 116 „Change the pull compensation to 0.4mm when using Auto Digitizer“; EasyStitch-Stile 0x273d |
| **Unterlage** | an für alle umgewandelten Flächen (fest) |  |  | an |  |  | werkzeug | automatisch | Handbuch S. 116 „Always remember to apply underlays“ |

## SVG einlesen

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Breite** | Breite des Ergebnisses (ohne Angabe: Größe aus der SVG, 1 px = 25,4/96 mm) |  |  | – |  |  | werkzeug | einfach | SVG-Norm (CSS 96 px je Zoll) |
| **Überdeckung aussparen** | Füllflächen sparen aus, was in der SVG später (darüber) gezeichnet ist |  |  | an |  |  | werkzeug | experte | eigene Festlegung (Bedeutung der SVG), exakt wie „Remove Overlay Stitching“ |
| **Kürzeste Verbindung** | Closest Join wie der Auto Digitizer |  |  | an |  |  | werkzeug | experte | Handbuch PDF-S. 207 (Auto Digitizer: closest join) |
| **Dünne Linie bis** | Laufstich (Thin Lines) |  |  | 0,3 mm |  |  | werkzeug | automatisch | EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C), 0x3f77 = 54 (hoch) |
| **Kräftige Linie bis** | Laufstich (Thick Lines) |  |  | 2 mm |  |  | werkzeug | automatisch | EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C), 360 (hoch) |
| **Satin bis** | Satinfläche bzw. Satinlinie (Narrow Objects) |  |  | 8 mm |  |  | werkzeug | automatisch | EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C), 1440 (hoch) |
| **Tatami 1 bis** | Tatami (Wide Objects), darüber Tatami (Other Objects) |  |  | 12 mm |  |  | werkzeug | automatisch | EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C), 2160 (hoch) |
| **Kleinste Fläche** | kleinere Flächen entfallen |  |  | 1 mm² |  |  | werkzeug | automatisch | EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C), 0x3f57 = 32 400 (Rolle nicht bestimmt – Annahme) |
| **Laufstich der Linien** | Stichlänge (Thin/Thick Lines) |  |  | 2 mm |  |  | werkzeug | automatisch | Stile <PRESET_EASYSTITCH>Thin/Thick Lines 0x271d (hoch) |
| **Tatami-Abstand** | Wide/Other Objects |  |  | 0,45 mm |  |  | werkzeug | automatisch | Stile <PRESET_EASYSTITCH>Wide/Other Objects 0x2716 (hoch) |
| **Zugausgleich der Flächen** | Narrow/Wide/Other Objects |  |  | 0,4 mm |  |  | werkzeug | automatisch | Stile <PRESET_EASYSTITCH>Narrow/Wide/Other 0x273d = 72 (hoch); Handbuch „Use 0.4 when using automatic digitizing“ |

## Werte je Stoff

| Parameter | 0 Software Defaults | Knit Light Weight | Knit Medium Weight | Knit Heavy Weight | Terry Medium Weight | Terry Heavy Weight | Woven Light Weight | Woven Medium Weight | Woven Heavy Weight |
|---|---|---|---|---|---|---|---|---|---|
| Stichabstand (fest) (mm) | 0,4 | 0,43 | 0,44 | 0,38 | 0,42 | 0,38 | 0,42 | 0,43 | 0,38 |
| Auto-Spacing-Faktor (Faktor) | 1 | 1,08 | 1,1 | 1 | 1,05 | 0,95 | 1,05 | 1,08 | 1,13 |
| Zugausgleich (mm) | 0,2 | 0,2 | 0,25 | 0,25 | 0,25 | 0,25 | 0,2 | 0,25 | 0,25 |
| Zickzack-Abstand (mm) | 2 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Stichabstand (mm) | 0,4 | 0,45 | 0,45 | 0,4 | 0,42 | 0,38 | 0,45 | 0,4 | 0,38 |
| Reihenversatz 2 (× Stichlänge) | – | 0,75 | 0,75 | 0,75 | 0,75 | 0,75 | 0,75 | 0,75 | 0,75 |
| Zugausgleich (mm) | 0,2 | 0,2 | 0,25 | 0,25 | 0,25 | 0,25 | 0,2 | 0,25 | 0,25 |
| Zickzack-Unterlage: Abstand (mm) | 5 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 |
| Doppel-Zickzack: Abstand (mm) | 7 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 |
| Trim ab (mm) | 3 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
