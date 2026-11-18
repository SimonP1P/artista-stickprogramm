# Verifikation des Projektstands – 18.11.2026

Alle Prüfungen liefen mit den **gesicherten** Werkzeugen aus `Arbeitsdaten/werkzeuge/`. Die Python-Umgebung: pefile, capstone, olefile; für die Testprogramme portable PowerShell 7.4.6 auf dem Mac.

## 1. Reproduzierbarkeit
| Prüfung | Ergebnis |
|---|---|
| `setup.z` neu entpacken (unz.py + blast.c) und mit bisherigem Stand vergleichen | ✅ alle **651** Dateien byte-identisch (inkl. der 7 bekannten Schadensfälle durch die defekte CD-Kopie) |
| DEFAULTS.DFL entschlüsseln | ✅ identisch mit gesicherter Fassung |
| NORMAL.AMT `Contents` (Substitution, Format-Key 0xD1) | ✅ identisch |
| Twofish-Implementierung, offizieller Testvektor | ✅ `9F589F5CF6122C32B6BFEC2F2AE8C35A` |
| NORMAL.AMT `DesignDocument` (Twofish-CFB1) | ✅ identisch |
| **alle 287 ART-Beispieldesigns**: `Contents` | ✅ 287/287 |
| davon 94 mit `DesignDocument` (Twofish, Key 0xD0) | ✅ 94/94 |
| BE-Verschleierung (Hin/Rück, 4 Längenbereiche) | ✅ |
| Rahmentabelle, Stoffvergleich, Stil-Kernwerte neu erzeugen | ✅ identisch mit den Dateien in `Analyse/daten/` |
| BEAGLink.dll komplett disassemblieren (`rdis.py full`) | ✅ identisch mit `Analyse_Teil2/daten/BEAGLink_komplett.asm` |
| Testprogramm v1 und v2 im Simulationsmodus | ✅ beide laufen vollständig durch |
| ausgeliefertes v2-Skript = gesicherte Quelle | ✅ |

## 2. Dokumente
| Prüfung | Ergebnis |
|---|---|
| Verweise in allen 27 Markdown-Dateien | ✅ 0 kaputte Links |
| 85 in Teil 2 / Protokoll / EXP zitierte BEAGLink-Adressen | ✅ alle existieren als Befehlsanfang |
| 5 zitierte DEFMGR-Adressen | ✅ vorhanden (0x1000c454 ist eine Operandenstelle, im Text präzisiert) |
| Kernzahlen im Bericht gegen Rohdaten: Satin 0,40 · Tatami 0,40/4,0 · Kontur-Lauf 2,5 · Edge-Walk 2,2 · Zugausgleich 0,2 · Format Bernina 0,3/12,7 mm, Typ 17 · Rahmen Oval 145×255, MEGA 150×400, Mittel 100×130 · 50 Alphabete · 32 Garnkarten · 8 Stoffe, Faktor Woven Heavy 1,13 | ✅ alle übereinstimmend |

## 3. Am Gerät (artista 185, 18.11.2026)
| Prüfung | Ergebnis |
|---|---|
| Verbindung, `T r M E`, Lesen mit `R`/Hex/`O` | ✅ fehlerfrei (Mitschnitte in `Arbeitsdaten/geraetetests/`) |
| Baudwechsel 57600, `Y Q`, Stickmodul `SRMV04.04` | ✅ |
| Verwaltungsblock besteht alle Plausibilitätsprüfungen des Originals | ✅ |

## 4. Bei der Verifikation korrigiert
- `rdis.py` in `Analyse/code/` war eine ältere Fassung ohne Befehl `full` → angeglichen.
- Testumfang ART-Entschlüsselung im Dokument 05 von „40 Beispiele“ auf „alle 287“ aktualisiert.
- Disassembler-Anleitung: Hinweis, dass `xref` Byte-Positionen meldet (0x1000c454).

## 5. Nicht verifizierbar (bewusst offen)
- **Schreiben** von Designs in die Maschine: nur aus dem Code bekannt, nie ausgeführt.
- Bedeutung einzelner Felder (Tabelle A/B/C, einige Tags in `0x27xx`) – als „vermutet“ markiert.
- BE-Verschleierung gegen echte Containerdaten (kein Beispiel vorhanden).
- Online-Hilfe ARTISTA.HLP (CD-Kopie beschädigt).
