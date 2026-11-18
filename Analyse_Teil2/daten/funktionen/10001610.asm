10001610  mov     eax, dword ptr [0x100400d4]
10001615  sub     esp, 0x20
10001618  push    esi
10001619  push    edi
1000161a  or      esi, 0xffffffff
1000161d  xor     edi, edi
1000161f  cmp     eax, esi
10001621  mov     dword ptr [0x10068e3c], edi
10001627  mov     dword ptr [0x10068e38], edi
1000162d  mov     dword ptr [0x10068e30], edi
10001633  je      0x1000163f
10001635  call    0x10001460   ; EXP:ExitCOM
1000163a  mov     eax, dword ptr [0x100400d4]
1000163f  cmp     dword ptr [0x10068e34], edi
10001645  je      0x100017dc
1000164b  mov     ecx, dword ptr [0x1007201c]
10001651  cmp     ecx, edi
10001653  je      0x100017dc
10001659  cmp     dword ptr [0x10072018], edi
1000165f  jne     0x100017dc
10001665  cmp     dword ptr [0x10068e8c], edi
1000166b  jne     0x10001677
1000166d  mov     dword ptr [0x10068e8c], 1
10001677  cmp     eax, esi
10001679  jne     0x10001737
1000167f  push    ecx
10001680  lea     eax, [esp + 0xc]
10001684  push    0x100400dc   ; 'COM%d'
10001689  push    eax
1000168a  call    0x1001c006
1000168f  add     esp, 0xc
10001692  lea     ecx, [esp + 8]
10001696  push    edi
10001697  push    0x40000080
1000169c  push    3
1000169e  push    edi
1000169f  push    edi
100016a0  push    0xc0000000
100016a5  push    ecx
100016a6  call    dword ptr [0x100351cc]   ; KERNEL32.dll!CreateFileA
100016ac  cmp     eax, esi
100016ae  mov     dword ptr [0x100400d4], eax
100016b3  je      0x10001716
100016b5  push    1
100016b7  push    eax
100016b8  call    dword ptr [0x100351d0]   ; KERNEL32.dll!SetCommMask
100016be  mov     edx, dword ptr [0x100400d4]
100016c4  push    0x1000
100016c9  push    0x1000
100016ce  push    edx
100016cf  call    dword ptr [0x100351d4]   ; KERNEL32.dll!SetupComm
100016d5  mov     ecx, dword ptr [0x100400d4]
100016db  lea     eax, [esp + 0x14]
100016df  push    eax
100016e0  push    ecx
100016e1  mov     dword ptr [esp + 0x1c], esi
100016e5  mov     dword ptr [esp + 0x20], edi
100016e9  mov     dword ptr [esp + 0x24], edi
100016ed  mov     dword ptr [esp + 0x28], edi
100016f1  mov     dword ptr [esp + 0x2c], 0x1388
100016f9  call    dword ptr [0x100351d8]   ; KERNEL32.dll!SetCommTimeouts
100016ff  cmp     dword ptr [0x100400d4], esi
10001705  je      0x10001716
10001707  push    0xc0000000
1000170c  call    0x100014c0
10001711  add     esp, 4
10001714  jmp     0x1000172a
10001716  push    8
10001718  call    0x10001120
1000171d  add     esp, 4
10001720  mov     dword ptr [0x10072094], 4
1000172a  mov     eax, dword ptr [0x100400d4]
1000172f  cmp     eax, esi
10001731  je      0x100017c9
10001737  push    0x100426b8
1000173c  push    eax
1000173d  mov     dword ptr [0x10072018], 1
10001747  mov     dword ptr [0x100426b8], 0x1c
10001751  call    dword ptr [0x100351dc]   ; KERNEL32.dll!GetCommState
10001757  test    eax, eax
10001759  jne     0x1000177d
1000175b  push    9
1000175d  call    0x10001120
10001762  add     esp, 4
10001765  call    0x10001460   ; EXP:ExitCOM
1000176a  mov     ecx, dword ptr [0x10068e30]
10001770  xor     eax, eax
10001772  cmp     ecx, edi
10001774  pop     edi
10001775  sete    al
10001778  pop     esi
10001779  add     esp, 0x20
1000177c  ret     
1000177d  mov     edx, dword ptr [esp + 0x34]
10001781  mov     eax, dword ptr [esp + 0x30]
10001785  mov     esi, dword ptr [esp + 0x2c]
10001789  push    edx
1000178a  push    eax
1000178b  push    esi
1000178c  call    0x100017f0
10001791  add     esp, 0xc
10001794  test    eax, eax
10001796  jne     0x100017ba
10001798  push    0xa
1000179a  call    0x10001120
1000179f  add     esp, 4
100017a2  call    0x10001460   ; EXP:ExitCOM
100017a7  mov     ecx, dword ptr [0x10068e30]
100017ad  xor     eax, eax
100017af  cmp     ecx, edi
100017b1  pop     edi
100017b2  sete    al
100017b5  pop     esi
100017b6  add     esp, 0x20
100017b9  ret     
100017ba  or      esi, 0xc0000000
100017c0  push    esi
100017c1  call    0x100014c0
100017c6  add     esp, 4
100017c9  mov     ecx, dword ptr [0x10068e30]
100017cf  xor     eax, eax
100017d1  cmp     ecx, edi
100017d3  pop     edi
100017d4  sete    al
100017d7  pop     esi
100017d8  add     esp, 0x20
100017db  ret     
100017dc  pop     edi
100017dd  xor     eax, eax
100017df  pop     esi
100017e0  add     esp, 0x20
100017e3  ret     
