10005980  sub     esp, 0x24
10005983  push    ebx
10005984  push    ebp
10005985  push    esi
10005986  push    edi
10005987  call    0x10002330
1000598c  test    ax, ax
1000598f  jne     0x10005999
10005991  pop     edi
10005992  pop     esi
10005993  pop     ebp
10005994  pop     ebx
10005995  add     esp, 0x24
10005998  ret     
10005999  mov     eax, dword ptr [0x10072020]
1000599e  mov     byte ptr [0x10068e88], 0
100059a5  test    eax, eax
100059a7  jne     0x10005de0
100059ad  push    0
100059af  push    0
100059b1  call    0x10005e30
100059b6  add     esp, 8
100059b9  test    ax, ax
100059bc  jne     0x100059c6
100059be  pop     edi
100059bf  pop     esi
100059c0  pop     ebp
100059c1  pop     ebx
100059c2  add     esp, 0x24
100059c5  ret     
100059c6  mov     al, byte ptr [0x100426b0]
100059cb  mov     ebx, 1
100059d0  test    al, al
100059d2  mov     dword ptr [0x10072024], ebx
100059d8  je      0x100059e5
100059da  pop     edi
100059db  pop     esi
100059dc  mov     ax, bx
100059df  pop     ebp
100059e0  pop     ebx
100059e1  add     esp, 0x24
100059e4  ret     
100059e5  lea     eax, [esp + 0x14]
100059e9  push    eax
100059ea  push    0x200100
100059ef  call    0x10002500   ; EXP:Lese32Bytes
100059f4  add     esp, 8
100059f7  test    eax, eax
100059f9  jne     0x10005a06
100059fb  pop     edi
100059fc  pop     esi
100059fd  pop     ebp
100059fe  xor     ax, ax
10005a01  pop     ebx
10005a02  add     esp, 0x24
10005a05  ret     
10005a06  mov     edi, 0x10035528   ; 'SRMV'
10005a0b  or      ecx, 0xffffffff
10005a0e  xor     eax, eax
10005a10  lea     esi, [esp + 0x14]
10005a14  repne scasb al, byte ptr es:[edi]
10005a16  not     ecx
10005a18  dec     ecx
10005a19  mov     byte ptr [esp + ecx + 0x14], al
10005a1d  mov     eax, 0x10035528   ; 'SRMV'
10005a22  mov     dl, byte ptr [eax]
10005a24  mov     cl, dl
10005a26  cmp     dl, byte ptr [esi]
10005a28  jne     0x10005a46
10005a2a  test    cl, cl
10005a2c  je      0x10005a42
10005a2e  mov     dl, byte ptr [eax + 1]
10005a31  mov     cl, dl
10005a33  cmp     dl, byte ptr [esi + 1]
10005a36  jne     0x10005a46
10005a38  add     eax, 2
10005a3b  add     esi, 2
10005a3e  test    cl, cl
10005a40  jne     0x10005a22
10005a42  xor     eax, eax
10005a44  jmp     0x10005a4b
10005a46  sbb     eax, eax
10005a48  sbb     eax, -1
10005a4b  xor     ebp, ebp
10005a4d  cmp     eax, ebp
10005a4f  je      0x10005a59
10005a51  mov     dword ptr [0x10072024], ebp
10005a57  jmp     0x10005a61
10005a59  cmp     dword ptr [0x10072024], ebp
10005a5f  jne     0x10005a78
10005a61  push    ebx
10005a62  push    ebx
10005a63  call    0x10005e30
10005a68  add     esp, 8
10005a6b  test    ax, ax
10005a6e  jne     0x10005a78
10005a70  pop     edi
10005a71  pop     esi
10005a72  pop     ebp
10005a73  pop     ebx
10005a74  add     esp, 0x24
10005a77  ret     
10005a78  lea     eax, [esp + 0x14]
10005a7c  push    eax
10005a7d  push    0x200100
10005a82  call    0x10002500   ; EXP:Lese32Bytes
10005a87  add     esp, 8
10005a8a  test    eax, eax
10005a8c  jne     0x10005a99
10005a8e  pop     edi
10005a8f  pop     esi
10005a90  pop     ebp
10005a91  xor     ax, ax
10005a94  pop     ebx
10005a95  add     esp, 0x24
10005a98  ret     
10005a99  mov     edi, 0x10035528   ; 'SRMV'
10005a9e  or      ecx, 0xffffffff
10005aa1  xor     eax, eax
10005aa3  repne scasb al, byte ptr es:[edi]
10005aa5  not     ecx
10005aa7  dec     ecx
10005aa8  mov     byte ptr [esp + ecx + 0x19], al
10005aac  mov     al, byte ptr [esp + ecx + 0x16]
10005ab0  cmp     al, 0x2e
10005ab2  jne     0x10005b2c
10005ab4  mov     al, byte ptr [esp + ecx + 0x14]
10005ab8  lea     ecx, [esp + ecx + 0x14]
10005abc  test    al, al
10005abe  mov     dword ptr [0x10068e2c], ebp
10005ac4  je      0x10005aec
10005ac6  cmp     al, 0x30
10005ac8  jl      0x10005ade
10005aca  mov     al, byte ptr [ecx]
10005acc  cmp     al, 0x39
10005ace  jg      0x10005ade
10005ad0  add     ebp, 0xffffffd
10005ad6  movsx   edx, al
10005ad9  shl     ebp, 4
10005adc  add     ebp, edx
10005ade  mov     al, byte ptr [ecx + 1]
10005ae1  inc     ecx
10005ae2  test    al, al
10005ae4  jne     0x10005ac6
10005ae6  mov     dword ptr [0x10068e2c], ebp
10005aec  mov     edi, 0x10035528   ; 'SRMV'
10005af1  or      ecx, 0xffffffff
10005af4  xor     eax, eax
10005af6  lea     esi, [esp + 0x14]
10005afa  repne scasb al, byte ptr es:[edi]
10005afc  not     ecx
10005afe  dec     ecx
10005aff  mov     byte ptr [esp + ecx + 0x14], al
10005b03  mov     eax, 0x10035528   ; 'SRMV'
10005b08  mov     dl, byte ptr [eax]
10005b0a  mov     cl, dl
10005b0c  cmp     dl, byte ptr [esi]
10005b0e  jne     0x10005b34
10005b10  test    cl, cl
10005b12  je      0x10005b28
10005b14  mov     dl, byte ptr [eax + 1]
10005b17  mov     cl, dl
10005b19  cmp     dl, byte ptr [esi + 1]
10005b1c  jne     0x10005b34
10005b1e  add     eax, 2
10005b21  add     esi, 2
10005b24  test    cl, cl
10005b26  jne     0x10005b08
10005b28  xor     eax, eax
10005b2a  jmp     0x10005b39
10005b2c  mov     ebp, dword ptr [0x10068e2c]
10005b32  jmp     0x10005aec
10005b34  sbb     eax, eax
10005b36  sbb     eax, -1
10005b39  xor     ecx, ecx
10005b3b  test    eax, eax
10005b3d  sete    cl
10005b40  mov     esi, ecx
10005b42  test    esi, esi
10005b44  je      0x10005b81
10005b46  test    ebp, ebp
10005b48  jne     0x10005b50
10005b4a  mov     dword ptr [0x10068e2c], ebx
10005b50  lea     edx, [esp + 0x14]
10005b54  xor     ebp, ebp
10005b56  push    edx
10005b57  push    0xfffd24
10005b5c  call    0x10002500   ; EXP:Lese32Bytes
10005b61  add     esp, 8
10005b64  test    eax, eax
10005b66  je      0x10005bf1
10005b6c  mov     eax, 0xfffd24
10005b71  mov     ebx, 0x1e
10005b76  sub     eax, 0x10072028
10005b7b  mov     dword ptr [esp + 0x10], eax
10005b7f  jmp     0x10005b94
10005b81  mov     dword ptr [0x10068e2c], 0
10005b8b  jmp     0x10005d75
10005b90  mov     eax, dword ptr [esp + 0x10]
10005b94  mov     ecx, 0x1e
10005b99  sub     ecx, ebp
10005b9b  cmp     ecx, 0x20
10005b9e  jae     0x10005bbc
10005ba0  mov     ecx, ebx
10005ba2  lea     esi, [esp + 0x14]
10005ba6  mov     edx, ecx
10005ba8  lea     edi, [ebp + 0x10072028]
10005bae  shr     ecx, 2
10005bb1  rep movsd dword ptr es:[edi], dword ptr [esi]
10005bb3  mov     ecx, edx
10005bb5  and     ecx, 3
10005bb8  rep movsb byte ptr es:[edi], byte ptr [esi]
10005bba  jmp     0x10005bcd
10005bbc  lea     edi, [ebp + 0x10072028]
10005bc2  mov     ecx, 8
10005bc7  lea     esi, [esp + 0x14]
10005bcb  rep movsd dword ptr es:[edi], dword ptr [esi]
10005bcd  add     ebp, 0x20
10005bd0  sub     ebx, 0x20
10005bd3  cmp     ebp, 0x1e
10005bd6  jae     0x10005bfc
10005bd8  lea     ecx, [esp + 0x14]
10005bdc  lea     edx, [eax + ebp + 0x10072028]
10005be3  push    ecx
10005be4  push    edx
10005be5  call    0x10002500   ; EXP:Lese32Bytes
10005bea  add     esp, 8
10005bed  test    eax, eax
10005bef  jne     0x10005b90
10005bf1  pop     edi
10005bf2  pop     esi
10005bf3  pop     ebp
10005bf4  xor     ax, ax
10005bf7  pop     ebx
10005bf8  add     esp, 0x24
10005bfb  ret     
10005bfc  mov     eax, dword ptr [0x10072028]
10005c01  cmp     al, 0x1c
10005c03  ja      0x10005c1e
10005c05  cmp     ah, 0x3c
10005c08  ja      0x10005c1e
10005c0a  push    0x1007202a
10005c0f  call    0x10003dd0
10005c14  add     esp, 4
10005c17  cmp     eax, 0x249f0
10005c1c  jbe     0x10005c25
10005c1e  xor     esi, esi
10005c20  jmp     0x10005d75
10005c25  call    0x100074d0
10005c2a  test    al, al
10005c2c  je      0x10005c83
10005c2e  push    0x1007203e
10005c33  call    0x10003dd0
10005c38  push    0x1007202a
10005c3d  mov     esi, eax
10005c3f  call    0x10003dd0
10005c44  add     esp, 8
10005c47  cmp     eax, esi
10005c49  ja      0x10005c7c
10005c4b  push    0x10072042
10005c50  call    0x10003dd0
10005c55  push    0x1007203e
10005c5a  mov     esi, eax
10005c5c  call    0x10003dd0
10005c61  add     esp, 8
10005c64  cmp     eax, esi
10005c66  ja      0x10005c7c
10005c68  push    0x10072042
10005c6d  call    0x10003dd0
10005c72  add     esp, 4
10005c75  cmp     eax, 0x249f0
10005c7a  jbe     0x10005c83
10005c7c  xor     esi, esi
10005c7e  jmp     0x10005d75
10005c83  mov     eax, dword ptr [0x10072028]
10005c88  push    0x1007202a
10005c8d  and     eax, 0xff
10005c92  mov     dword ptr [0x10072048], 0
10005c9c  lea     ecx, [eax + eax]
10005c9f  lea     edx, [eax*4]
10005ca6  lea     eax, [eax + eax*2]
10005ca9  mov     dword ptr [0x1007204c], ecx
10005caf  shl     eax, 1
10005cb1  mov     dword ptr [0x10072054], eax
10005cb6  add     eax, 4
10005cb9  mov     dword ptr [0x10072058], eax
10005cbe  add     eax, 4
10005cc1  mov     dword ptr [0x10072050], edx
10005cc7  mov     dword ptr [0x1007205c], eax
10005ccc  call    0x10003dd0
10005cd1  mov     edx, dword ptr [0x1007205c]
10005cd7  add     esp, 4
10005cda  add     eax, edx
10005cdc  cmp     eax, 0x249f0
10005ce1  sbb     esi, esi
10005ce3  neg     esi
10005ce5  je      0x10005d75
10005ceb  cmp     dword ptr [0x10068e2c], 0x309
10005cf5  jb      0x10005d67
10005cf7  lea     eax, [esp + 0x14]
10005cfb  push    4
10005cfd  push    eax
10005cfe  push    0x24044
10005d03  call    0x100029b0   ; EXP:ReadComBytes
10005d08  mov     esi, eax
10005d0a  add     esp, 0xc
10005d0d  test    esi, esi
10005d0f  je      0x10005d75
10005d11  lea     ecx, [esp + 0x14]
10005d15  push    ecx
10005d16  call    0x10003dd0
10005d1b  push    0x12
10005d1d  push    0x10072060
10005d22  push    eax
10005d23  call    0x100029b0   ; EXP:ReadComBytes
10005d28  mov     esi, eax
10005d2a  add     esp, 0x10
10005d2d  test    esi, esi
10005d2f  je      0x10005d75
10005d31  lea     edx, [esp + 0x14]
10005d35  push    4
10005d37  push    edx
10005d38  push    0x24004
10005d3d  call    0x100029b0   ; EXP:ReadComBytes
10005d42  mov     esi, eax
10005d44  add     esp, 0xc
10005d47  test    esi, esi
10005d49  je      0x10005d75
10005d4b  lea     eax, [esp + 0x14]
10005d4f  push    eax
10005d50  call    0x10003dd0
10005d55  push    0xb
10005d57  push    0x10072078
10005d5c  push    eax
10005d5d  call    0x100029b0   ; EXP:ReadComBytes
10005d62  add     esp, 0x10
10005d65  mov     esi, eax
10005d67  test    esi, esi
10005d69  je      0x10005d75
10005d6b  call    0x10004150
10005d70  mov     dword ptr [0x10072084], eax
10005d75  mov     eax, dword ptr [0x10072024]
10005d7a  test    eax, eax
10005d7c  jne     0x10005d9f
10005d7e  cmp     word ptr [esp + 0x38], 0
10005d84  jne     0x10005d9f
10005d86  push    1
10005d88  push    0
10005d8a  call    0x10005e30
10005d8f  add     esp, 8
10005d92  test    ax, ax
10005d95  jne     0x10005d9f
10005d97  pop     edi
10005d98  pop     esi
10005d99  pop     ebp
10005d9a  pop     ebx
10005d9b  add     esp, 0x24
10005d9e  ret     
10005d9f  test    esi, esi
10005da1  je      0x10005dcb
10005da3  mov     eax, dword ptr [0x10072024]
10005da8  mov     dword ptr [0x10072020], 1
10005db2  test    eax, eax
10005db4  jne     0x10005de9
10005db6  mov     ecx, dword ptr [0x10068e30]
10005dbc  xor     eax, eax
10005dbe  pop     edi
10005dbf  pop     esi
10005dc0  test    ecx, ecx
10005dc2  pop     ebp
10005dc3  pop     ebx
10005dc4  sete    al
10005dc7  add     esp, 0x24
10005dca  ret     
10005dcb  push    0x12
10005dcd  call    0x10001120
10005dd2  add     esp, 4
10005dd5  xor     ax, ax
10005dd8  pop     edi
10005dd9  pop     esi
10005dda  pop     ebp
10005ddb  pop     ebx
10005ddc  add     esp, 0x24
10005ddf  ret     
10005de0  mov     eax, dword ptr [0x10072024]
10005de5  test    eax, eax
10005de7  je      0x10005e10
10005de9  mov     esi, dword ptr [esp + 0x38]
10005ded  test    si, si
10005df0  je      0x10005dff
10005df2  push    0x1c200
10005df7  call    0x10005950   ; EXP:ChangeBaudrate
10005dfc  add     esp, 4
10005dff  push    esi
10005e00  call    0x10006070
10005e05  add     esp, 4
10005e08  pop     edi
10005e09  pop     esi
10005e0a  pop     ebp
10005e0b  pop     ebx
10005e0c  add     esp, 0x24
10005e0f  ret     
10005e10  mov     ecx, dword ptr [esp + 0x38]
10005e14  push    1
10005e16  push    ecx
10005e17  call    0x10005e30
10005e1c  add     esp, 8
10005e1f  pop     edi
10005e20  pop     esi
10005e21  pop     ebp
10005e22  pop     ebx
10005e23  add     esp, 0x24
10005e26  ret     
