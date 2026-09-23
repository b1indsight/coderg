//! Unified process benchmarks. Run `cargo bench --bench suite -- --help`.
#[path = "harness/config.rs"]
mod config;
#[path = "harness/corpus.rs"]
mod corpus;
#[path = "harness/history.rs"]
mod history;
#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest;
#[path = "harness/process.rs"]
mod process;
#[path = "harness/report.rs"]
mod report;
#[path = "harness/search.rs"]
mod search;
#[path = "harness/session.rs"]
mod session;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use config::{Config, Scenario, Source};
use serde_json::json;
use session::{Session, Variant};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(
    about = "Full real-corpus benchmark suite; use --config benches/smoke.json for validation"
)]
struct Args {
    #[arg(long, default_value = "benches/full.json")]
    config: PathBuf,
    /// Optional corpus-name to local source-path mapping.
    #[arg(long)]
    sources: Option<PathBuf>,
    /// A new directory. Existing runs are never overwritten.
    #[arg(long)]
    output: Option<PathBuf>,
    /// Parent of disposable corpus clones and indexes.
    #[arg(long, default_value = ".cache/benchmark-workspaces")]
    workspace: PathBuf,
    /// Maximum concurrent corpora; use 1 for measurements without cross-corpus contention.
    #[arg(long, default_value_t = 2)]
    jobs: usize,
    /// Validate dependencies and inputs without preparing corpora or sampling.
    #[arg(long)]
    preflight: bool,
    /// Print the selected workload configuration without launching processes.
    #[arg(long)]
    list: bool,
    /// Recreate summary.json/report.md from an existing run.
    #[arg(long)]
    render: Option<PathBuf>,
    #[arg(long, hide = true)]
    bench: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("benchmark: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    ensure!(args.jobs > 0, "jobs must be positive");
    if let Some(path) = args.render {
        return report::render(&path);
    }
    let mut config: Config = serde_json::from_slice(
        &fs::read(&args.config).with_context(|| format!("read {}", args.config.display()))?,
    )?;
    let sources_path = args
        .sources
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new("benches/sources.local.json"));
    if args.sources.is_some() || sources_path.exists() {
        let sources: BTreeMap<String, PathBuf> = serde_json::from_slice(
            &fs::read(sources_path).with_context(|| format!("read {}", sources_path.display()))?,
        )?;
        for c in &mut config.corpora {
            if let Some(source) = sources.get(&c.name) {
                match &mut c.source {
                    Source::Git { path, .. } | Source::Archive { path, .. } => {
                        *path = source.clone()
                    }
                    Source::Generated { .. } => {}
                }
            }
        }
    }
    config.validate()?;
    if args.list {
        println!("{}", serde_json::to_string_pretty(&config)?);
        return Ok(());
    }
    let mut variants = [Variant {
        name: "current".into(),
        binary: PathBuf::from(env!("CARGO_BIN_EXE_coderg")).canonicalize()?,
        sha256: String::new(),
    }];
    let rg_version = process::text(Command::new("rg").arg("--version"))?;
    let git_version = process::text(Command::new("git").arg("--version"))?;
    for c in &config.corpora {
        corpus::preflight(c).with_context(|| {
            format!(
                "preflight {} (configure benches/sources.local.json)",
                c.name
            )
        })?;
    }
    for v in &mut variants {
        v.sha256 = process::hash_file(&v.binary)?;
        process::checked(Command::new(&v.binary).arg("--help"), false)?;
    }
    if config.rss_runs > 0 {
        process::rss(Command::new(&variants[0].binary).arg("--help"), false)
            .context("RSS preflight failed; run with permission to read OS resource counters")?;
    }
    if args.preflight {
        println!(
            "Preflight passed: {} corpora, {} coderg binaries + rg",
            config.corpora.len(),
            variants.len()
        );
        return Ok(());
    }
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("target/benchmarks/{}-{stamp}", config.name)));
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut report = report::Report::create(&output)?;
    let workspace_root = args.workspace;
    fs::create_dir_all(&workspace_root)?;
    let workspace = tempfile::Builder::new()
        .prefix("rust-benchmark-")
        .tempdir_in(&workspace_root)?;
    let mut sources = BTreeMap::new();
    for dir in ["src", "benches/harness"] {
        let mut paths = Vec::new();
        corpus::walk(
            std::path::Path::new(dir),
            std::path::Path::new(dir),
            &mut paths,
        )?;
        for p in paths {
            let p = PathBuf::from(dir).join(p);
            sources.insert(p.clone(), process::hash_file(&p)?);
        }
    }
    for p in [
        PathBuf::from("Cargo.toml"),
        PathBuf::from("Cargo.lock"),
        PathBuf::from("benches/suite.rs"),
        args.config,
    ] {
        sources.insert(p.clone(), process::hash_file(&p)?);
    }
    let mut metadata = json!({"schema_version": 1, "jobs": args.jobs, "status": "incomplete", "started_unix_ms": stamp, "config": config, "variants": variants, "source_sha256": sources, "source_branch": process::git(std::path::Path::new("."), &["rev-parse", "--abbrev-ref", "HEAD"])?, "source_commit": process::git(std::path::Path::new("."), &["rev-parse", "HEAD"])?, "git_status": process::git(std::path::Path::new("."), &["status", "--short"])?, "os": std::env::consts::OS, "arch": std::env::consts::ARCH, "logical_cpus": std::thread::available_parallelism()?.get(), "rustc": process::text(Command::new("rustc").arg("-Vv"))?, "rg": rg_version, "git": git_version, "cache": "warm filesystem cache; CLI process per sample", "timing": "CLI launch, output to null, exit; excludes Git operations, preparation and verification; RSS independent"});
    metadata["machine"] = process::machine_metadata();
    metadata["harness_debug_assertions"] = json!(cfg!(debug_assertions));
    fs::write(
        output.join("run.json"),
        serde_json::to_vec_pretty(&metadata)?,
    )?;
    let result = (|| -> Result<()> {
        let mut failures = Vec::new();
        // Reports and fixtures are owned per corpus; no writer or index is shared.
        for batch in config.corpora.chunks(args.jobs) {
            let results = std::thread::scope(|scope| {
                let handles: Vec<_> = batch
                    .iter()
                    .map(|corpus_config| {
                        let config = &config;
                        let variants = &variants;
                        let workspace = &workspace;
                        let corpus_output = output.join(&corpus_config.name);
                        scope.spawn(move || -> Result<()> {
                            let mut corpus_report = report::Report::create(&corpus_output)?;
                            eprintln!("Preparing {}", corpus_config.name);
                            let prepared = corpus::prepare(
                                corpus_config,
                                workspace.path(),
                                config.seed,
                                &mut corpus_report,
                            )?;
                            let mut session = Session {
                                config,
                                corpus: &prepared,
                                variants,
                                workspace: workspace.path().into(),
                                report: &mut corpus_report,
                                seed: config.seed,
                            };
                            for scenario in &config.scenarios {
                                eprintln!("{}: {scenario:?}", corpus_config.name);
                                match scenario {
                                    Scenario::Build => search::build(&mut session),
                                    Scenario::Search => search::search(&mut session),
                                    Scenario::Workflow => history::workflow(&mut session, false),
                                    Scenario::Branches => history::workflow(&mut session, true),
                                    Scenario::History => history::history(&mut session),
                                    Scenario::Manifest => search::manifest(&mut session),
                                }
                                .with_context(|| format!("{} {scenario:?}", corpus_config.name))?;
                            }
                            Ok(())
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|handle| {
                        handle
                            .join()
                            .unwrap_or_else(|_| Err(anyhow::anyhow!("corpus worker panicked")))
                    })
                    .collect::<Vec<_>>()
            });
            for (corpus_config, result) in batch.iter().zip(results) {
                report.merge(&output.join(&corpus_config.name))?;
                if let Err(error) = result {
                    failures.push(format!("{}: {error:#}", corpus_config.name));
                }
            }
        }
        ensure!(failures.is_empty(), "{}", failures.join("\n"));
        for v in &variants {
            ensure!(
                process::hash_file(&v.binary)? == v.sha256,
                "binary changed during benchmark: {}",
                v.name
            );
        }
        for (path, sha) in &sources {
            ensure!(
                process::hash_file(path)? == *sha,
                "source changed during benchmark: {}",
                path.display()
            );
        }
        Ok(())
    })();
    metadata["finished_unix_ms"] = json!(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis());
    if let Err(error) = &result {
        metadata["error"] = json!(format!("{error:#}"));
    } else {
        metadata["status"] = json!("complete");
    }
    fs::write(
        output.join("run.json"),
        serde_json::to_vec_pretty(&metadata)?,
    )?;
    report::render(&output)?;
    eprintln!("Report: {}", output.join("report.md").display());
    result
}
