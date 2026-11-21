"""Katalog der Google Fonts für die Schriftauswahl erzeugen: kern/stick-formate/daten/google_fonts.json.

Quellen (öffentlich, ohne Konto/Schlüssel; Stand siehe „stand“ im Katalog):
  * https://fonts.google.com/metadata/fonts – Familie, Kategorie, Schriftsysteme (subsets), Beliebtheit
  * https://raw.githubusercontent.com/google/fonts/main/tags/all/families.csv – Stil-Einordnung von Google
    (Familie, Gruppe/Tag, Gewicht 0–100)
  * https://api.github.com/repos/google/fonts/git/trees/main?recursive=1 – Dateipfade (ofl/…, apache/…, ufl/…)
Heruntergeladen wird später einzeln aus https://raw.githubusercontent.com/google/fonts/main/<pfad>.

Aufnahme: nur Familien mit lateinischer Schrift („latin“) und Ordner im Repo; je Familie **eine** Datei: aufrecht,
„-Regular“ vor variabler Schrift (Standardinstanz) vor erster aufrechter Datei.

Stile (deutsche Namen, **eigene Zuordnung** der Google-Tags – siehe STILE): Tags der Gruppe „Expressive“ zählen ab
Gewicht 80, alle anderen ab 50. Kategorie „Handwriting“ ohne Script-Tag → „Handschrift“.

Aufruf:  python3 Arbeitsdaten/werkzeuge/google_fonts_katalog.py [ORDNER_MIT_ROHDATEN]
  ohne Ordner werden die drei Quellen geladen (ca. 12 MB, nicht im Projekt gespeichert).
"""
import csv
import datetime
import io
import json
import os
import re
import sys
import urllib.request

QUELLEN = {
    'gf_meta.json': 'https://fonts.google.com/metadata/fonts',
    'gf_tags.csv': 'https://raw.githubusercontent.com/google/fonts/main/tags/all/families.csv',
    'gf_tree.json': 'https://api.github.com/repos/google/fonts/git/trees/main?recursive=1',
}
ZIEL = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'kern', 'stick-formate', 'daten',
                    'google_fonts.json')

KATEGORIE = {
    'Sans Serif': 'Ohne Serifen',
    'Serif': 'Mit Serifen',
    'Display': 'Zierschrift',
    'Handwriting': 'Handschrift',
    'Monospace': 'Schreibmaschine',
}

# Reihenfolge = Reihenfolge der Filter in der Oberfläche
STILE = [
    ('Gotisch', ['/Theme/Blackletter', '/Theme/Medieval']),
    ('Schreibschrift', ['/Script/Formal']),
    ('Handschrift', ['/Script/Handwritten', '/Script/Informal', '/Script/Upright Script']),
    ('Pinsel', ['/Theme/Brush']),
    ('Modern', ['/Sans/Geometric', '/Sans/Neo Grotesque', '/Sans/Superellipse', '/Expressive/Innovative']),
    ('Klassisch', ['/Serif/Old Style Garalde', '/Serif/Transitional', '/Serif/Humanist Venetian', '/Serif/Scotch',
                   '/Slab/Clarendon']),
    ('Elegant', ['/Expressive/Sophisticated', '/Expressive/Fancy', '/Serif/Didone', '/Serif/Modern']),
    ('Verspielt', ['/Expressive/Playful', '/Theme/Wacky', '/Theme/Blobby']),
    ('Kindlich', ['/Expressive/Childlike', '/Expressive/Cute', '/Purpose/Learn To Write']),
    ('Vintage', ['/Expressive/Vintage', '/Theme/Art Deco', '/Theme/Art Nouveau', '/Theme/Woodtype', '/Theme/Tuscan']),
    ('Kräftig', ['/Expressive/Loud', '/Serif/Fat Face']),
    ('Rund', ['/Sans/Rounded']),
    ('Technik', ['/Theme/Techno', '/Expressive/Futuristic']),
    ('Schablone', ['/Theme/Stencil']),
    ('Pixel', ['/Theme/Pixel']),
    ('Verziert', ['/Theme/Inline', '/Theme/Shaded']),
    ('Weihnachten', ['/Seasonal/Christmas']),
    ('Valentinstag', ["/Seasonal/Valentine's Day"]),
    ('Halloween', ['/Seasonal/Halloween']),
]


def lesen(ordner, name):
    if ordner:
        with open(os.path.join(ordner, name), 'rb') as f:
            return f.read()
    req = urllib.request.Request(QUELLEN[name], headers={'User-Agent': 'stickprogramm-katalog'})
    with urllib.request.urlopen(req, timeout=120) as r:
        return r.read()


def norm(s):
    return re.sub(r'[^a-z0-9]', '', s.lower())


def datei_waehlen(dateien):
    """dateien: [(lizenz, name, groesse)] → (lizenz, name, groesse) der aufrechten Standarddatei"""
    aufrecht = [d for d in dateien if 'italic' not in d[1].lower()]
    if not aufrecht:
        return None
    for d in aufrecht:
        if d[1].endswith('-Regular.ttf'):
            return d
    for d in aufrecht:
        if '[' in d[1]:
            return d
    return sorted(aufrecht, key=lambda d: d[1])[0]


def main():
    ordner = sys.argv[1] if len(sys.argv) > 1 else None
    meta = json.loads(lesen(ordner, 'gf_meta.json'))['familyMetadataList']
    # Tags je Familie und Achsenlage (variable Schriften: „wght@400“, „wght@900“ …); es gilt die Lage der
    # Standarddatei: ohne Angabe, sonst wght@400, sonst alle
    roh = {}
    for r in csv.reader(io.StringIO(lesen(ordner, 'gf_tags.csv').decode('utf-8'))):
        if len(r) < 4 or not r[2].startswith('/'):
            continue
        try:
            gewicht = float(r[3].strip('"'))
        except ValueError:
            continue
        schwelle = 80 if r[2].startswith('/Expressive/') else 50
        if gewicht >= schwelle:
            roh.setdefault(r[0], {}).setdefault(r[1], set()).add(r[2])
    tags = {}
    for fam, lagen in roh.items():
        wahl = [lagen[k] for k in lagen if k == ''] or [lagen[k] for k in lagen if 'wght@400' in k] or list(lagen.values())
        tags[fam] = set().union(*wahl)
    baum = json.loads(lesen(ordner, 'gf_tree.json'))
    assert not baum.get('truncated'), 'Dateiliste von GitHub abgeschnitten'
    ordner_dateien = {}
    for x in baum['tree']:
        m = re.match(r'^(ofl|apache|ufl)/([^/]+)/([^/]+\.ttf)$', x['path'])
        if m:
            ordner_dateien.setdefault(m.group(2), []).append((m.group(1), m.group(3), x.get('size', 0)))

    schriften = []
    for f in meta:
        if 'latin' not in f.get('subsets', []) or not f.get('isOpenSource', True):
            continue
        dateien = ordner_dateien.get(norm(f['family']))
        d = datei_waehlen(dateien or [])
        if not d:
            continue
        lizenz, name, groesse = d
        t = tags.get(f['family'], set())
        stile = [s for s, liste in STILE if t & set(liste)]
        if f['category'] == 'Handwriting' and not any(s in stile for s in ('Schreibschrift', 'Handschrift')):
            stile.insert(0, 'Handschrift')
        schriften.append({
            'familie': f['family'],
            'kategorie': KATEGORIE.get(f['category'], f['category']),
            'stile': stile,
            'pfad': f'{lizenz}/{norm(f["family"])}/{name}',
            'groesse': groesse,
            'beliebtheit': f.get('popularity', 9999),
            'lizenz': {'ofl': 'SIL Open Font License', 'apache': 'Apache 2.0', 'ufl': 'Ubuntu Font Licence'}[lizenz],
        })
    schriften.sort(key=lambda s: s['beliebtheit'])
    katalog = {
        'stand': datetime.date.today().isoformat(),
        'quelle': 'Google Fonts (fonts.google.com/metadata/fonts, github.com/google/fonts: tags/all/families.csv, '
                  'Dateien); Stile = eigene Zuordnung der Google-Tags (Arbeitsdaten/werkzeuge/google_fonts_katalog.py)',
        'stile': [s for s, _ in STILE],
        'schriften': schriften,
    }
    os.makedirs(os.path.dirname(ZIEL), exist_ok=True)
    with open(ZIEL, 'w', encoding='utf-8') as f:
        json.dump(katalog, f, ensure_ascii=False, separators=(',', ':'))
        f.write('\n')
    from collections import Counter
    z = Counter(s for x in schriften for s in x['stile'])
    print(f'{len(schriften)} Schriften -> {os.path.normpath(ZIEL)}')
    print(', '.join(f'{s} {z[s]}' for s, _ in STILE))


if __name__ == '__main__':
    main()
