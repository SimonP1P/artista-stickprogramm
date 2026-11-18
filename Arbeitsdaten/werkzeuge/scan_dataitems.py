"""Sucht in einem Binärblob alle gültigen DataItem-Bäume (Typ 6 = Liste) und gibt sie aus."""
import sys, struct
sys.path.insert(0, sys.path[0]); from dataitem import parse, dump, TYPES
def valid(n, depth=0):
    if depth>40: return False
    if n['flags'] not in (0,0x2000,0x8000,0xa000,0x4000): return False
    if n['type'] not in TYPES.values() and n['value'] is not None: return False
    if isinstance(n.get('value'),float) and not (abs(n['value'])<1e9): return False
    return all(valid(c,depth+1) for c in n.get('children',[]))
d=open(sys.argv[1],'rb').read(); minlen=int(sys.argv[2]) if len(sys.argv)>2 else 200
o=0; out=open(sys.argv[3],'w') if len(sys.argv)>3 else sys.stdout
while o < len(d)-12:
    a,b,c,t,n = struct.unpack_from('<HHHHI',d,o)
    if t==6 and 0<n<5000 and c in (0,0x2000,0x8000):
        try:
            node,e = parse(d,o)
            if e-o>=minlen and valid(node):
                print('=== Offset 0x%x .. 0x%x (%d Bytes)'%(o,e,e-o),file=out); dump(node,0,out); o=e; continue
        except Exception: pass
    o+=1
