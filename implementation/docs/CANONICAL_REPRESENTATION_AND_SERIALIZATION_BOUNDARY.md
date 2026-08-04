# Canonical Representation and Serialization Boundary

The Phase 12 binding is a deterministic UTF-8, line-oriented reference serialization. It is an implementation binding, not the IMP-005 constitutional model itself.

The layers remain distinct:

1. domain object and domain-owned meaning;
2. canonical representation;
3. serialization instance;
4. non-authoritative identity/digest material;
5. persistence, which is not implemented here;
6. transport, which is not implemented here;
7. admitted constitutional objects, which are not created by decoding.

The binding uses fixed metadata order, lexicographically ordered canonical field names, explicit field counts, percent-escaped UTF-8 values, typed value discriminators, explicit `absent` optional bindings, and recursively encoded nested objects. Unknown, duplicate, missing, malformed, out-of-order, trailing, and unsupported material is rejected.

No JSON, wire protocol, persistence format, transport envelope, cryptographic signature, or concrete storage technology is claimed. Exact subordinate normalization and concrete binding governance remain deferred by IMP-005 §27 and are not silently invented here.

Successful decoding does not admit a source, establish truth or validity, execute an act, persist or transmit a record, verify an implementation, establish evidence sufficiency, create authority, authorize release/activation, or produce operational recognition.

