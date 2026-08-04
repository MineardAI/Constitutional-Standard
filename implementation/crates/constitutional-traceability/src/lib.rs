//! Bounded IMP-004 traceability realization.
//!
//! The graph records attributable relationships among implementation objects.
//! It does not interpret constitutional sources, establish correctness,
//! provide assurance, certify conformance, authorize release or activation,
//! or invoke any represented component.

use constitutional_contracts::{
    ConstitutionalSourceRef, EvidenceRecordId, ImplementationMappingId, ImplementationVersion,
    RequirementRef, SourceSetId,
};
use std::collections::{BTreeMap, BTreeSet};

pub const PROFILE_ID: &str = "reference-implementation-traceability";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const PRIMARY_SOURCE_VERSION: &str = "0.1.0";
pub const IMPLEMENTATION_VERSION: &str = "reference-foundation-0.15.0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplementationMapping {
    pub mapping_id: ImplementationMappingId,
    pub requirement: RequirementRef,
    pub requirement_class: String,
    pub constitutional_owner: String,
    pub implementation_responsibility: String,
    pub implementing_component: String,
    pub implementation_reference: String,
    pub verification_obligation: String,
    pub test_reference: String,
    pub evidence_reference: EvidenceRecordId,
    pub implementation_version: ImplementationVersion,
    pub status: String,
    pub known_limitations: String,
    pub unresolved_issues: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraceRef(String);

impl TraceRef {
    pub fn new(value: impl Into<String>) -> Result<Self, TraceabilityError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(|c| c.is_control() || c.is_whitespace()) {
            return Err(TraceabilityError::MalformedReference(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for TraceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TraceSubjectKind {
    Source,
    Requirement,
    Component,
    Artifact,
    ImplementationVersion,
    VerificationActivity,
    VerificationObligation,
    Finding,
    Evidence,
    Deviation,
    Dependency,
    LifecyclePhase,
    Gate,
    Mapping,
    Report,
    Constraint,
    Invariant,
    AuthorityRecord,
    ReleaseRecord,
    ActivationRecord,
    OperationalRecognitionRecord,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraceSubject {
    pub id: TraceRef,
    pub kind: TraceSubjectKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceBinding {
    pub subject: TraceRef,
    pub source: ConstitutionalSourceRef,
    pub source_set_id: Option<SourceSetId>,
    pub scope: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementBinding {
    pub subject: TraceSubject,
    pub requirement: RequirementRef,
    pub class: TraceRef,
    pub source_binding: SourceBinding,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentBinding {
    pub subject: TraceSubject,
    pub responsibility: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub locator: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactBinding {
    pub subject: TraceSubject,
    pub role: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub locator: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplementationVersionBinding {
    pub subject: TraceSubject,
    pub version: ImplementationVersion,
    pub baseline: Option<TraceRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReference {
    pub subject: TraceSubject,
    pub record_id: EvidenceRecordId,
    pub implementation_version: ImplementationVersion,
    pub scope: TraceRef,
    pub producer: TraceRef,
    pub context: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviationReference {
    pub subject: TraceSubject,
    pub disposition: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub scope: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyReference {
    pub subject: TraceSubject,
    pub upstream: TraceSubject,
    pub downstream: TraceSubject,
    pub required: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecyclePhaseReference {
    pub subject: TraceSubject,
    pub implementation_version: ImplementationVersion,
    pub status: TraceRef,
    pub scope: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateReference {
    pub subject: TraceSubject,
    pub phase: TraceRef,
    pub disposition: TraceRef,
    pub scope: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceOriginReference {
    pub subject: TraceSubject,
    pub origin: TraceRef,
    pub recorded_by: TraceRef,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TraceRelation {
    Implements,
    PartiallyImplements,
    Supports,
    Enforces,
    Prevents,
    Validates,
    Verifies,
    ProducesEvidenceFor,
    DependsOn,
    ConstrainedBy,
    Represents,
    SupersedesMapping,
    AffectedByChange,
    DeviatesFrom,
    Extends,
    EvidencesNonOccurrence,
    DerivesFrom,
    IsVerifiedBy,
    IsSupportedBy,
    IsGovernedBy,
    IsAdmittedUnder,
    IsBoundTo,
    IsApplicableTo,
    IsIncompatibleWith,
    IsReplacedBy,
    IsUnchangedFrom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceEdge {
    pub id: TraceRef,
    pub source: TraceSubject,
    pub target: TraceSubject,
    pub relation: TraceRelation,
    pub scope: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub provenance: Option<ProvenanceOriginReference>,
    pub limitations: Vec<TraceRef>,
    pub status: TraceRef,
    pub declares_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceRecord {
    pub id: TraceRef,
    pub subject: TraceSubject,
    pub edges: Vec<TraceRef>,
    pub implementation_version: ImplementationVersion,
    pub status: TraceRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MandatoryTracePath {
    pub id: TraceRef,
    pub start: TraceSubjectKind,
    pub relation: TraceRelation,
    pub end: TraceSubjectKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceGraph {
    pub graph_id: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub source_bindings: Vec<SourceBinding>,
    pub subjects: Vec<TraceSubject>,
    pub edges: Vec<TraceEdge>,
    pub records: Vec<TraceRecord>,
    pub mandatory_paths: Vec<MandatoryTracePath>,
    pub allow_cycles: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TraceabilityFindingCode {
    MissingTraceIdentifier,
    DuplicateTraceIdentifier,
    DuplicateEdge,
    SelfReference,
    UnknownSubject,
    UnknownRelation,
    MalformedSourceBinding,
    MissingSourceVersion,
    ImplementationVersionMismatch,
    IncompatibleEndpointTypes,
    InvalidLifecycleReference,
    InvalidGateReference,
    InvalidDeviationReference,
    InvalidEvidenceReference,
    InvalidDependencyReference,
    ProhibitedAuthorityImplication,
    CycleDetected,
    UnresolvedRequiredDependency,
    InconsistentSourceBinding,
    InconsistentArtifactBinding,
    InconsistentComponentBinding,
    ContradictoryRelation,
    IncompleteMandatoryPath,
    MissingRequiredField,
    UnsupportedRelation,
    CanonicalSupportDeferred,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceabilityFinding {
    pub code: TraceabilityFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceabilityValidationStatus {
    Valid,
    Invalid,
    Indeterminate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceabilityValidationResult {
    pub graph_id: TraceRef,
    pub implementation_version: ImplementationVersion,
    pub profile_version: TraceRef,
    pub status: TraceabilityValidationStatus,
    pub findings: Vec<TraceabilityFinding>,
    pub checked_subjects: usize,
    pub checked_edges: usize,
    pub execution_performed: bool,
    pub authority_created: bool,
    pub conformance_established: bool,
    pub non_claims: Vec<String>,
}

fn finding(
    code: TraceabilityFindingCode,
    subject: impl Into<String>,
    detail: impl Into<String>,
    fatal: bool,
) -> TraceabilityFinding {
    TraceabilityFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
fn is_ref_valid(value: &str) -> bool {
    !value.is_empty() && !value.chars().any(|c| c.is_control() || c.is_whitespace())
}
fn endpoint_compatible(
    relation: &TraceRelation,
    source: &TraceSubjectKind,
    target: &TraceSubjectKind,
) -> bool {
    use TraceRelation::*;
    use TraceSubjectKind::*;
    match relation {
        Implements | PartiallyImplements | ConstrainedBy => {
            matches!(source, Requirement | Constraint | Invariant)
                && matches!(target, Component | Artifact | ImplementationVersion)
        }
        Supports | Extends => {
            matches!(source, Component | Artifact | VerificationActivity)
                && matches!(target, Requirement | Constraint | Invariant)
        }
        Validates => {
            matches!(source, VerificationActivity | Component)
                && matches!(target, Requirement | VerificationObligation | Finding)
        }
        Verifies => {
            matches!(source, VerificationActivity)
                && matches!(target, VerificationObligation | Requirement)
        }
        ProducesEvidenceFor => {
            matches!(source, VerificationActivity | Component)
                && matches!(target, Evidence | Requirement | Finding | Gate)
        }
        DependsOn => !matches!(source, Source) && !matches!(target, Source),
        Represents => {
            matches!(source, Report | Artifact | Component | Evidence | Mapping)
                && !matches!(target, Source)
        }
        SupersedesMapping | IsReplacedBy => {
            matches!(source, Mapping | Requirement | ImplementationVersion)
                && matches!(target, Mapping | Requirement | ImplementationVersion)
        }
        AffectedByChange => {
            matches!(
                source,
                Mapping | Component | Artifact | Requirement | Evidence
            ) && matches!(target, Deviation | Dependency)
        }
        DeviatesFrom => {
            matches!(source, Mapping | Component | Artifact)
                && matches!(target, Requirement | Mapping)
        }
        EvidencesNonOccurrence => {
            matches!(source, Evidence | VerificationActivity)
                && matches!(target, Requirement | Finding | Invariant)
        }
        DerivesFrom | IsSupportedBy | IsGovernedBy | IsAdmittedUnder | IsBoundTo
        | IsApplicableTo | IsUnchangedFrom => true,
        Enforces | Prevents => {
            matches!(source, Component | VerificationActivity)
                && matches!(target, Constraint | Invariant | Requirement)
        }
        IsVerifiedBy => true,
        IsIncompatibleWith => true,
    }
}

fn has_cycle(graph: &TraceGraph) -> bool {
    let mut adjacency: BTreeMap<TraceSubject, Vec<TraceSubject>> = BTreeMap::new();
    for edge in &graph.edges {
        adjacency
            .entry(edge.source.clone())
            .or_default()
            .push(edge.target.clone());
    }
    fn visit(
        node: &TraceSubject,
        adjacency: &BTreeMap<TraceSubject, Vec<TraceSubject>>,
        visiting: &mut BTreeSet<TraceSubject>,
        visited: &mut BTreeSet<TraceSubject>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }
        visiting.insert(node.clone());
        if adjacency.get(node).is_some_and(|targets| {
            targets
                .iter()
                .any(|target| visit(target, adjacency, visiting, visited))
        }) {
            return true;
        }
        visiting.remove(node);
        visited.insert(node.clone());
        false
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    adjacency
        .keys()
        .any(|node| visit(node, &adjacency, &mut visiting, &mut visited))
}

pub fn validate_trace_graph(graph: &TraceGraph) -> TraceabilityValidationResult {
    let mut findings = Vec::new();
    if !is_ref_valid(graph.graph_id.as_str()) {
        findings.push(finding(
            TraceabilityFindingCode::MissingTraceIdentifier,
            "graph",
            "graph identifier is missing or malformed",
            true,
        ));
    }
    let subject_set: BTreeSet<_> = graph.subjects.iter().cloned().collect();
    if subject_set.len() != graph.subjects.len() {
        findings.push(finding(
            TraceabilityFindingCode::DuplicateTraceIdentifier,
            "subjects",
            "duplicate trace subject identifier and kind",
            true,
        ));
    }
    let mut ids = BTreeSet::new();
    for subject in &graph.subjects {
        if !is_ref_valid(subject.id.as_str()) {
            findings.push(finding(
                TraceabilityFindingCode::MissingTraceIdentifier,
                subject.id.to_string(),
                "trace subject identifier is malformed",
                true,
            ));
        }
        if !ids.insert(subject.id.clone()) {
            findings.push(finding(
                TraceabilityFindingCode::DuplicateTraceIdentifier,
                subject.id.to_string(),
                "trace identifier is duplicated",
                true,
            ));
        }
    }
    let mut edge_keys = BTreeSet::new();
    let mut relation_keys = BTreeSet::new();
    for edge in &graph.edges {
        if !is_ref_valid(edge.id.as_str()) {
            findings.push(finding(
                TraceabilityFindingCode::MissingTraceIdentifier,
                "edge",
                "edge identifier is malformed",
                true,
            ));
        }
        if !subject_set.contains(&edge.source) || !subject_set.contains(&edge.target) {
            findings.push(finding(
                TraceabilityFindingCode::UnknownSubject,
                edge.id.to_string(),
                "edge endpoint is not in the graph",
                true,
            ));
        }
        if edge.source == edge.target {
            findings.push(finding(
                TraceabilityFindingCode::SelfReference,
                edge.id.to_string(),
                "self-reference is prohibited",
                true,
            ));
        }
        let key = (
            edge.source.clone(),
            edge.target.clone(),
            edge.relation.clone(),
        );
        if !edge_keys.insert(key) {
            findings.push(finding(
                TraceabilityFindingCode::DuplicateEdge,
                edge.id.to_string(),
                "duplicate edge relation",
                true,
            ));
        }
        if !relation_keys.insert(edge.id.clone()) {
            findings.push(finding(
                TraceabilityFindingCode::DuplicateTraceIdentifier,
                edge.id.to_string(),
                "edge identifier is duplicated",
                true,
            ));
        }
        if !endpoint_compatible(&edge.relation, &edge.source.kind, &edge.target.kind) {
            findings.push(finding(
                TraceabilityFindingCode::IncompatibleEndpointTypes,
                edge.id.to_string(),
                "relation endpoint types are incompatible",
                true,
            ));
        }
        if edge.declares_authority {
            findings.push(finding(
                TraceabilityFindingCode::ProhibitedAuthorityImplication,
                edge.id.to_string(),
                "trace relation cannot declare or grant authority",
                true,
            ));
        }
        if edge.implementation_version != graph.implementation_version {
            findings.push(finding(
                TraceabilityFindingCode::ImplementationVersionMismatch,
                edge.id.to_string(),
                "edge implementation version differs from graph",
                true,
            ));
        }
    }
    for binding in &graph.source_bindings {
        if !is_ref_valid(binding.subject.as_str()) || binding.source.path.is_empty() {
            findings.push(finding(
                TraceabilityFindingCode::MalformedSourceBinding,
                binding.subject.to_string(),
                "source binding is malformed",
                true,
            ));
        }
        if binding.source.version.as_str().is_empty() {
            findings.push(finding(
                TraceabilityFindingCode::MissingSourceVersion,
                binding.subject.to_string(),
                "source version is missing",
                true,
            ));
        }
        if !subject_set.iter().any(|s| s.id == binding.subject) {
            findings.push(finding(
                TraceabilityFindingCode::UnknownSubject,
                binding.subject.to_string(),
                "source binding subject is unknown",
                true,
            ));
        }
    }
    for path in &graph.mandatory_paths {
        if !graph.edges.iter().any(|edge| {
            edge.source.kind == path.start
                && edge.relation == path.relation
                && edge.target.kind == path.end
        }) {
            findings.push(finding(
                TraceabilityFindingCode::IncompleteMandatoryPath,
                path.id.to_string(),
                "mandatory trace path is incomplete",
                true,
            ));
        }
    }
    if !graph.allow_cycles && has_cycle(graph) {
        findings.push(finding(
            TraceabilityFindingCode::CycleDetected,
            graph.graph_id.to_string(),
            "trace graph contains a prohibited cycle",
            true,
        ));
    }
    let status = if findings.iter().any(|f| f.fatal) {
        TraceabilityValidationStatus::Invalid
    } else {
        TraceabilityValidationStatus::Valid
    };
    findings.sort_by(|a, b| a.subject.cmp(&b.subject).then_with(|| a.code.cmp(&b.code)));
    TraceabilityValidationResult{graph_id:graph.graph_id.clone(),implementation_version:graph.implementation_version.clone(),profile_version:TraceRef::new(PROFILE_VERSION).unwrap(),status,findings,checked_subjects:graph.subjects.len(),checked_edges:graph.edges.len(),execution_performed:false,authority_created:false,conformance_established:false,non_claims:vec!["traceability is not constitutional validity or implementation correctness".into(),"traceability is not verification assurance, evidence sufficiency, conformance, certification, release authorization, activation, or operational recognition".into()]}
}

/// Validates a bounded source-required composition without turning
/// traceability into correctness, conformance, certification, or authority.
pub fn validate_required_composition(
    graph: &TraceGraph,
    required_subjects: &[TraceSubject],
    required_edges: &[(TraceRef, TraceRef, TraceRelation)],
) -> TraceabilityValidationResult {
    let mut result = validate_trace_graph(graph);
    for subject in required_subjects {
        if !graph.subjects.contains(subject) {
            result.findings.push(finding(
                TraceabilityFindingCode::UnknownSubject,
                subject.id.to_string(),
                "required source-derived composition subject is unresolved",
                true,
            ));
        }
    }
    for (source, target, relation) in required_edges {
        let present = graph.edges.iter().any(|edge| {
            edge.source.id == *source && edge.target.id == *target && edge.relation == *relation
        });
        if !present {
            result.findings.push(finding(
                TraceabilityFindingCode::IncompleteMandatoryPath,
                format!("{source}->{target}"),
                "required source-derived composition edge is unresolved",
                true,
            ));
        }
    }
    result
        .findings
        .sort_by(|a, b| a.subject.cmp(&b.subject).then_with(|| a.code.cmp(&b.code)));
    result.status = if result.findings.iter().any(|finding| finding.fatal) {
        TraceabilityValidationStatus::Invalid
    } else {
        TraceabilityValidationStatus::Valid
    };
    result
}

impl ImplementationMapping {
    pub fn reverse_lookup(&self, implementation_reference: &str) -> bool {
        self.implementation_reference == implementation_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceabilityError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use constitutional_contracts::{
        ConstitutionalSourceId, ConstitutionalSourceVersion, ContextId, OperationId,
    };
    fn r(v: &str) -> TraceRef {
        TraceRef::new(v).unwrap()
    }
    fn subject(id: &str, kind: TraceSubjectKind) -> TraceSubject {
        TraceSubject { id: r(id), kind }
    }
    fn graph(edges: Vec<TraceEdge>) -> TraceGraph {
        TraceGraph {
            graph_id: r("graph-1"),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            source_bindings: vec![SourceBinding {
                subject: r("source-1"),
                source: ConstitutionalSourceRef::new(
                    ConstitutionalSourceId::new("IMP-004").unwrap(),
                    ConstitutionalSourceVersion::new("0.1.0").unwrap(),
                    "harmonization-v1.0/IMP-004_Constitutional_Traceability_Model_v0.1.0_Draft.md",
                )
                .unwrap(),
                source_set_id: None,
                scope: r("phase-11"),
            }],
            subjects: vec![
                subject("source-1", TraceSubjectKind::Source),
                subject("req-1", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                subject("verify-1", TraceSubjectKind::VerificationActivity),
                subject("evidence-1", TraceSubjectKind::Evidence),
            ],
            edges,
            records: vec![],
            mandatory_paths: vec![MandatoryTracePath {
                id: r("path-1"),
                start: TraceSubjectKind::Requirement,
                relation: TraceRelation::Implements,
                end: TraceSubjectKind::Component,
            }],
            allow_cycles: false,
        }
    }
    fn edge(
        id: &str,
        source: TraceSubject,
        target: TraceSubject,
        relation: TraceRelation,
    ) -> TraceEdge {
        TraceEdge {
            id: r(id),
            source,
            target,
            relation,
            scope: r("phase-11"),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            provenance: None,
            limitations: vec![],
            status: r("declared"),
            declares_authority: false,
        }
    }
    #[test]
    fn valid_multi_domain_trace_graph_passes() {
        let g = graph(vec![
            edge(
                "edge-1",
                subject("req-1", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
            edge(
                "edge-2",
                subject("verify-1", TraceSubjectKind::VerificationActivity),
                subject("evidence-1", TraceSubjectKind::Evidence),
                TraceRelation::ProducesEvidenceFor,
            ),
        ]);
        let x = validate_trace_graph(&g);
        assert_eq!(x.status, TraceabilityValidationStatus::Valid);
        assert!(!x.execution_performed && !x.authority_created && !x.conformance_established)
    }
    #[test]
    fn duplicate_identifier_and_edge_are_rejected() {
        let mut g = graph(vec![
            edge(
                "edge-1",
                subject("req-1", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
            edge(
                "edge-1",
                subject("req-1", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
        ]);
        g.subjects
            .push(subject("req-1", TraceSubjectKind::Requirement));
        let x = validate_trace_graph(&g);
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::DuplicateTraceIdentifier)
        );
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::DuplicateEdge)
        )
    }
    #[test]
    fn unknown_endpoint_and_invalid_relation_are_rejected() {
        let g = graph(vec![
            edge(
                "edge-1",
                subject("unknown", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
            edge(
                "edge-2",
                subject("component-1", TraceSubjectKind::Component),
                subject("req-1", TraceSubjectKind::Requirement),
                TraceRelation::Implements,
            ),
        ]);
        let x = validate_trace_graph(&g);
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::UnknownSubject)
        );
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::IncompatibleEndpointTypes)
        )
    }
    #[test]
    fn binding_and_version_mismatches_are_explicit() {
        let mut g = graph(vec![edge(
            "edge-1",
            subject("req-1", TraceSubjectKind::Requirement),
            subject("component-1", TraceSubjectKind::Component),
            TraceRelation::Implements,
        )]);
        g.edges[0].implementation_version =
            ImplementationVersion::new("reference-foundation-9.9.9").unwrap();
        let x = validate_trace_graph(&g);
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::ImplementationVersionMismatch)
        )
    }
    #[test]
    fn prohibited_authority_implication_is_rejected() {
        let mut g = graph(vec![edge(
            "edge-1",
            subject("component-1", TraceSubjectKind::Component),
            subject("req-1", TraceSubjectKind::Requirement),
            TraceRelation::Supports,
        )]);
        g.edges[0].declares_authority = true;
        let x = validate_trace_graph(&g);
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::ProhibitedAuthorityImplication)
        );
        assert!(!x.authority_created)
    }
    #[test]
    fn self_reference_cycle_and_missing_path_are_rejected() {
        let mut g = graph(vec![
            edge(
                "edge-1",
                subject("component-1", TraceSubjectKind::Component),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::DependsOn,
            ),
            edge(
                "edge-2",
                subject("component-1", TraceSubjectKind::Component),
                subject("req-1", TraceSubjectKind::Requirement),
                TraceRelation::DependsOn,
            ),
            edge(
                "edge-3",
                subject("req-1", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::DependsOn,
            ),
        ]);
        g.mandatory_paths.push(MandatoryTracePath {
            id: r("missing"),
            start: TraceSubjectKind::Artifact,
            relation: TraceRelation::Implements,
            end: TraceSubjectKind::ImplementationVersion,
        });
        let x = validate_trace_graph(&g);
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::SelfReference)
        );
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::CycleDetected)
        );
        assert!(
            x.findings
                .iter()
                .any(|f| f.code == TraceabilityFindingCode::IncompleteMandatoryPath)
        )
    }
    #[test]
    fn findings_are_deterministically_ordered() {
        let mut g = graph(vec![
            edge(
                "z",
                subject("unknown-z", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
            edge(
                "a",
                subject("unknown-a", TraceSubjectKind::Requirement),
                subject("component-1", TraceSubjectKind::Component),
                TraceRelation::Implements,
            ),
        ]);
        let first = validate_trace_graph(&g);
        g.edges.reverse();
        let second = validate_trace_graph(&g);
        assert_eq!(first.findings, second.findings)
    }

    #[test]
    fn required_release_assignment_activation_recognition_composition_is_bound() {
        let release = subject("release-1", TraceSubjectKind::ReleaseRecord);
        let assignment = subject("assignment-1", TraceSubjectKind::ActivationRecord);
        let activation = subject("activation-1", TraceSubjectKind::ActivationRecord);
        let recognition = subject(
            "recognition-1",
            TraceSubjectKind::OperationalRecognitionRecord,
        );
        let mut g = graph(vec![
            edge(
                "release-assignment",
                release.clone(),
                assignment.clone(),
                TraceRelation::IsBoundTo,
            ),
            edge(
                "assignment-activation",
                assignment.clone(),
                activation.clone(),
                TraceRelation::IsBoundTo,
            ),
            edge(
                "activation-recognition",
                activation.clone(),
                recognition.clone(),
                TraceRelation::IsBoundTo,
            ),
        ]);
        g.subjects.extend([
            release.clone(),
            assignment.clone(),
            activation.clone(),
            recognition.clone(),
        ]);
        g.mandatory_paths.clear();
        let valid = validate_required_composition(
            &g,
            &[
                release.clone(),
                assignment.clone(),
                activation.clone(),
                recognition.clone(),
            ],
            &[
                (
                    release.id.clone(),
                    assignment.id.clone(),
                    TraceRelation::IsBoundTo,
                ),
                (
                    assignment.id.clone(),
                    activation.id.clone(),
                    TraceRelation::IsBoundTo,
                ),
                (
                    activation.id.clone(),
                    recognition.id.clone(),
                    TraceRelation::IsBoundTo,
                ),
            ],
        );
        assert_eq!(valid.status, TraceabilityValidationStatus::Valid);
        let missing = validate_required_composition(
            &g,
            &[
                release.clone(),
                assignment.clone(),
                activation.clone(),
                recognition.clone(),
            ],
            &[(release.id, recognition.id, TraceRelation::IsBoundTo)],
        );
        assert_eq!(missing.status, TraceabilityValidationStatus::Invalid);
    }
    #[test]
    fn mapping_reverse_lookup_remains_descriptive() {
        let mapping = ImplementationMapping {
            mapping_id: ImplementationMappingId::new("mapping-1").unwrap(),
            requirement: RequirementRef {
                source: ConstitutionalSourceRef::new(
                    ConstitutionalSourceId::new("IMP-004").unwrap(),
                    ConstitutionalSourceVersion::new("0.1.0").unwrap(),
                    "source",
                )
                .unwrap(),
                requirement_id: constitutional_contracts::RequirementId::new("IMP-004-001")
                    .unwrap(),
                locator: "Â§1".into(),
            },
            requirement_class: "traceability".into(),
            constitutional_owner: "IMP-004".into(),
            implementation_responsibility: "traceability".into(),
            implementing_component: "constitutional-traceability".into(),
            implementation_reference: "crates/constitutional-traceability/src/lib.rs".into(),
            verification_obligation: "deterministic validation".into(),
            test_reference: "traceability tests".into(),
            evidence_reference: EvidenceRecordId::new("EV-TRACE-0001").unwrap(),
            implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
            status: "implemented".into(),
            known_limitations: "no assurance".into(),
            unresolved_issues: "none".into(),
        };
        assert!(mapping.reverse_lookup("crates/constitutional-traceability/src/lib.rs"));
        let _ = ContextId::new("ctx").unwrap();
        let _ = OperationId::new("op").unwrap();
    }
}
