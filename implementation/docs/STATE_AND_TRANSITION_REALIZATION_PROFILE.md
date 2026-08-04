# Reference Implementation State and Transition Realization Profile

Profile identifier: `reference-implementation-state-transition`  
Profile version: `1.0.0`

## Governing source binding

Admitted source: `harmonization-v1.0/CORE-004_Constitutional_State_and_Transition_Doctrine_v0.3.0_Harmonization_Draft.md`.

Exact internal observation: Identifier `CORE-004`; primary metadata Version `0.2.0`; Status `Revised Baseline Draft`; the filename and harmonization addendum identify `0.3.0` as a working copy. The implementation preserves `PRIMARY_SOURCE_VERSION=0.2.0` and `ADDENDUM_SOURCE_VERSION=0.3.0` without amendment or silent reconciliation.

## Bounded realization

The profile models Constitutional State claims, primary subjects, state families, dimensions, domains, scopes, contexts, values, bases, categories, proposed transitions, transition kinds/effects, preconditions, invariants, constraints, authority/participation/artifact/provenance references, transition admission, transition support, continuity, and projected resulting state.

State claim support requires explicit subject, domain, basis, source, and non-duplicated dimensions. Transition admission requires explicit source and target states, matching subject/context/source-set/version, and a supported transition kind. Transition support requires admission, satisfied preconditions, preserved invariants, and an explicit authority reference for non-no-op transitions.

Projection applies only declared effects to a bounded proposal and returns a projected, non-executed state. It does not mutate state, establish actual current state, create constitutional effect, or authorize execution. Historical continuity is represented separately from current applicability.

## Determinations and operations

Separate determination families cover state support, transition admission, transition support, preconditions, invariant preservation, and projection. Operations include `evaluate_state_claim`, `evaluate_transition_admission`, `evaluate_transition_support`, `evaluate_preconditions`, `evaluate_invariant_preservation`, `project_resulting_state`, `evaluate_state_continuity`, and `evaluate_transition_relationship`.

## Explicit non-scope

This profile does not implement runtime mutation, workflows, tools, event processing, persistence, transactions, rollback, consensus, locking, networking, APIs, policy interpretation, authentication, access control, artifact storage, publication, evidence assurance, conformance, certification, activation, interaction processing, boundary enforcement, or production deployment.
