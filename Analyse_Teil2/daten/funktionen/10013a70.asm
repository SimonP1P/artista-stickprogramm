10013a70  push    ebx
10013a71  push    esi
10013a72  push    edi
10013a73  mov     edi, dword ptr [esp + 0x14]
10013a77  xor     bl, bl
10013a79  test    edi, edi
10013a7b  je      0x10013abd
10013a7d  mov     esi, dword ptr [esp + 0x10]
10013a81  push    0x1f4
10013a86  push    esi
10013a87  call    0x10001f40   ; EXP:ReadByte
10013a8c  add     esp, 8
10013a8f  test    eax, eax
10013a91  je      0x10013adf
10013a93  mov     al, byte ptr [esi]
10013a95  add     bl, al
10013a97  cmp     al, 0x10
10013a99  jne     0x10013ab9
10013a9b  push    0x1f4
10013aa0  push    esi
10013aa1  call    0x10001f40   ; EXP:ReadByte
10013aa6  add     esp, 8
10013aa9  test    eax, eax
10013aab  je      0x10013adf
10013aad  mov     al, byte ptr [esi]
10013aaf  add     bl, al
10013ab1  cmp     al, 0x10
10013ab3  je      0x10013ab9
10013ab5  cmp     al, 1
10013ab7  jne     0x10013adf
10013ab9  inc     esi
10013aba  dec     edi
10013abb  jne     0x10013a81
10013abd  lea     eax, [esp + 0x14]
10013ac1  push    0x1f4
10013ac6  push    eax
10013ac7  call    0x10001f40   ; EXP:ReadByte
10013acc  add     esp, 8
10013acf  test    eax, eax
10013ad1  jne     0x10013ad9
10013ad3  pop     edi
10013ad4  pop     esi
10013ad5  xor     al, al
10013ad7  pop     ebx
10013ad8  ret     
10013ad9  cmp     byte ptr [esp + 0x14], bl
10013add  je      0x10013ae5
10013adf  pop     edi
10013ae0  pop     esi
10013ae1  xor     al, al
10013ae3  pop     ebx
10013ae4  ret     
10013ae5  push    6
10013ae7  call    0x10001a20   ; EXP:WriteToCOM
10013aec  add     esp, 4
10013aef  neg     eax
10013af1  sbb     eax, eax
10013af3  pop     edi
10013af4  pop     esi
10013af5  pop     ebx
10013af6  neg     eax
10013af8  ret     
