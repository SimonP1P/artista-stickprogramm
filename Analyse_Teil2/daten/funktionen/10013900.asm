10013900  sub     esp, 0xc
10013903  mov     ecx, dword ptr [esp + 0x14]
10013907  mov     al, byte ptr [esp + 0x10]
1001390b  lea     edx, [esp + 1]
1001390f  push    ecx
10013910  push    edx
10013911  mov     byte ptr [esp + 8], al
10013915  call    0x10013940
1001391a  mov     eax, dword ptr [esp + 0x20]
1001391e  lea     ecx, [esp + 0xd]
10013922  push    eax
10013923  push    ecx
10013924  call    0x10013940
10013929  lea     edx, [esp + 0x10]
1001392d  push    9
1001392f  push    edx
10013930  call    0x10013720
10013935  add     esp, 0x24
10013938  ret     
