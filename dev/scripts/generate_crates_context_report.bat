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
$rootDir = [IO.Path]::GetFullPath((Join-Path $scriptDir '..\..'))
$outputDir = Join-Path $rootDir 'user\output'
$reportPath = Join-Path $outputDir 'crates_context_report.md'
$rootCargoToml = Join-Path $rootDir 'Cargo.toml'
$cargoLock = Join-Path $rootDir 'Cargo.lock'
$cratesDir = Join-Path $rootDir 'crates'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

if (-not (Test-Path -LiteralPath $rootCargoToml)) {
    Write-Host '[ERROR] Root Cargo.toml not found.'
    exit 1
}

function Get-RelativePath([string]$basePath, [string]$targetPath) {
    $baseUri = [Uri]((Resolve-Path -LiteralPath $basePath).Path + [IO.Path]::DirectorySeparatorChar)
    $targetUri = [Uri](Resolve-Path -LiteralPath $targetPath).Path
    return [Uri]::UnescapeDataString($baseUri.MakeRelativeUri($targetUri).ToString()).Replace('\', '/')
}

function Get-DependencySummary([string]$manifestPath) {
    $lines = Get-Content -LiteralPath $manifestPath
    $results = New-Object System.Collections.Generic.List[object]
    $currentSection = ''

    foreach ($rawLine in $lines) {
        $line = [string]$rawLine
        $trimmed = $line.Trim()

        if ($trimmed -match '^\[(.+)\]$') {
            $currentSection = $matches[1]
            continue
        }

        if ([string]::IsNullOrWhiteSpace($trimmed) -or $trimmed.StartsWith('#')) {
            continue
        }

        $isDependencySection =
            $currentSection -eq 'dependencies' -or
            $currentSection -eq 'dev-dependencies' -or
            $currentSection -eq 'build-dependencies' -or
            $currentSection -like 'target.*.dependencies' -or
            $currentSection -like 'target.*.dev-dependencies' -or
            $currentSection -like 'target.*.build-dependencies'

        if (-not $isDependencySection) {
            continue
        }

        if ($trimmed -match '^([A-Za-z0-9_.-]+)\s*=') {
            $results.Add([pscustomobject]@{
                Section = $currentSection
                Name = $matches[1]
                Spec = $trimmed
            })
        }
    }

    return $results
}

$rootCargoContent = Get-Content -Raw -LiteralPath $rootCargoToml
$crateManifests = @()
if (Test-Path -LiteralPath $cratesDir) {
    $crateManifests = Get-ChildItem -LiteralPath $cratesDir -Directory | ForEach-Object {
        $manifest = Join-Path $_.FullName 'Cargo.toml'
        if (Test-Path -LiteralPath $manifest) { Get-Item -LiteralPath $manifest }
    } | Sort-Object FullName
}

$crateInfos = foreach ($manifest in $crateManifests) {
    $crateName = Split-Path -Leaf (Split-Path -Parent $manifest.FullName)
    $deps = @(Get-DependencySummary -manifestPath $manifest.FullName)
    $externalDeps = @($deps | Where-Object { $_.Spec -notmatch 'path\s*=' -and $_.Spec -notmatch 'workspace\s*=' })
    [pscustomobject]@{
        CrateName = $crateName
        ManifestPath = Get-RelativePath -basePath $rootDir -targetPath $manifest.FullName
        DependencyCount = @($deps).Count
        Dependencies = $deps
        ExternalDependencies = $externalDeps
    }
}

$externalDependencyUsage = @{}
foreach ($crate in $crateInfos) {
    foreach ($dep in $crate.ExternalDependencies) {
        if (-not $externalDependencyUsage.ContainsKey($dep.Name)) {
            $externalDependencyUsage[$dep.Name] = New-Object System.Collections.Generic.List[string]
        }
        if (-not $externalDependencyUsage[$dep.Name].Contains($crate.CrateName)) {
            $externalDependencyUsage[$dep.Name].Add($crate.CrateName)
        }
    }
}

$supportSignals = [ordered]@{
    JSON = $false
    JSONL = $false
    PDF = $false
    Diff = $false
    Text = $false
    LLM = $false
    Tools = $false
}

$signalEvidence = [ordered]@{
    JSON = New-Object System.Collections.Generic.List[string]
    JSONL = New-Object System.Collections.Generic.List[string]
    PDF = New-Object System.Collections.Generic.List[string]
    Diff = New-Object System.Collections.Generic.List[string]
    Text = New-Object System.Collections.Generic.List[string]
    LLM = New-Object System.Collections.Generic.List[string]
    Tools = New-Object System.Collections.Generic.List[string]
}

foreach ($crate in $crateInfos) {
    $crateNameLower = $crate.CrateName.ToLowerInvariant()
    $depNames = @($crate.Dependencies | ForEach-Object { $_.Name.ToLowerInvariant() })

    if ($depNames -contains 'serde_json' -or $crateNameLower -eq 'verify_progress' -or $crateNameLower -eq 'io_runtime' -or $crateNameLower -eq 'tool_runtime') {
        $supportSignals.JSON = $true
        $signalEvidence.JSON.Add($crate.CrateName) | Out-Null
    }
    if ($crateNameLower -eq 'io_runtime') {
        $supportSignals.JSONL = $true
        $signalEvidence.JSONL.Add($crate.CrateName) | Out-Null
    }
    if ($depNames -contains 'lopdf' -or $crateNameLower -eq 'document_text_runtime') {
        $supportSignals.PDF = $true
        $signalEvidence.PDF.Add($crate.CrateName) | Out-Null
    }
    if ($crateNameLower -eq 'action_core' -or $crateNameLower -eq 'tool_runtime') {
        $supportSignals.Diff = $true
        $signalEvidence.Diff.Add($crate.CrateName) | Out-Null
    }
    if ($crateNameLower -eq 'document_text_runtime' -or $crateNameLower -eq 'workspace_core') {
        $supportSignals.Text = $true
        $signalEvidence.Text.Add($crate.CrateName) | Out-Null
    }
    if ($crateNameLower -like 'llm_*') {
        $supportSignals.LLM = $true
        $signalEvidence.LLM.Add($crate.CrateName) | Out-Null
    }
    if ($crateNameLower -eq 'tool_runtime' -or $crateNameLower -eq 'ui_slint' -or $crateNameLower -eq 'cli_app') {
        $supportSignals.Tools = $true
        $signalEvidence.Tools.Add($crate.CrateName) | Out-Null
    }
}

$toolDesignImplications = New-Object System.Collections.Generic.List[string]
if ($supportSignals.JSON -or $supportSignals.JSONL) {
    $toolDesignImplications.Add('Tool inputs and outputs can plausibly stay structured and traceable through JSON or JSONL-friendly crates already present in the workspace.') | Out-Null
}
if ($supportSignals.PDF) {
    $toolDesignImplications.Add('Document-oriented tools can reasonably assume governed PDF inspection or derivation support exists in the current crate graph, without implying mutation or export runtime.') | Out-Null
}
if ($supportSignals.LLM) {
    $toolDesignImplications.Add('LLM-facing tool design should stay separated from execution because LLM crates exist, but operational authority still belongs outside the model path.') | Out-Null
}
if ($supportSignals.Tools) {
    $toolDesignImplications.Add('The workspace already exposes a distinct tools boundary, so new tool declarations should continue to reference that boundary instead of leaking behavior into project or UI crates.') | Out-Null
}
if ($supportSignals.Text) {
    $toolDesignImplications.Add('Text-first tools remain a good fit because document text derivation support is already visible in the workspace dependency graph.') | Out-Null
}

$cargoCommand = Get-Command cargo -ErrorAction SilentlyContinue
$cargoAvailable = $null -ne $cargoCommand
$cargoTreeCommand = 'cargo tree --workspace'
$cargoTreeStatus = 'NOT_AVAILABLE'
$cargoTreeExitCode = $null
$cargoTreeStdout = ''
$cargoTreeStderr = ''
if ($null -ne $cargoCommand) {
    try {
        $psi = New-Object System.Diagnostics.ProcessStartInfo
        $psi.FileName = $cargoCommand.Source
        $psi.Arguments = 'tree --workspace'
        $psi.WorkingDirectory = $rootDir
        $psi.UseShellExecute = $false
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true

        $process = New-Object System.Diagnostics.Process
        $process.StartInfo = $psi
        $null = $process.Start()
        $cargoTreeStdout = $process.StandardOutput.ReadToEnd()
        $cargoTreeStderr = $process.StandardError.ReadToEnd()
        $process.WaitForExit()
        $cargoTreeExitCode = $process.ExitCode

        if ($cargoTreeExitCode -eq 0) {
            $cargoTreeStatus = 'OK'
        } else {
            $cargoTreeStatus = 'FAILED'
        }
    } catch {
        $cargoTreeStatus = 'FAILED'
        $cargoTreeExitCode = -1
        $cargoTreeStderr = $_ | Out-String
    }
}

$report = New-Object System.Collections.Generic.List[string]
$report.Add('# Crates Context Report')
$report.Add('')
$report.Add(('- Generated at: `{0}`' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$report.Add(('- Workspace root: `{0}`' -f '.'))
$report.Add(('- Root manifest: `{0}`' -f 'Cargo.toml'))
$report.Add(('- Cargo.lock present: `{0}`' -f ($(if (Test-Path -LiteralPath $cargoLock) { 'yes' } else { 'no' }))))
$report.Add(('- Internal crates detected: `{0}`' -f $crateInfos.Count))
$report.Add(('- cargo available: `{0}`' -f ($(if ($cargoAvailable) { 'yes' } else { 'no' }))))
$report.Add(('- cargo tree status: `{0}`' -f $cargoTreeStatus))
$report.Add('')
$report.Add('---')
$report.Add('')
$report.Add('## Root Cargo.toml')
$report.Add('')
$report.Add('```toml')
$report.AddRange(($rootCargoContent -split "`r?`n"))
$report.Add('```')
$report.Add('')
$report.Add('## Internal crates summary')
$report.Add('')

if ($crateInfos.Count -eq 0) {
    $report.Add('No internal crates were detected under `crates/`.')
} else {
    $report.Add('| Crate | Manifest | Dependency count |')
    $report.Add('| --- | --- | ---: |')
    foreach ($crate in $crateInfos) {
        $report.Add(('| `{0}` | `{1}` | {2} |' -f $crate.CrateName, $crate.ManifestPath, $crate.DependencyCount))
    }
}

$report.Add('')
$report.Add('## Dependencies by crate')
$report.Add('')

foreach ($crate in $crateInfos) {
    $report.Add(('### `{0}`' -f $crate.CrateName))
    $report.Add('')
    $report.Add(('- Manifest: `{0}`' -f $crate.ManifestPath))
    $report.Add(('- Dependencies detected: `{0}`' -f $crate.DependencyCount))
    $report.Add('')

    if ($crate.DependencyCount -eq 0) {
        $report.Add('No dependencies detected in dependency sections.')
    } else {
        foreach ($group in ($crate.Dependencies | Group-Object Section | Sort-Object Name)) {
            $report.Add(('- `{0}`' -f $group.Name))
            foreach ($dep in ($group.Group | Sort-Object Name)) {
                $report.Add(('  - `{0}`' -f $dep.Spec))
            }
        }
    }

    $report.Add('')
}

$report.Add('## Aggregated external dependencies')
$report.Add('')

if ($externalDependencyUsage.Count -eq 0) {
    $report.Add('No external crate dependencies were detected.')
} else {
    $report.Add('| Dependency | Used by crates |')
    $report.Add('| --- | --- |')
    foreach ($name in ($externalDependencyUsage.Keys | Sort-Object)) {
        $report.Add(('| `{0}` | `{1}` |' -f $name, (($externalDependencyUsage[$name] | Sort-Object) -join ', ')))
    }
}

$report.Add('')
$report.Add('## Support signals')
$report.Add('')

foreach ($signalName in $supportSignals.Keys) {
    $statusText = if ($supportSignals[$signalName]) { 'detected' } else { 'not_detected' }
    $evidenceText = if ($signalEvidence[$signalName].Count -gt 0) { ($signalEvidence[$signalName] | Sort-Object -Unique) -join ', ' } else { 'none' }
    $report.Add(('- `{0}`: `{1}`' -f $signalName.ToLowerInvariant(), $statusText))
    $report.Add(('  - evidence: `{0}`' -f $evidenceText))
}

$report.Add('')
$report.Add('## Tool design implications')
$report.Add('')
$report.Add('This section is heuristic and non-normative.')
$report.Add('')

if ($toolDesignImplications.Count -eq 0) {
    $report.Add('- No strong tool-design implications were inferred from the current crate dependency graph.')
} else {
    foreach ($implication in $toolDesignImplications) {
        $report.Add(('- {0}' -f $implication))
    }
}

$report.Add('## cargo tree')
$report.Add('')
$report.Add(('- Command: `{0}`' -f $cargoTreeCommand))
if ($null -ne $cargoTreeExitCode) {
    $report.Add(('- Exit code: `{0}`' -f $cargoTreeExitCode))
}
$report.Add('')

if ($cargoTreeStatus -eq 'OK') {
    $report.Add('```text')
    $report.AddRange((($cargoTreeStdout.TrimEnd()) -split "`r?`n"))
    $report.Add('```')
} elseif ($cargoTreeStatus -eq 'FAILED') {
    $report.Add('`cargo tree` returned an error. Report generation continued.')
    $report.Add('')
    if (-not [string]::IsNullOrWhiteSpace($cargoTreeStderr)) {
        $report.Add('### stderr')
        $report.Add('')
        $report.Add('```text')
        $report.AddRange((($cargoTreeStderr.TrimEnd()) -split "`r?`n"))
        $report.Add('```')
        $report.Add('')
    }
    if (-not [string]::IsNullOrWhiteSpace($cargoTreeStdout)) {
        $report.Add('### stdout')
        $report.Add('')
        $report.Add('```text')
        $report.AddRange((($cargoTreeStdout.TrimEnd()) -split "`r?`n"))
        $report.Add('```')
    }
} else {
    $report.Add('`cargo tree` is not available in the current environment. Report generation continued.')
}

$report.Add('')
$report.Add('## Tools summary')
$report.Add('')
$report.Add(('- cargo_available: `{0}`' -f ($(if ($cargoAvailable) { 'yes' } else { 'no' }))))
$report.Add(('- cargo_tree_status: `{0}`' -f $cargoTreeStatus))
$report.Add(('- cargo_lock_present: `{0}`' -f ($(if (Test-Path -LiteralPath $cargoLock) { 'yes' } else { 'no' }))))
$report.Add(('- internal_crates_detected: `{0}`' -f $crateInfos.Count))
$report.Add(('- Report path: `{0}`' -f 'user/output/crates_context_report.md'))

Set-Content -LiteralPath $reportPath -Value $report -Encoding UTF8

Write-Host ('[OK] crates_context_report written to {0}' -f 'user/output/crates_context_report.md')
Write-Host ('[OK] internal_crates={0}' -f $crateInfos.Count)
Write-Host ('[OK] cargo_available={0}' -f ($(if ($cargoAvailable) { 'yes' } else { 'no' })))
Write-Host ('[OK] cargo_lock_present={0}' -f ($(if (Test-Path -LiteralPath $cargoLock) { 'yes' } else { 'no' })))
Write-Host ('[OK] cargo_tree_status={0}' -f $cargoTreeStatus)

exit 0
