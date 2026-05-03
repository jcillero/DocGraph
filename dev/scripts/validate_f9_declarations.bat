@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..
set PS_SCRIPT=%SCRIPT_DIR%tools\validate_f9_declarations.ps1

echo [INFO] Validating F9 declarative resources...

powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" -WorkspaceRoot "%ROOT_DIR%"
set "exit_code=%errorlevel%"

if %exit_code% neq 0 (
    echo [ERROR] validate_f9_declarations failed with exit code %exit_code%
) else (
    echo [OK] validate_f9_declarations=true
)

endlocal & exit /b %exit_code%
