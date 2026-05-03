@echo off
setlocal enabledelayedexpansion

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..\..
for %%I in ("%ROOT_DIR%") do set ROOT_DIR=%%~fI
set OUTPUT_FILE=%ROOT_DIR%\user\output\audit_f12_file_intake_boundary_report.txt
for %%I in ("%OUTPUT_FILE%") do set OUTPUT_FILE=%%~fI

if not exist "%ROOT_DIR%\user\output" mkdir "%ROOT_DIR%\user\output"

set CARGO_FILE=%ROOT_DIR%\crates\io_runtime\Cargo.toml
set INTAKE_FILE=%ROOT_DIR%\crates\io_runtime\src\file_intake.rs
set FAILURES=0

> "%OUTPUT_FILE%" echo # F12 File Intake Boundary Audit
>> "%OUTPUT_FILE%" echo.
>> "%OUTPUT_FILE%" echo root=%ROOT_DIR%
>> "%OUTPUT_FILE%" echo.

findstr /I /C:"project_runtime" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on project_runtime& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on project_runtime
findstr /I /C:"tool_runtime" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on tool_runtime& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on tool_runtime
findstr /I /C:"app_services" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on app_services& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on app_services
findstr /I /C:"ui_core" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on ui_core& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on ui_core
findstr /I /C:"ui_slint" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on ui_slint& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on ui_slint
findstr /I /C:"document_text_runtime" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on document_text_runtime& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on document_text_runtime
findstr /I /C:"llm_" "%CARGO_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] io_runtime must not depend on LLM crates& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] io_runtime must not depend on LLM crates

findstr /I /C:"Command::" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not invoke external processes& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not invoke external processes
findstr /I /C:"TcpStream" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not access network TCP& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not access network TCP
findstr /I /C:"UdpSocket" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not access network UDP& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not access network UDP
findstr /I /C:"project_manifest.json" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not mutate project_manifest.json& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not mutate project_manifest.json
findstr /I /C:"registry.json" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not generate registry.json& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not generate registry.json
findstr /I /C:"\"graph\"" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not write graph paths& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not write graph paths
findstr /I /C:"\"derived\"" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not generate derived paths& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not generate derived paths
findstr /I /C:"\"chunks\"" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not generate chunk paths& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not generate chunk paths
findstr /I /C:"\"pages\"" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not generate page paths& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not generate page paths
findstr /I /C:"tool_runtime" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not orchestrate tool_runtime& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not orchestrate tool_runtime
findstr /I /C:"project_runtime" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not bypass project_runtime& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not bypass project_runtime
findstr /I /C:"app_services" "%INTAKE_FILE%" >nul 2>nul && (>> "%OUTPUT_FILE%" echo [FAIL] file intake must not add app_services orchestration& set /a FAILURES+=1) || >> "%OUTPUT_FILE%" echo [PASS] file intake must not add app_services orchestration

findstr /I /C:"MissingOwnerRef" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] owner_ref required& set /a FAILURES+=1)
findstr /I /C:"MissingOwnerRef" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] owner_ref required
findstr /I /C:"MissingTraceRef" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] trace_ref required& set /a FAILURES+=1)
findstr /I /C:"MissingTraceRef" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] trace_ref required
findstr /I /C:"sanitize_comment" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] user_comment sanitized& set /a FAILURES+=1)
findstr /I /C:"sanitize_comment" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] user_comment sanitized
findstr /I /C:"ImportedNotExposed" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] imported items remain imported_not_exposed& set /a FAILURES+=1)
findstr /I /C:"ImportedNotExposed" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] imported items remain imported_not_exposed
findstr /I /C:"SUPPORTED_TEXT_MEDIA_TYPE" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] text intake explicitly supported& set /a FAILURES+=1)
findstr /I /C:"SUPPORTED_TEXT_MEDIA_TYPE" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] text intake explicitly supported
findstr /I /C:"SUPPORTED_MARKDOWN_MEDIA_TYPE" "%INTAKE_FILE%" >nul 2>nul || (>> "%OUTPUT_FILE%" echo [FAIL] markdown intake explicitly supported& set /a FAILURES+=1)
findstr /I /C:"SUPPORTED_MARKDOWN_MEDIA_TYPE" "%INTAKE_FILE%" >nul 2>nul && >> "%OUTPUT_FILE%" echo [PASS] markdown intake explicitly supported

>> "%OUTPUT_FILE%" echo.
if "%FAILURES%"=="0" (
  >> "%OUTPUT_FILE%" echo status=PASS
  type "%OUTPUT_FILE%"
  exit /b 0
)

>> "%OUTPUT_FILE%" echo status=FAIL
>> "%OUTPUT_FILE%" echo failures=%FAILURES%
type "%OUTPUT_FILE%"
exit /b 1
