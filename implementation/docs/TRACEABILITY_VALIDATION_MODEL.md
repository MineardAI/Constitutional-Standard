# Traceability Validation Model

The validation entry point is `validate_trace_graph`. It validates a declared trace graph deterministically and returns a structured result with findings and explicit non-claims.

Validation checks include:

- graph, subject, edge, and binding identifier shape;
- duplicate subject and edge identifiers;
- unknown endpoints, self-reference, and duplicate relation edges;
- relation endpoint compatibility;
- source path and source-version bindings;
- graph/edge implementation-version consistency;
- mandatory path presence;
- prohibited authority declarations; and
- prohibited cycles when the graph disallows cycles.

Findings are sorted by subject and finding code so equivalent inputs produce stable result ordering. Invalid input is represented as a validation result; the validator does not execute a constitutional process or infer an absent relation.

`Valid` means only that the bounded structural checks found no fatal finding in the supplied graph. It does not mean the mapped requirement is satisfied, the source is adopted, evidence is sufficient, the implementation conforms, or any authority exists. `Invalid` identifies one or more fatal structural findings. `Indeterminate` is reserved for future validation conditions that cannot be resolved by this bounded model.

The focused test suite covers valid multi-domain paths, duplicate identifiers and edges, unknown/incompatible endpoints, version mismatch, prohibited authority implication, self-reference/cycles/missing paths, deterministic finding order, and preservation of the existing descriptive reverse-lookup seam.

