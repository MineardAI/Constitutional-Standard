# Constitutional Standard

Constitutional Standard is a constitutional envelope and deterministic Reference Foundation for governed artificial intelligence.

![Constitutional Standard](implementation/docs/Constitutional-Standard-image.png)

## What it is

The project provides a bounded, typed Rust realization of the records and checks needed to describe an AI operation in constitutional context. It makes source admission, authority boundaries, identity and participation, artifact provenance, state, interaction boundaries, canonical representation, traceability, verification, evidence, release continuity, and operational-recognition records explicit and inspectable.

It is a reference implementation. It does not adopt a constitution, decide what is true, grant authority, certify conformance, or operate an AI system.

## Why it exists

Governed AI needs more than an input and an output. A reviewable operation also needs a declared source set, baseline, jurisdiction, scope, effective times, implementation version, provenance, verification obligations, evidence references, and explicit limits on what a record means.

Constitutional Standard supplies deterministic data structures and validation boundaries for those concerns. The result is a small foundation that can be inspected, tested, serialized, and connected to a larger governed system without silently becoming that system's sovereign authority or execution engine.

## Current implementation

The current public implementation is `reference-foundation-0.15.0`. It contains the following bounded domains:

| Domain | Implemented responsibility |
| --- | --- |
| Contracts | Shared identifiers, references, findings, timestamps, source and implementation records |
| Source admission | Typed admission records and fail-closed handling of unavailable or conflicting source input |
| Context | Operation-scoped resolution of required sources, baseline, jurisdiction, scope, times, and boundaries |
| Authority | Explicit authority evaluation records with claims, basis, findings, and non-claims |
| Identity and participation | Canonical identity and participation records without asserting real-world identity or agency |
| Artifacts and provenance | Artifact references, provenance, custody, and origin records |
| State and transition | Typed state and transition records with bounded transition findings |
| Interaction and boundary | Interaction records and explicit boundary conditions |
| Canonical representation | Strict, deterministic, line-oriented encodings, decoding, validation, and identity digests |
| Traceability | Source-to-requirement-to-component-to-test/evidence mappings and graph validation |
| Verification and assurance | Typed verification activities, findings, assessments, and bounded assurance results |
| Evidence and provenance | Evidence objects, origins, custody, integrity, conflicts, admission, and sufficiency boundaries |
| Release continuity | Release records, compatibility, change impact, and continuity relationships |
| Activation and recognition | Typed activation and operational-recognition records that remain non-executing and non-authoritative |

These domains record and validate declared information. They do not infer constitutional meaning, resolve unresolved authority, invoke represented components, persist state, transport messages, or perform an operation.

## Architecture

The workspace is organized as small Rust crates with explicit responsibilities:

```text
constitutional-contracts
        |
        +--> source-admission --> context --> core
        |
        +--> authority, identity, artifacts, state, interaction
        |
        +--> canonical <--> validation
        |
        +--> traceability --> evidence --> release --> activation
```

`constitutional-test-support` provides shared fixture construction for the workspace tests. The public crate boundaries and responsibilities are catalogued in [`implementation/docs/COMPONENT_CATALOG.md`](implementation/docs/COMPONENT_CATALOG.md) and [`implementation/docs/COMPONENT_RESPONSIBILITY_MATRIX.md`](implementation/docs/COMPONENT_RESPONSIBILITY_MATRIX.md).

## Determinism and traceability

Canonical representations use an explicit format and version, ordered fields, typed values, strict decoding, and stable identity digests. Non-canonical input is rejected rather than normalized silently. Canonical output is therefore suitable for repeatable comparison and evidence binding within this reference profile; it is not a universal wire protocol or a substitute for constitutional interpretation.

Traceability records relationships among sources, requirements, components, artifacts, implementation versions, verification activities, evidence, gates, and lifecycle records. A traceability result describes the graph that was checked. It does not prove correctness, establish conformance, authorize a release, or certify a system.

Verification and evidence are similarly bounded. They preserve what was checked, by whom or what record, against which scope and implementation version, and with which findings. They do not convert a test result or evidence package into constitutional authority.

Release continuity and activation records describe declared compatibility, lifecycle relationships, activation conditions, and operational-recognition boundaries. Creating such a record does not release software, activate a system, or recognize operational effectiveness.

## Non-sovereign, non-executing design

The Reference Foundation is deliberately non-sovereign and non-executing:

- it does not create or transfer constitutional authority;
- it does not adopt or amend a constitution;
- it does not decide whether a constitutional source is substantively correct;
- it does not execute AI actions, make operational decisions, or compel downstream behavior;
- it does not provide transport, persistence, deployment, orchestration, secrets management, or a production control plane;
- it does not claim certification, conformance, production readiness, or real-world operational effectiveness.

Unknown, missing, conflicting, ambiguous, or out-of-scope inputs are represented as typed findings or indeterminate results where the relevant API requires it.

## Repository structure

- [`implementation/Cargo.toml`](implementation/Cargo.toml) — Rust workspace manifest.
- [`implementation/crates/`](implementation/crates/) — public implementation crates and tests.
- [`implementation/docs/`](implementation/docs/) — realization profiles, boundaries, component records, and implementation documentation.
- [`implementation/evidence/`](implementation/evidence/) — implementation evidence records associated with the public realization.
- [`implementation/docs/IMPLEMENTATION_SCOPE.md`](implementation/docs/IMPLEMENTATION_SCOPE.md) — bounded scope and exclusions.
- [`implementation/docs/CANONICAL_ENCODING_PROFILE.md`](implementation/docs/CANONICAL_ENCODING_PROFILE.md) — canonical encoding profile.
- [`implementation/docs/TRACEABILITY_BOUNDARY_AND_NON_AUTHORITY.md`](implementation/docs/TRACEABILITY_BOUNDARY_AND_NON_AUTHORITY.md) — traceability limits.
- [`implementation/docs/VERIFICATION_AND_ASSURANCE_NON_AUTHORITY.md`](implementation/docs/VERIFICATION_AND_ASSURANCE_NON_AUTHORITY.md) — verification and assurance limits.

## Usage

The workspace is library-first. For example, a consumer can construct a validated trace reference using the public traceability API:

```rust
use constitutional_traceability::TraceRef;

let reference = TraceRef::new("evidence/EV-001").expect("reference must be non-empty and whitespace-free");
assert_eq!(reference.as_str(), "evidence/EV-001");
```

The implementation also exposes typed context resolution and canonical representation APIs. Applications should build those records from their own admitted inputs and treat indeterminate results and validation findings as meaningful outcomes, not as permission to guess.

## Build and verify

From the `implementation` directory:

```text
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The workspace targets Rust edition 2024 and forbids unsafe Rust at the workspace lint level. Generated build output is not part of the repository.

## Status and limitations

`reference-foundation-0.15.0` is the current implementation version represented by the public workspace. It is a bounded reference implementation, not a 1.0 authorization, constitutional adoption, certification, or production release.

The implementation does not supply constitutional source interpretation, governance policy, external identity proofing, an execution runtime, deployment automation, persistence, transport, a registry, a security boundary, or an operational authority. Integrators remain responsible for their own source governance, infrastructure, threat model, access control, data handling, and deployment decisions.

## Contributing

Contributions should preserve the explicit, typed, deterministic, non-sovereign, and non-executing boundaries of the project. Keep changes scoped to the affected crate, add behavioral tests for new behavior, document any new limitation or non-claim, and run the formatting, workspace test, and Clippy commands above before submitting a change.

There is not currently a separate contribution policy in this repository. The implementation and its public boundary documents are the source of truth for supported behavior.

## Security

Do not commit credentials, private keys, personal data, or production evidence to this repository. A dedicated security policy is not currently included; report suspected vulnerabilities privately to the project maintainers rather than publishing exploitable details in an issue.

## License

No public open-source license is currently provided. The workspace metadata identifies the implementation as `UNLICENSED`; obtain permission from the project owner before copying, modifying, or redistributing it.
