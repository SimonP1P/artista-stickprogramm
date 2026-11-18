# 03 – artista 200: USB/ActiveSync, Ordner, Dateinamen (ADX)

**Quellen:** `adx.dll`, `AdxCe.dll` (Strings, Ressourcen, Dialoge), `ADX.pdf` (ADX-Handbuch, Text in [`../daten/handbuch/ADX_Handbuch.txt`](../daten/handbuch/ADX_Handbuch.txt))
**Sicherheit:** hoch für Pfade, Abläufe und Meldungen (Klartext im Programm); mittel für die Rolle der einzelnen Pfade

## Verbindung
- Physisch: **Standard-USB-Kabel**. Logisch: **Microsoft ActiveSync** (ab Version 3.1/3.5) mit RAPI (Remote API von Windows CE): `CeRapiInitEx`, `CeCreateFile`, `CeWriteFile`, `CeFindAllFiles`, `CeRapiInvoke` usw. aus `rapi.dll`.
- Die artista 200 ist also ein **Windows-CE-Gerät (StrongARM)**. ADX kopiert beim ersten Verbinden einen eigenen Server auf die Maschine: `AdxCe.dll` („AdxCe for WinCE (StrongArm)“) nach `\Windows\AdxCe.dll`.
- Befehle gehen über `CeRapiInvoke("\\Windows\\AdxCe.dll", "A200Command", …)`. Zusätzlich gibt es einen TCP-„Messenger“ an 127.0.0.1 mit Kopf (Magic Number und Version; Meldungen „Wrong Magic Number“, „Wrong Message Version“).
- Die Antworten sind **XML** mit den Feldern `design_list`, `design_item`, `number_of_designs`, `adx_art_name`, `adx_exp_name`, `title`, `width`, `height`, `no_stitches`, `no_colours`, `thread_list`, `thrd_brand`, `thrd_code`, `thrd_color`, `thrd_description`, `adx_error`.
- Befehle an die Maschine (Strings in adx.dll): `StitchFile`, `GetARTFileInformation`, `GetARTFileThumbNail` (→ `\temp\icon.bmp`), `UpdateFileSystem`, `QueryMachineState` (Antwort `Running`), `AnsStop`, `UsbHostEnable` (`disable`), `GetDiskFreeSpace`, `QueryCommands`.

## Speicherorte auf der artista 200
| Pfad auf der Maschine | Bedeutung (Reiter im ADX-Dialog) |
|---|---|
| `\IPSM\ANS\Data\Embroidery\` | interner Designspeicher („Intern“) |
| `\IPSM\ANS\Data\Embroidery\Motifs\` | Motive (Beispiel im Code: `…\Motifs\Daffodils.ART`) |
| `\IPSM\ANS\Data\Embroidery\Alphabets\` | Alphabete (geschützt) |
| `…\My Designs` | eigene Designs |
| `\EMBFiles\` | **EXP-Dateien** (Beispiel im Code: `\EmbFiles\10.exp`) |
| `\Storage Card\` | Personal Design Card („PcCard“) |
| `\USBCdrom\`, `\USBCdromEncrypted\` | CD-ROM am Modul, auch verschlüsselte Design-CDs |
| `\PC Card\`, `\Embroidery Module\` | weitere Ziele |

Registry auf dem PC: `HKCU\Software\Bernina\Adx\Settings` mit `TargetDevice`, `SourcePathDesigns` (Standard `c:\My Designs\`), `TempPathDesign`, `MotifPath`, `AlphabetPath`, `ActiveTargetPath`, `TargetPathAux`, `TargetPathStitchout`, `TargetPathATA`, `TargetPathBlank`, `TargetPathInternal`.

## Dateinamen und -formate
- Formatstrings im Code: **`%s.exp`** (Designname + .exp) und **`M%d.exp`** (nummeriert, z. B. M1.exp). Dazu `*.exp` als Filter.
- Die artista 200 nimmt **ART und EXP** an („Verify that your ART and EXP designs are saved there“). Die Thumbnails zeigen „Exp“ oder „Art“, Alphabete ein „α“.
- **Einschränkung:** „Designs in EXP format are limited to single operation only.“ EXP lässt sich also nur einzeln übertragen, nicht in Mehrfachauswahl.
- **Lizenzstufen** (ADX-Handbuch 5.2.1): Stufe 1 kopiergeschützt (bearbeitbar, nur intern speicherbar), Stufe 2 kopier- und änderungsgeschützt.
- **Grenzen:** maximal **6 eigene Ordner**; „Alphabets, Motifs, My Designs“ lassen sich weder löschen noch umbenennen; Meldung „The number of designs on this storage device is limited to %d“.
- **INF-Dateien oder ein Ordner `EmbF5` kommen in keiner Datei dieser Software vor.** Diese Struktur stammt aus späteren Bernina-Generationen mit USB-Stick.

## Ablauf „Write to Machine“ (Handbuch S. 264–268, ADX-Handbuch Kap. 3)
1. Design in der Software öffnen, „Write to Machine“ wählen.
2. Beim ersten Mal den Dialog „Device Selection“ bestätigen (artista 200 oder 165–185/R/W-Box mit COM-Port).
3. Dialog „Write to Embroidery System“: Ziel (Maschine, Karte, CD) und Ordner wählen.
4. **„Embroider“ / „Stitch out Selected Design“** überträgt das Design und öffnet es direkt zum Sticken. **„Download“** speichert es nur.
5. Die Nadel muss oben sein; der Touchscreen ist während der Übertragung schwarz.
