//! Isolated, pinned corpora and deterministic synthetic Git history.
use super::{
    config::{Corpus, Source},
    process::{checked, git, hash_file},
    report::Report,
};
use anyhow::{Context, Result, ensure};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Prepared {
    pub config: Corpus,
    pub root: PathBuf,
    pub commits: Vec<String>,
    pub history_kind: &'static str,
}

pub fn configure_git(root: &Path) -> Result<()> {
    for (key, value) in [
        ("user.name", "Coderg benchmark"),
        ("user.email", "bench@example.invalid"),
        ("commit.gpgsign", "false"),
        ("core.hooksPath", "/dev/null"),
        ("core.autocrlf", "false"),
    ] {
        git(root, &["config", key, value])?;
    }
    Ok(())
}

pub fn commit(root: &Path, label: &str, step: usize) -> Result<String> {
    git(root, &["add", "-A"])?;
    let date = format!("{} +0000", 1_600_000_000 + step);
    checked(
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args([
                "commit",
                "--quiet",
                "--no-verify",
                "--no-gpg-sign",
                "-m",
                label,
            ])
            .env("GIT_AUTHOR_DATE", &date)
            .env("GIT_COMMITTER_DATE", &date),
        false,
    )?;
    git(root, &["rev-parse", "HEAD"])
}

pub fn clone_at(source: &Path, destination: &Path, revision: &str) -> Result<()> {
    checked(
        Command::new("git")
            .args(["clone", "--quiet", "--shared", "--no-checkout"])
            .arg(source)
            .arg(destination),
        false,
    )?;
    configure_git(destination)?;
    git(destination, &["checkout", "--quiet", "--detach", revision])?;
    Ok(())
}

/// Reject missing/insufficient inputs before performing any benchmark measurements.
pub fn preflight(corpus: &Corpus) -> Result<()> {
    match &corpus.source {
        Source::Git { path, commit } => {
            ensure!(
                path.is_dir(),
                "{}: missing source {}",
                corpus.name,
                path.display()
            );
            let commits = git(
                path,
                &[
                    "rev-list",
                    "--first-parent",
                    &format!("--max-count={}", corpus.updates + 1),
                    commit,
                ],
            )?;
            ensure!(
                commits.lines().count() == corpus.updates + 1,
                "{}: need {} commits including base",
                corpus.name,
                corpus.updates + 1
            );
        }
        Source::Archive { path, .. } => {
            ensure!(
                path.is_dir() && !path.join(".git").exists(),
                "{}: expected an archive directory without .git",
                corpus.name
            );
            let mut files = Vec::new();
            walk(path, path, &mut files)?;
            ensure!(
                !files.is_empty(),
                "{}: archive has no regular files",
                corpus.name
            );
        }
        Source::Generated { .. } => {}
    }
    Ok(())
}

pub fn prepare(
    config: &Corpus,
    workspace: &Path,
    seed: u64,
    report: &mut Report,
) -> Result<Prepared> {
    let root = workspace.join(&config.name);
    let history_kind = match &config.source {
        Source::Git { path, commit } => {
            clone_at(&path.canonicalize()?, &root, commit)?;
            "real"
        }
        Source::Archive { path, revision } => {
            let mut paths = Vec::new();
            walk(path, path, &mut paths)?;
            paths.sort();
            let mut hashes = BTreeMap::new();
            for relative in &paths {
                hashes.insert(relative, hash_file(&path.join(relative))?);
            }
            let bytes = serde_json::to_vec(&hashes)?;
            fs::write(
                report
                    .path
                    .join(format!("{}-source-files.json", config.name)),
                &bytes,
            )?;
            report.event(json!({"event": "archive_identity", "corpus": config.name, "declared_revision": revision, "files": paths.len(), "manifest_sha256": format!("{:x}", Sha256::digest(&bytes))}))?;
            copy_tree(path, &root)?;
            for (relative, sha) in &hashes {
                ensure!(
                    hash_file(&root.join(relative))? == *sha,
                    "archive changed while copying: {}",
                    relative.display()
                );
            }
            git(&root, &["init", "--quiet"])?;
            configure_git(&root)?;
            commit(&root, "benchmark archive base", 0)?;
            "synthetic"
        }
        Source::Generated {
            files,
            bytes_per_file,
        } => {
            fs::create_dir(&root)?;
            for i in 0..*files {
                let mut content = format!("// CODERG_FIXTURE_{i} SamplingParams\nfn main() {{}}\n");
                if i % 7 == 0 {
                    content.push_str("// CODERG_BENCHMARK_NEEDLE TODO\n");
                }
                while content.len() < *bytes_per_file {
                    content.push_str("// ordinary_value return 1234 import module\n");
                }
                fs::write(
                    root.join(format!("file_{i:05}{}", config.extension)),
                    content,
                )?;
            }
            git(&root, &["init", "--quiet"])?;
            configure_git(&root)?;
            commit(&root, "benchmark generated base", 0)?;
            "synthetic"
        }
    };
    if history_kind == "synthetic" {
        for step in 1..=config.updates {
            let operation = synthetic_edit(&root, &config.extension, seed, step)?;
            let id = commit(&root, &format!("benchmark update {step}"), step)?;
            report.event(json!({"event": "synthetic_commit", "generator_version": 1, "seed": seed, "corpus": config.name, "step": step, "commit": id, "tree": git(&root, &["rev-parse", "HEAD^{tree}"])?, "operation": operation}))?;
        }
    }
    let mut commits: Vec<_> = git(
        &root,
        &[
            "rev-list",
            "--first-parent",
            &format!("--max-count={}", config.updates + 1),
            "HEAD",
        ],
    )?
    .lines()
    .map(str::to_owned)
    .collect();
    commits.reverse();
    ensure!(
        commits.len() == config.updates + 1,
        "incomplete prepared history"
    );
    report.event(json!({"event": "prepared", "corpus": config.name, "history_kind": history_kind, "commits": commits}))?;
    Ok(Prepared {
        config: config.clone(),
        root,
        commits,
        history_kind,
    })
}

pub fn candidates(root: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let output = checked(
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["ls-files", "-z"]),
        false,
    )?;
    let mut paths = Vec::new();
    for name in output.stdout.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let path = PathBuf::from(
            std::str::from_utf8(name).context("benchmark mutation requires UTF-8 source paths")?,
        );
        if !path.to_string_lossy().ends_with(extension) {
            continue;
        }
        let metadata = fs::symlink_metadata(root.join(&path))?;
        if metadata.is_file() && metadata.len() > 0 && metadata.len() <= 1024 * 1024 {
            paths.push(path);
        }
    }
    paths.sort();
    ensure!(
        !paths.is_empty(),
        "no editable {extension} files in {}",
        root.display()
    );
    Ok(paths)
}

pub fn append(root: &Path, paths: &[PathBuf], marker: &str, extension: &str) -> Result<()> {
    let prefix = if extension == ".py" { "#" } else { "//" };
    for path in paths {
        let mut file = OpenOptions::new().append(true).open(root.join(path))?;
        writeln!(file)?;
        for line in 0..32 {
            writeln!(file, "{prefix} {marker} {line:04} benchmark source change")?;
        }
    }
    Ok(())
}

/// A repeatable five-step cycle exercises edits, bursts, add, rename and delete.
fn synthetic_edit(
    root: &Path,
    extension: &str,
    seed: u64,
    step: usize,
) -> Result<serde_json::Value> {
    let added = root.join(format!("coderg_bench_added{extension}"));
    let renamed = root.join(format!("coderg_bench_renamed{extension}"));
    let label = match step % 5 {
        1 | 2 => {
            let mut paths = candidates(root, extension)?;
            let mut state = seed.wrapping_add(step as u64);
            super::process::shuffle(&mut paths, &mut state);
            let count = if step % 5 == 2 {
                (paths.len() / 100).max(1)
            } else {
                1
            };
            paths.truncate(count);
            append(root, &paths, &format!("CODERG_HISTORY_{step}"), extension)?;
            return Ok(
                json!({"kind": "edit", "paths": paths, "diff_numstat": git(root, &["diff", "--numstat"])?}),
            );
        }
        3 => {
            ensure!(
                !added.exists() && !renamed.exists(),
                "fixture path collision"
            );
            fs::write(&added, "// CODERG_HISTORY_ADDED\n")?;
            "add"
        }
        4 => {
            fs::rename(&added, &renamed)?;
            "rename"
        }
        _ => {
            fs::remove_file(&renamed)?;
            "delete"
        }
    };
    Ok(json!({"kind": label, "added_path": added.file_name(), "renamed_path": renamed.file_name()}))
}

/// Walk regular files without following symlinks; .git is never part of a corpus digest.
pub fn walk(root: &Path, current: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        if entry.file_name() == ".git" {
            continue;
        }
        let kind = entry.file_type()?;
        if kind.is_dir() {
            walk(root, &entry.path(), paths)?;
        } else if kind.is_file() {
            paths.push(entry.path().strip_prefix(root)?.into());
        }
    }
    Ok(())
}

pub fn copy_tree(source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir(dest)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == ".git" {
            continue;
        }
        let target = dest.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), target)?;
        } else if kind.is_symlink() {
            #[cfg(unix)]
            std::os::unix::fs::symlink(fs::read_link(entry.path())?, target)?;
            #[cfg(not(unix))]
            anyhow::bail!("copying corpus symlinks requires Unix");
        }
    }
    Ok(())
}

pub fn directory_bytes(path: &Path) -> Result<u64> {
    let mut paths = Vec::new();
    walk(path, path, &mut paths)?;
    paths.iter().try_fold(0u64, |total, p| {
        Ok(total + fs::metadata(path.join(p))?.len())
    })
}
