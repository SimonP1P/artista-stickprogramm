# Analyse Teil 2: Maschinenkommunikation im Detail (BEAGLink.dll)

**Stand:** 18.11.2026 · Fortsetzung von [`../Analyse/ANALYSE.md`](../Analyse/ANALYSE.md)
**Gegenstand:** `BEAGLink.dll` aus `ADX/data1.cab`. Das ist die Bibliothek, über die die Software mit der **artista 165/170/180/185** (Stickmodul, Karte im Modul) und der **Embroidery Card Reader/Writer-Box** spricht.
**Methode:** komplette Disassembly (75 967 Zeilen, [`daten/BEAGLink_komplett.asm`](daten/BEAGLink_komplett.asm)), Verfolgung der Aufrufketten, Übersetzung der Kernfunktionen. Alle Aussagen beziehen sich auf Code-Adressen; die zugehörigen Funktionen liegen einzeln unter [`daten/funktionen/`](daten/funktionen/).
**Grenze:** Nichts davon ist an einer echten Maschine getestet. Die Bedeutung von Feldern ist aus der Verwendung abgeleitet. Jede Datei nennt die Sicherheit.

## Ergebnisse auf einen Blick

### Wie sich die Maschine erkennen lässt → [01_verbindung_und_erkennung.md](01_verbindung_und_erkennung.md)
- Port 19200 8N1. Synchronisieren mit `'R'`-Echo, dann `'F'` und `'?'`. Bis zu 48 Versuche, die Baudrate rotiert 4800 → 19200 → 57600 → 115200.
- **R/W-Box / neuere Geräte:** Binärabfragen `'f'` (6 Byte Geräteinfo), `'k'` (Name **„BERNINA RWUSER“**), `'i'` (Kartenstatus).
- **Stickmodul:** Transfermodus mit `'T' 'r' 'M' 'E'`, Firmware-Kennung **„SRMV x.yy“ an Adresse 0x200100**, Verwaltungsblock an **0xFFFD24**.

### Welche Adressen in welcher Reihenfolge → [02](02_maschinenspeicher_und_kommandos.md) und [03](03_design_schreiben_schritt_fuer_schritt.md)
- **Briefkasten 0xFFFED0:** 16-Bit-Kommando schreiben (0x10 vorbereiten, 0x200 speichern, 0x400 laden, 0x800 löschen, 0x80 formatieren, 0x08 sticken, 0x30/0x60/0xC0 Bank, 0x50/0xA0 Ziel), Status abfragen.
- Schreiben: Kommando 0x10 → Datenblock an **DATEN** → Icon an **ICON** → Farbanzahl, Flags und Name an **INFO + (N+1) / (2N+1) / (3N+1)** → (FW ≥ 3.09) Farbtabelle über Zeiger **0x24048** → Kommando 0x200.

### Was die Felder im Kartenspeicher bedeuten → [04_kartenabbild_felder.md](04_kartenabbild_felder.md)
- 128-KB-Abbild: Anzahl bei 0x30, Verzeichnis D (0x32), Datenbereich E (0x36), Belegungstabelle ab 0x48 (1 Bit je 256-Byte-Block), 64-Byte-Verzeichniseinträge (belegt, Farben−1, Datenadresse, Flags, Name 31 Zeichen, 0xFF).

### Designcontainer und Verschleierung → [05_designcontainer_verschleierung.md](05_designcontainer_verschleierung.md)
- Chunks 0x0A Kopf, 0x14 Farben, 0x1E Stiche, 0x28 Zusatz, 0x32 Icon. Rollierendes 8-Byte-XOR mit Startwert „BE“. Python-Nachbau: [`code/be_scramble.py`](code/be_scramble.py).

### Der Disassembler → [06_disassembler_anleitung.md](06_disassembler_anleitung.md)
- Installation, PE-Grundlagen, alle Befehle von `rdis.py`, `asmq.py`, `callers.py`, x86-Lesehilfe mit den Mustern aus dieser Analyse, durchgespielte Beispiele, Grenzen.

### Am echten Gerät bestätigt → [07_befund_am_geraet.md](07_befund_am_geraet.md)
- Lesetests an der artista 185 (18.11.2026): Erkennung, Umschalten zum Stickmodul und Speicheradressen funktionieren fehlerfrei.

### Alle Befehle an die Maschine → [08_befehlsliste.md](08_befehlsliste.md)
- Zeichenprotokoll, Befehlsfeld, Schreibziele, Binärprotokoll – mit Risiko und Teststatus.

## Inhalt des Ordners
| Pfad | Inhalt |
|---|---|
| `01_…06_*.md` | Erklärungen (siehe oben) |
| `code/rdis.py` | Disassembler (neu: Befehl `full`) |
| `code/asmq.py` | Abfragen auf der Gesamtliste |
| `code/callers.py` | Aufrufer finden |
| `code/be_scramble.py` | BE-Verschleierung + Chunk-Zerlegung, mit Selbsttest |
| `daten/BEAGLink_komplett.asm` | vollständige kommentierte Disassembly |
| `daten/funktionen/*.asm` | die 55 in den Texten zitierten Funktionen einzeln |

## Noch offen
- **Echte Prüfung:** Ein Mitschnitt der seriellen Leitung (oder ein Kartenabbild) würde die Feldbedeutungen bestätigen. Mit [`code/be_scramble.py`](code/be_scramble.py) und den Adressen hier lässt sich ein Mitschnitt direkt dekodieren.
- **Inhalt der Felder `Tabelle A/B/C` (N × u16)** und der drei Statusbytes aus `'i'` sind nur der Lage nach bekannt.
- Die **artista 200** (USB/ActiveSync) ist in Teil 1 beschrieben ([`../Analyse/code/03_artista200_adx.md`](../Analyse/code/03_artista200_adx.md)). Sie nutzt ein ganz anderes Verfahren (Dateien statt Speicheradressen).
