10002f00  push    ecx
10002f01  mov     eax, dword ptr [0x10068e30]
10002f06  push    ebx
10002f07  mov     ebx, dword ptr [esp + 0xc]
10002f0b  push    esi
10002f0c  xor     esi, esi
10002f0e  push    edi
10002f0f  test    eax, eax
10002f11  jne     0x10002f2b
10002f13  mov     eax, dword ptr [esp + 0x18]
10002f17  mov     ecx, ebx
10002f19  add     eax, 0x1388
10002f1e  or      ecx, 1
10002f21  push    eax
10002f22  push    ecx
10002f23  call    0x10003130
10002f28  add     esp, 8
10002f2b  call    dword ptr [0x100351e8]   ; KERNEL32.dll!GetTickCount
10002f31  mov     dword ptr [esp + 0xc], eax
10002f35  mov     eax, dword ptr [0x10068e30]
10002f3a  test    eax, eax
10002f3c  jne     0x10002fa3
10002f3e  mov     edi, dword ptr [0x10035284]   ; KERNEL32.dll!Sleep
10002f44  mov     eax, dword ptr [esp + 0x18]
10002f48  test    eax, eax
10002f4a  jle     0x10003007
10002f50  lea     edx, [esp + 0x14]
10002f54  push    edx
10002f55  call    0x10003160
10002f5a  add     esp, 4
10002f5d  test    eax, eax
10002f5f  je      0x10002f84
10002f61  mov     eax, dword ptr [esp + 0x14]
10002f65  cmp     ebx, 0x100
10002f6b  je      0x10002f73
10002f6d  mov     word ptr [0x10072088], ax
10002f73  test    al, 6
10002f75  jne     0x10002fb5
10002f77  cmp     ebx, 0x100
10002f7d  jne     0x10002f84
10002f7f  test    ah, 1
10002f82  je      0x10002fb5
10002f84  push    0x64
10002f86  call    edi
10002f88  lea     eax, [esp + 0xc]
10002f8c  lea     ecx, [esp + 0x18]
10002f90  push    eax
10002f91  push    ecx
10002f92  call    0x100012d0
10002f97  mov     eax, dword ptr [0x10068e30]
10002f9c  add     esp, 8
10002f9f  test    eax, eax
10002fa1  je      0x10002f44
10002fa3  mov     ecx, dword ptr [0x10068e30]
10002fa9  xor     eax, eax
10002fab  test    ecx, ecx
10002fad  pop     edi
10002fae  pop     esi
10002faf  sete    al
10002fb2  pop     ebx
10002fb3  pop     ecx
10002fb4  ret     
10002fb5  cmp     ebx, 8
10002fb8  mov     esi, 1
10002fbd  je      0x100030df
10002fc3  cmp     word ptr [esp + 0x20], 0
10002fc9  jne     0x100030df
10002fcf  test    ah, 0x10
10002fd2  je      0x10003064
10002fd8  mov     al, byte ptr [0x100670c8]
10002fdd  test    al, al
10002fdf  je      0x10002ffd
10002fe1  mov     dword ptr [0x10068e30], 3
10002feb  mov     ecx, dword ptr [0x10068e30]
10002ff1  xor     eax, eax
10002ff3  pop     edi
10002ff4  test    ecx, ecx
10002ff6  pop     esi
10002ff7  pop     ebx
10002ff8  sete    al
10002ffb  pop     ecx
10002ffc  ret     
10002ffd  push    0x16
10002fff  call    0x10001120
10003004  add     esp, 4
10003007  mov     eax, dword ptr [0x10068e30]
1000300c  test    eax, eax
1000300e  jne     0x10002fa3
10003010  test    si, si
10003013  jne     0x10002fa3
10003015  cmp     ebx, 0x30
10003018  je      0x10003129
1000301e  cmp     ebx, 0x60
10003021  je      0x10003129
10003027  cmp     ebx, 0xc0
1000302d  je      0x10003129
10003033  cmp     ebx, 0x50
10003036  je      0x10003129
1000303c  cmp     ebx, 0xa0
10003042  je      0x10003129
10003048  push    0x11
1000304a  call    0x10001120
1000304f  mov     ecx, dword ptr [0x10068e30]
10003055  add     esp, 4
10003058  xor     eax, eax
1000305a  test    ecx, ecx
1000305c  pop     edi
1000305d  pop     esi
1000305e  sete    al
10003061  pop     ebx
10003062  pop     ecx
10003063  ret     
10003064  test    ah, 0x20
10003067  je      0x1000306d
10003069  push    0x17
1000306b  jmp     0x10002fff
1000306d  test    ah, 0x40
10003070  je      0x10003076
10003072  push    0x18
10003074  jmp     0x10002fff
10003076  test    ah, 0x80
10003079  je      0x10003092
1000307b  mov     al, byte ptr [0x100670c8]
10003080  test    al, al
10003082  je      0x1000308b
10003084  push    0x14
10003086  jmp     0x10002fff
1000308b  push    0x19
1000308d  jmp     0x10002fff
10003092  shr     eax, 2
10003095  not     al
10003097  and     eax, 1
1000309a  mov     esi, eax
1000309c  test    si, si
1000309f  jne     0x10003007
100030a5  cmp     ebx, 0x200
100030ab  jne     0x100030bd
100030ad  mov     al, byte ptr [0x100670c8]
100030b2  test    al, al
100030b4  je      0x1000308b
100030b6  push    0x14
100030b8  jmp     0x10002fff
100030bd  cmp     ebx, 0x400
100030c3  jne     0x100030cc
100030c5  push    0x1f
100030c7  jmp     0x10002fff
100030cc  cmp     ebx, 0x800
100030d2  jne     0x10003007
100030d8  push    3
100030da  jmp     0x10002fff
100030df  mov     esi, eax
100030e1  shr     esi, 1
100030e3  and     esi, 1
100030e6  test    si, si
100030e9  jne     0x10003007
100030ef  mov     ecx, dword ptr [0x10068e30]
100030f5  test    ecx, ecx
100030f7  jne     0x10002fa3
100030fd  test    al, 4
100030ff  je      0x10003010
10003105  mov     dl, byte ptr [0x100670c8]
1000310b  pop     edi
1000310c  neg     dl
1000310e  sbb     edx, edx
10003110  xor     eax, eax
10003112  and     edx, 0xffffffed
10003115  pop     esi
10003116  add     edx, 0x16
10003119  pop     ebx
1000311a  mov     ecx, edx
1000311c  mov     dword ptr [0x10068e30], edx
10003122  test    ecx, ecx
10003124  sete    al
10003127  pop     ecx
10003128  ret     
10003129  pop     edi
1000312a  pop     esi
1000312b  xor     eax, eax
1000312d  pop     ebx
1000312e  pop     ecx
1000312f  ret     
10003130  mov     eax, dword ptr [esp + 4]
10003134  mov     edx, dword ptr [esp + 8]
10003138  mov     ecx, eax
1000313a  mov     byte ptr [esp + 5], al
1000313e  push    edx
1000313f  lea     eax, [esp + 8]
10003143  push    2
10003145  push    eax
10003146  sar     ecx, 8
10003149  push    0xfffed0
1000314e  mov     byte ptr [esp + 0x14], cl
10003152  call    0x10002b20
10003157  add     esp, 0x10
1000315a  ret     
