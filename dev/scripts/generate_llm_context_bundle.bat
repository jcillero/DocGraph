@echo off
setlocal EnableExtensions EnableDelayedExpansion
goto :main

:queue_expected
set "REL_PATH=%~1"
set "ABS_PATH=%ROOT_DIR%\!REL_PATH:/=\!"
if exist "!ABS_PATH!" (
    call :queue_absolute "!ABS_PATH!"
) else (
    call :record_warning "!REL_PATH!"
)
goto :eof

:queue_tree
set "REL_DIR=%~1"
set "PATTERN=%~2"
set "ABS_DIR=%ROOT_DIR%\!REL_DIR!"
if not exist "!ABS_DIR!" goto :eof
for /f "delims=" %%F in ('dir /b /s "!ABS_DIR!\!PATTERN!" 2^>nul') do (
    call :queue_absolute "%%~fF"
)
goto :eof

:queue_absolute
set "ABS_PATH=%~1"
call :relative_path "%~1"
set "REL_PATH=!RELATIVE_RESULT!"
call :is_queued "!REL_PATH!"
if "!IS_QUEUED!"=="0" (
    set /a INCLUDED_COUNT+=1
    >> "%MANIFEST_FILE%" echo !REL_PATH!^|!ABS_PATH!
)
goto :eof

:is_queued
set "LOOKUP=%~1"
set "IS_QUEUED=0"
if exist "%MANIFEST_FILE%" (
    for /f "usebackq tokens=1 delims=|" %%A in ("%MANIFEST_FILE%") do (
        if /I "%%A"=="!LOOKUP!" set "IS_QUEUED=1"
    )
)
goto :eof

:record_warning
set "MISSING_PATH=%~1"
set /a WARNING_COUNT+=1
echo WARNING: missing !MISSING_PATH!
>> "%WARNINGS_FILE%" echo !MISSING_PATH!
goto :eof

:relative_path
set "ABS_INPUT=%~1"
set "RELATIVE_RESULT=%ABS_INPUT%"
set "ROOT_PREFIX=%ROOT_DIR%\"
call set "RELATIVE_RESULT=%%RELATIVE_RESULT:%ROOT_PREFIX%=%%"
set "RELATIVE_RESULT=%RELATIVE_RESULT:\=/%"
goto :eof

:append_section
set "REL_PATH=%~1"
set "ABS_PATH=%~2"
(
echo.
echo ==============================
echo FILE: !REL_PATH!
echo ==============================
echo.
) >> "%BUNDLE_FILE%"

if exist "!ABS_PATH!" (
    type "!ABS_PATH!" >> "%BUNDLE_FILE%"
) else (
    echo [MISSING] !REL_PATH! >> "%BUNDLE_FILE%"
)
goto :eof

:write_bundle
(
echo # LLM Context Bundle
echo.
echo - Workspace root: `%ROOT_DIR%`
echo - Generated at: `%DATE% %TIME%`
echo - Script version: `%SCRIPT_VERSION%`
echo - Purpose: consolidated readonly context for LLM handoff, audit, and knowledge transfer.
echo - Included files: `%INCLUDED_COUNT%`
echo - Warnings: `%WARNING_COUNT%`
echo.
echo ---
echo.
echo ## Index
echo.
) > "%BUNDLE_FILE%"

set /a INDEX=0
if exist "%MANIFEST_FILE%" (
    for /f "usebackq tokens=1 delims=|" %%A in ("%MANIFEST_FILE%") do (
        set /a INDEX+=1
        >> "%BUNDLE_FILE%" echo [!INDEX!] %%A
    )
)

if exist "%WARNINGS_FILE%" (
    (
    echo.
    echo ## Warnings
    echo.
    ) >> "%BUNDLE_FILE%"
    for /f "usebackq delims=" %%M in ("%WARNINGS_FILE%") do (
        >> "%BUNDLE_FILE%" echo WARNING: missing %%M
    )
)

(
echo.
echo ---
echo.
echo ## Files
echo.
) >> "%BUNDLE_FILE%"

if exist "%MANIFEST_FILE%" (
    for /f "usebackq tokens=1,* delims=|" %%A in ("%MANIFEST_FILE%") do (
        call :append_section "%%A" "%%B"
    )
)
goto :eof

:main
set "SCRIPT_VERSION=2.0.0"
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..\..") do set "ROOT_DIR=%%~fI"
set "OUTPUT_DIR=%ROOT_DIR%\user\output"
set "BUNDLE_FILE=%OUTPUT_DIR%\llm_context_bundle.md"
set "MANIFEST_FILE=%TEMP%\docgraph_llm_context_manifest_%RANDOM%_%RANDOM%.tmp"
set "WARNINGS_FILE=%TEMP%\docgraph_llm_context_warnings_%RANDOM%_%RANDOM%.tmp"

set /a INCLUDED_COUNT=0
set /a WARNING_COUNT=0

echo [INFO] Generating LLM context bundle...

if not exist "%OUTPUT_DIR%" (
    mkdir "%OUTPUT_DIR%"
    if errorlevel 1 (
        echo [ERROR] Failed to create output directory.
        endlocal
        exit /b 1
    )
)

if exist "%MANIFEST_FILE%" del "%MANIFEST_FILE%" >nul 2>nul
if exist "%WARNINGS_FILE%" del "%WARNINGS_FILE%" >nul 2>nul

rem ---------------------------------------------------------------------------
rem 1. Canonical top-level context seed list.
rem
rem This list is intentionally hardcoded. It is not stale root coupling:
rem these are the canonical governance and architecture anchor documents that
rem must always appear first in the bundle even if broader recursive discovery
rem changes elsewhere in the repository.
rem ---------------------------------------------------------------------------
call :queue_expected "README.md"
call :queue_expected "governance/GOVERNANCE.md"
call :queue_expected "architecture/ARCHITECTURE.md"
call :queue_expected "governance/FUNCTIONAL_SCOPE.md"
call :queue_expected "governance/WORKSPACE_RULES.md"
call :queue_expected "architecture/MIGRATION_BASELINE.md"
call :queue_expected "governance/UI_SLINT_POLICY.md"
call :queue_expected "governance/I18N_POLICY.md"
call :queue_expected "governance/UI_THEME_POLICY.md"
call :queue_expected "governance/LLM_RUNTIME_POLICY.md"
call :queue_expected "governance/INHERITED_GOVERNANCE.md"
call :queue_expected "architecture/KNOWLEDGE_TRANSFER_MAP.md"
call :queue_expected "architecture/MODULE_MAPPING.md"

rem ---------------------------------------------------------------------------
rem 2. Policies and specs.
rem ---------------------------------------------------------------------------
call :queue_tree "docs\specs" "*.md"

rem ---------------------------------------------------------------------------
rem 3. Declarative resources.
rem ---------------------------------------------------------------------------
call :queue_tree "resources" "*.json"
call :queue_tree "resources" "*.ftl"
call :queue_tree "resources" "*.yaml"
call :queue_tree "resources" "*.toml"

rem ---------------------------------------------------------------------------
rem 4. Outputs and generated status artifacts.
rem ---------------------------------------------------------------------------
call :queue_tree "user\output" "*snapshot*.md"
call :queue_tree "user\output" "*report*.json"
call :queue_tree "user\output" "*.log"

call :write_bundle

echo [OK] LLM context bundle written to %BUNDLE_FILE%
echo [OK] included_files=%INCLUDED_COUNT%
echo [OK] warnings=%WARNING_COUNT%

if exist "%MANIFEST_FILE%" del "%MANIFEST_FILE%" >nul 2>nul
if exist "%WARNINGS_FILE%" del "%WARNINGS_FILE%" >nul 2>nul

endlocal
exit /b 0
