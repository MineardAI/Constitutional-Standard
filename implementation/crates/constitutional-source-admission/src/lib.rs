//! Bounded filesystem discovery and structural source admission.
//!
//! Admission identifies files for implementation processing. It does not parse
//! constitutional meaning, determine applicability, assign authority, or resolve
//! precedence.

use constitutional_contracts::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const PROFILE_ID: &str = "reference-implementation-source-admission";
pub const PROFILE_VERSION: &str = "1.0.0";
const METADATA_LINE_LIMIT: usize = 40;
const DIGEST_ALGORITHM: &str = "SHA-256";

#[derive(Clone, Debug)]
pub struct DeclaredSourceRoot {
    pub id: SourceRootId,
    pub absolute_path: PathBuf,
    pub logical_root_name: String,
    pub permitted_extensions: Vec<String>,
    pub excluded_directories: Vec<String>,
    pub repository_boundary: PathBuf,
    pub recursive: bool,
    pub root_provenance: String,
}

#[derive(Clone, Debug)]
pub struct SourceDiscoveryRequest {
    pub root: DeclaredSourceRoot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredSourceFile {
    pub root_id: SourceRootId,
    pub absolute_path: PathBuf,
    pub relative_path: String,
    pub extension: String,
}

#[derive(Clone, Debug)]
pub struct SourceAdmissionRequest {
    pub root: DeclaredSourceRoot,
    pub file: DiscoveredSourceFile,
}

pub fn validate_source_root(root: &DeclaredSourceRoot) -> Result<(), SourceAdmissionFinding> {
    if !root.absolute_path.is_absolute() || !root.repository_boundary.is_absolute() {
        return Err(finding(
            SourceAdmissionFindingCode::PathTraversal,
            root.absolute_path.display().to_string(),
            "source root and repository boundary must be absolute",
            true,
        ));
    }
    if has_parent_component(&root.absolute_path) || has_parent_component(&root.repository_boundary)
    {
        return Err(finding(
            SourceAdmissionFindingCode::PathTraversal,
            root.absolute_path.display().to_string(),
            "parent traversal components are not accepted in declared roots",
            true,
        ));
    }
    let boundary = fs::canonicalize(&root.repository_boundary).map_err(|_| {
        finding(
            SourceAdmissionFindingCode::RootNotFound,
            root.repository_boundary.display().to_string(),
            "repository boundary does not exist",
            true,
        )
    })?;
    let root_path = fs::canonicalize(&root.absolute_path).map_err(|_| {
        finding(
            SourceAdmissionFindingCode::RootNotFound,
            root.absolute_path.display().to_string(),
            "declared source root does not exist",
            true,
        )
    })?;
    if !root_path.starts_with(&boundary) {
        return Err(finding(
            SourceAdmissionFindingCode::RootOutsideBoundary,
            root.absolute_path.display().to_string(),
            "declared source root is outside repository boundary",
            true,
        ));
    }
    if !root_path.is_dir() {
        return Err(finding(
            SourceAdmissionFindingCode::RootNotDirectory,
            root.absolute_path.display().to_string(),
            "declared source root is not a directory",
            true,
        ));
    }
    Ok(())
}

pub fn discover_sources(
    request: &SourceDiscoveryRequest,
) -> Result<Vec<DiscoveredSourceFile>, SourceAdmissionReport> {
    if let Err(error) = validate_source_root(&request.root) {
        return Err(report(
            AdmissionReportId::new("DISCOVERY-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![error],
        ));
    }
    let mut files = Vec::new();
    if let Err(error) = walk(&request.root, &request.root.absolute_path, &mut files) {
        return Err(report(
            AdmissionReportId::new("DISCOVERY-FAILED").unwrap(),
            SourceAdmissionStatus::new("INDETERMINATE").unwrap(),
            vec![error],
        ));
    }
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(files)
}

pub fn inspect_source(
    file: &DiscoveredSourceFile,
) -> Result<SourceDescriptor, Vec<SourceAdmissionFinding>> {
    let bytes = fs::read(&file.absolute_path).map_err(|_| {
        vec![finding(
            SourceAdmissionFindingCode::UnreadableFile,
            file.relative_path.clone(),
            "source file could not be read",
            true,
        )]
    })?;
    if bytes.is_empty() {
        return Err(vec![finding(
            SourceAdmissionFindingCode::EmptySource,
            file.relative_path.clone(),
            "source file is empty",
            true,
        )]);
    }
    let text = String::from_utf8(bytes.clone()).map_err(|_| {
        vec![finding(
            SourceAdmissionFindingCode::InvalidUtf8,
            file.relative_path.clone(),
            "metadata-supported source must be UTF-8",
            true,
        )]
    })?;
    let metadata = extract_metadata(&text, &file.relative_path, file.extension.as_str());
    let mut findings = metadata.findings;
    let filename = filename_hints(&file.relative_path);
    if let (Some(file_id), Some(metadata_id)) =
        (filename.identifier.as_ref(), metadata.identifier.as_ref())
        && file_id != metadata_id
    {
        findings.push(finding(
            SourceAdmissionFindingCode::FilenameMetadataMismatch,
            file.relative_path.clone(),
            "filename identifier differs from metadata identifier",
            true,
        ));
    }
    if let (Some(file_version), Some(metadata_version)) =
        (filename.version.as_ref(), metadata.version.as_ref())
        && file_version != metadata_version
    {
        findings.push(finding(
            SourceAdmissionFindingCode::FilenameMetadataMismatch,
            file.relative_path.clone(),
            "filename version differs from metadata version",
            true,
        ));
    }
    if let (Some(file_family), Some(metadata_family)) =
        (filename.family.as_ref(), metadata.family.as_ref())
        && file_family != metadata_family
    {
        findings.push(finding(
            SourceAdmissionFindingCode::FilenameMetadataMismatch,
            file.relative_path.clone(),
            "filename family differs from metadata family",
            true,
        ));
    }
    let identifier = match metadata.identifier {
        Some(value) => ConstitutionalSourceId::new(value).map_err(|_| {
            vec![finding(
                SourceAdmissionFindingCode::InvalidIdentifier,
                file.relative_path.clone(),
                "identifier is malformed",
                true,
            )]
        })?,
        None => {
            return Err(push(
                finding(
                    SourceAdmissionFindingCode::MissingIdentifier,
                    file.relative_path.clone(),
                    "required Identifier metadata is absent",
                    true,
                ),
                findings,
            ));
        }
    };
    let version = match metadata.version {
        Some(value) => ConstitutionalSourceVersion::new(value).map_err(|_| {
            vec![finding(
                SourceAdmissionFindingCode::InvalidVersion,
                file.relative_path.clone(),
                "version is malformed",
                true,
            )]
        })?,
        None => {
            return Err(push(
                finding(
                    SourceAdmissionFindingCode::MissingVersion,
                    file.relative_path.clone(),
                    "required Version metadata is absent",
                    true,
                ),
                findings,
            ));
        }
    };
    let family = metadata.family.or(filename.family).ok_or_else(|| {
        push(
            finding(
                SourceAdmissionFindingCode::UnknownFamily,
                file.relative_path.clone(),
                "source family cannot be bounded from metadata or filename",
                true,
            ),
            findings.clone(),
        )
    })?;
    if !is_known_family(&family) {
        return Err(push(
            finding(
                SourceAdmissionFindingCode::UnknownFamily,
                file.relative_path.clone(),
                "source family is not supported by the bounded profile",
                true,
            ),
            findings,
        ));
    }
    let family = SourceFamily::new(family).map_err(|_| {
        vec![finding(
            SourceAdmissionFindingCode::UnknownFamily,
            file.relative_path.clone(),
            "source family is malformed",
            true,
        )]
    })?;
    let digest = SourceContentDigest {
        algorithm: DIGEST_ALGORITHM.into(),
        value: sha256_hex(&bytes),
    };
    let root_id = SourceRootId::new("UNBOUND").unwrap();
    let identity = SourceIdentity {
        identifier: identifier.clone(),
        family: family.clone(),
        version: version.clone(),
        root_id: root_id.clone(),
        relative_path: normalize_relative_path(&file.relative_path),
        content_digest: digest,
    };
    let source_ref = ConstitutionalSourceRef::new(
        identifier,
        version,
        format!("{}/{}", root_id, identity.relative_path),
    )
    .map_err(|_| {
        vec![finding(
            SourceAdmissionFindingCode::InvalidIdentifier,
            file.relative_path.clone(),
            "source reference could not be constructed",
            true,
        )]
    })?;
    if findings.iter().any(|item| item.fatal) {
        return Err(findings);
    }
    Ok(SourceDescriptor {
        source_ref,
        identity,
        title: metadata.title,
        normative_status: metadata
            .status
            .map(SourceNormativeStatus::new)
            .transpose()
            .map_err(|_| {
                vec![finding(
                    SourceAdmissionFindingCode::MalformedMetadata,
                    file.relative_path.clone(),
                    "status metadata is malformed",
                    true,
                )]
            })?,
        classification: metadata
            .classification
            .map(SourceClassification::new)
            .transpose()
            .map_err(|_| {
                vec![finding(
                    SourceAdmissionFindingCode::MalformedMetadata,
                    file.relative_path.clone(),
                    "classification metadata is malformed",
                    true,
                )]
            })?,
        revision_type: metadata.revision_type,
        dependencies: metadata.dependencies,
    })
}

pub fn admit_source(
    request: &SourceAdmissionRequest,
) -> Result<AdmittedSource, SourceAdmissionReport> {
    if let Err(error) = validate_source_root(&request.root) {
        return Err(report(
            AdmissionReportId::new("ADMISSION-ROOT-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![error],
        ));
    }
    let path = &request.file.absolute_path;
    if request.file.root_id != request.root.id
        || has_parent_component(Path::new(&request.file.relative_path))
    {
        return Err(report(
            AdmissionReportId::new("ADMISSION-PATH-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![finding(
                SourceAdmissionFindingCode::PathTraversal,
                request.file.relative_path.clone(),
                "file does not belong to the declared root",
                true,
            )],
        ));
    }
    if !request.root.permitted_extensions.iter().any(|allowed| {
        allowed
            .trim_start_matches('.')
            .eq_ignore_ascii_case(&request.file.extension)
    }) {
        return Err(report(
            AdmissionReportId::new("ADMISSION-EXTENSION-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![finding(
                SourceAdmissionFindingCode::UnsupportedFileType,
                request.file.relative_path.clone(),
                "file extension is not permitted by the declared root",
                true,
            )],
        ));
    }
    if fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(report(
            AdmissionReportId::new("ADMISSION-SYMLINK-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![finding(
                SourceAdmissionFindingCode::SymlinkEscape,
                request.file.relative_path.clone(),
                "symlinked source files are not supported",
                true,
            )],
        ));
    }
    let canonical_root = fs::canonicalize(&request.root.absolute_path).map_err(|_| {
        report_simple(
            "ADMISSION-ROOT-REJECTED",
            SourceAdmissionFindingCode::RootNotFound,
            "root cannot be canonicalized",
        )
    })?;
    let canonical_boundary = fs::canonicalize(&request.root.repository_boundary).map_err(|_| {
        report_simple(
            "ADMISSION-BOUNDARY-REJECTED",
            SourceAdmissionFindingCode::RootNotFound,
            "boundary cannot be canonicalized",
        )
    })?;
    let canonical_file = fs::canonicalize(path).map_err(|_| {
        report_simple(
            "ADMISSION-PATH-REJECTED",
            SourceAdmissionFindingCode::UnreadableFile,
            "file cannot be canonicalized",
        )
    })?;
    if !canonical_file.starts_with(&canonical_root)
        || !canonical_file.starts_with(&canonical_boundary)
    {
        return Err(report(
            AdmissionReportId::new("ADMISSION-OUT-OF-ROOT").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            vec![finding(
                SourceAdmissionFindingCode::RootOutsideBoundary,
                request.file.relative_path.clone(),
                "file is outside declared root or repository boundary",
                true,
            )],
        ));
    }
    let mut descriptor = inspect_source(&request.file).map_err(|findings| {
        report(
            AdmissionReportId::new("ADMISSION-INSPECTION-REJECTED").unwrap(),
            SourceAdmissionStatus::new("REJECTED").unwrap(),
            findings,
        )
    })?;
    descriptor.identity.root_id = request.root.id.clone();
    descriptor.source_ref.path =
        format!("{}/{}", request.root.id, descriptor.identity.relative_path);
    let status = if descriptor.identity.relative_path.is_empty() {
        SourceAdmissionStatus::new("REJECTED").unwrap()
    } else {
        SourceAdmissionStatus::new("ADMITTED").unwrap()
    };
    Ok(AdmittedSource {
        descriptor,
        admission_status: status,
    })
}

pub fn build_admitted_source_set(
    mut sources: Vec<AdmittedSource>,
) -> Result<AdmittedSourceSet, SourceAdmissionReport> {
    let mut findings = Vec::new();
    sources.sort_by_key(|source| source.descriptor.identity.clone());
    let mut identities = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for source in &sources {
        if source.admission_status.as_str() == "REJECTED" {
            findings.push(finding(
                SourceAdmissionFindingCode::RejectedSource,
                source.descriptor.identity.relative_path.clone(),
                "rejected source cannot enter admitted source set",
                true,
            ));
        }
        let identity = &source.descriptor.identity;
        if !paths.insert((identity.root_id.to_string(), identity.relative_path.clone())) {
            findings.push(finding(
                SourceAdmissionFindingCode::DuplicateRelativePath,
                identity.relative_path.clone(),
                "relative path appears more than once",
                true,
            ));
        }
        let key = format!("{}@{}", identity.identifier, identity.version);
        if let Some(previous) =
            identities.insert(key.clone(), identity.content_digest.value.clone())
        {
            if previous != identity.content_digest.value {
                findings.push(finding(
                    SourceAdmissionFindingCode::ConflictingSourceContent,
                    key.clone(),
                    "same source identity and version have different content digests",
                    true,
                ));
            } else {
                findings.push(finding(
                    SourceAdmissionFindingCode::DuplicateSourceIdentity,
                    key.clone(),
                    "same source identity and version appears more than once",
                    true,
                ));
            }
        }
    }
    if !findings.is_empty() {
        return Err(report(
            AdmissionReportId::new("ADMISSION-SET-CONFLICT").unwrap(),
            SourceAdmissionStatus::new("CONFLICT").unwrap(),
            findings,
        ));
    }
    let root_ids: Vec<_> = sources
        .iter()
        .map(|source| source.descriptor.identity.root_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut material = String::new();
    for source in &sources {
        material.push_str(&format!(
            "{}|{}|{}|{}|{}|{}\n",
            source.descriptor.identity.identifier,
            source.descriptor.identity.family,
            source.descriptor.identity.version,
            source.descriptor.identity.root_id,
            source.descriptor.identity.relative_path,
            source.descriptor.identity.content_digest.value
        ));
    }
    let source_set_id =
        SourceSetId::new(format!("source-set-{}", sha256_hex(material.as_bytes()))).unwrap();
    Ok(AdmittedSourceSet {
        source_set_id,
        admission_report_id: AdmissionReportId::new(format!(
            "admission-{}",
            sha256_hex(material.as_bytes())
        ))
        .unwrap(),
        root_ids,
        sources,
    })
}

fn walk(
    root: &DeclaredSourceRoot,
    directory: &Path,
    output: &mut Vec<DiscoveredSourceFile>,
) -> Result<(), SourceAdmissionFinding> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .map_err(|_| {
            finding(
                SourceAdmissionFindingCode::UnreadableFile,
                directory.display().to_string(),
                "directory could not be read",
                true,
            )
        })?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let file_type = entry.file_type().map_err(|_| {
            finding(
                SourceAdmissionFindingCode::UnreadableFile,
                path.display().to_string(),
                "directory entry type could not be read",
                true,
            )
        })?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if root
                .excluded_directories
                .iter()
                .any(|excluded| excluded == &name)
            {
                continue;
            }
            if root.recursive {
                walk(root, &path, output)?;
            }
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !root.permitted_extensions.iter().any(|allowed| {
            allowed
                .trim_start_matches('.')
                .eq_ignore_ascii_case(&extension)
        }) {
            continue;
        }
        let relative = path
            .strip_prefix(&root.absolute_path)
            .map_err(|_| {
                finding(
                    SourceAdmissionFindingCode::RootOutsideBoundary,
                    path.display().to_string(),
                    "file could not be made relative to root",
                    true,
                )
            })?
            .to_string_lossy()
            .to_string();
        output.push(DiscoveredSourceFile {
            root_id: root.id.clone(),
            absolute_path: path,
            relative_path: normalize_relative_path(&relative),
            extension,
        });
    }
    Ok(())
}

#[derive(Default)]
struct Metadata {
    identifier: Option<String>,
    title: Option<String>,
    version: Option<String>,
    status: Option<String>,
    status_alias: Option<String>,
    classification: Option<String>,
    family: Option<String>,
    family_alias: Option<String>,
    revision_type: Option<String>,
    dependencies: Vec<String>,
    findings: Vec<SourceAdmissionFinding>,
}

fn extract_metadata(text: &str, relative_path: &str, extension: &str) -> Metadata {
    let mut metadata = Metadata::default();
    let lines: Vec<_> = text.lines().take(METADATA_LINE_LIMIT).collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed == "---" {
            break;
        }
        let Some((raw_key, raw_value)) = line.split_once(':') else {
            continue;
        };
        let key = raw_key.trim().trim_matches('*').trim().to_ascii_lowercase();
        let value = raw_value.trim().trim_matches('*').trim().to_string();
        if matches!(key.as_str(), "status" | "normative status") {
            if let Some(previous_key) = &metadata.status_alias {
                metadata.findings.push(finding(
                    if previous_key == &key {
                        SourceAdmissionFindingCode::DuplicateMetadataField
                    } else {
                        SourceAdmissionFindingCode::ConflictingMetadataAlias
                    },
                    relative_path,
                    "status aliases are duplicated or conflicting",
                    true,
                ));
            } else {
                metadata.status_alias = Some(key.clone());
                metadata.status = Some(value);
            }
            continue;
        }
        if matches!(key.as_str(), "family" | "specification family") {
            if let Some(previous_key) = &metadata.family_alias {
                metadata.findings.push(finding(
                    if previous_key == &key {
                        SourceAdmissionFindingCode::DuplicateMetadataField
                    } else {
                        SourceAdmissionFindingCode::ConflictingMetadataAlias
                    },
                    relative_path,
                    "family aliases are duplicated or conflicting",
                    true,
                ));
            } else {
                metadata.family_alias = Some(key.clone());
                metadata.family = Some(value);
            }
            continue;
        }
        let slot = match key.as_str() {
            "identifier" => Some(&mut metadata.identifier),
            "title" => Some(&mut metadata.title),
            "version" => Some(&mut metadata.version),
            "classification" => Some(&mut metadata.classification),
            "revision type" => Some(&mut metadata.revision_type),
            "dependencies" => {
                metadata.dependencies = value
                    .split(',')
                    .map(str::trim)
                    .filter(|item| !item.is_empty())
                    .map(str::to_string)
                    .collect();
                None
            }
            _ => None,
        };
        if let Some(slot) = slot {
            if slot.is_some() {
                metadata.findings.push(finding(
                    SourceAdmissionFindingCode::DuplicateMetadataField,
                    relative_path,
                    "metadata field appears more than once",
                    true,
                ));
            } else {
                *slot = Some(value);
            }
        }
    }
    if extension == "json" && metadata.identifier.is_none() {
        metadata.identifier = json_value(text, "identifier");
        metadata.title = metadata.title.or_else(|| json_value(text, "title"));
        metadata.version = metadata.version.or_else(|| json_value(text, "version"));
        metadata.status = metadata.status.or_else(|| json_value(text, "status"));
        metadata.classification = metadata
            .classification
            .or_else(|| json_value(text, "classification"));
        metadata.family = metadata.family.or_else(|| json_value(text, "family"));
    }
    metadata
}

struct FilenameHints {
    identifier: Option<String>,
    version: Option<String>,
    family: Option<String>,
}
fn filename_hints(relative_path: &str) -> FilenameHints {
    let stem = Path::new(relative_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let token = stem
        .split(['_', ' ', '—'])
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let family = ["CORE", "GOV", "IMP", "AFD", "BASE", "REG", "META", "OPS"]
        .iter()
        .find(|prefix| token.starts_with(**prefix))
        .map(|value| (*value).to_string());
    let identifier = family.as_ref().and_then(|prefix| {
        token.strip_prefix(prefix).and_then(|suffix| {
            if suffix.starts_with('-') && suffix.len() > 1 {
                Some(token.clone())
            } else {
                None
            }
        })
    });
    let version = stem
        .split(['v', 'V'])
        .nth(1)
        .map(|value| {
            value
                .split(|character: char| !character.is_ascii_digit() && character != '.')
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .filter(|value| value.contains('.'));
    FilenameHints {
        identifier,
        version,
        family,
    }
}

fn json_value(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = text.find(&needle)?;
    let rest = &text[start + needle.len()..];
    let value = rest.split_once(':')?.1.trim().trim_start_matches('"');
    Some(value.split('"').next()?.to_string())
}
fn is_known_family(value: &str) -> bool {
    matches!(
        value,
        "CORE" | "GOV" | "IMP" | "AFD" | "BASE" | "REG" | "META" | "OPS" | "ADMIN" | "MANIFEST"
    )
}
fn normalize_relative_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}
fn has_parent_component(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::ParentDir))
}
fn finding(
    code: SourceAdmissionFindingCode,
    subject: impl Into<String>,
    detail: impl Into<String>,
    fatal: bool,
) -> SourceAdmissionFinding {
    SourceAdmissionFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
fn push(
    finding: SourceAdmissionFinding,
    mut findings: Vec<SourceAdmissionFinding>,
) -> Vec<SourceAdmissionFinding> {
    findings.push(finding);
    findings
}
fn report(
    id: AdmissionReportId,
    disposition: SourceAdmissionStatus,
    findings: Vec<SourceAdmissionFinding>,
) -> SourceAdmissionReport {
    SourceAdmissionReport {
        report_id: id,
        disposition,
        findings,
    }
}
fn report_simple(
    id: &str,
    code: SourceAdmissionFindingCode,
    detail: &str,
) -> SourceAdmissionReport {
    report(
        AdmissionReportId::new(id).unwrap(),
        SourceAdmissionStatus::new("REJECTED").unwrap(),
        vec![finding(code, id, detail, true)],
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut data = bytes.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    for chunk in data.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (index, bytes) in chunk.chunks_exact(4).take(16).enumerate() {
            w[index] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
            state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7],
        );
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            (h, g, f, e, d, c, b, a) = (
                g,
                f,
                e,
                d.wrapping_add(temp1),
                c,
                b,
                a,
                temp1.wrapping_add(temp2),
            );
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}
