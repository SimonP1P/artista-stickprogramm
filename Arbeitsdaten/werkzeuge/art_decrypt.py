"""Entschlüsselung der Wilcom/BERNINA OLE-Streams (Contents, DesignDocument) in ART/AMT/ESB.
Rekonstruiert aus IMPORT.DLL (0x10114c00, 0x10114a90, 0x10114b30, 0x10114b80) und PRODUCT.DLL (0x10001040..)."""
import struct, zlib
FORMAT_KEYS = {0xd0:0x01c0d932, 0xd1:0x0005b805, 0xcd:0x02431a01, 0xca:0x058fab49, 0xd2:0x002470e8, 0x12c:0x05873c7a}
def rotl1(b): return ((b<<1)|(b>>7)) & 0xff
def derive_key(format_key, extra=0, mode=0):
    """IMPORT.DLL 0x10114c00"""
    s = (format_key + extra) & 0xffffffff
    sb = s.to_bytes(4,'little'); h = (sum(sb)*13) & 0xff
    mb = mode.to_bytes(4,'little'); out=[0]*4
    for k in range(4):
        out[3-k] = rotl1(mb[k]) ^ ((sb[3-k] + 17*k + h) & 0xff)
    return int.from_bytes(bytes(out),'little')
def f(i, key):
    """IMPORT.DLL 0x10114b30: Permutationsfunktion"""
    if key & 0x80000000: key -= 1<<32
    x = ((key >> 10) & 0xff) ^ (i & 0xff) ^ ((key >> 2) & 0xff)
    sh = 8 - (key & 3)
    x <<= sh
    return (x | (x >> 8)) & 0xff
def tables(key):
    t=[0]*256
    for i in range(256): t[f(i,key)] = i      # Init: table[f(i)] = i
    inv=[0]*256
    for j,v in enumerate(t): inv[v]=j          # memchr-Suche = Umkehrung
    return t, inv
def decode(data, key, variant):
    t,inv = tables(key); m = inv if variant else t
    return bytes(m[b] for b in data)
def try_all(data):
    for fid,fk in FORMAT_KEYS.items():
        k=derive_key(fk)
        for v in (0,1):
            d=decode(data,k,v)
            try:
                out=zlib.decompress(d[4:])
                if struct.unpack_from('<I',d,0)[0]==len(out): return fid,v,out
            except Exception: pass
    return None
if __name__=='__main__':
    import sys
    r=try_all(open(sys.argv[1],'rb').read())
    if r: print('format-id 0x%x variant %d -> %d bytes'%(r[0],r[1],len(r[2]))); open(sys.argv[2],'wb').write(r[2])
    else: print('kein Schlüssel passt')
