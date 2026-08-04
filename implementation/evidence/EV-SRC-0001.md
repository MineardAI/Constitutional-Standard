# EV-SRC-0001 - Controlled Constitutional Source Admission and Resolution

Evidence ID: `EV-SRC-0001`  
Evidence class: Deterministic reference-implementation verification evidence  
Subject: `Controlled Constitutional Source Admission and Resolution`  
Subject version: `reference-implementation-source-admission` `1.0.0`  
Implementation version: `reference-foundation-0.3.0`

## Scope and provenance

This record covers the non-sovereign Reference Implementation Controlled Source Admission Slice. It is associated with the temporary deterministic test-root configuration `TEST-ROOT`, logical root `test-root`, recursive discovery, permitted extensions `.md`, `.txt`, and `.json`, and declared exclusions `.git`, `target`, and `generated`. Canonical references use the declared root identity and normalized relative path; machine-specific absolute paths are operational inputs only.

Supported source types are UTF-8 Markdown and text, plus bounded JSON inputs when explicitly submitted. The content digest algorithm is `SHA-256`, encoded as lowercase hexadecimal over exact source bytes. The generated admitted source-set identity is deterministic in the form `source-set-{SHA-256}` and is bound to the sorted identity-relevant admitted members; it is not a constitutional identity or authority claim.

## Components

Evidence covers `constitutional-contracts`, `constitutional-source-admission`, `constitutional-context`, `constitutional-canonical`, `constitutional-traceability`, `constitutional-evidence`, and `constitutional-test-support`. Filesystem discovery and admission are isolated in `constitutional-source-admission` because they are a replaceable filesystem-boundary responsibility distinct from context resolution, canonical encoding, and evidence conclusions.

## Verification references

Focused tests: `crates/constitutional-source-admission/tests/source_admission.rs`.

The focused suite covers deterministic discovery, metadata preservation and placeholders, SHA-256 identity separation, missing metadata, conflicting aliases, filename mismatch, root/path/extension rejection, conflicting source content, admitted-set construction, context resolution, and Phase 2 canonical round-trip compatibility.

Exact commands:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p constitutional-source-admission --test source_admission
```

Historical evidence `EV-CTX-0001` and `EV-CAN-0001` remains preserved and is not rewritten by this record. This evidence is generated from temporary fixtures and source metadata patterns; it does not mutate constitutional source directories.

## Result

Deterministic controlled-source admission and resolution evidence; no constitutional applicability, authority, adoption, conformance, certification, production, activation, or operational-recognition conclusion.

## Limitations and deviations

The slice performs bounded metadata inspection only; it does not parse full Markdown semantics, extract requirements, resolve amendments, adjudicate authoritative precedence, determine applicability, verify Git history or signatures, watch files, persist a registry, or assess conformance/certification. Explicit placeholders remain values, while missing required identity/version metadata and unsafe or conflicting inputs fail closed. The prior fixture-only source catalog limitation is remediated only for bounded file admission, and the prior non-cryptographic comparison aid is superseded for admitted content by SHA-256 but remains retained for its historical Phase 1 comparison scope.

## Non-claims

File presence, discovery, admission, digest equality, passing tests, mappings, canonical bytes, and this evidence record do not create constitutional authority, applicability, adoption, effectiveness, operational recognition, conformance, certification, production authorization, or software constitutional authority.
