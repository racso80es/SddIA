@echo off
setlocal EnableExtensions

if "%~1"=="" (
    echo [ERROR] Uso: %~nx0 ^<daemon-name^> [args...]
    exit /b 1
)

set "DAEMON=%~1"
shift

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"
set "EXTRA_ARGS=%*"

set "PYTHON="
where py >nul 2>&1 && set "PYTHON=py -3"
if not defined PYTHON where python >nul 2>&1 && set "PYTHON=python"
if not defined PYTHON (
    echo [ERROR] Python 3 requerido para cargar boveda (.dev/.env).
    exit /b 1
)

set "PYTHONUTF8=1"
cd /d "%REPO_ROOT%"

for /f "delims=" %%L in ('%PYTHON% "%SCRIPT_DIR%_exec_daemon.py" --emit-bat-env "%REPO_ROOT%"') do %%L

echo [%DAEMON%] Modo continuo — esperando estimulos (Ctrl+C para detener)
echo [%DAEMON%] Repo: %REPO_ROOT%
echo [%DAEMON%] Bovedas: .dev/.env + .SddIA/.dev/.env

%PYTHON% "%SCRIPT_DIR%_exec_daemon.py" "%REPO_ROOT%" "%DAEMON%" %EXTRA_ARGS%
set "RC=%ERRORLEVEL%"
if not "%RC%"=="0" (
    echo.
    echo [%DAEMON%] finalizado con exit=%RC%
    pause
)
endlocal & exit /b %RC%
