@echo off
rem Version 3.4 MIT Ladebefehl 0x400: liest Design Nr 1 (andere Nummer: START_DESIGN_LESEN.bat -Nummer 5)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_lesen_v3_4.ps1" -Port COM3 -MitLadebefehl %*
echo.
echo Fertig. Bitte Design_*.bin, Icon_*.bin, Ergebnis_*.txt und Mitschnitt_*.log schicken.
pause
