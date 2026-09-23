//! Shared CLI construction, verification, and sample recording for every scenario.
use super::{
    config::{Config, Query},
    corpus::{self, Prepared},
    process::{self, checked, normalized},
    report::{Report, Sample},
};
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use serde_json::{Value, json};
use std::{path::PathBuf, process::Command};
use tempfile::TempDir;

#[derive(Clone, Serialize)]
pub struct Variant {
    pub name: String,
    pub binary: PathBuf,
    pub sha256: String,
}

pub struct Fixture {
    // Own the workspace until all commands and index readers have finished.
    pub _temp: TempDir,
    pub root: PathBuf,
    pub indexes: Vec<PathBuf>,
}

pub struct Session<'a> {
    pub config: &'a Config,
    pub corpus: &'a Prepared,
    pub variants: &'a [Variant; 1],
    pub workspace: PathBuf,
    pub report: &'a mut Report,
    pub seed: u64,
}

impl Session<'_> {
    pub fn fixture(&self, revision: &str) -> Result<Fixture> {
        let temp = tempfile::Builder::new()
            .prefix("case-")
            .tempdir_in(&self.workspace)?;
        let root = temp.path().join("repo");
        corpus::clone_at(&self.corpus.root, &root, revision)?;
        let indexes = self
            .variants
            .iter()
            .map(|v| temp.path().join(format!("index-{}", v.name)))
            .collect();
        Ok(Fixture {
            _temp: temp,
            root,
            indexes,
        })
    }

    pub fn command(&self, f: &Fixture, variant: usize, action: &str) -> Command {
        let mut command = Command::new(&self.variants[variant].binary);
        command
            .current_dir(&f.root)
            .env("LC_ALL", "C")
            // Always let the current implementation select its own thread count.
            .env_remove("RAYON_NUM_THREADS")
            .arg(action)
            .arg("--index-dir")
            .arg(&f.indexes[variant]);
        command
    }

    pub fn build(&self, f: &Fixture, v: usize) -> Command {
        let mut command = self.command(f, v, "index");
        command.arg(".");
        command
    }

    pub fn search(&self, f: &Fixture, v: usize, q: &Query) -> Command {
        let mut command = self.command(f, v, "search");
        command.args(&q.flags);
        command.arg("--").arg(&q.pattern).arg(".");
        command
    }

    pub fn rg(&self, f: &Fixture, q: &Query) -> Command {
        let mut command = Command::new("rg");
        command
            .current_dir(&f.root)
            .env("LC_ALL", "C")
            .args([
                "--no-config",
                "--hidden",
                "-g",
                "!.git",
                "--color",
                "never",
                "--no-heading",
                "-n",
            ])
            .args(&q.flags)
            .arg("--")
            .arg(&q.pattern)
            .arg(".");
        command
    }

    // Keep the raw sample dimensions explicit at recording sites.
    #[allow(clippy::too_many_arguments)]
    pub fn record(
        &mut self,
        scenario: &str,
        case: &str,
        variant: &str,
        round: usize,
        metric: &str,
        value: f64,
        unit: &str,
    ) -> Result<()> {
        self.report.sample(Sample {
            corpus: self.corpus.config.name.clone(),
            scenario: scenario.into(),
            case: case.into(),
            variant: variant.into(),
            round,
            metric: metric.into(),
            value,
            unit: unit.into(),
        })
    }

    pub fn measure(
        &mut self,
        scenario: &str,
        case: &str,
        variant: &str,
        round: usize,
        command: &mut Command,
        search: bool,
    ) -> Result<Vec<u8>> {
        let (ms, output) = process::timed(command, search)?;
        self.report.event(json!({"event": "command", "corpus": self.corpus.config.name, "scenario": scenario, "case": case, "variant": variant, "round": round, "command": format!("{command:?}"), "exit": output.status.code(), "stderr": String::from_utf8_lossy(&output.stderr)}))?;
        self.record(scenario, case, variant, round, "wall", ms, "ms")?;
        let diagnostics = String::from_utf8_lossy(&output.stderr);
        for (pattern, metrics) in [
            (
                r"snapshot: (\d+) extracted, (\d+) reused",
                vec![("extracted", "count"), ("reused", "count")],
            ),
            (
                r"spilled (\d+) build runs \((\d+) temporary bytes written, (\d+) peak temporary bytes\)",
                vec![
                    ("spill_runs", "count"),
                    ("temporary_written", "bytes"),
                    ("peak_temporary", "bytes"),
                ],
            ),
        ] {
            if let Some(captures) = regex::Regex::new(pattern)?.captures(&diagnostics) {
                for (i, (metric, unit)) in metrics.iter().enumerate() {
                    self.record(
                        scenario,
                        case,
                        variant,
                        round,
                        metric,
                        captures[i + 1].parse::<u64>()? as f64,
                        unit,
                    )?;
                }
            }
        }
        Ok(output.stderr)
    }

    pub fn resource(
        &mut self,
        scenario: &str,
        case: &str,
        variant: &str,
        round: usize,
        command: &Command,
        search: bool,
    ) -> Result<()> {
        match process::rss(command, search)? {
            Some(bytes) => self.record(scenario, case, variant, round, "peak_rss", bytes as f64, "bytes"),
            None => self.report.event(json!({"event": "unavailable", "metric": "peak_rss", "corpus": self.corpus.config.name, "scenario": scenario, "case": case, "variant": variant, "reason": "unsupported platform"})),
        }
    }

    pub fn initialize(&self, f: &Fixture) -> Result<()> {
        for v in 0..self.variants.len() {
            checked(&mut self.build(f, v), false)?;
        }
        Ok(())
    }

    /// Compare exit status plus full byte output; verification is outside timing.
    pub fn verify(&mut self, f: &Fixture, case: &str, queries: &[Query]) -> Result<()> {
        for q in queries {
            let expected = checked(&mut self.rg(f, q), true)?;
            let expected_lines = normalized(&expected.stdout);
            for v in 0..self.variants.len() {
                let actual = checked(&mut self.search(f, v, q), true)?;
                ensure!(
                    actual.status.code() == expected.status.code()
                        && normalized(&actual.stdout) == expected_lines,
                    "{} {case} {} {}: results differ from rg",
                    self.corpus.config.name,
                    self.variants[v].name,
                    q.name
                );
            }
        }
        self.report.event(json!({"event": "verified", "corpus": self.corpus.config.name, "case": case, "comparisons": queries.len()}))
    }

    pub fn stats(&self, f: &Fixture, v: usize) -> Result<Value> {
        let mut command = self.command(f, v, "stats");
        command.args(["--json", "."]);
        let stats: Value = serde_json::from_slice(&checked(&mut command, false)?.stdout)?;
        ensure!(
            stats["segments"].is_array(),
            "stats response has no segment array"
        );
        Ok(stats)
    }

    pub fn state(&mut self, f: &Fixture, scenario: &str, case: &str, round: usize) -> Result<()> {
        for v in 0..self.variants.len() {
            let stats = self.stats(f, v)?;
            let name = self.variants[v].name.clone();
            let documents = stats["documents"]
                .as_array()
                .context("stats has no documents")?;
            let mut files = 0;
            let mut bytes = 0_u64;
            for document in documents {
                if document["active"] == true && document["searchable"] == true {
                    files += 1;
                    bytes += document["len"].as_u64().context("invalid document size")?;
                }
            }
            self.record(
                scenario,
                case,
                &name,
                round,
                "searchable_files",
                files as f64,
                "count",
            )?;
            self.record(
                scenario,
                case,
                &name,
                round,
                "searchable_bytes",
                bytes as f64,
                "bytes",
            )?;

            self.record(
                scenario,
                case,
                &name,
                round,
                "index_size",
                corpus::directory_bytes(&f.indexes[v])? as f64,
                "bytes",
            )?;
            self.record(
                scenario,
                case,
                &name,
                round,
                "segments",
                stats["segments"].as_array().map_or(0, Vec::len) as f64,
                "count",
            )?;
            self.report.event(json!({"event": "state", "corpus": self.corpus.config.name, "scenario": scenario, "case": case, "round": round, "variant": name, "git_commit": process::git(&f.root, &["rev-parse", "HEAD"])?, "segments": stats["segments"]}))?;
        }
        Ok(())
    }
}
