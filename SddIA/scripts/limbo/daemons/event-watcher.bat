@echo off
setlocal EnableExtensions

rem Raiz del repositorio (SddIA\scripts\daemons -> ../../../)
set "REPO_ROOT=%~dp0..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"

set "WATCHER_SCRIPT=%REPO_ROOT%\SddIA\scripts\daemons\event-watcher.py"
set "EXTRA_ARGS=%*"

if not exist "%WATCHER_SCRIPT%" (
    echo [ERROR] No se encuentra: %WATCHER_SCRIPT%
    exit /b 1
)

set "PYTHON="
where py >nul 2>&1 && set "PYTHON=py -3"
if not defined PYTHON where python >nul 2>&1 && set "PYTHON=python"
if not defined PYTHON (
    echo [ERROR] No se encontro Python. Instale Python 3 o el launcher py.
    exit /b 1
)

set "PYTHONUTF8=1"

echo [BAT] Deteniendo instancias previas de event-watcher.py...
powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object { ($_.Name -eq 'python.exe' -or $_.Name -eq 'python3.exe') -and ($_.CommandLine -like '*event-watcher.py*') } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }"

ping -n 2 127.0.0.1 >nul

echo [BAT] Iniciando Despertador Inerte (event-watcher)...
echo [BAT] Repo: %REPO_ROOT%
echo [BAT] Modo: bucle continuo ^(use --once para un solo ciclo^)
echo [BAT] IOTA Testnet: SDDIA_LAB_SIMULATE_IOTA=0 en boveda ^(.dev/.env + .SddIA/.dev/.env^)
echo [BAT] Bovedas: .dev/.env + .SddIA/.dev/.env ^(carga automatica en Python^)

cd /d "%REPO_ROOT%"
start "SddIA Event Watcher" /D "%REPO_ROOT%" cmd /k %PYTHON% "%WATCHER_SCRIPT%" %EXTRA_ARGS%

echo [BAT] Watcher lanzado en ventana nueva.
endlocal
exit /b 0
