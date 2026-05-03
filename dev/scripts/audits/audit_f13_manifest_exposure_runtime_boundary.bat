@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..\..
for %%I in ("%ROOT_DIR%") do set ROOT_DIR=%%~fI

set OUTPUT_DIR=%ROOT_DIR%\user\output
set OUTPUT_FILE=%OUTPUT_DIR%\audit_f13_manifest_exposure_runtime_boundary_report.txt
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
$appFile = Join-Path $root 'crates\app_services\src\lib.rs'
$ioFile = Join-Path $root 'crates\io_runtime\src\lib.rs'
$toolDir = Join-Path $root 'crates\tool_runtime\src'
$projectDir = Join-Path $root 'crates\project_runtime\src'
$docTextDir = Join-Path $root 'crates\document_text_runtime\src'

$ignoreMarkers = 'SPEC-ONLY|FUTURE|TODO'

$pass = New-Object 'System.Collections.Generic.List[string]'
$fail = New-Object 'System.Collections.Generic.List[string]'
$warn = New-Object 'System.Collections.Generic.List[string]'
$findings = New-Object 'System.Collections.Generic.List[string]'

function Get-ProductionMatches {
    param(
        [string[]]$Paths,
        [string]$Pattern,
        [string]$IgnorePattern = $null
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
            if ($match.LineNumber -ge $marker) {
                continue
            }

            $lineText = $match.Line.Trim()
            if ($IgnorePattern -and $lineText -match $IgnorePattern) {
                continue
            }

            $line = '{0}:{1}:{2}' -f $match.Path, $match.LineNumber, $lineText
            if (-not $seen.ContainsKey($line)) {
                $seen[$line] = $true
                $null = $results.Add($line)
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

function Add-RequirementWhenActive {
    param(
        [string]$Label,
        [bool]$IsActive,
        [string[]]$Matches,
        [string]$MissingNote
    )

    if (-not $IsActive) {
        $null = $pass.Add('- ' + $Label)
        return
    }

    if ($Matches -and $Matches.Count -gt 0) {
        $null = $pass.Add('- ' + $Label)
        return
    }

    $null = $fail.Add('- ' + $Label)
    $null = $findings.Add('[FAIL] ' + $Label)
    $null = $findings.Add('note=' + $MissingNote)
    $null = $findings.Add('')
}

$toolFiles = (Get-ChildItem -Path $toolDir -Recurse -File -Filter *.rs).FullName
$projectFiles = (Get-ChildItem -Path $projectDir -Recurse -File -Filter *.rs).FullName
$docTextFiles = (Get-ChildItem -Path $docTextDir -Recurse -File -Filter *.rs).FullName
$nonProjectFiles = @($uiFile, $appFile, $ioFile) + $toolFiles + $docTextFiles
$allRelevantFiles = $nonProjectFiles + $projectFiles

$manifestWritePattern = '((fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer).*(project_manifest\.json))|((project_manifest\.json).*(fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer))'
$registryWritePattern = '((fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer).*(registry\.json))|((registry\.json).*(fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer))'
$graphWritePattern = '((fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer).*(graph_manifest\.json|artifact_nodes\.jsonl|artifact_edges\.jsonl))|((graph_manifest\.json|artifact_nodes\.jsonl|artifact_edges\.jsonl).*(fs::write|std::fs::write|File::create|OpenOptions::new|write_all|to_writer))'
$derivativeWritePattern = '((fs::write|std::fs::write|File::create|OpenOptions::new|write_all|create_dir_all).*(derived|chunks|pages|text))|((derived|chunks|pages|text).*(fs::write|std::fs::write|File::create|OpenOptions::new|write_all|create_dir_all))'

Add-Check 'FAIL' 'project_manifest.json must not be written from ui_slint' `
    (Get-ProductionMatches @($uiFile) $manifestWritePattern $ignoreMarkers) `
    'Only a future approved project_runtime exposure slice may write manifest authority.'

Add-Check 'FAIL' 'project_manifest.json must not be written from app_services' `
    (Get-ProductionMatches @($appFile) $manifestWritePattern $ignoreMarkers) `
    'app_services must remain thin and non-authoritative for project exposure.'

Add-Check 'FAIL' 'project_manifest.json must not be written from io_runtime' `
    (Get-ProductionMatches @($ioFile) $manifestWritePattern $ignoreMarkers) `
    'io_runtime may provide low-level IO only and must not write manifest exposure authority.'

Add-Check 'FAIL' 'project_manifest.json must not be written from tool_runtime' `
    (Get-ProductionMatches $toolFiles $manifestWritePattern $ignoreMarkers) `
    'tool_runtime must not orchestrate or persist manifest exposure authority.'

Add-Check 'FAIL' 'project_manifest.json must not be written from document_text_runtime' `
    (Get-ProductionMatches $docTextFiles $manifestWritePattern $ignoreMarkers) `
    'document_text_runtime must remain separate from manifest exposure.'

Add-Check 'FAIL' 'project_manifest writer must not appear outside the approved project_runtime boundary' `
    (Get-ProductionMatches $nonProjectFiles $manifestWritePattern $ignoreMarkers) `
    'Manifest writer ownership is reserved to project_runtime only.'

Add-Check 'FAIL' 'registry.json must not be written as part of exposure flow' `
    (Get-ProductionMatches $allRelevantFiles $registryWritePattern $ignoreMarkers) `
    'Registry remains derivable and non-authoritative.'

Add-Check 'FAIL' 'graph files must not be written as part of exposure flow' `
    (Get-ProductionMatches $allRelevantFiles $graphWritePattern $ignoreMarkers) `
    'Graph remains optional and future-only and must not be materialized by exposure.'

Add-Check 'FAIL' 'derivatives text/pages/chunks must not be created as part of exposure flow' `
    (Get-ProductionMatches $projectFiles $derivativeWritePattern $ignoreMarkers) `
    'Manifest exposure must not create derivatives or derived storage by convenience.'

$docTextInvocation = @()
$docTextInvocation += Get-ProductionMatches $projectFiles 'derive_document_text\(' $ignoreMarkers
$docTextInvocation += Get-ProductionMatches $projectFiles 'document_text_runtime' $ignoreMarkers
Add-Check 'FAIL' 'document_text_runtime must not be invoked from exposure code' `
    $docTextInvocation `
    'Exposure must not trigger document text derivation.'

$toolOrchestration = @()
$toolOrchestration += Get-ProductionMatches $projectFiles 'tool_runtime' $ignoreMarkers
$toolOrchestration += Get-ProductionMatches $toolFiles 'ExposureRequest|ExposureCandidate|HumanConfirmation|ManifestExposureEntry|exposed_to_project|project_manifest\.json' $ignoreMarkers
Add-Check 'FAIL' 'tool_runtime must not orchestrate exposure' `
    $toolOrchestration `
    'tool_runtime must remain outside manifest exposure orchestration.'

$fsAuthority = @()
$fsAuthority += Get-ProductionMatches $projectFiles 'list_project_documents\(' $ignoreMarkers
$fsAuthority += Get-ProductionMatches $projectFiles 'read_dir\(' $ignoreMarkers
Add-Check 'FAIL' 'filesystem scanning or list_project_documents must not be used as exposure authority' `
    $fsAuthority `
    'Exposure must remain manifest-driven and must not depend on filesystem scanning authority.'

$hostPathPatterns = '([A-Za-z]:\\\\|/Users/|/home/|/private/var/)'
$manifestOutputHostPaths = @()
$manifestOutputHostPaths += Get-ProductionMatches $projectFiles '(ManifestExposureEntry|exposed_to_project|project_manifest\.json).*(display\(\)|to_string_lossy\(\)|canonicalize\(\))' $ignoreMarkers
$manifestOutputHostPaths += Get-ProductionMatches $projectFiles $hostPathPatterns $ignoreMarkers
Add-Check 'FAIL' 'raw absolute host paths must not be persisted into manifest exposure outputs' `
    $manifestOutputHostPaths `
    'Manifest exposure outputs must use sanitized portable refs only.'

$projectManifestWrites = Get-ProductionMatches $projectFiles $manifestWritePattern $ignoreMarkers
if ($projectManifestWrites.Count -gt 0) {
    Add-RequirementWhenActive 'future manifest exposure code must not bypass ExposureRequest' `
        $true `
        (Get-ProductionMatches $projectFiles 'ExposureRequest' $ignoreMarkers) `
        'A project_runtime manifest writer exists without explicit ExposureRequest contract evidence.'

    Add-RequirementWhenActive 'future manifest exposure code must not bypass ExposureCandidate' `
        $true `
        (Get-ProductionMatches $projectFiles 'ExposureCandidate' $ignoreMarkers) `
        'A project_runtime manifest writer exists without explicit ExposureCandidate contract evidence.'

    $confirmationType = Get-ProductionMatches $projectFiles 'HumanConfirmation' $ignoreMarkers
    $confirmationAccepted = Get-ProductionMatches $projectFiles 'accepted' $ignoreMarkers
    $confirmationMatches = New-Object 'System.Collections.Generic.List[string]'
    foreach ($match in $confirmationType) { $null = $confirmationMatches.Add($match) }
    foreach ($match in $confirmationAccepted) { $null = $confirmationMatches.Add($match) }

    Add-RequirementWhenActive 'future manifest exposure code must not bypass accepted HumanConfirmation' `
        $true `
        $confirmationMatches `
        'A project_runtime manifest writer exists without explicit accepted HumanConfirmation contract evidence.'
} else {
    $null = $pass.Add('- future manifest exposure code must not bypass ExposureRequest')
    $null = $pass.Add('- future manifest exposure code must not bypass ExposureCandidate')
    $null = $pass.Add('- future manifest exposure code must not bypass accepted HumanConfirmation')
}

$uiLegacyBlocked = @()
$uiLegacyBlocked += Get-ProductionMatches @($uiFile) 'import_project_document\(' $ignoreMarkers
$uiLegacyBlocked += Get-ProductionMatches @($uiFile) 'list_project_documents\(' $ignoreMarkers
$uiLegacyBlocked += Get-ProductionMatches @($uiFile) 'derive_document_text\(' $ignoreMarkers

$ioLegacy = @()
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn import_project_document\(' $ignoreMarkers
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn list_project_documents\(' $ignoreMarkers
$ioLegacy += Get-ProductionMatches @($ioFile) 'pub fn register_chat_resource\(' $ignoreMarkers
if ($ioLegacy.Count -gt 0 -and $uiLegacyBlocked.Count -eq 0) {
    Add-Check 'WARN' 'legacy helpers remain in io_runtime but are not called by production UI' `
        $ioLegacy `
        'Legacy helpers may remain temporarily while production UI keeps the governed boundary closed.'
} else {
    $null = $pass.Add('- legacy helpers remain in io_runtime but are not called by production UI')
}

Add-Check 'WARN' 'register_chat_resource remains in chat-local context' `
    (Get-ProductionMatches @($uiFile) 'register_chat_resource\(' $ignoreMarkers) `
    'Allowed only as chat-local staging. This must not become intake or project exposure authority.'

Add-Check 'WARN' 'deprecated shorthand exposed appears where exposed_to_project should be preferred' `
    (Get-ProductionMatches $allRelevantFiles '\bexposed\b' $ignoreMarkers) `
    'Prefer exposed_to_project as the canonical exposure term in production code.'

$lines = New-Object 'System.Collections.Generic.List[string]'
$null = $lines.Add('# F13 Manifest Exposure Runtime Boundary Audit')
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
$null = $lines.Add('- ' + $appFile)
$null = $lines.Add('- ' + $ioFile)
$null = $lines.Add('- ' + (Join-Path $projectDir '*.rs'))
$null = $lines.Add('- ' + (Join-Path $toolDir '*.rs'))
$null = $lines.Add('- ' + (Join-Path $docTextDir '*.rs'))
$null = $lines.Add('')
$null = $lines.Add('## Interpretation Notes')
$null = $lines.Add('- This audit scans production Rust code only and ignores lines after the first `#[cfg(test)]` marker in each file.')
$null = $lines.Add('- Docs, specs, and audit scripts are not scanned for violations, so prohibitive wording there does not create false positives.')
$null = $lines.Add('- Lines marked as SPEC-ONLY, FUTURE, or TODO are ignored when they appear as clearly non-active comments.')
$null = $lines.Add('- This audit is deterministic and pattern-based: it checks explicit paths and tokens, not semantic control-flow inference.')
$null = $lines.Add('- Any FAIL indicates a boundary violation that must be resolved before future F13.5B closure.')
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
