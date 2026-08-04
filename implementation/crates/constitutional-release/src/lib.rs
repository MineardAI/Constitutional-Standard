//! Bounded IMP-008 release continuity and compatibility records.
//!
//! Release standing is represented, not operationally enacted. This crate
//! does not publish packages, execute migrations, activate implementations,
//! establish authority, or infer constitutional precedence from versions.

use constitutional_canonical::{
    CanonicalField, CanonicalObject, CanonicalRepresentationId, CanonicalRepresentationKind,
    CanonicalValue, REPRESENTATION_MODEL_VERSION,
};
use constitutional_contracts::{BaselineRef, EvidenceRecordId, ImplementationVersion, TimePoint};
use constitutional_traceability::TraceRef;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const RELEASE_PROFILE_ID: &str = "reference-implementation-release-continuity-compatibility";
pub const RELEASE_PROFILE_VERSION: &str = "1.0.0";

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ReleaseError> {
                let value = value.into();
                if value.is_empty() || value.chars().any(|c| c.is_whitespace() || c.is_control()) {
                    return Err(ReleaseError::MalformedIdentifier(value));
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

id_type!(ReleaseIdentifier);
id_type!(ReleaseVersionIdentifier);
id_type!(ReleaseManifestIdentifier);
id_type!(ReleasePackageIdentifier);
id_type!(ReleaseRelationshipIdentifier);
id_type!(CompatibilityAssessmentIdentifier);
id_type!(CompatibilityClaimIdentifier);
id_type!(MigrationRequirementIdentifier);
id_type!(MigrationPlanIdentifier);
id_type!(MigrationRecordIdentifier);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseError {
    MalformedIdentifier(String),
    Invalid(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseIdentity {
    pub id: ReleaseIdentifier,
    pub version: ReleaseVersionIdentifier,
    pub implementation_version: ImplementationVersion,
    pub baseline: BaselineRef,
}

impl ReleaseIdentity {
    pub fn validate_version(&self) -> Result<(), ReleaseError> {
        let value = self.version.as_str();
        if value.split('.').count() < 2
            || value
                .split('.')
                .any(|part| part.is_empty() || part.parse::<u64>().is_err())
        {
            return Err(ReleaseError::Invalid(format!(
                "invalid release version: {value}"
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseState {
    Candidate,
    Released,
    Deprecated,
    Superseded,
    Retired,
    Historical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationState {
    Unpublished,
    Published,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseManifest {
    pub id: ReleaseManifestIdentifier,
    pub release: ReleaseIdentity,
    pub constitutional_baseline: Option<String>,
    pub applicable_specifications: Vec<String>,
    pub components: Vec<String>,
    pub evidence: Vec<EvidenceRecordId>,
    pub assurance_references: Vec<String>,
    pub traceability_references: Vec<TraceRef>,
    pub limitations: Vec<String>,
    pub publication: PublicationState,
    pub claims_activation: bool,
    pub claims_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleasePackage {
    pub id: ReleasePackageIdentifier,
    pub manifest: ReleaseManifest,
    pub assembled_at: TimePoint,
    pub assembler: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleasedImplementation {
    pub identity: ReleaseIdentity,
    pub state: ReleaseState,
    pub manifest: ReleaseManifest,
    pub effective_from: Option<TimePoint>,
    pub support_until: Option<TimePoint>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LineageRelation {
    Predecessor,
    Successor,
    Supersedes,
    SupersededBy,
    DerivedFrom,
    MigratedFrom,
    CompatibleWith,
    ConditionallyCompatibleWith,
    IncompatibleWith,
    HistoricallyRetained,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseRelationship {
    pub id: ReleaseRelationshipIdentifier,
    pub from: ReleaseIdentifier,
    pub to: ReleaseIdentifier,
    pub relation: LineageRelation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseLineage {
    pub releases: Vec<ReleaseIdentity>,
    pub relationships: Vec<ReleaseRelationship>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseResolutionContext {
    pub releases: Vec<ReleasedImplementation>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseTraceabilityChain {
    pub subject: ReleaseIdentifier,
    pub implementation_version: ImplementationVersion,
    pub release: ReleaseIdentifier,
    pub lineage: Vec<ReleaseRelationshipIdentifier>,
    pub compatibility: Vec<CompatibilityAssessmentIdentifier>,
    pub migration: Vec<MigrationRequirementIdentifier>,
    pub evidence: Vec<EvidenceRecordId>,
    pub activation: Vec<String>,
    pub recognition: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseHistoryEntry {
    pub sequence: u64,
    pub release: ReleaseIdentity,
    pub state: ReleaseState,
    pub predecessor: Option<ReleaseIdentifier>,
    pub migration: Vec<MigrationRequirementIdentifier>,
    pub compatibility: Vec<CompatibilityAssessmentIdentifier>,
    pub evidence: Vec<EvidenceRecordId>,
    pub assurance: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseHistoryAttachment {
    pub id: String,
    pub release: ReleaseIdentifier,
    pub implementation_version: ImplementationVersion,
    pub applicability: Option<ApplicabilityDetermination>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalTraceReference {
    pub id: String,
    pub subject: ReleaseIdentifier,
    pub release: ReleaseIdentifier,
    pub implementation_version: ImplementationVersion,
    pub authority: Option<TraceRef>,
    pub baseline: Option<TraceRef>,
    pub scope: Option<TraceRef>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseOperationalTrace {
    pub release: ReleaseIdentifier,
    pub implementation_version: ImplementationVersion,
    pub assignment: OperationalTraceReference,
    pub activation_request: OperationalTraceReference,
    pub activation_decision: OperationalTraceReference,
    pub activation_record: OperationalTraceReference,
    pub recognition: Option<OperationalTraceReference>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompatibilityDimension {
    Representation,
    Interface,
    Behavioral,
    Data,
    Dependency,
    ConstitutionalContext,
    EvidenceApplicability,
    Other(String),
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompatibilityDirection {
    SubjectToTarget,
    TargetToSubject,
    Bidirectional,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompatibilityOutcome {
    Compatible,
    Incompatible,
    ConditionallyCompatible,
    Indeterminate,
    NotApplicable,
    InvalidOrUnsupportedRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityClaim {
    pub id: CompatibilityClaimIdentifier,
    pub subject: ReleaseIdentifier,
    pub target: ReleaseIdentifier,
    pub dimension: CompatibilityDimension,
    pub statement: String,
    pub evidence: Vec<EvidenceRecordId>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityAssessment {
    pub id: CompatibilityAssessmentIdentifier,
    pub subject: ReleaseIdentifier,
    pub target: ReleaseIdentifier,
    pub dimension: CompatibilityDimension,
    pub direction: CompatibilityDirection,
    pub scope: String,
    pub criteria: Vec<String>,
    pub assumptions: Vec<String>,
    pub baseline: Option<BaselineRef>,
    pub implementation_context: String,
    pub admitted_source_set: Option<String>,
    pub traceability: Vec<TraceRef>,
    pub evidence: Vec<EvidenceRecordId>,
    pub verification: Vec<String>,
    pub claims: Vec<CompatibilityClaimIdentifier>,
    pub outcome: CompatibilityOutcome,
    pub limitations: Vec<String>,
    pub unresolved_conditions: Vec<String>,
    pub implementation_version: ImplementationVersion,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeEffect {
    NoConstitutionalEffect,
    RepresentationalEffect,
    BehavioralEffect,
    CompatibilityEffect,
    MigrationRequiredEffect,
    BreakingEffect,
    HistoricalContinuityEffect,
    IndeterminateEffect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeClassification {
    pub effect: ChangeEffect,
    pub basis: String,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationStatus {
    Declared,
    Planned,
    ExecutionRecorded,
    VerificationRecorded,
    OutcomeRecorded,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationRequirement {
    pub id: MigrationRequirementIdentifier,
    pub source: ReleaseIdentifier,
    pub target: ReleaseIdentifier,
    pub requirements: Vec<String>,
    pub preconditions: Vec<String>,
    pub preserved_properties: Vec<String>,
    pub known_losses: Vec<String>,
    pub compatibility_effects: Vec<ChangeEffect>,
    pub evidence: Vec<EvidenceRecordId>,
    pub verification: Vec<String>,
    pub limitations: Vec<String>,
    pub unresolved_conditions: Vec<String>,
    pub status: MigrationStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPlan {
    pub id: MigrationPlanIdentifier,
    pub requirement: MigrationRequirementIdentifier,
    pub source: ReleaseIdentifier,
    pub target: ReleaseIdentifier,
    pub steps: Vec<String>,
    pub transformations: Vec<String>,
    pub limitations: Vec<String>,
    pub status: MigrationStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationExecutionRecord {
    pub id: MigrationRecordIdentifier,
    pub plan: MigrationPlanIdentifier,
    pub declared_only: bool,
    pub status: MigrationStatus,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicabilityDetermination {
    Applicable,
    ApplicableWithRestrictions,
    RequiresRevalidation,
    Superseded,
    Invalidated,
    HistoricalOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceApplicability {
    pub evidence: EvidenceRecordId,
    pub release: ReleaseIdentifier,
    pub determination: ApplicabilityDetermination,
    pub limitations: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssuranceApplicability {
    pub assurance: String,
    pub release: ReleaseIdentifier,
    pub determination: ApplicabilityDetermination,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseFindingCode {
    DuplicateReleaseIdentity,
    MalformedReleaseIdentity,
    InvalidReleaseVersion,
    UnsupportedReleaseState,
    UnknownReleaseReference,
    SelfRelationship,
    ContradictoryLineage,
    ImpossibleSupersession,
    DuplicateCompatibilityClaim,
    ConflictingCompatibilityClaim,
    MissingCompatibilityDimension,
    UnsupportedCompatibilityDimension,
    MissingDeterminationContext,
    InvalidChangeEffect,
    MissingMigrationSource,
    MissingMigrationTarget,
    CircularMigration,
    IdentityErasure,
    MissingProvenanceBinding,
    MissingTraceabilityBinding,
    MissingEvidenceReference,
    MissingAssuranceReference,
    ProhibitedActivationClaim,
    ProhibitedAuthorityClaim,
    PublicationIsNotActivation,
    NewerIsNotSuperior,
    HistoricalRewrite,
    NonCanonicalRepresentation,
    InvalidHistoricalAttachment,
    TraceabilitySubjectMismatch,
    TraceabilityVersionMismatch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseFinding {
    pub code: ReleaseFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseValidationResult {
    pub valid: bool,
    pub findings: Vec<ReleaseFinding>,
    pub activation_performed: bool,
    pub authority_created: bool,
    pub migration_executed: bool,
    pub conformance_established: bool,
}

pub fn validate_released_implementation(
    release: &ReleasedImplementation,
) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    if release.identity.validate_version().is_err() {
        findings.push(finding(
            ReleaseFindingCode::InvalidReleaseVersion,
            release.identity.id.to_string(),
            "release version must contain numeric dot-separated components",
        ));
    }
    if release.manifest.release.id != release.identity.id {
        findings.push(finding(
            ReleaseFindingCode::MalformedReleaseIdentity,
            release.identity.id.to_string(),
            "manifest and released implementation identities differ",
        ));
    }
    if release.manifest.claims_activation {
        findings.push(finding(
            ReleaseFindingCode::ProhibitedActivationClaim,
            release.identity.id.to_string(),
            "IMP-008 cannot activate an implementation",
        ));
    }
    if release.manifest.claims_authority {
        findings.push(finding(
            ReleaseFindingCode::ProhibitedAuthorityClaim,
            release.identity.id.to_string(),
            "release state does not grant authority",
        ));
    }
    if matches!(release.manifest.publication, PublicationState::Published)
        && release.state == ReleaseState::Candidate
    {
        findings.push(finding(
            ReleaseFindingCode::PublicationIsNotActivation,
            release.identity.id.to_string(),
            "publication does not establish activation",
        ));
    }
    ReleaseValidationResult {
        valid: findings.is_empty(),
        findings,
        activation_performed: false,
        authority_created: false,
        migration_executed: false,
        conformance_established: false,
    }
}

pub fn validate_lineage(lineage: &ReleaseLineage) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    for relationship in &lineage.relationships {
        if relationship.from == relationship.to {
            findings.push(finding(
                ReleaseFindingCode::SelfRelationship,
                relationship.id.to_string(),
                "release lineage cannot self-reference",
            ));
        }
        if !lineage
            .releases
            .iter()
            .any(|release| release.id == relationship.from)
            || !lineage
                .releases
                .iter()
                .any(|release| release.id == relationship.to)
        {
            findings.push(finding(
                ReleaseFindingCode::UnknownReleaseReference,
                relationship.id.to_string(),
                "lineage relationship references an unknown release",
            ));
        }
    }
    let mut seen = Vec::new();
    for relationship in &lineage.relationships {
        let key = (
            relationship.from.clone(),
            relationship.to.clone(),
            relationship.relation.clone(),
        );
        if seen.contains(&key) {
            findings.push(finding(
                ReleaseFindingCode::ContradictoryLineage,
                relationship.id.to_string(),
                "duplicate lineage relationship",
            ));
        }
        seen.push(key);
    }
    let mut directional =
        BTreeMap::<(ReleaseIdentifier, ReleaseIdentifier), BTreeSet<LineageRelation>>::new();
    for relationship in &lineage.relationships {
        directional
            .entry((relationship.from.clone(), relationship.to.clone()))
            .or_default()
            .insert(relationship.relation.clone());
    }
    for ((from, to), relations) in &directional {
        if relations.contains(&LineageRelation::Supersedes)
            && relations.contains(&LineageRelation::IncompatibleWith)
        {
            findings.push(finding(
                ReleaseFindingCode::ImpossibleSupersession,
                format!("{from}->{to}"),
                "a superseding release cannot be declared incompatible on the same lineage edge",
            ));
        }
    }
    ReleaseValidationResult {
        valid: findings.is_empty(),
        findings,
        activation_performed: false,
        authority_created: false,
        migration_executed: false,
        conformance_established: false,
    }
}

pub fn validate_release_resolution(context: &ReleaseResolutionContext) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    let mut ids = BTreeSet::new();
    for release in &context.releases {
        if !ids.insert(release.identity.id.clone()) {
            findings.push(finding(
                ReleaseFindingCode::DuplicateReleaseIdentity,
                release.identity.id.to_string(),
                "release resolution context contains a duplicate identity",
            ));
        }
        if release.identity.implementation_version != context.implementation_version {
            findings.push(finding(
                ReleaseFindingCode::IdentityErasure,
                release.identity.id.to_string(),
                "release implementation version differs from the bounded resolution context",
            ));
        }
        if release.manifest.release.id != release.identity.id {
            findings.push(finding(
                ReleaseFindingCode::MalformedReleaseIdentity,
                release.identity.id.to_string(),
                "release manifest identity does not resolve to the release identity",
            ));
        }
    }
    result(findings)
}

pub fn validate_compatibility_with_context(
    assessment: &CompatibilityAssessment,
    claims: &[CompatibilityClaim],
    context: &ReleaseResolutionContext,
) -> ReleaseValidationResult {
    let mut result = validate_compatibility(assessment);
    let known: BTreeSet<_> = context
        .releases
        .iter()
        .map(|r| r.identity.id.clone())
        .collect();
    for id in [&assessment.subject, &assessment.target] {
        if !known.contains(id) {
            result.findings.push(finding(
                ReleaseFindingCode::UnknownReleaseReference,
                id.to_string(),
                "compatibility assessment references an unresolved release",
            ));
        }
    }
    let claim_map: BTreeMap<_, _> = claims
        .iter()
        .map(|claim| (claim.id.clone(), claim))
        .collect();
    for claim_id in &assessment.claims {
        let Some(claim) = claim_map.get(claim_id) else {
            result.findings.push(finding(
                ReleaseFindingCode::MissingEvidenceReference,
                claim_id.to_string(),
                "compatibility assessment references a missing claim",
            ));
            continue;
        };
        if claim.subject != assessment.subject
            || claim.target != assessment.target
            || claim.dimension != assessment.dimension
        {
            result.findings.push(finding(
                ReleaseFindingCode::ConflictingCompatibilityClaim,
                claim.id.to_string(),
                "compatibility claim does not bind to the assessment subject, target, and dimension",
            ));
        }
    }
    if matches!(
        assessment.outcome,
        CompatibilityOutcome::ConditionallyCompatible
    ) && assessment.unresolved_conditions.is_empty()
    {
        result.findings.push(finding(
            ReleaseFindingCode::MissingDeterminationContext,
            assessment.id.to_string(),
            "conditional compatibility requires explicit unresolved conditions",
        ));
    }
    result.valid = result.findings.is_empty();
    result
}

pub fn validate_migration_with_context(
    requirement: &MigrationRequirement,
    plan: Option<&MigrationPlan>,
    context: &ReleaseResolutionContext,
) -> ReleaseValidationResult {
    let mut result = validate_migration(requirement, plan);
    let known: BTreeSet<_> = context
        .releases
        .iter()
        .map(|r| r.identity.id.clone())
        .collect();
    if !known.contains(&requirement.source) {
        result.findings.push(finding(
            ReleaseFindingCode::MissingMigrationSource,
            requirement.source.to_string(),
            "migration source is not present in the bounded release context",
        ));
    }
    if !known.contains(&requirement.target) {
        result.findings.push(finding(
            ReleaseFindingCode::MissingMigrationTarget,
            requirement.target.to_string(),
            "migration target is not present in the bounded release context",
        ));
    }
    if let Some(plan) = plan
        && (plan.requirement != requirement.id || plan.steps.is_empty())
    {
        result.findings.push(finding(
            ReleaseFindingCode::IdentityErasure,
            plan.id.to_string(),
            "migration plan must bind to its requirement and declare at least one step",
        ));
    }
    result.valid = result.findings.is_empty();
    result
}

pub fn validate_release_traceability_chain(
    chain: &ReleaseTraceabilityChain,
    lineage: &[ReleaseRelationship],
    assessments: &[CompatibilityAssessment],
    requirements: &[MigrationRequirement],
    evidence: &[EvidenceRecordId],
) -> ReleaseValidationResult {
    validate_release_traceability_chain_with_activation(
        chain,
        lineage,
        assessments,
        requirements,
        evidence,
        &[],
        &[],
    )
}

pub fn validate_release_traceability_chain_with_activation(
    chain: &ReleaseTraceabilityChain,
    lineage: &[ReleaseRelationship],
    assessments: &[CompatibilityAssessment],
    requirements: &[MigrationRequirement],
    evidence: &[EvidenceRecordId],
    activation: &[String],
    recognition: &[String],
) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    if chain.release != chain.subject {
        findings.push(finding(
            ReleaseFindingCode::IdentityErasure,
            chain.release.to_string(),
            "traceability chain release and subject differ",
        ));
    }
    for id in &chain.lineage {
        if !lineage
            .iter()
            .any(|item| item.id == *id && (item.from == chain.subject || item.to == chain.subject))
        {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                id.to_string(),
                "release chain references missing lineage",
            ));
        }
    }
    for id in &chain.compatibility {
        if !assessments.iter().any(|item| {
            item.id == *id
                && (item.subject == chain.subject || item.target == chain.subject)
                && item.implementation_version == chain.implementation_version
        }) {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                id.to_string(),
                "release chain references missing compatibility assessment",
            ));
        }
    }
    for id in &chain.migration {
        if !requirements.iter().any(|item| {
            item.id == *id && (item.source == chain.subject || item.target == chain.subject)
        }) {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                id.to_string(),
                "release chain references missing migration requirement",
            ));
        }
    }
    for id in &chain.evidence {
        if !evidence.contains(id) {
            findings.push(finding(
                ReleaseFindingCode::MissingEvidenceReference,
                id.to_string(),
                "release chain references missing evidence",
            ));
        }
    }
    for id in &chain.activation {
        if !activation.contains(id) {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                id.clone(),
                "release chain references missing or foreign activation",
            ));
        }
    }
    for id in &chain.recognition {
        if !recognition.contains(id) {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                id.clone(),
                "release chain references missing or foreign recognition",
            ));
        }
    }
    result(findings)
}

pub fn validate_migration_graph(requirements: &[MigrationRequirement]) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    let mut edges = BTreeMap::<ReleaseIdentifier, BTreeSet<ReleaseIdentifier>>::new();
    for requirement in requirements {
        if requirement.source == requirement.target {
            findings.push(finding(
                ReleaseFindingCode::CircularMigration,
                requirement.id.to_string(),
                "migration cannot return to its own release",
            ));
        }
        if !edges
            .entry(requirement.source.clone())
            .or_default()
            .insert(requirement.target.clone())
        {
            findings.push(finding(
                ReleaseFindingCode::CircularMigration,
                requirement.id.to_string(),
                "duplicate migration edge is not independently resolvable",
            ));
        }
    }
    fn visit(
        node: &ReleaseIdentifier,
        edges: &BTreeMap<ReleaseIdentifier, BTreeSet<ReleaseIdentifier>>,
        active: &mut BTreeSet<ReleaseIdentifier>,
        complete: &mut BTreeSet<ReleaseIdentifier>,
    ) -> bool {
        if active.contains(node) {
            return true;
        }
        if !complete.insert(node.clone()) {
            return false;
        }
        active.insert(node.clone());
        let cycle = edges.get(node).is_some_and(|targets| {
            targets
                .iter()
                .any(|target| visit(target, edges, active, complete))
        });
        active.remove(node);
        cycle
    }
    let mut active = BTreeSet::new();
    let mut complete = BTreeSet::new();
    for node in edges.keys() {
        if visit(node, &edges, &mut active, &mut complete) {
            findings.push(finding(
                ReleaseFindingCode::CircularMigration,
                node.to_string(),
                "migration graph contains a cycle",
            ));
        }
    }
    result(findings)
}

pub fn validate_supersession_graph(lineage: &ReleaseLineage) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    let mut edges = BTreeMap::<ReleaseIdentifier, BTreeSet<ReleaseIdentifier>>::new();
    for relationship in &lineage.relationships {
        if relationship.relation == LineageRelation::Supersedes {
            if relationship.from == relationship.to {
                findings.push(finding(
                    ReleaseFindingCode::ImpossibleSupersession,
                    relationship.id.to_string(),
                    "release cannot supersede itself",
                ));
            }
            edges
                .entry(relationship.from.clone())
                .or_default()
                .insert(relationship.to.clone());
            if lineage.relationships.iter().any(|other| {
                other.from == relationship.to
                    && other.to == relationship.from
                    && other.relation == LineageRelation::Supersedes
            }) {
                findings.push(finding(
                    ReleaseFindingCode::ContradictoryLineage,
                    relationship.id.to_string(),
                    "supersession cannot be reciprocal",
                ));
            }
        }
    }
    let graph = validate_migration_graph(
        &edges
            .iter()
            .flat_map(|(source, targets)| {
                targets.iter().map(move |target| MigrationRequirement {
                    id: MigrationRequirementIdentifier::new(format!(
                        "supersession-{source}-{target}"
                    ))
                    .expect("generated identifier"),
                    source: source.clone(),
                    target: target.clone(),
                    requirements: Vec::new(),
                    preconditions: Vec::new(),
                    preserved_properties: Vec::new(),
                    known_losses: Vec::new(),
                    compatibility_effects: Vec::new(),
                    evidence: Vec::new(),
                    verification: Vec::new(),
                    limitations: Vec::new(),
                    unresolved_conditions: Vec::new(),
                    status: MigrationStatus::Declared,
                })
            })
            .collect::<Vec<_>>(),
    );
    findings.extend(graph.findings);
    result(findings)
}

pub fn validate_release_history(entries: &[ReleaseHistoryEntry]) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    let mut ordered = entries.to_vec();
    ordered.sort_by_key(|entry| entry.sequence);
    let mut sequences = BTreeSet::new();
    let mut identities = BTreeMap::new();
    for entry in &ordered {
        if !sequences.insert(entry.sequence) {
            findings.push(finding(
                ReleaseFindingCode::HistoricalRewrite,
                entry.release.id.to_string(),
                "release history sequence is duplicated",
            ));
        }
        if let Some(previous) = identities.insert(entry.release.id.clone(), entry.release.clone())
            && previous != entry.release
        {
            findings.push(finding(
                ReleaseFindingCode::IdentityErasure,
                entry.release.id.to_string(),
                "release identity was rewritten in history",
            ));
        }
        if entry.sequence > 0
            && !ordered
                .iter()
                .any(|candidate| candidate.sequence + 1 == entry.sequence)
        {
            findings.push(finding(
                ReleaseFindingCode::UnknownReleaseReference,
                entry.release.id.to_string(),
                "release history predecessor sequence is missing",
            ));
        }
    }
    result(findings)
}

pub fn validate_release_history_attachments(
    entries: &[ReleaseHistoryEntry],
    attachments: &[ReleaseHistoryAttachment],
    expected_version: &ImplementationVersion,
) -> ReleaseValidationResult {
    let mut findings = validate_release_history(entries).findings;
    for entry in entries {
        for id in entry.migration.iter().map(ToString::to_string) {
            if !attachments.iter().any(|attachment| {
                attachment.id == id
                    && attachment.release == entry.release.id
                    && &attachment.implementation_version == expected_version
            }) {
                findings.push(finding(
                    ReleaseFindingCode::InvalidHistoricalAttachment,
                    id,
                    "release history migration attachment is unresolved or foreign",
                ));
            }
        }
        if entry.sequence > 0 && entry.predecessor.is_none() {
            findings.push(finding(
                ReleaseFindingCode::InvalidHistoricalAttachment,
                entry.release.id.to_string(),
                "non-origin release history entry requires an explicit predecessor",
            ));
        }
    }
    result(findings)
}

/// Validates only the explicit successor applicability and historical
/// references required by IMP-008. It does not require a global graph shape.
pub fn validate_release_history_applicability(
    entries: &[ReleaseHistoryEntry],
    attachments: &[ReleaseHistoryAttachment],
    evidence_applicability: &[EvidenceApplicability],
    assurance_applicability: &[AssuranceApplicability],
    expected_version: &ImplementationVersion,
) -> ReleaseValidationResult {
    let mut findings = validate_release_history(entries).findings;
    let known_releases: BTreeSet<_> = entries
        .iter()
        .map(|entry| entry.release.id.clone())
        .collect();
    for entry in entries {
        if entry.sequence > 0
            && entry
                .predecessor
                .as_ref()
                .is_none_or(|predecessor| !known_releases.contains(predecessor))
        {
            findings.push(finding(
                ReleaseFindingCode::UnknownReleaseReference,
                entry.release.id.to_string(),
                "release history predecessor does not resolve to a preserved release",
            ));
        }
        for evidence in &entry.evidence {
            if !evidence_applicability.iter().any(|applicability| {
                applicability.evidence == *evidence && applicability.release == entry.release.id
            }) {
                findings.push(finding(
                    ReleaseFindingCode::MissingEvidenceReference,
                    evidence.to_string(),
                    "successor release requires an explicit evidence applicability determination",
                ));
            }
        }
        for assurance in &entry.assurance {
            if !assurance_applicability.iter().any(|applicability| {
                applicability.assurance == *assurance && applicability.release == entry.release.id
            }) {
                findings.push(finding(
                    ReleaseFindingCode::MissingAssuranceReference,
                    assurance.clone(),
                    "successor release requires an explicit assurance applicability determination",
                ));
            }
        }
        for migration in &entry.migration {
            if !attachments.iter().any(|attachment| {
                attachment.id == migration.to_string()
                    && attachment.release == entry.release.id
                    && &attachment.implementation_version == expected_version
                    && attachment.applicability.is_some()
            }) {
                findings.push(finding(
                    ReleaseFindingCode::InvalidHistoricalAttachment,
                    migration.to_string(),
                    "successor release migration reference lacks an explicit applicability attachment",
                ));
            }
        }
    }
    result(findings)
}

pub fn validate_release_operational_trace(
    trace: &ReleaseOperationalTrace,
) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    let references = [
        &trace.assignment,
        &trace.activation_request,
        &trace.activation_decision,
        &trace.activation_record,
    ];
    let Some(recognition) = trace.recognition.as_ref() else {
        findings.push(finding(
            ReleaseFindingCode::MissingTraceabilityBinding,
            trace.release.to_string(),
            "operational composition requires a recognition reference",
        ));
        return result(findings);
    };
    for reference in references.into_iter().chain(Some(recognition)) {
        if reference.id.trim().is_empty()
            || reference.authority.is_none()
            || reference.baseline.is_none()
            || reference.scope.is_none()
        {
            findings.push(finding(
                ReleaseFindingCode::MissingTraceabilityBinding,
                reference.id.clone(),
                "operational composition reference requires identity, authority, baseline, and scope",
            ));
        }
        if reference.subject != trace.release || reference.release != trace.release {
            findings.push(finding(
                ReleaseFindingCode::TraceabilitySubjectMismatch,
                reference.id.clone(),
                "operational trace reference is bound to another subject or release",
            ));
        }
        if reference.implementation_version != trace.implementation_version {
            findings.push(finding(
                ReleaseFindingCode::TraceabilityVersionMismatch,
                reference.id.clone(),
                "operational trace reference has a different implementation version",
            ));
        }
        if reference.authority != references[0].authority
            || reference.baseline != references[0].baseline
            || reference.scope != references[0].scope
        {
            findings.push(finding(
                ReleaseFindingCode::TraceabilitySubjectMismatch,
                reference.id.clone(),
                "operational composition reference is not bound to the same authority, baseline, and scope",
            ));
        }
    }
    result(findings)
}

pub fn validate_compatibility_contradictions(
    assessments: &[CompatibilityAssessment],
) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    for (index, left) in assessments.iter().enumerate() {
        for right in assessments.iter().skip(index + 1) {
            if left.subject == right.subject
                && left.target == right.target
                && left.dimension == right.dimension
                && left.direction == right.direction
                && left.scope == right.scope
                && left.criteria == right.criteria
                && left.assumptions == right.assumptions
                && left.baseline == right.baseline
                && left.implementation_context == right.implementation_context
                && left.implementation_version == right.implementation_version
                && ((left.outcome == CompatibilityOutcome::Compatible
                    && right.outcome == CompatibilityOutcome::Incompatible)
                    || (left.outcome == CompatibilityOutcome::Incompatible
                        && right.outcome == CompatibilityOutcome::Compatible))
            {
                findings.push(finding(
                    ReleaseFindingCode::ConflictingCompatibilityClaim,
                    left.id.to_string(),
                    "compatible and incompatible determinations conflict in one context",
                ));
            }
        }
    }
    result(findings)
}

pub fn validate_compatibility(assessment: &CompatibilityAssessment) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    if assessment.subject == assessment.target {
        findings.push(finding(
            ReleaseFindingCode::SelfRelationship,
            assessment.id.to_string(),
            "compatibility requires distinct subject and target releases",
        ));
    }
    if assessment.scope.trim().is_empty()
        || assessment.implementation_context.trim().is_empty()
        || assessment.criteria.is_empty()
    {
        findings.push(finding(
            ReleaseFindingCode::MissingDeterminationContext,
            assessment.id.to_string(),
            "compatibility determination requires scope, context, and criteria",
        ));
    }
    if assessment
        .claims
        .iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != assessment.claims.len()
    {
        findings.push(finding(
            ReleaseFindingCode::DuplicateCompatibilityClaim,
            assessment.id.to_string(),
            "compatibility claims must be unique",
        ));
    }
    ReleaseValidationResult {
        valid: findings.is_empty(),
        findings,
        activation_performed: false,
        authority_created: false,
        migration_executed: false,
        conformance_established: false,
    }
}

pub fn validate_migration(
    requirement: &MigrationRequirement,
    plan: Option<&MigrationPlan>,
) -> ReleaseValidationResult {
    let mut findings = Vec::new();
    if requirement.source == requirement.target {
        findings.push(finding(
            ReleaseFindingCode::SelfRelationship,
            requirement.id.to_string(),
            "migration source and target must differ",
        ));
    }
    if let Some(plan) = plan
        && (plan.source != requirement.source || plan.target != requirement.target)
    {
        findings.push(finding(
            ReleaseFindingCode::IdentityErasure,
            plan.id.to_string(),
            "migration plan must preserve source and target identity",
        ));
    }
    ReleaseValidationResult {
        valid: findings.is_empty(),
        findings,
        activation_performed: false,
        authority_created: false,
        migration_executed: false,
        conformance_established: false,
    }
}

pub fn release_canonical(release: &ReleasedImplementation) -> CanonicalObject {
    let mut fields = vec![
        field(
            "release_id",
            CanonicalValue::Text(release.identity.id.to_string()),
            true,
            true,
        ),
        field(
            "release_version",
            CanonicalValue::Text(release.identity.version.to_string()),
            true,
            true,
        ),
        field(
            "implementation_version",
            CanonicalValue::Text(release.identity.implementation_version.to_string()),
            true,
            true,
        ),
        field(
            "baseline",
            CanonicalValue::Text(release.identity.baseline.to_string()),
            true,
            true,
        ),
        field(
            "release_state",
            CanonicalValue::Text(format!("{:?}", release.state)),
            true,
            false,
        ),
    ];
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    CanonicalObject {
        representation_id: CanonicalRepresentationId::new(format!(
            "release-{}",
            release.identity.id
        ))
        .expect("validated release identity"),
        kind: CanonicalRepresentationKind::ReleaseReference,
        representation_version: REPRESENTATION_MODEL_VERSION.into(),
        schema_id: RELEASE_PROFILE_ID.into(),
        source_binding: None,
        implementation_version: Some(release.identity.implementation_version.clone()),
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
fn finding(code: ReleaseFindingCode, subject: String, detail: &str) -> ReleaseFinding {
    ReleaseFinding {
        code,
        subject,
        detail: detail.into(),
        fatal: true,
    }
}

fn result(findings: Vec<ReleaseFinding>) -> ReleaseValidationResult {
    ReleaseValidationResult {
        valid: findings.is_empty(),
        findings,
        activation_performed: false,
        authority_created: false,
        migration_executed: false,
        conformance_established: false,
    }
}
