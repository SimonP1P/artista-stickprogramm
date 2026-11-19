<#
  artista_design_senden.ps1  –  VERSION 4.0 – EIN DESIGN AN DIE BERNINA artista 185 SENDEN

  Zweck: ein Design in den Übertragungspuffer DATEN schreiben, zurücklesen und vergleichen und – nur auf ausdrücklichen
  Wunsch – mit Befehl 0x08 „sticken ohne speichern“ starten. Es wird NICHTS dauerhaft gespeichert (kein 0x10, kein
  0x200), nichts gelöscht (0x800) und nichts formatiert (0x80).

  Ablauf wie die Original-Software beim „Embroider“ (BEAGLink.dll A1xxWriteDesign mit Ziel 3 -> 0x100032b0 ohne die
  Befehle 0x10/0x200, dann 0x10006ea0; Analyse_Teil2/03, 07):
    1. Verbinden wie Version 3.6 (T r M E, 57600 Baud, Y Q zum Stickmodul), Verwaltungsblock lesen (DATEN, INFO …)
    2. Größenprüfung (Original-Fehler 31 „Design is too big“)
    3. Puffer DATEN schreiben wie WriteComBytes (0x100029d0):
         bis zur nächsten 256-Byte-Grenze 'W' + 3 Adressbytes + ≤ 32 Datenbytes (Hex mit Echo) + '?'
         dann 256-Byte-Seiten: 'P' 'S' + Adressbits 23–8 (Hex mit Echo) -> 'O' 'E' -> 256 Byte binär -> 'O' (0x10002ce0)
         Rest wieder mit 'W'. Jedes Stück bis zu 5-mal wiederholt (nach 'R' 'F' '?').
    4. DATEN zurücklesen ('R'/'N' wie Version 3.6) und Byte für Byte vergleichen
    5. INFO-Felder: INFO+(N+1) = Farbanzahl−1, INFO+(2N+1) = Flags, INFO+(3N+1) = Name (32 Byte)   (Schalter -OhneInfo: nicht)
    6. nur mit -Sticken: Befehl 0x08 an 0xFFFED0 (Maschine übernimmt das Design zum Sticken)
    7. PC-Modus verlassen: Befehl 0x100, danach T r M E (wie das Original; erst dann ist das Display wieder an
       und nach 0x08 erscheint das Design – Gerätetest 19.11.2026)
  Ohne -Schreiben ist es ein TROCKENLAUF: nur verbinden, lesen, prüfen und anzeigen, was geschrieben würde.

  Der Puffer kommt aus dem Rechenkern:   stick puffer DESIGN.json | DATEI.exp | Design_NrXX_*.bin  -o NAME_puffer.bin
  (schreibt NAME_puffer.bin = Bytes für DATEN und NAME_puffer.txt = Längen und INFO-Felder).

  Aufruf:   SELBSTTEST.bat                                      (ohne Maschine)
            START_TROCKENLAUF.bat [PUFFER.bin]                  (nur prüfen, nichts schreiben)
            START_SCHREIBEN.bat   [PUFFER.bin]                  (DATEN + INFO schreiben, zurücklesen, kein Sticken)
            START_STICKEN.bat     [PUFFER.bin]                  (wie oben, danach Befehl 0x08)
            ohne PUFFER.bin: Nr47_puffer.bin (kleines Design aus dem Maschinenspeicher, 328 Byte)
  Schreiben und Sticken nur nach ausdrücklicher Zustimmung des Nutzers (CLAUDE.md Regel 1a).
#>
param(
    [string]$Port = "",
    [int]$Baud = 19200,
    [switch]$DtrRts,
    [string]$Puffer = "",          # NAME_puffer.bin aus "stick puffer"; daneben NAME_puffer.txt mit den INFO-Feldern
    [switch]$Schreiben,            # erst damit wird geschrieben (DATEN + INFO), sonst nur Trockenlauf
    [switch]$Sticken,              # danach Befehl 0x08 (sticken ohne speichern) – nur zusammen mit -Schreiben
    [switch]$OhneInfo,             # INFO-Felder nicht schreiben (nur DATEN)
    [switch]$OhneNachfrage,        # J/N-Abfragen überspringen (nur wenn der Nutzer das Schreiben ausdrücklich freigegeben hat)
    [switch]$NurR,                 # Zurücklesen nur mit 'R' (32 Byte), nicht mit 'N' (256 Byte)
    [switch]$NurW,                 # Schreiben nur mit 'W' (32 Byte), nicht mit 'P' 'S' (256 Byte)
    [switch]$Simulation,
    [int]$SimBaud = 19200,         # nur für den Selbsttest: Baudrate der simulierten Maschine
    [switch]$SimOhneModul,         # nur für den Selbsttest: Stickmodul fehlt
    [int]$SimDesigns = 30,         # nur für den Selbsttest
    [switch]$SimMitKarte,          # nur für den Selbsttest
    [int]$SimStoerung = 0,         # nur für den Selbsttest: beim n-ten 'N' geht ein Echo verloren
    [int]$SimStoerungSchreiben = 0 # nur für den Selbsttest: beim n-ten 'P' 'S' antwortet die Maschine nicht mit 'E'
)

$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$logFile = Join-Path $here "Mitschnitt_v4_$stamp.log"
$resFile = Join-Path $here "Ergebnis_v4_$stamp.txt"
$script:sp = $null
$script:baud = $Baud
$script:portName = ''
$script:result = New-Object System.Collections.Generic.List[string]
$script:stumm = $false          # true: Bytes beim Massenlesen/-schreiben nicht einzeln protokollieren

# ---------------------------------------------------------------- Ausgabe / Protokoll
$script:logW = New-Object System.IO.StreamWriter($logFile, $false, (New-Object System.Text.UTF8Encoding($true)))
$script:logW.AutoFlush = $true
function Write-Log([string]$text) {
    $script:logW.WriteLine((Get-Date -Format 'HH:mm:ss.fff') + '  ' + $text)
}
function Say([string]$text, [string]$color = 'Gray') {
    Write-Host $text -ForegroundColor $color
    Write-Log ('# ' + $text)
    $script:result.Add($text)
}
function Show-Byte([int]$b) {
    if ($b -ge 0x20 -and $b -lt 0x7f) { return ('{0:X2} ''{1}''' -f $b, [char]$b) }
    return ('{0:X2}' -f $b)
}
function Hex([byte[]]$data) { return (($data | ForEach-Object { '{0:X2}' -f $_ }) -join ' ') }
function Ascii([byte[]]$data) {
    return -join ($data | ForEach-Object { if ($_ -ge 0x20 -and $_ -lt 0x7f) { [char]$_ } else { '.' } })
}


# ---------------------------------------------------------------- Simulierte Maschine (nur -Simulation)
$script:simQ = New-Object System.Collections.Generic.Queue[byte]
$script:simAddrMode = 0        # 0 = frei, sonst Anzahl erwarteter Hexziffern
$script:simHex = ''
$script:simMem = @{}
$script:simLast = ''
$script:simSimBaud = $SimBaud
$script:simBaudPending = 0
$script:simTransfer = $false
$script:simZiel = 0
function Sim-Put([int]$addr, [byte[]]$data) { for ($i = 0; $i -lt $data.Length; $i++) { $script:simMem[$addr + $i] = $data[$i] } }
$script:simStage = 'NM'          # NM = Nähmaschine antwortet, SR = Stickmodul durchgeschaltet
$script:simBaudSwitch = $null    # Stopwatch nach 'J'-Kommando
$script:simMemNM = @{}
function Sim-Init {
    $nm = [Text.Encoding]::ASCII.GetBytes('NMMV04.04') + [byte[]](0) + [Text.Encoding]::ASCII.GetBytes('German      ') + [byte[]](0) + [Text.Encoding]::ASCII.GetBytes('Fritz Gegauf SIM')
    for ($i = 0; $i -lt $nm.Length; $i++) { $script:simMemNM[0x200100 + $i] = $nm[$i] }
    Sim-Put 0x200100 ([Text.Encoding]::ASCII.GetBytes('SRMV 3.09 SIMULATION'))
    # Verwaltungsblock: N=8, 20, Groesse, INFO, AUSWAHL, ICON, DATEN, ENDE1, ENDE2 (u32 Big-Endian)
    # Verwaltungsblock wie an der echten artista 185 gelesen (18.11.2026)
    $blk = [byte[]](0x1C,0x3C, 0,1,0xB3,0x42, 0,2,0x40,0x80, 0,2,1,0xEA, 0,2,0x44,0x80, 0,2,0x8E,0x9A, 0,1,0xFD,0x7A, 0,2,0x38,0x12)
    Sim-Put 0xFFFD24 $blk
    Sim-Put 0x024004 ([byte[]](0,0xFF,0xFE,0xD4)); Sim-Put 0xFFFED4 ([byte[]](0,0,0,0,0,0x86,0,0x04))
    Sim-Put 0xFFFEE0 ([byte[]](0))                                  # wie echte 185: Transfer nicht aktiv
    Sim-Put 0x024040 ([byte[]](0,4,1,0)); Sim-Put 0x024044 ([byte[]](0,4,2,0)); Sim-Put 0x024048 ([byte[]](0,4,3,0))
}
$script:simW = 0; $script:simWHex = ''
$script:simP = 0; $script:simPHex = ''; $script:simPAddr = 0; $script:simPZahl = 0; $script:simGestickt = $false
$script:simPDaten = New-Object System.Collections.Generic.List[byte]
function Sim-Count { if ($script:simZiel -eq 0x50) { return 3 }; return $SimDesigns }   # Karte: 3 Designs
function Sim-Names([int]$bank) {
    $info = 0x024080; $n = 28
    $count = [Math]::Max(0, [Math]::Min(27, (Sim-Count) - 27 * $bank))
    for ($k = 0; $k -lt 27; $k++) {
        $nm = New-Object byte[] 32
        if ($k -lt $count) { $t = [Text.Encoding]::ASCII.GetBytes(('SIM DESIGN {0:D2}' -f (27 * $bank + $k + 1))); [Array]::Copy($t, $nm, $t.Length) }
        Sim-Put ($info + 3 * $n + 1 + 32 * $k) $nm
        Sim-Put ($info + 2 * $n + 1 + $k) ([byte[]](0x08))
        Sim-Put ($info + $n + 1 + $k) ([byte[]](2))
    }
}
function Sim-Design {
    # Kopf 6N+8 = 176 Byte (N = 28): drei u16-Tabellen, dann u32 Länge 1 (Stiche) und u32 Länge 2 (Zusatz)
    $k = New-Object byte[] 176
    $k[1] = 2; $k[3] = 5                                                 # Tabelle 1: zwei Einträge
    $k[168] = 0; $k[169] = 0; $k[170] = 0x0B; $k[171] = 0xB8             # Länge 1 = 3000
    $k[172] = 0; $k[173] = 0; $k[174] = 0; $k[175] = 200                 # Länge 2 = 200
    Sim-Put 0x028E9A $k
    $d = New-Object byte[] 3200; for ($i = 0; $i -lt 3200; $i++) { $d[$i] = [byte]($i % 251) }
    $d[3000] = 0; $d[3001] = 0; $d[3002] = 200; $d[3003] = 0; $d[3004] = 0; $d[3005] = 0       # Anhang: Containerkopf, Länge 200
    Sim-Put (0x028E9A + 176) $d
    $ic = New-Object byte[] 42; $ic[2] = 8; $ic[3] = 4; for ($i = 6; $i -lt 42; $i++) { $ic[$i] = 0xAA }
    Sim-Put 0x024480 $ic
}
function Sim-WriteDone {
    $h = $script:simWHex; if ($h.Length -lt 6) { return }
    $addr = [Convert]::ToInt32($h.Substring(0, 6), 16)
    $data = for ($i = 6; $i + 1 -lt $h.Length; $i += 2) { [Convert]::ToByte($h.Substring($i, 2), 16) }
    Sim-Put $addr ([byte[]]@($data))
    if ($addr -eq 0xFFFED0) {                                        # Befehl ins Befehlsfeld
        $cmd = (([int]$data[0]) -shl 8) + [int]$data[1] - 1
        $bank = 0; if ($script:simMem.ContainsKey(0x0201EA + 5)) { $bank = $script:simMem[0x0201EA + 5] }
        if ($cmd -in 0x50, 0xA0) { $script:simZiel = $cmd; Sim-Put 0xFFFED0 ([byte[]](0, 2)); return }
        if ($cmd -eq 0x100) { Sim-Put 0xFFFED0 ([byte[]](0, 0)); return }       # wie am Gerät (3.3): 0x0000
        if ($cmd -eq 0x400) { Sim-Design; Sim-Put 0xFFFED0 ([byte[]](0, 2)); return }
        if ($cmd -eq 0x08) { $script:simGestickt = $true; Sim-Put 0xFFFED0 ([byte[]](0, 2)); return }   # sticken: fertig, ok
        # Annahme 3.3: 0x50 = Karte. Ohne Karte -> Status 0x1005 wie im Gerätetest v3
        if ($cmd -in 0x30, 0x20, 0x60, 0xC0 -and $script:simZiel -eq 0x50 -and -not $SimMitKarte) { Sim-Put 0xFFFED0 ([byte[]](0x10, 5)); return }
        if ($cmd -eq 0x20) { Sim-Put 0x024080 ([byte[]]([Math]::Min(255, (Sim-Count)))); Sim-Names 0 }
        if ($cmd -in 0x30, 0x60, 0xC0) { Sim-Names $bank }
        Sim-Put 0xFFFED0 ([byte[]](0, 2))                            # Status: fertig, ok
    }
}
$script:simBlock = $false; $script:simNZahl = 0
function Sim-Rx([byte]$b) {
    if ($script:baud -ne $script:simSimBaud) { return }              # falsche Baudrate: Maschine versteht nichts
    if ($script:simP -eq 3) {                                        # 'P' 'S': 256 Byte binär, ohne Echo
        $script:simPDaten.Add($b)
        if ($script:simPDaten.Count -eq 256) { Sim-Put $script:simPAddr ($script:simPDaten.ToArray()); $script:simPDaten.Clear(); $script:simP = 0; $script:simQ.Enqueue([byte][char]'O') }
        return
    }
    $c = [char]$b
    # Störung (Selbsttest der Wiederholung): beim n-ten 'N' geht das Echo der ersten Adressziffer verloren,
    # die simulierte Maschine bricht den Befehl ab
    if ($SimStoerung -gt 0 -and $script:simBlock -and $script:simAddrMode -gt 0 -and $script:simHex.Length -eq 0 -and $script:simNZahl -eq $SimStoerung) {
        $script:simNZahl++; $script:simAddrMode = 0; $script:simBlock = $false; return
    }
    $script:simQ.Enqueue($b)                                         # Echo jedes Zeichens
    if ($script:simP -eq 1) { if ($c -ceq 'S') { $script:simP = 2; $script:simPHex = '' } else { $script:simP = 0 }; return }
    if ($script:simP -eq 2) {                                        # 4 Hexziffern = Adressbits 23–8
        if (-not '0123456789ABCDEF'.Contains([string]$c)) { $script:simP = 0; return }
        $script:simPHex += $c
        if ($script:simPHex.Length -eq 4) {
            $script:simPZahl++; $script:simPAddr = [Convert]::ToInt32($script:simPHex, 16) -shl 8
            if ($SimStoerungSchreiben -gt 0 -and $script:simPZahl -eq $SimStoerungSchreiben) { $script:simQ.Enqueue([byte][char]'O'); $script:simP = 0; return }   # Störung: kein 'E'
            $script:simP = 3; $script:simQ.Enqueue([byte][char]'O'); $script:simQ.Enqueue([byte][char]'E')
        }
        return
    }
    if ($script:simW -eq 1) {                                         # Schreiben: 6 Hex Adresse, dann Hexpaare, Ende '?'
        if ($c -ceq '?') { Sim-WriteDone; $script:simW = 0; return }
        if ('0123456789ABCDEF'.Contains([string]$c)) { $script:simWHex += $c }
        return
    }
    if ($script:simAddrMode -gt 0) {
        if ('0123456789ABCDEF'.Contains([string]$c)) {
            $script:simHex += $c
            if ($script:simHex.Length -eq $script:simAddrMode) {
                $addr = [Convert]::ToInt32($script:simHex, 16); $script:simAddrMode = 0
                $mem = $script:simMem; if ($script:simStage -eq 'NM') { $mem = $script:simMemNM }
                if ($script:simBlock) {                              # 'N': 256 Byte binär, dann 'O'
                    for ($i = 0; $i -lt 256; $i++) { $v = 0; if ($mem.ContainsKey($addr + $i)) { $v = $mem[$addr + $i] }; $script:simQ.Enqueue([byte]$v) }
                } else {
                    for ($i = 0; $i -lt 32; $i++) {
                        $v = 0; if ($mem.ContainsKey($addr + $i)) { $v = $mem[$addr + $i] }
                        foreach ($h in ('{0:X2}' -f $v).ToCharArray()) { $script:simQ.Enqueue([byte][char]$h) }
                    }
                }
                $script:simQ.Enqueue([byte][char]'O'); $script:simBlock = $false
            }
        } else { $script:simAddrMode = 0; $script:simBlock = $false }
        return
    }
    $script:simLast = ($script:simLast + $c)
    if ($script:simLast.Length -gt 8) { $script:simLast = $script:simLast.Substring($script:simLast.Length - 8) }
    if ($script:simLast.EndsWith('TrME')) { $script:simTransfer = $true; return }
    if (-not $script:simTransfer) { return }                         # Lesebefehle erst im Transfermodus
    if ($script:simLast.EndsWith('J05')) { $script:simBaudPending = 57600; $script:simBaudSwitch = [Diagnostics.Stopwatch]::StartNew(); return }
    if ($script:simLast.EndsWith('J02')) { $script:simBaudPending = 115200; $script:simBaudSwitch = [Diagnostics.Stopwatch]::StartNew(); return }
    if ($script:simLast.EndsWith('J11')) { $script:simBaudPending = 19200; $script:simBaudSwitch = [Diagnostics.Stopwatch]::StartNew(); return }
    if ($script:simLast.EndsWith('YQ') -and $script:simStage -eq 'NM' -and -not $SimOhneModul) { $script:simStage = 'SR'; $script:simQ.Enqueue([byte][char]'O'); return }
    if ($c -ceq 'J') { return }
    if ($c -ceq 'W') { $script:simW = 1; $script:simWHex = ''; return }
    if ($c -ceq 'P') { $script:simP = 1; return }
    if ($c -ceq 'R') { $script:simAddrMode = 6; $script:simHex = ''; $script:simBlock = $false }
    elseif ($c -ceq 'N' -and $script:baud -ne 4800) { $script:simAddrMode = 6; $script:simHex = ''; $script:simBlock = $true; $script:simNZahl++ }
    elseif ($c -ceq 'E' -and $script:baud -eq 4800) { $script:simAddrMode = 4; $script:simHex = '' }
}

# ---------------------------------------------------------------- Port
function Open-Port([string]$name, [int]$baud) {
    if ($Simulation) { Write-Log "PORT  SIMULATION $baud Baud"; return }
    if ($null -ne $script:sp) { try { $script:sp.Close() } catch {} }
    $p = New-Object System.IO.Ports.SerialPort $name, $baud, ([System.IO.Ports.Parity]::None), 8, ([System.IO.Ports.StopBits]::One)
    $p.Handshake = [System.IO.Ports.Handshake]::None
    $p.DtrEnable = [bool]$DtrRts
    $p.RtsEnable = [bool]$DtrRts
    $p.ReadBufferSize = 4096; $p.WriteBufferSize = 4096
    $p.ReadTimeout = 50; $p.WriteTimeout = 5000
    $p.Open()
    $script:sp = $p
    Write-Log ("PORT  {0} geöffnet, {1} Baud 8N1, DTR/RTS={2}" -f $name, $baud, [bool]$DtrRts)
}
function Close-Port { if ($null -ne $script:sp) { try { $script:sp.Close() } catch {}; $script:sp = $null } }

function Tx([byte]$b) {
    if (-not $script:stumm) { Write-Log ('PC  -> MASCH  ' + (Show-Byte $b)) }
    if ($Simulation) { Sim-Rx $b; return }
    $script:sp.Write([byte[]]@($b), 0, 1)
}
function Rx([int]$timeoutMs) {
    $sw = [Diagnostics.Stopwatch]::StartNew()
    while ($true) {
        if ($Simulation -and $null -ne $script:simBaudSwitch -and $script:simBaudSwitch.ElapsedMilliseconds -ge 250 -and $script:baud -eq $script:simBaudPending) {
            $script:simBaudSwitch = $null; $SimBaud = $script:simBaudPending; $script:simSimBaud = $script:simBaudPending
            foreach ($ch in 'B','O','S') { $script:simQ.Enqueue([byte][char]$ch) }
        }
        if ($Simulation) { if ($script:simQ.Count -gt 0) { $b = $script:simQ.Dequeue(); if (-not $script:stumm) { Write-Log ('MASCH -> PC  ' + (Show-Byte $b)) }; return [int]$b } }
        elseif ($script:sp.BytesToRead -gt 0) { $b = $script:sp.ReadByte(); if (-not $script:stumm) { Write-Log ('MASCH -> PC  ' + (Show-Byte $b)) }; return [int]$b }
        if ($sw.ElapsedMilliseconds -ge $timeoutMs) { return -1 }
        if ($sw.ElapsedMilliseconds -lt 40) { [Threading.Thread]::SpinWait(500) } else { Start-Sleep -Milliseconds 1 }
    }
}
# Eingang leeren. Wie im Original (0x100027d0): kommt dabei ein druckbares Zeichen, ist der Puffer "nicht leer" (Fehler 14)
function Flush-In {
    $bad = $false
    while ($true) { $b = Rx 0; if ($b -lt 0) { break }; if ($b -ge 0x20 -and $b -le 0x7f) { $bad = $true } }
    return (-not $bad)
}
# Zeichen senden und Echo prüfen (Original 0x100027d0, Timeout 1000 ms)
function Send-Echo([char]$c, [int]$timeoutMs = 1000) {
    if (-not (Flush-In)) { Write-Log 'FEHLER 14 Puffer nicht leer'; return $false }
    Tx ([byte]$c)
    $r = Rx $timeoutMs
    if ($r -ne [int][byte]$c) { Write-Log ("FEHLER 15 falsches/kein Echo auf '{0}' (erhalten {1})" -f $c, $r); return $false }
    return $true
}
# Byte als zwei Hexziffern mit Echo (Original 0x10002750)
function Send-HexByte([int]$v) {
    $h = '{0:X2}' -f ($v -band 0xFF)
    return ((Send-Echo $h[0]) -and (Send-Echo $h[1]))
}
# zwei Hexziffern empfangen (Original 0x10002670, je 2000 ms)
function Rx-HexByte {
    $v = 0
    for ($k = 0; $k -lt 2; $k++) {
        $c = Rx 2000
        if ($c -ge 0x30 -and $c -le 0x39) { $v = ($v -shl 4) + ($c - 0x30) }
        elseif ($c -ge 0x41 -and $c -le 0x46) { $v = ($v -shl 4) + ($c - 0x37) }
        else { Write-Log "FEHLER 16 keine Hexziffer ($c)"; return -1 }
    }
    return $v
}
# 32 Byte lesen (Original Lese32Bytes 0x1000250a)
function Read-32([int]$addr) {
    $ok = $true
    if ($script:baud -eq 4800 -and $addr -lt 0x1000) {
        $ok = (Send-Echo 'E') -and (Send-HexByte ($addr -shr 8)) -and (Send-HexByte $addr)
    } else {
        $ok = (Send-Echo 'R') -and (Send-HexByte ($addr -shr 16)) -and (Send-HexByte ($addr -shr 8)) -and (Send-HexByte $addr)
    }
    if (-not $ok) { return $null }
    $buf = New-Object byte[] 32
    for ($i = 0; $i -lt 32; $i++) { $v = Rx-HexByte; if ($v -lt 0) { return $null }; $buf[$i] = [byte]$v }
    $o = Rx 1200
    if ($o -ne 0x4F) { Write-Log "FEHLER 16 Abschluss 'O' fehlt ($o)"; return $null }
    return ,$buf
}
function Read-Mem([int]$addr, [int]$len) {
    $out = New-Object System.Collections.Generic.List[byte]
    $a = $addr
    while ($out.Count -lt $len) { $b = Read-32 $a; if ($null -eq $b) { return $null }; $out.AddRange([byte[]]$b); $a += 32 }
    return ,($out.GetRange(0, $len).ToArray())
}
function U32BE([byte[]]$d, [int]$o) { return ([int64]$d[$o] -shl 24) + ([int64]$d[$o+1] -shl 16) + ([int64]$d[$o+2] -shl 8) + [int64]$d[$o+3] }

# ---------------------------------------------------------------- Synchronisieren (Original 0x10001d00, ohne Binärprobe)
function Next-Baud([int]$b) { switch ($b) { 4800 { 19200 } 19200 { 57600 } 57600 { 115200 } default { 4800 } } }
function Sync-Machine {
    for ($v = 0; $v -lt 48; $v++) {
        if ($v % 6 -eq 5) { $script:baud = Next-Baud $script:baud; Open-Port $script:portName $script:baud; Say ("  … wechsle auf {0} Baud" -f $script:baud) }
        if ($v -gt 0) { Start-Sleep -Milliseconds 50 }
        $null = Flush-In
        Tx ([byte][char]'R')
        $ans = -1; $tries = 5
        while ($true) {
            $b = Rx 0
            if ($b -eq 0x51) { continue }                          # 'Q' wird übersprungen
            if ($b -lt 0) { Start-Sleep -Milliseconds 20; $tries--; if ($tries -le 0) { break }; continue }
            $ans = $b; break
        }
        if ($ans -eq 0x52) {
            Tx ([byte][char]'F'); $a2 = Rx 100
            if ($a2 -eq 0x46) {
                Tx ([byte][char]'?'); $a3 = Rx 100
                if ($a3 -eq 0x3F) { Write-Log "SYNC ok nach $($v+1) Versuchen bei $($script:baud) Baud"; return $true }
            }
        }
    }
    return $false
}
# Transfermodus betreten (Original 0x10005e30): 'T' 'r' 'M' 'E' mit Echo
function Enter-Transfer {
    foreach ($c in 'T', 'r', 'M', 'E') { if (-not (Send-Echo ([char]$c))) { return $false } }
    return $true
}

# Baudrate mit der Maschine wechseln (Original ChangeBaudrate 0x100057b0)
function Change-Baud([int]$newBaud) {
    $code = @{ 19200 = 0x11; 57600 = 0x05; 115200 = 0x02 }[$newBaud]
    if ($null -eq $code) { return $false }
    if ($script:baud -eq $newBaud) { return $true }                 # wie Original: schon eingestellt
    if (-not (Send-Echo 'J')) { return $false }
    if (-not (Send-HexByte $code)) { return $false }
    $script:baud = $newBaud
    Open-Port $script:portName $newBaud
    $gotB = $false
    for ($i = 0; $i -lt 20; $i++) {
        while ((Rx 0) -ge 0) { }                                   # Eingang leeren
        Start-Sleep -Milliseconds 100
        if ((Rx 0) -eq 0x42) { $gotB = $true; break }              # 'B'
    }
    if (-not $gotB) { Write-Log "FEHLER 17 kein 'B' nach Baudwechsel"; return $false }
    if ((Rx 1200) -ne 0x4F) { Write-Log "FEHLER 16 'O' fehlt nach Baudwechsel"; return $false }
    if ((Rx 1200) -ne 0x53) { Write-Log "FEHLER 16 'S' fehlt nach Baudwechsel"; return $false }
    if (-not (Send-Echo 'E')) { return $false }
    if (-not (Send-Echo 'B')) { return $false }
    Write-Log "BAUD  jetzt $newBaud"
    return $true
}
# Zum Stickmodul durchschalten (Original 0x10005e30 mit Parameter 1): TrME, 57600 Baud, 'Y' 'Q' -> 'O'
function Switch-ToModule {
    if (-not (Enter-Transfer)) { Say '  TrME vor dem Umschalten nicht bestätigt' 'Yellow'; return $false }
    Say '  vereinbare 57600 Baud (J 0 5 … B O S … E B) …'
    if (-not (Change-Baud 57600)) { Say '  Baudwechsel fehlgeschlagen' 'Yellow'; return $false }
    Start-Sleep -Milliseconds 250
    Say '  schalte zum Stickmodul durch (Y Q) …'
    if (-not (Send-Echo 'Y')) { return $false }
    if (-not (Send-Echo 'Q')) { return $false }
    $o = Rx 3000
    if ($o -ne 0x4F) { Write-Log "FEHLER 30 Stickmodul antwortet nicht mit 'O' ($o)"; Say '  Stickmodul hat nicht mit O geantwortet (ist es angesteckt?)' 'Yellow'; return $false }
    return $true
}
# Bytes schreiben (Original 0x10002b20, ASCII): 'W' + 3 Adressbytes + Daten (Hex mit Echo) + '?' (Echo, Timeout)
function Write-Mem([int]$addr, [byte[]]$data, [int]$endTimeoutMs = 1000) {
    if (-not (Send-Echo 'W')) { return $false }
    if (-not ((Send-HexByte ($addr -shr 16)) -and (Send-HexByte ($addr -shr 8)) -and (Send-HexByte $addr))) { return $false }
    foreach ($d in $data) { if (-not (Send-HexByte $d)) { return $false } }
    if (-not (Send-Echo '?' $endTimeoutMs)) { return $false }
    return $true
}
# Befehl ans Befehlsfeld 0xFFFED0 (Original 0x10002f00): Wort (Befehl|1) Big-Endian schreiben, Status abfragen
function Invoke-Mailbox([int]$cmd, [int]$t1, [int]$t2, [int]$flag) {
    $w = $cmd -bor 1
    Write-Log ("BEFEHL 0x{0:X} an 0xFFFED0 (Flag {1})" -f $cmd, $flag)
    if (-not (Write-Mem 0xFFFED0 ([byte[]]@((($w -shr 8) -band 0xFF), ($w -band 0xFF))) ($t1 + 5000))) { return @{ ok = $false; st = -1 } }
    $sw = [Diagnostics.Stopwatch]::StartNew(); $st = -1
    while ($sw.ElapsedMilliseconds -lt $t2) {
        $r = Read-Mem 0xFFFED0 2
        if ($null -eq $r) { return @{ ok = $false; st = -1 } }
        $st = ([int]$r[0] -shl 8) + [int]$r[1]; $al = $st -band 0xFF; $ah = ($st -shr 8) -band 0xFF
        if (($al -band 6) -ne 0 -or ($cmd -eq 0x100 -and ($ah -band 1) -eq 0)) {
            Write-Log ("STATUS 0x{0:X4}" -f $st)
            if ($cmd -eq 8 -or $flag -ne 0) { return @{ ok = (($al -band 2) -ne 0); st = $st } }
            if ($ah -band 0xF0) { return @{ ok = $false; st = $st } }
            return @{ ok = (($al -band 4) -eq 0); st = $st }
        }
        Start-Sleep -Milliseconds 100
    }
    Write-Log ("STATUS Zeitüberschreitung, letzter Wert 0x{0:X4}" -f $st)
    return @{ ok = $false; st = $st }
}
function Status-Text([int]$st) {
    $ah = ($st -shr 8) -band 0xFF; $t = @()
    if ($ah -band 0x10) { $t += 'keine Speicherkarte im Stickmodul' }
    if ($ah -band 0x20) { $t += 'Karte nicht formatiert' }
    if ($ah -band 0x40) { $t += 'keine Flashkarte' }
    if ($ah -band 0x80) { $t += 'Karte voll / zu viele Designs' }
    if ($t.Count -eq 0) { return '' }; return ' (' + ($t -join ', ') + ')'
}

# n Byte binär empfangen, je Byte höchstens $timeoutMs warten (Original ReadByte 0x10001f40 mit 3000 ms)
function Rx-Bytes([int]$n, [int]$timeoutMs) {
    $buf = New-Object byte[] $n
    if ($Simulation) {
        for ($i = 0; $i -lt $n; $i++) { $v = Rx $timeoutMs; if ($v -lt 0) { Write-Log "FEHLER 16 Block unvollständig ($i von $n Byte)"; return $null }; $buf[$i] = [byte]$v }
        return ,$buf
    }
    $got = 0; $sw = [Diagnostics.Stopwatch]::StartNew()
    while ($got -lt $n) {
        $avail = $script:sp.BytesToRead
        if ($avail -gt 0) { $got += $script:sp.Read($buf, $got, [Math]::Min($avail, $n - $got)); $sw.Restart(); continue }
        if ($sw.ElapsedMilliseconds -ge $timeoutMs) { Write-Log "FEHLER 16 Block unvollständig ($got von $n Byte)"; return $null }
        if ($sw.ElapsedMilliseconds -lt 40) { [Threading.Thread]::SpinWait(500) } else { Start-Sleep -Milliseconds 1 }
    }
    return ,$buf
}
# 256 Byte lesen (Original 0x10002880): 'N' + 3 Adressbytes mit Echo, 256 Byte binär, dann 'O' (Fehler 0x10)
function Read-256([int]$addr) {
    $ok = (Send-Echo 'N') -and (Send-HexByte ($addr -shr 16)) -and (Send-HexByte ($addr -shr 8)) -and (Send-HexByte $addr)
    if (-not $ok) { return $null }
    $buf = Rx-Bytes 256 3000
    if ($null -eq $buf) { return $null }
    $o = Rx 1200
    if ($o -ne 0x4F) { Write-Log "FEHLER 16 Abschluss 'O' nach Block fehlt ($o)"; return $null }
    return ,$buf
}
# Nach einem Lesefehler: warten, Eingang leeren, mit 'R' 'F' '?' bei derselben Baudrate neu synchronisieren
# ('F' ist eine Hexziffer, '?' nicht -> ein angefangener Lesebefehl wird abgebrochen). Kein Baudwechsel.
function Resync {
    $alt = $script:stumm; $script:stumm = $false                  # Resync immer vollständig protokollieren
    try { return (Resync-Inner) } finally { $script:stumm = $alt }
}
function Resync-Inner {
    for ($v = 0; $v -lt 10; $v++) {
        Start-Sleep -Milliseconds (200 + 100 * $v)
        while ((Rx 0) -ge 0) { }
        Tx ([byte][char]'R'); $a1 = Rx 300
        while ($a1 -eq 0x51) { $a1 = Rx 300 }                    # 'Q' überspringen (wie Original)
        if ($a1 -ne 0x52) { continue }
        Tx ([byte][char]'F'); if ((Rx 300) -ne 0x46) { continue }
        Tx ([byte][char]'?'); if ((Rx 300) -ne 0x3F) { continue }
        Start-Sleep -Milliseconds 50; while ((Rx 0) -ge 0) { }
        Write-Log "RESYNC ok nach $($v + 1) Versuchen"
        return $true
    }
    Write-Log 'RESYNC fehlgeschlagen'
    return $false
}
# Größere Bereiche lesen wie das Original (ReadComBytes 0x10002040): erstes Stück mit 'R' (32 Byte), dann 'N'-Blöcke
# zu 256 Byte, solange noch mindestens 256 Byte fehlen, den Rest mit 'R'. Jedes Stück bis zu 5-mal wiederholt.
# $teil: Datei, in die jedes gelesene Stück sofort angehängt wird. Bytes werden nicht einzeln protokolliert.
function Read-Gross([int]$addr, [int]$len, [string]$was, [string]$teil = '') {
    $out = New-Object System.Collections.Generic.List[byte]
    $fs = $null
    if ($teil -ne '') { $fs = New-Object System.IO.FileStream($teil, [System.IO.FileMode]::Append, [System.IO.FileAccess]::Write) }
    $script:stumm = $true
    $sw = [Diagnostics.Stopwatch]::StartNew(); $wdh = 0; $nextLog = 16384
    try {
        while ($out.Count -lt $len) {
            $rest = $len - $out.Count; $a = $addr + $out.Count
            $block = (-not $NurR) -and ($script:baud -ne 4800) -and ($out.Count -gt 0) -and ($rest -ge 256)
            $b = $null
            for ($try = 0; $try -le 5 -and $null -eq $b; $try++) {
                if ($try -gt 0) {
                    $wdh++
                    Write-Log ("WIEDERHOLUNG {0} bei 0x{1:X} ({2})" -f $try, $a, $(if ($block) { 'N' } else { 'R' }))
                    if (-not (Resync)) { break }
                }
                if ($block) { $b = Read-256 $a } else { $b = Read-32 $a }
            }
            if ($null -eq $b) { Write-Log ("FEHLER beim Lesen bei 0x{0:X} (Offset {1})" -f $a, $out.Count); return $null }
            $n = [Math]::Min($b.Length, $rest)
            $stueck = New-Object byte[] $n; [Array]::Copy([byte[]]$b, $stueck, $n)
            $out.AddRange($stueck)
            if ($null -ne $fs) { $fs.Write($stueck, 0, $n); $fs.Flush() }
            if ($out.Count -ge $nextLog) { Write-Log ("FORTSCHRITT {0}: {1} von {2} Byte, {3:N1} s, Wiederholungen {4}" -f $was, $out.Count, $len, $sw.Elapsed.TotalSeconds, $wdh); $nextLog += 16384 }
            Write-Host ("`r  {0}: {1} von {2} Byte ({3:N0} s, Wiederholungen {4})   " -f $was, $out.Count, $len, $sw.Elapsed.TotalSeconds, $wdh) -NoNewline
        }
    } finally { $script:stumm = $false; Write-Host ''; if ($null -ne $fs) { $fs.Close() } }
    $rate = 0; if ($sw.Elapsed.TotalSeconds -gt 0) { $rate = $len / $sw.Elapsed.TotalSeconds }
    Write-Log ("GELESEN {0}: {1} Byte ab 0x{2:X} in {3:N1} s ({4:N0} Byte/s), Wiederholungen {5}" -f $was, $len, $addr, $sw.Elapsed.TotalSeconds, $rate, $wdh)
    Say ("  {0}: {1} Byte in {2:N1} s ({3:N0} Byte/s), Wiederholungen {4}" -f $was, $len, $sw.Elapsed.TotalSeconds, $rate, $wdh)
    return ,($out.ToArray())
}

function Exit-Transfer {
    $r = Invoke-Mailbox 0x100 2000 2000 1
    Write-Log ("Befehl 0x100 Status 0x{0:X4}" -f $r.st)
    # wie das Original (0x10005e30, Parameter 0): danach 'T' 'r' 'M' 'E' mit Echo – erst damit verlässt die artista 185
    # den PC-Modus (Gerätetest v4, 19.11.2026: Display wieder an, gesendetes Design erscheint, Rahmen fährt)
    $t = Enter-Transfer
    Write-Log ("Abschluss T r M E: {0}" -f $(if ($t) { 'bestätigt' } else { 'NICHT bestätigt' }))
    if (($r.ok -or $r.st -eq 0) -and $t) { return 'Befehl' }        # Gerätetest 3.3: die 185 antwortet auf 0x100 mit 0x0000
    return 'Fehler'
}
function Parse-Version([string]$txt) {
    $ver = 0
    foreach ($ch in $txt.Substring(4).ToCharArray()) { if ($ch -eq [char]0) { break }; if ($ch -ge '0' -and $ch -le '9') { $ver = (($ver - 3) -shl 4) + [int][char]$ch } }
    return $ver
}

# ---------------------------------------------------------------- Schreiben (neu in Version 4)
# 256 Byte schreiben (Original 0x10002ce0): 'P' 'S' + Adressbits 23–16 und 15–8 (Hex mit Echo), Antwort 'O' 'E',
# dann 256 Byte binär (ohne Echo), Antwort 'O'
function Write-256([int]$addr, [byte[]]$data) {
    if ($data.Length -ne 256 -or ($addr -band 0xFF) -ne 0) { Write-Log 'FEHLER Write-256: Seite nicht ausgerichtet'; return $false }
    $ok = (Send-Echo 'P') -and (Send-Echo 'S') -and (Send-HexByte ($addr -shr 16)) -and (Send-HexByte ($addr -shr 8))
    if (-not $ok) { return $false }
    $o = Rx 1200; if ($o -ne 0x4F) { Write-Log "FEHLER 16 'O' nach P S fehlt ($o)"; return $false }
    $e = Rx 1200; if ($e -ne 0x45) { Write-Log "FEHLER 16 'E' nach P S fehlt ($e)"; return $false }
    if ($Simulation) { foreach ($b in $data) { Sim-Rx $b } }
    else { $script:sp.Write($data, 0, 256) }
    if (-not $script:stumm) { Write-Log 'PC  -> MASCH  256 Byte binär' }
    $o = Rx 3000
    if ($o -ne 0x4F) { Write-Log "FEHLER 16 'O' nach dem 256-Byte-Block fehlt ($o)"; return $false }
    return $true
}
# Größeren Bereich schreiben wie das Original (WriteComBytes 0x100029d0): bis zur 256-Byte-Grenze 'W' mit ≤ 32 Byte,
# ab der Grenze 256-Byte-Seiten mit 'P' 'S', den Rest wieder mit 'W'. Jedes Stück bis zu 5-mal wiederholt.
# Flash-Bereiche (0x280000–0x3FFFFF, 0xC00000–0xDFFFFF) werden nie beschrieben.
function Write-Gross([int]$addr, [byte[]]$data, [string]$was) {
    $ende = $addr + $data.Length
    foreach ($fl in @(@(0x280000, 0x3FFFFF), @(0xC00000, 0xDFFFFF))) {
        if ($addr -le $fl[1] -and $ende -gt $fl[0]) { Say ("  ABBRUCH: Bereich 0x{0:X}–0x{1:X} berührt Flash – wird nie beschrieben" -f $addr, $ende) 'Red'; return $false }
    }
    $script:stumm = $true
    $sw = [Diagnostics.Stopwatch]::StartNew(); $wdh = 0; $pos = 0; $seiten = 0; $kleine = 0
    try {
        while ($pos -lt $data.Length) {
            $a = $addr + $pos; $rest = $data.Length - $pos
            $seite = (-not $NurW) -and ($script:baud -ne 4800) -and (($a -band 0xFF) -eq 0) -and ($rest -ge 256)
            if ($seite) { $n = 256 }
            else {
                $n = [Math]::Min(32, $rest)
                $bisGrenze = 0x100 - ($a -band 0xFF)
                if ($rest -ge $bisGrenze + 0x100 -and $bisGrenze -lt $n -and $bisGrenze -gt 0) { $n = $bisGrenze }
            }
            $stueck = New-Object byte[] $n; [Array]::Copy($data, $pos, $stueck, 0, $n)
            $ok = $false
            for ($try = 0; $try -le 5 -and -not $ok; $try++) {
                if ($try -gt 0) {
                    $wdh++
                    Write-Log ("WIEDERHOLUNG {0} beim Schreiben bei 0x{1:X} ({2})" -f $try, $a, $(if ($seite) { 'P S' } else { 'W' }))
                    if (-not (Resync)) { break }
                }
                if ($seite) { $ok = Write-256 $a $stueck } else { $ok = Write-Mem $a $stueck 1000 }
            }
            if (-not $ok) { Write-Log ("FEHLER beim Schreiben bei 0x{0:X} (Offset {1})" -f $a, $pos); Say ("  Schreiben bei Offset {0} fehlgeschlagen" -f $pos) 'Red'; return $false }
            if ($seite) { $seiten++ } else { $kleine++ }
            $pos += $n
            Write-Host ("`r  {0}: {1} von {2} Byte geschrieben ({3:N0} s, Wiederholungen {4})   " -f $was, $pos, $data.Length, $sw.Elapsed.TotalSeconds, $wdh) -NoNewline
        }
    } finally { $script:stumm = $false; Write-Host '' }
    $rate = 0; if ($sw.Elapsed.TotalSeconds -gt 0) { $rate = $data.Length / $sw.Elapsed.TotalSeconds }
    Write-Log ("GESCHRIEBEN {0}: {1} Byte ab 0x{2:X} in {3:N1} s ({4:N0} Byte/s), {5} Seiten P S, {6} W-Stücke, Wiederholungen {7}" -f $was, $data.Length, $addr, $sw.Elapsed.TotalSeconds, $rate, $seiten, $kleine, $wdh)
    Say ("  {0}: {1} Byte geschrieben in {2:N1} s ({3} Seiten mit P S, {4} Stücke mit W, Wiederholungen {5})" -f $was, $data.Length, $sw.Elapsed.TotalSeconds, $seiten, $kleine, $wdh)
    return $true
}
# Puffer-Datei aus "stick puffer" laden: NAME_puffer.bin (Bytes für DATEN) und NAME_puffer.txt (INFO-Felder)
function Read-PufferDatei([string]$pfad) {
    if ($pfad -eq '' -or -not (Test-Path -LiteralPath $pfad)) { return $null }
    $bytes = [IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $pfad).Path)
    $txt = [IO.Path]::ChangeExtension((Resolve-Path -LiteralPath $pfad).Path, '.txt')
    $infoTab = @{}
    if (Test-Path -LiteralPath $txt) {
        foreach ($z in (Get-Content -LiteralPath $txt -Encoding UTF8)) { if ($z -match '^([a-z_0-9]+)=(.*)$') { $infoTab[$Matches[1]] = $Matches[2].Trim() } }
    }
    return @{ bytes = $bytes; info = $infoTab; txt = $txt; name = (Split-Path -Leaf $pfad) }
}
function HexText-Bytes([string]$h) {
    $l = New-Object System.Collections.Generic.List[byte]
    for ($i = 0; $i + 1 -lt $h.Length; $i += 2) { $l.Add([Convert]::ToByte($h.Substring($i, 2), 16)) }
    return ,($l.ToArray())
}
function Ask-JN([string]$frage) {
    if ($Simulation -or $OhneNachfrage) { Write-Log ("FREIGABE ohne Nachfrage: {0}" -f $frage); return $true }
    Write-Host ''
    Write-Host $frage -ForegroundColor Yellow
    Write-Host 'Fortfahren? (J/N)' -ForegroundColor Yellow
    $a = Read-Host
    $ja = $a -match '^[JjYy]'
    Write-Log ("BENUTZER: {0} -> {1}" -f $frage, $(if ($ja) { 'JA' } else { 'NEIN' }))
    return $ja
}

# Design senden (Original 0x100032b0 ohne 0x10/0x200, Ziel 3 „Embroider“)
function Send-Design([int]$N, $f, $pd) {
    $INFO = [int]$f['INFO']; $DATEN = [int]$f['DATEN']
    $b = $pd.bytes; $k = 6 * $N + 8
    Say ''; Say ("  Puffer-Datei: {0} ({1} Byte)" -f $pd.name, $b.Length) 'Cyan'
    if ($b.Length -lt $k) { Say ("  Puffer kürzer als der Kopf ({0} Byte bei N = {1}) – nicht gesendet" -f $k, $N) 'Red'; return $false }
    $len1 = [int64](U32BE $b (6 * $N)); $len2 = [int64](U32BE $b (6 * $N + 4))
    if ($k + $len1 + $len2 -ne $b.Length) { Say ("  Puffer passt nicht zu N = {0}: Kopf {1} + Stiche {2} + Anhang {3} ≠ {4} – nicht gesendet (Puffer für N = 28 gebaut?)" -f $N, $k, $len1, $len2, $b.Length) 'Red'; return $false }
    $endeOk = ($len1 -ge 2) -and ($b[$k + $len1 - 2] -eq 0x80) -and ($b[$k + $len1 - 1] -eq 0x81)
    if (-not $endeOk) { Say '  Stichdaten enden nicht mit 80 81 – nicht gesendet' 'Red'; return $false }
    $anhangOk = $true
    if ($len2 -gt 0) {
        $o = $k + [int]$len1
        $cl = [int64]$b[$o + 2] + ([int64]$b[$o + 3] -shl 8) + ([int64]$b[$o + 4] -shl 16) + ([int64]$b[$o + 5] -shl 24)
        $anhangOk = ($cl -eq $len2)
    }
    if (-not $anhangOk) { Say '  Anhang ungültig (Containerlänge ≠ Länge 2) – nicht gesendet' 'Red'; return $false }
    # Größenprüfung wie Original Schritt 5 (Fehler 31 „Design is too big“): Stiche + Anhang + 2 < Speichergröße
    $grenze = [Math]::Max([int64]$f['Groesse Datenbereich'], [int64]$f['Ende Speicher'])
    if ($len1 + $len2 + 2 -ge $grenze) { Say ("  Design zu groß: {0} Byte ≥ Grenze {1} (Original: Fehler 31) – nicht gesendet" -f ($len1 + $len2 + 2), $grenze) 'Red'; return $false }
    Say ("  Stichdaten {0} Byte (Ende 80 81), Anhang {1} Byte{2}, Grenze {3} Byte – passt" -f $len1, $len2, $(if ($len2 -gt 0) { ' (Container gültig)' } else { ' (kein Anhang)' }), $grenze) 'Green'
    # INFO-Felder aus der .txt
    $infoTab = $pd.info; $mitInfo = -not $OhneInfo
    if ($mitInfo -and -not ($infoTab.ContainsKey('info_farben_minus_1') -and $infoTab.ContainsKey('info_flags') -and $infoTab.ContainsKey('info_name'))) {
        Say ("  {0} fehlt oder unvollständig – INFO-Felder werden NICHT geschrieben" -f (Split-Path -Leaf $pd.txt)) 'Yellow'; $mitInfo = $false
    }
    $fm1 = 0; $flg = 0; $nameB = New-Object byte[] 32
    if ($mitInfo) {
        $fm1 = [Convert]::ToInt32($infoTab['info_farben_minus_1'], 16); $flg = [Convert]::ToInt32($infoTab['info_flags'], 16)
        $nb = HexText-Bytes $infoTab['info_name']; [Array]::Copy($nb, $nameB, [Math]::Min(32, $nb.Length))
    }
    Say '  Geplante Schreibzugriffe:' 'Cyan'
    Say ("    DATEN 0x{0:X6}: {1} Byte (Kopf {2} + Stiche {3} + Anhang {4})" -f $DATEN, $b.Length, $k, $len1, $len2)
    if ($mitInfo) {
        Say ("    INFO+(N+1)  0x{0:X6}: Farbanzahl−1 = 0x{1:X2}" -f ($INFO + $N + 1), $fm1)
        Say ("    INFO+(2N+1) 0x{0:X6}: Flags = 0x{1:X2}" -f ($INFO + 2 * $N + 1), $flg)
        Say ("    INFO+(3N+1) 0x{0:X6}: Name = '{1}'" -f ($INFO + 3 * $N + 1), (Ascii $nameB).TrimEnd('.'))
    } else { Say '    INFO-Felder: werden nicht geschrieben' }
    if ($Sticken) { Say '    danach Befehl 0x08 an 0xFFFED0 (sticken ohne speichern)' } else { Say '    kein Befehl 0x08 (ohne -Sticken)' }
    Say '    zum Schluss Befehl 0x100 und T r M E (PC-Modus verlassen)'
    if (-not $Schreiben) {
        Say ''; Say '  TROCKENLAUF: nichts geschrieben (Schalter -Schreiben fehlt); nur 0x100 + T r M E zum Verlassen.' 'Cyan'
        return $true
    }
    if (-not (Ask-JN ("JETZT WIRD IN DIE MASCHINE GESCHRIEBEN: Puffer DATEN ({0} Byte){1}. Gespeicherte Designs werden nicht verändert." -f $b.Length, $(if ($mitInfo) { ' und INFO-Felder' } else { '' })))) {
        Say '  Abgebrochen durch Benutzer – nichts geschrieben.' 'Yellow'; return $false
    }
    # 3) DATEN schreiben
    if (-not (Write-Gross $DATEN $b 'DATEN')) { return $false }
    # 4) zurücklesen und vergleichen
    $zurueck = Read-Gross $DATEN $b.Length 'DATEN zurücklesen'
    if ($null -eq $zurueck) { Say '  Zurücklesen fehlgeschlagen' 'Red'; return $false }
    $erste = -1; $anz = 0
    for ($i = 0; $i -lt $b.Length; $i++) { if ($zurueck[$i] -ne $b[$i]) { $anz++; if ($erste -lt 0) { $erste = $i } } }
    $kopie = Join-Path $here ("Zurueckgelesen_{0}_{1}" -f $stamp, $pd.name)
    [IO.File]::WriteAllBytes($kopie, [byte[]]$zurueck)
    if ($anz -gt 0) {
        Say ("  VERGLEICH: {0} Byte verschieden, erstes bei Offset {1} (gesendet 0x{2:X2}, gelesen 0x{3:X2}) – {4}" -f $anz, $erste, $b[$erste], $zurueck[$erste], $kopie) 'Red'
        Say '  Kein INFO-Schreiben und kein Sticken.' 'Yellow'
        return $false
    }
    Say ("  >>> VERGLEICH OK: alle {0} Byte stimmen (zurückgelesen: {1})" -f $b.Length, (Split-Path -Leaf $kopie)) 'Green'
    # 5) INFO-Felder (Original Schritte 9–11)
    if ($mitInfo) {
        $ok = (Write-Mem ($INFO + $N + 1) ([byte[]]@($fm1))) -and (Write-Mem ($INFO + 2 * $N + 1) ([byte[]]@($flg))) -and (Write-Mem ($INFO + 3 * $N + 1) $nameB)
        if (-not $ok) { Say '  INFO-Felder schreiben fehlgeschlagen – kein Sticken' 'Red'; return $false }
        $r1 = Read-Mem ($INFO + $N + 1) 1; $r2 = Read-Mem ($INFO + 2 * $N + 1) 1; $r3 = Read-Mem ($INFO + 3 * $N + 1) 32
        $infoOk = ($null -ne $r1) -and ($null -ne $r2) -and ($null -ne $r3) -and ($r1[0] -eq $fm1) -and ($r2[0] -eq $flg) -and ((Hex $r3) -eq (Hex $nameB))
        if ($infoOk) { Say '  INFO-Felder geschrieben und zurückgelesen – stimmen' 'Green' } else { Say '  INFO-Felder: Zurücklesen weicht ab – kein Sticken' 'Red'; return $false }
    }
    # 6) Sticken
    if ($Sticken) {
        if (-not (Ask-JN 'JETZT BEFEHL 0x08 (STICKEN): Ist jemand an der Maschine? Rahmen eingesetzt, Nadel oben, Hände weg vom Nadelbereich?')) {
            Say '  Sticken abgebrochen durch Benutzer – Daten stehen im Puffer, kein Befehl 0x08 gesendet.' 'Yellow'; return $true
        }
        $r = Invoke-Mailbox 0x08 3000 7000 0
        Say ("  Befehl 0x08 (sticken ohne speichern): Status 0x{0:X4}{1}" -f $r.st, (Status-Text $r.st)) $(if ($r.ok) { 'Green' } else { 'Yellow' })
        if ($r.ok) { Say '  >>> MASCHINE HAT DAS DESIGN ÜBERNOMMEN – bitte am Display nachsehen und Start an der Maschine' 'Green' }
        else { Say '  >>> Befehl 0x08 nicht bestätigt – bitte Display ansehen und Mitschnitt schicken' 'Yellow' }
        return $r.ok
    }
    return $true
}

# ---------------------------------------------------------------- Hauptablauf
function Test-Port([string]$name, $pd) {
    $script:portName = $name; $script:baud = $Baud
    Say ''; Say "=== Port $name ===" 'Cyan'
    try { Open-Port $name $script:baud } catch { Say "  Port lässt sich nicht öffnen: $($_.Exception.Message)" 'Yellow'; return $false }
    Start-Sleep -Milliseconds 200
    $connected = $false
    for ($attempt = 1; $attempt -le 3 -and -not $connected; $attempt++) {
        Say "  Versuch $attempt : Transfermodus (T r M E) …"
        if (Enter-Transfer) { $connected = $true; break }
        Say '  keine passende Antwort – synchronisiere (R / F / ?) …' 'Yellow'
        if (Sync-Machine) { Say "  synchron bei $($script:baud) Baud" 'Green' } else { Say '  keine Synchronisation möglich' 'Yellow' }
    }
    if (-not $connected) { Say "  Keine Verbindung über $name." 'Yellow'; Close-Port; return $false }
    Say "  VERBUNDEN (Transfermodus aktiv, $($script:baud) Baud)" 'Green'

    # 1) Firmware-Kennung – zuerst meldet sich die Nähmaschine (NMMV), dann das Stickmodul (SRMV)
    $fw = Read-Mem 0x200100 64
    if ($null -eq $fw) { Say '  Lesen von 0x200100 fehlgeschlagen' 'Red'; Close-Port; return $true }
    $txt = [Text.Encoding]::ASCII.GetString($fw)
    if ($txt.StartsWith('NMMV')) {
        Say ("  >>> STUFE 1 OK: Nähmaschine antwortet, Firmware '{0}'" -f $txt.Substring(4, 5)) 'Green'
        $switched = $false
        for ($t = 1; $t -le 2 -and -not $switched; $t++) { Say "  Umschalten zum Stickmodul, Versuch $t …"; $switched = Switch-ToModule }
        if (-not $switched) { Say '  >>> Stickmodul NICHT erreicht. Mitschnitt bitte schicken.' 'Yellow'; Close-Port; Say 'Bitte die Maschine jetzt AUS- und wieder EINSCHALTEN.' 'Cyan'; return $true }
        $fw = Read-Mem 0x200100 64
        if ($null -eq $fw) { Say '  Lesen von 0x200100 (Stickmodul) fehlgeschlagen' 'Red'; Close-Port; return $true }
        $txt = [Text.Encoding]::ASCII.GetString($fw)
    }
    if ($txt.StartsWith('SRMV')) { Say ("  >>> STUFE 2 OK: Stickmodul antwortet ('{0}')" -f (Ascii $fw[0..15])) 'Green' }
    else { Say '  >>> Kennung SRMV NICHT gefunden – Mitschnitt bitte schicken' 'Yellow'; Close-Port; Say 'Bitte die Maschine jetzt AUS- und wieder EINSCHALTEN.' 'Cyan'; return $true }

    # 2) Verwaltungsblock
    $blk = Read-Mem 0xFFFD24 32
    if ($null -eq $blk) { Say '  Lesen von 0xFFFD24 fehlgeschlagen' 'Red'; Close-Port; return $true }
    Say ("  0xFFFD24: {0}" -f (Hex $blk))
    $N = $blk[0]
    $f = [ordered]@{ 'N (Byte 0)' = $blk[0]; 'Byte 1' = $blk[1]; 'Groesse Datenbereich' = (U32BE $blk 2); 'INFO' = (U32BE $blk 6); 'AUSWAHL' = (U32BE $blk 10);
                     'ICON' = (U32BE $blk 14); 'DATEN' = (U32BE $blk 18); 'Ende Karte' = (U32BE $blk 22); 'Ende Speicher' = (U32BE $blk 26) }
    foreach ($k in $f.Keys) { Say ("     {0,-22} = 0x{1:X} ({1})" -f $k, $f[$k]) }
    $plaus = ($blk[0] -le 0x1C) -and ($blk[1] -le 0x3C) -and ($f['Groesse Datenbereich'] -le 0x249F0)
    if (-not $plaus) { Say '  >>> Verwaltungsblock NICHT plausibel – nichts gesendet' 'Yellow'; Close-Port; return $true }
    Say '  >>> Verwaltungsblock plausibel' 'Green'

    # 3) Senden
    $ok = $false
    try { $ok = Send-Design ([int]$N) $f $pd } catch { Say ("  Ausnahme: {0}" -f $_.Exception.Message) 'Red'; $ok = $false }
    Say ''
    # PC-Modus immer verlassen wie das Original (auch nach dem Trockenlauf): 0x100, dann T r M E
    switch (Exit-Transfer) {
        'Befehl' { Say '  PC-Modus verlassen (Befehl 0x100 + T r M E wie das Original) – Display sollte wieder an sein' 'Green' }
        default  { Say '  PC-Modus nicht bestätigt verlassen – Display ansehen, ggf. Maschine aus/ein' 'Yellow' }
    }
    Close-Port
    if ($ok) { Say ''; Say 'FERTIG.' 'Cyan' } else { Say ''; Say 'FERTIG MIT FEHLER – Mitschnitt und Ergebnis bitte schicken.' 'Yellow' }
    return $true
}

Say ("artista Design senden VERSION 4.0 – {0}" -f (Get-Date)) 'Cyan'
Say ("Mitschnitt: {0}" -f $logFile)
if ($Sticken -and -not $Schreiben) { Say '-Sticken geht nur zusammen mit -Schreiben – Abbruch.' 'Red'; $script:logW.Close(); exit 1 }
$pd = Read-PufferDatei $Puffer
if ($null -eq $pd) { Say ("Puffer-Datei nicht gefunden: '{0}' (Schalter -Puffer NAME_puffer.bin)" -f $Puffer) 'Red'; $script:result | Set-Content -Path $resFile -Encoding UTF8; $script:logW.Close(); exit 1 }
Say ("Modus: {0}" -f $(if (-not $Schreiben) { 'TROCKENLAUF (nichts schreiben)' } elseif ($Sticken) { 'SCHREIBEN + STICKEN (0x08)' } else { 'SCHREIBEN (ohne Sticken)' })) 'Cyan'
if ($Simulation) { Say '*** SIMULATION – keine echte Maschine, nur Selbsttest des Programms ***' 'Magenta'; Sim-Init; $ports = @('SIM') }
elseif ($Port -ne '') { $ports = @($Port) }
else {
    $ports = [System.IO.Ports.SerialPort]::GetPortNames() | Sort-Object -Unique
    Say ("Gefundene COM-Ports: {0}" -f (($ports) -join ', '))
    if (-not $ports) { Say 'KEIN COM-Port gefunden.' 'Red' }
}
$done = $false
foreach ($pn in $ports) { if (Test-Port $pn $pd) { $done = $true; break } }
if (-not $done) { Say ''; Say 'Keine Maschine gefunden. Tipp: -Port COM3, ggf. -DtrRts.' 'Red' }
if ($Simulation) {
    $g = $script:simMem; $DAT = 0x028E9A; $diff = 0
    for ($i = 0; $i -lt $pd.bytes.Length; $i++) { $v = 0; if ($g.ContainsKey($DAT + $i)) { $v = $g[$DAT + $i] }; if ($v -ne $pd.bytes[$i]) { $diff++ } }
    if ($Schreiben) { Say ("SELBSTTEST: simulierter Speicher DATEN – {0} Byte verschieden; P-S-Seiten {1}; Befehl 0x08 {2}" -f $diff, $script:simPZahl, $(if ($script:simGestickt) { 'erhalten' } else { 'nicht erhalten' })) 'Magenta' }
}
$script:result | Set-Content -Path $resFile -Encoding UTF8
Say ("Ergebnis gespeichert: {0}" -f $resFile)
$script:logW.Close()
