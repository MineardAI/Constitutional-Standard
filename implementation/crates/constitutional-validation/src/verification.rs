//! Bounded IMP-006 verification and assurance realization.
//!
//! Verification evaluates supplied, declared records. It does not execute
//! tests, own evidence, establish truth, determine conformance, certify,
//! authorize release or activation, or produce operational recognition.

use constitutional_canonical::{
    CanonicalEncodingResult, CanonicalField, CanonicalObject, CanonicalRepresentationId,
    CanonicalRepresentationKind, CanonicalValue, canonical_encode,
};
use constitutional_contracts::{
    ConstitutionalSourceRef, EvidenceRecordId, ImplementationVersion, RepresentationFormatId,
    RepresentationVersion,
};
use constitutional_traceability::TraceRef;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const VERIFICATION_PROFILE_ID: &str = "reference-implementation-verification-assurance";
pub const VERIFICATION_PROFILE_VERSION: &str = "1.0.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VerificationIdentifier(String);
impl VerificationIdentifier {
    pub fn new(value: impl Into<String>) -> Result<Self, VerificationModelError> {
        let value = value.into();
        if value.is_empty()
            || value
                .chars()
                .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(VerificationModelError::MalformedIdentifier(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for VerificationIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! verification_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(VerificationIdentifier);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, VerificationModelError> {
                Ok(Self(VerificationIdentifier::new(value)?))
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
verification_id!(VerificationRequirementId);
verification_id!(VerificationPlanId);
verification_id!(VerificationActivityId);
verification_id!(VerificationObservationId);
verification_id!(VerificationFindingId);
verification_id!(VerificationAssessmentId);
verification_id!(VerificationAssuranceId);
verification_id!(VerificationCriterionId);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerificationSubjectKind {
    ConstitutionalImplementationRequirement,
    ImplementationComponent,
    ImplementationArtifact,
    CanonicalRepresentation,
    TraceabilityRecord,
    ImplementationVersion,
    SourceBinding,
    LifecycleGateArtifact,
    DeviationRecord,
    EvidenceReference,
    ReleaseReference,
    ActivationReference,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VerificationSubject {
    pub id: VerificationIdentifier,
    pub kind: VerificationSubjectKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerificationMethod {
    StructuralValidation,
    SchemaValidation,
    CanonicalRoundTripValidation,
    DeterministicOutputValidation,
    SourceBindingValidation,
    ImplementationVersionValidation,
    TraceabilityValidation,
    PositiveBehavioralTest,
    NegativeBehavioralTest,
    BoundaryTest,
    InvariantTest,
    CompatibilityCheck,
    ManualReview,
    GovernedRecordReview,
    DocumentationReview,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerificationOutcome {
    Satisfied,
    NotSatisfied,
    PartiallySatisfied,
    Indeterminate,
    NotApplicable,
    InvalidVerificationRequest,
    Incomplete,
    Conflicted,
    Verified,
    VerifiedWithConditions,
    NotVerified,
    Inconclusive,
    Invalid,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerificationFindingSeverity {
    Informational,
    Advisory,
    Warning,
    Blocking,
    Invalidating,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VerificationFindingDisposition {
    Open,
    Resolved,
    AcceptedAsLimitation,
    Deferred,
    Rejected,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FindingPolarity {
    Positive,
    Negative,
    Neutral,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AssuranceStatus {
    Issued,
    Current,
    Suspended,
    Superseded,
    Withdrawn,
    Expired,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationCriterion {
    pub id: VerificationCriterionId,
    pub requirement_id: VerificationRequirementId,
    pub statement: String,
    pub limitations: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationRequirement {
    pub id: VerificationRequirementId,
    pub subject: VerificationSubject,
    pub source_binding: ConstitutionalSourceRef,
    pub implementation_version: ImplementationVersion,
    pub statement: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationContext {
    pub context_id: VerificationIdentifier,
    pub baseline: VerificationIdentifier,
    pub source_binding: ConstitutionalSourceRef,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationInputReference {
    pub id: VerificationIdentifier,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReference {
    pub id: EvidenceRecordId,
    pub implementation_version: ImplementationVersion,
    pub locator: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceabilityBinding {
    pub trace_id: TraceRef,
    pub scope: VerificationIdentifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRepresentationBinding {
    pub representation_id: CanonicalRepresentationId,
    pub format: RepresentationFormatId,
    pub version: RepresentationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationPlan {
    pub id: VerificationPlanId,
    pub context: VerificationContext,
    pub requirement_ids: Vec<VerificationRequirementId>,
    pub criterion_ids: Vec<VerificationCriterionId>,
    pub activity_ids: Vec<VerificationActivityId>,
    pub scope: Vec<VerificationSubject>,
    pub limitations: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationActivity {
    pub id: VerificationActivityId,
    pub plan_id: VerificationPlanId,
    pub requirement_id: VerificationRequirementId,
    pub criterion_id: VerificationCriterionId,
    pub subject: VerificationSubject,
    pub method: VerificationMethod,
    pub context: VerificationContext,
    pub inputs: Vec<VerificationInputReference>,
    pub evidence_references: Vec<EvidenceReference>,
    pub traceability: Option<TraceabilityBinding>,
    pub canonical: CanonicalRepresentationBinding,
    pub observation_ids: Vec<VerificationObservationId>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationObservation {
    pub id: VerificationObservationId,
    pub activity_id: VerificationActivityId,
    pub subject: VerificationSubject,
    pub outcome: VerificationOutcome,
    pub value: String,
    pub provenance: VerificationIdentifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationFinding {
    pub id: VerificationFindingId,
    pub subject: VerificationSubject,
    pub observation_id: Option<VerificationObservationId>,
    pub severity: VerificationFindingSeverity,
    pub disposition: VerificationFindingDisposition,
    pub polarity: FindingPolarity,
    pub detail: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationAssessment {
    pub id: VerificationAssessmentId,
    pub activity_ids: Vec<VerificationActivityId>,
    pub finding_ids: Vec<VerificationFindingId>,
    pub scope: Vec<VerificationSubject>,
    pub outcome: VerificationOutcome,
    pub basis: String,
    pub limitations: Vec<String>,
    pub canonical: CanonicalRepresentationBinding,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationAssurance {
    pub id: VerificationAssuranceId,
    pub assessment_id: VerificationAssessmentId,
    pub scope: Vec<VerificationSubject>,
    pub implementation_version: ImplementationVersion,
    pub source_binding: ConstitutionalSourceRef,
    pub basis: Vec<VerificationAssessmentId>,
    pub limitations: Vec<String>,
    pub unresolved_finding_ids: Vec<VerificationFindingId>,
    pub outcome: VerificationOutcome,
    pub status: AssuranceStatus,
    pub canonical: CanonicalRepresentationBinding,
    pub claims_conformance: bool,
    pub claims_certification: bool,
    pub claims_release_authority: bool,
    pub claims_activation: bool,
    pub claims_operational_recognition: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationBundle {
    pub implementation_version: ImplementationVersion,
    pub requirements: Vec<VerificationRequirement>,
    pub criteria: Vec<VerificationCriterion>,
    pub plans: Vec<VerificationPlan>,
    pub activities: Vec<VerificationActivity>,
    pub observations: Vec<VerificationObservation>,
    pub findings: Vec<VerificationFinding>,
    pub assessments: Vec<VerificationAssessment>,
    pub assurances: Vec<VerificationAssurance>,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum VerificationFindingCode {
    MissingVerificationIdentifier,
    DuplicateVerificationIdentifier,
    UnknownVerificationSubject,
    MalformedVerificationSubject,
    MissingVerificationRequirement,
    MissingVerificationMethod,
    UnsupportedVerificationMethod,
    MissingVerificationCriterion,
    MalformedSourceBinding,
    MissingSourceVersion,
    MalformedImplementationVersionBinding,
    ImplementationVersionMismatch,
    MissingTraceabilityBinding,
    MalformedTraceabilityBinding,
    MalformedEvidenceReference,
    MissingVerificationInput,
    UnresolvedVerificationDependency,
    FindingWithoutObservationBasis,
    AssessmentWithoutFindingBasis,
    AssuranceWithoutAssessmentBasis,
    AssuranceScopeExceedsVerifiedScope,
    UnresolvedBlockingFinding,
    ContradictoryFindings,
    IncompatibleAssessmentOutcomes,
    UnsupportedAssuranceStatus,
    MissingAssuranceLimitation,
    MalformedProvenanceReference,
    InvalidLifecycleReference,
    InvalidGateReference,
    ProhibitedConformanceClaim,
    ProhibitedCertificationClaim,
    ProhibitedReleaseAuthorityClaim,
    ProhibitedActivationClaim,
    ProhibitedOperationalRecognitionClaim,
    NondeterministicOrdering,
    CanonicalRoundTripMismatch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationFindingRecord {
    pub code: VerificationFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationValidationResult {
    pub valid: bool,
    pub findings: Vec<VerificationFindingRecord>,
    pub assurance_created: bool,
    pub evidence_owned: bool,
    pub evidence_sufficiency_established: bool,
    pub conformance_established: bool,
    pub certification_established: bool,
    pub release_authorized: bool,
    pub activation_authorized: bool,
    pub operational_recognition_produced: bool,
    pub execution_performed: bool,
    pub non_claims: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationModelError {
    MalformedIdentifier(String),
    InvalidAssurance(VerificationValidationResult),
}

pub fn validate_verification_bundle(bundle: &VerificationBundle) -> VerificationValidationResult {
    let mut findings = Vec::new();
    let mut ids = BTreeSet::new();
    for requirement in &bundle.requirements {
        register_id(&mut ids, &mut findings, requirement.id.to_string());
        if !valid_subject(&requirement.subject) {
            push(
                &mut findings,
                VerificationFindingCode::MalformedVerificationSubject,
                requirement.id.to_string(),
                "verification requirement subject is malformed",
            );
        }
        validate_source(
            &requirement.source_binding,
            &requirement.id.to_string(),
            &mut findings,
        );
        if requirement.implementation_version != bundle.implementation_version {
            push(
                &mut findings,
                VerificationFindingCode::ImplementationVersionMismatch,
                requirement.id.to_string(),
                "requirement implementation version differs from bundle",
            );
        }
    }
    for criterion in &bundle.criteria {
        register_id(&mut ids, &mut findings, criterion.id.to_string());
        if criterion.statement.trim().is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::MissingVerificationCriterion,
                criterion.id.to_string(),
                "criterion statement is missing",
            );
        }
    }
    let subjects: BTreeSet<_> = bundle
        .requirements
        .iter()
        .map(|requirement| requirement.subject.id.clone())
        .collect();
    for plan in &bundle.plans {
        register_id(&mut ids, &mut findings, plan.id.to_string());
        for requirement_id in &plan.requirement_ids {
            if !bundle
                .requirements
                .iter()
                .any(|item| &item.id == requirement_id)
            {
                push(
                    &mut findings,
                    VerificationFindingCode::MissingVerificationRequirement,
                    plan.id.to_string(),
                    "plan requirement is unresolved",
                );
            }
        }
        for activity_id in &plan.activity_ids {
            if !bundle.activities.iter().any(|item| &item.id == activity_id) {
                push(
                    &mut findings,
                    VerificationFindingCode::UnresolvedVerificationDependency,
                    plan.id.to_string(),
                    "plan activity is unresolved",
                );
            }
        }
        if plan
            .scope
            .iter()
            .any(|subject| !subjects.contains(&subject.id))
        {
            push(
                &mut findings,
                VerificationFindingCode::UnknownVerificationSubject,
                plan.id.to_string(),
                "plan scope contains an unknown subject",
            );
        }
    }
    for activity in &bundle.activities {
        register_id(&mut ids, &mut findings, activity.id.to_string());
        if !subjects.contains(&activity.subject.id) {
            push(
                &mut findings,
                VerificationFindingCode::UnknownVerificationSubject,
                activity.id.to_string(),
                "activity subject is unknown",
            );
        }
        if !bundle
            .requirements
            .iter()
            .any(|item| item.id == activity.requirement_id)
        {
            push(
                &mut findings,
                VerificationFindingCode::MissingVerificationRequirement,
                activity.id.to_string(),
                "activity requirement is missing",
            );
        }
        if !bundle
            .criteria
            .iter()
            .any(|item| item.id == activity.criterion_id)
        {
            push(
                &mut findings,
                VerificationFindingCode::MissingVerificationCriterion,
                activity.id.to_string(),
                "activity criterion is missing",
            );
        }
        validate_context(
            &activity.context,
            &activity.id.to_string(),
            &mut findings,
            &bundle.implementation_version,
        );
        if activity.inputs.is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::MissingVerificationInput,
                activity.id.to_string(),
                "activity has no declared input",
            );
        }
        if activity.traceability.is_none() {
            push(
                &mut findings,
                VerificationFindingCode::MissingTraceabilityBinding,
                activity.id.to_string(),
                "activity has no traceability binding",
            );
        }
        if activity.canonical.version.as_str().is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::CanonicalRoundTripMismatch,
                activity.id.to_string(),
                "canonical binding version is missing",
            );
        }
        for evidence in &activity.evidence_references {
            validate_evidence(
                evidence,
                &activity.id.to_string(),
                &bundle.implementation_version,
                &mut findings,
            );
        }
    }
    for observation in &bundle.observations {
        register_id(&mut ids, &mut findings, observation.id.to_string());
        if !bundle
            .activities
            .iter()
            .any(|activity| activity.id == observation.activity_id)
        {
            push(
                &mut findings,
                VerificationFindingCode::UnresolvedVerificationDependency,
                observation.id.to_string(),
                "observation activity is unresolved",
            );
        }
        if observation.provenance.as_str().is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::MalformedProvenanceReference,
                observation.id.to_string(),
                "observation provenance is missing",
            );
        }
    }
    for finding in &bundle.findings {
        register_id(&mut ids, &mut findings, finding.id.to_string());
        if finding.observation_id.as_ref().is_none_or(|id| {
            !bundle
                .observations
                .iter()
                .any(|observation| &observation.id == id)
        }) {
            push(
                &mut findings,
                VerificationFindingCode::FindingWithoutObservationBasis,
                finding.id.to_string(),
                "finding has no observation basis",
            );
        }
    }
    for assessment in &bundle.assessments {
        register_id(&mut ids, &mut findings, assessment.id.to_string());
        if assessment.finding_ids.is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::AssessmentWithoutFindingBasis,
                assessment.id.to_string(),
                "assessment has no finding basis",
            );
        }
        if assessment
            .activity_ids
            .iter()
            .any(|id| !bundle.activities.iter().any(|activity| &activity.id == id))
        {
            push(
                &mut findings,
                VerificationFindingCode::UnresolvedVerificationDependency,
                assessment.id.to_string(),
                "assessment activity is unresolved",
            );
        }
        if assessment.basis.trim().is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::AssessmentWithoutFindingBasis,
                assessment.id.to_string(),
                "assessment basis is empty",
            );
        }
    }
    for assurance in &bundle.assurances {
        register_id(&mut ids, &mut findings, assurance.id.to_string());
        if assurance.basis.is_empty()
            || !assurance.basis.iter().all(|id| {
                bundle
                    .assessments
                    .iter()
                    .any(|assessment| assessment.id == *id)
            })
        {
            push(
                &mut findings,
                VerificationFindingCode::AssuranceWithoutAssessmentBasis,
                assurance.id.to_string(),
                "assurance has no resolved assessment basis",
            );
        }
        if assurance.limitations.is_empty() {
            push(
                &mut findings,
                VerificationFindingCode::MissingAssuranceLimitation,
                assurance.id.to_string(),
                "assurance limitation is missing",
            );
        }
        validate_source(
            &assurance.source_binding,
            &assurance.id.to_string(),
            &mut findings,
        );
        if assurance.implementation_version != bundle.implementation_version {
            push(
                &mut findings,
                VerificationFindingCode::ImplementationVersionMismatch,
                assurance.id.to_string(),
                "assurance implementation version differs from bundle",
            );
        }
        if assurance.claims_conformance {
            push(
                &mut findings,
                VerificationFindingCode::ProhibitedConformanceClaim,
                assurance.id.to_string(),
                "assurance cannot claim conformance",
            );
        }
        if assurance.claims_certification {
            push(
                &mut findings,
                VerificationFindingCode::ProhibitedCertificationClaim,
                assurance.id.to_string(),
                "assurance cannot claim certification",
            );
        }
        if assurance.claims_release_authority {
            push(
                &mut findings,
                VerificationFindingCode::ProhibitedReleaseAuthorityClaim,
                assurance.id.to_string(),
                "assurance cannot authorize release",
            );
        }
        if assurance.claims_activation {
            push(
                &mut findings,
                VerificationFindingCode::ProhibitedActivationClaim,
                assurance.id.to_string(),
                "assurance cannot authorize activation",
            );
        }
        if assurance.claims_operational_recognition {
            push(
                &mut findings,
                VerificationFindingCode::ProhibitedOperationalRecognitionClaim,
                assurance.id.to_string(),
                "assurance cannot produce operational recognition",
            );
        }
        let blocking: BTreeSet<_> = bundle
            .findings
            .iter()
            .filter(|finding| {
                matches!(
                    finding.severity,
                    VerificationFindingSeverity::Blocking
                        | VerificationFindingSeverity::Invalidating
                ) && !matches!(
                    finding.disposition,
                    VerificationFindingDisposition::Resolved
                )
            })
            .map(|finding| finding.id.clone())
            .collect();
        if !blocking.is_empty()
            || assurance
                .unresolved_finding_ids
                .iter()
                .any(|id| blocking.contains(id))
        {
            push(
                &mut findings,
                VerificationFindingCode::UnresolvedBlockingFinding,
                assurance.id.to_string(),
                "assurance includes unresolved blocking findings",
            );
        }
        if let Some(assessment) = bundle
            .assessments
            .iter()
            .find(|assessment| assessment.id == assurance.assessment_id)
            && assurance.scope.iter().any(|subject| {
                !assessment
                    .scope
                    .iter()
                    .any(|verified| verified.id == subject.id)
            })
        {
            push(
                &mut findings,
                VerificationFindingCode::AssuranceScopeExceedsVerifiedScope,
                assurance.id.to_string(),
                "assurance scope exceeds the verified assessment scope",
            );
        }
    }
    let mut subject_polarity: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for finding in &bundle.findings {
        subject_polarity
            .entry(finding.subject.id.clone())
            .or_default()
            .insert(finding.polarity.clone());
    }
    if subject_polarity.values().any(|values| {
        values.contains(&FindingPolarity::Positive) && values.contains(&FindingPolarity::Negative)
    }) {
        push(
            &mut findings,
            VerificationFindingCode::ContradictoryFindings,
            "findings".into(),
            "positive and negative findings conflict for a subject",
        );
    }
    findings.sort_by(|left, right| {
        left.subject
            .cmp(&right.subject)
            .then_with(|| left.code.cmp(&right.code))
    });
    let valid = findings.iter().all(|finding| !finding.fatal);
    VerificationValidationResult { valid, findings, assurance_created: valid && !bundle.assurances.is_empty(), evidence_owned: false, evidence_sufficiency_established: false, conformance_established: false, certification_established: false, release_authorized: false, activation_authorized: false, operational_recognition_produced: false, execution_performed: false, non_claims: vec!["verification and assurance are not constitutional truth, evidence ownership or sufficiency, conformance, certification, release authorization, activation, operational recognition, persistence, transport, execution, or adjudication".into()] }
}

pub fn issue_assurance(
    bundle: &VerificationBundle,
    assurance: VerificationAssurance,
) -> Result<VerificationAssurance, VerificationModelError> {
    let mut candidate = bundle.clone();
    candidate.assurances.push(assurance.clone());
    let result = validate_verification_bundle(&candidate);
    if result.valid {
        Ok(assurance)
    } else {
        Err(VerificationModelError::InvalidAssurance(result))
    }
}

pub fn verification_activity_canonical(
    activity: &VerificationActivity,
) -> Result<CanonicalEncodingResult, constitutional_canonical::CanonicalDecodeError> {
    canonical_encode(&CanonicalObject {
        representation_id: activity.canonical.representation_id.clone(),
        kind: CanonicalRepresentationKind::VerificationReference,
        representation_version: activity.canonical.version.as_str().into(),
        schema_id: VERIFICATION_PROFILE_ID.into(),
        source_binding: Some(activity.context.source_binding.clone()),
        implementation_version: Some(activity.context.implementation_version.clone()),
        required_fields: vec![
            "activity_id".into(),
            "method".into(),
            "requirement_id".into(),
            "subject_id".into(),
        ],
        extension_namespaces: vec![],
        fields: vec![
            field("activity_id", &activity.id.to_string(), true, true),
            field("method", &format!("{:?}", activity.method), true, true),
            field(
                "requirement_id",
                &activity.requirement_id.to_string(),
                true,
                true,
            ),
            field("subject_id", activity.subject.id.as_str(), true, true),
            field(
                "traceability",
                &activity
                    .traceability
                    .as_ref()
                    .map(|binding| binding.trace_id.to_string())
                    .unwrap_or_else(|| "absent".into()),
                false,
                false,
            ),
        ],
    })
}

pub fn verification_assurance_canonical(
    assurance: &VerificationAssurance,
) -> Result<CanonicalEncodingResult, constitutional_canonical::CanonicalDecodeError> {
    canonical_encode(&CanonicalObject {
        representation_id: assurance.canonical.representation_id.clone(),
        kind: CanonicalRepresentationKind::VerificationReference,
        representation_version: assurance.canonical.version.as_str().into(),
        schema_id: VERIFICATION_PROFILE_ID.into(),
        source_binding: Some(assurance.source_binding.clone()),
        implementation_version: Some(assurance.implementation_version.clone()),
        required_fields: vec![
            "assurance_id".into(),
            "assessment_id".into(),
            "outcome".into(),
            "status".into(),
        ],
        extension_namespaces: vec![],
        fields: vec![
            field(
                "assessment_id",
                &assurance.assessment_id.to_string(),
                true,
                true,
            ),
            field("assurance_id", &assurance.id.to_string(), true, true),
            field("limitations", &assurance.limitations.join("|"), true, false),
            field("outcome", &format!("{:?}", assurance.outcome), true, true),
            field("status", &format!("{:?}", assurance.status), true, true),
        ],
    })
}

fn field(name: &str, value: &str, required: bool, identity: bool) -> CanonicalField {
    CanonicalField {
        name: name.into(),
        value: CanonicalValue::Text(value.into()),
        required,
        identity_participates: identity,
        extension: false,
    }
}
fn valid_subject(subject: &VerificationSubject) -> bool {
    !subject.id.as_str().is_empty()
}
fn validate_source(
    source: &ConstitutionalSourceRef,
    subject: &str,
    findings: &mut Vec<VerificationFindingRecord>,
) {
    if source.path.is_empty() || source.path.chars().any(char::is_control) {
        push(
            findings,
            VerificationFindingCode::MalformedSourceBinding,
            subject.into(),
            "source binding path is malformed",
        );
    }
    if source.version.as_str().is_empty() {
        push(
            findings,
            VerificationFindingCode::MissingSourceVersion,
            subject.into(),
            "source version is missing",
        );
    }
}
fn validate_context(
    context: &VerificationContext,
    subject: &str,
    findings: &mut Vec<VerificationFindingRecord>,
    expected: &ImplementationVersion,
) {
    validate_source(&context.source_binding, subject, findings);
    if context.implementation_version != *expected {
        push(
            findings,
            VerificationFindingCode::ImplementationVersionMismatch,
            subject.into(),
            "verification context implementation version differs from bundle",
        );
    }
}
fn validate_evidence(
    reference: &EvidenceReference,
    subject: &str,
    expected: &ImplementationVersion,
    findings: &mut Vec<VerificationFindingRecord>,
) {
    if reference.locator.trim().is_empty() {
        push(
            findings,
            VerificationFindingCode::MalformedEvidenceReference,
            subject.into(),
            "evidence reference locator is empty",
        );
    }
    if reference.implementation_version != *expected {
        push(
            findings,
            VerificationFindingCode::ImplementationVersionMismatch,
            subject.into(),
            "evidence reference implementation version differs from bundle",
        );
    }
}
fn push(
    findings: &mut Vec<VerificationFindingRecord>,
    code: VerificationFindingCode,
    subject: String,
    detail: &str,
) {
    findings.push(VerificationFindingRecord {
        code,
        subject,
        detail: detail.into(),
        fatal: true,
    });
}
fn register_id(
    ids: &mut BTreeSet<String>,
    findings: &mut Vec<VerificationFindingRecord>,
    id: String,
) {
    if id.is_empty() {
        push(
            findings,
            VerificationFindingCode::MissingVerificationIdentifier,
            id,
            "verification identifier is missing",
        );
    } else if !ids.insert(id.clone()) {
        push(
            findings,
            VerificationFindingCode::DuplicateVerificationIdentifier,
            id,
            "verification identifier is duplicated",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constitutional_canonical::{CanonicalRepresentationId, canonical_decode};
    use constitutional_contracts::{
        ConstitutionalSourceId, ConstitutionalSourceVersion, RepresentationFormatId,
        RepresentationVersion,
    };

    fn id(value: &str) -> VerificationIdentifier {
        VerificationIdentifier::new(value).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(
            ConstitutionalSourceId::new("IMP-006").unwrap(),
            ConstitutionalSourceVersion::new("0.1.0").unwrap(),
            "source.md",
        )
        .unwrap()
    }
    fn binding(id: &str) -> CanonicalRepresentationBinding {
        CanonicalRepresentationBinding {
            representation_id: CanonicalRepresentationId::new(id).unwrap(),
            format: RepresentationFormatId::new("verification").unwrap(),
            version: RepresentationVersion::new("0.1.0").unwrap(),
        }
    }
    fn bundle() -> VerificationBundle {
        let version = ImplementationVersion::new("reference-foundation-0.12.0").unwrap();
        let subject = VerificationSubject {
            id: id("component-1"),
            kind: VerificationSubjectKind::ImplementationComponent,
        };
        let requirement = VerificationRequirement {
            id: VerificationRequirementId::new("requirement-1").unwrap(),
            subject: subject.clone(),
            source_binding: source(),
            implementation_version: version.clone(),
            statement: "component is structurally valid".into(),
        };
        let criterion = VerificationCriterion {
            id: VerificationCriterionId::new("criterion-1").unwrap(),
            requirement_id: requirement.id.clone(),
            statement: "structural validation returns no fatal finding".into(),
            limitations: vec!["bounded reference check".into()],
        };
        let context = VerificationContext {
            context_id: id("context-1"),
            baseline: id("baseline-1"),
            source_binding: source(),
            implementation_version: version.clone(),
        };
        let activity = VerificationActivity {
            id: VerificationActivityId::new("activity-1").unwrap(),
            plan_id: VerificationPlanId::new("plan-1").unwrap(),
            requirement_id: requirement.id.clone(),
            criterion_id: criterion.id.clone(),
            subject: subject.clone(),
            method: VerificationMethod::StructuralValidation,
            context,
            inputs: vec![VerificationInputReference {
                id: id("input-1"),
                implementation_version: version.clone(),
            }],
            evidence_references: vec![EvidenceReference {
                id: EvidenceRecordId::new("evidence-1").unwrap(),
                implementation_version: version.clone(),
                locator: "tests".into(),
            }],
            traceability: Some(TraceabilityBinding {
                trace_id: TraceRef::new("trace-1").unwrap(),
                scope: id("phase-13"),
            }),
            canonical: binding("activity-representation-1"),
            observation_ids: vec![VerificationObservationId::new("observation-1").unwrap()],
        };
        let plan = VerificationPlan {
            id: VerificationPlanId::new("plan-1").unwrap(),
            context: VerificationContext {
                context_id: id("context-1"),
                baseline: id("baseline-1"),
                source_binding: source(),
                implementation_version: version.clone(),
            },
            requirement_ids: vec![requirement.id.clone()],
            criterion_ids: vec![criterion.id.clone()],
            activity_ids: vec![activity.id.clone()],
            scope: vec![subject.clone()],
            limitations: vec!["reference-only".into()],
        };
        let observation = VerificationObservation {
            id: VerificationObservationId::new("observation-1").unwrap(),
            activity_id: activity.id.clone(),
            subject: subject.clone(),
            outcome: VerificationOutcome::Satisfied,
            value: "no fatal findings".into(),
            provenance: id("test-run-1"),
        };
        let finding = VerificationFinding {
            id: VerificationFindingId::new("finding-1").unwrap(),
            subject,
            observation_id: Some(observation.id.clone()),
            severity: VerificationFindingSeverity::Informational,
            disposition: VerificationFindingDisposition::Resolved,
            polarity: FindingPolarity::Positive,
            detail: "bounded structural check completed".into(),
        };
        let assessment = VerificationAssessment {
            id: VerificationAssessmentId::new("assessment-1").unwrap(),
            activity_ids: vec![activity.id.clone()],
            finding_ids: vec![finding.id.clone()],
            scope: vec![activity.subject.clone()],
            outcome: VerificationOutcome::Verified,
            basis: "finding-1".into(),
            limitations: vec!["not conformance".into()],
            canonical: binding("assessment-representation-1"),
        };
        VerificationBundle {
            implementation_version: version,
            requirements: vec![requirement],
            criteria: vec![criterion],
            plans: vec![plan],
            activities: vec![activity],
            observations: vec![observation],
            findings: vec![finding],
            assessments: vec![assessment],
            assurances: vec![],
        }
    }
    #[test]
    fn valid_chain_is_deterministic_and_non_authoritative() {
        let result = validate_verification_bundle(&bundle());
        assert!(result.valid);
        assert!(!result.assurance_created);
        assert!(!result.evidence_owned);
        assert!(!result.conformance_established);
        assert!(!result.execution_performed);
    }
    #[test]
    fn missing_method_input_and_traceability_are_rejected() {
        let mut value = bundle();
        value.activities[0].inputs.clear();
        value.activities[0].traceability = None;
        let result = validate_verification_bundle(&value);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::MissingVerificationInput)
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::MissingTraceabilityBinding)
        );
    }
    #[test]
    fn finding_observation_and_assessment_basis_are_required() {
        let mut value = bundle();
        value.findings[0].observation_id = None;
        value.assessments[0].finding_ids.clear();
        let result = validate_verification_bundle(&value);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::FindingWithoutObservationBasis)
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::AssessmentWithoutFindingBasis)
        );
    }
    #[test]
    fn unresolved_blocking_finding_prevents_assurance() {
        let mut value = bundle();
        value.findings[0].severity = VerificationFindingSeverity::Blocking;
        value.findings[0].disposition = VerificationFindingDisposition::Open;
        let assurance = VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: value.assessments[0].scope.clone(),
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![value.assessments[0].id.clone()],
            limitations: vec!["bounded".into()],
            unresolved_finding_ids: vec![value.findings[0].id.clone()],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: false,
            claims_certification: false,
            claims_release_authority: false,
            claims_activation: false,
            claims_operational_recognition: false,
        };
        value.assurances.push(assurance);
        let result = validate_verification_bundle(&value);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::UnresolvedBlockingFinding)
        );
    }
    #[test]
    fn prohibited_assurance_claims_are_rejected() {
        let mut value = bundle();
        value.assurances.push(VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: value.assessments[0].scope.clone(),
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![value.assessments[0].id.clone()],
            limitations: vec!["bounded".into()],
            unresolved_finding_ids: vec![],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: true,
            claims_certification: true,
            claims_release_authority: true,
            claims_activation: true,
            claims_operational_recognition: true,
        });
        let result = validate_verification_bundle(&value);
        assert!(!result.valid);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::ProhibitedConformanceClaim)
        );
    }
    #[test]
    fn assurance_requires_explicit_basis_and_limitation() {
        let mut value = bundle();
        value.assurances.push(VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: value.assessments[0].scope.clone(),
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![],
            limitations: vec![],
            unresolved_finding_ids: vec![],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: false,
            claims_certification: false,
            claims_release_authority: false,
            claims_activation: false,
            claims_operational_recognition: false,
        });
        let result = validate_verification_bundle(&value);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::AssuranceWithoutAssessmentBasis)
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::MissingAssuranceLimitation)
        );
    }
    #[test]
    fn assurance_scope_cannot_exceed_assessment_scope() {
        let mut value = bundle();
        value.assurances.push(VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: vec![VerificationSubject {
                id: id("unverified-subject"),
                kind: VerificationSubjectKind::ImplementationArtifact,
            }],
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![value.assessments[0].id.clone()],
            limitations: vec!["bounded".into()],
            unresolved_finding_ids: vec![],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: false,
            claims_certification: false,
            claims_release_authority: false,
            claims_activation: false,
            claims_operational_recognition: false,
        });
        let result = validate_verification_bundle(&value);
        assert!(result.findings.iter().any(|finding| {
            finding.code == VerificationFindingCode::AssuranceScopeExceedsVerifiedScope
        }));
    }
    #[test]
    fn version_and_source_bindings_are_checked() {
        let mut value = bundle();
        value.activities[0].context.implementation_version =
            ImplementationVersion::new("reference-foundation-9.9.9").unwrap();
        let result = validate_verification_bundle(&value);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::ImplementationVersionMismatch)
        );
    }
    #[test]
    fn contradictory_findings_are_explicit_and_sorted() {
        let mut value = bundle();
        value.findings.push(VerificationFinding {
            id: VerificationFindingId::new("finding-2").unwrap(),
            subject: value.findings[0].subject.clone(),
            observation_id: Some(value.observations[0].id.clone()),
            severity: VerificationFindingSeverity::Warning,
            disposition: VerificationFindingDisposition::Open,
            polarity: FindingPolarity::Negative,
            detail: "contradiction".into(),
        });
        let first = validate_verification_bundle(&value);
        value.findings.reverse();
        let second = validate_verification_bundle(&value);
        assert!(
            first
                .findings
                .iter()
                .any(|f| f.code == VerificationFindingCode::ContradictoryFindings)
        );
        assert_eq!(first.findings, second.findings);
    }
    #[test]
    fn assurance_issue_is_explicit_not_automatic() {
        let value = bundle();
        let assurance = VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: value.assessments[0].scope.clone(),
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![value.assessments[0].id.clone()],
            limitations: vec!["bounded".into()],
            unresolved_finding_ids: vec![],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: false,
            claims_certification: false,
            claims_release_authority: false,
            claims_activation: false,
            claims_operational_recognition: false,
        };
        assert!(issue_assurance(&value, assurance).is_ok());
    }
    #[test]
    fn canonical_activity_and_assurance_bindings_are_deterministic() {
        let value = bundle();
        let first = verification_activity_canonical(&value.activities[0]).unwrap();
        let second = verification_activity_canonical(&value.activities[0]).unwrap();
        assert_eq!(first.bytes, second.bytes);
        assert_eq!(
            canonical_decode(&first.bytes).unwrap().identity_digest,
            first.identity_digest
        );
        let assurance = VerificationAssurance {
            id: VerificationAssuranceId::new("assurance-1").unwrap(),
            assessment_id: value.assessments[0].id.clone(),
            scope: value.assessments[0].scope.clone(),
            implementation_version: value.implementation_version.clone(),
            source_binding: source(),
            basis: vec![value.assessments[0].id.clone()],
            limitations: vec!["bounded".into()],
            unresolved_finding_ids: vec![],
            outcome: VerificationOutcome::Verified,
            status: AssuranceStatus::Issued,
            canonical: binding("assurance-representation-1"),
            claims_conformance: false,
            claims_certification: false,
            claims_release_authority: false,
            claims_activation: false,
            claims_operational_recognition: false,
        };
        assert_eq!(
            verification_assurance_canonical(&assurance).unwrap().bytes,
            verification_assurance_canonical(&assurance).unwrap().bytes
        );
    }
}
