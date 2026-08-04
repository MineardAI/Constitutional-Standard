# Verification Obligations

- Identical canonical inputs produce identical representations and comparison digests.
- Required source, version, scope, jurisdiction, and context elements are explicit.
- Missing or ambiguous inputs fail closed with structured findings.
- Effective, observation, and processing times remain distinct.
- Evidence is bound to the exact implementation version under test.
- Canonical decoding rejects unknown, duplicate, missing, unsupported, ambiguous, conflicting, malformed, non-canonical, and trailing input.
- Re-encoding a valid decoded representation returns byte-identical canonical output.
- Validation and passing tests produce no authority, conformance, or certification claim.

Phase 11 traceability obligations: preserve source identity, source version, implementation version, explicit relation classes, deterministic findings, mandatory path checks, and non-authority flags. Verification demonstrates repeatable structural behavior only; it does not establish requirement satisfaction, assurance, evidence sufficiency, conformance, certification, release authority, activation, execution, or operational recognition.
