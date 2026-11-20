"""Löcher in Flächenobjekten der ART-Musterdesigns gegen Umrisse anderer Objekte (Aussparen/Knockout im Original,
Analyse_Teil3/14_aussparen.md). Je Loch: das andere Objekt mit dem kleinsten mittleren Abstand (beide Richtungen);
„genau“ = ≤ 0,05 mm; sonst Abstand mit Vorzeichen (negativ = Loch kleiner als das Objekt = Überlappung).

Aufruf:  python art_aussparen.py [SAMPLES_ORDNER] [TWOFISH_EXE]
"""
import glob
import math
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import objekt_auswerten  # noqa: E402
from art_messen import _umriss, laden  # noqa: E402

ORDNER = sys.argv[1] if len(sys.argv) > 1 else r'F:\Entwicklung\artista_ziel\Main\Samples'
TWOFISH = sys.argv[2] if len(sys.argv) > 2 else r'F:\Entwicklung\werkzeuge\twofish.exe'
pfade = sorted(glob.glob(os.path.join(ORDNER, '**', '*.ART'), recursive=True))


def d_punkt_strecke(p, a, b):
    vx, vy = b[0] - a[0], b[1] - a[1]
    l2 = vx * vx + vy * vy
    t = 0 if l2 == 0 else max(0, min(1, ((p[0] - a[0]) * vx + (p[1] - a[1]) * vy) / l2))
    return math.hypot(p[0] - a[0] - t * vx, p[1] - a[1] - t * vy)


def d_punkt_ring(p, ring):
    return min(d_punkt_strecke(p, ring[i], ring[(i + 1) % len(ring)]) for i in range(len(ring)))


def innen(p, ring):
    c = False
    for i in range(len(ring)):
        a, b = ring[i], ring[(i + 1) % len(ring)]
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < a[0] + (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]):
            c = not c
    return c


def box(r):
    xs, ys = [p[0] for p in r], [p[1] for p in r]
    return min(xs), min(ys), max(xs), max(ys)


stat = Counter()
treffer = []
for pfad in pfade:
    try:
        d, h, pts, segs, objs = laden(pfad, TWOFISH)
    except Exception:  # noqa: BLE001
        continue
    aus = []
    for o in objs:
        try:
            aus.append(objekt_auswerten(o))
        except Exception:  # noqa: BLE001
            aus.append({'klasse': o['klasse'], 'formen': []})
    ringe = []   # (objekt, formnr, ring)
    for n, a in enumerate(aus):
        for k, f in enumerate(a['formen']):
            if f['geschlossen'] and len(f['knoten']) >= 2:
                ringe.append((n, k, _umriss(f, 8)))
    for n, a in enumerate(aus):
        geschl = [k for k, f in enumerate(a['formen']) if f['geschlossen']]
        if len(geschl) < 2:
            continue
        stat['objekte_mit_mehreren_ringen'] += 1
        haupt = _umriss(a['formen'][geschl[0]], 8)
        for k in geschl[1:]:
            loch = _umriss(a['formen'][k], 8)
            if not all(innen(p, haupt) for p in loch[::4]):
                continue
            stat['loecher'] += 1
            bl = box(loch)
            best = None
            for (m, kk, r) in ringe:
                if m == n:
                    continue
                br = box(r)
                if br[2] < bl[0] - 2 or br[0] > bl[2] + 2 or br[3] < bl[1] - 2 or br[1] > bl[3] + 2:
                    continue
                dl = sum(d_punkt_ring(p, r) for p in loch) / len(loch)
                dr = sum(d_punkt_ring(p, loch) for p in r) / len(r)
                if best is None or dl + dr < best[0]:
                    # Vorzeichen: Lochpunkte innerhalb des anderen Umrisses → Loch kleiner (Überlappung)
                    drin = sum((-1 if innen(p, r) else 1) * d_punkt_ring(p, r) for p in loch) / len(loch)
                    best = (dl + dr, dl, dr, m, kk, drin)
            if best and best[0] < 1.0:
                stat['loch_passt_zu_objekt'] += 1
                treffer.append((os.path.basename(pfad), n, k, aus[n]['klasse'], best[3], aus[best[3]]['klasse'], best[3] > n,
                                round(best[1], 3), round(best[2], 3), round(best[5], 2)))
print(stat)
ex = [t for t in treffer if t[7] < 0.05]
for t in ex:
    print('genau', t)
vz = sorted(t[9] for t in treffer if t[7] >= 0.05)
print('Loch mit Vorzeichen (negativ = Loch kleiner als Objekt = Überlappung), n', len(vz))
print('  Quantile 10/25/50/75/90 %:', [vz[int(q * (len(vz) - 1))] for q in (0.1, 0.25, 0.5, 0.75, 0.9)])
print('  Anteil Überlappung:', sum(v < 0 for v in vz) / len(vz))
dl = [t[7] for t in treffer]
if dl:
    dl.sort()
    print('mittlerer Abstand Loch→Objekt: Median', dl[len(dl) // 2], 'Anteil < 0,05 mm', sum(x < 0.05 for x in dl) / len(dl))
    print('Schneidendes Objekt liegt danach (oben):', sum(t[6] for t in treffer), 'von', len(treffer))
