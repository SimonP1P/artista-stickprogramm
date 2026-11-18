10004d10  call    0x10004de0
10004d15  mov     eax, dword ptr [esp + 8]
10004d19  test    eax, eax
10004d1b  jl      0x10004d5a
10004d1d  cmp     eax, 0x1b
10004d20  push    esi
10004d21  jge     0x10004d2a
10004d23  mov     esi, 0x30
10004d28  jmp     0x10004d3e
10004d2a  xor     ecx, ecx
10004d2c  cmp     eax, 0x36
10004d2f  setge   cl
10004d32  dec     ecx
10004d33  and     ecx, 0xffffffa0
10004d36  add     ecx, 0xc0
10004d3c  mov     esi, ecx
10004d3e  push    eax
10004d3f  call    0x10004c90
10004d44  push    1
10004d46  push    0xfa0
10004d4b  push    0x3e8
10004d50  push    esi
10004d51  call    0x10002f00
10004d56  add     esp, 0x14
10004d59  pop     esi
10004d5a  mov     eax, dword ptr [0x10068e30]
10004d5f  test    eax, eax
10004d61  jne     0x10004dc3
10004d63  mov     edx, dword ptr [esp + 4]
10004d67  push    edx
10004d68  push    0x7d0
10004d6d  push    0x3e8
10004d72  push    0x20
10004d74  call    0x10002f00
10004d79  mov     eax, dword ptr [0x10068e30]
10004d7e  add     esp, 0x10
10004d81  test    eax, eax
10004d83  jne     0x10004dc3
10004d85  push    1
10004d87  push    0x10068d69
10004d8c  push    0x1007202e
10004d91  call    0x10003dd0
10004d96  add     esp, 4
10004d99  push    eax
10004d9a  call    0x100029b0   ; EXP:ReadComBytes
10004d9f  mov     eax, dword ptr [0x10068e30]
10004da4  add     esp, 0xc
10004da7  test    eax, eax
10004da9  jne     0x10004dc3
10004dab  xor     eax, eax
10004dad  mov     al, byte ptr [0x10068d69]
10004db2  cmp     eax, 0x1b00
10004db7  jbe     0x10004dc3
10004db9  push    0x13
10004dbb  call    0x10001120
10004dc0  add     esp, 4
10004dc3  mov     ecx, dword ptr [0x10068e30]
10004dc9  xor     eax, eax
10004dcb  test    ecx, ecx
10004dcd  sete    al
10004dd0  ret     
