1000ac70  push    ecx
1000ac71  push    ebx
1000ac72  push    esi
1000ac73  mov     esi, ecx
1000ac75  push    edi
1000ac76  push    0x30
1000ac78  lea     edi, [esi + 0x54]
1000ac7b  mov     ecx, edi
1000ac7d  call    0x1000a8b0
1000ac82  push    0x32
1000ac84  mov     ecx, edi
1000ac86  mov     byte ptr [esp + 0x10], al
1000ac8a  call    0x1000a920
1000ac8f  push    0x36
1000ac91  mov     ecx, edi
1000ac93  mov     dword ptr [esi], eax
1000ac95  call    0x1000a920
1000ac9a  mov     dword ptr [esi + 4], eax
1000ac9d  mov     eax, dword ptr [esi]
1000ac9f  cmp     eax, 0x1000
1000aca4  jl      0x1000acec
1000aca6  cmp     eax, 0x2000
1000acab  jg      0x1000acec
1000acad  push    0x46
1000acaf  mov     ecx, edi
1000acb1  mov     ebx, eax
1000acb3  call    0x1000a8e0
1000acb8  and     eax, 0xffff
1000acbd  add     eax, 0x48
1000acc0  cmp     ebx, eax
1000acc2  jl      0x1000acec
1000acc4  mov     esi, dword ptr [esi + 4]
1000acc7  lea     eax, [ebx + 0x1000]
1000accd  cmp     esi, eax
1000accf  jl      0x1000acec
1000acd1  cmp     esi, 0x10000
1000acd7  jge     0x1000acec
1000acd9  mov     ecx, dword ptr [esp + 0xc]
1000acdd  and     ecx, 0xff
1000ace3  shl     ecx, 6
1000ace6  add     ecx, ebx
1000ace8  cmp     esi, ecx
1000acea  jge     0x1000acef
1000acec  mov     byte ptr [edi], 0
1000acef  mov     al, byte ptr [edi]
1000acf1  pop     edi
1000acf2  pop     esi
1000acf3  pop     ebx
1000acf4  pop     ecx
1000acf5  ret     
