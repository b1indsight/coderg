mod build;
mod compaction;
mod git_state;
mod index;
mod manifest;
mod ngram;
mod query;
mod search;
mod segment;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args)]
struct BuildOptions {
    /// Working memory budget for index construction; excess records spill to disk.
    #[arg(long, value_name = "MiB", default_value = "256")]
    build_memory_mib: build::MemoryBudget,
}

#[derive(Debug, Parser)]
#[command(
    name = "coderg",
    version,
    about = "Fast indexed regex search for source trees"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Merge existing index segments without reading source contents.
    Compact {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_name = "DIR")]
        index_dir: Option<PathBuf>,
        /// Show the selected inputs without writing the index.
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        json: bool,
    },
    /// Build or replace the on-disk search index.
    Index {
        #[command(flatten)]
        build: BuildOptions,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_name = "DIR")]
        index_dir: Option<PathBuf>,
    },
    /// Search a source tree, rebuilding a stale index by default.
    Search {
        #[command(flatten)]
        build: BuildOptions,
        pattern: String,
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'i', long)]
        ignore_case: bool,
        #[arg(short = 'F', long)]
        fixed_strings: bool,
        #[arg(short = 'l', long)]
        files_with_matches: bool,
        #[arg(short = 'c', long, conflicts_with = "files_with_matches")]
        count: bool,
        #[arg(short = 'm', long, value_name = "NUM")]
        max_count: Option<usize>,
        #[arg(long, value_name = "DIR")]
        index_dir: Option<PathBuf>,
        /// Use the existing index without checking the source tree.
        #[arg(long)]
        no_refresh: bool,
    },
    /// Show statistics for an existing index.
    Stats {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_name = "DIR")]
        index_dir: Option<PathBuf>,
        /// Export the complete manifest as JSON for inspection and tooling.
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("coderg: {error:#}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut pool = rayon::ThreadPoolBuilder::new();
    if std::env::var_os("RAYON_NUM_THREADS").is_none() {
        pool = pool.num_threads(4);
    }
    pool.build_global()?;
    match cli.command {
        Command::Index {
            path,
            index_dir,
            build,
        } => {
            let summary = index::build(&path, index_dir.as_deref(), build.build_memory_mib)?;
            eprintln!(
                "indexed {} files ({} bytes, {} n-grams) in {}",
                summary.files,
                summary.bytes,
                summary.ngrams,
                summary.index_dir.display()
            );
        }
        Command::Search {
            build,
            pattern,
            path,
            ignore_case,
            fixed_strings,
            files_with_matches,
            count,
            max_count,
            index_dir,
            no_refresh,
        } => {
            let options = search::Options {
                ignore_case,
                fixed_strings,
                files_with_matches,
                count,
                max_count,
                no_refresh,
            };
            let found = search::run(
                &path,
                index_dir.as_deref(),
                &pattern,
                &options,
                build.build_memory_mib,
            )?;
            if !found {
                std::process::exit(1);
            }
        }
        Command::Compact {
            path,
            index_dir,
            dry_run,
            json,
        } => {
            let summary = index::compact(&path, index_dir.as_deref(), dry_run)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else if summary.inputs.is_empty() {
                println!("no compaction needed");
            } else {
                println!(
                    "{} {} {} segments ({} input bytes, {} output bytes)",
                    if dry_run {
                        "would compact"
                    } else {
                        "compacted"
                    },
                    summary.inputs.len(),
                    if summary.full {
                        "base + middle"
                    } else {
                        "middle"
                    },
                    summary.input_bytes,
                    summary.output_bytes
                );
            }
        }
        Command::Stats {
            path,
            index_dir,
            json,
        } => {
            if json {
                let index = index::load(&path, index_dir.as_deref())?;
                println!("{}", serde_json::to_string_pretty(&index.manifest)?);
                return Ok(());
            }
            let stats = index::stats(&path, index_dir.as_deref())?;
            println!("root: {}", stats.root.display());
            println!("files: {}", stats.files);
            println!("source bytes: {}", stats.source_bytes);
            println!("n-grams: {}", stats.ngrams);
            println!("segments: {}", stats.segments);
            println!("middle segments: {}", stats.middle_segments);
            println!("middle bytes: {}", stats.middle_bytes);
            println!("base bytes: {}", stats.base_bytes);
            println!("generational: {}", stats.generational);
            println!(
                "automatic base compaction threshold bytes: {}",
                stats.full_compaction_threshold_bytes
            );
            println!("maintenance due: {}", stats.maintenance_due);
            println!("index bytes: {}", stats.index_bytes);
            println!(
                "git tree: {}",
                stats.git_tree.as_deref().unwrap_or("not a Git worktree")
            );
        }
    }
    Ok(())
}
