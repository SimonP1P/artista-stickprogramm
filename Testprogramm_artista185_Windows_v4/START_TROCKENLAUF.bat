@echo off
rem Nur verbinden, pruefen und anzeigen - NICHTS schreiben. Aufruf: START_TROCKENLAUF.bat [PUFFER.bin]
cd /d "%~dp0"
set PUFFER=%~1
if "%PUFFER%"=="" set PUFFER=%~dp0Nr47_puffer.bin
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4.ps1" -Port COM3 -Puffer "%PUFFER%"
pause
