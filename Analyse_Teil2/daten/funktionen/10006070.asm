10006070  sub     esp, 0xc
10006073  call    0x10002330
10006078  test    ax, ax
1000607b  jne     0x10006081
1000607d  add     esp, 0xc
10006080  ret     
10006081  push    esi
10006082  push    edi
10006083  mov     di, word ptr [esp + 0x18]
10006088  test    di, di
1000608b  je      0x10006138
10006091  lea     eax, [esp + 0xa]
10006095  push    1
10006097  push    eax
10006098  push    0xfffee0
1000609d  mov     byte ptr [esp + 0x16], 1
100060a2  call    0x10002e50   ; EXP:WriteComBytes
100060a7  add     esp, 0xc
100060aa  call    dword ptr [0x100351e8]   ; KERNEL32.dll!GetTickCount
100060b0  mov     dword ptr [esp + 0x10], eax
100060b4  mov     eax, dword ptr [0x10068e30]
100060b9  test    eax, eax
100060bb  mov     dword ptr [esp + 0xc], 0x1388
100060c3  jne     0x10006193
100060c9  mov     esi, dword ptr [0x10035284]   ; KERNEL32.dll!Sleep
100060cf  mov     ecx, dword ptr [esp + 0xc]
100060d3  test    ecx, ecx
100060d5  jle     0x10006120
100060d7  lea     ecx, [esp + 0xa]
100060db  push    1
100060dd  push    ecx
100060de  push    0xfffee0
100060e3  call    0x100029b0   ; EXP:ReadComBytes
100060e8  mov     eax, dword ptr [0x10068e30]
100060ed  add     esp, 0xc
100060f0  test    eax, eax
100060f2  jne     0x10006193
100060f8  test    byte ptr [esp + 0xa], 2
100060fd  jne     0x10006124
100060ff  push    0xa
10006101  call    esi
10006103  lea     edx, [esp + 0x10]
10006107  lea     eax, [esp + 0xc]
1000610b  push    edx
1000610c  push    eax
1000610d  call    0x100012d0
10006112  mov     eax, dword ptr [0x10068e30]
10006117  add     esp, 8
1000611a  test    eax, eax
1000611c  je      0x100060cf
1000611e  jmp     0x10006193
10006120  test    eax, eax
10006122  jne     0x10006193
10006124  mov     ecx, dword ptr [esp + 0xc]
10006128  test    ecx, ecx
1000612a  jg      0x1000617d
1000612c  push    0x11
1000612e  call    0x10001120
10006133  add     esp, 4
10006136  jmp     0x10006174
10006138  lea     ecx, [esp + 0xa]
1000613c  push    1
1000613e  push    ecx
1000613f  push    0xfffee0
10006144  call    0x100029b0   ; EXP:ReadComBytes
10006149  mov     eax, dword ptr [0x10068e30]
1000614e  add     esp, 0xc
10006151  test    eax, eax
10006153  jne     0x10006193
10006155  test    byte ptr [esp + 0xa], 2
1000615a  je      0x10006193
1000615c  push    eax
1000615d  push    0x7d0
10006162  push    0x7d0
10006167  push    0x100
1000616c  call    0x10002f00
10006171  add     esp, 0x10
10006174  mov     eax, dword ptr [0x10068e30]
10006179  test    eax, eax
1000617b  jne     0x10006193
1000617d  test    di, di
10006180  je      0x10006193
10006182  mov     cl, byte ptr [0x100426b0]
10006188  mov     byte ptr [0x10068e88], 1
1000618f  test    cl, cl
10006191  je      0x1000619a
10006193  mov     byte ptr [0x10068e88], 0
1000619a  xor     edx, edx
1000619c  pop     edi
1000619d  test    eax, eax
1000619f  sete    dl
100061a2  mov     ax, dx
100061a5  pop     esi
100061a6  add     esp, 0xc
100061a9  ret     
