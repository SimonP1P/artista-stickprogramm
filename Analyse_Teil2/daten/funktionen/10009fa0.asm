10009fa0  mov     cl, byte ptr [0x100426b0]
10009fa6  push    ebx
10009fa7  test    cl, cl
10009fa9  push    edi
10009faa  je      0x10009fc7
10009fac  cmp     dword ptr [0x10068eb4], 0xf
10009fb3  jl      0x10009fc7
10009fb5  push    0
10009fb7  push    0x19
10009fb9  call    0x1000f870
10009fbe  add     esp, 8
10009fc1  xor     ax, ax
10009fc4  pop     edi
10009fc5  pop     ebx
10009fc6  ret     
10009fc7  mov     ebx, dword ptr [esp + 0xc]
10009fcb  mov     eax, ebx
10009fcd  sub     eax, 0
10009fd0  je      0x10009ff9
10009fd2  dec     eax
10009fd3  je      0x10009fea
10009fd5  sub     eax, 2
10009fd8  je      0x10009fe0
10009fda  pop     edi
10009fdb  xor     ax, ax
10009fde  pop     ebx
10009fdf  ret     
10009fe0  test    cl, cl
10009fe2  je      0x1000a01f
10009fe4  pop     edi
10009fe5  xor     ax, ax
10009fe8  pop     ebx
10009fe9  ret     
10009fea  mov     al, byte ptr [0x100670c8]
10009fef  test    al, al
10009ff1  jne     0x1000a008
10009ff3  pop     edi
10009ff4  xor     ax, ax
10009ff7  pop     ebx
10009ff8  ret     
10009ff9  mov     al, byte ptr [0x100670c8]
10009ffe  test    al, al
1000a000  je      0x1000a008
1000a002  pop     edi
1000a003  xor     ax, ax
1000a006  pop     ebx
1000a007  ret     
1000a008  test    cl, cl
1000a00a  jne     0x1000a01f
1000a00c  test    al, al
1000a00e  jne     0x1000a01f
1000a010  test    byte ptr [0x1006ae84], 8
1000a017  jne     0x1000a01f
1000a019  pop     edi
1000a01a  xor     ax, ax
1000a01d  pop     ebx
1000a01e  ret     
1000a01f  call    0x10007540   ; EXP:_MakeExclusiveAccess
1000a024  mov     edi, eax
1000a026  mov     eax, dword ptr [0x1006ae80]
1000a02b  test    eax, eax
1000a02d  jne     0x1000a035
1000a02f  pop     edi
1000a030  xor     ax, ax
1000a033  pop     ebx
1000a034  ret     
1000a035  mov     eax, dword ptr [esp + 0x10]
1000a039  push    esi
1000a03a  push    eax
1000a03b  call    0x10008e70
1000a040  mov     esi, eax
1000a042  add     esp, 4
1000a045  neg     esi
1000a047  sbb     esi, esi
1000a049  inc     esi
1000a04a  test    si, si
1000a04d  je      0x1000a0ab
1000a04f  cmp     ebx, 3
1000a052  push    1
1000a054  je      0x1000a083
1000a056  push    1
1000a058  call    0x10009360
1000a05d  dec     eax
1000a05e  push    eax
1000a05f  call    0x100032b0
1000a064  add     esp, 0xc
1000a067  test    ax, ax
1000a06a  jbe     0x1000a09c
1000a06c  call    0x10009360
1000a071  dec     eax
1000a072  push    eax
1000a073  call    0x100092e0
1000a078  add     esp, 4
1000a07b  test    eax, eax
1000a07d  jne     0x1000a09c
1000a07f  xor     esi, esi
1000a081  jmp     0x1000a09c
1000a083  call    0x10009360
1000a088  dec     eax
1000a089  push    eax
1000a08a  call    0x10006e90
1000a08f  mov     si, ax
1000a092  add     esp, 8
1000a095  neg     si
1000a098  sbb     esi, esi
1000a09a  neg     esi
1000a09c  call    0x10009360
1000a0a1  dec     eax
1000a0a2  push    eax
1000a0a3  call    0x10008c60
1000a0a8  add     esp, 4
1000a0ab  test    di, di
1000a0ae  je      0x1000a0b5
1000a0b0  call    0x10007570   ; EXP:_ReleaseExclusiveAccess
1000a0b5  mov     ax, si
1000a0b8  pop     esi
1000a0b9  pop     edi
1000a0ba  pop     ebx
1000a0bb  ret     
