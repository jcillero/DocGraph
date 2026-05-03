@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..

echo [INFO] Validating F9.3/F9.4 AI declarative specs...

pushd "%ROOT_DIR%" >nul

where cargo >nul 2>&1
if errorlevel 1 (
    if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
        set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
    ) else (
        echo [ERROR] cargo not found in PATH or in %%USERPROFILE%%\.cargo\bin
        popd >nul
        endlocal
        exit /b 1
    )
)

cargo run -p verify_progress -- --validate-ai-specs --workspace-root "%ROOT_DIR%"
set "exit_code=%errorlevel%"

popd >nul

if %exit_code% neq 0 (
    echo [ERROR] validate_ai_specs failed with exit code %exit_code%
) else (
    echo [OK] validate_ai_specs=true
)

endlocal & exit /b %exit_code%
