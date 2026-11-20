"""Wendefüllung (EmbroideryPatch mit Winkellinien, Step 0x1b): Richtung der Step-Stiche gegen die nächste
Winkellinie (Analyse_Teil3/06 §6). Aufruf:  python art_wendung.py  (die ersten ~400 Objekte der Musterdesigns)
"""
import collections
import glob
import math
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_saeulenfolge import objekte2, TW, saeule, seg_punkt  # noqa: E402
from art_design import felder, objekt_auswerten  # noqa: E402
from art_messen import funktionen_je_objekt, funktions_verweise, kette  # noqa: E402
from art_stiche import design_daten, kopf, segmente_dekodieren, stiche_dekodieren  # noqa: E402
from esb_lesen import winkellinien  # noqa: E402

stat = collections.Counter()
diff = collections.Counter()
n_obj = 0
for pfad in sorted(glob.glob(r'F:/Entwicklung/artista_ziel/Main/Samples/*/*.ART')):
    if n_obj > 400:
        break
    try:
        d = design_daten(pfad, TW); h = kopf(d)
        pts, _ = stiche_dekodieren(d, h[6], h[5]); pts = [(x / 180, y / 360) for x, y in pts]
        segs, _ = segmente_dekodieren(d, h[9], h[8], h[4])
        objs = objekte2(d, h); fo = funktionen_je_objekt(segs, pts)
    except Exception:
        continue
    if len(fo) == len(objs):
        mit = objs
    elif len(fo) == len(objs) - 1:
        mit = objs[1:]
    else:
        stat['Design: Zuordnung unklar'] += 1
        continue
    for o, f in zip(mit, fo):
        refs = funktions_verweise(o)
        if o['klasse'] != 'EmbroideryPatch' or len(refs) != len(f):
            continue
        roh = next((e[3] for e in o['datensaetze'] if e[1] == 0x06 and not e[2]), None)
        r = objekt_auswerten(o)
        formen = [x for x in r['formen'] if x['geschlossen'] and len(x['knoten']) >= 3]
        if not roh or not formen:
            continue
        w, _ = winkellinien(roh)
        kn = formen[0]['knoten']
        mm = lambda q: (q[0] / 180, q[1] / 180)
        linien = [(mm(seg_punkt(kn, l[0] + l[1])), mm(seg_punkt(kn, l[2] + l[3]))) for l in w]
        for e, blk in zip(refs, f):
            if e[1] != 0x1b:
                continue
            ch = kette(e)
            t = next((felder(0x1b, x[3]) for x in ch if x[1] == 0x1b), None)
            if not t or 'laenge' not in t:
                continue
            L = t['laenge'] / 180
            P = blk['punkte']
            if len(P) < 50:
                continue
            n_obj += 1
            for a, b in zip(P, P[1:]):
                dd = math.dist(a, b)
                if abs(dd - L) > 0.1 * L:
                    continue
                m = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
                # nächste Winkellinie (Abstand Mittelpunkt – Strecke)
                def abst(l):
                    (x1, y1), (x2, y2) = l
                    dx, dy = x2 - x1, y2 - y1
                    LL = dx * dx + dy * dy or 1e-9
                    tt = max(0, min(1, ((m[0] - x1) * dx + (m[1] - y1) * dy) / LL))
                    return math.hypot(m[0] - x1 - tt * dx, m[1] - y1 - tt * dy)
                l = min(linien, key=abst)
                wl = math.degrees(math.atan2(l[1][1] - l[0][1], l[1][0] - l[0][0]))
                ws = math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))
                dw = abs(((ws - wl) + 90) % 180 - 90)
                diff[int(dw // 10) * 10] += 1
            break
print('Objekte', n_obj, dict(stat))
g = sum(diff.values())
for k in sorted(diff):
    print(f'{k:3}–{k + 10:3}°: {100 * diff[k] / g:5.1f} %')
