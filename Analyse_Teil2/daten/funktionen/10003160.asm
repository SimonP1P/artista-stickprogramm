10003160  sub     esp, 8
10003163  lea     eax, [esp]
10003167  push    2
10003169  push    eax
1000316a  push    0xfffed0
1000316f  call    0x100029b0   ; EXP:ReadComBytes
10003174  add     esp, 0xc
10003177  test    eax, eax
10003179  je      0x100031a3
1000317b  mov     ecx, dword ptr [esp]
1000317f  mov     edx, dword ptr [esp + 1]
10003183  mov     eax, dword ptr [esp + 0xc]
10003187  and     ecx, 0xff
1000318d  shl     ecx, 8
10003190  and     edx, 0xff
10003196  add     ecx, edx
10003198  mov     dword ptr [eax], ecx
1000319a  mov     eax, 1
1000319f  add     esp, 8
100031a2  ret     
100031a3  xor     eax, eax
100031a5  add     esp, 8
100031a8  ret     
