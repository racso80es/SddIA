@echo off
setlocal EnableExtensions
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%invoke.bat" sandbox-breacher %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
