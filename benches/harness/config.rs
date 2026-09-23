//! Serializable workload definitions. Paths are relative to the package root.
use anyhow::{Result, bail, ensure};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub name: String,
    pub seed: u64,
    pub warmup: usize,
    pub samples: usize,
    pub rounds: usize,
    pub history_rounds: usize,
    pub rss_runs: usize,
    pub scenarios: Vec<Scenario>,
    pub corpora: Vec<Corpus>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Scenario {
    Build,
    Search,
    Workflow,
    Branches,
    History,
    Manifest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub name: String,
    pub source: Source,
    pub updates: usize,
    pub extension: String,
    pub queries: Vec<Query>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Source {
    Git { path: PathBuf, commit: String },
    Archive { path: PathBuf, revision: String },
    Generated { files: usize, bytes_per_file: usize },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub name: String,
    pub pattern: String,
    pub flags: Vec<String>,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        ensure!(safe_name(&self.name), "invalid suite name");
        ensure!(
            self.scenarios
                .iter()
                .enumerate()
                .all(|(i, s)| !self.scenarios[..i].contains(s)),
            "duplicate scenario"
        );
        ensure!(
            self.samples > 0 && self.rounds > 0 && self.history_rounds > 0,
            "sample and round counts must be positive"
        );
        ensure!(
            !self.scenarios.is_empty() && !self.corpora.is_empty(),
            "empty suite"
        );
        let mut names = BTreeSet::new();
        for corpus in &self.corpora {
            ensure!(
                safe_name(&corpus.name) && names.insert(&corpus.name),
                "invalid or duplicate corpus name: {}",
                corpus.name
            );
            ensure!(
                corpus.updates > 0 && !corpus.queries.is_empty(),
                "{}: empty history or queries",
                corpus.name
            );
            ensure!(
                corpus.extension.starts_with('.') && !corpus.extension.contains('/'),
                "invalid extension"
            );
            let mut queries = BTreeSet::new();
            for q in &corpus.queries {
                ensure!(
                    safe_name(&q.name) && queries.insert(&q.name),
                    "invalid or duplicate query name"
                );
                ensure!(
                    !(q.flags.iter().any(|s| s == "-l") && q.flags.iter().any(|s| s == "-c")),
                    "conflicting query output modes"
                );
                for flag in &q.flags {
                    ensure!(
                        ["-F", "-i", "-l", "-c"].contains(&flag.as_str()),
                        "unsupported query flag: {flag}"
                    );
                }
            }
            match &corpus.source {
                Source::Git { commit, .. } => ensure!(
                    commit.len() == 40 && commit.bytes().all(|b| b.is_ascii_hexdigit()),
                    "pin Git corpora to full commit IDs"
                ),
                Source::Generated {
                    files,
                    bytes_per_file,
                } if *files == 0 || *bytes_per_file < 128 => {
                    bail!("generated corpus must have files of at least 128 bytes")
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub fn safe_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
