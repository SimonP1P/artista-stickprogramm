# 02 – Maschinenspeicher, Briefkasten-Kommandos, Statusbits, Fehlercodes

**Quelle:** BEAGLink.dll. Funktionen in [`daten/funktionen/`](daten/funktionen/).
**Sicherheit:** hoch für Adressen, Längen und Reihenfolgen; mittel für die *Namen* der Felder und Kommandos (aus der Verwendung abgeleitet). Die Spalte „Beleg“ nennt jeweils die Code-Stelle.

## 1. Speicherzugriff im ASCII-Protokoll
Die Maschine stellt einen **24-Bit-Adressraum** bereit. Der PC liest und schreibt beliebige Adressen:
| Funktion | Befehl auf der Leitung |
|---|---|
| `Lese32Bytes(adr, buf)` 0x1000250a | `'R'` + 3 Adressbytes (je 2 Hexziffern mit Echo) → 32 Byte als 64 Hexziffern → `'O'` (bei 4800 Baud und Adresse < 0x1000 stattdessen `'E'` + 2 Adressbytes) |
| `ReadComBytes(adr, buf, n)` 0x100029b0 | zerlegt in 32-Byte-Blöcke über `Lese32Bytes` |
| `WriteComBytes(adr, buf, n)` 0x10002e50 → 0x100029d0 | zerlegt in Stücke ≤ 32 Byte (bzw. 256-Byte-Seiten über 0x10002ce0) |
| Kleinschreiben 0x10002b20 | **`'W'` + 3 Adressbytes + Datenbytes (alle als 2 Hexziffern, jedes Zeichen mit Echo) + `'?'` (Echo, Timeout = Parameter; beim Befehlsfeld Timeout + 5000 ms)**. Bei 4800 Baud: je Byte `'W'` + 2 Adressbytes + Datenbyte, Antwort `'O'`. Beispiel Befehl 0x50: `W F F F E D 0 0 0 5 1 ?` |

**Alle Zeiger und Mehrbyte-Werte im Maschinenspeicher sind Big-Endian** (Hilfsfunktionen 0x10003dd0 = u32 BE lesen, 0x10003db0 = u16 BE lesen, 0x10003df0/0x10003e00 = u16/u32 BE schreiben).

## 2. Feste Adressen
| Adresse | Länge | Inhalt | Beleg |
|---|---|---|---|
| `0x200100` | 32 | Firmware-Kennung `"SRMV"` + Version `x.yy` | 0x100059ea |
| `0xFFFD24` | 30 | **Speicher-Verwaltungsblock** (siehe 3.) | 0x10005b57 |
| `0xFFFED0` | 2 | **Briefkasten**: Kommando schreiben, Status lesen (siehe 4.) | 0x10003130, 0x10003160 |
| `0xFFFEE0` | 1 | Transfer-Statusbyte; Bit 0x02 = Maschine bereit/übernommen | 0x10006070 |
| `0x24004` | 4 | Zeiger → Maschinenstatus (Byte +5, Byte +7 = Fähigkeitsbits) und 11-Byte-Zusatzblock | 0x10004080, 0x10005d38 |
| `0x24040` | 4 | Zeiger → 32-Byte-Block (Byte +7 Bit 0x01 = Sticken erlaubt) | 0x100040e4 |
| `0x24044` | 4 | Zeiger → 18-Byte-Block; daraus benutzt: u8 bei +0x0C (Länge eines Zusatzkopfs der Farbtabelle, +6) und u16 BE bei +0x10 (Obergrenze der Farbtabelle, < 0xFD00) | 0x10005cfe, 0x10003a99 |
| `0x24048` | 4 | Zeiger → **Garnfarbtabelle** (Firmware ≥ 3.09) | 0x10003a74 |
| `0xFF0062` | | Zieladresse bei Firmware-Updates/Spezialfunktionen | 0x10007b0d |

## 3. Speicher-Verwaltungsblock (30 Byte ab `0xFFFD24`)
Wird nach `0x10072028` kopiert. Offsets relativ zum Blockanfang:
| Offset | Typ | Name (abgeleitet) | Verwendung im Code |
|---|---|---|---|
| +0x00 | u8 | **N** = Anzahl Farb-/Slot-Einträge (≤ 28) | bestimmt Längen der Tabellen und die Offsets der Infofelder |
| +0x01 | u8 | (≤ 60) | Plausibilitätsprüfung |
| +0x02 | u32 BE | **Größe des Design-Datenbereichs** (≤ 150 000 = 0x249F0) | Grenze beim Schreiben, Fehler 31 „Design is too big“ |
| +0x06 | u32 BE | **Zeiger INFO**: Byte +0 = Anzahl gespeicherter Designs; ab +N+1 Farbanzahl−1; ab +2N+1 Funktions-Flags; ab +3N+1 **Name (32 Byte)** | 0x10004d85, 0x10003915–0x10003a48 |
| +0x0A | u32 BE | **Zeiger AUSWAHL**: Byte +0 = (Designindex + 1), Byte +5 = Bank (Index / 27) | 0x10004c90 |
| +0x0E | u32 BE | **Zeiger ICON**: 0xAE-Byte-Kopf + 1-Bit-Bitmap | 0x100038aa |
| +0x12 | u32 BE | **Zeiger DATEN**: Transferpuffer für das Design | 0x1000378f |
| +0x16 | u32 BE | **Kapazität Kartenmodus** in Byte (artista 185: 130 426) – am Gerät korrigiert, siehe [07](07_befund_am_geraet.md) | 0x1000350f |
| +0x1A | u32 BE | **Kapazität Maschinenspeicher** in Byte (artista 185: 145 426) | 0x10003516 |

Daraus berechnet die Software Offsets im PC-Puffer (0x10072048 …): `0`, `2N`, `4N` (drei u16-Tabellen), `6N` (u32 Stichlänge), `6N+4` (u32 Zusatzlänge), `6N+8` (Beginn der Stichdaten).

## 4. Briefkasten 0xFFFED0 – Kommandos an die Maschine
**Ablauf (`0x10002f00`):**
1. Kommando `k` als **16-Bit-Wort Big-Endian mit gesetztem Bit 0** (`k | 1`) an `0xFFFED0` schreiben. Bit 0 = „Auftrag liegt an“.
2. Alle 100 ms das Statuswort an `0xFFFED0` lesen (2 Byte, Big-Endian), bis die Maschine fertig ist oder die Zeit (2. Parameter) abläuft:
   - fertig, wenn Low-Byte & 0x06 ≠ 0; bei Kommando 0x100: wenn High-Byte Bit 0 = 0
3. Status auswerten:

| Statusbit (High-Byte) | Fehler |
|---|---|
| 0x10 | 22 „No memory card in the embroidery modul“ |
| 0x20 | 23 „Card not formatted“ |
| 0x40 | 24 „No flashcard“ |
| 0x80 | 25 „Card is full“ (bzw. 20 „Too much designs“ im Kartenmodus) |
| Low-Byte **Bit 0x04 = 1** (Kommando gescheitert) | je nach Kommando: 0x200 → 25 „Card is full“ (Kartenmodus 20), 0x400 → 31 „Design is too big“, 0x800 → 3 „Design not deleted“, sonst 17 (außer bei Bank-/Zielwahl 0x30/0x60/0xC0/0x50/0xA0, dort kein Fehler). Bit 0x04 = 0 bedeutet Erfolg. |

**Kommandos** (Spalte „Timeout“ = Wartezeit / Zusatzzeit in ms):
| Kommando | Bedeutung | aufgerufen in | Timeout |
|---|---|---|---|
| **0x10** | Schreiben vorbereiten (Transferpuffer freigeben) | Design schreiben 0x100033b9 | 1000 / 1000 |
| **0x20** | Design aus Speicher in Transferpuffer laden (nach Auswahl) | 0x10004d74, 0x10002ec8 | 1000 / 2000 |
| **0x30 / 0x60 / 0xC0** | **Bank wählen**: Designs 0–26 / 27–53 / ab 54 | 0x10004d10, 0x100064c0 | 1000 / 4000 |
| **0x40** | Löschen vorbereiten | Löschen 0x10006dd0 | 1000 / 1000 |
| **0x50 / 0xA0** | Speicherziel: **0x50 = Maschinenspeicher, 0xA0 = Karte** (ab FW 2.10) *– Zuordnung vermutlich vertauscht, siehe [07](07_befund_am_geraet.md)* | 0x10004de0 | 1000 / 4000 |
| **0x80** | **Karte formatieren** | `_FormatCard`, `_WriteAllDesigns` | 5000 / 2000 |
| **0x100** | Transfermodus verlassen | 0x10005e30, 0x10006070 | 2000 / 2000 |
| **0x200** | **Design dauerhaft speichern** (aus Transferpuffer) | Ende von 0x100032b0 | 7000 / 4000 |
| **0x400** | gespeichertes Design in den Stickbereich laden | Lesen 0x100043ef, Sticken 0x10006efa | 3000 / 4000 |
| **0x800** | **Design löschen** | 0x10006e28 | 5000 / 4000 |
| **0x08** | **Sticken starten** (Design an Stickmodul übergeben) | 0x10006f41 | 1000 / 1000 |

## 5. Binärrahmen (Card Reader/Writer-Box, neuere Firmware)
```
PC → Gerät:   02 | Nutzdaten | 03 | Prüfsumme      (Bytes 0x01/0x10 mit vorangestelltem 0x10)
Gerät → PC:   06 (ACK)
Antwortdaten: n Nutzbytes (0x10-Maskierung) | Prüfsumme  → PC bestätigt mit 06
Prüfsumme = Summe aller gesendeten Bytes mod 256 (einschließlich 02, 03 und Maskierungsbytes)
```
| Befehl | Nutzdaten | Zweck |
|---|---|---|
| `'f'` 0x66 | – | Geräteinfo, Antwort 6 Byte |
| `'k'` 0x6B | – | Gerätename, Antwort 20 Byte (`"BERNINA RWUSER"`) |
| `'i'` 0x69 | – | Kartenstatus, Antwort 3 Byte |
| `'l'` 0x6C | Adr u32 BE + Länge u32 BE | Lesen (max. 0x200 pro Abruf) |
| `'w'` 0x77 | Adr u32 BE + Länge u32 BE, dann Datenrahmen | SRAM schreiben |
| `'a'` 0x61 | wie `'w'` | Flash (ATMEL) schreiben |
| `'d'` / `'p'` | – | Schreibschutz aus / ein |
| `'b'` | 1 Byte: 3 oder 4 | Baudrate umschalten |

Die Karte wird im Binärprotokoll als **128-KB-Abbild ab Adresse 0** gelesen und geschrieben (im ASCII-Kartenmodus ab `0x280000`); siehe [04](04_kartenabbild_felder.md).

## 6. Alle Fehlercodes (String-Tabelle in BEAGLink.dll, englisch; deutsch ebenfalls enthalten)
| Code | Meldung | Code | Meldung |
|---|---|---|---|
| 1 | Design not copied | 19 | Faulty memory card |
| 2 | Cannot embroider design | 20 | Too much designs |
| 3 | Design not deleted | 21 | Program error! |
| 4 | Card not formatted | 22 | No memory card in the embroidery modul |
| 5 | Design not downloaded | 23 | Card not formatted |
| 6 | Design not renamed | 24 | No flashcard |
| 7 | Impossible to install exit procedure | 25 | Card is full |
| 8 | Serial port is engaged | 30 | Embroider modul not connected |
| 9 | Cannot read the parameters of the serial port | 31 | Design is too big |
| 10 | Cannot initialize serial port | 32 | Data not saved properly |
| 11 | Serial port: read error | 33 | Could not free memory |
| 12 | Serial port: write error | 34 | Could not synchronize |
| 13 | No link to serial port | 35 | Not enough memory |
| 14 | Serial port: buffer not empty | 36 | Power supply is too low |
| 15 | Serial port: wrong echo | 37 | Warning: Only a limitated functionality is given with very long designs. |
| 16 | Serial port: wrong protocol | | |
| 17 | Serial port: synchronization not possible | | |
| 18 | Wrong software version | | |

Der Code wird in der globalen Variable `0x10068e30` gehalten (Funktion 0x10001120 setzt ihn nur, wenn noch kein Fehler aktiv ist). Die Nummer ist gleichzeitig die String-ID.
