"""Ordnet Wilcom-Eigenschafts-Tags (DataItem-IDs) Dialogbeschriftungen zu:
DIALOG.DLL: Member-Offset <-> Control-ID (Subclass/DDX) und Member-Offset <-> Tag (Bindungsaufruf);
RESENU.DLL: Control-ID -> Beschriftung (STATIC mit id-1 oder Text des Buttons)."""
import re, collections
asm=[l.split(None,2) for l in open('text/dialog_dll.asm')]
asm=[(int(a,16),m,(o[0] if o else '')) for a,m,*o in [(x[0],x[1],*x[2:]) for x in asm if len(x)>=2]]
ctrl=collections.defaultdict(list)  # (addr, member)->ctrlids
bind=[]                              # (addr, member, tag)
for i,(a,m,o) in enumerate(asm):
    mm=re.match(r'(?:ecx|edx|eax|ebx|edi|esi|ebp), \[(?:esi|edi|ebx|ebp) \+ (0x[0-9a-f]+)\]',o)
    if m=='lea' and mm:
        mem=int(mm.group(1),16)
        prev=[asm[j] for j in range(max(0,i-4),i)]
        ids=[int(p[2],16) for p in prev if p[1]=='push' and re.match(r'^0x[0-9a-f]+$',p[2]) and 0x3e8<=int(p[2],16)<0x1000]
        nxt=[asm[j] for j in range(i+1,min(len(asm),i+6))]
        tags=[int(p[2],16) for p in nxt if p[1]=='push' and re.match(r'^0x[0-9a-f]+$',p[2]) and 0x2700<=int(p[2],16)<0x4100]
        if ids and any(n[1]=='call' for n in nxt): ctrl[mem].append((a,ids))
        # Bindung: tag vor lea, call danach
        if tags and any(n[1]=='call' for n in nxt): bind.append((a,mem,tags[-1]))
# Labels aus RESENU
lab={}; dlg=None; cur={}
for l in open('res/RESENU.DLL.txt'):
    m=re.match(r'DIALOG "(.*)"',l)
    if m: dlg=m.group(1); continue
    m=re.match(r'\s+\[(\w+|\{.*\}) id=(\d+)\] "(.*)" @',l)
    if m:
        cls,i,t=m.group(1),int(m.group(2)),m.group(3)
        if t and cls in('STATIC','BUTTON'): lab.setdefault(i,collections.Counter())[(dlg,t)]+=1
def label(cid):
    out=[]
    for c in (cid-1,cid,cid-2):
        if c in lab: out+= ['%s: %s'%(d,t) for (d,t),_ in lab[c].most_common(2)]
        if out: break
    return out
res=collections.defaultdict(collections.Counter)
for a,mem,tag in bind:
    cands=[(abs(ca-a),ids) for ca,ids in ctrl.get(mem,[]) if abs(ca-a)<0x1500]
    if cands:
        ids=min(cands)[1]
        for cid in ids[:2]:
            for L in label(cid): res[tag][L]+=1
for tag in sorted(res):
    print('0x%04x'%tag, ' | '.join('%s (%d)'%(k,v) for k,v in res[tag].most_common(4)))
