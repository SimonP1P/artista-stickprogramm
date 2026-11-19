param([int]$Baud = 57600, [string]$Port = 'COM3')
# Letzter Schritt des Originals beim Verlassen des PC-Modus (BEAGLink 0x10005e30 mit Parameter 0, nach Befehl 0x100):
# 'T' 'r' 'M' 'E' mit Echo bei der aktuellen Baudrate. Kein Schreiben, kein Befehl.
$log = Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Path) ("TrME_nach_0x100_{0}.log" -f (Get-Date -Format 'yyyyMMdd_HHmmss'))
function L([string]$t) { $z = (Get-Date -Format 'HH:mm:ss.fff') + '  ' + $t; Add-Content -Path $log -Value $z -Encoding UTF8; Write-Host $z }
$p = New-Object System.IO.Ports.SerialPort $Port, $Baud, ([System.IO.Ports.Parity]::None), 8, ([System.IO.Ports.StopBits]::One)
$p.Handshake = [System.IO.Ports.Handshake]::None; $p.ReadTimeout = 50; $p.WriteTimeout = 2000
$p.Open(); L ("PORT {0} {1} Baud" -f $Port, $Baud)
Start-Sleep -Milliseconds 100
while ($p.BytesToRead -gt 0) { $b = $p.ReadByte(); L ('vorher im Eingang: {0:X2}' -f $b) }
$ok = $true
foreach ($c in 'T', 'r', 'M', 'E') {
    $p.Write([byte[]]@([byte][char]$c), 0, 1); L ("PC -> MASCH '{0}'" -f $c)
    $sw = [Diagnostics.Stopwatch]::StartNew(); $e = -1
    while ($sw.ElapsedMilliseconds -lt 1000) { if ($p.BytesToRead -gt 0) { $e = $p.ReadByte(); break }; Start-Sleep -Milliseconds 1 }
    if ($e -ge 0) { L ("MASCH -> PC {0:X2} '{1}'" -f $e, [char]$e) } else { L 'kein Echo (1000 ms)' }
    if ($e -ne [int][byte][char]$c) { $ok = $false; break }
}
Start-Sleep -Milliseconds 300
while ($p.BytesToRead -gt 0) { $b = $p.ReadByte(); L ('danach im Eingang: {0:X2}' -f $b) }
$p.Close(); L ("PORT geschlossen – TrME {0}" -f $(if ($ok) { 'mit Echo bestätigt' } else { 'NICHT bestätigt' }))
