//! Stateful CLI benchmark on disposable clones of a real Git repository.
#[allow(dead_code)]
#[path = "../src/manifest.rs"]
mod manifest_format;

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde::Serialize;
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Instant,
};

const MARKER: &str = "CODERG_COMMIT_BENCH";
const QUERIES: &[&str] = &["AsyncMock", "SamplingParams", MARKER];

#[derive(Parser)]
struct Args {
    /// Source repository; all mutations happen in disposable local clones.
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    baseline: PathBuf,
    #[arg(long)]
    binary: PathBuf,
    #[arg(long, default_value_t = 100)]
    commits: usize,
    #[arg(long, default_value_t = 10)]
    files_per_commit: usize,
    #[arg(long, default_value_t = 100)]
    lines_per_file: usize,
    #[arg(long, default_value_t = 7)]
    iterations: usize,
    #[arg(long, default_value_t = 3)]
    warmup: usize,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    keep: bool,
    /// Parent directory for disposable benchmark files.
    #[arg(long)]
    workspace: PathBuf,
    #[arg(long, hide = true)]
    bench: bool,
}

#[derive(Serialize)]
struct Report {
    source: PathBuf,
    source_commit: String,
    baseline: PathBuf,
    binary: PathBuf,
    os: String,
    arch: String,
    commits: usize,
    files_per_commit: usize,
    lines_per_file: usize,
    iterations: usize,
    warmup: usize,
    workspace: PathBuf,
    scenarios: Vec<Scenario>,
}

#[derive(Serialize)]
struct Scenario {
    name: String,
    eligible_files: usize,
    selected_files: Vec<PathBuf>,
    builds: Vec<Measurement>,
    updates: Vec<Step>,
    checkpoints: Vec<QueryTiming>,
    rollbacks: Vec<Step>,
    compactions: Vec<Measurement>,
}

#[derive(Serialize)]
struct Step {
    step: usize,
    commit: String,
    changed_source_bytes: u64,
    changed_files: usize,
    measurements: Vec<Measurement>,
}

#[derive(Serialize)]
struct Measurement {
    version: String,
    wall_ms: f64,
    peak_rss_bytes: Option<u64>,
    diagnostic: String,
    base_id: u64,
    segments: usize,
    active_bytes: u64,
    middle_bytes: u64,
    manifest_bytes: u64,
    retained_bytes: u64,
}

#[derive(Serialize)]
struct QueryTiming {
    step: usize,
    phase: String,
    version: String,
    query: String,
    no_refresh: bool,
    samples_ms: Vec<f64>,
    median_ms: f64,
}

struct Version<'a> {
    name: &'static str,
    binary: &'a Path,
    index: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = Args::parse();
    if args.output.exists() {
        bail!("report already exists: {}", args.output.display());
    }
    if args.commits == 0
        || args.files_per_commit == 0
        || args.lines_per_file == 0
        || args.iterations == 0
    {
        bail!("commits, files per commit, lines per file and iterations must be positive");
    }
    args.root = args.root.canonicalize()?;
    args.baseline = args.baseline.canonicalize()?;
    args.binary = args.binary.canonicalize()?;
    fs::create_dir_all(&args.workspace)?;
    let temporary = tempfile::Builder::new()
        .prefix("coderg-commit-updates-")
        .tempdir_in(&args.workspace)?;
    let workspace = temporary.path().to_path_buf();
    let mut report = Report {
        source_commit: git(&args.root, &["rev-parse", "HEAD"])?,
        source: args.root.clone(),
        baseline: args.baseline.clone(),
        binary: args.binary.clone(),
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        commits: args.commits,
        files_per_commit: args.files_per_commit,
        lines_per_file: args.lines_per_file,
        iterations: args.iterations,
        warmup: args.warmup,
        workspace: workspace.clone(),
        scenarios: Vec::new(),
    };
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.output)?;
    drop(file);
    for name in ["repeated", "rotating"] {
        report.scenarios.push(scenario(&args, &workspace, name)?);
        fs::write(&args.output, serde_json::to_vec_pretty(&report)?)?;
    }
    if args.keep {
        eprintln!("retained workspace: {}", temporary.keep().display());
    }
    eprintln!("report: {}", args.output.display());
    Ok(())
}

fn scenario(args: &Args, workspace: &Path, name: &str) -> Result<Scenario> {
    let directory = workspace.join(name);
    fs::create_dir(&directory)?;
    let root = directory.join("source");
    let mut clone = Command::new("git");
    clone
        .args(["clone", "--shared", "--quiet", "--"])
        .arg(&args.root)
        .arg(&root);
    checked(clone, false)?;
    git(&root, &["config", "user.name", "Coderg Benchmark"])?;
    git(&root, &["config", "user.email", "coderg@example.invalid"])?;
    git(&root, &["config", "commit.gpgsign", "false"])?;
    git(&root, &["config", "core.hooksPath", "/dev/null"])?;
    let versions = [
        Version {
            name: "before",
            binary: &args.baseline,
            index: directory.join("before-index"),
        },
        Version {
            name: "after",
            binary: &args.binary,
            index: directory.join("after-index"),
        },
    ];
    let tracked = git(&root, &["ls-files", "-z"])?;
    let mut files = Vec::new();
    for path in tracked
        .split('\0')
        .filter(|path| path.starts_with("vllm/") && path.ends_with(".py"))
    {
        let metadata = fs::symlink_metadata(root.join(path))?;
        if metadata.is_file() && (8 * 1024..=128 * 1024).contains(&metadata.len()) {
            files.push(PathBuf::from(path));
        }
    }
    files.sort_by_cached_key(|path| {
        path.as_os_str()
            .as_encoded_bytes()
            .iter()
            .fold(0xcbf29ce484222325_u64, |hash, byte| {
                (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
            })
    });
    if files.len() < args.files_per_commit {
        bail!("not enough 8–128 KiB vllm Python files");
    }
    let mut result = Scenario {
        name: name.to_owned(),
        eligible_files: files.len(),
        selected_files: Vec::new(),
        builds: Vec::new(),
        updates: Vec::new(),
        checkpoints: Vec::new(),
        rollbacks: Vec::new(),
        compactions: Vec::new(),
    };
    for version in &versions {
        result
            .builds
            .push(measure(command(version, &root, &["index"]), version, true)?);
    }
    verify(&versions, &root)?;
    checkpoint(
        args,
        &versions,
        &root,
        0,
        &mut result.checkpoints,
        "updates",
    )?;
    let mut commits = vec![git(&root, &["rev-parse", "HEAD"])?];
    let mut selected = BTreeSet::new();
    for step in 1..=args.commits {
        let start = if name == "repeated" {
            0
        } else {
            (step - 1) * args.files_per_commit
        };
        let mut changed_bytes = 0;
        let mut add = Command::new("git");
        add.current_dir(&root).args(["add", "--"]);
        for offset in 0..args.files_per_commit {
            let path = &files[(start + offset) % files.len()];
            let full = root.join(path);
            let mut file = OpenOptions::new().append(true).open(&full)?;
            // Valid comments, with exactly 100 newly added lines per file by default.
            // Include a leading newline so files lacking a final newline remain valid.
            write!(
                file,
                "\n# {MARKER} step={step:03} file={offset:02} line=000"
            )?;
            for line in 1..args.lines_per_file {
                write!(
                    file,
                    "\n# {MARKER} step={step:03} file={offset:02} line={line:03}"
                )?;
            }
            file.flush()?;
            changed_bytes += fs::metadata(&full)?.len();
            selected.insert(path.clone());
            add.arg(path);
        }
        checked(add, false)?;
        git(
            &root,
            &["commit", "--quiet", "-m", &format!("benchmark {step}")],
        )?;
        let commit = git(&root, &["rev-parse", "HEAD"])?;
        commits.push(commit.clone());
        let measurements = update_pair(&versions, &root, step)?;
        verify(&versions, &root)?;
        result.updates.push(Step {
            step,
            commit,
            changed_source_bytes: changed_bytes,
            changed_files: args.files_per_commit,
            measurements,
        });
        if [1, 8, 16, 32, 64].contains(&step) || step == args.commits {
            checkpoint(
                args,
                &versions,
                &root,
                step,
                &mut result.checkpoints,
                "updates",
            )?;
        }
        if step % 10 == 0 || step == args.commits {
            let current = result.updates.last().unwrap();
            eprintln!(
                "{name} commit {step}/{}: {}",
                args.commits,
                current
                    .measurements
                    .iter()
                    .map(|sample| format!(
                        "{} {:.2} ms / {} segments",
                        sample.version, sample.wall_ms, sample.segments
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    result.selected_files = selected.into_iter().collect();
    for (order, target) in [
        args.commits.saturating_sub(1),
        args.commits / 2,
        0,
        args.commits,
    ]
    .into_iter()
    .enumerate()
    {
        let paths = git(
            &root,
            &["diff", "--name-only", "-z", "HEAD", &commits[target]],
        )?;
        let paths: Vec<_> = paths.split('\0').filter(|path| !path.is_empty()).collect();
        git(
            &root,
            &["checkout", "--quiet", "--detach", &commits[target]],
        )?;
        let changed_source_bytes = paths
            .iter()
            .map(|path| fs::metadata(root.join(path)).map(|metadata| metadata.len()))
            .collect::<std::io::Result<Vec<_>>>()?
            .iter()
            .sum();
        let measurements = update_pair(&versions, &root, order)?;
        verify(&versions, &root)?;
        result.rollbacks.push(Step {
            step: target,
            commit: commits[target].clone(),
            changed_source_bytes,
            changed_files: paths.len(),
            measurements,
        });
    }
    result.compactions.push(measure(
        command(&versions[1], &root, &["compact"]),
        &versions[1],
        true,
    )?);
    verify(&versions, &root)?;
    checkpoint(
        args,
        &versions,
        &root,
        args.commits,
        &mut result.checkpoints,
        "after_middle_compaction",
    )?;
    Ok(result)
}

fn update_pair(versions: &[Version<'_>; 2], root: &Path, step: usize) -> Result<Vec<Measurement>> {
    let mut results = Vec::new();
    for offset in 0..2 {
        let version = &versions[(step + offset) % 2];
        results.push(measure(
            command(version, root, &["search", "-F", "-l", MARKER]),
            version,
            true,
        )?);
    }
    results.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(results)
}

fn command(version: &Version<'_>, root: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(version.binary);
    command
        .args(args)
        .arg(root)
        .arg("--index-dir")
        .arg(&version.index);
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

fn measure(command: Command, version: &Version<'_>, rss: bool) -> Result<Measurement> {
    let search = command.get_args().next().is_some_and(|arg| arg == "search");
    let mut measured = command;
    if rss && cfg!(target_os = "macos") {
        let mut wrapper = Command::new("/usr/bin/time");
        wrapper
            .arg("-l")
            .arg(measured.get_program())
            .args(measured.get_args());
        measured = wrapper;
    }
    let started = Instant::now();
    let output = checked(measured, search)?;
    let wall_ms = started.elapsed().as_secs_f64() * 1000.0;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let peak_rss_bytes = stderr
        .lines()
        .find(|line| line.contains("maximum resident set size"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|value| value.parse().ok());
    let manifest = manifest_format::read(&version.index.join(manifest_format::FILE_NAME))?;
    let mut active_bytes = 0;
    let mut middle_bytes = 0;
    for (position, segment) in manifest.segments.iter().enumerate() {
        let bytes = fs::metadata(version.index.join(&segment.lookup))?.len()
            + fs::metadata(version.index.join(&segment.postings))?.len();
        active_bytes += bytes;
        if position > 0 {
            middle_bytes += bytes;
        }
    }
    let manifest_bytes = fs::metadata(version.index.join(manifest_format::FILE_NAME))?.len();
    Ok(Measurement {
        version: version.name.to_owned(),
        wall_ms,
        peak_rss_bytes,
        diagnostic: stderr
            .lines()
            .filter(|line| line.starts_with("coderg:"))
            .collect::<Vec<_>>()
            .join("\n"),
        base_id: manifest.segments[0].id,
        segments: manifest.segments.len(),
        active_bytes: active_bytes + manifest_bytes,
        middle_bytes,
        manifest_bytes,
        retained_bytes: directory_bytes(&version.index)?,
    })
}

fn directory_bytes(path: &Path) -> Result<u64> {
    let mut bytes = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            bytes += directory_bytes(&entry.path())?;
        } else {
            bytes += entry.metadata()?.len();
        }
    }
    Ok(bytes)
}

fn normalized(output: &Output) -> BTreeSet<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim_start_matches("./").to_owned())
        .collect()
}

fn verify(versions: &[Version<'_>; 2], root: &Path) -> Result<()> {
    for query in QUERIES {
        let mut rg = Command::new("rg");
        rg.current_dir(root)
            .args(["--hidden", "-g", "!.git", "-F", "-l", query, "."]);
        let expected = checked(rg, true)?;
        for version in versions {
            let actual = checked(
                command(
                    version,
                    root,
                    &["search", "--no-refresh", "-F", "-l", query],
                ),
                true,
            )?;
            if actual.status.code() != expected.status.code()
                || normalized(&actual) != normalized(&expected)
            {
                bail!("{} output mismatch against rg for {query}", version.name);
            }
        }
    }
    Ok(())
}

fn checkpoint(
    args: &Args,
    versions: &[Version<'_>; 2],
    root: &Path,
    step: usize,
    result: &mut Vec<QueryTiming>,
    phase: &str,
) -> Result<()> {
    for query in QUERIES {
        for no_refresh in [true, false] {
            let mut samples = [Vec::new(), Vec::new()];
            for iteration in 0..args.warmup + args.iterations {
                for offset in 0..2 {
                    let i = (iteration + offset + step) % 2;
                    let mut options = vec!["search", "-F", "-l", query];
                    if no_refresh {
                        options.push("--no-refresh");
                    }
                    let start = Instant::now();
                    checked(command(&versions[i], root, &options), true)?;
                    if iteration >= args.warmup {
                        samples[i].push(start.elapsed().as_secs_f64() * 1000.0);
                    }
                }
            }
            for (i, values) in samples.into_iter().enumerate() {
                let mut sorted = values.clone();
                sorted.sort_by(f64::total_cmp);
                result.push(QueryTiming {
                    step,
                    phase: phase.to_owned(),
                    version: versions[i].name.to_owned(),
                    query: (*query).to_owned(),
                    no_refresh,
                    median_ms: sorted[sorted.len() / 2],
                    samples_ms: values,
                });
            }
        }
    }
    Ok(())
}
