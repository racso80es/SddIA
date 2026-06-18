@echo off
setlocal EnableExtensions

rem Raiz del repositorio (SddIA\scripts\tools -> ..\..\..)
set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"

set "PYTHON="
where py >nul 2>&1 && set "PYTHON=py -3"
if not defined PYTHON where python >nul 2>&1 && set "PYTHON=python"
if not defined PYTHON (
    echo [ERROR] Python 3 requerido.
    exit /b 1
)

set "PYTHONUTF8=1"
cd /d "%REPO_ROOT%"
%PYTHON% "%SCRIPT_DIR%invoke.py" schema-corruptor %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
