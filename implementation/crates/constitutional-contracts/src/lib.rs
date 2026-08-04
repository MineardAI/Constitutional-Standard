//! Stable, non-sovereign contracts for the reference implementation slice.
//!
//! These types represent implementation inputs, outputs, references, and findings.
//! They do not establish constitutional authority, state, adoption, effectiveness,
//! conformance, certification, or operational recognition.

use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(ContractError::MalformedIdentifier(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! identifier_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(Identifier);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
                Ok(Self(Identifier::new(value)?))
            }
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

identifier_type!(ConstitutionalSourceId);
identifier_type!(ConstitutionalSourceVersion);
identifier_type!(BaselineRef);
identifier_type!(JurisdictionRef);
identifier_type!(ScopeRef);
identifier_type!(EffectiveContextRef);
identifier_type!(BoundaryContextRef);
identifier_type!(ContextId);
identifier_type!(OperationId);
identifier_type!(ImplementationVersion);
identifier_type!(ImplementationProfileRef);
identifier_type!(RepresentationFormatId);
identifier_type!(RepresentationVersion);
identifier_type!(ProvenanceRef);
identifier_type!(RequirementId);
identifier_type!(ImplementationMappingId);
identifier_type!(EvidenceRecordId);
identifier_type!(SourceRootId);
identifier_type!(SourceSetId);
identifier_type!(AdmissionReportId);
identifier_type!(SourceFamily);
identifier_type!(SourceAdmissionStatus);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceNormativeStatus(String);

impl SourceNormativeStatus {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        if value.is_empty() || value.contains(['\n', '\r']) {
            return Err(ContractError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceClassification(String);

impl SourceClassification {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        if value.is_empty() || value.contains(['\n', '\r']) {
            return Err(ContractError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstitutionalSourceRef {
    pub id: ConstitutionalSourceId,
    pub version: ConstitutionalSourceVersion,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceContentDigest {
    pub algorithm: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceIdentity {
    pub identifier: ConstitutionalSourceId,
    pub family: SourceFamily,
    pub version: ConstitutionalSourceVersion,
    pub root_id: SourceRootId,
    pub relative_path: String,
    pub content_digest: SourceContentDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDescriptor {
    pub source_ref: ConstitutionalSourceRef,
    pub identity: SourceIdentity,
    pub title: Option<String>,
    pub normative_status: Option<SourceNormativeStatus>,
    pub classification: Option<SourceClassification>,
    pub revision_type: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedSource {
    pub descriptor: SourceDescriptor,
    pub admission_status: SourceAdmissionStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedSourceSet {
    pub source_set_id: SourceSetId,
    pub admission_report_id: AdmissionReportId,
    pub root_ids: Vec<SourceRootId>,
    pub sources: Vec<AdmittedSource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceAdmissionFindingCode {
    RootNotFound,
    RootNotDirectory,
    RootOutsideBoundary,
    PathTraversal,
    SymlinkEscape,
    UnreadableFile,
    FileChangedDuringAdmission,
    UnsupportedFileType,
    InvalidUtf8,
    EmptySource,
    MalformedMetadata,
    DuplicateMetadataField,
    ConflictingMetadataAlias,
    MissingIdentifier,
    MissingVersion,
    InvalidIdentifier,
    InvalidVersion,
    UnknownFamily,
    FilenameMetadataMismatch,
    DuplicateSourceIdentity,
    ConflictingSourceContent,
    ConflictingSourceVersion,
    DuplicateRelativePath,
    ConflictingRoot,
    DigestFailure,
    RejectedSource,
    UnresolvedConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceAdmissionFinding {
    pub code: SourceAdmissionFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceAdmissionReport {
    pub report_id: AdmissionReportId,
    pub disposition: SourceAdmissionStatus,
    pub findings: Vec<SourceAdmissionFinding>,
}

impl ConstitutionalSourceRef {
    pub fn new(
        id: ConstitutionalSourceId,
        version: ConstitutionalSourceVersion,
        path: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let path = path.into();
        if path.is_empty() || path.contains('\n') {
            return Err(ContractError::MalformedReference(path));
        }
        Ok(Self { id, version, path })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementRef {
    pub source: ConstitutionalSourceRef,
    pub requirement_id: RequirementId,
    pub locator: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePoint(String);

impl TimePoint {
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(ContractError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalConstitutionalContext {
    pub operation_id: OperationId,
    pub source_set: Vec<ConstitutionalSourceRef>,
    pub baseline: BaselineRef,
    pub jurisdiction: JurisdictionRef,
    pub scope: ScopeRef,
    pub effective_context: EffectiveContextRef,
    pub boundary_context: BoundaryContextRef,
    pub constitutional_effective_time: TimePoint,
    pub observation_time: TimePoint,
    pub processing_time: TimePoint,
    pub implementation_version: ImplementationVersion,
    pub admitted_source_set: Option<AdmittedSourceSet>,
}

impl OperationalConstitutionalContext {
    /// This is an immutable operation-scoped representation only. It grants no authority.
    pub fn source(&self, id: &ConstitutionalSourceId) -> Option<&ConstitutionalSourceRef> {
        self.source_set.iter().find(|source| &source.id == id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FindingSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FindingCode {
    MissingSource,
    MissingVersion,
    AmbiguousSource,
    ConflictingInput,
    UnresolvedJurisdiction,
    UnresolvedScope,
    MalformedReference,
    IncompleteCanonicalInput,
    UnresolvedRequirement,
    EvidenceSubjectMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationFinding {
    pub code: FindingCode,
    pub severity: FindingSeverity,
    pub subject: String,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextResolutionResult {
    Resolved(Box<OperationalConstitutionalContext>),
    Indeterminate { findings: Vec<ValidationFinding> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationResult {
    pub valid: bool,
    pub findings: Vec<ValidationFinding>,
    pub authority_established: bool,
    pub conformance_claim: bool,
    pub certification_claim: bool,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            valid: true,
            findings: Vec::new(),
            authority_established: false,
            conformance_claim: false,
            certification_claim: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    MalformedIdentifier(String),
    MalformedReference(String),
}
