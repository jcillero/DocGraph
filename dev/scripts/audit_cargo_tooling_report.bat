@echo off
setlocal

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$file = '%~f0';" ^
  "$scriptArgs = '%*';" ^
  "$content = Get-Content -LiteralPath $file;" ^
  "$marker = ($content | Select-String '^# POWERSHELL$' | Select-Object -First 1).LineNumber;" ^
  "if (-not $marker) { throw 'PowerShell marker not found.' }" ^
  "$script = ($content[($marker)..($content.Length - 1)] -join [Environment]::NewLine);" ^
  "Invoke-Expression $script"

set EXIT_CODE=%ERRORLEVEL%
endlocal & exit /b %EXIT_CODE%
# POWERSHELL
$scriptDir = Split-Path -Parent $file
$rootDir = [IO.Path]::GetFullPath((Join-Path $scriptDir '..\..'))
$outputDir = Join-Path $rootDir 'user\output'
$reportPath = Join-Path $outputDir 'audit_cargo_tooling_report.md'
$deepMode = ($scriptArgs -split '\s+') -contains '--deep'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

function Invoke-ToolAudit {
    param(
        [string]$SectionTitle,
        [string]$CommandName,
        [string[]]$Arguments
    )

    $command = Get-Command $CommandName -ErrorAction SilentlyContinue
    $commandText = if ($Arguments.Count -gt 0) {
        '{0} {1}' -f $CommandName, ($Arguments -join ' ')
    } else {
        $CommandName
    }

    if ($null -eq $command) {
        return [pscustomobject]@{
            Title = $SectionTitle
            Command = $commandText
            Status = 'NOT_AVAILABLE'
            ExitCode = $null
            Stdout = ''
            Stderr = ''
        }
    }

    try {
        $psi = New-Object System.Diagnostics.ProcessStartInfo
        $psi.FileName = $command.Source
        $psi.Arguments = ($Arguments -join ' ')
        $psi.WorkingDirectory = $rootDir
        $psi.UseShellExecute = $false
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true

        $process = New-Object System.Diagnostics.Process
        $process.StartInfo = $psi
        $null = $process.Start()
        $stdout = $process.StandardOutput.ReadToEnd()
        $stderr = $process.StandardError.ReadToEnd()
        $process.WaitForExit()
        $exitCode = $process.ExitCode

        return [pscustomobject]@{
            Title = $SectionTitle
            Command = $commandText
            Status = $(if ($exitCode -eq 0) { 'OK' } else { 'FAILED' })
            ExitCode = $exitCode
            Stdout = $stdout
            Stderr = $stderr
        }
    } catch {
        return [pscustomobject]@{
            Title = $SectionTitle
            Command = $commandText
            Status = 'FAILED'
            ExitCode = -1
            Stdout = ''
            Stderr = ($_ | Out-String)
        }
    }
}

function New-SkippedAudit {
    param(
        [string]$SectionTitle,
        [string]$CommandText
    )

    return [pscustomobject]@{
        Title = $SectionTitle
        Command = $CommandText
        Status = 'SKIPPED_HEAVY_BY_DEFAULT'
        ExitCode = $null
        Stdout = ''
        Stderr = ''
    }
}

$audits = New-Object System.Collections.Generic.List[object]
$audits.Add((Invoke-ToolAudit -SectionTitle 'cargo tree' -CommandName 'cargo' -Arguments @('tree', '--workspace')))
$audits.Add((Invoke-ToolAudit -SectionTitle 'cargo metadata' -CommandName 'cargo' -Arguments @('metadata', '--format-version', '1', '--no-deps')))

if ($deepMode) {
    $audits.Add((Invoke-ToolAudit -SectionTitle 'cargo clippy' -CommandName 'cargo' -Arguments @('clippy', '--workspace', '--all-targets', '--', '--no-deps')))
    $audits.Add((Invoke-ToolAudit -SectionTitle 'cargo udeps' -CommandName 'cargo-udeps' -Arguments @('--workspace')))
    $audits.Add((Invoke-ToolAudit -SectionTitle 'cargo bloat' -CommandName 'cargo-bloat' -Arguments @('--workspace', '--crates')))
} else {
    $audits.Add((New-SkippedAudit -SectionTitle 'cargo clippy' -CommandText 'cargo clippy --workspace --all-targets -- --no-deps'))
    $audits.Add((New-SkippedAudit -SectionTitle 'cargo udeps' -CommandText 'cargo-udeps --workspace'))
    $audits.Add((New-SkippedAudit -SectionTitle 'cargo bloat' -CommandText 'cargo-bloat --workspace --crates'))
}

$audits.Add((Invoke-ToolAudit -SectionTitle 'cargo deny' -CommandName 'cargo-deny' -Arguments @('check')))

$report = New-Object System.Collections.Generic.List[string]
$report.Add('# Cargo Tooling Audit Report')
$report.Add('')
$report.Add(('- Generated at: `{0}`' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$report.Add(('- Workspace root: `{0}`' -f '.'))
$report.Add(('- Report path: `{0}`' -f 'user/output/audit_cargo_tooling_report.md'))
$report.Add(('- Deep mode: `{0}`' -f ($(if ($deepMode) { 'yes' } else { 'no' }))))
$report.Add('')
$report.Add('---')
$report.Add('')

foreach ($audit in $audits) {
    $report.Add(('## {0}' -f $audit.Title))
    $report.Add('')
    $report.Add(('- Status: `{0}`' -f $audit.Status))
    $report.Add(('- Command: `{0}`' -f $audit.Command))
    if ($null -ne $audit.ExitCode) {
        $report.Add(('- Exit code: `{0}`' -f $audit.ExitCode))
    }
    $report.Add('')

    if (-not [string]::IsNullOrWhiteSpace($audit.Stdout)) {
        $report.Add('### stdout')
        $report.Add('')
        $report.Add('```text')
        $report.AddRange((($audit.Stdout.TrimEnd()) -split "`r?`n"))
        $report.Add('```')
        $report.Add('')
    }

    if (-not [string]::IsNullOrWhiteSpace($audit.Stderr)) {
        $report.Add('### stderr')
        $report.Add('')
        $report.Add('```text')
        $report.AddRange((($audit.Stderr.TrimEnd()) -split "`r?`n"))
        $report.Add('```')
        $report.Add('')
    }

    if ([string]::IsNullOrWhiteSpace($audit.Stdout) -and [string]::IsNullOrWhiteSpace($audit.Stderr)) {
        if ($audit.Status -eq 'SKIPPED_HEAVY_BY_DEFAULT') {
            $report.Add('Skipped by default. Re-run with `--deep` to execute this tool.')
        } else {
            $report.Add('No output captured.')
        }
        $report.Add('')
    }
}

$report.Add('## Summary')
$report.Add('')
foreach ($audit in $audits) {
    $slug = ($audit.Title -replace '\s+', '_')
    $report.Add(('- `{0}`: `{1}`' -f $slug, $audit.Status))
}

Set-Content -LiteralPath $reportPath -Value $report -Encoding UTF8

Write-Host ('[OK] audit_cargo_tooling_report written to {0}' -f 'user/output/audit_cargo_tooling_report.md')
foreach ($audit in $audits) {
    Write-Host ('[OK] {0}={1}' -f (($audit.Title -replace '\s+', '_').ToLowerInvariant()), $audit.Status)
}

exit 0
