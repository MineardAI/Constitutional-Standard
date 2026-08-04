//! Bounded IMP-009 activation and operational-recognition records.
//!
//! This crate records constitutional standing only. It does not deploy,
//! execute, publish, monitor, mutate external environments, or create itself
//! authority. Activation and recognition are deliberately separate records.

use constitutional_canonical::{
    CanonicalCollection, CanonicalField, CanonicalObject, CanonicalRepresentationId,
    CanonicalRepresentationKind, CanonicalValue, REPRESENTATION_MODEL_VERSION,
};
use constitutional_contracts::{EvidenceRecordId, ImplementationVersion};
use constitutional_release::{
    CompatibilityOutcome, ReleaseIdentifier, ReleaseState, ReleaseVersionIdentifier,
    ReleasedImplementation,
};
use constitutional_traceability::TraceRef;
use std::fmt;

pub const ACTIVATION_PROFILE_ID: &str =
    "reference-implementation-activation-operational-recognition";
pub const ACTIVATION_PROFILE_VERSION: &str = "1.0.0";

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ActivationError> {
                let value = value.into();
                if value.is_empty() || value.chars().any(|c| c.is_whitespace() || c.is_control()) {
                    return Err(ActivationError::MalformedIdentifier(value));
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

id_type!(ActivationSubjectIdentifier);
id_type!(ActivationTargetIdentifier);
id_type!(ActivationScopeIdentifier);
id_type!(ActivationEnvironmentIdentifier);
id_type!(ActivationContextIdentifier);
id_type!(ActivationRequestIdentifier);
id_type!(EligibilityDeterminationIdentifier);
id_type!(ActivationDecisionIdentifier);
id_type!(ActivationActIdentifier);
id_type!(ActivationRecordIdentifier);
id_type!(OperationalAssignmentIdentifier);
id_type!(RecognitionSubjectIdentifier);
id_type!(RecognitionRequestIdentifier);
id_type!(RecognitionCriteriaIdentifier);
id_type!(RecognitionFindingIdentifier);
id_type!(RecognitionDecisionIdentifier);
id_type!(RecognitionRecordIdentifier);
id_type!(OperationalBaselineIdentifier);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationError {
    MalformedIdentifier(String),
    Invalid(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseBinding {
    pub release: ReleaseIdentifier,
    pub version: ReleaseVersionIdentifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityBinding {
    pub authority_reference: String,
    pub jurisdiction_reference: String,
    pub scope: ActivationScopeIdentifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorizationStatus {
    Current,
    Expired,
    Withdrawn,
    Superseded,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationAuthorization {
    pub reference: String,
    pub release: ReleaseBinding,
    pub environment: ActivationEnvironmentIdentifier,
    pub scope: ActivationScopeIdentifier,
    pub authority: AuthorityBinding,
    pub status: AuthorizationStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationSubject {
    pub id: ActivationSubjectIdentifier,
    pub release: ReleaseBinding,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationTarget {
    pub id: ActivationTargetIdentifier,
    pub scope: ActivationScopeIdentifier,
    pub environment: ActivationEnvironmentIdentifier,
    pub context: ActivationContextIdentifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationInferenceBasis {
    ExplicitRequest,
    Publication,
    Deployment,
    Compatibility,
    TestSuccess,
    Evidence,
    Assurance,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationRequest {
    pub id: ActivationRequestIdentifier,
    pub subject: ActivationSubject,
    pub target: ActivationTarget,
    pub compatibility_target: Option<ReleaseBinding>,
    pub source_set: String,
    pub authority: AuthorityBinding,
    pub authorization: Option<ActivationAuthorization>,
    pub evidence: Vec<EvidenceRecordId>,
    pub verification: Vec<String>,
    pub assurance: Vec<String>,
    pub traceability: Vec<TraceRef>,
    pub inference_basis: ActivationInferenceBasis,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EligibilityOutcome {
    Eligible,
    Ineligible,
    Indeterminate,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalEligibilityDetermination {
    pub id: EligibilityDeterminationIdentifier,
    pub request: ActivationRequestIdentifier,
    pub outcome: EligibilityOutcome,
    pub prerequisites: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationDecisionOutcome {
    Authorized,
    Denied,
    Indeterminate,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationDecision {
    pub id: ActivationDecisionIdentifier,
    pub request: ActivationRequestIdentifier,
    pub eligibility: EligibilityDeterminationIdentifier,
    pub outcome: ActivationDecisionOutcome,
    pub authorization: Option<ActivationAuthorization>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationState {
    Proposed,
    Requested,
    Eligible,
    Ineligible,
    PendingAuthorization,
    Authorized,
    Activated,
    ActivationFailed,
    Suspended,
    Withdrawn,
    Deactivated,
    EligibleForReactivation,
    Reactivated,
    Indeterminate,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationTransition {
    Request,
    DetermineEligibility,
    Authorize,
    Activate,
    Fail,
    Suspend,
    Restore,
    Transfer,
    Rollback,
    Withdraw,
    Deactivate,
    MarkEligibleForReactivation,
    Reactivate,
}

pub fn transition_allowed(from: &ActivationState, transition: &ActivationTransition) -> bool {
    matches!(
        (from, transition),
        (ActivationState::Proposed, ActivationTransition::Request)
            | (
                ActivationState::Requested,
                ActivationTransition::DetermineEligibility
            )
            | (ActivationState::Eligible, ActivationTransition::Authorize)
            | (ActivationState::Authorized, ActivationTransition::Activate)
            | (ActivationState::Authorized, ActivationTransition::Fail)
            | (ActivationState::Activated, ActivationTransition::Suspend)
            | (ActivationState::Suspended, ActivationTransition::Restore)
            | (ActivationState::Activated, ActivationTransition::Transfer)
            | (ActivationState::Activated, ActivationTransition::Rollback)
            | (ActivationState::Activated, ActivationTransition::Withdraw)
            | (ActivationState::Activated, ActivationTransition::Deactivate)
            | (ActivationState::Suspended, ActivationTransition::Withdraw)
            | (ActivationState::Suspended, ActivationTransition::Reactivate)
            | (
                ActivationState::Deactivated,
                ActivationTransition::MarkEligibleForReactivation
            )
            | (
                ActivationState::EligibleForReactivation,
                ActivationTransition::Reactivate
            )
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationAct {
    pub id: ActivationActIdentifier,
    pub decision: ActivationDecisionIdentifier,
    pub subject: ActivationSubject,
    pub target: ActivationTarget,
    pub state: ActivationState,
    pub executed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationEffect {
    pub act: ActivationActIdentifier,
    pub resulting_state: ActivationState,
    pub runtime_execution_claimed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalAssignment {
    pub id: OperationalAssignmentIdentifier,
    pub subject: ActivationSubject,
    pub role: String,
    pub operational_baseline: OperationalBaselineIdentifier,
    pub deployment_context: ActivationContextIdentifier,
    pub recognition_authority: AuthorityBinding,
    pub scope: ActivationScopeIdentifier,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalBaseline {
    pub id: OperationalBaselineIdentifier,
    pub subject: ActivationSubject,
    pub configuration_baseline: String,
    pub limitations: Vec<String>,
    pub evidence: Vec<EvidenceRecordId>,
    pub assurance: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationRecord {
    pub id: ActivationRecordIdentifier,
    pub act: ActivationActIdentifier,
    pub assignment: OperationalAssignment,
    pub historical: bool,
    pub runtime_execution_claimed: bool,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalStanding {
    pub assignment: OperationalAssignmentIdentifier,
    pub release: ReleaseBinding,
    pub scope: ActivationScopeIdentifier,
    pub environment: ActivationEnvironmentIdentifier,
    pub state: ActivationState,
    pub limitations: Vec<String>,
    pub historical: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalTransition {
    pub id: ActivationRecordIdentifier,
    pub sequence: u64,
    pub transition: ActivationTransition,
    pub prior_standing: Option<OperationalStanding>,
    pub resulting_standing: Option<OperationalStanding>,
    pub subject: ActivationSubject,
    pub target: ActivationTarget,
    pub decision: ActivationDecisionIdentifier,
    pub authorization: Option<ActivationAuthorization>,
    pub evidence: Vec<EvidenceRecordId>,
    pub assurance: Vec<String>,
    pub traceability: Vec<TraceRef>,
    pub limitations: Vec<String>,
    pub conditions: Vec<String>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailedActivationRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationSuspensionRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationRestorationRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationRollbackRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeactivationRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactivationRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationTransferRecord {
    pub transition: HistoricalTransition,
    pub new_target: ActivationTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionSubject {
    pub id: RecognitionSubjectIdentifier,
    pub release: ReleaseBinding,
    pub activation: Option<ActivationRecordIdentifier>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionRequest {
    pub id: RecognitionRequestIdentifier,
    pub subject: RecognitionSubject,
    pub scope: ActivationScopeIdentifier,
    pub context: ActivationContextIdentifier,
    pub criteria: Vec<RecognitionCriteriaIdentifier>,
    pub authority: AuthorityBinding,
    pub evidence: Vec<EvidenceRecordId>,
    pub verification: Vec<String>,
    pub assurance: Vec<String>,
    pub traceability: Vec<TraceRef>,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionCriteria {
    pub id: RecognitionCriteriaIdentifier,
    pub statement: String,
    pub required: bool,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionFinding {
    pub id: RecognitionFindingIdentifier,
    pub subject: RecognitionSubjectIdentifier,
    pub criteria: RecognitionCriteriaIdentifier,
    pub statement: String,
    pub limitation: Option<String>,
    pub status: RecognitionFindingStatus,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecognitionFindingStatus {
    Satisfied,
    Unsatisfied,
    Indeterminate,
    NotApplicable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecognitionDecisionOutcome {
    Recognized,
    RecognizedWithConditions,
    Denied,
    Indeterminate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionDecision {
    pub id: RecognitionDecisionIdentifier,
    pub request: RecognitionRequestIdentifier,
    pub subject: RecognitionSubjectIdentifier,
    pub scope: ActivationScopeIdentifier,
    pub outcome: RecognitionDecisionOutcome,
    pub findings: Vec<RecognitionFindingIdentifier>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub authority: AuthorityBinding,
    pub jurisdiction: String,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecognitionState {
    RecognitionPending,
    OperationallyRecognized,
    RecognizedWithConditions,
    RecognitionDenied,
    RecognitionSuspended,
    RecognitionWithdrawn,
    RecognitionIndeterminate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalRecognition {
    pub id: RecognitionRecordIdentifier,
    pub subject: RecognitionSubject,
    pub scope: ActivationScopeIdentifier,
    pub state: RecognitionState,
    pub decision: RecognitionDecisionIdentifier,
    pub historical: bool,
    pub truth_claimed: bool,
    pub conformance_claimed: bool,
    pub certification_claimed: bool,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionSuspensionRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionRestorationRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionWithdrawalRecord {
    pub transition: HistoricalTransition,
    pub reason: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionHistoricalStandingRecord {
    pub recognition: OperationalRecognition,
    pub preserved_from: RecognitionRecordIdentifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationTraceabilityBinding {
    pub request: ActivationRequestIdentifier,
    pub decision: ActivationDecisionIdentifier,
    pub act: ActivationActIdentifier,
    pub record: ActivationRecordIdentifier,
    pub recognition: Option<RecognitionRecordIdentifier>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityBinding {
    pub assessment: String,
    pub subject: ReleaseBinding,
    pub target: ReleaseBinding,
    pub outcome: CompatibilityOutcome,
    pub dimension: String,
    pub conditions: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityResolution {
    pub reference: String,
    pub jurisdiction: String,
    pub subject: ActivationSubjectIdentifier,
    pub scope: ActivationScopeIdentifier,
    pub environment: ActivationEnvironmentIdentifier,
    pub valid: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationResolutionContext {
    pub releases: Vec<ReleasedImplementation>,
    pub compatibility: Vec<CompatibilityBinding>,
    pub authorities: Vec<AuthorityResolution>,
    pub admitted_source_set: String,
    pub implementation_version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionResolutionContext {
    pub authorities: Vec<AuthorityResolution>,
    pub criteria: Vec<RecognitionCriteriaIdentifier>,
    pub implementation_version: ImplementationVersion,
    pub findings: Vec<RecognitionFindingIdentifier>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub traceability_version: ImplementationVersion,
    pub required_criteria: Vec<RecognitionCriteria>,
    pub finding_records: Vec<RecognitionFinding>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecognitionStandingTransition {
    pub id: RecognitionRecordIdentifier,
    pub prior: RecognitionState,
    pub resulting: RecognitionState,
    pub subject: RecognitionSubject,
    pub decision: RecognitionDecisionIdentifier,
    pub implementation_version: ImplementationVersion,
    pub historical: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleCorrection {
    pub replacement: ActivationRecordIdentifier,
    pub preserved_from: ActivationRecordIdentifier,
    pub subject: ActivationSubject,
    pub implementation_version: ImplementationVersion,
    pub historical: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationFindingCode {
    MissingSubject,
    MissingReleaseBinding,
    UnknownRelease,
    MissingTarget,
    MissingScope,
    MissingEnvironment,
    MissingContext,
    MissingSourceSet,
    MissingAuthority,
    MissingJurisdiction,
    MissingAuthorization,
    MismatchedAuthorizationRelease,
    MismatchedAuthorizationScope,
    MismatchedAuthorizationEnvironment,
    InvalidAuthorizationStatus,
    SelfAuthorization,
    InferredFromPublication,
    InferredFromDeployment,
    InferredFromCompatibility,
    InferredFromTest,
    InferredFromEvidence,
    InferredFromAssurance,
    IneligibleActivation,
    ActivationNotAuthorized,
    IllegalTransition,
    RuntimeExecutionClaim,
    MissingRecognitionScope,
    MissingRecognitionCriteria,
    RecognitionWithoutActivation,
    SelfRecognition,
    RecognitionClaimBoundary,
    DuplicateRequest,
    ConflictingDetermination,
    HistoricalMutation,
    MissingCompatibility,
    IncompatibleRelease,
    IndeterminateCompatibility,
    MissingResolvedAuthority,
    JurisdictionOutOfScope,
    AuthorizationDecisionMismatch,
    InvalidHistory,
    MissingPriorStanding,
    InvalidRestoration,
    InvalidRollback,
    InvalidTransfer,
    InvalidDeactivation,
    InvalidReactivation,
    DuplicateHistory,
    DuplicateSequence,
    MissingPredecessor,
    StandingMismatch,
    TransitionResultMismatch,
    HistorySubjectMismatch,
    HistoryReleaseMismatch,
    HistoryAssignmentMismatch,
    HistoryVersionMismatch,
    RollbackRestoration,
    OrphanHistory,
    RecognitionVersionMismatch,
    RecognitionAuthorityMismatch,
    DuplicateRecognitionCriterion,
    UnknownRecognitionFinding,
    RecognitionOutcomeMismatch,
    MissingRecognitionCondition,
    InvalidRecognitionCondition,
    InvalidRecognitionLimitation,
    RecognitionScopeMismatch,
    DuplicateRecognitionFinding,
    MissingRequiredRecognitionFinding,
    UnsatisfiedRecognitionCriterion,
    ContradictoryRecognitionFinding,
    ActivationTraceabilityMismatch,
    ActivationTraceabilityVersionMismatch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationFinding {
    pub code: ActivationFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivationValidationResult {
    pub valid: bool,
    pub findings: Vec<ActivationFinding>,
    pub execution_performed: bool,
    pub deployment_performed: bool,
    pub authority_created: bool,
    pub conformance_established: bool,
    pub certification_established: bool,
}

pub fn validate_activation(
    request: &ActivationRequest,
    eligibility: &OperationalEligibilityDetermination,
    decision: &ActivationDecision,
) -> ActivationValidationResult {
    let mut findings = Vec::new();
    if request.subject.release.release.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingReleaseBinding,
            "release binding is required",
        );
    }
    if request.target.id.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingTarget,
            "activation target is required",
        );
    }
    if request.target.scope.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingScope,
            "activation scope is required",
        );
    }
    if request.target.environment.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingEnvironment,
            "activation environment is required",
        );
    }
    if request.target.context.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingContext,
            "activation context is required",
        );
    }
    if request.source_set.trim().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingSourceSet,
            "admitted source set binding is required",
        );
    }
    if request.authority.authority_reference.trim().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingAuthority,
            "external authority reference is required",
        );
    }
    if request.authority.jurisdiction_reference.trim().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingJurisdiction,
            "jurisdiction reference is required",
        );
    }
    if eligibility.request != request.id {
        push(
            &mut findings,
            ActivationFindingCode::ConflictingDetermination,
            "eligibility does not belong to this request",
        );
    }
    if decision.request != request.id || decision.eligibility != eligibility.id {
        push(
            &mut findings,
            ActivationFindingCode::ConflictingDetermination,
            "activation decision bindings are inconsistent",
        );
    }
    if decision.authorization != request.authorization {
        push(
            &mut findings,
            ActivationFindingCode::AuthorizationDecisionMismatch,
            "activation decision must reference the request authorization exactly",
        );
    }
    if !matches!(
        request.inference_basis,
        ActivationInferenceBasis::ExplicitRequest
    ) {
        push(
            &mut findings,
            inference_code(&request.inference_basis),
            "activation must not be inferred from a non-constitutional signal",
        );
    }
    let Some(authorization) = request.authorization.as_ref() else {
        push(
            &mut findings,
            ActivationFindingCode::MissingAuthorization,
            "activation requires recorded authorization",
        );
        return result(findings);
    };
    if authorization.release != request.subject.release {
        push(
            &mut findings,
            ActivationFindingCode::MismatchedAuthorizationRelease,
            "authorization release does not match subject release",
        );
    }
    if authorization.scope != request.target.scope {
        push(
            &mut findings,
            ActivationFindingCode::MismatchedAuthorizationScope,
            "authorization scope does not match target scope",
        );
    }
    if authorization.environment != request.target.environment {
        push(
            &mut findings,
            ActivationFindingCode::MismatchedAuthorizationEnvironment,
            "authorization environment does not match target environment",
        );
    }
    if !matches!(authorization.status, AuthorizationStatus::Current) {
        push(
            &mut findings,
            ActivationFindingCode::InvalidAuthorizationStatus,
            "authorization is not current",
        );
    }
    if authorization.authority.authority_reference == request.subject.id.as_str() {
        push(
            &mut findings,
            ActivationFindingCode::SelfAuthorization,
            "implementation cannot self-authorize",
        );
    }
    if !matches!(eligibility.outcome, EligibilityOutcome::Eligible) {
        push(
            &mut findings,
            ActivationFindingCode::IneligibleActivation,
            "activation requires eligible determination",
        );
    }
    if !matches!(decision.outcome, ActivationDecisionOutcome::Authorized) {
        push(
            &mut findings,
            ActivationFindingCode::ActivationNotAuthorized,
            "activation requires authorized decision",
        );
    }
    result(findings)
}

/// Resolves only the bounded references supplied by the caller. It does not
/// create release, authority, jurisdiction, compatibility, or traceability meaning.
pub fn validate_activation_with_context(
    request: &ActivationRequest,
    eligibility: &OperationalEligibilityDetermination,
    decision: &ActivationDecision,
    context: &ActivationResolutionContext,
) -> ActivationValidationResult {
    let mut result = validate_activation(request, eligibility, decision);
    if context.admitted_source_set.trim().is_empty()
        || context.implementation_version != request.implementation_version
    {
        push_result(
            &mut result,
            ActivationFindingCode::MissingSourceSet,
            "resolution context is not bound to the request",
        );
    }
    let Some(release) = context
        .releases
        .iter()
        .find(|release| release.identity.id == request.subject.release.release)
    else {
        push_result(
            &mut result,
            ActivationFindingCode::UnknownRelease,
            "activation release is not in the admitted release set",
        );
        return result;
    };
    if release.identity.version != request.subject.release.version
        || release.identity.implementation_version != request.implementation_version
    {
        push_result(
            &mut result,
            ActivationFindingCode::MismatchedAuthorizationRelease,
            "release identity/version/implementation binding mismatch",
        );
    }
    if !matches!(release.state, ReleaseState::Released) {
        push_result(
            &mut result,
            ActivationFindingCode::IneligibleActivation,
            "release state is not eligible for activation",
        );
    }
    if let Some(target) = &request.compatibility_target {
        let Some(binding) = context.compatibility.iter().find(|binding| {
            binding.subject == request.subject.release && binding.target == *target
        }) else {
            push_result(
                &mut result,
                ActivationFindingCode::MissingCompatibility,
                "required compatibility determination is missing",
            );
            return result;
        };
        if !matches!(
            binding.outcome,
            CompatibilityOutcome::Compatible | CompatibilityOutcome::ConditionallyCompatible
        ) {
            push_result(
                &mut result,
                if matches!(binding.outcome, CompatibilityOutcome::Indeterminate) {
                    ActivationFindingCode::IndeterminateCompatibility
                } else {
                    ActivationFindingCode::IncompatibleRelease
                },
                "compatibility outcome does not support activation",
            );
        }
        if matches!(
            binding.outcome,
            CompatibilityOutcome::ConditionallyCompatible
        ) && binding.conditions.is_empty()
        {
            push_result(
                &mut result,
                ActivationFindingCode::MissingCompatibility,
                "conditional compatibility requires conditions",
            );
        }
    }
    let Some(authority) = context.authorities.iter().find(|authority| {
        authority.reference == request.authority.authority_reference
            && authority.subject == request.subject.id
    }) else {
        push_result(
            &mut result,
            ActivationFindingCode::MissingResolvedAuthority,
            "authority reference is not resolved for the activation subject",
        );
        return result;
    };
    if !authority.valid
        || authority.jurisdiction != request.authority.jurisdiction_reference
        || authority.scope != request.target.scope
        || authority.environment != request.target.environment
    {
        push_result(
            &mut result,
            ActivationFindingCode::JurisdictionOutOfScope,
            "authority/jurisdiction resolution does not cover target scope and environment",
        );
    }
    result
}

pub fn validate_transition_history(history: &[HistoricalTransition]) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    let mut ids = std::collections::BTreeSet::new();
    let mut sequences = std::collections::BTreeSet::new();
    let mut ordered = history.to_vec();
    ordered.sort_by_key(|record| record.sequence);
    for record in &ordered {
        if !ids.insert(record.id.clone()) {
            push_result(
                &mut result,
                ActivationFindingCode::DuplicateHistory,
                "historical transition identifier is duplicated",
            );
        }
        if !sequences.insert(record.sequence) {
            push_result(
                &mut result,
                ActivationFindingCode::DuplicateSequence,
                "historical transition sequence position is duplicated",
            );
        }
        match record.transition {
            ActivationTransition::Request
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Requested) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "request must produce requested standing",
                )
            }
            ActivationTransition::DetermineEligibility
                if record.resulting_standing.as_ref().is_none_or(|standing| {
                    !matches!(
                        standing.state,
                        ActivationState::Eligible
                            | ActivationState::Ineligible
                            | ActivationState::Indeterminate
                            | ActivationState::InvalidRequest
                    )
                }) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "eligibility determination has an invalid resulting standing",
                )
            }
            ActivationTransition::Authorize
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Authorized) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "authorization must produce authorized standing",
                )
            }
            ActivationTransition::Activate
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Activated) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "activation must produce active standing",
                );
            }
            ActivationTransition::Fail
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::ActivationFailed) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "failed activation must produce failed standing",
                );
            }
            ActivationTransition::Suspend
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Suspended) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "suspension must produce suspended standing",
                );
            }
            ActivationTransition::Restore => {
                if record
                    .prior_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Suspended)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::InvalidRestoration,
                        "restoration requires prior suspended standing",
                    );
                }
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Activated)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::TransitionResultMismatch,
                        "restoration must produce active standing",
                    );
                }
            }
            ActivationTransition::Rollback => {
                if record.prior_standing.is_none() || record.resulting_standing.is_none() {
                    push_result(
                        &mut result,
                        ActivationFindingCode::InvalidRollback,
                        "rollback requires prior activation standing",
                    );
                }
                if record
                    .resulting_standing
                    .as_ref()
                    .is_some_and(|standing| standing.state == ActivationState::Activated)
                    || record.resulting_standing == record.prior_standing
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::RollbackRestoration,
                        "rollback cannot restore or reproduce the prior active standing",
                    );
                }
            }
            ActivationTransition::Transfer => {
                if record.authorization.is_none()
                    || record
                        .resulting_standing
                        .as_ref()
                        .is_none_or(|standing| standing.scope != record.target.scope)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::InvalidTransfer,
                        "transfer requires authorization and target-scoped resulting standing",
                    );
                }
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Activated)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::TransitionResultMismatch,
                        "transfer must preserve active standing",
                    );
                }
            }
            ActivationTransition::Deactivate => {
                if record
                    .prior_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Activated)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::InvalidDeactivation,
                        "deactivation requires prior active standing",
                    );
                }
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Deactivated)
                {
                    push_result(
                        &mut result,
                        ActivationFindingCode::TransitionResultMismatch,
                        "deactivation must produce deactivated standing",
                    );
                }
            }
            ActivationTransition::Reactivate
                if record.prior_standing.as_ref().is_none_or(|standing| {
                    standing.state != ActivationState::EligibleForReactivation
                }) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::InvalidReactivation,
                    "reactivation requires prior eligible standing",
                );
            }
            ActivationTransition::Reactivate
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Reactivated) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "reactivation must produce reactivated standing",
                );
            }
            ActivationTransition::Withdraw
                if record
                    .resulting_standing
                    .as_ref()
                    .is_none_or(|standing| standing.state != ActivationState::Withdrawn) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "withdrawal must produce withdrawn standing",
                )
            }
            ActivationTransition::MarkEligibleForReactivation
                if record.resulting_standing.as_ref().is_none_or(|standing| {
                    standing.state != ActivationState::EligibleForReactivation
                }) =>
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::TransitionResultMismatch,
                    "eligibility marking must produce reactivation-eligible standing",
                )
            }
            _ => {}
        }
        if record.prior_standing.is_none() && record.resulting_standing.is_none() {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidHistory,
                "historical transition must preserve prior or resulting standing",
            );
        }
        for standing in record
            .prior_standing
            .iter()
            .chain(record.resulting_standing.iter())
        {
            if standing.release != record.subject.release {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryReleaseMismatch,
                    "standing release does not match the historical subject",
                );
            }
            if standing.scope != record.target.scope
                || standing.environment != record.target.environment
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryAssignmentMismatch,
                    "standing scope or environment does not match the historical target",
                );
            }
            if standing.assignment.as_str().is_empty() {
                push_result(
                    &mut result,
                    ActivationFindingCode::InvalidHistory,
                    "every governed transition requires an assignment-bound standing",
                );
            }
        }
        if matches!(record.transition, ActivationTransition::Suspend)
            && record
                .prior_standing
                .as_ref()
                .is_none_or(|standing| standing.state != ActivationState::Activated)
        {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidHistory,
                "suspension must preserve the prior active standing",
            );
        }
        if matches!(record.transition, ActivationTransition::Withdraw)
            && record.prior_standing.as_ref().is_none_or(|standing| {
                !matches!(
                    standing.state,
                    ActivationState::Activated | ActivationState::Suspended
                )
            })
        {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidHistory,
                "withdrawal must preserve prior active or suspended standing",
            );
        }
        if record.sequence > 0 {
            let Some(previous) = ordered
                .iter()
                .find(|candidate| candidate.sequence + 1 == record.sequence)
            else {
                push_result(
                    &mut result,
                    ActivationFindingCode::MissingPredecessor,
                    "history sequence has a missing predecessor",
                );
                continue;
            };
            if previous.resulting_standing != record.prior_standing {
                push_result(
                    &mut result,
                    ActivationFindingCode::StandingMismatch,
                    "prior standing does not equal the predecessor result",
                );
            }
            if previous.subject != record.subject {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistorySubjectMismatch,
                    "history subject changed without an explicit transfer",
                );
            }
            if previous.subject.release != record.subject.release {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryReleaseMismatch,
                    "history release binding changed",
                );
            }
            if previous.implementation_version != record.implementation_version {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryVersionMismatch,
                    "history implementation version changed",
                );
            }
            if previous.target != record.target
                && record.transition != ActivationTransition::Transfer
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryAssignmentMismatch,
                    "history target changed outside transfer",
                );
            }
        }
    }
    result
}

/// Validates assignment-bound continuity without imposing a predecessor
/// requirement on an initial record or creating a global assignment graph.
pub fn validate_assignment_history(
    history: &[HistoricalTransition],
    assignments: &[OperationalAssignment],
    expected_version: &ImplementationVersion,
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    let mut known = std::collections::BTreeMap::new();
    for assignment in assignments {
        if assignment.role.trim().is_empty()
            || assignment.operational_baseline.as_str().is_empty()
            || assignment.deployment_context.as_str().is_empty()
            || assignment
                .recognition_authority
                .authority_reference
                .trim()
                .is_empty()
            || assignment.scope.as_str().is_empty()
        {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidHistory,
                "assignment requires role, baseline, context, authority, and scope",
            );
        }
        if let Some(previous) = known.insert(assignment.id.clone(), assignment)
            && previous != assignment
        {
            push_result(
                &mut result,
                ActivationFindingCode::HistoryAssignmentMismatch,
                "assignment identity is reused for contradictory bound fields",
            );
        }
    }
    let ordered = {
        let mut records = history.to_vec();
        records.sort_by_key(|record| record.sequence);
        records
    };
    for record in &ordered {
        if record.implementation_version != *expected_version {
            push_result(
                &mut result,
                ActivationFindingCode::HistoryVersionMismatch,
                "historical assignment record differs from the expected implementation version",
            );
        }
        for standing in record
            .prior_standing
            .iter()
            .chain(record.resulting_standing.iter())
        {
            let Some(assignment) = known.get(&standing.assignment) else {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryAssignmentMismatch,
                    "standing references an unresolved assignment",
                );
                continue;
            };
            if assignment.subject != record.subject
                || assignment.scope != standing.scope
                || assignment.deployment_context != record.target.context
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistorySubjectMismatch,
                    "standing is not bound to the historical assignment subject, scope, or context",
                );
            }
        }
        if record.sequence > 0
            && let Some(previous) = ordered
                .iter()
                .find(|candidate| candidate.sequence + 1 == record.sequence)
        {
            let previous_assignment = previous
                .resulting_standing
                .as_ref()
                .map(|standing| standing.assignment.clone());
            let current_assignment = record
                .prior_standing
                .as_ref()
                .map(|standing| standing.assignment.clone());
            if previous_assignment != current_assignment
                && record.transition != ActivationTransition::Transfer
            {
                push_result(
                    &mut result,
                    ActivationFindingCode::HistoryAssignmentMismatch,
                    "assignment-bound identity changed without an explicit transfer",
                );
            }
        }
    }
    result
}

pub fn validate_recognition(
    request: &RecognitionRequest,
    decision: &RecognitionDecision,
) -> ActivationValidationResult {
    let mut findings = Vec::new();
    if request.scope.as_str().is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingRecognitionScope,
            "recognition scope is required",
        );
    }
    if request.criteria.is_empty() {
        push(
            &mut findings,
            ActivationFindingCode::MissingRecognitionCriteria,
            "recognition criteria are required",
        );
    }
    if request.subject.activation.is_none() {
        push(
            &mut findings,
            ActivationFindingCode::RecognitionWithoutActivation,
            "recognition requires an activation record where bounded by IMP-009",
        );
    }
    if request.authority.authority_reference == request.subject.id.as_str() {
        push(
            &mut findings,
            ActivationFindingCode::SelfRecognition,
            "implementation cannot self-recognize",
        );
    }
    if decision.request != request.id {
        push(
            &mut findings,
            ActivationFindingCode::ConflictingDetermination,
            "recognition decision does not belong to request",
        );
    }
    if decision.subject != request.subject.id || decision.scope != request.scope {
        push(
            &mut findings,
            ActivationFindingCode::RecognitionOutcomeMismatch,
            "recognition decision subject or scope does not match the request",
        );
    }
    result(findings)
}

pub fn validate_recognition_with_context(
    request: &RecognitionRequest,
    decision: &RecognitionDecision,
    context: &RecognitionResolutionContext,
) -> ActivationValidationResult {
    let mut result = validate_recognition(request, decision);
    if request.implementation_version != context.implementation_version
        || decision.implementation_version != request.implementation_version
        || context.traceability_version != request.implementation_version
    {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionVersionMismatch,
            "recognition request, decision, and resolution context versions must match",
        );
    }
    if decision.authority != request.authority
        || decision.jurisdiction != request.authority.jurisdiction_reference
    {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionAuthorityMismatch,
            "recognition decision authority or jurisdiction does not match the request",
        );
    }
    if !context.authorities.iter().any(|authority| {
        authority.reference == request.authority.authority_reference
            && authority.valid
            && authority.scope == request.scope
    }) {
        push_result(
            &mut result,
            ActivationFindingCode::MissingResolvedAuthority,
            "recognizing authority is not resolved for the requested scope",
        );
    }
    if context.authorities.iter().any(|authority| {
        authority.reference == request.authority.authority_reference
            && authority.subject.as_str() != request.subject.id.as_str()
    }) {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionAuthorityMismatch,
            "recognition authority subject does not match the recognition subject",
        );
    }
    for criterion in &request.criteria {
        if !context.criteria.contains(criterion) {
            push_result(
                &mut result,
                ActivationFindingCode::MissingRecognitionCriteria,
                "recognition criterion is not resolved",
            );
        }
    }
    let mut criteria = std::collections::BTreeSet::new();
    for criterion in &request.criteria {
        if !criteria.insert(criterion) {
            push_result(
                &mut result,
                ActivationFindingCode::DuplicateRecognitionCriterion,
                "recognition criteria must be unique",
            );
        }
    }
    for finding in &decision.findings {
        if !context.findings.contains(finding) {
            push_result(
                &mut result,
                ActivationFindingCode::UnknownRecognitionFinding,
                "recognition decision references an unknown finding",
            );
        }
    }
    let mut finding_statuses = std::collections::BTreeMap::new();
    for finding in &context.finding_records {
        if !request.criteria.contains(&finding.criteria)
            || finding.subject != request.subject.id
            || finding.implementation_version != request.implementation_version
        {
            push_result(
                &mut result,
                ActivationFindingCode::RecognitionOutcomeMismatch,
                "recognition finding is bound to another subject, criterion, or version",
            );
        }
        if let Some(previous) =
            finding_statuses.insert(finding.criteria.clone(), finding.status.clone())
            && previous != finding.status
        {
            push_result(
                &mut result,
                ActivationFindingCode::ContradictoryRecognitionFinding,
                "recognition findings for one criterion contradict one another",
            );
        }
    }
    for criterion in &context.required_criteria {
        if criterion.required && !request.criteria.contains(&criterion.id) {
            push_result(
                &mut result,
                ActivationFindingCode::MissingRequiredRecognitionFinding,
                "required recognition criterion was omitted",
            );
        }
        if criterion.implementation_version != request.implementation_version {
            push_result(
                &mut result,
                ActivationFindingCode::RecognitionVersionMismatch,
                "recognition criterion version does not match the request",
            );
        }
        if criterion.required {
            match finding_statuses.get(&criterion.id) {
                Some(
                    RecognitionFindingStatus::Satisfied | RecognitionFindingStatus::NotApplicable,
                ) => {}
                Some(
                    RecognitionFindingStatus::Unsatisfied | RecognitionFindingStatus::Indeterminate,
                )
                | None => {
                    push_result(
                        &mut result,
                        ActivationFindingCode::UnsatisfiedRecognitionCriterion,
                        "required recognition criterion is not satisfied",
                    );
                }
            }
        }
    }
    if matches!(decision.outcome, RecognitionDecisionOutcome::Recognized)
        && (decision.findings.is_empty() || !decision.conditions.is_empty())
    {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionOutcomeMismatch,
            "unconditional recognition requires findings and no unresolved conditions",
        );
    }
    if matches!(
        decision.outcome,
        RecognitionDecisionOutcome::RecognizedWithConditions
    ) && decision.conditions.is_empty()
    {
        push_result(
            &mut result,
            ActivationFindingCode::MissingRecognitionCondition,
            "conditional recognition requires explicit conditions",
        );
    }
    for condition in &decision.conditions {
        if !context.conditions.contains(condition)
            || decision
                .conditions
                .iter()
                .filter(|item| *item == condition)
                .count()
                > 1
        {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidRecognitionCondition,
                "recognition condition is unknown or duplicated",
            );
        }
    }
    for limitation in &decision.limitations {
        let lower = limitation.to_ascii_lowercase();
        if !context.limitations.contains(limitation)
            || lower.contains("certif")
            || lower.contains("conform")
            || lower.contains("expand")
            || lower.contains("global")
        {
            push_result(
                &mut result,
                ActivationFindingCode::InvalidRecognitionLimitation,
                "recognition limitation is unknown or makes a prohibited claim",
            );
        }
    }
    result
}

pub fn validate_recognition_standing_history(
    transitions: &[RecognitionStandingTransition],
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    let mut ids = std::collections::BTreeSet::new();
    for transition in transitions {
        if !ids.insert(transition.id.clone()) || !transition.historical {
            push_result(
                &mut result,
                ActivationFindingCode::HistoricalMutation,
                "recognition transition identity must be unique and historical",
            );
        }
        if transition.subject.activation.is_none()
            || transition.implementation_version.as_str().is_empty()
        {
            push_result(
                &mut result,
                ActivationFindingCode::RecognitionOutcomeMismatch,
                "recognition transition requires activation and implementation bindings",
            );
        }
        let allowed = matches!(
            (&transition.prior, &transition.resulting),
            (
                RecognitionState::OperationallyRecognized,
                RecognitionState::RecognitionSuspended
            ) | (
                RecognitionState::RecognitionSuspended,
                RecognitionState::OperationallyRecognized
            ) | (
                RecognitionState::OperationallyRecognized | RecognitionState::RecognitionSuspended,
                RecognitionState::RecognitionWithdrawn
            )
        );
        if !allowed {
            push_result(
                &mut result,
                ActivationFindingCode::RecognitionOutcomeMismatch,
                "recognition transition is outside the bounded standing transition set",
            );
        }
    }
    result
}

pub fn validate_lifecycle_corrections(
    corrections: &[LifecycleCorrection],
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    let mut replacements = std::collections::BTreeSet::new();
    for correction in corrections {
        if correction.replacement == correction.preserved_from
            || !replacements.insert(correction.replacement.clone())
            || !correction.historical
        {
            push_result(
                &mut result,
                ActivationFindingCode::HistoricalMutation,
                "lifecycle correction must preserve a distinct historical predecessor",
            );
        }
        if correction.subject.id.as_str().is_empty()
            || correction.implementation_version.as_str().is_empty()
        {
            push_result(
                &mut result,
                ActivationFindingCode::HistorySubjectMismatch,
                "lifecycle correction requires subject and version bindings",
            );
        }
    }
    result
}

pub fn validate_recognition_record(
    recognition: &OperationalRecognition,
    request: &RecognitionRequest,
    decision: &RecognitionDecision,
    findings: &[RecognitionFinding],
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    if recognition.subject != request.subject
        || recognition.scope != request.scope
        || recognition.decision != decision.id
        || recognition.implementation_version != decision.implementation_version
    {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionOutcomeMismatch,
            "recognition record is not bound to its request, decision, scope, or version",
        );
    }
    if matches!(
        decision.outcome,
        RecognitionDecisionOutcome::Denied | RecognitionDecisionOutcome::Indeterminate
    ) && matches!(
        recognition.state,
        RecognitionState::OperationallyRecognized | RecognitionState::RecognizedWithConditions
    ) {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionOutcomeMismatch,
            "denied or indeterminate recognition cannot produce recognized standing",
        );
    }
    if decision.conditions != recognition.conditions
        || decision.limitations != recognition.limitations
    {
        push_result(
            &mut result,
            ActivationFindingCode::RecognitionOutcomeMismatch,
            "recognition conditions and limitations must be preserved",
        );
    }
    let mut seen = std::collections::BTreeSet::new();
    for finding in findings {
        if !seen.insert(finding.id.clone()) {
            push_result(
                &mut result,
                ActivationFindingCode::DuplicateRecognitionFinding,
                "recognition findings must be unique",
            );
        }
        if finding.subject != request.subject.id
            || !request.criteria.contains(&finding.criteria)
            || finding.implementation_version != request.implementation_version
        {
            push_result(
                &mut result,
                ActivationFindingCode::RecognitionOutcomeMismatch,
                "recognition finding subject, criterion, or version does not match the request",
            );
        }
        if matches!(decision.outcome, RecognitionDecisionOutcome::Recognized)
            && matches!(
                finding.status,
                RecognitionFindingStatus::Unsatisfied | RecognitionFindingStatus::Indeterminate
            )
        {
            push_result(
                &mut result,
                ActivationFindingCode::UnsatisfiedRecognitionCriterion,
                "recognized standing cannot contain an unsatisfied or indeterminate criterion",
            );
        }
    }
    validate_recognition_claims(recognition)
        .findings
        .into_iter()
        .for_each(|finding| {
            result.findings.push(finding);
        });
    result.valid = result.findings.is_empty();
    result
}

pub fn validate_recognition_history(
    records: &[RecognitionHistoricalStandingRecord],
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    let mut preserved = std::collections::BTreeSet::new();
    for record in records {
        if !record.recognition.historical {
            push_result(
                &mut result,
                ActivationFindingCode::HistoricalMutation,
                "recognition history record must remain historical",
            );
        }
        if !preserved.insert(record.preserved_from.clone()) {
            push_result(
                &mut result,
                ActivationFindingCode::DuplicateHistory,
                "recognition history source identity is duplicated",
            );
        }
        if record.recognition.id == record.preserved_from {
            push_result(
                &mut result,
                ActivationFindingCode::HistoricalMutation,
                "recognition history cannot preserve itself as a replacement",
            );
        }
    }
    result
}

pub fn validate_activation_traceability(
    binding: &ActivationTraceabilityBinding,
    request: &ActivationRequest,
    decision: &ActivationDecision,
    record: &ActivationRecord,
    recognition: Option<&OperationalRecognition>,
) -> ActivationValidationResult {
    let mut result = result(Vec::new());
    if binding.request != request.id
        || binding.decision != decision.id
        || binding.act != record.act
        || binding.record != record.id
        || decision.request != request.id
        || record.assignment.subject != request.subject
    {
        push_result(
            &mut result,
            ActivationFindingCode::ActivationTraceabilityMismatch,
            "activation request, decision, act, and assignment are not one subject-bound chain",
        );
    }
    if binding.implementation_version != request.implementation_version
        || record.implementation_version != request.implementation_version
    {
        push_result(
            &mut result,
            ActivationFindingCode::ActivationTraceabilityVersionMismatch,
            "activation record version differs from request version",
        );
    }
    if let Some(recognition) = recognition
        && (binding.recognition != Some(recognition.id.clone())
            || recognition.subject.release != request.subject.release
            || recognition.subject.activation != Some(record.id.clone())
            || recognition.implementation_version != request.implementation_version)
    {
        push_result(
            &mut result,
            ActivationFindingCode::ActivationTraceabilityMismatch,
            "recognition is not bound to the activated release and record",
        );
    }
    result
}

pub fn validate_recognition_claims(
    recognition: &OperationalRecognition,
) -> ActivationValidationResult {
    let mut findings = Vec::new();
    if recognition.truth_claimed
        || recognition.conformance_claimed
        || recognition.certification_claimed
    {
        push(
            &mut findings,
            ActivationFindingCode::RecognitionClaimBoundary,
            "recognition does not establish truth, conformance, or certification",
        );
    }
    result(findings)
}

pub fn recognition_canonical(recognition: &OperationalRecognition) -> CanonicalObject {
    let mut fields = vec![
        field(
            "recognition_id",
            CanonicalValue::Text(recognition.id.to_string()),
            true,
            true,
        ),
        field(
            "subject",
            CanonicalValue::Text(recognition.subject.id.to_string()),
            true,
            true,
        ),
        field(
            "scope",
            CanonicalValue::Text(recognition.scope.to_string()),
            true,
            true,
        ),
        field(
            "state",
            CanonicalValue::Text(format!("{:?}", recognition.state)),
            true,
            false,
        ),
        field(
            "decision",
            CanonicalValue::Text(recognition.decision.to_string()),
            true,
            false,
        ),
        field(
            "implementation_version",
            CanonicalValue::Text(recognition.implementation_version.to_string()),
            true,
            true,
        ),
        field(
            "conditions",
            CanonicalValue::Collection(CanonicalCollection::ordered(
                recognition
                    .conditions
                    .iter()
                    .cloned()
                    .map(CanonicalValue::Text)
                    .collect(),
            )),
            false,
            false,
        ),
        field(
            "limitations",
            CanonicalValue::Collection(CanonicalCollection::ordered(
                recognition
                    .limitations
                    .iter()
                    .cloned()
                    .map(CanonicalValue::Text)
                    .collect(),
            )),
            false,
            false,
        ),
    ];
    fields.sort_by(|left, right| left.name.cmp(&right.name));
    CanonicalObject {
        representation_id: CanonicalRepresentationId::new(format!(
            "recognition-{}",
            recognition.id
        ))
        .expect("validated recognition id"),
        kind: CanonicalRepresentationKind::ActivationReference,
        representation_version: REPRESENTATION_MODEL_VERSION.into(),
        schema_id: ACTIVATION_PROFILE_ID.into(),
        source_binding: None,
        implementation_version: Some(recognition.implementation_version.clone()),
        required_fields: fields
            .iter()
            .filter(|field| field.required)
            .map(|field| field.name.clone())
            .collect(),
        extension_namespaces: Vec::new(),
        fields,
    }
}

pub fn activation_canonical(
    subject: &ActivationSubject,
    target: &ActivationTarget,
    state: &ActivationState,
) -> CanonicalObject {
    let mut fields = vec![
        field(
            "environment",
            CanonicalValue::Text(target.environment.to_string()),
            true,
            true,
        ),
        field(
            "release",
            CanonicalValue::Text(subject.release.release.to_string()),
            true,
            true,
        ),
        field(
            "scope",
            CanonicalValue::Text(target.scope.to_string()),
            true,
            true,
        ),
        field(
            "state",
            CanonicalValue::Text(format!("{state:?}")),
            true,
            false,
        ),
        field(
            "subject",
            CanonicalValue::Text(subject.id.to_string()),
            true,
            true,
        ),
        field(
            "target",
            CanonicalValue::Text(target.id.to_string()),
            true,
            true,
        ),
    ];
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    CanonicalObject {
        representation_id: CanonicalRepresentationId::new(format!("activation-{}", subject.id))
            .expect("validated activation identity"),
        kind: CanonicalRepresentationKind::ActivationReference,
        representation_version: REPRESENTATION_MODEL_VERSION.into(),
        schema_id: ACTIVATION_PROFILE_ID.into(),
        source_binding: None,
        implementation_version: None,
        required_fields: fields.iter().map(|f| f.name.clone()).collect(),
        extension_namespaces: Vec::new(),
        fields,
    }
}

fn inference_code(basis: &ActivationInferenceBasis) -> ActivationFindingCode {
    match basis {
        ActivationInferenceBasis::Publication => ActivationFindingCode::InferredFromPublication,
        ActivationInferenceBasis::Deployment => ActivationFindingCode::InferredFromDeployment,
        ActivationInferenceBasis::Compatibility => ActivationFindingCode::InferredFromCompatibility,
        ActivationInferenceBasis::TestSuccess => ActivationFindingCode::InferredFromTest,
        ActivationInferenceBasis::Evidence => ActivationFindingCode::InferredFromEvidence,
        ActivationInferenceBasis::Assurance => ActivationFindingCode::InferredFromAssurance,
        ActivationInferenceBasis::ExplicitRequest => unreachable!(),
    }
}
fn push(findings: &mut Vec<ActivationFinding>, code: ActivationFindingCode, detail: &str) {
    findings.push(ActivationFinding {
        code,
        subject: "activation".into(),
        detail: detail.into(),
        fatal: true,
    });
}
fn push_result(result: &mut ActivationValidationResult, code: ActivationFindingCode, detail: &str) {
    result.findings.push(ActivationFinding {
        code,
        subject: "activation".into(),
        detail: detail.into(),
        fatal: true,
    });
    result.valid = false;
}
fn result(findings: Vec<ActivationFinding>) -> ActivationValidationResult {
    ActivationValidationResult {
        valid: findings.is_empty(),
        findings,
        execution_performed: false,
        deployment_performed: false,
        authority_created: false,
        conformance_established: false,
        certification_established: false,
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
