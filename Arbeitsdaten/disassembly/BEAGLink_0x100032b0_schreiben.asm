100032b0  mov     eax, 0x1022c
100032b5  call    0x1001c0b0
100032ba  mov     al, byte ptr [0x100426b0]
100032bf  push    esi
100032c0  xor     esi, esi
100032c2  mov     byte ptr [esp + 0xf], 0
100032c7  test    al, al
100032c9  mov     dword ptr [esp + 0x2c], esi
100032cd  mov     dword ptr [0x10068e30], esi
100032d3  jne     0x100032fc
100032d5  mov     eax, dword ptr [0x10072020]
100032da  test    eax, eax
100032dc  jne     0x100032fc
100032de  push    0x15
100032e0  call    0x10001120
100032e5  mov     eax, dword ptr [0x10068e30]
100032ea  add     esp, 4
100032ed  test    eax, eax
100032ef  je      0x100032fc
100032f1  xor     ax, ax
100032f4  pop     esi
100032f5  add     esp, 0x1022c
100032fb  ret     
100032fc  cmp     word ptr [esp + 0x1023c], 0
10003305  je      0x1000333d
10003307  mov     eax, dword ptr [0x10068d8c]
1000330c  test    eax, eax
1000330e  jne     0x10003333
10003310  push    0x84
10003315  mov     ecx, 0x10068d70
1000331a  call    0x1000fd80
1000331f  mov     esi, 1
10003324  mov     ecx, 0x10068d70
10003329  push    esi
1000332a  mov     dword ptr [esp + 0x30], esi
1000332e  call    0x1000ff00
10003333  mov     dword ptr [0x10068dcc], 1
1000333d  mov     al, byte ptr [0x100426b0]
10003342  test    al, al
10003344  je      0x100033a0
10003346  push    1
10003348  push    0
1000334a  call    0x1000d3b0
1000334f  add     esp, 8
10003352  test    al, al
10003354  je      0x10003388
10003356  push    0
10003358  call    0x100136a0   ; EXP:McProtect
1000335d  add     esp, 4
10003360  test    al, al
10003362  je      0x10003388
10003364  mov     eax, dword ptr [esp + 0x10234]
1000336b  push    0
1000336d  push    0
1000336f  push    0
10003371  push    eax
10003372  call    0x1000d620   ; EXP:EnterDesign
10003377  add     esp, 0x10
1000337a  test    al, al
1000337c  je      0x10003388
1000337e  push    1
10003380  call    0x100136a0   ; EXP:McProtect
10003385  add     esp, 4
10003388  test    si, si
1000338b  je      0x10003d93
10003391  mov     ecx, 0x10068d70
10003396  call    0x1000fd60
1000339b  jmp     0x10003d93
100033a0  cmp     word ptr [esp + 0x10238], 0
100033a9  je      0x100033c1
100033ab  push    0
100033ad  push    0x3e8
100033b2  push    0x3e8
100033b7  push    0x10
100033b9  call    0x10002f00
100033be  add     esp, 0x10
100033c1  mov     eax, dword ptr [0x10068e30]
100033c6  push    ebx
100033c7  push    ebp
100033c8  push    edi
100033c9  test    eax, eax
100033cb  jne     0x10003d63
100033d1  mov     ecx, dword ptr [esp + 0x10240]
100033d8  push    0xa
100033da  lea     edx, [esp + 0x44]
100033de  push    ecx
100033df  push    edx
100033e0  call    0x10007600
100033e5  add     esp, 0xc
100033e8  test    eax, eax
100033ea  jne     0x100033f8
100033ec  push    0x15
100033ee  call    0x10001120
100033f3  add     esp, 4
100033f6  jmp     0x10003424
100033f8  mov     ebx, eax
100033fa  lea     eax, [esp + 0x108]
10003401  xor     esi, esi
10003403  sub     ebx, eax
10003405  lea     edi, [esp + esi + 0x108]
1000340c  lea     ecx, [edi + ebx]
1000340f  push    ecx
10003410  lea     ecx, [esp + 0x44]
10003414  call    0x1000a4c0
10003419  inc     esi
1000341a  mov     byte ptr [edi], al
1000341c  cmp     esi, 0x134
10003422  jb      0x10003405
10003424  mov     eax, dword ptr [0x10068e30]
10003429  test    eax, eax
1000342b  jne     0x10003d63
10003431  mov     esi, dword ptr [esp + 0x10240]
10003438  push    0x14
1000343a  lea     edx, [esp + 0x44]
1000343e  push    esi
1000343f  push    edx
10003440  call    0x10007600
10003445  xor     edi, edi
10003447  add     esp, 0xc
1000344a  cmp     eax, edi
1000344c  mov     dword ptr [esp + 0x3c], eax
10003450  mov     dword ptr [esp + 0x24], edi
10003454  je      0x1000345e
10003456  mov     eax, dword ptr [esp + 0x5a]
1000345a  mov     dword ptr [esp + 0x24], eax
1000345e  push    0x28
10003460  lea     ecx, [esp + 0x44]
10003464  push    esi
10003465  push    ecx
10003466  call    0x10007600
1000346b  add     esp, 0xc
1000346e  cmp     eax, edi
10003470  mov     dword ptr [esp + 0x2c], eax
10003474  mov     dword ptr [esp + 0x28], edi
10003478  je      0x10003482
1000347a  mov     edx, dword ptr [esp + 0x5a]
1000347e  mov     dword ptr [esp + 0x28], edx
10003482  push    0x32
10003484  lea     eax, [esp + 0x44]
10003488  push    esi
10003489  push    eax
1000348a  call    0x10007600
1000348f  add     esp, 0xc
10003492  cmp     eax, edi
10003494  mov     dword ptr [esp + 0x14], eax
10003498  mov     dword ptr [esp + 0x30], edi
1000349c  je      0x100034a6
1000349e  mov     ecx, dword ptr [esp + 0x5a]
100034a2  mov     dword ptr [esp + 0x30], ecx
100034a6  push    0x1e
100034a8  lea     edx, [esp + 0x44]
100034ac  push    esi
100034ad  push    edx
100034ae  call    0x10007600
100034b3  mov     esi, eax
100034b5  add     esp, 0xc
100034b8  lea     ecx, [esp + 0x40]
100034bc  mov     dword ptr [esp + 0x20], esi
100034c0  call    0x1000a350
100034c5  cmp     esi, edi
100034c7  mov     dword ptr [esp + 0x1c], eax
100034cb  jne     0x100034d7
100034cd  push    0x15
100034cf  call    0x10001120
100034d4  add     esp, 4
100034d7  cmp     dword ptr [0x10068e30], edi
100034dd  jne     0x10003d63
100034e3  push    0x1007202a
100034e8  call    0x10003dd0
100034ed  add     esp, 4
100034f0  mov     ebp, eax
100034f2  call    0x100074d0
100034f7  test    al, al
100034f9  je      0x10003529
100034fb  mov     al, byte ptr [0x100670c8]
10003500  test    al, al
10003502  je      0x10003516
10003504  cmp     word ptr [esp + 0x10244], 0
1000350d  je      0x10003516
1000350f  push    0x1007203e
10003514  jmp     0x1000351b
10003516  push    0x10072042
1000351b  call    0x10003dd0
10003520  add     esp, 4
10003523  cmp     ebp, eax
10003525  jae     0x10003529
10003527  mov     ebp, eax
10003529  mov     eax, dword ptr [esp + 0x5a]
1000352d  xor     esi, esi
1000352f  lea     ebx, [eax - 6]
10003532  mov     al, byte ptr [0x10072028]
10003537  test    al, al
10003539  mov     dword ptr [esp + 0x34], ebx
1000353d  jbe     0x100035a1
1000353f  mov     edi, dword ptr [esp + 0x238]
10003546  mov     ecx, dword ptr [0x10072048]
1000354c  mov     eax, esi
1000354e  neg     eax
10003550  sbb     eax, eax
10003552  lea     edx, [ecx + esi*2 + 0x100426d8]
10003559  not     eax
1000355b  and     eax, edi
1000355d  push    eax
1000355e  push    edx
1000355f  call    0x10003df0
10003564  mov     eax, dword ptr [0x1007204c]
10003569  push    0
1000356b  lea     ecx, [eax + esi*2 + 0x100426d8]
10003572  push    ecx
10003573  call    0x10003df0
10003578  mov     edx, dword ptr [0x10072050]
1000357e  push    0
10003580  lea     eax, [edx + esi*2 + 0x100426d8]
10003587  push    eax
10003588  call    0x10003df0
1000358d  mov     ecx, dword ptr [0x10072028]
10003593  add     esp, 0x18
10003596  inc     esi
10003597  and     ecx, 0xff
1000359d  cmp     esi, ecx
1000359f  jl      0x10003546
100035a1  mov     eax, dword ptr [0x10072054]
100035a6  lea     edx, [ebx + 2]
100035a9  push    edx
100035aa  lea     ecx, [eax + 0x100426d8]
100035b0  push    ecx
100035b1  call    0x10003e00
100035b6  mov     edi, dword ptr [esp + 0x24]
100035ba  mov     edx, dword ptr [esp + 0x38]
100035be  mov     eax, dword ptr [0x10068e2c]
100035c3  sub     edi, ebx
100035c5  sub     edi, edx
100035c7  add     esp, 8
100035ca  sub     edi, 6
100035cd  cmp     eax, 0x309
100035d2  jb      0x100035d8
100035d4  sub     edi, dword ptr [esp + 0x24]
100035d8  mov     eax, dword ptr [esp + 0x28]
100035dc  mov     edx, dword ptr [0x10072058]
100035e2  sub     edi, eax
100035e4  lea     eax, [edx + 0x100426d8]
100035ea  push    edi
100035eb  push    eax
100035ec  call    0x10003e00
100035f1  add     esp, 8
100035f4  test    ebx, ebx
100035f6  jl      0x10003625
100035f8  test    edi, edi
100035fa  jl      0x10003625
100035fc  lea     esi, [edi + ebx + 2]
10003600  cmp     esi, ebp
10003602  jge     0x10003625
10003604  call    0x100074d0
10003609  test    al, al
1000360b  je      0x1000362f
1000360d  push    0x1007202a
10003612  call    0x10003dd0
10003617  add     esp, 4
1000361a  cmp     esi, eax
1000361c  jl      0x1000362f
1000361e  mov     byte ptr [esp + 0x1b], 1
10003623  jmp     0x1000362f
10003625  push    0x1f
10003627  call    0x10001120
1000362c  add     esp, 4
1000362f  mov     eax, dword ptr [0x10068e30]
10003634  test    eax, eax
10003636  jne     0x10003d63
1000363c  mov     ebp, dword ptr [esp + 0x34]
10003640  xor     esi, esi
10003642  test    ebp, ebp
10003644  jle     0x1000366c
10003646  mov     ecx, dword ptr [esp + 0x20]
1000364a  lea     ebx, [ecx + 6]
1000364d  push    ebx
1000364e  lea     ecx, [esp + 0x44]
10003652  call    0x1000a4c0
10003657  mov     ecx, dword ptr [0x1007205c]
1000365d  inc     esi
1000365e  inc     ebx
1000365f  cmp     esi, ebp
10003661  mov     byte ptr [ecx + esi + 0x100426d7], al
10003668  jl      0x1000364d
1000366a  jmp     0x10003672
1000366c  mov     ecx, dword ptr [0x1007205c]
10003672  mov     eax, dword ptr [0x10068e30]
10003677  mov     byte ptr [ecx + ebp + 0x100426d8], 0x80
1000367f  test    eax, eax
10003681  mov     byte ptr [ecx + ebp + 0x100426d9], 0x81
10003689  jne     0x10003d63
1000368f  mov     edx, dword ptr [esp + 0x34]
10003693  lea     ebx, [ecx + edx + 0x100426da]
1000369a  lea     ecx, [esp + 0x40]
1000369e  call    0x1000a360
100036a3  mov     ecx, dword ptr [eax]
100036a5  mov     esi, 8
100036aa  mov     dword ptr [ebx], ecx
100036ac  mov     ebp, esi
100036ae  mov     edx, dword ptr [eax + 4]
100036b1  mov     dword ptr [ebx + 4], edx
100036b4  mov     dword ptr [ebx + 2], edi
100036b7  mov     eax, dword ptr [0x10068e30]
100036bc  test    eax, eax
100036be  jne     0x1000373a
100036c0  cmp     ebp, edi
100036c2  jl      0x100036ca
100036c4  cmp     esi, dword ptr [esp + 0x1c]
100036c8  jge     0x1000373e
100036ca  cmp     esi, dword ptr [esp + 0x3c]
100036ce  jne     0x100036e2
100036d0  cmp     dword ptr [0x10068e2c], 0x309
100036da  jb      0x100036e2
100036dc  add     esi, dword ptr [esp + 0x24]
100036e0  jmp     0x10003731
100036e2  cmp     esi, dword ptr [esp + 0x2c]
100036e6  jne     0x100036ee
100036e8  add     esi, dword ptr [esp + 0x28]
100036ec  jmp     0x10003731
100036ee  cmp     esi, dword ptr [esp + 0x20]
100036f2  jne     0x100036fe
100036f4  mov     eax, dword ptr [esp + 0x34]
100036f8  lea     esi, [esi + eax + 6]
100036fc  jmp     0x10003731
100036fe  cmp     esi, dword ptr [esp + 0x14]
10003702  jne     0x10003710
10003704  mov     eax, dword ptr [esp + 0x30]
10003708  test    eax, eax
1000370a  je      0x10003710
1000370c  add     esi, eax
1000370e  jmp     0x10003731
10003710  cmp     ebp, edi
10003712  jge     0x10003727
10003714  mov     eax, esi
10003716  lea     ecx, [esp + 0x40]
1000371a  push    eax
1000371b  inc     esi
1000371c  call    0x1000a4c0
10003721  mov     byte ptr [ebx + ebp], al
10003724  inc     ebp
10003725  jmp     0x10003731
10003727  push    0x15
10003729  call    0x10001120
1000372e  add     esp, 4
10003731  mov     eax, dword ptr [0x10068e30]
10003736  test    eax, eax
10003738  je      0x100036c0
1000373a  cmp     esi, dword ptr [esp + 0x1c]
1000373e  je      0x1000374a
10003740  push    0x15
10003742  call    0x10001120
10003747  add     esp, 4
1000374a  push    1
1000374c  push    ebx
1000374d  call    0x1000a640
10003752  mov     eax, dword ptr [0x10068e30]
10003757  add     esp, 8
1000375a  test    eax, eax
1000375c  jne     0x10003d63
10003762  mov     eax, dword ptr [0x10072020]
10003767  test    eax, eax
10003769  je      0x100038df
1000376f  mov     ecx, dword ptr [0x1007205c]
10003775  mov     edx, dword ptr [esp + 0x34]
10003779  add     ecx, edi
1000377b  mov     dword ptr [0x10068dcc], 0
10003785  lea     eax, [ecx + edx + 2]
10003789  push    eax
1000378a  push    0x100426d8
1000378f  push    0x1007203a
10003794  call    0x10003dd0
10003799  add     esp, 4
1000379c  push    eax
1000379d  call    0x10002e50   ; EXP:WriteComBytes
100037a2  mov     eax, dword ptr [0x10068e30]
100037a7  add     esp, 0xc
100037aa  test    eax, eax
100037ac  mov     dword ptr [0x10068dcc], 1
100037b6  jne     0x10003d63
100037bc  mov     eax, dword ptr [esp + 0x14]
100037c0  test    eax, eax
100037c2  je      0x100038df
100037c8  mov     ebx, eax
100037ca  lea     ecx, [esp + 0x40]
100037ce  add     ebx, 6
100037d1  push    ebx
100037d2  call    0x1000a4c0
100037d7  lea     ecx, [ebx + 2]
100037da  push    ecx
100037db  lea     ecx, [esp + 0x44]
100037df  movzx   di, al
100037e3  call    0x1000a4c0
100037e8  movzx   si, al
100037ec  mov     eax, edi
100037ee  add     ebx, 4
100037f1  and     eax, 0xffff
100037f6  mov     dword ptr [esp + 0x14], ebx
100037fa  add     eax, 7
100037fd  mov     dword ptr [esp + 0x20], esi
10003801  cdq     
10003802  and     edx, 7
10003805  and     esi, 0xffff
1000380b  add     eax, edx
1000380d  mov     ebx, eax
1000380f  mov     eax, dword ptr [esp + 0x30]
10003813  sar     ebx, 3
10003816  mov     edx, ebx
10003818  add     eax, -0xa
1000381b  and     edx, 0xffff
10003821  imul    esi, edx
10003824  cmp     eax, esi
10003826  jne     0x100038c8
1000382c  lea     eax, [esi + 0xae]
10003832  push    eax
10003833  mov     dword ptr [esp + 0x30], eax
10003837  call    0x100282ae
1000383c  mov     ebp, eax
1000383e  mov     ecx, 0x2b
10003843  xor     eax, eax
10003845  mov     edi, ebp
10003847  rep stosd dword ptr es:[edi], eax
10003849  mov     cl, byte ptr [esp + 0x24]
1000384d  add     esp, 4
10003850  stosw   word ptr es:[edi], ax
10003852  xor     edi, edi
10003854  mov     byte ptr [ebp], 0
10003858  test    esi, esi
1000385a  mov     byte ptr [ebp + 1], 0
1000385e  mov     byte ptr [ebp + 2], bl
10003861  mov     byte ptr [ebp + 3], cl
10003864  mov     byte ptr [ebp + 4], 0xff
10003868  mov     byte ptr [ebp + 5], 0
1000386c  jle     0x1000388f
1000386e  mov     ecx, dword ptr [esp + 0x14]
10003872  mov     eax, ecx
10003874  inc     ecx
10003875  mov     dword ptr [esp + 0x14], ecx
10003879  push    eax
1000387a  lea     ecx, [esp + 0x44]
1000387e  call    0x1000a4c0
10003883  mov     byte ptr [edi + ebp + 0xae], al
1000388a  inc     edi
1000388b  cmp     edi, esi
1000388d  jl      0x1000386e
1000388f  mov     edx, dword ptr [esp + 0x20]
10003893  lea     eax, [ebp + 0xae]
10003899  push    edx
1000389a  push    ebx
1000389b  push    eax
1000389c  call    0x10003e20
100038a1  mov     ecx, dword ptr [esp + 0x38]
100038a5  add     esp, 0xc
100038a8  push    ecx
100038a9  push    ebp
100038aa  push    0x10072036
100038af  call    0x10003dd0
100038b4  add     esp, 4
100038b7  push    eax
100038b8  call    0x10002e50   ; EXP:WriteComBytes
100038bd  push    ebp
100038be  call    0x100282d7
100038c3  add     esp, 0x10
100038c6  jmp     0x100038d2
100038c8  push    0x15
100038ca  call    0x10001120
100038cf  add     esp, 4
100038d2  mov     eax, dword ptr [0x10068e30]
100038d7  test    eax, eax
100038d9  jne     0x10003d63
100038df  mov     edx, dword ptr [esp + 0x10240]
100038e6  push    edx
100038e7  call    0x100092a0
100038ec  add     esp, 4
100038ef  mov     byte ptr [esp + 0x1a], al
100038f3  test    al, al
100038f5  je      0x10003934
100038f7  dec     al
100038f9  mov     ecx, dword ptr [0x10068e30]
100038ff  mov     byte ptr [esp + 0x1a], al
10003903  mov     eax, dword ptr [0x10072028]
10003908  mov     esi, eax
1000390a  and     esi, 0xff
10003910  inc     esi
10003911  test    ecx, ecx
10003913  jne     0x10003939
10003915  lea     eax, [esp + 0x1a]
10003919  push    1
1000391b  push    eax
1000391c  push    0x1007202e
10003921  call    0x10003dd0
10003926  add     esp, 4
10003929  add     eax, esi
1000392b  push    eax
1000392c  call    0x10002e50   ; EXP:WriteComBytes
10003931  add     esp, 0xc
10003934  mov     eax, dword ptr [0x10072028]
10003939  cmp     word ptr [esp + 0x224], 0
10003942  mov     byte ptr [esp + 0x13], 0
10003947  jne     0x10003954
10003949  cmp     word ptr [esp + 0x222], 0
10003952  je      0x10003959
10003954  mov     byte ptr [esp + 0x13], 1
10003959  cmp     word ptr [esp + 0x21e], 0
10003962  je      0x10003969
10003964  or      byte ptr [esp + 0x13], 2
10003969  cmp     word ptr [esp + 0x220], 0
10003972  je      0x10003979
10003974  or      byte ptr [esp + 0x13], 4
10003979  cmp     word ptr [esp + 0x226], 0
10003982  je      0x10003989
10003984  or      byte ptr [esp + 0x13], 8
10003989  cmp     word ptr [esp + 0x228], 0
10003992  je      0x10003999
10003994  or      byte ptr [esp + 0x13], 0x10
10003999  cmp     word ptr [esp + 0x22c], 0
100039a2  je      0x100039a9
100039a4  or      byte ptr [esp + 0x13], 0x20
100039a9  cmp     word ptr [esp + 0x22e], 0
100039b2  je      0x100039b9
100039b4  or      byte ptr [esp + 0x13], 0x80
100039b9  mov     ecx, eax
100039bb  and     ecx, 0xff
100039c1  lea     esi, [ecx + ecx + 1]
100039c5  mov     ecx, dword ptr [0x10068e30]
100039cb  test    ecx, ecx
100039cd  jne     0x100039f3
100039cf  lea     edx, [esp + 0x13]
100039d3  push    1
100039d5  push    edx
100039d6  push    0x1007202e
100039db  call    0x10003dd0
100039e0  add     esp, 4
100039e3  add     eax, esi
100039e5  push    eax
100039e6  call    0x10002e50   ; EXP:WriteComBytes
100039eb  mov     eax, dword ptr [0x10072028]
100039f0  add     esp, 0xc
100039f3  and     eax, 0xff
100039f8  mov     ecx, 8
100039fd  lea     esi, [esp + 0x11c]
10003a04  lea     edi, [esp + 0x60]
10003a08  lea     ebp, [eax + eax*2 + 1]
10003a0c  mov     eax, 0x20
10003a11  rep movsd dword ptr es:[edi], dword ptr [esi]
10003a13  dec     eax
10003a14  test    eax, eax
10003a16  mov     byte ptr [esp + eax + 0x60], 0
10003a1b  jle     0x10003a24
10003a1d  cmp     byte ptr [esp + eax + 0x5f], 0x20
10003a22  je      0x10003a13
10003a24  mov     eax, dword ptr [0x10068e30]
10003a29  test    eax, eax
10003a2b  jne     0x10003d63
10003a31  lea     eax, [esp + 0x60]
10003a35  push    0x20
10003a37  push    eax
10003a38  push    0x1007202e
10003a3d  call    0x10003dd0
10003a42  add     esp, 4
10003a45  add     eax, ebp
10003a47  push    eax
10003a48  call    0x10002e50   ; EXP:WriteComBytes
10003a4d  mov     eax, dword ptr [0x10068e30]
10003a52  add     esp, 0xc
10003a55  test    eax, eax
10003a57  jne     0x10003d63
10003a5d  cmp     dword ptr [0x10068e2c], 0x309
10003a67  jb      0x10003d3f
10003a6d  lea     ecx, [esp + 0x2c]
10003a71  push    4
10003a73  push    ecx
10003a74  push    0x24048
10003a79  xor     ebx, ebx
10003a7b  call    0x100029b0   ; EXP:ReadComBytes
10003a80  add     esp, 0xc
10003a83  cmp     eax, ebx
10003a85  je      0x10003d32
10003a8b  lea     edx, [esp + 0x2c]
10003a8f  push    edx
10003a90  call    0x10003dd0
10003a95  mov     dword ptr [esp + 0x2c], eax
10003a99  mov     eax, dword ptr [0x1007206c]
10003a9e  and     eax, 0xff
10003aa3  push    0x10072070
10003aa8  add     eax, 6
10003aab  mov     dword ptr [esp + 0x1c], ebx
10003aaf  mov     dword ptr [esp + 0x28], eax
10003ab3  call    0x10003db0
10003ab8  add     esp, 8
10003abb  cmp     ax, 0xfd00
10003abf  sbb     eax, eax
10003ac1  mov     ecx, 0x19
10003ac6  neg     eax
10003ac8  mov     dword ptr [esp + 0x1c], eax
10003acc  mov     eax, ebx
10003ace  lea     edi, [esp + 0x23c]
10003ad5  mov     dword ptr [esp + 0x14], 0x10
10003add  rep stosd dword ptr es:[edi], eax
10003adf  mov     byte ptr [esp + 0x23f], 1
10003ae7  mov     byte ptr [esp + 0x243], 8
10003aef  je      0x10003d32
10003af5  mov     eax, dword ptr [esp + 0x24]
10003af9  mov     ebp, 0x8c
10003afe  cmp     eax, ebp
10003b00  mov     dword ptr [esp + 0x1c], 6
10003b08  jb      0x10003c84
10003b0e  mov     ecx, dword ptr [esp + 0x3c]
10003b12  inc     ebx
10003b13  xor     esi, esi
10003b15  lea     edi, [ecx + ebp - 0x86]
10003b1c  mov     ecx, dword ptr [esp + 0x1c]
10003b20  add     ecx, 0x86
10003b26  add     ebp, 0x86
10003b2c  mov     dword ptr [esp + 0x1c], ecx
10003b30  push    edi
10003b31  lea     ecx, [esp + 0x44]
10003b35  call    0x1000a4c0
10003b3a  mov     byte ptr [esp + esi + 0x80], al
10003b41  inc     edi
10003b42  inc     esi
10003b43  cmp     esi, 0x86
10003b49  jb      0x10003b30
10003b4b  mov     eax, 0x3e
10003b50  cmp     byte ptr [esp + eax + 0x86], 0x20
10003b58  jne     0x10003b5f
10003b5a  dec     eax
10003b5b  jne     0x10003b50
10003b5d  jmp     0x10003b67
10003b5f  mov     byte ptr [esp + eax + 0x87], 0
10003b67  mov     eax, 0x3e
10003b6c  cmp     byte ptr [esp + eax + 0xc6], 0x20
10003b74  jne     0x10003b7b
10003b76  dec     eax
10003b77  jne     0x10003b6c
10003b79  jmp     0x10003b83
10003b7b  mov     byte ptr [esp + eax + 0xc7], 0
10003b83  mov     eax, dword ptr [esp + 0x14]
10003b87  mov     cl, byte ptr [esp + 0x80]
10003b8e  mov     edx, ebx
10003b90  shr     edx, 8
10003b93  mov     byte ptr [esp + eax + 0x23e], dl
10003b9a  mov     dl, byte ptr [esp + 0x81]
10003ba1  mov     byte ptr [esp + eax + 0x23f], bl
10003ba8  mov     byte ptr [esp + eax + 0x240], cl
10003baf  mov     cl, byte ptr [esp + 0x82]
10003bb6  mov     byte ptr [esp + eax + 0x241], dl
10003bbd  mov     byte ptr [esp + eax + 0x242], cl
10003bc4  mov     cx, word ptr [esp + 0x84]
10003bcc  xor     edx, edx
10003bce  mov     dl, ch
10003bd0  mov     byte ptr [esp + eax + 0x243], dl
10003bd7  mov     dl, byte ptr [esp + 0x86]
10003bde  mov     byte ptr [esp + eax + 0x244], cl
10003be5  mov     ecx, 9
10003bea  test    dl, dl
10003bec  je      0x10003c08
10003bee  lea     edi, [esp + 0x86]
10003bf5  lea     esi, [esp + eax + 0x245]
10003bfc  mov     byte ptr [esi], dl
10003bfe  mov     dl, byte ptr [edi + 1]
10003c01  inc     ecx
10003c02  inc     esi
10003c03  inc     edi
10003c04  test    dl, dl
10003c06  jne     0x10003bfc
10003c08  lea     edx, [ecx + eax]
10003c0b  inc     ecx
10003c0c  mov     byte ptr [esp + edx + 0x23c], 0
10003c14  mov     dl, byte ptr [esp + 0xc6]
10003c1b  test    dl, dl
10003c1d  je      0x10003c3c
10003c1f  lea     esi, [ecx + eax]
10003c22  lea     edi, [esp + 0xc6]
10003c29  lea     esi, [esp + esi + 0x23c]
10003c30  mov     byte ptr [esi], dl
10003c32  mov     dl, byte ptr [edi + 1]
10003c35  inc     ecx
10003c36  inc     esi
10003c37  inc     edi
10003c38  test    dl, dl
10003c3a  jne     0x10003c30
10003c3c  lea     edx, [ecx + eax]
10003c3f  inc     ecx
10003c40  mov     byte ptr [esp + edx + 0x23c], 0
10003c48  mov     edx, ecx
10003c4a  shr     edx, 8
10003c4d  mov     byte ptr [esp + eax + 0x23c], dl
10003c54  mov     byte ptr [esp + eax + 0x23d], cl
10003c5b  add     eax, ecx
10003c5d  mov     dword ptr [esp + 0x14], eax
10003c61  mov     al, byte ptr [esp + 0x24b]
10003c68  inc     al
10003c6a  mov     byte ptr [esp + 0x24b], al
10003c71  jne     0x10003c7a
10003c73  inc     byte ptr [esp + 0x24a]
10003c7a  cmp     ebp, dword ptr [esp + 0x24]
10003c7e  jbe     0x10003b0e
10003c84  mov     eax, dword ptr [esp + 0x24]
10003c88  mov     ecx, dword ptr [esp + 0x1c]
10003c8c  cmp     ecx, eax
10003c8e  jne     0x10003d32
10003c94  push    0x10072070
10003c99  call    0x10003db0
10003c9e  mov     edi, dword ptr [esp + 0x24]
10003ca2  mov     ecx, dword ptr [esp + 0x18]
10003ca6  and     eax, 0xffff
10003cab  add     ecx, edi
10003cad  add     esp, 4
10003cb0  cmp     ecx, eax
10003cb2  jae     0x10003d32
10003cb4  mov     edx, dword ptr [0x1007206c]
10003cba  and     edx, 0xff
10003cc0  cmp     ebx, edx
10003cc2  ja      0x10003d32
10003cc4  mov     ebp, dword ptr [esp + 0x14]
10003cc8  lea     eax, [esp + 0x14]
10003ccc  sub     ebp, 8
10003ccf  push    eax
10003cd0  mov     dword ptr [esp + 0x18], ebp
10003cd4  call    0x10003dd0
10003cd9  mov     esi, dword ptr [esp + 0x2c]
10003cdd  mov     dword ptr [esp + 0x248], eax
10003ce4  mov     eax, dword ptr [esp + 0x18]
10003ce8  lea     ecx, [esp + 0x240]
10003cef  add     eax, 8
10003cf2  add     edi, esi
10003cf4  push    eax
10003cf5  push    ecx
10003cf6  push    edi
10003cf7  mov     dword ptr [esp + 0x24], eax
10003cfb  call    0x10002e50   ; EXP:WriteComBytes
10003d00  add     esp, 0x10
10003d03  mov     byte ptr [esp + 0x23c], 0
10003d0b  test    eax, eax
10003d0d  mov     byte ptr [esp + 0x23d], 0
10003d15  mov     byte ptr [esp + 0x23e], 0
10003d1d  je      0x10003d32
10003d1f  lea     edx, [esp + 0x23c]
10003d26  push    3
10003d28  push    edx
10003d29  push    esi
10003d2a  call    0x10002e50   ; EXP:WriteComBytes
10003d2f  add     esp, 0xc
10003d32  cmp     word ptr [esp + 0x10244], 0
10003d3b  je      0x10003d63
10003d3d  jmp     0x10003d4a
10003d3f  cmp     word ptr [esp + 0x10244], 0
10003d48  je      0x10003d63
10003d4a  push    0
10003d4c  push    0xfa0
10003d51  push    0x1b58
10003d56  push    0x200
10003d5b  call    0x10002f00
10003d60  add     esp, 0x10
10003d63  cmp     word ptr [esp + 0x38], 0
10003d69  pop     edi
10003d6a  pop     ebp
10003d6b  pop     ebx
10003d6c  je      0x10003d78
10003d6e  mov     ecx, 0x10068d70
10003d73  call    0x1000fd60
10003d78  mov     eax, dword ptr [0x10068e30]
10003d7d  test    eax, eax
10003d7f  jne     0x10003d93
10003d81  mov     al, byte ptr [esp + 0xf]
10003d85  test    al, al
10003d87  je      0x10003d93
10003d89  push    0x25
10003d8b  call    0x10001150
10003d90  add     esp, 4
10003d93  mov     ecx, dword ptr [0x10068e30]
10003d99  xor     eax, eax
10003d9b  test    ecx, ecx
10003d9d  sete    al
10003da0  pop     esi
10003da1  add     esp, 0x1022c
10003da7  ret     
