# 01 – Installations-CD entpacken (InstallShield 3 + PKWARE DCL)

**Dateien:** [`unz.py`](unz.py), [`blast.c`](blast.c)
**Sicherheit:** hoch (alle intakten Dateien entpacken sich mit exakt der im Archiv angegebenen Größe)

## Aufbau der CD
| Pfad | Inhalt |
|---|---|
| `autorun.inf`, `InstPro.exe`, `instpro.ini` | Start-Menü der CD (Produkt-ID „BE“, Knöpfe für Custom/Quick-Install, IE6, Acrobat) |
| `Setup/English/setup.z` (148 MB) | **die eigentliche Software**: 651 Dateien in 27 Verzeichnissen |
| `Setup/English/setup.ins`, `_setup.lib` … | InstallShield-3-Skript und -Laufzeit |
| `ADX/data1.cab` (InstallShield 6) | **ADX = Artista Data eXchange**, das Kommunikationsmodul (16 Dateien, u. a. `adx.dll`, `BEAGLink.dll`, `AdxCe.dll`, `ADX.pdf`) |
| `HASP/` | Treiber und Diagnose für den HASP-Dongle (Kopierschutz) |
| `IE6/`, `Acrobat/` | Internet Explorer 6, Acrobat Reader (Fremdsoftware) |
| `instoem/` | Bitmaps für das CD-Menü |

`ADX/data1.cab` lässt sich mit dem Standardwerkzeug `unshield` entpacken. Für `setup.z` gibt es auf macOS kein Werkzeug, deshalb habe ich die beiden Programme oben geschrieben.

## Format von `setup.z` (InstallShield 3 „Z“-Archiv)
Alle Zahlen sind Little-Endian.

**Kopf (Offset 0):**
| Offset | Typ | Bedeutung | Wert hier |
|---|---|---|---|
| 0x00 | u32 | Signatur | `0x8C655D13` |
| 0x0C | u16 | Anzahl Dateien | 651 |
| 0x12 | u32 | Archivgröße | 148 384 210 |
| 0x29 | u32 | Offset Verzeichnistabelle | |
| 0x31 | u16 | Anzahl Verzeichnisse | 27 |
| 0x33 | u32 | Offset Dateitabelle | |

**Verzeichniseintrag:** `u16 anzahlDateien, u16 eintragsLänge, u16 namensLänge, char name[]`, z. B. `Main\PROGRAM\BIN`.

**Dateieintrag:**
| Offset | Typ | Bedeutung |
|---|---|---|
| 0x01 | u16 | Index des Verzeichnisses |
| 0x03 | u32 | Größe entpackt |
| 0x07 | u32 | Größe gepackt |
| 0x0B | u32 | Offset im Archiv |
| 0x0F | u16+u16 | DOS-Datum/-Zeit |
| 0x13 | u32 | Attribute |
| 0x17 | u16 | Länge des Eintrags (zum nächsten springen) |
| 0x1D | u8 | Namenslänge |
| 0x1E | char[] | Dateiname |

`unz.py` liest beide Tabellen, schneidet für jede Datei die gepackten Bytes aus und gibt sie an `blast` weiter.

## Kompression: PKWARE „Data Compression Library“ (implode/explode)
`blast.c` ist eine Neuimplementierung des bekannten Algorithmus (Struktur wie zlib/contrib/blast von Mark Adler):
- Byte 0: 0 = Literale unkodiert, 1 = Literale Huffman-kodiert (hier immer 0)
- Byte 1: Wörterbuchgröße 4/5/6 → 1 KB, 2 KB oder 4 KB Fenster (hier immer 6)
- Danach Bitstrom (LSB zuerst): Bit 0 → Literal (8 Bit), Bit 1 → Kopie mit Länge (Huffman-Tabelle `lenlen` + Extrabits) und Distanz (Tabelle `distlen` + 4–6 Bit)
- Länge 519 = Ende des Datenstroms
- Die Huffman-Codes sind invertiert gespeichert (`code |= (bitbuf & 1) ^ 1`), das ist eine PKWARE-Eigenheit.

## Befund: Die Kopie auf der Festplatte ist beschädigt
Bei 7 Dateien stimmte die entpackte Größe nicht. Das Durchsuchen von `setup.z` in 64-KB-Blöcken zeigte: **14 Blöcke sind exakte Kopien des jeweils vorigen Blocks** (bei 0x81C0000, 0x8270000, 0x83F0000, 0x84B0000, 0x8560000, 0x8610000, 0x86C0000, 0x8780000, 0x8830000, 0x88E0000, 0x8A60000, 0x8B20000, 0x8BD0000 und 0x8D30000). Das ist typisch für einen Lesefehler beim Kopieren der CD. Der Originalinhalt dieser Blöcke fehlt. Betroffen sind:

| Datei | Folge |
|---|---|
| `Languages/man_enu.pdf` (Handbuch, 522 S.) | ein Block nahe dem Ende fehlt; **der Text aller 522 Seiten ließ sich trotzdem extrahieren** |
| `Languages/qrg_enu.pdf` (Kurzreferenz) | unlesbar |
| `Help/English/ARTISTA.HLP` (Online-Hilfe) | unlesbar (das Verzeichnis am Dateiende ist zerstört) |
| `Help/English/Quilter.hlp` | unlesbar |
| `Design Explorer/Program/portfolio.exe`, `vcl60.bpl` | beschädigt (Portfolio, für die Analyse unwichtig) |
| `BERNINAartistaPortfolio.hlp.en` | beschädigt; die identische Kopie `BERNINAartistaPortfolio.hlp` ist intakt |

**Empfehlung:** Die CD noch einmal sauber einlesen. Dann lässt sich auch die Online-Hilfe auswerten.
