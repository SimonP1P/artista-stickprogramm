100040d0  push    ebx
100040d1  push    esi
100040d2  mov     bl, 1
100040d4  call    0x100074d0
100040d9  test    al, al
100040db  je      0x10004114
100040dd  mov     esi, dword ptr [esp + 0xc]
100040e1  push    4
100040e3  push    esi
100040e4  push    0x24040
100040e9  call    0x100029b0   ; EXP:ReadComBytes
100040ee  add     esp, 0xc
100040f1  test    eax, eax
100040f3  setne   al
100040f6  test    al, al
100040f8  je      0x10004116
100040fa  push    esi
100040fb  call    0x10003dd0
10004100  push    0x20
10004102  push    esi
10004103  push    eax
10004104  call    0x100029b0   ; EXP:ReadComBytes
10004109  add     esp, 0x10
1000410c  test    eax, eax
1000410e  pop     esi
1000410f  pop     ebx
10004110  setne   al
10004113  ret     
10004114  mov     al, bl
10004116  pop     esi
10004117  pop     ebx
10004118  ret     
