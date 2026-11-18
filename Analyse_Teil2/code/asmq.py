"""Abfragen auf einer vollständigen Disassembly-Datei (Ausgabe von rdis-Komplettlauf).
  fn  DATEI ADR        -> Funktion ab ADR bis zum Ende (ret + Füllbytes)
  w   DATEI VAR        -> alle Instruktionen, die die globale Variable VAR beschreiben (mov/inc/or/and ... [VAR])
  r   DATEI VAR        -> alle Zugriffe auf VAR
  fs  DATEI ADR        -> Funktionsanfang zu einer Adresse (letzte Stelle nach Füllbytes)"""
import sys,re
mode,path=sys.argv[1],sys.argv[2]
L=[l.rstrip('\n') for l in open(path)]
A=[int(l[:8],16) for l in L]
def starts():
    s=set([A[0]])
    for i in range(1,len(L)):
        if re.search(r'\s(nop|int3)\s*$',L[i-1]) and not re.search(r'\s(nop|int3)\s*$',L[i]): s.add(A[i])
        if re.search(r'\sret\b',L[i-1]) and not re.search(r'\s(nop|int3)',L[i]) : s.add(A[i])
    return sorted(s)
if mode=='fn':
    a=int(sys.argv[3],16); i=A.index(a); out=[]
    while i<len(L):
        if re.search(r'\s(nop|int3)\s*$',L[i]):
            # Füllbytes: Ende, außer es kommt noch ein Sprungziel in der Funktion
            j=i
            while j<len(L) and re.search(r'\s(nop|int3)\s*$',L[j]): j+=1
            tg=[int(x,16) for o in out for x in re.findall(r'\b(?:j\w+|call)\s+0x([0-9a-f]+)',o) if 'j' in o.split()[1]]
            if j<len(L) and any(A[j]<=t<A[j]+0x40 for t in tg): i=j; continue
            break
        out.append(L[i]); i+=1
    print('\n'.join(out))
elif mode in('w','r'):
    v=sys.argv[3].lower().replace('0x','')
    for l in L:
        if '0x'+v+']' in l:
            ins=l[10:].split()
            if mode=='r' or (ins and ins[0] in('mov','inc','dec','or','and','add','sub','xor') and ('ptr [0x'+v+'], ' in l)): print(l)
elif mode=='fs':
    a=int(sys.argv[3],16); s=[x for x in starts() if x<=a]; print(hex(s[-1]))
