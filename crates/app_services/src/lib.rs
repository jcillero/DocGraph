//! Thin application services over governed runtime verticals.
//!
//! `app_services` consumes stable vertical entrypoints such as the project
//! pipeline. It does not replace those pipelines, duplicate their critical
//! observability, or introduce UI, LLM, or tool concerns.

use project_runtime::run_project_pipeline;
use workspace_core::{ProjectRoot, WorkspaceRoot};

pub use project_runtime::{
    ManifestValidationStatus, ProjectRuntimeOutput, RuntimeEvent, RuntimeFailure, RuntimeOperation,
    RuntimeResult, RuntimeStatus,
};

/// Minimal application-service error type for project opening.
///
/// `app_services` does not redefine runtime failure taxonomy at this phase. It
/// forwards the governed vertical failure unchanged.
pub type AppServiceError = RuntimeFailure;

/// Open a project through the thin application-service boundary.
///
/// This function exists to give upper layers a small, explicit application
/// entrypoint without reimplementing the governed project pipeline.
pub fn open_project_app_service(
    workspace_root: &WorkspaceRoot,
    project_root: ProjectRoot,
) -> Result<RuntimeResult<ProjectRuntimeOutput>, AppServiceError> {
    ProjectApplicationService.open_project(workspace_root, project_root)
}

/// Minimal application-facing service over the governed project vertical.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectApplicationService;

impl ProjectApplicationService {
    /// Build a zero-configuration application service for project opening.
    pub fn new() -> Self {
        Self
    }

    /// Open a project through the single governed project pipeline.
    pub fn open_project(
        &self,
        workspace_root: &WorkspaceRoot,
        project_root: ProjectRoot,
    ) -> Result<RuntimeResult<ProjectRuntimeOutput>, AppServiceError> {
        run_project_pipeline(workspace_root, project_root)
    }
}

#[cfg(test)]
mod tests {
    use super::{open_project_app_service, ProjectApplicationService};
    use crate::{RuntimeOperation, RuntimeStatus};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workspace_core::{ProjectRoot, WorkspaceRoot};

    #[test]
    fn open_project_delegates_to_the_single_governed_pipeline() {
        let workspace_dir = unique_temp_dir("app_services_open_project");
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
        let service = ProjectApplicationService::new();

        let result = service
            .open_project(&workspace_root, project_root)
            .expect("open project result");

        assert_eq!(result.value.identity().project_id().as_str(), "alpha");
        assert_eq!(
            result.value.contract().manifest_id.as_str(),
            "manifest.alpha"
        );
        assert_eq!(result.value.surface().entries().len(), 1);
        assert_eq!(result.value.resolved_viewer_targets().len(), 1);
        assert_eq!(
            result
                .events
                .iter()
                .map(|event| (event.operation(), event.status()))
                .collect::<Vec<_>>(),
            vec![
                (RuntimeOperation::OpenProject, RuntimeStatus::Start),
                (RuntimeOperation::OpenProject, RuntimeStatus::Success),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Start),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Success),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Start,
                ),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Success,
                ),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Start),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Success),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Start),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Success),
                (RuntimeOperation::ResolveViewerTargets, RuntimeStatus::Start),
                (
                    RuntimeOperation::ResolveViewerTargets,
                    RuntimeStatus::Success,
                ),
            ]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn open_project_preserves_vertical_failure_trace_without_extra_service_events() {
        let workspace_dir = unique_temp_dir("app_services_open_project_failure");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let service = ProjectApplicationService::default();

        let failure = service
            .open_project(&workspace_root, project_root)
            .expect_err("open project failure");

        assert_eq!(
            failure
                .events()
                .iter()
                .map(|event| (event.operation(), event.status()))
                .collect::<Vec<_>>(),
            vec![
                (RuntimeOperation::OpenProject, RuntimeStatus::Start),
                (RuntimeOperation::OpenProject, RuntimeStatus::Success),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Start),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Success),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Start,
                ),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Failure,
                ),
            ]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn open_project_app_service_exposes_the_same_thin_boundary_without_new_application_shape() {
        let workspace_dir = unique_temp_dir("app_services_open_project_function");
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

        let result = open_project_app_service(&workspace_root, project_root)
            .expect("open project through app service function");

        assert_eq!(result.value.identity().project_id().as_str(), "alpha");
        assert_eq!(
            result.value.contract().manifest_id.as_str(),
            "manifest.alpha"
        );
        assert_eq!(result.value.validation().issues.len(), 0);
        assert_eq!(result.value.surface().entries().len(), 1);
        assert_eq!(result.value.resolved_viewer_targets().len(), 1);
        assert_eq!(
            result
                .events
                .iter()
                .map(|event| (event.operation(), event.status()))
                .collect::<Vec<_>>(),
            vec![
                (RuntimeOperation::OpenProject, RuntimeStatus::Start),
                (RuntimeOperation::OpenProject, RuntimeStatus::Success),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Start),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Success),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Start,
                ),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Success,
                ),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Start),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Success),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Start),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Success),
                (RuntimeOperation::ResolveViewerTargets, RuntimeStatus::Start),
                (
                    RuntimeOperation::ResolveViewerTargets,
                    RuntimeStatus::Success,
                ),
            ]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    fn create_project_workspace(workspace_dir: &PathBuf, manifest_contents: &str) -> PathBuf {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(config_dir.join("project_manifest.json"), manifest_contents)
            .expect("write manifest");
        project_dir
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
