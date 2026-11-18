# Analyse: BERNINA artista Designer V4 (Installations-CD)

**Stand:** 17.11.2026 · **Basis:** Software von Wilcom Pty (Sydney) für Fritz Gegauf AG/BERNINA, Copyright 1997–2003, Version 4 (BLOT.EXE 4.0.18, ADX 0.0.8.6, Handbuch „Version 4“)

## Wie ich vorgegangen bin
1. CD entpackt: Das InstallShield-3-Archiv und die PKWARE-Kompression habe ich selbst nachgebaut ([01](code/01_setup_entpacken.md)), ADX per `unshield`. Ergebnis: 651 + 16 Dateien.
2. Aus allen 92 Programmdateien Texte (ASCII/UTF-16), String-Tabellen, Dialoge und Menüs extrahiert.
3. Handbuch (522 Seiten) und ADX-Handbuch als Text gelesen.
4. Kernfunktionen disassembliert und in Python/C nachgebaut: serielles Protokoll, DFL-Entschlüsselung, ART/AMT-Entschlüsselung (Twofish + Substitution), EXP-Leser.
5. Die Werksvorlage `NORMAL.AMT` vollständig entschlüsselt und ausgewertet: alle Standardwerte, 8 Stoffe, EasyStitch- und Auto-Spacing-Tabellen.

**Nicht möglich war:** die Software zu starten. Sie braucht Windows und den HASP-Dongle. Deshalb konnte ich kein EXP „speichern und vermessen“; das Format ist stattdessen aus dem Code beschrieben.

> ⚠️ **Die vorliegende CD-Kopie ist beschädigt.** 14 Blöcke à 64 KB in `setup.z` sind Duplikate des jeweils vorigen Blocks. Unlesbar sind dadurch die **Online-Hilfe `ARTISTA.HLP`**, `qrg_enu.pdf`, `Quilter.hlp` und Teile von Portfolio. Das Handbuch-PDF war trotzdem vollständig als Text lesbar. Details: [01](code/01_setup_entpacken.md).

### Sicherheitsstufen
- **hoch:** direkt im Code oder in Daten nachgewiesen und durch Prüfsumme oder Länge bestätigt
- **mittel:** aus Struktur und Werten geschlossen und mit dem Handbuch abgeglichen
- **niedrig:** Vermutung

---

## 1. Unterstützte Maschinen und Übertragungswege — Sicherheit: hoch

| Maschine/Gerät | Weg | Protokoll/Details | Fundstelle |
|---|---|---|---|
| **artista 200** | **USB-Kabel + Microsoft ActiveSync** (die artista 200 ist ein Windows-CE-Gerät) | RAPI, eigener Server `AdxCe.dll` wird nach `\Windows\` kopiert; XML-Antworten | [03](code/03_artista200_adx.md); ADX-Handbuch Kap. 2.2 |
| **artista 165, 170, 180, 185** | **serielles RS-232-Kabel**, COM1–COM4 | **19200 Baud 8N1** (umschaltbar 57600/115200), ASCII-Protokoll mit Echo oder Binärrahmen STX…ETX+Prüfsumme | [02](code/02_serielles_protokoll.md); ADX-Handbuch 2.3 |
| **Embroidery Card Reader/Writer Box („R/W box“)** | seriell wie 165–185 | Design-Karte (128-KB-Abbild) | [02](code/02_serielles_protokoll.md) |
| **Personal Design Card** | in der artista 200 (`\Storage Card\`) oder in der R/W-Box | | ADX-Handbuch |
| **CD-ROM** an der artista 200 | `\USBCdrom\`, verschlüsselte Design-CDs `\USBCdromEncrypted\` | | adx.dll |
| **Magic Box** (seriell, USB nach Update) | Fremdkarten (Brother, Janome, Singer, Toyota …) lesen, schreiben nur auf **Magic Cards** (4 Typen/Slots) | | Handbuch Kap. 30 |
| Diskette/Festplatte | Speichern in Fremdformaten | | Handbuch Kap. 28/29 |
| **artista 730 / 630 / 640** | **nicht unterstützt/erwähnt**; diese Version ist älter | – | keine Fundstelle |

Im Dialog „Select Embroidery System“ stehen genau zwei Einträge: *„artista 200“* und *„artista 165 to 185 or R/W box“* (adx.dll, Ressource). Interne Maschinennummer für BERNINA: **17**.

## 2. artista 200: ART oder EXP? Dateien, Ordner, Namen — Sicherheit: hoch (Pfade/Strings), mittel (Bewertung)

- **Beide Formate werden übertragen.** ADX schreibt: „Verify that your ART and EXP designs are saved there.“ Die Thumbnails zeigen „Exp“ oder „Art“. **ART ist das Standardformat** („Embroider“/„Download“ sendet das geöffnete ART-Design; Farben, Vorschau und Infos liest ADX aus der ART-Datei). EXP ist erlaubt, aber eingeschränkt: „**Designs in EXP format are limited to single operation only.**“
- **Ordner auf der artista 200:**
  - `\IPSM\ANS\Data\Embroidery\` (intern), darin `Motifs\`, `Alphabets\`, `My Designs`
  - **`\EMBFiles\` für EXP** (Beispiel im Code: `\EmbFiles\10.exp`)
  - `\Storage Card\` = Personal Design Card
- **Dateinamen:** Formatstrings `%s.exp` (Designname) und `M%d.exp` (nummeriert). Höchstens 6 eigene Ordner. Die Grundordner lassen sich weder umbenennen noch löschen.
- **INF + BMP und Ordner `EmbF5`:** Diese gibt es **in dieser Software nicht**. Weder Code noch Strings noch Handbuch kennen sie. Sie gehören zu späteren BERNINA-Generationen mit USB-Stick (artista 630/640/730). Für sie wäre eine neuere Softwareversion zu untersuchen.

## 3. Aufbau INF/BMP; EXP-Format — Sicherheit: hoch für EXP-Lesen

- **INF:** existiert in V4 nicht (siehe oben).
- **BMP-Vorschau:** Bei der artista 200 liefert die Maschine das Vorschaubild über `GetARTFileThumbNail` als `\temp\icon.bmp`. Für Karten (165–185) erzeugt BEAGLink ein **1-Bit-Icon** mit ((Breite+7)/8)·Höhe + 10 Byte.
- **EXP** = Melco Expanded: kein Kopf, 2-Byte-Datensätze `dx dy` (signed, 0,1 mm, +y nach oben), Escape `0x80`: ungerade 1–15 = Farbwechsel/Stopp, gerade = Sprung, `0x82` = langer Sprung mit 16-Bit-Koordinaten. Größter Schritt 12,7 mm. **EXP schreiben ist laut Handbuch eine kostenpflichtige Option.** → [07_exp_format.md](code/07_exp_format.md)

## 4. Original-Stickrahmen — Sicherheit: hoch
Quelle: `RES/hooplibr.dsc` ([08](code/08_rahmen_dsc.md), vollständige Tabelle [daten/hoops_table.md](daten/hoops_table.md)). Maße = Stickfeld.

| Rahmen (Name in der Software) | Stickfeld B × H | Form des Stickfelds | Rahmen außen ca. |
|---|---|---|---|
| **artista 255 x 145 Oval** (großer Ovalrahmen) | **145 × 255 mm** | „Oval“: gerade Seiten bis ±60 mm, Schrägen bis (±65, ±90), Bogen bis ±127,5 | 222 × 326 mm |
| **artista 200 x 155** | **155 × 200 mm** | rechteckig | 216 × 283 mm |
| **artista 130 x 100** (mittlerer Rahmen) | **100 × 130 mm** | rechteckig | 179 × 203 mm |
| **artista 50 x 72 Free Arm** (kleiner Freiarmrahmen) | **72 × 50 mm** | rechteckig, eine Seite gerundet | 182 × 92 mm |
| **artista 50 x 72 Normal** (kleiner Rahmen) | **72 × 50 mm** | rechteckig, eine Seite gerundet | 185 × 89 mm |
| **artista 40 x 60** | **60 × 40 mm** | rechteckig | 124 × 88 mm |
| **artista 400 x 150 MEGA** | **150 × 400 mm** | rechteckig | 217 × 500 mm |
| **400 x 155 Giant artista** | **155 × 400 mm** | rechteckig | – |

Zu jedem gibt es Varianten „Rotated“ (90° gedreht), „w/Grid“ und „w/X-Y Lines“. Dazu Fremdrahmen (Deco, Brother, Pfaff, Janome, Viking, Singer). Eigene Rahmen (Rechteck, Oval, Sliding, 3-Positionen) werden in `EUSRHOOPS.DSC` gespeichert. Beim Senden muss der Rahmen wieder in Normallage stehen (Handbuch S. 51).

## 5. Standardwerte der Sticharten — Sicherheit: hoch (Zahlen), mittel (einzelne Zuordnungen)
Quelle: entschlüsselte `NORMAL.AMT`, Stil „**0 Software Defaults**“, plus DEFAULTS.DFL und Handbuch. Ausführlich: [10](code/10_algorithmen_schmale_bahnen.md), Tag-Bedeutungen: [06](code/06_dataitem_und_tags.md).

| Wert | Standard | Quelle |
|---|---|---|
| **Satin-Stichabstand** (fest) | **0,40 mm** | AMT 0x2723 |
| **Satin Auto Spacing** | Tabelle: 0,51 mm bei 0,5 mm Breite … 0,40 mm bei 4 mm … 0,34 mm bei 7–8 mm | AMT 0x2735 |
| **Tatami/Step-Reihenabstand** | **0,40 mm** | AMT 0x2716 |
| **Tatami/Step-Stichlänge** | **4,0 mm** | AMT 0x2717 |
| Tatami-Mindeststich am Rand / Reihenversatz | 0,40 mm / 0,25 | AMT 0x2718/0x2719 |
| **Geradstich (Kontur)** | **2,5 mm** | AMT 0x27a8/0x271d |
| Geradstich-Wege in Flächen | 1,8 mm (variabel, min. 0,4 mm) | AMT |
| **Dreifachstich** | kein eigener Standardwert gefunden. Der Dialog „Triple Run Length“ (RESENU Dialog 1026) benutzt dasselbe Eingabefeld wie „Run Length“, die Länge folgt also der Geradstich-Einstellung. *Sicherheit: mittel* | RESENU Dialog 1026 |
| **Zugausgleich** | **0,20 mm** (Handbuch: „automatically set at 0.2 mm“; bei Auto-Digitizing 0,4 mm) | AMT 0x2743, Handbuch S. 201 |
| **Unterlagen** | Edge Walk 2,2 mm; Zickzack 5,0/3,0 mm; Doppel-Zickzack 7,0/3,0 mm; Tatami-Unterlage 3,0 mm Abstand / 4,0 mm Länge; Unterlagenrand 0,2 mm; Winkel 90° | AMT 0x27a7 |
| **Minimale Stichlänge** (Maschinenformat Bernina) | **0,3 mm** | DEFAULTS.DFL |
| **Maximale Stichlänge** | **12,7 mm** | DEFAULTS.DFL, AMT 0x2711 |
| **Auto Jump** | Stiche > 12,7 mm werden in Sprünge zu 7,0 mm geteilt; Standard ein (bei Schrift aus) | AMT 0x2711/2712/2758, Handbuch S. 203 |
| Satin-Auto-Split | 10 mm (vermutlich) | AMT 0x2722 |
| Verbinder | Trim ab 3,0 mm; Vernähen, wenn nächster/vorheriger Verbinder > 3,0 mm | AMT 0x272a |
| Fancy-Füllung | 0,40 mm / 4,0 mm | AMT 0x278c |

Handbuch-Richtwerte (S. 201): Zugausgleich Drillich/Baumwolle 0,1 · T-Shirt 0,2 · Fleece/Strick 0,25–0,3 · Schrift 0,2–0,3 mm.

## 6. Stoffeinstellungen — Sicherheit: hoch
Dialog *Settings → Fabric Settings*: „Fabric Type“ (Liste), „Use Standard Settings“ / „Adjust Settings To … %“, „Apply to Lettering Objects“. Die **Stoffliste** steht in NORMAL.AMT:

**1 Knit Light Weight · 2 Knit Medium Weight · 3 Knit Heavy Weight · 4 Terry Medium Weight · 5 Terry Heavy Weight · 6 Woven Light Weight · 7 Woven Medium Weight · 8 Woven Heavy Weight**

| Stoff | Satin-Abstand | Tatami-Abstand | Auto-Spacing-Faktor | Zugausgleich |
|---|---|---|---|---|
| Knit Light | 0,43 | 0,45 | 1,08 | 0,20 |
| Knit Medium | 0,44 | 0,45 | 1,10 | 0,25 |
| Knit Heavy | 0,38 | 0,40 | 1,00 | 0,25 |
| Terry Medium | 0,42 | 0,42 | 1,05 | 0,25 |
| Terry Heavy | 0,38 | 0,38 | 0,95 | 0,25 |
| Woven Light | 0,42 | 0,45 | 1,05 | 0,20 |
| Woven Medium | 0,43 | 0,40 | 1,08 | 0,25 |
| Woven Heavy | 0,38 | 0,38 | 1,13 | 0,25 |

Für **alle** Stoffe gilt zusätzlich, abweichend von den Software Defaults: Trim ab 6 mm, Zickzack-Unterlagen 3,0/3,0 mm, Tatami-Reihenversatz 0,25/0,75. Sonst ändert sich nichts (automatischer Vergleich aller rund 695 Felder). Die Stoffbilder in `PROGRAM/fabrics/*.bmp` dienen nur als Bildschirmhintergrund.

## 7. Garnfarbkarten — Sicherheit: hoch
32 CSV-Dateien `PROGRAM/RES/*.tch`, Format `Nummer,Marke,Name,A,R,G,B` → [09](code/09_garnkarten_tch.md), alle Dateien unter [daten/garnkarten/](daten/garnkarten/).
Marken: **Isacord 40, Madeira 40 (+ Neon/Poly Neon), Mettler Poly Sheen / Silky Sheen / Cotton Silk Finish, Sulky, Robison-Anton** (Super Brite Poly, Super Strength Rayon, Quilting), **DMC**, außerdem die Format-Paletten von CSD/HUS/PCS/PES/SEW/XXX. ⚠️ Die Dateien „Mettler 40 wt“ enthalten in Wahrheit Isacord-Daten.

## 8. Kleine, filigrane Stickerei — Sicherheit: hoch (Daten), mittel (Ableitung)
- **Satin-Mindestbreite:** Es gibt keine harte Grenze. Die **EasyStitch-Automatik** setzt Satin nur bei Objekten von **über 2 mm bis 8 mm Breite** ein. Darunter wird eine Linie erzeugt (≤ 0,3 mm einfacher Lauf, ≤ 2 mm kräftige Linie), darüber Tatami. Die Auto-Spacing-Tabelle reicht hinunter bis 0,5 mm Breite.
- **Schmale Säulen werden automatisch lockerer:** 0,51 mm Abstand bei 0,5 mm Breite, 0,49 mm bei 1 mm, 0,46 mm bei 2 mm (Handbuch: zu viele Einstiche beschädigen den Stoff).
- **Kleine Schrift:** Auto-Unterlage abschalten (Handbuch S. 186, 379). Schrift wird mit Faktor 0,9 dichter gesetzt, ohne Auto Jump. Kleinste empfohlene Höhen: **Quilting Text, Roman Small 4–6 mm**; Prestige 5 mm; Chicago, Drifter, Fortress, Nightowl, Swiss Block, Varsity Block, Zurich 6 mm. Vollständige Tabelle für 50 Alphabete: [daten/alphabets.md](daten/alphabets.md).
- Geradstich an engen Kurven auf 1,8 mm verkürzen (S. 68). Kürzester Maschinenstich 0,3 mm.
- **60er Garn / kleine Nadeln:** In der Software gibt es **keinerlei Angaben** dazu. Alle Garnkarten sind 40er bzw. Quiltgarne.

---

## Weitere Erkenntnisse
- **Eigenes Format ART:** OLE-Datei; `Contents` mit Substitutionschiffre (Format-Key 0xD0), `DesignDocument` mit Twofish-128-CFB1. Der Schlüssel ist fest im Programm: `"1d43f2ae%08x%08x%08x"`, IV `a885f60d…`. An 40 Beispiel-ARTs geprüft → [05](code/05_art_amt_verschluesselung.md). Grade A–D je nach Herkunft (Handbuch S. 255–259).
- **Unterstützte Formate:** lesen und schreiben PES/PEC, HUS/SHV, SEW/JEF, PCS/PCD/PCQ, CSD, XXX; EXP und DST schreiben nur als Kostenoption; ARX (Kreuzstich), ARQ (Quilter); Bilder BMP/JPG/PNG/PCX, Vektor EMF/WMF (Handbuch Anhang E). Beim PES-Speichern: 100×100 und 180×130 als PES V1, 260×160 und benutzerdefiniert als PES V4.
- **Programmdateien** mit Funktion: [daten/programmdateien.md](daten/programmdateien.md). Wichtig: `BLOT.EXE` (Hauptprogramm), `MOD1.DLL` (Designmodell, Maschinen-Kodierung), `IMPORT.DLL` (Dateiformate, Verschlüsselung), `DEFMGR.DLL` (Voreinstellungen), `WESSYS.DLL` (Basis, DataItem), `DIALOG.DLL` (Dialoge), `RESENU.DLL` (englische Texte), `PRODUCT.DLL` (Produktstufen, Format-Schlüssel), `artlink.dll`/`BEAGLink.dll` (serielle Maschinenverbindung), `adx.dll`/`AdxCe.dll` (ADX), `MagicBox.exe`, `QUILTER.EXE`, HASP-Dongle-Treiber.
- **Produktstufen:** Editor, Auto Designer, Designer Plus; Freischaltung über Dongle und Access-Codes (Handbuch S. 1–2, 28–30).

## Index aller Dateien in diesem Ordner
| Datei | Inhalt |
|---|---|
| [code/01_setup_entpacken.md](code/01_setup_entpacken.md) | CD-Aufbau, InstallShield-3-Format, PKWARE-Explode, Schadensbefund |
| [code/02_serielles_protokoll.md](code/02_serielles_protokoll.md) | RS-232: Port-Parameter, ASCII- und Binärprotokoll, Befehle, Kartenabbild |
| [code/03_artista200_adx.md](code/03_artista200_adx.md) | artista 200: ActiveSync/RAPI, Pfade, Dateinamen, XML, Ablauf |
| [code/04_dfl_verschluesselung.md](code/04_dfl_verschluesselung.md) | DEFAULTS.DFL: XOR-Schlüssel, zlib, Maschinenformat „Bernina“ |
| [code/05_art_amt_verschluesselung.md](code/05_art_amt_verschluesselung.md) | ART/AMT/ESB: Format-Schlüssel, Substitution, Twofish-CFB1 |
| [code/06_dataitem_und_tags.md](code/06_dataitem_und_tags.md) | DataItem-Binärformat, Einheit 1/180 mm, Tag-Tabelle |
| [code/07_exp_format.md](code/07_exp_format.md) | EXP-Datensätze und Steuercodes |
| [code/08_rahmen_dsc.md](code/08_rahmen_dsc.md) | Rahmenbibliothek-Format und Ovalform |
| [code/09_garnkarten_tch.md](code/09_garnkarten_tch.md) | Garnkarten-Format und Inhalt |
| [code/10_algorithmen_schmale_bahnen.md](code/10_algorithmen_schmale_bahnen.md) | **Auto Spacing, EasyStitch, Stoffe, Werksvorgaben, filigrane Stickerei** |
| [code/11_werkzeuge.md](code/11_werkzeuge.md) | alle Skripte und ihre Benutzung |
| `code/*.py`, `code/*.c`, `code/tfq.h` | Quellcode der Werkzeuge |
| [daten/hoops_table.md](daten/hoops_table.md), [daten/hooplibr.dsc](daten/hooplibr.dsc) | Rahmen |
| [daten/alphabets.md](daten/alphabets.md) | Mindest-/Höchsthöhe aller 50 Alphabete |
| [daten/garnkarten/](daten/garnkarten/) | 32 Garnkarten + Übersicht |
| [daten/programmdateien.md](daten/programmdateien.md) | Inventar der Programm-DLL/EXE |
| `daten/entschluesselt/DEFAULTS.DFL.bin`, `defaults_tree_named.txt` | entschlüsseltes DEFAULTS.DFL als Baum |
| `daten/entschluesselt/NORMAL.AMT.*.bin`, `normal_amt_dataitems_named.txt`, `styles.txt` | entschlüsselte Vorlage, alle 22 Stile |
| `daten/entschluesselt/style_keyvalues_all.txt`, `software_defaults_flat.txt`, `fabrics_diff.txt`, `easystitch.txt` | ausgewertete Werte |
| `daten/entschluesselt/tagnames.tsv`, `tag_labels.txt` | Tag-Namen, Tag → Dialogfeld |
| `daten/handbuch/` | Handbuch als Text (522 S.), ADX-Handbuch, alle Strings und Dialoge aus RESENU.DLL |

## Fortsetzung
Die Feinanalyse der seriellen Kommunikation (Erkennung, Adressen und Reihenfolge beim Schreiben, Kartenfelder, Verschleierung) und die Anleitung zum Disassembler stehen in [`../Analyse_Teil2/README.md`](../Analyse_Teil2/README.md).

## Offene Punkte
1. **Online-Hilfe** (ARTISTA.HLP): braucht eine saubere CD-Kopie; dort stehen ggf. weitere Erklärungen zu Stoffen und Werten.
2. **Bedeutung einzelner Tags** (z. B. 0x2722, 0x2739, 0x277e, EasyStitch-Nebenschwellen) ist nur vermutet.
3. **EXP-Schreibcode** (MOD1/IMPORT) nicht im Detail verfolgt; die Leselogik ist vollständig.
4. **INF/BMP-Begleitdateien und `EmbF5`**: existieren in V4 nicht. Für artista 630/640/730 braucht es eine neuere Software (Designer V5/V6).
5. Buchstaben in ESB- und Objekte in ART-Dateien liegen im binären Wilcom-Objektformat; das ist entschlüsselt, aber nicht dekodiert.
