@echo off
rem Lesetest Version 2, probiert alle COM-Ports durch
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_lesetest_v2.ps1" %*
echo.
echo Fertig. Bitte Ergebnis_*.txt und Mitschnitt_*.log aus diesem Ordner schicken.
pause
