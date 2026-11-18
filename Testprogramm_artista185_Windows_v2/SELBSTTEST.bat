@echo off
rem Selbsttest ohne Maschine
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_lesetest_v2.ps1" -Simulation
pause
