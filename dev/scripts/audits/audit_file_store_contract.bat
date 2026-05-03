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
$reportPath = Join-Path $outputDir 'audit_file_store_contract_report.txt'

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
        Title = 'Exposure authority drift'
        Rules = @(
            @{ Pattern = '(?i)\bfile_store\b.*\b(exposure authority|governs exposure|defines exposure)\b'; Severity = 'FAIL'; Message = 'file_store may be treated as exposure authority.' }
            @{ Pattern = '(?i)\bproject_manifest\b.*\b(bypass|ignored|not required)\b'; Severity = 'FAIL'; Message = 'project_manifest exposure authority may be bypassed.' }
            @{ Pattern = '(?i)\bschema validity\b.*\b(exposure authority|project exposure)\b'; Severity = 'FAIL'; Message = 'Schema validity may be treated as exposure authority.' }
            @{ Pattern = '(?i)\bschema validity\b.*\b(execution authority|execution permission)\b'; Severity = 'FAIL'; Message = 'Schema validity may be treated as execution authority.' }
        )
    }
    @{
        Id = 'B'
        Title = 'Physical storage drift'
        Rules = @(
            @{ Pattern = '(?i)\b(duplicate|duplication|duplicated)\b.*\b(usage family|usage kind|chat|document|knowledge)\b'; Severity = 'WARN'; Message = 'Possible physical duplication by usage family.' }
            @{ Pattern = '(?i)\b(refs?|usage_ref)\b.*\b(physical cop(?:y|ies)|stored copy|blob copy)\b'; Severity = 'FAIL'; Message = 'Refs may be treated as physical copies.' }
            @{ Pattern = '(?i)\bindex(?:es)?\b.*\b(authoritative|source of truth)\b'; Severity = 'FAIL'; Message = 'Indexes may be treated as authority.' }
            @{ Pattern = '(?i)\bchat\b.*\b(hidden blob store|blob storage|source of truth)\b'; Severity = 'FAIL'; Message = 'Chat may be treated as hidden blob store.' }
        )
    }
    @{
        Id = 'C'
        Title = 'Portable truth drift'
        Rules = @(
            @{ Pattern = '(?i)\babsolute host paths?\b.*\b(portable truth|authoritative|source of truth)\b'; Severity = 'FAIL'; Message = 'Absolute host paths may be treated as portable truth.' }
            @{ Pattern = '(?i)\bhost paths?\b.*\b(authoritative|portable)\b'; Severity = 'WARN'; Message = 'Host-path portability wording detected.' }
        )
    }
)

$scannedPaths = @(
    'docs/specs/storage_policy.md',
    'docs/specs/document_references.md',
    'resources/storage/storage_policy.json',
    'resources/ai/schemas/*.json',
    'governance/GOVERNANCE.md',
    'governance/FUNCTIONAL_SCOPE.md'
)

$filesToScan = New-Object System.Collections.Generic.List[object]

foreach ($relative in @(
    'docs/specs/storage_policy.md',
    'docs/specs/document_references.md',
    'resources/storage/storage_policy.json',
    'governance/GOVERNANCE.md',
    'governance/FUNCTIONAL_SCOPE.md'
)) {
    $full = Join-Path $rootDir $relative
    if (Test-Path -LiteralPath $full) {
        $filesToScan.Add([pscustomobject]@{ FullPath = $full; RelativePath = $relative })
    }
}

$schemaDir = Join-Path $rootDir 'resources\ai\schemas'
if (Test-Path -LiteralPath $schemaDir) {
    Get-ChildItem -LiteralPath $schemaDir -File -Filter *.json | Sort-Object FullName | ForEach-Object {
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
$reportLines.Add('File Store Contract Audit Report')
$reportLines.Add('')
$reportLines.Add(('timestamp: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('repository_root: {0}' -f $rootDir))
$reportLines.Add('status: ADVISORY')
$reportLines.Add('')
$reportLines.Add('Limitations:')
$reportLines.Add('- textual scan only')
$reportLines.Add('- false positives expected')
$reportLines.Add('- human review required')
$reportLines.Add('- no autofix')
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

Write-Host '=== FILE STORE CONTRACT AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'

exit 0
