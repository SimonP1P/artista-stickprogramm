import pefile, struct, sys, os
def u16z(d,o):
    e=o
    while d[e:e+2]!=b'\0\0': e+=2
    return d[o:e].decode('utf-16le','replace'), e+2
def sz_or_ord(d,o):
    if d[o:o+2]==b'\xff\xff': return '#%d'%struct.unpack_from('<H',d,o+2)[0], o+4
    return u16z(d,o)
def dialog(d):
    out=[]
    ex = struct.unpack_from('<HH',d,0)==(1,0xFFFF)
    if ex:
        helpid,exstyle,style,n,x,y,cx,cy=struct.unpack_from('<IIIHhhhh',d,4); o=26
    else:
        style,exstyle,n,x,y,cx,cy=struct.unpack_from('<IIHhhhh',d,0); o=18
    menu,o=sz_or_ord(d,o); cls,o=sz_or_ord(d,o); title,o=u16z(d,o)
    if style & 0x40:
        o+= 2 + (4 if ex else 0); _,o=u16z(d,o)
    out.append('DIALOG "%s" %dx%d'%(title,cx,cy))
    for i in range(n):
        o=(o+3)&~3
        if ex:
            hid,exs,st,x,y,cx,cy,cid=struct.unpack_from('<IIIhhhhI',d,o); o+=24
        else:
            st,exs,x,y,cx,cy,cid=struct.unpack_from('<IIhhhhH',d,o); o+=18
        c,o=sz_or_ord(d,o); t,o=sz_or_ord(d,o)
        ex_n=struct.unpack_from('<H',d,o)[0]; o+=2+ex_n
        cmap={'#128':'BUTTON','#129':'EDIT','#130':'STATIC','#131':'LISTBOX','#132':'SCROLL','#133':'COMBO'}
        out.append('   [%s id=%d] "%s" @%d,%d %dx%d'%(cmap.get(c,c),cid,t,x,y,cx,cy))
    return out
p=pefile.PE(sys.argv[1]); d=p.__data__
for t in p.DIRECTORY_ENTRY_RESOURCE.entries:
    tid=t.id
    for e in t.directory.entries:
        for l in e.directory.entries:
            r=l.data.struct; data=p.get_data(r.OffsetToData,r.Size)
            name=e.id if e.name is None else str(e.name)
            if tid==6:
                o=0
                for i in range(16):
                    ln=struct.unpack_from('<H',data,o)[0]; o+=2
                    if ln: print('STR %d: %s'%((name-1)*16+i, data[o:o+2*ln].decode('utf-16le').replace('\n','\\n').replace('\r','')))
                    o+=2*ln
            elif tid==5:
                try:
                    print('--- DLG %s'%name); print('\n'.join(dialog(data)))
                except Exception as ex: print('dlg err',ex)
            elif tid==4:
                print('--- MENU %s (%d bytes)'%(name,len(data)))
                s=data.decode('utf-16le','replace'); print('   '+' | '.join(x for x in s.split('\0') if len(x)>1))
            elif tid not in (3,14,2,1,12,24,16):
                print('--- RES type=%s name=%s len=%d'%(tid,name,len(data)))
