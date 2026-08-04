//! Bounded IMP-007 evidence and provenance records.
//!
//! This crate models evidence; it does not collect, persist, transmit, admit,
//! assess, certify, or authorize anything.  In particular, a digest is not a
//! truth claim, provenance is not authority, and a package is not sufficiency.

use constitutional_canonical::{
    CanonicalField, CanonicalObject, CanonicalRepresentationId, CanonicalRepresentationKind,
    CanonicalValue, REPRESENTATION_MODEL_VERSION,
};
use constitutional_contracts::{
    BaselineRef, ConstitutionalSourceRef, EvidenceRecordId, ImplementationMappingId,
    ImplementationVersion, OperationalConstitutionalContext, TimePoint,
};
use constitutional_traceability::{ImplementationMapping, TraceRef};
use std::fmt;

pub const EVIDENCE_PROFILE_ID: &str = "reference-implementation-evidence-provenance";
pub const EVIDENCE_PROFILE_VERSION: &str = "1.0.0";

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
                let value = value.into();
                if value.is_empty() || value.chars().any(|c| c.is_whitespace() || c.is_control()) {
                    return Err(EvidenceError::MalformedIdentifier(value));
                }
                Ok(Self(value))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_type!(EvidenceIdentifier);
id_type!(EvidenceObjectIdentifier);
id_type!(EvidenceSourceIdentifier);
id_type!(EvidenceCollectionIdentifier);
id_type!(EvidencePackageIdentifier);
id_type!(EvidenceOriginIdentifier);
id_type!(ProvenanceEventIdentifier);
id_type!(EvidenceCriterionIdentifier);
id_type!(AdmissionEvaluationIdentifier);
id_type!(SufficiencyAssessmentIdentifier);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceError {
    MalformedIdentifier(String),
    MissingField(&'static str),
    Invalid(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceSubject {
    ConstitutionalSource(String),
    ImplementationRequirement(String),
    ComponentRealization(String),
    ImplementationArtifact(String),
    ImplementationVersion(ImplementationVersion),
    TraceabilityRelation(TraceRef),
    VerificationRequirement(String),
    VerificationActivity(String),
    VerificationObservation(String),
    VerificationFinding(String),
    VerificationAssessment(String),
    AssuranceBasis(String),
    LifecycleGate(String),
    Deviation(String),
    CompatibilityClaim(String),
    ReleaseReadiness(String),
    ActivationReadiness(String),
}

impl EvidenceSubject {
    pub fn key(&self) -> String {
        format!("{self:?}")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceKind {
    SourceDocument,
    ImplementationArtifact,
    TestResult,
    StaticAnalysis,
    CanonicalRoundTrip,
    Traceability,
    SourceBinding,
    VersionBinding,
    DocumentationReview,
    GovernedRecordReview,
    ManualReview,
    LifecycleGate,
    Deviation,
    VerificationObservation,
    VerificationFinding,
    Derived,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceFormat {
    Text,
    Json,
    CanonicalRepresentation,
    Binary,
    Tabular,
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceOrigin {
    pub id: EvidenceOriginIdentifier,
    pub source: EvidenceSourceIdentifier,
    pub producer: String,
    pub collector: Option<String>,
    pub method: String,
    pub produced_at: TimePoint,
    pub collected_at: Option<TimePoint>,
    pub environment: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceContentReference {
    pub locator: String,
    pub format: EvidenceFormat,
    pub representation_version: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceClaim {
    pub statement: String,
    pub declared_scope: String,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceProvenance {
    pub origin: EvidenceOrigin,
    pub events: Vec<ProvenanceEvent>,
    pub lineage: Vec<ProvenanceLineage>,
    pub inputs: Vec<EvidenceReference>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceEvent {
    pub id: ProvenanceEventIdentifier,
    pub action: ProvenanceAction,
    pub actor: String,
    pub occurred_at: TimePoint,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ProvenanceAction {
    Produced,
    Collected,
    Transformed,
    Derived,
    Redacted,
    Corrected,
    Supplemented,
    Packaged,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceLineage {
    pub source: EvidenceReference,
    pub derived: EvidenceIdentifier,
    pub method: String,
    pub conditions: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustodyEvent {
    pub custodian: String,
    pub started_at: TimePoint,
    pub ended_at: Option<TimePoint>,
    pub transferor: Option<String>,
    pub recipient: Option<String>,
    pub basis: String,
    pub integrity_at_transfer: Option<IntegrityStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedCustodyEvent {
    pub id: String,
    pub chain_id: String,
    pub evidence_id: EvidenceObjectIdentifier,
    pub sequence: u64,
    pub predecessor: Option<String>,
    pub custodian: String,
    pub started_at: TimePoint,
    pub ended_at: Option<TimePoint>,
    pub transferor: Option<String>,
    pub recipient: Option<String>,
    pub basis: String,
    pub integrity_at_transfer: Option<IntegrityStatus>,
    pub limitations: Vec<String>,
    pub known_gaps: Vec<String>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegrityAlgorithm {
    Fnv1a64,
    Sha256,
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DigestDescriptor {
    pub algorithm: IntegrityAlgorithm,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegrityDescriptor {
    pub status: IntegrityStatus,
    pub digest: Option<DigestDescriptor>,
    pub reference_state: String,
    pub scope: String,
    pub assessed_by: Option<String>,
    pub assessed_at: Option<TimePoint>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegrityStatus {
    Unassessed,
    Intact,
    Compromised,
    Indeterminate,
    NotApplicable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthenticityStatus {
    Unassessed,
    Supported,
    Disputed,
    Rejected,
    Indeterminate,
    NotApplicable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionStatus {
    NotCollected,
    Collected,
    PartiallyCollected,
    CollectionFailed,
    NotApplicable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssociationStatus {
    Unassociated,
    Associated,
    PackageIncluded,
    ReferenceOnly,
    AssociationDisputed,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicabilityStatus {
    Current,
    Limited,
    Stale,
    Inapplicable,
    SupersededContext,
    Indeterminate,
    HistoricalOnly,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreservationStatus {
    Active,
    Archived,
    HistoricallyPreserved,
    PreservationAtRisk,
    Lost,
    PartiallyLost,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChallengeStatus {
    Unchallenged,
    Challenged,
    Contradicted,
    UnderReview,
    Resolved,
    Unresolved,
    Withdrawn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceObject {
    pub id: EvidenceObjectIdentifier,
    pub subject: EvidenceSubject,
    pub kind: EvidenceKind,
    pub content: EvidenceContentReference,
    pub claim: EvidenceClaim,
    pub relevance: String,
    pub origin: EvidenceOrigin,
    pub provenance: EvidenceProvenance,
    pub custody: Vec<CustodyEvent>,
    pub integrity: IntegrityDescriptor,
    pub authenticity: AuthenticityStatus,
    pub implementation_version: ImplementationVersion,
    pub baseline: Option<BaselineRef>,
    pub source_binding: Option<ConstitutionalSourceRef>,
    pub limitations: Vec<EvidenceLimitation>,
    pub collection_status: CollectionStatus,
    pub association_status: AssociationStatus,
    pub applicability: ApplicabilityStatus,
    pub preservation: PreservationStatus,
    pub challenge: ChallengeStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReference {
    pub id: EvidenceIdentifier,
    pub object_id: EvidenceObjectIdentifier,
    pub version: ImplementationVersion,
    pub scope: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLimitation {
    pub statement: String,
    pub scope: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceConflict {
    pub id: EvidenceIdentifier,
    pub affected: Vec<EvidenceReference>,
    pub context: String,
    pub status: ChallengeStatus,
    pub detail: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRelationship {
    pub from: EvidenceIdentifier,
    pub to: EvidenceIdentifier,
    pub relation: EvidenceRelation,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceRelation {
    Supports,
    DerivedFrom,
    Contradicts,
    Supplements,
    Corrects,
    Redacts,
    IncludedIn,
    Challenges,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidencePackage {
    pub id: EvidencePackageIdentifier,
    pub purpose: String,
    pub scope: String,
    pub baseline: Option<BaselineRef>,
    pub implementation_version: ImplementationVersion,
    pub members: Vec<EvidenceReference>,
    pub exclusions: Vec<String>,
    pub assembly_actor: String,
    pub assembled_at: TimePoint,
    pub limitations: Vec<EvidenceLimitation>,
    pub integrity: Option<IntegrityDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmissionCriterion {
    pub id: EvidenceCriterionIdentifier,
    pub statement: String,
    pub required: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdmissionStatus {
    NotReviewed,
    Admissible,
    AdmissibleWithLimitations,
    Inadmissible,
    Indeterminate,
    Unsupported(String),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmissionEvaluation {
    pub id: AdmissionEvaluationIdentifier,
    pub evidence: EvidenceReference,
    pub criteria: Vec<AdmissionCriterion>,
    pub status: AdmissionStatus,
    pub scope: String,
    pub actor: String,
    pub assessed_at: TimePoint,
    pub reasons: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedEvidenceReference {
    pub evidence: EvidenceReference,
    pub admission: AdmissionEvaluation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SufficiencyCriterion {
    pub id: EvidenceCriterionIdentifier,
    pub statement: String,
    pub required: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SufficiencyStatus {
    Sufficient,
    PartiallySufficient,
    Insufficient,
    Indeterminate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SufficiencyAssessment {
    pub id: SufficiencyAssessmentIdentifier,
    pub target: String,
    pub scope: String,
    pub baseline: Option<BaselineRef>,
    pub implementation_version: ImplementationVersion,
    pub criteria: Vec<SufficiencyCriterion>,
    pub admitted: Vec<AdmittedEvidenceReference>,
    pub status: SufficiencyStatus,
    pub limitations: Vec<String>,
    pub assessed_at: TimePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceFindingCode {
    DuplicateIdentifier,
    MissingSubject,
    MissingClaim,
    MissingRelevance,
    MalformedOrigin,
    MissingProvenance,
    BrokenLineage,
    InvalidCustodyOrder,
    UnsupportedIntegrityAlgorithm,
    MalformedDigest,
    DigestMismatch,
    VersionMismatch,
    MissingAdmissionCriterion,
    SubjectMismatch,
    BlockingIntegrityFailure,
    UnsupportedAdmissionStatus,
    SufficiencyWithoutAdmission,
    ScopeExpansion,
    UnresolvedConflict,
    DuplicatePackageMember,
    IncompletePackage,
    ProhibitedClaim,
    MissingCustodyField,
    CustodyIdentityMismatch,
    MissingCustodyPredecessor,
    CustodySequenceMismatch,
    CustodyGap,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceFinding {
    pub code: EvidenceFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceValidationResult {
    pub valid: bool,
    pub findings: Vec<EvidenceFinding>,
    pub authority_created: bool,
    pub truth_established: bool,
    pub assurance_issued: bool,
    pub conformance_established: bool,
}

impl EvidenceValidationResult {
    fn new(findings: Vec<EvidenceFinding>) -> Self {
        Self {
            valid: findings.iter().all(|f| !f.fatal),
            findings,
            authority_created: false,
            truth_established: false,
            assurance_issued: false,
            conformance_established: false,
        }
    }
}

pub fn compare_digest(
    expected: &DigestDescriptor,
    actual: &DigestDescriptor,
) -> Result<bool, EvidenceError> {
    validate_digest_descriptor(expected)?;
    validate_digest_descriptor(actual)?;
    if expected.algorithm != actual.algorithm {
        return Err(EvidenceError::Invalid("digest algorithms differ".into()));
    }
    Ok(expected.value == actual.value)
}

pub fn validate_digest(
    expected: &DigestDescriptor,
    observed: &DigestDescriptor,
    subject: &str,
) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    if let Err(_error) = validate_digest_descriptor(expected) {
        findings.push(finding(
            if matches!(expected.algorithm, IntegrityAlgorithm::Other(_)) {
                EvidenceFindingCode::UnsupportedIntegrityAlgorithm
            } else {
                EvidenceFindingCode::MalformedDigest
            },
            subject.into(),
            "expected digest descriptor is invalid",
            true,
        ));
    }
    if let Err(_error) = validate_digest_descriptor(observed) {
        findings.push(finding(
            if matches!(observed.algorithm, IntegrityAlgorithm::Other(_)) {
                EvidenceFindingCode::UnsupportedIntegrityAlgorithm
            } else {
                EvidenceFindingCode::MalformedDigest
            },
            subject.into(),
            "observed digest descriptor is invalid",
            true,
        ));
    }
    if findings.is_empty()
        && (expected.algorithm != observed.algorithm || expected.value != observed.value)
    {
        findings.push(finding(
            EvidenceFindingCode::DigestMismatch,
            subject.into(),
            "valid expected and observed digests differ",
            true,
        ));
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_version_binding(
    actual: &ImplementationVersion,
    expected: &ImplementationVersion,
    subject: &str,
) -> EvidenceValidationResult {
    if actual == expected {
        EvidenceValidationResult::new(Vec::new())
    } else {
        EvidenceValidationResult::new(vec![finding(
            EvidenceFindingCode::VersionMismatch,
            subject.into(),
            "evidence version does not match the bounded validation context",
            true,
        )])
    }
}

pub fn validate_unique_identifiers(ids: &[EvidenceIdentifier]) -> EvidenceValidationResult {
    let mut seen = std::collections::BTreeSet::new();
    let mut findings = Vec::new();
    for id in ids {
        if !seen.insert(id) {
            findings.push(finding(
                EvidenceFindingCode::DuplicateIdentifier,
                id.to_string(),
                "evidence identifier is duplicated in the bounded collection",
                true,
            ));
        }
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_subject_presence(
    subject: Option<&EvidenceSubject>,
    reference: &str,
) -> EvidenceValidationResult {
    if subject.is_some() {
        EvidenceValidationResult::new(Vec::new())
    } else {
        EvidenceValidationResult::new(vec![finding(
            EvidenceFindingCode::MissingSubject,
            reference.into(),
            "evidence subject is required at this validation boundary",
            true,
        )])
    }
}

pub fn validate_scope_containment(
    parent_scope: &str,
    child_scope: &str,
    subject: &str,
) -> EvidenceValidationResult {
    let contained = child_scope == parent_scope
        || child_scope
            .strip_prefix(parent_scope)
            .is_some_and(|suffix| suffix.starts_with('/'));
    if contained {
        EvidenceValidationResult::new(Vec::new())
    } else {
        EvidenceValidationResult::new(vec![finding(
            EvidenceFindingCode::ScopeExpansion,
            subject.into(),
            "derived or packaged evidence scope exceeds its source scope",
            true,
        )])
    }
}

pub fn validate_digest_descriptor(digest: &DigestDescriptor) -> Result<(), EvidenceError> {
    let (prefix, length) = match digest.algorithm {
        IntegrityAlgorithm::Fnv1a64 => ("fnv1a64:", 16),
        IntegrityAlgorithm::Sha256 => ("sha256:", 64),
        IntegrityAlgorithm::Other(_) => {
            return Err(EvidenceError::Invalid(
                "unsupported integrity algorithm".into(),
            ));
        }
    };
    let Some(value) = digest.value.strip_prefix(prefix) else {
        return Err(EvidenceError::Invalid("malformed digest prefix".into()));
    };
    if value.len() != length
        || value.is_empty()
        || !value.chars().all(|character| character.is_ascii_hexdigit())
    {
        return Err(EvidenceError::Invalid("malformed digest value".into()));
    }
    Ok(())
}

pub fn validate_integrity(
    integrity: &IntegrityDescriptor,
    subject: &str,
) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    if matches!(integrity.status, IntegrityStatus::Compromised) {
        findings.push(finding(
            EvidenceFindingCode::BlockingIntegrityFailure,
            subject.into(),
            "compromised integrity is a blocking finding",
            true,
        ));
    }
    if let Some(digest) = integrity.digest.as_ref() {
        match (&digest.algorithm, validate_digest_descriptor(digest)) {
            (IntegrityAlgorithm::Other(_), _) => findings.push(finding(
                EvidenceFindingCode::UnsupportedIntegrityAlgorithm,
                subject.into(),
                "integrity algorithm is outside the bounded profile",
                true,
            )),
            (_, Err(_)) => findings.push(finding(
                EvidenceFindingCode::MalformedDigest,
                subject.into(),
                "integrity digest has an invalid prefix, length, or character set",
                true,
            )),
            _ => {}
        }
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_custody(events: &[CustodyEvent]) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    for pair in events.windows(2) {
        if pair[0].started_at.as_str() > pair[1].started_at.as_str() {
            findings.push(finding(
                EvidenceFindingCode::InvalidCustodyOrder,
                "custody".into(),
                "custody events are not chronological",
                true,
            ));
        }
    }
    for event in events {
        if event.custodian.trim().is_empty() || event.basis.trim().is_empty() {
            findings.push(finding(
                EvidenceFindingCode::InvalidCustodyOrder,
                event.custodian.clone(),
                "custody event requires a custodian and basis",
                true,
            ));
        }
        if event.transferor.is_some() && event.transferor == event.recipient {
            findings.push(finding(
                EvidenceFindingCode::InvalidCustodyOrder,
                event.custodian.clone(),
                "custody event cannot transfer to itself",
                true,
            ));
        }
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_custody_chain(
    events: &[CustodyEvent],
    evidence: &EvidenceObjectIdentifier,
    require_gapless: bool,
) -> EvidenceValidationResult {
    let mut findings = validate_custody(events).findings;
    if events
        .first()
        .is_some_and(|event| event.transferor.is_some())
    {
        findings.push(finding(
            EvidenceFindingCode::InvalidCustodyOrder,
            evidence.to_string(),
            "the first custody event cannot be an orphaned transfer",
            true,
        ));
    }
    for pair in events.windows(2) {
        let expected_transferor = pair[0].recipient.as_ref().unwrap_or(&pair[0].custodian);
        if pair[1].transferor.as_ref() != Some(expected_transferor) {
            findings.push(finding(
                EvidenceFindingCode::InvalidCustodyOrder,
                evidence.to_string(),
                "custody transfer lineage does not connect to the next event",
                true,
            ));
        }
        if let Some(end) = &pair[0].ended_at {
            if end.as_str() > pair[1].started_at.as_str() {
                findings.push(finding(
                    EvidenceFindingCode::InvalidCustodyOrder,
                    evidence.to_string(),
                    "custody intervals overlap",
                    true,
                ));
            } else if require_gapless && end.as_str() != pair[1].started_at.as_str() {
                findings.push(finding(
                    EvidenceFindingCode::InvalidCustodyOrder,
                    evidence.to_string(),
                    "custody interval contains an unexplained gap",
                    true,
                ));
            }
        }
    }
    EvidenceValidationResult::new(findings)
}

/// Validates custody in the context of the evidence object that owns it.
/// Gaplessness is conditional; custody never establishes an unrelated
/// ownership, authenticity, authority, approval, or truth conclusion.
pub fn validate_custody_for_evidence(
    events: &[CustodyEvent],
    evidence: &EvidenceObjectIdentifier,
    limitations: &[EvidenceLimitation],
    challenge: &ChallengeStatus,
    require_gapless: bool,
) -> EvidenceValidationResult {
    let mut result = validate_custody_chain(events, evidence, require_gapless);
    let gap_is_recorded = limitations.iter().any(|limitation| {
        let statement = limitation.statement.to_ascii_lowercase();
        statement.contains("gap") || statement.contains("challenge")
    }) || !matches!(challenge, ChallengeStatus::Unchallenged);
    for pair in events.windows(2) {
        let gap = match &pair[0].ended_at {
            Some(end) => end.as_str() < pair[1].started_at.as_str(),
            None => true,
        };
        if gap && !gap_is_recorded {
            result.findings.push(finding(
                EvidenceFindingCode::MissingCustodyField,
                evidence.to_string(),
                "known custody gap requires a limitation or challenge condition",
                true,
            ));
        }
    }
    result.valid = !result.findings.iter().any(|finding| finding.fatal);
    result
}

pub fn validate_typed_custody_chain(
    events: &[TypedCustodyEvent],
    require_gapless: bool,
) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    let mut ids = std::collections::BTreeSet::new();
    let mut sequences = std::collections::BTreeSet::new();
    let mut ordered = events.to_vec();
    ordered.sort_by_key(|event| event.sequence);
    for event in &ordered {
        if event.id.trim().is_empty()
            || event.chain_id.trim().is_empty()
            || event.custodian.trim().is_empty()
            || event.basis.trim().is_empty()
            || event
                .limitations
                .iter()
                .any(|limitation| limitation.trim().is_empty())
            || event.known_gaps.iter().any(|gap| gap.trim().is_empty())
        {
            findings.push(finding(
                EvidenceFindingCode::MissingCustodyField,
                event.id.clone(),
                "typed custody event is missing a required field",
                true,
            ));
        }
        if !ids.insert(event.id.clone()) {
            findings.push(finding(
                EvidenceFindingCode::CustodyIdentityMismatch,
                event.id.clone(),
                "custody event identity is duplicated",
                true,
            ));
        }
        if !sequences.insert(event.sequence) {
            findings.push(finding(
                EvidenceFindingCode::CustodySequenceMismatch,
                event.id.clone(),
                "custody sequence is duplicated",
                true,
            ));
        }
        if event.sequence == 0 {
            if event.predecessor.is_some() || event.transferor.is_some() {
                findings.push(finding(
                    EvidenceFindingCode::MissingCustodyPredecessor,
                    event.id.clone(),
                    "origin custody event cannot claim a predecessor or transferor",
                    true,
                ));
            }
        } else {
            let Some(previous) = ordered
                .iter()
                .find(|candidate| candidate.sequence + 1 == event.sequence)
            else {
                findings.push(finding(
                    EvidenceFindingCode::MissingCustodyPredecessor,
                    event.id.clone(),
                    "non-origin custody event has no resolvable sequence predecessor",
                    true,
                ));
                continue;
            };
            if event.predecessor.as_deref() != Some(previous.id.as_str()) {
                findings.push(finding(
                    EvidenceFindingCode::MissingCustodyPredecessor,
                    event.id.clone(),
                    "custody predecessor does not resolve to the prior sequence event",
                    true,
                ));
            }
            if previous.chain_id != event.chain_id
                || previous.evidence_id != event.evidence_id
                || event.predecessor.as_deref() == Some(event.id.as_str())
            {
                findings.push(finding(
                    EvidenceFindingCode::CustodyIdentityMismatch,
                    event.id.clone(),
                    "custody predecessor changes chain/evidence identity or self-references",
                    true,
                ));
            }
            if previous.implementation_version != event.implementation_version {
                findings.push(finding(
                    EvidenceFindingCode::VersionMismatch,
                    event.id.clone(),
                    "custody chain implementation versions differ",
                    true,
                ));
            }
            let expected_transferor = previous.recipient.as_ref().unwrap_or(&previous.custodian);
            if event.transferor.as_ref() != Some(expected_transferor) {
                findings.push(finding(
                    EvidenceFindingCode::InvalidCustodyOrder,
                    event.id.clone(),
                    "custody transferor does not continue the previous custodian",
                    true,
                ));
            }
            if let Some(end) = &previous.ended_at {
                if end.as_str() > event.started_at.as_str() {
                    findings.push(finding(
                        EvidenceFindingCode::InvalidCustodyOrder,
                        event.id.clone(),
                        "custody intervals overlap",
                        true,
                    ));
                } else if end.as_str() != event.started_at.as_str() {
                    let gap_recorded = event
                        .known_gaps
                        .iter()
                        .chain(event.limitations.iter())
                        .any(|condition| condition.to_ascii_lowercase().contains("gap"));
                    if !gap_recorded {
                        findings.push(finding(
                            EvidenceFindingCode::MissingCustodyField,
                            event.id.clone(),
                            "known custody gap requires a limitation or explicit gap condition",
                            true,
                        ));
                    }
                    findings.push(finding(
                        EvidenceFindingCode::CustodyGap,
                        event.id.clone(),
                        "custody gap must be represented as a limitation or challenge condition",
                        require_gapless,
                    ));
                }
            } else {
                let gap_recorded = event
                    .known_gaps
                    .iter()
                    .chain(event.limitations.iter())
                    .any(|condition| condition.to_ascii_lowercase().contains("gap"));
                if !gap_recorded {
                    findings.push(finding(
                        EvidenceFindingCode::MissingCustodyField,
                        event.id.clone(),
                        "known custody gap requires a limitation or explicit gap condition",
                        true,
                    ));
                }
                findings.push(finding(
                    EvidenceFindingCode::CustodyGap,
                    event.id.clone(),
                    "open-ended custody interval cannot precede a later event without an explicit gap condition",
                    require_gapless,
                ));
            }
        }
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_package(
    package: &EvidencePackage,
    known_objects: &[EvidenceObjectIdentifier],
    required_members: &[EvidenceObjectIdentifier],
) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for member in &package.members {
        if !seen.insert(member.object_id.clone()) {
            findings.push(finding(
                EvidenceFindingCode::DuplicatePackageMember,
                member.object_id.to_string(),
                "package member is duplicated",
                true,
            ));
        }
        if !known_objects.contains(&member.object_id) {
            findings.push(finding(
                EvidenceFindingCode::SubjectMismatch,
                member.object_id.to_string(),
                "package member is not resolved",
                true,
            ));
        }
        if member.version != package.implementation_version {
            findings.push(finding(
                EvidenceFindingCode::VersionMismatch,
                member.object_id.to_string(),
                "package member version differs from package context",
                true,
            ));
        }
        findings.extend(
            validate_scope_containment(
                &package.scope,
                &member.scope,
                &member.object_id.to_string(),
            )
            .findings,
        );
    }
    for required in required_members {
        if !seen.contains(required) {
            findings.push(finding(
                EvidenceFindingCode::IncompletePackage,
                required.to_string(),
                "required package member is missing",
                true,
            ));
        }
    }
    if package.purpose.trim().is_empty() || package.scope.trim().is_empty() {
        findings.push(finding(
            EvidenceFindingCode::IncompletePackage,
            package.id.to_string(),
            "package purpose and scope are required",
            true,
        ));
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_conflict(conflict: &EvidenceConflict) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    if conflict.affected.len() < 2 {
        findings.push(finding(
            EvidenceFindingCode::UnresolvedConflict,
            conflict.id.to_string(),
            "a conflict requires at least two affected references",
            true,
        ));
    }
    if matches!(conflict.status, ChallengeStatus::Resolved) && conflict.detail.trim().is_empty() {
        findings.push(finding(
            EvidenceFindingCode::UnresolvedConflict,
            conflict.id.to_string(),
            "resolved conflict requires a resolution basis",
            true,
        ));
    }
    EvidenceValidationResult::new(findings)
}

/// Validates the source-required identity references used by corrections,
/// supplements, packages, and conflict records without requiring a global
/// evidence graph.
pub fn validate_evidence_history_references(
    known_objects: &[EvidenceObjectIdentifier],
    references: &[EvidenceReference],
    relationships: &[EvidenceRelationship],
    conflicts: &[EvidenceConflict],
) -> EvidenceValidationResult {
    let mut findings = Vec::new();
    for reference in references {
        if !known_objects.contains(&reference.object_id) {
            findings.push(finding(
                EvidenceFindingCode::SubjectMismatch,
                reference.id.to_string(),
                "historical evidence reference does not resolve to a known object",
                true,
            ));
        }
        if reference.scope.trim().is_empty() {
            findings.push(finding(
                EvidenceFindingCode::MissingSubject,
                reference.id.to_string(),
                "historical evidence reference requires an applicable scope",
                true,
            ));
        }
    }
    for relationship in relationships {
        if !references
            .iter()
            .any(|reference| reference.id == relationship.from)
            || !references
                .iter()
                .any(|reference| reference.id == relationship.to)
        {
            findings.push(finding(
                EvidenceFindingCode::UnresolvedConflict,
                relationship.from.to_string(),
                "evidence relationship references an unresolved historical record",
                true,
            ));
        }
        if relationship.from == relationship.to {
            findings.push(finding(
                EvidenceFindingCode::UnresolvedConflict,
                relationship.from.to_string(),
                "evidence correction or supplementation cannot replace itself",
                true,
            ));
        }
    }
    for conflict in conflicts {
        for reference in &conflict.affected {
            if !references.iter().any(|known| known.id == reference.id) {
                findings.push(finding(
                    EvidenceFindingCode::UnresolvedConflict,
                    conflict.id.to_string(),
                    "conflict record contains an unresolved evidence identity",
                    true,
                ));
            }
        }
    }
    EvidenceValidationResult::new(findings)
}

pub fn validate_evidence_object(object: &EvidenceObject) -> EvidenceValidationResult {
    let mut f = Vec::new();
    if object.claim.statement.trim().is_empty() {
        f.push(finding(
            EvidenceFindingCode::MissingClaim,
            object.id.to_string(),
            "evidence claim is required",
            true,
        ));
    }
    if object.relevance.trim().is_empty() {
        f.push(finding(
            EvidenceFindingCode::MissingRelevance,
            object.id.to_string(),
            "relevance statement is required",
            true,
        ));
    }
    if object.origin.producer.trim().is_empty() {
        f.push(finding(
            EvidenceFindingCode::MalformedOrigin,
            object.id.to_string(),
            "producer is required",
            true,
        ));
    }
    if object.provenance.events.is_empty() {
        f.push(finding(
            EvidenceFindingCode::MissingProvenance,
            object.id.to_string(),
            "at least one provenance event is required",
            true,
        ));
    }
    f.extend(validate_integrity(&object.integrity, &object.id.to_string()).findings);
    let custody = validate_custody_for_evidence(
        &object.custody,
        &object.id,
        &object.limitations,
        &object.challenge,
        false,
    );
    f.extend(custody.findings);
    if object
        .provenance
        .lineage
        .iter()
        .any(|lineage| lineage.method.trim().is_empty())
    {
        f.push(finding(
            EvidenceFindingCode::BrokenLineage,
            object.id.to_string(),
            "provenance lineage method is required",
            true,
        ));
    }
    if object
        .claim
        .statement
        .to_ascii_lowercase()
        .contains("constitutional truth")
        || object
            .claim
            .statement
            .to_ascii_lowercase()
            .contains("certif")
    {
        f.push(finding(
            EvidenceFindingCode::ProhibitedClaim,
            object.id.to_string(),
            "evidence cannot establish truth or certification",
            true,
        ));
    }
    EvidenceValidationResult::new(f)
}

pub fn validate_admission(
    evaluation: &AdmissionEvaluation,
    object: &EvidenceObject,
) -> EvidenceValidationResult {
    let mut f = Vec::new();
    if matches!(evaluation.status, AdmissionStatus::Unsupported(_)) {
        f.push(finding(
            EvidenceFindingCode::UnsupportedAdmissionStatus,
            evaluation.id.to_string(),
            "admission status is outside the bounded profile",
            true,
        ));
    }
    if evaluation.criteria.is_empty() {
        f.push(finding(
            EvidenceFindingCode::MissingAdmissionCriterion,
            evaluation.id.to_string(),
            "admission requires an explicit criterion",
            true,
        ));
    }
    if evaluation.evidence.object_id != object.id {
        f.push(finding(
            EvidenceFindingCode::SubjectMismatch,
            evaluation.id.to_string(),
            "admission reference does not identify the evaluated object",
            true,
        ));
    }
    if object.integrity.status == IntegrityStatus::Compromised {
        f.push(finding(
            EvidenceFindingCode::BlockingIntegrityFailure,
            object.id.to_string(),
            "integrity failure blocks this bounded admission",
            true,
        ));
    }
    EvidenceValidationResult::new(f)
}

pub fn validate_sufficiency(assessment: &SufficiencyAssessment) -> EvidenceValidationResult {
    let mut f = Vec::new();
    if assessment.criteria.is_empty() {
        f.push(finding(
            EvidenceFindingCode::MissingAdmissionCriterion,
            assessment.id.to_string(),
            "sufficiency requires explicit target criteria",
            true,
        ));
    }
    if assessment.admitted.is_empty() {
        f.push(finding(
            EvidenceFindingCode::SufficiencyWithoutAdmission,
            assessment.id.to_string(),
            "sufficiency cannot be assessed without admitted evidence",
            true,
        ));
    }
    EvidenceValidationResult::new(f)
}

fn finding(
    code: EvidenceFindingCode,
    subject: String,
    detail: &str,
    fatal: bool,
) -> EvidenceFinding {
    EvidenceFinding {
        code,
        subject,
        detail: detail.into(),
        fatal,
    }
}

pub fn evidence_canonical(object: &EvidenceObject) -> CanonicalObject {
    let fields = vec![
        field(
            "evidence_object_id",
            CanonicalValue::Text(object.id.to_string()),
            true,
            true,
        ),
        field(
            "subject",
            CanonicalValue::Text(object.subject.key()),
            true,
            true,
        ),
        field(
            "kind",
            CanonicalValue::Text(format!("{:?}", object.kind)),
            true,
            true,
        ),
        field(
            "claim",
            CanonicalValue::Text(object.claim.statement.clone()),
            true,
            false,
        ),
        field(
            "implementation_version",
            CanonicalValue::Text(object.implementation_version.to_string()),
            true,
            true,
        ),
        field(
            "integrity_status",
            CanonicalValue::Text(format!("{:?}", object.integrity.status)),
            true,
            false,
        ),
    ];
    let mut fields = fields;
    fields.sort_by(|left, right| left.name.cmp(&right.name));
    CanonicalObject {
        representation_id: CanonicalRepresentationId::new(format!("evidence-{}", object.id))
            .expect("validated evidence id"),
        kind: CanonicalRepresentationKind::EvidenceReference,
        representation_version: REPRESENTATION_MODEL_VERSION.into(),
        schema_id: EVIDENCE_PROFILE_ID.into(),
        source_binding: object.source_binding.clone(),
        implementation_version: Some(object.implementation_version.clone()),
        required_fields: fields
            .iter()
            .filter(|f| f.required)
            .map(|f| f.name.clone())
            .collect(),
        extension_namespaces: Vec::new(),
        fields,
    }
}

fn field(name: &str, value: CanonicalValue, required: bool, identity: bool) -> CanonicalField {
    CanonicalField {
        name: name.into(),
        value,
        required,
        identity_participates: identity,
        extension: false,
    }
}

/// Compatibility constructor for the earlier mapping evidence slice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRecord {
    pub id: EvidenceRecordId,
    pub subject: String,
    pub implementation_version: ImplementationVersion,
    pub representation_format: constitutional_contracts::RepresentationFormatId,
    pub representation_version: constitutional_contracts::RepresentationVersion,
    pub test_reference: String,
    pub mapping_id: ImplementationMappingId,
    pub observed_at: TimePoint,
    pub result: String,
    pub conclusion: Option<String>,
}
impl EvidenceRecord {
    pub fn for_mapping(
        id: EvidenceRecordId,
        subject: impl Into<String>,
        context: &OperationalConstitutionalContext,
        mapping: &ImplementationMapping,
        test_reference: impl Into<String>,
        result: impl Into<String>,
    ) -> Result<Self, constitutional_contracts::ValidationFinding> {
        if mapping.implementation_version != context.implementation_version {
            return Err(constitutional_contracts::ValidationFinding {
                code: constitutional_contracts::FindingCode::EvidenceSubjectMismatch,
                severity: constitutional_contracts::FindingSeverity::Error,
                subject: id.to_string(),
                detail:
                    "evidence implementation version does not match the exact context under test"
                        .into(),
            });
        }
        Ok(Self {
            id,
            subject: subject.into(),
            implementation_version: context.implementation_version.clone(),
            representation_format: constitutional_contracts::RepresentationFormatId::new(
                "macs-reference-canonical-context",
            )
            .expect("constant"),
            representation_version: constitutional_contracts::RepresentationVersion::new("1.0.0")
                .expect("constant"),
            test_reference: test_reference.into(),
            mapping_id: mapping.mapping_id.clone(),
            observed_at: context.observation_time.clone(),
            result: result.into(),
            conclusion: None,
        })
    }
}
