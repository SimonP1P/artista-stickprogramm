"""Musterdesigns: Effekte (0x21 AutoSpacing, 0x22 Faktor, 0x23 Kurzstiche, 0x24, 0x26) und Unterlagen je Objektart."""
import glob
import os
import struct
import sys
from collections import Counter, defaultdict

sys.path.insert(0, r'C:\Users\Simon\Desktop\AI Projekte\artista-stickprogramm\Arbeitsdaten\werkzeuge')
from art_design import objekte, KLASSEN  # noqa: E402
from art_stiche import design_daten, kopf  # noqa: E402

TF = r'F:\Entwicklung\werkzeuge\twofish.exe'
SCHRIFT = {0x2f, 0x30, 0x7a, 0x7b}
UNTER = {0x1d: 'Zickzack', 0x1e: 'DoppelZZ', 0x1f: 'Mittellauf', 0x20: 'EdgeWalk', 0x65: 'Step'}
ART = {0x16: 'Satin', 0x1b: 'Tatami', 0x18: 'Lauf', 0x19: 'Dreifach', 0x17: 'Zickzack', 0x3d: 'Motiv'}

zaehl = Counter()
werte23 = Counter()
werte24 = Counter()
faktor22 = Counter()
beispiele = defaultdict(list)
for f in sorted(glob.glob(r'F:\Entwicklung\artista_ziel\Main\Samples\**\*.ART', recursive=True)):
    try:
        d = design_daten(f, TF)
        h = kopf(d)
        objs, folge = objekte(d, h)
    except Exception:
        continue
    in_schrift = False
    for o in objs:
        k = next((e[1] for e in o['datensaetze'] if not e[2]), None)
        if k in SCHRIFT:
            in_schrift = True
        elif k not in (0x06, 0x05, 0x0c, 0x08):
            in_schrift = False
        # alle Datensätze inkl. Verweisziele
        flach = []

        def auf(e, t=0):
            if e[2]:
                if t < 8:
                    for x in e[3] or []:
                        auf(x, t + 1)
            else:
                flach.append(e)
        for e in o['datensaetze']:
            auf(e)
        klassen = [e[1] for e in flach]
        art = next((ART[c] for c in klassen if c in ART), '?')
        gruppe = ('Schrift' if in_schrift else 'Bild') + '/' + KLASSEN.get(k, hex(k or 0)) + '/' + art
        zaehl[(gruppe, 'Objekte')] += 1
        for c, n in ((0x21, 'AutoSpacing'), (0x24, '0x24'), (0x26, '0x26')):
            if c in klassen:
                zaehl[(gruppe, n)] += 1
        for e in flach:
            if e[1] == 0x23 and not e[2]:
                dd = e[3]
                if len(dd) >= 5:
                    fl = struct.unpack_from('<f', dd)[0]; an = dd[4]
                    zaehl[(gruppe, f'KS {fl:.1f}/{an}')] += 1
                break
        for c in set(klassen) & set(UNTER):
            zaehl[(gruppe, 'U-' + UNTER[c])] += 1
        for e in flach:
            if e[1] == 0x23 and e[3]:
                werte23[e[3][:24].hex()] += 1
            if e[1] == 0x24 and e[3]:
                werte24[e[3].hex()] += 1
            if e[1] == 0x22 and len(e[3]) >= 4:
                faktor22[(gruppe.split('/')[0], round(struct.unpack_from('<f', e[3])[0], 3))] += 1
        if 0x23 in klassen and len(beispiele[gruppe]) < 3:
            beispiele[gruppe].append(os.path.basename(f))

gruppen = sorted({g for g, _ in zaehl})
for g in gruppen:
    teile = {n: zaehl[(g, n)] for gg, n in zaehl if gg == g}
    print(f'{g:42s} ' + ', '.join(f'{n} {v}' for n, v in sorted(teile.items(), key=lambda x: -x[1])))
print('0x23-Daten:', werte23.most_common(5))
print('0x24-Daten:', werte24.most_common(5))
print('0x22-Faktor:', faktor22.most_common(10))
for g, b in beispiele.items():
    print('Beispiele Kurzstiche', g, b)
