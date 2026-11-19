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

### Nächster Schritt: ein Design lesen – Ablauf des Originals (hoch, Code) und Testprogramm v3.4
`_RetrieveDesign` (Export, 0x10009b30 → 0x10009880) öffnet den Auswahldialog „Card/Machine Read“ (Handbuch PDF-S. 266);
gelesen wird in **0x10004310** (Design-Index in esi):
1. 0x10004d10(0, Index): Ziel (0x10004de0), AUSWAHL +0 = Index+1, +5 = Index/27 (0x10004c90 → 0x10004ce0),
   Bankbefehl 0x30/0x60/0xC0 (1000/4000), dann **0x20** (1000/2000), INFO+0 = Anzahl lesen.
2. 0x10004080: Statusbyte (Zeiger @0x24004, +7) lesen. Index < Anzahl prüfen.
3. 0x10004c90(Index): AUSWAHL +0/+5 erneut.
4. **Befehl 0x400** (Zeiten 3000/4000, ohne Fehlerauswertung des Ziels) – lädt das gewählte Design in den
   Übertragungspuffer **DATEN**.
5. Lesen: Kopf **6·N+8 Byte** ab DATEN (N = Verwaltungsblock +0, bei der 185: 28 → 176 Byte), 6 Byte ab ICON
   (Bildgröße = Byte 2 × Byte 3 + 10), Länge 1 = u32 BE bei Kopf+6N, Länge 2 = u32 BE bei Kopf+6N+4; Prüfung
   Länge 1 > 2 und Länge 1 + Länge 2 + Bild + 0x142 < Größe Datenbereich (sonst Fehler 19); dann **Länge 1 + Länge 2
   Byte ab DATEN + Kopf**.
6. Danach Entschleierung/Prüfung im PC (0x1000a640, 0x10001400, vgl. [05](05_designcontainer_verschleierung.md)).

**0x400 ist nach Regel 1a nicht freigegeben.** `Testprogramm_artista185_Windows_v3_4/` (Simulation ok, Mitschnitt der
Simulation: nur `W FFFED0` mit 00A1/0031/0061/0021/0401/0101 und `W 0201EA`/`W 0201EF`) sendet 0x400 nur mit dem
Schalter `-MitLadebefehl` (`START_DESIGN_LESEN.bat`) – erst nach Freigabe durch den Nutzer.


## Test v3.4 / v3.5 – ein Design lesen (19.11.2026, 13:32–13:52, Dateien `Arbeitsdaten/geraetetests/*_v3_4*`, `*_v3_5*`)
Von Claude Code auf dem Windows-PC ausgeführt. **0x400 vom Nutzer einmalig freigegeben** („nur dieses eine Mal“, für Design Nr. 1).
| Lauf | Ablauf | Ergebnis | Sicherheit |
|---|---|---|---|
| 1 (13:32, v3.4 `-MitLadebefehl`) | Verbindung bei 57600 (Maschine noch im PC-Modus vom v3.3-Test), Liste 51 Designs, dann Ziel 0xA0 → AUSWAHL 1/0 → 0x30 → 0x20 → AUSWAHL erneut → **0x400** | alle Befehle Status **0x0002**, auch **0x400** (Antwort nach ~2,5 s). Kopf ab DATEN (176 Byte): Tabellen A–C alle **0**, Länge 1 = **103 432**, Länge 2 = **19 752**, Icon 9 Byte × 62 Zeilen (72 × 62 Pixel) → 568 Byte. Danach 0x100 → 0x0000 | hoch |
| | Längenprüfung | 103 432 + 19 752 + 568 + 0x142 = 124 074 ≥ 111 426 („Größe Datenbereich“) → v3.4 hat abgebrochen. **Fehler im Testprogramm, nicht in der Maschine:** Das Original (0x100044ad–0x100044e3) nimmt beim Maschinenspeicher das Maximum aus +0x02 und **+0x1A (145 426)** als Grenze | hoch (Code) |
| 2 (13:33, v3.4 `-NurPuffer`, nur Lesen) | neu verbunden ohne Aus/Ein, nur Puffer gelesen | Kopf: Länge 1 weiter **103 432**, Länge 2 jetzt **0** (der Anhang-Eintrag wurde nach 0x100/Neuverbindung gelöscht). Lesen mit `R` (32 Byte, ~250 Byte/s wegen 16 ms Echo-Laufzeit je Zeichen über den FTDI-Adapter) brach nach **52 224 Byte** ab: kein Echo auf eine Adressziffer (1 s). Ohne Wiederholung und ohne Zwischenspeicher → Daten verloren | hoch |
| 3 (13:52, v3.5 `-NurPuffer`) | **PC war zwischendurch aus, Maschine wurde aus/ein geschaltet** (Verbindung wieder bei 19200) | Kopf und beide Längen **0** – der Übertragungspuffer DATEN überlebt das Ausschalten nicht. Statusbyte @0xFFFED4+1: 0x40 nach 0x400, 0x00 nach Neustart | hoch |

**Folgerungen:**
- **0x400 funktioniert an der artista 185** und lädt das gewählte Design in DATEN (Status 0x0002). Lesen muss **in derselben
  Sitzung direkt nach 0x400** geschehen – nach Aus/Ein ist der Puffer leer, nach 0x100 fehlt schon die Länge 2.
- Design Nr. 1 („Drifterv45Rev7“): 103 432 Byte Stichdaten (inkl. Endkennung, nach 03 = Stichlänge + 2), 19 752 Byte Anhang
  (verschleierter Container, vgl. 05), Icon 72 × 62.
- **Testprogramm v3.5** (`Testprogramm_artista185_Windows_v3_5/`): liest große Bereiche wie das Original (ReadComBytes
  0x10002040): erstes Stück mit `R`, dann **`N`-Blöcke zu 256 Byte binär** (0x10002880: `N` + 3 Adressbytes mit Echo,
  256 Byte je ≤ 3 s, dann `O`), Rest mit `R`; bei Fehler Neusynchronisation `R F ?` (gleiche Baudrate) und bis zu 5
  Wiederholungen wie das Original (0x100022a0); alles Gelesene geht sofort in `Teil_Design_*.bin`. Selbsttest mit
  simuliertem Echo-Verlust ok (Daten byte-genau). `N` ist am Gerät noch **nicht** getestet (Lauf 3 kam wegen leerem Puffer
  nicht bis zum Datenlesen).
- Nächster Schritt: v3.5 mit `-MitLadebefehl` (0x400 **neu freigeben lassen**) – Design Nr. 1 in einem Durchgang lesen.

## Test v3.5 – Design Nr. 1 vollständig gelesen (19.11.2026, 13:55, `Arbeitsdaten/geraetetests/*_135532_v3_5*`)
0x400 vom Nutzer **für Lesetests generell freigegeben** (Regel 1a). Maschine frisch eingeschaltet (Verbindung bei 19200).
| Schritt | Ergebnis | Sicherheit |
|---|---|---|
| Ziel 0xA0 → Auswahl 1/0 → 0x30 → 0x20 → Auswahl → **0x400** | alle Status 0x0002; Kopf wie in Lauf 1: Tabellen 0, Länge 1 = 103 432, Länge 2 = 19 752 | hoch |
| Lesen mit **`N`** (256 Byte binär) | **funktioniert an der 185**: 123 184 Byte in 75 s (**1 642 Byte/s**, 6,5 × schneller als `R`), **0 Wiederholungen**; Icon 568 Byte | hoch |
| Prüfung | Endkennung **`80 81`** genau an Byte 103 430–103 431 der Stichdaten; Kopf + Länge 1 + Länge 2 = Dateigröße | hoch |
| 0x100 | Status 0x0000 wie bisher | hoch |

**Auswertung** (`Arbeitsdaten/werkzeuge/maschinen_design.py`, Ergebnis `Arbeitsdaten/maschinendesigns/`):
- **Stichdaten = EXP-Kodierung** (dx, dy in 0,1 mm, `80 04` Sprung, `80 01 00 00` Farbwechsel, Ende `80 81`): 51 113 Stiche,
  157 Sprünge, **144 Farbwechsel → 145 Farben = Wert der Farbspalte der Designliste** (INFO+N+1 = Farbanzahl − 1, bestätigt).
  Beginn mit Vernähstichen `08 00 08 00 F8 00 F8 00`. Die Vorschau zeigt saubere Satin-Buchstaben A, B, C … (hoch).
- Design Nr. 1 „Drifterv45Rev7“ ist ein **ganzer Stick-Zeichensatz** („Drifter“): jeder Buchstabe ein Farbblock, alle in einer
  Reihe (2 164 mm × 40 mm). Die Designs 2–7 (144–149 Farben) sind vermutlich weitere Zeichensätze (mittel).
- **Anhang = BE-Container, Entschleierung aus 05 am echten Gerät bestätigt:** Kopf u16 0, u32 Länge 19 752 (= Länge 2),
  nach `be_scramble.transform(…, encrypt=False)` zwei Chunks: **0x0A Kopf (0x134 Byte, Name „Drifterv45Rev7“ ab +0x14)** und
  **0x14 Farbtabelle (6 + 145 × 0x86 Byte, alle Einträge 0** – keine Garninformation gespeichert). Kein Chunk 0x28/0x32.
- Icon: Kopf `01 00 09 3E FF 00 00 00 00 00`, dann 9 × 62 Byte 1-Bit-Bitmap (72 × 62 Pixel, Bit gesetzt = schwarz); zeigt die
  ersten Buchstaben.
- Damit ist **Phase 2 „ein echtes Design auslesen“ erreicht.** Nächste Schritte: Designs 2–51 lesen (Sammlung echter
  Maschinendaten, Vergleich mit eigenem EXP), dann der Weg zurück (Schreiben 0x10/0x200 oder Sticken 0x08 – nur nach Zustimmung).

## Test v3.6 – alle 51 Designs gelesen (19.11.2026, 14:32–14:58, `Arbeitsdaten/geraetetests/*_v3_6*`)
Ein Durchgang für Nr. 2–51 (eine Verbindung, je Design Ziel 0xA0 → Auswahl → Bank 0x30/0x60 → 0x20 → Auswahl → 0x400 →
Puffer lesen), danach Nr. 5 einzeln. Ergebnis: `Arbeitsdaten/maschinendesigns/` (`UEBERSICHT.md`, `kontaktbogen.png`).
| Befund | Einzelheiten | Sicherheit |
|---|---|---|
| **Alle 51 Designs gelesen** | 49 von 50 im Durchgang in 24 min, **0 Wiederholungen**; jedes Mal Status 0x0002; Endkennung `80 81` bei allen | hoch |
| Inhalt | Nr. 1–7: **7 Stick-Zeichensätze** (je 144–149 Zeichen als Farbblöcke, 2,2–3 m breit, 40–49 mm hoch); Nr. 8–41: Motive (Ente, Holzpferd, Lok, Teddy, „Sunflower“, Oldtimer, Hirsch, Fisch …, bis 128 × 178 mm); Nr. 42–51: kleinere Designs (Trolle, Uhr, Monogramm „M“, Kreis/Rechteck-Formen) | hoch |
| **Nr. 8 = Nr. 34** | Stichdaten byte-gleich („Outdoorsman - 805023“ zweimal gespeichert) | hoch |
| **Länge 2 wird bei 0x400 nicht zurückgesetzt** | Hat ein Design keinen Anhang, bleibt im Kopf (DATEN+6N+4) die Anhanglänge des **vorher** geladenen Designs stehen. Nr. 5 zeigte im Durchgang 20 288 (= Nr. 4) → Längenprüfung 153 514 ≥ 145 426 → nicht gelesen (wäre im Original Fehler 19). Einzeln nach neuer Verbindung: Länge 2 = 0, Nr. 5 lesbar (132 336 Byte). Ebenso Nr. 42–51: 1126 (= Nr. 41), Containerkopf dort Unsinn. Ein neuer Verbindungsaufbau setzt Länge 2 auf 0 (vgl. Test v3.4 Lauf 2) | hoch |
| Anhang echt? | Prüfung: u32 LE bei Anhang+2 = Länge 2. **Echt bei Nr. 1, 4, 6–41**; **kein Anhang** bei Nr. 2, 3, 5 und 42–51 (Flags 0x86 in der Liste – alle Designs ohne Anhang dort; Nr. 2/3/5 haben Flags 0x8C wie 4/6/7) | hoch |
| Farbtabelle (Chunk 0x14) | Chunk = 6 Byte Chunkkopf + **n × 0x86, n = Farbanzahl** (= Farbwechsel + 1) bei allen 39 echten Anhängen; alle Einträge 0 (keine Garnangaben gespeichert) | hoch |
| Name im Anhang | Chunk 0x0A enthält denselben Namen wie die Liste („Drifterv45Rev7“, „Outdoorsman - 805023“ oder Nummern „1“ … „30“) | hoch |
| Spalte „Farben?“ der Liste (INFO+N+1) | stimmt nur für Nr. 1–7 (145/144/148/149/146/147/147 = Farben der Stichdaten); Nr. 8–27 zeigen 1 (echte Farben 3–17), Bank 1 zeigt die Werte von Bank 0 → Bedeutung bzw. Leseadresse dieser Spalte offen | mittel |

**Folgerungen:**
- Beim Lesen mehrerer Designs in einer Verbindung den Anhang immer prüfen (Containerlänge) – v3.6 tut das jetzt
  („Anhang UNGÜLTIG … Rest des vorher geladenen Designs“), das Werkzeug ebenso. Alternativ je Design neu verbinden (wie das
  Original: ein Design je „Card/Machine Read“).
- Für das eigene Programm (Phase 5): Beim Senden immer einen vollständigen Kopf mit beiden Längen schreiben; beim Lesen Länge 2
  nur nach Prüfung des Containers übernehmen.
- Die 51 Designs sind eine Sammlung echter Maschinendaten (EXP-Kodierung) zum Vergleich mit der eigenen Stichberechnung
  (Satin, Tatami, Lauf, Schrift).
