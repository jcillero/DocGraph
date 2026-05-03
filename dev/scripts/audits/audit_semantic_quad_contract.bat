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
$reportPath = Join-Path $outputDir 'audit_semantic_quad_contract_report.txt'

if (-not (Test-Path -LiteralPath (Join-Path $rootDir 'docs\specs'))) {
    throw 'Repository root could not be resolved safely.'
}

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

function Get-RelativePath {
    param([string]$FullPath)
    if ($FullPath.StartsWith($rootDir, [System.StringComparison]::OrdinalIgnoreCase)) {
        return $FullPath.Substring($rootDir.Length + 1).Replace('\', '/')
    }
    return $FullPath
}

function Test-NegativeContext {
    param([string]$Line)
    $negativeRegex = '(?i)\b(must not|mustn''t|do not|does not|did not|should not|not allowed|forbidden|prohibited|blocked|disabled|closed|remains closed|advisory only|proposal only|declarative only|read-only|readonly|without|never|not implemented|no )'
    return ($Line -match $negativeRegex)
}

$categories = @(
    @{
        Id = 'A'
        Title = 'Approval and fact drift'
        Rules = @(
            @{ Pattern = '(?i)\bPROPOSED\b.*\bAPPROVED\b'; Severity = 'WARN'; Message = 'Possible PROPOSED/APPROVED conflation detected.' }
            @{ Pattern = '(?i)\bsemantic quads?\b.*\b(facts|authoritative knowledge|approved knowledge)\b'; Severity = 'FAIL'; Message = 'Semantic quads may be treated as facts.' }
            @{ Pattern = '(?i)\bgeneration metadata\b.*\b(authority|approval|permission)\b'; Severity = 'FAIL'; Message = 'Generation metadata may be treated as authority.' }
        )
    }
    @{
        Id = 'B'
        Title = 'Storage and container drift'
        Rules = @(
            @{ Pattern = '(?i)\bquads?\b.*\binside derivatives\b'; Severity = 'FAIL'; Message = 'Quads may be stored inside derivatives.' }
            @{ Pattern = '(?i)\bquads?\b.*\binside chunks\b.*\bfacts\b'; Severity = 'FAIL'; Message = 'Quads may be embedded inside chunks as facts.' }
            @{ Pattern = '(?i)\bQuadRelation\b.*\b(embedded|child field|hierarchy)\b.*\bSemanticQuad\b'; Severity = 'FAIL'; Message = 'QuadRelation may be embedded as hierarchy inside SemanticQuad.' }
        )
    }
    @{
        Id = 'C'
        Title = 'Identity and lifecycle drift'
        Rules = @(
            @{ Pattern = '(?i)\bquad_id\b.*\bquad_instance_id\b.*\b(same|equivalent|identical)\b'; Severity = 'FAIL'; Message = 'quad_id and quad_instance_id may be conflated.' }
            @{ Pattern = '(?i)\blifecycle\b.*\bonly\b.*\bquad_id\b'; Severity = 'FAIL'; Message = 'Lifecycle may be attached only to quad_id.' }
            @{ Pattern = '(?i)\bsource_quad_instance_id\b.*\bquad_id only\b'; Severity = 'FAIL'; Message = 'Relation source/target may use only quad_id.' }
            @{ Pattern = '(?i)\brelation\b.*\bquad_id\b.*\binstead of\b.*\bQuadInstance\b'; Severity = 'FAIL'; Message = 'Relation source/target may use only quad_id instead of QuadInstance.' }
            @{ Pattern = '(?i)\bgraph\b.*\b(approves|approving|approval of)\b.*\bfacts\b'; Severity = 'FAIL'; Message = 'Graph may be treated as approving facts.' }
        )
    }
)

$scannedPaths = @(
    'docs/specs/semantic_quad_model.md',
    'docs/specs/semantic_quad_lifecycle.md',
    'docs/specs/semantic_source_scope.md',
    'docs/specs/artifact_graph.md',
    'resources/ai/schemas/semantic_quad.schema.json',
    'resources/ai/schemas/quad_instance.schema.json',
    'resources/ai/schemas/quad_lifecycle.schema.json',
    'resources/ai/schemas/quad_relation.schema.json',
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
                    if (Test-NegativeContext -Line $line) {
                        continue
                    }
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
$reportLines.Add('Semantic Quad Contract Audit Report')
$reportLines.Add('')
$reportLines.Add(('timestamp: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('repository_root: {0}' -f $rootDir))
$reportLines.Add('status: ADVISORY')
$reportLines.Add('')
$reportLines.Add('Limitations:')
$reportLines.Add('- advisory only')
$reportLines.Add('- textual matches may be false positives')
$reportLines.Add('- human review required')
$reportLines.Add('- not suitable for automatic patching')
$reportLines.Add('')
$reportLines.Add('Scanned paths:')
foreach ($path in $scannedPaths) {
    $reportLines.Add(('- {0}' -f $path))
}
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
    if ($categoryFindings.Count -eq 0) {
        $reportLines.Add('No suspicious matches detected.')
        continue
    }
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
Write-Host '=== SEMANTIC QUAD CONTRACT AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'
exit 0
