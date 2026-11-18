10006dd0  call    0x10007500
10006dd5  test    al, al
10006dd7  jne     0x10006e3e
10006dd9  mov     al, byte ptr [0x100426b0]
10006dde  test    al, al
10006de0  jne     0x10006e3e
10006de2  mov     eax, dword ptr [0x10068e30]
10006de7  test    eax, eax
10006de9  jne     0x10006e01
10006deb  push    0
10006ded  push    0x3e8
10006df2  push    0x3e8
10006df7  push    0x40
10006df9  call    0x10002f00
10006dfe  add     esp, 0x10
10006e01  mov     eax, dword ptr [esp + 4]
10006e05  push    eax
10006e06  call    0x10004c90
10006e0b  mov     eax, dword ptr [0x10068e30]
10006e10  add     esp, 4
10006e13  test    eax, eax
10006e15  jne     0x10006e30
10006e17  push    0
10006e19  push    0xfa0
10006e1e  push    0x1388
10006e23  push    0x800
10006e28  call    0x10002f00
10006e2d  add     esp, 0x10
10006e30  mov     ecx, dword ptr [0x10068e30]
10006e36  xor     eax, eax
10006e38  test    ecx, ecx
10006e3a  sete    al
10006e3d  ret     
10006e3e  xor     ax, ax
10006e41  ret     
