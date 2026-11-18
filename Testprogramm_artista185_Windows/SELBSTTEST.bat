@echo off
rem Selbsttest ohne Maschine (prueft nur, ob das Programm auf diesem PC laeuft)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_lesetest.ps1" -Simulation
pause
