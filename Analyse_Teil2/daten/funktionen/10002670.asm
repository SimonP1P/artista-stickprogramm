10002670  push    ecx
10002671  mov     eax, dword ptr [0x10068e30]
10002676  test    eax, eax
10002678  je      0x1000267e
1000267a  xor     eax, eax
1000267c  pop     ecx
1000267d  ret     
1000267e  lea     eax, [esp + 3]
10002682  push    0x7d0
10002687  push    eax
10002688  call    0x10001f40   ; EXP:ReadByte
1000268d  add     esp, 8
10002690  test    eax, eax
10002692  je      0x100026f0
10002694  mov     al, byte ptr [esp + 3]
10002698  cmp     al, 0x30
1000269a  jb      0x100026bf
1000269c  cmp     al, 0x39
1000269e  ja      0x100026bf
100026a0  mov     ecx, dword ptr [esp + 8]
100026a4  mov     dl, byte ptr [ecx]
100026a6  add     dl, 0xd
100026a9  shl     dl, 4
100026ac  add     dl, al
100026ae  xor     eax, eax
100026b0  mov     byte ptr [ecx], dl
100026b2  mov     ecx, dword ptr [0x10068e30]
100026b8  test    ecx, ecx
100026ba  sete    al
100026bd  pop     ecx
100026be  ret     
100026bf  cmp     al, 0x41
100026c1  jb      0x100026e6
100026c3  cmp     al, 0x46
100026c5  ja      0x100026e6
100026c7  mov     ecx, dword ptr [esp + 8]
100026cb  mov     dl, byte ptr [ecx]
100026cd  shl     dl, 4
100026d0  add     dl, al
100026d2  xor     eax, eax
100026d4  sub     dl, 0x37
100026d7  mov     byte ptr [ecx], dl
100026d9  mov     ecx, dword ptr [0x10068e30]
100026df  test    ecx, ecx
100026e1  sete    al
100026e4  pop     ecx
100026e5  ret     
100026e6  push    0x10
100026e8  call    0x10001120
100026ed  add     esp, 4
100026f0  mov     ecx, dword ptr [0x10068e30]
100026f6  xor     eax, eax
100026f8  test    ecx, ecx
100026fa  sete    al
100026fd  pop     ecx
100026fe  ret     
