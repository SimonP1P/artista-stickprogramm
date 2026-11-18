@echo off
rem Version 3: Designliste aus dem MASCHINENSPEICHER lesen (COM3)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_designliste_v3.ps1" -Port COM3 %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause
