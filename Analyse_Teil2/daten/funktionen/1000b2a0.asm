1000b2a0  push    -1
1000b2a2  mov     eax, dword ptr fs:[0]
1000b2a8  push    0x1003302b
1000b2ad  push    eax
1000b2ae  mov     eax, 0x223c
1000b2b3  mov     dword ptr fs:[0], esp
1000b2ba  call    0x1001c0b0
1000b2bf  push    ebx
1000b2c0  push    ebp
1000b2c1  mov     ebp, ecx
1000b2c3  push    esi
1000b2c4  push    edi
1000b2c5  lea     ecx, [esp + 0x28]
1000b2c9  mov     dword ptr [esp + 0x10], ebp
1000b2cd  call    0x1000a750
1000b2d2  lea     ebx, [ebp + 0x54]
1000b2d5  xor     esi, esi
1000b2d7  push    0x30
1000b2d9  mov     ecx, ebx
1000b2db  mov     dword ptr [esp + 0x2258], esi
1000b2e2  call    0x1000a8b0
1000b2e7  and     eax, 0xff
1000b2ec  mov     ecx, ebp
1000b2ee  mov     edi, eax
1000b2f0  mov     dword ptr [esp + 0x6c], edi
1000b2f4  call    0x1000ac70
1000b2f9  test    al, al
1000b2fb  je      0x1000b758
1000b301  cmp     edi, 0xff
1000b307  je      0x1000b758
1000b30d  xor     eax, eax
1000b30f  lea     edi, [ebp + 0xc]
1000b312  mov     dword ptr [esp + 0x60], eax
1000b316  mov     ecx, 0x10
1000b31b  mov     dword ptr [esp + 0x64], eax
1000b31f  mov     dword ptr [esp + 0x24], esi
1000b323  rep stosd dword ptr es:[edi], eax
1000b325  mov     eax, dword ptr [esp + 0x2264]
1000b32c  mov     dword ptr [esp + 0x48], esi
1000b330  cmp     eax, esi
1000b332  mov     dword ptr [esp + 0x68], esi
1000b336  mov     dword ptr [esp + 0x54], esi
1000b33a  mov     byte ptr [esp + 0x1f], 0
1000b33f  mov     dword ptr [esp + 0x40], esi
1000b343  je      0x1000b355
1000b345  mov     eax, dword ptr [esp + 0x2268]
1000b34c  mov     byte ptr [esp + 0x17], 1
1000b351  cmp     eax, esi
1000b353  jne     0x1000b35a
1000b355  mov     byte ptr [esp + 0x17], 0
1000b35a  mov     ecx, ebp
1000b35c  call    0x1000ab70
1000b361  mov     ecx, dword ptr [esp + 0x225c]
1000b368  push    0xa
1000b36a  lea     edx, [esp + 0x74]
1000b36e  push    ecx
1000b36f  push    edx
1000b370  mov     dword ptr [esp + 0x68], eax
1000b374  call    0x10007600
1000b379  mov     edi, eax
1000b37b  add     esp, 0xc
1000b37e  cmp     edi, esi
1000b380  jle     0x1000b38f
1000b382  cmp     dword ptr [esp + 0x8a], 0x134
1000b38d  jae     0x1000b392
1000b38f  mov     byte ptr [ebx], 0
1000b392  cmp     byte ptr [ebx], 0
1000b395  je      0x1000b6c9
1000b39b  lea     ecx, [esp + 0x70]
1000b39f  call    0x1000a350
1000b3a4  mov     dword ptr [esp + 0x50], eax
1000b3a8  mov     ebx, edi
1000b3aa  lea     eax, [esp + 0x118]
1000b3b1  xor     esi, esi
1000b3b3  sub     ebx, eax
1000b3b5  lea     edi, [esp + esi + 0x118]
1000b3bc  lea     ecx, [ebx + edi]
1000b3bf  push    ecx
1000b3c0  lea     ecx, [esp + 0x74]
1000b3c4  call    0x1000a4c0
1000b3c9  inc     esi
1000b3ca  mov     byte ptr [edi], al
1000b3cc  cmp     esi, 0x134
1000b3d2  jb      0x1000b3b5
1000b3d4  mov     ecx, dword ptr [esp + 0x5c]
1000b3d8  mov     edx, dword ptr [esp + 0x224]
1000b3df  mov     eax, dword ptr [esp + 0x228]
1000b3e6  push    ecx
1000b3e7  mov     dword ptr [esp + 0x64], edx
1000b3eb  mov     dword ptr [esp + 0x68], eax
1000b3ef  mov     byte ptr [ebp + 0xc], 1
1000b3f3  call    0x1000b030
1000b3f8  mov     edx, dword ptr [esp + 0x2260]
1000b3ff  mov     dword ptr [ebp + 0xe], eax
1000b402  push    edx
1000b403  call    0x10009c60   ; EXP:_GetColorCount
1000b408  add     esp, 8
1000b40b  dec     al
1000b40d  cmp     al, 0xff
1000b40f  mov     byte ptr [ebp + 0xd], al
1000b412  jne     0x1000b418
1000b414  mov     byte ptr [ebp + 0x54], 0
1000b418  cmp     word ptr [esp + 0x232], 0
1000b421  jne     0x1000b436
1000b423  cmp     word ptr [esp + 0x234], 0
1000b42c  mov     dword ptr [esp + 0x20], 0
1000b434  je      0x1000b43e
1000b436  mov     dword ptr [esp + 0x20], 1
1000b43e  cmp     word ptr [esp + 0x236], 0
1000b447  setne   cl
1000b44a  cmp     word ptr [esp + 0x23e], 0
1000b453  mov     byte ptr [esp + 0x1f], cl
1000b457  setne   dl
1000b45a  shl     dl, 2
1000b45d  cmp     word ptr [esp + 0x23c], 0
1000b466  setne   bl
1000b469  add     dl, bl
1000b46b  shl     dl, 1
1000b46d  cmp     word ptr [esp + 0x238], 0
1000b476  setne   bl
1000b479  add     dl, bl
1000b47b  shl     dl, 1
1000b47d  add     dl, cl
1000b47f  shl     dl, 1
1000b481  cmp     word ptr [esp + 0x230], 0
1000b48a  setne   bl
1000b48d  add     dl, bl
1000b48f  shl     dl, 1
1000b491  cmp     word ptr [esp + 0x22e], 0
1000b49a  setne   bl
1000b49d  add     dl, bl
1000b49f  mov     bl, byte ptr [esp + 0x20]
1000b4a3  shl     dl, 1
1000b4a5  add     dl, bl
1000b4a7  test    cl, cl
1000b4a9  mov     byte ptr [ebp + 0x1a], dl
1000b4ac  je      0x1000b4ba
1000b4ae  and     eax, 0xff
1000b4b3  inc     eax
1000b4b4  mov     dword ptr [esp + 0x54], eax
1000b4b8  jmp     0x1000b4c2
1000b4ba  mov     dword ptr [esp + 0x54], 1
1000b4c2  mov     al, byte ptr [esp + 0x24b]
1000b4c9  test    cl, cl
1000b4cb  mov     byte ptr [ebp + 0x1d], al
1000b4ce  jne     0x1000b4d3
1000b4d0  mov     byte ptr [ebp + 0x1d], cl
1000b4d3  mov     byte ptr [ebp + 0x1c], 1
1000b4d7  mov     byte ptr [ebp + 0x4a], 0xff
1000b4db  xor     eax, eax
1000b4dd  mov     cl, byte ptr [esp + eax + 0x12c]
1000b4e4  test    cl, cl
1000b4e6  je      0x1000b4f2
1000b4e8  mov     byte ptr [eax + ebp + 0x2a], cl
1000b4ec  inc     eax
1000b4ed  cmp     eax, 0x1f
1000b4f0  jb      0x1000b4dd
1000b4f2  mov     ecx, 0xffffffd6
1000b4f7  lea     eax, [ebp + 0x48]
1000b4fa  sub     ecx, ebp
1000b4fc  cmp     byte ptr [eax], 0x20
1000b4ff  jne     0x1000b50d
1000b501  mov     byte ptr [eax], 0
1000b504  dec     eax
1000b505  lea     edx, [ecx + eax]
1000b508  cmp     edx, 4
1000b50b  jg      0x1000b4fc
1000b50d  mov     al, byte ptr [esp + 0x17]
1000b511  test    al, al
1000b513  je      0x1000b51c
1000b515  mov     eax, dword ptr [ebp + 0x2a]
1000b518  mov     dword ptr [esp + 0x64], eax
1000b51c  mov     al, byte ptr [ebp + 0x54]
1000b51f  mov     ecx, dword ptr [esp + 0x248]
1000b526  lea     ebx, [ebp + 0x54]
1000b529  mov     dword ptr [esp + 0x40], ecx
1000b52d  test    al, al
1000b52f  je      0x1000b54c
1000b531  mov     edx, dword ptr [esp + 0x225c]
1000b538  push    0x1e
1000b53a  lea     eax, [esp + 0x74]
1000b53e  push    edx
1000b53f  push    eax
1000b540  call    0x10007600
1000b545  add     esp, 0xc
1000b548  mov     dword ptr [esp + 0x18], eax
1000b54c  mov     eax, dword ptr [esp + 0x18]
1000b550  test    eax, eax
1000b552  jg      0x1000b559
1000b554  mov     byte ptr [ebx], 0
1000b557  jmp     0x1000b564
1000b559  mov     ecx, dword ptr [esp + 0x8a]
1000b560  mov     dword ptr [esp + 0x24], ecx
1000b564  mov     al, byte ptr [ebx]
1000b566  xor     esi, esi
1000b568  test    al, al
1000b56a  je      0x1000b6c9
1000b570  mov     edx, dword ptr [esp + 0x225c]
1000b577  push    0x32
1000b579  lea     eax, [esp + 0x74]
1000b57d  push    edx
1000b57e  push    eax
1000b57f  mov     dword ptr [esp + 0x50], esi
1000b583  call    0x10007600
1000b588  add     esp, 0xc
1000b58b  cmp     eax, esi
1000b58d  mov     dword ptr [esp + 0x20], eax
1000b591  je      0x1000b59e
1000b593  mov     ecx, dword ptr [esp + 0x8a]
1000b59a  mov     dword ptr [esp + 0x44], ecx
1000b59e  cmp     byte ptr [ebx], 0
1000b5a1  je      0x1000b6c9
1000b5a7  mov     eax, dword ptr [esp + 0x44]
1000b5ab  test    eax, eax
1000b5ad  je      0x1000b5fc
1000b5af  push    6
1000b5b1  lea     ecx, [esp + 0x74]
1000b5b5  call    0x1000a560
1000b5ba  and     eax, 0xffff
1000b5bf  push    8
1000b5c1  add     eax, 7
1000b5c4  lea     ecx, [esp + 0x74]
1000b5c8  cdq     
1000b5c9  and     edx, 7
1000b5cc  add     eax, edx
1000b5ce  mov     esi, eax
1000b5d0  sar     esi, 3
1000b5d3  call    0x1000a560
1000b5d8  and     eax, 0xffff
1000b5dd  and     esi, 0xffff
1000b5e3  imul    eax, esi
1000b5e6  mov     ecx, dword ptr [esp + 0x44]
1000b5ea  add     eax, 0xa
1000b5ed  cmp     eax, ecx
1000b5ef  setle   dl
1000b5f2  cmp     ecx, 0xa
1000b5f5  mov     byte ptr [ebx], dl
1000b5f7  ja      0x1000b5fc
1000b5f9  mov     byte ptr [ebx], 0
1000b5fc  cmp     byte ptr [ebx], 0
1000b5ff  je      0x1000b6c9
1000b605  mov     eax, dword ptr [esp + 0x225c]
1000b60c  push    0x14
1000b60e  lea     ecx, [esp + 0x74]
1000b612  xor     esi, esi
1000b614  push    eax
1000b615  push    ecx
1000b616  mov     dword ptr [esp + 0x54], esi
1000b61a  call    0x10007600
1000b61f  mov     edi, eax
1000b621  add     esp, 0xc
1000b624  cmp     edi, esi
1000b626  mov     dword ptr [esp + 0x58], edi
1000b62a  je      0x1000b6c9
1000b630  mov     edx, dword ptr [esp + 0x8a]
1000b637  mov     ebx, 0x86
1000b63c  lea     ecx, [edx - 6]
1000b63f  xor     edx, edx
1000b641  mov     eax, ecx
1000b643  div     ebx
1000b645  test    edx, edx
1000b647  je      0x1000b657
1000b649  cmp     ecx, esi
1000b64b  jle     0x1000b657
1000b64d  mov     dword ptr [esp + 0x48], esi
1000b651  mov     dword ptr [esp + 0x68], esi
1000b655  jmp     0x1000b6c9
1000b657  mov     eax, 0x7a44c6b
1000b65c  add     edi, 6
1000b65f  mul     ecx
1000b661  shr     edx, 2
1000b664  test    edx, edx
1000b666  lea     eax, [edx + edx*8 + 0x10]
1000b66a  mov     dword ptr [esp + 0x68], edx
1000b66e  mov     dword ptr [esp + 0x48], eax
1000b672  jle     0x1000b6c9
1000b674  mov     ebx, edx
1000b676  xor     esi, esi
1000b678  mov     eax, edi
1000b67a  lea     ecx, [esp + 0x70]
1000b67e  push    eax
1000b67f  inc     edi
1000b680  call    0x1000a4c0
1000b685  mov     byte ptr [esp + esi + 0x90], al
1000b68c  inc     esi
1000b68d  cmp     esi, 0x86
1000b693  jb      0x1000b678
1000b695  lea     ecx, [esp + 0x96]
1000b69c  push    0x40
1000b69e  push    ecx
1000b69f  call    0x1000a720
1000b6a4  mov     edx, dword ptr [esp + 0x50]
1000b6a8  push    0x40
1000b6aa  lea     esi, [edx + eax + 1]
1000b6ae  lea     eax, [esp + 0xe2]
1000b6b5  push    eax
1000b6b6  call    0x1000a720
1000b6bb  add     esp, 0x10
1000b6be  lea     ecx, [esi + eax + 1]
1000b6c2  dec     ebx
1000b6c3  mov     dword ptr [esp + 0x48], ecx
1000b6c7  jne     0x1000b676
1000b6c9  mov     al, byte ptr [ebp + 0x54]
1000b6cc  lea     ebx, [ebp + 0x54]
1000b6cf  test    al, al
1000b6d1  je      0x1000b73b
1000b6d3  mov     al, byte ptr [esp + 0x2260]
1000b6da  test    al, al
1000b6dc  je      0x1000b6ed
1000b6de  mov     edx, dword ptr [esp + 0x44]
1000b6e2  mov     ecx, 0x13c
1000b6e7  mov     dword ptr [esp + 0x4c], ecx
1000b6eb  jmp     0x1000b701
1000b6ed  mov     ecx, dword ptr [esp + 0x50]
1000b6f1  mov     edx, dword ptr [esp + 0x44]
1000b6f5  mov     eax, dword ptr [esp + 0x24]
1000b6f9  sub     ecx, edx
1000b6fb  sub     ecx, eax
1000b6fd  mov     dword ptr [esp + 0x4c], ecx
1000b701  mov     esi, dword ptr [esp + 0x48]
1000b705  mov     edi, dword ptr [esp + 0x24]
1000b709  test    edx, edx
1000b70b  lea     eax, [ecx + esi]
1000b70e  lea     edi, [eax + edi + 0x11e]
1000b715  jle     0x1000b725
1000b717  mov     eax, dword ptr [esp + 0x54]
1000b71b  add     eax, -2
1000b71e  lea     edx, [edx + eax*2]
1000b721  add     eax, edx
1000b723  add     edi, eax
1000b725  push    ecx
1000b726  call    0x1000b030
1000b72b  add     esp, 4
1000b72e  mov     dword ptr [ebp + 0x22], eax
1000b731  test    esi, esi
1000b733  je      0x1000b73f
1000b735  mov     byte ptr [ebp + 0x1b], 2
1000b739  jmp     0x1000b73f
1000b73b  mov     edi, dword ptr [esp + 0x58]
1000b73f  cmp     byte ptr [ebx], 0
1000b742  je      0x1000bd4d
1000b748  push    edi
1000b749  lea     ecx, [esp + 0x2c]
1000b74d  call    0x1000a7a0
1000b752  test    al, al
1000b754  jne     0x1000b789
1000b756  mov     byte ptr [ebx], al
1000b758  lea     ecx, [esp + 0x28]
1000b75c  mov     dword ptr [esp + 0x2254], 0xffffffff
1000b767  call    0x1000a770
1000b76c  xor     al, al
1000b76e  mov     ecx, dword ptr [esp + 0x224c]
1000b775  pop     edi
1000b776  pop     esi
1000b777  pop     ebp
1000b778  pop     ebx
1000b779  mov     dword ptr fs:[0], ecx
1000b780  add     esp, 0x2248
1000b786  ret     0x10
1000b789  cmp     byte ptr [ebx], 0
1000b78c  je      0x1000bd4d
1000b792  push    0x14
1000b794  push    4
1000b796  lea     ecx, [esp + 0x30]
1000b79a  mov     byte ptr [esp + 0x30], 1
1000b79f  call    0x1000aa50
1000b7a4  mov     eax, dword ptr [esp + 0x40]
1000b7a8  lea     ecx, [esp + 0x28]
1000b7ac  and     eax, 0xff
1000b7b1  mov     esi, 0x15
1000b7b6  push    eax
1000b7b7  push    0x14
1000b7b9  call    0x1000a980
1000b7be  mov     edi, 0x10d
1000b7c3  mov     eax, esi
1000b7c5  push    0
1000b7c7  push    eax
1000b7c8  lea     ecx, [esp + 0x30]
1000b7cc  inc     esi
1000b7cd  call    0x1000a980
1000b7d2  dec     edi
1000b7d3  jne     0x1000b7c3
1000b7d5  push    esi
1000b7d6  push    8
1000b7d8  lea     ecx, [esp + 0x30]
1000b7dc  mov     ebp, esi
1000b7de  call    0x1000aa50
1000b7e3  mov     eax, dword ptr [esp + 0x24]
1000b7e7  mov     edi, 6
1000b7ec  cmp     eax, edi
1000b7ee  jle     0x1000b81d
1000b7f0  mov     eax, dword ptr [esp + 0x18]
1000b7f4  mov     ebx, esi
1000b7f6  inc     esi
1000b7f7  lea     ecx, [edi + eax]
1000b7fa  push    ecx
1000b7fb  lea     ecx, [esp + 0x74]
1000b7ff  call    0x1000a4c0
1000b804  and     eax, 0xff
1000b809  lea     ecx, [esp + 0x28]
1000b80d  push    eax
1000b80e  push    ebx
1000b80f  call    0x1000a980
1000b814  mov     eax, dword ptr [esp + 0x24]
1000b818  inc     edi
1000b819  cmp     edi, eax
1000b81b  jl      0x1000b7f0
1000b81d  mov     eax, esi
1000b81f  push    0x80
1000b824  push    eax
1000b825  lea     ecx, [esp + 0x30]
1000b829  inc     esi
1000b82a  call    0x1000a980
1000b82f  mov     eax, esi
1000b831  push    0x81
1000b836  push    eax
1000b837  lea     ecx, [esp + 0x30]
1000b83b  inc     esi
1000b83c  call    0x1000a980
1000b841  push    esi
1000b842  push    0xc
1000b844  lea     ecx, [esp + 0x30]
1000b848  call    0x1000aa50
1000b84d  mov     al, byte ptr [esp + 0x17]
1000b851  test    al, al
1000b853  je      0x1000b8e7
1000b859  mov     edx, dword ptr [esp + 0x24]
1000b85d  mov     cl, byte ptr [esp + 0x1f]
1000b861  test    cl, cl
1000b863  lea     eax, [edx - 6]
1000b866  je      0x1000b86d
1000b868  mov     eax, 0xfc
1000b86d  mov     ecx, dword ptr [esp + 0x2268]
1000b874  mov     dword ptr [esp + 0x40], 0
1000b87c  test    ecx, ecx
1000b87e  je      0x1000b899
1000b880  mov     dl, byte ptr [esp + 0x28]
1000b884  test    dl, dl
1000b886  je      0x1000b899
1000b888  push    eax
1000b889  mov     eax, dword ptr [esp + 0x3e]
1000b88d  add     eax, ebp
1000b88f  push    eax
1000b890  call    ecx
1000b892  add     esp, 8
1000b895  mov     dword ptr [esp + 0x40], eax
1000b899  push    0xc
1000b89b  lea     ecx, [esp + 0x2c]
1000b89f  call    0x1000a920
1000b8a4  push    8
1000b8a6  lea     ecx, [esp + 0x2c]
1000b8aa  mov     ebx, eax
1000b8ac  call    0x1000a920
1000b8b1  sub     ebx, eax
1000b8b3  mov     eax, dword ptr [esp + 0x40]
1000b8b7  xor     edx, edx
1000b8b9  mov     ecx, ebx
1000b8bb  mov     dl, ah
1000b8bd  mov     byte ptr [esp + 0x63], al
1000b8c1  mov     eax, dword ptr [esp + 0x2264]
1000b8c8  mov     byte ptr [esp + 0x61], bl
1000b8cc  sar     ecx, 8
1000b8cf  test    eax, eax
1000b8d1  mov     byte ptr [esp + 0x60], cl
1000b8d5  mov     byte ptr [esp + 0x62], dl
1000b8d9  je      0x1000b8e7
1000b8db  lea     ecx, [esp + 0x60]
1000b8df  push    8
1000b8e1  push    ecx
1000b8e2  call    eax
1000b8e4  add     esp, 8
1000b8e7  mov     edi, 0x16
1000b8ec  lea     ebx, [esp + 0x60]
1000b8f0  sub     ebx, edi
1000b8f2  xor     edx, edx
1000b8f4  lea     ecx, [esp + 0x28]
1000b8f8  mov     dl, byte ptr [ebx + edi]
1000b8fb  push    edx
1000b8fc  push    edi
1000b8fd  call    0x1000a980
1000b902  inc     edi
1000b903  lea     eax, [edi - 0x16]
1000b906  cmp     eax, 8
1000b909  jb      0x1000b8f2
1000b90b  mov     ebp, dword ptr [esp + 0x4c]
1000b90f  xor     ebx, ebx
1000b911  xor     edi, edi
1000b913  lea     ecx, [esi + ebp]
1000b916  mov     dword ptr [esp + 0x4c], ecx
1000b91a  lea     ecx, [esp + 0x28]
1000b91e  call    0x1000a780
1000b923  cmp     dword ptr [esp + 0x4c], eax
1000b927  jle     0x1000b933
1000b929  mov     edx, dword ptr [esp + 0x10]
1000b92d  lea     eax, [edx + 0x54]
1000b930  mov     byte ptr [eax], 0
1000b933  mov     eax, dword ptr [esp + 0x10]
1000b937  mov     cl, byte ptr [eax + 0x54]
1000b93a  test    cl, cl
1000b93c  je      0x1000bb42
1000b942  cmp     ebx, ebp
1000b944  jl      0x1000b94c
1000b946  cmp     edi, dword ptr [esp + 0x50]
1000b94a  jge     0x1000b9a3
1000b94c  cmp     edi, dword ptr [esp + 0x20]
1000b950  jne     0x1000b958
1000b952  add     edi, dword ptr [esp + 0x44]
1000b956  jmp     0x1000b99c
1000b958  cmp     edi, dword ptr [esp + 0x18]
1000b95c  jne     0x1000b964
1000b95e  add     edi, dword ptr [esp + 0x24]
1000b962  jmp     0x1000b99c
1000b964  cmp     ebx, ebp
1000b966  jge     0x1000b98d
1000b968  push    edi
1000b969  lea     ecx, [esp + 0x74]
1000b96d  call    0x1000a4c0
1000b972  and     eax, 0xff
1000b977  lea     ecx, [esp + 0x28]
1000b97b  push    eax
1000b97c  lea     eax, [esi + ebx]
1000b97f  push    eax
1000b980  call    0x1000a980
1000b985  mov     eax, dword ptr [esp + 0x10]
1000b989  inc     ebx
1000b98a  inc     edi
1000b98b  jmp     0x1000b99c
1000b98d  mov     cl, byte ptr [esp + 0x2260]
1000b994  test    cl, cl
1000b996  jne     0x1000b9a3
1000b998  mov     byte ptr [eax + 0x54], 0
1000b99c  mov     cl, byte ptr [eax + 0x54]
1000b99f  test    cl, cl
1000b9a1  jne     0x1000b942
1000b9a3  mov     cl, byte ptr [eax + 0x54]
1000b9a6  test    cl, cl
1000b9a8  je      0x1000bb42
1000b9ae  mov     cl, byte ptr [esp + 0x28]
1000b9b2  test    cl, cl
1000b9b4  je      0x1000b9d7
1000b9b6  mov     ecx, dword ptr [esp + 0x3a]
1000b9ba  push    1
1000b9bc  mov     dword ptr [ecx + esi + 2], ebp
1000b9c0  mov     edx, dword ptr [esp + 0x3e]
1000b9c4  add     edx, esi
1000b9c6  push    edx
1000b9c7  call    0x1000a640
1000b9cc  mov     esi, dword ptr [esp + 0x54]
1000b9d0  mov     eax, dword ptr [esp + 0x18]
1000b9d4  add     esp, 8
1000b9d7  mov     cl, byte ptr [eax + 0x54]
1000b9da  test    cl, cl
1000b9dc  je      0x1000bb42
1000b9e2  mov     eax, dword ptr [esp + 0x44]
1000b9e6  test    eax, eax
1000b9e8  jle     0x1000bb42
1000b9ee  mov     eax, dword ptr [esp + 0x20]
1000b9f2  push    esi
1000b9f3  push    0
1000b9f5  lea     ecx, [esp + 0x30]
1000b9f9  lea     ebx, [eax + 6]
1000b9fc  call    0x1000aa50
1000ba01  push    ebx
1000ba02  lea     ecx, [esp + 0x74]
1000ba06  call    0x1000a560
1000ba0b  and     eax, 0xffff
1000ba10  lea     ecx, [ebx + 2]
1000ba13  add     eax, 7
1000ba16  push    ecx
1000ba17  cdq     
1000ba18  and     edx, 7
1000ba1b  lea     ecx, [esp + 0x74]
1000ba1f  add     eax, edx
1000ba21  mov     edi, eax
1000ba23  sar     edi, 3
1000ba26  call    0x1000a560
1000ba2b  mov     ebp, eax
1000ba2d  and     edi, 0xffff
1000ba33  and     ebp, 0xffff
1000ba39  mov     eax, edi
1000ba3b  imul    eax, ebp
1000ba3e  lea     ecx, [esp + 0x28]
1000ba42  mov     dword ptr [esp + 0x40], eax
1000ba46  call    0x1000a780
1000ba4b  mov     edx, dword ptr [esp + 0x40]
1000ba4f  lea     ecx, [edx + esi + 7]
1000ba53  cmp     ecx, eax
1000ba55  mov     eax, dword ptr [esp + 0x10]
1000ba59  jle     0x1000ba5f
1000ba5b  mov     byte ptr [eax + 0x54], 0
1000ba5f  mov     cl, byte ptr [eax + 0x54]
1000ba62  add     ebx, 4
1000ba65  test    cl, cl
1000ba67  je      0x1000bb2e
1000ba6d  push    0xff
1000ba72  push    esi
1000ba73  lea     ecx, [esp + 0x30]
1000ba77  call    0x1000a980
1000ba7c  lea     edx, [esi + 4]
1000ba7f  push    edi
1000ba80  push    edx
1000ba81  lea     ecx, [esp + 0x30]
1000ba85  call    0x1000a980
1000ba8a  lea     eax, [esi + 5]
1000ba8d  push    ebp
1000ba8e  push    eax
1000ba8f  lea     ecx, [esp + 0x30]
1000ba93  call    0x1000a980
1000ba98  lea     ecx, [ebp - 1]
1000ba9b  add     esi, 7
1000ba9e  test    ecx, ecx
1000baa0  jl      0x1000bb08
1000baa2  mov     eax, edi
1000baa4  mov     edx, edi
1000baa6  imul    eax, ecx
1000baa9  neg     edx
1000baab  add     eax, esi
1000baad  inc     ecx
1000baae  mov     dword ptr [esp + 0x24], edx
1000bab2  mov     dword ptr [esp + 0x18], eax
1000bab6  mov     dword ptr [esp + 0x4c], ecx
1000baba  test    edi, edi
1000babc  jle     0x1000baf7
1000babe  mov     ebp, dword ptr [esp + 0x18]
1000bac2  mov     dword ptr [esp + 0x50], edi
1000bac6  push    ebx
1000bac7  lea     ecx, [esp + 0x74]
1000bacb  call    0x1000a4c0
1000bad0  not     al
1000bad2  and     eax, 0xff
1000bad7  lea     ecx, [esp + 0x28]
1000badb  push    eax
1000badc  push    ebp
1000badd  call    0x1000a980
1000bae2  mov     eax, dword ptr [esp + 0x50]
1000bae6  inc     ebx
1000bae7  inc     ebp
1000bae8  dec     eax
1000bae9  mov     dword ptr [esp + 0x50], eax
1000baed  jne     0x1000bac6
1000baef  mov     edx, dword ptr [esp + 0x24]
1000baf3  mov     eax, dword ptr [esp + 0x18]
1000baf7  mov     ecx, dword ptr [esp + 0x4c]
1000bafb  add     eax, edx
1000bafd  dec     ecx
1000bafe  mov     dword ptr [esp + 0x18], eax
1000bb02  mov     dword ptr [esp + 0x4c], ecx
1000bb06  jne     0x1000baba
1000bb08  mov     ecx, dword ptr [esp + 0x40]
1000bb0c  mov     eax, dword ptr [esp + 0x54]
1000bb10  add     esi, ecx
1000bb12  dec     eax
1000bb13  mov     dword ptr [esp + 0x54], eax
1000bb17  mov     eax, dword ptr [esp + 0x10]
1000bb1b  mov     cl, byte ptr [eax + 0x54]
1000bb1e  test    cl, cl
1000bb20  je      0x1000bb2e
1000bb22  mov     ecx, dword ptr [esp + 0x54]
1000bb26  test    ecx, ecx
1000bb28  jg      0x1000ba01
1000bb2e  mov     ecx, dword ptr [esp + 0x20]
1000bb32  mov     edx, dword ptr [esp + 0x44]
1000bb36  add     ecx, edx
1000bb38  cmp     ebx, ecx
1000bb3a  je      0x1000bb53
1000bb3c  mov     byte ptr [eax + 0x54], 0
1000bb40  jmp     0x1000bb53
1000bb42  push    0
1000bb44  push    0
1000bb46  lea     ecx, [esp + 0x30]
1000bb4a  call    0x1000aa50
1000bb4f  mov     eax, dword ptr [esp + 0x10]
1000bb53  mov     cl, byte ptr [eax + 0x54]
1000bb56  test    cl, cl
1000bb58  je      0x1000bd2e
1000bb5e  mov     eax, dword ptr [esp + 0x48]
1000bb62  test    eax, eax
1000bb64  jle     0x1000bd2e
1000bb6a  mov     eax, dword ptr [esp + 0x58]
1000bb6e  push    esi
1000bb6f  push    0x10
1000bb71  lea     ecx, [esp + 0x30]
1000bb75  lea     ebp, [eax + 6]
1000bb78  call    0x1000aa50
1000bb7d  push    1
1000bb7f  push    esi
1000bb80  lea     ecx, [esp + 0x30]
1000bb84  call    0x1000aa50
1000bb89  lea     ecx, [esi + 4]
1000bb8c  push    8
1000bb8e  push    ecx
1000bb8f  lea     ecx, [esp + 0x30]
1000bb93  call    0x1000aa50
1000bb98  mov     edx, dword ptr [esp + 0x48]
1000bb9c  lea     eax, [esi + 8]
1000bb9f  add     edx, -8
1000bba2  lea     ecx, [esp + 0x28]
1000bba6  push    edx
1000bba7  push    eax
1000bba8  call    0x1000aa50
1000bbad  mov     edi, dword ptr [esp + 0x68]
1000bbb1  lea     ecx, [esi + 0xc]
1000bbb4  push    edi
1000bbb5  push    ecx
1000bbb6  lea     ecx, [esp + 0x30]
1000bbba  call    0x1000aa50
1000bbbf  add     esi, 0x10
1000bbc2  xor     ebx, ebx
1000bbc4  test    edi, edi
1000bbc6  jle     0x1000bd2e
1000bbcc  jmp     0x1000bbd2
1000bbce  mov     ebp, dword ptr [esp + 0x58]
1000bbd2  mov     edx, dword ptr [esp + 0x10]
1000bbd6  mov     cl, byte ptr [edx + 0x54]
1000bbd9  lea     eax, [edx + 0x54]
1000bbdc  test    cl, cl
1000bbde  je      0x1000bd2e
1000bbe4  xor     edi, edi
1000bbe6  mov     eax, ebp
1000bbe8  lea     ecx, [esp + 0x70]
1000bbec  push    eax
1000bbed  inc     ebp
1000bbee  call    0x1000a4c0
1000bbf3  mov     byte ptr [esp + edi + 0x90], al
1000bbfa  inc     edi
1000bbfb  cmp     edi, 0x86
1000bc01  jb      0x1000bbe6
1000bc03  lea     eax, [esp + 0x96]
1000bc0a  push    0x40
1000bc0c  push    eax
1000bc0d  mov     dword ptr [esp + 0x60], ebp
1000bc11  call    0x1000a720
1000bc16  lea     ecx, [esp + 0xde]
1000bc1d  push    0x40
1000bc1f  push    ecx
1000bc20  mov     edi, eax
1000bc22  call    0x1000a720
1000bc27  mov     ebp, eax
1000bc29  add     esp, 0x10
1000bc2c  lea     ecx, [esp + 0x28]
1000bc30  lea     edx, [edi + ebp + 0xb]
1000bc34  push    edx
1000bc35  push    esi
1000bc36  call    0x1000a9f0
1000bc3b  lea     eax, [ebx + 1]
1000bc3e  lea     ecx, [esp + 0x28]
1000bc42  mov     dword ptr [esp + 0x50], eax
1000bc46  push    eax
1000bc47  lea     eax, [esi + 2]
1000bc4a  push    eax
1000bc4b  call    0x1000a9f0
1000bc50  mov     ecx, dword ptr [esp + 0x90]
1000bc57  lea     edx, [esi + 4]
1000bc5a  and     ecx, 0xff
1000bc60  push    ecx
1000bc61  push    edx
1000bc62  lea     ecx, [esp + 0x30]
1000bc66  call    0x1000a980
1000bc6b  mov     eax, dword ptr [esp + 0x91]
1000bc72  lea     ecx, [esi + 5]
1000bc75  and     eax, 0xff
1000bc7a  push    eax
1000bc7b  push    ecx
1000bc7c  lea     ecx, [esp + 0x30]
1000bc80  call    0x1000a980
1000bc85  mov     edx, dword ptr [esp + 0x92]
1000bc8c  lea     eax, [esi + 6]
1000bc8f  and     edx, 0xff
1000bc95  lea     ecx, [esp + 0x28]
1000bc99  push    edx
1000bc9a  push    eax
1000bc9b  call    0x1000a980
1000bca0  mov     ecx, dword ptr [esp + 0x94]
1000bca7  lea     edx, [esi + 7]
1000bcaa  and     ecx, 0xffff
1000bcb0  push    ecx
1000bcb1  push    edx
1000bcb2  lea     ecx, [esp + 0x30]
1000bcb6  call    0x1000a9f0
1000bcbb  add     esi, 9
1000bcbe  xor     ebx, ebx
1000bcc0  test    edi, edi
1000bcc2  jle     0x1000bcdf
1000bcc4  movsx   ecx, byte ptr [esp + ebx + 0x96]
1000bccc  mov     eax, esi
1000bcce  push    ecx
1000bccf  push    eax
1000bcd0  lea     ecx, [esp + 0x30]
1000bcd4  inc     esi
1000bcd5  call    0x1000a980
1000bcda  inc     ebx
1000bcdb  cmp     ebx, edi
1000bcdd  jl      0x1000bcc4
1000bcdf  mov     eax, esi
1000bce1  push    0
1000bce3  push    eax
1000bce4  lea     ecx, [esp + 0x30]
1000bce8  inc     esi
1000bce9  call    0x1000a980
1000bcee  xor     edi, edi
1000bcf0  test    ebp, ebp
1000bcf2  jle     0x1000bd0f
1000bcf4  movsx   edx, byte ptr [esp + edi + 0xd6]
1000bcfc  mov     eax, esi
1000bcfe  push    edx
1000bcff  push    eax
1000bd00  lea     ecx, [esp + 0x30]
1000bd04  inc     esi
1000bd05  call    0x1000a980
1000bd0a  inc     edi
1000bd0b  cmp     edi, ebp
1000bd0d  jl      0x1000bcf4
1000bd0f  mov     eax, esi
1000bd11  push    0
1000bd13  push    eax
1000bd14  lea     ecx, [esp + 0x30]
1000bd18  inc     esi
1000bd19  call    0x1000a980
1000bd1e  mov     ebx, dword ptr [esp + 0x50]
1000bd22  mov     eax, dword ptr [esp + 0x68]
1000bd26  cmp     ebx, eax
1000bd28  jl      0x1000bbce
1000bd2e  lea     ecx, [esp + 0x28]
1000bd32  call    0x1000a780
1000bd37  cmp     esi, eax
1000bd39  jne     0x1000bd43
1000bd3b  mov     al, byte ptr [esp + 0x28]
1000bd3f  test    al, al
1000bd41  jne     0x1000bd4d
1000bd43  mov     eax, dword ptr [esp + 0x10]
1000bd47  add     eax, 0x54
1000bd4a  mov     byte ptr [eax], 0
1000bd4d  mov     esi, dword ptr [esp + 0x10]
1000bd51  mov     al, byte ptr [esi + 0x54]
1000bd54  lea     ebx, [esi + 0x54]
1000bd57  test    al, al
1000bd59  je      0x1000bff9
1000bd5f  mov     ecx, 0x10068d70
1000bd64  call    0x1000ff80
1000bd69  mov     edi, eax
1000bd6b  lea     ecx, [esp + 0x28]
1000bd6f  mov     dword ptr [esp + 0x58], edi
1000bd73  call    0x1000a780
1000bd78  mov     ecx, eax
1000bd7a  mov     eax, 0x82082083
1000bd7f  add     ecx, 0xfd
1000bd85  imul    ecx
1000bd87  add     edx, ecx
1000bd89  sar     edx, 7
1000bd8c  mov     ecx, edx
1000bd8e  shr     ecx, 0x1f
1000bd91  lea     edx, [edx + ecx + 2]
1000bd95  mov     ecx, 0x10068d70
1000bd9a  imul    edx, edi
1000bd9d  push    edx
1000bd9e  call    0x1000ff00
1000bda3  mov     ecx, 0x800
1000bda8  xor     eax, eax
1000bdaa  lea     edi, [esp + 0x24c]
1000bdb1  rep stosd dword ptr es:[edi], eax
1000bdb3  mov     edi, dword ptr [esp + 0x6c]
1000bdb7  mov     eax, dword ptr [esi]
1000bdb9  shl     edi, 6
1000bdbc  add     edi, eax
1000bdbe  xor     ebp, ebp
1000bdc0  xor     eax, eax
1000bdc2  lea     ecx, [edi + ebp]
1000bdc5  mov     al, byte ptr [esi + ebp + 0xc]
1000bdc9  push    eax
1000bdca  push    ecx
1000bdcb  mov     ecx, ebx
1000bdcd  call    0x1000a980
1000bdd2  inc     ebp
1000bdd3  cmp     ebp, 0x40
1000bdd6  jb      0x1000bdc0
1000bdd8  mov     eax, edi
1000bdda  push    0x100
1000bddf  cdq     
1000bde0  and     edx, 0xff
1000bde6  mov     ecx, esi
1000bde8  add     eax, edx
1000bdea  sar     eax, 8
1000bded  shl     eax, 8
1000bdf0  push    eax
1000bdf1  call    0x1000abd0
1000bdf6  test    al, al
1000bdf8  jne     0x1000bdfc
1000bdfa  mov     byte ptr [ebx], al
1000bdfc  mov     ecx, 0x10068d70
1000be01  call    0x1000ff30
1000be06  lea     ecx, [esp + 0x28]
1000be0a  mov     dword ptr [esp + 0x18], 0
1000be12  call    0x1000a780
1000be17  test    eax, eax
1000be19  jle     0x1000bf70
1000be1f  mov     ebp, dword ptr [esp + 0x5c]
1000be23  cmp     byte ptr [ebx], 0
1000be26  je      0x1000bf70
1000be2c  mov     edi, dword ptr [esp + 0x18]
1000be30  xor     esi, esi
1000be32  lea     ecx, [esp + 0x28]
1000be36  call    0x1000a780
1000be3b  cmp     edi, eax
1000be3d  jge     0x1000be46
1000be3f  push    edi
1000be40  lea     ecx, [esp + 0x2c]
1000be44  jmp     0x1000be50
1000be46  lea     edx, [esi + ebp - 0x100]
1000be4d  mov     ecx, ebx
1000be4f  push    edx
1000be50  call    0x1000a8b0
1000be55  mov     byte ptr [esp + 0x5c], al
1000be59  lea     ecx, [esi + ebp]
1000be5c  mov     eax, dword ptr [esp + 0x5c]
1000be60  and     eax, 0xff
1000be65  push    eax
1000be66  push    ecx
1000be67  mov     ecx, ebx
1000be69  call    0x1000a980
1000be6e  inc     edi
1000be6f  inc     esi
1000be70  cmp     esi, 0xfc
1000be76  jl      0x1000be32
1000be78  mov     edx, dword ptr [esp + 0x10]
1000be7c  mov     eax, ebp
1000be7e  mov     dword ptr [esp + 0x18], edi
1000be82  mov     ecx, dword ptr [edx + 4]
1000be85  sub     eax, ecx
1000be87  mov     ecx, ebx
1000be89  cdq     
1000be8a  and     edx, 0xff
1000be90  add     eax, edx
1000be92  mov     esi, eax
1000be94  sar     esi, 8
1000be97  mov     eax, esi
1000be99  cdq     
1000be9a  and     edx, 7
1000be9d  add     eax, edx
1000be9f  sar     eax, 3
1000bea2  lea     edi, [eax + 0x48]
1000bea5  push    edi
1000bea6  call    0x1000a8b0
1000beab  mov     ecx, esi
1000bead  and     ecx, 0x80000007
1000beb3  jns     0x1000beba
1000beb5  dec     ecx
1000beb6  or      ecx, 0xfffffff8
1000beb9  inc     ecx
1000beba  mov     dl, 1
1000bebc  shl     dl, cl
1000bebe  or      al, dl
1000bec0  cmp     edi, 0x48
1000bec3  mov     byte ptr [esp + 0x20], al
1000bec7  jl      0x1000beea
1000bec9  cmp     edi, 0x2000
1000becf  jge     0x1000beea
1000bed1  push    0x46
1000bed3  mov     ecx, ebx
1000bed5  call    0x1000a8e0
1000beda  and     eax, 0xffff
1000bedf  add     eax, 0x48
1000bee2  cmp     edi, eax
1000bee4  mov     al, byte ptr [esp + 0x20]
1000bee8  jl      0x1000beed
1000beea  mov     byte ptr [ebx], 0
1000beed  cmp     byte ptr [ebx], 0
1000bef0  je      0x1000bef9
1000bef2  mov     byte ptr [esp + edi + 0x24c], al
1000bef9  mov     eax, dword ptr [esp + 0x20]
1000befd  mov     ecx, ebx
1000beff  and     eax, 0xff
1000bf04  push    eax
1000bf05  push    edi
1000bf06  call    0x1000a980
1000bf0b  lea     ecx, [esp + 0x28]
1000bf0f  call    0x1000a780
1000bf14  mov     edi, dword ptr [esp + 0x18]
1000bf18  cmp     edi, eax
1000bf1a  jge     0x1000bf29
1000bf1c  mov     ecx, dword ptr [esp + 0x10]
1000bf20  call    0x1000ab70
1000bf25  mov     esi, eax
1000bf27  jmp     0x1000bf2b
1000bf29  xor     esi, esi
1000bf2b  lea     ecx, [ebp + 0xfc]
1000bf31  push    esi
1000bf32  push    ecx
1000bf33  mov     ecx, ebx
1000bf35  call    0x1000aa50
1000bf3a  mov     ecx, dword ptr [esp + 0x10]
1000bf3e  push    0x100
1000bf43  push    ebp
1000bf44  call    0x1000abd0
1000bf49  test    al, al
1000bf4b  jne     0x1000bf4f
1000bf4d  mov     byte ptr [ebx], al
1000bf4f  mov     ecx, 0x10068d70
1000bf54  call    0x1000ff30
1000bf59  lea     ecx, [esp + 0x28]
1000bf5d  mov     ebp, esi
1000bf5f  call    0x1000a780
1000bf64  mov     esi, dword ptr [esp + 0x10]
1000bf68  cmp     edi, eax
1000bf6a  jl      0x1000be23
1000bf70  mov     ecx, dword ptr [esi]
1000bf72  mov     eax, 0x48
1000bf77  cmp     ecx, eax
1000bf79  jle     0x1000bfbc
1000bf7b  cmp     byte ptr [ebx], 0
1000bf7e  je      0x1000bfbc
1000bf80  mov     cl, byte ptr [esp + eax + 0x24c]
1000bf87  test    cl, cl
1000bf89  jne     0x1000bf8e
1000bf8b  inc     eax
1000bf8c  jmp     0x1000bfb8
1000bf8e  cdq     
1000bf8f  and     edx, 0xff
1000bf95  push    0x100
1000bf9a  add     eax, edx
1000bf9c  mov     ecx, esi
1000bf9e  mov     edi, eax
1000bfa0  sar     edi, 8
1000bfa3  shl     edi, 8
1000bfa6  push    edi
1000bfa7  call    0x1000abd0
1000bfac  test    al, al
1000bfae  jne     0x1000bfb2
1000bfb0  mov     byte ptr [ebx], al
1000bfb2  lea     eax, [edi + 0x100]
1000bfb8  cmp     eax, dword ptr [esi]
1000bfba  jl      0x1000bf7b
1000bfbc  mov     edx, dword ptr [esp + 0x6c]
1000bfc0  mov     ecx, ebx
1000bfc2  inc     edx
1000bfc3  push    edx
1000bfc4  push    0x30
1000bfc6  call    0x1000a980
1000bfcb  push    0x100
1000bfd0  push    0
1000bfd2  mov     ecx, esi
1000bfd4  call    0x1000abd0
1000bfd9  test    al, al
1000bfdb  jne     0x1000bfdf
1000bfdd  mov     byte ptr [ebx], al
1000bfdf  mov     ecx, 0x10068d70
1000bfe4  call    0x1000ff30
1000bfe9  mov     eax, dword ptr [esp + 0x58]
1000bfed  mov     ecx, 0x10068d70
1000bff2  dec     eax
1000bff3  push    eax
1000bff4  call    0x1000ff00
1000bff9  lea     ecx, [esp + 0x28]
1000bffd  call    0x1000a7f0
1000c002  mov     bl, byte ptr [ebx]
1000c004  lea     ecx, [esp + 0x28]
1000c008  mov     dword ptr [esp + 0x2254], 0xffffffff
1000c013  call    0x1000a770
1000c018  mov     al, bl
1000c01a  jmp     0x1000b76e
