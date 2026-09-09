#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn coderg(root: &Path, index: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_coderg"))
        .current_dir(root)
        .args(args)
        .arg("--index-dir")
        .arg(index)
        .output()
        .unwrap()
}

fn check(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn small_budget_preserves_index_bytes_and_incremental_search() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("source");
    let low = directory.path().join("low");
    let high = directory.path().join("high");
    fs::create_dir(&root).unwrap();
    let mut random = 54321_u32;
    let mut source: Vec<u8> = (0..1024 * 1024)
        .map(|_| {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            b' ' + (random % 95) as u8
        })
        .collect();
    let marker = b"unique_boundary_marker_0123456789";
    source[32 * 1024 - 10..32 * 1024 - 10 + marker.len()].copy_from_slice(marker);
    fs::write(root.join("large.txt"), &source).unwrap();
    fs::write(root.join("empty.txt"), "").unwrap();
    fs::write(root.join("binary.dat"), [0, 1, 2, 3]).unwrap();
    let bounded = coderg(&root, &low, &["index", "--build-memory-mib", "16"]);
    check(&bounded);
    assert!(String::from_utf8_lossy(&bounded.stderr).contains("spilled"));
    let normal = coderg(&root, &high, &["index"]);
    check(&normal);
    assert!(!String::from_utf8_lossy(&normal.stderr).contains("spilled"));
    let manifest = |index: &Path| -> serde_json::Value {
        serde_json::to_value(
            manifest_format::read(&index.join(manifest_format::FILE_NAME)).unwrap(),
        )
        .unwrap()
    };
    let low_manifest = manifest(&low);
    let high_manifest = manifest(&high);
    for field in ["lookup", "postings"] {
        let path = |manifest: &serde_json::Value| {
            manifest["segments"][0][field].as_str().unwrap().to_owned()
        };
        assert_eq!(
            fs::read(low.join(path(&low_manifest))).unwrap(),
            fs::read(high.join(path(&high_manifest))).unwrap()
        );
    }
    assert_eq!(low_manifest["documents"].as_array().unwrap().len(), 3);
    let result = coderg(
        &root,
        &low,
        &[
            "search",
            "-Fl",
            "unique_boundary_marker_0123456789",
            "--no-refresh",
        ],
    );
    check(&result);
    assert_eq!(result.stdout, b"large.txt\n");

    source.extend_from_slice(b"\nnew_incremental_marker\n");
    fs::write(root.join("large.txt"), &source).unwrap();
    let refreshed = coderg(
        &root,
        &low,
        &[
            "search",
            "-Fl",
            "new_incremental_marker",
            "--build-memory-mib",
            "16",
        ],
    );
    check(&refreshed);
    assert_eq!(refreshed.stdout, b"large.txt\n");
    let stderr = String::from_utf8_lossy(&refreshed.stderr);
    assert!(stderr.contains("spilled"));
    assert!(stderr.contains("incrementally indexed 1"));
    assert_eq!(manifest(&low)["segments"].as_array().unwrap().len(), 2);
    assert!(!fs::read_dir(&low).unwrap().any(|entry| {
        entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".build-")
    }));
}

#[test]
fn invalid_budgets_fail_before_creating_an_index() {
    let directory = tempfile::tempdir().unwrap();
    let index = directory.path().join("index");
    for value in ["0", "15", "garbage", "18446744073709551615"] {
        let output = coderg(
            directory.path(),
            &index,
            &["index", "--build-memory-mib", value],
        );
        assert_eq!(output.status.code(), Some(2));
        assert!(!index.exists());
    }
}
