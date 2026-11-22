"""Klassen der Effekte in IMPORT.DLL finden: Vtabellen mit CreateData-Thunks → Klassenname (Analyse_Teil3/23 §1).
Aufruf: python import_klassen.py IMPORT.DLL KLASSE [KLASSE …] (hex, z. B. 0x32 0x34)"""
import sys, struct, re, pefile, capstone
pe = pefile.PE(sys.argv[1]); base = pe.OPTIONAL_HEADER.ImageBase
iat = {}
for e in pe.DIRECTORY_ENTRY_IMPORT:
    for i in e.imports:
        iat[i.address] = i.name.decode() if i.name else str(i.ordinal)
img = pe.get_memory_mapped_image()
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
def u32(va): return struct.unpack_from('<I', img, va - base)[0]
def ins_at(va, n=300):
    return list(md.disasm(img[va - base: va - base + n], va))
def thunk(va):
    try:
        i = ins_at(va, 8)[0]
    except IndexError: return None
    m = re.match(r'dword ptr \[(0x[0-9a-f]+)\]', i.op_str)
    if i.mnemonic == 'jmp' and m and int(m.group(1), 16) in iat: return iat[int(m.group(1), 16)]
    return None
FAB = 0x10191e48
text = next(s for s in pe.sections if s.Name.startswith(b'.text')); code = text.get_data()
fab = {}
for i in range(len(code) - 10):
    if code[i] == 0xC7 and code[i+1] == 0x05:
        adr, imm = struct.unpack_from('<II', code, i + 2)
        if FAB <= adr < FAB + 4 * 0xa0 and (adr - FAB) % 4 == 0: fab[(adr - FAB) // 4] = imm
def vtables(fn, tiefe=0):
    out = []
    for ins in ins_at(fn, 500):
        m = re.match(r'dword ptr \[e[a-z]x\], (0x[0-9a-f]+)$', ins.op_str)
        if ins.mnemonic == 'mov' and m: out.append(int(m.group(1), 16))
        if ins.mnemonic == 'call' and tiefe < 2 and not ins.op_str.startswith('dword'):
            try: out += vtables(int(ins.op_str, 16), tiefe + 1)
            except ValueError: pass
        if ins.mnemonic == 'ret': break
    return out
for k in [int(x, 16) for x in sys.argv[2:]]:
    if k not in fab: print(hex(k), 'keine Fabrik'); continue
    res = []
    for vt in vtables(fab[k]):
        nm = []
        for j in range(14):
            try: e = u32(vt + 4 * j)
            except struct.error: break
            t = thunk(e)
            nm.append(('%s:%s' % (t.split('@')[0].lstrip('?'), t.split('@')[1])) if t and '@' in t else '-')
        res.append((hex(vt), nm))
    print(hex(k), res[-1:])
