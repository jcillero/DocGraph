@echo off
setlocal

set "ROOT=%~dp0..\.."
set "PY=%ROOT%\system\bin\runtimes\python\current\python\python.exe"
set "SCRIPT=%ROOT%\dev\scripts\diagnostics\validate_logs.py"

set "EVENT_CATALOG=%ROOT%\system\config\event_catalog.json"
set "WORK_UNITS_CATALOG=%ROOT%\system\config\work_units_catalog.json"

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

if not exist "%EVENT_CATALOG%" (
    echo ERROR: event catalog not found:
    echo   %EVENT_CATALOG%
    exit /b 1
)

if not exist "%WORK_UNITS_CATALOG%" (
    echo ERROR: work units catalog not found:
    echo   %WORK_UNITS_CATALOG%
    exit /b 1
)

"%PY%" "%SCRIPT%" ^
  "%ROOT%" ^
  --event_catalog "%EVENT_CATALOG%" ^
  --work_units_catalog "%WORK_UNITS_CATALOG%"

exit /b %ERRORLEVEL%