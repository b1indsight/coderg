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
fn default_search_matches_real_lines_without_crossing_newlines() {
    let root = tempfile::tempdir().unwrap();
    for (name, text) in [
        ("cross.txt", "async\n\ndef pack\n"),
        ("inline.txt", "async def pack\n"),
        ("blanks.txt", " \n\nx\n"),
        ("empty.txt", ""),
    ] {
        fs::write(root.path().join(name), text).unwrap();
    }
    assert!(coderg(root.path(), &["index"]).status.success());
    for (arguments, mut expected) in [
        (
            vec![r"async\s+def\s+\w+"],
            vec!["inline.txt:1:async def pack"],
        ),
        (
            vec![r"(?s)async.*pack"],
            vec!["inline.txt:1:async def pack"],
        ),
        (
            vec![r"^[ \t]*$"],
            vec!["blanks.txt:1: ", "blanks.txt:2:", "cross.txt:2:"],
        ),
        (vec![r"\Aasync\z"], vec!["cross.txt:1:async"]),
        (
            vec!["-c", "^"],
            vec!["blanks.txt:3", "cross.txt:3", "inline.txt:1"],
        ),
        (
            vec!["-m", "1", r"^[ \t]*$"],
            vec!["blanks.txt:1: ", "cross.txt:2:"],
        ),
    ] {
        let mut command = vec!["search", "--no-refresh"];
        command.extend(arguments);
        let output = coderg(root.path(), &command);
        assert!(output.status.success(), "{command:?}: {output:?}");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut actual: Vec<_> = stdout.lines().collect();
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected, "{command:?}");
    }
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
        ("quoted-devices.txt", "\"cuda\"\n'cpu'\n\"rocm'\n"),
        ("unquoted-devices.txt", "cuda\ncpu_count\nrocm_backend\n"),
        ("decorator-torch.txt", "@torch.no_grad()\n"),
        (
            "decorator-pytest.txt",
            "    @pytest.mark.parametrize('x', [1])\n",
        ),
        (
            "decorator-decoys.txt",
            "@torch.123\n@pytest.\nvalue = torch.no_grad()\n",
        ),
        ("long-left.txt", "abcdefghijklmnopqrstuvwxyz_0123456789\n"),
        ("long-right.txt", "zyxwvutsrqponmlkjihgfedcba_9876543210\n"),
        ("long-upper.txt", "ABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789\n"),
        (
            "repeated.txt",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
        ),
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
        (r#"["'](?:cuda|cpu|rocm)["']"#, false, false),
        (r"^[ \t]*@(?:torch|pytest)\.[A-Za-z_]+", false, false),
        ("abcdefghijklmnopqrstuvwxyz_0123456789", false, true),
        ("abcdefghijklmnopqrstuvwxyz_0123456789", true, true),
        (
            "abcdefghijklmnopqrstuvwxyz_0123456789|zyxwvutsrqponmlkjihgfedcba_9876543210",
            false,
            false,
        ),
        ("abcdefghijklmnopqrstuvwxyz_0123456789|x", false, false),
        ("(?:abcdefghijklmnopqrstuvwxyz_0123456789)?", false, false),
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false, true),
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

#[test]
fn refresh_tracks_nested_files_and_changed_ignore_rules() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join(".ignore"), "*.skip\n").unwrap();
    let mut expected = Vec::new();
    for shard in 0..32 {
        let directory = root.path().join(format!("shard-{shard:02}"));
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("source.txt"), "needle\n").unwrap();
        fs::write(directory.join("hidden.skip"), "needle\n").unwrap();
        expected.push(format!("shard-{shard:02}/source.txt"));
    }
    let check = |expected: &[String]| {
        let output = coderg(root.path(), &["search", "-l", "needle"]);
        assert!(output.status.success(), "{:?}", output);
        let mut actual: Vec<_> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect();
        actual.sort_by(|left, right| std::path::Path::new(left).cmp(std::path::Path::new(right)));
        assert_eq!(actual, expected);
        output.stderr
    };
    check(&expected);

    fs::write(root.path().join(".ignore"), "").unwrap();
    fs::remove_file(root.path().join("shard-00/source.txt")).unwrap();
    fs::rename(
        root.path().join("shard-01/source.txt"),
        root.path().join("shard-01/renamed.txt"),
    )
    .unwrap();
    fs::write(root.path().join("shard-02/source.txt"), "haystack\n").unwrap();
    fs::create_dir_all(root.path().join(".hidden/deep")).unwrap();
    fs::write(root.path().join(".hidden/deep/new.txt"), "needle\n").unwrap();
    expected.retain(|path| !path.starts_with("shard-00/") && !path.starts_with("shard-02/"));
    expected[0] = "shard-01/renamed.txt".to_owned();
    expected.push(".hidden/deep/new.txt".to_owned());
    expected.extend((0..32).map(|shard| format!("shard-{shard:02}/hidden.skip")));
    expected.sort_by(|left, right| std::path::Path::new(left).cmp(std::path::Path::new(right)));
    assert!(!check(&expected).is_empty());

    let manifest_path = root.path().join(".coderg-index/manifest.json");
    let manifest = fs::read(&manifest_path).unwrap();
    assert!(check(&expected).is_empty());
    assert_eq!(fs::read(manifest_path).unwrap(), manifest);
}

fn coderg(root: &std::path::Path, arguments: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_coderg"));
    command.args(arguments).arg(root);
    command.output().unwrap()
}
