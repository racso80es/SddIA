@echo off
setlocal EnableExtensions
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%invoke.bat" send-telegram-notification %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
