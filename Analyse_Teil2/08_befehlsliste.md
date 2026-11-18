# 08 – Vollständige Liste aller Befehle an die Maschine

**Quelle:** BEAGLink.dll. Erfasst ist jeder Aufruf der Sendefunktionen (0x100027d0 Zeichen mit Echo, 0x10001a20 Rohzeichen, 0x10002f00 Befehlsfeld, 0x10013710/0x10013870 Binärrahmen) mit seinem Argument (Suche am 18.11.2026 über die komplette Disassembly).
**Legende Status:** ✅ an der artista 185 getestet · 🟦 im Testprogramm v3 (ausgeliefert, Gerätetest ausstehend) · ⬜ nur aus dem Code bekannt
**Legende Risiko:** 🟢 nur lesen / Verbindung · 🟡 Steuerung, verändert keine Designs · 🔴 verändert gespeicherte Daten · ⛔ Firmware/Reset – nie ohne Not

## A. Zeichenprotokoll (artista 165–185, Nähmaschine und Stickmodul)
Alle Zeichen, Adressen und Daten (2 Hexziffern je Byte) werden **mit Echo** gesendet, sofern nicht anders angegeben.

| Befehl | Daten danach | Antwort der Maschine | Bedeutung | Status | Risiko | Code |
|---|---|---|---|---|---|---|
| `R` `F` `?` (ohne Echo-Prüfung) | – | `R`, `F`, `?` | Synchronisieren | ⬜ (nie nötig) | 🟢 | 0x10001d00 |
| `T` `r` `M` `E` | – | Echo | PC-Modus („Transfermodus“) betreten | ✅ | 🟢 | 0x10005e30 |
| `J` | 1 Byte Code: 0x11 = 19200, 0x05 = 57600, 0x02 = 115200 | nach Umschalten `B` `O` `S`, PC sendet `E` `B` | Baudrate wechseln | ✅ (57600) | 🟢 | 0x100057b0 |
| `Y` `Q` | – | `O` | Verbindung zum **Stickmodul** durchschalten | ✅ | 🟢 | 0x10005e30 |
| `R` | 3 Adressbytes | 32 Byte (64 Hexziffern), dann `O` | 32 Byte lesen | ✅ | 🟢 | 0x1000250a |
| `E` | 2 Adressbytes | 32 Byte, `O` | 32 Byte lesen (nur bei 4800 Baud, Adresse < 0x1000) | ⬜ | 🟢 | 0x1000250a |
| `N` | 3 Adressbytes | 256 Byte **binär**, dann `O` | Block lesen (256 Byte) | ⬜ | 🟢 | 0x100028e6 |
| `W` | 3 Adressbytes + n Datenbytes + `?` | Echo jedes Zeichens, `?` nach Abschluss | **Speicher schreiben** (≤ 32 Byte) – Risiko hängt von der Adresse ab, siehe B/D | ✅ (Befehls-/Auswahlfeld, v3) | 🟡/🔴 | 0x10002b20 |
| `C` | 2 Adressbytes + 1 Datenbyte | `O` | 1 Byte schreiben (4800 Baud) | ⬜ | 🔴 | 0x10002b93 |
| `P` `S` | 2 Adressbytes (Bits 23–8) | `O` `E`, dann PC sendet 256 Byte **binär**, Antwort `O` | 256-Byte-Seite schreiben | ⬜ | 🔴 | 0x10002ce0 |
| `X` | – | `B` `O` `S`, PC sendet `E` `B`, Maschine meldet sich neu (`N`/`M`) | **Software-Reset** der Maschine | ⬜ | ⛔ | `SoftwareReset` 0x10005385 |
| `L`, `H`, `Y`/`N` + 256 Byte binär | Flash-Seiten | – | **Firmware flashen** (`ProgramFlash`, Update-Dateien *.A20/*.BIN/*.BE) | ⬜ | ⛔ | 0x10005100–0x10005340 |
| `G` (ohne Echo) | – | – | Programm starten nach BIOS-Modus (`PowerUpWithBIOS`) | ⬜ | ⛔ | 0x1000560e |

## B. Befehlsfeld 0xFFFED0 (über `W FFFED0 hh ll ?`, Wort = Befehl | 1, danach Status lesen)
| Befehl | Bedeutung | Status | Risiko |
|---|---|---|---|
| 0x50 | Speicherziel **Maschinenspeicher** wählen (FW ≥ 2.10) | ✅ Status 0x0002 | 🟡 |
| 0xA0 | Speicherziel **Karte im Stickmodul** wählen | 🟦 (Option `-Karte`) | 🟡 |
| 0x30 / 0x60 / 0xC0 | Bank wählen: Designs 0–26 / 27–53 / ab 54 | 🟦 (0x30 ohne Transfer-Anforderung → 0x1005) | 🟡 |
| 0x20 | Designliste bzw. gewähltes Design in den Übertragungsbereich laden | 🟦 | 🟡 |
| 0x100 | PC-Modus verlassen | 🟦 | 🟡 |
| 0x40 | Löschen vorbereiten | ⬜ | 🟡 |
| 0x10 | Schreiben vorbereiten | ⬜ | 🟡 |
| 0x400 | gespeichertes Design in den Stickbereich laden | ⬜ | 🟡 |
| 0x08 | **Sticken starten** (Design ans Stickmodul übergeben) | ⬜ | 🟡 (Maschine bewegt sich!) |
| 0x200 | **Design dauerhaft speichern** | ⬜ | 🔴 |
| 0x800 | **Design löschen** | ⬜ | 🔴 |
| 0x80 | **Karte formatieren** | ⬜ | 🔴 |

## C. Schreibziele im Speicher (über `W` bzw. `P S`)
| Ziel | Inhalt | Status | Risiko |
|---|---|---|---|
| AUSWAHL +0 / +5 | Designnummer + 1 / Bank | ✅ (Schreiben am Gerät bestätigt) | 🟡 |
| 0xFFFEE0 | Übertragungsbyte: **0x01 = Transfer anfordern**, Maschine setzt Bit 0x02 = bereit; Pflicht vor allen Befehlen außer 0x50 (Gerätetest v3: sonst 0x1005). Test v3.1 bei 57600 Baud: 0x01 gespeichert, Bit 0x02 kam nicht → v3.2 mit 115200 wie Original | 🟦 (v3.2) | 🟡 |
| DATEN | Design-Datenblock (Stiche usw.) | ⬜ | 🔴 (nur zusammen mit 0x10/0x200 dauerhaft) |
| ICON | Vorschaubild | ⬜ | 🔴 |
| INFO + N+1 / 2N+1 / 3N+1 | Farbanzahl, Flags, Name | ⬜ | 🔴 |
| Zeiger 0x24048 | Garnfarbtabelle | ⬜ | 🔴 |

## D. Binärprotokoll (Card Reader/Writer-Box, neuere Firmware) – Rahmen `02 … 03 Prüfsumme`, Antwort `06`
| Befehl | Bedeutung | Status | Risiko |
|---|---|---|---|
| `f` | Geräteinfo (6 Byte) | ⬜ | 🟢 |
| `k` | Gerätename (`BERNINA RWUSER`) | ⬜ | 🟢 |
| `i` | Kartenstatus (3 Byte) | ⬜ | 🟢 |
| `l` + Adresse + Länge | lesen | ⬜ | 🟢 |
| `b` + 3/4 | Baudrate | ⬜ | 🟢 |
| `d` / `p` | Schreibschutz aus / ein | ⬜ | 🟡 |
| `w` + Adresse + Länge + Daten | SRAM schreiben | ⬜ | 🔴 |
| `a` + Adresse + Länge + Daten | Flash (ATMEL) schreiben | ⬜ | ⛔ |
Für die artista 185 nicht nötig: Sie antwortet im Zeichenprotokoll.

**Sicherheit der Bedeutungen:** A (bis auf `X`, `L`, `H`, `G`, `N`, `C`, `P S`) und B (Ziel/Bank/Liste/Verlassen) sind hoch; diese Zeichen sind entweder am Gerät getestet oder ihr Ablauf ist vollständig verfolgt. `X`, `L`, `H`, `G`, `N`, `C`, `P S` sind mittel: Bedeutung aus Funktionsnamen und Ablauf, nicht im Detail verfolgt.
