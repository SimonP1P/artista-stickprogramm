100061b0  mov     eax, dword ptr [esp + 4]
100061b4  push    eax
100061b5  call    0x10005980
100061ba  add     esp, 4
100061bd  test    ax, ax
100061c0  je      0x100061d3
100061c2  mov     cl, byte ptr [0x100426b0]
100061c8  test    cl, cl
100061ca  je      0x100061d3
100061cc  mov     byte ptr [0x100670c8], 0
100061d3  ret     
