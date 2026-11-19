@echo off
rem Version 3.6 MIT Ladebefehl 0x400: liest ALLE Designs aus dem Maschinenspeicher nacheinander
rem (nur bestimmte: START_ALLE_LESEN.bat -Nummern 2-10)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_lesen_v3_6.ps1" -Port COM3 -MitLadebefehl -Nummern alle %*
echo.
echo Fertig. Bitte Design_*.bin, Icon_*.bin, Ergebnis_*.txt und Mitschnitt_*.log schicken.
pause
