@echo off
setlocal EnableDelayedExpansion
cd /d "%~dp0"
set "CAPSULE_JSON=%*"
if not "!CAPSULE_JSON!"=="" (
  npx --yes ts-node index.ts "!CAPSULE_JSON!"
) else (
  npx --yes ts-node index.ts
)
endlocal
