@echo off
rem Selbsttest ohne Maschine
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_designliste_v3_1.ps1" -Simulation
pause
