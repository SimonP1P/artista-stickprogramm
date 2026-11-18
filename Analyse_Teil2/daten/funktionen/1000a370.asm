1000a370  mov     ax, word ptr [esp + 0xc]
1000a375  mov     edx, dword ptr [esp + 4]
1000a379  mov     word ptr [ecx + 0xe], ax
1000a37d  mov     eax, dword ptr [esp + 8]
1000a381  test    eax, eax
1000a383  mov     dword ptr [ecx + 0x14], edx
1000a386  jne     0x1000a393
1000a388  test    edx, edx
1000a38a  je      0x1000a391
1000a38c  mov     eax, dword ptr [edx + 2]
1000a38f  jmp     0x1000a393
1000a391  xor     eax, eax
1000a393  push    ebx
1000a394  push    esi
1000a395  lea     esi, [eax - 8]
1000a398  mov     dword ptr [ecx + 0x10], eax
1000a39b  mov     eax, esi
1000a39d  push    edi
1000a39e  cdq     
1000a39f  mov     edi, 0xff
1000a3a4  mov     byte ptr [ecx], 0x42
1000a3a7  idiv    edi
1000a3a9  mov     eax, esi
1000a3ab  mov     byte ptr [ecx + 1], 0x45
1000a3af  sar     eax, 8
1000a3b2  mov     bl, dl
1000a3b4  cdq     
1000a3b5  idiv    edi
1000a3b7  cmp     esi, 0xfff
1000a3bd  mov     al, dl
1000a3bf  mov     byte ptr [esp + 0x18], al
1000a3c3  jg      0x1000a3f8
1000a3c5  mov     al, bl
1000a3c7  mov     dl, bl
1000a3c9  xor     al, 0x76
1000a3cb  xor     dl, 0xd
1000a3ce  mov     byte ptr [ecx + 4], al
1000a3d1  xor     al, 0xb3
1000a3d3  mov     byte ptr [ecx + 2], al
1000a3d6  mov     al, bl
1000a3d8  shl     al, 4
1000a3db  shl     bl, 5
1000a3de  shl     dl, 2
1000a3e1  xor     al, 0x73
1000a3e3  xor     bl, 0xd5
1000a3e6  mov     byte ptr [ecx + 3], 0xb3
1000a3ea  mov     byte ptr [ecx + 5], dl
1000a3ed  mov     byte ptr [ecx + 6], al
1000a3f0  mov     byte ptr [ecx + 7], bl
1000a3f3  jmp     0x1000a48e
1000a3f8  cmp     esi, 0x3fff
1000a3fe  jg      0x1000a42a
1000a400  mov     dl, bl
1000a402  xor     al, 0xea
1000a404  shl     dl, 1
1000a406  xor     dl, 0xc9
1000a409  mov     byte ptr [ecx + 4], al
1000a40c  mov     byte ptr [ecx + 7], dl
1000a40f  mov     dl, bl
1000a411  shl     dl, 6
1000a414  xor     bl, 0x56
1000a417  xor     dl, 0x30
1000a41a  xor     al, bl
1000a41c  mov     byte ptr [ecx + 6], dl
1000a41f  mov     byte ptr [ecx + 5], bl
1000a422  mov     byte ptr [ecx + 3], 0x24
1000a426  xor     al, 0x24
1000a428  jmp     0x1000a48b
1000a42a  cmp     esi, 0xcfff
1000a430  mov     dl, bl
1000a432  jg      0x1000a45b
1000a434  shl     dl, 1
1000a436  xor     dl, 0xc9
1000a439  xor     al, 0x4c
1000a43b  mov     byte ptr [ecx + 7], dl
1000a43e  mov     dl, bl
1000a440  xor     bl, 0xf3
1000a443  mov     byte ptr [ecx + 4], al
1000a446  xor     dl, 0x27
1000a449  xor     al, bl
1000a44b  xor     al, dl
1000a44d  mov     byte ptr [ecx + 6], dl
1000a450  mov     byte ptr [ecx + 5], bl
1000a453  mov     byte ptr [ecx + 3], 0xd1
1000a457  xor     al, 0xd1
1000a459  jmp     0x1000a48b
1000a45b  mov     al, bl
1000a45d  xor     bl, 0x11
1000a460  xor     al, 0x89
1000a462  mov     byte ptr [ecx + 5], bl
1000a465  mov     byte ptr [esp + 0x10], al
1000a469  mov     byte ptr [ecx + 6], al
1000a46c  mov     al, byte ptr [esp + 0x18]
1000a470  xor     dl, 0x52
1000a473  xor     al, 0x6a
1000a475  mov     byte ptr [ecx + 7], dl
1000a478  mov     byte ptr [ecx + 4], al
1000a47b  xor     al, bl
1000a47d  mov     bl, byte ptr [esp + 0x10]
1000a481  mov     byte ptr [ecx + 3], 0x38
1000a485  xor     al, bl
1000a487  xor     al, dl
1000a489  xor     al, 0x38
1000a48b  mov     byte ptr [ecx + 2], al
1000a48e  xor     eax, eax
1000a490  mov     bl, byte ptr [eax + ecx]
1000a493  mov     dl, al
1000a495  shl     dl, 3
1000a498  xor     bl, dl
1000a49a  mov     byte ptr [eax + ecx], bl
1000a49d  inc     eax
1000a49e  cmp     eax, 8
1000a4a1  jl      0x1000a490
1000a4a3  mov     eax, esi
1000a4a5  pop     edi
1000a4a6  sar     eax, 1
1000a4a8  and     al, 7
1000a4aa  pop     esi
1000a4ab  mov     dword ptr [ecx + 0xa], 8
1000a4b2  mov     byte ptr [ecx + 8], al
1000a4b5  pop     ebx
1000a4b6  ret     0xc
