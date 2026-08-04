# Reference Implementation Artifact and Provenance Realization Profile

Profile identifier: `reference-implementation-artifacts-provenance`  
Profile version: `1.0.0`

## Governing source binding

Admitted source: `harmonization-v1.0/CORE-003_Constitutional_Artifacts_and_Provenance_v0.2.0_Harmonization_Draft.md`.

Exact internal observation: Identifier `CORE-003`; primary metadata Version `0.1.1`; Status `Stabilization Draft`; Revision Type `Artifact Identity–Instance Stabilization`. The file name and later harmonization addendum identify a `0.2.0` working copy. This implementation preserves the discrepancy as `PRIMARY_SOURCE_VERSION=0.1.1` and `ADDENDUM_SOURCE_VERSION=0.2.0`; it does not amend, normalize, or silently select between them.

## Bounded realization

The profile models Constitutional Representation, Artifact, Constitutional Artifact, Non-Constitutional Artifact, Constitutional Record, Normative Artifact, Evidence Artifact, Reference Artifact, Historical Artifact, Operational Artifact, Artifact Identity, Artifact Instance Identity, recognition basis, roles, provenance, lineage, integrity, relationships, copying, transformation, replay, compound containment, and historical preservation.

Artifact recognition requires an explicit governing-source basis, available source binding, expressly recognized role, and complete artifact identity. Non-Constitutional Artifact classification remains distinct from constitutional recognition. Artifact Identity and Artifact Instance Identity are always separate; identity-preserving instantiation requires an explicit basis and byte equality is insufficient.

Provenance and integrity are separate determinations. Provenance does not prove truth, validity, authority, effectiveness, authenticity, conformance, or certification. Integrity does not prove truth, constitutional validity, or effectiveness. Replay does not create a new act or effect. Containment does not merge component identities. Historical preservation does not create current applicability.

## Supported operations and determinations

Supported operations include `evaluate_artifact_recognition`, `evaluate_artifact_instance`, `evaluate_identity_preservation`, `evaluate_provenance`, `evaluate_artifact_integrity`, and `evaluate_artifact_relationship`.

Separate determinations are provided for artifact recognition, instance validity, identity preservation, provenance sufficiency, and integrity. Findings preserve conflict and indeterminacy rather than collapsing them into a boolean or a general validity claim.

## Explicit non-scope

This profile does not implement storage, databases, registries, publication services, rendering, document management, signing, cryptographic verification, key management, custody enforcement, evidence assessment, truth evaluation, effectiveness, conformance, certification, activation, execution, state, interaction, policy interpretation, or source amendment.
