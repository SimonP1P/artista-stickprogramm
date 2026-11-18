100027d0  push    ecx
100027d1  mov     eax, dword ptr [0x10068e34]
100027d6  mov     byte ptr [esp + 3], 0
100027db  test    eax, eax
100027dd  jne     0x100027e3
100027df  xor     eax, eax
100027e1  pop     ecx
100027e2  ret     
100027e3  mov     al, byte ptr [0x100426b0]
100027e8  test    al, al
100027ea  je      0x100027f6
100027ec  push    0xc
100027ee  call    0x10001120
100027f3  add     esp, 4
100027f6  mov     eax, dword ptr [0x10068e30]
100027fb  test    eax, eax
100027fd  je      0x10002803
100027ff  xor     eax, eax
10002801  pop     ecx
10002802  ret     
10002803  call    0x100018c0   ; EXP:ReadFromCOM
10002808  cmp     eax, 0x1000
1000280d  je      0x1000282c
1000280f  cmp     al, 0x20
10002811  jb      0x10002803
10002813  cmp     al, 0x7f
10002815  ja      0x10002803
10002817  cmp     eax, 0x1000
1000281c  je      0x1000282c
1000281e  push    0xe
10002820  call    0x10001120
10002825  add     esp, 4
10002828  xor     eax, eax
1000282a  pop     ecx
1000282b  ret     
1000282c  push    ebx
1000282d  mov     ebx, dword ptr [esp + 0xc]
10002831  push    ebx
10002832  call    0x10001a20   ; EXP:WriteToCOM
10002837  mov     eax, dword ptr [esp + 0x14]
1000283b  lea     ecx, [esp + 0xb]
1000283f  push    eax
10002840  push    ecx
10002841  call    0x10001f40   ; EXP:ReadByte
10002846  mov     eax, dword ptr [0x10068e30]
1000284b  add     esp, 0xc
1000284e  test    eax, eax
10002850  jne     0x10002862
10002852  cmp     bl, byte ptr [esp + 7]
10002856  je      0x10002862
10002858  push    0xf
1000285a  call    0x10001120
1000285f  add     esp, 4
10002862  mov     ecx, dword ptr [0x10068e30]
10002868  xor     eax, eax
1000286a  test    ecx, ecx
1000286c  sete    al
1000286f  pop     ebx
10002870  pop     ecx
10002871  ret     
