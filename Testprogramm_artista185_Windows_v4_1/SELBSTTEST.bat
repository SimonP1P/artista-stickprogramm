@echo off
rem Selbsttest ohne Maschine (Simulation): Trockenlauf mit vorhandener Farbtabelle, Testquadrat und Nr 21
rem schreiben + sticken (mit Stoerungen), Nr 21 bei zu wenig Farbplaetzen, Nr 47 ohne Farben
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Simulation -Puffer "%~dp0Nr21_puffer.bin" -SimMitFarbtabelle
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Simulation -Puffer "%~dp0Testquadrat_puffer.bin" -Schreiben -Sticken
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Simulation -Puffer "%~dp0Nr21_puffer.bin" -Schreiben -Sticken -SimStoerungSchreiben 3 -SimStoerung 2
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Simulation -Puffer "%~dp0Nr21_puffer.bin" -Schreiben -SimMaxFarben 2
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0artista_design_senden_v4_1.ps1" -Simulation -Puffer "%~dp0Nr47_puffer.bin" -Schreiben
pause
