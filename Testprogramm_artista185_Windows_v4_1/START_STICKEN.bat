@echo off
rem Wie START_SCHREIBEN, danach Befehl 0x08 (sticken ohne speichern). Nur mit Person an der Maschine!
rem Aufruf: START_STICKEN.bat [PUFFER.bin]   (ohne Angabe: Testquadrat_puffer.bin)
cd /d "%~dp0"
set PUFFER=%~1
if "%PUFFER%"=="" set PUFFER=%~dp0Testquadrat_puffer.bin
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Port COM3 -Puffer "%PUFFER%" -Schreiben -Sticken
pause
