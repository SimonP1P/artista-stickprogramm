"""Schrift im Original: Reihenfolge von Unterlagen und Säulen (Analyse_Teil3/11).

Alle LetterText-Objekte der ART-Musterdesigns mit Unterlage (Stichfunktion Zickzack 0x17). Säulen (Satin 0x16), deren
Einstiche sich berühren (< 0,6 mm, Begrenzungen überlappen, höchstens 8 Funktionen auseinander), bilden eine Gruppe;
jede Unterlage gehört zur Säule, in deren Begrenzung mehr als 70 % ihrer Punkte liegen. Ausgabe: je Gruppe, ob alle
Unterlagen vor den Säulen kommen und ob ihre Reihenfolge umgekehrt zur Säulenfolge ist.

Aufruf:  python art_schrift.py [TWOFISH_EXE]
"""
import collections
import glob
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_messen import laden, funktionen_je_objekt, funktions_verweise, MUSTER  # noqa: E402
TW = sys.argv[1] if len(sys.argv) > 1 else r'F:\Entwicklung\werkzeuge\twofish.exe'


def bbox(P):
    xs = [p[0] for p in P]; ys = [p[1] for p in P]
    return min(xs), min(ys), max(xs), max(ys)


def ueberlapp(a, b, tol=0.25):
    return a[0] - tol <= b[2] and b[0] - tol <= a[2] and a[1] - tol <= b[3] and b[1] - tol <= a[3]


def nahe(P, Q, tol=0.6):
    """Berühren sich zwei Säulen (irgendein Einstich näher als tol)?"""
    Q2 = Q[::max(1, len(Q) // 60)]
    return any(math.dist(p, q) < tol for p in P[::max(1, len(P) // 60)] for q in Q2)


def innen(u, s):
    """Anteil der Unterlagen-Punkte innerhalb der Säulen-Begrenzung."""
    b = bbox(s)
    return sum(b[0] - 0.2 <= p[0] <= b[2] + 0.2 and b[1] - 0.2 <= p[1] <= b[3] + 0.2 for p in u) / len(u)


stat = collections.Counter()
beispiele = collections.defaultdict(list)
for pfad in sorted(glob.glob(MUSTER)):
    try:
        d, h, pts, segs, objs = laden(pfad, TW)
    except Exception:
        continue
    fo = funktionen_je_objekt(segs, pts)
    if len(fo) != len(objs):
        continue
    for o, f in zip(objs, fo):
        if o['klasse'] != 'LetterText':
            continue
        refs = funktions_verweise(o)
        if len(refs) != len(f):
            continue
        arten = [e[1] for e in refs]
        if 0x17 not in arten:
            continue
        S = [(k, blk['punkte']) for k, (a, blk) in enumerate(zip(arten, f)) if a == 0x16 and len(blk['punkte']) >= 4]
        U = [(k, blk['punkte']) for k, (a, blk) in enumerate(zip(arten, f)) if a == 0x17 and len(blk['punkte']) >= 2]
        # Gruppen: Säulen, die sich berühren (Union-Find), nur innerhalb eines Fensters von Nachbarn in der Stichfolge
        eltern = {k: k for k, _ in S}
        def wurzel(k):
            while eltern[k] != k:
                eltern[k] = eltern[eltern[k]]; k = eltern[k]
            return k
        for i, (k1, P) in enumerate(S):
            for k2, Q in S[i + 1:i + 8]:
                if ueberlapp(bbox(P), bbox(Q)) and nahe(P, Q):
                    eltern[wurzel(k2)] = wurzel(k1)
        # Unterlage -> Säule mit größtem Anteil
        zu = {}
        for ku, Pu in U:
            best = max(S, key=lambda s: innen(Pu, s[1]), default=None)
            if best and innen(Pu, best[1]) > 0.7:
                zu[ku] = best[0]
        gruppen = collections.defaultdict(lambda: {'S': [], 'U': []})
        for k, _ in S:
            gruppen[wurzel(k)]['S'].append(k)
        for ku, ks in zu.items():
            gruppen[wurzel(ks)]['U'].append((ku, ks))
        for g in gruppen.values():
            if not g['U']:
                stat['Gruppe ohne Unterlage'] += 1
                continue
            us = sorted(g['U']); ss = sorted(g['S'])
            zuerst = max(u for u, _ in us) < min(ss)
            if len(us) == 1:
                stat['1 Unterlage: vor der Gruppe' if zuerst else '1 Unterlage: nicht vor der Gruppe'] += 1
                continue
            # Reihenfolge der Unterlagen gegen die Reihenfolge ihrer Säulen
            sa = [ks for _, ks in us]
            umgekehrt = sa == sorted(sa, reverse=True)
            vorwaerts = sa == sorted(sa)
            key = f'{len(us)} Unterlagen: ' + ('alle zuerst' if zuerst else 'gemischt') + ', ' + (
                'umgekehrt zur Säulenfolge' if umgekehrt else 'wie Säulenfolge' if vorwaerts else 'andere Folge')
            stat[key] += 1
            if len(beispiele[key]) < 3:
                beispiele[key].append((pfad.split('\\')[-1].split('/')[-1], us, ss))
for k, v in sorted(stat.items(), key=lambda x: -x[1]):
    print(f'{v:5}  {k}')
for k, b in beispiele.items():
    print(k)
    for x in b:
        print('    ', x)
