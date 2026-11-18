"""Parser für hooplibr.dsc (Rahmenbibliothek). Gestrichelte Sektion (PEN 0,DOTTED) = Stickfeld,
durchgezogene Sektionen (PEN 1,SOLID) = Rahmenkontur. Koordinaten in mm, Ursprung = Rahmenmitte, y nach oben."""
import re,sys,math
def nums(s):
    """Zahlen aus "(x, y)". Die Originaldatei enthält Tippfehler wie "(-87.18, -101,23)" (Dezimalkomma)
    und "(79.6 -72.2)" (fehlendes Komma) – beides wird toleriert."""
    t=re.findall(r'-?\d+(?:\.\d+)?', s)
    if len(t)==3:   # ein Komma war Dezimalkomma: das ohne Leerzeichen zwischen zwei Ziffern
        m=[k.start() for k in re.finditer(r'\d,\d', s)]
        if m and m[-1] > s.find(t[1]): t=[t[0], t[1]+'.'+t[2]]
        else: t=[t[0]+'.'+t[1], t[2]]
    return [float(x) for x in t]
def head(s):
    return [float(x) for x in re.findall(r'\d+(?:\.\d+)?', s)]
def parse(path):
    hoops=[]; cur=None; pen=None
    for line in open(path,encoding='latin1'):
        line=line.strip()
        m=re.match(r'HOOP\s+(\w+)\s+\(([^)]*)\)\s*(.*)',line)
        if m:
            w,h=head(m.group(2))[:2]; cur={'typ':m.group(1),'breite':w,'hoehe':h,'name':m.group(3),'feld':[],'rahmen':[],'arcs':0}; hoops.append(cur); continue
        m=re.match(r'SECTION PEN \((\d)',line)
        if m: pen=int(m.group(1)); continue
        if cur and (line.startswith('LINE') or line.startswith('ARC')):
            pts=re.findall(r'\(([^)]*)\)',line); pts=[tuple(nums(p)[:2]) for p in pts]; pts=[q for q in pts if len(q)==2]
            (cur['feld'] if pen==0 else cur['rahmen']).extend(pts)
            if line.startswith('ARC') and pen==0: cur['arcs']+=1
    return hoops
def bbox(p):
    if not p: return None
    xs=[a for a,b in p]; ys=[b for a,b in p]; return (min(xs),max(xs),min(ys),max(ys))
if __name__=='__main__':
    print('| Name | Kopf (B×H) | Stickfeld X | Stickfeld Y | Form | Rahmen-Außenmaß ca. |')
    print('|---|---|---|---|---|---|')
    for h in parse(sys.argv[1]):
        f=bbox(h['feld']); r=bbox(h['rahmen'])
        form='oval/abgerundet (%d Bögen)'%h['arcs'] if h['arcs'] else 'rechteckig'
        fs='%.1f…%.1f'%(f[0],f[1]) if f else '-'; fy='%.1f…%.1f'%(f[2],f[3]) if f else '-'
        rs='%.0f × %.0f'%(r[1]-r[0],r[3]-r[2]) if r else '-'
        print('| %s | %g × %g | %s | %s | %s | %s |'%(h['name'],h['breite'],h['hoehe'],fs,fy,form,rs))
