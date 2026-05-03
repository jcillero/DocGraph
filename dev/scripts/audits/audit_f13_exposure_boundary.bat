@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..\..
for %%I in ("%ROOT_DIR%") do set ROOT_DIR=%%~fI

set OUTPUT_DIR=%ROOT_DIR%\user\output
set OUTPUT_FILE=%OUTPUT_DIR%\audit_f13_exposure_boundary_report.txt
for %%I in ("%OUTPUT_FILE%") do set OUTPUT_FILE=%%~fI

if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$self = Get-Content -Raw '%~f0';" ^
  "$marker = ':__POWERSHELL__';" ^
  "$index = $self.LastIndexOf($marker);" ^
  "if ($index -lt 0) { throw 'Missing embedded PowerShell block.' }" ^
  "$tmp = [System.IO.Path]::GetTempFileName() + '.ps1';" ^
  "Set-Content -Path $tmp -Value $self.Substring($index + $marker.Length);" ^
  "& powershell -NoProfile -ExecutionPolicy Bypass -File $tmp '%ROOT_DIR%' '%OUTPUT_FILE%';" ^
  "$code = $LASTEXITCODE;" ^
  "Remove-Item -Path $tmp -Force;" ^
  "exit $code"

exit /b %ERRORLEVEL%

:__POWERSHELL__
param(
    [string]$root,
    [string]$output
)

$uiFile = Join-Path $root 'crates\ui_slint\src\lib.rs'
$ioFile = Join-Path $root 'crates\io_runtime\src\lib.rs'
$appFile = Join-Path $root 'crates\app_services\src\lib.rs'
$toolDir = Join-Path $root 'crates\tool_runtime\src'

$pass = New-Object 'System.Collections.Generic.List[string]'
$fail = New-Object 'System.Collections.Generic.List[string]'
$warn = New-Object 'System.Collections.Generic.List[string]'
$findings = New-Object 'System.Collections.Generic.List[string]'

function Get-ProductionMatches {
    param(
        [string[]]$Paths,
        [string]$Pattern
    )

    $results = New-Object 'System.Collections.Generic.List[string]'
    $seen = @{}

    foreach ($path in $Paths) {
        if (-not (Test-Path $path)) {
            continue
        }

        $content = Get-Content -Path $path
        $marker = $content.Count + 1
        for ($i = 0; $i -lt $content.Count; $i++) {
            if ($content[$i] -match '^\s*#\[cfg\(test\)\]') {
                $marker = $i + 1
                break
            }
        }

        $matches = Select-String -Path $path -Pattern $Pattern
        foreach ($match in $matches) {
            if ($match.LineNumber -lt $marker) {
                $line = '{0}:{1}:{2}' -f $match.Path, $match.LineNumber, $match.Line.Trim()
                if (-not $seen.ContainsKey($line)) {
                    $seen[$line] = $true
                    $null = $results.Add($line)
                }
            }
        }
    }

    return $results
}

function Add-Check {
    param(
        [string]$Level,
        [string]$Label,
        [string[]]$Matches,
        [string]$Note
    )

    if (-not $Matches -or $Matches.Count -eq 0) {
        $null = $pass.Add('- ' + $Label)
        return
    }

    if ($Level -eq 'FAIL') {
        $null = $fail.Add('- ' + $Label)
    } else {
        $null = $warn.Add('- ' + $Label)
    }

    $null = $findings.Add('[' + $Level + '] ' + $Label)
    foreach ($line in $Matches) {
        $null = $findings.Add($line)
    }
    $null = $findings.Add('note=' + $Note)
    $null = $findings.Add('')
}

Add-Check 'FAIL' 'ui_slint must not call import_project_document as production intake/exposure path' `
    (Get-ProductionMatches @($uiFile) 'import_project_document\(') `
    'F13.1 STOP violation: UI still imports directly into project-facing areas.'

Add-Check 'FAIL' 'document tree must not treat list_project_documents or filesystem scanning as exposure authority' `
    (Get-ProductionMatches @($uiFile) 'list_project_documents\(') `
    'F13.1 STOP violation: tree visibility still depends on legacy filesystem listing.'

Add-Check 'FAIL' 'ui_slint must not call derive_document_text as import/exposure shortcut' `
    (Get-ProductionMatches @($uiFile) 'derive_document_text\(') `
    'F13.1 STOP violation: UI still triggers direct derivation on a legacy import path.'

Add-Check 'WARN' 'ui_slint still references register_chat_resource' `
    (Get-ProductionMatches @($uiFile) 'register_chat_resource\(') `
    'Allowed only as chat-local staging. This must not become intake or project exposure authority.'

Add-Check 'FAIL' 'ui_slint must not write project_manifest.json in production code' `
    (Get-ProductionMatches @($uiFile) 'fs::write\(.*project_manifest\.json') `
    'Production manifest writes would bypass the future explicit exposure gate.'

Add-Check 'FAIL' 'io_runtime must not write project_manifest.json outside a future explicit exposure gate' `
    (Get-ProductionMatches @($ioFile) 'fs::write\(.*project_manifest\.json') `
    'Manifest writes belong only to a later governed exposure implementation slice.'

Add-Check 'FAIL' 'app_services must not write project_manifest.json or shadow exposure logic' `
    (Get-ProductionMatches @($appFile) 'fs::write\(.*project_manifest\.json') `
    'app_services must remain a thin boundary and must not mutate manifest authority.'

$scanDirs = (
    Get-ChildItem -Path @(
        (Join-Path $root 'crates\ui_slint\src'),
        (Join-Path $root 'crates\io_runtime\src'),
        (Join-Path $root 'crates\app_services\src'),
        $toolDir
    ) -Recurse -File -Filter *.rs
).FullName

Add-Check 'FAIL' 'registry.json writes must not be introduced on active exposure/import paths' `
    (Get-ProductionMatches $scanDirs 'registry\.json') `
    'Registry remains derivable and non-authoritative.'

Add-Check 'FAIL' 'graph writes must not be introduced by exposure/import paths' `
    (Get-ProductionMatches $scanDirs 'graph\.json') `
    'Graph remains future and optional; exposure must not trigger graph materialization.'

$toolFiles = (Get-ChildItem -Path $toolDir -Recurse -File -Filter *.rs).FullName
$toolScan = @()
$toolScan += Get-ProductionMatches $toolFiles 'file_intake'
$toolScan += Get-ProductionMatches $toolFiles 'import_project_document'
$toolScan += Get-ProductionMatches $toolFiles 'register_chat_resource'
$toolScan += Get-ProductionMatches $toolFiles 'list_project_documents'
$toolScan += Get-ProductionMatches $toolFiles 'derive_document_text'
$toolScan += Get-ProductionMatches $toolFiles 'project_manifest\.json'

Add-Check 'FAIL' 'tool_runtime must not orchestrate file intake or project exposure' `
    $toolScan `
    'tool_runtime must stay outside intake and exposure orchestration.'

$appShadow = @()
$appShadow += Get-ProductionMatches @($appFile) 'ExposureRequest'
$appShadow += Get-ProductionMatches @($appFile) 'ExposureCandidate'
$appShadow += Get-ProductionMatches @($appFile) 'register_chat_resource'
$appShadow += Get-ProductionMatches @($appFile) 'import_project_document'
$appShadow += Get-ProductionMatches @($appFile) 'list_project_documents'
$appShadow += Get-ProductionMatches @($appFile) 'derive_document_text'
$appShadow += Get-ProductionMatches @($appFile) 'project_manifest\.json'

Add-Check 'FAIL' 'app_services must not shadow exposure logic' `
    $appShadow `
    'app_services must not absorb request, candidate, confirmation, or manifest authority.'

$ioLegacy = @()
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn import_project_document\('
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn list_project_documents\('
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn register_chat_resource\('

Add-Check 'WARN' 'io_runtime still contains legacy helpers that require migration behind F12/F13' `
    $ioLegacy `
    'Legacy helpers may remain temporarily, but they must not retain production authority once F13 runtime work begins.'

$lines = New-Object 'System.Collections.Generic.List[string]'
$null = $lines.Add('# F13 Exposure Boundary Audit')
$null = $lines.Add('')
$null = $lines.Add('root=' + $root)
$null = $lines.Add('report=' + $output)
$null = $lines.Add('')
$null = $lines.Add('## PASS')
foreach ($line in $pass) { $null = $lines.Add($line) }
$null = $lines.Add('')
$null = $lines.Add('## FAIL')
foreach ($line in $fail) { $null = $lines.Add($line) }
$null = $lines.Add('')
$null = $lines.Add('## WARN')
foreach ($line in $warn) { $null = $lines.Add($line) }
$null = $lines.Add('')
$null = $lines.Add('## Findings')
foreach ($line in $findings) { $null = $lines.Add($line) }
$null = $lines.Add('')
$null = $lines.Add('## Scanned Files')
$null = $lines.Add('- ' + $uiFile)
$null = $lines.Add('- ' + $ioFile)
$null = $lines.Add('- ' + $appFile)
$null = $lines.Add('- ' + (Join-Path $toolDir '*.rs'))
$null = $lines.Add('')
$null = $lines.Add('## Interpretation Notes')
$null = $lines.Add('- This audit scans production Rust code only and ignores lines after the first `#[cfg(test)]` marker in each file.')
$null = $lines.Add('- Docs and specs are not scanned for violations, so prohibitive wording there does not create false positives.')
$null = $lines.Add('- `register_chat_resource` is warning-only here because F13.1 still allows chat-local staging when it does not create project exposure authority.')
$null = $lines.Add('- Any FAIL indicates an active code-path or production helper usage that conflicts with F13.1 STOP conditions before F13 runtime.')
$null = $lines.Add('')

$status = if ($fail.Count -gt 0) { 'FAIL' } else { 'PASS' }
$null = $lines.Add('status=' + $status)
$null = $lines.Add('failures=' + $fail.Count)
$null = $lines.Add('warnings=' + $warn.Count)

Set-Content -Path $output -Value $lines
Get-Content -Path $output

if ($fail.Count -gt 0) {
    exit 1
}
