100057b0  push    ebp
100057b1  call    0x10002330
100057b6  test    ax, ax
100057b9  jne     0x100057bd
100057bb  pop     ebp
100057bc  ret     
100057bd  mov     ebp, dword ptr [esp + 8]
100057c1  mov     eax, dword ptr [0x100400d8]
100057c6  cmp     eax, ebp
100057c8  jne     0x100057d0
100057ca  mov     ax, 1
100057ce  pop     ebp
100057cf  ret     
100057d0  cmp     ebp, 0x4b00
100057d6  je      0x100057fb
100057d8  cmp     ebp, 0xe100
100057de  je      0x100057f4
100057e0  cmp     ebp, 0x1c200
100057e6  je      0x100057ed
100057e8  xor     ax, ax
100057eb  pop     ebp
100057ec  ret     
100057ed  mov     byte ptr [esp + 8], 2
100057f2  jmp     0x10005800
100057f4  mov     byte ptr [esp + 8], 5
100057f9  jmp     0x10005800
100057fb  mov     byte ptr [esp + 8], 0x11
10005800  call    0x100022f0
10005805  test    eax, eax
10005807  jne     0x1000580e
10005809  xor     ax, ax
1000580c  pop     ebp
1000580d  ret     
1000580e  push    esi
1000580f  mov     esi, dword ptr [esp + 0xc]
10005813  push    edi
10005814  push    1
10005816  push    1
10005818  push    0
1000581a  call    0x10001c80   ; EXP:OpenComInterface
1000581f  mov     eax, dword ptr [0x10068e30]
10005824  add     esp, 0xc
10005827  test    eax, eax
10005829  jne     0x10005853
1000582b  push    0x3e8
10005830  push    0x4a
10005832  call    0x100027d0
10005837  mov     eax, dword ptr [0x10068e30]
1000583c  add     esp, 8
1000583f  test    eax, eax
10005841  jne     0x10005853
10005843  mov     eax, esi
10005845  and     eax, 0xff
1000584a  push    eax
1000584b  call    0x10002750
10005850  add     esp, 4
10005853  call    0x100022a0
10005858  test    eax, eax
1000585a  jne     0x10005814
1000585c  mov     eax, dword ptr [0x10068e30]
10005861  test    eax, eax
10005863  jne     0x10005937
10005869  call    0x10001460   ; EXP:ExitCOM
1000586e  push    1
10005870  push    1
10005872  push    ebp
10005873  call    0x10001610
10005878  mov     eax, dword ptr [0x10068e30]
1000587d  add     esp, 0xc
10005880  test    eax, eax
10005882  jne     0x10005937
10005888  mov     edi, dword ptr [0x10035284]   ; KERNEL32.dll!Sleep
1000588e  xor     esi, esi
10005890  call    0x100018c0   ; EXP:ReadFromCOM
10005895  cmp     eax, 0x1000
1000589a  je      0x100058a8
1000589c  call    0x100018c0   ; EXP:ReadFromCOM
100058a1  cmp     eax, 0x1000
100058a6  jne     0x1000589c
100058a8  push    0x64
100058aa  call    edi
100058ac  call    0x100018c0   ; EXP:ReadFromCOM
100058b1  cmp     eax, 0x42
100058b4  je      0x100058c8
100058b6  inc     esi
100058b7  cmp     esi, 0x14
100058ba  jl      0x10005890
100058bc  mov     dword ptr [0x10068e30], 0x11
100058c6  jmp     0x10005932
100058c8  cmp     esi, 0x14
100058cb  jge     0x100058bc
100058cd  push    0x4f
100058cf  push    0x10
100058d1  call    0x10002700
100058d6  push    0x53
100058d8  push    0x10
100058da  call    0x10002700
100058df  mov     eax, dword ptr [0x10068e30]
100058e4  add     esp, 0x10
100058e7  test    eax, eax
100058e9  jne     0x10005932
100058eb  push    0x3e8
100058f0  push    0x45
100058f2  call    0x100027d0
100058f7  mov     eax, dword ptr [0x10068e30]
100058fc  add     esp, 8
100058ff  test    eax, eax
10005901  jne     0x10005932
10005903  push    0x3e8
10005908  push    0x42
1000590a  call    0x100027d0
1000590f  mov     eax, dword ptr [0x10068e30]
10005914  add     esp, 8
10005917  test    eax, eax
10005919  jne     0x10005932
1000591b  mov     ecx, dword ptr [0x10068e30]
10005921  xor     eax, eax
10005923  test    ecx, ecx
10005925  pop     edi
10005926  mov     dword ptr [0x100400d8], ebp
1000592c  pop     esi
1000592d  pop     ebp
1000592e  sete    al
10005931  ret     
10005932  call    0x10001460   ; EXP:ExitCOM
10005937  mov     ecx, dword ptr [0x10068e30]
1000593d  xor     eax, eax
1000593f  test    ecx, ecx
10005941  pop     edi
10005942  pop     esi
10005943  sete    al
10005946  pop     ebp
10005947  ret     
