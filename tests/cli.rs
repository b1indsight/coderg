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

#[test]
fn case_insensitive_index_search_agrees_with_regex_matching() {
    let root = tempfile::tempdir().unwrap();
    let documents = [
        ("mixed.txt", "AsyncMock\nCuDa\nfooBAR\na+b[1]\n"),
        ("upper.txt", "ASYNCMOCK\nROCM\nFOOBAR\nA+B[1]\n"),
        ("unicode.txt", "AſyncMocK\nKelvin\nſigma\nΣςσ\n"),
        ("short.txt", "x\nbar\n42\n"),
        ("optional.txt", "foofooBAR\n"),
        ("unrelated.txt", "ordinary unrelated text\n"),
    ];
    for (name, text) in documents {
        fs::write(root.path().join(name), text).unwrap();
    }
    assert!(coderg(root.path(), &["index"]).status.success());
    let manifest_path = root.path().join(".coderg-index/manifest.json");
    let original_manifest = fs::read(&manifest_path).unwrap();
    for (pattern, ignore_case, fixed) in [
        ("AsyncMock", true, false),
        (r"\b(cuda|rocm)\b", true, false),
        ("kelvin|sigma|σσσ", true, false),
        ("(?:foo|x)", true, false),
        ("(?:foo)?bar", true, false),
        ("(?:foo)+bar", true, false),
        ("(?i:foo)(?-i:BAR)", false, false),
        ("a+b[1]", true, true),
        (r"\b[0-9]{2}\b", true, false),
        ("AsyncMock", false, false),
    ] {
        let mut arguments = vec!["search", "-l", "--no-refresh"];
        if ignore_case {
            arguments.push("-i");
        }
        if fixed {
            arguments.push("-F");
        }
        arguments.push(pattern);
        let result = coderg(root.path(), &arguments);
        let regex_pattern = if fixed {
            regex::escape(pattern)
        } else {
            pattern.to_owned()
        };
        let regex = regex::RegexBuilder::new(&regex_pattern)
            .case_insensitive(ignore_case)
            .multi_line(true)
            .build()
            .unwrap();
        let expected: std::collections::BTreeSet<_> = documents
            .iter()
            .filter(|(_, text)| regex.is_match(text))
            .map(|(name, _)| *name)
            .collect();
        assert_eq!(
            result.status.code(),
            Some(if expected.is_empty() { 1 } else { 0 }),
            "{pattern}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let output = String::from_utf8(result.stdout).unwrap();
        let actual: std::collections::BTreeSet<_> = output.lines().collect();
        assert_eq!(actual, expected, "pattern {pattern:?}");
    }
    assert_eq!(fs::read(manifest_path).unwrap(), original_manifest);
}

fn coderg(root: &std::path::Path, arguments: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_coderg"));
    command.args(arguments).arg(root);
    command.output().unwrap()
}
