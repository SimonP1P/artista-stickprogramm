100017f0  mov     eax, dword ptr [esp + 4]
100017f4  mov     edx, dword ptr [0x100426c0]
100017fa  push    ebx
100017fb  mov     dword ptr [0x100426bc], eax
10001800  mov     eax, dword ptr [0x10068e7c]
10001805  xor     ebx, ebx
10001807  xor     ecx, ecx
10001809  cmp     eax, ebx
1000180b  setne   cl
1000180e  and     ecx, 1
10001811  mov     byte ptr [0x100426cb], al
10001816  shl     ecx, 1
10001818  and     edx, 0xffffc4c1
1000181e  mov     eax, 0x100
10001823  or      ecx, edx
10001825  mov     word ptr [0x100426c6], ax
1000182b  mov     word ptr [0x100426c8], ax
10001831  mov     eax, dword ptr [0x100400d4]
10001836  push    esi
10001837  mov     esi, dword ptr [0x100351e0]   ; KERNEL32.dll!SetCommState
1000183d  or      ecx, 1
10001840  push    0x100426b8
10001845  push    eax
10001846  mov     byte ptr [0x100426ca], 8
1000184d  mov     byte ptr [0x100426cc], bl
10001853  mov     dword ptr [0x100426c0], ecx
10001859  mov     byte ptr [0x100426cd], bl
1000185f  mov     byte ptr [0x100426ce], bl
10001865  mov     byte ptr [0x100426d0], bl
1000186b  mov     byte ptr [0x100426d1], bl
10001871  call    esi
10001873  test    eax, eax
10001875  jne     0x1000187a
10001877  pop     esi
10001878  pop     ebx
10001879  ret     
1000187a  mov     eax, dword ptr [esp + 0x14]
1000187e  cmp     eax, ebx
10001880  mov     eax, dword ptr [0x100426c0]
10001885  je      0x1000188d
10001887  and     al, 0xdf
10001889  or      al, 0x10
1000188b  jmp     0x1000188f
1000188d  and     al, 0xcf
1000188f  cmp     dword ptr [esp + 0x10], ebx
10001893  je      0x1000189d
10001895  and     ah, 0xdf
10001898  or      ah, 0x10
1000189b  jmp     0x100018a0
1000189d  and     ah, 0xcf
100018a0  mov     ecx, dword ptr [0x100400d4]
100018a6  push    0x100426b8
100018ab  push    ecx
100018ac  mov     dword ptr [0x100426c0], eax
100018b1  call    esi
100018b3  pop     esi
100018b4  pop     ebx
100018b5  ret     
