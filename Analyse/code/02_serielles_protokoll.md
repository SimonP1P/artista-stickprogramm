# 02 – Serielle Kommunikation artista 165/170/180/185 und Card-Reader/Writer-Box

**Quelle:** `ADX/data1.cab → BEAGLink.dll` (disassembliert mit [`rdis.py`](rdis.py)); Adressen = virtuelle Adressen in BEAGLink.dll
**Sicherheit:** hoch für Port-Parameter und Rahmenformat (direkt aus dem Maschinencode); mittel für die Bedeutung einzelner Befehle (aus Funktionsnamen abgeleitet)

## 1. Wer benutzt das?
`adx.dll` zeigt im Dialog „Select Embroidery System“ zwei Optionen: **„artista 200“** und **„artista 165 to 185 or R/W box“**, dazu „COM Port: COM1…COM4“ und „Apply these settings (and suppress this dialog)“. Wählt man 165–185, lädt ADX `BEAGLink.dll` und ruft dort die Funktionen `A1xxWriteDesign`, `A1xxReadDesign`, `A1xxGetDesignInformation`, `A1xxLoadDesignInformation`, `A1xxDelete`, `A1xxRename` und `A1xxGetEXPIcon` auf („A1xx“ = artista 1xx).

## 2. Port-Einstellungen (Funktion `0x10001610`, `0x100017f0`)
```c
CreateFileA("COM%d", GENERIC_READ|GENERIC_WRITE, 0, NULL, OPEN_EXISTING, FILE_FLAG_OVERLAPPED|FILE_ATTRIBUTE_NORMAL, NULL);
SetCommMask(h, EV_RXCHAR);
SetupComm(h, 4096, 4096);                    // 4 KB Ein-/Ausgabepuffer
SetCommTimeouts(h, {ReadIntervalTimeout=MAXDWORD, 0, 0, 0, WriteTotalTimeoutConstant=5000});
DCB.BaudRate = baud;                         // Standard 19200 (globale Variable 0x100400d8)
DCB.ByteSize = 8; DCB.StopBits = ONESTOPBIT; // 8 Datenbits, 1 Stoppbit
DCB.Parity   = parity;                       // SetParity(1..4), Standard 0 = keine Parität
DCB.fBinary = 1; DCB.fParity = (parity != 0);
DCB.XonChar/XoffChar = 0, keine Hardware-Flusskontrolle gesetzt
```
**Ergebnis: 19200 Baud, 8N1, keine Flusskontrolle.** Das Handbuch nennt maximal 4 COM-Ports (S. 277).

### Baudratenwechsel (`ChangeBaudrate`, 0x100057b0)
Erlaubt sind nur drei Raten, jede mit einem Code für die Maschine:
| Baud | Code an Maschine |
|---|---|
| 19200 | 0x11 |
| 57600 | 0x05 |
| 115200 | 0x02 |

Ablauf: Code senden → Port schließen → mit neuer Rate öffnen → Eingangspuffer leeren → bis zu 20-mal 100 ms auf `'B'` warten → `'O'`,`'S'` erwarten → `'E'`, `'B'` senden. Zusätzlich gibt es einen Altmodus mit **4800 Baud** (0x12C0), siehe unten.

## 3. Protokoll A – ASCII mit Echo (ältere Firmware)
Grundbausteine:
- **Zeichen senden (`0x100027d0`):** Zeichen senden, dann **muss die Maschine dasselbe Zeichen als Echo zurückschicken**, sonst Fehler 0x0F. Vor dem Senden werden alle Zeichen im Puffer verworfen, Bytes < 0x20 oder > 0x7F gelten als Störung (Fehler 0x0E).
- **Byte senden (`0x10002750`):** als **zwei ASCII-Hexziffern** (`'0'–'9'`, `'A'–'F'`, oberes Nibble zuerst), jede einzeln mit Echo.
- **Byte empfangen (`0x10002670`):** zwei Hexziffern mit 2000 ms Timeout.

**Beispiel „32 Bytes lesen“ (`Lese32Bytes`, 0x1000250a):**
```
bei 4800 Baud und Adresse < 0x1000:  'E'  Adr-Hi Adr-Lo          (4 Hexziffern)
sonst:                               'R'  A23..16 A15..8 A7..0   (6 Hexziffern)
Antwort: 32 Bytes = 64 Hexziffern, danach 'O' (0x4F) innerhalb 1200 ms
```

## 4. Protokoll B – Binär „Mc“ (neuere Firmware, „TransparentMode“)
Welches Protokoll gilt, entscheidet das Flag `0x100426b0` (wird gesetzt, wenn die Maschine sich passend meldet).

**Rahmenaufbau (`0x10013720` + `0x100137a0`):**
```
0x02 | Nutzdaten ... | 0x03 | Prüfsumme
```
- Jedes Byte mit Wert **0x01 oder 0x10** wird mit vorangestelltem **0x10** (DLE) gesendet (Byte-Stuffing).
- **Prüfsumme** = Summe (mod 256) aller tatsächlich gesendeten Bytes inklusive STX, ETX und DLE-Bytes.
- Die Maschine antwortet mit **0x06 (ACK)**.
- Maximal 0x244 Nutzbytes pro Rahmen; Daten werden in Stücken zu 0x200 Byte übertragen.

**Befehle (erstes Nutzbyte):**
| Funktion | Byte | Nutzdaten |
|---|---|---|
| `McWriteSRAM` | `'w'` 0x77 | Adresse u32 **Big-Endian** + Länge u32 BE (9 Byte), danach Datenrahmen ≤ 0x200 |
| `McWriteATMEL` | `'a'` 0x61 | wie oben (Flash des Mikrocontrollers) |
| `McReadData` | `'l'` 0x6C | Adresse + Länge, max. 0x200 pro Abruf |
| `McReadVersion` | `'f'`, `'k'` | Versionsabfrage |
| `McProtect` | `'d'` 0x64 / `'p'` 0x70 | Schreibschutz aus/ein |
| `McChangeBaudrate` | `'b'` + 3 oder 4 | Baudwechsel |

## 5. Design-Karte (Image im PC-Speicher)
- Das Kartenabbild wird als **128-KB-Puffer (0x20000)** verwaltet (`InitImage`, `BuildImage`).
- Die Anzahl der Designs steht bei **Offset 0x30**.
- Pro Design wird beim Eintragen (`EnterDesign` → 0x1000b2a0) Folgendes gespeichert: **Name ≤ 31 Zeichen** (abschließende Leerzeichen werden entfernt), **Farbanzahl−1**, Flag-Byte (welche Funktionen das Design nutzt), **Icon als 1-Bit-Bitmap** (Bytes = ((Breite+7)/8)·Höhe + 10 Kopf) und Farbblöcke zu je 0x86 Byte.
- Die Designdaten selbst liegen als **EXP-Stichdaten** auf der Karte (siehe [07_exp_format.md](07_exp_format.md)); daher `A1xxGetEXPIcon`.
- Firmware-Update der Maschine über dieselbe Leitung: Dateien `*.A20`, `*.BIN`, `*.BE` und `UpdateBE.ini`, Adressbereich 0x280000–0x680000 (`BuildImageA20_File`, `BuildImageBE_File`, `ProgramFlash`, `UpdateArtista`).

## 6. Fehlermeldungen (Auszug, Klartext in BEAGLink/adx.dll)
„Another process uses the serial port.“ · „Device recognition failed. Please switch off and on device and repeat procedure.“ · „Embroidery system or sewing machine is running and not ready for data exchange.“
