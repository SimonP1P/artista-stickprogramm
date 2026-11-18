@echo off
rem Version 3.2: Designliste von der KARTE im Stickmodul lesen (COM3)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_designliste_v3_2.ps1" -Port COM3 -Karte %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause
