1000a4c0  push    esi
1000a4c1  mov     esi, ecx
1000a4c3  mov     eax, dword ptr [esi + 0x14]
1000a4c6  test    eax, eax
1000a4c8  jne     0x1000a4d0
1000a4ca  xor     al, al
1000a4cc  pop     esi
1000a4cd  ret     4
1000a4d0  mov     ecx, dword ptr [esi + 0xa]
1000a4d3  push    edi
1000a4d4  mov     edi, dword ptr [esp + 0xc]
1000a4d8  cmp     ecx, edi
1000a4da  jle     0x1000a4f1
1000a4dc  mov     cx, word ptr [esi + 0xe]
1000a4e0  mov     edx, dword ptr [esi + 0x10]
1000a4e3  push    ecx
1000a4e4  push    edx
1000a4e5  push    eax
1000a4e6  mov     ecx, esi
1000a4e8  call    0x1000a370
1000a4ed  xor     al, al
1000a4ef  jmp     0x1000a4f5
1000a4f1  mov     al, byte ptr [esp + 0xc]
1000a4f5  cmp     dword ptr [esi + 0xa], edi
1000a4f8  jg      0x1000a54c
1000a4fa  push    ebx
1000a4fb  mov     eax, dword ptr [esi + 0x14]
1000a4fe  mov     ecx, dword ptr [esi + 0xa]
1000a501  mov     dl, byte ptr [eax + ecx]
1000a504  xor     ecx, ecx
1000a506  mov     cl, byte ptr [esi + 8]
1000a509  lea     eax, [ecx + 2]
1000a50c  lea     ebx, [ecx - 1]
1000a50f  and     eax, 7
1000a512  and     ebx, 7
1000a515  mov     al, byte ptr [eax + esi]
1000a518  xor     al, byte ptr [ebx + esi]
1000a51b  lea     ebx, [ecx + 1]
1000a51e  and     ebx, 7
1000a521  xor     al, byte ptr [ebx + esi]
1000a524  xor     al, dl
1000a526  cmp     word ptr [esi + 0xe], 0
1000a52b  je      0x1000a52f
1000a52d  mov     dl, al
1000a52f  mov     byte ptr [ecx + esi], dl
1000a532  mov     cl, byte ptr [esi + 8]
1000a535  mov     edx, dword ptr [esi + 0xa]
1000a538  sub     cl, 3
1000a53b  and     cl, 7
1000a53e  inc     edx
1000a53f  mov     byte ptr [esi + 8], cl
1000a542  mov     ecx, edx
1000a544  cmp     ecx, edi
1000a546  mov     dword ptr [esi + 0xa], edx
1000a549  jle     0x1000a4fb
1000a54b  pop     ebx
1000a54c  pop     edi
1000a54d  pop     esi
1000a54e  ret     4
