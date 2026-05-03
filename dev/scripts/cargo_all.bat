@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..

echo [INFO] Running full Rust mechanical validation...

call "%SCRIPT_DIR%cargo_check.bat"
if errorlevel 1 (
    echo [ERROR] cargo_check.bat failed
    endlocal
    exit /b %ERRORLEVEL%
)

call "%SCRIPT_DIR%cargo_test.bat"
if errorlevel 1 (
    echo [ERROR] cargo_test.bat failed
    endlocal
    exit /b %ERRORLEVEL%
)

echo [OK] Rust mechanical validation passed.
endlocal
exit /b 0
