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

set "EXEC_BIN="
if defined SDDIA_EXECUTE_PROCESS_BIN set "EXEC_BIN=%SDDIA_EXECUTE_PROCESS_BIN%"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\debug\execute-process.exe" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\debug\execute-process.exe"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\release\execute-process.exe" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\release\execute-process.exe"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\debug\execute-process" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\debug\execute-process"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\release\execute-process" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\release\execute-process"

if not defined EXEC_BIN (
    echo [ERROR] binario execute-process no encontrado para cargar boveda
    exit /b 1
)

cd /d "%REPO_ROOT%"

for /f "delims=" %%L in ('"%EXEC_BIN%" --emit-shell-env bat') do %%L

set "DAEMON_BIN="
if exist "%REPO_ROOT%\SddIA\target\release\%DAEMON%.exe" set "DAEMON_BIN=%REPO_ROOT%\SddIA\target\release\%DAEMON%.exe"
if not defined DAEMON_BIN if exist "%REPO_ROOT%\SddIA\target\debug\%DAEMON%.exe" set "DAEMON_BIN=%REPO_ROOT%\SddIA\target\debug\%DAEMON%.exe"
if not defined DAEMON_BIN if exist "%REPO_ROOT%\SddIA\target\release\%DAEMON%" set "DAEMON_BIN=%REPO_ROOT%\SddIA\target\release\%DAEMON%"
if not defined DAEMON_BIN if exist "%REPO_ROOT%\SddIA\target\debug\%DAEMON%" set "DAEMON_BIN=%REPO_ROOT%\SddIA\target\debug\%DAEMON%"

if not defined DAEMON_BIN (
    echo [ERROR] Binario no encontrado para %DAEMON% bajo SddIA\target\{release^|debug}\
    exit /b 1
)

echo [%DAEMON%] Modo continuo — esperando estimulos (Ctrl+C para detener)
echo [%DAEMON%] Repo: %REPO_ROOT%
echo [%DAEMON%] Bovedas: .dev/.env + .SddIA/.dev/.env

"%DAEMON_BIN%" %EXTRA_ARGS%
set "RC=%ERRORLEVEL%"
if not "%RC%"=="0" (
    echo.
    echo [%DAEMON%] finalizado con exit=%RC%
    pause
)
endlocal & exit /b %RC%
