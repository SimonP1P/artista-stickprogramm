"""Säulenreihenfolge der Schrift im Original messen (Analyse_Teil3/11 §5).

Je LetterText: Buchstaben (LetterObj: Zeichen 0x3a, Grundlinie 0x2e) mit ihren Säulen (EmbroideryPatch: Umriss 0x01,
Winkellinien 0x06). Die gestickten Satin-Funktionen (0x16) des LetterText werden den Säulen zugeordnet (Punkte im
Umriss); Ergebnis je Buchstabe: Folge der Säulen (Nummer in Objektfolge) und Richtung (v = vom ersten zum letzten
Winkel, r = umgekehrt) samt Lage von Anfang/Ende des Stücks entlang der Säule (0 … 1).

Aufruf:  python art_saeulenfolge.py [ART …]   (ohne Angabe alle Musterdesigns; Twofish wie art_messen.py)
Hinweis: art_design.objekte bricht am Zeichen-Datensatz 0x3a eines LetterObj ab – hier `objekte2` ohne
diesen Abbruch.
"""
import collections
import glob
import math
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import KLASSEN, OBJEKT_KLASSEN, datensatz, objekt_auswerten  # noqa: E402
from art_messen import funktionen_je_objekt, funktions_verweise  # noqa: E402
from art_stiche import design_daten, kopf, segmente_dekodieren, stiche_dekodieren  # noqa: E402
from esb_lesen import winkellinien  # noqa: E402

TW = r'F:\Entwicklung\werkzeuge\twofish.exe'
MUSTER = r'F:/Entwicklung/artista_ziel/Main/Samples/*/*.ART'
KIND = {'LetterObj', 'EmbroideryBoundGroup', 'EmbroideryPatch', 'EmbroideryCurve', 'InputCCurve'}


def objekte2(d, h):
    """wie art_design.objekte, aber bricht nicht am Zeichen-Datensatz 0x3a eines LetterObj ab."""
    start, ende = h[14], h[14] + h[15]
    i = start
    geteilt, folge = [], []
    while i < ende:
        k, verw, l, a, b, j = datensatz(d, i)
        if (k > 0x9d and k != 0x2a2) or (k and not verw and l > ende - j):
            break
        if k == 0:
            folge.append((i, 0, False, b"")); i = j; continue
        if verw:
            ziel = None
            if 0 < l <= len(geteilt):
                n = geteilt[l - 1]; ziel = [folge[n]]
                while folge[n][1] not in (0, 0x0d) and n + 1 < len(folge):
                    n += 1; ziel.append(folge[n])
            folge.append((i, k, True, ziel)); i = j; continue
        roh = d[j:j + l]
        if a:
            geteilt.append(len(folge))
        folge.append((i, k, False, roh)); i = j + l
    pos = next(n for n, e in enumerate(folge) if e[1] == 0x0a and not e[2])
    objs, aktuell = [], None
    oben = 0
    for e in folge[pos + 1:]:
        off, k, verw, inhalt = e
        if k in (0x85, 0x41, 0x43) and not verw and objs:
            break
        if k in OBJEKT_KLASSEN and not verw:
            aktuell = {"offset": off, "klasse": KLASSEN.get(k, hex(k)), "datensaetze": []}
            objs.append(aktuell)
        if aktuell is not None:
            aktuell["datensaetze"].append(e)
    return objs


def bezier(kn, n=12):
    """geschlossener Umriss aus Knoten (Design-Einheiten) als Punktliste, je Segment n Stücke."""
    out = []
    m = len(kn)
    for s in range(m):
        a, b = kn[s], kn[(s + 1) % m]
        p0, p3 = a['punkt'], b['punkt']
        p1 = a['aus'] or p0
        p2 = b['ein'] or p3
        for i in range(n):
            t = i / n
            k = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
            out.append((k[0] * p0[0] + k[1] * p1[0] + k[2] * p2[0] + k[3] * p3[0],
                        k[0] * p0[1] + k[1] * p1[1] + k[2] * p2[1] + k[3] * p3[1]))
    return out


def seg_punkt(kn, pos):
    m = len(kn)
    pos = pos % m
    s = int(math.floor(pos)); t = pos - s
    a, b = kn[s], kn[(s + 1) % m]
    p0, p3 = a['punkt'], b['punkt']
    p1 = a['aus'] or p0
    p2 = b['ein'] or p3
    k = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
    return (k[0] * p0[0] + k[1] * p1[0] + k[2] * p2[0] + k[3] * p3[0],
            k[0] * p0[1] + k[1] * p1[1] + k[2] * p2[1] + k[3] * p3[1])


def innen(q, ring):
    x, y = q; c = False
    n = len(ring)
    for i in range(n):
        x1, y1 = ring[i]; x2, y2 = ring[(i + 1) % n]
        if (y1 > y) != (y2 > y) and x < x1 + (y - y1) * (x2 - x1) / (y2 - y1):
            c = not c
    return c


def abst_ring(q, ring):
    best = 1e18
    n = len(ring)
    for i in range(n):
        a, b = ring[i], ring[(i + 1) % n]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = dx * dx + dy * dy
        t = 0 if L == 0 else max(0, min(1, ((q[0] - a[0]) * dx + (q[1] - a[1]) * dy) / L))
        best = min(best, math.hypot(q[0] - a[0] - t * dx, q[1] - a[1] - t * dy))
    return best


def uni(roh):
    n = struct.unpack_from('<I', roh, 0)[0]
    return roh[4:4 + 2 * n].decode('utf-16-le', 'replace')


def saeule(o, mm):
    """EmbroideryPatch -> (Umriss mm, Anfang mm, Ende mm) oder None."""
    r = objekt_auswerten(o)
    formen = [f for f in r['formen'] if f['geschlossen'] and len(f['knoten']) >= 3]
    roh = next((e[3] for e in o['datensaetze'] if e[1] == 0x06 and not e[2]), None)
    if not formen or roh is None:
        return None
    kn = formen[0]['knoten']
    w, _ = winkellinien(roh)
    if len(w) < 2:
        return None
    ring = [mm(q) for q in bezier(kn)]
    def mitte(l):
        a = mm(seg_punkt(kn, l[0] + l[1])); c = mm(seg_punkt(kn, l[2] + l[3]))
        return ((a[0] + c[0]) / 2, (a[1] + c[1]) / 2), a, c
    achse = [mitte(l)[0] for l in w]
    return ring, mitte(w[0]), mitte(w[-1]), achse


def lage(q, achse):
    """Anteil 0 … 1 entlang der Achse (Mitten der Winkellinien) beim nächsten Punkt."""
    kum = [0.0]
    for a, b in zip(achse, achse[1:]):
        kum.append(kum[-1] + math.dist(a, b))
    if kum[-1] == 0:
        return 0.0
    best, bs = 1e18, 0.0
    for i, (a, b) in enumerate(zip(achse, achse[1:])):
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = dx * dx + dy * dy
        t = 0 if L == 0 else max(0, min(1, ((q[0] - a[0]) * dx + (q[1] - a[1]) * dy) / L))
        dd = math.hypot(q[0] - a[0] - t * dx, q[1] - a[1] - t * dy)
        if dd < best:
            best, bs = dd, kum[i] + t * math.sqrt(L)
    return bs / kum[-1]


def einstieg(gl, stat):
    """Je Säule beim ersten Stück: Einstieg am Anfang/Ende/innen, und ob das das nähere Ende zum vorigen Ausstich ist."""
    gesehen = set()
    vorher = None
    for k, (b, s, t0, t1, q0, q1, g) in enumerate(gl):
        if (b, s) not in gesehen:
            gesehen.add((b, s))
            ring, (m0, a0, c0), (m1, a1, c1), achse = g
            art = 'Anfang' if t0 < 0.05 else 'Ende' if t0 > 0.95 else 'innen'
            stat['Säule beginnt ' + art] += 1
            if vorher is not None and art != 'innen':
                d0, d1 = math.dist(vorher, m0), math.dist(vorher, m1)
                naeher = 'Anfang' if d0 < d1 else 'Ende'
                if abs(d0 - d1) > 1.0:
                    stat['Einstieg am näheren Ende' if naeher == art else 'Einstieg am ferneren Ende'] += 1
            # ganze Säule in einem Stück?
        vorher = q1
    # Stücke je Säule
    zahl = collections.Counter((b, s) for b, s, *_ in gl)
    for n in zahl.values():
        stat[f'Säule in {min(n, 4)} Stück(en)'] += 1


def auswerten(pfad, ausgabe, stat):
    d = design_daten(pfad, TW)
    h = kopf(d)
    pts, _ = stiche_dekodieren(d, h[6], h[5])
    pts = [(x / 180, y / 360) for x, y in pts]
    segs, _ = segmente_dekodieren(d, h[9], h[8], h[4])
    objs = objekte2(d, h)
    fo = funktionen_je_objekt(segs, pts)
    name = pfad.replace('\\', '/').split('/')[-1]
    for i, o in enumerate(objs):
        if o['klasse'] != 'LetterText':
            continue
        refs = funktions_verweise(o)
        arten = [e[1] for e in refs]
        kand = [f for f in fo if len(f) == len(refs)]
        if not kand:
            continue
        funk = None
        rec = [e for e in o['datensaetze'] if not e[2]]
        alphabet = next((uni(e[3]) for e in rec if e[1] == 0x3a and len(e[3]) > 4), '?')
        n_buchst = sum(1 for e in rec if e[1] == 0x80)
        # Buchstaben mit ihren Säulen
        buchst = []
        j = i + 1
        while j < len(objs) and objs[j]['klasse'] in KIND:
            oj = objs[j]
            if oj['klasse'] == 'LetterObj':
                z = next((uni(e[3]) for e in oj['datensaetze'] if e[1] == 0x3a and not e[2]), '?')
                buchst.append({'z': z, 'saeulen': []})
            elif oj['klasse'] == 'EmbroideryPatch' and buchst:
                buchst[-1]['saeulen'].append(oj)
            j += 1
        # Einheit der Formen: 1/180 mm in x und y (geprüft an den Stichen)
        mm = lambda q: (q[0] / 180, q[1] / 180)
        geo = []
        for b, bu in enumerate(buchst):
            for s, oj in enumerate(bu['saeulen']):
                g = saeule(oj, mm)
                if g:
                    geo.append((b, s, g))
        if not geo:
            continue
        def zuordnen(funk):
            erg = collections.defaultdict(list); ohne = 0
            for a, blk in zip(arten, funk):
                if a != 0x16 or len(blk['punkte']) < 4:
                    continue
                P = blk['punkte']; S = P[::max(1, len(P) // 30)]
                def treffer(g):
                    ring = g[2][0]
                    return sum(1 for q in S if innen(q, ring) or abst_ring(q, ring) < 0.5)
                best = max(geo, key=treffer)
                if treffer(best) / len(S) < 0.8:
                    ohne += 1
                    continue
                b, s_, (ring, (m0, a0, c0), (m1, a1, c1), achse) = best
                t0, t1 = lage(P[0], achse), lage(P[-1], achse)
                erg[b].append((s_, 'v' if t1 >= t0 else 'r', (round(t0, 2), round(t1, 2)), len(P)))
                erg['global'].append((b, s_, t0, t1, P[0], P[-1], best[2]))
            return erg, ohne
        bewertet = [(zuordnen(f), f) for f in kand]
        (folge, ohne), funk = max(bewertet, key=lambda x: sum(len(v) for v in x[0][0].values()) - x[0][1])
        stat['Satinfunktion ohne Säule'] += ohne
        glob_ = folge.pop('global', [])
        stat['Satinfunktion zugeordnet'] += len(glob_)
        einstieg(glob_, stat)
        stat['ohne Säule: ' + alphabet] += ohne
        stat['ohne Datei: ' + name + ' #' + str(i)] += ohne
        for b, bu in enumerate(buchst):
            f = folge.get(b, [])
            zus = [(s, r, si) for s, r, si, n in f]
            n_s = len(bu['saeulen'])
            reihen = [s for s, _, _ in zus]
            wie_digit = reihen == sorted(reihen) and len(set(reihen)) == len(reihen)
            alle_v = all(r == 'v' for _, r, _ in zus)
            key = ('Reihenfolge wie Objektfolge' if wie_digit else 'Reihenfolge anders') + \
                  (', alle vorwärts' if alle_v else ', teils rückwärts')
            if n_s >= 2:
                stat[key] += 1
                stat['Buchstaben mit >= 2 Säulen'] += 1
            stat['Säulen vorwärts'] += sum(1 for _, r, _ in zus if r == 'v')
            stat['Säulen rückwärts'] += sum(1 for _, r, _ in zus if r == 'r')
            ausgabe.append(f'{name} #{i} {alphabet} {bu["z"]!r}: {n_s} Säulen, gestickt {zus}')


if __name__ == '__main__':
    pfade = sys.argv[1:] or sorted(glob.glob(MUSTER))
    ausgabe, stat = [], collections.Counter()
    for pf in pfade:
        try:
            auswerten(pf, ausgabe, stat)
        except Exception as e:  # noqa: BLE001
            stat['Fehler: ' + type(e).__name__] += 1
    for k, v in sorted(stat.items()):
        print(f'{v:6}  {k}')
    print()
    for z in ausgabe[:400]:
        print(z)
