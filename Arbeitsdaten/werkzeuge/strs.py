import re,sys,os
for p in sys.argv[2:]:
    d=open(p,'rb').read(); out=os.path.join(sys.argv[1],os.path.basename(p)+'.txt')
    with open(out,'w') as o:
        for m in re.finditer(rb'[\x20-\x7e\t]{5,}',d): o.write('A %08x %s\n'%(m.start(),m.group().decode()))
        for m in re.finditer(rb'(?:[\x20-\x7e\xa0-\xff]\x00){4,}',d): o.write('U %08x %s\n'%(m.start(),m.group().decode('utf-16le')))
