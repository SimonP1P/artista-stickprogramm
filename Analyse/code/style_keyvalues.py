"""Kernwerte je Stil aus NORMAL.AMT: für jede Stichart-Liste (Kind von 0x2710) die Felder in mm."""
import sys,re,collections; sys.path.insert(0,sys.path[0]); from dataitem import parse
d=open(sys.argv[1],'rb').read()
ST={0x271c:'Lauf/Run',0x2721:'Satin',0x2714:'Step/Tatami',0x2724:'Zickzack',0x2727:'Blanket?',0x272a:'Verbinder',
    0x277e:'0x277e',0x278c:'Fancy/Muster',0x279a:'0x279a',0x2799:'0x2799',0x2713:'0x2713'}
def v(x):
    val=x.get('value')
    if x['type']=='ulong' and isinstance(val,int) and val<30000: return '%.2f'%(val/180)
    if x['type']=='double' and isinstance(val,float):
        return '%.0f°'%(val*57.29578) if 0<abs(val)<7 and abs(val-1.5708)<0.01 else '%g'%val
    return str(val)
def walk(n,chain,out):
    for c in n.get('children',[]):
        if c['id'] in ST and n['id']==0x2710:
            out.append(('/'.join('%04x'%i for i in chain), ST[c['id']], ' '.join('%04x=%s'%(g['id'],v(g)) for g in c.get('children',[]) if 'children' not in g)))
        if c['id']==0x2743: out.append(('/'.join('%04x'%i for i in chain),'Zugausgleich-Gruppe','2743=%s mm'%v(c)))
        walk(c,chain+[c['id']],out)
for m in re.finditer(rb'\xb3\x3e\x00\x00\x00\x00\x04\x00',d):
    n,e=parse(d,m.start()-12); name=n['children'][0]['value']
    if len(sys.argv)>2 and sys.argv[2] not in name: continue
    out=[]; walk(n['children'][1],[],out)
    print('#####',name)
    for c,t,s in out: print('  %-28s %-14s %s'%(c,t,s))
