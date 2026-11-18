10007590  mov     ecx, dword ptr [esp + 0xc]
10007594  test    ecx, ecx
10007596  jne     0x1000759b
10007598  xor     eax, eax
1000759a  ret     
1000759b  mov     eax, dword ptr [ecx + 2]
1000759e  push    ebx
1000759f  mov     ebx, dword ptr [esp + 0x14]
100075a3  push    ebp
100075a4  push    esi
100075a5  push    edi
100075a6  test    ebx, ebx
100075a8  je      0x100075ae
100075aa  cmp     eax, ebx
100075ac  jae     0x100075b0
100075ae  mov     ebx, eax
100075b0  mov     esi, dword ptr [esp + 0x14]
100075b4  push    0
100075b6  push    0
100075b8  push    ecx
100075b9  mov     ecx, esi
100075bb  call    0x1000a370
100075c0  mov     bp, word ptr [esp + 0x18]
100075c5  mov     edi, 8
100075ca  push    edi
100075cb  mov     ecx, esi
100075cd  call    0x1000a590
100075d2  cmp     word ptr [esi + 0x18], bp
100075d6  je      0x100075f3
100075d8  mov     eax, dword ptr [esi + 0x1a]
100075db  cmp     eax, ebx
100075dd  jae     0x100075f1
100075df  cmp     eax, 6
100075e2  jbe     0x100075f1
100075e4  sub     ebx, eax
100075e6  add     edi, eax
100075e8  jne     0x100075ca
100075ea  mov     eax, edi
100075ec  pop     edi
100075ed  pop     esi
100075ee  pop     ebp
100075ef  pop     ebx
100075f0  ret     
100075f1  xor     edi, edi
100075f3  mov     eax, edi
100075f5  pop     edi
100075f6  pop     esi
100075f7  pop     ebp
100075f8  pop     ebx
100075f9  ret     
