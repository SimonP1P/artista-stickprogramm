"""Motivlinien (MotifText 0x7a) der ART-Musterdesigns vermessen (Analyse_Teil3/20_motive.md §3).

Je MotifText: Vorlage (LetterObj 0x2f + 0x2e + 0x3a + Form), Datensatz 0x7c (Größe = double 1), dann je Kopie ein
LetterObj 0x2f mit gepackten Wörtern [Oberpunkt x,y, Grundlinie Anfang x,y, Ende x,y]. Die Linie = GeoCurve 0x01 direkt
vor 0x7a. Ausgabe: Linienlänge L, Größe, L/Größe, Zähler (0x7c Byte 56 = Kopien + 1), Anfänge der Kopien entlang der
Linie, gemessener Schritt, L/n mit n = round(L/Größe).

Aufruf:  python art_motive.py "…/Samples/Reference/Sampler - Pattern Outlines - *.ART"
"""
import sys,struct,math,glob,os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_stiche import design_daten, kopf
from art_objekte import datensatz, packed_wort_s, geocurve
def recs_of(path):
    d=design_daten(path)
    h=kopf(d); i=h[14]; ende=i+h[15]; R=[]
    while i<ende:
        k,v,l,a,b,j=datensatz(d,i)
        if k==0: R.append((0,None)); i=j; continue
        if v: R.append((k,None)); i=j; continue
        R.append((k,d[j:j+l])); i=j+l
    return R
def words(r,q,n):
    w=[]
    for _ in range(n):
        x,q=packed_wort_s(r,q); w.append(x)
    return w,q
def blen(k):
    import itertools
    pts=[kn['punkt'] for kn in k]
    return sum(math.dist(a,b) for a,b in zip(pts,pts[1:])), pts
for path in sorted(glob.glob(sys.argv[1])):
    R=recs_of(path)
    for idx,(k,r) in enumerate(R):
        if k!=0x7a: continue
        bl=None
        for kk,rr in reversed(R[max(0,idx-3):idx]):
            if kk==1 and rr: bl,_=geocurve(rr,0); break
        size=None; cps=[]; name=None
        for kk,rr in R[idx+1:]:
            if kk in (0x7a,0x7b,0x3d,0x91): break
            if rr is None: continue
            if kk==0x2f:
                w,q=words(rr,1,6); cps.append(w)
            elif kk==0x3a and len(rr)>4 and name is None:
                n=struct.unpack('<I',rr[:4])[0]; name=rr[4:4+2*n].decode('utf-16le','replace')
            elif kk==0x7c and size is None:
                size,sp=struct.unpack('<2d',rr[3:19]); cnt=rr[56]
        if not bl or len(bl['knoten'])!=2: continue
        P=[kn['punkt'] for kn in bl['knoten']]; L=math.dist(*P)
        ux,uy=(P[1][0]-P[0][0])/L,(P[1][1]-P[0][1])/L
        tm=cps[0]; cps=cps[1:]
        s=[((c[2]-P[0][0])*ux+(c[3]-P[0][1])*uy) for c in cps]
        e=[((c[4]-P[0][0])*ux+(c[5]-P[0][1])*uy) for c in cps]
        off=[(-(c[2]-P[0][0])*uy+(c[3]-P[0][1])*ux) for c in cps]
        n=max(1,round(L/size))
        print(f"{os.path.basename(path)[28:45]:17s} {name:6s} L={L:.0f} size={size:.0f} L/size={L/size:.2f} cnt={cnt} ncop={len(cps)} starts={[round(x) for x in s[:3]]}..{[round(x) for x in s[-3:]]} lastend={(e[-2] if len(e)>1 else 0):.0f} off={off[0]:.0f} step={((s[-2]-s[0])/(len(s)-2) if len(s)>2 else 0):.1f} L/n={L/n:.1f} (L-size)/(n-1)={((L-size)/(n-1) if n>1 else 0):.1f}")
