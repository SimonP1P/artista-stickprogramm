10004c90  push    ebx
10004c91  mov     ebx, dword ptr [esp + 8]
10004c95  mov     al, bl
10004c97  inc     al
10004c99  mov     byte ptr [0x10068d68], al
10004c9e  mov     eax, dword ptr [0x10068e30]
10004ca3  test    eax, eax
10004ca5  jne     0x10004cc4
10004ca7  push    1
10004ca9  push    0x10068d68
10004cae  push    0x10072032
10004cb3  call    0x10003dd0
10004cb8  add     esp, 4
10004cbb  push    eax
10004cbc  call    0x10002e50   ; EXP:WriteComBytes
10004cc1  add     esp, 0xc
10004cc4  mov     eax, 0x4bda12f7
10004cc9  imul    ebx
10004ccb  sar     edx, 3
10004cce  mov     ecx, edx
10004cd0  shr     ecx, 0x1f
10004cd3  add     edx, ecx
10004cd5  push    edx
10004cd6  call    0x10004ce0
10004cdb  add     esp, 4
10004cde  pop     ebx
10004cdf  ret     
10004ce0  mov     eax, dword ptr [0x10068e30]
10004ce5  test    eax, eax
10004ce7  jne     0x10004d09
10004ce9  lea     eax, [esp + 4]
10004ced  push    1
10004cef  push    eax
10004cf0  push    0x10072032
10004cf5  call    0x10003dd0
10004cfa  add     esp, 4
10004cfd  add     eax, 5
10004d00  push    eax
10004d01  call    0x10002e50   ; EXP:WriteComBytes
10004d06  add     esp, 0xc
10004d09  ret     
