# Verification Evidence Boundary

IMP-006 may reference evidence but does not own evidence objects. The Phase 13 `EvidenceReference` preserves an evidence identifier, exact implementation version, and locator. It supports structural binding and can block assurance when malformed; it does not assess admissibility, sufficiency, custody, retention, authenticity, integrity, provenance, or legal status.

Observations remain verification records. They are not automatically converted into constitutional evidence. Passing tests, complete traceability, successful canonical decoding, and an existing evidence reference do not independently produce assurance or establish constitutional validity.

Evidence-object and provenance expansion remains deferred to IMP-007.

