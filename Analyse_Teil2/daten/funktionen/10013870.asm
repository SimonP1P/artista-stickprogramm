10013870  mov     eax, dword ptr [esp + 0xc]
10013874  sub     esp, 0x200
1001387a  mov     ecx, dword ptr [esp + 0x204]
10013881  push    ebp
10013882  push    esi
10013883  push    edi
10013884  mov     edi, dword ptr [esp + 0x21c]
1001388b  push    edi
1001388c  push    eax
1001388d  push    ecx
1001388e  call    0x10013900
10013893  add     esp, 0xc
10013896  test    al, al
10013898  jne     0x100138a6
1001389a  pop     edi
1001389b  pop     esi
1001389c  xor     al, al
1001389e  pop     ebp
1001389f  add     esp, 0x200
100138a5  ret     
100138a6  mov     ebp, dword ptr [esp + 0x214]
100138ad  xor     esi, esi
100138af  test    edi, edi
100138b1  jle     0x100138d1
100138b3  mov     ecx, ebp
100138b5  lea     edx, [esp + 0xc]
100138b9  sub     ecx, edx
100138bb  cmp     esi, 0x200
100138c1  jae     0x100138d1
100138c3  lea     eax, [esp + esi + 0xc]
100138c7  inc     esi
100138c8  cmp     esi, edi
100138ca  mov     dl, byte ptr [ecx + eax]
100138cd  mov     byte ptr [eax], dl
100138cf  jl      0x100138bb
100138d1  push    0x1f4
100138d6  lea     eax, [esp + 0x10]
100138da  push    esi
100138db  push    eax
100138dc  call    0x100137a0
100138e1  add     esp, 0xc
100138e4  test    al, al
100138e6  je      0x1001389a
100138e8  sub     edi, esi
100138ea  add     ebp, esi
100138ec  test    edi, edi
100138ee  jne     0x100138ad
100138f0  pop     edi
100138f1  pop     esi
100138f2  mov     al, 1
100138f4  pop     ebp
100138f5  add     esp, 0x200
100138fb  ret     
