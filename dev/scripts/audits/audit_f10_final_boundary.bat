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
$reportPath = Join-Path $outputDir 'audit_f10_final_boundary_report.txt'

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
        Title = 'Execution drift'
        Rules = @(
            @{ Pattern = '(?i)\bexecute tool\b'; Severity = 'FAIL'; Message = 'Direct tool execution wording detected.' }
            @{ Pattern = '(?i)\brun tool\b'; Severity = 'WARN'; Message = 'Run-tool wording detected.' }
            @{ Pattern = '(?i)\bdispatch\b'; Severity = 'WARN'; Message = 'Dispatch wording detected.' }
            @{ Pattern = '(?i)\bexecutor\b'; Severity = 'WARN'; Message = 'Executor wording detected.' }
            @{ Pattern = '(?i)\brunner\b'; Severity = 'WARN'; Message = 'Runner wording detected.' }
            @{ Pattern = '(?i)\bauto-?run\b'; Severity = 'FAIL'; Message = 'Auto-run wording detected.' }
        )
    }
    @{
        Id = 'B'
        Title = 'LLM authority drift'
        Rules = @(
            @{ Pattern = '(?i)\bLLM executes\b'; Severity = 'FAIL'; Message = 'LLM may be described as executing.' }
            @{ Pattern = '(?i)\bLLM calls tool\b'; Severity = 'FAIL'; Message = 'LLM may be described as directly calling tools.' }
            @{ Pattern = '(?i)\bautomatic tool execution\b'; Severity = 'FAIL'; Message = 'Automatic tool execution wording detected.' }
        )
    }
    @{
        Id = 'C'
        Title = 'UI authority drift'
        Rules = @(
            @{ Pattern = '(?i)\bUI executes\b'; Severity = 'FAIL'; Message = 'UI may be described as executing.' }
            @{ Pattern = '(?i)\bUI decides permission\b'; Severity = 'FAIL'; Message = 'UI may be deciding permission.' }
            @{ Pattern = '(?i)\bUI resolves capability\b'; Severity = 'FAIL'; Message = 'UI may be resolving capability.' }
        )
    }
    @{
        Id = 'D'
        Title = 'Provider drift'
        Rules = @(
            @{ Pattern = '(?i)\bdefault provider\b'; Severity = 'WARN'; Message = 'Default provider wording detected.' }
            @{ Pattern = '(?i)\bprovider required\b'; Severity = 'FAIL'; Message = 'Provider may be treated as required.' }
            @{ Pattern = '(?i)\bauto connect\b'; Severity = 'FAIL'; Message = 'Auto-connect wording detected.' }
        )
    }
    @{
        Id = 'E'
        Title = 'ActionResolution bypass'
        Rules = @(
            @{ Pattern = '(?i)\bbypass ActionResolution\b'; Severity = 'FAIL'; Message = 'Bypass ActionResolution wording detected.' }
            @{ Pattern = '(?i)\bdirect execution\b'; Severity = 'FAIL'; Message = 'Direct execution wording detected.' }
        )
    }
    @{
        Id = 'F'
        Title = 'Secret and context exposure'
        Rules = @(
            @{ Pattern = '(?i)\bcredentials\b'; Severity = 'INFO'; Message = 'Credentials wording detected; review context.' }
            @{ Pattern = '(?i)\bapi key\b'; Severity = 'FAIL'; Message = 'API key wording detected; review exposure risk.' }
            @{ Pattern = '(?i)\bsecret\b'; Severity = 'FAIL'; Message = 'Secret wording detected; review exposure risk.' }
            @{ Pattern = '(?i)\btoken\b'; Severity = 'WARN'; Message = 'Token wording detected; review context.' }
        )
    }
    @{
        Id = 'G'
        Title = 'Catalog exposure'
        Rules = @(
            @{ Pattern = '(?i)\bfull tool catalog\b'; Severity = 'FAIL'; Message = 'Full tool catalog wording detected.' }
            @{ Pattern = '(?i)\braw tool catalog\b'; Severity = 'FAIL'; Message = 'Raw tool catalog wording detected.' }
        )
    }
)

$scannedPaths = @(
    'governance/GOVERNANCE.md',
    'governance/FUNCTIONAL_SCOPE.md',
    'governance/WORKSPACE_RULES.md',
    'docs/specs/*.md',
    'crates/**/*.rs'
)

$filesToScan = New-Object System.Collections.Generic.List[object]

foreach ($relative in @('governance/GOVERNANCE.md','governance/FUNCTIONAL_SCOPE.md','governance/WORKSPACE_RULES.md')) {
    $full = Join-Path $rootDir $relative
    if (Test-Path -LiteralPath $full) {
        $filesToScan.Add([pscustomobject]@{ FullPath = $full; RelativePath = $relative })
    }
}

$specsDir = Join-Path $rootDir 'docs\specs'
if (Test-Path -LiteralPath $specsDir) {
    Get-ChildItem -LiteralPath $specsDir -Recurse -File -Filter *.md | Sort-Object FullName | ForEach-Object {
        $filesToScan.Add([pscustomobject]@{
            FullPath = $_.FullName
            RelativePath = Get-RelativePath -FullPath $_.FullName
        })
    }
}

$cratesDir = Join-Path $rootDir 'crates'
if (Test-Path -LiteralPath $cratesDir) {
    Get-ChildItem -LiteralPath $cratesDir -Recurse -File -Filter *.rs | Sort-Object FullName | ForEach-Object {
        $filesToScan.Add([pscustomobject]@{
            FullPath = $_.FullName
            RelativePath = Get-RelativePath -FullPath $_.FullName
        })
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
$reportLines.Add('F10 Final Boundary Audit Report')
$reportLines.Add('')
$reportLines.Add(('timestamp: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('repository_root: {0}' -f $rootDir))
$reportLines.Add('audit_status: ADVISORY')
$reportLines.Add('')
$reportLines.Add('Limitations:')
$reportLines.Add('- textual scan only')
$reportLines.Add('- false positives expected')
$reportLines.Add('- false negatives possible')
$reportLines.Add('- human review required')
$reportLines.Add('- must not be used as Codex patch input')
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
$reportLines.Add('Suspicious matches by category:')

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
$reportLines.Add('result: HUMAN_REVIEW_REQUIRED')
$reportLines.Add('exit_code_policy: always 0 unless script crashes')

Set-Content -LiteralPath $reportPath -Value $reportLines -Encoding UTF8

Write-Host '=== F10 FINAL BOUNDARY AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'

exit 0
