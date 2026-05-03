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
$reportPath = Join-Path $outputDir 'audit_graph_analysis_boundary_report.txt'

if (-not (Test-Path -LiteralPath (Join-Path $rootDir 'docs\specs'))) {
    throw 'Repository root could not be resolved safely.'
}

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

function Test-NegativeContext {
    param([string]$Line)
    $negativeRegex = '(?i)\b(must not|mustn''t|do not|does not|did not|should not|not allowed|forbidden|prohibited|blocked|disabled|closed|remains closed|advisory only|proposal only|declarative only|read-only|readonly|without|never|not implemented|no )'
    return ($Line -match $negativeRegex)
}

$categories = @(
    @{
        Id = 'A'
        Title = 'Runtime and engine drift'
        Rules = @(
            @{ Pattern = '(?i)\bgraph analysis\b.*\b(runtime|active|implemented)\b'; Severity = 'FAIL'; Message = 'Graph analysis may be treated as runtime.' }
            @{ Pattern = '(?i)\bcritical path( computation)?\b.*\b(implemented|computed|runtime)\b'; Severity = 'WARN'; Message = 'Critical path computation may be implemented too early.' }
            @{ Pattern = '(?i)\bgraph engine\b.*\b(enabled|active|implemented)?'; Severity = 'FAIL'; Message = 'Graph engine wording detected.' }
            @{ Pattern = '(?i)\binference (runtime|engine)\b.*\b(enabled|active|implemented)?'; Severity = 'FAIL'; Message = 'Inference runtime wording detected.' }
        )
    }
    @{
        Id = 'B'
        Title = 'Eligibility and authority drift'
        Rules = @(
            @{ Pattern = '(?i)\banalysis\b.*\bPROPOSED\b'; Severity = 'FAIL'; Message = 'Analysis may consume PROPOSED material.' }
            @{ Pattern = '(?i)\banalysis results?\b.*\b(authoritative facts|source of truth|approved knowledge)\b'; Severity = 'FAIL'; Message = 'Analysis results may be treated as authoritative facts.' }
            @{ Pattern = '(?i)\banalysis\b.*\b(modif(?:y|ies)|updates?)\b.*\b(lifecycle|approval|graph)\b'; Severity = 'FAIL'; Message = 'Analysis may modify lifecycle or graph.' }
            @{ Pattern = '(?i)\banalysis\b.*\b(approve|reject|supersede|stale|orphan)\b.*\b(quads?|relations?)\b'; Severity = 'FAIL'; Message = 'Analysis may be mutating semantic lifecycle states.' }
        )
    }
    @{
        Id = 'C'
        Title = 'Execution drift'
        Rules = @(
            @{ Pattern = '(?i)\banalysis\b.*\bActionResolution\b'; Severity = 'FAIL'; Message = 'Analysis may trigger ActionResolution.' }
            @{ Pattern = '(?i)\banalysis\b.*\btool execution\b'; Severity = 'FAIL'; Message = 'Analysis may trigger tool execution.' }
        )
    }
)

$scannedPaths = @(
    'docs/specs/graph_analysis_policy.md',
    'docs/specs/artifact_graph.md',
    'docs/specs/semantic_quad_model.md',
    'docs/specs/semantic_quad_lifecycle.md',
    'docs/specs/rdf_projection_policy.md',
    'resources/ai/schemas/graph_analysis.schema.json',
    'governance/GOVERNANCE.md',
    'governance/FUNCTIONAL_SCOPE.md'
)

$filesToScan = New-Object System.Collections.Generic.List[object]
foreach ($relative in $scannedPaths) {
    $full = Join-Path $rootDir $relative
    if (Test-Path -LiteralPath $full) {
        $filesToScan.Add([pscustomobject]@{ FullPath = $full; RelativePath = $relative })
    }
}

$findings = New-Object System.Collections.Generic.List[object]
foreach ($fileInfo in $filesToScan) {
    $lines = Get-Content -LiteralPath $fileInfo.FullPath
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = [string]$lines[$i]
        foreach ($category in $categories) {
            foreach ($rule in $category.Rules) {
                if ($line -match $rule.Pattern) {
                    if (Test-NegativeContext -Line $line) { continue }
                    $findings.Add([pscustomobject]@{
                        CategoryId = $category.Id
                        CategoryTitle = $category.Title
                        Severity = $rule.Severity
                        File = $fileInfo.RelativePath
                        Line = $i + 1
                        Message = $rule.Message
                        Snippet = $line.Trim()
                    })
                }
            }
        }
    }
}

$reportLines = New-Object System.Collections.Generic.List[string]
$reportLines.Add('Graph Analysis Boundary Audit Report')
$reportLines.Add('')
$reportLines.Add(('timestamp: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('repository_root: {0}' -f $rootDir))
$reportLines.Add('status: ADVISORY')
$reportLines.Add('')
$reportLines.Add('Limitations:')
$reportLines.Add('- textual scan only')
$reportLines.Add('- false positives expected')
$reportLines.Add('- analysis is future decision support only')
$reportLines.Add('- no execution or inference runtime is opened')
$reportLines.Add('- findings require human review')
$reportLines.Add('')
$reportLines.Add('Scanned paths:')
foreach ($path in $scannedPaths) { $reportLines.Add(('- {0}' -f $path)) }
$reportLines.Add(('scanned_file_count: {0}' -f $filesToScan.Count))
$reportLines.Add('')
$reportLines.Add('Category summaries:')
foreach ($category in $categories) {
    $categoryFindings = @($findings | Where-Object { $_.CategoryId -eq $category.Id })
    $failCount = @($categoryFindings | Where-Object { $_.Severity -eq 'FAIL' }).Count
    $warnCount = @($categoryFindings | Where-Object { $_.Severity -eq 'WARN' }).Count
    $infoCount = @($categoryFindings | Where-Object { $_.Severity -eq 'INFO' }).Count
    $reportLines.Add(('- [{0}] {1} :: FAIL={2} WARN={3} INFO={4}' -f $category.Id, $category.Title, $failCount, $warnCount, $infoCount))
}
$reportLines.Add('')
$reportLines.Add('Possible findings:')
foreach ($category in $categories) {
    $reportLines.Add('')
    $reportLines.Add(('[{0}] {1}' -f $category.Id, $category.Title))
    $reportLines.Add((''.PadLeft(($category.Title.Length + 5), '-')))
    $categoryFindings = @($findings | Where-Object { $_.CategoryId -eq $category.Id })
    if ($categoryFindings.Count -eq 0) { $reportLines.Add('No suspicious matches detected.'); continue }
    foreach ($finding in $categoryFindings | Sort-Object Severity, File, Line) {
        $reportLines.Add(('- {0} :: {1}:{2} :: {3}' -f $finding.Severity, $finding.File, $finding.Line, $finding.Message))
        $reportLines.Add(('  snippet: {0}' -f $finding.Snippet))
    }
}
$failTotal = @($findings | Where-Object { $_.Severity -eq 'FAIL' }).Count
$warnTotal = @($findings | Where-Object { $_.Severity -eq 'WARN' }).Count
$infoTotal = @($findings | Where-Object { $_.Severity -eq 'INFO' }).Count
$reportLines.Add('')
$reportLines.Add('Final summary:')
$reportLines.Add(('fail_count: {0}' -f $failTotal))
$reportLines.Add(('warn_count: {0}' -f $warnTotal))
$reportLines.Add(('info_count: {0}' -f $infoTotal))
$reportLines.Add('advisory_only: true')
$reportLines.Add('exit_code_policy: non-zero only on script failure')
Set-Content -LiteralPath $reportPath -Value $reportLines -Encoding UTF8
Write-Host '=== GRAPH ANALYSIS BOUNDARY AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'
exit 0
