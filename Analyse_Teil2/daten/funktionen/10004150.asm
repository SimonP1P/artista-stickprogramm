10004150  sub     esp, 0x20
10004153  lea     eax, [esp]
10004157  push    esi
10004158  push    4
1000415a  push    eax
1000415b  push    0x24004
10004160  xor     esi, esi
10004162  call    0x100029b0   ; EXP:ReadComBytes
10004167  add     esp, 0xc
1000416a  test    eax, eax
1000416c  setne   al
1000416f  test    al, al
10004171  je      0x100041cb
10004173  lea     ecx, [esp + 4]
10004177  push    ecx
10004178  call    0x10003dd0
1000417d  lea     edx, [esp + 8]
10004181  add     eax, 5
10004184  push    1
10004186  push    edx
10004187  push    eax
10004188  call    0x100029b0   ; EXP:ReadComBytes
1000418d  add     esp, 0x10
10004190  test    eax, eax
10004192  setne   al
10004195  test    al, al
10004197  je      0x100041cb
10004199  test    byte ptr [esp + 4], 2
1000419e  je      0x100041cb
100041a0  mov     esi, 1
100041a5  call    0x100074d0
100041aa  test    al, al
100041ac  je      0x100041cb
100041ae  lea     eax, [esp + 4]
100041b2  push    eax
100041b3  call    0x100040d0
100041b8  add     esp, 4
100041bb  test    al, al
100041bd  je      0x100041cb
100041bf  test    byte ptr [esp + 0xb], 1
100041c4  mov     eax, 2
100041c9  jne     0x100041cd
100041cb  mov     eax, esi
100041cd  pop     esi
100041ce  add     esp, 0x20
100041d1  ret     
