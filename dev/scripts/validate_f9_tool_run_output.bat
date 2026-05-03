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
$reportPath = Join-Path $rootDir 'user\output\validate_f9_tool_run_output_report.txt'
$toolRunsDir = Join-Path $rootDir 'user\output\tool_runs'
$rootOutputsDir = Join-Path $rootDir 'outputs'
$issues = New-Object System.Collections.Generic.List[string]
$warnings = New-Object System.Collections.Generic.List[string]

function Parse-OwnerScope {
    param([string]$OwnerRef)

    if ([string]::IsNullOrWhiteSpace($OwnerRef)) {
        return $null
    }

    $parts = $OwnerRef -split '://', 2
    if ($parts.Count -ne 2) {
        return $null
    }

    $ownerKind = $parts[0].ToLowerInvariant()
    $ownerId = $parts[1].Trim()

    $ownerKindPath = switch ($ownerKind) {
        'chat' { 'chat' }
        'documentholder' { 'document_holder' }
        'knowledge' { 'knowledge' }
        default { $null }
    }

    if ([string]::IsNullOrWhiteSpace($ownerKindPath) -or [string]::IsNullOrWhiteSpace($ownerId)) {
        return $null
    }

    if ($ownerId.Contains('/') -or $ownerId.Contains('\') -or $ownerId.Contains(':') -or $ownerId -eq '.' -or $ownerId -eq '..') {
        return $null
    }

    foreach ($ch in $ownerId.ToCharArray()) {
        if (-not [char]::IsLetterOrDigit($ch) -and $ch -notin @('-', '_', '.')) {
            return $null
        }
    }

    return [pscustomobject]@{
        OwnerKindPath = $ownerKindPath
        OwnerIdPath = $ownerId
    }
}

if (Test-Path -LiteralPath $rootOutputsDir) {
    $issues.Add('root outputs/ directory must not exist')
}

if (-not (Test-Path -LiteralPath $toolRunsDir)) {
    $issues.Add('user/output/tool_runs/ does not exist')
}

$manifestsChecked = 0
$completedRunsChecked = 0
$failedRunsChecked = 0
$legacyRunsExcluded = 0
$ownerScopedRunsChecked = 0

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
            $issues.Add("invalid JSON manifest in $runRelative")
            continue
        }

        if ([string]::IsNullOrWhiteSpace([string]$manifest.owner_ref)) {
            $issues.Add("owner_ref missing in $runRelative")
        }
        if ([string]::IsNullOrWhiteSpace([string]$manifest.schema_version)) {
            $issues.Add("schema_version missing in $runRelative")
        }
        if ([string]::IsNullOrWhiteSpace([string]$manifest.run_id)) {
            $issues.Add("run_id missing in $runRelative")
        }
        if ([string]::IsNullOrWhiteSpace([string]$manifest.tool_id)) {
            $issues.Add("tool_id missing in $runRelative")
        }
        if ([string]::IsNullOrWhiteSpace([string]$manifest.capability_id)) {
            $issues.Add("capability_id missing in $runRelative")
        }
        if ([string]::IsNullOrWhiteSpace([string]$manifest.status)) {
            $issues.Add("status missing in $runRelative")
        }
        if (-not $manifest.inputs_summary) {
            $issues.Add("inputs_summary missing in $runRelative")
        }
        if (-not $manifest.outputs) {
            $issues.Add("outputs missing in $runRelative")
        }
        if (-not $manifest.trace) {
            $issues.Add("trace missing in $runRelative")
        }

        $ownerScope = Parse-OwnerScope ([string]$manifest.owner_ref)
        if ($null -eq $ownerScope) {
            $issues.Add("owner_ref is not portable or uses unsupported owner kind in $runRelative")
        }

        $statusValue = [string]$manifest.status
        if ($statusValue -notin @('completed', 'failed')) {
            $issues.Add("invalid status '$statusValue' in $runRelative")
        }

        $isLegacyRun = ($segments.Count -eq 1 -and [string]$manifest.run_id -eq $segments[0])
        if ($isLegacyRun) {
            $legacyRunsExcluded++
            $warnings.Add("legacy non-owner-scoped run excluded from F10 gate: $runRelative")
            continue
        }

        if ($segments.Count -ne 3) {
            $issues.Add("run path must be owner-scoped under user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>: $runRelative")
            continue
        }

        $ownerScopedRunsChecked++
        $expectedRunId = [string]$manifest.run_id
        if ($segments[2] -ne $expectedRunId) {
            $issues.Add("run_id does not match owner-scoped folder name in $runRelative")
        }

        if ($null -ne $ownerScope) {
            if ($segments[0] -ne $ownerScope.OwnerKindPath -or $segments[1] -ne $ownerScope.OwnerIdPath) {
                $issues.Add("owner-scoped path does not match owner_ref in $runRelative")
            }
        }

        $expectedOutputPrefix = "user/output/tool_runs/$($segments[0])/$($segments[1])/$($segments[2])/"
        if ($manifest.outputs -and $manifest.outputs.result_path) {
            $resultPathValue = [string]$manifest.outputs.result_path
            if ($resultPathValue -match '^[A-Za-z]:\\' -or $resultPathValue -match '^\\\\\?\\') {
                $issues.Add("absolute result_path detected in $runRelative")
            } elseif (-not $resultPathValue.StartsWith($expectedOutputPrefix)) {
                $issues.Add("result_path is not owner-scoped in manifest for $runRelative")
            }
        }
        if ($manifest.outputs -and $manifest.outputs.manifest_path) {
            $manifestPathValue = [string]$manifest.outputs.manifest_path
            if ($manifestPathValue -match '^[A-Za-z]:\\' -or $manifestPathValue -match '^\\\\\?\\') {
                $issues.Add("absolute manifest_path detected in $runRelative")
            } elseif (-not $manifestPathValue.StartsWith($expectedOutputPrefix)) {
                $issues.Add("manifest_path is not owner-scoped in manifest for $runRelative")
            }
        }

        if ($statusValue -eq 'completed') {
            $completedRunsChecked++
            $resultPath = Join-Path $runDir 'result.json'
            if (-not (Test-Path -LiteralPath $resultPath)) {
                $issues.Add("completed run missing result.json in $runRelative")
            }
            if ($manifest.errors -and @($manifest.errors).Count -gt 0) {
                $issues.Add("completed run contains errors in $runRelative")
            }
        }

        if ($statusValue -eq 'failed') {
            $failedRunsChecked++
            $resultPath = Join-Path $runDir 'result.json'
            if (Test-Path -LiteralPath $resultPath) {
                $issues.Add("failed run must not keep valid result.json in $runRelative")
            }

            $errorCount = if ($manifest.errors) { @($manifest.errors).Count } else { 0 }
            $warningCount = if ($manifest.warnings) { @($manifest.warnings).Count } else { 0 }
            if (($errorCount + $warningCount) -eq 0) {
                $issues.Add("failed run missing errors/warnings in $runRelative")
            }
        }
    }
}

if ($manifestsChecked -gt 0 -and $ownerScopedRunsChecked -eq 0) {
    $issues.Add('no owner-scoped persisted tool runs found')
}

$status = if ($issues.Count -eq 0) { 'PASS' } else { 'FAIL' }
$report = @()
$report += 'F9 Tool Run Output Validation'
$report += ''
$report += "tool_runs_dir=$toolRunsDir"
$report += "manifests_checked=$manifestsChecked"
$report += "owner_scoped_runs_checked=$ownerScopedRunsChecked"
$report += "completed_runs_checked=$completedRunsChecked"
$report += "failed_runs_checked=$failedRunsChecked"
$report += "legacy_runs_excluded=$legacyRunsExcluded"
$report += "status=$status"
$report += ''
if ($warnings.Count -gt 0) {
    $report += 'Warnings:'
    foreach ($warning in $warnings) {
        $report += "- $warning"
    }
    $report += ''
}
if ($issues.Count -eq 0) {
    $report += 'No issues detected.'
} else {
    $report += 'Issues:'
    foreach ($issue in $issues) {
        $report += "- $issue"
    }
}

Set-Content -LiteralPath $reportPath -Value $report -Encoding UTF8
Write-Host "manifests_checked=$manifestsChecked"
Write-Host "owner_scoped_runs_checked=$ownerScopedRunsChecked"
Write-Host "completed_runs_checked=$completedRunsChecked"
Write-Host "failed_runs_checked=$failedRunsChecked"
Write-Host "legacy_runs_excluded=$legacyRunsExcluded"
Write-Host "status=$status"

if ($status -eq 'FAIL') { exit 1 }
exit 0
