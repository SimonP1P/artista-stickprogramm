"""Liest die 8 Stoff-Stile aus der entschlüsselten NORMAL.AMT (DesignDocument) und vergleicht sie."""
import sys,re,collections; sys.path.insert(0,sys.path[0]); from dataitem import parse
d=open(sys.argv[1],'rb').read()
fab={}
for m in re.finditer(rb'([1-8]) (Knit|Terry|Woven) (Light|Medium|Heavy) Weight\x00',d):
    p=m.start()-24
    node,e=parse(d,p)
    fab[m.group().rstrip(b'\0').decode()]=node
def flat(n,path='',out=None,cnt=None):
    if out is None: out=collections.OrderedDict()
    if cnt is None: cnt=collections.Counter()
    key='%s/%04x'%(path,n['id']); cnt[key]+=1; key+='#%d'%cnt[key] if cnt[key]>1 else ''
    if 'children' in n:
        for c in n['children']: flat(c,key,out,cnt)
    else: out[key]=n.get('value')
    return out
F={k:flat(v['children'][1]) for k,v in fab.items()}
names=list(F)
keys=list(F[names[0]].keys())
print('Stoffe:',names)
print('Anzahl Felder pro Stoff:',[len(F[n]) for n in names])
diff=[k for k in keys if len(set(repr(F[n].get(k)) for n in names))>1]
print('Felder, die sich unterscheiden:',len(diff))
def fmt(v):
    if isinstance(v,int) and v>=0 and v<100000: return '%d (=%.2f mm)'%(v,v/180)
    return repr(v)
for k in diff:
    print(k)
    for n in names: print('    %-24s %s'%(n,fmt(F[n].get(k))))
