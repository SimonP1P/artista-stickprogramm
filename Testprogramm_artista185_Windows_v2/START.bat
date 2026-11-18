@echo off
rem Lesetest Version 2 an COM3 (dort hing die artista beim ersten Test)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_lesetest_v2.ps1" -Port COM3 %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause
