# 04 – Aufbau der Design-Karte (128-KB-Abbild) und Bedeutung der Felder

**Quelle:** BEAGLink.dll, Klasse „Image“ (Objekt bei 0x1006afec bzw. 0x1006af98): `InitImage` 0x1000d690, Lesen 0x1000d3b0, Verwaltung 0x1000ac70, freien Block suchen 0x1000ab70, Namen 0x1000b060, Icon 0x1000b0e0, Eintragen 0x1000b2a0 (`EnterDesign`)
**Sicherheit:** hoch für Offsets und Prüfregeln (aus dem Code); mittel für die Bedeutung einzelner Felder. Nicht an einer echten Karte geprüft.

## Wo liegt die Karte?
- Größe des Abbilds: **0x20000 = 131 072 Byte (128 KB)**.
- Gelesen und geschrieben wird es über `ReadComBytes`/`WriteComBytes` bzw. Binärbefehle `'l'`/`'w'`, ab
  - Adresse **0x000000** im Binärprotokoll (R/W-Box),
  - Adresse **0x280000** im ASCII-Kartenmodus (Karte im Stickmodul, Firmware ≥ 2.10).
- Firmware-Images (`*.A20`, `*.BE`) liegen im Bereich 0x280000–0x680000 (`BuildImageA20_File`).

## Kopf des Abbilds
| Offset | Typ | Bedeutung | Prüfung im Code |
|---|---|---|---|
| 0x30 | u8 | **Anzahl Designs** auf der Karte (`ImageDesignCount`) | Index < Anzahl |
| 0x32 | u32 | **D** = Beginn des Verzeichnisses | 0x1000 ≤ D ≤ 0x2000 |
| 0x36 | u32 | **E** = Beginn des Datenbereichs | E ≥ D + 0x1000, E < 0x10000, E ≥ D + Anzahl·64 |
| 0x46 | u16 | Länge der Belegungstabelle | Länge + 0x48 ≤ D |
| 0x48 … | Bits | **Belegungstabelle**: 1 Bit je 256-Byte-Block; 0xFF = 8 Blöcke belegt | |
Bei einem Prüfungsfehler gilt die Karte als „nicht formatiert“ (Fehler 4 bzw. 23).

**Freien Block finden (0x1000ab70):** Ab Byte 0x48 alle Bytes überspringen, die 0xFF sind. Im ersten anderen Byte das erste Null-Bit suchen (bit = 0…7).
```
Blockadresse = ((byteOffset·8 + bit − 0x240) << 8) + E          (0x240 = 0x48·8)
```
Daten werden also in **256-Byte-Blöcken** ab E abgelegt.

## Verzeichniseintrag (64 Byte je Design, ab D)
Aus `EnterDesign` (0x1000b2a0) und `ReadImageName` (0x1000b060, kopiert 64 Byte):
| Offset im Eintrag | Typ | Bedeutung | Wert beim Schreiben |
|---|---|---|---|
| 0x00 | u8 | belegt | 1 |
| 0x01 | u8 | **Farbanzahl − 1** | aus Stichdaten gezählt (`_GetColorCount`) |
| 0x02 | u32 | **Adresse der Designdaten** (Block im Datenbereich) | vom Blockverwalter vergeben |
| 0x0E | u8 | **Funktions-Flags**: Bit 0x01 = Kopffeld +0x11A/+0x11C, 0x02 = +0x116, 0x04 = +0x118, 0x08 = +0x11E, 0x10 = +0x120, 0x20 = +0x124, **0x40** = +0x126 (im Maschinenspeicher ist es dort Bit 0x80) | aus Kopf-Chunk |
| 0x10 | u8 | (immer 1) | 1 |
| 0x11 | u8 | Zusatzwert aus Kopf-Offset 0x133 | nur wenn Kopffeld +0x11E ≠ 0 (Flag 0x08), sonst 0 |
| 0x1E … 0x3C | char[31] | **Designname**, 0-terminiert, Leerzeichen am Ende entfernt | aus Kopf-Chunk Offset 0x14 |
| 0x3E | u8 | Endkennung | 0xFF |

## Designdaten im Datenbereich
Ab der Adresse aus Eintrag +0x02:
| Offset | Inhalt |
|---|---|
| +0x00 … | Kopf (u16/u32-Werte wie im Speichermodus) |
| +0x04 | u8 Icon: Bytes je Zeile |
| +0x05 | u8 Icon: Höhe in Zeilen |
| +0x07 | Icon-Bitmap, 1 Bit je Pixel, Zeilen rückwärts gespeichert (`ReadImageBitmap` liest ab der letzten Zeile) |
| danach | Stichdaten (EXP-artig, Ende `80 81`) und der verschleierte Anhang (Farben usw.) |
Prüfung beim Eintragen: Iconbytes = ((Breite+7)/8)·Höhe + 10 ≤ Länge des Icon-Chunks; Farbtabelle = Vielfaches von 0x86 + 6 Byte.

## Grenzen
- Höchstens **15 Designs** im Binärprotokoll (`A1xxWriteDesign` prüft Anzahl ≥ 15 → Fehler 25 „Card is full“).
- Im Speichermodus der Maschine: Bänke zu je **27 Designs** (Kommandos 0x30/0x60/0xC0, also maximal 81), Anzahl im INFO-Bereich ≤ 0x1B00.
- Designgröße ≤ Speichergröße aus Verwaltungsblock (≤ 150 000 Byte).
