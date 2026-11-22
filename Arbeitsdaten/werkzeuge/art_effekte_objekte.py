"""Objekte eines ART-Musterdesigns mit Formen und Stichblöcken als JSON (für die Effekt-Messungen, Analyse_Teil3/23).
Aufruf: python art_effekte_objekte.py WERKZEUGORDNER TWOFISH DATEI.ART NR,NR,… AUS.json"""
import sys, os, json, math
sys.path.insert(0, sys.argv[1])
from art_messen import laden, funktionen_je_objekt, funktions_verweise
from art_design import objekt_auswerten
tw = sys.argv[2]; pfad = sys.argv[3]; nrn = [int(x) for x in sys.argv[4].split(',')]
d, h, pts, segs, objs = laden(pfad, tw)
fo = funktionen_je_objekt(segs, pts)
print(os.path.basename(pfad), 'Objekte', len(objs), 'Stichobjekte', len(fo))
out = {}
for n in nrn:
    o = objs[n]; r = objekt_auswerten(o)
    refs = funktions_verweise(o)
    print(f'#{n} {r["klasse"]} Funktionen {[hex(e[1]) for e in refs]} Blöcke {[len(f["punkte"]) for f in fo[n]] if n < len(fo) else None}')
    for k, t, f in r['params']:
        if k in (0x16, 0x1b, 0x15, 0x18):
            print('    ', hex(k), hex(t) if t else '', f)
    out[n] = {'formen': [[(x / 180, y / 180) for x, y, *_ in f['knoten']] if f['knoten'] and isinstance(f['knoten'][0], (list, tuple)) else f['knoten'] for f in r['formen']],
              'funktionen': [[hex(e[1]) for e in refs]], 'bloecke': [f['punkte'] for f in fo[n]] if n < len(fo) else []}
json.dump(out, open(sys.argv[5], 'w'), default=str)
