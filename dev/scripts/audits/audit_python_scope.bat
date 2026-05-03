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
$rootDir = [IO.Path]::GetFullPath((Join-Path $scriptDir '..\..\..'))
$outputDir = Join-Path $rootDir 'user\output'
$reportPath = Join-Path $outputDir 'audit_python_scope_report.txt'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

# Policy:
# - PASS: Python appears only in allowed doctrinal/migration contexts.
# - WARN: Python appears in specs/reports as inherited doctrine or migration reference.
# - FAIL: Python appears in active implementation/runtime/tool areas, or in any dangerous runtime context.
#
# Intentional security/governance allowlist:
# These canonical governance and architecture documents are explicitly allowed
# because the script enforces where Python may still appear as doctrine or
# migration context. This is not path drift and should remain explicit.

$allowDocFiles = @(
    'README.md',
    'governance/GOVERNANCE.md',
    'architecture/MIGRATION_BASELINE.md',
    'architecture/KNOWLEDGE_TRANSFER_MAP.md',
    'governance/INHERITED_GOVERNANCE.md',
    'architecture/MODULE_MAPPING.md',
    'architecture/ARCHITECTURE.md',
    'governance/FUNCTIONAL_SCOPE.md',
    'governance/WORKSPACE_RULES.md',
    'governance/I18N_POLICY.md',
    'governance/UI_SLINT_POLICY.md',
    'governance/LLM_RUNTIME_POLICY.md'
)

$toolingAllowedFiles = @(
    'dev/scripts/SCRIPTS_INDEX.md',
    'dev/scripts/audits/audit_python_scope.bat'
)

$warnPathPrefixes = @(
    'docs/specs/',
    'docs/'
)

$warnFiles = @(
    'governance/reports/MASTER_GOVERNANCE_AND_SPECS_REPORT.md'
)

$failPathPrefixes = @(
    'crates/',
    'resources/runtime/',
    'resources/tools/',
    'resources/tool_runtime/'
)

$safePhrases = @(
    'not part of the active rust runtime',
    'not required to run',
    'not required to operate',
    'does not declare a runtime dependency',
    'doctrinal reference',
    'doctrinal and behavioral migration reference',
    'historical doctrinal',
    'historical',
    'migration reference',
    'migration baseline',
    'knowledge transfer',
    'baseline',
    'stable python behavior',
    'reference areas in python',
    'what still remains primarily in python',
    'inherited',
    'transferred',
    'transfer',
    'replace python',
    'not python helper names',
    'python source document',
    'python responsibility',
    'python source cues',
    'must not be invoked',
    'must not be embedded',
    'must not be required',
    'not the current execution path',
    'historical and doctrinal source material'
)

$dangerousPatterns = @(
    '(python.{0,60}\binvoke\b|\binvoke\b.{0,60}python)',
    '(python.{0,60}\bexecute\b|\bexecute\b.{0,60}python)',
    '(python.{0,60}\brun\b|\brun\b.{0,60}python)',
    '(python.{0,60}\bembedded\b|\bembedded\b.{0,60}python)',
    '(python.{0,60}\brequired\b|\brequired\b.{0,60}python)',
    '(python.{0,60}runtime dependency|runtime dependency.{0,60}python)',
    'install python',
    'download python',
    'python runtime',
    'python\.exe'
)

$negatedDangerPhrases = @(
    'not part of the active rust runtime',
    'not required to run',
    'not required to operate',
    'must not be invoked',
    'must not be embedded',
    'must not be required',
    'does not declare a runtime dependency',
    'not the current execution path'
)

$skipDirNames = @(
    '.git',
    'target'
)

$skipPathPrefixes = @(
    'user/output/'
)

function Test-StartsWithAny {
    param(
        [string]$Value,
        [string[]]$Prefixes
    )
    foreach ($prefix in $Prefixes) {
        if ($Value.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            return $true
        }
    }
    return $false
}

function Test-ContainsAnyPhrase {
    param(
        [string]$Value,
        [string[]]$Phrases
    )
    foreach ($phrase in $Phrases) {
        if ($Value.Contains($phrase)) {
            return $true
        }
    }
    return $false
}

function Get-Zone {
    param([string]$RelativePath)

    if ($toolingAllowedFiles -contains $RelativePath) {
        return 'tooling_allowed'
    }

    if ($allowDocFiles -contains $RelativePath) {
        return 'allowed_doctrinal_root'
    }

    if ($warnFiles -contains $RelativePath) {
        return 'warn_doc_or_report'
    }

    if (Test-StartsWithAny -Value $RelativePath -Prefixes $warnPathPrefixes) {
        return 'warn_doc_or_spec'
    }

    if (Test-StartsWithAny -Value $RelativePath -Prefixes $failPathPrefixes) {
        return 'fail_active_runtime_area'
    }

    if ($RelativePath.StartsWith('dev/scripts/', [System.StringComparison]::OrdinalIgnoreCase)) {
        return 'fail_script_area'
    }

    return 'unknown'
}

$allFiles = Get-ChildItem -LiteralPath $rootDir -Recurse -File | Where-Object {
    $relative = $_.FullName.Substring($rootDir.Length + 1).Replace('\', '/')

    foreach ($skipDir in $skipDirNames) {
        if ($relative -match "(^|/)$([regex]::Escape($skipDir))(/|$)") {
            return $false
        }
    }

    foreach ($skipPrefix in $skipPathPrefixes) {
        if ($relative.StartsWith($skipPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            return $false
        }
    }

    return $true
} | Sort-Object FullName

$filesScanned = 0
$pythonMentions = 0
$warnings = New-Object System.Collections.Generic.List[object]
$findings = New-Object System.Collections.Generic.List[object]

foreach ($fileInfo in $allFiles) {
    $relativePath = $fileInfo.FullName.Substring($rootDir.Length + 1).Replace('\', '/')

    try {
        $lines = Get-Content -LiteralPath $fileInfo.FullName -ErrorAction Stop
    } catch {
        continue
    }

    $filesScanned++
    $zone = Get-Zone -RelativePath $relativePath

    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = [string]$lines[$i]
        if ($null -eq $line) {
            continue
        }

        $currentLower = $line.ToLowerInvariant()
        if ($currentLower -notmatch '\bpython\b') {
            continue
        }

        $pythonMentions++

        if ($zone -eq 'tooling_allowed') {
            continue
        }

        $windowStart = [Math]::Max(0, $i - 4)
        $windowEnd = [Math]::Min($lines.Count - 1, $i + 4)
        $window = (($lines[$windowStart..$windowEnd]) -join ' ').ToLowerInvariant()

        $hasSafeContext = Test-ContainsAnyPhrase -Value $window -Phrases $safePhrases
        $hasNegatedDanger = Test-ContainsAnyPhrase -Value $window -Phrases $negatedDangerPhrases

        $hasDangerousContext = $false
        foreach ($danger in $dangerousPatterns) {
            if ($window -match $danger) {
                if (-not $hasNegatedDanger) {
                    $hasDangerousContext = $true
                    break
                }
            }
        }

        if ($hasDangerousContext) {
            $findings.Add([pscustomobject]@{
                File = $relativePath
                Line = $i + 1
                Category = 'dangerous_python_runtime_context'
                Zone = $zone
                Snippet = $line.Trim()
            })
            continue
        }

        switch ($zone) {
            'allowed_doctrinal_root' {
                if (-not $hasSafeContext) {
                    $warnings.Add([pscustomobject]@{
                        File = $relativePath
                        Line = $i + 1
                        Category = 'python_root_doc_missing_clear_doctrinal_context'
                        Zone = $zone
                        Snippet = $line.Trim()
                    })
                }
            }
            'warn_doc_or_spec' {
                $warnings.Add([pscustomobject]@{
                    File = $relativePath
                    Line = $i + 1
                    Category = 'python_in_spec_or_doc_review_context'
                    Zone = $zone
                    Snippet = $line.Trim()
                })
            }
            'warn_doc_or_report' {
                $warnings.Add([pscustomobject]@{
                    File = $relativePath
                    Line = $i + 1
                    Category = 'python_in_report_review_context'
                    Zone = $zone
                    Snippet = $line.Trim()
                })
            }
            'fail_active_runtime_area' {
                $findings.Add([pscustomobject]@{
                    File = $relativePath
                    Line = $i + 1
                    Category = 'python_in_active_runtime_area'
                    Zone = $zone
                    Snippet = $line.Trim()
                })
            }
            'fail_script_area' {
                $findings.Add([pscustomobject]@{
                    File = $relativePath
                    Line = $i + 1
                    Category = 'python_in_script_area'
                    Zone = $zone
                    Snippet = $line.Trim()
                })
            }
            default {
                if ($hasSafeContext) {
                    $warnings.Add([pscustomobject]@{
                        File = $relativePath
                        Line = $i + 1
                        Category = 'python_in_unclassified_file_with_safe_context'
                        Zone = $zone
                        Snippet = $line.Trim()
                    })
                } else {
                    $findings.Add([pscustomobject]@{
                        File = $relativePath
                        Line = $i + 1
                        Category = 'python_in_unclassified_file'
                        Zone = $zone
                        Snippet = $line.Trim()
                    })
                }
            }
        }
    }
}

$status = if ($findings.Count -eq 0) { 'PASS' } else { 'FAIL' }

$reportLines = New-Object System.Collections.Generic.List[string]
$reportLines.Add('Python Scope Audit Report')
$reportLines.Add('')
$reportLines.Add(('generated_at: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('workspace_root: {0}' -f $rootDir))
$reportLines.Add('')
$reportLines.Add('Policy:')
$reportLines.Add('- PASS: no failing Python runtime/implementation leaks detected.')
$reportLines.Add('- WARN: Python appears in docs/specs/reports and should remain doctrinal or historical only.')
$reportLines.Add('- FAIL: Python appears in active runtime/tool/code/script areas, or with dangerous runtime semantics.')
$reportLines.Add('')

if ($warnings.Count -gt 0) {
    $reportLines.Add('Warnings:')
    $reportLines.Add('')
    foreach ($warning in $warnings) {
        $reportLines.Add(('- {0}:{1} [{2}] zone={3}' -f $warning.File, $warning.Line, $warning.Category, $warning.Zone))
        $reportLines.Add(('  {0}' -f $warning.Snippet))
    }
    $reportLines.Add('')
}

if ($findings.Count -gt 0) {
    $reportLines.Add('Findings:')
    $reportLines.Add('')
    foreach ($finding in $findings) {
        $reportLines.Add(('- {0}:{1} [{2}] zone={3}' -f $finding.File, $finding.Line, $finding.Category, $finding.Zone))
        $reportLines.Add(('  {0}' -f $finding.Snippet))
    }
    $reportLines.Add('')
}

if ($warnings.Count -eq 0 -and $findings.Count -eq 0) {
    $reportLines.Add('Python mentions are confined to allowed doctrinal migration files and safe contexts.')
    $reportLines.Add('')
}

$reportLines.Add(('files_scanned={0}' -f $filesScanned))
$reportLines.Add(('python_mentions={0}' -f $pythonMentions))
$reportLines.Add(('warnings_count={0}' -f $warnings.Count))
$reportLines.Add(('findings_count={0}' -f $findings.Count))
$reportLines.Add(('status={0}' -f $status))

Set-Content -LiteralPath $reportPath -Value $reportLines -Encoding UTF8

Write-Host ('files_scanned={0}' -f $filesScanned)
Write-Host ('python_mentions={0}' -f $pythonMentions)
Write-Host ('warnings_count={0}' -f $warnings.Count)
Write-Host ('findings_count={0}' -f $findings.Count)
Write-Host ('status={0}' -f $status)

if ($status -eq 'FAIL') {
    exit 1
}

exit 0
