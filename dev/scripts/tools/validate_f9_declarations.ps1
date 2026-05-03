param(
    [Parameter(Mandatory = $true)]
    [string]$WorkspaceRoot
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path -LiteralPath $WorkspaceRoot).Path
$errors = New-Object System.Collections.Generic.List[object]
$jsonData = @{}
$jsonCheckedCount = 0
$missingRequiredCount = 0
$invalidJsonCount = 0
$f9RuleErrorCount = 0

$requiredFiles = @(
    "resources/lume/lume_identity.json",
    "resources/lume/lume_symbols.json",
    "resources/lume/lume_tone_policy.json",
    "resources/lume/lume_response_policy.json",
    "resources/lume/lume_analogy_policy.json",
    "resources/lume/lume_onboarding_modal.json",
    "resources/lume/lume_onboarding_flow.json",
    "resources/i18n/es/lume_messages.ftl",
    "resources/i18n/en/lume_messages.ftl",
    "resources/preferences/app_preferences_schema.json",
    "resources/preferences/user_profile_policy.json",
    "resources/preferences/local_sandbox_policy.json",
    "resources/policy/action_policy.json",
    "resources/policy/flow_control_policy.json",
    "user/preferences/app_preferences.json",
    "user/preferences/user_profile.json",
    "user/preferences/local_sandbox_roots.json",
    "resources/runtime/runtime_locations.json",
    "resources/tools/tools_metacatalog.json",
    "resources/tools/internal/operational/tools_operational_document_catalog.json",
    "resources/tools/internal/llm/tools_llm_document_catalog.json",
    "resources/tools/external/tools_external_catalog.json",
    "resources/llm/llm_models_catalog.json",
    "resources/llm/llm_engines_catalog.json",
    "resources/llm/llm_runtime_policy.json",
    "resources/sandboxes/sandbox_actions_catalog.json",
    "resources/sandboxes/sandbox_context_schema.json",
    "resources/sandboxes/sandbox_logging_policy.json",
    "resources/storage/storage_policy.json",
    "docs/specs/action_resolution.md",
    "runtime/external/tectonic/manifest.json",
    "runtime/external/tectonic/VERSION.txt",
    "runtime/engines/llm/engines_manifest.json",
    "runtime/models/llm/models_manifest.json",
    "docs/specs/lume_interaction_model.md",
    "docs/specs/lume_onboarding_modal.md",
    "docs/specs/action_policy.md",
    "docs/specs/flow_control_policy.md",
    "docs/specs/pending_action_state.md",
    "docs/specs/runtime_layout.md",
    "docs/specs/tools_catalogs.md",
    "docs/specs/local_sandbox_context.md",
    "docs/specs/storage_policy.md"
)

$jsonFiles = $requiredFiles | Where-Object { $_.EndsWith(".json") }
$allowedTones = @("neutral", "friendly", "focused", "caution", "deescalating", "supportive_pedagogical", "light_ironic")
$expectedSymbols = @("greeting", "completed", "warning", "suggestion", "processing")
$forbiddenLumeKeys = @("emotion_state", "mood", "feelings", "anthropomorphic_state")
$i18nKeys = @(
    "lume.onboarding.greeting",
    "lume.onboarding.input_placeholder",
    "lume.onboarding.do_not_show_again",
    "lume.onboarding.video_button",
    "lume.common.close",
    "lume.common.help_title",
    "lume.common.help_subtitle",
    "lume.status.thinking",
    "lume.status.waiting",
    "lume.feedback.completed",
    "lume.feedback.warning",
    "lume.feedback.suggestion"
)

function Add-ValidationError {
    param(
        [string]$Path,
        [string]$Reason,
        [bool]$RuleError = $true
    )
    $script:errors.Add([ordered]@{ path = $Path; reason = $Reason }) | Out-Null
    Write-Host "[ERROR] ${Path}: $Reason"
    if ($RuleError) {
        $script:f9RuleErrorCount += 1
    }
}

function Join-WorkspacePath {
    param([string]$Relative)
    Join-Path $root ($Relative -replace "/", "\")
}

function Read-WorkspaceText {
    param([string]$Relative)
    Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-WorkspacePath $Relative)
}

function Get-ObjectPropertyNames {
    param($Value)
    $names = New-Object System.Collections.Generic.List[string]
    if ($null -eq $Value) {
        return $names
    }
    if ($Value -is [System.Array]) {
        foreach ($item in $Value) {
            $nestedNames = @(Get-ObjectPropertyNames $item)
            foreach ($nestedName in $nestedNames) {
                $names.Add([string]$nestedName)
            }
        }
        return $names
    }
    if ($Value.PSObject -and $Value.PSObject.Properties) {
        foreach ($prop in $Value.PSObject.Properties) {
            $names.Add($prop.Name)
            $nestedNames = @(Get-ObjectPropertyNames $prop.Value)
            foreach ($nestedName in $nestedNames) {
                $names.Add([string]$nestedName)
            }
        }
    }
    return $names
}

foreach ($relative in $requiredFiles) {
    if (-not (Test-Path -LiteralPath (Join-WorkspacePath $relative))) {
        $missingRequiredCount += 1
        Add-ValidationError $relative "missing required file" $false
    }
}

foreach ($relative in $jsonFiles) {
    $path = Join-WorkspacePath $relative
    if (Test-Path -LiteralPath $path) {
        try {
            $jsonData[$relative] = Read-WorkspaceText $relative | ConvertFrom-Json
            $jsonCheckedCount += 1
        } catch {
            $invalidJsonCount += 1
            Add-ValidationError $relative "invalid JSON: $($_.Exception.Message)" $false
        }
    }
}

$identity = $jsonData["resources/lume/lume_identity.json"]
$symbols = $jsonData["resources/lume/lume_symbols.json"]
$tones = $jsonData["resources/lume/lume_tone_policy.json"]
$response = $jsonData["resources/lume/lume_response_policy.json"]

if ($identity.app_display_name -ne "DocGraph") { Add-ValidationError "resources/lume/lume_identity.json" "app_display_name must be DocGraph" }
if ($identity.assistant_name -ne "Lume") { Add-ValidationError "resources/lume/lume_identity.json" "assistant_name must be Lume" }
if ($identity.assistant_display_name -ne "Lume") { Add-ValidationError "resources/lume/lume_identity.json" "assistant_display_name must be Lume" }
if ($identity.direct_execution_allowed -ne $false) { Add-ValidationError "resources/lume/lume_identity.json" "direct_execution_allowed must be false" }
if ($identity.user_profile_memory_requires_consent -ne $true) { Add-ValidationError "resources/lume/lume_identity.json" "user_profile_memory_requires_consent must be true" }

$symbolKeys = @($symbols.symbols.PSObject.Properties.Name | Sort-Object)
if ((Compare-Object $symbolKeys ($expectedSymbols | Sort-Object))) {
    Add-ValidationError "resources/lume/lume_symbols.json" "symbol set must be closed"
}

$toneKeys = @($tones.allowed_tones | Sort-Object)
if ((Compare-Object $toneKeys ($allowedTones | Sort-Object))) {
    Add-ValidationError "resources/lume/lume_tone_policy.json" "allowed_tones must match governed set"
}

$lumeKeys = @()
$lumeKeys += Get-ObjectPropertyNames $identity
$lumeKeys += Get-ObjectPropertyNames $symbols
$lumeKeys += Get-ObjectPropertyNames $tones
$lumeKeys += Get-ObjectPropertyNames $response
foreach ($forbidden in $forbiddenLumeKeys) {
    if ($lumeKeys -contains $forbidden) {
        Add-ValidationError "resources/lume" "forbidden Lume key present: $forbidden"
    }
}

foreach ($key in @("execute_tools", "mutate_filesystem", "trigger_workflows", "call_llm_providers")) {
    if ($response.non_execution_guarantee.$key -ne $false) {
        Add-ValidationError "resources/lume/lume_response_policy.json" "$key must be false"
    }
}

$modal = $jsonData["resources/lume/lume_onboarding_modal.json"]
$modalExpected = [ordered]@{
    ephemeral_chat = $true
    persist_chat_history = $false
    allow_file_outputs = $false
    allow_tool_execution = $false
    allow_llm_execution = $false
    show_on_startup_preference_key = "show_onboarding_modal"
}
foreach ($key in $modalExpected.Keys) {
    if ($modal.$key -ne $modalExpected[$key]) {
        Add-ValidationError "resources/lume/lume_onboarding_modal.json" "$key must be $($modalExpected[$key])"
    }
}

foreach ($relative in @("resources/i18n/es/lume_messages.ftl", "resources/i18n/en/lume_messages.ftl")) {
    if (Test-Path -LiteralPath (Join-WorkspacePath $relative)) {
        $text = Read-WorkspaceText $relative
        foreach ($key in $i18nKeys) {
            if (-not $text.Contains($key)) {
                Add-ValidationError $relative "missing i18n key $key"
            }
        }
    }
}

$runtimeLocations = $jsonData["resources/runtime/runtime_locations.json"].runtime_roots
$runtimeExpected = [ordered]@{
    external_tools = "runtime/external"
    llm_engines = "runtime/engines"
    llm_models = "runtime/models"
    cache = "runtime/cache"
    temp = "runtime/temp"
}
foreach ($key in $runtimeExpected.Keys) {
    if ($runtimeLocations.$key -ne $runtimeExpected[$key]) {
        Add-ValidationError "resources/runtime/runtime_locations.json" "$key must be $($runtimeExpected[$key])"
    }
    $value = [string]$runtimeLocations.$key
    if ($value.Contains("C:") -or $value.Contains("\") -or $value.StartsWith("/") -or $value.StartsWith("~/") -or $value.StartsWith("/home/")) {
        Add-ValidationError "resources/runtime/runtime_locations.json" "$key is not portable: $value"
    }
}

$tectonic = $jsonData["runtime/external/tectonic/manifest.json"]
if ($tectonic.installed -ne $false) { Add-ValidationError "runtime/external/tectonic/manifest.json" "installed must be false" }
if ($tectonic.status -ne "missing") { Add-ValidationError "runtime/external/tectonic/manifest.json" "status must be missing" }
if ($tectonic.binary_relative_path -ne "runtime/external/tectonic/bin/tectonic.exe") { Add-ValidationError "runtime/external/tectonic/manifest.json" "binary_relative_path must point to Tectonic placeholder" }
if ($null -ne $tectonic.source) { Add-ValidationError "runtime/external/tectonic/manifest.json" "source must be null" }
if ($null -ne $tectonic.checksum) { Add-ValidationError "runtime/external/tectonic/manifest.json" "checksum must be null" }
if ((Read-WorkspaceText "runtime/external/tectonic/VERSION.txt").Trim() -ne "not installed") {
    Add-ValidationError "runtime/external/tectonic/VERSION.txt" "VERSION.txt must contain 'not installed'"
}

$toolsSpec = Read-WorkspaceText "docs/specs/tools_catalogs.md"
foreach ($phrase in @('resources/tool_runtime/* is the current operative canonical source', 'resources/tools/* is not consumed by runtime in the current phase')) {
    if (-not $toolsSpec.Contains($phrase)) {
        Add-ValidationError "docs/specs/tools_catalogs.md" "missing canonical coexistence phrase: $phrase"
    }
}

$externalTools = $jsonData["resources/tools/external/tools_external_catalog.json"].tools
$tectonicTool = @($externalTools | Where-Object { $_.id -eq "tectonic" }) | Select-Object -First 1
if ($null -eq $tectonicTool) {
    Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic tool declaration missing"
} else {
    if ($tectonicTool.kind -ne "external_binary") { Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic kind must be external_binary" }
    if ($tectonicTool.surface -ne "runtime_dependency") { Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic surface must be runtime_dependency" }
    if ($tectonicTool.required -ne $false) { Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic required must be false" }
    if ($tectonicTool.execution_implemented -ne $false) { Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic execution_implemented must be false" }
    if (($tectonicTool.PSObject.Properties.Name -contains "runtime_status") -and $tectonicTool.runtime_status -ne "missing") {
        Add-ValidationError "resources/tools/external/tools_external_catalog.json" "tectonic runtime_status must be missing"
    }
}

$llmTools = $jsonData["resources/tools/internal/llm/tools_llm_document_catalog.json"].tools
if (@($llmTools).Count -ne 0) {
    Add-ValidationError "resources/tools/internal/llm/tools_llm_document_catalog.json" "tools must be []"
}

$llmPolicy = $jsonData["resources/llm/llm_runtime_policy.json"]
if ($llmPolicy.real_execution_enabled -ne $false) { Add-ValidationError "resources/llm/llm_runtime_policy.json" "real_execution_enabled must be false" }
foreach ($mode in @("local", "cloud", "auto")) {
    if (-not (@($llmPolicy.allowed_modes) -contains $mode)) {
        Add-ValidationError "resources/llm/llm_runtime_policy.json" "allowed_modes missing $mode"
    }
}
foreach ($item in @("explicit_args", "user_preferences", "system_config", "defaults")) {
    if (-not (@($llmPolicy.resolution_precedence) -contains $item)) {
        Add-ValidationError "resources/llm/llm_runtime_policy.json" "resolution_precedence missing $item"
    }
}

$sandboxRoots = $jsonData["user/preferences/local_sandbox_roots.json"]
if (@($sandboxRoots.sandboxes).Count -ne 0) {
    Add-ValidationError "user/preferences/local_sandbox_roots.json" "sandboxes must be []"
}

$actions = @{}
foreach ($action in $jsonData["resources/sandboxes/sandbox_actions_catalog.json"].actions) {
    $actions[$action.id] = $action.enabled
}
$expectedActions = [ordered]@{
    read_tree = $true
    dry_run_plan = $true
    rename_files = $false
    move_files = $false
    create_folders = $false
    delete_files = $false
    generate_sandbox_index = $false
}
foreach ($key in $expectedActions.Keys) {
    if ($actions[$key] -ne $expectedActions[$key]) {
        Add-ValidationError "resources/sandboxes/sandbox_actions_catalog.json" "$key.enabled must be $($expectedActions[$key])"
    }
}

$sandboxPolicy = $jsonData["resources/preferences/local_sandbox_policy.json"]
$optionalSandboxChecks = [ordered]@{
    allow_mutation_outside_sandbox = $false
    must_be_empty_on_registration = $true
    must_be_external_to_app_workspace = $true
}
foreach ($key in $optionalSandboxChecks.Keys) {
    if (($sandboxPolicy.PSObject.Properties.Name -contains $key) -and $sandboxPolicy.$key -ne $optionalSandboxChecks[$key]) {
        Add-ValidationError "resources/preferences/local_sandbox_policy.json" "$key must be $($optionalSandboxChecks[$key])"
    }
}

$storage = $jsonData["resources/storage/storage_policy.json"]
if ($storage.model -ne "file_based") { Add-ValidationError "resources/storage/storage_policy.json" "model must be file_based" }
if ($storage.deduplication -ne "checksum") { Add-ValidationError "resources/storage/storage_policy.json" "deduplication must be checksum" }
if ($storage.index_strategy -ne "regenerable_jsonl") { Add-ValidationError "resources/storage/storage_policy.json" "index_strategy must be regenerable_jsonl" }
if ($storage.database.enabled -ne $false) { Add-ValidationError "resources/storage/storage_policy.json" "database.enabled must be false" }
if ($storage.database.sqlite -ne "future_optional") { Add-ValidationError "resources/storage/storage_policy.json" "sqlite must not be active" }
if ($storage.database.oxigraph -ne "future_optional") { Add-ValidationError "resources/storage/storage_policy.json" "oxigraph must not be active" }

$actionPolicy = $jsonData["resources/policy/action_policy.json"]
if ($actionPolicy.real_execution_enabled -ne $false) { Add-ValidationError "resources/policy/action_policy.json" "real_execution_enabled must be false" }
foreach ($level in @("read_only", "low", "medium", "high", "forbidden")) {
    if (-not (@($actionPolicy.risk_levels) -contains $level)) {
        Add-ValidationError "resources/policy/action_policy.json" "risk_levels missing $level"
    }
}
foreach ($decision in @("allow", "confirm", "reject")) {
    if (-not (@($actionPolicy.decisions) -contains $decision)) {
        Add-ValidationError "resources/policy/action_policy.json" "decisions missing $decision"
    }
}
if ($actionPolicy.human_in_the_loop.enabled -ne $true) { Add-ValidationError "resources/policy/action_policy.json" "human_in_the_loop.enabled must be true" }
if ($actionPolicy.forbidden_cannot_be_overridden -ne $true) { Add-ValidationError "resources/policy/action_policy.json" "forbidden_cannot_be_overridden must be true" }
if ($actionPolicy.document_rules.SOURCE -ne "readonly") { Add-ValidationError "resources/policy/action_policy.json" "SOURCE must be readonly" }
if ($actionPolicy.document_rules.DERIVED -ne "readonly") { Add-ValidationError "resources/policy/action_policy.json" "DERIVED must be readonly" }
if ($actionPolicy.document_rules.ARTIFACT -ne "confirm_before_mutation") { Add-ValidationError "resources/policy/action_policy.json" "ARTIFACT must require confirmation before mutation" }
foreach ($blocked in @("mutate_source_document", "mutate_derived_document", "llm_direct_filesystem_write", "execute_unregistered_tool")) {
    if (-not (@($actionPolicy.blocked_actions) -contains $blocked)) {
        Add-ValidationError "resources/policy/action_policy.json" "blocked_actions missing $blocked"
    }
}

$flowPolicy = $jsonData["resources/policy/flow_control_policy.json"]
if ($flowPolicy.real_execution_enabled -ne $false) { Add-ValidationError "resources/policy/flow_control_policy.json" "real_execution_enabled must be false" }
if ($flowPolicy.confirmation_rules.confirm_readonly_actions -ne $false) { Add-ValidationError "resources/policy/flow_control_policy.json" "confirm_readonly_actions must be false" }
if ($flowPolicy.confirmation_rules.confirm_low_risk_derivations -ne $false) { Add-ValidationError "resources/policy/flow_control_policy.json" "confirm_low_risk_derivations must be false" }
foreach ($reason in @("risk_escalation", "artifact_mutation", "external_execution", "ambiguous_target", "irreversible_action")) {
    if (-not (@($flowPolicy.confirmation_rules.confirm_only_for) -contains $reason)) {
        Add-ValidationError "resources/policy/flow_control_policy.json" "confirm_only_for missing $reason"
    }
}
if ($flowPolicy.confirmation_rules.max_confirmations_per_logical_flow -ne 1) { Add-ValidationError "resources/policy/flow_control_policy.json" "max_confirmations_per_logical_flow must be 1" }
if ($flowPolicy.confirmation_rules.group_pending_actions -ne $true) { Add-ValidationError "resources/policy/flow_control_policy.json" "group_pending_actions must be true" }
if ($flowPolicy.confirmation_rules.prefer_inline_confirmation -ne $true) { Add-ValidationError "resources/policy/flow_control_policy.json" "prefer_inline_confirmation must be true" }
foreach ($modalReason in @("high_risk", "external_execution", "irreversible_action")) {
    if (-not (@($flowPolicy.confirmation_rules.modal_only_for) -contains $modalReason)) {
        Add-ValidationError "resources/policy/flow_control_policy.json" "modal_only_for missing $modalReason"
    }
}

$pendingSpec = Read-WorkspaceText "docs/specs/pending_action_state.md"
foreach ($phrase in @('Execute always references a specific `action_id`', 'The Execute button never means', 'does not imply')) {
    if (-not $pendingSpec.Contains($phrase)) {
        Add-ValidationError "docs/specs/pending_action_state.md" "missing pending action guard phrase: $phrase"
    }
}

$resolutionSpec = Read-WorkspaceText "docs/specs/action_resolution.md"
foreach ($phrase in @(
    'ActionRequest -> ContextResolution -> RiskClassification -> PolicyEvaluation -> ResolutionDecision -> Interaction -> Trace',
    'execution_enabled=false',
    '`ALLOW` does not mean invisible execution',
    'forbidden produces `BLOCKED`, not ordinary `REJECTED`',
    'human confirmation does not revive `Stale` or `Expired` actions',
    'every resolution must produce minimum trace'
)) {
    if (-not $resolutionSpec.Contains($phrase)) {
        Add-ValidationError "docs/specs/action_resolution.md" "missing action resolution guard phrase: $phrase"
    }
}

$status = if ($errors.Count -eq 0) { "OK" } else { "ERROR" }
$errorArray = @()
foreach ($validationError in $errors) {
    $errorArray += [ordered]@{
        path = $validationError.path
        reason = $validationError.reason
    }
}
$report = [ordered]@{
    status = $status
    json_checked_count = $jsonCheckedCount
    missing_required_count = $missingRequiredCount
    invalid_json_count = $invalidJsonCount
    f9_rule_error_count = $f9RuleErrorCount
    errors = $errorArray
}
$reportPath = Join-WorkspacePath "user/output/validate_f9_declarations_report.json"
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $reportPath) | Out-Null
($report | ConvertTo-Json -Depth 8) | Set-Content -Encoding UTF8 -LiteralPath $reportPath

Write-Host "json_checked_count=$jsonCheckedCount"
Write-Host "missing_required_count=$missingRequiredCount"
Write-Host "invalid_json_count=$invalidJsonCount"
Write-Host "f9_rule_error_count=$f9RuleErrorCount"

if ($errors.Count -gt 0) {
    exit 1
}

Write-Host "[OK] validate_f9_declarations=true"
exit 0
