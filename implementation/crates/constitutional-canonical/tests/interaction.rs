use constitutional_canonical::{
    interaction_decode, interaction_evaluation_encode, interaction_identity_encode,
};
use constitutional_contracts::{ContextId, ImplementationVersion, SourceSetId};
use constitutional_interaction::*;

fn r(v: &str) -> Ref {
    Ref::new(v).unwrap()
}
fn identity() -> InteractionIdentity {
    InteractionIdentity {
        id: r("interaction-1"),
        subject: r("subject-1"),
        kind: InteractionKind::Request,
        purpose: r("purpose"),
        scope: r("scope"),
        context: ContextId::new("ctx-1").unwrap(),
        source_set_id: Some(SourceSetId::new("set-1").unwrap()),
        version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
    }
}

#[test]
fn interaction_identity_is_deterministic_and_bound() {
    let bytes = interaction_identity_encode(&identity()).unwrap();
    assert!(interaction_decode(&bytes, "macs-reference-canonical-interaction-identity").is_ok());
    assert_eq!(bytes, interaction_identity_encode(&identity()).unwrap());
}

#[test]
fn interaction_evaluation_rejects_duplicate_fields() {
    let result = InteractionEvaluationResult {
        evaluation_id: r("eval"),
        operation_id: r("op"),
        context_id: ContextId::new("ctx").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new(IMPLEMENTATION_VERSION).unwrap(),
        profile_version: r(PROFILE_VERSION),
        recognition: InteractionRecognitionDetermination::Indeterminate,
        admission: InteractionAdmissionDetermination::NotApplicable,
        support: InteractionSupportDetermination::NotApplicable,
        boundary_recognition: BoundaryRecognitionDetermination::NotApplicable,
        crossing: BoundaryCrossingDetermination::NotApplicable,
        handoff: HandoffDetermination::NotApplicable,
        propagation: PropagationDetermination::NotApplicable,
        containment: ContainmentDetermination::NotApplicable,
        response: ResponseStatusDetermination::NotApplicable,
        projection: ProjectionDetermination::NotApplicable,
        interaction: None,
        boundary: None,
        projected: None,
        findings: vec![],
        execution_performed: false,
        transmission_performed: false,
        receipt_established: false,
        effect_established: false,
        non_claims: vec!["no execution".into()],
    };
    let bytes = interaction_evaluation_encode(&result).unwrap();
    assert!(interaction_decode(&bytes, "macs-reference-canonical-interaction-evaluation").is_ok());
    let duplicate = format!(
        "{}evaluation_id=duplicate\n",
        String::from_utf8(bytes).unwrap()
    );
    assert!(
        interaction_decode(
            duplicate.as_bytes(),
            "macs-reference-canonical-interaction-evaluation"
        )
        .is_err()
    );
}

#[test]
fn unknown_interaction_field_is_rejected() {
    let input = b"format_id=macs-reference-canonical-interaction-identity\nrepresentation_version=1.0.0\nunknown=x\n";
    assert!(interaction_decode(input, "macs-reference-canonical-interaction-identity").is_err());
}
