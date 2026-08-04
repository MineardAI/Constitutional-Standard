use constitutional_canonical::{
    canonical_decode, canonical_encode, canonical_identity_digest, validate_canonical_object,
};
use constitutional_contracts::{BaselineRef, EvidenceRecordId, ImplementationVersion, TimePoint};
use constitutional_release::*;
use constitutional_traceability::TraceRef;

fn identity(id: &str, version: &str) -> ReleaseIdentity {
    ReleaseIdentity {
        id: ReleaseIdentifier::new(id).unwrap(),
        version: ReleaseVersionIdentifier::new(version).unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
        baseline: BaselineRef::new("baseline-001").unwrap(),
    }
}
fn released(id: &str, version: &str) -> ReleasedImplementation {
    let identity = identity(id, version);
    let manifest = ReleaseManifest {
        id: ReleaseManifestIdentifier::new(format!("MANIFEST-{id}")).unwrap(),
        release: identity.clone(),
        constitutional_baseline: Some("MACS-baseline".into()),
        applicable_specifications: vec!["IMP-008@0.2.0".into()],
        components: vec!["constitutional-release".into()],
        evidence: vec![],
        assurance_references: vec![],
        traceability_references: vec![],
        limitations: vec!["not activation".into()],
        publication: PublicationState::Unpublished,
        claims_activation: false,
        claims_authority: false,
    };
    ReleasedImplementation {
        identity,
        state: ReleaseState::Released,
        manifest,
        effective_from: Some(TimePoint::new("2026-07-29T12:00:00Z").unwrap()),
        support_until: None,
    }
}

#[test]
fn valid_release_and_canonical_projection_are_deterministic() {
    let release = released("REL-001", "1.0.0");
    assert!(validate_released_implementation(&release).valid);
    let canonical = release_canonical(&release);
    assert!(validate_canonical_object(&canonical).valid);
    assert_eq!(
        canonical_identity_digest(&canonical),
        canonical_identity_digest(&canonical)
    );
}

#[test]
fn lineage_preserves_predecessor_and_rejects_self_reference() {
    let first = identity("REL-001", "1.0.0");
    let second = identity("REL-002", "2.0.0");
    let valid = ReleaseLineage {
        releases: vec![first.clone(), second.clone()],
        relationships: vec![ReleaseRelationship {
            id: ReleaseRelationshipIdentifier::new("LINK-001").unwrap(),
            from: first.id.clone(),
            to: second.id.clone(),
            relation: LineageRelation::Successor,
        }],
    };
    assert!(validate_lineage(&valid).valid);
    let invalid = ReleaseLineage {
        releases: vec![first.clone()],
        relationships: vec![ReleaseRelationship {
            id: ReleaseRelationshipIdentifier::new("LINK-SELF").unwrap(),
            from: first.id.clone(),
            to: first.id,
            relation: LineageRelation::Predecessor,
        }],
    };
    assert!(!validate_lineage(&invalid).valid);
}

#[test]
fn compatibility_claim_and_determination_remain_distinct() {
    let subject = identity("REL-001", "1.0.0");
    let target = identity("REL-002", "2.0.0");
    let claim = CompatibilityClaim {
        id: CompatibilityClaimIdentifier::new("CLAIM-001").unwrap(),
        subject: subject.id.clone(),
        target: target.id.clone(),
        dimension: CompatibilityDimension::Representation,
        statement: "canonical representation remains readable".into(),
        evidence: vec![],
        limitations: vec![],
    };
    let assessment = CompatibilityAssessment {
        id: CompatibilityAssessmentIdentifier::new("ASSESS-001").unwrap(),
        subject: subject.id,
        target: target.id,
        dimension: CompatibilityDimension::Representation,
        direction: CompatibilityDirection::Bidirectional,
        scope: "canonical representation".into(),
        criteria: vec!["strict round trip".into()],
        assumptions: vec![],
        baseline: Some(subject.baseline),
        implementation_context: "reference context".into(),
        admitted_source_set: Some("SOURCE-SET-001".into()),
        traceability: vec![],
        evidence: vec![],
        verification: vec![],
        claims: vec![claim.id],
        outcome: CompatibilityOutcome::ConditionallyCompatible,
        limitations: vec!["not assurance".into()],
        unresolved_conditions: vec!["migration review".into()],
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
    };
    assert!(validate_compatibility(&assessment).valid);
}

#[test]
fn compatibility_without_context_and_duplicate_claims_fails() {
    let a = identity("REL-001", "1.0.0");
    let b = identity("REL-002", "2.0.0");
    let claim = CompatibilityClaimIdentifier::new("CLAIM-001").unwrap();
    let assessment = CompatibilityAssessment {
        id: CompatibilityAssessmentIdentifier::new("ASSESS-FAIL").unwrap(),
        subject: a.id,
        target: b.id,
        dimension: CompatibilityDimension::Representation,
        direction: CompatibilityDirection::SubjectToTarget,
        scope: String::new(),
        criteria: vec![],
        assumptions: vec![],
        baseline: None,
        implementation_context: String::new(),
        admitted_source_set: None,
        traceability: vec![],
        evidence: vec![],
        verification: vec![],
        claims: vec![claim.clone(), claim],
        outcome: CompatibilityOutcome::Indeterminate,
        limitations: vec![],
        unresolved_conditions: vec![],
        implementation_version: a.implementation_version,
    };
    let result = validate_compatibility(&assessment);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ReleaseFindingCode::MissingDeterminationContext)
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ReleaseFindingCode::DuplicateCompatibilityClaim)
    );
}

#[test]
fn migration_is_declared_but_never_executed() {
    let requirement = MigrationRequirement {
        id: MigrationRequirementIdentifier::new("MIG-REQ-001").unwrap(),
        source: ReleaseIdentifier::new("REL-001").unwrap(),
        target: ReleaseIdentifier::new("REL-002").unwrap(),
        requirements: vec!["preserve historical identity".into()],
        preconditions: vec!["review required".into()],
        preserved_properties: vec!["source lineage".into()],
        known_losses: vec![],
        compatibility_effects: vec![ChangeEffect::MigrationRequiredEffect],
        evidence: vec![],
        verification: vec![],
        limitations: vec!["not executed".into()],
        unresolved_conditions: vec!["verification pending".into()],
        status: MigrationStatus::Declared,
    };
    let plan = MigrationPlan {
        id: MigrationPlanIdentifier::new("MIG-PLAN-001").unwrap(),
        requirement: requirement.id.clone(),
        source: requirement.source.clone(),
        target: requirement.target.clone(),
        steps: vec!["review declared transformation".into()],
        transformations: vec![],
        limitations: vec![],
        status: MigrationStatus::Planned,
    };
    let result = validate_migration(&requirement, Some(&plan));
    assert!(result.valid && !result.migration_executed);
}

#[test]
fn activation_and_authority_claims_are_rejected() {
    let mut release = released("REL-001", "1.0.0");
    release.manifest.claims_activation = true;
    release.manifest.claims_authority = true;
    let result = validate_released_implementation(&release);
    assert!(!result.valid && !result.activation_performed && !result.authority_created);
}

#[test]
fn bounded_resolution_and_strict_canonical_round_trip_close_release_bindings() {
    let first = released("REL-001", "1.0.0");
    let second = released("REL-002", "2.0.0");
    let context = ReleaseResolutionContext {
        implementation_version: first.identity.implementation_version.clone(),
        releases: vec![first.clone(), second.clone()],
    };
    assert!(validate_release_resolution(&context).valid);

    let assessment = CompatibilityAssessment {
        id: CompatibilityAssessmentIdentifier::new("ASSESS-ROUNDTRIP").unwrap(),
        subject: first.identity.id.clone(),
        target: second.identity.id.clone(),
        dimension: CompatibilityDimension::Representation,
        direction: CompatibilityDirection::Bidirectional,
        scope: "canonical representation".into(),
        criteria: vec!["strict round trip".into()],
        assumptions: vec![],
        baseline: None,
        implementation_context: "reference context".into(),
        admitted_source_set: None,
        traceability: vec![],
        evidence: vec![],
        verification: vec!["decode and re-encode".into()],
        claims: vec![CompatibilityClaimIdentifier::new("CLAIM-ROUNDTRIP").unwrap()],
        outcome: CompatibilityOutcome::Compatible,
        limitations: vec![],
        unresolved_conditions: vec![],
        implementation_version: first.identity.implementation_version.clone(),
    };
    let claim = CompatibilityClaim {
        id: assessment.claims[0].clone(),
        subject: assessment.subject.clone(),
        target: assessment.target.clone(),
        dimension: assessment.dimension.clone(),
        statement: "representation round trip remains stable".into(),
        evidence: vec![],
        limitations: vec![],
    };
    assert!(validate_compatibility_with_context(&assessment, &[claim], &context).valid);

    let canonical = release_canonical(&first);
    let encoded = canonical_encode(&canonical).unwrap();
    let decoded = canonical_decode(&encoded.bytes).unwrap();
    assert_eq!(
        encoded.bytes,
        canonical_encode(&decoded.object).unwrap().bytes
    );
    assert_eq!(encoded.identity_digest, decoded.identity_digest);
}

fn migration(id: &str, source: &str, target: &str) -> MigrationRequirement {
    MigrationRequirement {
        id: MigrationRequirementIdentifier::new(id).unwrap(),
        source: ReleaseIdentifier::new(source).unwrap(),
        target: ReleaseIdentifier::new(target).unwrap(),
        requirements: vec!["preserve identity".into()],
        preconditions: vec![],
        preserved_properties: vec!["history".into()],
        known_losses: vec![],
        compatibility_effects: vec![ChangeEffect::MigrationRequiredEffect],
        evidence: vec![],
        verification: vec![],
        limitations: vec![],
        unresolved_conditions: vec![],
        status: MigrationStatus::Declared,
    }
}

#[test]
fn migration_graph_and_supersession_cycles_are_rejected() {
    let cycle = vec![
        migration("MIG-A-B", "REL-A", "REL-B"),
        migration("MIG-B-C", "REL-B", "REL-C"),
        migration("MIG-C-A", "REL-C", "REL-A"),
    ];
    let result = validate_migration_graph(&cycle);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ReleaseFindingCode::CircularMigration)
    );

    let lineage = ReleaseLineage {
        releases: vec![identity("REL-A", "1.0.0"), identity("REL-B", "2.0.0")],
        relationships: vec![
            ReleaseRelationship {
                id: ReleaseRelationshipIdentifier::new("SUP-A-B").unwrap(),
                from: ReleaseIdentifier::new("REL-A").unwrap(),
                to: ReleaseIdentifier::new("REL-B").unwrap(),
                relation: LineageRelation::Supersedes,
            },
            ReleaseRelationship {
                id: ReleaseRelationshipIdentifier::new("SUP-B-A").unwrap(),
                from: ReleaseIdentifier::new("REL-B").unwrap(),
                to: ReleaseIdentifier::new("REL-A").unwrap(),
                relation: LineageRelation::Supersedes,
            },
        ],
    };
    assert!(!validate_supersession_graph(&lineage).valid);
}

#[test]
fn release_history_and_subject_bound_activation_traceability_are_enforced() {
    let first = identity("REL-001", "1.0.0");
    let second = identity("REL-002", "2.0.0");
    let mut history = vec![
        ReleaseHistoryEntry {
            sequence: 0,
            release: first.clone(),
            state: ReleaseState::Released,
            predecessor: None,
            migration: vec![],
            compatibility: vec![],
            evidence: vec![],
            assurance: vec![],
        },
        ReleaseHistoryEntry {
            sequence: 1,
            release: second.clone(),
            state: ReleaseState::Superseded,
            predecessor: Some(first.id.clone()),
            migration: vec![],
            compatibility: vec![],
            evidence: vec![],
            assurance: vec![],
        },
    ];
    assert!(validate_release_history(&history).valid);
    history[1].release.version = ReleaseVersionIdentifier::new("9.0.0").unwrap();
    history.push(history[1].clone());
    assert!(!validate_release_history(&history).valid);

    let chain = ReleaseTraceabilityChain {
        subject: first.id.clone(),
        implementation_version: first.implementation_version.clone(),
        release: first.id.clone(),
        lineage: vec![],
        compatibility: vec![],
        migration: vec![],
        evidence: vec![],
        activation: vec!["ACT-001".into()],
        recognition: vec!["REC-001".into()],
    };
    let result = validate_release_traceability_chain_with_activation(
        &chain,
        &[],
        &[],
        &[],
        &[],
        &["ACT-001".into()],
        &["REC-001".into()],
    );
    assert!(result.valid);
    let foreign = validate_release_traceability_chain_with_activation(
        &chain,
        &[],
        &[],
        &[],
        &[],
        &["ACT-FOREIGN".into()],
        &[],
    );
    assert!(!foreign.valid);
}

fn compatibility_assessment(id: &str, outcome: CompatibilityOutcome) -> CompatibilityAssessment {
    CompatibilityAssessment {
        id: CompatibilityAssessmentIdentifier::new(id).unwrap(),
        subject: ReleaseIdentifier::new("REL-001").unwrap(),
        target: ReleaseIdentifier::new("REL-002").unwrap(),
        dimension: CompatibilityDimension::Representation,
        direction: CompatibilityDirection::Bidirectional,
        scope: "canonical".into(),
        criteria: vec!["round trip".into()],
        assumptions: vec![],
        baseline: None,
        implementation_context: "context-001".into(),
        admitted_source_set: None,
        traceability: vec![],
        evidence: vec![],
        verification: vec![],
        claims: vec![],
        outcome,
        limitations: vec![],
        unresolved_conditions: vec![],
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
    }
}

#[test]
fn compatibility_contradictions_are_detected_without_collapsing_claims() {
    let result = validate_compatibility_contradictions(&[
        compatibility_assessment("ASSESS-COMPATIBLE", CompatibilityOutcome::Compatible),
        compatibility_assessment("ASSESS-INCOMPATIBLE", CompatibilityOutcome::Incompatible),
    ]);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.code == ReleaseFindingCode::ConflictingCompatibilityClaim)
    );
}

fn operational_reference(id: &str) -> OperationalTraceReference {
    OperationalTraceReference {
        id: id.into(),
        subject: ReleaseIdentifier::new("REL-001").unwrap(),
        release: ReleaseIdentifier::new("REL-001").unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
        authority: Some(TraceRef::new("AUTH-001").unwrap()),
        baseline: Some(TraceRef::new("BASE-001").unwrap()),
        scope: Some(TraceRef::new("SCOPE-001").unwrap()),
    }
}

#[test]
fn typed_release_operational_trace_rejects_foreign_subject_and_version() {
    let trace = ReleaseOperationalTrace {
        release: ReleaseIdentifier::new("REL-001").unwrap(),
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
        assignment: operational_reference("ASSIGNMENT"),
        activation_request: operational_reference("REQUEST"),
        activation_decision: operational_reference("DECISION"),
        activation_record: operational_reference("RECORD"),
        recognition: Some(operational_reference("RECOGNITION")),
    };
    assert!(validate_release_operational_trace(&trace).valid);
    let mut foreign = trace;
    foreign.activation_record.subject = ReleaseIdentifier::new("REL-002").unwrap();
    let result = validate_release_operational_trace(&foreign);
    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == ReleaseFindingCode::TraceabilitySubjectMismatch)
    );
}

#[test]
fn release_history_attachments_require_release_and_version_binding() {
    let release = identity("REL-001", "1.0.0");
    let entry = ReleaseHistoryEntry {
        sequence: 0,
        release: release.clone(),
        state: ReleaseState::Released,
        predecessor: None,
        migration: vec![MigrationRequirementIdentifier::new("MIG-001").unwrap()],
        compatibility: vec![],
        evidence: vec![],
        assurance: vec![],
    };
    let attachment = ReleaseHistoryAttachment {
        id: "MIG-001".into(),
        release: release.id,
        implementation_version: ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
        applicability: Some(ApplicabilityDetermination::Applicable),
    };
    assert!(
        validate_release_history_attachments(
            &[entry],
            &[attachment],
            &ImplementationVersion::new("reference-foundation-0.14.0").unwrap(),
        )
        .valid
    );
}

#[test]
fn successor_history_requires_explicit_applicability_and_preserves_prior_release() {
    let first = identity("REL-001", "1.0.0");
    let second = identity("REL-002", "2.0.0");
    let evidence = EvidenceRecordId::new("EV-001").unwrap();
    let history = vec![
        ReleaseHistoryEntry {
            sequence: 0,
            release: first.clone(),
            state: ReleaseState::Released,
            predecessor: None,
            migration: vec![],
            compatibility: vec![],
            evidence: vec![],
            assurance: vec![],
        },
        ReleaseHistoryEntry {
            sequence: 1,
            release: second.clone(),
            state: ReleaseState::Released,
            predecessor: Some(first.id.clone()),
            migration: vec![MigrationRequirementIdentifier::new("MIG-002").unwrap()],
            compatibility: vec![],
            evidence: vec![evidence.clone()],
            assurance: vec!["ASSURANCE-001".into()],
        },
    ];
    let attachments = vec![ReleaseHistoryAttachment {
        id: "MIG-002".into(),
        release: second.id.clone(),
        implementation_version: second.implementation_version.clone(),
        applicability: Some(ApplicabilityDetermination::Applicable),
    }];
    let valid = validate_release_history_applicability(
        &history,
        &attachments,
        &[EvidenceApplicability {
            evidence: evidence.clone(),
            release: second.id.clone(),
            determination: ApplicabilityDetermination::RequiresRevalidation,
            limitations: vec!["successor requires review".into()],
        }],
        &[AssuranceApplicability {
            assurance: "ASSURANCE-001".into(),
            release: second.id.clone(),
            determination: ApplicabilityDetermination::HistoricalOnly,
            limitations: vec!["not inherited".into()],
        }],
        &second.implementation_version,
    );
    assert!(valid.valid);
    let missing = validate_release_history_applicability(
        &history,
        &attachments,
        &[],
        &[],
        &second.implementation_version,
    );
    assert!(!missing.valid);
}

#[test]
fn operational_composition_requires_typed_boundary_context() {
    let reference = operational_reference("ASSIGNMENT");
    let trace = ReleaseOperationalTrace {
        release: ReleaseIdentifier::new("REL-001").unwrap(),
        implementation_version: reference.implementation_version.clone(),
        assignment: reference.clone(),
        activation_request: operational_reference("REQUEST"),
        activation_decision: operational_reference("DECISION"),
        activation_record: operational_reference("RECORD"),
        recognition: Some(operational_reference("RECOGNITION")),
    };
    assert!(validate_release_operational_trace(&trace).valid);
    let mut missing = trace;
    missing.activation_record.scope = None;
    assert!(!validate_release_operational_trace(&missing).valid);
}
