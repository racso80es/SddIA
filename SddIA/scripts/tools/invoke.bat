@echo off
setlocal EnableExtensions
if "%~1"=="" (
  echo [ERROR] Uso: invoke.bat ^<tool-name^> [--prefer-native] [args...]
  exit /b 1
)

set "TOOL=%~1"
shift

set "PREFER_NATIVE="
if /I "%~1"=="--prefer-native" (
  set "PREFER_NATIVE=--prefer-native"
  shift
)

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
for %%I in ("%REPO_ROOT%") do set "REPO_ROOT=%%~fI"

set "EXEC_BIN="
if defined SDDIA_EXECUTE_PROCESS_BIN (
  set "EXEC_BIN=%SDDIA_EXECUTE_PROCESS_BIN%"
)
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\debug\execute-process.exe" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\debug\execute-process.exe"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\release\execute-process.exe" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\release\execute-process.exe"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\debug\execute-process" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\debug\execute-process"
if not defined EXEC_BIN if exist "%REPO_ROOT%\SddIA\target\release\execute-process" set "EXEC_BIN=%REPO_ROOT%\SddIA\target\release\execute-process"

if not defined EXEC_BIN (
  echo [ERROR] binario execute-process no encontrado. Compilar: cd SddIA ^&^& cargo build -p execute-process
  exit /b 1
)

cd /d "%REPO_ROOT%"
"%EXEC_BIN%" --tool "%TOOL%" %PREFER_NATIVE% %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
