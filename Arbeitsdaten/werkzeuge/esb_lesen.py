"""Original-Alphabete der BERNINA artista V4 (ESB, Ordner wletter/) lesen.

Aufbau (Analyse_Teil3/09_esb_alphabete.md):
  OLE-Datei, Stream "\\x06HEADER" (unverschlüsselt, IMPORT.DLL 0x100040f0): u32 0x0d0a1aed, u32 Länge, Byte Version,
      Byte Unterversion, [u32 wenn Unterversion >= 2], Name UTF-16 (0xa2 Byte), float, float, 11 Byte, [Version >= 4:
      32 Byte], u16, [Version >= 4: u32 + 8 Byte]
  je Zeichen ein Stream, Name = Zeichencode hex ("0041" = A), Substitution 0xD2 + zlib (art_decrypt.try_all).
  Zeichen-Stream (IMPORT.DLL 0x100c6230): 0x24 Byte Kopf (u32 0x0d0a1a0d, u32 0xeaae8356, …, u32 Prüfsumme = Summe der
      ersten 8), dann Datensätze im Rahmen der ART-Objekte (Analyse_Teil3/03):
        0x2e  Buchstabenkopf (IMPORT.DLL 0x1000b330): 4 vorzeichenbehaftete gepackte Wörter = Grundlinie x1, y1, x2, y2
        0x3a  Zeichen (u32 Anzahl + UTF-16)
        danach die Objekte des Buchstabens (meist 0x08 Gruppe mit 0x06 Satinsäulen, Formen 0x01, Stichart, Unterlage)
  0x06 EmbroideryPatch (IMPORT.DLL 0x10018970): u16 n, n × (gepackt a, float x, gepackt c, float y) = Winkellinien
      von Segment a bei Anteil x zu Segment c bei Anteil y des Umrisses; dann der Umriss (0x01 GeoCurve).

Aufruf:  python esb_lesen.py ALPHABET.ESB [--json AUS.json] [--png AUS.png] [--zeichen 0041,0042]
"""
import argparse
import json
import os
import struct
import sys
import zlib

import olefile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import art_decrypt  # noqa: E402
from art_design import KLASSEN, OBJEKT_KLASSEN, PARAM_TAG, felder  # noqa: E402
from art_objekte import datensatz, knoten_lesen, packed_wort_s  # noqa: E402
from art_stiche import packed  # noqa: E402


def kopf_lesen(h: bytes):
    """Stream \\x06HEADER (IMPORT.DLL 0x100040f0)."""
    magic, laenge = struct.unpack_from('<II', h, 0)
    ver, unter = h[8], h[9]
    i = 10
    extra = None
    if unter >= 2:
        extra = struct.unpack_from('<I', h, i)[0]; i += 4
    name = h[i:i + 0xa2].decode('utf-16-le', 'replace').split('\0')[0]; i += 0xa2
    f1, f2 = struct.unpack_from('<ff', h, i); i += 8
    elf = h[i:i + 11]; i += 11
    if ver >= 4:
        i += 32
    w = struct.unpack_from('<H', h, i)[0] if i + 2 <= len(h) else None
    return {'magic_ok': magic == 0x0d0a1aed, 'version': ver, 'unterversion': unter, 'extra': extra, 'name': name,
            'float1': f1, 'float2': f2, 'bytes11': elf.hex(), 'u16': w}


def winkellinien(roh: bytes):
    """0x06 EmbroideryPatch: u16 n, n × (gepackt a, float x, gepackt c, float y)."""
    n = struct.unpack_from('<H', roh, 0)[0]
    i = 2
    out = []
    for _ in range(n):
        a, i = packed(roh, i)
        x = struct.unpack_from('<f', roh, i)[0]; i += 4
        c, i = packed(roh, i)
        y = struct.unpack_from('<f', roh, i)[0]; i += 4
        out.append((a, round(x, 6), c, round(y, 6)))
    return out, i


def zeichen_lesen(d: bytes):
    """Einen entschlüsselten Zeichen-Stream lesen: Grundlinie, Zeichen, Objekte (flach, mit aufgelösten Verweisen)."""
    kopf = struct.unpack_from('<9I', d, 0)
    if kopf[0] != 0x0d0a1a0d or kopf[1] != 0xeaae8356 or (sum(kopf[:8]) & 0xffffffff) != kopf[8]:
        raise ValueError('Zeichen-Kopf ungültig')
    i = 0x24
    folge, geteilt = [], []
    while i < len(d):
        k, verw, ln, a, b, j = datensatz(d, i)
        if k == 0:
            folge.append((i, 0, False, b'')); i = j; continue
        if verw:
            ziel = None
            if 0 < ln <= len(geteilt):
                n = geteilt[ln - 1]; ziel = [folge[n]]
                while folge[n][1] not in (0, 0x0d) and n + 1 < len(folge):
                    n += 1; ziel.append(folge[n])
            folge.append((i, k, True, ziel)); i = j; continue
        if ln > len(d) - j:
            break
        if a:
            geteilt.append(len(folge))
        folge.append((i, k, False, d[j:j + ln])); i = j + ln
    erg = {'grundlinie': None, 'zeichen': None, 'objekte': []}
    akt = None
    for off, k, verw, inhalt in folge:
        if k == 0x2e and not verw:
            w = []; p = 0
            for _ in range(4):
                v, p = packed_wort_s(inhalt, p); w.append(v)
            erg['grundlinie'] = w
            continue
        if k == 0x3a and not verw and erg['zeichen'] is None:
            n = struct.unpack_from('<I', inhalt, 0)[0]
            erg['zeichen'] = inhalt[4:4 + 2 * n].decode('utf-16-le', 'replace')
            continue
        if k in OBJEKT_KLASSEN and not verw:
            akt = {'klasse': KLASSEN.get(k, hex(k)), 'klasse_nr': k, 'formen': [], 'winkel': [], 'params': []}
            erg['objekte'].append(akt)
            if k == 0x06:
                akt['winkel'] = winkellinien(inhalt)[0]
            continue
        if akt is None:
            continue
        eintraege = []

        def aufloesen(e, tiefe=0):
            if e[2]:
                if tiefe < 8:
                    for x in e[3] or []:
                        aufloesen(x, tiefe + 1)
            else:
                eintraege.append(e)
        aufloesen((off, k, verw, inhalt))
        for _, kk, _, roh in eintraege:
            if kk == 0x01 and roh:
                f, p = packed(roh, 0)
                kn, _ = knoten_lesen(roh, p)
                akt['formen'].append({'geschlossen': bool(f & 1), 'knoten': kn})
            elif kk in PARAM_TAG or kk in (0x10, 0x12, 0x15, 0x0d):
                akt['params'].append([kk, PARAM_TAG.get(kk), felder(kk, roh)])
    return erg


def alphabet_lesen(pfad: str):
    o = olefile.OleFileIO(pfad)
    kopf = None
    zeichen = {}
    for s in o.listdir():
        name = s[-1]
        roh = o.openstream(s).read()
        if 'HEADER' in name:
            kopf = kopf_lesen(roh)
            continue
        r = art_decrypt.try_all(roh)
        if not r:
            zeichen[name] = {'fehler': 'nicht entschlüsselbar'}
            continue
        try:
            zeichen[name] = zeichen_lesen(r[2])
        except Exception as e:  # einzelnes Zeichen defekt: weiter
            zeichen[name] = {'fehler': str(e)}
    return {'datei': os.path.basename(pfad), 'kopf': kopf, 'zeichen': zeichen}


# ---------------------------------------------------------------- Geometrie / Bild
def bezier_punkte(form, n=10):
    """Bézier-Knoten als Punktliste (Einheit 1/180 mm); je Segment n Teilstücke. Segment s beginnt bei Index s*n."""
    kn = form['knoten']
    seq = kn + ([kn[0]] if form['geschlossen'] else [])
    pts = []
    for a, b in zip(seq, seq[1:]):
        p0, p3 = a['punkt'], b['punkt']
        p1 = a['aus'] or p0
        p2 = b['ein'] or p3
        for i in range(n):
            t = i / n
            k = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
            pts.append((sum(c * q[0] for c, q in zip(k, (p0, p1, p2, p3))), sum(c * q[1] for c, q in zip(k, (p0, p1, p2, p3)))))
    pts.append(seq[-1]['punkt'])
    return pts


def segment_punkt(form, s, t):
    """Punkt auf Segment s beim Anteil t (0…1) des Umrisses."""
    kn = form['knoten']
    seq = kn + ([kn[0]] if form['geschlossen'] else [])
    s = max(0, min(s, len(seq) - 2))
    a, b = seq[s], seq[s + 1]
    p0, p3 = a['punkt'], b['punkt']; p1 = a['aus'] or p0; p2 = b['ein'] or p3
    k = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
    return (sum(c * q[0] for c, q in zip(k, (p0, p1, p2, p3))), sum(c * q[1] for c, q in zip(k, (p0, p1, p2, p3))))


def png_bogen(alpha, pfad, spalten=16, zelle=110):
    """Alle Zeichen als Umriss (schwarz) mit Winkellinien (rot) und Grundlinie (blau) in einem Raster."""
    codes = sorted(c for c, z in alpha['zeichen'].items() if 'objekte' in z)
    # gemeinsamer Maßstab: größte Zeichenhöhe
    hoehen = []
    for c in codes:
        z = alpha['zeichen'][c]
        ys = [p[1] for o in z['objekte'] for f in o['formen'] for p in bezier_punkte(f, 4)]
        if ys and z['grundlinie']:
            hoehen.append(max(ys) - min(min(ys), z['grundlinie'][1]))
    href = max(hoehen) if hoehen else 1
    sk = (zelle - 20) / href
    zeilen = (len(codes) + spalten - 1) // spalten
    W, H = spalten * zelle, zeilen * zelle
    bild = bytearray([255]) * (W * H * 3)

    def linie(x0, y0, x1, y1, f):
        x0, y0, x1, y1 = int(x0), int(y0), int(x1), int(y1)
        dx = abs(x1 - x0); dy = -abs(y1 - y0); sx = 1 if x0 < x1 else -1; sy = 1 if y0 < y1 else -1; e = dx + dy
        for _ in range(4000):
            if 0 <= x0 < W and 0 <= y0 < H:
                o = (y0 * W + x0) * 3; bild[o:o + 3] = f
            if x0 == x1 and y0 == y1:
                break
            e2 = 2 * e
            if e2 >= dy: e += dy; x0 += sx
            if e2 <= dx: e += dx; y0 += sy

    for n, c in enumerate(codes):
        z = alpha['zeichen'][c]
        gl = z['grundlinie'] or [0, 0, 0, 0]
        ox, oy = (n % spalten) * zelle + 8, (n // spalten) * zelle + zelle - 12
        abb = lambda p: (ox + (p[0] - gl[0]) * sk, oy - (p[1] - gl[1]) * sk)  # noqa: E731
        linie(*abb((gl[0], gl[1])), *abb((gl[2], gl[3])), bytes((60, 110, 220)))
        for o in z['objekte']:
            for f in o['formen']:
                pts = bezier_punkte(f, 8)
                for p, q in zip(pts, pts[1:]):
                    linie(*abb(p), *abb(q), bytes((0, 0, 0)))
            if o['winkel'] and o['formen']:
                f = o['formen'][0]
                for a, x, cc, y in o['winkel']:
                    linie(*abb(segment_punkt(f, a, x)), *abb(segment_punkt(f, cc, y)), bytes((210, 40, 40)))
    roh = b''.join(b'\0' + bytes(bild[r * W * 3:(r + 1) * W * 3]) for r in range(H))

    def chunk(t, dd):
        return struct.pack('>I', len(dd)) + t + dd + struct.pack('>I', zlib.crc32(t + dd) & 0xffffffff)
    with open(pfad, 'wb') as fh:
        fh.write(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', W, H, 8, 2, 0, 0, 0))
                 + chunk(b'IDAT', zlib.compress(roh, 9)) + chunk(b'IEND', b''))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('esb')
    ap.add_argument('--json', default=None)
    ap.add_argument('--png', default=None)
    ap.add_argument('--zeichen', default=None, help='nur diese Zeichen ausführlich ausgeben, z. B. 0041,0042')
    a = ap.parse_args()
    alpha = alphabet_lesen(a.esb)
    k = alpha['kopf']
    print(f"{alpha['datei']}: Name {k['name']!r}, Version {k['version']}.{k['unterversion']}, float {k['float1']:g}/{k['float2']:g}, "
          f"u16 {k['u16']}, {len(alpha['zeichen'])} Zeichen")
    arten = {}
    for c, z in sorted(alpha['zeichen'].items()):
        if 'fehler' in z:
            print(f'  {c}: FEHLER {z["fehler"]}'); continue
        for o in z['objekte']:
            for kk, t, f in o['params']:
                if t in (0x2721, 0x2714, 0x271c, 0x271f, 0x2724, 0x2744, 0x2745, 0x2746, 0x2747):
                    arten[t] = arten.get(t, 0) + 1
    print('  Sticharten/Unterlagen (Anzahl Objekte):', {hex(t): n for t, n in sorted(arten.items())})
    if a.zeichen:
        for c in a.zeichen.split(','):
            z = alpha['zeichen'].get(c.upper()) or alpha['zeichen'].get(c)
            if not z:
                continue
            print(f'  {c} {z.get("zeichen")!r} Grundlinie {z.get("grundlinie")}')
            for o in z.get('objekte', []):
                print(f'    {o["klasse"]}: Formen {[len(f["knoten"]) for f in o["formen"]]}, Winkellinien {o["winkel"]}')
                for kk, t, f in o['params']:
                    print(f'      0x{kk:02x} {("0x%04x" % t) if t else "":7} {f}')
    if a.json:
        with open(a.json, 'w', encoding='utf-8') as fh:
            json.dump(alpha, fh, ensure_ascii=False)
    if a.png:
        png_bogen(alpha, a.png)
        print('  Bild:', a.png)


if __name__ == '__main__':
    main()
