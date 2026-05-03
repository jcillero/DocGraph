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
$reportPath = Join-Path $outputDir 'audit_rdf_projection_boundary_report.txt'

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
        Title = 'RDF authority drift'
        Rules = @(
            @{ Pattern = '(?i)\bRDF\b.*\b(authority|source of truth|replaces internal semantic model)\b'; Severity = 'FAIL'; Message = 'RDF may be treated as authority.' }
            @{ Pattern = '(?i)\bRDF\b.*\breplac(?:e|es|ing)\b.*\b(StoredObject|SemanticQuad|QuadInstance|QuadRelation)\b'; Severity = 'FAIL'; Message = 'RDF may replace the internal semantic model.' }
            @{ Pattern = '(?i)\bRDF\b.*\breplac(?:e|es|ing)\b.*\bproject_manifest\b'; Severity = 'FAIL'; Message = 'RDF may replace project_manifest.' }
            @{ Pattern = '(?i)\bRDF projection\b.*\b(approval|approves?)\b'; Severity = 'FAIL'; Message = 'RDF projection may be treated as approval.' }
        )
    }
    @{
        Id = 'B'
        Title = 'RDF runtime drift'
        Rules = @(
            @{ Pattern = '(?i)\bOxigraph\b.*\b(enabled|active runtime|activated)\b'; Severity = 'FAIL'; Message = 'Oxigraph may be enabled as active runtime.' }
            @{ Pattern = '(?i)\bSPARQL\b.*\b(enabled|active|endpoint|runtime)\b'; Severity = 'FAIL'; Message = 'SPARQL may be enabled.' }
            @{ Pattern = '(?i)\bN-Quads?\b.*\b(active persistence|persisted|generated now)\b'; Severity = 'FAIL'; Message = 'N-Quads may be treated as active persistence.' }
            @{ Pattern = '(?i)\bRDF projection\b.*\bgraph analysis runtime\b'; Severity = 'FAIL'; Message = 'RDF projection may be treated as graph analysis runtime.' }
        )
    }
    @{
        Id = 'C'
        Title = 'Lifecycle filtering drift'
        Rules = @(
            @{ Pattern = '(?i)\bPROPOSED\b.*\bRDF\b.*\b(fact|factual|projected)\b'; Severity = 'FAIL'; Message = 'PROPOSED material may be projected as factual RDF.' }
            @{ Pattern = '(?i)\blifecycle filtering\b.*\b(bypass|ignored|not required)\b'; Severity = 'FAIL'; Message = 'Lifecycle filtering may be bypassed.' }
        )
    }
)

$scannedPaths = @(
    'docs/specs/rdf_projection_policy.md',
    'docs/specs/semantic_quad_model.md',
    'docs/specs/semantic_quad_lifecycle.md',
    'docs/specs/artifact_graph.md',
    'docs/specs/storage_policy.md',
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
$reportLines.Add('RDF Projection Boundary Audit Report')
$reportLines.Add('')
$reportLines.Add(('timestamp: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('repository_root: {0}' -f $rootDir))
$reportLines.Add('status: ADVISORY')
$reportLines.Add('')
$reportLines.Add('Limitations:')
$reportLines.Add('- textual scan only')
$reportLines.Add('- false positives expected')
$reportLines.Add('- RDF remains future projection only')
$reportLines.Add('- Oxigraph/SPARQL remain inactive')
$reportLines.Add('- human review required')
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
Write-Host '=== RDF PROJECTION BOUNDARY AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'
exit 0
