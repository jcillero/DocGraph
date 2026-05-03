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
$reportPath = Join-Path $outputDir 'audit_f9_output_ownership_codex.json'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$ownershipTerms = @(
    'tool_outputs',
    'tool_run_manifest',
    'owner_ref',
    'owner',
    'documentholder',
    'artifact',
    'chat',
    'knowledge'
)

$missingOwnerPatterns = @(
    'missing owner',
    'no owner',
    'without owner',
    'owner_ref missing',
    'owner not set',
    'without `owner_ref`',
    'without owner_ref',
    'has no `owner_ref`',
    'has no owner_ref'
)

$guardTerms = @(
    'must have owner',
    'must include owner_ref',
    'must declare `owner_ref`',
    'must declare owner_ref',
    'required owner_ref',
    'requires `owner_ref`',
    'requires owner_ref',
    'enforced ownership',
    'validated owner',
    'owner_ref is mandatory',
    'every persisted tool output has `owner_ref`',
    'every tool output must have an `owner_ref`',
    'tool output without `owner_ref` is invalid',
    'invalid or pending clarification',
    'must live under the functional owner',
    'must be scoped',
    'must carry an `owner_ref`',
    'must carry owner_ref'
)

$layoutTerms = @(
    'folder',
    'structure',
    'layout',
    'path',
    'directory',
    'tree',
    'schema',
    'contract',
    'dirname',
    'filename',
    'root',
    'jsonl',
    '.json'
)

$declarativeTerms = @(
    'proposal',
    'future',
    'conceptual',
    'example',
    'spec',
    'contract',
    'invariant',
    'declarative',
    'documents ',
    'defines ',
    'not opened',
    'not implemented'
)

$negativeTerms = @(
    'no ',
    'must not',
    'never',
    'without',
    'not ',
    'does not',
    'do not',
    'is not',
    'are not'
)

$assetExtensions = @(
    '.png',
    '.jpg',
    '.jpeg',
    '.webp',
    '.ico',
    '.gif',
    '.bmp',
    '.pdf',
    '.zip',
    '.exe',
    '.dll'
)

$results = New-Object System.Collections.Generic.List[object]
$readErrors = New-Object System.Collections.Generic.List[object]

$files = Get-ChildItem -LiteralPath $rootDir -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
    $_.FullName -notmatch '\\target\\' -and
    $_.FullName -notmatch '\\user\\output\\' -and
    $_.FullName -notmatch '\\.git\\' -and
    $_.FullName -notmatch '\\dev\\scripts\\audits\\audit_f9_output_ownership_codex\.bat$'
}

$filesChecked = 0

foreach ($fileInfo in $files) {
    $relative = $fileInfo.FullName.Substring($rootDir.Length + 1).Replace('\', '/')
    $extension = [IO.Path]::GetExtension($fileInfo.FullName).ToLowerInvariant()

    $isBinaryOrAsset = $false
    foreach ($assetExtension in $assetExtensions) {
        if ($extension -eq $assetExtension) {
            $isBinaryOrAsset = $true
            break
        }
    }

    if ($isBinaryOrAsset) {
        continue
    }

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

        $mentionsOwnership = $false
        foreach ($term in $ownershipTerms) {
            if ($lower.Contains($term)) {
                $mentionsOwnership = $true
                break
            }
        }

        if (-not $mentionsOwnership) {
            continue
        }

        $start = [Math]::Max(0, $i - 5)
        $end = [Math]::Min($lines.Count - 1, $i + 5)
        $window = (($lines[$start..$end]) -join ' ').ToLowerInvariant()

        $hasGuard = $false
        foreach ($g in $guardTerms) {
            if ($window.Contains($g)) {
                $hasGuard = $true
                break
            }
        }

        $hasNegative = $false
        foreach ($n in $negativeTerms) {
            if ($window.Contains($n)) {
                $hasNegative = $true
                break
            }
        }

        $isLayout = $false
        foreach ($l in $layoutTerms) {
            if ($window.Contains($l)) {
                $isLayout = $true
                break
            }
        }

        $isDeclarative = $false
        foreach ($d in $declarativeTerms) {
            if ($window.Contains($d)) {
                $isDeclarative = $true
                break
            }
        }

        if ($relative -match '^docs/' -or $relative -match '^resources/' -or $relative -match '\.md$') {
            if ($isLayout -or $isDeclarative) {
                $isDeclarative = $true
            }
        }

        $missingOwner = $false
        foreach ($m in $missingOwnerPatterns) {
            if ($window.Contains($m)) {
                $missingOwner = $true
                break
            }
        }

        $looksLikeErrorMessage = (
            $lower.Contains('write!(') -or
            $lower.Contains('format!(') -or
            $lower.Contains('f.write_str(') -or
            $lower.Contains('message') -or
            $lower.Contains('err(') -or
            $lower.Contains('error')
        )

        $classification = 'REVIEW_REQUIRED'
        $action = 'review'

        if ($insideTestBlock -or $relative -match '/tests/' -or $relative -match '_test' -or $looksLikeErrorMessage) {
            $classification = 'TEST_OR_ERROR_MESSAGE'
            $action = 'ignore'
        }
        elseif ($missingOwner -and -not $hasGuard -and -not $hasNegative) {
            $classification = 'MISSING_OWNER_REF'
            $action = 'propose_minimal_fix'
        }
        elseif ($hasGuard -or $hasNegative) {
            $classification = 'GUARDED_DECLARATION'
            $action = 'ignore'
        }
        elseif ($isLayout) {
            $classification = 'LAYOUT_DECLARATION'
            $action = 'ignore'
        }
        elseif ($isDeclarative) {
            $classification = 'DECLARATIVE_CONTEXT'
            $action = 'ignore'
        }
        elseif ($mentionsOwnership -and -not $missingOwner -and -not $hasGuard) {
            $classification = 'OWNERSHIP_PRESENT_NO_GUARD'
            $action = 'review_light'
        }

        $results.Add([pscustomobject]@{
            file = $relative
            line = $i + 1
            classification = $classification
            action = $action
            snippet = $line.Trim()
        })
    }
}

$summary = [ordered]@{
    files_checked = $filesChecked
    read_errors = $readErrors.Count
    total = $results.Count
    missing_owner_ref = ($results | Where-Object { $_.classification -eq 'MISSING_OWNER_REF' }).Count
    guarded_declaration = ($results | Where-Object { $_.classification -eq 'GUARDED_DECLARATION' }).Count
    layout_declaration = ($results | Where-Object { $_.classification -eq 'LAYOUT_DECLARATION' }).Count
    declarative_context = ($results | Where-Object { $_.classification -eq 'DECLARATIVE_CONTEXT' }).Count
    ownership_present_no_guard = ($results | Where-Object { $_.classification -eq 'OWNERSHIP_PRESENT_NO_GUARD' }).Count
    test_or_error_message = ($results | Where-Object { $_.classification -eq 'TEST_OR_ERROR_MESSAGE' }).Count
    review_required = ($results | Where-Object { $_.classification -eq 'REVIEW_REQUIRED' }).Count
}

$status = 'PASS'
if ($summary.missing_owner_ref -gt 0 -or $summary.review_required -gt 0) {
    $status = 'REVIEW'
}

$output = [ordered]@{
    audit = 'audit_f9_output_ownership_codex'
    generated_at = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    workspace_root = $rootDir
    status = $status
    summary = $summary
    codex_rules = @(
        'Do not modify GUARDED_DECLARATION entries.',
        'Do not modify LAYOUT_DECLARATION entries.',
        'Do not modify DECLARATIVE_CONTEXT entries.',
        'Do not modify TEST_OR_ERROR_MESSAGE entries.',
        'Do not modify assets or binary files.',
        'Only fix MISSING_OWNER_REF with minimal documentation changes.',
        'For OWNERSHIP_PRESENT_NO_GUARD, review lightly and only add owner_ref wording if the text describes persisted outputs.',
        'For REVIEW_REQUIRED, classify first before editing.',
        'Do not touch project_runtime.',
        'Do not touch UI.',
        'Do not create root outputs/ as fallback.',
        'Persisted outputs must be owned by chat, DocumentHolder, or knowledge scope.'
    )
    codex_actions = [ordered]@{
        MISSING_OWNER_REF = 'propose_minimal_documentation_fix'
        OWNERSHIP_PRESENT_NO_GUARD = 'review_light'
        GUARDED_DECLARATION = 'ignore'
        LAYOUT_DECLARATION = 'ignore'
        DECLARATIVE_CONTEXT = 'ignore'
        TEST_OR_ERROR_MESSAGE = 'ignore'
        REVIEW_REQUIRED = 'analyze_before_edit'
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

Write-Host '=== F9 OUTPUT OWNERSHIP CODEX AUDIT ==='
Write-Host ('status=' + $status)
Write-Host ('files_checked=' + $summary.files_checked)
Write-Host ('total=' + $summary.total)
Write-Host ('missing_owner_ref=' + $summary.missing_owner_ref)
Write-Host ('guarded_declaration=' + $summary.guarded_declaration)
Write-Host ('layout_declaration=' + $summary.layout_declaration)
Write-Host ('declarative_context=' + $summary.declarative_context)
Write-Host ('ownership_present_no_guard=' + $summary.ownership_present_no_guard)
Write-Host ('test_or_error_message=' + $summary.test_or_error_message)
Write-Host ('review_required=' + $summary.review_required)
Write-Host ('read_errors=' + $summary.read_errors)
Write-Host ('report_path=' + $reportPath)

if (Test-Path -LiteralPath $reportPath) {
    Write-Host 'report_written=true'
} else {
    Write-Host 'report_written=false'
    exit 1
}

exit 0