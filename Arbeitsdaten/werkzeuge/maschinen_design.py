"""Aus der artista 185 gelesene Designs auswerten (Testprogramm v3.5/v3.6: Design_NrXX_*.bin, Icon_NrXX_*.bin).

Aufbau des Puffers DATEN (Analyse_Teil2/03, Lesen 0x10004310; am Gerät bestätigt, Analyse_Teil2/07 Test v3.5):
  Offset 0      N × u16 BE  Tabelle A   (N = Verwaltungsblock Byte 0, artista 185: 28)
  Offset 2N     N × u16 BE  Tabelle B
  Offset 4N     N × u16 BE  Tabelle C
  Offset 6N     u32 BE      Länge 1 = Stichdaten inkl. Endkennung 80 81
  Offset 6N+4   u32 BE      Länge 2 = Anhang (BE-Container, verschleiert, Analyse_Teil2/05)
  Offset 6N+8   Stichdaten (EXP-Kodierung), dann Anhang
Icon (Puffer ICON): Länge = Byte2 × Byte3 + 10 (Byte 2 = Bytes je Zeile, Byte 3 = Höhe), Bitmap ab Byte 10.

Aufruf:  python maschinen_design.py Design_Nr01_....bin [Icon_Nr01_....bin] [--n 28] [--aus ORDNER] [--svg]
         python maschinen_design.py --sammlung ORDNER [--liste Ergebnis_….txt]
                  (alle Design_NrXX_*.bin; schreibt ORDNER/UEBERSICHT.md und ORDNER/kontaktbogen.png)
Schreibt je Design: stiche.exp, vorschau.png, anhang_klar.bin, icon.png (mit --svg auch vorschau.svg)
"""
import argparse
import re
import struct
import sys
import zlib
from pathlib import Path

HIER = Path(__file__).resolve().parent
sys.path.insert(0, str(HIER))
import be_scramble  # noqa: E402

CHUNK_NAMEN = {0x0A: 'Kopf', 0x14: 'Farbtabelle', 0x1E: 'Stichdaten', 0x28: 'Zusatzdaten', 0x32: 'Icon'}
FARBEN = [(31, 58, 147), (192, 57, 43), (39, 174, 96), (243, 156, 18), (142, 68, 173), (22, 160, 133),
          (44, 62, 80), (211, 84, 0), (41, 128, 185), (231, 76, 60), (46, 204, 113), (155, 89, 182)]


def _s8(v):
    return v - 256 if v > 127 else v


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
                x += _s8(d[i + 2]); y += _s8(d[i + 3]); i += 4
            elif b1 == 0x82:
                vx, vy = struct.unpack_from('>hh', d, i + 2); i += 6
                x += vx; y += vy; lang += 1
            else:
                unbekannt += 1; i += 2; continue
        else:
            x += _s8(b0); y += _s8(b1); i += 2; st += 1
        minx, maxx, miny, maxy = min(minx, x), max(maxx, x), min(miny, y), max(maxy, y)
    return dict(stiche=st, spruenge=sp, farbwechsel=fw, lange_spruenge=lang, unbekannt=unbekannt, ende=ende,
                breite_mm=(maxx - minx) / 10, hoehe_mm=(maxy - miny) / 10, links=minx / 10, rechts=maxx / 10,
                unten=miny / 10, oben=maxy / 10, endpunkt=(x / 10, y / 10))


def stich_segmente(d: bytes):
    """Je Farbblock eine Liste von Linienzügen (Punkte in 0,1 mm); Sprünge trennen die Züge."""
    bloecke = [[[(0, 0)]]]; x = y = 0; i = 0; n = len(d)
    while i + 1 < n:
        b0, b1 = d[i], d[i + 1]
        if b0 == 0x80:
            if b1 == 0x81:
                break
            if 1 <= b1 <= 15 and i + 3 < n:
                x += _s8(d[i + 2]); y += _s8(d[i + 3]); i += 4
                if b1 & 1:
                    bloecke.append([])
                bloecke[-1].append([(x, y)]); continue
            if b1 == 0x82 and i + 5 < n:
                vx, vy = struct.unpack_from('>hh', d, i + 2); x += vx; y += vy; i += 6
                bloecke[-1].append([(x, y)]); continue
            i += 2; continue
        x += _s8(b0); y += _s8(b1); i += 2
        if not bloecke[-1]:
            bloecke[-1].append([])
        bloecke[-1][-1].append((x, y))
    return bloecke


def _png(pfad: Path, W: int, H: int, zeilen_roh: bytes, farbtyp: int):
    def chunk(t, dd):
        return struct.pack('>I', len(dd)) + t + dd + struct.pack('>I', zlib.crc32(t + dd) & 0xffffffff)
    pfad.write_bytes(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', W, H, 8, farbtyp, 0, 0, 0))
                     + chunk(b'IDAT', zlib.compress(zeilen_roh, 9)) + chunk(b'IEND', b''))


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
    _png(pfad, breite * skala, hoehe * skala, bytes(zeilen), 0)


def vorschau_png(d: bytes, pfad: Path, max_breite=1400, max_hoehe=700):
    """Stichbild als PNG (ohne Fremdbibliothek), Farbblöcke in Ersatzfarben, Sprünge nicht gezeichnet.
    Sehr breite Designs werden in Streifen untereinander gelegt. Liefert die Anzahl der Streifen."""
    bl = stich_segmente(d)
    pts = [p for b in bl for z in b for p in z]
    minx = min(p[0] for p in pts); maxx = max(p[0] for p in pts)
    miny = min(p[1] for p in pts); maxy = max(p[1] for p in pts)
    w = max(1, maxx - minx); h = max(1, maxy - miny)
    streifen = 1
    while w / streifen > 4 * h * streifen and streifen < 12:
        streifen += 1
    sl = w / streifen
    sk = min(max_breite / sl, max_hoehe / (h * streifen * 1.15), 0.8)      # Pixel je 0,1 mm
    rand = 8; hs = int(h * sk) + 1; abst = int(h * 0.15 * sk)
    W = int(sl * sk) + 2 * rand + 1; H = hs * streifen + abst * (streifen - 1) + 2 * rand
    bild = bytearray([250]) * (W * H * 3)

    def linie(x0, y0, x1, y1, f):
        dx = abs(x1 - x0); dy = -abs(y1 - y0); sx = 1 if x0 < x1 else -1; sy = 1 if y0 < y1 else -1; e = dx + dy
        while True:
            if 0 <= x0 < W and 0 <= y0 < H:
                o = (y0 * W + x0) * 3; bild[o:o + 3] = f
            if x0 == x1 and y0 == y1:
                break
            e2 = 2 * e
            if e2 >= dy: e += dy; x0 += sx
            if e2 <= dx: e += dx; y0 += sy

    def abb(x, y, k):
        return int((x - minx - k * sl) * sk) + rand, int((maxy - y) * sk) + rand + k * (hs + abst)

    for bi, b in enumerate(bl):
        f = bytes(FARBEN[bi % len(FARBEN)])
        for z in b:
            for (xa, ya), (xb, yb) in zip(z, z[1:]):
                k = min(streifen - 1, int((xa - minx) / sl))
                linie(*abb(xa, ya, k), *abb(xb, yb, k), f)
    zeilen = b''.join(b'\0' + bytes(bild[r * W * 3:(r + 1) * W * 3]) for r in range(H))
    _png(pfad, W, H, zeilen, 2)
    return streifen


def auswerten(design: Path, icon, aus: Path, N: int = 28, ausfuehrlich: bool = True, svg: bool = False):
    """Ein gelesenes Design auswerten; schreibt Dateien nach aus/ und liefert eine Zusammenfassung."""
    p = print if ausfuehrlich else (lambda *x, **y: None)
    roh = design.read_bytes()
    aus.mkdir(parents=True, exist_ok=True)
    k = 6 * N + 8
    tab = [struct.unpack_from('>%dH' % N, roh, 2 * N * t) for t in range(3)]
    len1, len2 = struct.unpack_from('>II', roh, 6 * N)
    r = dict(datei=design.name, byte=len(roh), len1=len1, len2=len2, passt=(k + len1 + len2 == len(roh)), name='', anhang_echt=False,
             chunks=[], farben_belegt=0, farben_tabelle=0,
             tabellen=[[(i, v) for i, v in enumerate(tab[t]) if v] for t in range(3)])
    p(f'Datei {len(roh)} Byte, Kopf {k} Byte, Länge 1 (Stiche) {len1}, Länge 2 (Anhang) {len2}, '
      f'Summe {k + len1 + len2} ({"passt" if r["passt"] else "PASST NICHT"})')
    for t, name in enumerate('ABC'):
        p(f'  Tabelle {name}: {"alle 0" if not r["tabellen"][t] else r["tabellen"][t]}')
    stiche = roh[k:k + len1]; anhang = roh[k + len1:k + len1 + len2]
    r['endkennung'] = stiche[-2:] == b'\x80\x81'
    p(f'Stichdaten: Ende {stiche[-2:].hex(" ")} ({"Endkennung 80 81" if r["endkennung"] else "keine Endkennung"})')
    s = stiche_auswerten(stiche); r.update(s)
    p('  ' + ', '.join(f'{x} {v}' for x, v in s.items()))
    (aus / 'stiche.exp').write_bytes(stiche[:-2] if r['endkennung'] else stiche)
    try:
        r['streifen'] = vorschau_png(stiche, aus / 'vorschau.png')
        p(f'  Vorschau: {aus / "vorschau.png"} ({r["streifen"]} Streifen)')
    except Exception as e:  # Vorschau ist Beiwerk
        p(f'  (keine Vorschau: {e})')
    if svg:
        try:
            sys.path.insert(0, str(HIER.parent.parent / 'stickprogramm' / 'src'))
            from stickprogramm.exp import exp_lesen_bytes
            from stickprogramm.vorschau import svg_schreiben
            svg_schreiben(exp_lesen_bytes(stiche), aus / 'vorschau.svg', titel=design.name)
        except Exception as e:
            p(f'  (keine SVG-Vorschau: {e})')
    if len(anhang) >= 8:
        kennung, L, w = struct.unpack_from('<HIH', anhang, 0)
        p(f'Anhang: {len(anhang)} Byte, Kopf u16 0x{kennung:04X}, Länge {L} '
          f'({"passt" if L == len(anhang) else "≠ Länge 2"}), u16 0x{w:04X}')
        r['anhang_kopf'] = (kennung, L, w)
        r['anhang_echt'] = (L == len(anhang))
        if not r['anhang_echt']:
            p('  KEIN echter Anhang: Containerlänge ≠ Länge 2 – Rest des vorher geladenen Designs im Puffer '
              '(die Maschine setzt Länge 2 bei 0x400 nicht zurück, Analyse_Teil2/07 Test v3.6)')
        if r['anhang_echt'] and L > 8:
            klar = be_scramble.transform(anhang[:L], encrypt=False)
            (aus / 'anhang_klar.bin').write_bytes(klar)
            ch = be_scramble.chunks(klar)
            p(f'  entschleiert: {len(ch)} Chunks')
            for cid, dd in ch:
                r['chunks'].append((cid, len(dd) + 6))
                p(f'   Chunk 0x{cid:02X} {CHUNK_NAMEN.get(cid, "?"):12s} {len(dd) + 6:6d} Byte  {dd[:24].hex(" ")}')
                if cid == 0x0A and len(dd) >= 0x0E + 32:
                    r['name'] = dd[0x0E:0x0E + 32].split(b'\0')[0].decode('latin-1').rstrip()
                    p(f'      Name (+0x14 inkl. Chunkkopf): {r["name"]!r}')
                if cid == 0x14:
                    n = len(dd) // 0x86                     # Chunk = 6 Byte Chunkkopf + n × 0x86 (n = Farbanzahl)
                    belegt = []
                    for j in range(n):
                        e = dd[j * 0x86:(j + 1) * 0x86]
                        if any(e):
                            belegt.append(j + 1)
                            txt = [t.decode('latin-1') for t in e.split(b'\0') if len(t) >= 2][:3]
                            p(f'        {j + 1:3d}: {e[:8].hex(" ")}  {txt}')
                    r['farben_tabelle'] = n; r['farben_belegt'] = len(belegt)
                    p(f'      Farbtabelle: {n} Einträge à 0x86 (Stiche: {r["farbwechsel"] + 1} Farben), davon {len(belegt)} belegt')
    if icon is not None and Path(icon).exists():
        ic = Path(icon).read_bytes()
        bpz, h = ic[2], ic[3]
        r['icon'] = (bpz * 8, h)
        p(f'Icon: {len(ic)} Byte, Kopf {ic[:10].hex(" ")}, {bpz} Byte je Zeile × {h} Zeilen = {bpz * 8} × {h} Pixel')
        png_schreiben(aus / 'icon.png', bpz * 8, h, ic[10:], bpz)
    return r


ZIFFERN = {'0': '111101101101111', '1': '010110010010111', '2': '111001111100111', '3': '111001111001111',
           '4': '101101111001001', '5': '111100111001111', '6': '111100111101111', '7': '111001010010010',
           '8': '111101111101111', '9': '111101111001111'}


def kontaktbogen(designs, pfad: Path, spalten=6, zb=260, zh=170):
    """Alle Designs als kleine Stichbilder in einem Raster (Nummer oben links), ohne Fremdbibliothek."""
    zeilen_n = (len(designs) + spalten - 1) // spalten
    W = spalten * zb; H = zeilen_n * zh
    bild = bytearray([255]) * (W * H * 3)

    def setz(x, y, f):
        if 0 <= x < W and 0 <= y < H:
            o = (y * W + x) * 3; bild[o:o + 3] = f

    def linie(x0, y0, x1, y1, f):
        dx = abs(x1 - x0); dy = -abs(y1 - y0); sx = 1 if x0 < x1 else -1; sy = 1 if y0 < y1 else -1; e = dx + dy
        while True:
            setz(x0, y0, f)
            if x0 == x1 and y0 == y1:
                break
            e2 = 2 * e
            if e2 >= dy: e += dy; x0 += sx
            if e2 <= dx: e += dx; y0 += sy

    for idx, (nr, stiche) in enumerate(designs):
        ox = (idx % spalten) * zb; oy = (idx // spalten) * zh
        for x in range(ox, ox + zb):
            setz(x, oy, bytes((200, 200, 200)))
        for y in range(oy, oy + zh):
            setz(ox, y, bytes((200, 200, 200)))
        for i, ch in enumerate(str(nr)):                       # Nummer, 3×5-Ziffern, Faktor 3
            for b, bit in enumerate(ZIFFERN[ch]):
                if bit == '1':
                    for u in range(3):
                        for v in range(3):
                            setz(ox + 5 + i * 12 + (b % 3) * 3 + u, oy + 5 + (b // 3) * 3 + v, bytes((0, 0, 0)))
        bl = stich_segmente(stiche)
        pts = [q for b in bl for z in b for q in z]
        minx = min(q[0] for q in pts); maxx = max(q[0] for q in pts)
        miny = min(q[1] for q in pts); maxy = max(q[1] for q in pts)
        sk = min((zb - 16) / max(1, maxx - minx), (zh - 30) / max(1, maxy - miny))
        mx = ox + 8 + ((zb - 16) - (maxx - minx) * sk) / 2; my = oy + 24 + ((zh - 30) - (maxy - miny) * sk) / 2
        for bi, b in enumerate(bl):
            f = bytes(FARBEN[bi % len(FARBEN)])
            for z in b:
                for (xa, ya), (xb, yb) in zip(z, z[1:]):
                    linie(int(mx + (xa - minx) * sk), int(my + (maxy - ya) * sk),
                          int(mx + (xb - minx) * sk), int(my + (maxy - yb) * sk), f)
    zeilen = b''.join(b'\0' + bytes(bild[r * W * 3:(r + 1) * W * 3]) for r in range(H))
    _png(pfad, W, H, zeilen, 2)


def liste_lesen(ergebnis: Path):
    """Designliste aus einer Ergebnis_*.txt des Testprogramms: {Nr: (Name roh, Flags)}."""
    out = {}
    for z in ergebnis.read_text(encoding='utf-8-sig', errors='replace').splitlines():
        m = re.search(r"Nr\s+(\d+)\s+Anzeige '.*?'\s+Name roh '(.*?)'\s+Flags 0x([0-9A-Fa-f]{2})", z)
        if m:
            out[int(m.group(1))] = (m.group(2), int(m.group(3), 16))
    return out


def sammlung(ordner: Path, N: int, liste=None):
    """Alle Design_NrXX_*.bin eines Ordners auswerten (je Nummer die neueste Datei), UEBERSICHT.md schreiben."""
    liste = liste or {}
    neueste = {}
    for f in sorted(ordner.glob('Design_Nr*_*.bin')):
        m = re.match(r'Design_Nr(\d+)_(\d{8}_\d{6})\.bin$', f.name)
        if m:
            neueste[int(m.group(1))] = (f, m.group(2))
    zeilen = ['# Designs aus dem Maschinenspeicher der artista 185', '',
              'Gelesen mit dem Testprogramm (Befehl 0x400 + Puffer DATEN), ausgewertet mit '
              '`Arbeitsdaten/werkzeuge/maschinen_design.py --sammlung`. Befund: `Analyse_Teil2/07_befund_am_geraet.md`.',
              'Maße aus den Stichdaten (EXP-Kodierung, 0,1 mm). Farben = Farbwechsel + 1. Vorschau in Ersatzfarben.', '',
              'Anhang „Rest“: Länge 2 stammt vom vorher geladenen Design (kein eigener Anhang). Übersichtsbild: `kontaktbogen.png`.', '',
              '| Nr | Name in der Liste | Flags | Name im Anhang (0x0A) | Byte | Stiche | Sprünge | Farben | Breite × Höhe mm | '
              'Ende 80 81 | Anhang | Farbtabelle belegt | Vorschau |',
              '|---|---|---|---|---|---|---|---|---|---|---|---|---|']
    bogen = []
    for nr in sorted(neueste):
        f, st = neueste[nr]
        ic = ordner / f'Icon_Nr{nr:02d}_{st}.bin'
        r = auswerten(f, ic, ordner / f'Nr{nr:02d}', N, ausfuehrlich=False)
        ch = ' '.join(f'0x{c:02X}' for c, _ in r['chunks'])
        anh = ('Chunks ' + ch) if r['anhang_echt'] else ('Rest' if r['len2'] else '–')
        ln, fl = liste.get(nr, ('', None))
        fls = '' if fl is None else f'0x{fl:02X}'
        zeilen.append(f'| {nr} | {ln} | {fls} | {r["name"]} | {r["byte"]} | {r["stiche"]} | {r["spruenge"]} | '
                      f'{r["farbwechsel"] + 1} | {r["breite_mm"]:.1f} × {r["hoehe_mm"]:.1f} | {"ja" if r["endkennung"] else "NEIN"} | '
                      f'{anh} | {r["farben_belegt"]}/{r["farben_tabelle"]} | [Bild](Nr{nr:02d}/vorschau.png) |')
        roh = f.read_bytes()
        bogen.append((nr, roh[6 * N + 8:6 * N + 8 + r['len1']]))
        print(f'Nr {nr:2d} {r["name"][:30]:30s} {r["stiche"]:6d} Stiche {r["farbwechsel"] + 1:4d} Farben '
              f'{r["breite_mm"]:7.1f} x {r["hoehe_mm"]:6.1f} mm  Chunks {ch}  '
              f'{"" if r["passt"] and r["endkennung"] else "PRÜFEN"}')
    kontaktbogen(bogen, ordner / 'kontaktbogen.png')
    (ordner / 'UEBERSICHT.md').write_text('\n'.join(zeilen) + '\n', encoding='utf-8')
    print(f'Übersicht: {ordner / "UEBERSICHT.md"}')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('design', help='Design_NrXX_*.bin oder (mit --sammlung) ein Ordner')
    ap.add_argument('icon', nargs='?')
    ap.add_argument('--n', type=int, default=28)
    ap.add_argument('--aus', default=None)
    ap.add_argument('--sammlung', action='store_true', help='alle Designs des Ordners auswerten, UEBERSICHT.md schreiben')
    ap.add_argument('--svg', action='store_true', help='zusätzlich SVG-Vorschau (groß)')
    ap.add_argument('--liste', default=None, help='Ergebnis_*.txt des Testprogramms mit der Designliste (Namen, Flags)')
    a = ap.parse_args()
    if a.sammlung:
        sammlung(Path(a.design), a.n, liste_lesen(Path(a.liste)) if a.liste else None)
        return
    aus = Path(a.aus) if a.aus else Path(a.design).with_suffix('')
    auswerten(Path(a.design), Path(a.icon) if a.icon else None, aus, a.n, True, a.svg)


if __name__ == '__main__':
    main()
