use constitutional_activation::*;
use constitutional_canonical::{
    canonical_decode, canonical_encode, canonical_identity_digest, validate_canonical_object,
};
use constitutional_contracts::{EvidenceRecordId, ImplementationVersion};
use constitutional_release::{ReleaseIdentifier, ReleaseVersionIdentifier};

fn request(basis: ActivationInferenceBasis) -> ActivationRequest {
    let release = ReleaseBinding {
        release: ReleaseIdentifier::new("REL-001").unwrap(),
        version: ReleaseVersionIdentifier::new("1.0.0").unwrap(),
    };
    let scope = ActivationScopeIdentifier::new("SCOPE-001").unwrap();
    let environment = ActivationEnvironmentIdentifier::new("ENV-001").unwrap();
    let authority = AuthorityBinding {
        authority_reference: "AUTH-001".into(),
        jurisdiction_reference: "JUR-001".into(),
        scope: scope.clone(),
    };
    ActivationRequest {
        id: ActivationRequestIdentifier::new("REQ-001").unwrap(),
        subject: ActivationSubject {
            id: ActivationSubjectIdentifier::new("SUBJECT-001").unwrap(),
            release: release.clone(),
        },
        target: ActivationTarget {
            id: ActivationTargetIdentifier::new("TARGET-001").unwrap(),
            scope: scope.clone(),
            environment: environment.clone(),
            context: ActivationContextIdentifier::new("CTX-001").unwrap(),
        },
        compatibility_target: None,
        source_set: "SOURCE-SET-001".into(),
        authority: authority.clone(),
        authorization: Some(ActivationAuthorization {
            reference: "AUTHZ-001".into(),
            release,
            environment,
            scope,
            authority,
            status: AuthorizationStatus::Current,
        }),
        evidence: vec![EvidenceRecordId::new("EV-001").unwrap()],
        verification: vec!["VERIFY-001".into()],
        assurance: vec!["ASSURE-001".into()],
        traceability: vec![],
        inference_basis: basis,
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
    }
}

fn eligible(request: &ActivationRequest) -> OperationalEligibilityDetermination {
    OperationalEligibilityDetermination {
        id: EligibilityDeterminationIdentifier::new("ELIG-001").unwrap(),
        request: request.id.clone(),
        outcome: EligibilityOutcome::Eligible,
        prerequisites: vec!["authority reviewed".into()],
        conditions: vec![],
        limitations: vec![],
        findings: vec![],
    }
}
fn authorized(
    request: &ActivationRequest,
    eligibility: &OperationalEligibilityDetermination,
) -> ActivationDecision {
    ActivationDecision {
        id: ActivationDecisionIdentifier::new("DEC-001").unwrap(),
        request: request.id.clone(),
        eligibility: eligibility.id.clone(),
        outcome: ActivationDecisionOutcome::Authorized,
        authorization: request.authorization.clone(),
        conditions: vec![],
        limitations: vec![],
    }
}

#[test]
fn valid_activation_requires_explicit_bindings() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let eligibility = eligible(&request);
    let decision = authorized(&request, &eligibility);
    let result = validate_activation(&request, &eligibility, &decision);
    assert!(result.valid);
    assert!(
        !result.execution_performed && !result.deployment_performed && !result.authority_created
    );
}

#[test]
fn activation_is_not_inferred_from_release_or_runtime_signals() {
    for basis in [
        ActivationInferenceBasis::Publication,
        ActivationInferenceBasis::Deployment,
        ActivationInferenceBasis::Compatibility,
        ActivationInferenceBasis::TestSuccess,
        ActivationInferenceBasis::Evidence,
        ActivationInferenceBasis::Assurance,
    ] {
        let request = request(basis);
        let eligibility = eligible(&request);
        let decision = authorized(&request, &eligibility);
        assert!(!validate_activation(&request, &eligibility, &decision).valid);
    }
}

#[test]
fn authorization_must_match_release_scope_and_environment() {
    let mut request = request(ActivationInferenceBasis::ExplicitRequest);
    request.authorization.as_mut().unwrap().scope =
        ActivationScopeIdentifier::new("OTHER-SCOPE").unwrap();
    let eligibility = eligible(&request);
    let decision = authorized(&request, &eligibility);
    let result = validate_activation(&request, &eligibility, &decision);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::MismatchedAuthorizationScope)
    );
}

#[test]
fn expired_withdrawn_and_self_authorization_are_rejected() {
    let mut request = request(ActivationInferenceBasis::ExplicitRequest);
    request.authorization.as_mut().unwrap().status = AuthorizationStatus::Withdrawn;
    request
        .authorization
        .as_mut()
        .unwrap()
        .authority
        .authority_reference = "SUBJECT-001".into();
    let eligibility = eligible(&request);
    let decision = authorized(&request, &eligibility);
    let result = validate_activation(&request, &eligibility, &decision);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::InvalidAuthorizationStatus)
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::SelfAuthorization)
    );
}

#[test]
fn activation_lifecycle_transitions_are_explicit() {
    assert!(transition_allowed(
        &ActivationState::Proposed,
        &ActivationTransition::Request
    ));
    assert!(transition_allowed(
        &ActivationState::Authorized,
        &ActivationTransition::Activate
    ));
    assert!(transition_allowed(
        &ActivationState::Activated,
        &ActivationTransition::Suspend
    ));
    assert!(transition_allowed(
        &ActivationState::Deactivated,
        &ActivationTransition::MarkEligibleForReactivation
    ));
    assert!(!transition_allowed(
        &ActivationState::Proposed,
        &ActivationTransition::Activate
    ));
    assert!(!transition_allowed(
        &ActivationState::Activated,
        &ActivationTransition::Reactivate
    ));
}

#[test]
fn recognition_is_independent_and_requires_activation_record() {
    let request = RecognitionRequest {
        id: RecognitionRequestIdentifier::new("REC-REQ-001").unwrap(),
        subject: RecognitionSubject {
            id: RecognitionSubjectIdentifier::new("REC-SUBJECT-001").unwrap(),
            release: request(ActivationInferenceBasis::ExplicitRequest)
                .subject
                .release,
            activation: None,
        },
        scope: ActivationScopeIdentifier::new("SCOPE-001").unwrap(),
        context: ActivationContextIdentifier::new("CTX-001").unwrap(),
        criteria: vec![RecognitionCriteriaIdentifier::new("CRIT-001").unwrap()],
        authority: AuthorityBinding {
            authority_reference: "AUTH-001".into(),
            jurisdiction_reference: "JUR-001".into(),
            scope: ActivationScopeIdentifier::new("SCOPE-001").unwrap(),
        },
        evidence: vec![],
        verification: vec![],
        assurance: vec![],
        traceability: vec![],
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
    };
    let decision = RecognitionDecision {
        id: RecognitionDecisionIdentifier::new("REC-DEC-001").unwrap(),
        request: request.id.clone(),
        subject: request.subject.id.clone(),
        scope: request.scope.clone(),
        outcome: RecognitionDecisionOutcome::Recognized,
        findings: vec![],
        conditions: vec![],
        limitations: vec![],
        authority: request.authority.clone(),
        jurisdiction: "JUR-001".into(),
        implementation_version: request.implementation_version.clone(),
    };
    let result = validate_recognition(&request, &decision);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::RecognitionWithoutActivation)
    );
}

#[test]
fn recognition_does_not_create_truth_conformance_or_certification() {
    let activation = ActivationRecordIdentifier::new("ACT-REC-001").unwrap();
    let recognition = OperationalRecognition {
        id: RecognitionRecordIdentifier::new("REC-001").unwrap(),
        subject: RecognitionSubject {
            id: RecognitionSubjectIdentifier::new("SUBJECT-001").unwrap(),
            release: request(ActivationInferenceBasis::ExplicitRequest)
                .subject
                .release,
            activation: Some(activation),
        },
        scope: ActivationScopeIdentifier::new("SCOPE-001").unwrap(),
        state: RecognitionState::OperationallyRecognized,
        decision: RecognitionDecisionIdentifier::new("REC-DEC-001").unwrap(),
        historical: false,
        truth_claimed: true,
        conformance_claimed: true,
        certification_claimed: true,
        conditions: vec![],
        limitations: vec![],
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
    };
    let result = validate_recognition_claims(&recognition);
    assert!(
        !result.valid
            && !result.authority_created
            && !result.conformance_established
            && !result.certification_established
    );
}

#[test]
fn canonical_activation_projection_is_stable() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let canonical = activation_canonical(
        &request.subject,
        &request.target,
        &ActivationState::Requested,
    );
    assert!(validate_canonical_object(&canonical).valid);
    assert_eq!(
        canonical_identity_digest(&canonical),
        canonical_identity_digest(&canonical)
    );
}

#[test]
fn missing_context_and_authorization_fail_closed() {
    let mut request = request(ActivationInferenceBasis::ExplicitRequest);
    request.source_set.clear();
    request.authorization = None;
    let eligibility = eligible(&request);
    let decision = ActivationDecision {
        id: ActivationDecisionIdentifier::new("DEC-INVALID").unwrap(),
        request: request.id.clone(),
        eligibility: eligibility.id.clone(),
        outcome: ActivationDecisionOutcome::Authorized,
        authorization: None,
        conditions: vec![],
        limitations: vec![],
    };
    let result = validate_activation(&request, &eligibility, &decision);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::MissingSourceSet)
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::MissingAuthorization)
    );
}

fn standing(request: &ActivationRequest, state: ActivationState) -> OperationalStanding {
    OperationalStanding {
        assignment: OperationalAssignmentIdentifier::new("ASSIGN-001").unwrap(),
        release: request.subject.release.clone(),
        scope: request.target.scope.clone(),
        environment: request.target.environment.clone(),
        state,
        limitations: vec![],
        historical: true,
    }
}

fn history_record(
    request: &ActivationRequest,
    sequence: u64,
    transition: ActivationTransition,
    prior: Option<OperationalStanding>,
    resulting: Option<OperationalStanding>,
) -> HistoricalTransition {
    HistoricalTransition {
        id: ActivationRecordIdentifier::new(format!("HIST-{sequence}")).unwrap(),
        sequence,
        transition,
        prior_standing: prior,
        resulting_standing: resulting,
        subject: request.subject.clone(),
        target: request.target.clone(),
        decision: ActivationDecisionIdentifier::new("DEC-HISTORY").unwrap(),
        authorization: request.authorization.clone(),
        evidence: vec![],
        assurance: vec![],
        traceability: vec![],
        limitations: vec![],
        conditions: vec![],
        implementation_version: request.implementation_version.clone(),
    }
}

#[test]
fn lifecycle_history_is_ordered_and_continuous() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let first = history_record(
        &request,
        0,
        ActivationTransition::Request,
        None,
        Some(standing(&request, ActivationState::Requested)),
    );
    let second = history_record(
        &request,
        1,
        ActivationTransition::DetermineEligibility,
        Some(standing(&request, ActivationState::Requested)),
        Some(standing(&request, ActivationState::Eligible)),
    );
    assert!(validate_transition_history(&[second, first]).valid);
}

#[test]
fn lifecycle_history_rejects_gaps_and_standing_substitution() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let first = history_record(
        &request,
        0,
        ActivationTransition::Request,
        None,
        Some(standing(&request, ActivationState::Requested)),
    );
    let mut third = history_record(
        &request,
        2,
        ActivationTransition::DetermineEligibility,
        Some(standing(&request, ActivationState::Eligible)),
        Some(standing(&request, ActivationState::Eligible)),
    );
    third.subject.id = ActivationSubjectIdentifier::new("SUBJECT-OTHER").unwrap();
    let result = validate_transition_history(&[first, third]);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::MissingPredecessor)
    );
}

#[test]
fn rollback_cannot_masquerade_as_restoration() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let prior = standing(&request, ActivationState::Activated);
    let result = validate_transition_history(&[history_record(
        &request,
        0,
        ActivationTransition::Rollback,
        Some(prior.clone()),
        Some(prior),
    )]);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ActivationFindingCode::RollbackRestoration)
    );
}

#[test]
fn recognition_context_validates_findings_conditions_versions_and_round_trips() {
    let activation_request = request(ActivationInferenceBasis::ExplicitRequest);
    let recognition_request = RecognitionRequest {
        id: RecognitionRequestIdentifier::new("REC-REQ-VALID").unwrap(),
        subject: RecognitionSubject {
            id: RecognitionSubjectIdentifier::new("SUBJECT-001").unwrap(),
            release: activation_request.subject.release.clone(),
            activation: Some(ActivationRecordIdentifier::new("ACT-VALID").unwrap()),
        },
        scope: activation_request.target.scope.clone(),
        context: activation_request.target.context.clone(),
        criteria: vec![RecognitionCriteriaIdentifier::new("CRIT-VALID").unwrap()],
        authority: activation_request.authority.clone(),
        evidence: vec![],
        verification: vec![],
        assurance: vec![],
        traceability: vec![],
        implementation_version: activation_request.implementation_version.clone(),
    };
    let finding = RecognitionFinding {
        id: RecognitionFindingIdentifier::new("FIND-VALID").unwrap(),
        subject: recognition_request.subject.id.clone(),
        criteria: recognition_request.criteria[0].clone(),
        statement: "criterion satisfied".into(),
        limitation: None,
        status: RecognitionFindingStatus::Satisfied,
        implementation_version: recognition_request.implementation_version.clone(),
    };
    let decision = RecognitionDecision {
        id: RecognitionDecisionIdentifier::new("REC-DEC-VALID").unwrap(),
        request: recognition_request.id.clone(),
        subject: recognition_request.subject.id.clone(),
        scope: recognition_request.scope.clone(),
        outcome: RecognitionDecisionOutcome::Recognized,
        findings: vec![finding.id.clone()],
        conditions: vec![],
        limitations: vec!["bounded reference use".into()],
        authority: recognition_request.authority.clone(),
        jurisdiction: "JUR-001".into(),
        implementation_version: recognition_request.implementation_version.clone(),
    };
    let context = RecognitionResolutionContext {
        authorities: vec![AuthorityResolution {
            reference: "AUTH-001".into(),
            jurisdiction: "JUR-001".into(),
            subject: activation_request.subject.id.clone(),
            scope: recognition_request.scope.clone(),
            environment: activation_request.target.environment.clone(),
            valid: true,
        }],
        criteria: recognition_request.criteria.clone(),
        implementation_version: recognition_request.implementation_version.clone(),
        findings: vec![finding.id.clone()],
        conditions: vec![],
        limitations: vec!["bounded reference use".into()],
        traceability_version: recognition_request.implementation_version.clone(),
        required_criteria: vec![RecognitionCriteria {
            id: recognition_request.criteria[0].clone(),
            statement: "criterion satisfied".into(),
            required: true,
            implementation_version: recognition_request.implementation_version.clone(),
        }],
        finding_records: vec![finding.clone()],
    };
    assert!(validate_recognition_with_context(&recognition_request, &decision, &context).valid);
    let recognition = OperationalRecognition {
        id: RecognitionRecordIdentifier::new("REC-RECORD-VALID").unwrap(),
        subject: recognition_request.subject.clone(),
        scope: recognition_request.scope.clone(),
        state: RecognitionState::OperationallyRecognized,
        decision: decision.id.clone(),
        historical: false,
        truth_claimed: false,
        conformance_claimed: false,
        certification_claimed: false,
        conditions: decision.conditions.clone(),
        limitations: decision.limitations.clone(),
        implementation_version: decision.implementation_version.clone(),
    };
    assert!(
        validate_recognition_record(&recognition, &recognition_request, &decision, &[finding])
            .valid
    );
    let encoded = canonical_encode(&recognition_canonical(&recognition)).unwrap();
    let decoded = canonical_decode(&encoded.bytes).unwrap();
    assert_eq!(
        encoded.bytes,
        canonical_encode(&decoded.object).unwrap().bytes
    );
}

#[test]
fn activation_traceability_rejects_cross_release_or_version_bindings() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let eligibility = eligible(&request);
    let decision = ActivationDecision {
        id: ActivationDecisionIdentifier::new("DEC-TRACE").unwrap(),
        request: request.id.clone(),
        eligibility: eligibility.id.clone(),
        outcome: ActivationDecisionOutcome::Authorized,
        authorization: request.authorization.clone(),
        conditions: vec![],
        limitations: vec![],
    };
    let act = ActivationAct {
        id: ActivationActIdentifier::new("ACT-TRACE").unwrap(),
        decision: decision.id.clone(),
        subject: request.subject.clone(),
        target: request.target.clone(),
        state: ActivationState::Activated,
        executed: false,
    };
    let record = ActivationRecord {
        id: ActivationRecordIdentifier::new("REC-TRACE").unwrap(),
        act: act.id.clone(),
        assignment: OperationalAssignment {
            id: OperationalAssignmentIdentifier::new("ASSIGN-TRACE").unwrap(),
            subject: request.subject.clone(),
            role: "bounded-role".into(),
            operational_baseline: OperationalBaselineIdentifier::new("BASE-TRACE").unwrap(),
            deployment_context: request.target.context.clone(),
            recognition_authority: request.authority.clone(),
            scope: request.target.scope.clone(),
        },
        historical: true,
        runtime_execution_claimed: false,
        implementation_version: request.implementation_version.clone(),
    };
    let binding = ActivationTraceabilityBinding {
        request: request.id.clone(),
        decision: decision.id.clone(),
        act: act.id.clone(),
        record: record.id.clone(),
        recognition: None,
        implementation_version: request.implementation_version.clone(),
    };
    assert!(validate_activation_traceability(&binding, &request, &decision, &record, None).valid);
    let mut foreign = binding;
    foreign.implementation_version =
        ImplementationVersion::new("reference-foundation-0.14.0").unwrap();
    assert!(!validate_activation_traceability(&foreign, &request, &decision, &record, None).valid);
}

fn standing_for(state: ActivationState, assignment: &str) -> OperationalStanding {
    OperationalStanding {
        assignment: OperationalAssignmentIdentifier::new(assignment).unwrap(),
        release: ReleaseBinding {
            release: ReleaseIdentifier::new("REL-001").unwrap(),
            version: ReleaseVersionIdentifier::new("1.0.0").unwrap(),
        },
        scope: ActivationScopeIdentifier::new("SCOPE-001").unwrap(),
        environment: ActivationEnvironmentIdentifier::new("ENV-001").unwrap(),
        state,
        limitations: vec![],
        historical: true,
    }
}

#[test]
fn bounded_transition_results_reject_invalid_success_and_accept_failure() {
    let subject = ActivationSubject {
        id: ActivationSubjectIdentifier::new("SUBJECT-001").unwrap(),
        release: ReleaseBinding {
            release: ReleaseIdentifier::new("REL-001").unwrap(),
            version: ReleaseVersionIdentifier::new("1.0.0").unwrap(),
        },
    };
    let invalid = HistoricalTransition {
        id: ActivationRecordIdentifier::new("HIST-1").unwrap(),
        sequence: 0,
        transition: ActivationTransition::Fail,
        prior_standing: Some(standing_for(ActivationState::Authorized, "ASSIGN-1")),
        resulting_standing: Some(standing_for(ActivationState::Activated, "ASSIGN-1")),
        subject: subject.clone(),
        target: ActivationTarget {
            id: ActivationTargetIdentifier::new("TARGET-001").unwrap(),
            scope: ActivationScopeIdentifier::new("SCOPE-001").unwrap(),
            environment: ActivationEnvironmentIdentifier::new("ENV-001").unwrap(),
            context: ActivationContextIdentifier::new("CTX-001").unwrap(),
        },
        decision: ActivationDecisionIdentifier::new("DEC-1").unwrap(),
        authorization: None,
        evidence: vec![],
        assurance: vec![],
        traceability: vec![],
        limitations: vec![],
        conditions: vec![],
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
    };
    let mut valid = invalid.clone();
    valid.id = ActivationRecordIdentifier::new("HIST-2").unwrap();
    valid.resulting_standing = Some(standing_for(ActivationState::ActivationFailed, "ASSIGN-1"));
    assert!(!validate_transition_history(&[invalid]).valid);
    assert!(validate_transition_history(&[valid]).valid);
}

#[test]
fn recognition_history_and_corrections_preserve_typed_links() {
    let subject = RecognitionSubject {
        id: RecognitionSubjectIdentifier::new("RECOG-SUBJECT").unwrap(),
        release: ReleaseBinding {
            release: ReleaseIdentifier::new("REL-001").unwrap(),
            version: ReleaseVersionIdentifier::new("1.0.0").unwrap(),
        },
        activation: Some(ActivationRecordIdentifier::new("ACT-RECORD").unwrap()),
    };
    let transition = RecognitionStandingTransition {
        id: RecognitionRecordIdentifier::new("RECOG-TRANSITION").unwrap(),
        prior: RecognitionState::OperationallyRecognized,
        resulting: RecognitionState::RecognitionSuspended,
        subject: subject.clone(),
        decision: RecognitionDecisionIdentifier::new("DECISION").unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
        historical: true,
    };
    assert!(validate_recognition_standing_history(&[transition]).valid);
    let correction = LifecycleCorrection {
        replacement: ActivationRecordIdentifier::new("REPLACEMENT").unwrap(),
        preserved_from: ActivationRecordIdentifier::new("ORIGINAL").unwrap(),
        subject: ActivationSubject {
            id: ActivationSubjectIdentifier::new("SUBJECT-001").unwrap(),
            release: subject.release,
        },
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
        historical: true,
    };
    assert!(validate_lifecycle_corrections(&[correction]).valid);
}

#[test]
fn assignment_history_preserves_bound_fields_and_allows_only_explicit_transfer() {
    let request = request(ActivationInferenceBasis::ExplicitRequest);
    let assignment = |id: &str| OperationalAssignment {
        id: OperationalAssignmentIdentifier::new(id).unwrap(),
        subject: request.subject.clone(),
        role: "bounded-role".into(),
        operational_baseline: OperationalBaselineIdentifier::new("BASE-001").unwrap(),
        deployment_context: request.target.context.clone(),
        recognition_authority: request.authority.clone(),
        scope: request.target.scope.clone(),
    };
    let first = history_record(
        &request,
        0,
        ActivationTransition::Request,
        None,
        Some(standing(&request, ActivationState::Requested)),
    );
    let mut transferred = history_record(
        &request,
        1,
        ActivationTransition::Transfer,
        Some(standing(&request, ActivationState::Requested)),
        Some(standing(&request, ActivationState::Activated)),
    );
    transferred.prior_standing.as_mut().unwrap().assignment =
        OperationalAssignmentIdentifier::new("ASSIGN-001").unwrap();
    transferred.resulting_standing.as_mut().unwrap().assignment =
        OperationalAssignmentIdentifier::new("ASSIGN-002").unwrap();
    assert!(
        validate_assignment_history(
            &[first.clone(), transferred],
            &[assignment("ASSIGN-001"), assignment("ASSIGN-002")],
            &request.implementation_version,
        )
        .valid
    );

    let mut substituted = history_record(
        &request,
        1,
        ActivationTransition::DetermineEligibility,
        Some(standing(&request, ActivationState::Requested)),
        Some(standing(&request, ActivationState::Eligible)),
    );
    substituted.prior_standing.as_mut().unwrap().assignment =
        OperationalAssignmentIdentifier::new("ASSIGN-002").unwrap();
    assert!(
        !validate_assignment_history(
            &[first, substituted],
            &[assignment("ASSIGN-001"), assignment("ASSIGN-002")],
            &request.implementation_version,
        )
        .valid
    );
}
