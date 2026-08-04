# EV-CAN-0001

- Evidence ID: `EV-CAN-0001`
- Evidence class: deterministic implementation evidence
- Subject: `Canonical Operational Constitutional Context Representation`
- Subject version: representation profile `macs-reference-canonical-context` v`1.0.0`
- Implementation version: `reference-foundation-0.2.0`
- Source commit identifier: `uncommitted local workspace; no commit exists`
- Workspace: `implementation`
- Components: `constitutional-contracts`, `constitutional-canonical`, `constitutional-evidence`, `constitutional-test-support`
- Tests: `crates/constitutional-evidence/tests/vertical_slice.rs`
- Fixture: complete source catalog and operation-scoped context fixture
- Commands:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
- Result: 12 tests passed; canonical bytes are deterministic; strict decoding rejects bounded invalid inputs; valid bytes round-trip and re-encode identically.
- Provenance: produced from the local implementation workspace and deterministic fixture catalog on 2026-07-29.
- Known limitations: context-only representation; fixture source catalog; no full parser, persistence, transport, cryptographic integrity, or assurance evaluation.
- Deviations: bounded textual reference profile, not a universal MACS serialization or wire protocol.
- Non-claim: Deterministic canonical-context representation evidence; no conformance, certification, authority, production, activation, or operational-recognition conclusion.

