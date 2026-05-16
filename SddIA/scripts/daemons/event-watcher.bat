@echo off
setlocal EnableExtensions

rem Raíz del repositorio (SddIA\scripts\daemons -> ../../../)
set "REPO_ROOT=%~dp0..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"

set "WATCHER_SCRIPT=%REPO_ROOT%\SddIA\scripts\daemons\event-watcher.py"
set "PYTHON=python"

if not exist "%WATCHER_SCRIPT%" (
    echo [ERROR] No se encuentra: %WATCHER_SCRIPT%
    exit /b 1
)

echo [BAT] Deteniendo instancias previas de event-watcher.py...
powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object { ($_.Name -eq 'python.exe' -or $_.Name -eq 'python3.exe') -and ($_.CommandLine -like '*event-watcher.py*') } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }"

ping -n 2 127.0.0.1 >nul

echo [BAT] Iniciando Despertador Inerte...
echo [BAT] Repo: %REPO_ROOT%
echo [BAT] Opcional lab: set SDDIA_LAB_SIMULATE_IOTA=1

cd /d "%REPO_ROOT%"
start "SddIA Event Watcher" /D "%REPO_ROOT%" %PYTHON% "%WATCHER_SCRIPT%"

echo [BAT] Watcher lanzado en ventana nueva.
endlocal
exit /b 0
