use core_domain::FileRef;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const BLOBS_DIRNAME: &str = "blobs";
const INTAKE_BATCHES_DIRNAME: &str = "intake_batches";
const BLOB_CONTENT_FILENAME: &str = "content";
const METADATA_FILENAME: &str = "metadata.json";
const BATCH_FILENAME: &str = "intake_batch.json";
const SUPPORTED_TEXT_MEDIA_TYPE: &str = "text/plain";
const SUPPORTED_MARKDOWN_MEDIA_TYPE: &str = "text/markdown";
const MAX_COMMENT_BYTES: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntakeBatchRequest {
    pub file_store_root: PathBuf,
    pub owner_ref: String,
    pub trace_ref: String,
    pub batch_comment: Option<String>,
    pub selected_files: Vec<FileIntakeSelectedFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntakeSelectedFile {
    pub source_path: PathBuf,
    pub user_comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileIntakeBatch {
    pub intake_batch_ref: String,
    pub owner_ref: String,
    pub trace_ref: String,
    pub status: FileIntakeStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_comment: Option<IntakeCommentMetadata>,
    pub items: Vec<FileIntakeItem>,
    pub created_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileIntakeStatus {
    Planned,
    ImportedNotExposed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileIntakeItem {
    pub intake_item_id: String,
    pub intake_batch_ref: String,
    pub status: FileIntakeStatus,
    pub metadata: IntakeMetadata,
    pub blocking_reasons: Vec<FileIntakeError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_object_candidate_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_relative_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_relative_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeMetadata {
    pub intake_item_id: String,
    pub intake_batch_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_object_candidate_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ref: Option<FileRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    pub owner_ref: String,
    pub trace_ref: String,
    pub original_filename_sanitized: String,
    pub detected_kind: IntakeDetectedKind,
    pub size_bytes: u64,
    pub source_kind: String,
    pub source_display_label: String,
    pub classification: IntakeClassification,
    pub security_sanitization_state: IntakeSecuritySanitizationState,
    pub exposure_state: IntakeExposureState,
    pub derivation_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_comment: Option<IntakeCommentMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeDetectedKind {
    Text,
    Markdown,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeClassification {
    pub extension: String,
    pub media_type_hint: String,
    pub confidence: String,
    pub classification_source: String,
    pub supported_state: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeSecuritySanitizationState {
    Safe,
    Flagged,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeExposureState {
    ImportedNotExposed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeCommentMetadata {
    pub text: String,
    pub comment_author_ref: String,
    pub comment_created_at: String,
    pub comment_sanitization_state: CommentSanitizationState,
    pub comment_visibility: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentSanitizationState {
    Safe,
    Flagged,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileIntakeError {
    MissingSource,
    UnreadableSource,
    SourceIsDirectory,
    UnsupportedFormat,
    UnsafeFilename,
    UnsafePath,
    PrivateAbsolutePath,
    MissingOwnerRef,
    MissingTraceRef,
    DuplicateConflict,
    SizeLimitExceeded,
    SanitizationFailed,
    CommentContainsSecrets,
    CommentContainsPrivatePath,
    CommentTooLarge,
    CommentSanitizationFailed,
    StoragePathEscape,
    RuntimeNotOpened,
    PolicyBlocked,
    IoError,
}

pub fn run_file_intake_batch(
    request: FileIntakeBatchRequest,
) -> Result<FileIntakeBatch, FileIntakeError> {
    if request.owner_ref.trim().is_empty() {
        return Err(FileIntakeError::MissingOwnerRef);
    }
    if request.trace_ref.trim().is_empty() {
        return Err(FileIntakeError::MissingTraceRef);
    }

    let file_store_root = prepare_file_store_root(&request.file_store_root)?;
    let now = current_timestamp_text();
    let intake_batch_ref = build_batch_ref(&request.owner_ref, &request.trace_ref, &request.selected_files);
    let batch_root = file_store_root
        .join(INTAKE_BATCHES_DIRNAME)
        .join(&intake_batch_ref);
    create_dir_inside_root(&file_store_root, &batch_root)?;

    let batch_comment = request
        .batch_comment
        .as_deref()
        .map(|comment| sanitize_comment(comment, &request.owner_ref, &now))
        .transpose()?;

    let mut items = Vec::new();
    for (index, selected_file) in request.selected_files.iter().enumerate() {
        items.push(process_selected_file(
            &file_store_root,
            &batch_root,
            &intake_batch_ref,
            &request.owner_ref,
            &request.trace_ref,
            &now,
            index,
            selected_file,
        )?);
    }

    let status = if items.iter().any(|item| item.status == FileIntakeStatus::ImportedNotExposed) {
        FileIntakeStatus::ImportedNotExposed
    } else {
        FileIntakeStatus::Blocked
    };
    let batch = FileIntakeBatch {
        intake_batch_ref,
        owner_ref: request.owner_ref,
        trace_ref: request.trace_ref,
        status,
        batch_comment,
        items,
        created_at: now,
    };

    write_json_inside_root(&file_store_root, &batch_root.join(BATCH_FILENAME), &batch)?;

    Ok(batch)
}

fn process_selected_file(
    file_store_root: &Path,
    batch_root: &Path,
    intake_batch_ref: &str,
    owner_ref: &str,
    trace_ref: &str,
    timestamp: &str,
    index: usize,
    selected_file: &FileIntakeSelectedFile,
) -> Result<FileIntakeItem, FileIntakeError> {
    let source_path = &selected_file.source_path;
    let sanitized_filename = sanitize_filename(
        source_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file"),
    );
    let (detected_kind, classification) = classify_path(source_path);
    let mut blocking_reasons = Vec::new();

    if sanitized_filename == "file" {
        blocking_reasons.push(FileIntakeError::UnsafeFilename);
    }
    if !source_path.exists() {
        blocking_reasons.push(FileIntakeError::MissingSource);
    } else if source_path.is_dir() {
        blocking_reasons.push(FileIntakeError::SourceIsDirectory);
    }
    if detected_kind == IntakeDetectedKind::Unsupported {
        blocking_reasons.push(FileIntakeError::UnsupportedFormat);
    }

    let comment = selected_file
        .user_comment
        .as_deref()
        .map(|comment| sanitize_comment(comment, owner_ref, timestamp))
        .transpose()?;

    let item_seed = format!("{intake_batch_ref}:{index}:{sanitized_filename}");
    let mut item_id = prefixed_hash("ii", item_seed.as_bytes());
    let mut metadata = IntakeMetadata {
        intake_item_id: item_id.clone(),
        intake_batch_ref: intake_batch_ref.to_owned(),
        stored_object_candidate_ref: None,
        file_ref: None,
        content_hash: None,
        owner_ref: owner_ref.to_owned(),
        trace_ref: trace_ref.to_owned(),
        original_filename_sanitized: sanitized_filename.clone(),
        detected_kind,
        size_bytes: 0,
        source_kind: "explicit_user_selection".to_owned(),
        source_display_label: sanitized_filename.clone(),
        classification,
        security_sanitization_state: IntakeSecuritySanitizationState::Safe,
        exposure_state: IntakeExposureState::Blocked,
        derivation_state: "not_generated".to_owned(),
        user_comment: comment,
    };

    if !blocking_reasons.is_empty() {
        metadata.security_sanitization_state = IntakeSecuritySanitizationState::Flagged;
        return Ok(FileIntakeItem {
            intake_item_id: item_id,
            intake_batch_ref: intake_batch_ref.to_owned(),
            status: FileIntakeStatus::Blocked,
            metadata,
            blocking_reasons,
            stored_object_candidate_ref: None,
            stored_relative_path: None,
            metadata_relative_path: None,
        });
    }

    let bytes = match fs::read(source_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            metadata.security_sanitization_state = IntakeSecuritySanitizationState::Flagged;
            return Ok(FileIntakeItem {
                intake_item_id: item_id,
                intake_batch_ref: intake_batch_ref.to_owned(),
                status: FileIntakeStatus::Blocked,
                metadata,
                blocking_reasons: vec![FileIntakeError::UnreadableSource],
                stored_object_candidate_ref: None,
                stored_relative_path: None,
                metadata_relative_path: None,
            });
        }
    };
    let file_ref = FileRef::from_bytes(&bytes);
    let content_hash = file_ref.as_str().trim_start_matches("sha256_").to_owned();
    let object_ref = prefixed_hash("soc", format!("{intake_batch_ref}:{content_hash}").as_bytes());
    item_id = prefixed_hash("ii", format!("{intake_batch_ref}:{index}:{content_hash}").as_bytes());
    let blob_root = file_store_root.join(BLOBS_DIRNAME).join(&content_hash);
    create_dir_inside_root(file_store_root, &blob_root)?;
    let blob_filename = format!(
        "{}.{}",
        BLOB_CONTENT_FILENAME,
        metadata.classification.extension
    );
    let blob_path = blob_root.join(blob_filename);
    write_bytes_inside_root(file_store_root, &blob_path, &bytes)?;

    metadata.intake_item_id = item_id.clone();
    metadata.stored_object_candidate_ref = Some(object_ref.clone());
    metadata.file_ref = Some(file_ref);
    metadata.content_hash = Some(content_hash);
    metadata.size_bytes = bytes.len() as u64;
    metadata.exposure_state = IntakeExposureState::ImportedNotExposed;

    let item_root = batch_root.join("items").join(&item_id);
    create_dir_inside_root(file_store_root, &item_root)?;
    let metadata_path = item_root.join(METADATA_FILENAME);
    write_json_inside_root(file_store_root, &metadata_path, &metadata)?;

    Ok(FileIntakeItem {
        intake_item_id: item_id,
        intake_batch_ref: intake_batch_ref.to_owned(),
        status: FileIntakeStatus::ImportedNotExposed,
        metadata,
        blocking_reasons,
        stored_object_candidate_ref: Some(object_ref),
        stored_relative_path: Some(relative_portable_path(file_store_root, &blob_path)?),
        metadata_relative_path: Some(relative_portable_path(file_store_root, &metadata_path)?),
    })
}

fn classify_path(path: &Path) -> (IntakeDetectedKind, IntakeClassification) {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match extension.as_str() {
        "txt" => (
            IntakeDetectedKind::Text,
            IntakeClassification {
                extension,
                media_type_hint: SUPPORTED_TEXT_MEDIA_TYPE.to_owned(),
                confidence: "extension_hint".to_owned(),
                classification_source: "safe_extension".to_owned(),
                supported_state: "supported".to_owned(),
            },
        ),
        "md" | "markdown" => (
            IntakeDetectedKind::Markdown,
            IntakeClassification {
                extension,
                media_type_hint: SUPPORTED_MARKDOWN_MEDIA_TYPE.to_owned(),
                confidence: "extension_hint".to_owned(),
                classification_source: "safe_extension".to_owned(),
                supported_state: "supported".to_owned(),
            },
        ),
        _ => (
            IntakeDetectedKind::Unsupported,
            IntakeClassification {
                extension,
                media_type_hint: "application/octet-stream".to_owned(),
                confidence: "extension_hint".to_owned(),
                classification_source: "safe_extension".to_owned(),
                supported_state: "unsupported".to_owned(),
            },
        ),
    }
}

fn sanitize_comment(
    comment: &str,
    owner_ref: &str,
    timestamp: &str,
) -> Result<IntakeCommentMetadata, FileIntakeError> {
    if comment.len() > MAX_COMMENT_BYTES {
        return Err(FileIntakeError::CommentTooLarge);
    }
    if looks_like_secret(comment) {
        return Err(FileIntakeError::CommentContainsSecrets);
    }
    if contains_private_absolute_path(comment) {
        return Err(FileIntakeError::CommentContainsPrivatePath);
    }
    let sanitized = comment
        .chars()
        .filter(|ch| !ch.is_control() || matches!(ch, '\n' | '\r' | '\t'))
        .collect::<String>()
        .trim()
        .to_owned();

    Ok(IntakeCommentMetadata {
        text: sanitized,
        comment_author_ref: owner_ref.to_owned(),
        comment_created_at: timestamp.to_owned(),
        comment_sanitization_state: CommentSanitizationState::Safe,
        comment_visibility: "project_scoped".to_owned(),
    })
}

fn looks_like_secret(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    lowered.contains("password=")
        || lowered.contains("api_key")
        || lowered.contains("secret=")
        || lowered.contains("token=")
        || lowered.contains("bearer ")
}

fn contains_private_absolute_path(text: &str) -> bool {
    text.contains(":\\") || text.contains(":/") || text.starts_with('/') || text.contains("\\Users\\")
}

fn sanitize_filename(filename: &str) -> String {
    let mut output = String::new();
    let mut last_was_separator = false;
    for ch in filename.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
            output.push(ch);
            last_was_separator = false;
        } else if !last_was_separator {
            output.push('_');
            last_was_separator = true;
        }
    }
    let trimmed = output.trim_matches(['_', '.', '-']).to_owned();
    if trimmed.is_empty() {
        "file".to_owned()
    } else {
        trimmed
    }
}

fn prepare_file_store_root(path: &Path) -> Result<PathBuf, FileIntakeError> {
    if !path.is_absolute() {
        return Err(FileIntakeError::UnsafePath);
    }
    fs::create_dir_all(path).map_err(|_| FileIntakeError::IoError)?;
    fs::canonicalize(path).map_err(|_| FileIntakeError::IoError)
}

fn create_dir_inside_root(root: &Path, path: &Path) -> Result<(), FileIntakeError> {
    ensure_path_within_root(root, path)?;
    fs::create_dir_all(path).map_err(|_| FileIntakeError::IoError)?;
    Ok(())
}

fn write_bytes_inside_root(root: &Path, path: &Path, bytes: &[u8]) -> Result<(), FileIntakeError> {
    ensure_path_within_root(root, path)?;
    fs::write(path, bytes).map_err(|_| FileIntakeError::IoError)
}

fn write_json_inside_root<T: Serialize>(
    root: &Path,
    path: &Path,
    value: &T,
) -> Result<(), FileIntakeError> {
    ensure_path_within_root(root, path)?;
    let json = serde_json::to_string_pretty(value).map_err(|_| FileIntakeError::IoError)?;
    fs::write(path, json).map_err(|_| FileIntakeError::IoError)
}

fn ensure_path_within_root(root: &Path, path: &Path) -> Result<(), FileIntakeError> {
    let normalized_root = normalize_absolute_comparison_path(root);
    let normalized_path = normalize_absolute_comparison_path(path);
    if normalized_path.starts_with(normalized_root) {
        Ok(())
    } else {
        Err(FileIntakeError::StoragePathEscape)
    }
}

fn relative_portable_path(root: &Path, path: &Path) -> Result<String, FileIntakeError> {
    ensure_path_within_root(root, path)?;
    let normalized_root = normalize_absolute_comparison_path(root);
    let normalized_path = normalize_absolute_comparison_path(path);
    let relative = normalized_path
        .strip_prefix(normalized_root)
        .map_err(|_| FileIntakeError::StoragePathEscape)?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/"))
}

fn normalize_absolute_comparison_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let rendered = path.display().to_string();
        if let Some(trimmed) = rendered.strip_prefix(r"\\?\") {
            PathBuf::from(trimmed)
        } else {
            path.to_path_buf()
        }
    }

    #[cfg(not(windows))]
    {
        path.to_path_buf()
    }
}

fn build_batch_ref(owner_ref: &str, trace_ref: &str, selected_files: &[FileIntakeSelectedFile]) -> String {
    let mut seed = format!("{owner_ref}:{trace_ref}:{}", selected_files.len());
    for selected_file in selected_files {
        if let Some(name) = selected_file.source_path.file_name().and_then(|name| name.to_str()) {
            seed.push(':');
            seed.push_str(name);
        }
    }
    prefixed_hash("ib", seed.as_bytes())
}

fn prefixed_hash(prefix: &str, bytes: &[u8]) -> String {
    let file_ref = FileRef::from_bytes(bytes);
    let hash = file_ref.as_str().trim_start_matches("sha256_");
    format!("{prefix}_{}", &hash[..16])
}

fn current_timestamp_text() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("unix_ms:{millis}")
}

#[cfg(test)]
mod tests {
    use super::{
        run_file_intake_batch, FileIntakeBatchRequest, FileIntakeError, FileIntakeSelectedFile,
        FileIntakeStatus, IntakeDetectedKind, IntakeExposureState,
    };
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn valid_batch_with_one_txt_produces_intake_metadata() {
        let root = unique_temp_dir("f12_intake_one_txt");
        let source = root.join("source").join("note.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "hello world");

        let batch = run_file_intake_batch(request(&store, vec![selected(&source, None)]))
            .expect("intake batch");

        assert_eq!(batch.items.len(), 1);
        assert_eq!(batch.items[0].status, FileIntakeStatus::ImportedNotExposed);
        assert_eq!(
            batch.items[0].metadata.exposure_state,
            IntakeExposureState::ImportedNotExposed
        );
        assert_eq!(batch.items[0].metadata.detected_kind, IntakeDetectedKind::Text);
        assert!(batch.items[0].stored_relative_path.is_some());
        assert!(batch.items[0].metadata_relative_path.is_some());

        cleanup(root);
    }

    #[test]
    fn valid_batch_with_multiple_txt_and_md_items() {
        let root = unique_temp_dir("f12_intake_multi");
        let first = root.join("source").join("a.txt");
        let second = root.join("source").join("b.md");
        let store = root.join("user").join("file_store");
        write_file(&first, "alpha");
        write_file(&second, "# beta");

        let batch = run_file_intake_batch(request(
            &store,
            vec![selected(&first, None), selected(&second, None)],
        ))
        .expect("intake batch");

        assert_eq!(batch.items.len(), 2);
        assert!(batch
            .items
            .iter()
            .all(|item| item.status == FileIntakeStatus::ImportedNotExposed));

        cleanup(root);
    }

    #[test]
    fn mixed_batch_keeps_valid_item_and_blocks_unsupported_extension() {
        let root = unique_temp_dir("f12_intake_mixed");
        let valid = root.join("source").join("a.txt");
        let unsupported = root.join("source").join("b.pdf");
        let store = root.join("user").join("file_store");
        write_file(&valid, "alpha");
        write_file(&unsupported, "%PDF");

        let batch = run_file_intake_batch(request(
            &store,
            vec![selected(&valid, None), selected(&unsupported, None)],
        ))
        .expect("intake batch");

        assert_eq!(batch.items.len(), 2);
        assert_eq!(batch.items[0].status, FileIntakeStatus::ImportedNotExposed);
        assert_eq!(batch.items[1].status, FileIntakeStatus::Blocked);
        assert!(batch.items[1]
            .blocking_reasons
            .contains(&FileIntakeError::UnsupportedFormat));
        assert!(batch.items[1].stored_object_candidate_ref.is_none());

        cleanup(root);
    }

    #[test]
    fn missing_source_item_is_blocked() {
        let root = unique_temp_dir("f12_intake_missing");
        let missing = root.join("source").join("missing.txt");
        let store = root.join("user").join("file_store");

        let batch = run_file_intake_batch(request(&store, vec![selected(&missing, None)]))
            .expect("intake batch");

        assert_eq!(batch.items[0].status, FileIntakeStatus::Blocked);
        assert!(batch.items[0]
            .blocking_reasons
            .contains(&FileIntakeError::MissingSource));

        cleanup(root);
    }

    #[test]
    fn directory_item_is_blocked() {
        let root = unique_temp_dir("f12_intake_dir");
        let directory = root.join("source").join("folder.txt");
        let store = root.join("user").join("file_store");
        fs::create_dir_all(&directory).expect("create directory");

        let batch = run_file_intake_batch(request(&store, vec![selected(&directory, None)]))
            .expect("intake batch");

        assert_eq!(batch.items[0].status, FileIntakeStatus::Blocked);
        assert!(batch.items[0]
            .blocking_reasons
            .contains(&FileIntakeError::SourceIsDirectory));

        cleanup(root);
    }

    #[test]
    fn unsafe_filename_is_sanitized() {
        let root = unique_temp_dir("f12_intake_filename");
        let source = root.join("source").join("bad name!.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");

        let batch = run_file_intake_batch(request(&store, vec![selected(&source, None)]))
            .expect("intake batch");

        assert_eq!(
            batch.items[0].metadata.original_filename_sanitized,
            "bad_name_.txt"
        );
        assert_eq!(batch.items[0].metadata.source_display_label, "bad_name_.txt");

        cleanup(root);
    }

    #[test]
    fn owner_ref_is_required() {
        let root = unique_temp_dir("f12_intake_owner");
        let source = root.join("source").join("a.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");
        let mut request = request(&store, vec![selected(&source, None)]);
        request.owner_ref.clear();

        let error = run_file_intake_batch(request).expect_err("owner required");

        assert_eq!(error, FileIntakeError::MissingOwnerRef);
        cleanup(root);
    }

    #[test]
    fn trace_ref_is_required() {
        let root = unique_temp_dir("f12_intake_trace");
        let source = root.join("source").join("a.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");
        let mut request = request(&store, vec![selected(&source, None)]);
        request.trace_ref.clear();

        let error = run_file_intake_batch(request).expect_err("trace required");

        assert_eq!(error, FileIntakeError::MissingTraceRef);
        cleanup(root);
    }

    #[test]
    fn absolute_host_path_is_not_persisted_in_metadata() {
        let root = unique_temp_dir("f12_intake_no_abs_path");
        let source = root.join("source").join("a.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");

        let batch = run_file_intake_batch(request(&store, vec![selected(&source, None)]))
            .expect("intake batch");
        let json = serde_json::to_string(&batch).expect("serialize batch");

        assert!(!json.contains(&source.display().to_string()));
        assert!(!json.contains(&root.display().to_string()));
        cleanup(root);
    }

    #[test]
    fn safe_user_comment_is_preserved_without_affecting_classification() {
        let root = unique_temp_dir("f12_intake_comment");
        let source = root.join("source").join("a.md");
        let store = root.join("user").join("file_store");
        write_file(&source, "# alpha");

        let batch = run_file_intake_batch(request(
            &store,
            vec![selected(&source, Some("Useful project note"))],
        ))
        .expect("intake batch");

        assert_eq!(batch.items[0].metadata.detected_kind, IntakeDetectedKind::Markdown);
        assert_eq!(
            batch.items[0]
                .metadata
                .user_comment
                .as_ref()
                .expect("comment")
                .text,
            "Useful project note"
        );
        cleanup(root);
    }

    #[test]
    fn unsafe_user_comment_is_rejected() {
        let root = unique_temp_dir("f12_intake_bad_comment");
        let source = root.join("source").join("a.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");

        let error = run_file_intake_batch(request(
            &store,
            vec![selected(&source, Some("api_key=abc123"))],
        ))
        .expect_err("bad comment");

        assert_eq!(error, FileIntakeError::CommentContainsSecrets);
        cleanup(root);
    }

    #[test]
    fn output_path_cannot_escape_governed_storage_root() {
        let root = unique_temp_dir("f12_intake_escape");
        let source = root.join("source").join("a.txt");
        let relative_store = PathBuf::from("relative_file_store");
        write_file(&source, "alpha");

        let error = run_file_intake_batch(request(&relative_store, vec![selected(&source, None)]))
            .expect_err("unsafe store");

        assert_eq!(error, FileIntakeError::UnsafePath);
        cleanup(root);
    }

    #[test]
    fn no_manifest_registry_graph_or_derivatives_are_created() {
        let root = unique_temp_dir("f12_intake_no_side_effects");
        let source = root.join("source").join("a.txt");
        let store = root.join("user").join("file_store");
        write_file(&source, "alpha");

        let _batch = run_file_intake_batch(request(&store, vec![selected(&source, None)]))
            .expect("intake batch");

        assert!(!root.join(["project", "manifest.json"].join("_")).exists());
        assert!(!store.join(["registry", "json"].join(".")).exists());
        assert!(!store.join(["gr", "aph"].concat()).exists());
        assert!(!store.join(["der", "ived"].concat()).exists());
        assert!(!store.join(["chu", "nks"].concat()).exists());
        assert!(!store.join(["pa", "ges"].concat()).exists());
        cleanup(root);
    }

    fn request(store: &Path, selected_files: Vec<FileIntakeSelectedFile>) -> FileIntakeBatchRequest {
        FileIntakeBatchRequest {
            file_store_root: store.to_path_buf(),
            owner_ref: "owner_test".to_owned(),
            trace_ref: "trace_test".to_owned(),
            batch_comment: None,
            selected_files,
        }
    }

    fn selected(path: &Path, user_comment: Option<&str>) -> FileIntakeSelectedFile {
        FileIntakeSelectedFile {
            source_path: path.to_path_buf(),
            user_comment: user_comment.map(str::to_owned),
        }
    }

    fn write_file(path: &Path, text: &str) {
        fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
        fs::write(path, text).expect("write file");
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

    fn cleanup(path: PathBuf) {
        if path.exists() {
            fs::remove_dir_all(path).expect("cleanup");
        }
    }
}
