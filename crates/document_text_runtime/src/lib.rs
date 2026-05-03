//! Minimal, regenerable document text derivation and chunking contracts.

mod contracts;

pub use contracts::{DerivationManifest, DerivationState, DerivativeKind};

use core_domain::sha256_hex;
use lopdf::Document as PdfDocument;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

const SCHEMA_VERSION: &str = "1";
const TEXT_DIRNAME: &str = "text";
const EXTRACTED_FILENAME: &str = "extracted.txt";
const TEXT_MANIFEST_FILENAME: &str = "text_manifest.json";
const PAGES_DIRNAME: &str = "pages";
const CHUNKS_DIRNAME: &str = "chunks";
const CHUNKS_MANIFEST_FILENAME: &str = "chunks_manifest.json";
const MAX_BLANK_LINE_RUN: usize = 1;
const DEFAULT_CHUNK_CHAR_LIMIT: usize = 1200;

pub type DocumentTextResult<T> = Result<T, DocumentTextDerivationError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentTextDerivationStatus {
    Ready,
    Unsupported,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextDerivationRequest {
    document_id: String,
    document_root: PathBuf,
    source_filename: String,
}

impl DocumentTextDerivationRequest {
    pub fn new(
        document_id: impl Into<String>,
        document_root: impl Into<PathBuf>,
        source_filename: impl Into<String>,
    ) -> DocumentTextResult<Self> {
        let document_id = document_id.into().trim().to_owned();
        if document_id.is_empty() {
            return Err(DocumentTextDerivationError::EmptyDocumentId);
        }

        let document_root = document_root.into();
        if !document_root.is_absolute() {
            return Err(DocumentTextDerivationError::DocumentRootMustBeAbsolute {
                path: document_root,
            });
        }
        if !document_root.exists() {
            return Err(DocumentTextDerivationError::DocumentRootDoesNotExist {
                path: document_root,
            });
        }
        if !document_root.is_dir() {
            return Err(DocumentTextDerivationError::DocumentRootIsNotDirectory {
                path: document_root,
            });
        }

        let source_filename = source_filename.into().trim().to_owned();
        if source_filename.is_empty() {
            return Err(DocumentTextDerivationError::EmptySourceFilename);
        }
        if Path::new(&source_filename).components().count() != 1 {
            return Err(
                DocumentTextDerivationError::SourceFilenameMustBePlainFileName {
                    value: source_filename,
                },
            );
        }

        let request = Self {
            document_id,
            document_root,
            source_filename,
        };
        let source_path = request.source_path();
        if !source_path.exists() {
            return Err(DocumentTextDerivationError::SourceFileDoesNotExist { path: source_path });
        }
        if !source_path.is_file() {
            return Err(DocumentTextDerivationError::SourceFileIsNotFile { path: source_path });
        }

        Ok(request)
    }

    pub fn document_id(&self) -> &str {
        &self.document_id
    }

    pub fn document_root(&self) -> &Path {
        &self.document_root
    }

    pub fn source_filename(&self) -> &str {
        &self.source_filename
    }

    pub fn source_path(&self) -> PathBuf {
        self.document_root.join(&self.source_filename)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentTextDerivationResult {
    pub document_id: String,
    pub source_filename: String,
    pub source_format: String,
    pub derivation_status: DocumentTextDerivationStatus,
    pub extracted_text_path: Option<String>,
    pub pages_count: usize,
    pub chunks_count: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextChunkRecord {
    pub chunk_id: String,
    pub document_id: String,
    pub text_path: String,
    pub page_start: Option<usize>,
    pub page_end: Option<usize>,
    pub char_start: usize,
    pub char_end: usize,
    pub source_hash: String,
    pub chunk_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentTextDerivationError {
    EmptyDocumentId,
    EmptySourceFilename,
    SourceFilenameMustBePlainFileName { value: String },
    DocumentRootMustBeAbsolute { path: PathBuf },
    DocumentRootDoesNotExist { path: PathBuf },
    DocumentRootIsNotDirectory { path: PathBuf },
    SourceFileDoesNotExist { path: PathBuf },
    SourceFileIsNotFile { path: PathBuf },
    SourceReadFailed { path: PathBuf, message: String },
    TextDerivationCleanupFailed { path: PathBuf, message: String },
    TextDirectoryCreateFailed { path: PathBuf, message: String },
    DerivedWriteFailed { path: PathBuf, message: String },
    ManifestWriteFailed { path: PathBuf, message: String },
}

impl std::fmt::Display for DocumentTextDerivationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyDocumentId => f.write_str("document_id must not be empty"),
            Self::EmptySourceFilename => f.write_str("source_filename must not be empty"),
            Self::SourceFilenameMustBePlainFileName { value } => {
                write!(f, "source_filename must be a plain file name: {value}")
            }
            Self::DocumentRootMustBeAbsolute { path } => {
                write!(f, "document_root must be absolute: {}", path.display())
            }
            Self::DocumentRootDoesNotExist { path } => {
                write!(f, "document_root does not exist: {}", path.display())
            }
            Self::DocumentRootIsNotDirectory { path } => {
                write!(f, "document_root is not a directory: {}", path.display())
            }
            Self::SourceFileDoesNotExist { path } => {
                write!(f, "source file does not exist: {}", path.display())
            }
            Self::SourceFileIsNotFile { path } => {
                write!(f, "source path is not a file: {}", path.display())
            }
            Self::SourceReadFailed { path, message } => {
                write!(f, "failed to read source '{}': {message}", path.display())
            }
            Self::TextDerivationCleanupFailed { path, message } => {
                write!(
                    f,
                    "failed to reset derived text '{}': {message}",
                    path.display()
                )
            }
            Self::TextDirectoryCreateFailed { path, message } => {
                write!(
                    f,
                    "failed to create text directory '{}': {message}",
                    path.display()
                )
            }
            Self::DerivedWriteFailed { path, message } => {
                write!(
                    f,
                    "failed to write derived file '{}': {message}",
                    path.display()
                )
            }
            Self::ManifestWriteFailed { path, message } => {
                write!(
                    f,
                    "failed to write manifest '{}': {message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for DocumentTextDerivationError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TextManifest {
    schema_version: String,
    document_id: String,
    source_filename: String,
    source_format: String,
    derivation_status: DocumentTextDerivationStatus,
    text_path: Option<String>,
    pages_count: usize,
    chunks_count: usize,
    source_hash: String,
    normalization: NormalizationMetadata,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ChunksManifest {
    schema_version: String,
    document_id: String,
    text_path: String,
    source_hash: String,
    chunks: Vec<TextChunkRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
struct NormalizationMetadata {
    utf8: bool,
    line_endings: &'static str,
    trim_trailing_spaces: bool,
    max_blank_line_run: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadyTextDerivation {
    extracted_text: String,
    pages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkMaterialization {
    record: TextChunkRecord,
    contents: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedSourceFormat {
    Txt,
    Md,
    Json,
    Jsonl,
    Pdf,
    Docx,
    Unsupported,
}

impl SupportedSourceFormat {
    fn detect(source_filename: &str) -> (Self, String) {
        let extension = Path::new(source_filename)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("unknown")
            .to_ascii_lowercase();

        let format = match extension.as_str() {
            "txt" => Self::Txt,
            "md" => Self::Md,
            "json" => Self::Json,
            "jsonl" => Self::Jsonl,
            "pdf" => Self::Pdf,
            "docx" => Self::Docx,
            _ => Self::Unsupported,
        };

        (format, extension)
    }
}

pub fn derive_document_text(
    request: &DocumentTextDerivationRequest,
) -> DocumentTextResult<DocumentTextDerivationResult> {
    let source_path = request.source_path();
    let source_bytes =
        fs::read(&source_path).map_err(|error| DocumentTextDerivationError::SourceReadFailed {
            path: source_path.clone(),
            message: error.to_string(),
        })?;
    let source_hash = sha256_hex(&source_bytes);
    let (source_format, source_format_text) =
        SupportedSourceFormat::detect(request.source_filename());
    let text_root = request.document_root().join(TEXT_DIRNAME);
    reset_text_root(&text_root)?;

    match source_format {
        SupportedSourceFormat::Unsupported => {
            let warning = format!(
                "source format '{}' is not supported for text derivation",
                source_format_text
            );
            write_status_only_manifest(
                request,
                &text_root,
                &source_format_text,
                DocumentTextDerivationStatus::Unsupported,
                &source_hash,
                vec![warning.clone()],
            )?;

            Ok(DocumentTextDerivationResult {
                document_id: request.document_id().to_owned(),
                source_filename: request.source_filename().to_owned(),
                source_format: source_format_text,
                derivation_status: DocumentTextDerivationStatus::Unsupported,
                extracted_text_path: None,
                pages_count: 0,
                chunks_count: 0,
                warnings: vec![warning],
            })
        }
        _ => match extract_ready_text(source_format, &source_path, &source_bytes) {
            Ok(ready) => materialize_ready_derivation(
                request,
                &text_root,
                &source_format_text,
                &source_hash,
                ready,
            ),
            Err(message) => {
                write_status_only_manifest(
                    request,
                    &text_root,
                    &source_format_text,
                    DocumentTextDerivationStatus::Error,
                    &source_hash,
                    vec![message.clone()],
                )?;

                Ok(DocumentTextDerivationResult {
                    document_id: request.document_id().to_owned(),
                    source_filename: request.source_filename().to_owned(),
                    source_format: source_format_text,
                    derivation_status: DocumentTextDerivationStatus::Error,
                    extracted_text_path: None,
                    pages_count: 0,
                    chunks_count: 0,
                    warnings: vec![message],
                })
            }
        },
    }
}

fn materialize_ready_derivation(
    request: &DocumentTextDerivationRequest,
    text_root: &Path,
    source_format_text: &str,
    source_hash: &str,
    ready: ReadyTextDerivation,
) -> DocumentTextResult<DocumentTextDerivationResult> {
    let extracted_text_path = text_root.join(EXTRACTED_FILENAME);
    write_text_file(&extracted_text_path, &ready.extracted_text)?;

    if !ready.pages.is_empty() {
        let pages_root = text_root.join(PAGES_DIRNAME);
        fs::create_dir_all(&pages_root).map_err(|error| {
            DocumentTextDerivationError::TextDirectoryCreateFailed {
                path: pages_root.clone(),
                message: error.to_string(),
            }
        })?;

        for (index, page) in ready.pages.iter().enumerate() {
            let page_path = pages_root.join(format!("page_{:03}.txt", index + 1));
            write_text_file(&page_path, page)?;
        }
    }

    let chunk_materializations = build_chunk_materializations(
        request.document_id(),
        &ready.extracted_text,
        &ready.pages,
        source_hash,
    );
    if !chunk_materializations.is_empty() {
        let chunks_root = text_root.join(CHUNKS_DIRNAME);
        fs::create_dir_all(&chunks_root).map_err(|error| {
            DocumentTextDerivationError::TextDirectoryCreateFailed {
                path: chunks_root.clone(),
                message: error.to_string(),
            }
        })?;

        for materialization in &chunk_materializations {
            let chunk_path = request
                .document_root()
                .join(&materialization.record.chunk_path);
            write_text_file(&chunk_path, &materialization.contents)?;
        }

        let chunks_manifest = ChunksManifest {
            schema_version: SCHEMA_VERSION.to_owned(),
            document_id: request.document_id().to_owned(),
            text_path: portable_path(&[TEXT_DIRNAME, EXTRACTED_FILENAME]),
            source_hash: source_hash.to_owned(),
            chunks: chunk_materializations
                .iter()
                .map(|item| item.record.clone())
                .collect(),
        };
        write_json_manifest(
            &text_root
                .join(CHUNKS_DIRNAME)
                .join(CHUNKS_MANIFEST_FILENAME),
            &chunks_manifest,
        )?;
    }

    let result = DocumentTextDerivationResult {
        document_id: request.document_id().to_owned(),
        source_filename: request.source_filename().to_owned(),
        source_format: source_format_text.to_owned(),
        derivation_status: DocumentTextDerivationStatus::Ready,
        extracted_text_path: Some(portable_path(&[TEXT_DIRNAME, EXTRACTED_FILENAME])),
        pages_count: ready.pages.len(),
        chunks_count: chunk_materializations.len(),
        warnings: Vec::new(),
    };

    let manifest = TextManifest {
        schema_version: SCHEMA_VERSION.to_owned(),
        document_id: result.document_id.clone(),
        source_filename: result.source_filename.clone(),
        source_format: result.source_format.clone(),
        derivation_status: result.derivation_status,
        text_path: result.extracted_text_path.clone(),
        pages_count: result.pages_count,
        chunks_count: result.chunks_count,
        source_hash: source_hash.to_owned(),
        normalization: normalization_metadata(),
        warnings: result.warnings.clone(),
    };
    write_json_manifest(&text_root.join(TEXT_MANIFEST_FILENAME), &manifest)?;

    Ok(result)
}

fn write_status_only_manifest(
    request: &DocumentTextDerivationRequest,
    text_root: &Path,
    source_format_text: &str,
    status: DocumentTextDerivationStatus,
    source_hash: &str,
    warnings: Vec<String>,
) -> DocumentTextResult<()> {
    let manifest = TextManifest {
        schema_version: SCHEMA_VERSION.to_owned(),
        document_id: request.document_id().to_owned(),
        source_filename: request.source_filename().to_owned(),
        source_format: source_format_text.to_owned(),
        derivation_status: status,
        text_path: None,
        pages_count: 0,
        chunks_count: 0,
        source_hash: source_hash.to_owned(),
        normalization: normalization_metadata(),
        warnings,
    };
    write_json_manifest(&text_root.join(TEXT_MANIFEST_FILENAME), &manifest)
}

fn extract_ready_text(
    source_format: SupportedSourceFormat,
    source_path: &Path,
    source_bytes: &[u8],
) -> Result<ReadyTextDerivation, String> {
    match source_format {
        SupportedSourceFormat::Txt
        | SupportedSourceFormat::Md
        | SupportedSourceFormat::Json
        | SupportedSourceFormat::Jsonl => {
            let text = String::from_utf8(source_bytes.to_vec())
                .map_err(|_| "source text is not valid UTF-8".to_owned())?;
            Ok(ReadyTextDerivation {
                extracted_text: normalize_text(&text),
                pages: Vec::new(),
            })
        }
        SupportedSourceFormat::Docx => {
            let text = extract_docx_text(source_path)?;
            Ok(ReadyTextDerivation {
                extracted_text: normalize_text(&text),
                pages: Vec::new(),
            })
        }
        SupportedSourceFormat::Pdf => {
            let raw_pages = extract_pdf_pages(source_path)?;
            let pages: Vec<String> = raw_pages
                .into_iter()
                .map(|page| normalize_text(&page))
                .collect();
            Ok(ReadyTextDerivation {
                extracted_text: pages.join("\n\n"),
                pages,
            })
        }
        SupportedSourceFormat::Unsupported => Err("unsupported source format".to_owned()),
    }
}

fn extract_docx_text(source_path: &Path) -> Result<String, String> {
    let file =
        File::open(source_path).map_err(|error| format!("failed to open docx archive: {error}"))?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("failed to read docx archive: {error}"))?;
    let mut document_xml = String::new();
    archive
        .by_name("word/document.xml")
        .map_err(|error| format!("missing word/document.xml: {error}"))?
        .read_to_string(&mut document_xml)
        .map_err(|error| format!("failed to read word/document.xml: {error}"))?;

    let tagged = document_xml
        .replace("</w:p>", "\n")
        .replace("<w:tab/>", "\t")
        .replace("<w:br/>", "\n")
        .replace("<w:cr/>", "\n");
    let mut stripped = String::new();
    let mut inside_tag = false;
    for character in tagged.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => stripped.push(character),
            _ => {}
        }
    }

    Ok(stripped
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'"))
}

fn extract_pdf_pages(source_path: &Path) -> Result<Vec<String>, String> {
    let document =
        PdfDocument::load(source_path).map_err(|error| format!("failed to load pdf: {error}"))?;
    let mut page_numbers: Vec<u32> = document.get_pages().keys().copied().collect();
    page_numbers.sort_unstable();

    let mut pages = Vec::new();
    for page_number in page_numbers {
        let text = document
            .extract_text(&[page_number])
            .map_err(|error| format!("failed to extract pdf page {page_number}: {error}"))?;
        pages.push(text);
    }

    Ok(pages)
}

fn build_chunk_materializations(
    document_id: &str,
    extracted_text: &str,
    pages: &[String],
    source_hash: &str,
) -> Vec<ChunkMaterialization> {
    if extracted_text.is_empty() {
        return Vec::new();
    }

    let mut materials = Vec::new();
    let mut chunk_index = 1usize;

    if pages.is_empty() {
        materials.extend(build_segment_chunks(
            document_id,
            extracted_text,
            None,
            0,
            &mut chunk_index,
            source_hash,
        ));
        return materials;
    }

    let mut global_offset = 0usize;
    for (page_index, page) in pages.iter().enumerate() {
        materials.extend(build_segment_chunks(
            document_id,
            page,
            Some(page_index + 1),
            global_offset,
            &mut chunk_index,
            source_hash,
        ));
        global_offset += page.chars().count();
        if page_index + 1 < pages.len() {
            global_offset += 2;
        }
    }

    materials
}

fn build_segment_chunks(
    document_id: &str,
    segment_text: &str,
    page_number: Option<usize>,
    global_start: usize,
    chunk_index: &mut usize,
    source_hash: &str,
) -> Vec<ChunkMaterialization> {
    let characters: Vec<char> = segment_text.chars().collect();
    if characters.is_empty() {
        return Vec::new();
    }

    let mut materials = Vec::new();
    let mut local_start = 0usize;
    while local_start < characters.len() {
        let local_end = (local_start + DEFAULT_CHUNK_CHAR_LIMIT).min(characters.len());
        let contents: String = characters[local_start..local_end].iter().collect();
        let chunk_id = format!("chunk_{:04}", *chunk_index);
        let chunk_filename = format!("{chunk_id}.txt");
        let chunk_path = portable_path(&[TEXT_DIRNAME, CHUNKS_DIRNAME, &chunk_filename]);

        materials.push(ChunkMaterialization {
            record: TextChunkRecord {
                chunk_id,
                document_id: document_id.to_owned(),
                text_path: portable_path(&[TEXT_DIRNAME, EXTRACTED_FILENAME]),
                page_start: page_number,
                page_end: page_number,
                char_start: global_start + local_start,
                char_end: global_start + local_end,
                source_hash: source_hash.to_owned(),
                chunk_path,
            },
            contents,
        });

        *chunk_index += 1;
        local_start = local_end;
    }

    materials
}

fn normalize_text(input: &str) -> String {
    let normalized_line_endings = input.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = Vec::new();
    let mut blank_run = 0usize;

    for line in normalized_line_endings.split('\n') {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            blank_run += 1;
            if blank_run <= MAX_BLANK_LINE_RUN {
                lines.push(String::new());
            }
        } else {
            blank_run = 0;
            lines.push(trimmed.to_owned());
        }
    }

    lines.join("\n")
}

fn reset_text_root(text_root: &Path) -> DocumentTextResult<()> {
    if text_root.exists() {
        fs::remove_dir_all(text_root).map_err(|error| {
            DocumentTextDerivationError::TextDerivationCleanupFailed {
                path: text_root.to_path_buf(),
                message: error.to_string(),
            }
        })?;
    }

    fs::create_dir_all(text_root).map_err(|error| {
        DocumentTextDerivationError::TextDirectoryCreateFailed {
            path: text_root.to_path_buf(),
            message: error.to_string(),
        }
    })
}

fn write_text_file(path: &Path, contents: &str) -> DocumentTextResult<()> {
    fs::write(path, contents).map_err(|error| DocumentTextDerivationError::DerivedWriteFailed {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn write_json_manifest<T: Serialize>(path: &Path, value: &T) -> DocumentTextResult<()> {
    let contents = serde_json::to_string_pretty(value).map_err(|error| {
        DocumentTextDerivationError::ManifestWriteFailed {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    })?;
    fs::write(path, contents).map_err(|error| DocumentTextDerivationError::ManifestWriteFailed {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn normalization_metadata() -> NormalizationMetadata {
    NormalizationMetadata {
        utf8: true,
        line_endings: "lf",
        trim_trailing_spaces: true,
        max_blank_line_run: MAX_BLANK_LINE_RUN,
    }
}

fn portable_path(parts: &[&str]) -> String {
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::{
        derive_document_text, DocumentTextDerivationRequest, DocumentTextDerivationStatus,
        TextChunkRecord,
    };
    use serde_json::Value;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use zip::write::FileOptions;

    #[test]
    fn supported_markdown_document_produces_ready_derivation() {
        let document_root = unique_temp_dir("document_text_runtime_markdown");
        fs::create_dir_all(&document_root).expect("create document root");
        let source_path = document_root.join("note.md");
        let original_contents = "Alpha  \r\n\r\n\r\nBeta\r\n";
        fs::write(&source_path, original_contents).expect("write source");
        let request =
            DocumentTextDerivationRequest::new("doc.note", document_root.clone(), "note.md")
                .expect("request");

        let result = derive_document_text(&request).expect("ready result");

        assert_eq!(
            result.derivation_status,
            DocumentTextDerivationStatus::Ready
        );
        assert_eq!(result.source_format, "md");
        assert_eq!(
            result.extracted_text_path.as_deref(),
            Some("text/extracted.txt")
        );
        assert_eq!(result.pages_count, 0);
        assert_eq!(result.chunks_count, 1);
        assert_eq!(
            fs::read_to_string(document_root.join("text").join("extracted.txt"))
                .expect("read extracted text"),
            "Alpha\n\nBeta\n"
        );
        assert_eq!(
            fs::read_to_string(&source_path).expect("read source"),
            original_contents
        );

        let manifest = read_json(document_root.join("text").join("text_manifest.json"));
        assert_eq!(manifest["derivation_status"], "ready");
        assert_eq!(manifest["text_path"], "text/extracted.txt");
        assert_eq!(manifest["chunks_count"], 1);

        fs::remove_dir_all(document_root).expect("cleanup document root");
    }

    #[test]
    fn unsupported_format_produces_unsupported_manifest() {
        let document_root = unique_temp_dir("document_text_runtime_unsupported");
        fs::create_dir_all(&document_root).expect("create document root");
        fs::write(document_root.join("image.png"), [0_u8, 1, 2, 3]).expect("write source");
        let request =
            DocumentTextDerivationRequest::new("doc.image", document_root.clone(), "image.png")
                .expect("request");

        let result = derive_document_text(&request).expect("unsupported result");

        assert_eq!(
            result.derivation_status,
            DocumentTextDerivationStatus::Unsupported
        );
        assert!(result.extracted_text_path.is_none());
        assert_eq!(result.pages_count, 0);
        assert_eq!(result.chunks_count, 0);
        assert!(result.warnings[0].contains("not supported"));
        assert!(!document_root.join("text").join("extracted.txt").exists());

        let manifest = read_json(document_root.join("text").join("text_manifest.json"));
        assert_eq!(manifest["derivation_status"], "unsupported");

        fs::remove_dir_all(document_root).expect("cleanup document root");
    }

    #[test]
    fn malformed_pdf_produces_error_manifest() {
        let document_root = unique_temp_dir("document_text_runtime_pdf_error");
        fs::create_dir_all(&document_root).expect("create document root");
        fs::write(document_root.join("broken.pdf"), "not a pdf").expect("write source");
        let request =
            DocumentTextDerivationRequest::new("doc.pdf", document_root.clone(), "broken.pdf")
                .expect("request");

        let result = derive_document_text(&request).expect("error result");

        assert_eq!(
            result.derivation_status,
            DocumentTextDerivationStatus::Error
        );
        assert!(result.extracted_text_path.is_none());
        assert!(result.warnings[0].contains("failed"));
        assert!(!document_root.join("text").join("extracted.txt").exists());

        let manifest = read_json(document_root.join("text").join("text_manifest.json"));
        assert_eq!(manifest["derivation_status"], "error");

        fs::remove_dir_all(document_root).expect("cleanup document root");
    }

    #[test]
    fn docx_derivation_writes_text_and_deterministic_chunks() {
        let document_root = unique_temp_dir("document_text_runtime_docx");
        fs::create_dir_all(&document_root).expect("create document root");
        let source_path = document_root.join("guide.docx");
        write_minimal_docx(&source_path, &["Alpha section", "Beta section"]);
        let request =
            DocumentTextDerivationRequest::new("doc.guide", document_root.clone(), "guide.docx")
                .expect("request");

        let first = derive_document_text(&request).expect("first derivation");
        let first_chunks_manifest = fs::read_to_string(
            document_root
                .join("text")
                .join("chunks")
                .join("chunks_manifest.json"),
        )
        .expect("read first chunks manifest");
        let second = derive_document_text(&request).expect("second derivation");
        let second_chunks_manifest = fs::read_to_string(
            document_root
                .join("text")
                .join("chunks")
                .join("chunks_manifest.json"),
        )
        .expect("read second chunks manifest");

        assert_eq!(first.derivation_status, DocumentTextDerivationStatus::Ready);
        assert_eq!(
            second.derivation_status,
            DocumentTextDerivationStatus::Ready
        );
        assert_eq!(first.pages_count, 0);
        assert_eq!(first.chunks_count, second.chunks_count);
        assert_eq!(first_chunks_manifest, second_chunks_manifest);
        assert!(
            fs::read_to_string(document_root.join("text").join("extracted.txt"))
                .expect("read extracted text")
                .contains("Alpha section")
        );

        let chunks_manifest: Value =
            serde_json::from_str(&first_chunks_manifest).expect("parse chunk manifest json");
        let chunk_records: Vec<TextChunkRecord> =
            serde_json::from_value(chunks_manifest["chunks"].clone())
                .expect("deserialize chunk records");
        assert!(!chunk_records.is_empty());
        assert_eq!(chunk_records[0].document_id, "doc.guide");
        assert_eq!(chunk_records[0].text_path, "text/extracted.txt");

        fs::remove_dir_all(document_root).expect("cleanup document root");
    }

    fn write_minimal_docx(path: &Path, paragraphs: &[&str]) {
        let file = fs::File::create(path).expect("create docx file");
        let mut archive = zip::ZipWriter::new(file);
        archive
            .start_file("word/document.xml", FileOptions::default())
            .expect("start docx xml entry");

        let body = paragraphs
            .iter()
            .map(|paragraph| format!("<w:p><w:r><w:t>{paragraph}</w:t></w:r></w:p>"))
            .collect::<Vec<_>>()
            .join("");
        let document_xml = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:body>{body}</w:body></w:document>"
        );
        archive
            .write_all(document_xml.as_bytes())
            .expect("write docx xml");
        archive.finish().expect("finish docx archive");
    }

    fn read_json(path: PathBuf) -> Value {
        serde_json::from_str(&fs::read_to_string(path).expect("read json file"))
            .expect("parse json file")
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
