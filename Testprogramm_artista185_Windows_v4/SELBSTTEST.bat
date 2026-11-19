@echo off
rem Selbsttest ohne Maschine: Trockenlauf, Nr 47 schreiben+sticken, Testquadrat mit Stoerung
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4.ps1" -Simulation -Puffer "%~dp0Nr47_puffer.bin"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4.ps1" -Simulation -Puffer "%~dp0Nr47_puffer.bin" -Schreiben -Sticken
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4.ps1" -Simulation -Puffer "%~dp0Testquadrat_puffer.bin" -Schreiben -Sticken -SimStoerungSchreiben 1 -SimStoerung 1
pause
