"""Step-Muster: Versatz der Einstiche je Reihe und Stichlängen in den Musterdesigns „Sampler Step stitch
pattern 1-10 / 11-20“ (Analyse_Teil3/06 §5).

Aufruf:  python art_stepmuster.py          Schritte des Rasters zwischen räumlich benachbarten Reihen
         python art_stepmuster.py laengen  Stichlängen in den Reihen (Zufallsversatz 0x271b)
Hinweis: Die Flächen liegen im Design in Schlangenlinie – Fläche 4 trägt Muster 6, Fläche 6 Muster 4 (Beschriftung).
"""
import cmath
import glob
import math
import struct
import sys

sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from art_design import felder  # noqa: E402
from art_messen import funktionen_je_objekt, funktions_verweise, kette  # noqa: E402
from art_stiche import design_daten, kopf, segmente_dekodieren, stiche_dekodieren  # noqa: E402
from art_saeulenfolge import objekte2, TW  # noqa: E402


def reihen_aus(P, L):
    cx = cy = 0.0
    for a, b in zip(P, P[1:]):
        if abs(math.dist(a, b) - L) < 0.03:
            w = math.atan2(b[1] - a[1], b[0] - a[0])
            cx, cy = cx + math.cos(2 * w), cy + math.sin(2 * w)
    th = math.atan2(cy, cx) / 2
    c, s = math.cos(-th), math.sin(-th)
    R = [(x * c - y * s, x * s + y * c) for x, y in P]
    reihen, akt = [], [R[0]]
    for a, b in zip(R, R[1:]):
        d = math.dist(a, b)
        gerade = d > 0.25 and abs(b[1] - a[1]) / d < math.sin(math.radians(4))
        if gerade and (len(akt) < 2 or (b[0] - a[0]) * (akt[-1][0] - akt[-2][0]) > 0):
            akt.append(b)
        else:
            if len(akt) >= 5:
                reihen.append(akt)
            akt = [a, b] if gerade else [b]
    if len(akt) >= 5:
        reihen.append(akt)
    return reihen, math.degrees(th)


def phase(r, L):
    z = sum(cmath.exp(2j * math.pi * x / L) for x, _ in r[1:-1])
    return (cmath.phase(z) / (2 * math.pi)) % 1.0, abs(z) / max(1, len(r) - 2)


def phasen_bericht():
  for name in ['Sampler Step stitch pattern - 1-10', 'Sampler Step stitch pattern - 11-20']:
    pfad = glob.glob(f'F:/Entwicklung/artista_ziel/Main/Samples/*/{name}.ART')[0]
    d = design_daten(pfad, TW); h = kopf(d)
    pts, _ = stiche_dekodieren(d, h[6], h[5]); pts = [(x / 180, y / 360) for x, y in pts]
    segs, _ = segmente_dekodieren(d, h[9], h[8], h[4])
    objs = objekte2(d, h); fo = funktionen_je_objekt(segs, pts)
    from art_design import objekt_auswerten
    flaechen = [o for o in objs if o['klasse'] == 'ComplexFillGroup']
    print('==', name)
    for nr, o in enumerate(flaechen):
        r = objekt_auswerten(o)
        saetze = [tuple(f['versatz']) + (f['laenge'],) for k_, t_, f in r['params'] if k_ == 0x1b and 'versatz' in f]
        eigen = [x for x in saetze if x != (0.25, 0.25, 0.0, 720)]
        v1, v2, v3, lg = eigen[0] if eigen else (0.25, 0.25, 0.0, 720)
        L = lg / 180
        funk = max(fo[nr], key=lambda f: len(f['punkte']))
        P = funk['punkte']
        reihen, w = reihen_aus(P, L)
        # Reihen mit Lage (y), Richtung, Phase; längste Folge räumlich benachbarter Reihen (Abstand ~ halber Abstand)
        info = []
        for r_ in reihen:
            ph_, k_ = phase(r_, L)
            if k_ > 0.85 and len(r_) >= 6:
                y = sum(q[1] for q in r_) / len(r_)
                info.append((y, '>' if r_[-1][0] > r_[0][0] else '<', ph_, min(q[0] for q in r_), max(q[0] for q in r_)))
        info.sort()
        best, akt = [], []
        for a_ in info:
            if akt and 0.12 < a_[0] - akt[-1][0] < 0.3 and min(a_[4], akt[-1][4]) - max(a_[3], akt[-1][3]) > 2 * L:
                akt.append(a_)
            else:
                akt = [a_]
            if len(akt) > len(best):
                best = list(akt)
        seq = [x[2] for x in best][:18]
        rel = [round(((x - seq[0]) % 1.0), 2) for x in seq]
        diffs = [f"{b[1]}{round(((b[2] - a_[2]) % 1.0), 2)}" for a_, b in zip(best, best[1:])][:17]
        print(f'  Muster {nr + 1 + (10 if "11-20" in name else 0)}: L={L:.2f} Versatz=({v1}, {v2}, {v3}) Winkel={w:.0f} Reihen={len(reihen)} Punkte={len(P)}')
        print('     Phase relativ:', rel)
        print('     Schritt      :', diffs)


def laengen_bericht():
    import collections
    for name in ['Sampler Step stitch pattern - 1-10', 'Sampler Step stitch pattern - 11-20']:
        pfad = glob.glob(f'F:/Entwicklung/artista_ziel/Main/Samples/*/{name}.ART')[0]
        d = design_daten(pfad, TW); h = kopf(d)
        pts, _ = stiche_dekodieren(d, h[6], h[5]); pts = [(x / 180, y / 360) for x, y in pts]
        segs, _ = segmente_dekodieren(d, h[9], h[8], h[4])
        objs = objekte2(d, h); fo = funktionen_je_objekt(segs, pts)
        from art_design import objekt_auswerten
        flaechen = [o for o in objs if o['klasse'] == 'ComplexFillGroup']
        for nr, o in enumerate(flaechen):
            if nr not in (0, 1, 2, 4, 5) and '11-20' in name:
                pass
            r = objekt_auswerten(o)
            saetze = [tuple(f['versatz']) + (f['laenge'],) for k_, t_, f in r['params'] if k_ == 0x1b and 'versatz' in f]
            eigen = [x for x in saetze if x != (0.25, 0.25, 0.0, 720)]
            v1, v2, v3, lg = eigen[0] if eigen else (0.25, 0.25, 0.0, 720)
            L = lg / 180
            P = max(fo[nr], key=lambda f: len(f['punkte']))['punkte']
            reihen, w = reihen_aus(P, L)
            # Stichlängen im Inneren der Reihen (ohne ersten/letzten Stich), in Teilen von L
            c = collections.Counter()
            folgen = []
            for r_ in reihen:
                xs = [q[0] for q in r_]
                st = [abs(b - a) / L for a, b in zip(xs[1:-1], xs[2:-1])]
                for x in st:
                    c[round(x, 2)] += 1
                if len(st) >= 6 and len(folgen) < 3:
                    folgen.append([round(x, 2) for x in st[:10]])
            nummer = nr + 1 + (10 if '11-20' in name else 0)
            print(f'Muster {nummer} L={L} v=({v1},{v2},{v3}): häufigste Stichlängen/L', c.most_common(6))
            for f_ in folgen:
                print('      Reihe:', f_)


if __name__ == '__main__':
    laengen_bericht() if len(sys.argv) > 1 else phasen_bericht()
