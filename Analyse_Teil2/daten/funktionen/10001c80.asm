10001c80  mov     ecx, dword ptr [0x1007201c]
10001c86  mov     eax, dword ptr [0x100400d8]
10001c8b  test    ecx, ecx
10001c8d  mov     edx, eax
10001c8f  jne     0x10001c94
10001c91  xor     eax, eax
10001c93  ret     
10001c94  mov     ecx, dword ptr [0x10068e34]
10001c9a  test    ecx, ecx
10001c9c  jne     0x10001ca1
10001c9e  xor     eax, eax
10001ca0  ret     
10001ca1  mov     ecx, dword ptr [esp + 4]
10001ca5  test    ecx, ecx
10001ca7  je      0x10001cb0
10001ca9  mov     eax, ecx
10001cab  mov     dword ptr [0x100400d8], eax
10001cb0  cmp     dword ptr [0x100400d4], -1
10001cb7  je      0x10001cbd
10001cb9  cmp     eax, edx
10001cbb  je      0x10001ce3
10001cbd  mov     ecx, dword ptr [esp + 0xc]
10001cc1  mov     edx, dword ptr [esp + 8]
10001cc5  push    ecx
10001cc6  push    edx
10001cc7  push    eax
10001cc8  call    0x10001610
10001ccd  mov     eax, dword ptr [0x10068e30]
10001cd2  add     esp, 0xc
10001cd5  test    eax, eax
10001cd7  jne     0x10001ce3
10001cd9  mov     dword ptr [0x10068e30], 0x15
10001ce3  cmp     dword ptr [0x100400d4], -1
10001cea  je      0x10001cf1
10001cec  call    0x10001d00
10001cf1  mov     ecx, dword ptr [0x10068e30]
10001cf7  xor     eax, eax
10001cf9  test    ecx, ecx
10001cfb  sete    al
10001cfe  ret     
