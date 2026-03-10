@echo off
setlocal

set "ROOT=%~dp0.."
set "PY=%ROOT%\system\bin\runtimes\python\current\python\python.exe"
set "LAUNCHER=%ROOT%\dev\scripts\dev_tool_launcher.py"
set "REGISTRY=%ROOT%\dev\config\dev_tools_registry.json"

if not exist "%PY%" (
    echo ERROR: portable Python runtime not found:
    echo   %PY%
    echo.
    pause
    exit /b 1
)

if not exist "%LAUNCHER%" (
    echo ERROR: launcher script not found:
    echo   %LAUNCHER%
    echo.
    pause
    exit /b 1
)

if not exist "%REGISTRY%" (
    echo ERROR: dev tools registry not found:
    echo   %REGISTRY%
    echo.
    pause
    exit /b 1
)

"%PY%" "%LAUNCHER%" "%ROOT%" "%REGISTRY%"
set "ERR=%ERRORLEVEL%"

if not "%ERR%"=="0" (
    echo.
    echo Launcher finished with errors. Exit code: %ERR%
    echo.
    pause
)

exit /b %ERR%