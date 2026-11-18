10002330  mov     eax, dword ptr fs:[0]
10002336  push    -1
10002338  push    0x10032ce7
1000233d  push    eax
1000233e  mov     eax, dword ptr [0x10068e34]
10002343  mov     dword ptr fs:[0], esp
1000234a  sub     esp, 0x14c
10002350  test    eax, eax
10002352  jne     0x1000236c
10002354  xor     ax, ax
10002357  mov     ecx, dword ptr [esp + 0x14c]
1000235e  mov     dword ptr fs:[0], ecx
10002365  add     esp, 0x158
1000236b  ret     
1000236c  mov     eax, dword ptr [0x10072094]
10002371  push    esi
10002372  mov     esi, 4
10002377  cmp     eax, esi
10002379  jne     0x10002418
1000237f  call    0x10001180
10002384  push    0
10002386  lea     ecx, [esp + 8]
1000238a  call    0x1000f9c0
1000238f  lea     ecx, [esp + 4]
10002393  mov     dword ptr [esp + 0x158], 0
1000239e  call    0x1000f5d0
100023a3  cmp     eax, 1
100023a6  je      0x100023ae
100023a8  mov     dword ptr [0x10072094], esi
100023ae  lea     ecx, [esp + 0x114]
100023b5  mov     dword ptr [esp + 0x158], esi
100023bc  call    0x1002efd8
100023c1  lea     ecx, [esp + 0xd8]
100023c8  mov     byte ptr [esp + 0x158], 3
100023d0  call    0x1002efd8
100023d5  lea     ecx, [esp + 0x9c]
100023dc  mov     byte ptr [esp + 0x158], 2
100023e4  call    0x1002efd8
100023e9  lea     ecx, [esp + 0x60]
100023ed  mov     byte ptr [esp + 0x158], 1
100023f5  call    0x1002efd8
100023fa  lea     ecx, [esp + 4]
100023fe  mov     dword ptr [esp + 0x158], 0xffffffff
10002409  call    0x1002754e
1000240e  call    0x100011b0
10002413  mov     eax, dword ptr [0x10072094]
10002418  cmp     eax, 3
1000241b  ja      0x10002439
1000241d  jmp     dword ptr [eax*4 + 0x10002470]
10002424  mov     esi, 1
10002429  jmp     0x1000243b
1000242b  mov     esi, 2
10002430  jmp     0x1000243b
10002432  mov     esi, 3
10002437  jmp     0x1000243b
10002439  xor     esi, esi
1000243b  mov     eax, dword ptr [0x1007201c]
10002440  cmp     eax, esi
10002442  je      0x10002450
10002444  call    0x10001460   ; EXP:ExitCOM
10002449  mov     eax, esi
1000244b  mov     dword ptr [0x1007201c], eax
10002450  xor     ecx, ecx
10002452  pop     esi
10002453  test    eax, eax
10002455  setne   cl
10002458  mov     ax, cx
1000245b  mov     ecx, dword ptr [esp + 0x14c]
10002462  mov     dword ptr fs:[0], ecx
10002469  add     esp, 0x158
1000246f  ret     
10002470  and     al, 0x24
10002472  add     byte ptr [eax], dl
10002474  sub     esp, dword ptr [eax + eax]
10002477  adc     byte ptr [edx], dh
10002479  and     al, 0
1000247b  adc     byte ptr [ebx], bh
1000247d  and     al, 0
1000247f  adc     byte ptr [edx - 1], ch
10002482  push    0x10032d2f
10002487  mov     eax, dword ptr fs:[0]
1000248d  push    eax
1000248e  mov     dword ptr fs:[0], esp
10002495  push    ecx
10002496  push    esi
10002497  mov     esi, ecx
10002499  mov     dword ptr [esp + 4], esi
1000249d  lea     ecx, [esi + 0x110]
100024a3  mov     dword ptr [esp + 0x10], 3
100024ab  call    0x1002efd8
100024b0  lea     ecx, [esi + 0xd4]
100024b6  mov     byte ptr [esp + 0x10], 2
100024bb  call    0x1002efd8
100024c0  lea     ecx, [esi + 0x98]
100024c6  mov     byte ptr [esp + 0x10], 1
100024cb  call    0x1002efd8
100024d0  lea     ecx, [esi + 0x5c]
100024d3  mov     byte ptr [esp + 0x10], 0
100024d8  call    0x1002efd8
100024dd  mov     ecx, esi
100024df  mov     dword ptr [esp + 0x10], 0xffffffff
100024e7  call    0x1002754e
100024ec  mov     ecx, dword ptr [esp + 8]
100024f0  pop     esi
100024f1  mov     dword ptr fs:[0], ecx
100024f8  add     esp, 0x10
100024fb  ret     
