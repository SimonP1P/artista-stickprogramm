@echo off
rem Version 3.3: Designliste lesen, Ziel 0xA0 und 0x50 nacheinander (COM3)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_designliste_v3_3.ps1" -Port COM3 %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause