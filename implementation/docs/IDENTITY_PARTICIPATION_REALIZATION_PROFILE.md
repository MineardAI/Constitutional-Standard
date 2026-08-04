# Reference Implementation Identity and Participation Realization Profile

Profile identifier: `reference-implementation-identity-participation`  
Profile version: `1.0.0`  
Governing source: `CORE-002_Constitutional_Identity_and_Participation_v0.2.0_Harmonization_Draft.md` (`CORE-002@0.2.0`)

This is a bounded, non-sovereign Reference Implementation profile. It realizes explicit CORE-002 concepts from the active harmonization working copy: Recognized Identity, Identity Standing, Recognition Scope, identity basis, identity distinction, identity continuity references, Participation Relationship, Procedural Standing references, participation basis, participation scope, participation constraints, and recognition/participation non-authority boundaries.

## Identity model

An identity claim requires an explicit basis containing a constitutional source reference, represented entity, identity class, recognition scope, effective status, and provenance. The profile preserves identity subject and representation as distinct typed references. A representation reference is not the identity, and names, files, roles, credentials, authentication-like data, source admission, or matching digests are not identity basis by themselves.

Identity evaluation produces `Supported`, `Denied`, `Conflict`, `Indeterminate`, `NotApplicable`, or `InvalidRequest`. It checks admitted basis-source availability, mandatory basis attributes, identity collisions, distinction, and bounded continuity. Continuity, succession, substitution, and replacement require explicit basis references; unsupported continuity remains indeterminate.

## Participation model

A participation claim requires an explicit participation basis, subject, and scope. The claim carries an identity-determination reference bound to the same context, source set, and implementation version. Participation scope cannot expand beyond the represented basis scope. Constraints are explicit; unsatisfied constraints deny and unsupported constraint semantics remain indeterminate.

Participation evaluation produces `Eligible`, `Admitted`, `NotAdmitted`, `Denied`, `Conflict`, `Indeterminate`, `NotApplicable`, or `InvalidRequest`. This profile uses `Eligible` for a supported bounded evaluation; it does not activate participation or establish authority. Required authority references remain opaque and are never reconstructed or evaluated by this crate.

## Exclusions and non-claims

This profile does not implement authentication, credentials, accounts, login, identity proofing, external identity providers, DID/VC protocols, role or permission administration, authority creation, runtime authorization, state, interaction, boundary crossing, execution, persistence, registries, signatures, conformance, certification, activation, or operational recognition. It does not create an identity or participant merely by evaluating a claim.
