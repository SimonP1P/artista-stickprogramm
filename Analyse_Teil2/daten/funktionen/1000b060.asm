1000b060  push    ebx
1000b061  push    esi
1000b062  mov     ebx, ecx
1000b064  call    0x1000ac70
1000b069  mov     esi, dword ptr [esp + 0xc]
1000b06d  test    esi, esi
1000b06f  jl      0x1000b0cc
1000b071  push    0x30
1000b073  lea     ecx, [ebx + 0x54]
1000b076  call    0x1000a8b0
1000b07b  and     eax, 0xff
1000b080  cmp     esi, eax
1000b082  jge     0x1000b0cc
1000b084  push    edi
1000b085  push    esi
1000b086  mov     ecx, ebx
1000b088  call    0x1000ad00
1000b08d  mov     edi, dword ptr [esp + 0x1c]
1000b091  lea     esi, [ebx + 0xc]
1000b094  mov     ecx, 0x10
1000b099  rep movsd dword ptr es:[edi], dword ptr [esi]
1000b09b  mov     esi, dword ptr [esp + 0x14]
1000b09f  test    esi, esi
1000b0a1  je      0x1000b0c4
1000b0a3  push    ebp
1000b0a4  mov     ebp, dword ptr [esp + 0x1c]
1000b0a8  xor     eax, eax
1000b0aa  lea     edi, [ebp - 1]
1000b0ad  cmp     eax, edi
1000b0af  jge     0x1000b0be
1000b0b1  mov     cl, byte ptr [eax + ebx + 0x2a]
1000b0b5  mov     byte ptr [eax + esi], cl
1000b0b8  inc     eax
1000b0b9  cmp     eax, 0x20
1000b0bc  jb      0x1000b0ad
1000b0be  mov     byte ptr [esi + ebp - 1], 0
1000b0c3  pop     ebp
1000b0c4  pop     edi
1000b0c5  pop     esi
1000b0c6  mov     al, 1
1000b0c8  pop     ebx
1000b0c9  ret     0x10
1000b0cc  pop     esi
1000b0cd  xor     al, al
1000b0cf  pop     ebx
1000b0d0  ret     0x10
