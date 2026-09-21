#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use std::{fs, path::Path, process::Command};

#[test]
fn small_index_commits_promote_and_rollbacks_reuse_snapshot_segments() {
    let root = tempfile::tempdir().unwrap();
    git(root.path(), &["init"]);
    git(root.path(), &["config", "user.name", "Coderg Test"]);
    git(
        root.path(),
        &["config", "user.email", "coderg@example.invalid"],
    );
    fs::write(root.path().join("source.txt"), "old needle\n").unwrap();
    fs::write(root.path().join("stable.txt"), "stable haystack\n").unwrap();
    git(root.path(), &["add", "source.txt", "stable.txt"]);
    git(root.path(), &["commit", "-m", "commit A"]);

    let initial = coderg(root.path(), &["search", "-F", "old needle"]);
    assert!(initial.status.success());
    let manifest_a = manifest(root.path());
    assert_eq!(manifest_a["segments"].as_array().unwrap().len(), 1);
    let tree_a = manifest_a["git_tree"].as_str().unwrap().to_owned();

    // Active legacy manifests remain readable. The old unverified tree-cache
    // directory is unrelated to the new content-verified snapshot cache.
    let binary = root.path().join(".coderg-index/manifest.bin");
    let records = manifest_format::read(&binary).unwrap();
    fs::create_dir_all(root.path().join(".coderg-index/manifests")).unwrap();
    fs::write(
        root.path()
            .join(format!(".coderg-index/manifests/{tree_a}.bin")),
        manifest_format::encode(&records).unwrap(),
    )
    .unwrap();
    fs::write(
        binary.with_extension("json"),
        serde_json::to_vec_pretty(&records).unwrap(),
    )
    .unwrap();
    fs::remove_file(binary).unwrap();

    git(root.path(), &["commit", "--allow-empty", "-m", "same tree"]);
    let same_tree_commit = coderg(root.path(), &["search", "-F", "old needle"]);
    assert!(same_tree_commit.status.success());
    assert!(String::from_utf8_lossy(&same_tree_commit.stderr).contains("advanced index"));
    let already_advanced = coderg(root.path(), &["search", "-F", "old needle"]);
    assert!(already_advanced.status.success());
    assert!(already_advanced.stderr.is_empty());

    fs::write(root.path().join("source.txt"), "new needle\n").unwrap();
    let modified = coderg(root.path(), &["search", "-F", "new needle"]);
    assert!(modified.status.success());
    assert!(String::from_utf8_lossy(&modified.stderr).contains("incrementally indexed 1"));
    assert_eq!(
        manifest(root.path())["segments"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        coderg(root.path(), &["search", "-F", "old needle"])
            .status
            .code(),
        Some(1)
    );

    git(root.path(), &["add", "source.txt"]);
    git(root.path(), &["commit", "-m", "commit B"]);
    let committed = coderg(root.path(), &["search", "-F", "new needle"]);
    assert!(committed.status.success());
    assert!(String::from_utf8_lossy(&committed.stderr).contains("advanced index"));
    let manifest_b = manifest(root.path());
    assert_eq!(manifest_b["segments"].as_array().unwrap().len(), 2);
    assert_ne!(manifest_b["git_tree"].as_str().unwrap(), tree_a);

    git(root.path(), &["reset", "--hard", "HEAD^"]);
    let rolled_back = coderg(root.path(), &["search", "-F", "old needle"]);
    assert!(rolled_back.status.success());
    assert!(String::from_utf8_lossy(&rolled_back.stderr).contains("incrementally indexed 1"));
    let restored = manifest(root.path());
    assert_eq!(restored["git_tree"].as_str().unwrap(), tree_a);
    assert!(String::from_utf8_lossy(&rolled_back.stderr).contains("0 extracted"));
    assert_eq!(restored["segments"].as_array().unwrap().len(), 1);
    assert_eq!(restored["segments"][0], manifest_a["segments"][0]);
    assert_eq!(restored["segments"][0], manifest_b["segments"][0]);
    assert_eq!(
        coderg(root.path(), &["search", "-F", "new needle"])
            .status
            .code(),
        Some(1)
    );

    fs::write(root.path().join("source.txt"), "dirty needle\n").unwrap();
    let dirty_after_rollback = coderg(root.path(), &["search", "-F", "dirty needle"]);
    assert!(dirty_after_rollback.status.success());
    assert!(
        String::from_utf8_lossy(&dirty_after_rollback.stderr).contains("incrementally indexed 1")
    );
    let dirty_manifest = manifest(root.path());
    assert_eq!(dirty_manifest["segments"].as_array().unwrap().len(), 2);
    let stable = dirty_manifest["documents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|document| document["path"] == "stable.txt")
        .unwrap();
    assert_eq!(stable["segment_id"], dirty_manifest["segments"][0]["id"]);
}

fn git(root: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn coderg(root: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_coderg"))
        .args(arguments)
        .arg(root)
        .output()
        .unwrap()
}

fn manifest(root: &Path) -> serde_json::Value {
    serde_json::to_value(manifest_format::read(&root.join(".coderg-index/manifest.bin")).unwrap())
        .unwrap()
}
