"""Verlauf (Gradient Fill) messen: Abstand der Reihen über die Fläche gegen lineare Profile (Analyse_Teil3/23 §3).
Aufruf: python art_effekte_verlauf.py OBJEKTE.json NR BLOCK MIN_MM MAX_MM (OBJEKTE.json aus art_effekte_objekte.py)"""
import sys, json, math, collections
o = json.load(open(sys.argv[1]))[sys.argv[2]]; bl = int(sys.argv[3]); mn, mx = float(sys.argv[4]), float(sys.argv[5])
pts = o['bloecke'][bl]
form = [(k['punkt'][0] / 180, k['punkt'][1] / 180) for f in o['formen'] for k in f]
h = collections.Counter()
for a, b in zip(pts, pts[1:]):
    dx, dy = b[0] - a[0], b[1] - a[1]; L = math.hypot(dx, dy)
    if L > 0.8: h[round(math.degrees(math.atan2(dy, dx)) % 180)] += L
w = h.most_common(1)[0][0]; r = math.radians(w); ux, uy = math.cos(r), math.sin(r); nx, ny = -uy, ux
reihen = []; akt = [pts[0]]; s0 = None
for a, b in zip(pts, pts[1:]):
    d = (b[0] - a[0]) * ux + (b[1] - a[1]) * uy; sg = 1 if d > 0 else -1
    if s0 is not None and sg != s0 and abs(d) > 0.3: reihen.append(akt); akt = [a]
    if abs(d) > 0.3: s0 = sg
    akt.append(b)
reihen.append(akt)
v = [sum(p[0] * nx + p[1] * ny for p in rr) / len(rr) for rr in reihen if len(rr) >= 2]
fn = [p[0] * nx + p[1] * ny for p in form]; lo, hi = min(fn), max(fn)
if v[-1] < v[0]: v = [-x for x in v]; lo, hi = -hi, -lo
print('Winkel', w, 'Form', round(hi - lo, 2), 'mm, Reihen', len(v))
for i in range(1, len(v)):
    t = ((v[i] + v[i-1]) / 2 - lo) / (hi - lo)
    lin_auf = (mn + (mx - mn) * t) / 2
    print(f't={t:4.2f} halb={v[i]-v[i-1]:5.2f} (lin auf {lin_auf:4.2f}, lin ab {(mx - (mx-mn)*t)/2:4.2f})')
