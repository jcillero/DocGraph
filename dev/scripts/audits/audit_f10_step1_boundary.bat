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
$reportPath = Join-Path $outputDir 'audit_f10_step1_boundary_report.txt'

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
    $negativeRegex = '(?i)\b(must not|mustn''t|do not|does not|did not|should not|not allowed|forbidden|prohibited|blocked|disabled|closed|remains closed|advisory only|proposal only|declarative only|read-only|readonly|without|never|no )'
    return ($Line -match $negativeRegex)
}

$categories = @(
    @{
        Id = 'A'
        Title = 'EffectiveToolSurface drift'
        Rules = @(
            @{ Pattern = '(?i)\binject(?:s|ed|ion)?\b.*\braw full tool catalog\b'; Severity = 'FAIL'; Message = 'Raw full tool catalog appears to be injected.' }
            @{ Pattern = '(?i)\bvisibility\b.*\b(permission|authorize|execution permission)\b'; Severity = 'WARN'; Message = 'Visibility may be treated as permission.' }
            @{ Pattern = '(?i)\b(EffectiveToolSurfaceSummary|summary)\b.*\b(execution|execute|permission|authorize)\b'; Severity = 'WARN'; Message = 'Summary may be treated as execution authority.' }
            @{ Pattern = '(?i)\b(UI|LLM)\b.*\b(decides?|determines?|resolves?|computes?)\b.*\b(policy|permission|effective tool surface)\b'; Severity = 'FAIL'; Message = 'UI or LLM may be deciding policy or permissions.' }
        )
    }
    @{
        Id = 'B'
        Title = 'LLM context exposure'
        Rules = @(
            @{ Pattern = '(?i)\b(credentials?|secrets?|passwords?|tokens?|api[_-]?keys?)\b.*\b(LLM context|prompt|context|logs?)\b'; Severity = 'FAIL'; Message = 'Credentials or secrets may be exposed to LLM context or logs.' }
            @{ Pattern = '(?i)\braw host[- ]specific filesystem paths?\b.*\b(context|prompt|LLM)\b'; Severity = 'WARN'; Message = 'Raw host-specific paths may enter LLM context.' }
            @{ Pattern = '(?i)\bfull raw tool catalogs?\b.*\b(LLM|prompt|context)\b'; Severity = 'FAIL'; Message = 'Full raw tool catalog may be injected into LLM context.' }
            @{ Pattern = '(?i)\bhidden system prompts?\b.*\b(exposed|shown|included|injected)\b'; Severity = 'FAIL'; Message = 'Hidden system prompts may be exposed.' }
            @{ Pattern = '(?i)\bdenied_context_refs\b.*\b(ignored|bypassed|unenforced|not enforced)\b'; Severity = 'FAIL'; Message = 'Denied context references may be ignored.' }
        )
    }
    @{
        Id = 'C'
        Title = 'ActionResolution boundary'
        Rules = @(
            @{ Pattern = '(?i)\bActionResolution runner\b.*\b(implemented|exists|available|enabled)\b'; Severity = 'FAIL'; Message = 'ActionResolution runner may be treated as implemented.' }
            @{ Pattern = '(?i)\b(dispatcher|authorized executor)\b.*\b(implemented|available|enabled)\b'; Severity = 'WARN'; Message = 'Dispatcher or executor wording may imply executable action flow.' }
            @{ Pattern = '(?i)\bapprove (and|&) run\b'; Severity = 'FAIL'; Message = 'Approve-and-run wording detected.' }
            @{ Pattern = '(?i)\b(LLM|UI)\b\s*(->|to)?\s*\bexecute\b'; Severity = 'FAIL'; Message = 'Direct execute path from LLM or UI detected.' }
            @{ Pattern = '(?i)\b(auto(?:matic)?|auto-?)\s*action execution\b'; Severity = 'FAIL'; Message = 'Automatic action execution wording detected.' }
        )
    }
    @{
        Id = 'D'
        Title = 'UI authority drift'
        Rules = @(
            @{ Pattern = '(?i)\bUI\b.*\b(decides?|determines?|resolves?)\b.*\b(permission|permissions|policy|capability state)\b'; Severity = 'FAIL'; Message = 'UI may be deciding permissions, policy, or capability state.' }
            @{ Pattern = '(?i)\bUI\b.*\b(validates?|resolves?)\b.*\b(credentials?)\b'; Severity = 'FAIL'; Message = 'UI may be validating or resolving credentials.' }
            @{ Pattern = '(?i)\bUI\b.*\bexecutes?\b.*\b(tools?|actions?)\b'; Severity = 'FAIL'; Message = 'UI may be executing tools or actions.' }
            @{ Pattern = '(?i)\bUI\b.*\bcomputes?\b.*\btool policy\b'; Severity = 'WARN'; Message = 'UI may be computing tool policy.' }
            @{ Pattern = '(?i)\bUI\b.*\bruntime authority\b'; Severity = 'FAIL'; Message = 'UI may be treated as runtime authority.' }
        )
    }
    @{
        Id = 'E'
        Title = 'Provider binding drift'
        Rules = @(
            @{ Pattern = '(?i)\bprovider required by default\b'; Severity = 'FAIL'; Message = 'Provider may be required by default.' }
            @{ Pattern = '(?i)\bdefault cloud provider\b'; Severity = 'WARN'; Message = 'Default cloud provider wording detected.' }
            @{ Pattern = '(?i)\blive provider validation required\b'; Severity = 'FAIL'; Message = 'Live provider validation may be required too early.' }
            @{ Pattern = '(?i)\bprovider binding\b.*\b(not phase-gated|without phase gate|always)\b'; Severity = 'FAIL'; Message = 'Provider binding may not be phase-gated.' }
            @{ Pattern = '(?i)\b(F10 step-1|F10\.1)\b.*\bprovider invocation\b'; Severity = 'WARN'; Message = 'Provider invocation referenced in F10 step-1 context.' }
        )
    }
    @{
        Id = 'F'
        Title = 'Tool proposal vs execution drift'
        Rules = @(
            @{ Pattern = '(?i)\btool proposal\b.*\b(execute|execution)\b'; Severity = 'WARN'; Message = 'Tool proposal may be collapsing into execution.' }
            @{ Pattern = '(?i)\brun suggested tool\b'; Severity = 'FAIL'; Message = 'Suggested tool may be directly runnable.' }
            @{ Pattern = '(?i)\bauto-?run tool candidate\b'; Severity = 'FAIL'; Message = 'Tool candidate may auto-run.' }
            @{ Pattern = '(?i)\bLLM tool call\b.*\b(execute|execution|run directly)\b'; Severity = 'FAIL'; Message = 'LLM tool call may execute directly.' }
            @{ Pattern = '(?i)\btool intent\b.*\bbypass(?:es)?\b.*\bActionResolution\b'; Severity = 'FAIL'; Message = 'Tool intent may bypass ActionResolution.' }
        )
    }
    @{
        Id = 'G'
        Title = 'Privacy / consent gate drift'
        Rules = @(
            @{ Pattern = '(?i)\bimplicit consent\b'; Severity = 'FAIL'; Message = 'Implicit consent wording detected.' }
            @{ Pattern = '(?i)\bhidden consent escalation\b'; Severity = 'FAIL'; Message = 'Hidden consent escalation wording detected.' }
            @{ Pattern = '(?i)\bprivacy level\b.*\b(grants?|enables?)\b.*\b(authority|permission|execution)\b'; Severity = 'FAIL'; Message = 'Privacy level may be granting authority.' }
            @{ Pattern = '(?i)\bconsent\b.*\b(implies?|grants?)\b.*\bexecution permission\b'; Severity = 'FAIL'; Message = 'Consent may be granting execution permission.' }
            @{ Pattern = '(?i)\bLLM processing without consent\b'; Severity = 'FAIL'; Message = 'LLM processing may occur without consent.' }
        )
    }
    @{
        Id = 'H'
        Title = 'Phase claim drift'
        Rules = @(
            @{ Pattern = '(?i)\bF10 fully opened\b'; Severity = 'FAIL'; Message = 'F10 may be claimed as fully opened.' }
            @{ Pattern = '(?i)\bbroad tool execution opened\b'; Severity = 'FAIL'; Message = 'Broad tool execution may be claimed as opened.' }
            @{ Pattern = '(?i)\bActionResolution runner implemented\b'; Severity = 'FAIL'; Message = 'ActionResolution runner may be claimed as implemented.' }
            @{ Pattern = '(?i)\bprovider execution enabled by default\b'; Severity = 'FAIL'; Message = 'Provider execution may be enabled by default.' }
            @{ Pattern = '(?i)\bEffectiveToolSurfaceResolver implemented\b'; Severity = 'FAIL'; Message = 'EffectiveToolSurfaceResolver may be claimed as implemented.' }
            @{ Pattern = '(?i)\bLLM runtime available\b'; Severity = 'WARN'; Message = 'LLM runtime availability wording may need qualification.' }
        )
    }
)

$scannedPaths = @(
    'governance/GOVERNANCE.md',
    'governance/FUNCTIONAL_SCOPE.md',
    'governance/WORKSPACE_RULES.md',
    'governance/LLM_RUNTIME_POLICY.md',
    'docs/specs/*.md',
    'crates/**/*.rs'
)

$filesToScan = New-Object System.Collections.Generic.List[object]

foreach ($relative in @('governance/GOVERNANCE.md','governance/FUNCTIONAL_SCOPE.md','governance/WORKSPACE_RULES.md','governance/LLM_RUNTIME_POLICY.md')) {
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
$reportLines.Add('F10 Step-1 Boundary Audit Report')
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
$reportLines.Add('- not an automatic fix input')
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

Write-Host '=== F10 STEP-1 BOUNDARY AUDIT ==='
Write-Host ('scanned_file_count={0}' -f $filesToScan.Count)
Write-Host ('fail_count={0}' -f $failTotal)
Write-Host ('warn_count={0}' -f $warnTotal)
Write-Host ('info_count={0}' -f $infoTotal)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY'

exit 0
