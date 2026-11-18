100041e0  push    ebx
100041e1  push    ebp
100041e2  push    0x7d0
100041e7  call    0x10001230
100041ec  add     esp, 4
100041ef  mov     ebx, eax
100041f1  call    0x10007540   ; EXP:_MakeExclusiveAccess
100041f6  mov     ebp, eax
100041f8  mov     eax, dword ptr [0x1006ae80]
100041fd  test    eax, eax
100041ff  jne     0x10004209
10004201  pop     ebp
10004202  mov     eax, 0xfffffffd
10004207  pop     ebx
10004208  ret     
10004209  push    1
1000420b  call    0x100061b0   ; EXP:TransparentMode
10004210  add     esp, 4
10004213  test    ax, ax
10004216  jne     0x10004234
10004218  test    bx, bx
1000421b  je      0x10004222
1000421d  call    0x10001320
10004222  test    bp, bp
10004225  je      0x1000422c
10004227  call    0x10007570   ; EXP:_ReleaseExclusiveAccess
1000422c  pop     ebp
1000422d  mov     eax, 0xfffffffe
10004232  pop     ebx
10004233  ret     
10004234  push    esi
10004235  call    0x10009360
1000423a  mov     esi, eax
1000423c  dec     esi
1000423d  js      0x1000424b
1000423f  push    esi
10004240  call    0x10008c60
10004245  add     esp, 4
10004248  dec     esi
10004249  jns     0x1000423f
1000424b  call    0x10009360
10004250  test    eax, eax
10004252  je      0x1000425e
10004254  mov     esi, 0xfffffffb
10004259  jmp     0x100042eb
1000425e  call    0x10007500
10004263  test    al, al
10004265  jne     0x100042c4
10004267  mov     al, byte ptr [0x100426b0]
1000426c  test    al, al
1000426e  jne     0x100042c4
10004270  push    edi
10004271  push    0x84
10004276  mov     ecx, 0x10068d70
1000427b  call    0x1000fd80
10004280  xor     edi, edi
10004282  push    0
10004284  push    0
10004286  push    0
10004288  push    edi
10004289  call    0x10004310
1000428e  mov     esi, eax
10004290  mov     eax, dword ptr [0x10068e30]
10004295  add     esp, 0x10
10004298  test    eax, eax
1000429a  mov     al, byte ptr [0x10068d69]
1000429f  jne     0x100042aa
100042a1  test    al, al
100042a3  jne     0x100042aa
100042a5  mov     esi, 1
100042aa  inc     edi
100042ab  test    esi, esi
100042ad  jle     0x100042b7
100042af  xor     ecx, ecx
100042b1  mov     cl, al
100042b3  cmp     edi, ecx
100042b5  jl      0x10004282   ; L'jjj'
100042b7  mov     ecx, 0x10068d70
100042bc  call    0x1000fd60
100042c1  pop     edi
100042c2  jmp     0x100042eb
100042c4  xor     esi, esi
100042c6  push    1
100042c8  push    esi
100042c9  call    0x1000d3b0
100042ce  add     esp, 8
100042d1  test    al, al
100042d3  je      0x100042eb
100042d5  push    esi
100042d6  push    esi
100042d7  call    0x1000d5c0   ; EXP:ExtractImage
100042dc  and     eax, 0xff
100042e1  add     esp, 8
100042e4  mov     esi, eax
100042e6  call    0x1000d470   ; EXP:ReleaseImage
100042eb  push    0
100042ed  call    0x100061b0   ; EXP:TransparentMode
100042f2  add     esp, 4
100042f5  test    bx, bx
100042f8  je      0x100042ff
100042fa  call    0x10001320
100042ff  test    bp, bp
10004302  je      0x10004309
10004304  call    0x10007570   ; EXP:_ReleaseExclusiveAccess
10004309  mov     eax, esi
1000430b  pop     esi
1000430c  pop     ebp
1000430d  pop     ebx
1000430e  ret     
