@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..
set "OUTPUT_FILE=%ROOT_DIR%\governance/reports/MASTER_GOVERNANCE_AND_SPECS_REPORT.md"
for %%I in ("%OUTPUT_FILE%") do set "OUTPUT_FILE=%%~fI"

echo [INFO] Generating master governance and specs report...

for %%I in ("%OUTPUT_FILE%") do set "OUTPUT_DIR=%%~dpI"
if not exist "%OUTPUT_DIR%" (
    mkdir "%OUTPUT_DIR%"
    if errorlevel 1 (
        echo [ERROR] Failed to create report output directory.
        endlocal
        exit /b 1
    )
)

(
echo # Master Governance And Specs Report
echo.
echo Generated from live sandbox documents.
echo.
echo - Workspace root: `%ROOT_DIR%`
echo - Generated at: `%DATE% %TIME%`
echo.
echo ---
echo.
echo ## Governance and architecture documents
echo.
) > "%OUTPUT_FILE%"

if errorlevel 1 (
    echo [ERROR] Failed to initialize generated report.
    endlocal
    exit /b 1
)

rem Canonical seed first: README is the intended workspace entrypoint.
for %%F in ("README.md") do (
    if exist "%ROOT_DIR%\%%~F" (
        (
        echo ### %%~F
        echo.
        type "%ROOT_DIR%\%%~F"
        echo.
        echo ---
        echo.
        ) >> "%OUTPUT_FILE%"
    )
)

rem Governance and architecture are collected recursively on purpose so the
rem report stays robust if either area gains nested folders such as reports/
rem or future structural sub-sections.
for /f "delims=" %%F in ('dir /b /s "%ROOT_DIR%\governance\*.md" 2^>nul') do (
    set "ABS_FILE=%%~fF"
    set "REL_FILE=!ABS_FILE:%ROOT_DIR%\=!"
    set "REL_FILE=!REL_FILE:\=/!"
    (
    echo ### !REL_FILE!
    echo.
    type "%%~fF"
    echo.
    echo ---
    echo.
    ) >> "%OUTPUT_FILE%"
)

for /f "delims=" %%F in ('dir /b /s "%ROOT_DIR%\architecture\*.md" 2^>nul') do (
    set "ABS_FILE=%%~fF"
    set "REL_FILE=!ABS_FILE:%ROOT_DIR%\=!"
    set "REL_FILE=!REL_FILE:\=/!"
    (
    echo ### !REL_FILE!
    echo.
    type "%%~fF"
    echo.
    echo ---
    echo.
    ) >> "%OUTPUT_FILE%"
)

(
echo ## Specs
echo.
) >> "%OUTPUT_FILE%"

for /f "delims=" %%F in ('dir /b /s "%ROOT_DIR%\docs\specs\*.md" 2^>nul') do (
    if /I not "%%~nxF"==".gitkeep" (
        set "ABS_FILE=%%~fF"
        set "REL_FILE=!ABS_FILE:%ROOT_DIR%\=!"
        set "REL_FILE=!REL_FILE:\=/!"
        (
        echo ### !REL_FILE!
        echo.
        type "%%~fF"
        echo.
        echo ---
        echo.
        ) >> "%OUTPUT_FILE%"
    )
)

if not exist "%OUTPUT_FILE%" (
    echo [ERROR] Failed to write generated report.
    endlocal
    exit /b 1
)

echo [OK] Master governance and specs report written to %OUTPUT_FILE%
endlocal
exit /b 0
