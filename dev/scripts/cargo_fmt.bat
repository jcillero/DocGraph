@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..

pushd "%ROOT_DIR%" >nul

echo [INFO] Running cargo fmt -- --check...

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

cargo fmt -- --check
set "exit_code=%errorlevel%"

if %exit_code% neq 0 (
    echo [ERROR] cargo fmt -- --check failed with exit code %exit_code%
) else (
    echo [OK] cargo fmt -- --check passed
)

popd >nul
endlocal & exit /b %exit_code%
