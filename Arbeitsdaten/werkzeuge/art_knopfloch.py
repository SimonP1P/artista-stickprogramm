"""Knopflöcher des Originals messen (Analyse_Teil3/19_knopfloecher.md).

Teil 1 – Lage der 9 Knopflöcher aus Samples/Reference/Buttonholes.ART (Datensatz 0x95: u32 Typ 0x3ff9…0x4001, dann
unterer und oberer Schlitzpunkt als gepackte Wörter) → Design-JSON für den Kern.

Aufruf:  python art_knopfloch.py AUS.json      (danach: stick erzeugen AUS.json --alphabete …/wletter, Vergleich mit
         art_knopfloch_vergleich.py)
"""
import sys,struct,json
sys.path.insert(0,'Arbeitsdaten/werkzeuge')
from art_stiche import design_daten
from art_objekte import packed_wort_s
d=design_daten('F:/Entwicklung/artista_ziel/Main/Samples/Reference/Buttonholes.ART','F:/Entwicklung/werkzeuge/twofish.exe')
typen={0x3ff9:'standard',0x3ffa:'standard_schmal',0x3ffb:'rund',0x3ffc:'rundes_ende',0x3ffd:'auge_eckig',0x3ffe:'auge_rund',0x3fff:'auge_spitz',0x4000:'dehnbar',0x4001:'heirloom'}
objs=[]
for t,n in typen.items():
    pat=struct.pack('<I',t)
    i=d.find(pat)
    while i>=0:
        # Datensatz 0x95: Klasse-Byte davor? Plausibel prüfen: 4 gepackte Wörter dahinter
        q=i+4; w=[]
        for _ in range(4):
            v,q=packed_wort_s(d,q); w.append(v)
        if w[0]==w[2] and 2000<abs(w[3]-w[1])<6000:
            x0,y0,x1,y1=[v/180 for v in w]
            objs.append({'art':'knopfloch','typ':n,'position':[round(x0,3),round((y0+y1)/2,3)],'laenge':round(abs(y1-y0),3),'name':n})
            print(n,w,'Länge',(w[3]-w[1])/180)
            break
        i=d.find(pat,i+1)
json.dump({'name':'Knopfloecher','rahmen':'Large Oval 145 x 255','objekte':objs},open(sys.argv[1],'w'),indent=1)
