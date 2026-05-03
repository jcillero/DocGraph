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
$reportPath = Join-Path $outputDir 'audit_i18n_consistency_report.txt'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$enDir = Join-Path $rootDir 'resources\i18n\en'
$esDir = Join-Path $rootDir 'resources\i18n\es'
$specsDir = Join-Path $rootDir 'docs\specs'

function Get-FtlKeys {
    param([string]$DirPath)

    $keys = New-Object System.Collections.Generic.List[object]
    if (-not (Test-Path -LiteralPath $DirPath)) {
        return $keys
    }

    Get-ChildItem -LiteralPath $DirPath -File -Filter *.ftl | Sort-Object Name | ForEach-Object {
        $relative = $_.FullName.Substring($rootDir.Length + 1).Replace('\','/')
        $lines = Get-Content -LiteralPath $_.FullName
        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = [string]$lines[$i]
            if ($line -match '^\s*([A-Za-z0-9_.-]+)\s*=') {
                $keys.Add([pscustomobject]@{
                    Key = $matches[1]
                    File = $relative
                    Line = $i + 1
                })
            }
        }
    }

    return $keys
}

function Add-Section {
    param(
        [System.Collections.Generic.List[string]]$Lines,
        [string]$Title
    )

    $Lines.Add($Title)
    $Lines.Add((''.PadLeft($Title.Length, '-')))
}

$enKeys = Get-FtlKeys -DirPath $enDir
$esKeys = Get-FtlKeys -DirPath $esDir

$enKeyMap = @{}
foreach ($entry in $enKeys) {
    $enKeyMap[$entry.Key] = $entry
}

$esKeyMap = @{}
foreach ($entry in $esKeys) {
    $esKeyMap[$entry.Key] = $entry
}

$enKeyNames = @($enKeyMap.Keys | Sort-Object)
$esKeyNames = @($esKeyMap.Keys | Sort-Object)

$missingInEs = Compare-Object $enKeyNames $esKeyNames | Where-Object { $_.SideIndicator -eq '<=' } | Select-Object -ExpandProperty InputObject
$missingInEn = Compare-Object $enKeyNames $esKeyNames | Where-Object { $_.SideIndicator -eq '=>' } | Select-Object -ExpandProperty InputObject

$specFiles = @()
if (Test-Path -LiteralPath $specsDir) {
    $specFiles = Get-ChildItem -LiteralPath $specsDir -Recurse -File -Filter *.md | Sort-Object FullName
}

$specTexts = @{}
foreach ($specFile in $specFiles) {
    $relative = $specFile.FullName.Substring($rootDir.Length + 1).Replace('\','/')
    $specTexts[$relative] = Get-Content -LiteralPath $specFile.FullName -Raw
}

$allKeys = @($enKeyNames + $esKeyNames | Sort-Object -Unique)
$potentiallyUnused = New-Object System.Collections.Generic.List[object]
foreach ($key in $allKeys) {
    $found = $false
    foreach ($pair in $specTexts.GetEnumerator()) {
        if ($pair.Value -match [regex]::Escape($key)) {
            $found = $true
            break
        }
    }
    if (-not $found) {
        $source = if ($enKeyMap.ContainsKey($key)) { $enKeyMap[$key].File } else { $esKeyMap[$key].File }
        $potentiallyUnused.Add([pscustomobject]@{
            Key = $key
            Source = $source
        })
    }
}

$hardcodedFindings = New-Object System.Collections.Generic.List[object]
$uiSpecPatterns = @(
    '^app_main_menu\.md$',
    '^help_menu\.md$',
    '^tools_panel\.md$',
    '^ui_.*\.md$',
    '^workspace_tabs\.md$',
    '^lume_.*\.md$'
)

$quotedTextRegex = '(?:"([^"]{3,})"|`([^`]{3,})`)'
$allowedPrefixRegex = '^(INV-|docs/specs/|resources/|user/|crates/|dev/|ActionRequest|ActionResolution|ConfigurationIntent|ExecutionIntent|LumeHelpPopup|LumeHelpTree|AboutPopup|DocGraph|Lume|File|Preferences|Tools|Help|Operational Tools|LLM Tools|External Dependencies|Open Lume Help|Help Topics|About DocGraph|App preferences|Project settings|Credentials management|Prepare execution|Install / Configure|Delete dependency|Remove reference|Add custom external dependency)$'

foreach ($specFile in $specFiles) {
    $name = $specFile.Name
    $isUiSpec = $false
    foreach ($pattern in $uiSpecPatterns) {
        if ($name -match $pattern) {
            $isUiSpec = $true
            break
        }
    }
    if (-not $isUiSpec) {
        continue
    }

    $relative = $specFile.FullName.Substring($rootDir.Length + 1).Replace('\','/')
    $lines = Get-Content -LiteralPath $specFile.FullName
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = [string]$lines[$i]
        if ($line.TrimStart().StartsWith('- `menu.') -or $line.TrimStart().StartsWith('- `popup.') -or $line.TrimStart().StartsWith('- `dialog.')) {
            continue
        }

        $matches = [regex]::Matches($line, $quotedTextRegex)
        foreach ($match in $matches) {
            $candidate = if ($match.Groups[1].Success) { $match.Groups[1].Value } else { $match.Groups[2].Value }
            $candidate = $candidate.Trim()
            if ($candidate.Length -lt 3) { continue }
            if ($candidate -match '^[a-z0-9_.-]+$') { continue }
            if ($candidate -match $allowedPrefixRegex) { continue }
            if ($candidate -match '^(PROPOSAL /|NORMATIVE|SPEC-ONLY|F9|F10|Static Mode)$') { continue }
            if ($candidate -match '(/|\\|::|://|->|\.md$|\.json$|\.ftl$)') { continue }
            if ($candidate -match '^[A-Za-z0-9_.:-]+$') { continue }
            if ($candidate -match '^\[.*\]$') { continue }

            $hardcodedFindings.Add([pscustomobject]@{
                File = $relative
                Line = $i + 1
                Text = $candidate
            })
        }
    }
}

$reportLines = New-Object System.Collections.Generic.List[string]
$reportLines.Add('I18N Consistency Audit Report')
$reportLines.Add('')
$reportLines.Add(('generated_at: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('workspace_root: {0}' -f $rootDir))
$reportLines.Add('status: ADVISORY_ONLY')
$reportLines.Add('')
$reportLines.Add('Rules:')
$reportLines.Add('- This audit is advisory only.')
$reportLines.Add('- It does not modify files.')
$reportLines.Add('- It must not be used as Codex patch input.')
$reportLines.Add('- Missing keys are consistency findings.')
$reportLines.Add('- Unused keys and hardcoded text findings require human review.')
$reportLines.Add('')

Add-Section -Lines $reportLines -Title '1. Missing keys: en -> es'
if ($missingInEs.Count -eq 0) {
    $reportLines.Add('No missing Spanish keys detected.')
} else {
    foreach ($key in $missingInEs) {
        $reportLines.Add(('- {0} [{1}:{2}]' -f $key, $enKeyMap[$key].File, $enKeyMap[$key].Line))
    }
}
$reportLines.Add('')

Add-Section -Lines $reportLines -Title '2. Missing keys: es -> en'
if ($missingInEn.Count -eq 0) {
    $reportLines.Add('No missing English keys detected.')
} else {
    foreach ($key in $missingInEn) {
        $reportLines.Add(('- {0} [{1}:{2}]' -f $key, $esKeyMap[$key].File, $esKeyMap[$key].Line))
    }
}
$reportLines.Add('')

Add-Section -Lines $reportLines -Title '3. Potentially unused keys in specs'
if ($potentiallyUnused.Count -eq 0) {
    $reportLines.Add('No potentially unused keys detected in docs/specs.')
} else {
    foreach ($entry in $potentiallyUnused | Sort-Object Key) {
        $reportLines.Add(('- {0} [source={1}]' -f $entry.Key, $entry.Source))
    }
}
$reportLines.Add('')

Add-Section -Lines $reportLines -Title '4. Potential hardcoded UI text in specs (heuristic)'
if ($hardcodedFindings.Count -eq 0) {
    $reportLines.Add('No heuristic hardcoded UI text findings detected.')
} else {
    foreach ($finding in $hardcodedFindings) {
        $reportLines.Add(('- {0}:{1} -> {2}' -f $finding.File, $finding.Line, $finding.Text))
    }
}
$reportLines.Add('')

$reportLines.Add('Summary')
$reportLines.Add('-------')
$reportLines.Add(('en_key_count={0}' -f $enKeyNames.Count))
$reportLines.Add(('es_key_count={0}' -f $esKeyNames.Count))
$reportLines.Add(('missing_in_es_count={0}' -f $missingInEs.Count))
$reportLines.Add(('missing_in_en_count={0}' -f $missingInEn.Count))
$reportLines.Add(('potentially_unused_count={0}' -f $potentiallyUnused.Count))
$reportLines.Add(('hardcoded_ui_text_count={0}' -f $hardcodedFindings.Count))
$reportLines.Add('result=REVIEW_REQUIRED_IF_FINDINGS_PRESENT')

Set-Content -LiteralPath $reportPath -Value $reportLines -Encoding UTF8

Write-Host '=== I18N CONSISTENCY AUDIT ==='
Write-Host ('en_key_count={0}' -f $enKeyNames.Count)
Write-Host ('es_key_count={0}' -f $esKeyNames.Count)
Write-Host ('missing_in_es_count={0}' -f $missingInEs.Count)
Write-Host ('missing_in_en_count={0}' -f $missingInEn.Count)
Write-Host ('potentially_unused_count={0}' -f $potentiallyUnused.Count)
Write-Host ('hardcoded_ui_text_count={0}' -f $hardcodedFindings.Count)
Write-Host ('report_path={0}' -f $reportPath)
Write-Host 'status=ADVISORY_ONLY'

exit 0
