"""Extrahiert alle benannten Stile aus der entschlüsselten NORMAL.AMT und vergleicht sie mit '0 Software Defaults'.
Ausgabe: Pfad (Tag-Kette) -> Wert je Stil; Längen in Wilcom-Einheiten (1 mm = 180) zusätzlich in mm."""
import sys,re,collections; sys.path.insert(0,sys.path[0]); from dataitem import parse
d=open(sys.argv[1],'rb').read()
styles=collections.OrderedDict()
for m in re.finditer(rb'\xb3\x3e\x00\x00\x00\x00\x04\x00',d):
    try:
        n,e=parse(d,m.start()-12)
        if n['type']=='list' and n['children'][0]['id']==0x3eb3: styles[n['children'][0]['value']]=n['children'][1]
    except Exception: pass
def flat(n,path='',out=None):
    if out is None: out=collections.OrderedDict()
    cnt=collections.Counter()
    for c in n.get('children',[]):
        cnt[c['id']]+=1; key='%s/%04x'%(path,c['id'])+('#%d'%cnt[c['id']] if cnt[c['id']]>1 else '')
        if 'children' in c: flat(c,key,out)
        else: out[key]=c.get('value')
    return out
F={k:flat(v) for k,v in styles.items()}
base=F['0 Software Defaults']
def mm(v):
    if isinstance(v,int) and 0<v<20000 and v%9==0: return '%d (%.2f mm)'%(v,v/180)
    if isinstance(v,int) and 0<v<20000: return '%d (%.3f mm?)'%(v,v/180)
    if isinstance(v,float): return '%g'%v
    return repr(v)
mode=sys.argv[2] if len(sys.argv)>2 else 'diff'
if mode=='base':
    for k,v in base.items(): print(k, mm(v))
else:
    for name,f in F.items():
        if name=='0 Software Defaults': continue
        diffs=[(k,base.get(k,'<fehlt>'),v) for k,v in f.items() if repr(base.get(k,'<fehlt>'))!=repr(v)]
        print('### %s  (%d Abweichungen von Software Defaults)'%(name,len(diffs)))
        for k,a,b in diffs[:400]: print('   %-55s Standard=%-22s hier=%s'%(k,mm(a) if a!='<fehlt>' else a,mm(b)))
