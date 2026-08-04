//! Bounded, non-executing realization of CORE-005 interaction and boundary doctrine.
//!
//! These types describe proposed or represented constitutional relationships. They
//! never transmit content, cross a boundary, execute a handoff, mutate state, or
//! establish occurrence, receipt, acceptance, agreement, or constitutional effect.

use constitutional_contracts::{
    ConstitutionalSourceRef, ContextId, ImplementationVersion, SourceSetId,
};
use std::fmt;

pub const PROFILE_ID: &str = "reference-implementation-interaction-boundary";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const PRIMARY_SOURCE_VERSION: &str = "0.2.0";
pub const ADDENDUM_SOURCE_VERSION: &str = "0.3.0";
pub const SOURCE_STATUS: &str = "Revised Baseline Draft";
pub const IMPLEMENTATION_VERSION: &str = "reference-foundation-0.15.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ref(String);
impl Ref {
    pub fn new(value: impl Into<String>) -> Result<Self, InteractionError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(|c| c.is_control() || c.is_whitespace()) {
            return Err(InteractionError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
pub type InteractionId = Ref;
pub type BoundaryId = Ref;
pub type SubjectRef = Ref;
pub type ParticipantRef = Ref;
pub type AuthorityRef = Ref;
pub type JurisdictionRef = Ref;
pub type ArtifactRef = Ref;
pub type ProvenanceRef = Ref;
pub type ContentRef = Ref;
pub type EvidenceRef = Ref;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum InteractionKind {
    Request,
    Response,
    Notification,
    Declaration,
    Submission,
    AdmissionRequest,
    Refusal,
    Acknowledgment,
    AcceptanceClaim,
    Reference,
    Handoff,
    Exchange,
    Consultation,
    Observation,
    Publication,
    Retrieval,
    BoundaryCrossingProposal,
    Compound,
    Historical,
    Projected,
    NoOp,
    Other(Ref),
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BoundaryKind {
    Authority,
    Jurisdiction,
    Identity,
    Participation,
    ArtifactScope,
    StateDomain,
    InteractionDomain,
    Organizational,
    Implementation,
    Operational,
    Trust,
    Information,
    Applicability,
    Other(Ref),
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RoleKind {
    Initiator,
    Recipient,
    Sender,
    Receiver,
    Requester,
    Responder,
    Source,
    Destination,
    Intermediary,
    BoundaryOwner,
    Custodian,
    Observer,
    AffectedSubject,
    RepresentedSubject,
    AuthorizingAuthority,
    ReferencedNonParticipant,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionIdentity {
    pub id: InteractionId,
    pub subject: SubjectRef,
    pub kind: InteractionKind,
    pub purpose: Ref,
    pub scope: Ref,
    pub context: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionClaim {
    pub identity: InteractionIdentity,
    pub basis: Ref,
    pub source: ConstitutionalSourceRef,
    pub content: Option<ContentRef>,
    pub artifact: Option<ArtifactRef>,
    pub provenance: Option<ProvenanceRef>,
    pub evidence: Vec<EvidenceRef>,
    pub historical: bool,
    pub projected: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionConstraint {
    pub id: Ref,
    pub satisfied: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionDependency {
    pub id: Ref,
    pub resolved: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTemporalPosition {
    pub value: Ref,
    pub historical: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantRole {
    pub participant: ParticipantRef,
    pub role: RoleKind,
    pub subject: Option<SubjectRef>,
    pub participation: Option<Ref>,
    pub authority: Option<AuthorityRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstitutionalBoundary {
    pub identity: BoundaryId,
    pub kind: BoundaryKind,
    pub domain: Ref,
    pub scope: Ref,
    pub subject: Option<SubjectRef>,
    pub source_side: Ref,
    pub destination_side: Ref,
    pub condition: Option<Ref>,
    pub constraint: Option<Ref>,
    pub admission_rule: Option<Ref>,
    pub exclusion: Option<Ref>,
    pub authority: Option<AuthorityRef>,
    pub jurisdiction: Option<JurisdictionRef>,
    pub provenance: Option<ProvenanceRef>,
    pub context: ContextId,
    pub source: ConstitutionalSourceRef,
    pub version: ImplementationVersion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedBoundaryCrossing {
    pub id: Ref,
    pub interaction: InteractionId,
    pub boundary: BoundaryId,
    pub source_side: Ref,
    pub destination_side: Ref,
    pub initiator: ParticipantRef,
    pub recipient: ParticipantRef,
    pub subject: SubjectRef,
    pub content: Option<ContentRef>,
    pub kind: InteractionKind,
    pub purpose: Ref,
    pub scope: Ref,
    pub authority: Option<AuthorityRef>,
    pub jurisdiction: Option<JurisdictionRef>,
    pub participation: Vec<Ref>,
    pub artifacts: Vec<ArtifactRef>,
    pub provenance: Vec<ProvenanceRef>,
    pub state: Vec<Ref>,
    pub preconditions: Vec<Ref>,
    pub constraints: Vec<InteractionConstraint>,
    pub exclusions: Vec<Ref>,
    pub handoff: Option<Ref>,
    pub propagation: Option<Ref>,
    pub context: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub evidence: Vec<EvidenceRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HandoffSubject {
    Representation,
    Custody,
    ProcessingResponsibility,
    ReviewResponsibility,
    InteractionResponsibility,
    EvidenceResponsibility,
    StateEvaluationResponsibility,
    ArtifactReference,
    BoundedTask,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffClaim {
    pub id: Ref,
    pub interaction: InteractionId,
    pub from: ParticipantRef,
    pub to: ParticipantRef,
    pub subject: HandoffSubject,
    pub authority_transfer_claimed: bool,
    pub scope: Ref,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseKind {
    Response,
    Acknowledgment,
    AcceptanceClaim,
    Refusal,
    NonResponse,
    Deferred,
    Conditional,
    Unsupported,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseClaim {
    pub id: Ref,
    pub interaction: InteractionId,
    pub kind: ResponseKind,
    pub responder: ParticipantRef,
    pub basis: Option<Ref>,
    pub condition: Option<Ref>,
    pub received_claimed: bool,
    pub delivered_claimed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropagationClaim {
    pub id: Ref,
    pub upstream: InteractionId,
    pub downstream: InteractionId,
    pub scope: Ref,
    pub limit: Option<Ref>,
    pub lineage: Vec<InteractionId>,
    pub prohibited: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainmentClaim {
    pub id: Ref,
    pub container: InteractionId,
    pub components: Vec<InteractionId>,
    pub scope: Ref,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationshipKind {
    Handoff,
    Response,
    Propagation,
    Containment,
    Historical,
    BoundaryCrossing,
    Reference,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionRelationship {
    pub id: Ref,
    pub from: InteractionId,
    pub to: InteractionId,
    pub kind: RelationshipKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub sources: Vec<ConstitutionalSourceRef>,
    pub boundaries: Vec<BoundaryId>,
    pub supported_kinds: Vec<InteractionKind>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionEvaluationRequest {
    pub evaluation_id: Ref,
    pub operation_id: Ref,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub interaction: Option<InteractionClaim>,
    pub participants: Vec<ParticipantRole>,
    pub boundary: Option<ConstitutionalBoundary>,
    pub crossing: Option<ProposedBoundaryCrossing>,
    pub handoff: Option<HandoffClaim>,
    pub response: Option<ResponseClaim>,
    pub propagation: Option<PropagationClaim>,
    pub containment: Option<ContainmentClaim>,
    pub relationships: Vec<InteractionRelationship>,
}

macro_rules! determination { ($name:ident { $($v:ident),+ $(,)? }) => { #[derive(Clone, Debug, Eq, PartialEq)] pub enum $name { $($v),+ } }; }
determination!(InteractionRecognitionDetermination {
    Recognized,
    NotRecognized,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(InteractionAdmissionDetermination {
    Admitted,
    NotAdmitted,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(InteractionSupportDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(BoundaryRecognitionDetermination {
    Recognized,
    NotRecognized,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(BoundaryCrossingDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(HandoffDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(PropagationDetermination {
    WithinScope,
    OutsideScope,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(ContainmentDetermination {
    Contained,
    NotContained,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(ResponseStatusDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});
determination!(ProjectionDetermination {
    Projected,
    NotProjectable,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InteractionFindingCode {
    MissingInteraction,
    MissingSubject,
    MissingKind,
    MissingBasis,
    MissingParticipant,
    ConflictingRole,
    UnsupportedKind,
    SourceUnavailable,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
    BoundaryNotRecognized,
    NotAdmitted,
    MissingAuthority,
    MissingJurisdiction,
    ScopeOutside,
    BoundarySideMismatch,
    HandoffAuthorityTransfer,
    ResponseAcceptanceEquivalence,
    ReceiptDeliveryEquivalence,
    PropagationExpansion,
    ContainmentIdentityMerge,
    HistoricalCurrentEquivalence,
    AutomaticChaining,
    IncompleteBasis,
    ConflictingSource,
    NonExecution,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionFinding {
    pub code: InteractionFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedInteractionResult {
    pub interaction: InteractionId,
    pub receiving_scope: Option<Ref>,
    pub response_required: bool,
    pub projected_boundary_status: Option<Ref>,
    pub projected: bool,
    pub non_executed: bool,
    pub non_transmitted: bool,
    pub non_received: bool,
    pub non_accepted: bool,
    pub non_effective: bool,
    pub context_bound: bool,
    pub source_set_bound: bool,
    pub implementation_version_bound: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionEvaluationResult {
    pub evaluation_id: Ref,
    pub operation_id: Ref,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: Ref,
    pub recognition: InteractionRecognitionDetermination,
    pub admission: InteractionAdmissionDetermination,
    pub support: InteractionSupportDetermination,
    pub boundary_recognition: BoundaryRecognitionDetermination,
    pub crossing: BoundaryCrossingDetermination,
    pub handoff: HandoffDetermination,
    pub propagation: PropagationDetermination,
    pub containment: ContainmentDetermination,
    pub response: ResponseStatusDetermination,
    pub projection: ProjectionDetermination,
    pub interaction: Option<InteractionClaim>,
    pub boundary: Option<ConstitutionalBoundary>,
    pub projected: Option<ProjectedInteractionResult>,
    pub findings: Vec<InteractionFinding>,
    pub execution_performed: bool,
    pub transmission_performed: bool,
    pub receipt_established: bool,
    pub effect_established: bool,
    pub non_claims: Vec<String>,
}

fn finding(
    code: InteractionFindingCode,
    subject: &str,
    detail: &str,
    fatal: bool,
) -> InteractionFinding {
    InteractionFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
fn bindings(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> Vec<InteractionFinding> {
    let mut f = Vec::new();
    if req.context_id != ctx.context_id {
        f.push(finding(
            InteractionFindingCode::ContextMismatch,
            "context_id",
            "context mismatch",
            true,
        ));
    }
    if req.source_set_id != ctx.source_set_id {
        f.push(finding(
            InteractionFindingCode::SourceSetMismatch,
            "source_set_id",
            "source set mismatch",
            true,
        ));
    }
    if req.implementation_version != ctx.implementation_version {
        f.push(finding(
            InteractionFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version mismatch",
            true,
        ));
    }
    f
}
fn base(
    req: &InteractionEvaluationRequest,
    mut f: Vec<InteractionFinding>,
) -> InteractionEvaluationResult {
    f.push(finding(
        InteractionFindingCode::NonExecution,
        "execution",
        "evaluation performs no interaction, transmission, receipt, or effect",
        false,
    ));
    InteractionEvaluationResult {
        evaluation_id: req.evaluation_id.clone(),
        operation_id: req.operation_id.clone(),
        context_id: req.context_id.clone(),
        source_set_id: req.source_set_id.clone(),
        implementation_version: req.implementation_version.clone(),
        profile_version: Ref::new(PROFILE_VERSION).unwrap(),
        recognition: InteractionRecognitionDetermination::NotApplicable,
        admission: InteractionAdmissionDetermination::NotApplicable,
        support: InteractionSupportDetermination::NotApplicable,
        boundary_recognition: BoundaryRecognitionDetermination::NotApplicable,
        crossing: BoundaryCrossingDetermination::NotApplicable,
        handoff: HandoffDetermination::NotApplicable,
        propagation: PropagationDetermination::NotApplicable,
        containment: ContainmentDetermination::NotApplicable,
        response: ResponseStatusDetermination::NotApplicable,
        projection: ProjectionDetermination::NotApplicable,
        interaction: req.interaction.clone(),
        boundary: req.boundary.clone(),
        projected: None,
        findings: f,
        execution_performed: false,
        transmission_performed: false,
        receipt_established: false,
        effect_established: false,
        non_claims: vec![
            "support is not execution".into(),
            "boundary recognition is not boundary crossing".into(),
            "response is not acceptance or agreement".into(),
            "handoff is not authority transfer".into(),
            "projection is not actual result".into(),
        ],
    }
}
fn valid_binding(req: &InteractionEvaluationRequest, ctx: &InteractionEvaluationContext) -> bool {
    bindings(req, ctx).iter().all(|f| !f.fatal)
}

pub fn evaluate_interaction_recognition(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = base(req, bindings(req, ctx));
    let Some(i) = req.interaction.as_ref() else {
        r.recognition = InteractionRecognitionDetermination::InvalidRequest;
        r.findings.push(finding(
            InteractionFindingCode::MissingInteraction,
            "interaction",
            "interaction claim absent",
            true,
        ));
        return r;
    };
    if i.identity.subject.as_str().is_empty() {
        r.findings.push(finding(
            InteractionFindingCode::MissingSubject,
            "subject",
            "subject absent",
            true,
        ));
    }
    if i.basis.as_str().is_empty() {
        r.findings.push(finding(
            InteractionFindingCode::MissingBasis,
            "basis",
            "basis absent",
            true,
        ));
    }
    if !ctx.sources.contains(&i.source) {
        r.findings.push(finding(
            InteractionFindingCode::SourceUnavailable,
            "source",
            "source not available",
            true,
        ));
    }
    r.recognition = if r.findings.iter().any(|f| f.fatal) {
        InteractionRecognitionDetermination::Indeterminate
    } else {
        InteractionRecognitionDetermination::Recognized
    };
    r
}
pub fn evaluate_interaction_admission(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_recognition(req, ctx);
    let Some(i) = req.interaction.as_ref() else {
        r.admission = InteractionAdmissionDetermination::InvalidRequest;
        return r;
    };
    if !valid_binding(req, ctx) {
        r.admission = InteractionAdmissionDetermination::InvalidRequest;
        return r;
    }
    if !ctx.supported_kinds.contains(&i.identity.kind) {
        r.admission = InteractionAdmissionDetermination::NotApplicable;
        r.findings.push(finding(
            InteractionFindingCode::UnsupportedKind,
            "kind",
            "interaction kind is outside profile",
            true,
        ));
        return r;
    }
    if req
        .participants
        .iter()
        .filter(|p| matches!(p.role, RoleKind::Initiator))
        .count()
        > 1
    {
        r.admission = InteractionAdmissionDetermination::Conflict;
        r.findings.push(finding(
            InteractionFindingCode::ConflictingRole,
            "participants",
            "multiple initiators conflict",
            true,
        ));
        return r;
    }
    r.admission = if r.recognition == InteractionRecognitionDetermination::Recognized {
        InteractionAdmissionDetermination::Admitted
    } else {
        InteractionAdmissionDetermination::NotAdmitted
    };
    r
}
pub fn evaluate_interaction_support(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_admission(req, ctx);
    if r.admission != InteractionAdmissionDetermination::Admitted {
        r.support = InteractionSupportDetermination::Denied;
        r.findings.push(finding(
            InteractionFindingCode::NotAdmitted,
            "interaction",
            "support requires admission",
            true,
        ));
        return r;
    }
    if req
        .interaction
        .as_ref()
        .and_then(|i| i.identity.scope.as_str().is_empty().then_some(()))
        .is_some()
    {
        r.support = InteractionSupportDetermination::Indeterminate;
        return r;
    }
    r.support = if req.participants.is_empty() {
        r.findings.push(finding(
            InteractionFindingCode::MissingParticipant,
            "participants",
            "participant roles absent",
            true,
        ));
        InteractionSupportDetermination::Denied
    } else {
        InteractionSupportDetermination::Supported
    };
    r
}
pub fn evaluate_boundary_recognition(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = base(req, bindings(req, ctx));
    let Some(b) = req.boundary.as_ref() else {
        r.boundary_recognition = BoundaryRecognitionDetermination::InvalidRequest;
        r.findings.push(finding(
            InteractionFindingCode::BoundaryNotRecognized,
            "boundary",
            "boundary absent",
            true,
        ));
        return r;
    };
    if !ctx.boundaries.contains(&b.identity) {
        r.boundary_recognition = BoundaryRecognitionDetermination::NotRecognized;
        r.findings.push(finding(
            InteractionFindingCode::BoundaryNotRecognized,
            "boundary",
            "boundary lacks admitted constitutional basis",
            true,
        ));
    } else if !valid_binding(req, ctx) {
        r.boundary_recognition = BoundaryRecognitionDetermination::InvalidRequest
    } else {
        r.boundary_recognition = BoundaryRecognitionDetermination::Recognized
    }
    r
}
pub fn evaluate_boundary_crossing(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_boundary_recognition(req, ctx);
    let Some(c) = req.crossing.as_ref() else {
        r.crossing = BoundaryCrossingDetermination::InvalidRequest;
        return r;
    };
    if r.boundary_recognition != BoundaryRecognitionDetermination::Recognized {
        r.crossing = BoundaryCrossingDetermination::Denied;
        r.findings.push(finding(
            InteractionFindingCode::BoundaryNotRecognized,
            "crossing",
            "crossing requires recognized boundary",
            true,
        ));
    } else if c.source_side == c.destination_side {
        r.crossing = BoundaryCrossingDetermination::Conflict;
        r.findings.push(finding(
            InteractionFindingCode::BoundarySideMismatch,
            "crossing",
            "source and destination sides collapse",
            true,
        ));
    } else if c.authority.is_none() && !matches!(c.kind, InteractionKind::NoOp) {
        r.crossing = BoundaryCrossingDetermination::Indeterminate;
        r.findings.push(finding(
            InteractionFindingCode::MissingAuthority,
            "crossing",
            "authority reference is unresolved",
            true,
        ));
    } else {
        r.crossing = BoundaryCrossingDetermination::Supported;
    }
    r
}
pub fn evaluate_handoff(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    let Some(h) = req.handoff.as_ref() else {
        r.handoff = HandoffDetermination::InvalidRequest;
        return r;
    };
    if h.authority_transfer_claimed {
        r.handoff = HandoffDetermination::Denied;
        r.findings.push(finding(
            InteractionFindingCode::HandoffAuthorityTransfer,
            "handoff",
            "handoff does not transfer authority",
            true,
        ));
    } else {
        r.handoff = HandoffDetermination::Supported
    }
    r
}
pub fn evaluate_response_claim(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    let Some(c) = req.response.as_ref() else {
        r.response = ResponseStatusDetermination::InvalidRequest;
        return r;
    };
    if c.received_claimed && c.delivered_claimed {
        r.findings.push(finding(
            InteractionFindingCode::ReceiptDeliveryEquivalence,
            "response",
            "claims are represented, not established facts",
            false,
        ));
    }
    r.response = if r.support == InteractionSupportDetermination::Supported {
        ResponseStatusDetermination::Supported
    } else {
        ResponseStatusDetermination::Denied
    };
    r
}
pub fn evaluate_propagation_scope(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    let Some(p) = req.propagation.as_ref() else {
        r.propagation = PropagationDetermination::InvalidRequest;
        return r;
    };
    if p.prohibited {
        r.propagation = PropagationDetermination::Denied
    } else if p.lineage.contains(&p.downstream) || p.upstream == p.downstream {
        r.propagation = PropagationDetermination::Conflict;
        r.findings.push(finding(
            InteractionFindingCode::PropagationExpansion,
            "propagation",
            "lineage cannot self-collapse",
            true,
        ));
    } else {
        r.propagation = PropagationDetermination::WithinScope
    }
    r
}
pub fn evaluate_interaction_containment(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    let Some(c) = req.containment.as_ref() else {
        r.containment = ContainmentDetermination::InvalidRequest;
        return r;
    };
    if c.components.contains(&c.container) || c.components.windows(2).any(|w| w[0] == w[1]) {
        r.containment = ContainmentDetermination::Conflict;
        r.findings.push(finding(
            InteractionFindingCode::ContainmentIdentityMerge,
            "containment",
            "component identities must remain distinct",
            true,
        ));
    } else {
        r.containment = ContainmentDetermination::Contained
    }
    r
}
pub fn project_interaction_result(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    if r.support != InteractionSupportDetermination::Supported {
        r.projection = ProjectionDetermination::NotProjectable;
        return r;
    }
    let Some(i) = req.interaction.as_ref() else {
        r.projection = ProjectionDetermination::InvalidRequest;
        return r;
    };
    r.projected = Some(ProjectedInteractionResult {
        interaction: i.identity.id.clone(),
        receiving_scope: Some(i.identity.scope.clone()),
        response_required: matches!(
            i.identity.kind,
            InteractionKind::Request | InteractionKind::Submission
        ),
        projected_boundary_status: None,
        projected: true,
        non_executed: true,
        non_transmitted: true,
        non_received: true,
        non_accepted: true,
        non_effective: true,
        context_bound: true,
        source_set_bound: req.source_set_id.is_some(),
        implementation_version_bound: req.implementation_version == ctx.implementation_version,
    });
    r.projection = ProjectionDetermination::Projected;
    r
}
pub fn evaluate_interaction_relationship(
    req: &InteractionEvaluationRequest,
    ctx: &InteractionEvaluationContext,
) -> InteractionEvaluationResult {
    let mut r = evaluate_interaction_support(req, ctx);
    if req.relationships.iter().any(|x| x.from == x.to) {
        r.findings.push(finding(
            InteractionFindingCode::AutomaticChaining,
            "relationship",
            "relationship cannot self-collapse",
            true,
        ));
        r.support = InteractionSupportDetermination::Conflict;
    }
    r
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InteractionError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(v: &str) -> Ref {
        Ref::new(v).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(constitutional_contracts::ConstitutionalSourceId::new("CORE-005").unwrap(),constitutional_contracts::ConstitutionalSourceVersion::new("0.2.0").unwrap(),"harmonization-v1.0/CORE-005_Constitutional_Interaction_and_Boundary_Doctrine_v0.3.0_Harmonization_Draft.md").unwrap()
    }
    fn ctx() -> InteractionEvaluationContext {
        InteractionEvaluationContext {
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("set-1").unwrap()),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            sources: vec![source()],
            boundaries: vec![r("boundary-1")],
            supported_kinds: vec![
                InteractionKind::Request,
                InteractionKind::Response,
                InteractionKind::NoOp,
            ],
        }
    }
    fn claim() -> InteractionClaim {
        InteractionClaim {
            identity: InteractionIdentity {
                id: r("interaction-1"),
                subject: r("subject-1"),
                kind: InteractionKind::Request,
                purpose: r("purpose"),
                scope: r("scope"),
                context: ContextId::new("ctx-1").unwrap(),
                source_set_id: Some(SourceSetId::new("set-1").unwrap()),
                version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            },
            basis: r("basis"),
            source: source(),
            content: None,
            artifact: None,
            provenance: Some(r("prov")),
            evidence: vec![],
            historical: false,
            projected: false,
        }
    }
    fn req() -> InteractionEvaluationRequest {
        InteractionEvaluationRequest {
            evaluation_id: r("eval-1"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("set-1").unwrap()),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            interaction: Some(claim()),
            participants: vec![ParticipantRole {
                participant: r("p-1"),
                role: RoleKind::Initiator,
                subject: Some(r("subject-1")),
                participation: Some(r("part")),
                authority: Some(r("auth")),
            }],
            boundary: None,
            crossing: None,
            handoff: None,
            response: None,
            propagation: None,
            containment: None,
            relationships: vec![],
        }
    }
    #[test]
    fn support_is_not_execution() {
        let x = evaluate_interaction_support(&req(), &ctx());
        assert_eq!(x.support, InteractionSupportDetermination::Supported);
        assert!(!x.execution_performed);
        assert!(!x.transmission_performed)
    }
    #[test]
    fn admission_precedes_support() {
        let mut q = req();
        q.interaction = None;
        let x = evaluate_interaction_support(&q, &ctx());
        assert_eq!(x.support, InteractionSupportDetermination::Denied)
    }
    #[test]
    fn unsupported_boundary_is_not_crossed() {
        let mut q = req();
        q.boundary = Some(ConstitutionalBoundary {
            identity: r("unknown"),
            kind: BoundaryKind::Operational,
            domain: r("d"),
            scope: r("s"),
            subject: None,
            source_side: r("a"),
            destination_side: r("b"),
            condition: None,
            constraint: None,
            admission_rule: None,
            exclusion: None,
            authority: None,
            jurisdiction: None,
            provenance: None,
            context: ContextId::new("ctx-1").unwrap(),
            source: source(),
            version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
        });
        let x = evaluate_boundary_recognition(&q, &ctx());
        assert_eq!(
            x.boundary_recognition,
            BoundaryRecognitionDetermination::NotRecognized
        )
    }
    #[test]
    fn projection_is_explicitly_non_effective() {
        let x = project_interaction_result(&req(), &ctx());
        assert_eq!(x.projection, ProjectionDetermination::Projected);
        let p = x.projected.unwrap();
        assert!(p.projected && p.non_effective && p.non_transmitted)
    }
}
