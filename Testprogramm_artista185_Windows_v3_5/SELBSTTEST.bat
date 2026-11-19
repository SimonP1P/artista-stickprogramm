@echo off
rem Selbsttest ohne Maschine (mit simuliertem Design)
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_lesen_v3_5.ps1" -Simulation -MitLadebefehl -Nummer 3 -SimStoerung 4
pause
