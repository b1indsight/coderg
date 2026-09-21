#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn repository() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    git(temp.path(), &["init"]);
    git(temp.path(), &["config", "user.name", "Snapshot test"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    fs::write(temp.path().join(".gitignore"), ".coderg-index/\n").unwrap();
    temp
}

fn commit(root: &Path) -> String {
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "snapshot"]);
    git(root, &["rev-parse", "HEAD"])
}

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(args)
        .arg(root)
        .output()
        .unwrap()
}

fn matches(root: &Path, needle: &str, expected: &[&str]) -> String {
    let output = run(root, &["search", "-Fl", needle]);
    assert_eq!(
        output.status.code(),
        Some(if expected.is_empty() { 1 } else { 0 }),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let mut actual: Vec<_> = text.lines().collect();
    actual.sort_unstable();
    assert_eq!(actual, expected, "{needle}");
    String::from_utf8(output.stderr).unwrap()
}

fn manifest(root: &Path) -> manifest_format::Manifest {
    manifest_format::read(&root.join(".coderg-index/manifest.bin")).unwrap()
}

#[test]
fn bounded_initial_contents_and_streaming_files_preserve_both_versions() {
    let temp = repository();
    let root = temp.path();
    fs::create_dir_all(root.join("nested/目录")).unwrap();
    // Exceed the 16 MiB build's content-cache allowance with small files.
    for id in 0..12 {
        fs::write(
            root.join(format!("nested/目录/{id:02}")),
            "baseline_token\n".repeat(20_000),
        )
        .unwrap();
    }
    fs::write(root.join("large"), "large_baseline\n".repeat(90_000)).unwrap();
    fs::write(root.join("binary"), b"\0baseline_token").unwrap();
    commit(root);
    fs::write(root.join("nested/目录/00"), "dirty_small\n").unwrap();
    fs::write(root.join("large"), "large_dirty\n".repeat(100_000)).unwrap();
    fs::create_dir_all(root.join("untracked/deep")).unwrap();
    fs::write(root.join("untracked/deep/new"), "untracked_token\n").unwrap();
    let output = run(root, &["index", "--build-memory-mib", "16"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    matches(root, "dirty_small", &["nested/目录/00"]);
    matches(root, "large_dirty", &["large"]);
    matches(root, "large_baseline", &[]);
    matches(root, "untracked_token", &["untracked/deep/new"]);
    git(root, &["restore", "nested/目录/00", "large"]);
    assert!(matches(root, "large_baseline", &["large"]).contains("0 extracted"));
    let expected: Vec<_> = (0..12).map(|id| format!("nested/目录/{id:02}")).collect();
    matches(
        root,
        "baseline_token",
        &expected.iter().map(String::as_str).collect::<Vec<_>>(),
    );
}

#[test]
fn dirty_initialization_partial_commit_and_discard_preserve_base() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("x"), "original alpha\n").unwrap();
    fs::write(root.join("y"), "original beta\n").unwrap();
    let a = commit(root);
    fs::write(root.join("x"), "dirty initial gamma\n").unwrap();
    fs::write(root.join("new"), "untracked delta\n").unwrap();
    matches(root, "gamma", &["x"]);
    matches(root, "alpha", &[]);
    matches(root, "delta", &["new"]);

    fs::write(root.join("x"), "staged epsilon\n").unwrap();
    git(root, &["add", "x"]);
    fs::write(root.join("x"), "unstaged zeta\n").unwrap();
    matches(root, "zeta", &["x"]);
    git(root, &["commit", "-m", "partial"]);
    let log = matches(root, "zeta", &["x"]);
    assert!(log.contains("1 extracted"), "{log}");
    matches(root, "epsilon", &[]);
    git(root, &["restore", "x"]);
    assert!(matches(root, "epsilon", &["x"]).contains("0 extracted"));
    git(root, &["reset", "--hard", &a]);
    assert!(matches(root, "alpha", &["x"]).contains("0 extracted"));
    matches(root, "delta", &["new"]);
}

#[test]
fn compaction_stable_ids_binary_delete_rename_and_branch_revisit() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("a"), "alpha original\n").unwrap();
    fs::write(root.join("z"), "zulu original\n").unwrap();
    let a = commit(root);
    matches(root, "original", &["a", "z"]);
    let ids_a: Vec<_> = manifest(root)
        .documents
        .iter()
        .map(|d| d.path.clone())
        .collect();
    fs::remove_file(root.join("a")).unwrap();
    fs::rename(root.join("z"), root.join("m")).unwrap();
    fs::write(root.join("binary"), b"\0hidden needle").unwrap();
    let b = commit(root);
    matches(root, "original", &["m"]);
    matches(root, "hidden", &[]);
    let output = run(root, &["compact"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for _ in 0..3 {
        git(root, &["reset", "--hard", &a]);
        let log = matches(root, "original", &["a", "z"]);
        assert!(log.contains("0 extracted"), "{log}");
        let snapshot = manifest(root);
        for (id, path) in ids_a.iter().enumerate() {
            assert_eq!(&snapshot.documents[id].path, path);
        }
        git(root, &["reset", "--hard", &b]);
        let log = matches(root, "original", &["m"]);
        assert!(log.contains("0 extracted"), "{log}");
    }
}

#[test]
fn ignored_paths_subtree_and_checkout_transformations_use_actual_bytes() {
    let temp = repository();
    let root = temp.path();
    fs::create_dir(root.join("sub")).unwrap();
    fs::write(root.join("sub/x.txt"), "line needle\n").unwrap();
    fs::write(root.join("outside"), "outside needle\n").unwrap();
    fs::write(root.join(".gitattributes"), "*.txt text eol=crlf\n").unwrap();
    let a = commit(root);
    fs::remove_file(root.join("sub/x.txt")).unwrap();
    git(root, &["checkout", "--", "sub/x.txt"]);
    assert!(fs::read(root.join("sub/x.txt")).unwrap().ends_with(b"\r\n"));
    let sub = root.join("sub");
    matches(&sub, "needle", &["x.txt"]);
    matches(&sub, "outside", &[]);
    matches(&sub, "needle\r", &["x.txt"]);
    fs::write(sub.join(".ignore"), "x.txt\n").unwrap();
    matches(&sub, "needle", &[]);
    fs::remove_file(sub.join(".ignore")).unwrap();
    matches(&sub, "needle\r", &["x.txt"]);
    fs::write(sub.join("x.txt"), "modified token\n").unwrap();
    matches(&sub, "token", &["x.txt"]);
    git(root, &["reset", "--hard", &a]);
    matches(&sub, "needle\r", &["x.txt"]);
}

#[test]
fn new_tree_diff_handles_subtree_type_changes_and_dirty_overlays() {
    let temp = repository();
    let root = temp.path();
    let sub = root.join("sub");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("stable"), "stable original\n").unwrap();
    fs::write(sub.join("node"), "old original\n").unwrap();
    let a = commit(root);
    matches(&sub, "original", &["node", "stable"]);

    fs::remove_file(sub.join("node")).unwrap();
    fs::create_dir(sub.join("node")).unwrap();
    fs::write(sub.join("node/child"), "new committed\n").unwrap();
    let b = commit(root);
    // An unchanged committed path must retain its baseline even when the
    // worktree differs while we derive the new snapshot from the old tree.
    fs::write(sub.join("stable"), "dirty overlay\n").unwrap();
    matches(&sub, "committed", &["node/child"]);
    matches(&sub, "overlay", &["stable"]);
    matches(&sub, "original", &[]);
    git(root, &["reset", "--hard", &b]);
    matches(&sub, "original", &["stable"]);
    git(root, &["reset", "--hard", &a]);
    let log = matches(&sub, "original", &["node", "stable"]);
    assert!(log.contains("0 extracted"), "{log}");
    git(root, &["reset", "--hard", &b]);
    let log = matches(&sub, "committed", &["node/child"]);
    assert!(log.contains("0 extracted"), "{log}");
}

#[test]
fn subtree_diff_backfills_newly_visible_unchanged_files() {
    let temp = repository();
    let root = temp.path();
    let sub = root.join("sub");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("visible"), "visible marker\n").unwrap();
    fs::write(sub.join("hidden"), "hidden marker\n").unwrap();
    fs::write(sub.join(".ignore"), "hidden\n").unwrap();
    fs::write(root.join("visible"), "outside marker\n").unwrap();
    let a = commit(root);
    matches(&sub, "marker", &["visible"]);
    let segments = manifest(&sub).segments;
    fs::write(root.join("visible"), "outside changed\n").unwrap();
    commit(root);
    matches(&sub, "marker", &["visible"]);
    assert_eq!(segments, manifest(&sub).segments);
    fs::remove_file(sub.join(".ignore")).unwrap();
    let b = commit(root);
    matches(&sub, "marker", &["hidden", "visible"]);
    matches(&sub, "outside", &[]);
    git(root, &["reset", "--hard", &a]);
    matches(&sub, "marker", &["visible"]);
    git(root, &["reset", "--hard", &b]);
    matches(&sub, "marker", &["hidden", "visible"]);
}

#[cfg(unix)]
#[test]
fn diff_mode_changes_do_not_reuse_regular_files_as_symlinks() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("target"), "target needle\n").unwrap();
    fs::write(root.join("node"), "old needle\n").unwrap();
    let a = commit(root);
    matches(root, "needle", &["node", "target"]);
    fs::remove_file(root.join("node")).unwrap();
    std::os::unix::fs::symlink("target", root.join("node")).unwrap();
    commit(root);
    matches(root, "old", &[]);
    matches(root, "needle", &["target"]);
    git(root, &["reset", "--hard", &a]);
    matches(root, "old", &["node"]);
}

#[test]
fn subtree_diff_handles_absent_trees_and_untracked_content() {
    let temp = repository();
    let root = temp.path();
    let sub = root.join("sub");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("old"), "original needle\n").unwrap();
    let a = commit(root);
    matches(&sub, "needle", &["old"]);
    fs::remove_file(sub.join("old")).unwrap();
    commit(root);
    fs::write(sub.join("new"), "untracked needle\n").unwrap();
    matches(&sub, "original", &[]);
    matches(&sub, "needle", &["new"]);
    commit(root);
    matches(&sub, "needle", &["new"]);
    git(root, &["reset", "--hard", &a]);
    matches(&sub, "needle", &["old"]);
}

#[test]
fn legacy_git_manifest_migrates_and_stale_publication_state_is_rejected() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("x"), "original needle\n").unwrap();
    commit(root);
    matches(root, "original", &["x"]);
    let directory = root.join(".coderg-index");
    let state_path = directory.join("snapshots/state.bin");
    let original_state = fs::read(&state_path).unwrap();
    let mut legacy = manifest(root);
    legacy.registry = None;
    legacy.publication = None;
    fs::write(
        directory.join("manifest.bin"),
        manifest_format::encode(&legacy).unwrap(),
    )
    .unwrap();
    matches(root, "original", &["x"]);
    fs::write(root.join("x"), "updated needle\n").unwrap();
    commit(root);
    matches(root, "updated", &["x"]);
    assert!(manifest(root).publication.is_some());
    assert!(manifest(root).registry.is_some());
    // A valid but stale sidecar must not supply old content identities.
    fs::write(&state_path, original_state).unwrap();
    fs::write(root.join("x"), "latest needle\n").unwrap();
    matches(root, "latest", &["x"]);
    matches(root, "original", &[]);
    matches(root, "updated", &[]);
}

#[test]
fn cache_eviction_corruption_and_unchanged_search_are_safe() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("x"), "version initial\n").unwrap();
    let a = commit(root);
    matches(root, "initial", &["x"]);
    let mut obsolete_segment = None;
    for i in 0..12 {
        fs::write(root.join("x"), format!("version token_{i};\n")).unwrap();
        let name = format!("added_{i}");
        fs::write(root.join(&name), format!("new token_{i};\n")).unwrap();
        commit(root);
        let mut expected = vec![name.as_str(), "x"];
        expected.sort_unstable();
        matches(root, &format!("token_{i};"), &expected);
        if i == 0 {
            let current = manifest(root);
            let id = current
                .documents
                .iter()
                .find(|d| d.path == Path::new("x"))
                .unwrap()
                .segment_id;
            obsolete_segment = Some(
                current
                    .segments
                    .iter()
                    .find(|s| s.id == id)
                    .unwrap()
                    .lookup
                    .clone(),
            );
        }
    }
    let cache = root.join(".coderg-index/snapshots");
    assert!(fs::read_dir(&cache).unwrap().count() <= 9);
    let state = cache.join("state.bin");
    let before = fs::read(&state).unwrap();
    let generation = manifest(root).generation;
    assert!(matches(root, "token_11", &["added_11", "x"]).is_empty());
    assert_eq!(manifest(root).generation, generation);
    assert_eq!(fs::read(&state).unwrap(), before);
    // A corrupt optional sidecar does not break a normal unchanged search.
    fs::write(&state, b"torn state").unwrap();
    matches(root, "token_11", &["added_11", "x"]);
    git(root, &["reset", "--hard", &a]);
    matches(root, "initial", &["x"]);
    matches(root, "token_11", &[]);
    #[cfg(unix)]
    assert!(
        !root
            .join(".coderg-index")
            // The initial segment still contains the unchanged .gitignore;
            // only the first delta is now guaranteed to be unreferenced.
            .join(obsolete_segment.unwrap())
            .exists()
    );
}

#[test]
fn nonsearchable_documents_do_not_keep_obsolete_segments_in_the_live_view() {
    let temp = repository();
    let root = temp.path();
    fs::write(root.join("x"), "original needle\n").unwrap();
    fs::write(root.join("binary"), b"\0hidden needle").unwrap();
    let original = commit(root);
    matches(root, "original", &["x"]);
    let old_segments = manifest(root).segments;
    // Move every searchable document off the initial segment, while the
    // unchanged binary keeps its old metadata-only segment ID.
    fs::write(root.join(".gitignore"), ".coderg-index/\n# changed\n").unwrap();
    for i in 0..3 {
        fs::write(root.join("x"), format!("replacement_{i} needle\n")).unwrap();
        commit(root);
        matches(root, &format!("replacement_{i}"), &["x"]);
        let current = manifest(root);
        assert!(
            current
                .segments
                .iter()
                .all(|s| !old_segments.iter().any(|old| old.id == s.id))
        );
        assert!(current.segments.iter().all(|s| {
            current
                .documents
                .iter()
                .any(|d| d.active && d.searchable && d.segment_id == s.id)
        }));
        matches(root, "hidden", &[]);
    }
    git(root, &["checkout", "--detach", &original]);
    assert!(matches(root, "original", &["x"]).contains("0 extracted"));
    matches(root, "hidden", &[]);
}

#[test]
fn small_snapshot_accumulates_segments_and_preserves_cached_rollback() {
    let temp = repository();
    let root = temp.path();
    let body: String = (0..10000)
        .map(|i| format!("stable_anchor symbol_{i:08};\n"))
        .collect();
    fs::write(root.join("stable.rs"), body).unwrap();
    commit(root);
    matches(root, "stable_anchor", &["stable.rs"]);
    let initial = manifest(root);
    let anchor = initial
        .documents
        .iter()
        .find(|d| d.path == Path::new("stable.rs"))
        .unwrap()
        .segment_id;
    let mut paths = Vec::new();
    let mut recent = String::new();
    for i in 0..18 {
        let name = format!("delta_{i:02}.rs");
        fs::write(root.join(&name), format!("delta_marker revision_{i:02}\n")).unwrap();
        let head = commit(root);
        paths.push(name);
        matches(
            root,
            "delta_marker",
            &paths.iter().map(String::as_str).collect::<Vec<_>>(),
        );
        let current = manifest(root);
        assert_eq!(
            current
                .documents
                .iter()
                .find(|d| d.path == Path::new("stable.rs"))
                .unwrap()
                .segment_id,
            anchor
        );
        assert_eq!(current.segments.len(), initial.segments.len() + i + 1);
        if i == 15 {
            recent = head;
        }
    }
    let latest = git(root, &["rev-parse", "HEAD"]);
    git(root, &["checkout", "--detach", &recent]);
    let diagnostic = matches(
        root,
        "delta_marker",
        &paths[..16].iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert!(diagnostic.contains("0 extracted"));
    git(root, &["checkout", "--detach", &latest]);
    assert!(
        matches(
            root,
            "delta_marker",
            &paths.iter().map(String::as_str).collect::<Vec<_>>()
        )
        .contains("0 extracted")
    );
    matches(root, "stable_anchor", &["stable.rs"]);
}

#[test]
fn worktree_dot_git_file_and_empty_repository_are_supported() {
    let temp = repository();
    let root = temp.path();
    // Unborn HEAD stays on the non-snapshot path.
    matches(root, "absent", &[]);
    fs::write(root.join("x"), "worktree first\n").unwrap();
    let a = commit(root);
    let linked = tempfile::tempdir().unwrap();
    let path = linked.path().join("checkout");
    git(
        root,
        &["worktree", "add", "--detach", path.to_str().unwrap(), &a],
    );
    assert!(path.join(".git").is_file());
    matches(&path, "first", &["x"]);
    fs::write(path.join("x"), "worktree second\n").unwrap();
    matches(&path, "second", &["x"]);
    git(&path, &["restore", "x"]);
    assert!(matches(&path, "first", &["x"]).contains("0 extracted"));
}
