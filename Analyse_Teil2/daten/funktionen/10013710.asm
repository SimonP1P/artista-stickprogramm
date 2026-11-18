10013710  lea     eax, [esp + 4]
10013714  push    1
10013716  push    eax
10013717  call    0x10013720
1001371c  add     esp, 8
1001371f  ret     
10013720  sub     esp, 0x258
10013726  call    0x100018c0   ; EXP:ReadFromCOM
1001372b  cmp     eax, 0x1000
10013730  je      0x1001373e
10013732  call    0x100018c0   ; EXP:ReadFromCOM
10013737  cmp     eax, 0x1000
1001373c  jne     0x10013732
1001373e  mov     eax, dword ptr [esp + 0x260]
10013745  cmp     eax, 0x244
1001374a  jle     0x10013755
1001374c  xor     al, al
1001374e  add     esp, 0x258
10013754  ret     
10013755  mov     ecx, eax
10013757  push    esi
10013758  mov     esi, dword ptr [esp + 0x260]
1001375f  mov     edx, ecx
10013761  push    edi
10013762  lea     edi, [esp + 9]
10013766  shr     ecx, 2
10013769  rep movsd dword ptr es:[edi], dword ptr [esi]
1001376b  mov     ecx, edx
1001376d  add     eax, 2
10013770  and     ecx, 3
10013773  push    0x64
10013775  rep movsb byte ptr es:[edi], byte ptr [esi]
10013777  mov     byte ptr [esp + 0xc], 2
1001377c  mov     byte ptr [esp + eax + 0xb], 3
10013781  push    eax
10013782  lea     eax, [esp + 0x10]
10013786  push    eax
10013787  call    0x100137a0
1001378c  add     esp, 0xc
1001378f  pop     edi
10013790  pop     esi
10013791  add     esp, 0x258
10013797  ret     
