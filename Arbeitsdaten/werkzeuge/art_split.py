"""Auto-Split im Original messen (Analyse_Teil3/10 §5): Satinfunktionen (0x16) der Musterdesigns in Querzüge (Folgen
fast gerader Stiche, Knick < 12°) zerlegen; je Querzug Gesamtlänge, Zahl der Teile und Teilungsstellen (Anteile).
Ergebnis: Arbeitsdaten/messungen/satin_split_musterdesigns.txt.

Aufruf:  python art_split.py
"""
import collections, glob, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_messen import zugeordnet, MUSTER  # noqa
from art_design import objekt_auswerten  # noqa
TW = r'F:\Entwicklung\werkzeuge\twofish.exe'


def winkel(a, b, c):
    v1 = (b[0] - a[0], b[1] - a[1]); v2 = (c[0] - b[0], c[1] - b[1])
    n1, n2 = math.hypot(*v1), math.hypot(*v2)
    if n1 < 1e-9 or n2 < 1e-9:
        return 180
    cosw = (v1[0] * v2[0] + v1[1] * v2[1]) / n1 / n2
    return math.degrees(math.acos(max(-1, min(1, cosw))))


teile = collections.Counter()
anteile = collections.defaultdict(list)
laengen = collections.defaultdict(list)
folge_bsp = []
n_obj = 0
for pfad, o, refs, f, _ in zugeordnet(sorted(glob.glob(MUSTER)), TW):
    split = [p.get('auto_split') for k, t, p in objekt_auswerten(o)['params'] if k == 0x16]
    for e, blk in zip(refs, f):
        if e[1] != 0x16:
            continue
        P = blk['punkte']
        # Querzüge: aufeinanderfolgende Stiche, die fast gerade weiterlaufen (Knick < 12°)
        i = 0
        zuege = []
        while i < len(P) - 1:
            j = i + 1
            while j + 1 < len(P) and winkel(P[j - 1], P[j], P[j + 1]) < 12 and math.dist(P[j], P[j + 1]) > 0.3:
                j += 1
            zuege.append((i, j))
            i = j
        lange = [(a, b) for a, b in zuege if math.dist(P[a], P[b]) > 9]
        if not lange:
            continue
        n_obj += 1
        for a, b in lange:
            L = math.dist(P[a], P[b])
            k = b - a
            teile[(round(L), k)] += 1
            laengen[k].append(L)
            if k >= 2:
                cum = 0
                fr = []
                for m in range(a, b - 1):
                    cum += math.dist(P[m], P[m + 1])
                    fr.append(round(cum / L, 2))
                anteile[k].append(tuple(fr))
        if len(folge_bsp) < 4:
            folge_bsp.append((pfad.split('\\')[-1].split('/')[-1], split, [(round(math.dist(P[a], P[b]), 1), b - a) for a, b in lange[:12]],
                              [tuple(round(math.dist(P[m], P[m + 1]), 2) for m in range(a, b)) for a, b in lange[:8]]))
print('Säulenfunktionen mit Querzügen > 9 mm:', n_obj)
for k in sorted(laengen):
    v = sorted(laengen[k])
    print(f'{k} Teile: n={len(v):6}  Länge {v[0]:.1f}–{v[-1]:.1f} mm, Median {v[len(v) // 2]:.1f}')
for k in sorted(anteile):
    c = collections.Counter(anteile[k]).most_common(8)
    print(f'{k} Teile, häufigste Teilungsstellen (Anteil der Länge):', c)
for b in folge_bsp:
    print(b)
