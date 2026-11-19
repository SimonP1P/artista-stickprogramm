# Arbeitsdaten – dauerhaft gesicherte Rohdaten und Werkzeuge

Hier liegt alles, was während der Analyse (17.–18.11.2026) zunächst nur im temporären Arbeitsordner entstanden ist. Die aufbereiteten Ergebnisse stehen in `../Analyse/` und `../Analyse_Teil2/`; dieser Ordner enthält die **Rohdaten** dazu.

| Ordner | Inhalt | neu erzeugbar mit |
|---|---|---|
| `werkzeuge/` | **maßgebliche Fassung aller Skripte** (Python/C) + `requirements.txt` | – |
| `entpackt_ADX/` | Inhalt von `ADX/data1.cab` (adx.dll, BEAGLink.dll, AdxCe.dll, ADX.pdf …) | `unshield -d . x ADX/data1.cab` |
| `entschluesselt/` | DEFAULTS.DFL entschlüsselt, NORMAL.AMT-Streams roh und entschlüsselt (`*.dec`), ein ESB-Buchstabe, DataItem-Dumps, Stil-Vergleich | `dfl_decrypt.py`, `art_decrypt.py`, `twofish_cfb1.c`, `scan_dataitems.py` |
| `strings/` | Texte (ASCII + UTF-16) aller 92 Programmdateien, Format `A/U Offset Text` | `strs.py` |
| `ressourcen/` | String-Tabellen, Dialoge, Menüs aus RESENU/BERESENU/XSLang/QUILTERRES/adx | `resdump.py` |
| `texte/` | Handbuch man_enu.pdf als Text (Layout, Rohtext, je Seite markiert), Quilter-Handbuch, ADX-Handbuch | `pdftotext` |
| `disassembly/` | komplette Disassembly BEAGLink.dll und DIALOG.DLL, Schreibfunktion einzeln | `rdis.py DATEI full` |
| `messungen/` | Messergebnisse an den ART-Musterdesigns (`werkzeuge/art_messen.py`), z. B. Satinabstand gespeichert ↔ gemessen | `Analyse_Teil3/04` |
| `geraetetests/` | Ergebnis- und Mitschnittdateien der echten Tests an der artista 185 (201028 = v1, 205736 = v2, 215616 = v3, 233547 = v3.1; **205632, 215532, 233539 = nur Simulation/Selbsttest auf dem Heim-PC**) | – |
| `testprogramm_quellen/` | Quelltext der Testprogramme v1/v2/v3/v3.1/v3.2 (LF, ohne BOM; die ausgelieferten `.ps1` sind CRLF + UTF-8-BOM) | – |

**Nicht gesichert (bewusst):** Die vollständig entpackte Software aus `Setup/English/setup.z` (193 MB, 651 Dateien), weil der Mac fast voll ist. Sie ist **byte-identisch reproduzierbar** (verifiziert, siehe `../VERIFIKATION.md`):
```sh
cc -O2 -o blast werkzeuge/blast.c
python3 werkzeuge/unz.py ../Setup/English/setup.z ZIELORDNER ./blast
```
