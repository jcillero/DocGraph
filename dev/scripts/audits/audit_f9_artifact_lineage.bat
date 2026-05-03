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
$reportPath = Join-Path $outputDir 'audit_f9_artifact_lineage_codex.json'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$lineageTerms = @(
    'source',
    'derived',
    'artifact',
    'document.md'
)

$mutationTerms = @(
    'edit',
    'editable',
    'mutate',
    'mutation',
    'modify',
    'modified',
    'write',
    'save',
    'delete',
    'rename',
    'patch',
    'apply'
)

$readonlyGuards = @(
    'readonly',
    'read-only',
    'must not be mutated',
    'must not mutate',
    'must not edit',
    'must not write',
    'not mutable',
    'no mutation',
    'no write-back',
    'forbidden',
    'blocked',
    'does not mutate',
    'without mutation',
    'never'
)

$derivedGuards = @(
    'regenerable',
    'derived',
    'secondary',
    'not source of truth',
    'readonly',
    'read-only'
)

$artifactAllowedGuards = @(
    'artifact',
    'document.md',
    'editable artifact',
    'markdown artifact',
    'governed patch',
    'accepted patch',
    'patch acceptance',
    'documentholder'
)

$declarativeGuards = @(
    'proposal',
    'future',
    'conceptual',
    'declarative',
    'example',
    'schema',
    'contract',
    'spec',
    'invariant'
)

$negativeGuards = @(
    'no ',
    'must not',
    'never',
    'without',
    'not ',
    'does not'
)

$results = New-Object System.Collections.Generic.List[object]
$readErrors = New-Object System.Collections.Generic.List[object]

$files = Get-ChildItem -LiteralPath $rootDir -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
    $_.FullName -notmatch '\\target\\' -and
    $_.FullName -notmatch '\\user\\output\\' -and
    $_.FullName -notmatch '\\.git\\' -and
    $_.FullName -notmatch '\\dev\\scripts\\audits\\audit_f9_artifact_lineage_codex\.bat$'
}

$filesChecked = 0

foreach ($fileInfo in $files) {
    $relative = $fileInfo.FullName.Substring($rootDir.Length + 1).Replace('\', '/')

    try {
        $lines = Get-Content -LiteralPath $fileInfo.FullName -ErrorAction Stop
    } catch {
        $readErrors.Add([pscustomobject]@{
            file = $relative
            error = $_.Exception.Message
        })
        continue
    }

    if ($null -eq $lines -or $lines.Count -eq 0) {
        continue
    }

    $filesChecked++

    $insideTestBlock = $false

    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = [string]$lines[$i]
        $lower = $line.ToLowerInvariant()

        if ($lower -match '#\[test\]' -or $lower -match 'fn .*test' -or $lower -match 'mod tests') {
            $insideTestBlock = $true
        }

        $looksLikeErrorMessage = (
            $lower.Contains('write!(') -or
            $lower.Contains('format!(') -or
            $lower.Contains('f.write_str(') -or
            $lower.Contains('message')
        )

        $mentionsLineage = $false
        foreach ($term in $lineageTerms) {
            if ($lower.Contains($term)) {
                $mentionsLineage = $true
                break
            }
        }

        if (-not $mentionsLineage) {
            continue
        }

        $mentionsMutation = $false
        foreach ($term in $mutationTerms) {
            if ($lower.Contains($term)) {
                $mentionsMutation = $true
                break
            }
        }

        if (-not $mentionsMutation) {
            continue
        }

        $start = [Math]::Max(0, $i - 5)
        $end = [Math]::Min($lines.Count - 1, $i + 5)
        $window = (($lines[$start..$end]) -join ' ').ToLowerInvariant()

        $mentionsSource = $lower -match '\bsource\b'
        $mentionsDerived = $lower -match '\bderived\b'
        $mentionsArtifact = $lower -match '\bartifact\b' -or $lower.Contains('document.md')

        $hasReadonlyGuard = $false
        foreach ($guard in $readonlyGuards) {
            if ($window.Contains($guard)) {
                $hasReadonlyGuard = $true
                break
            }
        }

        $hasDerivedGuard = $false
        foreach ($guard in $derivedGuards) {
            if ($window.Contains($guard)) {
                $hasDerivedGuard = $true
                break
            }
        }

        $hasArtifactAllowedGuard = $false
        foreach ($guard in $artifactAllowedGuards) {
            if ($window.Contains($guard)) {
                $hasArtifactAllowedGuard = $true
                break
            }
        }

        $hasDeclarativeGuard = $false
        foreach ($guard in $declarativeGuards) {
            if ($window.Contains($guard) -or $relative -match '\.json$') {
                $hasDeclarativeGuard = $true
                break
            }
        }

        $hasNegativeGuard = $false
        foreach ($guard in $negativeGuards) {
            if ($window.Contains($guard)) {
                $hasNegativeGuard = $true
                break
            }
        }

        $classification = 'REVIEW_REQUIRED'
        $action = 'review'

        if ($insideTestBlock -or $relative -match '/tests/' -or $relative -match '_test' -or $looksLikeErrorMessage) {
            $classification = 'TEST_OR_ERROR_MESSAGE'
            $action = 'ignore'
        }
        elseif ($hasNegativeGuard -or (($mentionsSource -or $mentionsDerived) -and $hasReadonlyGuard)) {
            $classification = 'GUARDED_DECLARATION'
            $action = 'ignore'
        }
        elseif ($mentionsDerived -and $hasDerivedGuard) {
            $classification = 'GUARDED_DERIVED_CONTEXT'
            $action = 'ignore'
        }
        elseif ($mentionsArtifact -and $hasArtifactAllowedGuard -and -not ($mentionsSource -or $mentionsDerived)) {
            $classification = 'ARTIFACT_EDITABLE_ALLOWED'
            $action = 'ignore'
        }
        elseif ($hasDeclarativeGuard) {
            $classification = 'DECLARATIVE_CONTEXT'
            $action = 'ignore'
        }
        elseif (($mentionsSource -or $mentionsDerived) -and $mentionsMutation -and -not $hasReadonlyGuard) {
            $classification = 'REAL_VIOLATION'
            $action = 'propose_minimal_fix'
        }

        $results.Add([pscustomobject]@{
            file = $relative
            line = $i + 1
            classification = $classification
            action = $action
            mentions_source = $mentionsSource
            mentions_derived = $mentionsDerived
            mentions_artifact = $mentionsArtifact
            snippet = $line.Trim()
        })
    }
}

$summary = [ordered]@{
    files_checked = $filesChecked
    read_errors = $readErrors.Count
    total = $results.Count
    real_violation = ($results | Where-Object { $_.classification -eq 'REAL_VIOLATION' }).Count
    review_required = ($results | Where-Object { $_.classification -eq 'REVIEW_REQUIRED' }).Count
    guarded_declaration = ($results | Where-Object { $_.classification -eq 'GUARDED_DECLARATION' }).Count
    guarded_derived_context = ($results | Where-Object { $_.classification -eq 'GUARDED_DERIVED_CONTEXT' }).Count
    artifact_editable_allowed = ($results | Where-Object { $_.classification -eq 'ARTIFACT_EDITABLE_ALLOWED' }).Count
    declarative_context = ($results | Where-Object { $_.classification -eq 'DECLARATIVE_CONTEXT' }).Count
    test_or_error_message = ($results | Where-Object { $_.classification -eq 'TEST_OR_ERROR_MESSAGE' }).Count
}

$status = 'PASS'
if ($summary.real_violation -gt 0 -or $summary.review_required -gt 0) {
    $status = 'REVIEW'
}

$output = [ordered]@{
    audit = 'audit_f9_artifact_lineage_codex'
    generated_at = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    workspace_root = $rootDir
    status = $status
    summary = $summary
    codex_rules = @(
        'Do not modify GUARDED_DECLARATION entries.',
        'Do not modify GUARDED_DERIVED_CONTEXT entries.',
        'Do not modify ARTIFACT_EDITABLE_ALLOWED entries.',
        'Do not modify DECLARATIVE_CONTEXT entries.',
        'Do not modify TEST_OR_ERROR_MESSAGE entries.',
        'For REAL_VIOLATION entries, propose minimal wording fixes only.',
        'For REVIEW_REQUIRED entries, classify first; do not edit until classification is clear.',
        'SOURCE must remain readonly.',
        'DERIVED must remain readonly and regenerable.',
        'ARTIFACT may be editable only through governed patch or accepted mutation flow.',
        'document.md is allowed to be editable only when treated as ARTIFACT, not SOURCE.',
        'Do not touch project_runtime.',
        'Do not touch UI logic.'
    )
    codex_actions = [ordered]@{
        REAL_VIOLATION = 'propose_minimal_documentation_fix'
        REVIEW_REQUIRED = 'analyze_before_edit'
        GUARDED_DECLARATION = 'ignore'
        GUARDED_DERIVED_CONTEXT = 'ignore'
        ARTIFACT_EDITABLE_ALLOWED = 'ignore'
        DECLARATIVE_CONTEXT = 'ignore'
        TEST_OR_ERROR_MESSAGE = 'ignore'
    }
    findings = $results
    read_errors = $readErrors
}

try {
    $output | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $reportPath -Encoding UTF8 -ErrorAction Stop
} catch {
    Write-Host ('write_error=' + $_.Exception.Message)
    exit 1
}

Write-Host '=== F9 ARTIFACT LINEAGE CODEX AUDIT ==='
Write-Host ('status=' + $status)
Write-Host ('files_checked=' + $summary.files_checked)
Write-Host ('total=' + $summary.total)
Write-Host ('real_violation=' + $summary.real_violation)
Write-Host ('review_required=' + $summary.review_required)
Write-Host ('guarded_declaration=' + $summary.guarded_declaration)
Write-Host ('guarded_derived_context=' + $summary.guarded_derived_context)
Write-Host ('artifact_editable_allowed=' + $summary.artifact_editable_allowed)
Write-Host ('declarative_context=' + $summary.declarative_context)
Write-Host ('test_or_error_message=' + $summary.test_or_error_message)
Write-Host ('read_errors=' + $summary.read_errors)
Write-Host ('report_path=' + $reportPath)

if (Test-Path -LiteralPath $reportPath) {
    Write-Host 'report_written=true'
} else {
    Write-Host 'report_written=false'
    exit 1
}

exit 0