10004de0  call    0x100074d0
10004de5  test    al, al
10004de7  je      0x10004e0e
10004de9  mov     al, byte ptr [0x100670c8]
10004dee  push    1
10004df0  neg     al
10004df2  sbb     eax, eax
10004df4  push    0xfa0
10004df9  and     eax, 0x50
10004dfc  push    0x3e8
10004e01  add     eax, 0x50
10004e04  push    eax
10004e05  call    0x10002f00
10004e0a  add     esp, 0x10
10004e0d  ret     
10004e0e  mov     ax, 1
10004e12  ret     
