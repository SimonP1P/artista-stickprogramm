10006ea0  cmp     word ptr [esp + 8], 0
10006ea6  jne     0x10006f04
10006ea8  push    esi
10006ea9  mov     esi, dword ptr [esp + 8]
10006ead  push    esi
10006eae  push    0
10006eb0  call    0x10004d10
10006eb5  mov     eax, dword ptr [0x10068e30]
10006eba  add     esp, 8
10006ebd  test    eax, eax
10006ebf  jne     0x10006ed6
10006ec1  xor     eax, eax
10006ec3  mov     al, byte ptr [0x10068d69]
10006ec8  cmp     esi, eax
10006eca  jl      0x10006ed6
10006ecc  push    0x15
10006ece  call    0x10001120
10006ed3  add     esp, 4
10006ed6  push    esi
10006ed7  call    0x10004c90
10006edc  mov     eax, dword ptr [0x10068e30]
10006ee1  add     esp, 4
10006ee4  test    eax, eax
10006ee6  pop     esi
10006ee7  jne     0x10006f49
10006ee9  push    0
10006eeb  push    0xfa1
10006ef0  push    0xbb8
10006ef5  push    0x400
10006efa  call    0x10002f00
10006eff  add     esp, 0x10
10006f02  jmp     0x10006f2a
10006f04  mov     al, byte ptr [0x100426b0]
10006f09  test    al, al
10006f0b  je      0x10006f19
10006f0d  push    2
10006f0f  call    0x10001120
10006f14  add     esp, 4
10006f17  jmp     0x10006f2a
10006f19  mov     ecx, dword ptr [esp + 4]
10006f1d  push    1
10006f1f  push    0
10006f21  push    ecx
10006f22  call    0x100032b0
10006f27  add     esp, 0xc
10006f2a  mov     eax, dword ptr [0x10068e30]
10006f2f  test    eax, eax
10006f31  jne     0x10006f49
10006f33  push    0
10006f35  push    0x3e8
10006f3a  push    0x3e8
10006f3f  push    8
10006f41  call    0x10002f00
10006f46  add     esp, 0x10
10006f49  push    0
10006f4b  call    0x100061b0   ; EXP:TransparentMode
10006f50  mov     ecx, dword ptr [0x10068e30]
10006f56  add     esp, 4
10006f59  xor     eax, eax
10006f5b  test    ecx, ecx
10006f5d  sete    al
10006f60  ret     
