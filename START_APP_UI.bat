@echo off
setlocal EnableExtensions

set "ROOT=%~dp0"
set "PY=%ROOT%system\bin\runtimes\python\current\python\python.exe"

REM Evitar generación de __pycache__ dentro de system/
set PYTHONDONTWRITEBYTECODE=1

cd /d "%ROOT%"

echo ==========================================
echo   APP_DOCGRAPH UI LAUNCHER
echo ==========================================
echo ROOT: %ROOT%
echo PY:   %PY%
echo MOD:  system.bin.entrypoints.app_ui
echo.

if not exist "%PY%" (
  echo ERROR: No encuentro Python runtime:
  echo   "%PY%"
  echo.
  pause
  exit /b 1
)

"%PY%" -m system.bin.entrypoints.app_ui

echo.
pause
endlocal