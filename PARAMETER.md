# Parameterkatalog

Stand 21.11.2026 · **156 Parameter**: 33 einfach, 77 experte, 46 automatisch. Erzeugt mit `stick parameter --markdown` aus `kern/stick-kern/daten/parameter.json` (Quelle: `Arbeitsdaten/werkzeuge/parameter_katalog.py`) – **nicht von Hand ändern**. Die Tests im Kern prüfen, dass jeder Wert unter `einstellungen` hier steht und die Standards stimmen.

**Modus:** *einfach* = im einfachen Modus sichtbar · *experte* = nur im Expertenmodus · *automatisch* = nicht anzeigen (fester oder abgeleiteter Wert, nur dokumentiert).  
**Ebene:** *design* = gilt für das ganze Design · *objekt* = am einzelnen Objekt · *beide* = Design-Standard, am Objekt überschreibbar · *werkzeug* = Bild → Umrisse.  
**JSON:** Pfad im Design-JSON bzw. `Objekt: feld` am Objekt. **Standard** = „0 Software Defaults“; *Stoff* = ändert sich mit dem Stoff (Tabelle am Ende). Bereiche mit „Vorschlag“ in der Quelle sind eigene Festlegungen.

## Design

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Rahmen** | Stickrahmen; legt das Stickfeld fest |  | `rahmen` | artista 130 x 100 |  |  | design | einfach | Rahmenliste der Software (Analyse/daten/hoops_table.md); Befehl rahmen_liste |
| **Stoff** | ändert Satin-/Step-Abstand, Zugausgleich, Unterlagen, Step-Versatz, Trim |  | `stoff` | 0 Software Defaults | 0 Software Defaults, Knit Light Weight, Knit Medium Weight, Knit Heavy Weight, Terry Medium Weight, Terry Heavy Weight, Woven Light Weight, Woven Medium Weight, Woven Heavy Weight |  | design | einfach | Stile in NORMAL.AMT (Analyse/code/10 §E) |
| **Garnkarte** | Farben werden dem nächsten Garn der Karte zugeordnet |  | `garnkarte` | Default |  |  | design | einfach | Garnkarten .tch; die Maschine nutzt Default.tch; Befehl garnkarten_liste |
| **Startpunkt** | wo der erste Stich beginnt |  | `start` | rahmenmitte | rahmenmitte, designmitte, unten_mitte, erster_stich |  | design | experte | Dialog „Start and End“ (Handbuch PDF-S. 205); Standard bis Gerätetest V7 eigene Festlegung |
| **Endpunkt** | wohin der Rahmen nach dem letzten Stich fährt |  | `ende` | rahmenmitte | rahmenmitte, designmitte, unten_mitte, start, letzter_stich |  | design | experte | Dialog „Start and End“; 34 von 51 Maschinendesigns enden auf 0,0 |

## Größe und Lage

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Größe** | skaliert um die Mitte (eine Zahl oder [x, y]) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `transform.skalieren` · Objekt: `transform.skalieren` | 100 % | 10 … 500 |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert; Bereich Vorschlag (eigene Festlegung) |
| **Drehen** | dreht um die Mitte, gegen den Uhrzeiger | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `transform.drehen` · Objekt: `transform.drehen` | 0 ° | -360 … 360 |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |
| **Spiegeln** | spiegelt um die Mitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `transform.spiegeln` · Objekt: `transform.spiegeln` | – | links_rechts, oben_unten, beide |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |
| **Schrägstellen** | verschiebt x mit der Höhe (Skew) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `transform.schraeg` · Objekt: `transform.schraeg` | 0 ° | -60 … 60 |  | beide | experte | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert; Bereich Vorschlag (eigene Festlegung) |
| **Verschieben** | Lage im Rahmen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `transform.verschieben` · Objekt: `transform.verschieben` | [0; 0] mm [x, y] |  |  | beide | einfach | Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert |

## Satin

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Auto Spacing** | Stichabstand automatisch nach der Säulenbreite (schmal = lockerer) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.auto_spacing` · Objekt: `einstellungen.satin.auto_spacing` | an |  |  | beide | experte | Effekt Auto Spacing 0x21, Tabelle nach Breite (Analyse/code/10, hoch); am Objekt aus, wenn „abstand“ gesetzt |
| **Stichabstand (fest)** | Weg zwischen zwei Einstichen derselben Seite, wenn Auto Spacing aus | satin | `einstellungen.satin.abstand_fest` · Objekt: `abstand` | 0,4 mm | 0,2 … 2 | ja | beide | experte | AMT 0x2723 (hoch); Bereich Vorschlag (eigene Festlegung) |
| **Auto-Spacing-Faktor** | Faktor auf die Tabelle (Stoff, Schrift 0,9) | satin | `einstellungen.satin.faktor` · Objekt: `faktor` | 1 Faktor | 0,5 … 2 | ja | beide | experte | AMT 0x2736 (hoch) |
| **Zugausgleich** | verlängert jeden Stich an beiden Enden („Fett“ = mehr Zugausgleich) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.zugausgleich` · Objekt: `einstellungen.satin.zugausgleich` | 0,2 mm | 0 … 1 | ja | beide | experte | Handbuch S. 199 (hoch); Stile 0x273d |
| **Zickzack-Abstand** | Abstand des Zickzack-Deckstichs (Stichart zickzack) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.zickzack_abstand` · Objekt: `einstellungen.satin.zickzack_abstand` | 2 mm | 0,5 … 6 | ja | beide | experte | Stichart Zigzag 0x2724/0x2725 (Stil-Liste 0x27a6, hoch) |
| **Kurzstiche** | kürzt Stiche an Innenkurven, damit es dort nicht zu dicht wird | satin, tatami (mit Winkellinien) | `einstellungen.satin.kurzstiche.an` · Objekt: `kurzstiche` | aus |  |  | beide | experte | Effekt 0x273a (MOD1 ShortStitchesEffect); Stil aus, Schrift an |
| **Kurzstiche: Schwelle** | ab welcher Dichte gekürzt wird (× Abstand) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.kurzstiche.schwelle` · Objekt: `einstellungen.satin.kurzstiche.schwelle` | 0,5 Faktor |  |  | beide | automatisch | 0x2819 (hoch) |
| **Kurzstiche: Zufall** | Kürzung zufällig ±0,04 verändert | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.kurzstiche.zufall` · Objekt: `einstellungen.satin.kurzstiche.zufall` | an |  |  | beide | automatisch | 0x281a (hoch) |
| **Kurzstiche: Gruppe** | höchstens so viele Stiche je Gruppe | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.kurzstiche.max_gruppe` · Objekt: `einstellungen.satin.kurzstiche.max_gruppe` | 5 Anzahl |  |  | beide | automatisch | 0x281b (hoch) |
| **Kurzstiche: Faktoren** | Kürzungsfaktoren als Dreieckstabelle | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.satin.kurzstiche.faktoren` · Objekt: `einstellungen.satin.kurzstiche.faktoren` | [0,8; 0,85; 0,72; 0,7; 0,9; 0,7; 0,7; 0,9; 0,8; 0,7; 0,7; 0,87; 0,65; 0,83; 0,7] Liste |  |  | beide | automatisch | 0x281c (hoch) |
| **Stichart Säule/Linie** | Satin (dicht) oder Zickzack (offen) | satin, satinlinie | Objekt: `stichart` | satin | satin, zickzack |  | objekt | experte | Stichartliste STR 2234/2236 |
| **Breite der Satinlinie** | Breite der Bahn um die Linie | satinlinie | Objekt: `breite` | 2 mm | 0,5 … 12 |  | objekt | einfach | Standard Vorschlag (eigene Festlegung) |
| **Lage der Satinlinie** | mittig um die Linie oder zur Seite (Linie = Kante) | satinlinie | Objekt: `versatz` | mitte | mitte, seite |  | objekt | experte | Dialog „Outline Stitch“ Center/Offset; Seite links Vorschlag (eigene Festlegung) |

## Tatami (Step)

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Stichabstand** | Weg zwischen zwei Hinreihen (Nachbarreihen halber Abstand) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.abstand` · Objekt: `einstellungen.tatami.abstand` | 0,4 mm | 0,2 … 2 | ja | beide | experte | AMT 0x2716 (hoch; Handbuch PDF-S. 74) |
| **Stichlänge** | Länge der Step-Stiche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.laenge` · Objekt: `einstellungen.tatami.laenge` | 4 mm | 1 … 12,7 |  | beide | experte | AMT 0x2717 (hoch) |
| **Mindeststich am Rand** | Einstiche näher am Rand entfallen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.min_rand` · Objekt: `einstellungen.tatami.min_rand` | 0,4 mm | 0,1 … 2 |  | beide | experte | AMT 0x2718 (hoch) |
| **Reihenversatz 1** | Verschiebung des Rasters (über Step-Muster gewählt) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.versatz` · Objekt: `einstellungen.tatami.versatz` | 0,25 × Stichlänge |  |  | beide | automatisch | AMT 0x2719 (hoch) |
| **Reihenversatz 2** | zweiter Versatz, abwechselnd mit dem ersten (null = wie Versatz 1) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.versatz2` · Objekt: `einstellungen.tatami.versatz2` | – |  | ja | beide | automatisch | AMT 0x271a (hoch); Stoffe 0,75 |
| **Zufallsversatz** | streut die Einstiche einer Reihe | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.zufall` · Objekt: `einstellungen.tatami.zufall` | 0 × Stichlänge |  |  | beide | automatisch | AMT 0x271b; Verteilung Vorschlag (eigene Festlegung) |
| **Stichwinkel** | Richtung der Stichreihen | tatami, muster, ttf | `einstellungen.tatami.winkel` · Objekt: `winkel` | 45 ° | 0 … 180 |  | beide | einfach | Handbuch Kap. 16 (Stitch Angle); Standard 45° Vorschlag (eigene Festlegung) |
| **Zugausgleich** | verlängert die Reihen an beiden Enden | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.zugausgleich` · Objekt: `einstellungen.tatami.zugausgleich` | 0,2 mm | 0 … 1 | ja | beide | experte | Handbuch S. 199 (hoch) |
| **Laufweg in der Fläche** | Stichlänge der Wege zwischen Teilbereichen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.laufweg_laenge` · Objekt: `einstellungen.tatami.laufweg_laenge` | 1,8 mm |  |  | beide | automatisch | Lauf 0x271c (hoch) |
| **Laufweg: Mindeststich** | variabler Lauf: kleinster Stich | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.laufweg.min_laenge` · Objekt: `einstellungen.tatami.laufweg.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **Laufweg: Sehnenabstand** | variabler Lauf: Abweichung von der Kurve | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.tatami.laufweg.sehnenabstand` · Objekt: `einstellungen.tatami.laufweg.sehnenabstand` | 0,089 mm |  |  | beide | automatisch | 0x2768 (hoch) |
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
| **Mindeststich** | kleinster Stich (variable Länge) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.lauf.min_laenge` · Objekt: `einstellungen.lauf.min_laenge` | 0,8 mm |  |  | beide | automatisch | AMT 0x27a8 (hoch) |
| **Variable Stichlänge** | in Kurven kürzere Stiche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.lauf.variabel` · Objekt: `einstellungen.lauf.variabel` | aus |  |  | beide | experte | Run-Dialog: feste Länge Standard (hoch) |
| **Sehnenabstand** | erlaubte Abweichung von der Kurve (variable Länge) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.lauf.sehnenabstand` · Objekt: `einstellungen.lauf.sehnenabstand` | 0,09 mm |  |  | beide | automatisch | Chord Gap (hoch) |
| **Eckwinkel** | ab diesem Knick Einstich an der Ecke (nur berechnete Linien) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.lauf.ecke_grad` · Objekt: `einstellungen.lauf.ecke_grad` | 30 ° |  |  | beide | automatisch | Vorschlag (eigene Festlegung) |

## Unterlage

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Unterlage automatisch** | Auto Underlay an/aus (Schrift: an) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.auto_an` · Objekt: `einstellungen.unterlage.auto_an` | aus |  |  | beide | einfach | Stil-Flags 0x2000 (Analyse_Teil3/05, mittel-hoch) |
| **Art der Auto-Unterlage** | vorgewählte Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.auto_art` · Objekt: `einstellungen.unterlage.auto_art` | [zickzack] | mittellauf, edge_walk, zickzack, doppelzickzack |  | beide | experte | Stil-Bit 0x0001 = Zickzack |
| **Unterlage der Säule/Schrift** | auto, keine oder Arten (z. B. edge_walk+zickzack) | satin, satinlinie, text | Objekt: `unterlage` | auto | auto, keine, mittellauf, edge_walk, zickzack, doppelzickzack |  | objekt | experte | Handbuch Kap. 22 |
| **Abstand zum Rand** | Unterlage bleibt so weit innen | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.rand` · Objekt: `einstellungen.unterlage.rand` | 0,2 mm | 0 … 2 |  | beide | experte | AMT 0x2769/0x276a (mittel) |
| **Zugausgleich der Unterlage** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.zugausgleich` · Objekt: `einstellungen.unterlage.zugausgleich` | 0,2 mm |  |  | beide | experte | AMT 0x2743 (hoch) |
| **Edge Walk: Stichlänge** | Lauf am Rand der Säule | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.edge_walk_laenge` · Objekt: `einstellungen.unterlage.edge_walk_laenge` | 2,2 mm |  |  | beide | experte | AMT 0x2747 (hoch) |
| **Mittellauf: Stichlänge** | Lauf auf der Säulenmitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.mittellauf_laenge` · Objekt: `einstellungen.unterlage.mittellauf_laenge` | 2,2 mm |  |  | beide | experte | AMT 0x2746 (hoch) |
| **Zickzack-Unterlage: Abstand** | Weg zwischen Einstichen derselben Seite | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.zickzack_laenge` · Objekt: `einstellungen.unterlage.zickzack_laenge` | 5 mm |  | ja | beide | experte | AMT 0x2744/0x2725 (mittel) |
| **Doppel-Zickzack: Abstand** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.doppelzickzack_laenge` · Objekt: `einstellungen.unterlage.doppelzickzack_laenge` | 7 mm |  | ja | beide | experte | AMT 0x2745/0x2725 (mittel) |
| **Zickzack: Rücklauf** | Lauf zurück zur Säulenmitte | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.rueck_lauf_laenge` · Objekt: `einstellungen.unterlage.rueck_lauf_laenge` | 3 mm |  |  | beide | automatisch | AMT 0x2744/0x271d (mittel) |
| **Step-Unterlage: Abstand** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_abstand` · Objekt: `einstellungen.unterlage.tatami_abstand` | 3 mm |  |  | beide | experte | AMT 0x275a (mittel) |
| **Step-Unterlage: Stichlänge** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_laenge` · Objekt: `einstellungen.unterlage.tatami_laenge` | 4 mm |  |  | beide | experte | AMT 0x275a (mittel) |
| **Step-Unterlage: Randlauf** | Stichlänge des Laufs um die Fläche | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_randlauf` · Objekt: `einstellungen.unterlage.tatami_randlauf` | 2,5 mm |  |  | beide | automatisch | AMT 0x275a (mittel) |
| **Step-Unterlage: Winkel** | gegen die Deckrichtung gedreht | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_winkel_versatz` · Objekt: `einstellungen.unterlage.tatami_winkel_versatz` | 90 ° |  |  | beide | automatisch | AMT 0x2759 |
| **edge_walk_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.edge_walk_lauf.min_laenge` · Objekt: `einstellungen.unterlage.edge_walk_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **edge_walk_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.edge_walk_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.edge_walk_lauf.sehnenabstand` | 0,072 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **mittellauf_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.mittellauf_lauf.min_laenge` · Objekt: `einstellungen.unterlage.mittellauf_lauf.min_laenge` | 0,728 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **mittellauf_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.mittellauf_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.mittellauf_lauf.sehnenabstand` | 0,061 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **zickzack_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.zickzack_lauf.min_laenge` · Objekt: `einstellungen.unterlage.zickzack_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **zickzack_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.zickzack_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.zickzack_lauf.sehnenabstand` | 0,2 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **doppelzickzack_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.doppelzickzack_lauf.min_laenge` · Objekt: `einstellungen.unterlage.doppelzickzack_lauf.min_laenge` | 0,994 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **doppelzickzack_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.doppelzickzack_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.doppelzickzack_lauf.sehnenabstand` | 0,089 mm |  |  | beide | automatisch | 0x2768 (hoch) |
| **tatami_randlauf_lauf: Mindeststich** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_randlauf_lauf.min_laenge` · Objekt: `einstellungen.unterlage.tatami_randlauf_lauf.min_laenge` | 0,4 mm |  |  | beide | automatisch | 0x2767 (hoch) |
| **tatami_randlauf_lauf: Sehnenabstand** | variabler Lauf der Unterlage | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.unterlage.tatami_randlauf_lauf.sehnenabstand` · Objekt: `einstellungen.unterlage.tatami_randlauf_lauf.sehnenabstand` | 0,072 mm |  |  | beide | automatisch | 0x2768 (hoch) |

## Verbinder und Vernähen

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Immer vernähen (Anfang)** | Tie In immer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.anfang_immer` · Objekt: `einstellungen.verbinder.anfang_immer` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2753 |
| **Vernähen nach Trim/Farbwechsel** | Tie In If After Trim | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.anfang_nach_trim_farbwechsel` · Objekt: `einstellungen.verbinder.anfang_nach_trim_farbwechsel` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2750 |
| **Vernähen nach langem Verbinder** | Tie In If Previous Connector Longer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.anfang_bei_langem_verbinder` · Objekt: `einstellungen.verbinder.anfang_bei_langem_verbinder` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272d |
| **… Verbinder länger als** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.anfang_verbinder_ab` · Objekt: `einstellungen.verbinder.anfang_verbinder_ab` | 3 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2755 |
| **Immer vernähen (Ende)** | Tie Off immer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.ende_immer` · Objekt: `einstellungen.verbinder.ende_immer` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2752 |
| **Vernähen vor Trim/Farbwechsel** | Tie Off If Before Trim | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.ende_vor_trim_farbwechsel` · Objekt: `einstellungen.verbinder.ende_vor_trim_farbwechsel` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2733 |
| **Vernähen vor langem Verbinder** | Tie Off If Next Connector Longer | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.ende_bei_langem_verbinder` · Objekt: `einstellungen.verbinder.ende_bei_langem_verbinder` | an |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272c |
| **… Verbinder länger als** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.ende_verbinder_ab` · Objekt: `einstellungen.verbinder.ende_verbinder_ab` | 3 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2754 |
| **Letztes Objekt vernähen** | Always Tie Off Last Object | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.letztes_objekt_vernaehen` · Objekt: `einstellungen.verbinder.letztes_objekt_vernaehen` | aus |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x276c |
| **Trim ab** | ab dieser Verbinderlänge gilt ein Sprung als Trim (die 185 schneidet nicht) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.trim_ab` · Objekt: `einstellungen.verbinder.trim_ab` | 3 mm |  | ja | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272e |
| **Länge der Vernähstiche** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.vernaeh_stichlaenge` · Objekt: `einstellungen.verbinder.vernaeh_stichlaenge` | 1 mm |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x2730/0x2731 |
| **Zahl der Vernähstiche** |  | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | `einstellungen.verbinder.vernaeh_stiche` · Objekt: `einstellungen.verbinder.vernaeh_stiche` | 2 Anzahl |  |  | beide | experte | Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch); 0x272f/0x275d |

## Auto Jump und Maschine

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Auto Jump** | lange Stiche als Sprünge statt geteilt (Standard an, Schrift aus) | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `auto_jump` | an |  |  | objekt | experte | Effects-Dialog; AMT 0x2758 (hoch) |
| **Auto Jump: Höchststich** | darüber Sprünge; klein (z. B. 6 mm) = Satin mit mehr Loft | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `auto_jump.max` | 12,7 mm | 0,3 … 12,7 |  | objekt | experte | AMT 0x2711 (hoch) |
| **Auto Jump: Sprunglänge** | Länge der Teilsprünge | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `auto_jump.sprung` | 7 mm | 0,3 … 12,7 |  | objekt | experte | AMT 0x2712 (hoch) |
| **Kürzester Stich** | kürzere Stiche werden entfernt |  |  | 0,3 mm |  |  | design | automatisch | DEFAULTS.DFL (hoch) |
| **Längster Stich** | Maschinengrenze artista 185 |  |  | 12,7 mm |  |  | design | automatisch | DEFAULTS.DFL (hoch) |

## Bearbeiten

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Verstärken (Double Run)** | gleiche Stiche rückwärts noch einmal | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `doppellauf` | aus |  |  | objekt | experte | Handbuch Kap. 23 |
| **Aussparen (Knockout)** | Fläche spart die genannten Objekte aus | tatami, muster | Objekt: `aussparen` | [] Objektnamen |  |  | objekt | einfach | Remove Overlay (Analyse_Teil3/14) |
| **Überlappung beim Aussparen** | Loch bleibt um so viel kleiner | tatami, muster | Objekt: `ueberlappung` | 0 mm | 0 … 3 |  | objekt | experte | Analyse_Teil3/14 |

## Reihenfolge

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Kürzeste Verbindung (Closest Join)** | jedes Objekt ohne eigenen Einstich beginnt am Punkt nächst dem vorigen Ende (Linien umgedreht, geschlossene Formen gedreht; Schrift ausgenommen) |  | `kuerzeste_verbindung` | aus |  |  | design | experte | MOD1 ResequenceByClosestJoin 0x1006d0c0, GEOMETRY FindExitEntry (hoch); im Original Option von Resequence und Auto Digitizer, sonst „wie digitalisiert“ (Messung Analyse_Teil3/16_reihenfolge.md) |
| **Einstich (entry point)** | Objekt beginnt hier bzw. am nächsten Punkt der Form | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `einstich` | – |  |  | objekt | experte | Handbuch Kap. 7/14 (entry point, Reshape); Umsetzung je Objektart eigene Festlegung (Analyse_Teil3/16_reihenfolge.md) |
| **Ausstich (exit point)** | Objekt endet hier bzw. so nah wie die Form erlaubt | lauf, dreifach, satin, satinlinie, satinflaeche, tatami, muster, blanket, applikation, text, ttf, monogramm | Objekt: `ausstich` | – |  |  | objekt | experte | Handbuch Kap. 7/14 (exit point); Flächen: Beginn gegenüber, eigene Festlegung (Analyse_Teil3/16_reihenfolge.md) |
| **Farbreihenfolge beim Sortieren** | gewünschte Folge der Farben (Move Up/Down), übrige Farben in der Reihenfolge ihres ersten Auftretens |  |  | [] Farbliste |  |  | werkzeug | experte | Dialog „Sequence By Color“ (Handbuch Kap. 14); Befehl reihenfolge --nach-farbe |
| **Überdeckung beachten** | beim Sortieren nach Farbe behalten Objekte, die sich überdecken, ihre Reihenfolge |  |  | aus |  |  | werkzeug | einfach | eigene Erweiterung (das Original prüft nicht, Analyse_Teil3/16_reihenfolge.md) |
| **Überdeckung ab** | kleinere gemeinsame Flächen zählen nicht |  |  | 0,5 mm² |  |  | werkzeug | automatisch | eigene Festlegung |
| **Breite einer Linie bei der Prüfung** | Laufstich zählt als Band dieser Breite |  |  | 0,5 mm |  |  | werkzeug | automatisch | eigene Festlegung |

## Blanket

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Zahnabstand** |  | blanket | Objekt: `abstand` | 2 mm | 0,5 … 10 |  | objekt | experte | Stil 0x2727/0x2728 |
| **Zahnbreite** |  | blanket | Objekt: `breite` | 3 mm | 0,5 … 12 |  | objekt | einfach | Stil 0x2729 |
| **Lage** | mittig oder zur Seite | blanket | Objekt: `versatz` | mitte | mitte, seite |  | objekt | experte | Dialog Center/Offset |

## Applikation

| Parameter | Wirkung | gilt für | JSON | Standard | Bereich / Auswahl | Stoff | Ebene | Modus | Quelle |
|---|---|---|---|---|---|---|---|---|---|
| **Abdeckung** | Satin oder Blanket | applikation | Objekt: `abdeckung` | satin | satin, blanket |  | objekt | einfach | Easy Appliqué (Analyse_Teil3/12) |
| **Stoff zuschneiden** | vorgeschnitten oder im Rahmen zuschneiden (mit Schnittlinie) | applikation | Objekt: `stil` | vorgeschnitten | vorgeschnitten, zuschneiden |  | objekt | einfach | Easy Appliqué (Analyse_Teil3/12) |
| **Breite der Abdeckung** |  | applikation | Objekt: `breite` | 2,5 mm | 1 … 8 |  | objekt | experte | Easy Appliqué (Analyse_Teil3/12) |
| **Abstand der Abdeckung** | fester Satinabstand (ohne Angabe: Stil) | applikation | Objekt: `abstand` | – |  |  | objekt | experte | Easy Appliqué (Analyse_Teil3/12) |
| **Heftstich: Breite** |  | applikation | Objekt: `heft_breite` | 3 mm |  |  | objekt | experte | Easy Appliqué (Analyse_Teil3/12) |
| **Heftstich: Abstand** |  | applikation | Objekt: `heft_abstand` | 2 mm |  |  | objekt | experte | Easy Appliqué (Analyse_Teil3/12) |
| **Platzierungslinie: Stichlänge** |  | applikation | Objekt: `platz_laenge` | 2 mm |  |  | objekt | experte | Easy Appliqué (Analyse_Teil3/12) |

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
| **Stichart der Schrift** | Satin (Alphabet), Step oder Füllmuster; Computer-Schrift: Tatami, Lauf, Dreifach oder Satin (Säulen wie die TrueType-Umwandlung) | text, ttf | Objekt: `stichart` | satin | satin, step, muster, tatami, lauf, dreifach |  | objekt | experte | Handbuch Kap. 20; Verteilung Vorschlag (eigene Festlegung); ttf satin: EZSTITCH Turning (Analyse_Teil3/17) |
| **Verzerren (Elastic)** | gerade (Wimpel) oder gebogen (Bogen), Griffe oben/unten | text, ttf, monogramm | Objekt: `elastisch` | – |  |  | objekt | einfach | Handbuch Kap. 21; Kantenform Vorschlag (eigene Festlegung) |
| **Initialen** | 2 oder 3 Buchstaben | monogramm | Objekt: `initialen` | – |  |  | objekt | einfach | Diamond2/Diamond3 (Analyse_Teil3/09 §7) |
| **Monogramm-Rahmen** | Raute, Zier oder keiner | monogramm | Objekt: `rahmen` | keine | raute, zier, keine |  | objekt | einfach | Analyse_Teil3/09 §7 |
| **Rahmenfarbe** |  | monogramm | Objekt: `rahmen_farbe` | – |  |  | objekt | einfach | Farbwechsel ^ |
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
| **Farben** | Zahl der Farben nach der Reduktion |  |  | 6 Anzahl |  |  | werkzeug | einfach | stick-vektor (eigene Festlegung) |
| **Kleinste Fläche** | kleinere Flächen entfallen |  |  | 4 mm² |  |  | werkzeug | experte | stick-vektor (eigene Festlegung) |
| **Glättung** | Toleranz der Vereinfachung |  |  | 0,2 mm |  |  | werkzeug | experte | stick-vektor (eigene Festlegung) |
| **Hintergrund weglassen** | häufigste Randfarbe entfällt |  |  | an |  |  | werkzeug | experte | stick-vektor (eigene Festlegung) |

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
