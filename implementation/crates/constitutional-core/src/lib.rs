//! Deterministic, non-sovereign composition of the five bounded CORE slices.
//!
//! This crate owns integration relationships only. It does not reinterpret or
//! replace domain results, create authority, mutate state, transmit interaction,
//! establish effect, or make conformance or certification claims.

use constitutional_artifacts::ArtifactEvaluationResult;
use constitutional_authority::AuthorityEvaluationResult;
use constitutional_contracts::{
    ContextId, EvidenceRecordId, ImplementationVersion, OperationalConstitutionalContext,
    SourceSetId,
};
use constitutional_identity::{IdentityEvaluationResult, ParticipationEvaluationResult};
use constitutional_interaction::InteractionEvaluationResult;
use constitutional_state::StateEvaluationResult;

pub const PROFILE_ID: &str = "reference-implementation-core-integration";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const IMPLEMENTATION_VERSION: &str = "reference-foundation-0.15.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ref(String);
impl Ref {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreIntegrationError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(|c| c.is_control() || c.is_whitespace()) {
            return Err(CoreIntegrationError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CoreDomain {
    Authority,
    Identity,
    Participation,
    Artifacts,
    State,
    Interaction,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainResultReference {
    pub domain: CoreDomain,
    pub result_id: Ref,
    pub subject: Option<Ref>,
    pub status: Ref,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub representation_version: Ref,
    pub historical: bool,
    pub projected: bool,
    pub executed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossDomainDependency {
    pub id: Ref,
    pub upstream: CoreDomain,
    pub downstream: CoreDomain,
    pub required: bool,
    pub satisfied: Option<bool>,
    pub upstream_result: Option<Ref>,
    pub downstream_result: Option<Ref>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvariantKind {
    NonCreationOfAuthority,
    NonExpansion,
    RecognitionNotAuthority,
    ParticipationNotActivation,
    RepresentationNotReality,
    ValidationNotExecution,
    ProjectionNotActuality,
    ProvenanceNotTruth,
    HistoricalNotCurrent,
    RecordsNotOccurrence,
    Determinism,
    ExplicitUncertainty,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniversalInvariantReference {
    pub id: Ref,
    pub kind: InvariantKind,
    pub evidence: Option<EvidenceRecordId>,
    pub preserved: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssertionKind {
    AuthorityScope,
    ParticipationScope,
    InteractionScope,
    JurisdictionBoundary,
    SubjectConsistency,
    ArtifactTransitionSubject,
    StateInteractionSubject,
    SourceBinding,
    ContextBinding,
    ImplementationVersion,
    RepresentationVersion,
    HistoricalApplicability,
    ProjectionActuality,
    TransitionExecution,
    InteractionTransmission,
    ArtifactTruth,
    ProvenanceAuthority,
    HandoffDelegation,
    ParticipationActivation,
    ContainmentIdentity,
    ReplayAct,
    DownstreamScope,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossDomainAssertion {
    pub kind: AssertionKind,
    pub left: Ref,
    pub right: Ref,
    pub consistent: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreEvaluationPurpose {
    pub id: Ref,
    pub scope: Ref,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreDomainResults {
    pub references: Vec<DomainResultReference>,
    pub authority: Option<AuthorityEvaluationResult>,
    pub identity: Option<IdentityEvaluationResult>,
    pub participation: Option<ParticipationEvaluationResult>,
    pub artifacts: Option<ArtifactEvaluationResult>,
    pub state: Option<StateEvaluationResult>,
    pub interaction: Option<InteractionEvaluationResult>,
}
impl CoreDomainResults {
    pub fn empty() -> Self {
        Self {
            references: Vec::new(),
            authority: None,
            identity: None,
            participation: None,
            artifacts: None,
            state: None,
            interaction: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreEvaluationRequest {
    pub evaluation_id: Ref,
    pub operation_id: Ref,
    pub context: Option<OperationalConstitutionalContext>,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub purpose: CoreEvaluationPurpose,
    pub subjects: Vec<Ref>,
    pub authority_refs: Vec<Ref>,
    pub jurisdiction_refs: Vec<Ref>,
    pub identity_refs: Vec<Ref>,
    pub participation_refs: Vec<Ref>,
    pub artifact_refs: Vec<Ref>,
    pub provenance_refs: Vec<Ref>,
    pub state_refs: Vec<Ref>,
    pub interaction_refs: Vec<Ref>,
    pub dependencies: Vec<CrossDomainDependency>,
    pub invariants: Vec<UniversalInvariantReference>,
    pub assertions: Vec<CrossDomainAssertion>,
    pub evidence: Vec<EvidenceRecordId>,
    pub domain_results: CoreDomainResults,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegrationDetermination {
    Compatible,
    Incompatible,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DependencyDetermination {
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
pub enum BindingCompatibilityDetermination {
    Compatible,
    Incompatible,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoreFindingCode {
    MissingDomainResult,
    MissingDependency,
    DependencyConflict,
    InvariantViolation,
    InvariantUnresolved,
    BindingMismatch,
    SubjectMismatch,
    ScopeExpansion,
    HistoricalCurrentConflict,
    ProjectionActualityConflict,
    ExecutionConflation,
    AuthorityCreation,
    DomainOwnershipConflict,
    UnsupportedRepresentation,
    NonDeterministicInput,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreFinding {
    pub code: CoreFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreEvaluationResult {
    pub evaluation_id: Ref,
    pub operation_id: Ref,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: Ref,
    pub integration: IntegrationDetermination,
    pub dependencies: DependencyDetermination,
    pub invariants: InvariantDetermination,
    pub bindings: BindingCompatibilityDetermination,
    pub domain_results: CoreDomainResults,
    pub findings: Vec<CoreFinding>,
    pub canonical_identity: Ref,
    pub evidence: Vec<EvidenceRecordId>,
    pub execution_performed: bool,
    pub mutation_performed: bool,
    pub effect_established: bool,
    pub non_claims: Vec<String>,
}

fn f(code: CoreFindingCode, subject: &str, detail: &str, fatal: bool) -> CoreFinding {
    CoreFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
fn base(req: &CoreEvaluationRequest, mut findings: Vec<CoreFinding>) -> CoreEvaluationResult {
    findings.sort_by(|a, b| {
        a.subject
            .cmp(&b.subject)
            .then_with(|| format!("{:?}", a.code).cmp(&format!("{:?}", b.code)))
    });
    CoreEvaluationResult{evaluation_id:req.evaluation_id.clone(),operation_id:req.operation_id.clone(),context_id:req.context_id.clone(),source_set_id:req.source_set_id.clone(),implementation_version:req.implementation_version.clone(),profile_version:Ref::new(PROFILE_VERSION).unwrap(),integration:IntegrationDetermination::NotApplicable,dependencies:DependencyDetermination::NotApplicable,invariants:InvariantDetermination::NotApplicable,bindings:BindingCompatibilityDetermination::NotApplicable,domain_results:req.domain_results.clone(),findings,canonical_identity:integration_identity(req),evidence:req.evidence.clone(),execution_performed:false,mutation_performed:false,effect_established:false,non_claims:vec!["integration does not replace domain results".into(),"compatible composition is not authority, execution, effect, conformance, or certification".into()]}
}
fn integration_identity(req: &CoreEvaluationRequest) -> Ref {
    let mut domains = req
        .domain_results
        .references
        .iter()
        .map(|r| format!("{:?}:{}", r.domain, r.result_id))
        .collect::<Vec<_>>();
    domains.sort();
    Ref::new(format!(
        "{}|{}|{}|{}|{}",
        req.evaluation_id,
        req.context_id,
        req.source_set_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        req.implementation_version,
        domains.join(",")
    ))
    .unwrap()
}
fn bindings(req: &CoreEvaluationRequest) -> (BindingCompatibilityDetermination, Vec<CoreFinding>) {
    let mut out = Vec::new();
    for r in &req.domain_results.references {
        if r.context_id != req.context_id
            || r.source_set_id != req.source_set_id
            || r.implementation_version != req.implementation_version
        {
            out.push(f(
                CoreFindingCode::BindingMismatch,
                &r.result_id.to_string(),
                "domain result binding is incompatible",
                true,
            ));
        }
        if r.representation_version.as_str() != "1.0.0" {
            out.push(f(
                CoreFindingCode::UnsupportedRepresentation,
                &r.result_id.to_string(),
                "representation version is unsupported",
                true,
            ));
        }
    }
    let d = if out.iter().any(|x| x.fatal) {
        BindingCompatibilityDetermination::Incompatible
    } else {
        BindingCompatibilityDetermination::Compatible
    };
    (d, out)
}
pub fn evaluate_domain_dependencies(
    req: &CoreEvaluationRequest,
) -> (DependencyDetermination, Vec<CoreFinding>) {
    let mut out = Vec::new();
    for d in &req.dependencies {
        match d.satisfied {
            Some(true) => {}
            Some(false) => out.push(f(
                CoreFindingCode::DependencyConflict,
                &d.id.to_string(),
                "declared dependency is not satisfied",
                true,
            )),
            None => out.push(f(
                CoreFindingCode::MissingDependency,
                &d.id.to_string(),
                "dependency status is unresolved",
                true,
            )),
        }
        if d.required && d.upstream_result.is_none() {
            out.push(f(
                CoreFindingCode::MissingDomainResult,
                &d.id.to_string(),
                "required upstream result is absent",
                true,
            ));
        }
    }
    let d = if out.iter().any(|x| x.fatal) {
        if out.iter().any(|x| {
            matches!(
                x.code,
                CoreFindingCode::MissingDependency | CoreFindingCode::MissingDomainResult
            )
        }) {
            DependencyDetermination::Indeterminate
        } else {
            DependencyDetermination::NotSatisfied
        }
    } else {
        DependencyDetermination::Satisfied
    };
    (d, out)
}
pub fn evaluate_universal_invariants(
    req: &CoreEvaluationRequest,
) -> (InvariantDetermination, Vec<CoreFinding>) {
    let mut out = Vec::new();
    for i in &req.invariants {
        match i.preserved {
            Some(true) => {}
            Some(false) => out.push(f(
                CoreFindingCode::InvariantViolation,
                &i.id.to_string(),
                "universal invariant is violated",
                true,
            )),
            None => out.push(f(
                CoreFindingCode::InvariantUnresolved,
                &i.id.to_string(),
                "invariant evidence is unresolved",
                true,
            )),
        }
    }
    let d = if out
        .iter()
        .any(|x| matches!(x.code, CoreFindingCode::InvariantViolation))
    {
        InvariantDetermination::Violated
    } else if out.iter().any(|x| x.fatal) {
        InvariantDetermination::Indeterminate
    } else {
        InvariantDetermination::Preserved
    };
    (d, out)
}
pub fn evaluate_binding_compatibility(
    req: &CoreEvaluationRequest,
) -> (BindingCompatibilityDetermination, Vec<CoreFinding>) {
    bindings(req)
}
pub fn evaluate_cross_domain_scope(req: &CoreEvaluationRequest) -> Vec<CoreFinding> {
    req.assertions
        .iter()
        .filter(|a| {
            matches!(
                a.kind,
                AssertionKind::AuthorityScope
                    | AssertionKind::ParticipationScope
                    | AssertionKind::InteractionScope
                    | AssertionKind::DownstreamScope
            ) && a.consistent != Some(true)
        })
        .map(|a| {
            f(
                CoreFindingCode::ScopeExpansion,
                &a.left.to_string(),
                "cross-domain scope containment is not established",
                a.consistent == Some(false),
            )
        })
        .collect()
}
pub fn evaluate_cross_domain_identity_consistency(req: &CoreEvaluationRequest) -> Vec<CoreFinding> {
    req.assertions
        .iter()
        .filter(|a| {
            matches!(
                a.kind,
                AssertionKind::SubjectConsistency
                    | AssertionKind::ArtifactTransitionSubject
                    | AssertionKind::StateInteractionSubject
            ) && a.consistent != Some(true)
        })
        .map(|a| {
            f(
                CoreFindingCode::SubjectMismatch,
                &a.left.to_string(),
                "cross-domain subject mapping is not consistent",
                a.consistent == Some(false),
            )
        })
        .collect()
}
pub fn evaluate_historical_composition(req: &CoreEvaluationRequest) -> Vec<CoreFinding> {
    req.assertions
        .iter()
        .filter(|a| {
            matches!(a.kind, AssertionKind::HistoricalApplicability) && a.consistent != Some(true)
        })
        .map(|a| {
            f(
                CoreFindingCode::HistoricalCurrentConflict,
                &a.left.to_string(),
                "historical reference is not current applicability",
                a.consistent == Some(false),
            )
        })
        .collect()
}
pub fn evaluate_core_composition(req: &CoreEvaluationRequest) -> CoreEvaluationResult {
    let mut findings = Vec::new();
    let (binding, mut b) = evaluate_binding_compatibility(req);
    findings.append(&mut b);
    let (dep, mut d) = evaluate_domain_dependencies(req);
    findings.append(&mut d);
    let (inv, mut i) = evaluate_universal_invariants(req);
    findings.append(&mut i);
    findings.extend(evaluate_cross_domain_scope(req));
    findings.extend(evaluate_cross_domain_identity_consistency(req));
    findings.extend(evaluate_historical_composition(req));
    for a in &req.assertions {
        if a.consistent == Some(false) {
            match a.kind {
                AssertionKind::TransitionExecution
                | AssertionKind::InteractionTransmission
                | AssertionKind::ProjectionActuality => findings.push(f(
                    CoreFindingCode::ExecutionConflation,
                    &a.left.to_string(),
                    "domain result was treated as executed or actual",
                    true,
                )),
                AssertionKind::ArtifactTruth
                | AssertionKind::ProvenanceAuthority
                | AssertionKind::HandoffDelegation
                | AssertionKind::ParticipationActivation => findings.push(f(
                    CoreFindingCode::AuthorityCreation,
                    &a.left.to_string(),
                    "cross-domain composition cannot create authority or activation",
                    true,
                )),
                _ => {}
            }
        }
    }
    let mut r = base(req, findings);
    r.bindings = binding;
    r.dependencies = dep;
    r.invariants = inv;
    r.integration = if r.dependencies == DependencyDetermination::Indeterminate
        || r.invariants == InvariantDetermination::Indeterminate
    {
        IntegrationDetermination::Indeterminate
    } else if r.findings.iter().any(|x| x.fatal) {
        if r.findings.iter().any(|x| {
            matches!(
                x.code,
                CoreFindingCode::BindingMismatch
                    | CoreFindingCode::DependencyConflict
                    | CoreFindingCode::InvariantViolation
                    | CoreFindingCode::SubjectMismatch
                    | CoreFindingCode::AuthorityCreation
                    | CoreFindingCode::ExecutionConflation
            )
        }) {
            IntegrationDetermination::Conflict
        } else {
            IntegrationDetermination::Incompatible
        }
    } else {
        IntegrationDetermination::Compatible
    };
    r
}
pub fn evaluate_core_composition_with_domain_results(
    req: &CoreEvaluationRequest,
) -> CoreEvaluationResult {
    evaluate_core_composition(req)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoreIntegrationError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(v: &str) -> Ref {
        Ref::new(v).unwrap()
    }
    fn req() -> CoreEvaluationRequest {
        CoreEvaluationRequest {
            evaluation_id: r("eval-1"),
            operation_id: r("op-1"),
            context: None,
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("set-1").unwrap()),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            purpose: CoreEvaluationPurpose {
                id: r("closure"),
                scope: r("core-five"),
            },
            subjects: vec![r("subject-1")],
            authority_refs: vec![],
            jurisdiction_refs: vec![],
            identity_refs: vec![],
            participation_refs: vec![],
            artifact_refs: vec![],
            provenance_refs: vec![],
            state_refs: vec![],
            interaction_refs: vec![],
            dependencies: vec![],
            invariants: vec![UniversalInvariantReference {
                id: r("INV-CORE-001"),
                kind: InvariantKind::NonCreationOfAuthority,
                evidence: None,
                preserved: Some(true),
            }],
            assertions: vec![],
            evidence: vec![],
            domain_results: CoreDomainResults {
                references: vec![],
                ..CoreDomainResults::empty()
            },
        }
    }
    #[test]
    fn compatible_empty_bounded_composition_is_not_execution() {
        let x = evaluate_core_composition(&req());
        assert_eq!(x.integration, IntegrationDetermination::Compatible);
        assert!(!x.execution_performed && !x.effect_established)
    }
    #[test]
    fn unresolved_dependency_is_not_silent_success() {
        let mut q = req();
        q.dependencies.push(CrossDomainDependency {
            id: r("dep-1"),
            upstream: CoreDomain::Authority,
            downstream: CoreDomain::Participation,
            required: true,
            satisfied: None,
            upstream_result: None,
            downstream_result: None,
        });
        let x = evaluate_core_composition(&q);
        assert_eq!(x.integration, IntegrationDetermination::Indeterminate)
    }
    #[test]
    fn authority_creation_and_execution_conflation_conflict() {
        let mut q = req();
        q.assertions = vec![
            CrossDomainAssertion {
                kind: AssertionKind::ArtifactTruth,
                left: r("artifact"),
                right: r("truth"),
                consistent: Some(false),
            },
            CrossDomainAssertion {
                kind: AssertionKind::TransitionExecution,
                left: r("transition"),
                right: r("executed"),
                consistent: Some(false),
            },
        ];
        let x = evaluate_core_composition(&q);
        assert_eq!(x.integration, IntegrationDetermination::Conflict)
    }
    #[test]
    fn integration_identity_is_stable() {
        let q = req();
        assert_eq!(
            evaluate_core_composition(&q).canonical_identity,
            evaluate_core_composition(&q).canonical_identity
        )
    }
    #[test]
    fn incompatible_domain_binding_is_explicit() {
        let mut q = req();
        q.domain_results.references.push(DomainResultReference {
            domain: CoreDomain::Authority,
            result_id: r("authority-result"),
            subject: Some(r("subject-1")),
            status: r("Supported"),
            context_id: ContextId::new("other-context").unwrap(),
            source_set_id: q.source_set_id.clone(),
            implementation_version: q.implementation_version.clone(),
            representation_version: r("1.0.0"),
            historical: false,
            projected: false,
            executed: false,
        });
        let result = evaluate_core_composition(&q);
        assert_eq!(
            result.bindings,
            BindingCompatibilityDetermination::Incompatible
        );
        assert_eq!(result.integration, IntegrationDetermination::Conflict);
    }
    #[test]
    fn scope_and_subject_conflicts_are_not_repaired() {
        let mut q = req();
        q.assertions = vec![
            CrossDomainAssertion {
                kind: AssertionKind::ParticipationScope,
                left: r("participation"),
                right: r("authority"),
                consistent: Some(false),
            },
            CrossDomainAssertion {
                kind: AssertionKind::SubjectConsistency,
                left: r("identity-subject"),
                right: r("state-subject"),
                consistent: Some(false),
            },
        ];
        let result = evaluate_core_composition(&q);
        assert_eq!(result.integration, IntegrationDetermination::Conflict);
        assert!(
            result
                .findings
                .iter()
                .any(|finding| finding.code == CoreFindingCode::ScopeExpansion)
        );
        assert!(
            result
                .findings
                .iter()
                .any(|finding| finding.code == CoreFindingCode::SubjectMismatch)
        );
    }
    #[test]
    fn historical_and_projection_distinctions_are_preserved() {
        let mut q = req();
        q.assertions = vec![
            CrossDomainAssertion {
                kind: AssertionKind::HistoricalApplicability,
                left: r("historical"),
                right: r("current"),
                consistent: Some(false),
            },
            CrossDomainAssertion {
                kind: AssertionKind::ProjectionActuality,
                left: r("projected-state"),
                right: r("actual-state"),
                consistent: Some(false),
            },
        ];
        let result = evaluate_core_composition(&q);
        assert_eq!(result.integration, IntegrationDetermination::Conflict);
        assert!(!result.effect_established && !result.execution_performed);
    }
}
