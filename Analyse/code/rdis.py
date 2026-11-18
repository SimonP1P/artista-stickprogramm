"""rdis.py – kleiner Disassembler für 32-Bit-Windows-DLLs/EXEs (siehe 06_disassembler_anleitung.md)"""
import pefile, capstone, sys, struct, re
class Bin:
    def __init__(s, path):
        s.pe=pefile.PE(path); s.base=s.pe.OPTIONAL_HEADER.ImageBase; s.img=s.pe.get_memory_mapped_image()
        s.imps={}
        if hasattr(s.pe,'DIRECTORY_ENTRY_IMPORT'):
            for d in s.pe.DIRECTORY_ENTRY_IMPORT:
                for i in d.imports:
                    s.imps[i.address]=d.dll.decode()+'!'+(i.name.decode() if i.name else '#%d'%i.ordinal)
        s.exps={}
        if hasattr(s.pe,'DIRECTORY_ENTRY_EXPORT'):
            for e in s.pe.DIRECTORY_ENTRY_EXPORT.symbols:
                s.exps[s.base+e.address]=(e.name.decode() if e.name else '#%d'%e.ordinal)
        s.md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_32); s.md.detail=False
    def rd(s,va,n): o=va-s.base; return s.img[o:o+n] if 0<=o<len(s.img) else b''
    def str_at(s,va):
        b=s.rd(va,300)
        m=re.match(rb'[\x20-\x7e\t\r\n]{3,}',b)
        if m and (len(m.group())==len(b) or b[len(m.group())]==0): return repr(m.group().decode())
        m=re.match(rb'(?:[\x20-\x7e]\x00){3,}',b)
        if m: return 'L'+repr(m.group().decode('utf-16le'))
        return None
    def ann(s,ins):
        out=[]
        for m in re.finditer(r'0x([0-9a-f]{5,8})',ins.op_str):
            v=int(m.group(1),16)
            if v in s.imps: out.append(s.imps[v])
            elif v in s.exps: out.append('EXP:'+s.exps[v])
            else:
                st=s.str_at(v)
                if st: out.append(st)
                elif s.base<=v<s.base+len(s.img) and ins.mnemonic in('fld','fmul','fdiv','fadd','fsub','fcomp','fcom','fstp','fild') :
                    b=s.rd(v,8)
                    if 'qword' in ins.op_str: out.append('dbl=%g'%struct.unpack('<d',b)[0])
                    elif 'dword' in ins.op_str: out.append('flt=%g'%struct.unpack('<f',b[:4])[0])
                elif ins.mnemonic=='call' and s.base<=v<s.base+len(s.img):
                    b=s.rd(v,6)
                    if b[:2]==b'\xff\x25':
                        t=struct.unpack('<I',b[2:6])[0]; out.append('->'+s.imps.get(t,'?'))
        return ('   ; '+' | '.join(out)) if out else ''
    def dis(s,va,n=400,stop_ret=True):
        code=s.rd(va,n*8); lines=[]
        for ins in s.md.disasm(code,va):
            lines.append('%08x  %-7s %s%s'%(ins.address,ins.mnemonic,ins.op_str,s.ann(ins)))
            if len(lines)>=n or (stop_ret and ins.mnemonic=='ret' and len(lines)>3): break
        return '\n'.join(lines)
    def find_refs(s,va):
        pat=struct.pack('<I',va); res=[]
        for sec in s.pe.sections:
            if sec.Characteristics & 0x20000000:
                d=sec.get_data(); st=0
                while True:
                    i=d.find(pat,st)
                    if i<0: break
                    res.append(s.base+sec.VirtualAddress+i); st=i+1
        return res
    def find_str(s,txt,wide=False):
        pat=txt.encode('utf-16le') if wide else txt.encode()
        res=[]; st=0
        while True:
            i=s.img.find(pat,st)
            if i<0: break
            res.append(s.base+i); st=i+1
        return res
    def func_start(s,va):
        # search backwards for typical prologue
        for back in range(0,0x3000):
            a=va-back; b=s.rd(a,3)
            if b[:3] in (b'\x55\x8b\xec',) or (s.rd(a-1,1) in (b'\xcc',b'\xc3',b'\x90') and b[:1] in (b'\x83',b'\x81',b'\x6a',b'\x64',b'\x53',b'\x56',b'\x55',b'\x8b',b'\xa1',b'\xb8')):
                if s.rd(a-1,1) in (b'\xcc',b'\xc3',b'\x90') or b[:3]==b'\x55\x8b\xec': return a
        return None
if __name__=='__main__':
    b=Bin(sys.argv[1]); cmd=sys.argv[2]
    if cmd=='exports':
        for k,v in sorted(b.exps.items()): print(hex(k),v)
    elif cmd=='dis': print(b.dis(int(sys.argv[3],16), int(sys.argv[4]) if len(sys.argv)>4 else 400, stop_ret=(len(sys.argv)<=5)))
    elif cmd=='xstr':
        w = len(sys.argv)>4
        for a in b.find_str(sys.argv[3], w):
            print('str at',hex(a)); 
            for r in b.find_refs(a):
                print('   ref at',hex(r),'func~',hex(b.func_start(r) or 0))
    elif cmd=='full':
        # komplette Disassembly aller ausführbaren Abschnitte, eine Instruktion pro Zeile
        for sec in b.pe.sections:
            if not sec.Characteristics & 0x20000000: continue
            va=b.base+sec.VirtualAddress
            for ins in b.md.disasm(sec.get_data(), va):
                print('%08x  %-7s %s%s'%(ins.address,ins.mnemonic,ins.op_str,b.ann(ins)))
    elif cmd=='xref':
        for r in b.find_refs(int(sys.argv[3],16)): print(hex(r),'func~',hex(b.func_start(r) or 0))
