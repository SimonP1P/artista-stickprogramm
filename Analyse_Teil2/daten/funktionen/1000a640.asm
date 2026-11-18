1000a640  sub     esp, 0x20
1000a643  push    ebx
1000a644  push    esi
1000a645  push    edi
1000a646  mov     edi, dword ptr [esp + 0x30]
1000a64a  test    edi, edi
1000a64c  je      0x1000a67d
1000a64e  mov     eax, dword ptr [esp + 0x34]
1000a652  mov     ebx, dword ptr [edi + 2]
1000a655  push    eax
1000a656  push    0
1000a658  push    edi
1000a659  lea     ecx, [esp + 0x18]
1000a65d  call    0x1000a370
1000a662  mov     esi, 8
1000a667  cmp     ebx, esi
1000a669  jle     0x1000a67d
1000a66b  push    esi
1000a66c  lea     ecx, [esp + 0x10]
1000a670  call    0x1000a4c0
1000a675  mov     byte ptr [esi + edi], al
1000a678  inc     esi
1000a679  cmp     esi, ebx
1000a67b  jl      0x1000a66b
1000a67d  pop     edi
1000a67e  pop     esi
1000a67f  pop     ebx
1000a680  add     esp, 0x20
1000a683  ret     
