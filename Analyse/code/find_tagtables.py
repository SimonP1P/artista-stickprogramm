import sys,struct,re; sys.path.insert(0,sys.path[0]); from rdis import Bin
def s_at(b,va):
    o=va-b.base
    if not 0<=o<len(b.img): return None
    m=re.match(rb'(?:[\x20-\x7e]\x00){2,}\x00\x00',b.img[o:o+200])
    if m: return m.group()[:-2].decode('utf-16le')
    m=re.match(rb'[\x20-\x7e]{2,}\x00',b.img[o:o+200])
    if m: return m.group()[:-1].decode()
for path in sys.argv[1:]:
    try: b=Bin(path)
    except Exception: continue
    img=b.img; res={}
    for layout in ((0,4,8),(4,0,8)):
        for i in range(0,len(img)-12,4):
            idv,ptr,typ=[struct.unpack_from('<I',img,i+k)[0] for k in layout]
            if 0<idv<0x10000 and typ<=7 and b.base<=ptr<b.base+len(img):
                s=s_at(b,ptr)
                if s and re.match(r'^[A-Za-z_][A-Za-z0-9_]*$',s): res[idv]=(s,typ)
    if res: print('##',path,len(res))
    for k,(s,t) in sorted(res.items()): print('0x%04x\t%s\t%d'%(k,s,t))
