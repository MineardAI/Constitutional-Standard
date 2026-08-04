# Canonical Identity Participation Profile

Only fields explicitly marked `identity_participates: true` contribute to the bounded canonical identity digest. The current shared model includes:

- representation identifier;
- representation kind;
- representation version;
- schema/profile identifier;
- source binding;
- implementation-version binding; and
- identity-participating canonical fields, including nested values.

Non-participating fields include display-only values, operational extensions, diagnostics, validation execution order, transport metadata, persistence metadata, and incidental notes. The digest is the existing deterministic FNV-1a fixture digest rendered as `fnv1a64:<hex>`; it is not a cryptographic integrity, authenticity, provenance, evidence, legal, or constitutional-validity claim.

Changing a non-participating field preserves the digest in the tested model. Changing an identity-participating field changes it. Representation identity remains distinct from constitutional object identity, serialization instance identity, stored artifact identity, and exchange package identity.

