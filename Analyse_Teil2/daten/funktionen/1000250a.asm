1000250a  push    ebx
1000250b  push    ebp
1000250c  mov     ebp, dword ptr [esp + 0xc]
10002510  push    esi
10002511  push    edi
10002512  mov     edi, dword ptr [esp + 0x18]
10002516  xor     ebx, ebx
10002518  push    1
1000251a  push    1
1000251c  push    ebx
1000251d  call    0x10001c80   ; EXP:OpenComInterface
10002522  mov     al, byte ptr [0x100426b0]
10002527  add     esp, 0xc
1000252a  cmp     al, bl
1000252c  je      0x1000253f
1000252e  push    0x20
10002530  push    edi
10002531  push    ebp
10002532  call    0x100139e0   ; EXP:McReadData
10002537  add     esp, 0xc
1000253a  jmp     0x10002648
1000253f  cmp     dword ptr [0x100400d8], 0x12c0
10002549  jne     0x10002586
1000254b  mov     eax, dword ptr [0x10068e30]
10002550  cmp     ebp, 0x1000
10002556  jae     0x10002571
10002558  cmp     eax, ebx
1000255a  jne     0x100025e3
10002560  push    0x3e8
10002565  push    0x45
10002567  call    0x100027d0
1000256c  add     esp, 8
1000256f  jmp     0x100025b5
10002571  cmp     eax, ebx
10002573  jne     0x100025e3
10002575  push    0x3e8
1000257a  push    0x52
1000257c  call    0x100027d0
10002581  add     esp, 8
10002584  jmp     0x100025b5
10002586  mov     eax, dword ptr [0x10068e30]
1000258b  cmp     eax, ebx
1000258d  jne     0x100025e3
1000258f  push    0x3e8
10002594  push    0x52
10002596  call    0x100027d0
1000259b  mov     eax, dword ptr [0x10068e30]
100025a0  add     esp, 8
100025a3  cmp     eax, ebx
100025a5  jne     0x100025e3
100025a7  mov     eax, ebp
100025a9  shr     eax, 0x10
100025ac  push    eax
100025ad  call    0x10002750
100025b2  add     esp, 4
100025b5  mov     eax, dword ptr [0x10068e30]
100025ba  cmp     eax, ebx
100025bc  jne     0x100025e3
100025be  mov     ecx, ebp
100025c0  shr     ecx, 8
100025c3  push    ecx
100025c4  call    0x10002750
100025c9  mov     eax, dword ptr [0x10068e30]
100025ce  add     esp, 4
100025d1  cmp     eax, ebx
100025d3  jne     0x100025e3
100025d5  push    ebp
100025d6  call    0x10002750
100025db  mov     eax, dword ptr [0x10068e30]
100025e0  add     esp, 4
100025e3  xor     esi, esi
100025e5  cmp     eax, ebx
100025e7  jne     0x1000263c
100025e9  cmp     si, 0x20
100025ed  jae     0x1000263c
100025ef  cmp     eax, ebx
100025f1  mov     byte ptr [esp + 0x18], bl
100025f5  jne     0x1000263c
100025f7  lea     edx, [esp + 0x18]
100025fb  push    edx
100025fc  call    0x10002670
10002601  mov     eax, dword ptr [0x10068e30]
10002606  add     esp, 4
10002609  cmp     eax, ebx
1000260b  jne     0x1000263c
1000260d  lea     eax, [esp + 0x18]
10002611  push    eax
10002612  call    0x10002670
10002617  mov     eax, dword ptr [0x10068e30]
1000261c  add     esp, 4
1000261f  cmp     eax, ebx
10002621  jne     0x1000263c
10002623  mov     dl, byte ptr [esp + 0x18]
10002627  mov     ecx, esi
10002629  and     ecx, 0xffff
1000262f  inc     esi
10002630  mov     byte ptr [ecx + edi], dl
10002633  mov     eax, dword ptr [0x10068e30]
10002638  cmp     eax, ebx
1000263a  je      0x100025e9
1000263c  push    0x4f
1000263e  push    0x10
10002640  call    0x10002700
10002645  add     esp, 8
10002648  call    0x100022a0
1000264d  test    eax, eax
1000264f  jne     0x10002518
10002655  mov     ecx, dword ptr [0x10068e30]
1000265b  pop     edi
1000265c  cmp     ecx, ebx
1000265e  pop     esi
1000265f  pop     ebp
10002660  pop     ebx
10002661  sete    al
10002664  ret     
