@echo off
setlocal EnableExtensions

set "ROOT=%~dp0"
set "PY=%ROOT%system\bin\runtimes\python\current\python\python.exe"
set "CLI=%ROOT%system\bin\entrypoints\app_cli.py"

REM Evitar generación de __pycache__ dentro de system/
set PYTHONDONTWRITEBYTECODE=1

echo ==========================================
echo   APP_TOOL LAUNCHER
echo ==========================================
echo ROOT: %ROOT%
echo PY:   %PY%
echo CLI:  %CLI%
echo.

if not exist "%PY%" (
  echo ERROR: No encuentro Python runtime:
  echo   "%PY%"
  echo.
  pause
  exit /b 1
)

if not exist "%CLI%" (
  echo ERROR: No encuentro entrypoint CLI:
  echo   "%CLI%"
  echo.
  pause
  exit /b 1
)

"%PY%" "%CLI%" shell

echo.
pause
endlocal