10003eb0  sub     esp, 8
10003eb3  push    ebp
10003eb4  push    esi
10003eb5  push    0x7d0
10003eba  call    0x10001230
10003ebf  add     esp, 4
10003ec2  mov     ebp, eax
10003ec4  call    0x10007540   ; EXP:_MakeExclusiveAccess
10003ec9  mov     esi, eax
10003ecb  mov     eax, dword ptr [0x1006ae80]
10003ed0  test    eax, eax
10003ed2  mov     dword ptr [esp + 0xc], esi
10003ed6  jne     0x10003ee3
10003ed8  pop     esi
10003ed9  mov     eax, 0xfffffffd
10003ede  pop     ebp
10003edf  add     esp, 8
10003ee2  ret     
10003ee3  push    1
10003ee5  call    0x100061b0   ; EXP:TransparentMode
10003eea  add     esp, 4
10003eed  test    ax, ax
10003ef0  jne     0x10003f11
10003ef2  test    bp, bp
10003ef5  je      0x10003efc
10003ef7  call    0x10001320
10003efc  test    si, si
10003eff  je      0x10003f06
10003f01  call    0x10007570   ; EXP:_ReleaseExclusiveAccess
10003f06  pop     esi
10003f07  mov     eax, 0xfffffffe
10003f0c  pop     ebp
10003f0d  add     esp, 8
10003f10  ret     
10003f11  push    edi
10003f12  call    0x10007500
10003f17  test    al, al
10003f19  jne     0x10003fb1
10003f1f  mov     al, byte ptr [0x100426b0]
10003f24  test    al, al
10003f26  jne     0x10003fb1
10003f2c  push    0x84
10003f31  mov     ecx, 0x10068d70
10003f36  call    0x1000fd80
10003f3b  push    0
10003f3d  push    0x7d0
10003f42  push    0x1388
10003f47  push    0x80
10003f4c  mov     dword ptr [0x10068dcc], 1
10003f56  call    0x10002f00
10003f5b  add     esp, 0x10
10003f5e  mov     esi, eax
10003f60  call    0x10009360
10003f65  push    eax
10003f66  mov     ecx, 0x10068d70
10003f6b  call    0x1000ff00
10003f70  xor     edi, edi
10003f72  test    esi, esi
10003f74  jle     0x10004043
10003f7a  call    0x10009360
10003f7f  cmp     edi, eax
10003f81  jge     0x10004043
10003f87  push    1
10003f89  push    1
10003f8b  push    edi
10003f8c  call    0x100032b0
10003f91  mov     si, ax
10003f94  add     esp, 0xc
10003f97  mov     ecx, 0x10068d70
10003f9c  and     esi, 0xffff
10003fa2  call    0x1000ff30
10003fa7  inc     edi
10003fa8  test    esi, esi
10003faa  jg      0x10003f7a
10003fac  jmp     0x10004043
10003fb1  xor     esi, esi
10003fb3  lea     eax, [esp + 0xc]
10003fb7  push    esi
10003fb8  push    esi
10003fb9  push    eax
10003fba  call    0x1000d490   ; EXP:DlgBuildImage
10003fbf  mov     edi, eax
10003fc1  add     esp, 0xc
10003fc4  test    edi, edi
10003fc6  je      0x1000404d
10003fcc  push    0x84
10003fd1  mov     ecx, 0x10068d70
10003fd6  call    0x1000fd80
10003fdb  push    1
10003fdd  mov     ecx, 0x10068d70
10003fe2  call    0x1000ff00
10003fe7  push    esi
10003fe8  mov     dword ptr [0x10068dcc], esi
10003fee  call    0x100136a0   ; EXP:McProtect
10003ff3  add     esp, 4
10003ff6  and     eax, 0xff
10003ffb  mov     esi, eax
10003ffd  je      0x10004034
10003fff  mov     dl, byte ptr [0x100426b0]
10004005  mov     ecx, dword ptr [esp + 0xc]
10004009  neg     dl
1000400b  sbb     edx, edx
1000400d  push    ecx
1000400e  and     edx, 0xffd80000
10004014  push    edi
10004015  add     edx, 0x280000
1000401b  push    edx
1000401c  call    0x10002e50   ; EXP:WriteComBytes
10004021  mov     esi, eax
10004023  add     esp, 0xc
10004026  test    esi, esi
10004028  je      0x10004034
1000402a  push    1
1000402c  call    0x100136a0   ; EXP:McProtect
10004031  add     esp, 4
10004034  mov     dword ptr [0x10068dcc], 1
1000403e  call    0x1000d470   ; EXP:ReleaseImage
10004043  mov     ecx, 0x10068d70
10004048  call    0x1000fd60
1000404d  push    0
1000404f  call    0x100061b0   ; EXP:TransparentMode
10004054  add     esp, 4
10004057  test    bp, bp
1000405a  pop     edi
1000405b  je      0x10004062
1000405d  call    0x10001320
10004062  cmp     word ptr [esp + 0xc], 0
10004068  je      0x1000406f
1000406a  call    0x10007570   ; EXP:_ReleaseExclusiveAccess
1000406f  mov     eax, esi
10004071  pop     esi
10004072  pop     ebp
10004073  add     esp, 8
10004076  ret     
