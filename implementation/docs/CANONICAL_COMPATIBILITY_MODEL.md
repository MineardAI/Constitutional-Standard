# Canonical Compatibility Model

`canonical_compatibility` reports only bounded representation-level compatibility:

- `ExactRepresentationVersionMatch` — same valid kind, schema, and representation version;
- `SupportedCompatibleVersion` — same valid kind/schema and numeric major with a non-decreasing minor version;
- `UnsupportedVersion` — valid representation versions outside that bounded relationship;
- `IncompatibleRepresentationKind` — different representation kinds;
- `IncompatibleSchemaProfile` — different schema/profile identifiers;
- `Indeterminate` — versions cannot be interpreted by the bounded numeric model; and
- `MalformedRepresentation` — either object fails structural validation.

These results do not imply source compatibility, semantic equivalence, migration safety, release compatibility, interoperability, conformance, authorization, activation, or operational compatibility. Release continuity remains governed by IMP-008.

