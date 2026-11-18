"""Parser für das Wilcom/BERNINA 'DataItem'-Binärformat (aus WESSYS.DLL 0x1003a6c0 rekonstruiert)."""
import struct, sys
TYPES={1:'long',2:'ulong',3:'double',4:'str',5:'wstr',6:'list',7:'byte'}
def parse(d,o):
    a,b,c,t,n=struct.unpack_from('<HHHHI',d,o); o+=12
    node={'id':a|(b<<16),'flags':c,'type':TYPES.get(t,t)}
    if n==0 or t not in TYPES:
        node['value']=None
    elif t==6:
        ch=[]
        for i in range(n):
            x,o=parse(d,o); ch.append(x)
        node['children']=ch
    elif t in (1,): node['value']=struct.unpack_from('<i',d,o)[0]; o+=4
    elif t==2: node['value']=struct.unpack_from('<I',d,o)[0]; o+=4
    elif t==3: node['value']=struct.unpack_from('<d',d,o)[0]; o+=8
    elif t==4: node['value']=d[o:o+n].rstrip(b'\0').decode('latin1'); o+=n
    elif t==5: node['value']=d[o:o+2*n].decode('utf-16le').rstrip('\0'); o+=2*n
    elif t==7: node['value']=d[o]; o+=1
    else: raise ValueError('type %r at %x'%(t,o))
    return node,o
def dump(n,ind=0,out=sys.stdout):
    s='  '*ind+'[0x%04x f=%d %s]'%(n['id'],n['flags'],n['type'])
    if 'value' in n: s+=' = %r'%(n['value'],)
    if 'children' in n: s+=' (%d)'%len(n['children'])
    print(s,file=out)
    for c in n.get('children',[]): dump(c,ind+1,out)
if __name__=='__main__':
    d=open(sys.argv[1],'rb').read(); off=int(sys.argv[2],0) if len(sys.argv)>2 else 4
    o=off
    while o<len(d):
        n,o=parse(d,o); dump(n)
