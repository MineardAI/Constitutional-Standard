# EV-AUTH-0001 - Constitutional Authority and Jurisdiction Realization

Evidence ID: `EV-AUTH-0001`  
Evidence class: Deterministic bounded-domain realization evidence  
Subject: `Constitutional Authority and Jurisdiction Realization`  
Subject version: `CORE-001@0.3.0`  
Governing source: `harmonization-v1.0/CORE-001_Constitutional_Authority_and_Jurisdiction_v0.3.0_Harmonization_Draft.md`  
Implementation version: `reference-foundation-0.4.0`  
Authority realization profile: `reference-implementation-authority-jurisdiction` `1.0.0`

## Bound context and components

The deterministic fixture context uses context identity `ctx-001`, source-set identity `source-set-001`, implementation version `reference-foundation-0.4.0`, and an explicitly admitted `CORE-001@0.3.0` source reference. The evaluation scope covers `constitutional-authority`, `constitutional-contracts`, `constitutional-context` fixture binding, `constitutional-canonical` authority representation, `constitutional-traceability`, and `constitutional-evidence` governance records.

Supported authority concepts are explicit assignments, authority sources, opaque holder/subject/act references, jurisdiction, scope, constraints, bounded delegation, claims, findings, and determinations. Supported jurisdiction concepts are explicit identity, source, boundary, and subject matter references. Supported delegation is non-expanding delegation with equal jurisdiction and contained scope.

## Verification

Focused tests: `crates/constitutional-authority/src/lib.rs` test module.

Positive verification covers supported assignment basis, exact source/context/version binding, constraint satisfaction, non-execution, deterministic evaluation, and canonical encode/decode re-encoding. Negative and prohibition verification covers missing authority, absent admitted source, scope expansion, unsupported constraints, delegation scope expansion, context mismatch, source-set mismatch, and the fact that evaluation never executes the act.

Exact commands:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p constitutional-authority --all-features
```

## Result

Deterministic evidence for the bounded Reference Implementation realization of Constitutional Authority and Jurisdiction; no execution, identity establishment, participation determination, constitutional adoption, conformance, certification, production authorization, activation, or operational-recognition conclusion.

## Limitations, deviations, and provenance

Only explicitly mapped CORE-001 doctrine is realized. The active source remains a harmonization working copy with governance metadata pending adoption; it does not supply a general legal/policy interpreter. Identity, participation, authority proof, persistence, runtime enforcement, constitutional acts/effects, and cross-domain validation remain outside this slice. Prior evidence `EV-CTX-0001`, `EV-CAN-0001`, and `EV-SRC-0001` remain unchanged. This record is local, version-specific, uncommitted evidence for deterministic tests and does not establish conformance or certification.

