"""Füllmuster („Program Split“, Fancy Fill) aus der Vorlage NORMAL.AMT lesen (Analyse_Teil3/13).

Quelle: entschlüsselter Stream `Analyse/daten/entschluesselt/NORMAL.AMT.DesignDocument.bin`, gepackter Teil ab etwa
0x82c00 (Datensätze wie die ART-Objekte, `art_objekte.datensatz`). Je Muster (Reihenfolge der Datensätze):
  … Stilwerte …, eine oder mehrere **GeoCurve (Kennung 0x01)** = Linienzüge des Musters (Bézier-Knoten),
  **Name (Kennung 0x28)**, z. B. „001 Diamond (90)“ – die Zahl in Klammern ist der Stichwinkel in Grad,
  Kennung 0x2a2, Matrix 0x63, **Gitter (Kennung 0x3f)**: gepackte Wörter Bx, By, Ax, Ay, Ursprung x, Ursprung y
  (1/180 mm) – das Muster wiederholt sich um i·A + j·B. Geprüft an den lesbaren Mustern der Vorlage (DataItem-Abzug:
  Gitter 0x3aab Winkel/Länge, 0x3ab6/0x3ab7 = A, 0x3abb/0x3aba = B) und der Ausdehnung von „027 Heart2“, „144 Dynasty“.

Ausgabe: JSON-Liste {name, winkel (Grad oder null), a: [x, y], b: [x, y], zuege: [[[x, y] …] …]} in mm, Linienzüge
relativ zur linken unteren Ecke des Musters, Kurven in Strecken zerlegt (Fehler ≤ 0,02 mm).

Aufruf:  python muster_lesen.py [AUS.json]
"""
import json
import math
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_objekte import datensatz, geocurve, packed_wort_s  # noqa: E402

QUELLE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'Analyse', 'daten', 'entschluesselt',
                      'NORMAL.AMT.DesignDocument.bin')
EINHEIT = 180.0


def namen_finden(d):
    """Namens-Datensätze (Kennung 0x28, Länge = Namenslänge) im gepackten Teil."""
    out = []
    for m in re.finditer(rb'(\d{2,3} [A-Za-z][\x20-\x7e]*?)\x00', d):
        s = m.start()
        if s < 2:
            continue
        try:
            k, v, l, a, b, j = datensatz(d, s - 2)
        except Exception:  # noqa: BLE001
            continue
        if k == 0x28 and not v and j == s and l == len(m.group(1)):
            out.append((s - 2, m.group(1).decode('latin-1')))
    return out


def kette(d, von, bis):
    i, out = von, []
    while i < bis:
        k, v, l, a, b, j = datensatz(d, i)
        if k == 0 or v:
            out.append((k, None))
            i = j
            continue
        out.append((k, d[j:j + l]))
        i = j + l
    return out, i


def woerter(b):
    i, out = 0, []
    while i + 1 < len(b):
        v, i = packed_wort_s(b, i)
        out.append(v)
    return out


def bezier(p0, p1, p2, p3, tol=0.02 * EINHEIT):
    lang = math.dist(p0, p1) + math.dist(p1, p2) + math.dist(p2, p3)
    n = max(1, min(64, math.ceil(math.sqrt(lang / tol))))
    pts = []
    for k in range(1, n + 1):
        t = k / n
        u = 1 - t
        pts.append((u ** 3 * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t ** 3 * p3[0],
                    u ** 3 * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t ** 3 * p3[1]))
    return pts


def zug(g):
    """GeoCurve → Punktliste (Bézier-Abschnitte zerlegt)."""
    kn = g['knoten']
    if not kn:
        return []
    pts = [kn[0]['punkt']]
    paare = list(zip(kn, kn[1:])) + ([(kn[-1], kn[0])] if g['geschlossen'] else [])
    for a, b in paare:
        if a['aus'] is None and b['ein'] is None:
            pts.append(b['punkt'])
        else:
            pts.extend(bezier(a['punkt'], a['aus'] or a['punkt'], b['ein'] or b['punkt'], b['punkt']))
    return pts


def lesen(pfad=QUELLE):
    d = open(pfad, 'rb').read()
    namen = namen_finden(d)
    muster, gesehen = [], set()
    for n in range(1, len(namen)):
        pos, name = namen[n]
        _, _, pl, _, _, pj = datensatz(d, namen[n - 1][0])
        vorher, ende = kette(d, pj + pl, pos)
        if ende != pos:
            continue
        _, _, l, _, _, j = datensatz(d, pos)
        nach, _ = kette(d, j + l, namen[n + 1][0] if n + 1 < len(namen) else len(d))
        gitter = next((woerter(r) for k, r in nach if k == 0x3f and r), None)
        zuege = [zug(geocurve(r, 0)[0]) for k, r in vorher if k == 0x01 and r]
        zuege = [z for z in zuege if len(z) >= 2]
        if not zuege or not gitter or len(gitter) < 4 or name in gesehen:
            continue
        gesehen.add(name)
        bx, by, ax, ay = gitter[:4]
        if math.hypot(ax, ay) < 1 or math.hypot(bx, by) < 1 or abs(ax * by - ay * bx) < 1:
            continue
        x0 = min(p[0] for z in zuege for p in z)
        y0 = min(p[1] for z in zuege for p in z)
        w = re.search(r'\((\d+)\)\s*$', name)
        muster.append({
            'name': name.strip(),
            'winkel': int(w.group(1)) if w else None,
            'a': [round(ax / EINHEIT, 4), round(ay / EINHEIT, 4)],
            'b': [round(bx / EINHEIT, 4), round(by / EINHEIT, 4)],
            'zuege': [[[round((p[0] - x0) / EINHEIT, 3), round((p[1] - y0) / EINHEIT, 3)] for p in z] for z in zuege],
        })
    return sorted(muster, key=lambda m: m['name'])


if __name__ == '__main__':
    m = lesen()
    if len(sys.argv) > 1:
        json.dump(m, open(sys.argv[1], 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
    print(len(m), 'Muster')
    for x in m[:10]:
        print(f"  {x['name']:<26} Winkel {x['winkel']}  A {x['a']}  B {x['b']}  {len(x['zuege'])} Züge")
