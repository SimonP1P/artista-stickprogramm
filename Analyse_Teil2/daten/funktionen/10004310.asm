10004310  mov     eax, 0x201dc
10004315  call    0x1001c0b0
1000431a  push    ebx
1000431b  push    ebp
1000431c  push    esi
1000431d  xor     ebx, ebx
1000431f  push    edi
10004320  mov     dword ptr [esp + 0x28], ebx
10004324  call    0x10007500
10004329  test    al, al
1000432b  jne     0x10004c45
10004331  mov     al, byte ptr [0x100426b0]
10004336  test    al, al
10004338  jne     0x10004c45
1000433e  mov     di, word ptr [esp + 0x201f4]
10004346  test    di, di
10004349  jne     0x1000436a
1000434b  push    0x84
10004350  mov     ecx, 0x10068d70
10004355  call    0x1000fd80
1000435a  mov     ebx, eax
1000435c  mov     dword ptr [0x10068dcc], 1
10004366  mov     dword ptr [esp + 0x28], ebx
1000436a  mov     esi, dword ptr [esp + 0x201f0]
10004371  mov     byte ptr [0x1006ae84], 0xc
10004378  push    esi
10004379  push    0
1000437b  call    0x10004d10
10004380  mov     eax, dword ptr [0x10068e30]
10004385  add     esp, 8
10004388  test    eax, eax
1000438a  jne     0x10004c36
10004390  call    0x10004080
10004395  mov     eax, dword ptr [0x10068e30]
1000439a  test    eax, eax
1000439c  jne     0x10004c36
100043a2  xor     eax, eax
100043a4  mov     al, byte ptr [0x10068d69]
100043a9  cmp     esi, eax
100043ab  jge     0x10004c36
100043b1  mov     ecx, dword ptr [0x10072020]
100043b7  test    ecx, ecx
100043b9  je      0x10004c36
100043bf  test    di, di
100043c2  jne     0x100043d8
100043c4  test    bx, bx
100043c7  jne     0x100043d8
100043c9  test    esi, esi
100043cb  jne     0x100043d8
100043cd  push    eax
100043ce  mov     ecx, 0x10068d70
100043d3  call    0x1000ff00
100043d8  push    esi
100043d9  call    0x10004c90
100043de  push    0
100043e0  push    0xfa0
100043e5  push    0xbb8
100043ea  push    0x400
100043ef  call    0x10002f00
100043f4  mov     eax, dword ptr [0x10068e30]
100043f9  add     esp, 0x14
100043fc  test    eax, eax
100043fe  jne     0x10004c0c
10004404  mov     eax, dword ptr [0x1007205c]
10004409  push    eax
1000440a  push    0x100426d8
1000440f  push    0x1007203a
10004414  call    0x10003dd0
10004419  add     esp, 4
1000441c  push    eax
1000441d  call    0x100029b0   ; EXP:ReadComBytes
10004422  mov     eax, dword ptr [0x10068e30]
10004427  add     esp, 0xc
1000442a  test    eax, eax
1000442c  jne     0x10004c0c
10004432  lea     ecx, [esp + 0x18]
10004436  push    6
10004438  push    ecx
10004439  push    0x10072036
1000443e  call    0x10003dd0
10004443  add     esp, 4
10004446  push    eax
10004447  call    0x100029b0   ; EXP:ReadComBytes
1000444c  movsx   cx, byte ptr [esp + 0x26]
10004452  movsx   ax, byte ptr [esp + 0x27]
10004458  and     eax, 0xffff
1000445d  and     ecx, 0xffff
10004463  imul    eax, ecx
10004466  add     eax, 0xa
10004469  add     esp, 0xc
1000446c  mov     dword ptr [esp + 0x10], eax
10004470  mov     eax, dword ptr [0x10068e30]
10004475  test    eax, eax
10004477  jne     0x10004c0c
1000447d  mov     edx, dword ptr [0x10072054]
10004483  lea     eax, [edx + 0x100426d8]
10004489  push    eax
1000448a  call    0x10003dd0
1000448f  mov     ecx, dword ptr [0x10072058]
10004495  mov     ebp, eax
10004497  lea     edx, [ecx + 0x100426d8]
1000449d  push    edx
1000449e  call    0x10003dd0
100044a3  add     esp, 8
100044a6  mov     ebx, eax
100044a8  test    di, di
100044ab  jne     0x10004527
100044ad  push    0x1007202a
100044b2  call    0x10003dd0
100044b7  add     esp, 4
100044ba  mov     esi, eax
100044bc  call    0x100074d0
100044c1  test    al, al
100044c3  je      0x100044e5
100044c5  push    0x10072042
100044ca  call    0x10003dd0
100044cf  add     esp, 4
100044d2  cmp     esi, eax
100044d4  jae     0x100044e5
100044d6  push    0x10072042
100044db  call    0x10003dd0
100044e0  add     esp, 4
100044e3  mov     esi, eax
100044e5  cmp     word ptr [esp + 0x201fc], 0
100044ee  je      0x10004506
100044f0  mov     ecx, 0x10068d70
100044f5  call    0x1000ff30
100044fa  push    1
100044fc  mov     ecx, 0x10068d70
10004501  call    0x1000ff00
10004506  cmp     ebp, 2
10004509  jle     0x1000451d
1000450b  mov     ecx, dword ptr [esp + 0x10]
1000450f  lea     eax, [ebx + ebp]
10004512  lea     edx, [eax + ecx + 0x142]
10004519  cmp     edx, esi
1000451b  jl      0x10004527
1000451d  push    0x13
1000451f  call    0x10001120
10004524  add     esp, 4
10004527  mov     eax, dword ptr [0x10068e30]
1000452c  test    eax, eax
1000452e  jne     0x10004c0c
10004534  xor     esi, esi
10004536  test    di, di
10004539  je      0x10004552
1000453b  mov     eax, dword ptr [esp + 0x201f8]
10004542  mov     esi, ebp
10004544  xor     ebp, ebp
10004546  test    eax, eax
10004548  je      0x1000455c
1000454a  cmp     eax, ebx
1000454c  jge     0x1000455c
1000454e  mov     ebx, eax
10004550  jmp     0x1000455c
10004552  mov     dword ptr [0x10068dcc], 0
1000455c  mov     ecx, dword ptr [0x1007205c]
10004562  lea     eax, [ebx + ebp]
10004565  push    eax
10004566  lea     edx, [ecx + 0x100426d8]
1000456c  push    edx
1000456d  push    0x1007203a
10004572  call    0x10003dd0
10004577  mov     edx, dword ptr [0x1007205c]
1000457d  add     esp, 4
10004580  add     eax, edx
10004582  add     eax, esi
10004584  push    eax
10004585  call    0x100029b0   ; EXP:ReadComBytes
1000458a  add     esp, 0xc
1000458d  test    di, di
10004590  je      0x100045aa
10004592  mov     ecx, dword ptr [0x10068e30]
10004598  xor     eax, eax
1000459a  pop     edi
1000459b  pop     esi
1000459c  test    ecx, ecx
1000459e  pop     ebp
1000459f  pop     ebx
100045a0  sete    al
100045a3  add     esp, 0x201dc
100045a9  ret     
100045aa  mov     eax, dword ptr [0x10068e30]
100045af  test    eax, eax
100045b1  jne     0x10004c0c
100045b7  mov     ecx, 0x10068d70
100045bc  call    0x1000ff30
100045c1  mov     eax, dword ptr [0x1007205c]
100045c6  push    0
100045c8  mov     dword ptr [0x10068dcc], 1
100045d2  lea     ecx, [eax + ebp + 0x100426d8]
100045d9  push    ecx
100045da  call    0x1000a640
100045df  mov     edx, dword ptr [0x1007205c]
100045e5  push    ebp
100045e6  lea     eax, [edx + 0x100426d8]
100045ec  push    eax
100045ed  call    0x10001400
100045f2  add     esp, 0x10
100045f5  test    eax, eax
100045f7  jne     0x10004603
100045f9  push    0x12
100045fb  call    0x10001120
10004600  add     esp, 4
10004603  mov     ecx, dword ptr [0x1007205c]
10004609  push    ebx
1000460a  lea     eax, [ecx + ebp + 0x100426d8]
10004611  push    eax
10004612  add     eax, -2
10004615  push    eax
10004616  call    0x1001c0e0
1000461b  mov     eax, dword ptr [0x10068e30]
10004620  add     esp, 0xc
10004623  sub     ebp, 2
10004626  test    eax, eax
10004628  jne     0x10004c0c
1000462e  mov     eax, dword ptr [0x1007205c]
10004633  lea     edx, [ebx + ebp]
10004636  push    edx
10004637  lea     ecx, [eax + 0x100426d8]
1000463d  lea     edx, [eax + 0x1004281a]
10004643  push    ecx
10004644  push    edx
10004645  call    0x1001c0e0
1000464a  mov     eax, dword ptr [0x1007205c]
1000464f  lea     ecx, [ebp + 0x142]
10004655  mov     dword ptr [eax + 0x100426da], ecx
1000465b  mov     ecx, 0x4d
10004660  lea     edx, [eax + 0x100426d8]
10004666  xor     eax, eax
10004668  lea     edi, [esp + 0xc4]
1000466f  lea     esi, [esp + 0xc4]
10004676  rep stosd dword ptr es:[edi], eax
10004678  mov     ecx, 0x3d
1000467d  mov     eax, 0x20202020
10004682  lea     edi, [esp + 0xdc]
10004689  mov     word ptr [edx], 0
1000468e  mov     word ptr [edx + 6], 0
10004694  add     edx, 8
10004697  rep stosd dword ptr es:[edi], eax
10004699  mov     edi, edx
1000469b  mov     ecx, 0x4d
100046a0  add     edx, 0x134
100046a6  mov     byte ptr [esp + 0xd8], 0x54
100046ae  mov     byte ptr [esp + 0xd9], 0x65
100046b6  mov     byte ptr [esp + 0xda], 0x6d
100046be  mov     byte ptr [esp + 0xdb], 0x70
100046c6  mov     word ptr [esp + 0xc4], 0xa
100046d0  mov     dword ptr [esp + 0xc6], 0x134
100046db  lea     eax, [ebp + 6]
100046de  add     esp, 0xc
100046e1  rep movsd dword ptr es:[edi], dword ptr [esi]
100046e3  mov     word ptr [edx], 0x1e
100046e8  mov     dword ptr [edx + 2], eax
100046eb  mov     eax, dword ptr [0x1007205c]
100046f0  test    ebx, ebx
100046f2  je      0x100047af
100046f8  mov     edx, dword ptr [eax + 0x100426da]
100046fe  lea     ecx, [ebx - 0x13c]
10004704  lea     ebp, [eax + ebp + 0x1004281a]
1000470b  add     edx, ecx
1000470d  mov     dword ptr [eax + 0x100426da], edx
10004713  mov     dx, word ptr [ebp + 6]
10004717  lea     esi, [ebp + 8]
1000471a  mov     word ptr [eax + 0x100426de], dx
10004721  cmp     word ptr [esi], 0xa
10004725  je      0x10004733
10004727  push    0x13
10004729  call    0x10001120
1000472e  add     esp, 4
10004731  jmp     0x100047aa
10004733  test    byte ptr [0x1006ae84], 4
1000473a  jne     0x1000477c
1000473c  mov     ecx, dword ptr [esp + 0x201f0]
10004743  cmp     ecx, 0x1b00
10004749  jae     0x1000477c
1000474b  xor     eax, eax
1000474d  mov     edx, 1
10004752  test    ecx, ecx
10004754  jle     0x10004765
10004756  test    byte ptr [eax + 0x10067204], 8
1000475d  jne     0x10004760
1000475f  inc     edx
10004760  inc     eax
10004761  cmp     eax, ecx
10004763  jl      0x10004756
10004765  push    edx
10004766  lea     eax, [ebp + 0x1c]
10004769  push    0x100400e4
1000476e  push    eax
1000476f  call    0x1001c006
10004774  mov     eax, dword ptr [0x1007205c]
10004779  add     esp, 0xc
1000477c  lea     edi, [eax + 0x100426e0]
10004782  mov     ecx, 0x4d
10004787  rep movsd dword ptr es:[edi], dword ptr [esi]
10004789  lea     ecx, [ebx - 0x13c]
1000478f  test    ecx, ecx
10004791  jle     0x100047af
10004793  add     ebx, 0xfffffec4
10004799  lea     edx, [ebp + 0x13c]
1000479f  push    ebx
100047a0  push    edx
100047a1  push    ebp
100047a2  call    0x1001c0e0
100047a7  add     esp, 0xc
100047aa  mov     eax, dword ptr [0x1007205c]
100047af  mov     ecx, dword ptr [0x10068e30]
100047b5  test    ecx, ecx
100047b7  jne     0x10004c0c
100047bd  mov     edx, dword ptr [esp + 0x10]
100047c1  test    edx, edx
100047c3  je      0x10004850
100047c9  mov     ecx, dword ptr [eax + 0x100426da]
100047cf  movsx   di, byte ptr [esp + 0x1a]
100047d5  movsx   bx, byte ptr [esp + 0x1b]
100047db  lea     esi, [ecx + eax + 0x100426d8]
100047e2  add     ecx, edx
100047e4  mov     dword ptr [eax + 0x100426da], ecx
100047ea  lea     eax, [edi*8]
100047f1  mov     word ptr [esi], 0x32
100047f6  mov     dword ptr [esi + 2], edx
100047f9  add     esi, 6
100047fc  add     edx, -0xa
100047ff  push    edx
10004800  mov     word ptr [esi], ax
10004803  mov     word ptr [esi + 2], bx
10004807  add     esi, 4
1000480a  push    esi
1000480b  push    0x10072036
10004810  call    0x10003dd0
10004815  add     esp, 4
10004818  add     eax, 0xae
1000481d  push    eax
1000481e  call    0x100029b0   ; EXP:ReadComBytes
10004823  mov     eax, dword ptr [0x10068e30]
10004828  add     esp, 0xc
1000482b  test    eax, eax
1000482d  jne     0x10004c0c
10004833  push    ebx
10004834  push    edi
10004835  push    esi
10004836  call    0x10003e20
1000483b  mov     eax, dword ptr [0x10068e30]
10004840  add     esp, 0xc
10004843  test    eax, eax
10004845  jne     0x10004c0c
1000484b  mov     eax, dword ptr [0x1007205c]
10004850  cmp     dword ptr [0x10068e2c], 0x309
1000485a  jb      0x10004bb4
10004860  lea     ecx, [esp + 0x2c]
10004864  push    4
10004866  push    ecx
10004867  push    0x24048
1000486c  call    0x100029b0   ; EXP:ReadComBytes
10004871  add     esp, 0xc
10004874  test    eax, eax
10004876  je      0x10004ba5
1000487c  lea     edx, [esp + 0x2c]
10004880  push    edx
10004881  call    0x10003dd0
10004886  mov     esi, dword ptr [0x1007206c]
1000488c  mov     ebx, eax
1000488e  and     esi, 0xff
10004894  lea     ecx, [esp + 0x1f0]
1000489b  add     esi, 6
1000489e  lea     eax, [esi + 0x14]
100048a1  push    eax
100048a2  push    ecx
100048a3  push    ebx
100048a4  call    0x100029b0   ; EXP:ReadComBytes
100048a9  add     esp, 0x10
100048ac  test    eax, eax
100048ae  je      0x10004ba5
100048b4  lea     edx, [esp + esi + 0x1f4]
100048bb  push    edx
100048bc  call    0x10003dd0
100048c1  mov     ebp, eax
100048c3  lea     eax, [esp + esi + 0x1fc]
100048ca  push    eax
100048cb  mov     dword ptr [esp + 0x1c], ebp
100048cf  call    0x10003dd0
100048d4  mov     edi, eax
100048d6  mov     al, byte ptr [esp + 0x1f4]
100048dd  add     esp, 8
100048e0  mov     dword ptr [esp + 0x10], edi
100048e4  test    al, al
100048e6  jne     0x1000495e
100048e8  lea     ecx, [esp + esi + 0x1ec]
100048ef  push    ecx
100048f0  call    0x10003dd0
100048f5  add     esp, 4
100048f8  cmp     eax, 1
100048fb  jne     0x1000495e
100048fd  lea     edx, [esp + esi + 0x1f0]
10004904  push    edx
10004905  call    0x10003dd0
1000490a  add     esp, 4
1000490d  cmp     eax, 8
10004910  jne     0x1000495e
10004912  push    0x10072070
10004917  call    0x10003db0
1000491c  and     eax, 0xffff
10004921  add     esp, 4
10004924  cmp     ebp, eax
10004926  jae     0x1000495e
10004928  push    0x10072070
1000492d  call    0x10003db0
10004932  add     esp, 4
10004935  cmp     ax, 0xfd00
10004939  jae     0x1000495e
1000493b  cmp     edi, 0xff
10004941  jae     0x1000495e
10004943  mov     eax, edi
10004945  shl     eax, 5
10004948  add     eax, edi
1000494a  lea     ecx, [edi + eax*2]
1000494d  shl     ecx, 1
1000494f  cmp     ecx, 0xff00
10004955  jae     0x1000495e
10004957  mov     eax, 1
1000495c  jmp     0x10004960
1000495e  xor     eax, eax
10004960  add     esi, 8
10004963  test    eax, eax
10004965  je      0x10004ba5
1000496b  mov     edx, dword ptr [esp + 0x14]
1000496f  lea     eax, [esp + 0x1ec]
10004976  push    edx
10004977  add     esi, ebx
10004979  push    eax
1000497a  push    esi
1000497b  call    0x100029b0   ; EXP:ReadComBytes
10004980  add     esp, 0xc
10004983  mov     dword ptr [esp + 0x20], eax
10004987  test    eax, eax
10004989  je      0x10004ba5
1000498f  xor     esi, esi
10004991  mov     ebp, 8
10004996  test    edi, edi
10004998  jbe     0x10004ad2
1000499e  lea     ecx, [esp + 0x101f2]
100049a5  mov     dword ptr [esp + 0x24], ecx
100049a9  test    eax, eax
100049ab  je      0x10004ad2
100049b1  mov     ecx, 0x21
100049b6  xor     eax, eax
100049b8  lea     edi, [esp + 0x30]
100049bc  lea     edx, [esp + ebp + 0x1ec]
100049c3  rep stosd dword ptr es:[edi], eax
100049c5  push    edx
100049c6  mov     dword ptr [esp + 0x1c], ebp
100049ca  stosw   word ptr es:[edi], ax
100049cc  call    0x10003db0
100049d1  add     ebp, 2
100049d4  mov     edi, eax
100049d6  lea     ebx, [esi + 1]
100049d9  lea     eax, [esp + ebp + 0x1f0]
100049e0  push    eax
100049e1  call    0x10003db0
100049e6  and     eax, 0xffff
100049eb  mov     dl, byte ptr [esp + ebp + 0x1f6]
100049f2  xor     ecx, ecx
100049f4  cmp     eax, ebx
100049f6  sete    cl
100049f9  add     ebp, 2
100049fc  mov     dword ptr [esp + 0x28], ecx
10004a00  inc     ebp
10004a01  mov     byte ptr [esp + 0x38], dl
10004a05  mov     al, byte ptr [esp + ebp + 0x1f4]
10004a0c  mov     cl, byte ptr [esp + ebp + 0x1f5]
10004a13  inc     ebp
10004a14  mov     byte ptr [esp + 0x39], al
10004a18  inc     ebp
10004a19  mov     byte ptr [esp + 0x3a], cl
10004a1d  lea     edx, [esp + ebp + 0x1f4]
10004a24  push    edx
10004a25  call    0x10003db0
10004a2a  mov     ecx, dword ptr [esp + 0x20]
10004a2e  mov     word ptr [esp + 0x40], ax
10004a33  add     ebp, 2
10004a36  add     esp, 0xc
10004a39  xor     eax, eax
10004a3b  cmp     ebp, ecx
10004a3d  mov     edx, ecx
10004a3f  jae     0x10004a5b
10004a41  mov     cl, byte ptr [esp + ebp + 0x1ec]
10004a48  test    cl, cl
10004a4a  je      0x10004a5b
10004a4c  cmp     eax, 0x40
10004a4f  jae     0x10004a56
10004a51  mov     byte ptr [esp + eax + 0x36], cl
10004a55  inc     eax
10004a56  inc     ebp
10004a57  cmp     ebp, edx
10004a59  jb      0x10004a41
10004a5b  inc     ebp
10004a5c  xor     ecx, ecx
10004a5e  cmp     ebp, edx
10004a60  jae     0x10004a7c
10004a62  mov     al, byte ptr [esp + ebp + 0x1ec]
10004a69  test    al, al
10004a6b  je      0x10004a7c
10004a6d  cmp     ecx, 0x40
10004a70  jae     0x10004a77
10004a72  mov     byte ptr [esp + ecx + 0x76], al
10004a76  inc     ecx
10004a77  inc     ebp
10004a78  cmp     ebp, edx
10004a7a  jb      0x10004a62
10004a7c  mov     ecx, dword ptr [esp + 0x18]
10004a80  and     edi, 0xffff
10004a86  inc     ebp
10004a87  add     edi, ecx
10004a89  cmp     edi, ebp
10004a8b  je      0x10004a97
10004a8d  mov     dword ptr [esp + 0x20], 0
10004a95  jmp     0x10004ab0
10004a97  mov     eax, dword ptr [esp + 0x20]
10004a9b  test    eax, eax
10004a9d  je      0x10004ab0
10004a9f  mov     edi, dword ptr [esp + 0x24]
10004aa3  mov     ecx, 0x21
10004aa8  lea     esi, [esp + 0x30]
10004aac  rep movsd dword ptr es:[edi], dword ptr [esi]
10004aae  movsw   word ptr es:[edi], word ptr [esi]
10004ab0  mov     ecx, dword ptr [esp + 0x24]
10004ab4  mov     eax, dword ptr [esp + 0x10]
10004ab8  mov     esi, ebx
10004aba  add     ecx, 0x86
10004ac0  cmp     esi, eax
10004ac2  mov     edi, eax
10004ac4  mov     eax, dword ptr [esp + 0x20]
10004ac8  mov     dword ptr [esp + 0x24], ecx
10004acc  jb      0x100049a9
10004ad2  cmp     ebp, dword ptr [esp + 0x14]
10004ad6  jne     0x10004ba5
10004adc  test    eax, eax
10004ade  je      0x10004ba5
10004ae4  mov     eax, edi
10004ae6  mov     edx, 8
10004aeb  shl     eax, 5
10004aee  add     eax, edi
10004af0  lea     ecx, [edi + eax*2]
10004af3  mov     eax, dword ptr [0x1007205c]
10004af8  mov     edi, dword ptr [eax + 0x100426da]
10004afe  lea     ebp, [ecx + ecx + 6]
10004b02  cmp     edi, edx
10004b04  mov     dword ptr [esp + 0x18], ebp
10004b08  jbe     0x10004b4d
10004b0a  cmp     word ptr [eax + edx + 0x100426d8], 0x14
10004b13  mov     ecx, dword ptr [eax + edx + 0x100426da]
10004b1a  je      0x10004b24
10004b1c  add     edx, ecx
10004b1e  cmp     edx, edi
10004b20  jb      0x10004b0a
10004b22  jmp     0x10004b4d
10004b24  mov     ebx, edi
10004b26  sub     ebx, ecx
10004b28  cmp     edx, ebx
10004b2a  jae     0x10004b41
10004b2c  mov     edi, ebx
10004b2e  lea     ebp, [edx + eax + 0x100426d8]
10004b35  sub     edi, edx
10004b37  mov     dl, byte ptr [ecx + ebp]
10004b3a  mov     byte ptr [ebp], dl
10004b3d  inc     ebp
10004b3e  dec     edi
10004b3f  jne     0x10004b37
10004b41  mov     ebp, dword ptr [esp + 0x18]
10004b45  mov     edi, ebx
10004b47  mov     dword ptr [eax + 0x100426da], edi
10004b4d  mov     ecx, esi
10004b4f  mov     word ptr [esp + 0x101ec], 0x14
10004b59  shl     ecx, 5
10004b5c  add     ecx, esi
10004b5e  lea     edx, [esi + ecx*2]
10004b61  lea     ecx, [edx + edx + 6]
10004b65  lea     edx, [edi + ebp]
10004b68  add     edx, eax
10004b6a  mov     dword ptr [esp + 0x101ee], ecx
10004b71  cmp     edx, 0x249f0
10004b77  jae     0x10004baa
10004b79  mov     esi, dword ptr [eax + 0x100426da]
10004b7f  mov     ecx, ebp
10004b81  add     esi, ebp
10004b83  lea     edi, [edi + eax + 0x100426d8]
10004b8a  mov     dword ptr [eax + 0x100426da], esi
10004b90  mov     eax, ecx
10004b92  lea     esi, [esp + 0x101ec]
10004b99  shr     ecx, 2
10004b9c  rep movsd dword ptr es:[edi], dword ptr [esi]
10004b9e  mov     ecx, eax
10004ba0  and     ecx, 3
10004ba3  rep movsb byte ptr es:[edi], byte ptr [esi]
10004ba5  mov     eax, dword ptr [0x1007205c]
10004baa  mov     ecx, dword ptr [0x10068e30]
10004bb0  test    ecx, ecx
10004bb2  jne     0x10004c0c
10004bb4  lea     ecx, [eax + 0x100426d8]
10004bba  push    1
10004bbc  push    ecx
10004bbd  call    0x1000a640
10004bc2  mov     eax, dword ptr [0x10068e30]
10004bc7  add     esp, 8
10004bca  test    eax, eax
10004bcc  jne     0x10004c0c
10004bce  mov     edx, dword ptr [0x1007205c]
10004bd4  lea     eax, [edx + 0x100426d8]
10004bda  push    eax
10004bdb  call    0x10008e70
10004be0  add     eax, 0x1e
10004be3  add     esp, 4
10004be6  cmp     eax, 0x1e
10004be9  ja      0x10004c02
10004beb  xor     ecx, ecx
10004bed  mov     cl, byte ptr [eax + 0x10004c64]
10004bf3  jmp     dword ptr [ecx*4 + 0x10004c54]
10004bfa  push    0x14
10004bfc  jmp     0x10004c04
10004bfe  push    0x15
10004c00  jmp     0x10004c04
10004c02  push    0x13
10004c04  call    0x10001120
10004c09  add     esp, 4
10004c0c  cmp     word ptr [esp + 0x28], 0
10004c12  je      0x10004c1e
10004c14  mov     ecx, 0x10068d70
10004c19  call    0x1000fd60
10004c1e  mov     ecx, dword ptr [0x10068e30]
10004c24  xor     eax, eax
10004c26  pop     edi
10004c27  pop     esi
10004c28  test    ecx, ecx
10004c2a  pop     ebp
10004c2b  pop     ebx
10004c2c  sete    al
10004c2f  add     esp, 0x201dc
10004c35  ret     
10004c36  test    bx, bx
10004c39  je      0x10004c45
10004c3b  mov     ecx, 0x10068d70
10004c40  call    0x1000fd60
10004c45  pop     edi
10004c46  pop     esi
10004c47  pop     ebp
10004c48  xor     eax, eax
10004c4a  pop     ebx
10004c4b  add     esp, 0x201dc
10004c51  ret     
10004c52  mov     edi, edi
10004c54  cli     
10004c55  dec     ebx
10004c56  add     byte ptr [eax], dl
10004c58  dec     byte ptr [ebx]
10004c5b  adc     byte ptr [esp + ecx*2], cl
10004c5e  add     byte ptr [eax], dl
10004c60  add     cl, byte ptr [eax + eax + 0x10]
10004c64  add     byte ptr [ebx], al
10004c66  add     eax, dword ptr [ebx]
10004c68  add     eax, dword ptr [ebx]
10004c6a  add     eax, dword ptr [ebx]
10004c6c  add     eax, dword ptr [ebx]
10004c6e  add     eax, dword ptr [ebx]
10004c70  add     eax, dword ptr [ebx]
10004c72  add     eax, dword ptr [ebx]
10004c74  add     eax, dword ptr [ebx]
10004c76  add     eax, dword ptr [ebx]
10004c78  add     eax, dword ptr [ebx]
10004c7a  add     eax, dword ptr [ebx]
10004c7c  add     eax, dword ptr [ebx]
10004c7e  add     eax, dword ptr [ebx]
10004c80  add     dword ptr [ecx], eax
10004c82  add     dl, byte ptr [eax - 0x6f6f6f70]
