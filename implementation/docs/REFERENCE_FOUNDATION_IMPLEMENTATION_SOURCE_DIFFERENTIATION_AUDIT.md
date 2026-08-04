# Reference Foundation Implementation-to-Source Differentiation Audit

Audit status: `IMPLEMENTATION-TO-SOURCE DIFFERENTIATION REVIEW COMPLETE`

Implementation version: `reference-foundation-0.15.0`

Repository status: `NOT READY FOR REFERENCE FOUNDATION V1.0 CLOSURE`

This is an analytical, review-only audit. No implementation code, tests, or constitutional source documents were changed. `reference-foundation-1.0.0` remains unauthorized.

## Review basis and source-status rule

The source register identifies `harmonization-v1.0/AFD-002_Implementation_Architecture_Freeze_Declaration_v1.0.md` as the controlling architecture declaration. The implementation source baseline is:

| Family | Governing source used | Status/discrepancy |
|---|---|---|
| CORE-000 | `CORE-000_Constitutional_Charter_and_Canonical_Constitutional_Ontology.txt` | Current root ontology input; implementation may not redefine it. |
| CORE-001 | `CORE-001_Constitutional_Authority_and_Jurisdiction_v0.2.2_Baseline_Harmonization.md` | Baseline harmonization input; earlier v0.2.0/v0.2.1 files retained historically. |
| CORE-002 | `CORE-002_Constitutional_Identity_and_Participation_v0.1.2_Semantic_Harmonization_Draft.md` | Semantic harmonization draft. |
| CORE-003 | `CORE-003_Constitutional_Artifacts_and_Provenance_v0.1.1_Stabilization_Draft.md` | Stabilization draft. |
| CORE-004 | `CORE-004_Constitutional_State_and_Transition_Doctrine_v0.2.0_Revised_Baseline_Draft.md` | Revised baseline draft. |
| CORE-005 | `CORE-005_Constitutional_Interaction_and_Boundary_Doctrine_v0.2.0_Revised_Baseline_Draft.md` | Primary v0.2.0; harmonization working copy/filename discrepancy is preserved by the source register. |
| IMP-000–004 | `harmonization-v1.0/IMP-000...IMP-004` | IMP-000/001 candidate review v0.2.0 where registered; IMP-002/003/004 v0.1.0 drafts. |
| IMP-005 | `IMP-005_Canonical_Constitutional_Representation_Model_v0.1.0_Draft.md` | Draft; definition-registry adoption remains pending. |
| IMP-006–007 | v0.1.0 draft files | Draft implementation inputs, not adopted constitutional authority. |
| IMP-008–009 | v0.2.0 candidate-review draft files | Candidate-review implementation inputs; release/activation claims remain bounded. |

Architecture and ownership records were used to distinguish a permitted realization from an unsupported source claim. The original audits, remediation reports, re-audits, criteria freeze, and third-remediation scope were used only as implementation history and audit-causation evidence.

## Part I — Source obligation extraction

The following 46 obligations are the source-level obligations used for differentiation. They are intentionally stated before mapping implementation detail.

| Source ID | Source version | Clause or section | Source obligation | Obligation type | Owning domain | Required observable behavior |
|---|---|---|---|---|---|---|
| SRC-000-01 | CORE-000 current root | §6, §7 | Constitutional ontology distinguishes subject, act, effect, relationship, state, representation, artifact, and record. | required object/distinction | CORE | Implementation does not collapse these categories. |
| SRC-000-02 | CORE-000 current root | §15, §19 | Canonical modeling and universal invariants preserve constitutional distinctions. | required invariant | CORE | Representation and validation preserve distinctions. |
| SRC-000-03 | CORE-000 current root | §5, §23 | Subordinate specifications do not redefine CORE objects or invariants absent authorized amendment. | required boundary | All domains | No implementation or IMP profile silently amends CORE meaning. |
| SRC-001-01 | CORE-001 v0.2.2 | authority/jurisdiction model | Authority and jurisdiction are explicit, scoped, and distinct from implementation behavior. | required relationship/invariant | authority | Validation requires explicit authority/scope references. |
| SRC-001-02 | CORE-001 v0.2.2 | harmonized invariants | Composition does not expand authority or create certification. | required prohibition | authority/core | Composition remains non-authoritative and non-certifying. |
| SRC-001-03 | CORE-001 v0.2.2 | jurisdiction doctrine | Jurisdiction is bounded by declared scope and applicable context. | required validation | authority | Out-of-scope requests fail closed. |
| SRC-002-01 | CORE-002 v0.1.2 | §§2–6 | Identity standing, procedural standing, recognition, and participation are distinct models. | required distinction | identity | Records do not equate identity, participation, or recognition. |
| SRC-002-02 | CORE-002 v0.1.2 | §§7, 12 | Distinct recognized identities remain distinct subjects; recognition does not create authority. | required invariant/prohibition | identity | Identity composition preserves subject distinction and non-authority. |
| SRC-002-03 | CORE-002 v0.1.2 | §§10–11 | Participation eligibility and governance designation do not themselves create authority. | required prohibition | identity | Participation checks remain bounded and non-authoritative. |
| SRC-003-01 | CORE-003 v0.1.1 | §§5–8, 11 | Representation, artifact, constitutional artifact, and record are distinct. | required distinction | artifacts | Artifact records preserve representation/artifact/record separation. |
| SRC-003-02 | CORE-003 v0.1.1 | §§17, 20, 23 | Identity, provenance, transformation, and derivation are attributable and distinct. | required relationship/history | artifacts | Copies/derivations preserve identity and lineage rules. |
| SRC-003-03 | CORE-003 v0.1.1 | §§17.1–17.6 | Artifact identity and instance identity remain distinct; identity-preserving instantiation is explicit. | required invariant | artifacts | Instance validation does not collapse identities. |
| SRC-003-04 | CORE-003 v0.1.1 | §18 | Role composition does not erase functional distinctions or expand authority. | required prohibition | artifacts | Artifact roles remain non-authoritative. |
| SRC-004-01 | CORE-004 v0.2.0 | §§3–8 | State is scoped to a primary subject, dimension, context, and transition model. | required object/relationship | state | State claims carry subject/context/basis. |
| SRC-004-02 | CORE-004 v0.2.0 | §§8–12 | Transitions have preconditions and constraints; suspension, expiration, revocation, and restoration are distinct. | required validation/distinction | state | Invalid transitions fail; distinct states are not merged. |
| SRC-004-03 | CORE-004 v0.2.0 | §§15, 19 | Historical state is preserved; state does not become current reality merely by representation. | required historical boundary | state | History remains attributable and non-authoritative. |
| SRC-004-04 | CORE-004 v0.2.0 | §19 | Concurrent/scoped state and non-mutation boundaries are preserved. | required invariant | state | Validation does not execute or mutate external state. |
| SRC-005-01 | CORE-005 v0.2.0 | §§3–7 | Interaction and boundary context are explicit and distinct from acts/effects. | required object/distinction | interaction | Interaction records preserve context and boundary. |
| SRC-005-02 | CORE-005 v0.2.0 | §§8–11 | Interaction acts, effects, stages, transmission, receipt, acceptance, reliance, and execution are distinct. | required distinction | interaction | No stage is inferred from another. |
| SRC-005-03 | CORE-005 v0.2.0 | §§15, 19–20 | Custody/possession/ownership/responsibility transfer remain distinct; provenance/integrity are bounded. | required prohibition | interaction | Transfer validation does not imply ownership or truth. |
| SRC-005-04 | CORE-005 v0.2.0 | §24 | Failure, replay, silence, and state do not collapse interaction meaning. | required invariant | interaction | Boundary findings remain explicit and non-executing. |
| SRC-IMP000-01 | IMP-000 v0.2.0 | implementation constitution | Reference implementation is non-sovereign and cannot create constitutional authority. | required boundary | all | Public results expose non-authority boundaries. |
| SRC-IMP001-01 | IMP-001 v0.2.0 | architecture doctrine | Logical responsibility, context, and domain ownership are explicit. | required ownership/boundary | architecture | Components have stable ownership and no hidden cross-domain semantics. |
| SRC-IMP002-01 | IMP-002 v0.1.0 | component model | Components and dependencies are attributable and acyclic at the architecture level. | required architecture | repository | Crate ownership/dependencies remain reviewable. |
| SRC-IMP003-01 | IMP-003 v0.1.0 | lifecycle/gates | Scope, evidence, verification, deviations, and version gates are explicit. | required gate | lifecycle | Records state readiness without advancing version implicitly. |
| SRC-IMP004-01 | IMP-004 v0.1.0 | traceability model | Requirements map bidirectionally to components, code, tests, evidence, and versions. | required relationship | traceability | Forward/reverse/negative mappings are attributable. |
| SRC-IMP004-02 | IMP-004 v0.1.0 | traceability boundaries | Traceability does not adjudicate correctness or constitutional meaning. | required prohibition | traceability | Trace validators remain structural/non-authoritative. |
| SRC-IMP005-01 | IMP-005 v0.1.0 | §§2–4 | Representation is distinct from constitutional object and shall not invent meaning. | required boundary | canonical | Encoding/decoding preserves object/representation distinction. |
| SRC-IMP005-02 | IMP-005 v0.1.0 | §§5–7 | Canonical representation, identity, revision, serialization instance, and stored artifact are distinct. | required object/distinction | canonical | Canonical identity participation is explicit. |
| SRC-IMP005-03 | IMP-005 v0.1.0 | §§8–14 | Supported transformations declare source/target/version/equivalence obligations; ambiguity is not silently resolved. | required validation | canonical | Strict deterministic decoding rejects ambiguity/unknowns. |
| SRC-IMP006-01 | IMP-006 v0.1.0 | §§4–5 | Verification, assurance, conformance, and certification are separate. | required distinction | validation | Assurance does not establish conformance/certification. |
| SRC-IMP006-02 | IMP-006 v0.1.0 | §§7–8 | Verification scope and criteria are declared, versioned, and traceable to normative requirements. | required relationship | validation | Scope/criteria/basis are explicit. |
| SRC-IMP006-03 | IMP-006 v0.1.0 | §§9–11 | Plans, activities, observations, findings, conclusions, limitations, and assurance records are attributable. | required record | validation | Findings and assurance have deterministic bindings. |
| SRC-IMP007-01 | IMP-007 v0.1.0 | §§4.1–4.6 | Evidence identity, subject, scope, source, provenance, representation, limitations, and context are preserved. | required object/history | evidence | Evidence validation retains identity and context. |
| SRC-IMP007-02 | IMP-007 v0.1.0 | §§7–8 | Evidence Item, Collection, Package, Reference, and Record remain distinct; Evidence Record is not Evidence Item. | required distinction | evidence | Package/reference/record validation does not collapse identity. |
| SRC-IMP007-03 | IMP-007 v0.1.0 | §12.1 | Where custody is relevant, custody history SHALL identify custodian, interval, transfer, parties, basis, integrity, limitations, and gaps. | required record | evidence | Custody fields and gap conditions are observable. |
| SRC-IMP007-04 | IMP-007 v0.1.0 | §§12.2–12.4 | Custody is distinct from provenance and transfer does not establish ownership, authenticity, integrity, authority, or approval; gaps are recorded and not automatically invalid. | required prohibition | evidence | Custody results remain non-authoritative and gap-aware. |
| SRC-IMP007-05 | IMP-007 v0.1.0 | IMP-EVD-INV-008, -009, -012 | Historical evidence is not erased; derivation and correction preserve attributable references and versions. | required historical preservation | evidence | History/correction/derivation remain resolvable. |
| SRC-IMP008-01 | IMP-008 v0.2.0 | §§5–7 | Release identity, manifest, package, lineage, and released implementation are distinct. | required object/distinction | release | Release records do not imply deployment/activation. |
| SRC-IMP008-02 | IMP-008 v0.2.0 | §8 | Compatibility identifies subject, target, dimension, direction, scope, criteria, assumptions, baseline, context, conclusion, and limitations; it is contextual. | required record/validation | release | Compatibility claims remain context-bound. |
| SRC-IMP008-03 | IMP-008 v0.2.0 | §§9–10 | Successor applicability/assurance is explicit; migration requirement, plan, execution, verification, and outcome remain distinct. | required history/distinction | release | No assurance inheritance or migration execution is inferred. |
| SRC-IMP008-04 | IMP-008 v0.2.0 | §11, IMP-CNT-INV-009, -010, -012 | Historical preservation, directional compatibility, and rollback non-restoration remain bounded invariants. | required invariant | release | History and directional checks are explicit. |
| SRC-IMP009-01 | IMP-009 v0.2.0 | §§5–6 | Eligibility, authorization, activation decision, act, effect, assignment, recognition, and standing are distinct objects. | required object/distinction | activation | Records retain these distinctions. |
| SRC-IMP009-02 | IMP-009 v0.2.0 | §§7–8 | Assignment binds released implementation, role, baseline, deployment context, recognition authority, and effective interval; standing applies to assignment, not globally. | required relationship | activation | Assignment-bound standing is validated. |
| SRC-IMP009-03 | IMP-009 v0.2.0 | §§9–10 | Activation, suspension, restoration, transfer, rollback, deactivation, and supporting records are attributable; rollback does not presume restoration. | required transition/history | activation | Transition records and non-restoration boundary are explicit. |
| SRC-IMP009-04 | IMP-009 v0.2.0 | §11 | Authority required, scope explicitness, recognition non-conformance, suspension non-erasure, baseline fidelity, self-recognition prohibition, and deactivation explicitness are invariants. | required invariant/prohibition | activation | Validators reject boundary violations without creating authority or recognition effects. |

## Part II — Implementation construct inventory

The inventory covers 16 workspace crates, 38 typed identifier families, 15 finding-code enums, 133 test attributes, and the material public/constitutional constructs listed below. “First introduced” is established from phase records and change-impact history, not Git history.

| Construct ID | Crate | File | Type or function | Construct category | First introduced | Public or internal | Source mapping claimed |
|---|---|---|---|---|---|---|---|
| CI-001 | constitutional-contracts | `src/lib.rs` | `ImplementationVersion`, `TimePoint`, record IDs | typed identifier/version | Phase 1 | public | IMP-000/003 |
| CI-002 | constitutional-context | `src/lib.rs` | context/source-set records | context | Phase 1–3 | public | CORE/IMP-003 |
| CI-003 | constitutional-source-admission | `src/lib.rs` | source discovery, admission, admitted set | validator/record | Phase 3 | public | IMP-003/007 |
| CI-004 | constitutional-authority | `src/lib.rs` | authority/jurisdiction assignments and validation | authority binding | Phase 4 | public | CORE-001 |
| CI-005 | constitutional-identity | `src/lib.rs` | identity/participation records and validation | identity/relationship | Phase 5 | public | CORE-002 |
| CI-006 | constitutional-artifacts | `src/lib.rs` | artifact, instance, provenance, integrity, relationships | artifact/provenance | Phase 6 | public | CORE-003 |
| CI-007 | constitutional-state | `src/lib.rs` | state claims, transitions, preconditions, projection | state/transition | Phase 7 | public | CORE-004 |
| CI-008 | constitutional-interaction | `src/lib.rs` | interaction acts/effects, boundary, stages, handoff | interaction/boundary | Phase 8 | public | CORE-005 |
| CI-009 | constitutional-core | `src/lib.rs` | cross-domain integration and closure result | integration validator | Phase 9 | public | CORE universal invariants |
| CI-010 | constitutional-traceability | `src/lib.rs` | trace subjects, relations, paths, bindings, findings | traceability edge | Phase 11 | public | IMP-004 |
| CI-011 | constitutional-canonical | `src/representation.rs` | representation model, objects, fields, strict encode/decode | canonical record | Phase 12 | public | IMP-005 |
| CI-012 | constitutional-validation | `src/verification.rs` | verification requirements, activities, findings, assurance | validator/record | Phase 13 | public | IMP-006 |
| CI-013 | constitutional-evidence | `src/lib.rs` | EvidenceObject, EvidenceReference, EvidencePackage, EvidenceRecord-like models | evidence construct | Phase 14 | public | IMP-007 |
| CI-014 | constitutional-evidence | `src/lib.rs` | ProvenanceEvent, ProvenanceLineage, CustodyEvent | provenance/custody | Phase 14 | public | IMP-007 |
| CI-015 | constitutional-evidence | `src/lib.rs` | EvidenceFindingCode and evidence validators | finding/validator | Phase 14; expanded remediation 2/3 | public | IMP-007 |
| CI-016 | constitutional-release | `src/lib.rs` | ReleaseIdentity, ReleaseManifest, ReleasedImplementation | release construct | Phase 15 | public | IMP-008 |
| CI-017 | constitutional-release | `src/lib.rs` | ReleaseLineage, ReleaseRelationship, ReleaseHistoryEntry | history/lineage | Phase 15; expanded remediation 2 | public | IMP-008 |
| CI-018 | constitutional-release | `src/lib.rs` | CompatibilityClaim, CompatibilityAssessment | compatibility construct | Phase 15 | public | IMP-008 |
| CI-019 | constitutional-release | `src/lib.rs` | MigrationRequirement/Plan/ExecutionRecord | migration construct | Phase 15 | public | IMP-008 |
| CI-020 | constitutional-release | `src/lib.rs` | release graph/history/contradiction validators | validator | Phase 15; expanded remediation 2 | public | IMP-008 |
| CI-021 | constitutional-activation | `src/lib.rs` | ActivationRequest, EligibilityDetermination, ActivationDecision | activation object | Phase 16 | public | IMP-009 |
| CI-022 | constitutional-activation | `src/lib.rs` | ActivationAct, ActivationEffect, ActivationRecord | act/effect/record | Phase 16 | public | IMP-009 |
| CI-023 | constitutional-activation | `src/lib.rs` | OperationalAssignment, OperationalBaseline, OperationalStanding | assignment/standing | Phase 16 | public | IMP-009 |
| CI-024 | constitutional-activation | `src/lib.rs` | ActivationState, ActivationTransition | state/transition | Phase 16 | public | IMP-009 |
| CI-025 | constitutional-activation | `src/lib.rs` | HistoricalTransition and transition-history validator | history/validator | Remediation 2 | public | IMP-009 derived |
| CI-026 | constitutional-activation | `src/lib.rs` | RecognitionSubject, RecognitionRequest, RecognitionCriteria | recognition object | Remediation 2 | public | IMP-009 derived |
| CI-027 | constitutional-activation | `src/lib.rs` | RecognitionFinding, RecognitionFindingStatus | finding/status | Remediation 2 | public | implementation choice; not explicit in IMP-009 |
| CI-028 | constitutional-activation | `src/lib.rs` | RecognitionDecision, OperationalRecognition | decision/recognition record | Phase 16; expanded remediation 2 | public | IMP-009 |
| CI-029 | constitutional-activation | `src/lib.rs` | RecognitionResolutionContext | validation context | Remediation 2 | public | implementation choice |
| CI-030 | constitutional-activation | `src/lib.rs` | ActivationTraceabilityBinding | cross-domain binding | Remediation 2 | public | IMP-004/008/009 derivation |
| CI-031 | constitutional-activation | `src/lib.rs` | RecognitionStandingTransition | recognition history | Remediation 3 | public | remediation-derived |
| CI-032 | constitutional-activation | `src/lib.rs` | LifecycleCorrection | correction history | Remediation 3 | public | remediation-derived |
| CI-033 | constitutional-evidence | `src/lib.rs` | TypedCustodyEvent | custody identity/sequence | Remediation 3 | public | remediation-derived from IMP-007 §12 |
| CI-034 | constitutional-evidence | `src/lib.rs` | custody finding additions | finding codes | Remediation 3 | public | remediation-derived |
| CI-035 | constitutional-release | `src/lib.rs` | ReleaseHistoryAttachment | release history attachment | Remediation 3 | public | remediation-derived from IMP-008 §§9–10 |
| CI-036 | constitutional-release | `src/lib.rs` | OperationalTraceReference/ReleaseOperationalTrace | release-to-operational trace | Remediation 3 | public | necessary derivation from IMP-008 §12/IMP-009 §7 |
| CI-037 | constitutional-release | `src/lib.rs` | migration/supersession cycle validators | graph validator | Remediation 2 | public | audit-driven/optional; not source-required |
| CI-038 | all domain crates | `src/lib.rs` | typed identifiers generated by `id_type!` or domain constructors | typed identifier | Phases 1–16 | public | type-safety/determinism refinement |
| CI-039 | all domain crates | `src/lib.rs` | domain-specific finding enums and non-authority flags | finding/boundary | Phases 1–16 | public | source prohibitions and implementation validation |
| CI-040 | canonical/domain crates | canonical modules | domain canonical wrappers and strict round trips | canonicalization | Phases 6–16 | public | IMP-005 refinement |

The complete named public surface is distributed across the 16 crate source files; the table groups only repeated primitive families (identifier constructors, result wrappers, and domain finding helpers) so that the inventory remains reviewable without treating every field accessor as a separate constitutional construct.

## Part III — Differentiation matrix

| Difference ID | Source requirement | Source location | Implementation construct | Implementation location | Difference type | Why it differs | Introduced when | Creates new obligation? | Audit consequence | Disposition |
|---|---|---|---|---|---|---|---|---|---|---|
| DIF-001 | CORE objects remain distinct | CORE-000 §§6–15 | Separate domain crates and typed records | CORE crates | DIRECT_MATCH | Domain ownership realizes explicit ontology distinctions. | Original implementation | No | Harmless | PRESERVE_REQUIRED |
| DIF-002 | Authority/jurisdiction is explicit | CORE-001 | AuthorityBinding and authority validators | authority crate | IMPLEMENTATION_REFINEMENT | Type safety makes scope/jurisdiction references explicit. | Phase 4 | No | None | PRESERVE_REVIEWED_DESIGN |
| DIF-003 | Identity/participation/recognition remain distinct | CORE-002 | Identity and participation result models | identity crate | DIRECT_MATCH | Direct model separation. | Phase 5 | No | None | PRESERVE_REQUIRED |
| DIF-004 | Artifact identity/instance distinction | CORE-003 §§17.1–17.4 | Artifact/instance/provenance types and canonical wrappers | artifacts/canonical | IMPLEMENTATION_REFINEMENT | Deterministic identity participation requires typed fields. | Phase 6 | Yes, canonical and provenance tests | Expanded audit surface but source-supported | PRESERVE_REVIEWED_DESIGN |
| DIF-005 | State transitions are scoped and historical | CORE-004 | State transition evaluator and projection | state crate | DIRECT_MATCH | Direct bounded realization. | Phase 7 | No | None | PRESERVE_REQUIRED |
| DIF-006 | Interaction stages remain distinct | CORE-005 §§8–18 | Interaction stage enums and boundary findings | interaction crate | IMPLEMENTATION_REFINEMENT | Explicit stage model supports non-conflation. | Phase 8 | Yes, stage validation | Low | PRESERVE_REVIEWED_DESIGN |
| DIF-007 | Traceability is bidirectional and non-authoritative | IMP-004 | Trace graph/path validators | traceability crate | DIRECT_MATCH | Architecture freeze explicitly assigns this owner. | Phase 11 | Yes, path/cycle tests | None | PRESERVE_REQUIRED |
| DIF-008 | Representation preserves meaning but does not create it | IMP-005 §§2–5 | Shared canonical representation model and strict decoder | canonical crate | CANONICALIZATION_EXPANSION | Source requires distinctions; deterministic encoding is a selected realization. | Phase 12 | Yes, canonical round-trip obligations | Added broad canonical audit surface | PRESERVE_REVIEWED_DESIGN |
| DIF-009 | Verification records are separate from assurance | IMP-006 §§4–8 | Verification activities/findings/assurance records | validation crate | IMPLEMENTATION_REFINEMENT | Source names the progression; typed records make it testable. | Phase 13 | Yes, finding/assurance binding | Source-supported | PRESERVE_REQUIRED |
| DIF-010 | Evidence Item/Package/Reference/Record distinct | IMP-007 §§4, 7, 8 | Evidence object/reference/package models | evidence crate | DIRECT_MATCH | Direct source object separation. | Phase 14 | No beyond required identity | None | PRESERVE_REQUIRED |
| DIF-011 | Custody history identifies listed fields | IMP-007 §12.1 | Original CustodyEvent omitted typed event ID, chain ID, sequence, limitation/gap structure | evidence crate | SOURCE_OMISSION | Original model implemented a narrower custody shape than source field obligations. | Phase 14 | Yes, later custody remediation | MAJ-003 | CORRECT_REQUIRED |
| DIF-012 | Custody transfer does not imply authority/truth | IMP-007 §§12.2–12.4 | custody validator and non-authority flags | evidence crate | DIRECT_MATCH | Boundary preserved. | Phase 14 | No | None | PRESERVE_REQUIRED |
| DIF-013 | Historical evidence is not erased | IMP-007 §4.6, INV-008/012 | preserved-from/history checks | evidence/activation | IMPLEMENTATION_NARROWING | Historical links exist but are incomplete for correction/attachment graphs. | Remediation 2/3 | Yes | MAJ-001/003/004 | CORRECT_REQUIRED |
| DIF-014 | Release identity/manifest/lineage distinct | IMP-008 §§5–7 | ReleaseIdentity/Manifest/Lineage | release crate | DIRECT_MATCH | Direct source object model. | Phase 15 | No | None | PRESERVE_REQUIRED |
| DIF-015 | Compatibility is contextual/directional | IMP-008 §8, INV-004/010 | CompatibilityAssessment and contradiction validator | release crate | IMPLEMENTATION_REFINEMENT | Context dimensions made explicit for deterministic comparison. | Phase 15; remediation 2 | Yes, contradiction identity | MAJ-004 but freeze rescoped it | PRESERVE_REVIEWED_DESIGN |
| DIF-016 | Successor applicability/assurance explicit | IMP-008 §9 | ReleaseHistoryEntry lacks complete attachment context | release crate | SOURCE_OMISSION | Initial release history implemented sequence/identity but not all continuity references. | Remediation 2 | Yes | MAJ-004 | CORRECT_REQUIRED |
| DIF-017 | Migration phases distinct | IMP-008 §10 | MigrationRequirement/Plan/ExecutionRecord | release crate | DIRECT_MATCH | Direct named objects. | Phase 15 | No | None | PRESERVE_REQUIRED |
| DIF-018 | Migration/supersession graphs acyclic | No explicit source clause | cycle validators | release crate | AUDIT_DRIVEN expansion | Added during remediation 2 to satisfy audit expectations; freeze removed it as closure-critical. | Remediation 2 | Yes, graph tests | Closure surface expanded then rescoped | REMOVE_FROM_CLOSURE_CRITERIA |
| DIF-019 | Activation objects are distinct | IMP-009 §§5–6 | ActivationRequest/Decision/Act/Effect/Record | activation crate | DIRECT_MATCH | Direct source object model. | Phase 16 | No | None | PRESERVE_REQUIRED |
| DIF-020 | Assignment anchors standing | IMP-009 §7 | OperationalAssignment/Standing | activation crate | DIRECT_MATCH | Direct source relationship. | Phase 16 | No | None | PRESERVE_REQUIRED |
| DIF-021 | Governed transitions and supporting records | IMP-009 §§9–10 | ActivationTransition/HistoricalTransition | activation crate | IMPLEMENTATION_REFINEMENT | Source enumerates transitions; one history object provides deterministic validation. | Phase 16; remediation 2 | Yes | MAJ-001 | PRESERVE_REVIEWED_DESIGN |
| DIF-022 | Rollback does not presume restoration | IMP-009 INV-009; IMP-008 INV-012 | RollbackRestoration finding and negative checks | activation/release | VALIDATION_EXPANSION | Negative enforcement makes the prohibition observable. | Remediation 2 | Yes, focused tests | MAJ-001 | PRESERVE_REQUIRED |
| DIF-023 | Recognition supporting records | IMP-009 §§8–10 | RecognitionCriteria/Finding/ResolutionContext | activation crate | IMPLEMENTATION_EXPANSION | Source names recognition and baseline/limitations but not this exact schema. | Remediation 2 | Yes, criterion/finding/condition/limitation validators | MAJ-002 surface expanded | PRESERVE_OPTIONAL; REMOVE_FROM_CLOSURE_CRITERIA |
| DIF-024 | Recognition authority is bound to assignment | IMP-009 §7, INV-003 | authority subject comparison | activation crate | NECESSARY_DERIVATION | Assignment authority must apply to the same recognized subject. | Remediation 3 | Yes | CCF-MAJ2-001 | CORRECT_REQUIRED |
| DIF-025 | Recognition transitions are separately typed | IMP-009 §§9–10 | RecognitionStandingTransition | activation crate | REMEDIATION_ADDITION | Added during third remediation to close a history gap. | Remediation 3 | Yes | New audit surface | RECONSIDER_AFTER_CLOSURE |
| DIF-026 | Lifecycle correction link | IMP-007 §4.6/INV-012 applied to lifecycle | LifecycleCorrection | activation crate | REMEDIATION_ADDITION | Added during third remediation to carry historical preservation into lifecycle records. | Remediation 3 | Yes | New audit surface; not source-named | RECONSIDER_AFTER_CLOSURE |
| DIF-027 | Custody event identity/sequence/predecessor | IMP-007 §§8.2, 12.1 | TypedCustodyEvent | evidence crate | REMEDIATION_ADDITION | Necessary derivation for event-level resolvability, but exact encoding is implementation choice. | Remediation 3 | Yes | MAJ-003 | PRESERVE_REVIEWED_DESIGN; do not make encoding itself constitutional |
| DIF-028 | Release-history attachment | IMP-008 §§9–10 | ReleaseHistoryAttachment | release crate | REMEDIATION_ADDITION | Added to make applicability/continuity references resolvable. | Remediation 3 | Yes | MAJ-004 | CORRECT_REQUIRED, bounded |
| DIF-029 | Release-to-operational typed chain | IMP-008 §12; IMP-009 §7 | ReleaseOperationalTrace | release crate | TRACEABILITY_EXPANSION | End-to-end typed chain is a necessary cross-boundary derivation, not a new owner. | Remediation 3 | Yes | MAJ-004 | PRESERVE_REVIEWED_DESIGN |
| DIF-030 | ActivationTraceabilityBinding uses opaque recognition IDs | IMP-004/009 | binding with `Option<RecognitionRecordIdentifier>` | activation crate | IMPLEMENTATION_NARROWING | Initial cross-domain binding stopped at identifier membership. | Remediation 2 | Yes | MAJ-004 | CORRECT_REQUIRED |
| DIF-031 | Finding enums exceed source vocabulary | IMP-007/008/009 | domain FindingCode enums | domain crates | IMPLEMENTATION_EXPANSION | Public fail-closed diagnostics are implementation mechanisms. | Phases 1–16; expanded remediation | Yes, reachability/tests | Later audits treated reachability as closure | PRESERVE_OPTIONAL; not independent source criteria |
| DIF-032 | Canonical wrappers for later records | IMP-005 | activation/release/evidence canonical functions | domain/canonical crates | CANONICALIZATION_EXPANSION | Canonical stability is a permitted architecture realization. | Phases 6–16 | Yes, round-trip obligations | Added audit surface | PRESERVE_REVIEWED_DESIGN |
| DIF-033 | Version fields on almost every record | IMP-003/005 | ImplementationVersion fields | all domain crates | NECESSARY_DERIVATION | Source register and phase gates require exact source/version binding. | Phases 1–16 | Yes, mismatch findings | Cross-domain closure surface | PRESERVE_REQUIRED |
| DIF-034 | Non-authority result flags on validators | IMP-000/005/006/007/008/009 | result structs with authority/truth/conformance flags | domain crates | IMPLEMENTATION_REFINEMENT | Prohibitions become observable without creating the prohibited effect. | Phases 1–16 | Yes, boundary tests | Harmless | PRESERVE_REVIEWED_DESIGN |
| DIF-035 | Architecture places release and activation in separate crates | IMP-001/002 and AFD-002 | constitutional-release / constitutional-activation | workspace | OWNERSHIP_SHIFT | Dedicated crates were selected by the architecture freeze; not a source semantic shift. | Phases 15–16 | Yes, typed cross-crate bindings | Requires composition tests | PRESERVE_REVIEWED_DESIGN |
| DIF-036 | Third-remediation typed contexts are public | Source abstract; scope permits bounded validation | new public structs | three crates | IMPLEMENTATION_ONLY | Public visibility was used for reusable focused validation rather than hidden test helpers. | Remediation 3 | Yes, public API maintenance | May harden optional detail into contract | RECONSIDER_AFTER_CLOSURE |

## Required classification summary

The differentiation types above include direct matches, source omissions, narrowings, expansions, substitutions/refinements, implementation-only structures, ownership/traceability/canonicalization expansions, validation expansions, and remediation additions. The corresponding normative classification is:

- `REQUIRED_REALIZATION`: direct domain objects, explicit prohibitions, version/source bindings, non-authority boundaries, and named IMP records.
- `NECESSARY_DERIVATION`: assignment subject binding, version fields, typed cross-domain references, custody event resolvability, contextual compatibility identity.
- `REVIEWED_ARCHITECTURAL_CHOICE`: dedicated crates, shared canonical model, traceability graph, domain result flags.
- `OPTIONAL_IMPLEMENTATION_DETAIL`: exact finding enum vocabulary, recognition finding/condition schema, graph algorithms, public helper contexts.
- `REMEDIATION_INTRODUCED_DETAIL`: recognition standing transition, lifecycle correction, typed custody event, release-history attachment, release operational trace.
- `SELF_CREATED_OBLIGATION`: public recognition schema, public finding enums, canonical wrappers, opaque-to-typed traceability layers where later validation was required because the implementation exposed the construct.
- `AUDIT_DRIVEN_EXPANSION`: migration/supersession cycle closure and exhaustive recognition/custody/release graph demands not directly supported by source; the criteria freeze correctly removed these as independent blockers.
- `SOURCE_REQUIREMENT_NOT_IMPLEMENTED`: complete custody field/gap integration, complete release applicability/attachment continuity, complete typed composed release-to-activation chain, and complete assignment/recognition history integration remain incomplete.
- `CONTRADICTORY_IMPLEMENTATION`: no confirmed constitutional contradiction was found. Narrowness and omission are present; contradiction is not.

## Part V — Self-Created Closure Obligations

### Recognition schema chain

`RecognitionFinding`/`RecognitionCriteria` public types → identity and subject/version bindings → `RecognitionResolutionContext` → finding/condition/limitation validators → canonical recognition record → focused tests → MAJ-002 audit surface. The source supports recognition and baseline limitations, but not this exact criterion/finding schema. It should remain a bounded optional realization and must not independently define constitutional closure.

### Finding-code chain

`FindingCode` enum variants → public emission paths → deterministic ordering → focused reachability tests → audit inventory. This is useful fail-closed infrastructure, but declared enum membership is an implementation maintenance rule, not automatically a source obligation.

### Typed custody chain

`TypedCustodyEvent` → event/chain/evidence identity → predecessor/sequence rules → gap/transfer findings → focused custody tests → MAJ-003 audit surface. The event-level identity is a reasonable derivation from IMP-007’s identity and custody requirements, but the exact sequence field and identifier encoding remain implementation detail.

### Release operational chain

`ReleaseOperationalTrace` → typed subject/release/version references → cross-crate resolution → mismatch findings → cross-domain tests → MAJ-004 audit surface. The source boundary and assignment binding justify a traceability relationship; they do not require one particular aggregate chain type.

### Canonical expansion chain

New public domain record → canonical field participation → strict encoding/decoding → duplicate/unknown/order rules → round-trip tests → canonical audit surface. This was an architecture-selected realization of IMP-005, not an accidental source amendment, but every newly canonicalized type creates durable maintenance obligations.

## Part VI — Domain-specific findings

### IMP-007

IMP-007 explicitly requires evidence identity/context, distinct evidence objects, historical preservation, custody field categories, custody/provenance separation, and non-authority boundaries. The original `CustodyEvent` was narrower than §12.1: no event identity, chain identity, explicit limitation/gap field, or deterministic predecessor/sequence existed. Remediation 2 added chain validation; remediation 3 added `TypedCustodyEvent`. MAJ-003 therefore began as a source omission plus incomplete validation, then acquired an implementation-specific event model. The event model should be corrected and integrated, but sequence/predecessor encoding should not become a separate constitutional criterion.

### IMP-008

Release identity, manifests, lineage, contextual/directional compatibility, migration distinctions, successor applicability, and history are source-defined. Compatibility contradiction classes beyond contextual/directional identity, migration/supersession acyclicity, and a complete release graph algorithm are implementation/audit additions. Release-history attachments and typed release-to-operational traceability are reasonable derived structures, but they were introduced after audits identified missing end-to-end proof rather than being fully present in the original Phase 15 design.

### IMP-009

IMP-009 explicitly defines eligibility, authorization, activation, assignment, recognition, standing, transitions, supporting records, and non-authority invariants. `RecognitionFinding`, `RecognitionCriteria`, typed conditions/limitations, and `RecognitionResolutionContext` were introduced in remediation 2 to make a preferred validation model explicit. The resulting MAJ-002 surface is partly implementation-created. Assignment authority subject binding and standing-history preservation are source-grounded derivations; the exact recognition schema is not.

### IMP-004, IMP-005, and IMP-006

IMP-004 required bidirectional mappings and non-authority traceability; release-to-activation and activation-to-recognition composition was not fully explicit in the original trace model and was strengthened during remediation. IMP-005 permits deterministic canonicalization but explicitly forbids inventing constitutional concepts; canonical expansion followed the architecture plan and later became an audit surface for newly public types. IMP-006 supplied verification/assurance separation and criteria traceability; many detailed finding codes validate implementation shapes rather than source-defined constitutional vocabulary.

## Part VIII — Audit causation review

| Audit finding | First audit appearance | Root cause | Source-based or implementation-created | Added construct involved | Was closure made harder by prior remediation? | Current assessment |
|---|---|---|---|---|---|---|
| MAJ-001 | Original closure audit | Incomplete lifecycle validation against named IMP-009 transitions and records; later history model was still partial. | Mixed: source requirement plus implementation gap | HistoricalTransition, sequence, predecessor, recognition history | Yes. Sequence/history structures created additional continuity obligations, though the core transition gap was pre-existing. | Keep source-grounded transition/result and assignment rules; exclude universal orphan/graph demands. |
| MAJ-002 | Original closure audit | Recognition was modeled more richly than the source required, but its added bindings were incompletely validated. | Primarily implementation-created, with authority/assignment derivation | RecognitionFinding, Criteria, ResolutionContext, conditions/limitations | Yes. Remediation 2 expanded the recognition audit surface. | Keep authority-subject binding; remove exact schema completeness as independent Major closure. |
| MAJ-003 | Original closure audit | Original evidence validators omitted declared reachability and custody detail required by IMP-007. | Source-based omission plus implementation gap | EvidenceFindingCode, CustodyEvent, later TypedCustodyEvent | Yes. Public finding inventory and typed custody paths increased maintenance obligations. | Correct field/gap/identity behavior; do not constitutionalize enum/sequence encoding. |
| MAJ-004 | Original closure audit | Release history and cross-domain traceability were structurally incomplete; later graph rules exceeded source. | Mixed: source continuity plus audit-driven expansion | Migration graphs, supersession validators, traceability strings, typed operational trace | Yes. Remediation 2 added cycles/graphs and opaque trace lists; remediation 3 added typed trace. | Keep source-grounded applicability/history/trace; remove acyclicity and exhaustive contradiction requirements. |

Remediation effect by stage:

- First remediation: preserved closure scope while adding broad bounded domain records and validators.
- Second remediation: expanded closure surface materially through sequence history, recognition schemas, finding reachability, graph rules, and opaque end-to-end lists.
- Criteria freeze: reduced closure scope by separating source obligations from implementation choices.
- Third remediation: added further typed contexts; it improves resolvability but also creates optional public maintenance surfaces. These additions should not expand the frozen closure criteria.

## Part IX — Preserve, reconsider, or remove from closure

### Preserve required

CORE domain distinctions; explicit authority/scope; artifact and state separation; IMP-004 bidirectional non-authoritative traceability; IMP-005 representation non-identity and deterministic decoding; IMP-006 assurance separation; IMP-007 evidence identity/history/custody boundaries; IMP-008 release identity/contextual compatibility/migration separation/history; IMP-009 assignment/standing/transition/non-authority invariants; implementation version/source bindings.

### Preserve reviewed design

Dedicated release and activation crates; shared canonical representation; domain-owned validators; typed result flags; domain-specific identifiers; typed cross-domain trace references. These are reviewed architecture choices, not source amendments.

### Preserve optional

Exact finding enum vocabulary, recognition criterion/finding schema, migration graph cycle checks, detailed condition/limitation strings, and helper validation contexts. They may remain in code, but incompleteness must not independently block constitutional closure unless the freeze explicitly retains a source-grounded criterion.

### Reconsider after closure

Public `RecognitionStandingTransition`, `LifecycleCorrection`, `TypedCustodyEvent`, `ReleaseHistoryAttachment`, and `ReleaseOperationalTrace` APIs. They are useful bounded realizations, but each was introduced during remediation and now creates additional public maintenance obligations.

### Remove from closure criteria

Universal sequence-zero predecessor requirements; migration/supersession acyclicity; exhaustive recognition outcome/schema completeness; exhaustive compatibility status taxonomies; complete graph shape as such; declared finding-code reachability as an independent constitutional requirement. The code may remain.

### Correct required

Integrate custody field/gap/identity validation with the canonical evidence object path; complete release applicability/evidence/assurance/history attachment resolution; compose release-to-assignment/activation/recognition typed references; complete assignment and recognition standing history where the source requires it.

## Part X — Conclusions and smallest faithful path

1. Yes, the implementation omitted source-defined obligations, principally IMP-007 custody fields/gaps/history, IMP-008 successor applicability/history attachments, and parts of IMP-009 assignment/standing history.
2. Yes, it added structures not expressly named by the original contracts, especially recognition findings/criteria/contexts, graph cycle validators, typed custody event identity, and aggregate release-operational trace types.
3. Dedicated crates, canonical strictness, traceability graphs, typed identifiers, and result boundaries were deliberate reviewed architecture.
4. Recognition schema, graph rules, and the new typed remediation contexts were introduced during remediation or audit response.
5. Yes. Public types created identity, canonicalization, validation, traceability, and test obligations.
6. Yes. Remediation 2 especially made closure harder by expanding the surface before the criteria freeze separated source requirements from implementation choices.
7. The largest expansions were recognition findings/criteria/conditions, migration/supersession graph closure, opaque release activation/recognition lists, and public custody sequence/identity models.
8. Yes. The second re-audit converted several implementation choices into apparent Major requirements; the criteria freeze corrected this for unsupported schema, acyclicity, exhaustive taxonomy, and graph-shape demands.
9. The criteria freeze correctly distinguishes most source obligations from implementation choices, but its remaining source-grounded criteria still require careful integration rather than merely adding parallel validators.
10. The third remediation should not proceed unchanged as a closure authorization path; its code may remain, but open source-required integration criteria must be completed narrowly.
11. The third remediation should be narrowed operationally to source-required integration and dedicated evidence, without adding new public structures.
12. Yes: remove implementation-only schema, graph, sequence encoding, and enum reachability from independent closure criteria while preserving code pending post-closure design review.
13. Yes: custody §12.1/§12.4 integration, successor applicability/history, typed composed release-to-operational binding, and assignment/standing continuity remain incomplete.
14. The smallest faithful path is: integrate existing typed validators into existing canonical domain records; add only source-required negative tests; validate typed cross-domain references at ownership boundaries; preserve optional structures without making them closure gates; rerun an independent differentiation-aware closure audit.

Final conclusion: the implementation is substantially source-aligned in domain boundaries and non-authority doctrine, but it contains remediation-driven expansion and several source omissions. Further work should correct the omissions without adding another layer of public closure machinery.
