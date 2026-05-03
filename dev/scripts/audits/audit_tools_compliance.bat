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
$reportPath = Join-Path $outputDir 'audit_tools_compliance_report.txt'

if (-not (Test-Path -LiteralPath $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$expectedFiles = @(
    'resources/tools/tools_master_catalog.json',
    'resources/tools/internal/operational/tools_operational_document_catalog.json',
    'resources/tools/internal/llm/tools_llm_document_catalog.json',
    'resources/tools/external/tools_external_catalog.json',
    'resources/tool_runtime/meta_catalog.json',
    'resources/tool_runtime/llm_tool_policy.json',
    'docs/specs/tools_catalogs.md',
    'docs/specs/tools_panel.md',
    'docs/specs/tool_implementation_governance.md',
    'docs/specs/how_to_add_a_tool.md'
)

$severityBuckets = @{
    FAIL = New-Object System.Collections.Generic.List[object]
    WARN = New-Object System.Collections.Generic.List[object]
    INFO = New-Object System.Collections.Generic.List[object]
}

function Add-Finding {
    param(
        [string]$Severity,
        [string]$Category,
        [string]$Target,
        [string]$Message
    )
    $severityBuckets[$Severity].Add([pscustomobject]@{
        Severity = $Severity
        Category = $Category
        Target = $Target
        Message = $Message
    })
}

function Get-RelativePath {
    param([string]$FullPath)
    return $FullPath.Substring($rootDir.Length + 1).Replace('\', '/')
}

foreach ($relative in $expectedFiles) {
    $full = Join-Path $rootDir $relative
    if (Test-Path -LiteralPath $full) {
        Add-Finding -Severity 'INFO' -Category 'catalog_or_spec_present' -Target $relative -Message 'Expected governed file found.'
    } else {
        Add-Finding -Severity 'FAIL' -Category 'catalog_or_spec_missing' -Target $relative -Message 'Expected governed file is missing.'
    }
}

$masterCatalogPath = Join-Path $rootDir 'resources/tools/tools_master_catalog.json'
if (Test-Path -LiteralPath $masterCatalogPath) {
    try {
        $masterCatalog = Get-Content -LiteralPath $masterCatalogPath -Raw | ConvertFrom-Json
        if ($masterCatalog.status -eq 'declared_only' -and $masterCatalog.execution_enabled -eq $false) {
            Add-Finding -Severity 'INFO' -Category 'master_catalog_state' -Target 'resources/tools/tools_master_catalog.json' -Message 'Master catalog remains declared_only with execution disabled.'
        } elseif ($masterCatalog.execution_enabled -eq $true) {
            Add-Finding -Severity 'FAIL' -Category 'master_catalog_execution_enabled' -Target 'resources/tools/tools_master_catalog.json' -Message 'Master catalog exposes execution_enabled=true in F9/F10_PREP.'
        } else {
            Add-Finding -Severity 'WARN' -Category 'master_catalog_state_ambiguous' -Target 'resources/tools/tools_master_catalog.json' -Message 'Master catalog state should stay declared_only with execution disabled.'
        }

        foreach ($tool in $masterCatalog.tools) {
            $toolTarget = 'resources/tools/tools_master_catalog.json::' + $tool.tool_id

            if ($tool.status -eq 'declared_only' -and $tool.execution_enabled -eq $true) {
                Add-Finding -Severity 'FAIL' -Category 'declared_only_implies_execution' -Target $toolTarget -Message 'declared_only tool has execution_enabled=true.'
            }

            if ($tool.execution_enabled -eq $true) {
                Add-Finding -Severity 'FAIL' -Category 'tool_execution_enabled' -Target $toolTarget -Message 'execution_enabled=true requires explicit later-phase exception and is not valid as default F9/F10_PREP declaration.'
            }

            if ($tool.tool_type -eq 'external_dependency') {
                $declText = ($tool.declaration | ConvertTo-Json -Depth 20 -Compress)
                if ($declText -match '(?i)\btool\b' -and $declText -notmatch '(?i)dependency') {
                    Add-Finding -Severity 'WARN' -Category 'external_dependency_tool_wording' -Target $toolTarget -Message 'External dependency wording may present the dependency as a tool.'
                }
            }

            $implRef = $null
            if ($tool.PSObject.Properties.Name -contains 'implementation_ref') {
                $implRef = $tool.implementation_ref.ref
            }
            if ($tool.tool_type -eq 'external_dependency') {
                $declRef = $null
                if ($tool.PSObject.Properties.Name -contains 'declaration') {
                    $declRef = $tool.declaration.catalog_ref
                }
                if ($implRef -and $implRef -match '(?i)^(user/runtime/external_dependencies/)') {
                    Add-Finding -Severity 'INFO' -Category 'external_dependency_runtime_root_reference' -Target $toolTarget -Message ('External dependency references governed user runtime root: ' + $implRef)
                } elseif ($implRef -and $implRef -match '(?i)^((assets|crates|project)/)') {
                    Add-Finding -Severity 'FAIL' -Category 'external_dependency_forbidden_root' -Target $toolTarget -Message ('External dependency reference points to forbidden root: ' + $implRef)
                } elseif ($implRef -and $declRef -and $implRef -eq $declRef) {
                    Add-Finding -Severity 'INFO' -Category 'external_dependency_declaration_reference' -Target $toolTarget -Message 'External dependency implementation_ref points back to its declaration catalog only.'
                }
            }

            $toolJson = $tool | ConvertTo-Json -Depth 50
            if ($toolJson -match '(?i)(api[_-]?key|secret|token|password)') {
                Add-Finding -Severity 'FAIL' -Category 'credential_like_value_in_catalog' -Target $toolTarget -Message 'Credential-like terminology found in tool declaration. Human review required.'
            }
        }
    } catch {
        Add-Finding -Severity 'FAIL' -Category 'master_catalog_parse_error' -Target 'resources/tools/tools_master_catalog.json' -Message $_.Exception.Message
    }
}

$scanTargets = @(
    'resources/tools/internal/operational',
    'resources/tools/internal/llm',
    'resources/tools/external',
    'resources/tool_runtime',
    'docs/specs/tools_catalogs.md',
    'docs/specs/tools_panel.md',
    'docs/specs/tool_implementation_governance.md',
    'docs/specs/how_to_add_a_tool.md'
)

$suspiciousPatterns = @(
    @{ Pattern = '(?i)autoexecute'; Severity = 'WARN'; Category = 'suspicious_terminology'; Message = 'Suspicious autoexecute wording.' },
    @{ Pattern = '(?i)automatic execution'; Severity = 'WARN'; Category = 'suspicious_terminology'; Message = 'Automatic execution wording detected.' },
    @{ Pattern = '(?i)run from ui'; Severity = 'FAIL'; Category = 'ui_executes_tools'; Message = 'UI described as running tools.' },
    @{ Pattern = '(?i)direct execution'; Severity = 'WARN'; Category = 'suspicious_terminology'; Message = 'Direct execution wording detected.' },
    @{ Pattern = '(?i)install automatically'; Severity = 'WARN'; Category = 'suspicious_terminology'; Message = 'Automatic install wording detected.' },
    @{ Pattern = '(?i)hardcoded path'; Severity = 'WARN'; Category = 'suspicious_terminology'; Message = 'Hardcoded path wording detected.' }
)

$secretPatterns = '(?i)(api[_-]?key|secret|token|password)'
$negativeSecretContext = '(?i)(must not|mustn''t|do not|does not|not be|without)'

foreach ($target in $scanTargets) {
    $fullTarget = Join-Path $rootDir $target
    if (-not (Test-Path -LiteralPath $fullTarget)) {
        continue
    }

    $files = @()
    if ((Get-Item -LiteralPath $fullTarget) -is [System.IO.DirectoryInfo]) {
        $files = Get-ChildItem -LiteralPath $fullTarget -Recurse -File
    } else {
        $files = @(Get-Item -LiteralPath $fullTarget)
    }

    foreach ($file in $files) {
        $relative = Get-RelativePath -FullPath $file.FullName
        $lines = Get-Content -LiteralPath $file.FullName
        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = [string]$lines[$i]
            foreach ($rule in $suspiciousPatterns) {
                if ($line -match $rule.Pattern) {
                    if ($rule.Category -eq 'ui_executes_tools' -and $line -match '(?i)must not be run from ui') {
                        continue
                    }
                    Add-Finding -Severity $rule.Severity -Category $rule.Category -Target ($relative + ':' + ($i + 1)) -Message ($rule.Message + ' Snippet: ' + $line.Trim())
                }
            }
            if ($line -match $secretPatterns) {
                $contextWindow = $line
                if ($i -gt 0) {
                    $contextWindow = ([string]$lines[$i - 1]) + ' ' + $contextWindow
                }
                if ($i -gt 1) {
                    $contextWindow = ([string]$lines[$i - 2]) + ' ' + $contextWindow
                }
                if ($i -gt 2) {
                    $contextWindow = ([string]$lines[$i - 3]) + ' ' + $contextWindow
                }
                if ($line -match '(?i)^\s*-\s*store secrets\s*$') {
                    continue
                }
                if ($contextWindow -notmatch $negativeSecretContext -and $contextWindow -notmatch '(?i)must not.*store secrets') {
                    Add-Finding -Severity 'FAIL' -Category 'credential_like_text_found' -Target ($relative + ':' + ($i + 1)) -Message ('Credential-like terminology found. Snippet: ' + $line.Trim())
                }
            }
        }
    }
}

$toolImplSpec = Join-Path $rootDir 'docs/specs/tool_implementation_governance.md'
if (Test-Path -LiteralPath $toolImplSpec) {
    $toolImplText = Get-Content -LiteralPath $toolImplSpec -Raw
    if ($toolImplText -match [regex]::Escape('owner_ref')) {
        Add-Finding -Severity 'INFO' -Category 'owner_ref_reference_present' -Target 'docs/specs/tool_implementation_governance.md' -Message 'owner_ref is referenced in tool implementation governance.'
    } else {
        Add-Finding -Severity 'WARN' -Category 'owner_ref_reference_missing' -Target 'docs/specs/tool_implementation_governance.md' -Message 'owner_ref reference not found in tool implementation governance.'
    }

    if ($toolImplText -match [regex]::Escape('tool_run_manifest.json')) {
        Add-Finding -Severity 'INFO' -Category 'manifest_reference_present' -Target 'docs/specs/tool_implementation_governance.md' -Message 'tool_run_manifest.json is referenced in tool implementation governance.'
    } else {
        Add-Finding -Severity 'WARN' -Category 'manifest_reference_missing' -Target 'docs/specs/tool_implementation_governance.md' -Message 'tool_run_manifest.json reference not found in tool implementation governance.'
    }
}

$toolsPanelSpec = Join-Path $rootDir 'docs/specs/tools_panel.md'
if (Test-Path -LiteralPath $toolsPanelSpec) {
    $toolsPanelText = Get-Content -LiteralPath $toolsPanelSpec -Raw
    if ($toolsPanelText -match '(?i)emit(s)? intent only') {
        Add-Finding -Severity 'INFO' -Category 'ui_intent_only' -Target 'docs/specs/tools_panel.md' -Message 'UI intent-only boundary is declared.'
    } else {
        Add-Finding -Severity 'FAIL' -Category 'ui_intent_only_missing' -Target 'docs/specs/tools_panel.md' -Message 'Tools panel should state that UI emits intent only.'
    }

    if ($toolsPanelText -match '(?i)not manually executable') {
        Add-Finding -Severity 'INFO' -Category 'llm_manual_execution_blocked' -Target 'docs/specs/tools_panel.md' -Message 'LLM tools are declared as not manually executable.'
    } else {
        Add-Finding -Severity 'WARN' -Category 'llm_manual_execution_wording_missing' -Target 'docs/specs/tools_panel.md' -Message 'Could not confirm explicit non-manual execution wording for LLM tools.'
    }

    if ($toolsPanelText -match '(?i)raw full tool catalogs? into LLM context') {
        Add-Finding -Severity 'INFO' -Category 'raw_catalog_blocked_for_llm' -Target 'docs/specs/tools_panel.md' -Message 'Raw catalog injection into LLM context is explicitly blocked.'
    } else {
        Add-Finding -Severity 'WARN' -Category 'raw_catalog_blocking_missing' -Target 'docs/specs/tools_panel.md' -Message 'Could not confirm explicit raw-catalog blocking wording in tools panel spec.'
    }
}

$howToSpecPath = Join-Path $rootDir 'docs/specs/how_to_add_a_tool.md'
if (Test-Path -LiteralPath $howToSpecPath) {
    $howToText = Get-Content -LiteralPath $howToSpecPath -Raw
    if ($howToText -match [regex]::Escape('declared != executable')) {
        Add-Finding -Severity 'INFO' -Category 'guide_core_rule_present' -Target 'docs/specs/how_to_add_a_tool.md' -Message 'Guide core rule declared != executable is present.'
    }
}

$status = 'PASS'
if ($severityBuckets.FAIL.Count -gt 0) {
    $status = 'FAIL'
} elseif ($severityBuckets.WARN.Count -gt 0) {
    $status = 'WARN'
}

$reportLines = New-Object System.Collections.Generic.List[string]
$reportLines.Add('Tools Compliance Audit Report')
$reportLines.Add('')
$reportLines.Add(('generated_at: {0}' -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')))
$reportLines.Add(('workspace_root: {0}' -f $rootDir))
$reportLines.Add(('status: {0}' -f $status))
$reportLines.Add('')
$reportLines.Add('Rules:')
$reportLines.Add('- advisory only')
$reportLines.Add('- no automatic fixes')
$reportLines.Add('- no destructive actions')
$reportLines.Add('- do not use as automatic Codex patch input')
$reportLines.Add('')

foreach ($severity in @('FAIL','WARN','INFO')) {
    $reportLines.Add($severity)
    $reportLines.Add((''.PadLeft($severity.Length, '-')))
    if ($severityBuckets[$severity].Count -eq 0) {
        $reportLines.Add('None.')
    } else {
        foreach ($finding in $severityBuckets[$severity]) {
            $reportLines.Add(('- [{0}] {1} :: {2}' -f $finding.Category, $finding.Target, $finding.Message))
        }
    }
    $reportLines.Add('')
}

$reportLines.Add('Summary')
$reportLines.Add('-------')
$reportLines.Add(('fail_count={0}' -f $severityBuckets.FAIL.Count))
$reportLines.Add(('warn_count={0}' -f $severityBuckets.WARN.Count))
$reportLines.Add(('info_count={0}' -f $severityBuckets.INFO.Count))
$reportLines.Add(('status={0}' -f $status))

Set-Content -LiteralPath $reportPath -Value $reportLines -Encoding UTF8

Write-Host '=== TOOLS COMPLIANCE AUDIT ==='
Write-Host ('fail_count={0}' -f $severityBuckets.FAIL.Count)
Write-Host ('warn_count={0}' -f $severityBuckets.WARN.Count)
Write-Host ('info_count={0}' -f $severityBuckets.INFO.Count)
Write-Host ('status={0}' -f $status)
Write-Host ('report_path={0}' -f $reportPath)

if ($status -eq 'FAIL') {
    exit 1
}

exit 0
