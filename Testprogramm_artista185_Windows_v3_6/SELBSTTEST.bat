@echo off
rem Selbsttest ohne Maschine: 4 simulierte Designs in einem Durchgang, ein Echo geht verloren
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_lesen_v3_6.ps1" -Simulation -MitLadebefehl -Nummern 2-4,29 -SimStoerung 4
pause
