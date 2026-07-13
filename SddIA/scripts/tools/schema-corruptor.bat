@echo off
setlocal EnableExtensions
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%invoke.bat" schema-corruptor %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
