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
$reportPath = Join-Path $outputDir 'audit_f10_llm_execution_guard_codex.json'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$dangerPatterns = @(
    'invoke_llm',
    'call_llm',
    'invoke_openai',
    'run_sparql',
    'embeddings',
    'oxigraph',
    'semantic_store',
    'provider invocation'
)

$guardTerms = @(
    'no ',
    'not ',
    'disabled',
    'false',
    'must not',
    'does not',
    'not implemented',
    'future',
    'proposal',
    'declarative',
    'mock',
    'without'
)

$results = New-Object System.Collections.Generic.List[object]
$readErrors = New-Object System.Collections.Generic.List[object]

$files = Get-ChildItem -LiteralPath $rootDir -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
    $_.FullName -notmatch '\\target\\' -and
    $_.FullName -notmatch '\\user\\output\\' -and
    $_.FullName -notmatch '\\.git\\' -and
    $_.FullName -notmatch '\\dev\\scripts\\audits\\audit_f10_llm_execution_guard_codex\.bat$'
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

    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = [string]$lines[$i]
        $lower = $line.ToLowerInvariant()

        foreach ($pattern in $dangerPatterns) {
            if ($lower.Contains($pattern)) {
                $start = [Math]::Max(0, $i - 3)
                $end = [Math]::Min($lines.Count - 1, $i + 3)
                $window = (($lines[$start..$end]) -join ' ').ToLowerInvariant()

                $classification = 'REVIEW_REQUIRED'
                $action = 'review'

                if ($relative -match '^crates/llm_cloud/') {
                    $classification = 'REAL_EXECUTION_SURFACE'
                    $action = 'review_only'
                }
                elseif ($relative -match '\.json$') {
                    $classification = 'DECLARATIVE_CAPABILITY'
                    $action = 'ignore'
                }
                else {
                    foreach ($guard in $guardTerms) {
                        if ($window.Contains($guard)) {
                            $classification = 'GUARDED_DECLARATION'
                            $action = 'ignore'
                            break
                        }
                    }
                }

                $results.Add([pscustomobject]@{
                    file = $relative
                    line = $i + 1
                    pattern = $pattern
                    classification = $classification
                    action = $action
                    snippet = $line.Trim()
                })
            }
        }
    }
}

$summary = [ordered]@{
    files_checked = $filesChecked
    read_errors = $readErrors.Count
    total = $results.Count
    real_execution_surface = ($results | Where-Object { $_.classification -eq 'REAL_EXECUTION_SURFACE' }).Count
    declarative_capability = ($results | Where-Object { $_.classification -eq 'DECLARATIVE_CAPABILITY' }).Count
    guarded_declaration = ($results | Where-Object { $_.classification -eq 'GUARDED_DECLARATION' }).Count
    review_required = ($results | Where-Object { $_.classification -eq 'REVIEW_REQUIRED' }).Count
}

$status = 'PASS'
if ($summary.real_execution_surface -gt 0 -or $summary.review_required -gt 0) {
    $status = 'REVIEW'
}

$output = [ordered]@{
    audit = 'audit_f10_llm_execution_guard_codex'
    generated_at = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
    workspace_root = $rootDir
    status = $status
    summary = $summary
    codex_rules = @(
        'Do not modify DECLARATIVE_CAPABILITY entries.',
        'Do not modify GUARDED_DECLARATION entries.',
        'Do not modify REAL_EXECUTION_SURFACE entries without explicit human approval.',
        'For REVIEW_REQUIRED entries, classify first; propose minimal documentation wording fixes only when the entry is not a generated report, forbidden action id, or controlled validator check.',
        'Do not touch project_runtime.',
        'Do not touch UI logic.',
        'Do not rename forbidden action ids or declarative capability ids.',
        'Do not delete catalog entries only because they mention LLM, embeddings, Oxigraph, SPARQL, or semantic store.'
    )
    codex_actions = [ordered]@{
        REAL_EXECUTION_SURFACE = 'never_modify_without_human_approval'
        DECLARATIVE_CAPABILITY = 'ignore'
        GUARDED_DECLARATION = 'ignore'
        REVIEW_REQUIRED = 'analyze_and_propose_minimal_fix'
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

Write-Host '=== CODEX READY AUDIT ==='
Write-Host ('status=' + $status)
Write-Host ('files_checked=' + $summary.files_checked)
Write-Host ('total=' + $summary.total)
Write-Host ('real_execution_surface=' + $summary.real_execution_surface)
Write-Host ('declarative_capability=' + $summary.declarative_capability)
Write-Host ('guarded_declaration=' + $summary.guarded_declaration)
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