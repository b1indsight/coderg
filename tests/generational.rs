#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn run(root: &Path, args: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(args)
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn manifest(root: &Path) -> manifest_format::Manifest {
    manifest_format::read(&root.join(".coderg-index/manifest.bin")).unwrap()
}

// Recreate an older multi-segment snapshot without depending on an old binary.
// Each copy owns a disjoint subset of the original document IDs.
fn legacy_segments(root: &Path, count: usize) {
    let directory = root.join(".coderg-index");
    let mut snapshot = manifest(root);
    assert_eq!(snapshot.segments.len(), 1);
    let base = snapshot.segments[0].clone();
    for i in 1..count {
        let mut meta = base.clone();
        meta.id += i as u64;
        meta.lookup = format!("segments/legacy-{i}.lookup").into();
        meta.postings = format!("segments/legacy-{i}.postings").into();
        fs::copy(directory.join(&base.lookup), directory.join(&meta.lookup)).unwrap();
        fs::copy(
            directory.join(&base.postings),
            directory.join(&meta.postings),
        )
        .unwrap();
        snapshot.segments.push(meta);
    }
    for (i, document) in snapshot.documents.iter_mut().enumerate() {
        document.segment_id = snapshot.segments[i % count].id;
    }
    fs::write(
        directory.join("manifest.bin"),
        manifest_format::encode(&snapshot).unwrap(),
    )
    .unwrap();
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git(root: &Path) {
    git(root, &["init"]);
    git(root, &["config", "user.name", "Coderg Test"]);
    git(root, &["config", "user.email", "coderg@example.invalid"]);
}

fn matched(root: &Path, pattern: &str, expected: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["search", "--no-refresh", "-F", "-l", pattern])
        .arg(root)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(if expected.is_empty() { 1 } else { 0 }),
        "{pattern}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let mut paths: Vec<_> = text.lines().collect();
    paths.sort_unstable();
    assert_eq!(paths, expected, "{pattern}");
}

#[test]
fn changed_head_with_dirty_worktree_updates_both_committed_and_uncommitted_files() {
    let root = tempfile::tempdir().unwrap();
    init_git(root.path());
    fs::write(root.path().join("x.txt"), "base x\n").unwrap();
    fs::write(root.path().join("y.txt"), "base y\n").unwrap();
    git(root.path(), &["add", "x.txt", "y.txt"]);
    git(root.path(), &["commit", "-m", "base"]);
    run(root.path(), &["index"]);
    let base = manifest(root.path()).segments[0].clone();
    fs::write(root.path().join("x.txt"), "committed unique needle\n").unwrap();
    git(root.path(), &["add", "x.txt"]);
    git(root.path(), &["commit", "-m", "new x"]);
    fs::write(root.path().join("y.txt"), "dirty unique needle\n").unwrap();
    run(root.path(), &["search", "unique needle"]);
    matched(root.path(), "committed unique", &["x.txt"]);
    matched(root.path(), "dirty unique", &["y.txt"]);
    assert_eq!(manifest(root.path()).segments[0], base);
    assert_eq!(manifest(root.path()).segments.len(), 2);
    assert!(!root.path().join(".coderg-index/manifests").exists());
}

#[test]
fn small_index_appends_past_eight_segments_and_preserves_current_versions() {
    let root = tempfile::tempdir().unwrap();
    for i in 0..30 {
        fs::write(
            root.path().join(format!("{i:02}.txt")),
            format!("base record {i}\n"),
        )
        .unwrap();
    }
    run(root.path(), &["index"]);
    let base = manifest(root.path()).segments[0].clone();
    let base_lookup = fs::read(root.path().join(".coderg-index").join(&base.lookup)).unwrap();
    let mut merges = 0;
    for i in 0..24 {
        fs::write(
            root.path().join(format!("{i:02}.txt")),
            format!("updated unique record {i:02}\n"),
        )
        .unwrap();
        let result = run(root.path(), &["search", "updated unique"]);
        merges += usize::from(String::from_utf8_lossy(&result.stderr).contains("compacted"));
        let snapshot = manifest(root.path());
        assert_eq!(snapshot.segments[0], base);
        assert_eq!(snapshot.segments.len(), i + 2);
        // Each update writes only its delta; B and earlier deltas stay immutable.
        assert_eq!(
            fs::read_dir(root.path().join(".coderg-index/segments"))
                .unwrap()
                .count(),
            2 * (i + 2)
        );
        for j in 0..=i {
            matched(
                root.path(),
                &format!("updated unique record {j:02}"),
                &[&format!("{j:02}.txt")],
            );
        }
    }
    assert_eq!(merges, 0);
    assert_eq!(
        fs::read(root.path().join(".coderg-index").join(&base.lookup)).unwrap(),
        base_lookup
    );
    // Edit a file again while its previous version is still in a delta.
    fs::write(root.path().join("00.txt"), "edited after merge\n").unwrap();
    run(root.path(), &["search", "edited after merge"]);
    run(root.path(), &["compact"]);
    matched(root.path(), "edited after merge", &["00.txt"]);
    matched(root.path(), "updated unique record 00", &[]);
    assert_eq!(manifest(root.path()).segments.len(), 1);
}

#[test]
fn deletion_binary_rename_restore_and_compaction_preserve_current_versions() {
    let root = tempfile::tempdir().unwrap();
    for (path, content) in [
        ("a.txt", "base alpha"),
        ("b.txt", "base beta"),
        ("c.txt", "base gamma"),
    ] {
        fs::write(root.path().join(path), content).unwrap();
    }
    run(root.path(), &["index"]);
    fs::write(root.path().join("a.txt"), "changed alpha").unwrap();
    run(root.path(), &["search", "changed alpha"]);
    fs::remove_file(root.path().join("a.txt")).unwrap();
    fs::write(root.path().join("b.txt"), b"\0binary beta").unwrap();
    fs::rename(root.path().join("c.txt"), root.path().join("renamed.txt")).unwrap();
    run(root.path(), &["search", "base gamma"]);
    run(root.path(), &["compact"]);
    matched(root.path(), "alpha", &[]);
    matched(root.path(), "beta", &[]);
    matched(root.path(), "gamma", &["renamed.txt"]);
    fs::write(root.path().join("a.txt"), "base alpha").unwrap();
    fs::write(root.path().join("b.txt"), "restored beta").unwrap();
    run(root.path(), &["search", "restored beta"]);
    run(root.path(), &["compact"]);
    legacy_segments(root.path(), 3);
    let before = fs::read(root.path().join(".coderg-index/manifest.bin")).unwrap();
    run(root.path(), &["compact", "--dry-run"]);
    assert_eq!(
        before,
        fs::read(root.path().join(".coderg-index/manifest.bin")).unwrap()
    );
    // Compaction must use the already-indexed records, even if all source files
    // disappear while maintenance runs. Restore them before no-refresh queries.
    let files = [
        ("a.txt", "base alpha"),
        ("b.txt", "restored beta"),
        ("renamed.txt", "base gamma"),
    ];
    for (path, _) in files {
        fs::remove_file(root.path().join(path)).unwrap();
    }
    run(root.path(), &["compact"]);
    for (path, content) in files {
        fs::write(root.path().join(path), content).unwrap();
    }
    assert_eq!(manifest(root.path()).segments.len(), 1);
    matched(root.path(), "base alpha", &["a.txt"]);
    matched(root.path(), "restored beta", &["b.txt"]);
    matched(root.path(), "base gamma", &["renamed.txt"]);
}

#[test]
fn concurrent_refreshes_publish_one_content_update() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("source.txt"), "initial\n").unwrap();
    run(root.path(), &["index"]);
    let generation = manifest(root.path()).generation;
    fs::write(root.path().join("source.txt"), "concurrent unique needle\n").unwrap();
    let children: Vec<_> = (0..6)
        .map(|_| {
            Command::new(env!("CARGO_BIN_EXE_coderg"))
                .args(["search", "concurrent unique"])
                .arg(root.path())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap()
        })
        .collect();
    for child in children {
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("concurrent unique needle"));
    }
    let snapshot = manifest(root.path());
    assert_eq!(snapshot.generation, generation + 1);
    assert_eq!(snapshot.segments.len(), 2);
}

#[test]
fn unchanged_small_legacy_index_keeps_its_segments_until_explicit_compaction() {
    let root = tempfile::tempdir().unwrap();
    for i in 0..35 {
        fs::write(root.path().join(format!("{i:02}.txt")), "legacy needle\n").unwrap();
    }
    run(root.path(), &["index"]);
    legacy_segments(root.path(), 35);
    let before = manifest(root.path());
    for args in [
        vec!["search", "--no-refresh", "-Fl", "legacy needle"],
        vec!["search", "-Fl", "legacy needle"],
    ] {
        let output = run(root.path(), &args);
        assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 35);
        assert!(output.stderr.is_empty());
        assert_eq!(manifest(root.path()), before);
    }
    let stats = run(root.path(), &["stats"]);
    assert!(String::from_utf8_lossy(&stats.stdout).contains("maintenance due: false"));
    run(root.path(), &["compact"]);
    assert_eq!(manifest(root.path()).segments.len(), 1);
    matched(
        root.path(),
        "legacy needle",
        &(0..35)
            .map(|i| format!("{i:02}.txt"))
            .collect::<Vec<_>>()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn small_rebuild_includes_the_new_delta_and_resets_accumulated_bytes() {
    const LIMIT: u64 = 8 * 1024 * 1024;
    let root = tempfile::tempdir().unwrap();
    let mut random = 54321_u32;
    let mut source: Vec<u8> = (0..128 * 1024)
        .map(|_| {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            b' ' + (random % 95) as u8
        })
        .collect();
    source.extend_from_slice(b"\nold_boundary_marker\n");
    fs::write(root.path().join("source.txt"), &source).unwrap();
    run(root.path(), &["index"]);
    let directory = root.path().join(".coderg-index");
    let base = manifest(root.path()).segments[0].clone();
    let bytes = fs::metadata(directory.join(&base.lookup)).unwrap().len()
        + fs::metadata(directory.join(&base.postings)).unwrap().len();
    assert!(bytes < LIMIT);
    // Valid encoded copies bring the cumulative delta size just below 8 MiB.
    // Fully obsolete deltas still count toward the small-index rebuild budget.
    legacy_segments(root.path(), ((LIMIT - 1) / bytes + 1) as usize);
    let before = manifest(root.path());
    run(root.path(), &["search", "old_boundary_marker"]);
    assert_eq!(manifest(root.path()), before);
    source.extend_from_slice(b"\nnew_boundary_marker\n");
    fs::write(root.path().join("source.txt"), &source).unwrap();
    let output = run(root.path(), &["search", "new_boundary_marker"]);
    assert!(String::from_utf8_lossy(&output.stderr).contains("rebuilt index"));
    let after = manifest(root.path());
    assert_eq!(after.segments.len(), 1);
    assert_ne!(after.segments[0], base);
    assert_eq!(after.generation, before.generation + 1);
    matched(root.path(), "old_boundary_marker", &["source.txt"]);
    matched(root.path(), "new_boundary_marker", &["source.txt"]);
    let output = run(root.path(), &["search", "new_boundary_marker"]);
    assert!(output.stderr.is_empty());
    assert_eq!(manifest(root.path()), after);
    fs::write(root.path().join("source.txt"), "after rebuild marker\n").unwrap();
    let output = run(root.path(), &["search", "after rebuild marker"]);
    assert!(String::from_utf8_lossy(&output.stderr).contains("incrementally indexed"));
    assert_eq!(manifest(root.path()).segments.len(), 2);
    matched(root.path(), "old_boundary_marker", &[]);
}

#[test]
fn many_changed_files_below_eight_mib_do_not_rebuild() {
    let root = tempfile::tempdir().unwrap();
    for i in 0..70 {
        fs::write(root.path().join(format!("{i}.txt")), "old marker").unwrap();
    }
    run(root.path(), &["index"]);
    let base = manifest(root.path()).segments[0].clone();
    for i in 0..64 {
        fs::write(root.path().join(format!("{i}.txt")), "new marker").unwrap();
    }
    let output = run(root.path(), &["search", "-Fl", "new marker"]);
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 64);
    assert!(!String::from_utf8_lossy(&output.stderr).contains("rebuilt"));
    assert_eq!(manifest(root.path()).segments[0], base);
    assert_eq!(manifest(root.path()).segments.len(), 2);
}

#[test]
fn failed_explicit_compaction_keeps_the_manifest_and_cleans_scratch_files() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("changed.txt"), "initial changed\n").unwrap();
    fs::write(root.path().join("stable.txt"), "stable needle\n").unwrap();
    run(root.path(), &["index"]);
    fs::write(root.path().join("changed.txt"), "new unique needle\n").unwrap();
    run(root.path(), &["search", "needle"]);
    let before = manifest(root.path());
    let directory = root.path().join(".coderg-index");
    let path = directory.join(&before.segments[0].postings);
    let original = fs::read(&path).unwrap();
    fs::write(&path, [original.as_slice(), &[0]].concat()).unwrap();
    fs::write(root.path().join("changed.txt"), "new unique needle\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["compact"])
        .arg(root.path())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("trailing postings data"));
    assert_eq!(manifest(root.path()), before);
    assert_eq!(fs::read_dir(directory.join("segments")).unwrap().count(), 4);
    fs::write(&path, original).unwrap();
    run(root.path(), &["compact"]);
    matched(root.path(), "needle", &["changed.txt", "stable.txt"]);
    assert_eq!(fs::read_dir(directory.join("segments")).unwrap().count(), 6);
    fs::remove_file(root.path().join("changed.txt")).unwrap();
    fs::remove_file(root.path().join("stable.txt")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["search", "needle"])
        .arg(root.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(manifest(root.path()).segments.len(), 1);
    assert!(
        manifest(root.path())
            .documents
            .iter()
            .all(|doc| !doc.active)
    );
}

#[test]
fn obsolete_full_compaction_flag_is_rejected() {
    let root = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(["compact", "--full"])
        .arg(root.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(!root.path().join(".coderg-index").exists());
}
