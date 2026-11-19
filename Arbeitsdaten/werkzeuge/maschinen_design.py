"""Ein aus der artista 185 gelesenes Design auswerten (Testprogramm v3.4/v3.5: Design_NrXX_*.bin, Icon_NrXX_*.bin).

Aufbau des Puffers DATEN (Analyse_Teil2/03, Lesen 0x10004310):
  Offset 0      N × u16 BE  Tabelle A   (N = Verwaltungsblock Byte 0, artista 185: 28)
  Offset 2N     N × u16 BE  Tabelle B
  Offset 4N     N × u16 BE  Tabelle C
  Offset 6N     u32 BE      Länge 1 = Stichdaten inkl. Endkennung 80 81
  Offset 6N+4   u32 BE      Länge 2 = Anhang (BE-Container, verschleiert, Analyse_Teil2/05)
  Offset 6N+8   Stichdaten (EXP-artig), dann Anhang
Icon (Puffer ICON): Länge = Byte2 × Byte3 + 10 (Byte 2 = Bytes je Zeile, Byte 3 = Höhe).

Aufruf:  python maschinen_design.py Design_Nr01_....bin [Icon_Nr01_....bin] [--n 28] [--aus ORDNER]
Schreibt in ORDNER: stiche.exp, vorschau.svg, anhang_roh.bin, anhang_klar.bin, chunk_XX.bin, icon.png
"""
import argparse
import struct
import sys
import zlib
from pathlib import Path

HIER = Path(__file__).resolve().parent
sys.path.insert(0, str(HIER))
import be_scramble  # noqa: E402

CHUNK_NAMEN = {0x0A: 'Kopf', 0x14: 'Farbtabelle', 0x1E: 'Stichdaten', 0x28: 'Zusatzdaten', 0x32: 'Icon'}


def stiche_auswerten(d: bytes):
    """EXP-artig lesen wie BEAGLink (0x10007c05): zählt Stiche, Sprünge, Farbwechsel; Rahmen in 0,1 mm."""
    x = y = 0; i = 0; n = len(d)
    st = sp = fw = lang = unbekannt = 0; ende = None
    minx = maxx = miny = maxy = 0
    while i + 1 < n:
        b0, b1 = d[i], d[i + 1]
        if b0 == 0x80:
            if b1 == 0x81:
                ende = i; break
            if 1 <= b1 <= 15:
                if b1 & 1: fw += 1
                else: sp += 1
                if i + 3 >= n: break
                dx, dy = d[i + 2], d[i + 3]; i += 4
                x += dx - 256 if dx > 127 else dx; y += dy - 256 if dy > 127 else dy
            elif b1 == 0x82:
                vx, vy = struct.unpack_from('>hh', d, i + 2); i += 6
                x += vx; y += vy; lang += 1
            else:
                unbekannt += 1; i += 2; continue
        else:
            x += b0 - 256 if b0 > 127 else b0; y += b1 - 256 if b1 > 127 else b1; i += 2; st += 1
        minx, maxx, miny, maxy = min(minx, x), max(maxx, x), min(miny, y), max(maxy, y)
    return dict(stiche=st, spruenge=sp, farbwechsel=fw, lange_spruenge=lang, unbekannt=unbekannt, ende=ende,
                breite_mm=(maxx - minx) / 10, hoehe_mm=(maxy - miny) / 10, links=minx / 10, rechts=maxx / 10,
                unten=miny / 10, oben=maxy / 10, endpunkt=(x / 10, y / 10))


def png_schreiben(pfad: Path, breite: int, hoehe: int, bits: bytes, bpz: int, skala: int = 4):
    """1-Bit-Bitmap (Bit gesetzt = schwarz) als Graustufen-PNG, vergrößert."""
    zeilen = bytearray()
    for r in range(hoehe):
        zeile = bytearray([0])
        for c in range(breite):
            byte = bits[r * bpz + c // 8] if r * bpz + c // 8 < len(bits) else 0
            v = 0 if (byte >> (7 - c % 8)) & 1 else 255
            zeile += bytes([v]) * skala
        for _ in range(skala):
            zeilen += zeile
    def chunk(t, d):
        return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d) & 0xffffffff)
    ihdr = struct.pack('>IIBBBBB', breite * skala, hoehe * skala, 8, 0, 0, 0, 0)
    pfad.write_bytes(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'IDAT', zlib.compress(bytes(zeilen))) + chunk(b'IEND', b''))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('design'); ap.add_argument('icon', nargs='?')
    ap.add_argument('--n', type=int, default=28); ap.add_argument('--aus', default=None)
    a = ap.parse_args()
    roh = Path(a.design).read_bytes(); N = a.n
    aus = Path(a.aus) if a.aus else Path(a.design).with_suffix('')
    aus.mkdir(parents=True, exist_ok=True)
    k = 6 * N + 8
    tab = [struct.unpack_from('>%dH' % N, roh, 2 * N * t) for t in range(3)]
    len1, len2 = struct.unpack_from('>II', roh, 6 * N)
    print(f'Datei {len(roh)} Byte, Kopf {k} Byte, Länge 1 (Stiche) {len1}, Länge 2 (Anhang) {len2}, '
          f'Summe {k + len1 + len2} ({"passt" if k + len1 + len2 == len(roh) else "PASST NICHT"})')
    for t, name in enumerate('ABC'):
        nz = [(i, v) for i, v in enumerate(tab[t]) if v]
        print(f'  Tabelle {name}: {"alle 0" if not nz else nz}')
    stiche = roh[k:k + len1]; anhang = roh[k + len1:k + len1 + len2]
    print(f'Stichdaten: Ende {stiche[-2:].hex(" ")} ({"Endkennung 80 81" if stiche[-2:] == b"\x80\x81" else "keine Endkennung"})')
    s = stiche_auswerten(stiche)
    print('  ' + ', '.join(f'{x} {v}' for x, v in s.items()))
    (aus / 'stiche.exp').write_bytes(stiche[:-2] if stiche[-2:] == b'\x80\x81' else stiche)
    try:
        sys.path.insert(0, str(HIER.parent.parent / 'stickprogramm' / 'src'))
        from stickprogramm.exp import exp_lesen_bytes
        from stickprogramm.vorschau import svg_schreiben
        svg_schreiben(exp_lesen_bytes(stiche), aus / 'vorschau.svg', titel=Path(a.design).name)
        print(f'  Vorschau: {aus / "vorschau.svg"}')
    except Exception as e:  # Vorschau ist Beiwerk
        print(f'  (keine Vorschau: {e})')
    if anhang:
        (aus / 'anhang_roh.bin').write_bytes(anhang)
        kennung, L, w = struct.unpack_from('<HIH', anhang, 0)
        print(f'Anhang: {len(anhang)} Byte, Kopf u16 0x{kennung:04X}, Länge {L} ({"passt" if L == len(anhang) else "≠ Länge 2"}), u16 0x{w:04X}')
        if 8 < L <= len(anhang):
            klar = be_scramble.transform(anhang[:L], encrypt=False)
            (aus / 'anhang_klar.bin').write_bytes(klar)
            ch = be_scramble.chunks(klar)
            print(f'  entschleiert: {len(ch)} Chunks')
            for cid, d in ch:
                print(f'   Chunk 0x{cid:02X} {CHUNK_NAMEN.get(cid, "?"):12s} {len(d) + 6:6d} Byte  {d[:24].hex(" ")}')
                (aus / f'chunk_{cid:02X}.bin').write_bytes(d)
                if cid == 0x0A and len(d) >= 0x0E + 32:
                    print(f'      Name (+0x14 inkl. Chunkkopf): {d[0x0E:0x0E + 32].split(b"\0")[0]!r}')
                if cid == 0x14 and len(d) >= 6:
                    n = (len(d)) // 0x86
                    print(f'      Farbtabelle: Kopf {d[:6].hex(" ")}, {n} Einträge à 0x86')
                    for j in range(n):
                        e = d[6 + j * 0x86: 6 + (j + 1) * 0x86]
                        txt = [t.decode('latin-1') for t in e.split(b'\0') if len(t) >= 2][:3]
                        print(f'        {j + 1:2d}: {e[:8].hex(" ")}  {txt}')
    if a.icon:
        ic = Path(a.icon).read_bytes()
        bpz, h = ic[2], ic[3]
        print(f'Icon: {len(ic)} Byte, Kopf {ic[:10].hex(" ")}, {bpz} Byte je Zeile × {h} Zeilen = {bpz * 8} × {h} Pixel')
        png_schreiben(aus / 'icon.png', bpz * 8, h, ic[10:], bpz)
        print(f'  Bild: {aus / "icon.png"}')


if __name__ == '__main__':
    main()
