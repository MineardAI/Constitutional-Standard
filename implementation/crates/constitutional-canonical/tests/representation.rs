use constitutional_canonical::*;
use constitutional_contracts::{
    ConstitutionalSourceId, ConstitutionalSourceRef, ConstitutionalSourceVersion,
    ImplementationVersion,
};

fn text(value: &str) -> CanonicalValue {
    CanonicalValue::Text(value.into())
}

fn object() -> CanonicalObject {
    CanonicalObject {
        representation_id: CanonicalRepresentationId::new("representation-1").unwrap(),
        kind: CanonicalRepresentationKind::TraceabilityRecord,
        representation_version: "0.1.0".into(),
        schema_id: "traceability-profile-1".into(),
        source_binding: Some(
            ConstitutionalSourceRef::new(
                ConstitutionalSourceId::new("IMP-005").unwrap(),
                ConstitutionalSourceVersion::new("0.1.0").unwrap(),
                "harmonization-v1.0/source,with-comma.md",
            )
            .unwrap(),
        ),
        implementation_version: Some(
            ImplementationVersion::new("reference-foundation-0.12.0").unwrap(),
        ),
        required_fields: vec!["alpha".into()],
        extension_namespaces: vec!["x-".into()],
        fields: vec![
            CanonicalField {
                name: "alpha".into(),
                value: text("identity"),
                required: true,
                identity_participates: true,
                extension: false,
            },
            CanonicalField {
                name: "nested".into(),
                value: CanonicalValue::Object(Box::new(CanonicalObject {
                    representation_id: CanonicalRepresentationId::new("nested-1").unwrap(),
                    kind: CanonicalRepresentationKind::Determination,
                    representation_version: "0.1.0".into(),
                    schema_id: "determination-profile-1".into(),
                    source_binding: None,
                    implementation_version: None,
                    required_fields: vec![],
                    extension_namespaces: vec![],
                    fields: vec![CanonicalField {
                        name: "value".into(),
                        value: CanonicalValue::Boolean(true),
                        required: true,
                        identity_participates: true,
                        extension: false,
                    }],
                })),
                required: false,
                identity_participates: false,
                extension: false,
            },
            CanonicalField {
                name: "x-note".into(),
                value: CanonicalValue::Collection(CanonicalCollection::unordered(vec![
                    text("b"),
                    text("a"),
                ])),
                required: false,
                identity_participates: false,
                extension: true,
            },
            CanonicalField {
                name: "zeta".into(),
                value: text("display-only"),
                required: false,
                identity_participates: false,
                extension: false,
            },
        ],
    }
}

#[test]
fn deterministic_nested_collection_and_omission_representation_round_trips() {
    let value = object();
    let first = canonical_encode(&value).unwrap();
    let second = canonical_encode(&value).unwrap();
    assert_eq!(first.bytes, second.bytes);
    let decoded = canonical_decode(&first.bytes).unwrap();
    assert_eq!(decoded.object, value);
    assert_eq!(decoded.identity_digest, first.identity_digest);
    assert!(!decoded.validation.authority_created);
    assert!(!decoded.validation.source_admitted);
    assert!(!decoded.validation.persisted);
    assert!(!decoded.validation.transmitted);
    assert!(!decoded.validation.execution_performed);
    assert!(!decoded.validation.domain_validity_established);
}

#[test]
fn source_and_implementation_bindings_are_preserved() {
    let value = object();
    let decoded = canonical_decode(&canonical_encode(&value).unwrap().bytes).unwrap();
    assert_eq!(decoded.object.source_binding, value.source_binding);
    assert_eq!(
        decoded.object.implementation_version,
        value.implementation_version
    );
    let expected = ImplementationVersion::new("reference-foundation-0.12.0").unwrap();
    assert!(canonical_validate_for_implementation(&value, &expected).valid);
    let mismatch = canonical_validate_for_implementation(
        &value,
        &ImplementationVersion::new("reference-foundation-9.9.9").unwrap(),
    );
    assert!(mismatch.findings.iter().any(
        |finding| finding.code == CanonicalValidationFindingCode::ImplementationVersionMismatch
    ));
}

#[test]
fn identity_participation_excludes_non_identity_fields() {
    let value = object();
    let first = canonical_identity_digest(&value).unwrap();
    let mut changed_non_identity = value.clone();
    changed_non_identity.fields[3].value = text("another-display-value");
    assert_eq!(
        first,
        canonical_identity_digest(&changed_non_identity).unwrap()
    );
    changed_non_identity.fields[0].value = text("changed-identity");
    assert_ne!(
        first,
        canonical_identity_digest(&changed_non_identity).unwrap()
    );
}

#[test]
fn compatibility_is_bounded_to_representation_dimensions() {
    let value = object();
    assert_eq!(
        canonical_compatibility(&value, &value),
        RepresentationCompatibility::ExactRepresentationVersionMatch
    );
    let mut compatible = value.clone();
    compatible.representation_version = "0.1.1".into();
    assert_eq!(
        canonical_compatibility(&value, &compatible),
        RepresentationCompatibility::SupportedCompatibleVersion
    );
    let mut incompatible_schema = value.clone();
    incompatible_schema.schema_id = "other-profile".into();
    assert_eq!(
        canonical_compatibility(&value, &incompatible_schema),
        RepresentationCompatibility::IncompatibleSchemaProfile
    );
    let mut incompatible_kind = value.clone();
    incompatible_kind.kind = CanonicalRepresentationKind::CoreEvaluation;
    assert_eq!(
        canonical_compatibility(&value, &incompatible_kind),
        RepresentationCompatibility::IncompatibleRepresentationKind
    );
}

#[test]
fn strict_decode_rejects_unknown_duplicate_missing_malformed_and_trailing_input() {
    let bytes = canonical_encode(&object()).unwrap().bytes;
    let text = String::from_utf8(bytes.clone()).unwrap();
    assert!(canonical_decode(format!("{text}unknown=x\n").as_bytes()).is_err());
    assert!(canonical_decode(format!("{text}representation_id=duplicate\n").as_bytes()).is_err());
    assert!(
        canonical_decode(
            text.replace("schema_id=traceability-profile-1\n", "")
                .as_bytes()
        )
        .is_err()
    );
    assert!(
        canonical_decode(
            text.replace("kind=traceability-record\n", "kind=unsupported-kind\n")
                .as_bytes()
        )
        .is_err()
    );
    assert!(
        canonical_decode(
            text.replace(
                "representation_version=0.1.0\n",
                "representation_version=not-a-version\n"
            )
            .as_bytes()
        )
        .is_err()
    );
    assert!(
        canonical_decode(
            text.replace("field.0.value=text:identity\n", "field.0.value=unknown\n")
                .as_bytes()
        )
        .is_err()
    );
    assert!(canonical_decode(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn serialized_validation_returns_typed_deterministic_findings() {
    let bytes = canonical_encode(&object()).unwrap().bytes;
    let unknown = canonical_validate_bytes(
        String::from_utf8(bytes)
            .unwrap()
            .replace("schema_id=traceability-profile-1", "unknown=x")
            .as_bytes(),
    );
    assert!(!unknown.valid);
    assert_eq!(
        unknown.findings[0].code,
        CanonicalValidationFindingCode::UnknownCanonicalField
    );
    let missing = canonical_validate_bytes(b"kind=traceability-record\n");
    assert!(!missing.valid);
    assert_eq!(
        missing.findings[0].code,
        CanonicalValidationFindingCode::MissingRepresentationIdentifier
    );
}

#[test]
fn validation_rejects_field_order_collection_order_and_prohibited_boundaries() {
    let mut reversed = object();
    reversed.fields.reverse();
    assert!(canonical_encode(&reversed).is_err());
    let mut unsorted_collection = object();
    unsorted_collection.fields[2].value = CanonicalValue::Collection(CanonicalCollection {
        ordered: false,
        items: vec![text("b"), text("a")],
    });
    assert!(canonical_encode(&unsorted_collection).is_err());
    let mut prohibited = object();
    prohibited.fields.push(CanonicalField {
        name: "transport.route".into(),
        value: text("x"),
        required: false,
        identity_participates: false,
        extension: false,
    });
    prohibited
        .fields
        .sort_by(|left, right| left.name.cmp(&right.name));
    assert!(canonical_encode(&prohibited).is_err());
    assert!(CanonicalRepresentationId::new("bad id").is_err());
}

#[test]
fn domain_wrapper_contracts_remain_available_and_non_authoritative() {
    let source = object();
    let result = canonical_validate_for_implementation(
        &source,
        source.implementation_version.as_ref().unwrap(),
    );
    assert!(result.valid);
    assert!(
        result
            .non_claims
            .iter()
            .any(|claim| claim.contains("not constitutional validity"))
    );
}
