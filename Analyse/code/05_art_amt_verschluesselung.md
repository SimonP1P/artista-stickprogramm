# 05 – ART-, AMT- und ESB-Dateien entschlüsseln (Twofish + Substitution)

**Dateien:** [`art_decrypt.py`](art_decrypt.py), [`twofish_cfb1.c`](twofish_cfb1.c) (+ Tabellen `tfq.h`, die beim Bau aus IMPORT.DLL erzeugt werden, siehe unten)
**Ergebnis:** NORMAL.AMT vollständig lesbar ([`../daten/entschluesselt/`](../daten/entschluesselt/))
**Sicherheit:** hoch (beide Streams liefern exakt die im Kopf angegebene Größe und eine gültige zlib-Prüfsumme)

## Dateiaufbau
ART (Designs), AMT (Vorlagen) und ESB (Alphabete) sind **OLE-Compound-Dateien** (wie alte Word-Dokumente) mit den Streams:
| Stream | Inhalt |
|---|---|
| `\x05SummaryInformation` | Windows-Standard (Titel, Autor …), unverschlüsselt |
| `WilcomDesignInformationDDD` | Eigenschaften wie „number of stitches“, „design width“, „fabricthickness“, „embroidery machine name“, unverschlüsselt |
| `AUX_INFO` | 5 Byte: `u8 Flags` (Bit 0 = komprimiert, Bit 1 = verschlüsselt) + `u32 Modus` |
| `Contents` | Design (ältere, kompatible Fassung), **Substitutionschiffre** |
| `DesignDocument` | Design (vollständige Fassung), **Twofish** |
| `DESIGN_ICON` | Vorschaubild (wenn vorhanden) |

## 1) Format-Schlüssel (PRODUCT.DLL, statische Initialisierer 0x10001040–0x10001230)
Fest einkompiliert, je Paar `(Dateityp-ID, 32-Bit-Schlüssel)`:
| ID | Schlüssel | Verwendung |
|---|---|---|
| 0xCA | 0x058FAB49 | |
| 0xCD | 0x02431A01 | |
| **0xD0** | **0x01C0D932** | **ART-Designs** (an allen 287 Beispiel-ARTs geprüft) |
| **0xD1** | **0x0005B805** | **AMT-Vorlagen** (NORMAL.AMT) |
| **0xD2** | **0x002470E8** | **ESB-Alphabete** (jeder Buchstabe ein Stream, geprüft an Alice.ESB) |
| 0x12C | 0x05873C7A | |
IMPORT.DLL legt sie in einer Tabelle ab (Tags 0x1453/0x1454, Funktion 0x100ae2d0) und fragt sie über 0x100ae440 ab. Ist ein Typ nicht registriert, entsteht ein Zufallsschlüssel.

## 2) `Contents`: Byte-Substitution (IMPORT.DLL 0x10114c00, 0x10114a90, 0x10114b30)
**Schlüsselableitung** (0x10114c00) aus Format-Schlüssel `fk`, Zusatzwert `x` (Modus 0 → 0) und Modus `m`:
```python
s  = (fk + x) & 0xffffffff;  sb = s.to_bytes(4,'little')
h  = (sum(sb) * 13) & 0xff
for k in range(4):           # Ergebnisbytes rückwärts
    out[3-k] = rotl1(m_byte[k]) ^ ((sb[3-k] + 17*k + h) & 0xff)
key = int.from_bytes(out,'little')
```
**Permutationstabelle** (0x10114a90 / 0x10114b30): Für i = 0…255 wird `table[f(i)] = i` gesetzt, mit
```python
f(i) = rotl8( ((key>>10)&0xff) ^ i ^ ((key>>2)&0xff) ,  (8-(key&3)) mod 8 )
```
**Entschlüsseln** = jedes Byte durch `table[b]` ersetzen. Danach folgt `u32 Größe` + zlib. NORMAL.AMT/Contents: 12 452 Byte.

## 3) `DesignDocument`: Twofish-128 im CFB-1-Modus (IMPORT.DLL 0x10114dc0 / 0x10114e80)
- Eingebettet ist die **Twofish-Referenzimplementierung** (AES-Kandidat 1998): `makeKey` 0x101164e0, `cipherInit` 0x10116560, `blockEncrypt` 0x101165c0, `blockDecrypt` 0x101173c0, `ParseHexDword` 0x101152a0. Die q0/q1-Tabellen liegen ab Datei-Offset 0x18A970/0x18AA70.
- **Schlüssel** = Text `sprintf("1d43f2ae%08x%08x%08x", formatKey, modus, x)`, als 32 Hex-Ziffern = 128 Bit. Konstruktor 0x10114d30, Formatstring bei 0x1018cf4c.
- **IV** = `a885f60d87e90b71c20a81bcc3498af1` (String bei 0x1018cf28).
- **Modus 3 = CFB1** (Referenzimplementierung): pro **Bit** wird der IV-Block verschlüsselt, das oberste Bit des Ergebnisses mit dem Datenbit verknüpft (XOR), und das Chiffretext-Bit von rechts in den IV geschoben.
- Für NORMAL.AMT passt der Schlüssel **`1d43f2ae0005b8050000000000000000`** (formatKey 0xD1, Modus 0, x = 0). Danach `u32 Größe` + zlib → **658 986 Byte** Klartext.
- Für ART-Designs: **`1d43f2ae01c0d9320000000000000000`** (formatKey 0xD0).

## Test an den mitgelieferten Beispielen
| Datei-Art | Ergebnis |
|---|---|
| **alle 287** ART aus `Samples/` (Verifikation 18.11.2026) | **alle 287** `Contents` per Substitution (0xD0) lesbar. **94** neuere ARTs (AUX_INFO = `03…`) haben zusätzlich `DesignDocument` → **alle 94** per Twofish lesbar. Ältere ARTs (AUX_INFO = `01…`) haben nur `Contents`. |
| ESB-Alphabete (`wletter/`, `userletw/`) | Stream `\x06HEADER` (unverschlüsselt: Alphabetname UTF-16, Skalierungswerte) + **ein Stream je Zeichen**, benannt nach dem Zeichencode in Hex (`0041` = „A“, `00E9` = „é“), Substitution mit 0xD2 |

```c
// Kern der Entschlüsselung (twofish_cfb1.c)
for (bit = 0; bit < n*8; bit++) {
    enc(iv, x);                                  // Twofish-Blockverschlüsselung des IV
    ct = in[bit/8] & (0x80 >> (bit&7));          // Chiffretext-Bit
    pt = ct ^ ((x[0] & 0x80) >> (bit&7));        // Klartext-Bit
    out[bit/8] = (out[bit/8] & ~(0x80>>(bit&7))) | pt;
    shift_left_128(iv); iv[15] |= ct ? 1 : 0;    // IV um 1 Bit schieben, Chiffretext-Bit rechts einfügen
}
```
Die eigene Twofish-Implementierung liefert den offiziellen Testvektor (Schlüssel 0, Klartext 0 → `9F589F5CF6122C32B6BFEC2F2AE8C35A`).

**Bauen:** q-Tabellen aus IMPORT.DLL extrahieren (`d[0x18a970:0x18a970+256]`, `d[0x18aa70:...]`) → `tfq.h`, dann `cc -O2 -o twofish_cfb1 twofish_cfb1.c`.
**Aufruf:** `./twofish_cfb1 1d43f2ae0005b8050000000000000000 a885f60d87e90b71c20a81bcc3498af1 DesignDocument out.raw` → `out.raw[4:]` mit zlib entpacken.

## Bedeutung
Damit lassen sich ART-Designs, AMT-Vorlagen und ESB-Alphabete dieser Software lesen (geprüft wie oben). Es gibt keine dongle-abhängigen Schlüssel für den Standardfall. Der Klartext ist ein DataItem-Baum, beschrieben in [06](06_dataitem_und_tags.md).
