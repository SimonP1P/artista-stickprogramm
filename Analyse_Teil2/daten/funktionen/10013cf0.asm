10013cf0  push    ecx
10013cf1  mov     cl, byte ptr [0x100426b0]
10013cf7  xor     al, al
10013cf9  cmp     cl, al
10013cfb  jne     0x10013cff
10013cfd  pop     ecx
10013cfe  ret     
10013cff  mov     cl, byte ptr [esp + 8]
10013d03  lea     edx, [esp]
10013d07  cmp     cl, al
10013d09  push    4
10013d0b  setne   cl
10013d0e  add     ecx, 3
10013d11  push    edx
10013d12  mov     byte ptr [esp + 8], 0x62
10013d17  mov     byte ptr [esp + 9], cl
10013d1b  mov     byte ptr [esp + 0xa], al
10013d1f  mov     byte ptr [esp + 0xb], al
10013d23  call    0x10013720
10013d28  add     esp, 8
10013d2b  pop     ecx
10013d2c  ret     
