@echo off
rem Nur verbinden, lesen (auch die Garnfarbtabelle der Maschine) und anzeigen - NICHTS schreiben.
rem Aufruf: START_TROCKENLAUF.bat [PUFFER.bin]   (ohne Angabe: Testquadrat_puffer.bin)
cd /d "%~dp0"
set PUFFER=%~1
if "%PUFFER%"=="" set PUFFER=%~dp0Testquadrat_puffer.bin
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Port COM3 -Puffer "%PUFFER%"
pause
