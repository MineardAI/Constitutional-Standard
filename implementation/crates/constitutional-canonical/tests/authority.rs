use constitutional_authority::{AuthorityDetermination, AuthorityEvaluationResult, DomainRef};
use constitutional_canonical::{authority_decode, authority_encode};
use constitutional_contracts::{ContextId, ImplementationVersion};

#[test]
fn authority_canonical_encoding_round_trips_identically() {
    let result = AuthorityEvaluationResult {
        evaluation_id: DomainRef::new("eval-001").unwrap(),
        operation_id: DomainRef::new("op-001").unwrap(),
        context_id: ContextId::new("ctx-001").unwrap(),
        source_set_id: None,
        authority_profile_version: DomainRef::new("1.0.0").unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.4.0").unwrap(),
        claim: None,
        determination: AuthorityDetermination::Indeterminate,
        basis: None,
        jurisdiction: None,
        findings: Vec::new(),
        execution_performed: false,
        non_claims: vec!["no execution".into()],
    };
    let bytes = authority_encode(&result).unwrap();
    let decoded = authority_decode(&bytes).unwrap();
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

#[test]
fn authority_canonical_decoder_rejects_unknown_duplicate_and_unsupported_fields() {
    let result = AuthorityEvaluationResult {
        evaluation_id: DomainRef::new("eval-001").unwrap(),
        operation_id: DomainRef::new("op-001").unwrap(),
        context_id: ContextId::new("ctx-001").unwrap(),
        source_set_id: None,
        authority_profile_version: DomainRef::new("1.0.0").unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.4.0").unwrap(),
        claim: None,
        determination: AuthorityDetermination::Denied,
        basis: None,
        jurisdiction: None,
        findings: Vec::new(),
        execution_performed: false,
        non_claims: Vec::new(),
    };
    let valid = String::from_utf8(authority_encode(&result).unwrap()).unwrap();
    assert!(authority_decode(format!("{valid}unknown=x\n").as_bytes()).is_err());
    assert!(authority_decode(format!("{valid}format_id=x\n").as_bytes()).is_err());
    assert!(
        authority_decode(
            valid
                .replace(
                    "representation_version=1.0.0",
                    "representation_version=9.9.9"
                )
                .as_bytes()
        )
        .is_err()
    );
}
