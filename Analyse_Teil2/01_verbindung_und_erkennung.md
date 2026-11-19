# 01 – Verbindungsaufbau und Erkennung der Maschine (artista 165–185, R/W-Box)

**Quelle:** `ADX/data1.cab → BEAGLink.dll` (Version 1.0.0.1). Alle Adressen sind virtuelle Adressen in dieser DLL; den Maschinencode jeder genannten Funktion findest du unter [`daten/funktionen/<adresse>.asm`](daten/funktionen/).
**Sicherheit:** hoch für Reihenfolge, Bytes und Zeiten (direkt aus dem Code). Die *Bedeutung* einzelner Antwortbytes ist mittel und steht dabei.

## Überblick: drei Ebenen
```
1. Serieller Port öffnen          0x10001610   COMx, 19200 8N1, Puffer 4 KB
2. Synchronisieren / Protokoll    0x10001d00   ASCII-Echo-Protokoll  oder  Binär-„Mc“-Protokoll
3. Maschine einlesen              0x10005980   Firmware-Kennung, Speicher-Verwaltungsblock
```
Jede höhere Funktion (Lesen, Schreiben, Löschen …) ruft zuerst `TransparentMode(1)` (0x100061b0) auf. Das führt Schritt 2 und 3 nur aus, wenn sie noch nicht erledigt sind. Am Ende kommt `TransparentMode(0)`.

## Schritt 1 – Port öffnen (`0x10001610`)
| Parameter | Wert |
|---|---|
| Name | `COM1` … `COM4` (Format `"COM%d"`) |
| Zugriff | `CreateFileA(..., GENERIC_READ|GENERIC_WRITE, 0, NULL, OPEN_EXISTING, FILE_FLAG_OVERLAPPED, NULL)` |
| Puffer | `SetupComm(4096, 4096)` |
| Timeouts | ReadIntervalTimeout = MAXDWORD (sofort zurück), WriteTotalTimeoutConstant = 5000 ms |
| DCB | Baud = aktuelle Rate (Start **19200**), **8 Datenbits, 1 Stoppbit**, Parität aus (über `SetParity` 1–4 änderbar), fBinary = 1, keine Flusskontrolle |

## Schritt 2 – Synchronisieren (`0x10001d00`)
Läuft, sobald ein Fehler gesetzt ist (z. B. beim ersten Kontakt). **Bis zu 48 Versuche** (`esi < 0x30`):

```
für Versuch v = 0 … 47:
    wenn v mod 6 == 5:  Baudrate weiterschalten  4800 → 19200 → 57600 → 115200 → 4800 …
                        und Port neu öffnen
    wenn v > 0: 50 ms warten
    Eingangspuffer leeren (lesen bis „leer“ = 0x1000)
    Windows-Nachrichten abarbeiten (0x10001350, damit die Oberfläche nicht einfriert)
    sende  'R' (0x52)
    lies Antwort, bis zu 5 × 20 ms; 'Q' (0x51) wird übersprungen
    ── Binärprobe: wenn v mod 6 == 1 und Baud ≥ 57600:
           600 ms warten, Binär-Flag löschen, McCheck() (siehe unten)
           gelingt sie und Baud == 57600: McChangeBaudrate(1) → 115200, Port neu, McCheck() erneut
           → Erfolg: Binärprotokoll aktiv, fertig
    ── ASCII-Probe: wenn Antwort == 'R' (Echo):
           sende 'F' (0x46), erwarte 'F'   (5 × 20 ms)
           sende '?' (0x3F), erwarte '?'
           → Erfolg: ASCII-Protokoll aktiv, fertig
scheitern alle 48 Versuche: Port schließen, Fehler 13 „No link to serial port“
```

### Binär-Erkennung `McReadVersion` (`0x10013b00`, aufgerufen von `McCheck` 0x10013d30)
Rahmenformat siehe [02](02_maschinenspeicher_und_kommandos.md#binärrahmen). Drei Abfragen:

| Nr | PC sendet | Antwort (Nutzbytes) | Auswertung |
|---|---|---|---|
| 1 | `'f'` (0x66) | **6 Byte** | Geräte-Byte = `antwort[0] & 0x3F`; wenn `antwort[5] ≠ 0` → zusätzlich Bit 0x80 |
| 2 | `'k'` (0x6B) | **20 Byte** (Name) | ab Byte 2 Vergleich mit **`"BERNINA RWUSER"`**; bei Gleichheit Bit 0x40 im Geräte-Byte (= **Card Reader/Writer-Box**) |
| 3 | `'i'` (0x69) | **3 Byte** | Byte 0 = Kartenstatus (0 → Fehler 24 „No flashcard“), Byte 1, Byte 2 = weitere Statuswerte |

Danach: Ist eine Karte vorhanden, wird mit `McProtect(0)` (Befehl `'d'`) der Schreibschutz aufgehoben. Das Binär-Flag (0x100426b0) wird auf 1 gesetzt. Ist das Geräte-Byte ohne Bit 0x40 gleich 0 und Byte 1 von Abfrage 3 kleiner als 2, folgt Fehler 36 **„Power supply is too low“**.

**Bedeutung (Sicherheit mittel):** Das Binärprotokoll gehört zur **Card Reader/Writer-Box** und zu Geräten mit neuerer Firmware. Die artista 165–185 mit Stickmodul antwortet im ASCII-Protokoll.

## Schritt 3 – Maschine einlesen (`0x10005980`, nur ASCII-Protokoll)
1. Verbindung prüfen (0x10002330).
2. Ist der Speicher-Verwaltungsblock noch nicht geladen: **Transfermodus betreten** (0x10005e30, siehe unten).
3. **32 Byte ab Maschinenadresse `0x200100` lesen.** Erwartet wird die Firmware-Kennung **`"SRMV"`** und eine Version im Format `x.yy`. Die Ziffern werden als Hex-BCD gespeichert: „3.09“ → `0x309` (globale Variable 0x10068e2c). Fehlt „SRMV“, wird der Transfermodus neu betreten und erneut gelesen.
4. **30 Byte (0x1E) ab Adresse `0xFFFD24` lesen** → Speicher-Verwaltungsblock (Felder siehe [02](02_maschinenspeicher_und_kommandos.md)). Prüfungen: Byte 0 ≤ 28, Byte 1 ≤ 60, Speichergröße ≤ 150 000 Byte (0x249F0); sonst Fehler 18 „Wrong software version“.
5. **Ab Firmware 3.09:** Zeiger (u32 BE) bei `0x24044` lesen → 18 Byte Zusatzinfo; Zeiger bei `0x24004` lesen → 11 Byte Zusatzinfo; danach eine Fähigkeitsabfrage (0x10004150, siehe unten).
6. Verwaltungsblock als gültig markieren (0x10072020 = 1).

### Transfermodus betreten (`0x10005e30`)
```
optional: Kommando 0x100 („Transfermodus verlassen“) an Briefkasten, damit ein alter Zustand endet
Port öffnen
sende mit Echo:  'T' 'r' 'M' 'E'        (je 1000 ms Timeout)
wenn schnelle Rate gewünscht und ASCII-Protokoll:
    ChangeBaudrate(57600)  (Code 0x05, siehe Teil 1)
    250 ms warten, Port neu öffnen
    sende mit Echo: 'Y' 'Q'
    lies 1 Byte (3000 ms) → muss 'O' (0x4F) sein, sonst Fehler 16 „wrong protocol“
Erfolg → Transfermodus aktiv (0x10068e88 = 1)
```

### Fähigkeitsbits (`0x10004080`, `0x10004150`)
- Zeiger bei **`0x24004`** lesen (u32 BE) → P. Das **Byte bei P+7** ist das Statusbyte der Maschine (0x1006ae84). Verwendet wird **Bit 0x04**: gesetzt bedeutet, dass der Designname aus dem Speicher übernommen wird; nicht gesetzt bedeutet, dass die Software als Namen eine laufende Nummer einträgt (Format `"%d"` bei 0x100400e4). **Bit 0x08** gibt Schreiben in Modus 0 frei.
- Das **Byte bei P+5**, Bit 0x02, und das Feld bei Zeiger `0x24040` (32 Byte), Byte 7 Bit 0x01, ergeben zusammen eine Stufe 0/1/2 (0x10072084). Diese steuert, ob „Embroider“ (Modus 3) möglich ist.

### Firmware-Grenzen
| Firmware | Auswirkung im Code |
|---|---|
| < 2.10 (`0x210`) | nur ein Speicherziel (Maschinenspeicher) |
| ≥ 2.10 | zwei Ziele: Kommando **0x50** = Maschinenspeicher, **0xA0** = Karte im Modul *(Zuordnung vermutlich vertauscht – siehe [07](07_befund_am_geraet.md), Code-Nachprüfung 19.11.2026)*; Modus 1 von `A1xxWriteDesign` erlaubt |
| ≥ 3.09 (`0x309`) | zusätzliche Blöcke (Zeiger 0x24044, 0x24048), **Garnfarbtabelle mit Marke und Name** wird mit übertragen |

## Ablaufbild
```
PC                                   Maschine (ASCII-Protokoll)
 |-- 'R' ------------------------------>|
 |<------------------------------- 'R' -|   (Echo)
 |-- 'F' ------------------------------>|
 |<------------------------------- 'F' -|
 |-- '?' ------------------------------>|
 |<------------------------------- '?' -|   → synchron
 |-- 'T' 'r' 'M' 'E' (je mit Echo) --->|   → Transfermodus
 |-- 'R' + Adresse 200100 ------------->|   (32 Byte lesen)
 |<-- 64 Hexziffern ... 'O' ------------|   "SRMV ... 3.09"
 |-- 'R' + Adresse FFFD24 ------------->|
 |<-- Verwaltungsblock ... 'O' ---------|
```
