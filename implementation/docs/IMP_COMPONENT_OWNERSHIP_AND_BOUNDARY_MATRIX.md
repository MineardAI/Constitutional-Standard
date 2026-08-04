# IMP Component Ownership and Boundary Matrix

| IMP responsibility | Existing owner / planned owner | Inputs | Outputs | Prohibited semantics |
|---|---|---|---|---|
| IMP-000 doctrine | contracts, scope/status/baseline records | admitted source bindings, implementation scope | non-sovereignty and limitation records | authority, interpretation, adoption |
| IMP-001 architecture | component/dependency records, `constitutional-core` coordination | domain ownership and dependency declarations | architecture mappings and gate inputs | architecture-derived authority, forced topology |
| IMP-002 repository/component model | Cargo workspace, manifests, component/responsibility matrices | source/component identities and dependencies | attributable component records | repository sovereignty or hidden ownership |
| IMP-003 lifecycle/gates | status, evidence index, deviation/change-impact records | declared scope, criteria, evidence | scope-bound gate dispositions | release, activation, conformance, certification |
| IMP-004 traceability | `constitutional-traceability` (extend) | requirements, components, code, tests, evidence, versions | typed forward/reverse/negative/change-impact mappings | correctness adjudication or constitutional interpretation |
| IMP-005 canonical representation | `constitutional-canonical` (extend) | typed public objects and representation profiles | deterministic strict encodings/decoders | transport, admission, reality, identity collapse |
| IMP-006 verification/assurance | `constitutional-validation` (extend), consuming traceability/canonical bindings | obligations, findings, supplied test results, mappings, typed evidence references | verification results and explicitly scoped assurance-boundary records | evidence ownership/sufficiency, conformance/certification, release, activation, operational recognition, execution |
| IMP-007 evidence/provenance | `constitutional-evidence` + traceability (extend) | evidence subjects, sources, versions, provenance | attributable evidence identities and packages | truth, authority, sufficiency, certification |
| IMP-008 release/continuity | `constitutional-release` (new dedicated crate) | version lineage, compatibility claims, evidence, verification, assurance, and traceability references | release, compatibility, continuity, and migration records | deployment, activation, operational recognition, authority, conformance, certification |
| IMP-009 activation/recognition | `constitutional-activation` (new dedicated crate) | release, compatibility, evidence, assurance, authority, jurisdiction, context, and traceability references | bounded eligibility, activation, assignment, recognition, standing, and historical transition records | runtime activation, deployment, self-authorization, authority creation, conformance, certification |

Closure remediation preserves these owners. Release resolution remains release-owned, evidence integrity/custody/package/conflict validation remains evidence-owned, and activation authority/jurisdiction/authorization resolution remains activation-owned over typed references; no ownership is relocated across crates.

The second remediation extends those same owners with activation-owned lifecycle/recognition and activation-chain validation, evidence-owned finding/custody-chain validation, and release-owned graph/history/subject-bound reference validation. Cross-domain traceability validates typed connections only and does not transfer semantic ownership.

The five CORE domain crates and `constitutional-core` remain outside IMP semantic ownership. IMP records may map or verify them but may not relocate their authority, identity, artifact, state, or interaction meaning.
