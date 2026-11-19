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
→ **Überholt durch die Code-Nachprüfung vom 19.11.2026 (siehe unten):** Der 115200-/0xFFFEE0-Weg gilt für die artista 185 gar nicht.

## Test v3.2 – 115200 Baud (19.11.2026, 00:56, ohne Person vor Ort, Mitschnitt `daten/Mitschnitt_artista185_v3_2_20261119.log`)
Selbstständig von Claude Code auf dem Windows-PC ausgeführt (Regel 1a). Niemand vor Ort: Nadelstellung und Display unbekannt.
| Schritt | Ergebnis | Sicherheit |
|---|---|---|
| Ausgangszustand | Maschine stand noch im PC-Modus auf **57600** (nach v3.1 nicht aus/ein geschaltet) – Synchronisierung bei 57600 gelang, danach normal weiter | hoch |
| NMMV04.04 → `J 0 5`/`Y Q` → SRMV04.04, Verwaltungsblock | wie v2/v3, identisch | hoch |
| Status @0xFFFED4 | `00 40 00 00 00 86 00 00` (Byte +1 = 0x40 seit v3, Maschine nicht neu gestartet) | hoch |
| `J 0 2` (115200) nach `Y Q`, also an das **Stickmodul** | `J`, `0`, `2` als Echo bestätigt | hoch |
| danach bei 115200 | **kein `B O S`**, stattdessen etwa alle 430 ms ein Bündel unlesbarer Bytes (`06 0C 06 0C 7E F0 E0 …`) → Abbruch Fehler 17 | hoch |
| Passives Mithören 4800 … 460800 Baud (PC sendet nichts) | bei **keiner** Rate lesbar; Simulation der Fehlabtastung passt zu keiner Senderate, auch nicht zu „BOS“ (`Arbeitsdaten/geraetetests/Mithoeren_20261119_nach_v3_2.txt`) | hoch (Beobachtung), Ursache offen |
| Geschrieben | **nichts** – kein `W`, kein Befehlsfeld, kein 0xFFFEE0 (Mitschnitt: 0 × `W`) | hoch |

**Zustand danach:** Die Verbindung hängt; die Maschine muss **aus- und wieder eingeschaltet** werden. Weiteres Senden wurde bewusst unterlassen, weil Zeichen bei falscher Rate beim Modul als zufällige Befehle ankommen könnten.

### Code-Nachprüfung: 115200 und 0xFFFEE0 gelten nicht für die artista 185 (hoch, Code)
`MaschineEinlesen` 0x10005980: Beim ersten Kontakt `TransferModus(0,0)` (TrME ohne Baudwechsel), dann Flag **0x10072024 = 1** und 32 Byte ab 0x200100 lesen. Steht dort **nicht** „SRMV“ (artista 185: „NMMV“), setzt das Original **0x10072024 = 0** (0x10005a51) und betritt den Modus neu mit `TransferModus(1,1)` = TrME → `ChangeBaudrate(57600)` → `Y Q` → `O` (0x10005e30).
Bei jedem späteren `TransparentMode(1)` (0x10005de0) gilt: Flag 0x10072024 = 0 → **0x10005e10 → wieder TrME/57600/`Y Q`**. Nur bei Flag = 1 (Gerät antwortet **direkt** mit SRMV) folgt `ChangeBaudrate(115200)` + 0x01 → 0xFFFEE0 + Warten auf Bit 0x02 (0x10005de9 → 0x10006070).
→ **Für die artista 185 schreibt das Original 0xFFFEE0 nie und wechselt nie auf 115200.** v3.1/v3.2 beruhten auf einer falschen Annahme. Das erklärt auch, warum Bit 0x02 in v3.1 nie kam.

### Code-Nachprüfung: Ziel 0x50 ist vermutlich die **Karte**, nicht der Maschinenspeicher (mittel)
- Listenfunktion 0x100064e0 → 0x10004d10(…, Index 0): Ziel (0x10004de0: **0x50 wenn Flag 0x100670c8 = 0, sonst 0xA0**, Fehlerauswertung aus), AUSWAHL+0 = Index+1, AUSWAHL+5 = Index/27, Bank **0x30** (Index < 27; 0x60 für 27–53, 0xC0 ab 54; Fehlerauswertung aus), dann **0x20** mit Fehlerauswertung. Das ist genau die Folge aus v3 – die Reihenfolge war richtig.
- Befehlsfeld 0x10002f00, Fehlerauswertung (0x10002fcf): Statusbit **0x10 (High-Byte)** → bei Flag 0x100670c8 = 0 (Ziel **0x50**) **Fehler 22 „No memory card in the embroidery modul“**, bei Flag = 1 (0xA0) Fehler 3.
- v3 bekam auf 0x20 genau Status 0x1005 → das Original hätte hier „No memory card in the embroidery modul“ gemeldet. Das passt nur, wenn **0x50 = Karte im Stickmodul** ist (und keine Karte steckt) – umgekehrt zur bisherigen Annahme in 01/02/03/08. Welche Optionsschaltfläche Flag 0x100670c8 = 1 setzt (0x1000df0f, 0x1000ec6c, BM_GETCHECK), ist noch nicht über die Dialog-Ressource geprüft.
- **Nächster Gerätetest: v3.3** (`Testprogramm_artista185_Windows_v3_3/`, gebaut 19.11.2026, Simulation ok): nach Aus/Ein Verbindung wie v3 (57600, `Y Q`), **ohne** 0xFFFEE0 und ohne 115200; Ziel **0xA0**, dann **0x50**, je AUSWAHL +0 = 1 / +5 = 0, 0x30, 0x20; am Ende 0x100. Erwartung bei richtiger Vermutung: 0xA0 liefert die Liste, 0x50 ohne Karte Status 0x1005. Alle Schritte durch Regel 1a abgedeckt (Mitschnitt der Simulation: nur `W FFFED0` mit 00A1/0051/0031/0021/0061/0101 und `W 0201EA`/`W 0201EF`).

## Test v3.3 – Designliste aus dem Maschinenspeicher (19.11.2026, 13:20, Mitschnitt `daten/Mitschnitt_artista185_v3_3_20261119.log`)
Selbstständig von Claude Code auf dem Windows-PC ausgeführt (Regel 1a; Selbsttest vorher ok). Maschine vorher aus/ein
(Verbindung bei 19200 Baud). Dateien: `Arbeitsdaten/geraetetests/*_v3_3*`.
| Schritt | Ergebnis | Sicherheit |
|---|---|---|
| Verbindung TrME (19200) → NMMV04.04 → `J 0 5`/`Y Q` → SRMV04.04, Verwaltungsblock | wie v2/v3, identisch; **ohne** 0xFFFEE0 und 115200 | hoch |
| Ziel **0xA0**, AUSWAHL +0 = 1 / +5 = 0, Bank 0x30, Laden 0x20 | jeweils Status **0x0002** | hoch |
| Anzahl Designs | **51** – Bank 0: Nr. 1–27, Bank 1 (Befehl 0x60): Nr. 28–51 → 27 Designs je Bank wie im Code (Index/27) | hoch |
| Namen | z. B. „Drifterv45Rev7“, „Outdoorsman - 805023“, sonst Nummern „1“ … „30“; Statusbyte 0x63 (die Original-Software ersetzt Namen dann durch Nummern) | hoch |
| Flags je Design | 0xAC, 0x8C, 0x84, 0xA4, 0x86 (Bedeutung offen) | – |
| Ziel **0x50** | Status 0x0002; danach 0x30 und 0x20 → **0x1005** „keine Speicherkarte im Stickmodul“ (keine Karte gesteckt) | hoch |
| Befehl **0x100** (PC-Modus verlassen) | Befehlsfeld danach **0x0000** – angenommen, aber ohne Fertig-Bit 0x02 (das Testprogramm meldet deshalb „nicht bestätigt“) | hoch |
| **Zweiter Durchlauf ohne Aus/Ein** (13:21, nur Ziel 0xA0) | TrME bei 19200 ohne Antwort → Synchronisation bei **57600** → alles wie im ersten Lauf, wieder 51 Designs; 0x100 wieder 0x0000 | hoch |

**Folgerungen:**
- **0xA0 = Maschinenspeicher, 0x50 = Karte im Stickmodul** – die Vermutung aus der Code-Nachprüfung ist am Gerät bestätigt
  (Sicherheit jetzt hoch). Die Designliste lässt sich lesen; der Weg für Phase 2 ist offen.
- **Kein Neustart nötig:** Nach 0x100 bleibt die Maschine auf 57600 Baud, ein neuer Verbindungsaufbau (Synchronisation bei
  57600, dann TrME) gelingt. Ob das Display nach 0x100 wieder den normalen Bildschirm zeigt, konnte niemand ansehen – bei
  der nächsten Gelegenheit vor Ort prüfen.
- Testprogramm: 0x100 künftig mit Status 0x0000 als „angenommen“ werten.
