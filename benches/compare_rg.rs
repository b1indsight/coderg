use std::{
    collections::BTreeSet,
    env,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde::Serialize;
use tempfile::TempDir;

#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest;

const DEFAULT_QUERY: &str = "CODERG_BENCHMARK_NEEDLE";

#[derive(Debug, Parser)]
#[command(about = "Compare coderg's end-to-end latency with ripgrep")]
struct Args {
    /// Benchmark an existing source tree instead of generating one.
    #[arg(long, requires = "query")]
    root: Option<PathBuf>,

    /// Fixed string to search for. Required with --root.
    #[arg(
        long,
        default_value_if("root", clap::builder::ArgPredicate::IsPresent, None)
    )]
    query: Option<String>,

    /// Number of files in the generated corpus.
    #[arg(long, default_value_t = 256)]
    files: usize,

    /// Approximate size of every generated file.
    #[arg(long, default_value_t = 16)]
    kib_per_file: usize,

    /// Timed iterations for each search command.
    #[arg(long, default_value_t = 30)]
    iterations: usize,

    /// Untimed warm-up iterations for each search command.
    #[arg(long, default_value_t = 3)]
    warmup: usize,

    /// Print machine-readable JSON instead of a Markdown table.
    #[arg(long)]
    json: bool,

    /// Compare matching lines instead of matching file paths.
    #[arg(long)]
    lines: bool,

    /// Save the JSON report to a new file after the run.
    #[arg(long)]
    output: Option<PathBuf>,

    /// Preserve generated corpus and index directories after the run.
    #[arg(long)]
    keep: bool,

    /// Accepted because Cargo can pass this flag to custom benchmark harnesses.
    #[arg(long, hide = true)]
    bench: bool,
}

#[derive(Debug, Serialize)]
struct Report {
    root: PathBuf,
    query: String,
    lines: bool,
    warmup: usize,
    files: usize,
    source_bytes: u64,
    index_bytes: u64,
    index_build_ms: f64,
    incremental_refresh_ms: Option<f64>,
    commit_promotion_ms: Option<f64>,
    cached_rollback_ms: Option<f64>,
    segments: usize,
    matches: usize,
    iterations: usize,
    coderg_no_refresh: Timing,
    coderg_with_refresh: Timing,
    ripgrep: Timing,
    speedup_no_refresh: f64,
    speedup_with_refresh: f64,
}

#[derive(Clone, Debug, Serialize)]
struct Timing {
    samples_ms: Vec<f64>,
    min_ms: f64,
    median_ms: f64,
    p95_ms: f64,
    mean_ms: f64,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("compare_rg: {error:#}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    if args.iterations == 0 {
        bail!("--iterations must be greater than zero");
    }
    if let Some(path) = &args.output {
        if path.exists() {
            bail!("report already exists: {}", path.display());
        }
    }
    if args.root.is_none() && (args.files == 0 || args.kib_per_file == 0) {
        bail!("--files and --kib-per-file must be greater than zero");
    }
    ensure_ripgrep()?;
    let coderg = ensure_coderg_binary()?;

    let generated = args.root.is_none().then(TempDir::new).transpose()?;
    let root = match (&args.root, &generated) {
        (Some(root), _) => root
            .canonicalize()
            .with_context(|| format!("cannot access {}", root.display()))?,
        (None, Some(directory)) => {
            generate_corpus(directory.path(), args.files, args.kib_per_file)?;
            initialize_git_corpus(directory.path())?;
            directory.path().canonicalize()?
        }
        (None, None) => unreachable!(),
    };
    let query = args.query.as_deref().unwrap_or(DEFAULT_QUERY);
    let index_temp = TempDir::new()?;
    let index_dir = index_temp.path().join("index");

    let started = Instant::now();
    checked_output(index_command(&coderg, &root, &index_dir), "coderg index")?;
    let index_build = started.elapsed();

    let coderg_result = checked_output(
        coderg_search_command(&coderg, &root, &index_dir, query, true, args.lines),
        "coderg search",
    )?;
    let rg_result = checked_output(rg_command(&root, query, args.lines), "rg")?;
    let coderg_matches = normalized_lines(&coderg_result.stdout);
    let rg_matches = normalized_lines(&rg_result.stdout);
    let refreshed_result = checked_output(
        coderg_search_command(&coderg, &root, &index_dir, query, false, args.lines),
        "coderg search with refresh",
    )?;
    if normalized_lines(&refreshed_result.stdout) != coderg_matches {
        bail!("result mismatch between coderg refresh modes");
    }
    if coderg_matches != rg_matches {
        bail!(
            "result mismatch: coderg found {} results, rg found {} results",
            coderg_matches.len(),
            rg_matches.len()
        );
    }

    let no_refresh = measure(args.warmup, args.iterations, || {
        coderg_search_command(&coderg, &root, &index_dir, query, true, args.lines)
    })?;
    let with_refresh = measure(args.warmup, args.iterations, || {
        coderg_search_command(&coderg, &root, &index_dir, query, false, args.lines)
    })?;
    let ripgrep = measure(args.warmup, args.iterations, || {
        rg_command(&root, query, args.lines)
    })?;
    let (incremental_refresh_ms, commit_promotion_ms, cached_rollback_ms) = if generated.is_some() {
        mutate_generated_file(&root)?;
        let started = Instant::now();
        run_quiet(coderg_search_command(
            &coderg, &root, &index_dir, query, false, args.lines,
        ))?;
        let incremental = milliseconds(started.elapsed());

        run_git(&root, &["add", "module_000/generated_000000.rs"])?;
        run_git(&root, &["commit", "-m", "benchmark update"])?;
        let started = Instant::now();
        run_quiet(coderg_search_command(
            &coderg, &root, &index_dir, query, false, args.lines,
        ))?;
        let promotion = milliseconds(started.elapsed());

        run_git(&root, &["reset", "--hard", "HEAD^"])?;
        let started = Instant::now();
        run_quiet(coderg_search_command(
            &coderg, &root, &index_dir, query, false, args.lines,
        ))?;
        let rollback = milliseconds(started.elapsed());
        (Some(incremental), Some(promotion), Some(rollback))
    } else {
        (None, None, None)
    };
    let (files, source_bytes, segments) = manifest_stats(&index_dir.join(manifest::FILE_NAME))?;
    let index_bytes = directory_bytes(&index_dir)?;

    let report = Report {
        root: root.clone(),
        query: query.to_owned(),
        lines: args.lines,
        warmup: args.warmup,
        files,
        source_bytes,
        index_bytes,
        index_build_ms: milliseconds(index_build),
        incremental_refresh_ms,
        commit_promotion_ms,
        cached_rollback_ms,
        segments,
        matches: coderg_matches.len(),
        iterations: args.iterations,
        speedup_no_refresh: ripgrep.median_ms / no_refresh.median_ms,
        speedup_with_refresh: ripgrep.median_ms / with_refresh.median_ms,
        coderg_no_refresh: no_refresh,
        coderg_with_refresh: with_refresh,
        ripgrep,
    };

    if let Some(path) = &args.output {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .with_context(|| format!("cannot create report {}", path.display()))?;
        serde_json::to_writer_pretty(&mut file, &report)?;
        writeln!(file)?;
    }
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_report(&report);
    }

    if args.keep {
        if let Some(directory) = generated {
            println!("generated corpus kept at {}", directory.keep().display());
        }
        println!("index kept at {}", index_temp.keep().display());
    }
    Ok(())
}

fn ensure_coderg_binary() -> Result<PathBuf> {
    if let Some(binary) = env::var_os("CODERG_BIN") {
        return Ok(PathBuf::from(binary));
    }
    let binary = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("release")
        .join(if cfg!(windows) {
            "coderg.exe"
        } else {
            "coderg"
        });
    let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["build", "--release", "--bin", "coderg"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("cannot run cargo to build the release binary")?;
    if !status.success() {
        bail!("cargo failed to build the release coderg binary");
    }
    Ok(binary)
}

fn ensure_ripgrep() -> Result<()> {
    let status = Command::new("rg")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("ripgrep (`rg`) is required for this benchmark")?;
    if !status.success() {
        bail!("`rg --version` failed");
    }
    Ok(())
}

fn generate_corpus(root: &Path, file_count: usize, kib_per_file: usize) -> Result<()> {
    let target_bytes = kib_per_file.saturating_mul(1024);
    for file_id in 0..file_count {
        let directory = root.join(format!("module_{:03}", file_id / 64));
        fs::create_dir_all(&directory)?;
        let path = directory.join(format!("generated_{file_id:06}.rs"));
        let mut file = BufWriter::new(File::create(path)?);
        let mut written = 0;
        let mut line = 0;
        while written < target_bytes {
            let marker = if file_id % 97 == 0 && line == 3 {
                DEFAULT_QUERY
            } else {
                "ordinary_value"
            };
            let text = format!(
                "pub fn generated_{file_id}_{line}(input: usize) -> usize {{ let {marker} = \"module_{}\"; input.wrapping_mul(31).wrapping_add({line}) }}\n",
                file_id % 41
            );
            file.write_all(text.as_bytes())?;
            written += text.len();
            line += 1;
        }
        file.flush()?;
    }
    Ok(())
}

fn mutate_generated_file(root: &Path) -> Result<()> {
    let path = root.join("module_000/generated_000000.rs");
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file, "// incremental benchmark update")?;
    file.flush()?;
    Ok(())
}

fn initialize_git_corpus(root: &Path) -> Result<()> {
    run_git(root, &["init", "--quiet"])?;
    run_git(root, &["config", "user.name", "Coderg Benchmark"])?;
    run_git(
        root,
        &["config", "user.email", "coderg-benchmark@example.invalid"],
    )?;
    run_git(root, &["add", "."])?;
    run_git(root, &["commit", "--quiet", "-m", "benchmark baseline"])
}

fn run_git(root: &Path, arguments: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .context("cannot run Git for generated benchmark corpus")?;
    if !output.status.success() {
        bail!(
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn index_command(binary: &Path, root: &Path, index_dir: &Path) -> Command {
    let mut command = Command::new(binary);
    command
        .args(["index"])
        .arg(root)
        .arg("--index-dir")
        .arg(index_dir);
    command
}

fn coderg_search_command(
    binary: &Path,
    root: &Path,
    index_dir: &Path,
    query: &str,
    no_refresh: bool,
    lines: bool,
) -> Command {
    let mut command = Command::new(binary);
    command
        .current_dir(root)
        .env("LC_ALL", "C")
        .args(["search", "--fixed-strings"])
        .arg("--index-dir")
        .arg(index_dir);
    if !lines {
        command.arg("--files-with-matches");
    }
    if no_refresh {
        command.arg("--no-refresh");
    }
    command.arg("--").arg(query).arg(".");
    command
}

fn rg_command(root: &Path, query: &str, lines: bool) -> Command {
    let mut command = Command::new("rg");
    command.current_dir(root).env("LC_ALL", "C").args([
        "--no-config",
        "--hidden",
        "--glob",
        "!.git/**",
        "--fixed-strings",
    ]);
    if lines {
        command.args(["-n", "--no-heading", "--color", "never"]);
    } else {
        command.arg("--files-with-matches");
    }
    command.arg("--").arg(query).arg(".");
    command
}

fn checked_output(mut command: Command, name: &str) -> Result<Output> {
    let output = command
        .output()
        .with_context(|| format!("cannot run {name}"))?;
    if !output.status.success() && !(name != "coderg index" && output.status.code() == Some(1)) {
        bail!(
            "{name} failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output)
}

fn measure<F>(warmup: usize, iterations: usize, mut command: F) -> Result<Timing>
where
    F: FnMut() -> Command,
{
    for _ in 0..warmup {
        run_quiet(command())?;
    }
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let started = Instant::now();
        run_quiet(command())?;
        samples.push(milliseconds(started.elapsed()));
    }
    let samples_ms = samples.clone();
    samples.sort_by(f64::total_cmp);
    let mean_ms = samples.iter().sum::<f64>() / samples.len() as f64;
    Ok(Timing {
        samples_ms,
        min_ms: samples[0],
        median_ms: percentile(&samples, 0.50),
        p95_ms: percentile(&samples, 0.95),
        mean_ms,
    })
}

fn run_quiet(mut command: Command) -> Result<()> {
    let status = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() && status.code() != Some(1) {
        bail!("timed command failed with {status}");
    }
    Ok(())
}

fn percentile(sorted: &[f64], percentile: f64) -> f64 {
    let index = ((sorted.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1);
    sorted[index]
}

fn milliseconds(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn normalized_lines(bytes: &[u8]) -> BTreeSet<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(|line| line.strip_prefix("./").unwrap_or(line).to_owned())
        .collect()
}

fn manifest_stats(path: &Path) -> Result<(usize, u64, usize)> {
    let manifest = manifest::read(path)?;
    let active: Vec<_> = manifest
        .documents
        .iter()
        .filter(|document| document.active && document.searchable)
        .collect();
    let bytes = active.iter().map(|document| document.len).sum();
    Ok((active.len(), bytes, manifest.segments.len()))
}

fn directory_bytes(path: &Path) -> Result<u64> {
    let mut bytes = 0;
    let mut directories = vec![path.to_path_buf()];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                directories.push(entry.path());
            } else {
                bytes += entry.metadata()?.len();
            }
        }
    }
    Ok(bytes)
}

fn print_report(report: &Report) {
    println!("dataset: {}", report.root.display());
    println!(
        "files: {}, source: {:.2} MiB, index: {:.2} MiB, segments: {}, matches: {}",
        report.files,
        report.source_bytes as f64 / 1_048_576.0,
        report.index_bytes as f64 / 1_048_576.0,
        report.segments,
        report.matches
    );
    println!("index build: {:.2} ms", report.index_build_ms);
    if let Some(elapsed) = report.incremental_refresh_ms {
        println!("single-file incremental refresh: {elapsed:.2} ms");
    }
    if let Some(elapsed) = report.commit_promotion_ms {
        println!("commit promotion (no reindex): {elapsed:.2} ms");
    }
    if let Some(elapsed) = report.cached_rollback_ms {
        println!("cached Git tree rollback: {elapsed:.2} ms");
    }
    println!();
    println!("| command | min | median | p95 | mean | speedup vs rg |");
    println!("|---|---:|---:|---:|---:|---:|");
    print_timing(
        "coderg --no-refresh",
        &report.coderg_no_refresh,
        report.speedup_no_refresh,
    );
    print_timing(
        "coderg (freshness check)",
        &report.coderg_with_refresh,
        report.speedup_with_refresh,
    );
    print_timing("rg", &report.ripgrep, 1.0);
}

fn print_timing(name: &str, timing: &Timing, speedup: f64) {
    println!(
        "| {name} | {:.3} ms | {:.3} ms | {:.3} ms | {:.3} ms | {:.2}x |",
        timing.min_ms, timing.median_ms, timing.p95_ms, timing.mean_ms, speedup
    );
}
