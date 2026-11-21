"""Satin-Einstichmuster gegen die gespeicherten Stichlinien (EmbroideryPatch 0x06):
r = Vorschub des Einstichs auf der Gegenseite / Vorschub zwischen zwei Einstichen derselben Seite, gemessen längs der
Säule (senkrecht zur Stichlinie). r≈0 oder 1: ein Stich liegt genau auf der Stichlinienrichtung (gerade quer),
r≈0,5: beide Stiche gleich schräg (symmetrischer Zickzack). Bericht: Analyse_Teil3/04_messungen_satin.md §5.

Aufruf:  python art_satinmuster.py   (Musterdesigns unter F:/Entwicklung/artista_ziel/Main/Samples, twofish.exe unter
         F:/Entwicklung/werkzeuge)
"""
import sys,glob,math,collections
sys.path.insert(0,'Arbeitsdaten/werkzeuge')
from art_messen import zugeordnet
from art_autosatin import stichlinien, segment_punkt
from art_design import objekt_auswerten
pfade=sorted(glob.glob('F:/Entwicklung/artista_ziel/Main/Samples/**/*.ART',recursive=True))
h=collections.Counter(); jeobj=[]; nobj=0
for pf,o,refs,f,_ in zugeordnet(pfade,'F:/Entwicklung/werkzeuge/twofish.exe'):
    if o['klasse']!='EmbroideryPatch': continue
    roh=next((e[3] for e in o['datensaetze'] if e[1]==0x06 and not e[2]),None)
    if not roh or len(roh)<2: continue
    linien,_=stichlinien(roh)
    formen=[x for x in objekt_auswerten(o)['formen'] if len(x['knoten'])>=2]
    if len(linien)<2 or len(formen)!=1: continue
    L=[(segment_punkt(formen[0],a,ta),segment_punkt(formen[0],b,tb)) for (a,ta),(b,tb) in linien]
    pts=[q for e,blk in zip(refs,f) if e[1]==0x16 for q in blk['punkte']]
    if len(pts)<10: continue
    nobj+=1; rs=[]
    for k in range(len(L)-1):
        (p1,q1),(p2,q2)=L[k],L[k+1]
        v1=(q1[0]-p1[0],q1[1]-p1[1]); v2=(q2[0]-p2[0],q2[1]-p2[1])
        l1,l2=math.hypot(*v1),math.hypot(*v2)
        if l1<1.5 or l2<1.5: continue
        w=math.remainder(math.atan2(v1[1],v1[0])-math.atan2(v2[1],v2[0]),2*math.pi)
        if abs(w)>math.radians(3) or abs(l1-l2)>0.3: continue
        rx,ry=(v1[0]/l1+v2[0]/l2)/2,(v1[1]/l1+v2[1]/l2)/2; rl=math.hypot(rx,ry); rx,ry=rx/rl,ry/rl
        ux,uy=-ry,rx
        t1=((p1[0]+q1[0])/2)*ux+((p1[1]+q1[1])/2)*uy; t2=((p2[0]+q2[0])/2)*ux+((p2[1]+q2[1])/2)*uy
        lo,hi=min(t1,t2)+0.3,max(t1,t2)-0.3
        if hi-lo<1.0: continue
        t=[x*ux+y*uy for x,y in pts]
        for i in range(len(pts)-2):
            if not all(lo<=t[j]<=hi for j in (i,i+1,i+2)): continue
            if math.dist(pts[i],pts[i+1])<1.2: continue
            d=t[i+2]-t[i]
            if abs(d)<0.15: continue
            rs.append((t[i+1]-t[i])/d)
    if len(rs)>=8:
        for r in rs: h[round(r*10)/10]+=1
        rs.sort(); jeobj.append(min(rs[len(rs)//2],1-rs[len(rs)//2]))
print('Objekte mit Stichlinien',nobj,'ausgewertet',len(jeobj))
print('Verteilung r:',sorted(h.items()))
jeobj.sort(); n=len(jeobj)
print('je Objekt min(r,1-r) Median: Quartile',[round(jeobj[int(n*q)],3) for q in (0.1,0.25,0.5,0.75,0.9)])
