10013b00  sub     esp, 0x28
10013b03  mov     eax, dword ptr [0x100407ac]
10013b08  mov     ecx, dword ptr [0x100407b0]   ; 'RNINA RWUSER'
10013b0e  mov     edx, dword ptr [0x100407b4]   ; 'A RWUSER'
10013b14  push    ebp
10013b15  mov     dword ptr [esp + 4], eax
10013b19  mov     eax, dword ptr [0x100407b8]   ; 'USER'
10013b1e  mov     dword ptr [esp + 8], ecx
10013b22  mov     cl, byte ptr [0x100407bc]
10013b28  push    esi
10013b29  push    0x66
10013b2b  mov     dword ptr [esp + 0x14], edx
10013b2f  mov     dword ptr [esp + 0x18], eax
10013b33  mov     byte ptr [esp + 0x1c], cl
10013b37  mov     byte ptr [0x1006d721], 0
10013b3e  mov     byte ptr [0x1006d720], 0
10013b45  call    0x10013710   ; EXP:McSendInstr
10013b4a  add     esp, 4
10013b4d  test    al, al
10013b4f  je      0x10013c8d
10013b55  lea     edx, [esp + 0x1c]
10013b59  push    6
10013b5b  push    edx
10013b5c  call    0x10013a70
10013b61  add     esp, 8
10013b64  test    al, al
10013b66  je      0x10013c8d
10013b6c  mov     al, byte ptr [esp + 0x1c]
10013b70  mov     ebp, dword ptr [esp + 0x34]
10013b74  mov     cl, byte ptr [esp + 0x21]
10013b78  and     al, 0x3f
10013b7a  test    cl, cl
10013b7c  mov     byte ptr [ebp], al
10013b7f  je      0x10013b86
10013b81  or      al, 0x80
10013b83  mov     byte ptr [ebp], al
10013b86  mov     al, byte ptr [ebp]
10013b89  push    0x6b
10013b8b  mov     byte ptr [0x1006d721], al
10013b90  call    0x10013710   ; EXP:McSendInstr
10013b95  add     esp, 4
10013b98  test    al, al
10013b9a  je      0x10013c8d
10013ba0  lea     ecx, [esp + 0x1c]
10013ba4  push    0x14
10013ba6  push    ecx
10013ba7  call    0x10013a70
10013bac  add     esp, 8
10013baf  test    al, al
10013bb1  je      0x10013c8d
10013bb7  push    edi
10013bb8  lea     edi, [esp + 0xc]
10013bbc  or      ecx, 0xffffffff
10013bbf  xor     eax, eax
10013bc1  repne scasb al, byte ptr es:[edi]
10013bc3  not     ecx
10013bc5  dec     ecx
10013bc6  lea     esi, [esp + 0xe]
10013bca  pop     edi
10013bcb  mov     byte ptr [esp + ecx + 0x1c], al
10013bcf  lea     eax, [esp + 0x1e]
10013bd3  mov     dl, byte ptr [eax]
10013bd5  mov     cl, dl
10013bd7  cmp     dl, byte ptr [esi]
10013bd9  jne     0x10013bf7
10013bdb  test    cl, cl
10013bdd  je      0x10013bf3
10013bdf  mov     dl, byte ptr [eax + 1]
10013be2  mov     cl, dl
10013be4  cmp     dl, byte ptr [esi + 1]
10013be7  jne     0x10013bf7
10013be9  add     eax, 2
10013bec  add     esi, 2
10013bef  test    cl, cl
10013bf1  jne     0x10013bd3
10013bf3  xor     eax, eax
10013bf5  jmp     0x10013bfc
10013bf7  sbb     eax, eax
10013bf9  sbb     eax, -1
10013bfc  test    eax, eax
10013bfe  jne     0x10013c04
10013c00  or      byte ptr [ebp], 0x40
10013c04  mov     al, byte ptr [ebp]
10013c07  push    0x69
10013c09  mov     byte ptr [0x1006d721], al
10013c0e  call    0x10013710   ; EXP:McSendInstr
10013c13  add     esp, 4
10013c16  test    al, al
10013c18  je      0x10013c8d
10013c1a  lea     ecx, [esp + 0x1c]
10013c1e  push    3
10013c20  push    ecx
10013c21  call    0x10013a70
10013c26  add     esp, 8
10013c29  test    al, al
10013c2b  je      0x10013c8d
10013c2d  mov     edx, dword ptr [esp + 0x38]
10013c31  mov     al, byte ptr [esp + 0x1c]
10013c35  mov     esi, dword ptr [esp + 0x3c]
10013c39  mov     ecx, dword ptr [esp + 0x40]
10013c3d  mov     byte ptr [edx], al
10013c3f  mov     dl, byte ptr [esp + 0x1e]
10013c43  mov     byte ptr [0x1006d720], al
10013c48  mov     al, byte ptr [esp + 0x1d]
10013c4c  mov     byte ptr [esi], al
10013c4e  mov     byte ptr [ecx], dl
10013c50  mov     al, byte ptr [0x1006d720]
10013c55  test    al, al
10013c57  jne     0x10013c62
10013c59  push    0x18
10013c5b  call    0x10001120
10013c60  jmp     0x10013c69
10013c62  push    0
10013c64  call    0x100136a0   ; EXP:McProtect
10013c69  mov     cl, byte ptr [0x1006d721]
10013c6f  add     esp, 4
10013c72  mov     al, 1
10013c74  test    cl, 0xbf
10013c77  mov     byte ptr [0x100426b0], al
10013c7c  jne     0x10013c8f
10013c7e  cmp     byte ptr [esi], 2
10013c81  jae     0x10013c8f
10013c83  push    0x24
10013c85  call    0x10001120
10013c8a  add     esp, 4
10013c8d  xor     al, al
10013c8f  pop     esi
10013c90  pop     ebp
10013c91  add     esp, 0x28
10013c94  ret     
