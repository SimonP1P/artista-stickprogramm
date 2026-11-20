@echo off
rem DATEN + ICON + INFO + Garnfarbtabelle schreiben, zuruecklesen - KEIN Sticken. Nur nach Freigabe!
rem Aufruf: START_SCHREIBEN.bat [PUFFER.bin]   (ohne Angabe: Testquadrat_puffer.bin)
cd /d "%~dp0"
set PUFFER=%~1
if "%PUFFER%"=="" set PUFFER=%~dp0Testquadrat_puffer.bin
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Port COM3 -Puffer "%PUFFER%" -Schreiben
pause
