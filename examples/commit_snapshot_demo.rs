//! Run with --help. All Git mutations are confined to a disposable fixture.
#![allow(dead_code)]
#[path = "../src/build.rs"]
mod build;
#[path = "../src/compaction.rs"]
mod compaction;
#[path = "../src/git_state.rs"]
mod git_state;
#[path = "../src/index.rs"]
mod index;
#[path = "../src/manifest.rs"]
mod manifest;
#[path = "../src/ngram.rs"]
mod ngram;
#[path = "../src/segment.rs"]
mod segment;
#[path = "support/commit_snapshot.rs"]
mod snapshot;

use anyhow::{Result, ensure};
use clap::Parser;
use git2::{Oid, Repository, Signature};
use serde::Serialize;
use snapshot::{Blob, Files, IncrementalIndex, Retained, SnapshotIndex, Work, contains};
use std::{fs, path::PathBuf, time::Instant};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 1024)]
    files: usize,
    #[arg(long, default_value_t = 8192)]
    bytes_per_file: usize,
    #[arg(long, default_value_t = 50)]
    changed_percent: usize,
    #[arg(long, default_value_t = 5)]
    iterations: usize,
    /// Disposable repositories and disk indexes are created here.
    #[arg(long, default_value = ".cache/2026-09-18/commit-snapshot")]
    workspace: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

#[derive(Serialize)]
struct Row {
    iteration: usize,
    step: String,
    tree: String,
    input_scan_ms: f64,
    snapshot_ms: f64,
    incremental_ms: f64,
    production_ms: f64,
    snapshot_work: Work,
    incremental_work: Work,
    snapshot_retained: Retained,
    incremental_retained: Retained,
    production_segments: usize,
    production_posting_bytes: u64,
    verified_queries: usize,
}

#[derive(Serialize)]
struct Report {
    schema: u32,
    source_revision: String,
    os: String,
    arch: String,
    files: usize,
    bytes_per_file: usize,
    changed_percent: usize,
    iterations: usize,
    notes: Vec<&'static str>,
    rows: Vec<Row>,
}

fn elapsed(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn source(file: usize, size: usize, version: &str) -> Blob {
    let mut content = format!("// SNAPSHOT_{version} file {file}\n");
    let mut state = (file as u64 + 1) ^ ngram::hash(version.as_bytes());
    let mut line = 0;
    while content.len() < size {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        content.push_str(&format!("pub fn handler_{file}_{line}() -> u64 {{ let value_{state:x} = {line}; value_{state:x} }}\n"));
        line += 1;
    }
    Blob::new(content.into_bytes())
}

fn commit(repo: &Repository, files: &Files, parent: Option<Oid>, message: &str) -> Result<Oid> {
    let mut builder = repo.treebuilder(None)?;
    for (path, blob) in files {
        let oid = repo.blob(&blob.bytes)?;
        builder.insert(path, oid, 0o100644)?;
    }
    let tree = repo.find_tree(builder.write()?)?;
    let signature = Signature::now("Snapshot demo", "demo@example.invalid")?;
    let parent = parent.map(|oid| repo.find_commit(oid)).transpose()?;
    let parents: Vec<_> = parent.iter().collect();
    Ok(repo.commit(None, &signature, &signature, message, &tree, &parents)?)
}

fn install(repo: &Repository, head: Oid, files: &Files) -> Result<()> {
    // Only this program's generated flat fixture is supported. Updating the
    // real Git index makes production's repository identity fully realistic.
    repo.set_head_detached(head)?;
    let tree = repo.find_commit(head)?.tree()?;
    let mut git_index = repo.index()?;
    git_index.read_tree(&tree)?;
    git_index.write()?;
    let root = repo.workdir().unwrap();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && !files.contains_key(&entry.file_name().to_string_lossy().into_owned())
        {
            fs::remove_file(entry.path())?;
        }
    }
    for (path, blob) in files {
        let path = root.join(path);
        if fs::read(&path).ok().as_deref() != Some(blob.bytes.as_ref()) {
            fs::write(path, &blob.bytes)?;
        }
    }
    Ok(())
}

fn inputs(repo: &Repository) -> Result<(Oid, Files, Files)> {
    let tree = repo.head()?.peel_to_commit()?.tree()?;
    let mut committed = Files::new();
    for entry in &tree {
        let blob = repo.find_blob(entry.id())?;
        committed.insert(
            entry.name().unwrap().to_owned(),
            Blob {
                oid: entry.id(),
                bytes: blob.content().into(),
            },
        );
    }
    let mut working = Files::new();
    for entry in fs::read_dir(repo.workdir().unwrap())? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            working.insert(
                entry.file_name().into_string().unwrap(),
                Blob::new(fs::read(entry.path())?),
            );
        }
    }
    Ok((tree.id(), committed, working))
}

const QUERIES: &[&str] = &[
    "SNAPSHOT_A",
    "SNAPSHOT_B",
    "SNAPSHOT_DIRTY",
    "SNAPSHOT_STAGED",
    "SNAPSHOT_UNSTAGED",
    "SNAPSHOT_NEW",
    "handler_0_",
    "handler_1_",
    "absent_token",
    "pub fn",
];

struct Runner {
    snapshot: SnapshotIndex,
    incremental: IncrementalIndex,
    disk: Option<index::DiskIndex>,
    index_dir: PathBuf,
}

impl Runner {
    fn step(
        &mut self,
        repo: &Repository,
        iteration: usize,
        name: &str,
        zero_extraction: bool,
    ) -> Result<Row> {
        let start = Instant::now();
        let (tree, committed, working) = inputs(repo)?;
        let input_scan_ms = elapsed(start);

        // Alternate order across repetitions to reduce consistent warm-cache bias.
        let mut snapshot_ms = 0.0;
        let mut incremental_ms = 0.0;
        let mut production_ms = 0.0;
        let mut snapshot_work = Work::default();
        let mut incremental_work = Work::default();
        let order = if iteration.is_multiple_of(2) {
            [0, 1, 2]
        } else {
            [2, 1, 0]
        };
        for variant in order {
            let start = Instant::now();
            match variant {
                0 => {
                    snapshot_work = self.snapshot.refresh(tree, &committed, &working);
                    snapshot_ms = elapsed(start);
                }
                1 => {
                    incremental_work = self.incremental.refresh(&working);
                    incremental_ms = elapsed(start);
                }
                _ => {
                    let root = repo.workdir().unwrap();
                    let budget = "256"
                        .parse::<build::MemoryBudget>()
                        .map_err(anyhow::Error::msg)?;
                    if let Some(disk) = &mut self.disk {
                        index::refresh(disk, Some(&self.index_dir), budget)?;
                    } else {
                        index::build(root, Some(&self.index_dir), budget)?;
                    }
                    // Include load: refresh publishes to disk but does not replace
                    // the caller's in-memory manifest/segment handles.
                    self.disk = Some(index::load(root, Some(&self.index_dir))?);
                    production_ms = elapsed(start);
                }
            }
        }
        if zero_extraction {
            ensure!(
                snapshot_work.extracted_files == 0,
                "{name} unexpectedly extracted files"
            );
        }
        let disk = self.disk.as_ref().unwrap();
        for query in QUERIES {
            let needle = query.as_bytes();
            let expected: Vec<_> = working
                .iter()
                .filter(|(_, blob)| contains(&blob.bytes, needle))
                .map(|(path, _)| path.clone())
                .collect();
            ensure!(
                self.snapshot.search(needle) == expected,
                "snapshot mismatch at {name}: {query}"
            );
            ensure!(
                self.incremental.search(needle) == expected,
                "incremental mismatch at {name}: {query}"
            );
            let ids = disk.postings(ngram::covering_hashes(needle, 1)[0])?;
            let mut actual = Vec::new();
            for id in ids {
                let path = disk.document_path(id)?.to_string_lossy().into_owned();
                let blob = working
                    .get(&path)
                    .ok_or_else(|| anyhow::anyhow!("stale disk candidate: {path}"))?;
                if contains(&blob.bytes, needle) {
                    actual.push(path);
                }
            }
            actual.sort();
            ensure!(actual == expected, "production mismatch at {name}: {query}");
        }
        let manifest = disk.full_manifest()?;
        let production_posting_bytes = manifest
            .segments
            .iter()
            .map(|segment| compaction::bytes(&self.index_dir, segment))
            .sum::<Result<u64>>()?;
        eprintln!(
            "{iteration} {name}: snapshot={snapshot_ms:.2}ms incremental={incremental_ms:.2}ms disk={production_ms:.2}ms extracted={}/{}",
            snapshot_work.extracted_files, incremental_work.extracted_files
        );
        Ok(Row {
            iteration,
            step: name.into(),
            tree: tree.to_string(),
            input_scan_ms,
            snapshot_ms,
            incremental_ms,
            production_ms,
            snapshot_work,
            incremental_work,
            snapshot_retained: self.snapshot.retained(),
            incremental_retained: self.incremental.retained(),
            production_segments: manifest.segments.len(),
            production_posting_bytes,
            verified_queries: QUERIES.len() * 3,
        })
    }
}

fn run(args: &Args) -> Result<Report> {
    ensure!(
        args.files >= 4 && args.bytes_per_file >= 128 && args.iterations > 0,
        "need >=4 files, >=128 bytes/file and >=1 iteration"
    );
    ensure!(
        (1..=100).contains(&args.changed_percent),
        "changed-percent must be 1..100"
    );
    fs::create_dir_all(&args.workspace)?;
    let workspace = args.workspace.canonicalize()?;
    let a: Files = (0..args.files)
        .map(|id| {
            (
                format!("file_{id:06}.rs"),
                source(id, args.bytes_per_file, "A"),
            )
        })
        .collect();
    let changed = (args.files * args.changed_percent / 100)
        .max(2)
        .min(args.files);
    let mut b = a.clone();
    for id in 0..changed {
        b.insert(
            format!("file_{id:06}.rs"),
            source(id, args.bytes_per_file, "B"),
        );
    }
    b.remove("file_000001.rs");
    b.insert(
        "new.rs".into(),
        source(args.files, args.bytes_per_file, "NEW"),
    );
    let mut dirty = b.clone();
    for id in 0..changed {
        if id != 1 {
            dirty.insert(
                format!("file_{id:06}.rs"),
                source(id, args.bytes_per_file, "DIRTY"),
            );
        }
    }
    dirty.remove("new.rs");
    dirty.insert(
        "untracked.rs".into(),
        source(args.files + 1, args.bytes_per_file, "DIRTY"),
    );
    let mut partial = b.clone();
    partial.insert(
        "file_000000.rs".into(),
        source(0, args.bytes_per_file, "STAGED"),
    );
    let mut unstaged = partial.clone();
    unstaged.insert(
        "file_000000.rs".into(),
        source(0, args.bytes_per_file, "UNSTAGED"),
    );
    unstaged.insert(
        "untracked.rs".into(),
        source(args.files + 1, args.bytes_per_file, "DIRTY"),
    );
    let mut rows = Vec::new();
    for iteration in 0..args.iterations {
        let temp = tempfile::Builder::new()
            .prefix("snapshot-")
            .tempdir_in(&workspace)?;
        let repo = Repository::init(temp.path().join("repo"))?;
        let ca = commit(&repo, &a, None, "A: baseline")?;
        let cb = commit(&repo, &b, Some(ca), "B: large agent commit")?;
        let cp = commit(&repo, &partial, Some(cb), "C: partial commit")?;
        let mut runner = Runner {
            snapshot: SnapshotIndex::default(),
            incremental: IncrementalIndex::default(),
            disk: None,
            index_dir: temp.path().join("disk-index"),
        };
        for (name, head, working, zero) in [
            ("cold_A", ca, &a, false),
            ("large_edit", ca, &b, false),
            ("commit_B", cb, &b, true),
            ("rollback_A", ca, &a, true),
            ("revisit_B", cb, &b, true),
            ("dirty_B", cb, &dirty, false),
            ("discard_dirty", cb, &b, true),
            ("partial_edit", cb, &unstaged, false),
            ("partial_commit", cp, &unstaged, false),
            ("discard_partial", cp, &partial, true),
            ("rollback_A_again", ca, &a, true),
            ("revisit_B_again", cb, &b, true),
        ] {
            install(&repo, head, working)?;
            rows.push(runner.step(&repo, iteration, name, zero)?);
        }
    }
    Ok(Report {
        schema: 1,
        source_revision: Repository::discover(".")?
            .head()?
            .peel_to_commit()?
            .id()
            .to_string(),
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        files: args.files,
        bytes_per_file: args.bytes_per_file,
        changed_percent: args.changed_percent,
        iterations: args.iterations,
        notes: vec![
            "Synthetic Rust-like text in real disposable Git repositories; setup/checkout excluded from timings.",
            "input_scan_ms reads HEAD blobs and all worktree bytes and computes blob IDs; shared by both RAM variants.",
            "RAM timings exclude input_scan_ms; production_ms includes its own metadata scan, extraction, disk publication and load.",
            "Production is the actual index::build/refresh/load implementation; RAM incremental is a separate policy control.",
            "All caches are warm at OS level; iterations recreate all indexes, variant order alternates; no warmup excluded.",
            "Memory counts are retained logical payload, not RSS; source bytes, hash-map overhead and snapshot metadata add costs.",
            "Prototype has no persistence, eviction, compaction, ignore/filter handling, nested trees or concurrent mutation support.",
        ],
        rows,
    })
}

fn main() -> Result<()> {
    let args = Args::parse();
    let report = run(&args)?;
    if let Some(parent) = args.output.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.output, serde_json::to_vec_pretty(&report)?)?;
    eprintln!("report: {}", args.output.display());
    Ok(())
}
