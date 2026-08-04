//! Deterministic fixtures used only by tests.

use constitutional_context::{ContextResolutionRequest, SourceCatalog};
use constitutional_contracts::*;

pub fn source(id: &str, version: &str, path: &str) -> ConstitutionalSourceRef {
    ConstitutionalSourceRef::new(
        ConstitutionalSourceId::new(id).unwrap(),
        ConstitutionalSourceVersion::new(version).unwrap(),
        path,
    )
    .unwrap()
}

pub fn complete_request() -> ContextResolutionRequest {
    complete_request_with_version("reference-foundation-0.15.0")
}

pub fn complete_request_with_version(version: &str) -> ContextResolutionRequest {
    ContextResolutionRequest {
        operation_id: OperationId::new("op-fixture-001").unwrap(),
        required_sources: vec![
            ConstitutionalSourceId::new("AFD-002").unwrap(),
            ConstitutionalSourceId::new("IMP-000").unwrap(),
        ],
        baseline: Some(BaselineRef::new("draft-baseline-2026-07").unwrap()),
        jurisdiction: Some(JurisdictionRef::new("implementation-reference").unwrap()),
        scope: Some(ScopeRef::new("context-vertical-slice").unwrap()),
        effective_context: Some(EffectiveContextRef::new("fixture-effective-context").unwrap()),
        boundary_context: Some(BoundaryContextRef::new("non-sovereign-reference").unwrap()),
        constitutional_effective_time: Some(TimePoint::new("2026-07-29T00:00:00Z").unwrap()),
        observation_time: Some(TimePoint::new("2026-07-29T12:00:00Z").unwrap()),
        processing_time: Some(TimePoint::new("2026-07-29T12:00:01Z").unwrap()),
        implementation_version: Some(ImplementationVersion::new(version).unwrap()),
    }
}

pub fn complete_catalog() -> SourceCatalog {
    SourceCatalog::new(vec![
        source(
            "AFD-002",
            "1.0.0",
            "harmonization-v1.0/AFD-002_Implementation_Architecture_Freeze_Declaration_v1.0.md",
        ),
        source(
            "IMP-000",
            "0.2.0",
            "harmonization-v1.0/IMP-000_Implementation_Constitution_v0.2.0_Candidate_Review_Draft.md",
        ),
    ])
}
