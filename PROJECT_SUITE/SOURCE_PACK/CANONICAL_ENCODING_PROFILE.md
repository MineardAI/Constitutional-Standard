# Reference Implementation Canonical Encoding Profile

Profile identifier: `macs-reference-canonical-context`

Representation version: `1.0.0`

This Phase 2 profile is a single UTF-8, line-oriented encoding using `field=value` records terminated by LF. Values use percent-encoding for all bytes outside the unreserved ASCII set. Fields are emitted in a fixed order; source references are sorted by source identifier, source version, and path.

Optional metadata is explicit: `absent` is distinct from `present:value`. The decoder rejects invalid UTF-8, malformed lines, duplicate fields, unknown fields, missing fields, unsupported format/version values, duplicate or conflicting source references, non-canonical ordering, and trailing content.

This is a bounded Reference Implementation profile, not a universal MACS serialization or wire protocol. Encoding, decoding, hashing, and round-trip stability do not establish constitutional reality, identity, authority, applicability, truth, acceptance, conformance, certification, or effect.

Phase 3 adds an optional `admitted_source_set` field carrying the deterministic source-set identifier, admission-report identifier, root identifiers, exact source identities, normalized relative paths, SHA-256 digest algorithm/value pairs, and admission statuses. Raw source bodies and absolute machine paths are never included.

Phase 4 adds the bounded `macs-reference-canonical-authority-evaluation` profile, version `1.0.0`. It preserves evaluation, operation, context, source-set, implementation/profile versions, claim summary, jurisdiction, basis, determination, findings, execution status, and non-claims in fixed field order. It contains no executable act, filesystem path, source body, identity proof, or authority-creating operation.

Phase 5 adds the bounded `macs-reference-canonical-identity-evaluation` and `macs-reference-canonical-participation-evaluation` profiles, each version `1.0.0`. They preserve claim summaries, basis/distinction/continuity or participation scope summaries, context/source-set/implementation/profile versions, determinations, findings, non-authentication/non-execution flags, and non-claims. They do not serialize credentials, secrets, accounts, authentication state, source bodies, absolute paths, or executable operations.

Phase 6 adds bounded version `1.0.0` representations for artifact identity, artifact instance identity, recognition basis, provenance, relationships, integrity claims, and artifact evaluation results. These forms distinguish artifact identity, instance identity, serialization identity, and evidence identity; they contain no raw source bodies, storage state, signatures, truth claims, or publication operations.

Phase 7 adds bounded version `1.0.0` representations for state identity, proposed transitions, and state/transition evaluation results. These forms preserve context/source-set/version bindings, state and transition identities, effects, determination families, projection status, execution/mutation flags, and non-claims. Projected state is never encoded as actual resulting state.

Phase 9 adds the bounded `macs-reference-canonical-core-evaluation` profile, version `1.0.0`. It preserves integration, dependency, invariant, binding, domain-reference, finding, evidence, identity, and non-execution fields in strict deterministic order. It does not merge domain identities or create constitutional effect.

Phase 11 boundary: the bounded IMP-004 traceability realization introduces no canonical trace encoding. Its graph is an in-memory typed validation surface only. Canonical trace object inventory, encoding, strict decoding, ordering, and round-trip obligations remain owned by the planned IMP-005 canonical representation expansion.
