/* Windows: stdin/stdout auf Binärmodus schalten (sonst CRLF-Umwandlung und Strg-Z = Dateiende).
   Nutzung:  cl /O2 /FI binaer_windows.h blast.c
   Auf Mac/Linux wirkungslos. Ohne diese Hülle liefert blast.exe unter Windows für fast alle Dateien „blast err 2“. */
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#include <stdio.h>
static int binaer_windows_init(void) {
    _setmode(_fileno(stdin), _O_BINARY);
    _setmode(_fileno(stdout), _O_BINARY);
    return 0;
}
#pragma section(".CRT$XIU", read)
__declspec(allocate(".CRT$XIU")) static int (*binaer_windows_ptr)(void) = binaer_windows_init;
#endif
