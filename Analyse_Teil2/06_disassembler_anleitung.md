# 06 – Der Disassembler: Aufbau, Bedienung, Lesen des Codes

Diese Anleitung beschreibt die drei Werkzeuge in [`code/`](code/), mit denen die gesamte Analyse gemacht wurde:

| Werkzeug | Aufgabe |
|---|---|
| [`rdis.py`](code/rdis.py) | DLL/EXE öffnen, Exporte listen, Funktionen disassemblieren, Strings und Referenzen finden, komplette Liste erzeugen |
| [`asmq.py`](code/asmq.py) | Abfragen auf der kompletten Liste: Funktion ausgeben, Schreibzugriffe auf Variablen, Funktionsanfang finden |
| [`callers.py`](code/callers.py) | alle `call`/`jmp` auf eine Adresse finden („wer ruft diese Funktion auf?“) |

---

## 1. Installation
```sh
python3 -m venv venv
venv/bin/pip install pefile capstone
```
- **pefile** liest das Windows-Dateiformat PE (Portable Executable): Kopf, Abschnitte, Import- und Exporttabellen, Ressourcen.
- **capstone** übersetzt Maschinencode in Assembler-Text (x86, 32 Bit).

⚠️ Die Datei darf **nicht** `dis.py` heißen: Python hat ein eingebautes Modul `dis`, das sonst überdeckt wird (das ist hier passiert, daher der Name `rdis.py`).

## 2. Grundbegriffe des PE-Formats (nötig zum Verstehen der Ausgabe)
| Begriff | Bedeutung | Beispiel BEAGLink.dll |
|---|---|---|
| **ImageBase** | Adresse, an die Windows die DLL lädt. Alle Adressen in der Ausgabe sind *virtuelle Adressen* (VA) = ImageBase + Abstand | `0x10000000` |
| **Abschnitt (Section)** | `.text` = Code (ausführbar), `.rdata` = Konstanten/Strings, `.data` = globale Variablen | Code ab `0x10001000` |
| **Export** | Funktionen, die andere Programme aufrufen dürfen, mit Namen | `A1xxWriteDesign` = `0x10009fa0` |
| **Import** | Funktionen aus anderen DLLs; im Code als `call dword ptr [Adresse]` über die Import-Tabelle | `[0x100351cc]` = `KERNEL32.dll!CreateFileA` |
| **globale Variable** | feste Adresse im `.data`-Bereich | `[0x100400d8]` = Baudrate |

`rdis.py` lädt die DLL mit `pe.get_memory_mapped_image()` so, wie Windows sie im Speicher anordnen würde. Dadurch lässt sich jede VA direkt nachschlagen (`img[va - ImageBase]`).

## 3. Aufbau von `rdis.py` (Klasse `Bin`)
| Methode | Was sie tut |
|---|---|
| `__init__(pfad)` | PE laden, Speicherabbild bauen, **Importtabelle** (Adresse → `DLL!Funktion`) und **Exporttabelle** (Adresse → Name) einlesen, capstone für x86-32 einrichten |
| `rd(va, n)` | n Bytes an virtueller Adresse lesen |
| `str_at(va)` | prüfen, ob an `va` ein ASCII- oder UTF-16-Text beginnt (mind. 3 Zeichen, 0-terminiert) |
| `ann(ins)` | **Kommentar erzeugen**: Jede Hex-Zahl ≥ 5 Stellen im Operanden wird nachgeschlagen: Import → `KERNEL32.dll!ReadFile`, Export → `EXP:Name`, String → `'Text'` bzw. `L'Text'`, Gleitkommakonstante bei `fld/fmul…` → `dbl=0.001`, Sprung in die Import-Sprungtabelle (`jmp [..]`) → `->DLL!Name` |
| `dis(va, n, stop_ret)` | ab `va` bis zu n Befehle disassemblieren; standardmäßig Ende beim ersten `ret` nach mind. 4 Befehlen |
| `find_refs(va)` | alle Stellen im Code, an denen die 4 Bytes von `va` vorkommen (Zeiger/Konstanten) |
| `find_str(text, wide)` | alle Fundstellen eines Textes (ASCII oder UTF-16) |
| `func_start(va)` | heuristisch den Funktionsanfang vor `va` suchen (Prolog `55 8b ec` oder Befehl nach Füllbytes `cc/c3/90`) |

## 4. Befehle von `rdis.py`
```sh
python3 rdis.py DATEI exports                 # alle exportierten Funktionen mit Adresse
python3 rdis.py DATEI dis ADRESSE [n]         # Funktion ab ADRESSE (hex, ohne 0x), Ende beim ret
python3 rdis.py DATEI dis ADRESSE n x         # n Befehle OHNE Stopp beim ret (4. Argument beliebig)
python3 rdis.py DATEI xstr TEXT               # wo steht TEXT (ASCII) und welcher Code benutzt ihn
python3 rdis.py DATEI xstr TEXT w             # dasselbe für UTF-16 (Windows-Unicode)
python3 rdis.py DATEI xref ADRESSE            # wo im Code wird ADRESSE als Konstante benutzt
python3 rdis.py DATEI full > liste.asm        # komplette Disassembly aller Code-Abschnitte
```
**Beispiel – Exporte:**
```
$ python3 rdis.py BEAGLink.dll exports
0x10001540 _InitCOM
0x10009fa0 A1xxWriteDesign
0x10013710 McSendInstr
...
```
**Beispiel – String-Referenz finden:**
```
$ python3 rdis.py DEFMGR.DLL xstr 'DEFAULTS.DFL' w
str at 0x10016122                    ← Text liegt hier, aber kein Code greift direkt darauf zu
$ python3 rdis.py IMPORT.DLL xstr a885f60d87e90b71c20a81bcc3498af1
str at 0x1018cf28
   ref at 0x10114e17 func~ 0x10114dc0   ← Code bei 0x10114e17 benutzt den String (in Funktion ab 0x10114dc0)
```
**Beispiel – Funktion lesen** (Ausgabeformat: `Adresse  Befehl  Operanden   ; Kommentar`):
```
$ python3 rdis.py BEAGLink.dll dis 10013710
10013710  lea     eax, [esp + 4]
10013714  push    1
10013716  push    eax
10013717  call    0x10013720
1001371c  add     esp, 8
1001371f  ret
```

## 5. `asmq.py` – Abfragen auf der kompletten Liste
Die komplette Liste (`full`) hat bei BEAGLink.dll rund 76 000 Zeilen. `asmq.py` arbeitet darauf:
```sh
python3 asmq.py fn LISTE ADRESSE     # ganze Funktion: läuft über mehrere ret hinweg, solange
                                     #   Sprünge noch hinter das ret zeigen; stoppt an Füllbytes
python3 asmq.py w  LISTE VARIABLE    # alle Befehle, die [VARIABLE] SCHREIBEN (mov/inc/or … [0x…], …)
python3 asmq.py r  LISTE VARIABLE    # alle Zugriffe auf [VARIABLE]
python3 asmq.py fs LISTE ADRESSE     # Funktionsanfang zu einer Adresse
```
**Beispiel:** Wo wird das Protokoll-Flag gesetzt?
```
$ python3 asmq.py w beaglink.asm 100426b0
10001e27  mov     byte ptr [0x100426b0], bl     ← in der Synchronisierung (auf 0)
10013c77  mov     byte ptr [0x100426b0], al     ← in McReadVersion (auf 1)
```
## 6. `callers.py` – wer ruft eine Funktion auf?
Sucht im Code-Abschnitt jedes `E8 rel32` (call) bzw. `E9 rel32` (jmp), dessen Ziel `Adresse + 5 + rel32` gleich der gesuchten Adresse ist.
```
$ python3 callers.py DEFMGR.DLL 1000af40
0x1000aebc call func~ 0x1000ae50
```
**Grenze:** Indirekte Aufrufe über Zeiger (virtuelle Methoden `call dword ptr [eax+8]`, Funktionszeiger in Tabellen) findet es nicht. Dafür sucht man die Adresse als Zahl im ganzen Abbild (`xref` bzw. Python `img.find(struct.pack('<I', adr))`), siehe Beispiel 8.3.

---

## 7. x86-Code lesen – das Nötigste
### Register und Speicher
- `eax, ebx, ecx, edx, esi, edi, ebp` = 32-Bit-Register; `al/ah` = unteres/zweites Byte von eax, `ax` = untere 16 Bit.
- `esp` = Stapelzeiger. `[esp + 4]` ist beim Funktionsbeginn das **erste Argument**, `[esp + 8]` das zweite …
- `byte/word/dword ptr [x]` = 1/2/4 Byte an Adresse x. `[0x100400d8]` = globale Variable.
- Rückgabewert steht in `eax` (bzw. `al`/`ax`).

### Aufrufkonventionen
| Konvention | Erkennungsmerkmal | Vorkommen |
|---|---|---|
| **cdecl** | Aufrufer räumt auf: `call X` gefolgt von `add esp, 4·Argumente` | die meisten Funktionen hier |
| **stdcall** | Funktion räumt auf: `ret 0x10` (= 4 Argumente) | Windows-API, Exporte mit `@n` |
| **thiscall** | Objektzeiger in `ecx` vor dem `call` (`mov ecx, 0x1006afec`) | C++-Methoden (`Image`-Klasse) |

**Argumente werden rückwärts auf den Stapel gelegt.** `push 0 / push 0x3e8 / push 0x3e8 / push 0x10 / call 0x10002f00` heißt also `f(0x10, 0x3e8, 0x3e8, 0)`. Das letzte `push` vor dem `call` ist das **erste** Argument.

### Wichtige Muster (alle kommen in dieser Analyse vor)
| Muster | Bedeutung |
|---|---|
| `test eax, eax` / `je X` | wenn eax == 0, springe |
| `xor eax, eax` | eax = 0 |
| `neg eax` / `sbb eax, eax` / `inc eax` | eax = (eax == 0) ? 1 : 0 |
| `neg al` / `sbb eax, eax` / `and eax, K` / `add eax, C` | eax = (al ≠ 0) ? (K + C) : C (verzweigungsfreie Auswahl). **Beispiel** 0x1000d3cb: Kartenadresse = Binärprotokoll ? 0 : 0x280000 |
| `setne cl` | cl = 1, wenn ungleich |
| `mov eax, 0x4bda12f7` / `imul ebx` / `sar edx, 3` | **Division durch Konstante** per Multiplikation: hier `ebx / 27` (Bankberechnung) |
| `shl eax, 5` / `add eax, edi` / `lea ecx, [edi + eax*2]` | Multiplikation mit Konstante: hier `edi·67`, dann `·2` = `·134` = 0x86 (Farbeintragsgröße) |
| `rep movsd` | Speicher kopieren (`ecx` × 4 Byte von `esi` nach `edi`) |
| `rep stosd` | Speicher füllen (mit `eax`) |
| `repne scasb` + `not ecx` + `dec ecx` | `strlen` |
| `mov eax, fs:[0]` / `push -1` / `push X` | C++-Ausnahmerahmen (SEH) am Funktionsanfang, für die Logik unwichtig |
| `jmp dword ptr [ecx*4 + 0x10004c54]` | **Sprungtabelle** (switch/case); die Ziele stehen als Zeiger an der Tabellenadresse |
| `cmp eax, 0x1000` nach `ReadFromCOM` | 0x1000 = Kennwert „keine Daten“ in BEAGLink |

### Stapel-Offsets richtig zählen
Jedes `push` verschiebt `esp` um 4. Ein `[esp + 0x10]` nach drei `push` meint deshalb etwas anderes als vorher. In großen Funktionen (`sub esp, 0x1022c`) zählt man ab dem Funktionsanfang mit. Beispiel 0x100032b0: Der Kopf-Chunk wird nach `[esp + 0x108]` kopiert. `[esp + 0x21e]` ist daher Kopf-Offset 0x116.

## 8. Vorgehen an echten Beispielen aus dieser Analyse
### 8.1 Von einem Text zur Funktion (DEFAULTS.DFL)
1. `strings`-Suche: DEFMGR.DLL enthält „DEFAULTS.DFL“ und „inflate 1.1.3“.
2. `xstr '1.1.3'` → Referenz bei 0x1000c454 (das ist die Operandenstelle im Befehl ab 0x1000c452, zlib-Versionsprüfung in `inflateInit_` ab 0x1000c440). `xref` meldet Byte-Positionen, nicht unbedingt Befehlsanfänge.
3. `callers.py … 1000c550` → 0x1000af40 → `callers.py … 1000af40` → 0x1000ae50 → … → 0x100011d9.
4. `dis 10001100 140 x` zeigt: Datei lesen (`CFile::Read`), dann `call 0x100017a0`, dann Entpacken. → 0x10001760 ist die Entschlüsselung (7 Zeilen XOR).

### 8.2 Von einer Variablen zur Bedeutung (Protokoll-Flag)
1. In `A1xxWriteDesign` fällt `cmp byte ptr [0x100426b0], 0` auf.
2. `asmq.py w … 100426b0` → nur zwei Schreibstellen → beide Funktionen lesen → McReadVersion setzt 1 nach erfolgreicher Binärabfrage.

### 8.3 Virtuelle Methoden finden (Twofish in IMPORT.DLL)
1. Die Funktionen 0x10114dc0/0x10114e80 haben keine direkten Aufrufer.
2. Nach den 4 Bytes ihrer Adresse im Abbild suchen → Fundstelle 0x10154a5c/0x10154a60 = **vtable** (Tabelle von Methodenzeigern).
3. `xref 10154a58` (vtable-Anfang) → Konstruktor 0x10114d30 schreibt die vtable in das Objekt und baut den Schlüssel mit `sprintf("1d43f2ae%08x%08x%08x")`.

### 8.4 Briefkasten-Kommandos zuordnen
```python
# alle Aufrufe von 0x10002f00 mit ihren Argumenten
for i, l in enumerate(zeilen):
    if 'call    0x10002f00' in l:
        pushes = [z for z in zeilen[i-9:i] if ' push ' in z]
        kommando = pushes[-1]      # letztes push = 1. Argument
```
Daraus entstand die Kommandotabelle in [02](02_maschinenspeicher_und_kommandos.md).

## 9. Grenzen und typische Fallen
- **Lineares Disassemblieren:** `full` übersetzt den Code-Abschnitt von vorn nach hinten. Liegen Daten zwischen den Funktionen (z. B. Sprungtabellen direkt hinter einer Funktion, wie bei 0x10004c52), erscheinen sie als sinnlose Befehle. `asmq.py fn` hört an Füllbytes auf, gibt solche Reste aber manchmal mit aus.
- **`func_start` ist heuristisch.** Bei Funktionen ohne Standardprolog kann der Anfang falsch sein; im Zweifel mit `dis ADRESSE 40 x` von etwas weiter vorn lesen.
- **Indirekte Aufrufe** (`call eax`, `call [edx+4]`) zeigen das Ziel nicht. Man muss den Wert im Register zurückverfolgen.
- **Strings über Zeiger:** Viele Texte stehen nur als Zeiger in Tabellen (z. B. Namenstabellen in AESLINK.DLL); `xstr` findet dann „str at“, aber keine „ref at“-Zeile. Dann nach dem Zeiger suchen.
- **C++-Namen** wie `?Read@InStreamBuf@@QAEIPAXI@Z` sind „dekorierte“ Namen: Klasse `InStreamBuf`, Methode `Read`. Sie verraten viel über die Funktion.
- **Big- vs. Little-Endian:** Der PC (x86) speichert Zahlen Little-Endian. Die Bernina-Maschine erwartet ihre Speicherwerte Big-Endian (Hilfsfunktionen 0x10003dd0/0x10003db0 drehen die Bytes).

## 10. Kurzreferenz: wichtige Adressen in BEAGLink.dll
| Adresse | Name/Funktion |
|---|---|
| 0x10001610 | Port öffnen, DCB setzen |
| 0x10001d00 | Synchronisieren (48 Versuche, Baudwechsel) |
| 0x100027d0 / 0x10002750 | Zeichen mit Echo senden / Byte als 2 Hexziffern |
| 0x1000250a | `Lese32Bytes` |
| 0x10002f00 | Briefkasten-Kommando an 0xFFFED0 |
| 0x10005980 | Maschine einlesen (SRMV, Verwaltungsblock) |
| 0x10005e30 | Transfermodus betreten (TrME, YQ) |
| 0x100032b0 | Design schreiben (Kern) |
| 0x10004310 | Design lesen |
| 0x10006dd0 / 0x10006ea0 | Löschen / Sticken |
| 0x10013720 / 0x100137a0 | Binärrahmen bauen/senden |
| 0x10013b00 | `McReadVersion` (Geräteerkennung) |
| 0x1000a370 / 0x1000a4c0 | BE-Verschleierung |
| 0x10068e30 | aktueller Fehlercode |
| 0x100400d8 | aktuelle Baudrate |
| 0x100426b0 | Binärprotokoll aktiv |
| 0x10072028 | Kopie des Verwaltungsblocks |
