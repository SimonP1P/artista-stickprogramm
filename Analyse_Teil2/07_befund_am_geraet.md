# 07 – Befund an der echten artista 185 (18.11.2026)

**Quelle:** zwei Lesetests mit dem eigenen Testprogramm über das Original-PC-Kabel an COM3.
Mitschnitte: [`daten/Mitschnitt_artista185_v1_20261118.log`](daten/Mitschnitt_artista185_v1_20261118.log), [`daten/Mitschnitt_artista185_v2_20261118.log`](daten/Mitschnitt_artista185_v2_20261118.log)
**Ergebnis:** Das aus BEAGLink.dll rekonstruierte Protokoll funktioniert ohne einen einzigen Übertragungsfehler.

## Bestätigt
| Punkt aus der Analyse | am Gerät |
|---|---|
| 19200 Baud 8N1, DTR/RTS aus | ✅ Antwort sofort |
| `T r M E` mit Echo je Zeichen | ✅ |
| Lesen: `R` + 6 Hexziffern (Echo), 64 Hexziffern, `O` | ✅ über 30 Lesevorgänge fehlerfrei |
| Big-Endian, 68000-Prozessor | ✅ Adresse 0 = `00 00 03 EC 00 00 18 2E` (Reset-Vektoren) |
| Baudwechsel `J 0 5` → Port auf 57600 → Maschine sendet `B O S` → `E B` | ✅ `B O S` kam ca. 770 ms nach dem Umschalten |
| `Y Q` → Antwort `O` (Stickmodul durchgeschaltet) | ✅ nach ca. 155 ms |
| Firmware-Kennung `SRMV` an 0x200100 | ✅ (vom Stickmodul) |
| Verwaltungsblock an 0xFFFD24 besteht alle Prüfungen der Original-Software | ✅ |

## Neu erkannt
- **Zwei Stufen:** Nach `T r M E` antwortet zuerst die **Nähmaschine**. Erst nach `J 0 5 … Y Q` ist das **Stickmodul** durchgeschaltet; alle weiteren Zugriffe gehen dann an dessen Speicher.
  - Nähmaschine, 0x200100: `NMMV04.04` · `German` · `Fritz Gegauf AG` · `May 01`
  - Stickmodul, 0x200100: `SRMV04.04` · `Fritz Gegauf AG` · `Janu 04` → Version-Code **0x404** (≥ 3.09, also alle Funktionen inkl. Garnfarbtabelle)
- **Verwaltungsblock des Stickmoduls** (0xFFFD24):
```
1C 3C | 00 01 B3 42 | 00 02 40 80 | 00 02 01 EA | 00 02 44 80 | 00 02 8E 9A | 00 01 FD 7A | 00 02 38 12
```
| Feld | Wert | Deutung (korrigiert) |
|---|---|---|
| N | 28 (0x1C) | Höchstwert, den die Software zulässt |
| Byte 1 | 60 (0x3C) | Höchstwert, den die Software zulässt |
| +0x02 | 111 426 Byte | **Grundkapazität** Designspeicher |
| +0x06 INFO | 0x024080 | Zeiger (RAM) |
| +0x0A AUSWAHL | 0x0201EA | Zeiger (RAM) |
| +0x0E ICON | 0x024480 | Zeiger (RAM) |
| +0x12 DATEN | 0x028E9A | Zeiger (RAM) |
| +0x16 | 130 426 Byte | **Kapazität Kartenmodus** (vorher als „Speicherende“ bezeichnet; es ist eine Größe) |
| +0x1A | 145 426 Byte | **Kapazität Maschinenspeicher** (ebenso) |

  Laut Code (0x100034e3) nimmt die Schreibroutine das Maximum aus Grundkapazität und der Kapazität des gewählten Ziels. **Ein Design darf damit höchstens etwa 145 KB groß sein.**
- **Zeiger 0x24004 → 0xFFFED4**, also in den Ein-/Ausgabebereich direkt neben dem Befehlsfeld 0xFFFED0. Statusbytes: `00 00 00 00 00 86 00 00`. Byte+5 = 0x86 (Bit 0x02 gesetzt → „Sticken“ möglich), Byte+7 = 0x00.
- **INFO+0 = 0:** Das ist **keine** Design-Anzahl. Der INFO-Bereich wird erst gefüllt, nachdem die Software Ziel und Bank wählt (Befehle 0x50/0xA0, 0x30) und den Ladebefehl 0x20 gibt. Ohne diese Befehle steht dort 0.

## Nächster möglicher Schritt
Um die Designliste (Anzahl und Namen) zu lesen, muss man wie die Original-Software Befehle an das Befehlsfeld 0xFFFED0 schreiben (Ziel 0x50, Bank 0x30, Laden 0x20) und die Auswahl (Index+1) nach AUSWAHL schreiben. Das sind Steuer-Schreibzugriffe, keine Änderungen an gespeicherten Designs.

## Test v3 – Designliste (18.11.2026, 21:56, Mitschnitt `daten/Mitschnitt_artista185_v3_20261118.log`)
| Schritt | Ergebnis | Sicherheit |
|---|---|---|
| Verbindung, NMMV04.04 → SRMV04.04, Verwaltungsblock | wie v2, identisch | hoch |
| Schreibprotokoll `W` + Adresse + Daten + `?` | **funktioniert**: jedes Zeichen kommt als Echo, `?` wird bestätigt | hoch (Gerätetest) |
| Befehl 0x50 (Ziel Maschinenspeicher) | Status 0x0002 = **ok** | hoch |
| AUSWAHL+0 = 1, AUSWAHL+5 = 0 | geschrieben, Echo ok | hoch |
| Befehl 0x30 (Bank 0) | sofort Status **0x1005** (hi 0x10 = „keine Speicherkarte“, lo Bit 0x04 = Fehler) | hoch |
| Befehl 0x20 (Liste laden) | ebenfalls 0x1005 → abgebrochen | hoch |
| 0xFFFEE0 (Übertragungsbyte) | **0x00** – Transfer war **nicht angefordert** | hoch |
| Status-Bereich 0xFFFED4 | jetzt `00 40 00 00 00 86 00 00` (Byte +1 = 0x40, bei v2 noch 0x00) | hoch (Bedeutung offen) |

**Ursache (hoch, Code 0x10006070 / 0x10005de9):** Die Original-Software ruft nach erfolgreicher SRMV-Erkennung `TransparentMode(1)` auf: optional ChangeBaudrate(115200), dann **0x01 nach 0xFFFEE0 schreiben** und alle 10 ms 0xFFFEE0 lesen, bis **Bit 0x02** gesetzt ist (max. 5000 ms, sonst Fehler 17). Erst danach kommen die Befehle. v3 hat diesen Schritt ausgelassen. **Testprogramm v3.1** holt ihn nach; Gerätetest ausstehend.

## Test v3.1 – Transfer anfordern (18.11.2026, 23:35, Mitschnitt `daten/Mitschnitt_artista185_v3_1_20261118.log`)
| Schritt | Ergebnis | Sicherheit |
|---|---|---|
| 0xFFFEE0 vorher | 0x00 | hoch |
| `W FFFEE0 01 ?` | angenommen, Echo ok; 0xFFFEE0 liest danach **0x01** | hoch |
| 5 s Abfrage alle ~10 ms | Bit 0x02 **nie gesetzt**, Wert bleibt 0x01 → Original-Fehler 17 | hoch |
| Befehle | keine gesendet; 0x100 nicht nötig (Bit 0x02 fehlt) | hoch |

**Unterschiede zum Original (Code 0x10005de9):** (1) Das Original wechselt unmittelbar vor `TransparentMode(1)` auf **115200 Baud** (`ChangeBaudrate(0x1C200)`), v3.1 blieb auf 57600. (2) Handbuch S. 262 (PDF 264/265): „Make sure the needle arm is up while downloading“, „The machine touch screen goes blank while the download takes place“ – die Maschine prüft vermutlich ihren Zustand (Nadel oben, kein Menü), bevor sie Bit 0x02 setzt (Vermutung, niedrig). **v3.2** wechselt auf 115200, wartet 30 s und bittet um Beobachtung des Displays.

