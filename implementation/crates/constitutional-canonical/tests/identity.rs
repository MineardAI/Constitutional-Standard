use constitutional_canonical::{identity_decode, identity_encode};
use constitutional_contracts::{ContextId, ImplementationVersion};
use constitutional_identity::{DomainRef, IdentityDetermination, IdentityEvaluationResult};

#[test]
fn identity_canonical_encoding_round_trips_identically() {
    let result = IdentityEvaluationResult {
        evaluation_id: DomainRef::new("eval-1").unwrap(),
        operation_id: DomainRef::new("op-1").unwrap(),
        context_id: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new("reference-foundation-0.5.0").unwrap(),
        profile_version: DomainRef::new("1.0.0").unwrap(),
        claim: None,
        determination: IdentityDetermination::Indeterminate,
        continuity: None,
        findings: Vec::new(),
        authentication_performed: false,
        identity_created: false,
        non_claims: vec!["no authentication".into()],
    };
    let bytes = identity_encode(&result).unwrap();
    let decoded = identity_decode(&bytes).unwrap();
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
