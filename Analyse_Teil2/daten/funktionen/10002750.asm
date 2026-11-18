10002750  mov     eax, dword ptr [0x10068e30]
10002755  push    esi
10002756  test    eax, eax
10002758  jne     0x1000277e
1000275a  mov     esi, dword ptr [esp + 8]
1000275e  mov     eax, esi
10002760  shr     eax, 4
10002763  push    eax
10002764  call    0x10002780
10002769  mov     eax, dword ptr [0x10068e30]
1000276e  add     esp, 4
10002771  test    eax, eax
10002773  jne     0x1000277e
10002775  push    esi
10002776  call    0x10002780
1000277b  add     esp, 4
1000277e  pop     esi
1000277f  ret     
10002780  mov     eax, dword ptr [0x10068e30]
10002785  test    eax, eax
10002787  jne     0x100027a4
10002789  mov     eax, dword ptr [esp + 4]
1000278d  push    0x3e8
10002792  push    eax
10002793  call    0x100027b0
10002798  add     esp, 4
1000279b  push    eax
1000279c  call    0x100027d0
100027a1  add     esp, 8
100027a4  ret     
