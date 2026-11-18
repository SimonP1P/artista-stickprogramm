10013d30  push    ecx
10013d31  lea     eax, [esp]
10013d35  lea     ecx, [esp + 1]
10013d39  push    eax
10013d3a  lea     edx, [esp + 6]
10013d3e  push    ecx
10013d3f  lea     eax, [esp + 0xb]
10013d43  push    edx
10013d44  push    eax
10013d45  call    0x10013b00   ; EXP:McReadVersion
10013d4a  add     esp, 0x14
10013d4d  ret     
