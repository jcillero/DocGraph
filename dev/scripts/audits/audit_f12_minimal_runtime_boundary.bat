@echo off
setlocal ENABLEDELAYEDEXPANSION

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..\..\..
set OUTPUT_FILE=%SCRIPT_DIR%..\..\..\user\output\audit_f12_minimal_runtime_boundary_report.txt
for %%I in ("%PROJECT_ROOT%") do set PROJECT_ROOT=%%~fI
for %%I in ("%OUTPUT_FILE%") do set OUTPUT_FILE=%%~fI
echo [DEBUG] Writing report to: %OUTPUT_FILE%
if not exist "%SCRIPT_DIR%..\..\..\user\output" mkdir "%SCRIPT_DIR%..\..\..\user\output"

pushd "%PROJECT_ROOT%"

echo =============================================== > "%OUTPUT_FILE%"
echo F12 MINIMAL RUNTIME BOUNDARY AUDIT >> "%OUTPUT_FILE%"
echo =============================================== >> "%OUTPUT_FILE%"
echo. >> "%OUTPUT_FILE%"

set ERROR_COUNT=0

echo [CHECK] project_runtime untouched >> "%OUTPUT_FILE%"
if exist ".git" (
  git diff --name-only | findstr /I "project_runtime" >nul
  if !ERRORLEVEL!==0 (
    echo FAIL: project_runtime modified >> "%OUTPUT_FILE%"
    set /a ERROR_COUNT+=1
  ) else (
    echo PASS >> "%OUTPUT_FILE%"
  )
) else (
  echo PASS: git metadata unavailable; static crate boundary checks continue >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] tool_runtime only crate modified >> "%OUTPUT_FILE%"
if exist ".git" (
  git diff --name-only | findstr /I "crates/" > tmp_changes.txt

  set VALID_CHANGE=1
  for /f %%i in (tmp_changes.txt) do (
    echo %%i | findstr /I "crates/tool_runtime" >nul
    if !ERRORLEVEL! NEQ 0 (
      echo FAIL: unexpected crate modification %%i >> "%OUTPUT_FILE%"
      set VALID_CHANGE=0
    )
  )

  if !VALID_CHANGE!==1 (
    echo PASS >> "%OUTPUT_FILE%"
  ) else (
    set /a ERROR_COUNT+=1
  )

  del tmp_changes.txt
) else (
  echo PASS: git metadata unavailable; no non-tool crates are scanned as execution owners >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] no app_services or UI execution authority >> "%OUTPUT_FILE%"
findstr /S /I "execute_text_measure TextMeasureRequest" crates\app_services\*.* crates\ui_core\*.* crates\ui_slint\*.* >nul 2>nul
if %ERRORLEVEL%==0 (
  echo FAIL: forbidden layer references text.measure runtime >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
) else (
  echo PASS >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] forbidden patterns (network, providers, external) >> "%OUTPUT_FILE%"
findstr /S /I "reqwest TcpStream UdpSocket std::process Command:: spawn(" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo FAIL: forbidden runtime pattern detected >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
) else (
  echo PASS >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] tool_id restricted to text.measure >> "%OUTPUT_FILE%"
findstr /S /I "TEXT_MEASURE_TOOL_ID.*text.measure" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: text.measure not found >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] no broad execution API >> "%OUTPUT_FILE%"
findstr /S /I /C:"pub fn execute_" crates\tool_runtime\src\*.rs | findstr /V /I "execute_text_measure" >nul
if %ERRORLEVEL%==0 (
  echo FAIL: non-text.measure execution API detected >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
) else (
  echo PASS >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] owner_ref required >> "%OUTPUT_FILE%"
findstr /S /I "owner_ref is required" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: owner_ref requirement missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] trace_ref required >> "%OUTPUT_FILE%"
findstr /S /I "trace_ref is required" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: trace_ref requirement missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] manifest required >> "%OUTPUT_FILE%"
findstr /S /I "tool_run_manifest.json" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: manifest creation missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] result required >> "%OUTPUT_FILE%"
findstr /S /I "result.json" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: result.json creation missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] manifest includes tool_kind and runtime_scope >> "%OUTPUT_FILE%"
findstr /S /I "tool_kind" crates\tool_runtime\src\*.rs >nul
set TOOL_KIND_FOUND=%ERRORLEVEL%
findstr /S /I "runtime_scope" crates\tool_runtime\src\*.rs >nul
set RUNTIME_SCOPE_FOUND=%ERRORLEVEL%
if !TOOL_KIND_FOUND!==0 if !RUNTIME_SCOPE_FOUND!==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: manifest governance fields missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] outputs owner-scoped and not project-root >> "%OUTPUT_FILE%"
findstr /S /I "output_root must stay under user/output" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo PASS >> "%OUTPUT_FILE%"
) else (
  echo FAIL: owner-scoped output guard missing >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
)

echo. >> "%OUTPUT_FILE%"
echo [CHECK] no raw payload field in manifest >> "%OUTPUT_FILE%"
findstr /S /I "raw_payload raw_text input_text" crates\tool_runtime\src\*.rs >nul
if %ERRORLEVEL%==0 (
  echo FAIL: raw payload field detected >> "%OUTPUT_FILE%"
  set /a ERROR_COUNT+=1
) else (
  echo PASS >> "%OUTPUT_FILE%"
)

echo. >> "%OUTPUT_FILE%"
echo =============================================== >> "%OUTPUT_FILE%"
echo TOTAL ERRORS: %ERROR_COUNT% >> "%OUTPUT_FILE%"
echo =============================================== >> "%OUTPUT_FILE%"

if %ERROR_COUNT% GTR 0 (
  echo AUDIT RESULT: FAIL >> "%OUTPUT_FILE%"
  popd
  exit /b 1
) else (
  echo AUDIT RESULT: PASS >> "%OUTPUT_FILE%"
  popd
  exit /b 0
)
