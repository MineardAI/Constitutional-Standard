# Dependency Model

```text
constitutional-contracts
        ↑
context   canonical   validation   traceability
        ↑             ↑             ↑
              evidence

constitutional-source-admission → constitutional-contracts
constitutional-source-admission (integration tests) → constitutional-context → constitutional-contracts
constitutional-source-admission (integration tests) → constitutional-canonical
```

The dependency graph is inward and acyclic. The workspace uses no external crates, services, databases, network clients, asynchronous runtime, or serialization framework.

`constitutional-authority` depends on `constitutional-contracts`. `constitutional-canonical` depends on `constitutional-authority` only for authority-evaluation representation. Authority tests use context/source-set fixtures without moving context resolution into the authority crate.

`constitutional-identity` depends on `constitutional-contracts` and uses opaque authority-determination references. `constitutional-canonical` depends on `constitutional-identity` only for identity and participation representation. Identity evaluation does not recreate authority, context resolution, or source admission.

`constitutional-artifacts` depends on `constitutional-contracts` and uses opaque authority, identity, custody, registry, and cryptographic references. `constitutional-canonical` depends on `constitutional-artifacts` only for artifact/provenance representations. Artifact evaluation does not own storage, publication, signing, evidence assessment, or source admission.

`constitutional-state` depends on `constitutional-contracts` and uses opaque authority, participation, artifact, provenance, and evidence references. `constitutional-canonical` depends on `constitutional-state` only for state/transition representations. State evaluation does not own runtime mutation, workflows, persistence, or execution.

`constitutional-interaction` depends on `constitutional-contracts` and owns bounded interaction/boundary evaluation only. `constitutional-core` depends on the five domain crates and `constitutional-contracts`; it does not depend on or replace domain ownership and provides no runtime, transport, persistence, or execution interface. `constitutional-canonical` depends on `constitutional-core` only for integration representation.

Phase 11 extends `constitutional-traceability` in place. It depends on `constitutional-contracts` and does not create a parallel traceability crate, policy engine, serializer, transport layer, persistence layer, or execution interface. IMP-005 may later consume the typed traceability seam for canonical representation; that dependency is planned, not implemented here.

Phase 12 extends `constitutional-canonical` in place with generic representation mechanics. It continues to depend on contracts and the existing domain crates only for domain-owned wrapper representations. The shared module does not depend on traceability, does not move domain semantics, and does not add a transport, persistence, admission, execution, assurance, or policy component.

Phase 13 extends `constitutional-validation` in place. Validation depends on contracts, canonical representation mechanics, and traceability bindings; it does not depend on or own `constitutional-evidence` objects. The dependency direction preserves evidence ownership for the existing evidence component and keeps verification from becoming execution, persistence, transport, admission, assurance authority, or conformance logic.

Phase 15 adds `constitutional-release` as the owner of IMP-008 release continuity and compatibility semantics. It depends on contracts, canonical mechanics, evidence references, and traceability references only. It does not perform publication, migration execution, activation, deployment, persistence, transport, or operational recognition.

Phase 16 adds `constitutional-activation` as the owner of IMP-009 activation and operational-recognition semantics. It references release bindings, authority/jurisdiction identifiers, evidence, assurance, traceability, and implementation context without redefining those domains. It performs no runtime execution, deployment, publication, persistence, transport, or external-environment mutation.
