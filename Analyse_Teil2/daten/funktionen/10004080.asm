10004080  sub     esp, 0xc
10004083  lea     eax, [esp]
10004087  push    4
10004089  push    eax
1000408a  push    0x24004
1000408f  call    0x100029b0   ; EXP:ReadComBytes
10004094  add     esp, 0xc
10004097  test    eax, eax
10004099  je      0x100040c5
1000409b  lea     ecx, [esp]
1000409f  push    ecx
100040a0  call    0x10003dd0
100040a5  lea     edx, [esp + 4]
100040a9  add     eax, 7
100040ac  push    1
100040ae  push    edx
100040af  push    eax
100040b0  call    0x100029b0   ; EXP:ReadComBytes
100040b5  add     esp, 0x10
100040b8  test    eax, eax
100040ba  je      0x100040c5
100040bc  mov     al, byte ptr [esp]
100040c0  mov     byte ptr [0x1006ae84], al
100040c5  add     esp, 0xc
100040c8  ret     
