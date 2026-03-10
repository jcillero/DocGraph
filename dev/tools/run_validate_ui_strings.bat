@echo off
setlocal

set "ROOT=%~dp0..\.."
set "PY=%ROOT%\system\bin\runtimes\python\current\python\python.exe"
set "SCRIPT=%ROOT%\dev\scripts\ui\validate_ui_strings.py"

set "CATALOG=%ROOT%\system\config\ui_strings.json"
set "SRC_UI=%ROOT%\system\app\ui"

if not exist "%PY%" (
    echo ERROR: portable Python runtime not found:
    echo   %PY%
    exit /b 1
)

if not exist "%SCRIPT%" (
    echo ERROR: script not found:
    echo   %SCRIPT%
    exit /b 1
)

if not exist "%CATALOG%" (
    echo ERROR: UI catalog not found:
    echo   %CATALOG%
    exit /b 1
)

if not exist "%SRC_UI%" (
    echo ERROR: UI source folder not found:
    echo   %SRC_UI%
    exit /b 1
)

"%PY%" "%SCRIPT%" ^
  --catalog "%CATALOG%" ^
  --src "%SRC_UI%"

exit /b %ERRORLEVEL%