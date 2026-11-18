100064c0  mov     eax, dword ptr fs:[0]
100064c6  push    -1
100064c8  push    0x10032d76
100064cd  push    eax
100064ce  mov     ecx, dword ptr [esp + 0x10]
100064d2  mov     dword ptr fs:[0], esp
100064d9  sub     esp, 0x3d4
100064df  push    ebx
100064e0  push    ebp
100064e1  push    esi
100064e2  mov     bl, 0xc
100064e4  push    edi
100064e5  mov     byte ptr [0x1006ae84], bl
100064eb  call    0x10026d60
100064f0  xor     esi, esi
100064f2  mov     ebp, 1
100064f7  cmp     word ptr [esp + 0x3f8], si
100064ff  mov     dword ptr [0x10068e80], esi
10006505  mov     dword ptr [0x10068e84], esi
1000650b  jne     0x10006801
10006511  cmp     word ptr [esp + 0x400], si
10006519  mov     ecx, 0x6c0
1000651e  mov     eax, 0x8080808
10006523  mov     edi, 0x10067204
10006528  rep stosd dword ptr es:[edi], eax
1000652a  je      0x10006557
1000652c  call    0x10002330
10006531  test    ax, ax
10006534  jne     0x1000653e
10006536  or      ebp, 0xffffffff
10006539  jmp     0x10006866
1000653e  push    1
10006540  call    0x100061b0   ; EXP:TransparentMode
10006545  add     esp, 4
10006548  test    ax, ax
1000654b  jne     0x10006557
1000654d  mov     ebp, 0xfffffffe
10006552  jmp     0x10006866
10006557  call    0x10007500
1000655c  test    al, al
1000655e  jne     0x10006751
10006564  mov     al, byte ptr [0x100426b0]
10006569  test    al, al
1000656b  jne     0x10006751
10006571  mov     eax, dword ptr [esp + 0x3fc]
10006578  push    0
1000657a  push    eax
1000657b  call    0x10004d10
10006580  mov     bp, ax
10006583  add     esp, 8
10006586  and     ebp, 0xffff
1000658c  jle     0x1000672b
10006592  mov     ecx, dword ptr [0x1004087c]
10006598  mov     dword ptr [esp + 0x20], ecx
1000659c  mov     eax, dword ptr [0x10072028]
100065a1  xor     edx, edx
100065a3  mov     dl, byte ptr [0x10068d69]
100065a9  and     eax, 0xff
100065ae  mov     esi, edx
100065b0  mov     dword ptr [esp + 0x3ec], 0
100065bb  lea     ecx, [eax + eax + 1]
100065bf  lea     edx, [eax + eax*2 + 1]
100065c3  mov     dword ptr [esp + 0x10], esi
100065c7  mov     dword ptr [esp + 0x1c], ecx
100065cb  mov     dword ptr [esp + 0x18], edx
100065cf  mov     dword ptr [esp + 0x14], 1
100065d7  xor     ebx, ebx
100065d9  call    0x10004080
100065de  push    ebx
100065df  call    0x10004ce0
100065e4  add     esp, 4
100065e7  test    ebx, ebx
100065e9  je      0x1000663c
100065eb  cmp     ebx, 1
100065ee  jne     0x1000660f
100065f0  push    ebx
100065f1  push    0xfa0
100065f6  push    0x3e8
100065fb  push    0x60
100065fd  call    0x10002f00
10006602  add     esp, 0x10
10006605  test    eax, eax
10006607  je      0x10006712
1000660d  jmp     0x1000663c
1000660f  cmp     ebx, 0x100
10006615  jae     0x10006712
1000661b  push    1
1000661d  push    0xfa0
10006622  push    0x3e8
10006627  push    0xc0
1000662c  call    0x10002f00
10006631  add     esp, 0x10
10006634  test    eax, eax
10006636  je      0x10006712
1000663c  lea     eax, [ebx + ebx*2]
1000663f  push    0x1b
10006641  lea     eax, [eax + eax*8 + 0x10067204]
10006648  push    eax
10006649  push    0x1007202e
1000664e  call    0x10003dd0
10006653  mov     edi, dword ptr [esp + 0x28]
10006657  add     esp, 4
1000665a  add     eax, edi
1000665c  push    eax
1000665d  call    0x100029b0   ; EXP:ReadComBytes
10006662  mov     ebp, eax
10006664  add     esp, 0xc
10006667  test    ebp, ebp
10006669  jle     0x100066a1
1000666b  cmp     esi, 0x1b
1000666e  mov     eax, 0x1b
10006673  jg      0x10006677
10006675  mov     eax, esi
10006677  shl     eax, 5
1000667a  lea     ecx, [esp + 0x84]
10006681  push    eax
10006682  push    ecx
10006683  push    0x1007202e
10006688  call    0x10003dd0
1000668d  mov     edi, dword ptr [esp + 0x24]
10006691  add     esp, 4
10006694  add     eax, edi
10006696  push    eax
10006697  call    0x100029b0   ; EXP:ReadComBytes
1000669c  add     esp, 0xc
1000669f  mov     ebp, eax
100066a1  xor     edi, edi
100066a3  test    ebp, ebp
100066a5  jle     0x100066fe
100066a7  lea     esi, [esp + 0x84]
100066ae  mov     eax, dword ptr [esp + 0x10]
100066b2  cmp     eax, 0x1b
100066b5  jle     0x100066bc
100066b7  mov     eax, 0x1b
100066bc  cmp     edi, eax
100066be  jge     0x100066fa
100066c0  test    byte ptr [0x1006ae84], 4
100066c7  jne     0x100066e3
100066c9  mov     ecx, dword ptr [esp + 0x14]
100066cd  mov     eax, ecx
100066cf  inc     ecx
100066d0  push    eax
100066d1  push    0x100400e4
100066d6  push    esi
100066d7  mov     dword ptr [esp + 0x20], ecx
100066db  call    0x1001c006
100066e0  add     esp, 0xc
100066e3  mov     ecx, dword ptr [esp + 0x3f4]
100066ea  push    esi
100066eb  mov     byte ptr [esi + 0x1f], 0
100066ef  call    0x10026e29
100066f4  inc     edi
100066f5  add     esi, 0x20
100066f8  jmp     0x100066ae
100066fa  mov     esi, dword ptr [esp + 0x10]
100066fe  sub     esi, 0x1b
10006701  inc     ebx
10006702  test    esi, esi
10006704  mov     dword ptr [esp + 0x10], esi
10006708  jle     0x10006712
1000670a  test    ebp, ebp
1000670c  jg      0x100065de
10006712  lea     ecx, [esp + 0x20]
10006716  mov     dword ptr [esp + 0x3ec], 0xffffffff
10006721  call    0x10027f41
10006726  jmp     0x100067d3
1000672b  mov     eax, dword ptr [0x10068e30]
10006730  test    eax, eax
10006732  je      0x1000673d
10006734  cmp     eax, 0x16
10006737  je      0x100067d3
1000673d  mov     ebp, 1
10006742  mov     dword ptr [0x10068e30], 0
1000674c  jmp     0x100067dc
10006751  push    1
10006753  push    0
10006755  call    0x1000d3b0
1000675a  add     esp, 8
1000675d  test    al, al
1000675f  jne     0x10006765
10006761  xor     ebp, ebp
10006763  jmp     0x100067d3
10006765  mov     byte ptr [0x1006ae84], 0
1000676c  call    0x1000d750   ; EXP:CardType
10006771  cmp     al, 1
10006773  je      0x1000677e
10006775  call    0x1000d750   ; EXP:CardType
1000677a  cmp     al, 5
1000677c  jb      0x10006784
1000677e  mov     byte ptr [0x1006ae84], bl
10006784  xor     esi, esi
10006786  call    0x1000d6e0   ; EXP:ImageDesignCount
1000678b  cmp     esi, eax
1000678d  jge     0x100067d3
1000678f  lea     edx, [esp + 0x44]
10006793  lea     eax, [esp + 0x24]
10006797  push    edx
10006798  push    0x20
1000679a  push    eax
1000679b  push    esi
1000679c  call    0x1000d700   ; EXP:ReadImageName
100067a1  add     esp, 0x10
100067a4  lea     ecx, [esp + 0x24]
100067a8  and     eax, 0xff
100067ad  push    ecx
100067ae  mov     ecx, dword ptr [esp + 0x3f8]
100067b5  mov     ebp, eax
100067b7  call    0x10026e29
100067bc  cmp     esi, 0x1b00
100067c2  jae     0x100067ce
100067c4  mov     dl, byte ptr [esp + 0x52]
100067c8  mov     byte ptr [esi + 0x10067204], dl
100067ce  inc     esi
100067cf  test    ebp, ebp
100067d1  jne     0x10006786
100067d3  mov     eax, dword ptr [0x10068e30]
100067d8  test    eax, eax
100067da  jne     0x100067ea
100067dc  mov     al, byte ptr [0x100670c8]
100067e1  test    al, al
100067e3  je      0x100067ea
100067e5  call    0x10004120
100067ea  cmp     word ptr [esp + 0x400], 0
100067f3  je      0x10006866
100067f5  push    0
100067f7  call    0x100061b0   ; EXP:TransparentMode
100067fc  add     esp, 4
100067ff  jmp     0x10006866
10006801  or      edi, 0xffffffff
10006804  call    0x10009360
10006809  cmp     esi, eax
1000680b  jge     0x10006866
1000680d  mov     eax, dword ptr [0x1004087c]
10006812  mov     dword ptr [esp + 0x10], eax
10006816  push    esi
10006817  mov     dword ptr [esp + 0x3f0], 1
10006822  call    0x100092e0
10006827  lea     ecx, [esp + 0x14]
1000682b  push    eax
1000682c  push    ecx
1000682d  call    0x10006340
10006832  mov     bp, ax
10006835  add     esp, 0xc
10006838  and     ebp, 0xffff
1000683e  jle     0x10006851
10006840  mov     ecx, dword ptr [esp + 0x3f4]
10006847  lea     edx, [esp + 0x10]
1000684b  push    edx
1000684c  call    0x10026e6c
10006851  lea     ecx, [esp + 0x10]
10006855  mov     dword ptr [esp + 0x3ec], edi
1000685c  call    0x10027f41
10006861  inc     esi
10006862  test    ebp, ebp
10006864  jg      0x10006804
10006866  mov     eax, dword ptr [0x10068e30]
1000686b  test    eax, eax
1000686d  jne     0x1000687a
1000686f  test    ebp, ebp
10006871  jle     0x1000687a
10006873  mov     eax, 1
10006878  jmp     0x1000687c
1000687a  xor     eax, eax
1000687c  mov     ecx, dword ptr [esp + 0x3e4]
10006883  pop     edi
10006884  pop     esi
10006885  pop     ebp
10006886  pop     ebx
10006887  mov     dword ptr fs:[0], ecx
1000688e  add     esp, 0x3e0
10006894  ret     
