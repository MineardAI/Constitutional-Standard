use constitutional_canonical::{canonical_identity_digest, validate_canonical_object};
use constitutional_contracts::{ImplementationVersion, TimePoint};
use constitutional_evidence::*;

fn valid_object() -> EvidenceObject {
    let id = EvidenceObjectIdentifier::new("EOBJ-001").unwrap();
    let origin = EvidenceOrigin {
        id: EvidenceOriginIdentifier::new("ORIGIN-001").unwrap(),
        source: EvidenceSourceIdentifier::new("SOURCE-001").unwrap(),
        producer: "manual-fixture".into(),
        collector: Some("reviewer".into()),
        method: "declared fixture construction".into(),
        produced_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
        collected_at: None,
        environment: Some("local".into()),
    };
    EvidenceObject {
        id: id.clone(),
        subject: EvidenceSubject::ImplementationArtifact("artifact-001".into()),
        kind: EvidenceKind::DocumentationReview,
        content: EvidenceContentReference {
            locator: "docs/example.md".into(),
            format: EvidenceFormat::Text,
            representation_version: None,
        },
        claim: EvidenceClaim {
            statement: "The bounded profile contains the declared boundary.".into(),
            declared_scope: "IMP-007 fixture".into(),
            limitations: vec!["not a conformance claim".into()],
        },
        relevance: "directly addresses the declared criterion".into(),
        origin: origin.clone(),
        provenance: EvidenceProvenance {
            origin,
            events: vec![ProvenanceEvent {
                id: ProvenanceEventIdentifier::new("EVENT-001").unwrap(),
                action: ProvenanceAction::Produced,
                actor: "manual-fixture".into(),
                occurred_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
                detail: "constructed for a bounded test".into(),
            }],
            lineage: vec![],
            inputs: vec![],
        },
        custody: vec![CustodyEvent {
            custodian: "reviewer".into(),
            started_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
            ended_at: None,
            transferor: None,
            recipient: None,
            basis: "fixture".into(),
            integrity_at_transfer: None,
        }],
        integrity: IntegrityDescriptor {
            status: IntegrityStatus::Intact,
            digest: Some(DigestDescriptor {
                algorithm: IntegrityAlgorithm::Fnv1a64,
                value: "fnv1a64:0000000000000000".into(),
            }),
            reference_state: "fixture bytes".into(),
            scope: "content".into(),
            assessed_by: Some("reviewer".into()),
            assessed_at: Some(TimePoint::new("2026-07-29T12:00:00Z").unwrap()),
        },
        authenticity: AuthenticityStatus::Unassessed,
        implementation_version: ImplementationVersion::new("reference-foundation-0.13.0").unwrap(),
        baseline: None,
        source_binding: None,
        limitations: vec![],
        collection_status: CollectionStatus::Collected,
        association_status: AssociationStatus::Associated,
        applicability: ApplicabilityStatus::Current,
        preservation: PreservationStatus::Active,
        challenge: ChallengeStatus::Unchallenged,
    }
}

#[test]
fn valid_object_keeps_integrity_provenance_and_authority_separate() {
    let object = valid_object();
    let result = validate_evidence_object(&object);
    assert!(result.valid);
    assert!(!result.truth_established && !result.authority_created && !result.assurance_issued);
    let canonical = evidence_canonical(&object);
    assert!(validate_canonical_object(&canonical).valid);
    assert!(canonical_identity_digest(&canonical).is_ok());
}

#[test]
fn digest_match_is_not_a_truth_or_authenticity_claim() {
    let digest = DigestDescriptor {
        algorithm: IntegrityAlgorithm::Fnv1a64,
        value: "fnv1a64:000000000000000a".into(),
    };
    assert!(compare_digest(&digest, &digest).unwrap());
    assert!(
        !compare_digest(
            &digest,
            &DigestDescriptor {
                value: "fnv1a64:000000000000000b".into(),
                ..digest.clone()
            }
        )
        .unwrap()
    );
}

#[test]
fn admission_and_sufficiency_are_explicit_and_separate() {
    let object = valid_object();
    let reference = EvidenceReference {
        id: EvidenceIdentifier::new("REF-001").unwrap(),
        object_id: object.id.clone(),
        version: object.implementation_version.clone(),
        scope: "IMP-007 fixture".into(),
    };
    let admission = AdmissionEvaluation {
        id: AdmissionEvaluationIdentifier::new("ADM-001").unwrap(),
        evidence: reference.clone(),
        criteria: vec![AdmissionCriterion {
            id: EvidenceCriterionIdentifier::new("CRIT-001").unwrap(),
            statement: "identity and provenance are present".into(),
            required: true,
        }],
        status: AdmissionStatus::AdmissibleWithLimitations,
        scope: "fixture".into(),
        actor: "reviewer".into(),
        assessed_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
        reasons: vec!["bounded use only".into()],
    };
    assert!(validate_admission(&admission, &object).valid);
    let admitted = AdmittedEvidenceReference {
        evidence: reference,
        admission,
    };
    let assessment = SufficiencyAssessment {
        id: SufficiencyAssessmentIdentifier::new("SUFF-001").unwrap(),
        target: "criterion-001".into(),
        scope: "fixture".into(),
        baseline: None,
        implementation_version: object.implementation_version.clone(),
        criteria: vec![SufficiencyCriterion {
            id: EvidenceCriterionIdentifier::new("CRIT-001").unwrap(),
            statement: "declared criterion".into(),
            required: true,
        }],
        admitted: vec![admitted],
        status: SufficiencyStatus::PartiallySufficient,
        limitations: vec!["not assurance".into()],
        assessed_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
    };
    assert!(validate_sufficiency(&assessment).valid);
}

#[test]
fn missing_provenance_and_prohibited_claims_fail_closed() {
    let mut object = valid_object();
    object.provenance.events.clear();
    object.claim.statement = "This establishes constitutional truth and certification".into();
    let result = validate_evidence_object(&object);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::MissingProvenance)
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::ProhibitedClaim)
    );
}

#[test]
fn empty_sufficiency_has_no_automatic_result() {
    let version = ImplementationVersion::new("reference-foundation-0.13.0").unwrap();
    let assessment = SufficiencyAssessment {
        id: SufficiencyAssessmentIdentifier::new("SUFF-EMPTY").unwrap(),
        target: "criterion".into(),
        scope: "scope".into(),
        baseline: None,
        implementation_version: version,
        criteria: vec![],
        admitted: vec![],
        status: SufficiencyStatus::Indeterminate,
        limitations: vec![],
        assessed_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
    };
    let result = validate_sufficiency(&assessment);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::SufficiencyWithoutAdmission)
    );
}

#[test]
fn digest_mismatch_is_a_distinct_validation_finding() {
    let expected = DigestDescriptor {
        algorithm: IntegrityAlgorithm::Fnv1a64,
        value: "fnv1a64:000000000000000a".into(),
    };
    let observed = DigestDescriptor {
        algorithm: IntegrityAlgorithm::Fnv1a64,
        value: "fnv1a64:000000000000000b".into(),
    };
    let result = validate_digest(&expected, &observed, "EOBJ-001");
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::DigestMismatch)
    );
}

#[test]
fn evidence_boundary_findings_are_reachable() {
    let version = ImplementationVersion::new("reference-foundation-0.13.0").unwrap();
    let other = ImplementationVersion::new("reference-foundation-0.14.0").unwrap();
    assert!(
        validate_version_binding(&other, &version, "EOBJ-001")
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::VersionMismatch)
    );
    let duplicate = EvidenceIdentifier::new("E-001").unwrap();
    assert!(
        validate_unique_identifiers(&[duplicate.clone(), duplicate])
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::DuplicateIdentifier)
    );
    assert!(
        validate_subject_presence(None, "EOBJ-MISSING")
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::MissingSubject)
    );
    assert!(!validate_scope_containment("scope/base", "scope/expanded/other", "EOBJ-001").valid);
}

#[test]
fn admission_status_and_custody_continuity_fail_closed() {
    let mut object = valid_object();
    let reference = EvidenceReference {
        id: EvidenceIdentifier::new("REF-UNSUPPORTED").unwrap(),
        object_id: object.id.clone(),
        version: object.implementation_version.clone(),
        scope: "IMP-007 fixture".into(),
    };
    let evaluation = AdmissionEvaluation {
        id: AdmissionEvaluationIdentifier::new("ADM-UNSUPPORTED").unwrap(),
        evidence: reference,
        criteria: vec![AdmissionCriterion {
            id: EvidenceCriterionIdentifier::new("CRIT-001").unwrap(),
            statement: "criterion".into(),
            required: true,
        }],
        status: AdmissionStatus::Unsupported("future-status".into()),
        scope: "fixture".into(),
        actor: "reviewer".into(),
        assessed_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
        reasons: vec![],
    };
    assert!(
        validate_admission(&evaluation, &object)
            .findings
            .iter()
            .any(|f| f.code == EvidenceFindingCode::UnsupportedAdmissionStatus)
    );
    object.custody.push(CustodyEvent {
        custodian: "second-reviewer".into(),
        started_at: TimePoint::new("2026-07-29T14:00:00Z").unwrap(),
        ended_at: None,
        transferor: Some("wrong-reviewer".into()),
        recipient: None,
        basis: "transfer".into(),
        integrity_at_transfer: None,
    });
    assert!(!validate_custody_chain(&object.custody, &object.id, true).valid);
}

fn typed_event(
    id: &str,
    sequence: u64,
    predecessor: Option<&str>,
    evidence: &str,
) -> TypedCustodyEvent {
    TypedCustodyEvent {
        id: id.into(),
        chain_id: "CHAIN-001".into(),
        evidence_id: EvidenceObjectIdentifier::new(evidence).unwrap(),
        sequence,
        predecessor: predecessor.map(str::to_owned),
        custodian: if sequence == 0 { "origin" } else { "recipient" }.into(),
        started_at: TimePoint::new(if sequence == 0 {
            "2026-07-29T12:00:00Z"
        } else {
            "2026-07-29T13:00:00Z"
        })
        .unwrap(),
        ended_at: if sequence == 0 {
            Some(TimePoint::new("2026-07-29T13:00:00Z").unwrap())
        } else {
            None
        },
        transferor: if sequence == 0 {
            None
        } else {
            Some("recipient".into())
        },
        recipient: if sequence == 0 {
            Some("recipient".into())
        } else {
            None
        },
        basis: "declared transfer".into(),
        integrity_at_transfer: None,
        limitations: vec![],
        known_gaps: vec![],
        implementation_version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
    }
}

#[test]
fn typed_custody_chain_enforces_identity_predecessor_and_sequence() {
    let origin = typed_event("EVENT-1", 0, None, "EOBJ-001");
    let successor = typed_event("EVENT-2", 1, Some("EVENT-1"), "EOBJ-001");
    assert!(validate_typed_custody_chain(&[successor.clone(), origin.clone()], false).valid);
    let mut substituted = successor.clone();
    substituted.evidence_id = EvidenceObjectIdentifier::new("EOBJ-002").unwrap();
    let result = validate_typed_custody_chain(&[origin.clone(), substituted], false);
    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == EvidenceFindingCode::CustodyIdentityMismatch)
    );
    let mut duplicate = successor;
    duplicate.id = "EVENT-1".into();
    let result = validate_typed_custody_chain(&[origin, duplicate], false);
    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == EvidenceFindingCode::CustodyIdentityMismatch)
    );
    let mut duplicate_sequence = typed_event("EVENT-3", 1, Some("EVENT-1"), "EOBJ-001");
    duplicate_sequence.custodian = "recipient-2".into();
    let result = validate_typed_custody_chain(
        &[
            typed_event("EVENT-1", 0, None, "EOBJ-001"),
            typed_event("EVENT-2", 1, Some("EVENT-1"), "EOBJ-001"),
            duplicate_sequence,
        ],
        false,
    );
    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == EvidenceFindingCode::CustodySequenceMismatch)
    );
}

#[test]
fn typed_custody_gap_is_recorded_without_automatic_invalidation() {
    let mut origin = typed_event("EVENT-1", 0, None, "EOBJ-001");
    origin.ended_at = Some(TimePoint::new("2026-07-29T12:30:00Z").unwrap());
    let mut successor = typed_event("EVENT-2", 1, Some("EVENT-1"), "EOBJ-001");
    successor.started_at = TimePoint::new("2026-07-29T13:00:00Z").unwrap();
    successor.known_gaps = vec!["known custody gap between custodians".into()];
    let permitted = validate_typed_custody_chain(&[origin.clone(), successor.clone()], false);
    assert!(permitted.valid);
    assert!(
        permitted
            .findings
            .iter()
            .any(|finding| finding.code == EvidenceFindingCode::CustodyGap)
    );
    assert!(!validate_typed_custody_chain(&[origin, successor], true).valid);
}

#[test]
fn custody_context_requires_explicit_gap_and_preserves_non_authority_boundary() {
    let evidence = EvidenceObjectIdentifier::new("EOBJ-001").unwrap();
    let events = vec![
        CustodyEvent {
            custodian: "origin".into(),
            started_at: TimePoint::new("2026-07-29T12:00:00Z").unwrap(),
            ended_at: Some(TimePoint::new("2026-07-29T12:30:00Z").unwrap()),
            transferor: None,
            recipient: Some("recipient".into()),
            basis: "declared transfer".into(),
            integrity_at_transfer: Some(IntegrityStatus::Intact),
        },
        CustodyEvent {
            custodian: "recipient".into(),
            started_at: TimePoint::new("2026-07-29T13:00:00Z").unwrap(),
            ended_at: None,
            transferor: Some("recipient".into()),
            recipient: None,
            basis: "declared transfer".into(),
            integrity_at_transfer: None,
        },
    ];
    let missing = validate_custody_for_evidence(
        &events,
        &evidence,
        &[],
        &ChallengeStatus::Unchallenged,
        false,
    );
    assert!(!missing.valid);
    let permitted = validate_custody_for_evidence(
        &events,
        &evidence,
        &[EvidenceLimitation {
            statement: "known custody gap retained as a limitation".into(),
            scope: "custody interval".into(),
        }],
        &ChallengeStatus::Unchallenged,
        false,
    );
    assert!(permitted.valid);
}

#[test]
fn evidence_history_references_reject_silent_replacement_and_unresolved_conflicts() {
    let object = EvidenceObjectIdentifier::new("EOBJ-001").unwrap();
    let first = EvidenceReference {
        id: EvidenceIdentifier::new("EVID-001").unwrap(),
        object_id: object.clone(),
        version: ImplementationVersion::new("reference-foundation-0.15.0").unwrap(),
        scope: "closure".into(),
    };
    let replacement = EvidenceReference {
        id: EvidenceIdentifier::new("EVID-002").unwrap(),
        object_id: object,
        version: first.version.clone(),
        scope: "closure".into(),
    };
    let relationship = EvidenceRelationship {
        from: replacement.id.clone(),
        to: first.id.clone(),
        relation: EvidenceRelation::Corrects,
    };
    assert!(
        validate_evidence_history_references(
            std::slice::from_ref(&replacement.object_id),
            &[first.clone(), replacement.clone()],
            &[relationship],
            &[]
        )
        .valid
    );
    let conflict = EvidenceConflict {
        id: EvidenceIdentifier::new("CONFLICT-001").unwrap(),
        affected: vec![EvidenceReference {
            id: EvidenceIdentifier::new("EVID-MISSING").unwrap(),
            object_id: replacement.object_id.clone(),
            version: first.version.clone(),
            scope: "closure".into(),
        }],
        context: "closure".into(),
        status: ChallengeStatus::Challenged,
        detail: "unresolved conflict".into(),
    };
    assert!(
        !validate_evidence_history_references(
            &[EvidenceObjectIdentifier::new("EOBJ-001").unwrap()],
            &[first, replacement],
            &[],
            &[conflict]
        )
        .valid
    );
}
