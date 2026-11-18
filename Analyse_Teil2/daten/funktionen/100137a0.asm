100137a0  sub     esp, 0x40c
100137a6  xor     dl, dl
100137a8  xor     eax, eax
100137aa  push    ebp
100137ab  push    esi
100137ac  push    edi
100137ad  mov     edi, dword ptr [esp + 0x420]
100137b4  xor     esi, esi
100137b6  test    edi, edi
100137b8  jle     0x100137ee
100137ba  mov     ebp, dword ptr [esp + 0x41c]
100137c1  cmp     eax, 0x402
100137c6  jae     0x1001382a
100137c8  mov     cl, byte ptr [esi + ebp]
100137cb  cmp     cl, 1
100137ce  mov     byte ptr [esp + 0xf], cl
100137d2  je      0x100137d9
100137d4  cmp     cl, 0x10
100137d7  jne     0x100137e2
100137d9  mov     byte ptr [esp + eax + 0x10], 0x10
100137de  inc     eax
100137df  add     dl, 0x10
100137e2  mov     byte ptr [esp + eax + 0x10], cl
100137e6  inc     eax
100137e7  add     dl, cl
100137e9  inc     esi
100137ea  cmp     esi, edi
100137ec  jl      0x100137c1
100137ee  mov     byte ptr [esp + eax + 0x10], dl
100137f2  inc     eax
100137f3  push    eax
100137f4  lea     eax, [esp + 0x14]
100137f8  push    eax
100137f9  call    0x10001ae0   ; EXP:WriteBufferToCOM
100137fe  add     esp, 8
10013801  test    eax, eax
10013803  jne     0x10013811
10013805  pop     edi
10013806  pop     esi
10013807  xor     al, al
10013809  pop     ebp
1001380a  add     esp, 0x40c
10013810  ret     
10013811  mov     ecx, dword ptr [esp + 0x424]
10013818  lea     edx, [esp + 0xf]
1001381c  push    ecx
1001381d  push    edx
1001381e  call    0x10001f40   ; EXP:ReadByte
10013823  add     esp, 8
10013826  test    eax, eax
10013828  jne     0x10013836
1001382a  pop     edi
1001382b  pop     esi
1001382c  xor     al, al
1001382e  pop     ebp
1001382f  add     esp, 0x40c
10013835  ret     
10013836  mov     cl, byte ptr [esp + 0xf]
1001383a  xor     eax, eax
1001383c  cmp     cl, 6
1001383f  pop     edi
10013840  pop     esi
10013841  pop     ebp
10013842  sete    al
10013845  add     esp, 0x40c
1001384b  ret     
