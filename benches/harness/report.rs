//! Append-only raw records and repeatable Markdown/JSON summaries.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sample {
    pub corpus: String,
    pub scenario: String,
    pub case: String,
    pub variant: String,
    pub round: usize,
    pub metric: String,
    pub value: f64,
    pub unit: String,
}

pub struct Report {
    pub path: PathBuf,
    samples: File,
    events: File,
}

type GroupKey = (String, String, String, String, String, String);

impl Report {
    pub fn create(path: &Path) -> Result<Self> {
        fs::create_dir(path)?;
        Ok(Self {
            path: path.into(),
            samples: OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(path.join("samples.jsonl"))?,
            events: OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(path.join("events.jsonl"))?,
        })
    }
    /// Merge a finished worker's records, including samples before a failure.
    pub fn merge(&mut self, path: &Path) -> Result<()> {
        std::io::copy(
            &mut File::open(path.join("samples.jsonl"))?,
            &mut self.samples,
        )?;
        std::io::copy(
            &mut File::open(path.join("events.jsonl"))?,
            &mut self.events,
        )?;
        self.samples.flush()?;
        self.events.flush()?;
        Ok(())
    }
    pub fn sample(&mut self, sample: Sample) -> Result<()> {
        serde_json::to_writer(&mut self.samples, &sample)?;
        writeln!(self.samples)?;
        self.samples.flush()?;
        Ok(())
    }
    pub fn event(&mut self, event: Value) -> Result<()> {
        serde_json::to_writer(&mut self.events, &event)?;
        writeln!(self.events)?;
        self.events.flush()?;
        Ok(())
    }
}

pub fn statistics(values: &[f64]) -> Value {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    assert!(n > 0);
    let median = if n.is_multiple_of(2) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };
    json!({"n": n, "min": sorted[0], "median": median, "mean": sorted.iter().sum::<f64>() / n as f64, "p95": sorted[(n * 95).div_ceil(100) - 1], "max": sorted[n - 1]})
}

/// Re-render from raw files without executing any benchmark or touching corpora.
pub fn render(path: &Path) -> Result<()> {
    let run: Value = serde_json::from_slice(&fs::read(path.join("run.json"))?)?;
    let mut groups: BTreeMap<GroupKey, Vec<f64>> = BTreeMap::new();
    for line in BufReader::new(File::open(path.join("samples.jsonl"))?).lines() {
        let s: Sample = serde_json::from_str(&line?)?;
        groups
            .entry((s.corpus, s.scenario, s.case, s.variant, s.metric, s.unit))
            .or_default()
            .push(s.value);
    }
    let mut rows = Vec::new();
    let mut markdown = format!(
        "# Benchmark: {}\n\nStatus: **{}**. Warm filesystem cache; fresh CLI processes. RSS sampled separately.\n\nMaximum concurrent corpora: **{}**; values above 1 include cross-corpus contention. Records are grouped by corpus, not global execution order.\n\nHistory steps are heterogeneous workloads; p95 is descriptive, not a confidence interval.\n\n| Corpus | Scenario | Case | Variant | Metric | N | Median | Mean | P95 | Max | Unit |\n|---|---|---|---|---|---:|---:|---:|---:|---:|---|\n",
        run["config"]["name"].as_str().unwrap_or("unknown"),
        run["status"].as_str().unwrap_or("incomplete"),
        run["jobs"].as_u64().unwrap_or(1)
    );
    let mut comparisons = Vec::new();
    for ((corpus, scenario, case, variant, metric, unit), values) in &groups {
        let stats = statistics(values);
        let reference = if groups.contains_key(&(
            corpus.clone(),
            scenario.clone(),
            case.clone(),
            "rg".into(),
            metric.clone(),
            unit.clone(),
        )) {
            "rg"
        } else {
            "current"
        };
        if variant != reference
            && let Some(base) = groups.get(&(
                corpus.clone(),
                scenario.clone(),
                case.clone(),
                reference.into(),
                metric.clone(),
                unit.clone(),
            ))
        {
            let base_median = statistics(base)["median"].as_f64().unwrap();
            let median = stats["median"].as_f64().unwrap();
            comparisons.push(json!({"corpus": corpus, "scenario": scenario, "case": case, "variant": variant, "reference": reference, "metric": metric, "unit": unit, "median_delta": median - base_median, "median_change_percent": if base_median > 0.0 { Some((median / base_median - 1.0) * 100.0) } else { None }, "method": "difference of medians within the same run; no confidence interval"}));
        }
        markdown.push_str(&format!("| {corpus} | {scenario} | {case} | {variant} | {metric} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {unit} |\n", values.len(), stats["median"].as_f64().unwrap(), stats["mean"].as_f64().unwrap(), stats["p95"].as_f64().unwrap(), stats["max"].as_f64().unwrap()));
        rows.push(json!({"corpus": corpus, "scenario": scenario, "case": case, "variant": variant, "metric": metric, "unit": unit, "statistics": stats}));
    }
    markdown.push_str("\n## Same-run comparisons\n\nNegative delta means lower cost. Differences of medians, without confidence intervals.\n\n| Corpus | Scenario | Case | Variant | Reference | Metric | Delta | Change % |\n|---|---|---|---|---|---|---:|---:|\n");
    for c in &comparisons {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {:.3} | {} |\n",
            c["corpus"].as_str().unwrap(),
            c["scenario"].as_str().unwrap(),
            c["case"].as_str().unwrap(),
            c["variant"].as_str().unwrap(),
            c["reference"].as_str().unwrap(),
            c["metric"].as_str().unwrap(),
            c["median_delta"].as_f64().unwrap(),
            c["median_change_percent"]
                .as_f64()
                .map(|n| format!("{n:.2}"))
                .unwrap_or_else(|| "n/a".into())
        ));
    }
    fs::write(
        path.join("summary.json"),
        serde_json::to_vec_pretty(
            &json!({"schema_version": 1, "status": run["status"], "groups": rows, "comparisons": comparisons}),
        )?,
    )?;
    fs::write(path.join("report.md"), markdown)?;
    Ok(())
}
