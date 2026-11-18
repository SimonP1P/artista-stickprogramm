100136a0  push    ecx
100136a1  mov     al, byte ptr [0x100426b0]
100136a6  test    al, al
100136a8  jne     0x100136ae
100136aa  mov     al, 1
100136ac  pop     ecx
100136ad  ret     
100136ae  mov     al, byte ptr [esp + 8]
100136b2  push    ebx
100136b3  neg     al
100136b5  sbb     al, al
100136b7  and     al, 0xc
100136b9  add     al, 0x64
100136bb  push    eax
100136bc  call    0x10013710   ; EXP:McSendInstr
100136c1  mov     bl, al
100136c3  add     esp, 4
100136c6  test    bl, bl
100136c8  je      0x100136f1
100136ca  lea     ecx, [esp + 7]
100136ce  push    0x1f4
100136d3  push    ecx
100136d4  call    0x10001f40   ; EXP:ReadByte
100136d9  add     esp, 8
100136dc  test    eax, eax
100136de  setne   bl
100136e1  test    bl, bl
100136e3  je      0x100136f1
100136e5  cmp     byte ptr [esp + 7], 6
100136ea  sete    bl
100136ed  test    bl, bl
100136ef  jne     0x10013703
100136f1  mov     al, byte ptr [esp + 0xc]
100136f5  test    al, al
100136f7  jne     0x10013703
100136f9  push    0xc
100136fb  call    0x10001120
10013700  add     esp, 4
10013703  mov     al, bl
10013705  pop     ebx
10013706  pop     ecx
10013707  ret     
