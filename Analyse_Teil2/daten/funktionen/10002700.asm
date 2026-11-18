10002700  push    ecx
10002701  mov     eax, dword ptr [0x10068e30]
10002706  test    eax, eax
10002708  jne     0x10002743
1000270a  lea     eax, [esp + 3]
1000270e  push    0x4b0
10002713  push    eax
10002714  mov     byte ptr [esp + 0xb], 0
10002719  call    0x10001f40   ; EXP:ReadByte
1000271e  mov     eax, dword ptr [0x10068e30]
10002723  add     esp, 8
10002726  test    eax, eax
10002728  jne     0x10002743
1000272a  mov     cl, byte ptr [esp + 3]
1000272e  mov     al, byte ptr [esp + 0xc]
10002732  cmp     cl, al
10002734  je      0x10002743
10002736  mov     edx, dword ptr [esp + 8]
1000273a  push    edx
1000273b  call    0x10001120
10002740  add     esp, 4
10002743  pop     ecx
10002744  ret     
