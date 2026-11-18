<#
  artista_designliste.ps1  –  VERSION 3.1 – DESIGNLISTE AUSLESEN, BERNINA artista 185 (serielles Kabel)

  Neu in 3.1 (nach Gerätetest v3 am 18.11.2026: Bankbefehl 0x30 und 0x20 mit Status 0x1005 abgelehnt,
  0xFFFEE0 stand auf 0x00): Wie die Original-Software (TransparentMode(1), BEAGLink.dll 0x10006070)
  wird vor den Befehlen der TRANSFER ANGEFORDERT: 0x01 nach 0xFFFEE0 schreiben, dann alle 10 ms
  0xFFFEE0 lesen, bis Bit 0x02 gesetzt ist (max. 5 s, sonst Original-Fehler 17).

  Neu in Version 3: liest die Liste der gespeicherten Designs aus dem Stickmodul – genau mit der
  Befehlsfolge der Original-Software beim Menüpunkt "Card/Machine Read" (BEAGLink.dll 0x100064c0).
  Dafür werden STEUERBEFEHLE gesendet (vor dem ersten fragt das Programm nach):
     Befehlsfeld 0xFFFED0:  0x50 (Ziel Maschinenspeicher) bzw. 0xA0 (Karte), 0x30 (Bank 0),
                            0x20 (Liste laden), ggf. 0x60/0xC0 (weitere Bänke), am Ende 0x100 (PC-Modus verlassen)
     Auswahlfeld (AUSWAHL): Byte +0 = 1, Byte +5 = Banknummer
     Übertragungsbyte 0xFFFEE0: 0x01 (Transfer anfordern, ab 3.1)
  Es werden KEINE Designs geschrieben, geändert, gelöscht und es wird nichts formatiert.

  --- ab hier unverändert aus Version 2 ---

  Neu in Version 2 (nach dem ersten Test an der artista 185 am 18.11.2026):
  Die Maschine meldet zuerst die Firmware der NÄHMASCHINE ("NMMV…"). Zum STICKMODUL ("SRMV…")
  schaltet die Original-Software mit Baudwechsel auf 57600 ('J' '0' '5') und 'Y' 'Q' durch.
  Genau das macht Version 2 jetzt auch (BEAGLink.dll 0x10005e30 / 0x100057b0).

  Nachbau des Protokolls aus BEAGLink.dll (BERNINA artista V4 / ADX), siehe Analyse_Teil2.
  Das Skript SCHREIBT NICHTS in den Speicher der Maschine. Es sendet nur die Steuerzeichen,
  die auch die Original-Software zum Verbinden und Lesen sendet:
     'T' 'r' 'M' 'E'          Transfermodus betreten
     'R' + 6 Hexziffern       32 Byte ab Adresse lesen
     'R' 'F' '?'              Synchronisieren (nur falls nötig)
     'J' '0' '5' / 'E' 'B'    Baudrate 57600 vereinbaren (wie Original)
     'Y' 'Q'                  Verbindung zum Stickmodul durchschalten (wie Original)
  Jedes gesendete und empfangene Byte wird mit Zeitstempel in eine Mitschnitt-Datei geschrieben.

  Aufruf:   START.bat                       (probiert alle COM-Ports)
            START.bat -Port COM3            (fester Port)
            START.bat -Port COM3 -DtrRts    (DTR/RTS einschalten, falls ohne keine Antwort kommt)
            START.bat -Simulation           (Selbsttest ohne Maschine)
            START.bat -Port COM3 -Karte     (Designs der Karte im Stickmodul statt Maschinenspeicher)
            START.bat -Port COM3 -OhneNachfrage   (keine Sicherheitsabfrage)
  Nach dem Test die Maschine AUS- und wieder EINSCHALTEN (Transfermodus wird nicht beendet).
#>
param(
    [string]$Port = "",
    [int]$Baud = 19200,
    [switch]$DtrRts,
    [switch]$Simulation,
    [int]$SimBaud = 19200,         # nur für den Selbsttest: Baudrate der simulierten Maschine
    [switch]$SimOhneModul,         # nur für den Selbsttest: Stickmodul fehlt
    [switch]$Karte,                # Designs der Karte im Stickmodul lesen (Befehl 0xA0) statt Maschinenspeicher (0x50)
    [switch]$OhneNachfrage,        # Sicherheitsabfrage vor den Steuerbefehlen überspringen
    [int]$SimDesigns = 30          # nur für den Selbsttest: Anzahl simulierter Designs
)

$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$logFile = Join-Path $here "Mitschnitt_$stamp.log"
$resFile = Join-Path $here "Ergebnis_$stamp.txt"
$script:sp = $null
$script:baud = $Baud
$script:portName = ''
$script:result = New-Object System.Collections.Generic.List[string]

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
function Sim-Names([int]$bank) {
    $info = 0x024080; $n = 28
    $count = [Math]::Max(0, [Math]::Min(27, $SimDesigns - 27 * $bank))
    for ($k = 0; $k -lt 27; $k++) {
        $nm = New-Object byte[] 32
        if ($k -lt $count) { $t = [Text.Encoding]::ASCII.GetBytes(('SIM DESIGN {0:D2}' -f (27 * $bank + $k + 1))); [Array]::Copy($t, $nm, $t.Length) }
        Sim-Put ($info + 3 * $n + 1 + 32 * $k) $nm
        Sim-Put ($info + 2 * $n + 1 + $k) ([byte[]](0x08))
        Sim-Put ($info + $n + 1 + $k) ([byte[]](2))
    }
}
function Sim-WriteDone {
    $h = $script:simWHex; if ($h.Length -lt 6) { return }
    $addr = [Convert]::ToInt32($h.Substring(0, 6), 16)
    $data = for ($i = 6; $i + 1 -lt $h.Length; $i += 2) { [Convert]::ToByte($h.Substring($i, 2), 16) }
    if ($addr -eq 0xFFFEE0) {                                        # Transfer anfordern -> Bit 0x02 setzen
        if ($data[0] -band 1) { Sim-Put 0xFFFEE0 ([byte[]](2)) }; return
    }
    Sim-Put $addr ([byte[]]@($data))
    if ($addr -eq 0xFFFED0) {                                        # Befehl ins Befehlsfeld
        $cmd = (([int]$data[0]) -shl 8) + [int]$data[1] - 1
        $bank = 0; if ($script:simMem.ContainsKey(0x0201EA + 5)) { $bank = $script:simMem[0x0201EA + 5] }
        $tr = 0; if ($script:simMem.ContainsKey(0xFFFEE0)) { $tr = $script:simMem[0xFFFEE0] }
        if ($cmd -eq 0x100) { Sim-Put 0xFFFEE0 ([byte[]](0)); Sim-Put 0xFFFED0 ([byte[]](0, 2)); return }
        if ($cmd -in 0x30, 0x20, 0x60, 0xC0 -and -not ($tr -band 2)) { Sim-Put 0xFFFED0 ([byte[]](0x10, 5)); return }   # wie Gerätetest v3
        if ($cmd -eq 0x20) { Sim-Put 0x024080 ([byte[]]([Math]::Min(255, $SimDesigns))); Sim-Names 0 }
        if ($cmd -in 0x30, 0x60, 0xC0) { Sim-Names $bank }
        Sim-Put 0xFFFED0 ([byte[]](0, 2))                            # Status: fertig, ok
    }
}
function Sim-Rx([byte]$b) {
    if ($script:baud -ne $script:simSimBaud) { return }              # falsche Baudrate: Maschine versteht nichts
    $script:simQ.Enqueue($b)                                         # Echo jedes Zeichens
    $c = [char]$b
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
                for ($i = 0; $i -lt 32; $i++) {
                    $v = 0; if ($mem.ContainsKey($addr + $i)) { $v = $mem[$addr + $i] }
                    foreach ($h in ('{0:X2}' -f $v).ToCharArray()) { $script:simQ.Enqueue([byte][char]$h) }
                }
                $script:simQ.Enqueue([byte][char]'O')
            }
        } else { $script:simAddrMode = 0 }
        return
    }
    $script:simLast = ($script:simLast + $c)
    if ($script:simLast.Length -gt 8) { $script:simLast = $script:simLast.Substring($script:simLast.Length - 8) }
    if ($script:simLast.EndsWith('TrME')) { $script:simTransfer = $true; return }
    if (-not $script:simTransfer) { return }                         # Lesebefehle erst im Transfermodus
    if ($script:simLast.EndsWith('J05')) { $script:simBaudPending = 57600; $script:simBaudSwitch = [Diagnostics.Stopwatch]::StartNew(); return }
    if ($script:simLast.EndsWith('YQ') -and $script:simStage -eq 'NM' -and -not $SimOhneModul) { $script:simStage = 'SR'; $script:simQ.Enqueue([byte][char]'O'); return }
    if ($c -ceq 'J') { return }
    if ($c -ceq 'W') { $script:simW = 1; $script:simWHex = ''; return }
    if ($c -ceq 'R') { $script:simAddrMode = 6; $script:simHex = '' }
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
    Write-Log ('PC  -> MASCH  ' + (Show-Byte $b))
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
        if ($Simulation) { if ($script:simQ.Count -gt 0) { $b = $script:simQ.Dequeue(); Write-Log ('MASCH -> PC  ' + (Show-Byte $b)); return [int]$b } }
        elseif ($script:sp.BytesToRead -gt 0) { $b = $script:sp.ReadByte(); Write-Log ('MASCH -> PC  ' + (Show-Byte $b)); return [int]$b }
        if ($sw.ElapsedMilliseconds -ge $timeoutMs) { return -1 }
        Start-Sleep -Milliseconds 1
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
# Designliste lesen (Original 0x100064c0 + 0x10004d10)
function Read-DesignList([int]$N, [int]$INFO, [int]$AUSWAHL) {
    $flag = 0; $ziel = 0x50; $zielName = 'Maschinenspeicher'
    if ($Karte) { $flag = 1; $ziel = 0xA0; $zielName = 'Karte im Stickmodul' }
    Say "  Ziel: $zielName" 'Cyan'
    $r = Invoke-Mailbox $ziel 1000 4000 1
    if (-not $r.ok) { Say ("  Befehl 0x{0:X} (Ziel) nicht bestätigt, Status 0x{1:X4}{2}" -f $ziel, $r.st, (Status-Text $r.st)) 'Yellow'; return $false }
    if (-not (Write-Mem $AUSWAHL ([byte[]](1)))) { Say '  Auswahl schreiben fehlgeschlagen' 'Red'; return $false }
    if (-not (Write-Mem ($AUSWAHL + 5) ([byte[]](0)))) { Say '  Bank schreiben fehlgeschlagen' 'Red'; return $false }
    $r = Invoke-Mailbox 0x30 1000 4000 1
    if (-not $r.ok) { Write-Log ("Bankbefehl 0x30 Status 0x{0:X4} (Original wertet das nicht als Fehler)" -f $r.st) }
    $r = Invoke-Mailbox 0x20 1000 2000 $flag
    if (-not $r.ok) { Say ("  Befehl 0x20 (Liste laden) nicht bestätigt, Status 0x{0:X4}{1}" -f $r.st, (Status-Text $r.st)) 'Yellow'; return $false }
    $c = Read-Mem $INFO 1
    if ($null -eq $c) { return $false }
    $count = [int]$c[0]
    Say ("  >>> Anzahl Designs ({0}): {1}" -f $zielName, $count) 'Green'
    $rest = $count; $bank = 0; $nr = 1
    while ($rest -gt 0 -and $bank -lt 256) {
        $p = Read-Mem 0x24004 4; $stByte = 0
        if ($null -ne $p) { $sb = Read-Mem ([int](U32BE $p 0) + 7) 1; if ($null -ne $sb) { $stByte = $sb[0] } }
        if (-not (Write-Mem ($AUSWAHL + 5) ([byte[]]($bank)))) { return $false }
        if ($bank -eq 1) { $r = Invoke-Mailbox 0x60 1000 4000 1; if (-not $r.ok) { Say '  Bank 1 nicht geladen' 'Yellow'; break } }
        elseif ($bank -ge 2) { $r = Invoke-Mailbox 0xC0 1000 4000 1; if (-not $r.ok) { Say "  Bank $bank nicht geladen" 'Yellow'; break } }
        $k = [Math]::Min(27, $rest)
        $farb = Read-Mem ($INFO + $N + 1) 27
        $flg = Read-Mem ($INFO + 2 * $N + 1) 27
        $nam = Read-Mem ($INFO + 3 * $N + 1) ($k * 32)
        if ($null -eq $flg -or $null -eq $nam) { Say '  Lesen der Liste fehlgeschlagen' 'Red'; return $false }
        Say ("  Bank {0} (Statusbyte 0x{1:X2}: Namen {2})" -f $bank, $stByte, $(if ($stByte -band 4) { 'werden angezeigt' } else { 'werden von der Original-Software durch Nummern ersetzt' }))
        for ($i = 0; $i -lt $k; $i++) {
            $raw = New-Object byte[] 32; [Array]::Copy($nam, 32 * $i, $raw, 0, 32)
            $name = (Ascii $raw).TrimEnd('.').Trim()
            $anzeige = $name; if (-not ($stByte -band 4)) { $anzeige = [string]$nr }
            $fc = ''; if ($null -ne $farb) { $fc = '{0}' -f ([int]$farb[$i] + 1) }
            Say ("   Nr {0,3}  Anzeige '{1}'  Name roh '{2}'  Flags 0x{3:X2}  Farben? {4}" -f (27 * $bank + $i + 1), $anzeige, $name, $flg[$i], $fc)
            $nr++
        }
        $rest -= 27; $bank++
    }
    return $true
}
# PC-Modus sauber verlassen (Original TransparentMode(0) -> 0x10006070): wenn 0xFFFEE0 Bit 1 gesetzt -> Befehl 0x100
function Exit-Transfer {
    $b = Read-Mem 0xFFFEE0 1
    if ($null -eq $b) { return 'Fehler' }
    Write-Log ("0xFFFEE0 = 0x{0:X2}" -f $b[0])
    if ($b[0] -band 2) { $r = Invoke-Mailbox 0x100 2000 2000 0; if ($r.ok) { return 'Befehl' } else { return 'Fehler' } }
    return 'nicht aktiv'
}
# Transfer anfordern (Original TransparentMode(1) -> 0x10006070): 0x01 nach 0xFFFEE0, warten auf Bit 0x02 (5 s)
function Request-Transfer {
    $b = Read-Mem 0xFFFEE0 1
    if ($null -ne $b) { Write-Log ("0xFFFEE0 vorher = 0x{0:X2}" -f $b[0]) }
    Write-Log 'TRANSFER ANFORDERN: 0x01 -> 0xFFFEE0'
    if (-not (Write-Mem 0xFFFEE0 ([byte[]](1)))) { return $false }
    $sw = [Diagnostics.Stopwatch]::StartNew()
    while ($sw.ElapsedMilliseconds -lt 5000) {
        $b = Read-Mem 0xFFFEE0 1
        if ($null -eq $b) { return $false }
        if ($b[0] -band 2) { Write-Log ("0xFFFEE0 = 0x{0:X2} nach {1} ms" -f $b[0], $sw.ElapsedMilliseconds); return $true }
        Start-Sleep -Milliseconds 10
    }
    Write-Log ("0xFFFEE0 Bit 0x02 nicht gesetzt nach 5 s (letzter Wert 0x{0:X2}) = Original-Fehler 17" -f $b[0])
    return $false
}
function Parse-Version([string]$txt) {
    $ver = 0
    foreach ($ch in $txt.Substring(4).ToCharArray()) { if ($ch -eq [char]0) { break }; if ($ch -ge '0' -and $ch -le '9') { $ver = (($ver - 3) -shl 4) + [int][char]$ch } }
    return $ver
}

# ---------------------------------------------------------------- Hauptablauf
function Test-Port([string]$name) {
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
    Say ("  0x200100: {0}" -f (Hex $fw)); Say ("            '{0}'" -f (Ascii $fw))
    $txt = [Text.Encoding]::ASCII.GetString($fw)
    if ($txt.StartsWith('NMMV')) {
        Say ("  >>> STUFE 1 OK: Nähmaschine antwortet, Firmware '{0}', Sprache '{1}'" -f $txt.Substring(4, 5), ($txt.Substring(10, 12).Trim())) 'Green'
        $switched = $false
        for ($t = 1; $t -le 2 -and -not $switched; $t++) {
            Say "  Umschalten zum Stickmodul, Versuch $t …"
            $switched = Switch-ToModule
        }
        if (-not $switched) {
            Say '  >>> STUFE 2 NICHT erreicht (Stickmodul). Mitschnitt bitte schicken.' 'Yellow'
            Close-Port; Say 'Bitte die Maschine jetzt AUS- und wieder EINSCHALTEN.' 'Cyan'; return $true
        }
        $fw = Read-Mem 0x200100 64
        if ($null -eq $fw) { Say '  Lesen von 0x200100 (Stickmodul) fehlgeschlagen' 'Red'; Close-Port; return $true }
        Say ("  0x200100: {0}" -f (Hex $fw)); Say ("            '{0}'" -f (Ascii $fw))
        $txt = [Text.Encoding]::ASCII.GetString($fw)
    }
    if ($txt.StartsWith('SRMV')) {
        Say ("  >>> STUFE 2 OK: Stickmodul antwortet – Kennung SRMV, Version-Code 0x{0:X}" -f (Parse-Version $txt)) 'Green'
    } else {
        Say '  >>> Kennung SRMV NICHT gefunden – Mitschnitt bitte schicken' 'Yellow'
        Close-Port; Say 'Bitte die Maschine jetzt AUS- und wieder EINSCHALTEN.' 'Cyan'; return $true
    }

    # 2) Verwaltungsblock
    $blk = Read-Mem 0xFFFD24 32
    if ($null -eq $blk) { Say '  Lesen von 0xFFFD24 fehlgeschlagen' 'Red'; Close-Port; return $true }
    Say ("  0xFFFD24: {0}" -f (Hex $blk))
    $N = $blk[0]
    $f = [ordered]@{ 'N (Byte 0)' = $blk[0]; 'Byte 1' = $blk[1]; 'Groesse Datenbereich' = (U32BE $blk 2); 'INFO' = (U32BE $blk 6); 'AUSWAHL' = (U32BE $blk 10);
                     'ICON' = (U32BE $blk 14); 'DATEN' = (U32BE $blk 18); 'Ende Karte' = (U32BE $blk 22); 'Ende Speicher' = (U32BE $blk 26) }
    foreach ($k in $f.Keys) { Say ("     {0,-22} = 0x{1:X} ({1})" -f $k, $f[$k]) }
    $plaus = ($blk[0] -le 0x1C) -and ($blk[1] -le 0x3C) -and ($f['Groesse Datenbereich'] -le 0x249F0)
    if ($plaus) { Say '  >>> Verwaltungsblock plausibel (N ≤ 28, Byte1 ≤ 60, Größe ≤ 150000)' 'Green' } else { Say '  >>> Verwaltungsblock NICHT plausibel' 'Yellow' }

    # 3) Zeiger an festen Adressen (nur lesen)
    foreach ($a in 0x24004, 0x24040, 0x24044, 0x24048) {
        $p = Read-Mem $a 4
        if ($null -ne $p) { Say ("  Zeiger @0x{0:X}: 0x{1:X}" -f $a, (U32BE $p 0)) }
    }
    $p = Read-Mem 0x24004 4
    if ($null -ne $p) {
        $st = Read-Mem ([int](U32BE $p 0)) 8
        if ($null -ne $st) { Say ("  Status @Zeiger(0x24004): {0}   Byte+5=0x{1:X2} Byte+7=0x{2:X2}" -f (Hex $st), $st[5], $st[7]) }
    }

    # 4) Designliste (Steuerbefehle!)
    if (-not $plaus) { Say '  Verwaltungsblock nicht plausibel – keine Steuerbefehle.' 'Yellow'; Close-Port; return $true }
    if (-not $Simulation -and -not $OhneNachfrage) {
        Write-Host ''
        Write-Host 'Jetzt würden Steuerbefehle gesendet (wie "Card/Machine Read" der Original-Software):' -ForegroundColor Yellow
        Write-Host '  Transfer anfordern (0xFFFEE0), Ziel/Bank wählen, Liste laden, PC-Modus verlassen.' -ForegroundColor Yellow
        Write-Host 'Es werden keine Designs verändert. Fortfahren? (J/N)' -ForegroundColor Yellow
        $a = Read-Host
        if ($a -notmatch '^[JjYy]') { Say '  Abgebrochen durch Benutzer – keine Steuerbefehle gesendet.' 'Yellow'; Close-Port; return $true }
    }
    Write-Log 'BENUTZER: Steuerbefehle freigegeben'
    $ok = $false
    if (Request-Transfer) {
        Say '  Transfer angefordert – Maschine bereit (0xFFFEE0 Bit 0x02 gesetzt)' 'Green'
        $ok = Read-DesignList ([int]$N) ([int]$f['INFO']) ([int]$f['AUSWAHL'])
    } else { Say '  Maschine hat den Transfer nicht bestätigt (0xFFFEE0 Bit 0x02 fehlt) – keine Listenbefehle gesendet' 'Yellow' }
    if ($ok) { Say '  >>> DESIGNLISTE GELESEN' 'Green' } else { Say '  >>> Designliste konnte nicht vollständig gelesen werden – Mitschnitt bitte schicken' 'Yellow' }
    switch (Exit-Transfer) {
        'Befehl'      { Say '  PC-Modus verlassen (Befehl 0x100 bestätigt)' }
        'nicht aktiv' { Say '  Transfer war nicht aktiv – Befehl 0x100 nicht nötig' }
        default       { Say '  PC-Modus nicht bestätigt verlassen' 'Yellow' }
    }
    Close-Port
    Say ''; Say 'FERTIG. Bitte die Maschine jetzt AUS- und wieder EINSCHALTEN.' 'Cyan'
    return $true
}

Say ("artista Designliste VERSION 3.1 – {0}" -f (Get-Date)) 'Cyan'
Say ("Mitschnitt: {0}" -f $logFile)
if ($Simulation) { Say '*** SIMULATION – keine echte Maschine, nur Selbsttest des Programms ***' 'Magenta'; Sim-Init; $ports = @('SIM') }
elseif ($Port -ne '') { $ports = @($Port) }
else {
    $ports = [System.IO.Ports.SerialPort]::GetPortNames() | Sort-Object -Unique
    Say ("Gefundene COM-Ports: {0}" -f (($ports) -join ', '))
    if (-not $ports) { Say 'KEIN COM-Port gefunden. Ist der USB-Seriell-Adapter eingesteckt und der Treiber installiert?' 'Red' }
}
$done = $false
foreach ($pn in $ports) { if (Test-Port $pn) { $done = $true; break } }
if (-not $done) { Say ''; Say 'Keine Maschine gefunden. Siehe ANLEITUNG (Fehlersuche). Tipp: mit -DtrRts erneut versuchen.' 'Red' }
$script:result | Set-Content -Path $resFile -Encoding UTF8
Say ("Ergebnis gespeichert: {0}" -f $resFile)
$script:logW.Close()
