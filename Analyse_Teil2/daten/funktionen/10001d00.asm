10001d00  sub     esp, 0xc
10001d03  mov     eax, dword ptr [0x100400d8]
10001d08  push    ebx
10001d09  push    ebp
10001d0a  push    esi
10001d0b  mov     dword ptr [esp + 0x14], eax
10001d0f  mov     ecx, eax
10001d11  mov     eax, dword ptr [0x10068e30]
10001d16  xor     esi, esi
10001d18  cmp     eax, esi
10001d1a  push    edi
10001d1b  mov     dword ptr [esp + 0x10], ecx
10001d1f  je      0x10001f33
10001d25  mov     edi, dword ptr [0x10035284]   ; KERNEL32.dll!Sleep
10001d2b  mov     dword ptr [0x10068e30], esi
10001d31  mov     dword ptr [esp + 0x14], esi
10001d35  jmp     0x10001d3f
10001d37  mov     ecx, dword ptr [esp + 0x10]
10001d3b  mov     esi, dword ptr [esp + 0x14]
10001d3f  cmp     esi, 0x30
10001d42  jge     0x10001f08
10001d48  mov     eax, esi
10001d4a  mov     ebx, 6
10001d4f  cdq     
10001d50  idiv    ebx
10001d52  mov     ebx, edx
10001d54  cmp     ebx, 5
10001d57  jne     0x10001dae
10001d59  cmp     ecx, 0x12c0
10001d5f  je      0x10001d8f
10001d61  cmp     ecx, 0x4b00
10001d67  je      0x10001d85
10001d69  sub     ecx, 0xe100
10001d6f  neg     ecx
10001d71  sbb     ecx, ecx
10001d73  and     ecx, 0xfffe50c0
10001d79  add     ecx, 0x1c200
10001d7f  mov     dword ptr [esp + 0x10], ecx
10001d83  jmp     0x10001d97
10001d85  mov     dword ptr [esp + 0x10], 0xe100
10001d8d  jmp     0x10001d97
10001d8f  mov     dword ptr [esp + 0x10], 0x4b00
10001d97  mov     ebp, dword ptr [esp + 0x10]
10001d9b  push    1
10001d9d  push    1
10001d9f  push    ebp
10001da0  call    0x10001610
10001da5  add     esp, 0xc
10001da8  mov     dword ptr [0x100400d8], ebp
10001dae  test    esi, esi
10001db0  je      0x10001db6
10001db2  push    0x32
10001db4  call    edi
10001db6  call    0x100018c0   ; EXP:ReadFromCOM
10001dbb  cmp     eax, 0x1000
10001dc0  je      0x10001dce
10001dc2  call    0x100018c0   ; EXP:ReadFromCOM
10001dc7  cmp     eax, 0x1000
10001dcc  jne     0x10001dc2
10001dce  push    0
10001dd0  call    edi
10001dd2  push    1
10001dd4  call    0x10001350
10001dd9  push    0x52
10001ddb  call    0x10001a20   ; EXP:WriteToCOM
10001de0  add     esp, 8
10001de3  mov     esi, 5
10001de8  call    0x100018c0   ; EXP:ReadFromCOM
10001ded  mov     ebp, eax
10001def  cmp     ebp, 0x51
10001df2  je      0x10001de8
10001df4  cmp     ebp, 0x1000
10001dfa  jne     0x10001e03
10001dfc  push    0x14
10001dfe  call    edi
10001e00  dec     esi
10001e01  jne     0x10001de8
10001e03  cmp     ebx, 1
10001e06  jne     0x10001e8e
10001e0c  mov     esi, dword ptr [esp + 0x10]
10001e10  cmp     esi, 0xe100
10001e16  jl      0x10001e8e
10001e18  xor     ebx, ebx
10001e1a  push    0x258
10001e1f  mov     dword ptr [0x10068e30], ebx
10001e25  call    edi
10001e27  mov     byte ptr [0x100426b0], bl
10001e2d  call    0x10013d30   ; EXP:McCheck
10001e32  test    al, al
10001e34  je      0x10001e88
10001e36  cmp     dword ptr [0x10068e30], 0x18
10001e3d  je      0x10001f01
10001e43  cmp     esi, 0xe100
10001e49  jne     0x10001f33
10001e4f  mov     esi, 0x1c200
10001e54  push    1
10001e56  mov     dword ptr [esp + 0x14], esi
10001e5a  call    0x10013cf0   ; EXP:McChangeBaudrate
10001e5f  add     esp, 4
10001e62  push    0x64
10001e64  call    edi
10001e66  push    1
10001e68  push    1
10001e6a  push    esi
10001e6b  call    0x10001610
10001e70  add     esp, 0xc
10001e73  mov     dword ptr [0x100400d8], esi
10001e79  call    0x10013d30   ; EXP:McCheck
10001e7e  test    al, al
10001e80  jne     0x10001f33
10001e86  jmp     0x10001e8e
10001e88  mov     dword ptr [0x10068e30], ebx
10001e8e  cmp     ebp, 0x52
10001e91  jne     0x10001ee5
10001e93  push    0x46
10001e95  call    0x10001a20   ; EXP:WriteToCOM
10001e9a  add     esp, 4
10001e9d  xor     esi, esi
10001e9f  call    0x100018c0   ; EXP:ReadFromCOM
10001ea4  cmp     eax, 0x1000
10001ea9  jne     0x10001eb7
10001eab  push    0x14
10001ead  call    edi
10001eaf  inc     esi
10001eb0  cmp     esi, 5
10001eb3  jl      0x10001e9f
10001eb5  jmp     0x10001ee5
10001eb7  cmp     eax, 0x46
10001eba  jne     0x10001ee5
10001ebc  push    0x3f
10001ebe  call    0x10001a20   ; EXP:WriteToCOM
10001ec3  add     esp, 4
10001ec6  xor     esi, esi
10001ec8  call    0x100018c0   ; EXP:ReadFromCOM
10001ecd  cmp     eax, 0x1000
10001ed2  jne     0x10001ee0
10001ed4  push    0x14
10001ed6  call    edi
10001ed8  inc     esi
10001ed9  cmp     esi, 5
10001edc  jl      0x10001ec8
10001ede  jmp     0x10001ee5
10001ee0  cmp     eax, 0x3f
10001ee3  je      0x10001efd
10001ee5  mov     ecx, dword ptr [esp + 0x14]
10001ee9  mov     eax, dword ptr [0x10068e30]
10001eee  inc     ecx
10001eef  test    eax, eax
10001ef1  mov     dword ptr [esp + 0x14], ecx
10001ef5  je      0x10001d37
10001efb  jmp     0x10001f01
10001efd  push    0
10001eff  call    edi
10001f01  cmp     dword ptr [esp + 0x14], 0x30
10001f06  jl      0x10001f33
10001f08  mov     eax, dword ptr [esp + 0x18]
10001f0c  mov     dword ptr [0x100400d8], eax
10001f11  call    0x10001460   ; EXP:ExitCOM
10001f16  push    0xd
10001f18  call    0x10001120
10001f1d  mov     eax, dword ptr [0x10072010]
10001f22  add     esp, 4
10001f25  test    eax, eax
10001f27  jle     0x10001f33
10001f29  mov     dword ptr [0x10072010], 1
10001f33  pop     edi
10001f34  pop     esi
10001f35  pop     ebp
10001f36  pop     ebx
10001f37  add     esp, 0xc
10001f3a  ret     
