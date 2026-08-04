use constitutional_canonical::{
    CanonicalDecodeError, FORMAT_ID, FORMAT_VERSION, canonical_context, canonicalize, decode,
    encode, encode_representation, non_authoritative_digest, round_trip,
};
use constitutional_context::SourceCatalog;
use constitutional_contracts::*;
use constitutional_evidence::EvidenceRecord;
use constitutional_test_support::{
    complete_catalog, complete_request, complete_request_with_version, source,
};
use constitutional_traceability::ImplementationMapping;
use constitutional_validation::validate_context;

fn mapping(version: &str) -> ImplementationMapping {
    ImplementationMapping {
        mapping_id: ImplementationMappingId::new("MAP-CTX-001").unwrap(),
        requirement: RequirementRef {
            source: source(
                "IMP-000",
                "0.2.0",
                "harmonization-v1.0/IMP-000_Implementation_Constitution_v0.2.0_Candidate_Review_Draft.md",
            ),
            requirement_id: RequirementId::new("IMP-000-TR-001").unwrap(),
            locator: "§9.2 Bidirectional Traceability".into(),
        },
        requirement_class: "structural-and-determinism".into(),
        constitutional_owner: "IMP-000/IMP-004".into(),
        implementation_responsibility: "Resolve and preserve operation-scoped context".into(),
        implementing_component: "constitutional-context".into(),
        implementation_reference: "crates/constitutional-context/src/lib.rs".into(),
        verification_obligation:
            "Repeated canonical input resolves identically and unresolved input fails closed".into(),
        test_reference: "vertical_slice::positive_and_negative_context_cases".into(),
        evidence_reference: EvidenceRecordId::new("EV-CTX-0001").unwrap(),
        implementation_version: ImplementationVersion::new(version).unwrap(),
        status: "implemented-slice".into(),
        known_limitations: "Fixture catalog only; no full specification parser or registry".into(),
        unresolved_issues: "No constitutional conformance conclusion is produced".into(),
    }
}

#[test]
fn positive_flow_is_deterministic_and_recoverable() {
    let request = complete_request_with_version("reference-foundation-0.1.0");
    let first = complete_catalog().resolve(&request);
    let second = complete_catalog().resolve(&request);
    let (first, second) = match (first, second) {
        (ContextResolutionResult::Resolved(a), ContextResolutionResult::Resolved(b)) => (a, b),
        _ => panic!("fixture must resolve"),
    };
    assert_eq!(first, second);
    assert_eq!(canonical_context(&first), canonical_context(&second));
    assert_eq!(
        non_authoritative_digest(&canonical_context(&first)),
        non_authoritative_digest(&canonical_context(&second))
    );
    assert_eq!(
        first
            .source(&ConstitutionalSourceId::new("AFD-002").unwrap())
            .unwrap()
            .version
            .as_str(),
        "1.0.0"
    );
    let validation = validate_context(&first);
    assert!(validation.valid);
    assert!(!validation.authority_established);
    assert!(!validation.conformance_claim);
    assert!(!validation.certification_claim);
    let evidence = EvidenceRecord::for_mapping(
        EvidenceRecordId::new("EV-CTX-0001").unwrap(),
        "context-fixture",
        &first,
        &mapping("reference-foundation-0.1.0"),
        "vertical_slice::positive_flow_is_deterministic_and_recoverable",
        "pass",
    )
    .unwrap();
    assert_eq!(
        evidence.implementation_version,
        first.implementation_version
    );
    assert_eq!(evidence.representation_format.as_str(), FORMAT_ID);
    assert_eq!(evidence.representation_version.as_str(), FORMAT_VERSION);
    assert!(evidence.conclusion.is_none());
}

#[test]
fn canonical_profile_round_trips_and_reencodes_identically() {
    let ContextResolutionResult::Resolved(context) =
        complete_catalog().resolve(&complete_request())
    else {
        panic!("fixture must resolve")
    };
    let bytes = encode(&context).unwrap();
    let decoded = decode(&bytes).unwrap();
    assert_eq!(decoded.to_context(), *context);
    assert_eq!(encode_representation(&decoded).unwrap(), bytes);
    assert!(round_trip(&context).is_ok());
    assert_eq!(decoded.context_id, None);
    assert_eq!(decoded.implementation_profile, None);
}

#[test]
fn canonical_order_is_independent_of_source_construction_order() {
    let request = complete_request();
    let first = complete_catalog().resolve(&request);
    let reversed_catalog = SourceCatalog::new(vec![
        source(
            "IMP-000",
            "0.2.0",
            "harmonization-v1.0/IMP-000_Implementation_Constitution_v0.2.0_Candidate_Review_Draft.md",
        ),
        source(
            "AFD-002",
            "1.0.0",
            "harmonization-v1.0/AFD-002_Implementation_Architecture_Freeze_Declaration_v1.0.md",
        ),
    ]);
    let second = reversed_catalog.resolve(&request);
    let (ContextResolutionResult::Resolved(first), ContextResolutionResult::Resolved(second)) =
        (first, second)
    else {
        panic!("fixtures must resolve")
    };
    assert_eq!(encode(&first).unwrap(), encode(&second).unwrap());
}

#[test]
fn strict_decode_rejects_unknown_duplicate_missing_unsupported_and_trailing_input() {
    let ContextResolutionResult::Resolved(context) =
        complete_catalog().resolve(&complete_request())
    else {
        panic!("fixture must resolve")
    };
    let valid = String::from_utf8(encode(&context).unwrap()).unwrap();
    assert!(matches!(
        decode(format!("{valid}unknown=x\n").as_bytes()),
        Err(CanonicalDecodeError::UnknownField(_))
    ));
    assert!(matches!(
        decode(format!("{valid}format_id=x\n").as_bytes()),
        Err(CanonicalDecodeError::DuplicateField(_))
    ));
    let missing = valid.replace("scope=context-vertical-slice\n", "");
    assert!(matches!(
        decode(missing.as_bytes()),
        Err(CanonicalDecodeError::MissingRequiredField(_))
    ));
    let unsupported = valid.replace(
        "representation_version=1.0.0",
        "representation_version=9.9.9",
    );
    assert!(matches!(
        decode(unsupported.as_bytes()),
        Err(CanonicalDecodeError::UnsupportedVersion(_))
    ));
    let unsupported_format = valid.replace(
        "format_id=macs-reference-canonical-context",
        "format_id=other-profile",
    );
    assert!(matches!(
        decode(unsupported_format.as_bytes()),
        Err(CanonicalDecodeError::UnsupportedFormat(_))
    ));
    assert!(matches!(
        decode(format!("{valid}\n").as_bytes()),
        Err(CanonicalDecodeError::TrailingData)
    ));
    assert!(matches!(
        decode(&[0xff, 0xfe, b'\n']),
        Err(CanonicalDecodeError::InvalidUtf8)
    ));
}

#[test]
fn representation_validation_rejects_duplicate_and_conflicting_sources() {
    let ContextResolutionResult::Resolved(context) =
        complete_catalog().resolve(&complete_request())
    else {
        panic!("fixture must resolve")
    };
    let mut representation = canonicalize(&context);
    representation
        .source_set
        .push(representation.source_set[0].clone());
    assert!(matches!(
        encode_representation(&representation),
        Err(CanonicalDecodeError::DuplicateSourceReference)
    ));
    let mut conflicting = canonicalize(&context);
    conflicting
        .source_set
        .push(source("AFD-002", "2.0.0", "other"));
    conflicting
        .source_set
        .sort_by_key(|item| format!("{}{}{}", item.id, item.version, item.path));
    assert!(matches!(
        encode_representation(&conflicting),
        Err(CanonicalDecodeError::ConflictingSourceVersion(_))
    ));
    let mut ambiguous = canonicalize(&context);
    ambiguous
        .source_set
        .push(source("AFD-002", "1.0.0", "different-path"));
    ambiguous
        .source_set
        .sort_by_key(|item| format!("{}{}{}", item.id, item.version, item.path));
    assert!(matches!(
        encode_representation(&ambiguous),
        Err(CanonicalDecodeError::AmbiguousSourceReference(_))
    ));
}

#[test]
fn strict_decode_rejects_noncanonical_field_order_and_invalid_identifier() {
    let ContextResolutionResult::Resolved(context) =
        complete_catalog().resolve(&complete_request())
    else {
        panic!("fixture must resolve")
    };
    let valid = String::from_utf8(encode(&context).unwrap()).unwrap();
    let mut lines: Vec<_> = valid.lines().map(str::to_string).collect();
    lines.swap(0, 1);
    let reordered = format!("{}\n", lines.join("\n"));
    assert!(matches!(
        decode(reordered.as_bytes()),
        Err(CanonicalDecodeError::NonCanonicalOrdering)
    ));
    let invalid = valid.replace("operation_id=op-fixture-001", "operation_id=bad%20id");
    assert!(
        matches!(decode(invalid.as_bytes()), Err(CanonicalDecodeError::InvalidValue { field, .. }) if field == "operation_id")
    );
}

#[test]
fn missing_source_fails_closed() {
    let mut request = complete_request();
    request
        .required_sources
        .push(ConstitutionalSourceId::new("IMP-999").unwrap());
    let result = complete_catalog().resolve(&request);
    assert!(
        matches!(result, ContextResolutionResult::Indeterminate { findings } if findings.iter().any(|f| f.code == FindingCode::MissingSource))
    );
}

#[test]
fn missing_version_and_context_elements_fail_closed() {
    let mut request = complete_request();
    request.implementation_version = None;
    request.scope = None;
    let result = complete_catalog().resolve(&request);
    assert!(
        matches!(result, ContextResolutionResult::Indeterminate { findings } if findings.iter().any(|f| f.code == FindingCode::UnresolvedScope) && findings.iter().any(|f| f.code == FindingCode::IncompleteCanonicalInput))
    );
}

#[test]
fn ambiguous_source_fails_closed_without_selection() {
    let catalog = SourceCatalog::new(vec![
        source("AFD-002", "1.0.0", "a"),
        source("AFD-002", "1.0.1", "b"),
        source("IMP-000", "0.2.0", "c"),
    ]);
    let result = catalog.resolve(&complete_request());
    assert!(
        matches!(result, ContextResolutionResult::Indeterminate { findings } if findings.iter().any(|f| f.code == FindingCode::AmbiguousSource))
    );
}

#[test]
fn malformed_reference_is_rejected() {
    assert!(ConstitutionalSourceId::new("bad reference").is_err());
    assert!(ConstitutionalSourceVersion::new("").is_err());
    assert!(TimePoint::new("2026-07-29 12:00:00Z").is_err());
}

#[test]
fn effective_and_processing_time_remain_distinct() {
    let result = complete_catalog().resolve(&complete_request());
    let ContextResolutionResult::Resolved(context) = result else {
        panic!("fixture must resolve")
    };
    assert_ne!(
        context.constitutional_effective_time,
        context.processing_time
    );
    assert_ne!(
        context.constitutional_effective_time,
        context.observation_time
    );
}

#[test]
fn evidence_requires_exact_implementation_version() {
    let ContextResolutionResult::Resolved(context) =
        complete_catalog().resolve(&complete_request())
    else {
        panic!("fixture must resolve")
    };
    let result = EvidenceRecord::for_mapping(
        EvidenceRecordId::new("EV-CTX-0002").unwrap(),
        "context-fixture",
        &context,
        &mapping("reference-foundation-1.1.0"),
        "test",
        "pass",
    );
    assert!(matches!(
        result,
        Err(ValidationFinding {
            code: FindingCode::EvidenceSubjectMismatch,
            ..
        })
    ));
}
