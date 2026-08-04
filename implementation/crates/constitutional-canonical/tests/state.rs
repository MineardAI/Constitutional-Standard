use constitutional_canonical::{
    state_decode, state_evaluation_encode, state_identity_encode, transition_encode,
};
use constitutional_contracts::{ContextId, ImplementationVersion};
use constitutional_state::{
    DomainRef, InvariantDetermination, PreconditionDetermination, ProjectionDetermination,
    StateDetermination, StateEvaluationResult, StateIdentity, TransitionAdmissionDetermination,
    TransitionSupportDetermination,
};

fn r(value: &str) -> DomainRef {
    DomainRef::new(value).unwrap()
}
fn identity() -> StateIdentity {
    StateIdentity {
        id: r("state-1"),
        subject: r("subject-1"),
        family: r("family-1"),
        dimension: r("dimension-1"),
        domain: r("domain-1"),
        scope: r("scope-1"),
        context: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        version: r("state-v1"),
    }
}

#[test]
fn state_identity_and_transition_canonical_forms_are_bound_and_deterministic() {
    let state_identity = identity();
    assert!(
        state_decode(
            &state_identity_encode(&state_identity).unwrap(),
            "macs-reference-canonical-state-identity"
        )
        .is_ok()
    );
    let transition = constitutional_state::ProposedTransition {
        id: r("transition-1"),
        subject: r("subject-1"),
        source_state: identity(),
        target_state: identity(),
        kind: constitutional_state::TransitionKind::NoOp,
        effects: vec![constitutional_state::TransitionEffect::NoOp],
        authority: None,
        participation: None,
        artifact: None,
        provenance: None,
        preconditions: vec![],
        invariants: vec![],
        constraints: vec![],
        context: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new("reference-foundation-0.12.0").unwrap(),
        evidence: vec![],
    };
    assert!(
        state_decode(
            &transition_encode(&transition).unwrap(),
            "macs-reference-canonical-transition"
        )
        .is_ok()
    );
}

#[test]
fn state_evaluation_canonical_form_rejects_duplicate_fields() {
    let result = StateEvaluationResult {
        evaluation_id: r("eval-1"),
        operation_id: r("op-1"),
        context_id: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new("reference-foundation-0.12.0").unwrap(),
        profile_version: r("1.0.0"),
        state_determination: StateDetermination::Indeterminate,
        admission: TransitionAdmissionDetermination::NotApplicable,
        support: TransitionSupportDetermination::Indeterminate,
        preconditions: PreconditionDetermination::NotApplicable,
        invariants: InvariantDetermination::NotApplicable,
        projection: ProjectionDetermination::NotProjectable,
        state: None,
        transition: None,
        projected_state: None,
        findings: Vec::new(),
        execution_performed: false,
        mutation_performed: false,
        non_claims: vec!["no mutation".into()],
    };
    let valid = state_evaluation_encode(&result).unwrap();
    assert!(state_decode(&valid, "macs-reference-canonical-state-evaluation").is_ok());
    assert!(
        state_decode(
            format!(
                "{}evaluation_id=duplicate\n",
                String::from_utf8(valid).unwrap()
            )
            .as_bytes(),
            "macs-reference-canonical-state-evaluation"
        )
        .is_err()
    );
}
