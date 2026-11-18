# 04 – `.DFL`-Dateien (z. B. `RES/DEFAULTS.DFL`): Verschlüsselung und Inhalt

**Dateien:** [`dfl_decrypt.py`](dfl_decrypt.py), [`dataitem.py`](dataitem.py); Ergebnis: [`../daten/entschluesselt/defaults_tree_named.txt`](../daten/entschluesselt/defaults_tree_named.txt)
**Sicherheit:** hoch (Ergebnis: korrekte zlib-Prüfsumme und genau die angegebene Länge)

## Wie ich den Code gefunden habe
1. Der String `DEFAULTS.DFL` steht in `DEFMGR.DLL`, dazu der zlib-Copyright-String „inflate 1.1.3“. Also wird zlib verwendet.
2. Der Aufrufer von `inflateInit_` (0x1000c550) ist 0x1000af40. Er erwartet `[u32 entpackte Größe][zlib-Strom]` und bricht bei mehr als 100 000 000 Byte ab.
3. Dessen Aufrufer (0x100011d9) liest die ganze Datei mit `CFile::Read` und ruft vorher **0x100017a0 → 0x10001760** auf. Das ist die Entschlüsselung.

## Die Entschlüsselung (DEFMGR.DLL 0x10001760), Maschinencode → Python
```asm
mov  ebp, [len]          ; n = Dateilänge
mov  esi, ebp
shl  esi, 12 ; add esi, ebp
shl  esi, 12 ; add esi, ebp   ; key = ((n<<12)+n)<<12)+n  = n·(2^24+2^12+1) (32 Bit)
loop: ecx = (i & 3) * 4
      edx = key  sar  ecx      ; arithmetischer Shift!
      buf[i] ^= dl
```
```python
def decrypt(d):
    n = len(d); key = ((((n << 12) + n) << 12) + n) & 0xffffffff
    if key & 0x80000000: key -= 1 << 32          # 'sar' = vorzeichenbehaftet
    return bytes(b ^ ((key >> (4*(i & 3))) & 0xff) for i, b in enumerate(d))
```
Der Schlüssel hängt nur von der **Dateilänge** ab. Es ist also eine Verschleierung, keine echte Verschlüsselung. Danach gilt: `u32 Größe` + zlib.

## Inhalt von DEFAULTS.DFL (24 471 Byte entpackt, DataItem-Baum, siehe [06](06_dataitem_und_tags.md))
- **Maschinenformate (20 Stück):** Barudan, Tajima, SWF, Tajima TMCE-600, Barudan Chenille, **Melco**, Toyota, Zangs, ZSK 1, ZSK 2, Pfaff, Barudan Z Series, Tajima TMCE-100, Melco Chenille, Melco EP1, **Bernina**, Tufting, Schiffli, Janome, ZSK TC.
- **Eintrag „Bernina“:**

| Tag | Name (aus Namenstabelle) | Wert | Bedeutung |
|---|---|---|---|
| 0x07d1 | emch_machine_type | 17 | interne Maschinennummer (auch `MACHINE 17` in hooplibr.dsc) |
| 0x07d2 | machine_name | „Bernina“ | |
| 0x07d6/0x07d7 | Kodierungen | 16, 7 | zugelassene Ausgabeformate (7 = EXP, wie bei „Melco“ 7 und 8) |
| 0x07d4 | (max. Sprung) | **12,7 mm** | |
| 0x07dd | trims_for_jump | 0 | kein Fadenschnitt per Sprungfolge (Melco: 2) |
| 0x07da / 0x07dc | | 0 / 0 | Melco: 1 / 1 (Trim-/Stopp-Kodierung) |
| 0x0827 | (min. Stich) | **0,3 mm** | kürzere Stiche werden entfernt |
| 0x0828 | (max. Stich) | **12,7 mm** | längere Stiche werden geteilt |
| 0x07ea / 0x07eb | | 0 / 0 | |

- **Serielle Standardwerte** (Digitizer- und Maschinen-Ports): Baudraten-Auswahl 110…56 000 (Standard 9600), Datenbits 6/7/8 (Standard 7), Stoppbits, Parität (Standard „2“), Handshake. Diese gelten für Industrie-Maschinen und Digitizer-Tabletts, **nicht** für die artista (die läuft über ADX/BEAGLink mit 19200 8N1).
- **Digitizer-Tablett-Formate:** Summa, Calcomp, GTCO, Numonics, Seiko … mit Formatstrings wie `@XXXX,YYYY,F`.
- **Dateityp-Registry** (Tags 0x1325/0x1326/0x1328): ordnet Dateitypen Maschinentypen zu, u. a. Typ 11 und 12 → Maschine 17 (Bernina).
- **Bildschirmmaße** 262 × 197 mm (Tag 0x12c0/0x12c1, Kalibrierung).
