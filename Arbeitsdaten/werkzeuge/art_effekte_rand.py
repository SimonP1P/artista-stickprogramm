"""Ausgefranster Rand (Textured Edge) messen: Abstand der Reihenenden zum Umriss je Seite (Analyse_Teil3/23 §4).
Aufruf: python art_effekte_rand.py OBJEKTE.json:NR:BLOCK"""
import sys, json, math, collections
def seg_d(p, a, b):
    dx, dy = b[0]-a[0], b[1]-a[1]; L = dx*dx+dy*dy
    t = 0 if L == 0 else max(0, min(1, ((p[0]-a[0])*dx + (p[1]-a[1])*dy) / L))
    return math.hypot(p[0]-a[0]-t*dx, p[1]-a[1]-t*dy)
def inside(p, poly):
    c = False
    for i in range(len(poly)):
        a, b = poly[i], poly[i-1]
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < (b[0]-a[0])*(p[1]-a[1])/(b[1]-a[1])+a[0]: c = not c
    return c
f, nr, bl = sys.argv[1].split(':')
o = json.load(open(f))[nr]; pts = o['bloecke'][int(bl)]
poly = [(k['punkt'][0]/180, k['punkt'][1]/180) for k in o['formen'][0]]
arten = collections.Counter(k['art'] for k in o['formen'][0])
# Wendepunkte: Richtungswechsel entlang der Hauptrichtung
h = collections.Counter()
for a, b in zip(pts, pts[1:]):
    dx, dy = b[0]-a[0], b[1]-a[1]; L = math.hypot(dx, dy)
    if L > 0.8: h[round(math.degrees(math.atan2(dy, dx)) % 180)] += L
w = math.radians(h.most_common(1)[0][0]); ux, uy = math.cos(w), math.sin(w)
ds = []
for i in range(1, len(pts)-1):
    d1 = (pts[i][0]-pts[i-1][0])*ux + (pts[i][1]-pts[i-1][1])*uy
    d2 = (pts[i+1][0]-pts[i][0])*ux + (pts[i+1][1]-pts[i][1])*uy
    if d1 * d2 < 0:
        d = min(seg_d(pts[i], poly[j], poly[j-1]) for j in range(len(poly)))
        ds.append(((d if inside(pts[i], poly) else -d), 'P' if d1 > 0 else 'M'))
for seite in 'PM':
    x = sorted(v for v, s_ in ds if s_ == seite)
    print(seite, len(x), ' '.join(f'{v:.2f}' for v in x))
