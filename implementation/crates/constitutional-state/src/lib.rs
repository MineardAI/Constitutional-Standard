//! Bounded, non-executing realization of CORE-004 state and transition doctrine.
//!
//! A supported transition is a determination about a proposal. It does not
//! execute, mutate state, or establish the projected state as actual.

use constitutional_contracts::{
    ConstitutionalSourceRef, ContextId, ImplementationVersion, SourceSetId,
};
use std::fmt;

pub const PROFILE_ID: &str = "reference-implementation-state-transition";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const PRIMARY_SOURCE_VERSION: &str = "0.2.0";
pub const ADDENDUM_SOURCE_VERSION: &str = "0.3.0";
pub const SOURCE_STATUS: &str = "Revised Baseline Draft";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainRef(String);
impl DomainRef {
    pub fn new(value: impl Into<String>) -> Result<Self, StateError> {
        let value = value.into();
        if value.is_empty()
            || value
                .chars()
                .any(|c| c.is_control() || c == '\n' || c == '\r')
        {
            return Err(StateError::MalformedReference(value));
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
pub type StateId = DomainRef;
pub type TransitionId = DomainRef;
pub type SubjectRef = DomainRef;
pub type StateBasisRef = DomainRef;
pub type AuthorityRef = DomainRef;
pub type ParticipationRef = DomainRef;
pub type ArtifactRef = DomainRef;
pub type EvidenceRef = DomainRef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StateCategory {
    Current,
    Prior,
    Historical,
    Proposed,
    Projected,
    Resulting,
    Terminal,
    Transitional,
    Indeterminate,
    Invalid,
    Superseded,
    Suspended,
    Inactive,
    NotApplicable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateValue {
    pub dimension: DomainRef,
    pub value: DomainRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateIdentity {
    pub id: StateId,
    pub subject: SubjectRef,
    pub family: DomainRef,
    pub dimension: DomainRef,
    pub domain: DomainRef,
    pub scope: DomainRef,
    pub context: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub version: DomainRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateClaim {
    pub identity: StateIdentity,
    pub values: Vec<StateValue>,
    pub basis: StateBasisRef,
    pub category: StateCategory,
    pub source: ConstitutionalSourceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateConstraint {
    pub id: DomainRef,
    pub satisfied: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Precondition {
    pub id: DomainRef,
    pub satisfied: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invariant {
    pub id: DomainRef,
    pub preserved: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionKind {
    Introduce,
    Replace,
    Remove,
    Activate,
    Deactivate,
    Supersede,
    Suspend,
    Restore,
    Terminate,
    NoOp,
    Compound,
    Partial,
    Other(DomainRef),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionEffect {
    Introduce(StateValue),
    Replace(StateValue),
    Remove(DomainRef),
    Activate(DomainRef),
    Deactivate(DomainRef),
    Suspend(DomainRef),
    Restore(DomainRef),
    Supersede(DomainRef),
    Terminate(DomainRef),
    NoOp,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedTransition {
    pub id: TransitionId,
    pub subject: SubjectRef,
    pub source_state: StateIdentity,
    pub target_state: StateIdentity,
    pub kind: TransitionKind,
    pub effects: Vec<TransitionEffect>,
    pub authority: Option<AuthorityRef>,
    pub participation: Option<ParticipationRef>,
    pub artifact: Option<ArtifactRef>,
    pub provenance: Option<DomainRef>,
    pub preconditions: Vec<Precondition>,
    pub invariants: Vec<Invariant>,
    pub constraints: Vec<StateConstraint>,
    pub context: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub evidence: Vec<EvidenceRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub available_sources: Vec<ConstitutionalSourceRef>,
    pub supported_kinds: Vec<TransitionKind>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEvaluationRequest {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub state: Option<StateClaim>,
    pub transition: Option<ProposedTransition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionAdmissionDetermination {
    Admitted,
    NotAdmitted,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionSupportDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreconditionDetermination {
    Satisfied,
    NotSatisfied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvariantDetermination {
    Preserved,
    Violated,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionDetermination {
    Projected,
    NotProjectable,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateFindingCode {
    MissingState,
    MissingSubject,
    MissingStateDomain,
    MissingStateBasis,
    DuplicateDimension,
    ConflictingStateValue,
    SourceUnavailable,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
    UnsupportedTransitionKind,
    MissingSourceState,
    MissingTargetState,
    NotAdmitted,
    FailedPrecondition,
    ViolatedInvariant,
    MissingAuthority,
    UnsupportedEffect,
    ProjectionIncomplete,
    HistoricalNotCurrent,
    TransitionNonExecution,
    AutoChainForbidden,
    ConflictingTransition,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateFinding {
    pub code: StateFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedState {
    pub identity: StateIdentity,
    pub values: Vec<StateValue>,
    pub projected: bool,
    pub execution_performed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEvaluationResult {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: DomainRef,
    pub state_determination: StateDetermination,
    pub admission: TransitionAdmissionDetermination,
    pub support: TransitionSupportDetermination,
    pub preconditions: PreconditionDetermination,
    pub invariants: InvariantDetermination,
    pub projection: ProjectionDetermination,
    pub state: Option<StateClaim>,
    pub transition: Option<ProposedTransition>,
    pub projected_state: Option<ProjectedState>,
    pub findings: Vec<StateFinding>,
    pub execution_performed: bool,
    pub mutation_performed: bool,
    pub non_claims: Vec<String>,
}

pub fn evaluate_state_claim(
    request: &StateEvaluationRequest,
    context: &StateEvaluationContext,
) -> StateEvaluationResult {
    let mut findings = binding_findings(request, context);
    let Some(state) = request.state.as_ref() else {
        findings.push(finding(
            StateFindingCode::MissingState,
            "state",
            "state claim is absent",
            true,
        ));
        return result(
            request,
            StateDetermination::InvalidRequest,
            TransitionAdmissionDetermination::NotApplicable,
            TransitionSupportDetermination::NotApplicable,
            PreconditionDetermination::NotApplicable,
            InvariantDetermination::NotApplicable,
            ProjectionDetermination::NotApplicable,
            None,
            findings,
        );
    };
    if state.identity.subject.as_str().is_empty() {
        findings.push(finding(
            StateFindingCode::MissingSubject,
            "subject",
            "state subject is absent",
            true,
        ));
    }
    if state.identity.domain.as_str().is_empty() {
        findings.push(finding(
            StateFindingCode::MissingStateDomain,
            "domain",
            "state domain is absent",
            true,
        ));
    }
    if state.basis.as_str().is_empty() {
        findings.push(finding(
            StateFindingCode::MissingStateBasis,
            "basis",
            "state basis is absent",
            true,
        ));
    }
    let mut dimensions = std::collections::BTreeSet::new();
    for value in &state.values {
        if !dimensions.insert(value.dimension.clone()) {
            findings.push(finding(
                StateFindingCode::DuplicateDimension,
                value.dimension.as_str(),
                "state dimension is duplicated",
                true,
            ));
        }
    }
    let determination = if findings.iter().any(|item| item.fatal) {
        StateDetermination::InvalidRequest
    } else if !context
        .available_sources
        .iter()
        .any(|source| source == &state.source)
    {
        findings.push(finding(
            StateFindingCode::SourceUnavailable,
            state.source.id.as_str(),
            "state source is unavailable",
            true,
        ));
        StateDetermination::Indeterminate
    } else {
        StateDetermination::Supported
    };
    result(
        request,
        determination,
        TransitionAdmissionDetermination::NotApplicable,
        TransitionSupportDetermination::NotApplicable,
        PreconditionDetermination::NotApplicable,
        InvariantDetermination::NotApplicable,
        ProjectionDetermination::NotApplicable,
        None,
        findings,
    )
}

pub fn evaluate_transition_admission(
    request: &StateEvaluationRequest,
    context: &StateEvaluationContext,
) -> TransitionAdmissionDetermination {
    let Some(transition) = request.transition.as_ref() else {
        return TransitionAdmissionDetermination::InvalidRequest;
    };
    if !binding_findings(request, context).is_empty() {
        return TransitionAdmissionDetermination::InvalidRequest;
    }
    if transition.source_state.id.as_str().is_empty() {
        return TransitionAdmissionDetermination::NotAdmitted;
    }
    if transition.context != context.context_id
        || transition.source_set_id != context.source_set_id
        || transition.implementation_version != context.implementation_version
    {
        return TransitionAdmissionDetermination::InvalidRequest;
    }
    if !context.supported_kinds.contains(&transition.kind) {
        return TransitionAdmissionDetermination::NotApplicable;
    }
    if transition.subject != transition.source_state.subject
        || transition.subject != transition.target_state.subject
    {
        return TransitionAdmissionDetermination::Conflict;
    }
    if transition.effects.is_empty() && !matches!(transition.kind, TransitionKind::NoOp) {
        return TransitionAdmissionDetermination::NotAdmitted;
    }
    TransitionAdmissionDetermination::Admitted
}

pub fn evaluate_preconditions(transition: &ProposedTransition) -> PreconditionDetermination {
    if transition
        .preconditions
        .iter()
        .any(|item| item.satisfied == Some(false))
    {
        PreconditionDetermination::NotSatisfied
    } else if transition
        .preconditions
        .iter()
        .any(|item| item.satisfied.is_none())
    {
        PreconditionDetermination::Indeterminate
    } else {
        PreconditionDetermination::Satisfied
    }
}
pub fn evaluate_invariant_preservation(transition: &ProposedTransition) -> InvariantDetermination {
    if transition
        .invariants
        .iter()
        .any(|item| item.preserved == Some(false))
    {
        InvariantDetermination::Violated
    } else if transition
        .invariants
        .iter()
        .any(|item| item.preserved.is_none())
    {
        InvariantDetermination::Indeterminate
    } else {
        InvariantDetermination::Preserved
    }
}
pub fn evaluate_transition_support(
    request: &StateEvaluationRequest,
    context: &StateEvaluationContext,
) -> StateEvaluationResult {
    let admission = evaluate_transition_admission(request, context);
    let Some(transition) = request.transition.as_ref() else {
        return result(
            request,
            StateDetermination::NotApplicable,
            admission,
            TransitionSupportDetermination::InvalidRequest,
            PreconditionDetermination::NotApplicable,
            InvariantDetermination::NotApplicable,
            ProjectionDetermination::NotApplicable,
            None,
            vec![finding(
                StateFindingCode::MissingTargetState,
                "transition",
                "transition proposal is absent",
                true,
            )],
        );
    };
    if admission != TransitionAdmissionDetermination::Admitted {
        return result(
            request,
            StateDetermination::NotApplicable,
            admission,
            TransitionSupportDetermination::Denied,
            PreconditionDetermination::NotApplicable,
            InvariantDetermination::NotApplicable,
            ProjectionDetermination::NotApplicable,
            None,
            vec![finding(
                StateFindingCode::NotAdmitted,
                "transition",
                "unsupported transition cannot receive supported determination",
                true,
            )],
        );
    }
    let preconditions = evaluate_preconditions(transition);
    let invariants = evaluate_invariant_preservation(transition);
    let mut findings = Vec::new();
    if preconditions == PreconditionDetermination::NotSatisfied {
        findings.push(finding(
            StateFindingCode::FailedPrecondition,
            "preconditions",
            "one or more preconditions failed",
            true,
        ));
    }
    if invariants == InvariantDetermination::Violated {
        findings.push(finding(
            StateFindingCode::ViolatedInvariant,
            "invariants",
            "one or more invariants would be violated",
            true,
        ));
    }
    if transition.authority.is_none() && !matches!(transition.kind, TransitionKind::NoOp) {
        findings.push(finding(
            StateFindingCode::MissingAuthority,
            "authority",
            "transition authority reference is absent",
            true,
        ));
    }
    let support = if findings.iter().any(|item| item.fatal) {
        TransitionSupportDetermination::Denied
    } else if preconditions == PreconditionDetermination::Indeterminate
        || invariants == InvariantDetermination::Indeterminate
    {
        TransitionSupportDetermination::Indeterminate
    } else {
        TransitionSupportDetermination::Supported
    };
    let projection = if support == TransitionSupportDetermination::Supported {
        ProjectionDetermination::Projected
    } else {
        ProjectionDetermination::NotProjectable
    };
    let projected = if projection == ProjectionDetermination::Projected {
        project_values(transition)
    } else {
        None
    };
    result(
        request,
        StateDetermination::NotApplicable,
        admission,
        support,
        preconditions,
        invariants,
        projection,
        projected,
        findings,
    )
}

pub fn project_resulting_state(
    transition: &ProposedTransition,
) -> Result<ProjectedState, ProjectionDetermination> {
    project_values(transition).ok_or(ProjectionDetermination::NotProjectable)
}
pub fn evaluate_state_continuity(
    source: &StateIdentity,
    target: &StateIdentity,
) -> StateDetermination {
    if source.subject == target.subject
        && source.family == target.family
        && source.dimension == target.dimension
    {
        StateDetermination::Supported
    } else {
        StateDetermination::Conflict
    }
}
pub fn evaluate_transition_relationship(
    from: &TransitionId,
    to: &TransitionId,
) -> Result<(), StateFinding> {
    if from == to {
        Err(finding(
            StateFindingCode::ConflictingTransition,
            "relationship",
            "transition relationship cannot self-collapse",
            true,
        ))
    } else {
        Ok(())
    }
}

fn project_values(transition: &ProposedTransition) -> Option<ProjectedState> {
    let mut values = Vec::new();
    for effect in &transition.effects {
        match effect {
            TransitionEffect::Introduce(value) | TransitionEffect::Replace(value) => {
                values.retain(|existing: &StateValue| existing.dimension != value.dimension);
                values.push(value.clone());
            }
            TransitionEffect::Remove(dimension)
            | TransitionEffect::Deactivate(dimension)
            | TransitionEffect::Suspend(dimension)
            | TransitionEffect::Terminate(dimension) => {
                values.retain(|existing| &existing.dimension != dimension)
            }
            TransitionEffect::Restore(dimension)
            | TransitionEffect::Activate(dimension)
            | TransitionEffect::Supersede(dimension) => {
                if !values.iter().any(|value| &value.dimension == dimension) {
                    values.push(StateValue {
                        dimension: dimension.clone(),
                        value: DomainRef::new("active").unwrap(),
                    });
                }
            }
            TransitionEffect::NoOp => {}
        }
    }
    Some(ProjectedState {
        identity: transition.target_state.clone(),
        values,
        projected: true,
        execution_performed: false,
    })
}
fn binding_findings(
    request: &StateEvaluationRequest,
    context: &StateEvaluationContext,
) -> Vec<StateFinding> {
    let mut findings = Vec::new();
    if request.context_id != context.context_id {
        findings.push(finding(
            StateFindingCode::ContextMismatch,
            "context_id",
            "context does not match request",
            true,
        ));
    }
    if request.source_set_id != context.source_set_id {
        findings.push(finding(
            StateFindingCode::SourceSetMismatch,
            "source_set_id",
            "source set does not match request",
            true,
        ));
    }
    if request.implementation_version != context.implementation_version {
        findings.push(finding(
            StateFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version does not match request",
            true,
        ));
    }
    findings
}
#[allow(clippy::too_many_arguments)]
fn result(
    request: &StateEvaluationRequest,
    state_determination: StateDetermination,
    admission: TransitionAdmissionDetermination,
    support: TransitionSupportDetermination,
    preconditions: PreconditionDetermination,
    invariants: InvariantDetermination,
    projection: ProjectionDetermination,
    projected_state: Option<ProjectedState>,
    findings: Vec<StateFinding>,
) -> StateEvaluationResult {
    StateEvaluationResult {
        evaluation_id: request.evaluation_id.clone(),
        operation_id: request.operation_id.clone(),
        context_id: request.context_id.clone(),
        source_set_id: request.source_set_id.clone(),
        implementation_version: request.implementation_version.clone(),
        profile_version: DomainRef::new(PROFILE_VERSION).unwrap(),
        state_determination,
        admission,
        support,
        preconditions,
        invariants,
        projection,
        state: request.state.clone(),
        transition: request.transition.clone(),
        projected_state,
        findings,
        execution_performed: false,
        mutation_performed: false,
        non_claims: vec![
            "transition support is not transition execution".into(),
            "projected resulting state is not actual resulting state".into(),
        ],
    }
}
fn finding(code: StateFindingCode, subject: &str, detail: &str, fatal: bool) -> StateFinding {
    StateFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(v: &str) -> DomainRef {
        DomainRef::new(v).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(constitutional_contracts::ConstitutionalSourceId::new("CORE-004").unwrap(), constitutional_contracts::ConstitutionalSourceVersion::new("0.2.0").unwrap(), "harmonization-v1.0/CORE-004_Constitutional_State_and_Transition_Doctrine_v0.3.0_Harmonization_Draft.md").unwrap()
    }
    fn identity(id: &str) -> StateIdentity {
        StateIdentity {
            id: r(id),
            subject: r("subject-1"),
            family: r("Participation"),
            dimension: r("Lifecycle"),
            domain: r("domain-1"),
            scope: r("scope-1"),
            context: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            version: r("state-v1"),
        }
    }
    fn context() -> StateEvaluationContext {
        StateEvaluationContext {
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.12.0")
                .unwrap(),
            available_sources: vec![source()],
            supported_kinds: vec![
                TransitionKind::Replace,
                TransitionKind::NoOp,
                TransitionKind::Suspend,
                TransitionKind::Restore,
            ],
        }
    }
    fn transition() -> ProposedTransition {
        ProposedTransition {
            id: r("transition-1"),
            subject: r("subject-1"),
            source_state: identity("state-source"),
            target_state: identity("state-target"),
            kind: TransitionKind::Replace,
            effects: vec![TransitionEffect::Replace(StateValue {
                dimension: r("Lifecycle"),
                value: r("active"),
            })],
            authority: Some(r("authority-ref")),
            participation: None,
            artifact: None,
            provenance: Some(r("provenance")),
            preconditions: vec![Precondition {
                id: r("precondition-1"),
                satisfied: Some(true),
            }],
            invariants: vec![Invariant {
                id: r("invariant-1"),
                preserved: Some(true),
            }],
            constraints: vec![],
            context: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.12.0")
                .unwrap(),
            evidence: vec![],
        }
    }
    fn request(transition: Option<ProposedTransition>) -> StateEvaluationRequest {
        StateEvaluationRequest {
            evaluation_id: r("eval-1"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.12.0")
                .unwrap(),
            state: None,
            transition,
        }
    }
    #[test]
    fn supported_transition_projects_without_execution() {
        let result = evaluate_transition_support(&request(Some(transition())), &context());
        assert_eq!(result.support, TransitionSupportDetermination::Supported);
        assert_eq!(result.projection, ProjectionDetermination::Projected);
        assert!(!result.execution_performed);
        assert!(!result.mutation_performed);
        assert!(!result.projected_state.unwrap().execution_performed);
    }
    #[test]
    fn failed_precondition_denies_support() {
        let mut proposed = transition();
        proposed.preconditions[0].satisfied = Some(false);
        assert_eq!(
            evaluate_transition_support(&request(Some(proposed)), &context()).support,
            TransitionSupportDetermination::Denied
        );
    }
    #[test]
    fn missing_authority_denies_non_noop_transition() {
        let mut proposed = transition();
        proposed.authority = None;
        assert_eq!(
            evaluate_transition_support(&request(Some(proposed)), &context()).support,
            TransitionSupportDetermination::Denied
        );
    }
    #[test]
    fn unsupported_kind_is_not_admitted() {
        let mut proposed = transition();
        proposed.kind = TransitionKind::Other(r("unsupported"));
        assert_eq!(
            evaluate_transition_admission(&request(Some(proposed)), &context()),
            TransitionAdmissionDetermination::NotApplicable
        );
    }
    #[test]
    fn state_claim_requires_basis_and_source() {
        let state = StateClaim {
            identity: identity("state-1"),
            values: vec![StateValue {
                dimension: r("Lifecycle"),
                value: r("active"),
            }],
            basis: r("basis"),
            category: StateCategory::Current,
            source: source(),
        };
        let req = StateEvaluationRequest {
            evaluation_id: r("eval"),
            operation_id: r("op"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.12.0")
                .unwrap(),
            state: Some(state),
            transition: None,
        };
        assert_eq!(
            evaluate_state_claim(&req, &context()).state_determination,
            StateDetermination::Supported
        );
    }
    #[test]
    fn projection_preserves_non_execution_boundary() {
        let projected = project_resulting_state(&transition()).unwrap();
        assert!(projected.projected);
        assert!(!projected.execution_performed);
    }
}
