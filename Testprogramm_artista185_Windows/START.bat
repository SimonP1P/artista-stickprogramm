@echo off
rem Startet den artista-Lesetest. Zusaetzliche Angaben werden durchgereicht, z.B.:  START.bat -Port COM3
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_lesetest.ps1" %*
echo.
echo Fertig. Die Dateien Ergebnis_*.txt und Mitschnitt_*.log liegen in diesem Ordner.
pause
