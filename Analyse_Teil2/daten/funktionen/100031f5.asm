100031f5  call    0x10007500
100031fa  test    al, al
100031fc  jne     0x10003222
100031fe  mov     al, byte ptr [0x100426b0]
10003203  test    al, al
10003205  jne     0x10003222
10003207  push    0
10003209  push    0x7d0
1000320e  push    0x1388
10003213  push    0x80
10003218  call    0x10002f00
1000321d  add     esp, 0x10
10003220  jmp     0x1000325c
10003222  push    1
10003224  push    1
10003226  call    0x1000d3b0
1000322b  add     esp, 8
1000322e  test    al, al
10003230  je      0x10003252
10003232  push    0
10003234  call    0x100136a0   ; EXP:McProtect
10003239  add     esp, 4
1000323c  test    al, al
1000323e  je      0x10003252
10003240  call    0x1000d740   ; EXP:FormatImage
10003245  test    al, al
10003247  je      0x10003252
10003249  push    1
1000324b  call    0x100136a0   ; EXP:McProtect
10003250  jmp     0x10003259
10003252  push    0x17
10003254  call    0x10001120
10003259  add     esp, 4
1000325c  push    0
1000325e  call    0x100061b0   ; EXP:TransparentMode
10003263  mov     ecx, dword ptr [0x10068e30]
10003269  add     esp, 4
1000326c  xor     eax, eax
1000326e  test    ecx, ecx
10003270  sete    al
10003273  ret     
