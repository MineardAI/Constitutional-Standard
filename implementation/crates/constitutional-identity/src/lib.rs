//! Bounded, non-sovereign realization of the explicit CORE-002 identity and
//! participation boundary.
//!
//! This crate evaluates explicit claims and bases. It does not authenticate,
//! create identity, create accounts or credentials, create authority, activate
//! participation, or execute constitutional acts.

use constitutional_contracts::{
    ConstitutionalSourceRef, ContextId, ImplementationVersion, SourceSetId,
};
use std::collections::BTreeMap;
use std::fmt;

pub const PROFILE_ID: &str = "reference-implementation-identity-participation";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const GOVERNING_SOURCE: &str = "CORE-002@0.2.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainRef(String);
impl DomainRef {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        if value.is_empty()
            || value
                .chars()
                .any(|c| c.is_control() || c == '\n' || c == '\r')
        {
            return Err(IdentityError::MalformedReference(value));
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

pub type ConstitutionalIdentityId = DomainRef;
pub type IdentityClaimId = DomainRef;
pub type IdentitySubjectRef = DomainRef;
pub type IdentityBasisRef = DomainRef;
pub type IdentityRepresentationRef = DomainRef;
pub type ParticipationId = DomainRef;
pub type ParticipationClaimId = DomainRef;
pub type ParticipationSubjectRef = DomainRef;
pub type ParticipationBasisRef = DomainRef;
pub type AuthorityDeterminationRef = DomainRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBasis {
    pub reference: IdentityBasisRef,
    pub source: ConstitutionalSourceRef,
    pub represented_entity: DomainRef,
    pub identity_class: DomainRef,
    pub recognition_scope: DomainRef,
    pub effective_status: DomainRef,
    pub provenance: DomainRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityDistinction {
    pub distinction_key: DomainRef,
    pub distinct_from: Vec<ConstitutionalIdentityId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContinuityKind {
    Continuity,
    Discontinuity,
    Succession,
    Substitution,
    Replacement,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityContinuityClaim {
    pub prior_identity: ConstitutionalIdentityId,
    pub basis: Option<DomainRef>,
    pub kind: ContinuityKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityClaim {
    pub id: IdentityClaimId,
    pub identity: ConstitutionalIdentityId,
    pub subject: IdentitySubjectRef,
    pub representation: Option<IdentityRepresentationRef>,
    pub basis: Option<IdentityBasis>,
    pub distinction: IdentityDistinction,
    pub continuity: Option<IdentityContinuityClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityFindingCode {
    MissingIdentityClaim,
    MissingIdentitySubject,
    MissingIdentityBasis,
    MissingRepresentation,
    UnsupportedIdentityBasis,
    DuplicateIdentityBasis,
    ConflictingIdentityBasis,
    IdentityCollision,
    AmbiguousIdentityEquivalence,
    SourceNotAdmitted,
    MissingContinuityBasis,
    UnsupportedContinuity,
    ContinuityConflict,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
    RepresentationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityFinding {
    pub code: IdentityFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvaluationRequest {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub claim: Option<IdentityClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub available_sources: Vec<ConstitutionalSourceRef>,
    pub known_claims: Vec<IdentityClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvaluationResult {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: DomainRef,
    pub claim: Option<IdentityClaim>,
    pub determination: IdentityDetermination,
    pub continuity: Option<IdentityContinuityClaim>,
    pub findings: Vec<IdentityFinding>,
    pub authentication_performed: bool,
    pub identity_created: bool,
    pub non_claims: Vec<String>,
}

pub fn validate_identity_claim(claim: Option<&IdentityClaim>) -> Vec<IdentityFinding> {
    let Some(claim) = claim else {
        return vec![finding(
            IdentityFindingCode::MissingIdentityClaim,
            "claim",
            "identity claim is absent",
            true,
        )];
    };
    let mut findings = Vec::new();
    if claim.subject.as_str().is_empty() {
        findings.push(finding(
            IdentityFindingCode::MissingIdentitySubject,
            "subject",
            "identity subject is absent",
            true,
        ));
    }
    if claim.basis.is_none() {
        findings.push(finding(
            IdentityFindingCode::MissingIdentityBasis,
            "basis",
            "identity basis is absent",
            true,
        ));
    }
    findings
}

pub fn evaluate_identity(
    request: &IdentityEvaluationRequest,
    context: &IdentityEvaluationContext,
) -> IdentityEvaluationResult {
    let mut findings = validate_identity_claim(request.claim.as_ref());
    if request.context_id != context.context_id {
        findings.push(finding(
            IdentityFindingCode::ContextMismatch,
            "context_id",
            "evaluation context does not match request",
            true,
        ));
    }
    if request.source_set_id != context.source_set_id {
        findings.push(finding(
            IdentityFindingCode::SourceSetMismatch,
            "source_set_id",
            "source-set identity does not match request",
            true,
        ));
    }
    if request.implementation_version != context.implementation_version {
        findings.push(finding(
            IdentityFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version does not match context",
            true,
        ));
    }
    let mut determination = if findings.iter().any(|item| item.fatal) {
        IdentityDetermination::InvalidRequest
    } else {
        IdentityDetermination::Indeterminate
    };
    let mut continuity = None;
    if determination != IdentityDetermination::InvalidRequest {
        let claim = request.claim.as_ref().expect("validated claim");
        let basis = claim.basis.as_ref().expect("validated basis");
        if !context
            .available_sources
            .iter()
            .any(|source| source == &basis.source)
        {
            findings.push(finding(
                IdentityFindingCode::SourceNotAdmitted,
                basis.source.id.as_str(),
                "identity basis source is not available in the admitted context",
                true,
            ));
            determination = IdentityDetermination::Denied;
        } else if basis.represented_entity.as_str().is_empty()
            || basis.identity_class.as_str().is_empty()
            || basis.recognition_scope.as_str().is_empty()
            || basis.effective_status.as_str().is_empty()
            || basis.provenance.as_str().is_empty()
        {
            findings.push(finding(
                IdentityFindingCode::ConflictingIdentityBasis,
                "basis",
                "mandatory recognition attributes are incomplete",
                true,
            ));
            determination = IdentityDetermination::Denied;
        } else if context.known_claims.iter().any(|other| {
            other.identity == claim.identity
                && other.distinction.distinction_key != claim.distinction.distinction_key
        }) {
            findings.push(finding(
                IdentityFindingCode::IdentityCollision,
                claim.identity.as_str(),
                "identity identifier collides with a distinct subject",
                true,
            ));
            determination = IdentityDetermination::Conflict;
        } else if claim
            .continuity
            .as_ref()
            .is_some_and(|continuity| matches!(continuity.kind, ContinuityKind::Unsupported))
        {
            findings.push(finding(
                IdentityFindingCode::UnsupportedContinuity,
                "continuity",
                "continuity kind is outside this profile",
                true,
            ));
            determination = IdentityDetermination::Indeterminate;
        } else if let Some(value) = &claim.continuity {
            if value.basis.is_none() {
                findings.push(finding(
                    IdentityFindingCode::MissingContinuityBasis,
                    "continuity",
                    "continuity claim lacks an explicit basis",
                    true,
                ));
                determination = IdentityDetermination::Indeterminate;
            } else {
                continuity = Some(value.clone());
            }
        }
        if determination == IdentityDetermination::Indeterminate
            && !findings.iter().any(|item| {
                matches!(
                    item.code,
                    IdentityFindingCode::MissingContinuityBasis
                        | IdentityFindingCode::UnsupportedContinuity
                )
            })
        {
            determination = IdentityDetermination::Supported;
        }
    }
    IdentityEvaluationResult {
        evaluation_id: request.evaluation_id.clone(),
        operation_id: request.operation_id.clone(),
        context_id: request.context_id.clone(),
        source_set_id: request.source_set_id.clone(),
        implementation_version: request.implementation_version.clone(),
        profile_version: DomainRef::new(PROFILE_VERSION).unwrap(),
        claim: request.claim.clone(),
        determination,
        continuity,
        findings,
        authentication_performed: false,
        identity_created: false,
        non_claims: vec![
            "evaluation does not authenticate or create identity".into(),
            "identity does not create participation or authority".into(),
        ],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationScope {
    pub activity: DomainRef,
    pub institution: DomainRef,
    pub process: DomainRef,
    pub exclusions: Vec<DomainRef>,
}
impl ParticipationScope {
    fn contains(&self, requested: &Self) -> bool {
        self.activity == requested.activity
            && self.institution == requested.institution
            && self.process == requested.process
            && requested
                .exclusions
                .iter()
                .all(|item| self.exclusions.contains(item))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParticipationConstraintRequirement {
    MustMatch { key: DomainRef, value: DomainRef },
    Unsupported { name: DomainRef },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationConstraint {
    pub id: DomainRef,
    pub requirement: ParticipationConstraintRequirement,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationBasis {
    pub reference: ParticipationBasisRef,
    pub source: ConstitutionalSourceRef,
    pub relationship: DomainRef,
    pub provenance: DomainRef,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityDeterminationRef {
    pub evaluation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub determination: IdentityDetermination,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationClaim {
    pub id: ParticipationClaimId,
    pub participation: ParticipationId,
    pub subject: ParticipationSubjectRef,
    pub identity: IdentityDeterminationRef,
    pub basis: Option<ParticipationBasis>,
    pub scope: Option<ParticipationScope>,
    pub constraints: Vec<ParticipationConstraint>,
    pub authority_reference: Option<AuthorityDeterminationRef>,
    pub requires_authority: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParticipationDetermination {
    Eligible,
    Admitted,
    NotAdmitted,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParticipationFindingCode {
    MissingParticipationClaim,
    MissingParticipationSubject,
    MissingParticipationBasis,
    MissingParticipationScope,
    UnsupportedParticipationBasis,
    UnsupportedIdentityDetermination,
    IdentityResultMismatch,
    ScopeOutsideBasis,
    UnsatisfiedConstraint,
    UnknownConstraint,
    MissingAuthorityReference,
    AuthorityReferenceMismatch,
    ConflictingParticipationBasis,
    ConflictingParticipationClaim,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationFinding {
    pub code: ParticipationFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationEvaluationRequest {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub claim: Option<ParticipationClaim>,
    pub constraint_facts: BTreeMap<DomainRef, DomainRef>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub available_sources: Vec<ConstitutionalSourceRef>,
    pub participation_basis_scope: Option<ParticipationScope>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipationEvaluationResult {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub profile_version: DomainRef,
    pub claim: Option<ParticipationClaim>,
    pub determination: ParticipationDetermination,
    pub findings: Vec<ParticipationFinding>,
    pub execution_performed: bool,
    pub participation_activated: bool,
    pub non_claims: Vec<String>,
}

pub fn validate_participation_claim(
    claim: Option<&ParticipationClaim>,
) -> Vec<ParticipationFinding> {
    let Some(claim) = claim else {
        return vec![p_finding(
            ParticipationFindingCode::MissingParticipationClaim,
            "claim",
            "participation claim is absent",
            true,
        )];
    };
    let mut findings = Vec::new();
    if claim.subject.as_str().is_empty() {
        findings.push(p_finding(
            ParticipationFindingCode::MissingParticipationSubject,
            "subject",
            "participation subject is absent",
            true,
        ));
    }
    if claim.basis.is_none() {
        findings.push(p_finding(
            ParticipationFindingCode::MissingParticipationBasis,
            "basis",
            "participation basis is absent",
            true,
        ));
    }
    if claim.scope.is_none() {
        findings.push(p_finding(
            ParticipationFindingCode::MissingParticipationScope,
            "scope",
            "participation scope is absent",
            true,
        ));
    }
    findings
}

pub fn evaluate_participation(
    request: &ParticipationEvaluationRequest,
    context: &ParticipationEvaluationContext,
) -> ParticipationEvaluationResult {
    let mut findings = validate_participation_claim(request.claim.as_ref());
    if request.context_id != context.context_id {
        findings.push(p_finding(
            ParticipationFindingCode::ContextMismatch,
            "context_id",
            "participation context does not match request",
            true,
        ));
    }
    if request.source_set_id != context.source_set_id {
        findings.push(p_finding(
            ParticipationFindingCode::SourceSetMismatch,
            "source_set_id",
            "source-set identity does not match request",
            true,
        ));
    }
    if request.implementation_version != context.implementation_version {
        findings.push(p_finding(
            ParticipationFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version does not match context",
            true,
        ));
    }
    let mut determination = if findings.iter().any(|item| item.fatal) {
        ParticipationDetermination::InvalidRequest
    } else {
        ParticipationDetermination::Indeterminate
    };
    if determination != ParticipationDetermination::InvalidRequest {
        let claim = request.claim.as_ref().expect("validated claim");
        let basis = claim.basis.as_ref().expect("validated basis");
        let scope = claim.scope.as_ref().expect("validated scope");
        if !context
            .available_sources
            .iter()
            .any(|source| source == &basis.source)
        {
            findings.push(p_finding(
                ParticipationFindingCode::MissingParticipationBasis,
                "basis",
                "participation basis source is unavailable",
                true,
            ));
            determination = ParticipationDetermination::Denied;
        } else if claim.identity.context_id != request.context_id
            || claim.identity.source_set_id != request.source_set_id
            || claim.identity.implementation_version != request.implementation_version
        {
            findings.push(p_finding(
                ParticipationFindingCode::IdentityResultMismatch,
                "identity",
                "identity determination is bound to another context, source set, or version",
                true,
            ));
            determination = ParticipationDetermination::Denied;
        } else if !matches!(
            claim.identity.determination,
            IdentityDetermination::Supported
        ) {
            findings.push(p_finding(
                ParticipationFindingCode::UnsupportedIdentityDetermination,
                "identity",
                "participation requires a supported identity determination",
                true,
            ));
            determination = ParticipationDetermination::Indeterminate;
        } else if context
            .participation_basis_scope
            .as_ref()
            .is_some_and(|allowed| !allowed.contains(scope))
        {
            findings.push(p_finding(
                ParticipationFindingCode::ScopeOutsideBasis,
                "scope",
                "participation scope exceeds the represented participation basis",
                true,
            ));
            determination = ParticipationDetermination::Denied;
        } else if claim.requires_authority && claim.authority_reference.is_none() {
            findings.push(p_finding(
                ParticipationFindingCode::MissingAuthorityReference,
                "authority",
                "required authority reference is absent",
                true,
            ));
            determination = ParticipationDetermination::Indeterminate;
        } else {
            for constraint in &claim.constraints {
                match &constraint.requirement {
                    ParticipationConstraintRequirement::MustMatch { key, value } => {
                        if request.constraint_facts.get(key) != Some(value) {
                            findings.push(p_finding(
                                ParticipationFindingCode::UnsatisfiedConstraint,
                                key.as_str(),
                                "participation constraint is unsatisfied",
                                true,
                            ));
                            determination = ParticipationDetermination::Denied;
                        }
                    }
                    ParticipationConstraintRequirement::Unsupported { name } => {
                        findings.push(p_finding(
                            ParticipationFindingCode::UnknownConstraint,
                            name.as_str(),
                            "constraint semantics are outside this profile",
                            true,
                        ));
                        determination = ParticipationDetermination::Indeterminate;
                    }
                }
            }
        }
        if determination == ParticipationDetermination::Indeterminate
            && !findings.iter().any(|item| {
                matches!(
                    item.code,
                    ParticipationFindingCode::UnknownConstraint
                        | ParticipationFindingCode::UnsupportedIdentityDetermination
                        | ParticipationFindingCode::MissingAuthorityReference
                )
            })
        {
            determination = ParticipationDetermination::Eligible;
        }
    }
    ParticipationEvaluationResult {
        evaluation_id: request.evaluation_id.clone(),
        operation_id: request.operation_id.clone(),
        context_id: request.context_id.clone(),
        source_set_id: request.source_set_id.clone(),
        implementation_version: request.implementation_version.clone(),
        profile_version: DomainRef::new(PROFILE_VERSION).unwrap(),
        claim: request.claim.clone(),
        determination,
        findings,
        execution_performed: false,
        participation_activated: false,
        non_claims: vec![
            "evaluation does not activate or execute participation".into(),
            "participation does not create authority".into(),
        ],
    }
}

fn finding(code: IdentityFindingCode, subject: &str, detail: &str, fatal: bool) -> IdentityFinding {
    IdentityFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
fn p_finding(
    code: ParticipationFindingCode,
    subject: &str,
    detail: &str,
    fatal: bool,
) -> ParticipationFinding {
    ParticipationFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(v: &str) -> DomainRef {
        DomainRef::new(v).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(
            constitutional_contracts::ConstitutionalSourceId::new("CORE-002").unwrap(),
            constitutional_contracts::ConstitutionalSourceVersion::new("0.2.0").unwrap(),
            "harmonization-v1.0/CORE-002.md",
        )
        .unwrap()
    }
    fn basis() -> IdentityBasis {
        IdentityBasis {
            reference: r("basis-1"),
            source: source(),
            represented_entity: r("entity-a"),
            identity_class: r("Participant Identity"),
            recognition_scope: r("scope-a"),
            effective_status: r("represented-status"),
            provenance: r("provenance-a"),
        }
    }
    fn claim() -> IdentityClaim {
        IdentityClaim {
            id: r("claim-1"),
            identity: r("identity-a"),
            subject: r("subject-a"),
            representation: Some(r("representation-a")),
            basis: Some(basis()),
            distinction: IdentityDistinction {
                distinction_key: r("subject-a"),
                distinct_from: vec![],
            },
            continuity: None,
        }
    }
    fn identity_context() -> IdentityEvaluationContext {
        IdentityEvaluationContext {
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            available_sources: vec![source()],
            known_claims: vec![],
        }
    }
    fn identity_request() -> IdentityEvaluationRequest {
        IdentityEvaluationRequest {
            evaluation_id: r("identity-eval-1"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            claim: Some(claim()),
        }
    }
    #[test]
    fn explicit_basis_supports_identity_without_authentication_or_creation() {
        let result = evaluate_identity(&identity_request(), &identity_context());
        assert_eq!(result.determination, IdentityDetermination::Supported);
        assert!(!result.authentication_performed);
        assert!(!result.identity_created);
    }
    #[test]
    fn missing_basis_is_invalid_request_not_supported() {
        let mut request = identity_request();
        request.claim.as_mut().unwrap().basis = None;
        assert_eq!(
            evaluate_identity(&request, &identity_context()).determination,
            IdentityDetermination::InvalidRequest
        );
    }
    #[test]
    fn identity_collision_is_conflict() {
        let mut context = identity_context();
        let mut other = claim();
        other.distinction.distinction_key = r("different-subject");
        context.known_claims.push(other);
        let result = evaluate_identity(&identity_request(), &context);
        assert_eq!(result.determination, IdentityDetermination::Conflict);
    }
    #[test]
    fn continuity_without_basis_remains_indeterminate() {
        let mut request = identity_request();
        request.claim.as_mut().unwrap().continuity = Some(IdentityContinuityClaim {
            prior_identity: r("old"),
            basis: None,
            kind: ContinuityKind::Continuity,
        });
        assert_eq!(
            evaluate_identity(&request, &identity_context()).determination,
            IdentityDetermination::Indeterminate
        );
    }
    #[test]
    fn representation_does_not_become_identity() {
        let result = evaluate_identity(&identity_request(), &identity_context());
        assert_ne!(result.claim.unwrap().identity, r("representation-a"));
    }
    #[test]
    fn participation_requires_supported_identity_and_explicit_basis_scope() {
        let identity = IdentityDeterminationRef {
            evaluation_id: r("identity-eval-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            determination: IdentityDetermination::Supported,
        };
        let scope = ParticipationScope {
            activity: r("activity"),
            institution: r("institution"),
            process: r("process"),
            exclusions: vec![],
        };
        let claim = ParticipationClaim {
            id: r("pclaim-1"),
            participation: r("participation-1"),
            subject: r("subject-a"),
            identity,
            basis: Some(ParticipationBasis {
                reference: r("pbasis"),
                source: source(),
                relationship: r("Participation Relationship"),
                provenance: r("provenance"),
            }),
            scope: Some(scope.clone()),
            constraints: vec![],
            authority_reference: None,
            requires_authority: false,
        };
        let request = ParticipationEvaluationRequest {
            evaluation_id: r("part-eval"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            claim: Some(claim),
            constraint_facts: BTreeMap::new(),
        };
        let context = ParticipationEvaluationContext {
            context_id: request.context_id.clone(),
            source_set_id: request.source_set_id.clone(),
            implementation_version: request.implementation_version.clone(),
            available_sources: vec![source()],
            participation_basis_scope: Some(scope),
        };
        let result = evaluate_participation(&request, &context);
        assert_eq!(result.determination, ParticipationDetermination::Eligible);
        assert!(!result.execution_performed);
        assert!(!result.participation_activated);
    }
    #[test]
    fn participation_scope_expansion_is_denied() {
        let context_scope = ParticipationScope {
            activity: r("activity"),
            institution: r("institution"),
            process: r("process"),
            exclusions: vec![],
        };
        let mut claim_scope = context_scope.clone();
        claim_scope.activity = r("broader");
        let identity = IdentityDeterminationRef {
            evaluation_id: r("identity-eval-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            determination: IdentityDetermination::Supported,
        };
        let claim = ParticipationClaim {
            id: r("pclaim-1"),
            participation: r("participation-1"),
            subject: r("subject-a"),
            identity,
            basis: Some(ParticipationBasis {
                reference: r("pbasis"),
                source: source(),
                relationship: r("relationship"),
                provenance: r("provenance"),
            }),
            scope: Some(claim_scope),
            constraints: vec![],
            authority_reference: None,
            requires_authority: false,
        };
        let request = ParticipationEvaluationRequest {
            evaluation_id: r("part-eval"),
            operation_id: r("op-1"),
            context_id: ContextId::new("ctx-1").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-1").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.5.0")
                .unwrap(),
            claim: Some(claim),
            constraint_facts: BTreeMap::new(),
        };
        let context = ParticipationEvaluationContext {
            context_id: request.context_id.clone(),
            source_set_id: request.source_set_id.clone(),
            implementation_version: request.implementation_version.clone(),
            available_sources: vec![source()],
            participation_basis_scope: Some(context_scope),
        };
        assert_eq!(
            evaluate_participation(&request, &context).determination,
            ParticipationDetermination::Denied
        );
    }
}
