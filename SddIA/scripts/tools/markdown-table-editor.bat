@echo off
setlocal EnableExtensions
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%invoke.bat" markdown-table-editor %*
set "RC=%ERRORLEVEL%"
endlocal & exit /b %RC%
