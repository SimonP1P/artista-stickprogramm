10005e30  push    ebx
10005e31  push    esi
10005e32  push    edi
10005e33  call    0x10002330
10005e38  test    ax, ax
10005e3b  jne     0x10005e41
10005e3d  pop     edi
10005e3e  pop     esi
10005e3f  pop     ebx
10005e40  ret     
10005e41  mov     di, word ptr [esp + 0x10]
10005e46  xor     ebx, ebx
10005e48  cmp     di, bx
10005e4b  jne     0x10005e89
10005e4d  cmp     dword ptr [esp + 0x14], ebx
10005e51  je      0x10005e89
10005e53  cmp     dword ptr [0x10068e2c], ebx
10005e59  je      0x10005e89
10005e5b  mov     esi, dword ptr [0x10068e30]
10005e61  push    ebx
10005e62  push    0x7d0
10005e67  push    0x7d0
10005e6c  push    0x100
10005e71  mov     dword ptr [0x10068e30], ebx
10005e77  call    0x10002f00
10005e7c  add     esp, 0x10
10005e7f  cmp     esi, ebx
10005e81  je      0x10005e89
10005e83  mov     dword ptr [0x10068e30], esi
10005e89  call    0x100022f0
10005e8e  test    eax, eax
10005e90  jne     0x10005e99
10005e92  pop     edi
10005e93  pop     esi
10005e94  xor     ax, ax
10005e97  pop     ebx
10005e98  ret     
10005e99  mov     esi, 1
10005e9e  push    esi
10005e9f  push    esi
10005ea0  push    ebx
10005ea1  call    0x10001c80   ; EXP:OpenComInterface
10005ea6  mov     al, byte ptr [0x100426b0]
10005eab  add     esp, 0xc
10005eae  cmp     al, bl
10005eb0  je      0x10005eba
10005eb2  mov     dword ptr [0x10072010], esi
10005eb8  jmp     0x10005f19
10005eba  cmp     dword ptr [0x10068e30], ebx
10005ec0  jne     0x10005f19
10005ec2  push    0x3e8
10005ec7  push    0x54
10005ec9  call    0x100027d0
10005ece  mov     eax, dword ptr [0x10068e30]
10005ed3  add     esp, 8
10005ed6  cmp     eax, ebx
10005ed8  jne     0x10005f19
10005eda  push    0x3e8
10005edf  push    0x72
10005ee1  call    0x100027d0
10005ee6  mov     eax, dword ptr [0x10068e30]
10005eeb  add     esp, 8
10005eee  cmp     eax, ebx
10005ef0  jne     0x10005f19
10005ef2  push    0x3e8
10005ef7  push    0x4d
10005ef9  call    0x100027d0
10005efe  mov     eax, dword ptr [0x10068e30]
10005f03  add     esp, 8
10005f06  cmp     eax, ebx
10005f08  jne     0x10005f19
10005f0a  push    0x3e8
10005f0f  push    0x45
10005f11  call    0x100027d0
10005f16  add     esp, 8
10005f19  call    0x100022a0
10005f1e  test    eax, eax
10005f20  jne     0x10005e99
10005f26  mov     eax, dword ptr [0x10068e30]
10005f2b  cmp     eax, ebx
10005f2d  jne     0x1000604f
10005f33  cmp     di, bx
10005f36  je      0x1000604f
10005f3c  cmp     byte ptr [0x100426b0], bl
10005f42  jne     0x1000604f
10005f48  push    0xe100
10005f4d  call    0x10005950   ; EXP:ChangeBaudrate
10005f52  mov     eax, dword ptr [0x10068e30]
10005f57  add     esp, 4
10005f5a  cmp     eax, ebx
10005f5c  jne     0x1000604f
10005f62  cmp     byte ptr [0x100426b0], bl
10005f68  jne     0x1000604f
10005f6e  push    0xfa
10005f73  call    dword ptr [0x10035284]   ; KERNEL32.dll!Sleep
10005f79  call    0x100022f0
10005f7e  test    eax, eax
10005f80  jne     0x10005f89
10005f82  pop     edi
10005f83  pop     esi
10005f84  xor     ax, ax
10005f87  pop     ebx
10005f88  ret     
10005f89  push    esi
10005f8a  push    esi
10005f8b  push    ebx
10005f8c  call    0x10001c80   ; EXP:OpenComInterface
10005f91  mov     eax, dword ptr [0x10068e30]
10005f96  add     esp, 0xc
10005f99  cmp     eax, ebx
10005f9b  jne     0x10006028
10005fa1  push    0x3e8
10005fa6  push    0x59
10005fa8  call    0x100027d0
10005fad  mov     eax, dword ptr [0x10068e30]
10005fb2  add     esp, 8
10005fb5  cmp     eax, ebx
10005fb7  jne     0x10006028
10005fb9  push    0x3e8
10005fbe  push    0x51
10005fc0  call    0x100027d0
10005fc5  mov     eax, dword ptr [0x10068e30]
10005fca  add     esp, 8
10005fcd  cmp     eax, ebx
10005fcf  jne     0x10006028
10005fd1  lea     eax, [esp + 0x10]
10005fd5  push    0xbb8
10005fda  push    eax
10005fdb  mov     byte ptr [esp + 0x18], bl
10005fdf  call    0x10001f40   ; EXP:ReadByte
10005fe4  mov     eax, dword ptr [0x10068e30]
10005fe9  add     esp, 8
10005fec  cmp     eax, ebx
10005fee  jne     0x10006001
10005ff0  cmp     byte ptr [esp + 0x10], 0x4f
10005ff5  je      0x10006028
10005ff7  push    0x10
10005ff9  call    0x10001120
10005ffe  add     esp, 4
10006001  cmp     dword ptr [0x10068e30], 0xd
10006008  jne     0x10006028
1000600a  cmp     dword ptr [0x10072010], ebx
10006010  jle     0x10006028
10006012  push    0x1e
10006014  mov     dword ptr [0x10068e30], ebx
1000601a  mov     dword ptr [0x10072010], esi
10006020  call    0x10001120
10006025  add     esp, 4
10006028  call    0x100022a0
1000602d  test    eax, eax
1000602f  jne     0x10005f89
10006035  mov     eax, dword ptr [0x10068e30]
1000603a  cmp     eax, ebx
1000603c  jne     0x1000604f
1000603e  mov     cl, byte ptr [0x100426b0]
10006044  mov     byte ptr [0x10068e88], 1
1000604b  cmp     cl, bl
1000604d  je      0x10006055
1000604f  mov     byte ptr [0x10068e88], bl
10006055  xor     ecx, ecx
10006057  cmp     eax, ebx
10006059  sete    cl
1000605c  pop     edi
1000605d  pop     esi
1000605e  mov     ax, cx
10006061  pop     ebx
10006062  ret     
