1000ab70  push    ebx
1000ab71  mov     ebx, ecx
1000ab73  push    esi
1000ab74  push    edi
1000ab75  mov     edi, 0x48
1000ab7a  lea     esi, [ebx + 0x54]
1000ab7d  push    edi
1000ab7e  mov     ecx, esi
1000ab80  call    0x1000a8b0
1000ab85  cmp     al, 0xff
1000ab87  jne     0x1000ab9d
1000ab89  cmp     edi, dword ptr [ebx]
1000ab8b  jl      0x1000ab90
1000ab8d  mov     byte ptr [esi], 0
1000ab90  inc     edi
1000ab91  mov     ecx, esi
1000ab93  push    edi
1000ab94  call    0x1000a8b0
1000ab99  cmp     al, 0xff
1000ab9b  je      0x1000ab89
1000ab9d  push    edi
1000ab9e  mov     ecx, esi
1000aba0  call    0x1000a8b0
1000aba5  and     eax, 0xff
1000abaa  xor     ecx, ecx
1000abac  test    al, 1
1000abae  je      0x1000abb8
1000abb0  inc     ecx
1000abb1  sar     eax, 1
1000abb3  cmp     ecx, 8
1000abb6  jl      0x1000abac
1000abb8  lea     eax, [ecx + edi*8 - 0x240]
1000abbf  mov     ecx, dword ptr [ebx + 4]
1000abc2  shl     eax, 8
1000abc5  pop     edi
1000abc6  pop     esi
1000abc7  add     eax, ecx
1000abc9  pop     ebx
1000abca  ret     
