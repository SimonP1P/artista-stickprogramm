@echo off
rem Version 3.6 OHNE Ladebefehl 0x400: Verbindung + Designliste (wie 3.3), kein Design-Lesen
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_lesen_v3_6.ps1" -Port COM3 %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause
