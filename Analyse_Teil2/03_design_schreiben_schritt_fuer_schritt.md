# 03 – Design auf Stickmodul/Karte schreiben: Schritt für Schritt mit Adressen

**Quelle:** BEAGLink.dll: `A1xxWriteDesign` (0x10009fa0) → `0x100032b0` (Schreibkern, Maschinencode in [`daten/funktionen/100032b0.asm`](daten/funktionen/100032b0.asm)); Sticken: 0x10006ea0
**Sicherheit:** hoch für Reihenfolge, Adressquellen und Längen; mittel für die Namen der Datenfelder. Nicht geprüft am echten Gerät.

Zeiger wie `INFO`, `DATEN`, `ICON` sind die u32-BE-Werte aus dem Verwaltungsblock bei 0xFFFD24 ([02](02_maschinenspeicher_und_kommandos.md#3-speicher-verwaltungsblock-30-byte-ab-0xfffd24)). `N` = Byte 0 dieses Blocks.

## Aufruf
`A1xxWriteDesign(ziel, datei)`:
| ziel | Bedeutung | Bedingung im Code |
|---|---|---|
| 0 | Maschinenspeicher | Speicherziel-Flag 0x100670c8 = 0 und Fähigkeitsbit 0x08 (Statusbyte P+7) |
| 1 | Karte im Stickmodul | Firmware ≥ 2.10 (Flag 0x100670c8 = 1) |
| 3 | sofort sticken („Embroider“) | nur ASCII-Protokoll |
Im Binärprotokoll (R/W-Box) sind höchstens **15 Designs** erlaubt (sonst Fehler 25).

## Ablauf ASCII-Protokoll (artista 165–185 mit Stickmodul)
```
0  Exklusivzugriff holen (Sperre gegen gleichzeitige Aufrufe)
1  Datei laden  (0x10008e70)          → Designcontainer in PC-Liste, Chunks entschlüsselt (siehe 05)
2  TransparentMode(1)                  → synchronisieren, Transfermodus, Verwaltungsblock lesen (01)
3  [nur wenn speichern] Kommando 0x10 an 0xFFFED0  (Schreiben vorbereiten), warten bis fertig
4  Chunks aus dem Container holen:
      0x0A  Kopf, 0x134 Byte  (Name, Flags, Startwert)        – Pflicht
      0x1E  Stichdaten                                         – Pflicht
      0x14  Farbtabelle (0x86 Byte je Farbe, +6)               – optional (FW ≥ 3.09 genutzt)
      0x28  Zusatzdaten                                         – optional
      0x32  Icon (Breite, Höhe, 1-Bit-Bitmap)                   – optional
5  Größenprüfung: Stichlänge + Zusatzlänge + 2 < Speichergröße (Verwaltungsblock +0x02,
   bzw. Speicherende +0x16/+0x1A)            sonst Fehler 31 „Design is too big“
6  Datenblock im PC zusammensetzen (Puffer 0x100426d8):
      Offset 0        N × u16 BE   Tabelle A: Eintrag 0 = Wert aus Kopf-Offset 0x130, sonst 0
      Offset 2N       N × u16 BE   Tabelle B: 0
      Offset 4N       N × u16 BE   Tabelle C: 0
      Offset 6N       u32 BE       Länge Stichdaten + 2
      Offset 6N+4     u32 BE       Länge des Anhangs (Farben, Zusatzdaten …)
      Offset 6N+8     Stichdaten (aus Chunk 0x1E), danach Endkennung 80 81
      danach          Anhang: 8 Byte Containerkopf (u32 bei +2 = Anhanglänge),
                      dann die übrigen Chunks; der Anhang wird mit der BE-Verschleierung
                      VERSCHLÜSSELT (0x1000a640, Modus 1)
7  ► WriteComBytes(DATEN, Puffer, 6N+8 + Stichlänge + 2 + Anhanglänge)
8  wenn Icon vorhanden:
      Kopf 0xAE Byte: [0]=0 [1]=0 [2]=Bytes je Zeile ((Breite+7)/8) [3]=Höhe [4]=0xFF [5]=0, Rest 0
      dahinter Bitmap (Bytes je Zeile × Höhe)
   ► WriteComBytes(ICON, Kopf+Bitmap, 0xAE + Bitmapgröße)
9  ► WriteComBytes(INFO + (N+1),   [Farbanzahl − 1],   1 Byte)
10 Funktions-Flags aus dem Kopf bilden (je ein Bit, wenn das Kopffeld ≠ 0):
      Bit 0x01 (Feld +0x11C oder +0x11A), 0x02 (+0x116), 0x04 (+0x118), 0x08 (+0x11E),
      0x10 (+0x120), 0x20 (+0x124), 0x80 (+0x126)   – Offsets im Kopf-Chunk inkl. 6-Byte-Chunkkopf
   ► WriteComBytes(INFO + (2N+1), Flags, 1 Byte)
11 Name: 32 Byte aus dem Kopf ab Offset 0x14 (inkl. Chunkkopf), abschließende Leerzeichen → 0
   ► WriteComBytes(INFO + (3N+1), Name, 32 Byte)
12 nur Firmware ≥ 3.09 – Garnfarbtabelle (Feldnamen vermutet, Sicherheit mittel):
      Zeiger T bei 0x24048 lesen
      je Farbe (0x86-Byte-Eintrag aus Chunk 0x14) einen Datensatz bilden:
         u16 BE Datensatzlänge | Index | Farbbytes R,G,B | u16 BE Nummer |
         Marke (bis 62 Zeichen, 0-terminiert) | Farbname (bis 62 Zeichen, 0-terminiert)
      Tabellenkopf: Anzahl, Länge; Obergrenze = u16 bei Zusatzblock 0x24044 +0x10
   ► WriteComBytes(Tabellenziel, Datensätze)  danach  WriteComBytes(T, 00 00 00, 3)
13 [nur wenn speichern] Kommando 0x200 an 0xFFFED0 (dauerhaft speichern), Timeout 7 s + 4 s
   Status: Bit 0x80 → „Card is full“, Bit 0x04 → Fehler
14 PC-Liste aufräumen, TransparentMode(0), Exklusivzugriff freigeben
```

## Sticken („Embroider“, ziel = 3) – 0x10006ea0
**a) Design aus dem PC:**
```
1–12 wie oben, aber OHNE Kommando 0x10 und OHNE 0x200 (nur Transferpuffer)
►  Kommando 0x08  an 0xFFFED0          → Maschine übernimmt und startet das Sticken
TransparentMode(0)
```
**b) bereits gespeichertes Design Nr. i:**
```
Bank wählen:   i < 27 → 0x30,  27 ≤ i < 54 → 0x60,  sonst 0xC0       (0x10004d10)
►  WriteComBytes(AUSWAHL+0, [i+1], 1)   und   WriteComBytes(AUSWAHL+5, [i / 27], 1)   (0x10004c90)
►  Kommando 0x400  (Design in Stickbereich laden), Timeout 3 s + 4 s
►  Kommando 0x08   (sticken)
```

## Lesen (umgekehrte Richtung) – 0x10004310
```
Speicherziel wählen (0x50/0xA0), Bank wählen (0x30/0x60/0xC0), Kommando 0x20 → Anzahl bei INFO+0
Auswahl schreiben (AUSWAHL), Kommando 0x400
► ReadComBytes(DATEN, 6N+8 + …)      Tabellen + Längen + Stichdaten
► ReadComBytes(ICON, 6)               Kopf → Icongröße = ((Breite+7)/8)·Höhe + 10
                                      (am Gerät: Kopf ist 0xAE Byte, Bitmap ab 0xAE – mit dieser Länge fehlen
                                       19 von 62 Zeilen; siehe 07, Auswertung 19.11.2026)
► ReadComBytes(INFO …)                Farbanzahl, Flags, Name
FW ≥ 3.09: Zeiger 0x24048 → Farbtabelle lesen und prüfen (Kopf: Byte 0 = 0, u32 = 1, u32 = 8 …)
Anhang mit BE-Verschleierung ENTschlüsseln (0x1000a640, Modus 0)
→ wieder als Container mit Chunks 0x0A, 0x1E, 0x32, 0x14 im PC
```

## Designliste lesen („Card/Machine Read“) – 0x100064c0 + 0x10004d10
```
Vorher:        TransparentMode(1): 0x01 → 0xFFFEE0, warten bis 0xFFFEE0 Bit 0x02 (≤ 5 s)   [am Gerät nötig, v3]
Ziel wählen:   Befehl 0x50 (Maschinenspeicher) bzw. 0xA0 (Karte, Flag 1)          [FW ≥ 2.10]   (Zuordnung vermutlich vertauscht, siehe 07)
► WriteComBytes(AUSWAHL+0, [1]) ; WriteComBytes(AUSWAHL+5, [0])     (Index 0 → Bank 0)
►  Befehl 0x30 (Bank 0, Flag 1)
►  Befehl 0x20 (Liste laden, Flag = Ziel 0/1)
► ReadComBytes(INFO+0, 1)  → Anzahl Designs
je Bank b = 0, 1, 2 …  (27 Designs pro Bank):
   Status lesen (Zeiger 0x24004, Byte +7; Bit 0x04 = Namen anzeigen, sonst laufende Nummer)
   ► WriteComBytes(AUSWAHL+5, [b]);  b = 1 → Befehl 0x60,  b ≥ 2 → Befehl 0xC0
   ► ReadComBytes(INFO + 2N+1, 27)        → Flags je Design
   ► ReadComBytes(INFO + 3N+1, k·32)      → Namen (32 Byte, Byte 31 = 0)
Ende: Byte 0xFFFEE0 lesen; Bit 0x02 gesetzt → Befehl 0x100 (PC-Modus verlassen)
```
Umgesetzt im Testprogramm v3 (`Testprogramm_artista185_Windows_v3/`). An der artista 185 war das Statusbyte 0x00, die Original-Software würde dort also Nummern statt Namen zeigen.

## Löschen – 0x10006dd0
```
Kommando 0x40 → Auswahl schreiben (Index+1, Bank) → Kommando 0x800 (Timeout 5 s + 4 s)
```

## Karte formatieren – `_FormatCard` / `_WriteAllDesigns`
```
Kommando 0x80 (Timeout 5 s + 2 s); im Binärprotokoll: Kartenabbild neu aufbauen (FormatImage),
McProtect(0) → Abbild schreiben ('w') → McProtect(1)
```

## Zeitverhalten
- Jeder Befehl wartet auf das Echo; Datenbytes werden als Hexziffern übertragen. Ein Byte kostet also 2 Zeichen hin und 2 zurück. Bei 19200 Baud sind das etwa 480 Byte/s netto.
- Die Briefkasten-Abfrage läuft alle **100 ms**.
- Vor Firmware-Updates fordert die Software zum Aus- und Einschalten auf. Für das normale Schreiben ist das nicht nötig.
