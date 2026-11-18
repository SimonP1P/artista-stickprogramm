import sys, zlib, struct
def decrypt(d):
    n=len(d); key=((((n<<12)+n)<<12)+n) & 0xffffffff
    if key & 0x80000000: key -= 1<<32   # sar = arithmetic shift
    return bytes(b ^ ((key >> (4*(i&3))) & 0xff) for i,b in enumerate(d))
if __name__=='__main__':
    d=open(sys.argv[1],'rb').read()
    x=decrypt(d); size=struct.unpack_from('<I',x,0)[0]
    out=zlib.decompress(x[4:]); print('declared',size,'got',len(out), file=sys.stderr)
    open(sys.argv[2],'wb').write(out)
