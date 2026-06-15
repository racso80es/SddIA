@echo off
setlocal EnableExtensions

rem Raiz del repositorio (SddIA\scripts\daemons -> ../../../)
set "REPO_ROOT=%~dp0..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"

set "WATCHER_SCRIPT=%REPO_ROOT%\SddIA\scripts\daemons\telegram-watcher.py"
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

echo [BAT] Deteniendo instancias previas de telegram-watcher.py...
powershell -NoProfile -Command "Get-CimInstance Win32_Process | Where-Object { ($_.Name -eq 'python.exe' -or $_.Name -eq 'python3.exe') -and ($_.CommandLine -like '*telegram-watcher.py*') } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }"

ping -n 2 127.0.0.1 >nul

echo [BAT] Iniciando Centinela Capa 0 (telegram-watcher)...
echo [BAT] Repo: %REPO_ROOT%
echo [BAT] Modo: bucle long-polling ^(use --once o --dry-run^)
echo [BAT] Requiere: TELEGRAM_BOT_TOKEN + TELEGRAM_ALLOWED_CHAT_ID en .SddIA/.dev/.env
echo [BAT] Estado idempotencia: .SddIA/.state/telegram_last_id

cd /d "%REPO_ROOT%"
start "SddIA Telegram Watcher" /D "%REPO_ROOT%" cmd /k %PYTHON% "%WATCHER_SCRIPT%" %EXTRA_ARGS%

echo [BAT] Centinela lanzado en ventana nueva.
endlocal
exit /b 0
