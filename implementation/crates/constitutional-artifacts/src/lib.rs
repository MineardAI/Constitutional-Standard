//! Bounded, non-sovereign realization of CORE-003 artifacts and provenance.
//!
//! An artifact can represent constitutional reality without becoming that
//! reality. This crate does not store, publish, sign, certify, or execute.

use constitutional_contracts::{
    ConstitutionalSourceRef, ContextId, ImplementationVersion, SourceSetId,
};
use std::fmt;

pub const PROFILE_ID: &str = "reference-implementation-artifacts-provenance";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const PRIMARY_SOURCE_VERSION: &str = "0.1.1";
pub const ADDENDUM_SOURCE_VERSION: &str = "0.2.0";
pub const SOURCE_STATUS: &str = "Stabilization Draft";
pub const SOURCE_REVISION: &str = "Artifact Identity-Instance Stabilization";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainRef(String);
impl DomainRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ArtifactError> {
        let value = value.into();
        if value.is_empty()
            || value
                .chars()
                .any(|c| c.is_control() || c == '\n' || c == '\r')
        {
            return Err(ArtifactError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for DomainRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type ArtifactId = DomainRef;
pub type ArtifactInstanceId = DomainRef;
pub type RepresentationId = DomainRef;
pub type ArtifactRoleRef = DomainRef;
pub type ProvenanceRef = DomainRef;
pub type LineageRef = DomainRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactCategory {
    Constitutional,
    NonConstitutional,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArtifactKind {
    ConstitutionalRecord,
    NormativeArtifact,
    EvidenceArtifact,
    ReferenceArtifact,
    HistoricalArtifact,
    OperationalArtifact,
    Other(DomainRef),
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArtifactRole {
    Representing,
    Evidencing,
    Constituting,
    Preserving,
    Publishing,
    Recording,
    Referencing,
    HistoricalPreservation,
    OperationalParticipation,
    Other(DomainRef),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIdentity {
    pub id: ArtifactId,
    pub kind: ArtifactKind,
    pub represented_subject: Option<DomainRef>,
    pub revision: DomainRef,
    pub roles: Vec<ArtifactRole>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInstanceIdentity {
    pub id: ArtifactInstanceId,
    pub artifact: ArtifactId,
    pub context: DomainRef,
    pub serialization: DomainRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInstance {
    pub identity: ArtifactInstanceIdentity,
    pub identity_preservation_basis: Option<DomainRef>,
    pub copied_from: Option<ArtifactInstanceId>,
    pub transformed_from: Option<ArtifactInstanceId>,
    pub replayed_from: Option<ArtifactInstanceId>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionBasis {
    pub source: ConstitutionalSourceRef,
    pub role: ArtifactRole,
    pub expressly_recognized: bool,
    pub authority_reference: Option<DomainRef>,
    pub identity_reference: Option<DomainRef>,
    pub provenance: ProvenanceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactProvenance {
    pub origin: Option<ProvenanceRef>,
    pub authority: Option<ProvenanceRef>,
    pub identity: Option<ProvenanceRef>,
    pub transformation: Option<ProvenanceRef>,
    pub version: Option<ProvenanceRef>,
    pub custody: Option<ProvenanceRef>,
    pub publication: Option<ProvenanceRef>,
    pub lineage: Vec<LineageRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntegrityClaim {
    pub representational: Option<bool>,
    pub identity: Option<bool>,
    pub instance: Option<bool>,
    pub lineage: Option<bool>,
    pub transformation: Option<bool>,
    pub version: Option<bool>,
    pub custody: Option<bool>,
    pub publication: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactRelationshipKind {
    Represents,
    Evidences,
    Constitutes,
    Records,
    Preserves,
    Publishes,
    References,
    Contains,
    DerivedFrom,
    TransformedFrom,
    CopiedFrom,
    InstantiatedFrom,
    Supersedes,
    SupersededBy,
    Replays,
    HistoricallyPreserves,
    HeldInCustodyBy,
    PossessedBy,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactRelationship {
    pub from: ArtifactId,
    pub kind: ArtifactRelationshipKind,
    pub to: ArtifactId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactClaim {
    pub identity: Option<ArtifactIdentity>,
    pub instance: Option<ArtifactInstance>,
    pub category: ArtifactCategory,
    pub recognition: Option<RecognitionBasis>,
    pub provenance: ArtifactProvenance,
    pub integrity: Option<ArtifactIntegrityClaim>,
    pub representation_of: Option<RepresentationId>,
    pub historical: bool,
    pub replay: bool,
    pub contained: Vec<ArtifactId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub available_sources: Vec<ConstitutionalSourceRef>,
    pub recognized_roles: Vec<ArtifactRole>,
    pub historical_context: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactEvaluationRequest {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub claim: Option<ArtifactClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactDetermination {
    Recognized,
    NotRecognized,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactInstanceDetermination {
    ValidInstance,
    InvalidInstance,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityPreservationDetermination {
    Preserved,
    NotPreserved,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceSufficiencyDetermination {
    Sufficient,
    Insufficient,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegrityDetermination {
    Satisfied,
    Unsatisfied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactFindingCode {
    MissingArtifactIdentity,
    IncompleteArtifactIdentity,
    MissingRecognitionBasis,
    RecognitionNotExpresslySupported,
    RecognitionSourceUnavailable,
    UnsupportedArtifactRole,
    RepresentationNonIdentity,
    InstanceIdentityCollapse,
    MissingPreservationBasis,
    TransformationLineageMissing,
    ReplayNonCreation,
    CompoundIdentityMerge,
    HistoricalCurrentApplicability,
    ProvenanceInsufficient,
    ProvenanceNonAuthority,
    IntegrityNonTruth,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
    ConflictingRelationship,
    UnsupportedSourceVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactFinding {
    pub code: ArtifactFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactEvaluationResult {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: DomainRef,
    pub claim: Option<ArtifactClaim>,
    pub recognition: ArtifactDetermination,
    pub instance: ArtifactInstanceDetermination,
    pub identity_preservation: IdentityPreservationDetermination,
    pub provenance: ProvenanceSufficiencyDetermination,
    pub integrity: IntegrityDetermination,
    pub relationships: Vec<ArtifactRelationship>,
    pub findings: Vec<ArtifactFinding>,
    pub non_claims: Vec<String>,
}

pub fn evaluate_artifact_recognition(
    request: &ArtifactEvaluationRequest,
    context: &ArtifactEvaluationContext,
) -> ArtifactEvaluationResult {
    let mut findings = validate_request(request, context);
    let mut recognition = ArtifactDetermination::Indeterminate;
    let mut instance = ArtifactInstanceDetermination::NotApplicable;
    let mut preservation = IdentityPreservationDetermination::NotApplicable;
    let mut provenance = ProvenanceSufficiencyDetermination::Indeterminate;
    let mut integrity = IntegrityDetermination::NotApplicable;
    if findings.iter().any(|item| item.fatal) {
        recognition = ArtifactDetermination::InvalidRequest;
    }
    if recognition == ArtifactDetermination::InvalidRequest {
        let claim = request.claim.clone().unwrap_or_else(|| ArtifactClaim {
            identity: None,
            instance: None,
            category: ArtifactCategory::NonConstitutional,
            recognition: None,
            provenance: ArtifactProvenance {
                origin: None,
                authority: None,
                identity: None,
                transformation: None,
                version: None,
                custody: None,
                publication: None,
                lineage: Vec::new(),
            },
            integrity: None,
            representation_of: None,
            historical: false,
            replay: false,
            contained: Vec::new(),
        });
        return result(
            request,
            &claim,
            recognition,
            instance,
            preservation,
            provenance,
            integrity,
            findings,
        );
    }
    if recognition != ArtifactDetermination::InvalidRequest {
        let claim = request.claim.as_ref().expect("validated claim");
        let Some(identity) = claim.identity.as_ref() else {
            findings.push(finding(
                ArtifactFindingCode::MissingArtifactIdentity,
                "identity",
                "artifact identity is required",
                true,
            ));
            recognition = ArtifactDetermination::InvalidRequest;
            return result(
                request,
                claim,
                recognition,
                instance,
                preservation,
                provenance,
                integrity,
                findings,
            );
        };
        if identity.id.as_str().is_empty() || identity.revision.as_str().is_empty() {
            findings.push(finding(
                ArtifactFindingCode::IncompleteArtifactIdentity,
                "identity",
                "artifact identity lacks identity-significant fields",
                true,
            ));
            return result(
                request,
                claim,
                ArtifactDetermination::InvalidRequest,
                instance,
                preservation,
                provenance,
                integrity,
                findings,
            );
        }
        if claim.category == ArtifactCategory::Constitutional {
            match &claim.recognition {
                Some(basis)
                    if basis.expressly_recognized
                        && context
                            .available_sources
                            .iter()
                            .any(|source| source == &basis.source)
                        && context
                            .recognized_roles
                            .iter()
                            .any(|role| role == &basis.role) =>
                {
                    recognition = ArtifactDetermination::Recognized
                }
                Some(basis)
                    if !context
                        .available_sources
                        .iter()
                        .any(|source| source == &basis.source) =>
                {
                    findings.push(finding(
                        ArtifactFindingCode::RecognitionSourceUnavailable,
                        basis.source.id.as_str(),
                        "recognition source is unavailable in the bound source set",
                        true,
                    ));
                    recognition = ArtifactDetermination::Denied;
                }
                Some(_) => {
                    findings.push(finding(
                        ArtifactFindingCode::RecognitionNotExpresslySupported,
                        "recognition",
                        "recognition basis is not expressly supported by the bound profile",
                        true,
                    ));
                    recognition = ArtifactDetermination::Denied;
                }
                None => {
                    findings.push(finding(
                        ArtifactFindingCode::MissingRecognitionBasis,
                        "recognition",
                        "constitutional artifact recognition requires an explicit governing basis",
                        true,
                    ));
                    recognition = ArtifactDetermination::Denied;
                }
            }
        } else {
            recognition = ArtifactDetermination::Recognized;
        }
        if claim.representation_of.is_some()
            && claim.category == ArtifactCategory::Constitutional
            && claim.recognition.is_none()
        {
            findings.push(finding(
                ArtifactFindingCode::RepresentationNonIdentity,
                "representation",
                "representation does not independently establish constitutional reality",
                true,
            ));
            recognition = ArtifactDetermination::Denied;
        }
        if claim.historical && !context.historical_context {
            findings.push(finding(
                ArtifactFindingCode::HistoricalCurrentApplicability,
                "historical",
                "historical preservation has no current-applicability basis",
                false,
            ));
        }
        if let Some(item) = &claim.instance {
            instance = evaluate_instance(item, claim);
            preservation = if item.identity_preservation_basis.is_some() {
                IdentityPreservationDetermination::Preserved
            } else {
                findings.push(finding(
                    ArtifactFindingCode::MissingPreservationBasis,
                    "instance",
                    "identity-preserving instantiation requires an admitted basis",
                    true,
                ));
                IdentityPreservationDetermination::NotPreserved
            };
        }
        provenance = evaluate_provenance(&claim.provenance, context, &mut findings);
        if let Some(value) = &claim.integrity {
            integrity = evaluate_artifact_integrity(value, &mut findings);
        }
        if claim.replay {
            findings.push(finding(
                ArtifactFindingCode::ReplayNonCreation,
                "replay",
                "replay is a representation and does not create a new act or effect",
                false,
            ));
        }
        if !claim.contained.is_empty() && claim.contained.contains(&identity.id) {
            findings.push(finding(
                ArtifactFindingCode::CompoundIdentityMerge,
                "contained",
                "compound containment must not merge component identity",
                true,
            ));
            recognition = ArtifactDetermination::Conflict;
        }
    }
    result(
        request,
        request.claim.as_ref().expect("validated claim"),
        recognition,
        instance,
        preservation,
        provenance,
        integrity,
        findings,
    )
}

pub fn evaluate_artifact_instance(
    instance: &ArtifactInstance,
    claim: &ArtifactClaim,
) -> ArtifactInstanceDetermination {
    evaluate_instance(instance, claim)
}
pub fn evaluate_identity_preservation(
    instance: &ArtifactInstance,
) -> IdentityPreservationDetermination {
    if instance.identity_preservation_basis.is_some() {
        IdentityPreservationDetermination::Preserved
    } else {
        IdentityPreservationDetermination::NotPreserved
    }
}
pub fn evaluate_provenance(
    provenance: &ArtifactProvenance,
    _context: &ArtifactEvaluationContext,
    findings: &mut Vec<ArtifactFinding>,
) -> ProvenanceSufficiencyDetermination {
    if provenance.origin.is_none() && provenance.lineage.is_empty() {
        findings.push(finding(
            ArtifactFindingCode::ProvenanceInsufficient,
            "provenance",
            "provenance has no origin or lineage reference",
            true,
        ));
        ProvenanceSufficiencyDetermination::Insufficient
    } else {
        ProvenanceSufficiencyDetermination::Sufficient
    }
}
pub fn evaluate_artifact_integrity(
    integrity: &ArtifactIntegrityClaim,
    findings: &mut Vec<ArtifactFinding>,
) -> IntegrityDetermination {
    let values = [
        integrity.representational,
        integrity.identity,
        integrity.instance,
        integrity.lineage,
        integrity.transformation,
        integrity.version,
        integrity.custody,
        integrity.publication,
    ];
    if values.contains(&Some(false)) {
        findings.push(finding(
            ArtifactFindingCode::IntegrityNonTruth,
            "integrity",
            "integrity failure does not establish truth or constitutional invalidity",
            false,
        ));
        IntegrityDetermination::Unsatisfied
    } else if values.iter().all(Option::is_some) {
        IntegrityDetermination::Satisfied
    } else {
        IntegrityDetermination::Indeterminate
    }
}
pub fn evaluate_artifact_relationship(
    relationship: &ArtifactRelationship,
) -> Result<(), ArtifactFinding> {
    if relationship.from == relationship.to
        && matches!(
            relationship.kind,
            ArtifactRelationshipKind::Contains
                | ArtifactRelationshipKind::CopiedFrom
                | ArtifactRelationshipKind::DerivedFrom
        )
    {
        Err(finding(
            ArtifactFindingCode::ConflictingRelationship,
            "relationship",
            "relationship would collapse distinct artifact identities",
            true,
        ))
    } else {
        Ok(())
    }
}

fn validate_request(
    request: &ArtifactEvaluationRequest,
    context: &ArtifactEvaluationContext,
) -> Vec<ArtifactFinding> {
    let mut findings = Vec::new();
    if request.claim.is_none() {
        findings.push(finding(
            ArtifactFindingCode::MissingArtifactIdentity,
            "claim",
            "artifact claim is absent",
            true,
        ));
    }
    if request.context_id != context.context_id {
        findings.push(finding(
            ArtifactFindingCode::ContextMismatch,
            "context_id",
            "artifact context does not match request",
            true,
        ));
    }
    if request.source_set_id != context.source_set_id {
        findings.push(finding(
            ArtifactFindingCode::SourceSetMismatch,
            "source_set_id",
            "artifact source-set identity does not match request",
            true,
        ));
    }
    if request.implementation_version != context.implementation_version {
        findings.push(finding(
            ArtifactFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version does not match context",
            true,
        ));
    }
    findings
}
fn evaluate_instance(
    instance: &ArtifactInstance,
    claim: &ArtifactClaim,
) -> ArtifactInstanceDetermination {
    if claim
        .identity
        .as_ref()
        .is_some_and(|identity| identity.id == instance.identity.artifact)
    {
        ArtifactInstanceDetermination::ValidInstance
    } else {
        ArtifactInstanceDetermination::Conflict
    }
}
#[allow(clippy::too_many_arguments)]
fn result(
    request: &ArtifactEvaluationRequest,
    claim: &ArtifactClaim,
    recognition: ArtifactDetermination,
    instance: ArtifactInstanceDetermination,
    preservation: IdentityPreservationDetermination,
    provenance: ProvenanceSufficiencyDetermination,
    integrity: IntegrityDetermination,
    findings: Vec<ArtifactFinding>,
) -> ArtifactEvaluationResult {
    ArtifactEvaluationResult { evaluation_id: request.evaluation_id.clone(), operation_id: request.operation_id.clone(), context_id: request.context_id.clone(), source_set_id: request.source_set_id.clone(), implementation_version: request.implementation_version.clone(), profile_version: DomainRef::new(PROFILE_VERSION).unwrap(), claim: Some(claim.clone()), recognition, instance, identity_preservation: preservation, provenance, integrity, relationships: Vec::new(), findings, non_claims: vec!["artifact representation does not become constitutional reality".into(), "provenance and integrity do not independently establish truth, authority, effectiveness, or conformance".into()] }
}
fn finding(code: ArtifactFindingCode, subject: &str, detail: &str, fatal: bool) -> ArtifactFinding {
    ArtifactFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(value: &str) -> DomainRef {
        DomainRef::new(value).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(constitutional_contracts::ConstitutionalSourceId::new("CORE-003").unwrap(), constitutional_contracts::ConstitutionalSourceVersion::new("0.1.1").unwrap(), "harmonization-v1.0/CORE-003_Constitutional_Artifacts_and_Provenance_v0.2.0_Harmonization_Draft.md").unwrap()
    }
    fn identity() -> ArtifactIdentity {
        ArtifactIdentity {
            id: r("artifact-1"),
            kind: ArtifactKind::EvidenceArtifact,
            represented_subject: Some(r("subject-1")),
            revision: r("revision-1"),
            roles: vec![ArtifactRole::Evidencing],
        }
    }
    fn context() -> ArtifactEvaluationContext {
        ArtifactEvaluationContext {
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.6.0")
                .unwrap(),
            available_sources: vec![source()],
            recognized_roles: vec![ArtifactRole::Evidencing],
            historical_context: false,
        }
    }
    fn request(
        category: ArtifactCategory,
        recognition: Option<RecognitionBasis>,
    ) -> ArtifactEvaluationRequest {
        ArtifactEvaluationRequest {
            evaluation_id: r("eval-1"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.6.0")
                .unwrap(),
            claim: Some(ArtifactClaim {
                identity: Some(identity()),
                instance: None,
                category,
                recognition,
                provenance: ArtifactProvenance {
                    origin: Some(r("origin")),
                    authority: None,
                    identity: None,
                    transformation: None,
                    version: None,
                    custody: None,
                    publication: None,
                    lineage: vec![r("lineage-1")],
                },
                integrity: None,
                representation_of: None,
                historical: false,
                replay: false,
                contained: vec![],
            }),
        }
    }
    #[test]
    fn explicit_recognition_basis_supports_bounded_recognition() {
        let req = request(
            ArtifactCategory::Constitutional,
            Some(RecognitionBasis {
                source: source(),
                role: ArtifactRole::Evidencing,
                expressly_recognized: true,
                authority_reference: None,
                identity_reference: None,
                provenance: r("recognition-provenance"),
            }),
        );
        let result = evaluate_artifact_recognition(&req, &context());
        assert_eq!(result.recognition, ArtifactDetermination::Recognized);
    }
    #[test]
    fn representation_or_hash_like_presence_does_not_create_recognition() {
        let req = request(ArtifactCategory::Constitutional, None);
        let result = evaluate_artifact_recognition(&req, &context());
        assert_eq!(result.recognition, ArtifactDetermination::Denied);
    }
    #[test]
    fn multiple_instances_preserve_artifact_identity_but_remain_distinct() {
        let mut req = request(ArtifactCategory::NonConstitutional, None);
        let base = identity();
        req.claim.as_mut().unwrap().identity = Some(base.clone());
        req.claim.as_mut().unwrap().instance = Some(ArtifactInstance {
            identity: ArtifactInstanceIdentity {
                id: r("instance-a"),
                artifact: base.id.clone(),
                context: r("ctx-1"),
                serialization: r("serialization-a"),
            },
            identity_preservation_basis: Some(r("authorized-instantiation")),
            copied_from: None,
            transformed_from: None,
            replayed_from: None,
        });
        let result = evaluate_artifact_recognition(&req, &context());
        assert_eq!(
            result.instance,
            ArtifactInstanceDetermination::ValidInstance
        );
        assert_eq!(
            result.identity_preservation,
            IdentityPreservationDetermination::Preserved
        );
    }
    #[test]
    fn uncontrolled_copy_does_not_preserve_identity() {
        let mut req = request(ArtifactCategory::NonConstitutional, None);
        req.claim.as_mut().unwrap().instance = Some(ArtifactInstance {
            identity: ArtifactInstanceIdentity {
                id: r("instance-a"),
                artifact: r("artifact-1"),
                context: r("ctx-1"),
                serialization: r("serialization-a"),
            },
            identity_preservation_basis: None,
            copied_from: Some(r("instance-original")),
            transformed_from: None,
            replayed_from: None,
        });
        let result = evaluate_artifact_recognition(&req, &context());
        assert_eq!(
            result.identity_preservation,
            IdentityPreservationDetermination::NotPreserved
        );
    }
    #[test]
    fn replay_and_integrity_do_not_create_truth_or_new_act() {
        let mut req = request(ArtifactCategory::NonConstitutional, None);
        req.claim.as_mut().unwrap().replay = true;
        req.claim.as_mut().unwrap().integrity = Some(ArtifactIntegrityClaim {
            representational: Some(true),
            identity: Some(true),
            instance: Some(true),
            lineage: Some(true),
            transformation: Some(true),
            version: Some(true),
            custody: Some(true),
            publication: Some(true),
        });
        let result = evaluate_artifact_recognition(&req, &context());
        assert_eq!(result.integrity, IntegrityDetermination::Satisfied);
        assert!(
            result
                .findings
                .iter()
                .any(|item| item.code == ArtifactFindingCode::ReplayNonCreation)
        );
        assert!(result.non_claims.iter().any(|item| item.contains("truth")));
    }
    #[test]
    fn compound_containment_does_not_merge_identity() {
        let mut req = request(ArtifactCategory::NonConstitutional, None);
        req.claim.as_mut().unwrap().contained.push(r("artifact-1"));
        assert_eq!(
            evaluate_artifact_recognition(&req, &context()).recognition,
            ArtifactDetermination::Conflict
        );
    }
}
