use constitutional_canonical::{participation_decode, participation_encode};
use constitutional_contracts::{ContextId, ImplementationVersion};
use constitutional_identity::{
    DomainRef, ParticipationDetermination, ParticipationEvaluationResult,
};

#[test]
fn participation_canonical_encoding_round_trips_identically() {
    let result = ParticipationEvaluationResult {
        evaluation_id: DomainRef::new("eval-1").unwrap(),
        operation_id: DomainRef::new("op-1").unwrap(),
        context_id: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new("reference-foundation-0.5.0").unwrap(),
        profile_version: DomainRef::new("1.0.0").unwrap(),
        claim: None,
        determination: ParticipationDetermination::Indeterminate,
        findings: Vec::new(),
        execution_performed: false,
        participation_activated: false,
        non_claims: vec!["no execution".into()],
    };
    let bytes = participation_encode(&result).unwrap();
    let decoded = participation_decode(&bytes).unwrap();
    let mut reencoded = String::new();
    for (key, value) in decoded.fields {
        reencoded.push_str(&key);
        reencoded.push('=');
        reencoded.push_str(
            &value
                .replace('%', "%25")
                .replace('\n', "%0A")
                .replace('=', "%3D"),
        );
        reencoded.push('\n');
    }
    assert_eq!(reencoded.as_bytes(), bytes.as_slice());
}
