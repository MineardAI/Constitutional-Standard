# Component Responsibility Matrix

| Responsibility | Owner | Consumers | Explicit non-responsibility |
|---|---|---|---|
| Public contracts | contracts | all components | authority or persistence |
| Context resolution | context | validation, evidence | applicability or authority creation |
| Canonical representation | canonical | domain wrappers, tests, evidence, traceability | constitutional reality, domain meaning, validity, admission, transport, persistence, authority, or effect |
| Findings, verification, and bounded assurance | validation | callers, traceability, canonical bindings | evidence ownership/sufficiency, conformance/certification, release, activation, operational recognition |
| Traceability | traceability | evidence/tests, future canonical expansion | correctness adjudication, constitutional interpretation, authority, or conformance |
| Evidence records | evidence | tests/reports | verification assurance |
| Evidence integrity, custody, package, and conflict validation | evidence | release, activation, tests/reports | truth, authenticity, persistence, authority, sufficiency, or certification |
| Fixtures | test-support | tests | production semantics |
| Filesystem source discovery and admission | source-admission | context, canonical, evidence | interpretation, applicability, authority, persistence |
| Authority and jurisdiction evaluation | authority | context, canonical, evidence | identity, participation, execution, enforcement, persistence |
| Identity and participation evaluation | identity | context, canonical, evidence, authority references | authentication, credentials, accounts, authority creation, execution, persistence |
| Artifact and provenance evaluation | artifacts | context, canonical, evidence, identity/authority references | storage, publication service, signing, truth, effectiveness, evidence assessment, persistence |
| State and transition evaluation | state | context, canonical, evidence, authority/identity/artifact references | mutation, workflow, execution, persistence, policy enforcement |
| Interaction and boundary evaluation | interaction | context, canonical, evidence, authority/identity/artifact/state references | transport, transmission, receipt, crossing, handoff execution, mutation, authority transfer, persistence |
| CORE integration and closure | core | existing domain results, canonical, evidence, contracts | domain reinterpretation, authority creation, execution, effect, persistence, transport, conformance, certification |
| Release continuity and bounded cross-domain resolution | release | activation, evidence, traceability, tests | publication, migration execution, activation, authority, conformance, certification |
| Activation/recognition lifecycle and historical standing | activation | release, authority, identity, evidence, tests | runtime activation, deployment, operational effectiveness, authority creation, persistence |
