10007600  mov     eax, dword ptr [esp + 8]
10007604  test    eax, eax
10007606  jl      0x10007635
10007608  cmp     eax, dword ptr [0x1006a6a0]
1000760e  jge     0x10007635
10007610  mov     eax, dword ptr [eax*4 + 0x1006a6a4]
10007617  mov     edx, dword ptr [0x1006ae7c]
1000761d  mov     ecx, dword ptr [esp + 4]
10007621  add     eax, edx
10007623  push    0
10007625  push    eax
10007626  mov     eax, dword ptr [esp + 0x14]
1000762a  push    eax
1000762b  push    ecx
1000762c  call    0x10007590
10007631  add     esp, 0x10
10007634  ret     
10007635  xor     eax, eax
10007637  ret     
