//! Replay actual first-parent Git commits against one or more CLI binaries.
#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde::Serialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Instant,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    root: PathBuf,
    /// Repeat NAME=/absolute/path/to/coderg for each version in the comparison.
    #[arg(long, required = true)]
    variant: Vec<String>,
    #[arg(long, default_value = "HEAD")]
    end: String,
    #[arg(long, default_value_t = 100)]
    commits: usize,
    #[arg(long, default_value_t = 5)]
    iterations: usize,
    #[arg(long, default_value_t = 2)]
    warmup: usize,
    /// Fixed strings checked against rg and timed at each checkpoint.
    #[arg(long, default_values = ["AsyncMock", "SamplingParams", "class "])]
    query: Vec<String>,
    /// Fixed string used by the first search after each checkout.
    #[arg(long, default_value = "SamplingParams")]
    update_query: String,
    /// Parent directory for disposable clones and indexes (outside the source).
    #[arg(long)]
    workspace: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    keep: bool,
    #[arg(long, hide = true)]
    bench: bool,
}

#[derive(Serialize)]
struct Variant {
    name: String,
    binary: PathBuf,
    index: PathBuf,
}

#[derive(Serialize)]
struct Report {
    source: PathBuf,
    workspace: PathBuf,
    base_commit: String,
    end_commit: String,
    os: String,
    arch: String,
    variants: Vec<Variant>,
    builds: Vec<Measurement>,
    updates: Vec<Step>,
    rollbacks: Vec<Step>,
    queries: Vec<QueryTiming>,
    iterations: usize,
    warmup: usize,
    verification_queries: Vec<String>,
    update_query: String,
    verified_comparisons: usize,
}

#[derive(Serialize)]
struct Step {
    step: usize,
    from_commit: String,
    commit: String,
    subject: String,
    changed_files: usize,
    added_lines: u64,
    deleted_lines: u64,
    added_files: usize,
    deleted_files: usize,
    renamed_files: usize,
    target_source_bytes: u64,
    measurements: Vec<Measurement>,
    rg: Option<RgMeasurement>,
}

#[derive(Serialize)]
struct RgMeasurement {
    wall_ms: f64,
    peak_rss_bytes: Option<u64>,
}

#[derive(Serialize)]
struct Measurement {
    version: String,
    wall_ms: f64,
    peak_rss_bytes: Option<u64>,
    diagnostic: String,
    base_id: u64,
    base_bytes_before: Option<u64>,
    middle_bytes_before: Option<u64>,
    base_bytes: u64,
    middle_bytes: u64,
    segments: usize,
    manifest_bytes: u64,
    retained_bytes: u64,
}

#[derive(Serialize)]
struct QueryTiming {
    step: usize,
    version: String,
    query: String,
    no_refresh: bool,
    samples_ms: Vec<f64>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    if args.commits == 0 || args.iterations == 0 {
        bail!("commits and iterations must be positive");
    }
    if args.output.exists() {
        bail!("report exists: {}", args.output.display());
    }
    let source = args.root.canonicalize()?;
    let mut commits: Vec<_> = git(
        &source,
        &[
            "rev-list",
            "--first-parent",
            &format!("--max-count={}", args.commits + 1),
            &args.end,
        ],
    )?
    .lines()
    .map(str::to_owned)
    .collect();
    if commits.len() != args.commits + 1 {
        bail!(
            "need {} consecutive first-parent commits; found {}. Fetch more real history first",
            args.commits + 1,
            commits.len()
        );
    }
    commits.reverse();
    fs::create_dir_all(&args.workspace)?;
    let temporary = tempfile::Builder::new()
        .prefix("history-")
        .tempdir_in(&args.workspace)?;
    let workspace = temporary.path().canonicalize()?;
    let root = workspace.join("source");
    let mut clone = Command::new("git");
    clone
        .args(["clone", "--shared", "--no-checkout", "--quiet", "--"])
        .arg(&source)
        .arg(&root);
    checked(clone, false)?;
    git(&root, &["config", "core.hooksPath", "/dev/null"])?;
    checkout(&root, &commits[0])?;
    let mut names = BTreeSet::new();
    let mut variants = Vec::new();
    for value in &args.variant {
        let (name, path) = value.split_once('=').context("variant must be NAME=PATH")?;
        if name.is_empty()
            || !name
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == b'-')
            || !names.insert(name.to_owned())
        {
            bail!("variant names must be unique alphanumeric names (hyphens allowed)");
        }
        variants.push(Variant {
            name: name.to_owned(),
            binary: Path::new(path).canonicalize()?,
            index: workspace.join(format!("index-{name}")),
        });
    }
    let mut report = Report {
        source,
        workspace: workspace.clone(),
        base_commit: commits[0].clone(),
        end_commit: commits.last().unwrap().clone(),
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        variants,
        builds: Vec::new(),
        updates: Vec::new(),
        rollbacks: Vec::new(),
        queries: Vec::new(),
        iterations: args.iterations,
        warmup: args.warmup,
        verification_queries: args.query.clone(),
        update_query: args.update_query.clone(),
        verified_comparisons: 0,
    };
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.output)?;
    for variant in &report.variants {
        report
            .builds
            .push(measure(&root, variant, true, &args.update_query)?);
    }
    report.verified_comparisons += verify(&root, &report.variants, &args.query)?;
    checkpoint(&args, &root, &report.variants, 0, &mut report.queries)?;
    for step in 1..commits.len() {
        let mut record = describe_step(&root, step, &commits[step - 1], &commits[step])?;
        checkout(&root, &record.commit)?;
        record.target_source_bytes = changed_bytes(&root, &record.from_commit, &record.commit)?;
        let (measurements, rg) = update(&root, &report.variants, step, &args.update_query)?;
        record.measurements = measurements;
        record.rg = Some(rg);
        report.verified_comparisons += report.variants.len();
        report.verified_comparisons += verify(&root, &report.variants, &args.query)?;
        report.updates.push(record);
        if [1, 25, 50, 100, 200, 300, 400, 500].contains(&step) || step == args.commits {
            checkpoint(&args, &root, &report.variants, step, &mut report.queries)?;
        }
        if step % 25 == 0 || step == args.commits {
            let last = report.updates.last().unwrap();
            eprintln!(
                "real commit {step}/{} {}: {}; rg {:.1} ms",
                args.commits,
                &last.commit[..8],
                last.measurements
                    .iter()
                    .map(|m| format!(
                        "{} {:.1} ms / M {:.2} MiB / {} segments",
                        m.version,
                        m.wall_ms,
                        m.middle_bytes as f64 / 1048576.0,
                        m.segments
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
                last.rg.as_ref().unwrap().wall_ms
            );
            fs::write(&args.output, serde_json::to_vec_pretty(&report)?)?;
        }
    }
    let mut previous = args.commits;
    for (order, target) in [args.commits - 1, args.commits / 2, 0, args.commits]
        .into_iter()
        .enumerate()
    {
        let mut record = describe_step(&root, target, &commits[previous], &commits[target])?;
        checkout(&root, &record.commit)?;
        record.target_source_bytes = changed_bytes(&root, &record.from_commit, &record.commit)?;
        let (measurements, rg) = update(&root, &report.variants, order, &args.update_query)?;
        record.measurements = measurements;
        record.rg = Some(rg);
        report.verified_comparisons += report.variants.len();
        report.verified_comparisons += verify(&root, &report.variants, &args.query)?;
        report.rollbacks.push(record);
        previous = target;
    }
    fs::write(&args.output, serde_json::to_vec_pretty(&report)?)?;
    if args.keep {
        eprintln!("retained workspace: {}", temporary.keep().display());
    }
    eprintln!(
        "verified {} comparisons; report: {}",
        report.verified_comparisons,
        args.output.display()
    );
    Ok(())
}

fn describe_step(root: &Path, step: usize, from: &str, to: &str) -> Result<Step> {
    let subject = git(root, &["show", "-s", "--format=%s", to])?;
    let status = git(root, &["diff", "--name-status", "--find-renames", from, to])?;
    let numstat = git(root, &["diff", "--numstat", from, to])?;
    let (mut added_lines, mut deleted_lines) = (0, 0);
    for line in numstat.lines() {
        let mut columns = line.split('\t');
        added_lines += columns.next().unwrap_or("").parse::<u64>().unwrap_or(0);
        deleted_lines += columns.next().unwrap_or("").parse::<u64>().unwrap_or(0);
    }
    Ok(Step {
        step,
        from_commit: from.to_owned(),
        commit: to.to_owned(),
        subject,
        changed_files: status.lines().count(),
        added_lines,
        deleted_lines,
        added_files: status
            .lines()
            .filter(|line| line.starts_with("A\t"))
            .count(),
        deleted_files: status
            .lines()
            .filter(|line| line.starts_with("D\t"))
            .count(),
        renamed_files: status.lines().filter(|line| line.starts_with('R')).count(),
        target_source_bytes: 0,
        measurements: Vec::new(),
        rg: None,
    })
}

fn changed_bytes(root: &Path, from: &str, to: &str) -> Result<u64> {
    let names = git(root, &["diff", "--name-only", "-z", from, to])?;
    let mut total = 0;
    for name in names.split('\0').filter(|name| !name.is_empty()) {
        match fs::symlink_metadata(root.join(name)) {
            Ok(metadata) if metadata.is_file() => total += metadata.len(),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(total)
}

fn checkout(root: &Path, commit: &str) -> Result<()> {
    git(root, &["checkout", "--quiet", "--detach", commit])?;
    Ok(())
}

fn command(root: &Path, variant: &Variant, args: &[&str]) -> Command {
    let mut command = Command::new(&variant.binary);
    command
        .args(args)
        .arg(root)
        .arg("--index-dir")
        .arg(&variant.index);
    command
}

fn checked(mut command: Command, search: bool) -> Result<Output> {
    let output = command
        .output()
        .with_context(|| format!("cannot run {command:?}"))?;
    if !(output.status.success() || search && output.status.code() == Some(1)) {
        bail!("{command:?}: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(output)
}

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command.current_dir(root).args(args);
    Ok(String::from_utf8(checked(command, false)?.stdout)?
        .trim_end_matches('\n')
        .to_owned())
}

fn update(
    root: &Path,
    variants: &[Variant],
    step: usize,
    query: &str,
) -> Result<(Vec<Measurement>, RgMeasurement)> {
    let mut result = Vec::new();
    let mut rg = None;
    let mut rg_output = None;
    // Include rg in the rotation so each participant sometimes runs first.
    for offset in 0..=variants.len() {
        let index = (step + offset) % (variants.len() + 1);
        if index == variants.len() {
            let (output, wall_ms, peak_rss_bytes) = timed(rg_command(root, query), true)?;
            rg_output = Some(output);
            rg = Some(RgMeasurement {
                wall_ms,
                peak_rss_bytes,
            });
        } else {
            result.push(measure(root, &variants[index], false, query)?);
        }
    }
    // Verify the command that was actually timed, including its working directory.
    let expected = rg_output.context("timed rg output was not collected")?;
    let expected_files = normalized(&expected);
    for variant in variants {
        let actual = checked(
            command(
                root,
                variant,
                &["search", "--no-refresh", "-F", "-l", query],
            ),
            true,
        )?;
        if actual.status.code() != expected.status.code() || normalized(&actual) != expected_files {
            bail!(
                "timed rg output disagrees with {} for {query}",
                variant.name
            );
        }
    }
    Ok((result, rg.context("rg timing was not collected")?))
}

fn timed(mut measured: Command, allow_no_match: bool) -> Result<(Output, f64, Option<u64>)> {
    if cfg!(target_os = "macos") {
        let mut wrapper = Command::new("/usr/bin/time");
        wrapper
            .arg("-l")
            .arg(measured.get_program())
            .args(measured.get_args());
        if let Some(directory) = measured.get_current_dir() {
            wrapper.current_dir(directory);
        }
        for (key, value) in measured.get_envs() {
            if let Some(value) = value {
                wrapper.env(key, value);
            } else {
                wrapper.env_remove(key);
            }
        }
        measured = wrapper;
    }
    let start = Instant::now();
    let output = checked(measured, allow_no_match)?;
    let wall_ms = start.elapsed().as_secs_f64() * 1000.0;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let peak_rss_bytes = stderr
        .lines()
        .find(|line| line.contains("maximum resident set size"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|value| value.parse().ok());
    Ok((output, wall_ms, peak_rss_bytes))
}

fn generation_bytes(index: &Path, manifest: &manifest_format::Manifest) -> Result<(u64, u64)> {
    let (mut base_bytes, mut middle_bytes) = (0, 0);
    for (i, segment) in manifest.segments.iter().enumerate() {
        let bytes = fs::metadata(index.join(&segment.lookup))?.len()
            + fs::metadata(index.join(&segment.postings))?.len();
        if i == 0 {
            base_bytes = bytes;
        } else {
            middle_bytes += bytes;
        }
    }
    Ok((base_bytes, middle_bytes))
}

fn measure(root: &Path, variant: &Variant, build: bool, query: &str) -> Result<Measurement> {
    let (base_bytes_before, middle_bytes_before) = if build {
        (None, None)
    } else {
        let manifest = manifest_format::read(&variant.index.join(manifest_format::FILE_NAME))?;
        let (base, middle) = generation_bytes(&variant.index, &manifest)?;
        (Some(base), Some(middle))
    };
    let search_args = ["search", "-F", "-l", query];
    let (output, wall_ms, peak_rss_bytes) = timed(
        command(root, variant, if build { &["index"] } else { &search_args }),
        !build,
    )?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let manifest = manifest_format::read(&variant.index.join(manifest_format::FILE_NAME))?;
    let (base_bytes, middle_bytes) = generation_bytes(&variant.index, &manifest)?;
    Ok(Measurement {
        version: variant.name.clone(),
        wall_ms,
        peak_rss_bytes,
        diagnostic: stderr
            .lines()
            .filter(|line| line.starts_with("coderg:"))
            .collect::<Vec<_>>()
            .join("\n"),
        base_id: manifest.segments[0].id,
        base_bytes_before,
        middle_bytes_before,
        base_bytes,
        middle_bytes,
        segments: manifest.segments.len(),
        manifest_bytes: fs::metadata(variant.index.join(manifest_format::FILE_NAME))?.len(),
        retained_bytes: directory_bytes(&variant.index)?,
    })
}

fn directory_bytes(path: &Path) -> Result<u64> {
    let mut total = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            total += directory_bytes(&entry.path())?;
        } else {
            total += entry.metadata()?.len();
        }
    }
    Ok(total)
}

fn normalized(output: &Output) -> BTreeSet<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim_start_matches("./").to_owned())
        .collect()
}

fn rg_command(root: &Path, query: &str) -> Command {
    let mut command = Command::new("rg");
    command.current_dir(root).args([
        "--no-config",
        "--hidden",
        "-g",
        "!.git",
        "-F",
        "-l",
        query,
        ".",
    ]);
    command
}

fn verify(root: &Path, variants: &[Variant], queries: &[String]) -> Result<usize> {
    for query in queries {
        let expected = checked(rg_command(root, query), true)?;
        for variant in variants {
            let actual = checked(
                command(
                    root,
                    variant,
                    &["search", "--no-refresh", "-F", "-l", query],
                ),
                true,
            )?;
            if actual.status.code() != expected.status.code()
                || normalized(&actual) != normalized(&expected)
            {
                bail!(
                    "{} disagrees with rg for {query} at {}",
                    variant.name,
                    git(root, &["rev-parse", "HEAD"])?
                );
            }
        }
    }
    Ok(queries.len() * variants.len())
}

fn checkpoint(
    args: &Args,
    root: &Path,
    variants: &[Variant],
    step: usize,
    result: &mut Vec<QueryTiming>,
) -> Result<()> {
    for query in &args.query {
        for no_refresh in [true, false] {
            let mut samples = vec![Vec::new(); variants.len()];
            for iteration in 0..args.warmup + args.iterations {
                for offset in 0..variants.len() {
                    let index = (step + iteration + offset) % variants.len();
                    let mut options = vec!["search", "-F", "-l", query.as_str()];
                    if no_refresh {
                        options.push("--no-refresh");
                    }
                    let start = Instant::now();
                    checked(command(root, &variants[index], &options), true)?;
                    if iteration >= args.warmup {
                        samples[index].push(start.elapsed().as_secs_f64() * 1000.0);
                    }
                }
            }
            for (i, samples_ms) in samples.into_iter().enumerate() {
                result.push(QueryTiming {
                    step,
                    version: variants[i].name.clone(),
                    query: query.clone(),
                    no_refresh,
                    samples_ms,
                });
            }
        }
    }
    Ok(())
}
