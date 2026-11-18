# 11 – Werkzeuge: was jedes Skript macht und wie man es benutzt

Alle Skripte sind Python 3 (für `rdis.py`, `art_decrypt`-Tests und OLE: `pip install pefile capstone olefile`) bzw. C (`cc -O2`).

| Datei | Zweck | Aufruf |
|---|---|---|
| `blast.c` | PKWARE-DCL-Dekompressor (stdin → stdout) | `cc -O2 -o blast blast.c` |
| `unz.py` | InstallShield-3-Archiv `setup.z` entpacken | `python3 unz.py setup.z ZIELORDNER ./blast` |
| `dfl_decrypt.py` | `.DFL` entschlüsseln und entpacken | `python3 dfl_decrypt.py DEFAULTS.DFL out.bin` |
| `dataitem.py` | DataItem-Baum lesen und ausgeben | `python3 dataitem.py out.bin 4` (Startoffset 4 bei DFL) |
| `art_decrypt.py` | Format-Schlüssel, Schlüsselableitung, Substitutionschiffre; `try_all()` probiert alle Schlüssel an einem Stream | `python3 art_decrypt.py Contents.raw Contents.dec` |
| `twofish_cfb1.c` | Twofish-128/CFB1 für `DesignDocument` | siehe [05](05_art_amt_verschluesselung.md) |
| `scan_dataitems.py` | alle DataItem-Bäume in einem Blob finden | `python3 scan_dataitems.py blob 100 out.txt` |
| `style_keyvalues.py` | Kernwerte (mm) aller Stile einer AMT | `python3 style_keyvalues.py DesignDocument.dec [Filter]` |
| `fabrics.py` | die 8 Stoffe vergleichen | `python3 fabrics.py DesignDocument.dec` |
| `styles_compare.py` | Stile gegen „Software Defaults“ (`base` = Basis flach ausgeben) | `python3 styles_compare.py DesignDocument.dec base` |
| `hoops.py` | Rahmenbibliothek als Tabelle | `python3 hoops.py hooplibr.dsc` |
| `resdump.py` | String-Tabellen, Dialoge und Menüs aus Windows-DLLs | `python3 resdump.py RESENU.DLL` |
| `rdis.py` | kleiner Disassembler mit Import-, Export- und String-Auflösung | `python3 rdis.py X.dll exports` · `dis ADRESSE [n]` · `xstr TEXT [w]` · `xref ADRESSE` |
| `callers.py` | alle `call`/`jmp` auf eine Adresse finden | `python3 callers.py X.dll ADRESSE` |
| `find_tagtables.py` | Namenstabellen (Tag, Name, Typ) in DLLs finden | `python3 find_tagtables.py *.DLL` |
| `tag_labels.py` | Tag → Dialogfeld-Beschriftung (DIALOG.DLL + RESENU) | erwartet `text/dialog_dll.asm` und `res/RESENU.DLL.txt` |

## Vorgehen beim Reverse Engineering (Beispiel DFL)
1. Strings aller DLLs extrahieren (ASCII + UTF-16) und nach Stichworten durchsuchen → `DEFAULTS.DFL` in DEFMGR.DLL, dazu der zlib-Copyright-String.
2. Mit `xstr` die Code-Stelle finden, die einen String benutzt, mit `callers.py` die Aufrufkette nach oben verfolgen.
3. Funktionen mit `dis` lesen und in Python nachbauen.
4. Am Ergebnis prüfen: stimmt die Längenangabe, ist die zlib-Prüfsumme gültig, ist der Inhalt lesbar.
