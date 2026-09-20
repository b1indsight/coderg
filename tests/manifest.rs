use std::{fs, path::Path, process::Command};

#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod format;

fn fixture() -> format::Manifest {
    format::Manifest {
        registry: None,
        publication: None,
        version: 4,
        root: "/repo".into(),
        generation: 8,
        git_repository: true,
        git_head: Some("head".into()),
        git_tree: None,
        source_state: vec![format::FileState {
            path: "目录/source.rs".into(),
            len: 123,
            modified_nanos: u128::MAX,
        }],
        documents: vec![format::Document {
            path: "目录/source.rs".into(),
            len: 123,
            modified_nanos: u128::MAX,
            active: true,
            searchable: false,
            segment_id: 7,
        }],
        segments: vec![format::SegmentMeta {
            id: 7,
            lookup: "segments/7.lookup".into(),
            postings: "segments/7.postings".into(),
            ngrams: 42,
        }],
    }
}

#[test]
fn binary_and_legacy_json_preserve_all_fields() {
    let value = fixture();
    let bytes = format::encode(&value).unwrap();
    assert_eq!(&bytes[..8], b"CDRGMF01");
    assert_eq!(format::decode(&bytes).unwrap(), value);
    assert_eq!(
        format::decode(&serde_json::to_vec_pretty(&value).unwrap()).unwrap(),
        value
    );
}

#[test]
fn mapped_query_view_preserves_v1_and_v2_semantics_without_owned_records() {
    let temp = tempfile::tempdir().unwrap();
    for cached in [false, true] {
        let mut value = fixture();
        value.documents[0].searchable = true;
        let mut inactive = value.documents[0].clone();
        inactive.path = "inactive".into();
        inactive.active = false;
        value.documents.push(inactive);
        if cached {
            value.registry = Some(format::RegistryIdentity {
                epoch: 12,
                count: value.documents.len(),
                digest: [3; 20],
            });
        }
        let path = temp.path().join(if cached { "v2.bin" } else { "v1.bin" });
        let bytes = format::encode(&value).unwrap();
        fs::write(&path, &bytes).unwrap();
        let (header, view) = format::view::View::open(&path).unwrap().unwrap();
        assert!(header.documents.is_empty());
        assert!(header.source_state.is_empty());
        assert!(header.publication.is_none());
        assert_eq!(header.segments, value.segments);
        assert_eq!(view.source_len(), 1);
        assert!(view.same_source(&value.source_state).unwrap());
        let mut changed = value.source_state.clone();
        changed[0].modified_nanos -= 1;
        assert!(!view.same_source(&changed).unwrap());
        assert_eq!(view.document_path(0).unwrap(), value.documents[0].path);
        assert_eq!(view.active_document_ids(), vec![0]);
        assert!(view.is_live(0, Some(7)).unwrap());
        assert!(!view.is_live(0, Some(8)).unwrap());
        assert!(!view.is_live(1, None).unwrap());
        assert!(view.document_path(2).is_err());
        assert_eq!(view.materialize().unwrap(), format::decode(&bytes).unwrap());
    }
}

#[test]
fn mapped_view_defers_checksum_but_rejects_invalid_layouts() {
    let temp = tempfile::tempdir().unwrap();
    let mut value = fixture();
    value.registry = Some(format::RegistryIdentity {
        epoch: 12,
        count: value.documents.len(),
        digest: [3; 20],
    });
    let mut bytes = format::encode(&value).unwrap();
    bytes[8] ^= 1;
    let path = temp.path().join("bad-checksum.bin");
    fs::write(&path, &bytes).unwrap();
    let (_, view) = format::view::View::open(&path).unwrap().unwrap();
    assert!(
        view.materialize().is_err(),
        "cache reuse must verify integrity"
    );
    for length in [8, 27, bytes.len() - 1] {
        let truncated = temp.path().join(format!("truncated-{length}.bin"));
        fs::write(&truncated, &bytes[..length]).unwrap();
        assert!(format::view::View::open(&truncated).is_err());
    }
    let needle = "目录/source.rs".as_bytes();
    let document_path = bytes
        .windows(needle.len())
        .rposition(|w| w == needle)
        .unwrap();
    bytes[document_path + needle.len() + 24] = 2; // invalid active flag
    let invalid = temp.path().join("bad-flags.bin");
    fs::write(&invalid, &bytes).unwrap();
    assert!(format::view::View::open(&invalid).is_err());
    let valid = format::encode(&value).unwrap();
    let source_path = valid
        .windows(needle.len())
        .position(|w| w == needle)
        .unwrap();
    for (name, offset) in [
        ("source-count", source_path - 16),
        ("doc-count", document_path - 16),
    ] {
        let mut oversized = valid.clone();
        oversized[offset..offset + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        let path = temp.path().join(format!("{name}.bin"));
        fs::write(&path, oversized).unwrap();
        assert!(format::view::View::open(&path).is_err());
    }
}

#[test]
fn cache_manifest_identity_is_verified_and_not_trusted_from_json() {
    let mut value = fixture();
    value.registry = Some(format::RegistryIdentity {
        epoch: 123,
        count: value.documents.len(),
        digest: [9; 20],
    });
    let bytes = format::encode(&value).unwrap();
    assert_eq!(&bytes[..8], b"CDRGMF02");
    let loaded = format::decode(&bytes).unwrap();
    assert_eq!(loaded.publication, format::publication(&bytes));
    value.publication = loaded.publication;
    assert_eq!(loaded, value);
    for offset in [8, 27, 28, bytes.len() - 1] {
        let mut broken = bytes.clone();
        broken[offset] ^= 1;
        assert!(format::decode(&broken).is_err());
    }
    for length in 0..bytes.len() {
        assert!(format::decode(&bytes[..length]).is_err());
    }
    let json = serde_json::to_vec(&loaded).unwrap();
    assert!(format::decode(&json).unwrap().publication.is_none());
}

#[test]
fn rejects_truncation_unknown_versions_and_trailing_data() {
    let bytes = format::encode(&fixture()).unwrap();
    for length in 0..bytes.len() {
        assert!(format::decode(&bytes[..length]).is_err());
    }
    let mut changed = bytes.clone();
    changed[7] = b'2';
    assert!(format::decode(&changed).is_err());
    let mut changed = bytes.clone();
    changed.push(0);
    assert!(format::decode(&changed).is_err());
    // First string length follows the magic and the u32 schema version.
    let mut changed = bytes;
    changed[12..20].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(format::decode(&changed).is_err());
}

fn run(root: &Path, args: &[&str]) -> std::process::Output {
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(args)
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn old_weight_version_requires_rebuild_before_searching() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("source.txt"), "equalize quartz\n").unwrap();
    run(root.path(), &["index"]);
    let binary = root.path().join(".coderg-index/manifest.bin");
    let mut old = format::read(&binary).unwrap();
    assert_eq!(old.version, 5);
    old.version = 4;
    fs::write(&binary, format::encode(&old).unwrap()).unwrap();

    let rejected = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["search", "--no-refresh", "-F", "equalize quartz"])
        .arg(root.path())
        .output()
        .unwrap();
    assert_eq!(rejected.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("rebuild the index"));
    assert_eq!(format::read(&binary).unwrap().version, 4);

    let rebuilt = run(root.path(), &["search", "-F", "equalize quartz"]);
    assert_eq!(rebuilt.stdout, b"source.txt:1:equalize quartz\n");
    assert!(String::from_utf8_lossy(&rebuilt.stderr).contains("building index"));
    assert_eq!(format::read(&binary).unwrap().version, 5);
    run(
        root.path(),
        &["search", "--no-refresh", "-F", "equalize quartz"],
    );
}

#[test]
fn legacy_index_reads_without_rewrite_then_migrates_on_refresh() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("source.txt"), "old needle\n").unwrap();
    run(root.path(), &["index"]);
    let binary = root.path().join(".coderg-index/manifest.bin");
    let legacy = binary.with_extension("json");
    let original = format::read(&binary).unwrap();
    fs::write(&legacy, serde_json::to_vec_pretty(&original).unwrap()).unwrap();
    fs::remove_file(&binary).unwrap();
    assert_eq!(format::resolve_path(&binary).unwrap(), legacy);
    assert_eq!(format::read(&binary).unwrap(), original);
    run(root.path(), &["search", "--no-refresh", "-F", "old needle"]);
    run(root.path(), &["search", "-F", "old needle"]);
    run(root.path(), &["stats"]);
    let exported = run(root.path(), &["stats", "--json"]);
    assert_eq!(
        serde_json::from_slice::<format::Manifest>(&exported.stdout).unwrap(),
        original
    );
    assert!(
        !binary.exists(),
        "unchanged/read-only searches must not rewrite the index"
    );
    fs::write(root.path().join("source.txt"), "new needle changed\n").unwrap();
    run(root.path(), &["search", "-F", "new needle"]);
    assert!(binary.exists());
    assert!(!legacy.exists());
    assert_ne!(
        format::read(&binary).unwrap().generation,
        original.generation
    );
    // A corrupt preferred binary must not silently use a stale legacy snapshot.
    fs::write(&legacy, serde_json::to_vec(&original).unwrap()).unwrap();
    fs::write(&binary, b"CDRGMF02").unwrap();
    assert!(format::read(&binary).is_err());
    let result = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["search", "--no-refresh", "-F", "new needle"])
        .arg(root.path())
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
}
