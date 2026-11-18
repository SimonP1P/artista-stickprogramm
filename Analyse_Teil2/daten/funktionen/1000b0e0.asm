1000b0e0  push    -1
1000b0e2  push    0x10033008
1000b0e7  mov     eax, dword ptr fs:[0]
1000b0ed  push    eax
1000b0ee  mov     dword ptr fs:[0], esp
1000b0f5  sub     esp, 0x20
1000b0f8  push    ebx
1000b0f9  push    ebp
1000b0fa  push    esi
1000b0fb  mov     esi, ecx
1000b0fd  push    edi
1000b0fe  lea     ecx, [esp + 0x18]
1000b102  call    0x1000a750
1000b107  mov     ebp, dword ptr [esp + 0x48]
1000b10b  mov     edi, dword ptr [esp + 0x40]
1000b10f  lea     eax, [esp + 0x18]
1000b113  push    ebp
1000b114  push    eax
1000b115  push    edi
1000b116  mov     ecx, esi
1000b118  mov     dword ptr [esp + 0x44], 0
1000b120  call    0x1000c5a0
1000b125  test    eax, eax
1000b127  je      0x1000b20b
1000b12d  mov     ecx, esi
1000b12f  call    0x1000ac70
1000b134  test    edi, edi
1000b136  jl      0x1000b20b
1000b13c  push    0x30
1000b13e  lea     ecx, [esi + 0x54]
1000b141  call    0x1000a8b0
1000b146  and     eax, 0xff
1000b14b  cmp     edi, eax
1000b14d  jge     0x1000b20b
1000b153  push    edi
1000b154  mov     ecx, esi
1000b156  call    0x1000ad00
1000b15b  push    0
1000b15d  lea     ecx, [esp + 0x1c]
1000b161  call    0x1000a920
1000b166  mov     edi, eax
1000b168  lea     ecx, [edi + 4]
1000b16b  push    ecx
1000b16c  lea     ecx, [esp + 0x1c]
1000b170  call    0x1000a8b0
1000b175  lea     edx, [edi + 5]
1000b178  and     eax, 0xff
1000b17d  push    edx
1000b17e  lea     ecx, [esp + 0x1c]
1000b182  mov     ebx, eax
1000b184  call    0x1000a8b0
1000b189  and     eax, 0xff
1000b18e  mov     ecx, eax
1000b190  imul    ecx, ebx
1000b193  cmp     ecx, ebp
1000b195  jne     0x1000b20b
1000b197  add     edi, 7
1000b19a  dec     eax
1000b19b  test    eax, eax
1000b19d  jl      0x1000b1f2
1000b19f  mov     ebp, eax
1000b1a1  mov     ecx, ebx
1000b1a3  imul    ebp, ebx
1000b1a6  neg     ecx
1000b1a8  inc     eax
1000b1a9  mov     dword ptr [esp + 0x14], ecx
1000b1ad  mov     dword ptr [esp + 0x10], eax
1000b1b1  test    ebx, ebx
1000b1b3  mov     esi, ebp
1000b1b5  jle     0x1000b1e5
1000b1b7  mov     dword ptr [esp + 0x40], ebx
1000b1bb  cmp     esi, dword ptr [esp + 0x48]
1000b1bf  jge     0x1000b1d4
1000b1c1  push    edi
1000b1c2  lea     ecx, [esp + 0x1c]
1000b1c6  call    0x1000a8b0
1000b1cb  mov     edx, dword ptr [esp + 0x44]
1000b1cf  not     al
1000b1d1  mov     byte ptr [esi + edx], al
1000b1d4  mov     eax, dword ptr [esp + 0x40]
1000b1d8  inc     esi
1000b1d9  inc     edi
1000b1da  dec     eax
1000b1db  mov     dword ptr [esp + 0x40], eax
1000b1df  jne     0x1000b1bb
1000b1e1  mov     ecx, dword ptr [esp + 0x14]
1000b1e5  mov     eax, dword ptr [esp + 0x10]
1000b1e9  add     ebp, ecx
1000b1eb  dec     eax
1000b1ec  mov     dword ptr [esp + 0x10], eax
1000b1f0  jne     0x1000b1b1
1000b1f2  mov     bl, byte ptr [esp + 0x18]
1000b1f6  lea     ecx, [esp + 0x18]
1000b1fa  mov     dword ptr [esp + 0x38], 0xffffffff
1000b202  call    0x1000a770
1000b207  mov     al, bl
1000b209  jmp     0x1000b21e
1000b20b  lea     ecx, [esp + 0x18]
1000b20f  mov     dword ptr [esp + 0x38], 0xffffffff
1000b217  call    0x1000a770
1000b21c  xor     al, al
1000b21e  mov     ecx, dword ptr [esp + 0x30]
1000b222  pop     edi
1000b223  pop     esi
1000b224  pop     ebp
1000b225  pop     ebx
1000b226  mov     dword ptr fs:[0], ecx
1000b22d  add     esp, 0x2c
1000b230  ret     0xc
