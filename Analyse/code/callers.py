import sys, struct; sys.path.insert(0, sys.path[0]); from rdis import Bin
b=Bin(sys.argv[1]); tgt=int(sys.argv[2],16)
txt=[s for s in b.pe.sections if s.Characteristics & 0x20000000][0]
d=txt.get_data(); base=b.base+txt.VirtualAddress
for i in range(len(d)-5):
    if d[i] in (0xe8,0xe9):
        rel=struct.unpack_from('<i',d,i+1)[0]
        if base+i+5+rel==tgt: print(hex(base+i), 'call' if d[i]==0xe8 else 'jmp', 'func~',hex(b.func_start(base+i) or 0))
