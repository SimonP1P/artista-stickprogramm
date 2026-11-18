import struct, sys, os, subprocess
src, out, blast = sys.argv[1], sys.argv[2], sys.argv[3]
f = open(src,'rb').read()
dirofs = struct.unpack_from('<I',f,0x29)[0]; ndir = struct.unpack_from('<H',f,0x31)[0]
fileofs = struct.unpack_from('<I',f,0x33)[0]; nfiles = struct.unpack_from('<H',f,0x0C)[0]
dirs=[]; o=dirofs
for i in range(ndir):
    cnt,esz,nl = struct.unpack_from('<HHH',f,o); dirs.append(f[o+6:o+6+nl].decode('latin1')); o+=esz
o=fileofs; bad=0
for i in range(nfiles):
    di,size,csize,off = struct.unpack_from('<HIII',f,o+1); esz=struct.unpack_from('<H',f,o+0x17)[0]
    nl=f[o+0x1D]; name=f[o+0x1E:o+0x1E+nl].decode('latin1'); o+=esz
    p=os.path.join(out,dirs[di].replace('\\','/'),name); os.makedirs(os.path.dirname(p),exist_ok=True)
    r=subprocess.run([blast],input=f[off:off+csize],capture_output=True)
    open(p,'wb').write(r.stdout)
    if len(r.stdout)!=size: bad+=1; print('SIZE MISMATCH',p,len(r.stdout),size,r.stderr)
print('files',nfiles,'bad',bad)
