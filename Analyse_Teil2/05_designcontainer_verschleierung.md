# 05 – Der Designcontainer („BE“-Format) und seine Verschleierung

**Code:** [`code/be_scramble.py`](code/be_scramble.py) (Python-Nachbau mit Selbsttest); Rust: `kern/stick-formate/src/artista.rs` (Container, Puffer DATEN, Kopf, Farbtabelle – Rundweg mit allen 51 Maschinendesigns byte-gleich)
**Quelle:** BEAGLink.dll 0x1000a370 (Startzustand), 0x1000a4c0 (Byte-Kodierung), 0x1000a640 (ganzer Container), 0x10007590/0x10007600 (Chunk-Suche)
**Sicherheit:** hoch für den Algorithmus (Befehl für Befehl übersetzt, Hin- und Rückweg identisch). **Am echten Gerät bestätigt (19.11.2026):** Der aus der artista 185 gelesene Anhang von Design Nr. 1 (19 752 Byte) ergibt entschleiert die Chunks 0x0A (Name „Drifterv45Rev7“) und 0x14 (145 Farben) – siehe [07](07_befund_am_geraet.md), `Arbeitsdaten/maschinendesigns/`.

## Containeraufbau
Diesen Datensatz übergibt die Stick-Software an BEAGLink (und BEAGLink baut ihn beim Lesen wieder auf):
```
Byte 0..1   u16   (Kennung/Version)
Byte 2..5   u32   Gesamtlänge L  (Little-Endian)
Byte 6..7   u16
Byte 8..L-1 VERSCHLEIERT: Folge von Chunks
              u16 id | u32 länge (inkl. dieser 6 Byte) | daten
```
| Chunk-ID | Inhalt | Länge |
|---|---|---|
| 0x0A | Kopf (Name ab +0x14, Sammlung +0x94, Farbanzahl +0x114, Funktionsfelder +0x116…+0x126, Breite/Höhe +0x12C/+0x12E, Wert +0x130) – Felder siehe [07](07_befund_am_geraet.md), Auswertung 19.11.2026 | 0x134 |
| 0x14 | Farbtabelle: 6 Byte Kopf + n × 0x86 Byte (R, G, B, 0, u16 Garnnummer, Marke 64 Byte, Name 64 Byte) | 6 + n·0x86 |
| 0x1E | Stichdaten (EXP-artig, Ende `80 81`) | variabel |
| 0x28 | Zusatzdaten | variabel |
| 0x32 | Icon: u8 Breite, u8 Höhe, 1-Bit-Bitmap | 10 + Bitmap |
Beim Lesen setzt BEAGLink als Platzhalternamen `"Temp"` ein.

## Verschleierung
**Startzustand (0x1000a370):** n = L − 8, a = n mod 255, b = (n >> 8) mod 255.
```
s[0] = 'B' (0x42),  s[1] = 'E' (0x45)
s[2..7] je nach n (vier Bereiche n ≤ 0xFFF, ≤ 0x3FFF, ≤ 0xCFFF, darüber) aus a, b und Konstanten
danach: s[i] ^= (i << 3)           für i = 0..7
Startindex i = (n >> 1) & 7
```
**Pro Byte ab Position 8 (0x1000a4c0):**
```
k   = s[(i+2)&7] ^ s[(i-1)&7] ^ s[(i+1)&7]
out = in ^ k
s[i] = Chiffretext-Byte          (beim Entschlüsseln = in, beim Verschlüsseln = out)
i   = (i - 3) & 7
```
Das ist eine Stromchiffre mit Rückkopplung des Chiffretexts (CFB-artig). Entschlüsseln und Verschlüsseln unterscheiden sich nur darin, welches Byte in den Zustand übernommen wird (Flag bei Objekt+0x0E).

## Wann wird was verschleiert?
- **Beim Laden einer Designdatei** (0x10008e70): Chunks werden byteweise entschlüsselt gelesen.
- **Beim Schreiben in die Maschine** (0x100032b0, Schritt 6): Stichdaten gehen **im Klartext** in den Datenbereich. Der angehängte Rest-Container (Farben usw.) wird **verschlüsselt** übertragen (0x1000a640, Modus 1).
- **Beim Lesen aus der Maschine** (0x10004310): Der Anhang wird wieder entschlüsselt (Modus 0).
