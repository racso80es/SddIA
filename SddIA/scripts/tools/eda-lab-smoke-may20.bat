@echo off
setlocal EnableExtensions
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%invoke.bat" eda-lab-smoke-may20 %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
