//! Bounded, non-executing realization of the explicit CORE-001 authority boundary.
//!
//! This crate evaluates an explicitly represented authority assignment. It never
//! derives authority from identity, capability, permissions, source admission, or
//! file presence, and it never executes the claimed act.

use constitutional_contracts::{
    ConstitutionalSourceRef, ContextId, ImplementationVersion, SourceSetId,
};
use std::collections::BTreeMap;
use std::fmt;

pub const PROFILE_ID: &str = "reference-implementation-authority-jurisdiction";
pub const PROFILE_VERSION: &str = "1.0.0";
pub const GOVERNING_SOURCE: &str = "CORE-001@0.3.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainRef(String);
impl DomainRef {
    pub fn new(value: impl Into<String>) -> Result<Self, AuthorityError> {
        let value = value.into();
        if value.is_empty()
            || value
                .chars()
                .any(|c| c.is_control() || c == '\n' || c == '\r')
        {
            return Err(AuthorityError::MalformedReference(value));
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

pub type AuthorityId = DomainRef;
pub type AuthorityHolderRef = DomainRef;
pub type AuthoritySubjectRef = DomainRef;
pub type ConstitutionalActRef = DomainRef;
pub type JurisdictionId = DomainRef;
pub type DelegationId = DomainRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Jurisdiction {
    pub id: JurisdictionId,
    pub source: ConstitutionalSourceRef,
    pub boundary: DomainRef,
    pub subject_matter: DomainRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityScope {
    pub subject: AuthoritySubjectRef,
    pub act: ConstitutionalActRef,
    pub domain: DomainRef,
    pub operational_boundary: DomainRef,
    pub exclusions: Vec<DomainRef>,
}

impl AuthorityScope {
    fn contains(&self, requested: &Self) -> bool {
        self.subject == requested.subject
            && self.act == requested.act
            && self.domain == requested.domain
            && self.operational_boundary == requested.operational_boundary
            && requested
                .exclusions
                .iter()
                .all(|item| self.exclusions.contains(item))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstraintRequirement {
    MustMatch { key: DomainRef, value: DomainRef },
    Unsupported { name: DomainRef },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityConstraint {
    pub id: DomainRef,
    pub requirement: ConstraintRequirement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityAssignment {
    pub id: AuthorityId,
    pub source: ConstitutionalSourceRef,
    pub holder: AuthorityHolderRef,
    pub subject: AuthoritySubjectRef,
    pub act: ConstitutionalActRef,
    pub jurisdiction: Jurisdiction,
    pub scope: AuthorityScope,
    pub constraints: Vec<AuthorityConstraint>,
    pub delegable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delegation {
    pub id: DelegationId,
    pub source_authority: AuthorityId,
    pub from_holder: AuthorityHolderRef,
    pub to_holder: AuthorityHolderRef,
    pub jurisdiction: JurisdictionId,
    pub scope: AuthorityScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityClaim {
    pub authority: AuthorityId,
    pub holder: AuthorityHolderRef,
    pub subject: AuthoritySubjectRef,
    pub act: ConstitutionalActRef,
    pub jurisdiction: JurisdictionId,
    pub scope: AuthorityScope,
    pub delegation: Option<Delegation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityEvaluationRequest {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub claim: Option<AuthorityClaim>,
    pub constraint_facts: BTreeMap<DomainRef, DomainRef>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityEvaluationContext {
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub implementation_version: ImplementationVersion,
    pub assignments: Vec<AuthorityAssignment>,
    pub available_sources: Vec<ConstitutionalSourceRef>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityDetermination {
    Supported,
    Denied,
    Conflict,
    Indeterminate,
    NotApplicable,
    InvalidRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityFindingCode {
    MissingClaim,
    MissingAuthoritySource,
    MissingHolder,
    MissingSubject,
    MissingAct,
    MissingJurisdiction,
    ContextMismatch,
    SourceSetMismatch,
    ImplementationVersionMismatch,
    AuthorityNotFound,
    SourceNotAdmitted,
    HolderMismatch,
    SubjectMismatch,
    ActMismatch,
    JurisdictionMismatch,
    ScopeOutsideAuthority,
    ConstraintUnsatisfied,
    UnknownConstraint,
    DelegationMissingSource,
    DelegationNotAllowed,
    DelegationScopeExpansion,
    DelegationJurisdictionExpansion,
    ConflictingAuthority,
    IndeterminateInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityFinding {
    pub code: AuthorityFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityBasis {
    pub authority: AuthorityId,
    pub source: ConstitutionalSourceRef,
    pub assignment_holder: AuthorityHolderRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityEvaluationResult {
    pub evaluation_id: DomainRef,
    pub operation_id: DomainRef,
    pub context_id: ContextId,
    pub source_set_id: Option<SourceSetId>,
    pub authority_profile_version: DomainRef,
    pub implementation_version: ImplementationVersion,
    pub claim: Option<AuthorityClaim>,
    pub determination: AuthorityDetermination,
    pub basis: Option<AuthorityBasis>,
    pub jurisdiction: Option<JurisdictionId>,
    pub findings: Vec<AuthorityFinding>,
    pub execution_performed: bool,
    pub non_claims: Vec<String>,
}

pub fn validate_authority_claim(claim: Option<&AuthorityClaim>) -> Vec<AuthorityFinding> {
    let Some(claim) = claim else {
        return vec![finding(
            AuthorityFindingCode::MissingClaim,
            "claim",
            "authority claim is absent",
            true,
        )];
    };
    let mut findings = Vec::new();
    if claim.authority.as_str().is_empty() {
        findings.push(finding(
            AuthorityFindingCode::MissingAuthoritySource,
            "authority",
            "authority assignment reference is absent",
            true,
        ));
    }
    if claim.holder.as_str().is_empty() {
        findings.push(finding(
            AuthorityFindingCode::MissingHolder,
            "holder",
            "authority holder reference is absent",
            true,
        ));
    }
    if claim.subject.as_str().is_empty() {
        findings.push(finding(
            AuthorityFindingCode::MissingSubject,
            "subject",
            "authority subject reference is absent",
            true,
        ));
    }
    if claim.act.as_str().is_empty() {
        findings.push(finding(
            AuthorityFindingCode::MissingAct,
            "act",
            "claimed constitutional act reference is absent",
            true,
        ));
    }
    if claim.jurisdiction.as_str().is_empty() {
        findings.push(finding(
            AuthorityFindingCode::MissingJurisdiction,
            "jurisdiction",
            "jurisdiction reference is absent",
            true,
        ));
    }
    findings
}

pub fn evaluate_authority(
    request: &AuthorityEvaluationRequest,
    context: &AuthorityEvaluationContext,
) -> AuthorityEvaluationResult {
    let mut findings = validate_authority_claim(request.claim.as_ref());
    let mut determination = AuthorityDetermination::Indeterminate;
    let mut basis = None;
    if request.context_id != context.context_id {
        findings.push(finding(
            AuthorityFindingCode::ContextMismatch,
            "context_id",
            "evaluation context does not match request",
            true,
        ));
    }
    if request.source_set_id != context.source_set_id {
        findings.push(finding(
            AuthorityFindingCode::SourceSetMismatch,
            "source_set_id",
            "admitted source-set identity does not match request",
            true,
        ));
    }
    if request.implementation_version != context.implementation_version {
        findings.push(finding(
            AuthorityFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "implementation version does not match context",
            true,
        ));
    }
    if findings.iter().any(|f| f.fatal) {
        determination = AuthorityDetermination::InvalidRequest;
    }
    if determination != AuthorityDetermination::InvalidRequest {
        let claim = request.claim.as_ref().expect("validated claim");
        let matches: Vec<_> = context
            .assignments
            .iter()
            .filter(|a| a.id == claim.authority)
            .collect();
        if matches.len() > 1 {
            findings.push(finding(
                AuthorityFindingCode::ConflictingAuthority,
                claim.authority.as_str(),
                "multiple authority assignments share one identifier",
                true,
            ));
            determination = AuthorityDetermination::Conflict;
        } else if let Some(assignment) = matches.first() {
            if !context_source_is_available(
                assignment,
                request.source_set_id.as_ref(),
                &context.available_sources,
            ) {
                findings.push(finding(
                    AuthorityFindingCode::SourceNotAdmitted,
                    assignment.source.id.as_str(),
                    "authority source is not represented by the admitted context",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if assignment.holder != claim.holder {
                findings.push(finding(
                    AuthorityFindingCode::HolderMismatch,
                    "holder",
                    "claim holder differs from assigned holder",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if assignment.subject != claim.subject {
                findings.push(finding(
                    AuthorityFindingCode::SubjectMismatch,
                    "subject",
                    "claim subject differs from assigned subject",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if assignment.act != claim.act {
                findings.push(finding(
                    AuthorityFindingCode::ActMismatch,
                    "act",
                    "claimed act differs from assigned act",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if assignment.jurisdiction.id != claim.jurisdiction {
                findings.push(finding(
                    AuthorityFindingCode::JurisdictionMismatch,
                    "jurisdiction",
                    "claim is outside the assigned jurisdiction",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if !assignment.scope.contains(&claim.scope) {
                findings.push(finding(
                    AuthorityFindingCode::ScopeOutsideAuthority,
                    "scope",
                    "claim scope would expand assigned authority",
                    true,
                ));
                determination = AuthorityDetermination::Denied;
            } else if let Some(delegation) = &claim.delegation {
                if !assignment.delegable {
                    findings.push(finding(
                        AuthorityFindingCode::DelegationNotAllowed,
                        "delegation",
                        "assignment does not declare delegation",
                        true,
                    ));
                    determination = AuthorityDetermination::Denied;
                } else if delegation.source_authority != assignment.id {
                    findings.push(finding(
                        AuthorityFindingCode::DelegationMissingSource,
                        "delegation",
                        "delegation source does not identify the assignment",
                        true,
                    ));
                    determination = AuthorityDetermination::Denied;
                } else if delegation.jurisdiction != assignment.jurisdiction.id {
                    findings.push(finding(
                        AuthorityFindingCode::DelegationJurisdictionExpansion,
                        "delegation",
                        "delegation changes jurisdiction",
                        true,
                    ));
                    determination = AuthorityDetermination::Denied;
                } else if !assignment.scope.contains(&delegation.scope) {
                    findings.push(finding(
                        AuthorityFindingCode::DelegationScopeExpansion,
                        "delegation",
                        "delegation scope exceeds original authority",
                        true,
                    ));
                    determination = AuthorityDetermination::Denied;
                }
            }
            if determination == AuthorityDetermination::Indeterminate {
                let mut unknown_constraint = false;
                for constraint in &assignment.constraints {
                    match &constraint.requirement {
                        ConstraintRequirement::MustMatch { key, value } => {
                            if request.constraint_facts.get(key) != Some(value) {
                                findings.push(finding(
                                    AuthorityFindingCode::ConstraintUnsatisfied,
                                    key.as_str(),
                                    "authority constraint is not satisfied",
                                    true,
                                ));
                                determination = AuthorityDetermination::Denied;
                            }
                        }
                        ConstraintRequirement::Unsupported { name } => {
                            findings.push(finding(
                                AuthorityFindingCode::UnknownConstraint,
                                name.as_str(),
                                "constraint semantics are outside this profile",
                                true,
                            ));
                            unknown_constraint = true;
                        }
                    }
                }
                if unknown_constraint {
                    determination = AuthorityDetermination::Indeterminate;
                }
            }
            if determination == AuthorityDetermination::Indeterminate
                && !findings
                    .iter()
                    .any(|item| item.code == AuthorityFindingCode::UnknownConstraint)
            {
                determination = AuthorityDetermination::Supported;
                basis = Some(AuthorityBasis {
                    authority: assignment.id.clone(),
                    source: assignment.source.clone(),
                    assignment_holder: assignment.holder.clone(),
                });
            }
        } else {
            findings.push(finding(
                AuthorityFindingCode::AuthorityNotFound,
                claim.authority.as_str(),
                "no explicit authority assignment was supplied",
                true,
            ));
            determination = AuthorityDetermination::Denied;
        }
    }
    AuthorityEvaluationResult { evaluation_id: request.evaluation_id.clone(), operation_id: request.operation_id.clone(), context_id: request.context_id.clone(), source_set_id: request.source_set_id.clone(), authority_profile_version: DomainRef::new(PROFILE_VERSION).unwrap(), implementation_version: request.implementation_version.clone(), claim: request.claim.clone(), determination, basis, jurisdiction: request.claim.as_ref().map(|c| c.jurisdiction.clone()), findings, execution_performed: false, non_claims: vec!["evaluation does not execute or activate the claimed act".into(), "evaluation does not establish identity, participation, adoption, conformance, or authority beyond the explicit assignment".into()] }
}

fn context_source_is_available(
    assignment: &AuthorityAssignment,
    source_set_id: Option<&SourceSetId>,
    available_sources: &[ConstitutionalSourceRef],
) -> bool {
    source_set_id.is_some()
        && available_sources
            .iter()
            .any(|source| source == &assignment.source)
}
fn finding(
    code: AuthorityFindingCode,
    subject: &str,
    detail: &str,
    fatal: bool,
) -> AuthorityFinding {
    AuthorityFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityError {
    MalformedReference(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use constitutional_contracts::{ContextId, ImplementationVersion, SourceSetId};

    fn r(value: &str) -> DomainRef {
        DomainRef::new(value).unwrap()
    }
    fn source() -> ConstitutionalSourceRef {
        ConstitutionalSourceRef::new(
            constitutional_contracts::ConstitutionalSourceId::new("CORE-001").unwrap(),
            constitutional_contracts::ConstitutionalSourceVersion::new("0.3.0").unwrap(),
            "harmonization-v1.0/CORE-001.md",
        )
        .unwrap()
    }
    fn assignment() -> AuthorityAssignment {
        AuthorityAssignment {
            id: r("AUTH-001"),
            source: source(),
            holder: r("holder-a"),
            subject: r("subject-a"),
            act: r("act-a"),
            jurisdiction: Jurisdiction {
                id: r("jurisdiction-a"),
                source: source(),
                boundary: r("boundary-a"),
                subject_matter: r("authority"),
            },
            scope: AuthorityScope {
                subject: r("subject-a"),
                act: r("act-a"),
                domain: r("domain-a"),
                operational_boundary: r("operation-a"),
                exclusions: vec![],
            },
            constraints: vec![AuthorityConstraint {
                id: r("constraint-a"),
                requirement: ConstraintRequirement::MustMatch {
                    key: r("mode"),
                    value: r("review"),
                },
            }],
            delegable: true,
        }
    }
    fn request(scope: AuthorityScope) -> AuthorityEvaluationRequest {
        AuthorityEvaluationRequest {
            evaluation_id: r("eval-001"),
            operation_id: r("op-001"),
            context_id: ContextId::new("ctx-001").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-001").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.4.0")
                .unwrap(),
            claim: Some(AuthorityClaim {
                authority: r("AUTH-001"),
                holder: r("holder-a"),
                subject: r("subject-a"),
                act: r("act-a"),
                jurisdiction: r("jurisdiction-a"),
                scope,
                delegation: None,
            }),
            constraint_facts: BTreeMap::from([(r("mode"), r("review"))]),
        }
    }
    fn context() -> AuthorityEvaluationContext {
        AuthorityEvaluationContext {
            context_id: ContextId::new("ctx-001").unwrap(),
            source_set_id: Some(SourceSetId::new("source-set-001").unwrap()),
            implementation_version: ImplementationVersion::new("reference-foundation-0.4.0")
                .unwrap(),
            available_sources: vec![source()],
            assignments: vec![assignment()],
        }
    }
    #[test]
    fn supported_claim_has_basis_and_does_not_execute() {
        let result = evaluate_authority(&request(assignment().scope.clone()), &context());
        assert_eq!(result.determination, AuthorityDetermination::Supported);
        assert!(result.basis.is_some());
        assert!(!result.execution_performed);
    }
    #[test]
    fn missing_assignment_is_denied_not_supported() {
        let mut req = request(assignment().scope.clone());
        req.claim.as_mut().unwrap().authority = r("AUTH-MISSING");
        let result = evaluate_authority(&req, &context());
        assert_eq!(result.determination, AuthorityDetermination::Denied);
    }
    #[test]
    fn scope_expansion_is_denied() {
        let mut scope = assignment().scope.clone();
        scope.domain = r("larger-domain");
        let result = evaluate_authority(&request(scope), &context());
        assert_eq!(result.determination, AuthorityDetermination::Denied);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == AuthorityFindingCode::ScopeOutsideAuthority)
        );
    }
    #[test]
    fn unsupported_constraint_remains_indeterminate() {
        let mut ctx = context();
        ctx.assignments[0].constraints[0].requirement = ConstraintRequirement::Unsupported {
            name: r("future-rule"),
        };
        let result = evaluate_authority(&request(assignment().scope), &ctx);
        assert_eq!(result.determination, AuthorityDetermination::Indeterminate);
    }
    #[test]
    fn delegation_cannot_expand_scope() {
        let mut req = request(assignment().scope.clone());
        let mut delegated = assignment().scope;
        delegated.domain = r("larger-domain");
        req.claim.as_mut().unwrap().delegation = Some(Delegation {
            id: r("DEL-001"),
            source_authority: r("AUTH-001"),
            from_holder: r("holder-a"),
            to_holder: r("holder-b"),
            jurisdiction: r("jurisdiction-a"),
            scope: delegated,
        });
        let result = evaluate_authority(&req, &context());
        assert_eq!(result.determination, AuthorityDetermination::Denied);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.code == AuthorityFindingCode::DelegationScopeExpansion)
        );
    }
    #[test]
    fn context_binding_mismatch_is_invalid_request() {
        let mut req = request(assignment().scope);
        req.context_id = ContextId::new("other").unwrap();
        let result = evaluate_authority(&req, &context());
        assert_eq!(result.determination, AuthorityDetermination::InvalidRequest);
    }

    #[test]
    fn admitted_source_presence_is_required_but_does_not_create_authority() {
        let mut ctx = context();
        ctx.available_sources.clear();
        let result = evaluate_authority(&request(assignment().scope), &ctx);
        assert_eq!(result.determination, AuthorityDetermination::Denied);
        assert!(
            result
                .findings
                .iter()
                .any(|item| item.code == AuthorityFindingCode::SourceNotAdmitted)
        );
    }
}
