# Reference Implementation Source Admission Profile

Profile: `Reference Implementation Source Admission Profile for the Controlled Source Admission Slice`

Profile identifier: `reference-implementation-source-admission`  
Profile version: `1.0.0`

## Supported formats

The profile supports UTF-8 `.md` and `.txt` source files, plus bounded `.json` manifest/source-control files when explicitly submitted. PDF, DOCX, YAML, TOML, HTML, and binary files are unsupported.

## Declared roots and discovery

Each root declares an identifier, absolute operational path, logical name, permitted extensions, excluded directory names, repository boundary, recursive mode, and root provenance. Roots must exist, be directories, be absolute, and remain within the declared repository boundary. Parent traversal components and symlinked files are rejected or skipped deterministically; discovery does not follow symlinks.

Discovery is recursive when configured, sorts directory entries and final relative paths lexicographically, uses case-insensitive extension matching, ignores unsupported files unless explicitly submitted, and excludes only the declared directories. The initial implementation commonly excludes `.git`, `target`, and generated temporary output directories.

## Metadata boundary

Admission inspects only the first 40 lines or until the first `---` delimiter. It accepts `Identifier`, `Title`, `Version`, `Status`, `Normative Status`, `Classification`, `Specification Family`, `Family`, `Revision Type`, and `Dependencies` in plain or Markdown-bold forms such as `Status: Draft` and `**Status:** Draft`. It does not scan arbitrary document text or extract clauses.

CORE, GOV, IMP, AFD, BASE, REG, META, and OPS families are recognized, with ADMIN and MANIFEST reserved for bounded administrative inputs. Identifier and version are required; title, status, classification, revision type, and dependencies are optional. Explicit placeholders such as `PENDING ADOPTION` remain values.

## Identity, digest, and conflicts

Source identity uses source identifier, family, exact version, declared root identity, normalized relative path, and a SHA-256 content digest. The digest is content-change evidence, not authority, provenance, signature, applicability, or certification.

Duplicate identities, conflicting content, conflicting versions, duplicate paths, metadata alias conflicts, filename/metadata mismatches, unsupported files, and unsafe paths produce typed findings. Conflicting source sets are not admitted into context resolution.

Admission dispositions are `ADMITTED`, `ADMITTED_WITH_FINDINGS`, `REJECTED`, `CONFLICT`, and `INDETERMINATE`. Admission establishes only bounded structural processing eligibility. It does not establish source applicability, constitutional authority, adoption, effectiveness, or effect.

