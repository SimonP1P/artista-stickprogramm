10003130  mov     eax, dword ptr [esp + 4]
10003134  mov     edx, dword ptr [esp + 8]
10003138  mov     ecx, eax
1000313a  mov     byte ptr [esp + 5], al
1000313e  push    edx
1000313f  lea     eax, [esp + 8]
10003143  push    2
10003145  push    eax
10003146  sar     ecx, 8
10003149  push    0xfffed0
1000314e  mov     byte ptr [esp + 0x14], cl
10003152  call    0x10002b20
10003157  add     esp, 0x10
1000315a  ret     
