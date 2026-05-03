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
$reportPath = Join-Path $outputDir 'audit_f9_boundary_drift_report.txt'

if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir | Out-Null
}

# -----------------------------
# Scope: SOLO docs/specs
# -----------------------------

$targetDir = Join-Path $rootDir 'docs\specs'

if (-not (Test-Path $targetDir)) {
    Write-Host "docs/specs not found"
    exit 0
}

# -----------------------------
# Patterns (coarse scan)
# -----------------------------

$patterns = @(
    'execute',
    'execution',
    'run',
    'runtime',
    'invoke',
    'call',
    'llm',
    'provider',
    'api call',
    'write',
    'modify',
    'mutation',
    'editable',
    'edit',
    'delete',
    'rename',
    'ui executes',
    'logic in ui',
    'business logic',
    'bypass',
    'direct execution',
    'external dependency',
    'network call'
)

# -----------------------------
# Scan
# -----------------------------

$results = @()
$files = Get-ChildItem $targetDir -Recurse -File -Filter *.md

foreach ($file in $files) {

    $relative = $file.FullName.Substring($rootDir.Length + 1).Replace('\','/')

    $lines = Get-Content $file.FullName

    for ($i = 0; $i -lt $lines.Count; $i++) {

        $line = $lines[$i].ToLowerInvariant()

        foreach ($p in $patterns) {
            if ($line.Contains($p)) {

                $results += [pscustomobject]@{
                    file = $relative
                    line = $i + 1
                    pattern = $p
                    text = $lines[$i].Trim()
                }

                break
            }
        }
    }
}

# -----------------------------
# Output (TXT simple)
# -----------------------------

$report = @()

$report += "=== F9 BOUNDARY DRIFT (EXPERIMENTAL) ==="
$report += "Generated: $(Get-Date)"
$report += ""
$report += "Scope: docs/specs"
$report += "Total findings: $($results.Count)"
$report += ""
$report += "NOTE:"
$report += "- This is a coarse scan."
$report += "- It does NOT distinguish between governance rules and real violations."
$report += "- DO NOT use this report as input for Codex."
$report += "- Human review required."
$report += ""

foreach ($r in $results | Select-Object -First 200) {
    $report += "$($r.file):$($r.line) [$($r.pattern)]"
    $report += "  $($r.text)"
}

if ($results.Count -gt 200) {
    $report += ""
    $report += "... truncated ..."
}

$report | Set-Content $reportPath

Write-Host "=== F9 BOUNDARY DRIFT (EXPERIMENTAL) ==="
Write-Host "findings=$($results.Count)"
Write-Host "report_path=$reportPath"
Write-Host "report_written=true"

exit 0