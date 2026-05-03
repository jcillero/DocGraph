@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..

pushd "%ROOT_DIR%" >nul

echo [INFO] Running cargo clippy -- -D warnings...

REM Try PATH first
where cargo >nul 2>&1
if errorlevel 1 (
    REM Fallback to default rustup cargo location
    if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
        set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
    ) else (
        echo [ERROR] cargo not found in PATH or in %%USERPROFILE%%\.cargo\bin
        popd >nul
        endlocal
        exit /b 1
    )
)

cargo clippy -- -D warnings
set "exit_code=%errorlevel%"

if %exit_code% neq 0 (
    echo [ERROR] cargo clippy -- -D warnings failed with exit code %exit_code%
) else (
    echo [OK] cargo clippy -- -D warnings passed
)

popd >nul
endlocal & exit /b %exit_code%
