use constitutional_artifacts::{
    ArtifactIdentity, ArtifactIntegrityClaim, ArtifactKind, ArtifactProvenance,
    ArtifactRelationship, ArtifactRelationshipKind, ArtifactRole, DomainRef,
};
use constitutional_canonical::{
    artifact_decode, artifact_evaluation_encode, artifact_identity_encode,
    artifact_integrity_encode, artifact_provenance_encode, artifact_relationship_encode,
};
use constitutional_contracts::{ContextId, ImplementationVersion};

fn r(value: &str) -> DomainRef {
    DomainRef::new(value).unwrap()
}

#[test]
fn artifact_identity_provenance_relationship_and_integrity_encodings_are_deterministic() {
    let identity = ArtifactIdentity {
        id: r("artifact-1"),
        kind: ArtifactKind::EvidenceArtifact,
        represented_subject: Some(r("subject-1")),
        revision: r("revision-1"),
        roles: vec![ArtifactRole::Evidencing],
    };
    let identity_bytes = artifact_identity_encode(&identity).unwrap();
    assert!(
        artifact_decode(
            &identity_bytes,
            "macs-reference-canonical-artifact-identity"
        )
        .is_ok()
    );
    let provenance = ArtifactProvenance {
        origin: Some(r("origin")),
        authority: None,
        identity: None,
        transformation: None,
        version: None,
        custody: None,
        publication: None,
        lineage: vec![r("lineage-1")],
    };
    assert!(
        artifact_decode(
            &artifact_provenance_encode(&provenance).unwrap(),
            "macs-reference-canonical-artifact-provenance"
        )
        .is_ok()
    );
    let integrity = ArtifactIntegrityClaim {
        representational: Some(true),
        identity: Some(true),
        instance: Some(true),
        lineage: Some(true),
        transformation: Some(true),
        version: Some(true),
        custody: Some(true),
        publication: Some(true),
    };
    assert!(
        artifact_decode(
            &artifact_integrity_encode(&integrity).unwrap(),
            "macs-reference-canonical-artifact-integrity"
        )
        .is_ok()
    );
    let relationship = ArtifactRelationship {
        from: r("artifact-1"),
        kind: ArtifactRelationshipKind::References,
        to: r("artifact-2"),
    };
    assert!(
        artifact_decode(
            &artifact_relationship_encode(&relationship).unwrap(),
            "macs-reference-canonical-artifact-relationship"
        )
        .is_ok()
    );
}

#[test]
fn artifact_evaluation_encoding_rejects_unknown_and_duplicate_fields() {
    let result = constitutional_artifacts::ArtifactEvaluationResult {
        evaluation_id: r("eval-1"),
        operation_id: r("op-1"),
        context_id: ContextId::new("ctx-1").unwrap(),
        source_set_id: None,
        implementation_version: ImplementationVersion::new("reference-foundation-0.6.0").unwrap(),
        profile_version: r("1.0.0"),
        claim: None,
        recognition: constitutional_artifacts::ArtifactDetermination::Indeterminate,
        instance: constitutional_artifacts::ArtifactInstanceDetermination::NotApplicable,
        identity_preservation:
            constitutional_artifacts::IdentityPreservationDetermination::NotApplicable,
        provenance: constitutional_artifacts::ProvenanceSufficiencyDetermination::Indeterminate,
        integrity: constitutional_artifacts::IntegrityDetermination::NotApplicable,
        relationships: Vec::new(),
        findings: Vec::new(),
        non_claims: vec!["no truth claim".into()],
    };
    let valid = artifact_evaluation_encode(&result).unwrap();
    assert!(artifact_decode(&valid, "macs-reference-canonical-artifact-evaluation").is_ok());
    assert!(
        artifact_decode(
            format!("{}unknown=x\n", String::from_utf8(valid.clone()).unwrap()).as_bytes(),
            "macs-reference-canonical-artifact-evaluation"
        )
        .is_err()
    );
    assert!(
        artifact_decode(
            format!("{}format_id=x\n", String::from_utf8(valid).unwrap()).as_bytes(),
            "macs-reference-canonical-artifact-evaluation"
        )
        .is_err()
    );
}
