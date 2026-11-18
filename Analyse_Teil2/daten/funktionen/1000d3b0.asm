1000d3b0  mov     al, byte ptr [0x100670c8]
1000d3b5  test    al, al
1000d3b7  mov     al, byte ptr [0x100426b0]
1000d3bc  jne     0x1000d3c3
1000d3be  test    al, al
1000d3c0  jne     0x1000d3c3
1000d3c2  ret     
1000d3c3  mov     ecx, dword ptr [esp + 8]
1000d3c7  mov     edx, dword ptr [esp + 4]
1000d3cb  neg     al
1000d3cd  push    0x10002e50   ; EXP:WriteComBytes
1000d3d2  push    0x100029b0   ; EXP:ReadComBytes
1000d3d7  sbb     eax, eax
1000d3d9  push    0
1000d3db  push    0
1000d3dd  and     eax, 0xffd80000
1000d3e2  push    0
1000d3e4  push    ecx
1000d3e5  add     eax, 0x280000
1000d3ea  push    edx
1000d3eb  push    eax
1000d3ec  push    0
1000d3ee  call    0x1000d690   ; EXP:InitImage
1000d3f3  add     esp, 0x24
1000d3f6  ret     
