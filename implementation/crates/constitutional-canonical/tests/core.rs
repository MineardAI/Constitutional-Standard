use constitutional_canonical::{core_evaluation_decode, core_evaluation_encode};
use constitutional_contracts::{ContextId, ImplementationVersion};
use constitutional_core::*;

fn r(v: &str) -> Ref {
    Ref::new(v).unwrap()
}
fn result() -> CoreEvaluationResult {
    CoreEvaluationResult {
        evaluation_id: r("eval"),
        operation_id: r("op"),
        context_id: ContextId::new("ctx").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
        profile_version: r(PROFILE_VERSION),
        integration: IntegrationDetermination::Compatible,
        dependencies: DependencyDetermination::Satisfied,
        invariants: InvariantDetermination::Preserved,
        bindings: BindingCompatibilityDetermination::Compatible,
        domain_results: CoreDomainResults::empty(),
        findings: vec![],
        canonical_identity: r("identity"),
        evidence: vec![],
        execution_performed: false,
        mutation_performed: false,
        effect_established: false,
        non_claims: vec!["no effect".into()],
    }
}

#[test]
fn core_canonical_form_is_stable_and_strict() {
    let bytes = core_evaluation_encode(&result()).unwrap();
    assert!(core_evaluation_decode(&bytes).is_ok());
    assert_eq!(bytes, core_evaluation_encode(&result()).unwrap());
    let duplicate = format!(
        "{}evaluation_id=duplicate\n",
        String::from_utf8(bytes).unwrap()
    );
    assert!(core_evaluation_decode(duplicate.as_bytes()).is_err());
}

#[test]
fn core_canonical_form_rejects_unknown_fields() {
    let input = b"format_id=macs-reference-canonical-core-evaluation\nrepresentation_version=1.0.0\nunknown=x\n";
    assert!(core_evaluation_decode(input).is_err());
}
