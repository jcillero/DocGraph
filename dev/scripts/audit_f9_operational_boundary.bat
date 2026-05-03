@echo off
setlocal

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$file = '%~f0';" ^
  "$content = Get-Content -LiteralPath $file;" ^
  "$marker = ($content | Select-String '^# POWERSHELL$' | Select-Object -First 1).LineNumber;" ^
  "if (-not $marker) { throw 'PowerShell marker not found.' }" ^
  "$script = ($content[($marker)..($content.Length - 1)] -join [Environment]::NewLine);" ^
  "Invoke-Expression $script"

set EXIT_CODE=%ERRORLEVEL%
endlocal & exit /b %EXIT_CODE%
# POWERSHELL
$scriptDir = Split-Path -Parent $file
$rootDir = [IO.Path]::GetFullPath((Join-Path $scriptDir '..\..'))
$reportPath = Join-Path $rootDir 'user\output\audit_f9_operational_boundary_report.md'
$toolRunsDir = Join-Path $rootDir 'user\output\tool_runs'
$rootOutputsDir = Join-Path $rootDir 'outputs'
$errors = New-Object System.Collections.Generic.List[string]
$warnings = New-Object System.Collections.Generic.List[string]

function Add-LiteralFindings {
    param(
        [string]$Path,
        [string[]]$Patterns,
        [string]$Severity
    )

    foreach ($pattern in $Patterns) {
        $matches = Select-String -Path $Path -Pattern $pattern -SimpleMatch -ErrorAction SilentlyContinue
        foreach ($match in $matches) {
            $message = "$($match.Path.Substring($rootDir.Length + 1).Replace('\','/')):$($match.LineNumber) -> $pattern"
            if ($Severity -eq 'ERROR') {
                $errors.Add($message)
            } else {
                $warnings.Add($message)
            }
        }
    }
}

function Parse-OwnerScope {
    param([string]$OwnerRef)

    if ([string]::IsNullOrWhiteSpace($OwnerRef)) {
        return $null
    }

    $parts = $OwnerRef -split '://', 2
    if ($parts.Count -ne 2) {
        return $null
    }

    $ownerKindPath = switch ($parts[0].ToLowerInvariant()) {
        'chat' { 'chat' }
        'documentholder' { 'document_holder' }
        'knowledge' { 'knowledge' }
        default { $null }
    }

    $ownerIdPath = $parts[1].Trim()
    if ([string]::IsNullOrWhiteSpace($ownerKindPath) -or [string]::IsNullOrWhiteSpace($ownerIdPath)) {
        return $null
    }

    if ($ownerIdPath.Contains('/') -or $ownerIdPath.Contains('\') -or $ownerIdPath.Contains(':') -or $ownerIdPath -eq '.' -or $ownerIdPath -eq '..') {
        return $null
    }

    foreach ($ch in $ownerIdPath.ToCharArray()) {
        if (-not [char]::IsLetterOrDigit($ch) -and $ch -notin @('-', '_', '.')) {
            return $null
        }
    }

    return [pscustomobject]@{
        OwnerKindPath = $ownerKindPath
        OwnerIdPath = $ownerIdPath
    }
}

if (Test-Path -LiteralPath $rootOutputsDir) {
    $errors.Add('root outputs/ directory must not exist')
}

if (-not (Test-Path -LiteralPath $toolRunsDir)) {
    $errors.Add('user/output/tool_runs/ does not exist')
}

$validTextMeasureRunCount = 0
$manifestsChecked = 0
$legacyRunsExcluded = 0

if (Test-Path -LiteralPath $toolRunsDir) {
    $manifestFiles = Get-ChildItem -LiteralPath $toolRunsDir -Recurse -File -Filter 'tool_run_manifest.json'
    foreach ($manifestFile in $manifestFiles) {
        $runDir = Split-Path -Parent $manifestFile.FullName
        $runRelative = $runDir.Substring($toolRunsDir.Length + 1).Replace('\', '/')
        $segments = @($runRelative -split '/')

        $manifestsChecked++
        try {
            $manifest = Get-Content -LiteralPath $manifestFile.FullName -Raw | ConvertFrom-Json
        } catch {
            $errors.Add("invalid JSON manifest in $runRelative")
            continue
        }

        if ([string]::IsNullOrWhiteSpace([string]$manifest.owner_ref)) {
            $errors.Add("owner_ref missing in $runRelative")
            continue
        }

        $ownerScope = Parse-OwnerScope ([string]$manifest.owner_ref)
        if ($null -eq $ownerScope) {
            $errors.Add("owner_ref is not portable or uses unsupported owner kind in $runRelative")
            continue
        }

        $isLegacyRun = ($segments.Count -eq 1 -and [string]$manifest.run_id -eq $segments[0])
        if ($isLegacyRun) {
            $legacyRunsExcluded++
            $warnings.Add("legacy non-owner-scoped run excluded from F10 gate: $runRelative")
            continue
        }

        if ($segments.Count -ne 3) {
            $errors.Add("run path is not owner-scoped: $runRelative")
            continue
        }

        if ($segments[0] -ne $ownerScope.OwnerKindPath -or $segments[1] -ne $ownerScope.OwnerIdPath -or $segments[2] -ne [string]$manifest.run_id) {
            $errors.Add("run path does not match owner_ref or run_id in $runRelative")
            continue
        }

        if ([string]$manifest.status -eq 'completed' -and [string]$manifest.tool_id -eq 'text.measure') {
            $resultPath = Join-Path $runDir 'result.json'
            if (Test-Path -LiteralPath $resultPath) {
                $validTextMeasureRunCount++
            } else {
                $errors.Add("completed text.measure run missing result.json in $runRelative")
            }
        }
    }
}

if ($validTextMeasureRunCount -lt 1) {
    $errors.Add('no valid owner-scoped completed text.measure run found')
}

$uiSlintPath = Join-Path $rootDir 'crates\ui_slint\src\lib.rs'
$llmCloudPath = Join-Path $rootDir 'crates\llm_cloud\src\lib.rs'
$llmCorePath = Join-Path $rootDir 'crates\llm_core\src\lib.rs'
$toolRuntimePath = Join-Path $rootDir 'crates\tool_runtime\src\lib.rs'
$cliAppPath = Join-Path $rootDir 'crates\cli_app\src\main.rs'
$actionCorePath = Join-Path $rootDir 'crates\action_core\src\lib.rs'

Add-LiteralFindings -Path $uiSlintPath -Patterns @(
    'ToolRuntimeRunner::new().dispatch',
    'resolve_effective_llm_tool_policy',
    'save_global_llm_tool_policy',
    'save_project_llm_tool_policy',
    'load_global_llm_tool_policy',
    'load_project_llm_tool_policy'
) -Severity 'ERROR'

Add-LiteralFindings -Path $llmCloudPath -Patterns @(
    'reqwest::',
    'api.openai.com',
    '.send(',
    'resolve_cloud_credential(',
    'evaluate_cloud_provider_readiness('
) -Severity 'ERROR'

Add-LiteralFindings -Path $actionCorePath -Patterns @(
    'ActionRunner',
    'ActionDispatcher',
    'ActionExecutor',
    'ToolRuntimeRunner',
    'std::process::Command',
    'reqwest::',
    'fs::write',
    'fs::create_dir_all'
) -Severity 'ERROR'

Add-LiteralFindings -Path $toolRuntimePath -Patterns @(
    'tools_master_catalog.json'
) -Severity 'ERROR'

Add-LiteralFindings -Path $cliAppPath -Patterns @(
    'tools_master_catalog.json'
) -Severity 'ERROR'

$crateSources = Get-ChildItem -LiteralPath (Join-Path $rootDir 'crates') -Recurse -File -Filter '*.rs'
foreach ($sourceFile in $crateSources) {
    $matches = Select-String -Path $sourceFile.FullName -Pattern 'tools_master_catalog.json' -SimpleMatch -ErrorAction SilentlyContinue
    foreach ($match in $matches) {
        $errors.Add("$($match.Path.Substring($rootDir.Length + 1).Replace('\','/')):$($match.LineNumber) -> tools_master_catalog.json")
    }
}

if (Test-Path -LiteralPath $llmCorePath) {
    $llmCoreText = Get-Content -LiteralPath $llmCorePath -Raw
    if ($llmCoreText -match 'api\.openai\.com' -or $llmCoreText -match 'reqwest::') {
        $errors.Add('llm_core must not embed provider invocation details')
    }
}

$status = if ($errors.Count -eq 0) { 'PASS' } else { 'FAIL' }
$lines = New-Object System.Collections.Generic.List[string]
$lines.Add('# F9 Operational Boundary Audit Report')
$lines.Add('')
$lines.Add("- generated_at: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
$lines.Add("- workspace_root: $rootDir")
$lines.Add("- status: $status")
$lines.Add("- manifests_checked: $manifestsChecked")
$lines.Add("- valid_text_measure_runs: $validTextMeasureRunCount")
$lines.Add("- legacy_runs_excluded: $legacyRunsExcluded")
$lines.Add('')
$lines.Add('## Verified')
$lines.Add('')
$lines.Add('- minimum governed output must be owner-scoped under `user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>/`')
$lines.Add('- completed `text.measure` runs require `tool_run_manifest.json`, `owner_ref`, and `result.json`')
$lines.Add('- UI, LLM cloud, ActionRequest, and master-catalog boundaries are scanned for F10 drift signals')
$lines.Add('')
$lines.Add('## Warnings')
$lines.Add('')
if ($warnings.Count -eq 0) {
    $lines.Add('- none')
} else {
    foreach ($warning in $warnings) {
        $lines.Add("- $warning")
    }
}
$lines.Add('')
$lines.Add('## Errors')
$lines.Add('')
if ($errors.Count -eq 0) {
    $lines.Add('- none')
} else {
    foreach ($error in $errors) {
        $lines.Add("- $error")
    }
}
$lines.Add('')
$lines.Add('## Conclusion')
$lines.Add('')
if ($status -eq 'PASS') {
    $lines.Add('- minimum governed F9 execution slice present')
    $lines.Add('- no clear runtime drift to F10 detected by this audit')
    $lines.Add('- F10 remains `NOT OPENED`')
} else {
    $lines.Add('- clear F9 operational boundary violations detected')
}

Set-Content -LiteralPath $reportPath -Value $lines -Encoding UTF8
Write-Host "manifests_checked=$manifestsChecked"
Write-Host "valid_text_measure_runs=$validTextMeasureRunCount"
Write-Host "legacy_runs_excluded=$legacyRunsExcluded"
Write-Host "warning_count=$($warnings.Count)"
Write-Host "error_count=$($errors.Count)"
Write-Host "status=$status"

if ($status -eq 'FAIL') { exit 1 }
exit 0
