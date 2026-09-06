mod git_state;
mod index;
mod ngram;
mod query;
mod search;
mod segment;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    /// Build or replace the on-disk search index.
    Index {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_name = "DIR")]
        index_dir: Option<PathBuf>,
    },
    /// Search a source tree, rebuilding a stale index by default.
    Search {
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
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("coderg: {error:#}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Index { path, index_dir } => {
            let summary = index::build(&path, index_dir.as_deref())?;
            eprintln!(
                "indexed {} files ({} bytes, {} n-grams) in {}",
                summary.files,
                summary.bytes,
                summary.ngrams,
                summary.index_dir.display()
            );
        }
        Command::Search {
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
            let found = search::run(&path, index_dir.as_deref(), &pattern, &options)?;
            if !found {
                std::process::exit(1);
            }
        }
        Command::Stats { path, index_dir } => {
            let stats = index::stats(&path, index_dir.as_deref())?;
            println!("root: {}", stats.root.display());
            println!("files: {}", stats.files);
            println!("source bytes: {}", stats.source_bytes);
            println!("n-grams: {}", stats.ngrams);
            println!("segments: {}", stats.segments);
            println!("index bytes: {}", stats.index_bytes);
            println!(
                "git tree: {}",
                stats.git_tree.as_deref().unwrap_or("not a Git worktree")
            );
        }
    }
    Ok(())
}
