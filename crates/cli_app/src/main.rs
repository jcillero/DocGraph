//! Thin CLI consumption over the current Rust sandbox boundaries.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use app_services::{
    open_project_app_service, ManifestValidationStatus, ProjectRuntimeOutput, RuntimeEvent,
    RuntimeFailure, RuntimeResult,
};
use core_domain::{ErrorDomain, PortableAppError};
use tool_runtime::{
    execute_text_measure, load_effective_meta_catalog, resolve_catalog_request, CatalogEntryKind,
    CatalogSelectionRequest, TextMeasureRequest, ToolExecutionOutcome, ToolRuntimeResult,
    ToolRuntimeRunner,
};
use workspace_core::{ProjectRoot, WorkspaceRoot};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), PortableAppError> {
    let cli = CliArgs::parse(env::args().skip(1))?;

    match cli.command {
        CliCommand::OpenProject { workspace, project } => {
            let workspace_root = WorkspaceRoot::new(PathBuf::from(&workspace))?;
            let project_root = ProjectRoot::new(&workspace_root, PathBuf::from(&project))?;
            let pipeline = match open_project_command(&workspace_root, project_root) {
                Ok(result) => result,
                Err(failure) => return print_runtime_failure(failure),
            };

            print_project_run_summary(&workspace_root, &pipeline.value, &pipeline.events);
            Ok(())
        }
        CliCommand::AcceptTool {
            workspace,
            project,
            catalog,
            entry,
        } => {
            let workspace_root = WorkspaceRoot::new(PathBuf::from(&workspace))?;
            let project_root = ProjectRoot::new(&workspace_root, PathBuf::from(&project))?;
            let result = accept_tool_command(&project_root, &catalog, &entry)?;

            print_tool_run_summary(&result);
            Ok(())
        }
        CliCommand::TextMeasure {
            workspace,
            project,
            mode,
            text,
            owner_ref,
        } => {
            let workspace_root = WorkspaceRoot::new(PathBuf::from(&workspace))?;
            let _project_root = ProjectRoot::new(&workspace_root, PathBuf::from(&project))?;
            let request = TextMeasureRequest::new(mode, text, owner_ref);
            let result = execute_text_measure_command(&workspace_root, &request)?;

            print_text_measure_run_summary(&result);
            Ok(())
        }
    }
}

fn open_project_command(
    workspace_root: &WorkspaceRoot,
    project_root: ProjectRoot,
) -> Result<RuntimeResult<ProjectRuntimeOutput>, RuntimeFailure> {
    open_project_app_service(workspace_root, project_root)
}

fn accept_tool_command(
    project_root: &ProjectRoot,
    catalog_id: &str,
    entry_id: &str,
) -> Result<ToolRuntimeResult, PortableAppError> {
    let effective_meta_catalog = load_effective_meta_catalog(project_root)
        .map_err(|error| cli_error("tool_catalog_load_failed", error.to_string()))?;
    let request = CatalogSelectionRequest::new(catalog_id, entry_id);
    let resolved_entry = resolve_catalog_request(&effective_meta_catalog, &request)
        .map_err(|error| cli_error("tool_catalog_resolve_failed", error.to_string()))?;

    ToolRuntimeRunner::new()
        .dispatch(&resolved_entry)
        .map_err(|error| cli_error("tool_dispatch_failed", error.to_string()))
}

fn execute_text_measure_command(
    workspace_root: &WorkspaceRoot,
    request: &TextMeasureRequest,
) -> Result<tool_runtime::TextMeasureRunOutcome, PortableAppError> {
    execute_text_measure(workspace_root, request)
        .map_err(|error| cli_error("text_measure_failed", error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CliCommand {
    OpenProject {
        workspace: String,
        project: String,
    },
    AcceptTool {
        workspace: String,
        project: String,
        catalog: String,
        entry: String,
    },
    TextMeasure {
        workspace: String,
        project: String,
        mode: String,
        text: String,
        owner_ref: String,
    },
}

impl CliArgs {
    fn parse<I>(args: I) -> Result<Self, PortableAppError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut mode = CliMode::OpenProject;
        let mut workspace: Option<String> = None;
        let mut project: Option<String> = None;
        let mut catalog: Option<String> = None;
        let mut entry: Option<String> = None;
        let mut mode_value: Option<String> = None;
        let mut text: Option<String> = None;
        let mut owner_ref: Option<String> = None;
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "open-project" => mode = CliMode::OpenProject,
                "tool-accept" => mode = CliMode::AcceptTool,
                "tool-text-measure" => mode = CliMode::TextMeasure,
                "--workspace" => {
                    workspace = Some(next_required_value(&mut iter, "--workspace")?);
                }
                "--project" => {
                    project = Some(next_required_value(&mut iter, "--project")?);
                }
                "--catalog" => {
                    catalog = Some(next_required_value(&mut iter, "--catalog")?);
                }
                "--entry" => {
                    entry = Some(next_required_value(&mut iter, "--entry")?);
                }
                "--mode" => {
                    mode_value = Some(next_required_value(&mut iter, "--mode")?);
                }
                "--text" => {
                    text = Some(next_required_value(&mut iter, "--text")?);
                }
                "--owner-ref" => {
                    owner_ref = Some(next_required_value(&mut iter, "--owner-ref")?);
                }
                "--help" | "-h" => return Err(cli_error("cli_usage", usage_text())),
                _ => {
                    return Err(cli_error(
                        "unknown_argument",
                        format!("unknown argument: {arg}"),
                    ));
                }
            }
        }

        let workspace = workspace.ok_or_else(|| {
            cli_error(
                "missing_workspace",
                "missing required argument: --workspace <path>",
            )
        })?;
        let project = project.ok_or_else(|| {
            cli_error(
                "missing_project",
                "missing required argument: --project <path>",
            )
        })?;

        let command = match mode {
            CliMode::OpenProject => CliCommand::OpenProject { workspace, project },
            CliMode::AcceptTool => CliCommand::AcceptTool {
                workspace,
                project,
                catalog: catalog.ok_or_else(|| {
                    cli_error(
                        "missing_catalog",
                        "missing required argument: --catalog <catalog-id>",
                    )
                })?,
                entry: entry.ok_or_else(|| {
                    cli_error(
                        "missing_entry",
                        "missing required argument: --entry <entry-id>",
                    )
                })?,
            },
            CliMode::TextMeasure => CliCommand::TextMeasure {
                workspace,
                project,
                mode: mode_value.ok_or_else(|| {
                    cli_error("missing_mode", "missing required argument: --mode <mode>")
                })?,
                text: text.ok_or_else(|| {
                    cli_error("missing_text", "missing required argument: --text <text>")
                })?,
                owner_ref: owner_ref.ok_or_else(|| {
                    cli_error(
                        "missing_owner_ref",
                        "missing required argument: --owner-ref <owner-ref>",
                    )
                })?,
            },
        };

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliMode {
    OpenProject,
    AcceptTool,
    TextMeasure,
}

fn next_required_value<I>(iter: &mut I, flag: &'static str) -> Result<String, PortableAppError>
where
    I: Iterator<Item = String>,
{
    iter.next().ok_or_else(|| {
        cli_error(
            "missing_argument_value",
            format!("missing value after {flag}"),
        )
    })
}

fn print_project_run_summary(
    workspace_root: &WorkspaceRoot,
    output: &ProjectRuntimeOutput,
    runtime_events: &[RuntimeEvent],
) {
    let project_identity = output.identity();
    let validation = output.validation();
    let surface_model = output.surface();
    let resolved_viewer_targets = output.resolved_viewer_targets();

    println!(
        "validation_status={}",
        validation_status_text(validation.status)
    );
    println!("validation_issue_count={}", validation.issues.len());
    println!("surface={}", surface_model.surface().as_str());
    println!("surface_entries={}", surface_model.entries().len());
    println!("resolved_viewer_targets={}", resolved_viewer_targets.len());
    println!("project_id={}", project_identity.project_id());
    println!("workspace_root={}", workspace_root.as_path().display());
    println!(
        "project_root={}",
        project_identity.project_root().as_path().display()
    );
    println!("manifest_id={}", output.contract().manifest_id);
    println!(
        "manifest_source={}",
        project_identity
            .manifest_source()
            .path()
            .as_path()
            .display()
    );

    for (index, entry) in surface_model.entries().iter().enumerate() {
        let ordinal = index + 1;
        println!("surface_entry_{ordinal}_ref={}", entry.target_ref());
        println!(
            "surface_entry_{ordinal}_kind={}",
            entry.target_kind().as_str()
        );
        println!(
            "surface_entry_{ordinal}_path={}",
            entry
                .relative_path()
                .map(|path| path.as_path().display().to_string())
                .unwrap_or_else(|| "-".to_owned())
        );
        println!(
            "surface_entry_{ordinal}_label={}",
            entry.label().unwrap_or("-")
        );
    }

    for (index, target) in resolved_viewer_targets.iter().enumerate() {
        let ordinal = index + 1;
        println!(
            "resolved_viewer_target_{ordinal}_ref={}",
            target.target_ref()
        );
        println!(
            "resolved_viewer_target_{ordinal}_kind={}",
            target.target_kind().as_str()
        );
        println!(
            "resolved_viewer_target_{ordinal}_path={}",
            target.resolved_path().display()
        );
        println!(
            "resolved_viewer_target_{ordinal}_label={}",
            target.label().unwrap_or("-")
        );
    }

    print_runtime_events(runtime_events);
}

fn print_tool_run_summary(result: &ToolRuntimeResult) {
    println!("tool_catalog_id={}", result.catalog_id());
    println!("tool_entry_id={}", result.entry_id());
    println!(
        "tool_entry_kind={}",
        tool_entry_kind_text(result.entry_kind())
    );
    println!("tool_outcome={}", tool_outcome_text(result.outcome()));
}

fn print_text_measure_run_summary(result: &tool_runtime::TextMeasureRunOutcome) {
    println!("tool_id=text.measure");
    println!("tool_outcome=completed");
    println!("run_id={}", result.run_id());
    println!("run_directory={}", result.run_directory().display());
    println!("char_count={}", result.result().char_count);
    println!("byte_count={}", result.result().byte_count);
    println!("line_count={}", result.result().line_count);
    println!("word_count={}", result.result().word_count);
}

fn print_runtime_events(runtime_events: &[RuntimeEvent]) {
    for (index, event) in runtime_events.iter().enumerate() {
        let ordinal = index + 1;
        println!(
            "runtime_event_{ordinal}_operation={}",
            event.operation().as_str()
        );
        println!("runtime_event_{ordinal}_status={}", event.status().as_str());
        println!(
            "runtime_event_{ordinal}_metadata={}",
            event.metadata().unwrap_or("-")
        );
    }
}

fn print_runtime_failure(failure: RuntimeFailure) -> Result<(), PortableAppError> {
    let (error, failure_events) = failure.into_parts();
    print_runtime_events(&failure_events);
    Err(error)
}

fn validation_status_text(status: ManifestValidationStatus) -> &'static str {
    match status {
        ManifestValidationStatus::Valid => "valid",
        ManifestValidationStatus::Invalid => "invalid",
    }
}

fn tool_entry_kind_text(kind: CatalogEntryKind) -> &'static str {
    match kind {
        CatalogEntryKind::Tool => "tool",
        CatalogEntryKind::Recipe => "recipe",
    }
}

fn tool_outcome_text(outcome: ToolExecutionOutcome) -> &'static str {
    match outcome {
        ToolExecutionOutcome::Accepted => "accepted",
    }
}

fn usage_text() -> &'static str {
    "usage:\n  cli_app --workspace <path> --project <path>\n  cli_app open-project --workspace <path> --project <path>\n  cli_app tool-accept --workspace <path> --project <path> --catalog <catalog-id> --entry <entry-id>\n  cli_app tool-text-measure --workspace <path> --project <path> --mode string --text <text> --owner-ref <owner-ref>"
}

fn cli_error(code: &'static str, message: impl Into<String>) -> PortableAppError {
    PortableAppError::new(ErrorDomain::CoreDomain, code, message)
}

#[cfg(test)]
mod tests {
    use super::{
        accept_tool_command, execute_text_measure_command, open_project_command, CliArgs,
        CliCommand,
    };
    use app_services::RuntimeOperation;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tool_runtime::{CatalogEntryKind, TextMeasureRequest, ToolExecutionOutcome};
    use workspace_core::{ProjectRoot, WorkspaceRoot};

    #[test]
    fn cli_project_flow_consumes_app_services_boundary() {
        let workspace_dir = unique_temp_dir("cli_project_flow");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");

        let result = open_project_command(&workspace_root, project_root).expect("project result");

        assert_eq!(result.value.identity().project_id().as_str(), "alpha");
        assert_eq!(
            result.value.contract().manifest_id.as_str(),
            "manifest.alpha"
        );
        assert_eq!(result.events[0].operation(), RuntimeOperation::OpenProject);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn cli_tool_flow_consumes_minimal_tool_runner_for_tool() {
        let workspace_dir = unique_temp_dir("cli_tool_flow_tool");
        let project_root = create_project_root(&workspace_dir);

        let result =
            accept_tool_command(&project_root, "project", "manifest").expect("tool result");

        assert_eq!(result.catalog_id(), "project");
        assert_eq!(result.entry_id(), "manifest");
        assert_eq!(result.entry_kind(), CatalogEntryKind::Tool);
        assert_eq!(result.outcome(), ToolExecutionOutcome::Accepted);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn cli_tool_flow_consumes_minimal_tool_runner_for_recipe() {
        let workspace_dir = unique_temp_dir("cli_tool_flow_recipe");
        let project_root = create_project_root(&workspace_dir);

        let result = accept_tool_command(&project_root, "project", "project_summary")
            .expect("recipe result");

        assert_eq!(result.catalog_id(), "project");
        assert_eq!(result.entry_id(), "project_summary");
        assert_eq!(result.entry_kind(), CatalogEntryKind::Recipe);
        assert_eq!(result.outcome(), ToolExecutionOutcome::Accepted);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn cli_text_measure_flow_persists_governed_output() {
        let workspace_dir = unique_temp_dir("cli_text_measure_flow");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::new("string", "hello world", "chat://alpha");

        let result =
            execute_text_measure_command(&workspace_root, &request).expect("text measure result");

        assert!(result.run_directory().join("result.json").exists());
        assert!(result.run_directory().join("tool_run_manifest.json").exists());
        assert_eq!(result.result().word_count, 2);
        assert!(project_root.as_path().starts_with(workspace_root.as_path()));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn cli_args_separate_project_and_tool_flows() {
        let project_args = CliArgs::parse([
            "--workspace".to_owned(),
            "workspace".to_owned(),
            "--project".to_owned(),
            "workspace/user/projects/alpha".to_owned(),
        ])
        .expect("project args");
        let tool_args = CliArgs::parse([
            "tool-accept".to_owned(),
            "--workspace".to_owned(),
            "workspace".to_owned(),
            "--project".to_owned(),
            "workspace/user/projects/alpha".to_owned(),
            "--catalog".to_owned(),
            "project".to_owned(),
            "--entry".to_owned(),
            "manifest".to_owned(),
        ])
        .expect("tool args");

        assert_eq!(
            project_args.command,
            CliCommand::OpenProject {
                workspace: "workspace".to_owned(),
                project: "workspace/user/projects/alpha".to_owned(),
            }
        );
        assert_eq!(
            tool_args.command,
            CliCommand::AcceptTool {
                workspace: "workspace".to_owned(),
                project: "workspace/user/projects/alpha".to_owned(),
                catalog: "project".to_owned(),
                entry: "manifest".to_owned(),
            }
        );
        let text_measure_args = CliArgs::parse([
            "tool-text-measure".to_owned(),
            "--workspace".to_owned(),
            "workspace".to_owned(),
            "--project".to_owned(),
            "workspace/user/projects/alpha".to_owned(),
            "--mode".to_owned(),
            "string".to_owned(),
            "--text".to_owned(),
            "hello".to_owned(),
            "--owner-ref".to_owned(),
            "chat://alpha".to_owned(),
        ])
        .expect("text measure args");

        assert_eq!(
            text_measure_args.command,
            CliCommand::TextMeasure {
                workspace: "workspace".to_owned(),
                project: "workspace/user/projects/alpha".to_owned(),
                mode: "string".to_owned(),
                text: "hello".to_owned(),
                owner_ref: "chat://alpha".to_owned(),
            }
        );
    }

    fn create_project_workspace(workspace_dir: &PathBuf, manifest_contents: &str) -> PathBuf {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(config_dir.join("project_manifest.json"), manifest_contents)
            .expect("write manifest");
        project_dir
    }

    fn create_project_root(workspace_dir: &PathBuf) -> ProjectRoot {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        ProjectRoot::new(&workspace_root, project_dir).expect("project root")
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        path.push(format!("rust_portable_app_{prefix}_{nanos}"));
        path
    }
}
