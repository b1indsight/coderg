use std::{fs, process::Command};

#[test]
fn indexes_searches_and_refreshes_a_tree() {
    let root = tempfile::tempdir().unwrap();
    fs::write(
        root.path().join("alpha.rs"),
        "fn alpha() { println!(\"needle\"); }\n",
    )
    .unwrap();
    fs::write(root.path().join("beta.txt"), "nothing here\n").unwrap();

    let first = coderg(root.path(), &["search", "needle"]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&first.stdout),
        "alpha.rs:1:fn alpha() { println!(\"needle\"); }\n"
    );
    assert!(root.path().join(".coderg-index/segments").is_dir());

    fs::write(root.path().join("beta.txt"), "a newly added needle\n").unwrap();
    let refreshed = coderg(root.path(), &["search", "-l", "needle"]);
    assert!(refreshed.status.success());
    assert_eq!(
        String::from_utf8_lossy(&refreshed.stdout),
        "alpha.rs\nbeta.txt\n"
    );
    assert!(String::from_utf8_lossy(&refreshed.stderr).contains("incrementally indexed 1"));

    fs::remove_file(root.path().join("beta.txt")).unwrap();
    let deleted = coderg(root.path(), &["search", "-l", "needle"]);
    assert!(deleted.status.success());
    assert_eq!(String::from_utf8_lossy(&deleted.stdout), "alpha.rs\n");
    assert!(String::from_utf8_lossy(&deleted.stderr).contains("incrementally indexed 1"));

    let insensitive = coderg(root.path(), &["search", "-i", "NEEDLE"]);
    assert!(insensitive.status.success());
    assert!(String::from_utf8_lossy(&insensitive.stdout).contains("alpha.rs:1:"));

    let no_literal = coderg(root.path(), &["search", "n.e+d.e"]);
    assert!(no_literal.status.success());

    let missing = coderg(root.path(), &["search", "does-not-exist"]);
    assert_eq!(missing.status.code(), Some(1));
}

fn coderg(root: &std::path::Path, arguments: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_coderg"));
    command.args(arguments).arg(root);
    command.output().unwrap()
}
