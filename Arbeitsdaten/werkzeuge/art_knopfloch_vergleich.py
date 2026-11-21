"""Kern gegen Original: je Knopfloch Abstand jedes Einstichs zum nächsten Einstich der Gegenseite (Analyse_Teil3/19 §4).

Eingaben im ORDNER: Knopfloecher.exp (Kern), bh_stiche.svg.json (Stiche des Originals je Objekt/Funktion, aus
art_messen.laden + funktionen_je_objekt), knopfloecher_muster.json (von art_knopfloch.py).
Aufruf:  python art_knopfloch_vergleich.py ORDNER
"""
import json,math,sys
S=sys.argv[1]
d=open(S+'/Knopfloecher.exp','rb').read()
x=y=0; kern=[]
i=0
while i+1<len(d):
    a,b=d[i],d[i+1]
    if a==0x80:
        if b in(0x02,0x04): 
            dx,dy=[v-256 if v>127 else v for v in d[i+2:i+4]]; x+=dx; y+=dy; i+=4; continue
        i+=2; continue
    dx=a-256 if a>127 else a; dy=b-256 if b>127 else b
    x+=dx; y+=dy; kern.append((x/10,y/10)); i+=2
fo=json.load(open(S+'/bh_stiche.svg.json'))
orig=[tuple(q) for o in fo[2:11] for f in o for q in f]
pos=json.load(open(S+'/knopfloecher_muster.json'))['objekte']
for o in pos:
    cx,cy=o['position']
    box=lambda q:abs(q[0]-cx)<4 and abs(q[1]-cy)<14
    K=[q for q in kern if box(q)]; O=[q for q in orig if box(q)]
    dk=sorted(min(math.dist(q,r) for r in O) for q in K)
    do=sorted(min(math.dist(q,r) for r in K) for q in O)
    m=lambda v:v[len(v)//2]; p9=lambda v:v[int(len(v)*.9)]
    print('%-16s Einstiche Kern %4d Original %4d | Kern→Orig Median %.2f 90%% %.2f | Orig→Kern Median %.2f 90%% %.2f'%(o['typ'],len(K),len(O),m(dk),p9(dk),m(do),p9(do)))
