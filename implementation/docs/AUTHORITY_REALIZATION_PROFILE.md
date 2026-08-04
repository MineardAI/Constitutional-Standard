# Reference Implementation Authority and Jurisdiction Realization Profile

Profile identifier: `reference-implementation-authority-jurisdiction`  
Profile version: `1.0.0`  
Governing source: `CORE-001_Constitutional_Authority_and_Jurisdiction_v0.3.0_Harmonization_Draft.md` (`CORE-001@0.3.0` harmonization addendum)

This is a bounded, non-sovereign Reference Implementation profile. It realizes only explicit authority assignments and the CORE-001 distinctions present in the active harmonization working copy: Authority, Jurisdiction, Assigned Authority, scope, limitations, delegation preservation/non-expansion, separation of Operational Execution from Constitutionally Effective Acts, and non-creation by identity, representation, possession, publication, or certification.

## Supported concepts and operations

The profile represents typed authority sources, opaque holder/subject/act references, explicit jurisdiction, bounded scope, explicit constraints, delegation references, claims, assignments, findings, and determinations. It provides `validate_authority_claim`, `evaluate_authority`, and deterministic canonical request/result representation through `constitutional-canonical`.

Determinations are `Supported`, `Denied`, `Conflict`, `Indeterminate`, `NotApplicable`, and `InvalidRequest`. A supported result carries an explicit assignment basis. Missing or mismatched context, source-set, implementation version, authority, holder, subject, act, jurisdiction, scope, or constraints fail closed or remain indeterminate as appropriate. Unsupported constraint semantics remain indeterminate.

Delegation is bounded to non-expansion: the assignment must declare delegation, the source assignment must match, jurisdiction must remain equal, and delegated scope must be contained by the original scope. Identity and participation are opaque references; their semantics are not implemented here.

## Source and context boundary

Evaluation consumes an authority evaluation context containing explicit assignments, admitted source references, context identity, source-set identity, and implementation version. Source admission alone does not create an assignment or authority. File presence, source naming, capability, permission, role labels, identity references, canonical bytes, and passing tests do not establish authority.

## Exclusions and non-authority boundary

This profile does not implement identity, authentication, credentials, participation, membership, RBAC/IAM/ACL enforcement, policy-language evaluation, execution, state, interaction, boundary crossing, persistence, networking, signatures, conformance, certification, activation, or operational recognition. It does not execute the claimed act and does not establish constitutional effect.
