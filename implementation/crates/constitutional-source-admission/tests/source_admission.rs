use constitutional_canonical::{decode, encode};
use constitutional_context::SourceCatalog;
use constitutional_contracts::{
    ContextResolutionResult, SourceAdmissionFindingCode, SourceAdmissionStatus, SourceRootId,
};
use constitutional_source_admission::{
    DeclaredSourceRoot, DiscoveredSourceFile, SourceAdmissionRequest, SourceDiscoveryRequest,
    admit_source, build_admitted_source_set, discover_sources,
};
use constitutional_test_support::complete_request;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root(label: &str) -> (PathBuf, DeclaredSourceRoot) {
    let root_path = std::env::temp_dir().join(format!(
        "mineard-source-admission-{}-{}",
        std::process::id(),
        label
    ));
    if root_path.exists() {
        fs::remove_dir_all(&root_path).unwrap();
    }
    fs::create_dir_all(&root_path).unwrap();
    let root = DeclaredSourceRoot {
        id: SourceRootId::new("TEST-ROOT").unwrap(),
        absolute_path: root_path.clone(),
        logical_root_name: "test-root".into(),
        permitted_extensions: vec!["md".into(), "txt".into(), "json".into()],
        excluded_directories: vec![".git".into(), "target".into(), "generated".into()],
        repository_boundary: root_path.parent().unwrap().to_path_buf(),
        recursive: true,
        root_provenance: "deterministic temporary fixture".into(),
    };
    (root_path, root)
}

fn valid_source(identifier: &str, body: &str) -> String {
    valid_source_family(identifier, "IMP", body)
}

fn valid_source_family(identifier: &str, family: &str, body: &str) -> String {
    format!(
        "# Test Source\n\n**Identifier:** {identifier}\n**Title:** Fixture Source\n**Version:** 0.1.0\n**Status:** PENDING ADOPTION\n**Classification:** Implementation Fixture\n**Specification Family:** {family}\n\n{body}\n"
    )
}

fn discovered(root: &DeclaredSourceRoot, relative: &str) -> DiscoveredSourceFile {
    DiscoveredSourceFile {
        root_id: root.id.clone(),
        absolute_path: root.absolute_path.join(relative),
        relative_path: relative.into(),
        extension: Path::new(relative)
            .extension()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    }
}

#[test]
fn discovery_and_admission_are_deterministic_and_preserve_metadata() {
    let (root_path, root) = fixture_root("positive");
    fs::create_dir_all(root_path.join("nested")).unwrap();
    fs::write(
        root_path.join("nested/IMP-002_Test_v0.1.0.md"),
        valid_source("IMP-002", "nested"),
    )
    .unwrap();
    fs::write(
        root_path.join("IMP-001_Test_v0.1.0.md"),
        valid_source("IMP-001", "root"),
    )
    .unwrap();
    fs::write(root_path.join("ignored.pdf"), b"not admitted by discovery").unwrap();
    let files = discover_sources(&SourceDiscoveryRequest { root: root.clone() }).unwrap();
    assert_eq!(
        files
            .iter()
            .map(|file| file.relative_path.as_str())
            .collect::<Vec<_>>(),
        vec!["IMP-001_Test_v0.1.0.md", "nested/IMP-002_Test_v0.1.0.md"]
    );
    let admitted: Vec<_> = files
        .iter()
        .map(|file| {
            admit_source(&SourceAdmissionRequest {
                root: root.clone(),
                file: file.clone(),
            })
            .unwrap()
        })
        .collect();
    assert_eq!(
        admitted[0].descriptor.identity.content_digest.algorithm,
        "SHA-256"
    );
    assert_eq!(
        admitted[0]
            .descriptor
            .normative_status
            .as_ref()
            .unwrap()
            .as_str(),
        "PENDING ADOPTION"
    );
    let set = build_admitted_source_set(admitted).unwrap();
    assert_eq!(set.sources.len(), 2);
    assert!(set.source_set_id.as_str().starts_with("source-set-"));
    fs::remove_dir_all(root_path).unwrap();
}

#[test]
fn sha256_digest_is_stable_and_source_identity_is_not_digest_only() {
    let (root_path, root) = fixture_root("digest");
    fs::write(
        root_path.join("IMP-003_Test_v0.1.0.md"),
        valid_source("IMP-003", "hello"),
    )
    .unwrap();
    let admitted = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: discovered(&root, "IMP-003_Test_v0.1.0.md"),
    })
    .unwrap();
    assert_eq!(admitted.descriptor.identity.content_digest.value.len(), 64);
    assert_ne!(
        admitted.descriptor.identity.identifier.as_str(),
        admitted.descriptor.identity.content_digest.value
    );
    fs::remove_dir_all(root_path).unwrap();
}

#[test]
fn missing_metadata_alias_conflict_and_filename_mismatch_fail_closed() {
    let (root_path, root) = fixture_root("metadata");
    fs::write(
        root_path.join("IMP-004_Test_v0.2.0.md"),
        "**Title:** Missing ID\n**Version:** 0.1.0\n",
    )
    .unwrap();
    let missing = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: discovered(&root, "IMP-004_Test_v0.2.0.md"),
    })
    .unwrap_err();
    assert!(
        missing
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::MissingIdentifier)
    );
    fs::write(root_path.join("IMP-005_Test_v0.1.0.md"), "**Identifier:** IMP-005\n**Version:** 0.1.0\nStatus: Draft\nNormative Status: Candidate\n**Specification Family:** IMP\n").unwrap();
    let alias = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: discovered(&root, "IMP-005_Test_v0.1.0.md"),
    })
    .unwrap_err();
    assert!(
        alias
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::ConflictingMetadataAlias)
    );
    fs::write(
        root_path.join("IMP-006_Test_v0.2.0.md"),
        valid_source("IMP-006", "mismatch"),
    )
    .unwrap();
    let mismatch = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: discovered(&root, "IMP-006_Test_v0.2.0.md"),
    })
    .unwrap_err();
    assert!(
        mismatch
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::FilenameMetadataMismatch)
    );
    fs::remove_dir_all(root_path).unwrap();
}

#[test]
fn root_boundaries_and_explicit_unsupported_files_fail_closed() {
    let (root_path, root) = fixture_root("boundaries");
    let missing_root = DeclaredSourceRoot {
        absolute_path: root_path.join("missing"),
        ..root.clone()
    };
    let discovery = discover_sources(&SourceDiscoveryRequest { root: missing_root }).unwrap_err();
    assert!(
        discovery
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::RootNotFound)
    );
    let outside = root_path.parent().unwrap().join("outside.md");
    fs::write(&outside, valid_source("IMP-007", "outside")).unwrap();
    let outside_result = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: DiscoveredSourceFile {
            root_id: root.id.clone(),
            absolute_path: outside.clone(),
            relative_path: "../outside.md".into(),
            extension: "md".into(),
        },
    })
    .unwrap_err();
    assert!(
        outside_result
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::PathTraversal)
    );
    let unsupported = root_path.join("source.pdf");
    fs::write(&unsupported, b"pdf").unwrap();
    let unsupported_result = admit_source(&SourceAdmissionRequest {
        root: root.clone(),
        file: DiscoveredSourceFile {
            root_id: root.id.clone(),
            absolute_path: unsupported,
            relative_path: "source.pdf".into(),
            extension: "pdf".into(),
        },
    })
    .unwrap_err();
    assert!(
        unsupported_result
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::UnsupportedFileType)
    );
    fs::remove_file(outside).unwrap();
    fs::remove_dir_all(root_path).unwrap();
}

#[test]
fn same_source_version_with_different_content_is_a_conflict() {
    let (root_path, root) = fixture_root("conflict");
    fs::write(
        root_path.join("IMP-008_a_v0.1.0.md"),
        valid_source("IMP-008", "a"),
    )
    .unwrap();
    fs::write(
        root_path.join("IMP-008_b_v0.1.0.md"),
        valid_source("IMP-008", "b"),
    )
    .unwrap();
    let files = discover_sources(&SourceDiscoveryRequest { root: root.clone() }).unwrap();
    let admitted: Vec<_> = files
        .iter()
        .map(|file| {
            admit_source(&SourceAdmissionRequest {
                root: root.clone(),
                file: file.clone(),
            })
            .unwrap()
        })
        .collect();
    let conflict = build_admitted_source_set(admitted).unwrap_err();
    assert_eq!(
        conflict.disposition,
        SourceAdmissionStatus::new("CONFLICT").unwrap()
    );
    assert!(
        conflict
            .findings
            .iter()
            .any(|finding| finding.code == SourceAdmissionFindingCode::ConflictingSourceContent)
    );
    fs::remove_dir_all(root_path).unwrap();
}

#[test]
fn admitted_source_set_drives_context_and_phase2_canonical_round_trip() {
    let (root_path, root) = fixture_root("context-integration");
    fs::write(
        root_path.join("AFD-002_Test_v0.1.0.md"),
        valid_source_family("AFD-002", "AFD", "freeze"),
    )
    .unwrap();
    fs::write(
        root_path.join("IMP-000_Test_v0.1.0.md"),
        valid_source("IMP-000", "implementation"),
    )
    .unwrap();
    let files = discover_sources(&SourceDiscoveryRequest { root: root.clone() }).unwrap();
    let admitted: Vec<_> = files
        .iter()
        .map(|file| {
            admit_source(&SourceAdmissionRequest {
                root: root.clone(),
                file: file.clone(),
            })
            .unwrap()
        })
        .collect();
    let set = build_admitted_source_set(admitted).unwrap();
    let result = SourceCatalog::resolve_admitted(&set, &complete_request());
    let ContextResolutionResult::Resolved(context) = result else {
        panic!("admitted source set should resolve the fixture request")
    };
    assert_eq!(
        context.admitted_source_set.as_ref().unwrap().source_set_id,
        set.source_set_id
    );
    let bytes = encode(&context).unwrap();
    let decoded = decode(&bytes).unwrap();
    assert_eq!(
        decoded.admitted_source_set.as_ref().unwrap().source_set_id,
        set.source_set_id
    );
    assert_eq!(encode(&decoded.to_context()).unwrap(), bytes);
    fs::remove_dir_all(root_path).unwrap();
}
