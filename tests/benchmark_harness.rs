//! Regression checks for measurement semantics rather than performance thresholds.
#[allow(dead_code)]
#[path = "../benches/harness/config.rs"]
mod config;
#[allow(dead_code)]
#[path = "../benches/harness/corpus.rs"]
mod corpus;
#[allow(dead_code)]
#[path = "../benches/harness/process.rs"]
mod process;
#[allow(dead_code)]
#[path = "../benches/harness/report.rs"]
mod report;

#[test]
fn output_comparison_keeps_duplicates_and_non_utf8_bytes() {
    // A set comparison hid duplicate matching lines in the old process harness.
    assert_ne!(
        process::normalized(b"./a:1:x\na:1:x\n"),
        process::normalized(b"a:1:x\n")
    );
    assert_eq!(
        process::normalized(b"./b:\xff\na:1:x\n"),
        process::normalized(b"a:1:x\nb:\xff\n")
    );
}

#[test]
fn resource_units_and_missing_values_are_explicit() {
    assert_eq!(
        process::parse_rss("  2048 maximum resident set size\n", true).unwrap(),
        2048
    );
    assert_eq!(
        process::parse_rss("diagnostic\nCODERG_RSS_KIB=2048\n", false).unwrap(),
        2_097_152
    );
    assert!(process::parse_rss("no RSS available", true).is_err());
}

#[test]
fn statistics_use_even_median_and_nearest_rank_percentile() {
    let stats = report::statistics(&[4.0, 1.0, 3.0, 2.0]);
    assert_eq!(stats["median"], 2.5);
    assert_eq!(stats["p95"], 4.0);
    assert_eq!(stats["n"], 4);
}

#[test]
fn configs_are_valid_and_reject_ambiguous_workloads() {
    let mut smoke: config::Config =
        serde_json::from_str(include_str!("../benches/smoke.json")).unwrap();
    smoke.validate().unwrap();
    let full: config::Config = serde_json::from_str(include_str!("../benches/full.json")).unwrap();
    full.validate().unwrap();
    let mut unsupported = serde_json::to_value(&full).unwrap();
    unsupported["scenarios"] = serde_json::json!(["maintenance"]);
    assert!(serde_json::from_value::<config::Config>(unsupported).is_err());
    // Removed tuning knobs must fail explicitly instead of silently changing defaults.
    for field in ["threads", "budgets_mib"] {
        let mut value = serde_json::to_value(&full).unwrap();
        value[field] = serde_json::json!([4]);
        assert!(serde_json::from_value::<config::Config>(value).is_err());
    }
    smoke.corpora[0].queries[0].flags = vec!["-l".into(), "-c".into()];
    assert!(smoke.validate().is_err());
    smoke.corpora[0].queries[0].flags.clear();
    smoke.corpora[0].name = "../escape".into();
    assert!(smoke.validate().is_err());
}

#[test]
fn interrupted_reports_remain_marked_incomplete() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".cache/2026-09-22");
    std::fs::create_dir_all(&root).unwrap();
    let temporary = tempfile::tempdir_in(root).unwrap();
    let output = temporary.path().join("run");
    let mut report = report::Report::create(&output).unwrap();
    std::fs::write(
        output.join("run.json"),
        r#"{"config":{"name":"interrupted"},"status":"incomplete"}"#,
    )
    .unwrap();
    report
        .sample(report::Sample {
            corpus: "fixture".into(),
            scenario: "build".into(),
            case: "fresh".into(),
            variant: "current".into(),
            round: 0,
            metric: "wall".into(),
            value: 10.0,
            unit: "ms".into(),
        })
        .unwrap();
    report::render(&output).unwrap();
    let summary: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output.join("summary.json")).unwrap()).unwrap();
    assert_eq!(summary["status"], "incomplete");
    assert_eq!(summary["groups"][0]["statistics"]["n"], 1);
    assert!(report::Report::create(&output).is_err());
}

#[test]
fn state_transition_reports_compare_against_rg() {
    // Workflow searches used to have no rg comparison outside the search scenario.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".cache/2026-09-22");
    std::fs::create_dir_all(&root).unwrap();
    let temporary = tempfile::tempdir_in(root).unwrap();
    let output = temporary.path().join("run");
    let mut report = report::Report::create(&output).unwrap();
    std::fs::write(
        output.join("run.json"),
        r#"{"config":{"name":"steps"},"status":"complete"}"#,
    )
    .unwrap();
    for (variant, value) in [("current", 10.0), ("rg", 20.0)] {
        report
            .sample(report::Sample {
                corpus: "fixture".into(),
                scenario: "workflow".into(),
                case: "edit".into(),
                variant: variant.into(),
                round: 0,
                metric: "wall".into(),
                value,
                unit: "ms".into(),
            })
            .unwrap();
    }
    report::render(&output).unwrap();
    let summary: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output.join("summary.json")).unwrap()).unwrap();
    assert_eq!(summary["comparisons"].as_array().unwrap().len(), 1);
    assert_eq!(summary["comparisons"][0]["reference"], "rg");
    assert_eq!(summary["comparisons"][0]["median_change_percent"], -50.0);
}

#[test]
fn synthetic_history_is_repeatable_and_pinned_clones_ignore_dirty_sources() {
    let cache = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".cache/2026-09-22");
    std::fs::create_dir_all(&cache).unwrap();
    let temporary = tempfile::tempdir_in(cache).unwrap();
    let config: config::Config =
        serde_json::from_str(include_str!("../benches/smoke.json")).unwrap();
    let mut commits = Vec::new();
    for i in 0..2 {
        let workspace = temporary.path().join(format!("work-{i}"));
        std::fs::create_dir(&workspace).unwrap();
        let mut report =
            report::Report::create(&temporary.path().join(format!("report-{i}"))).unwrap();
        let prepared =
            corpus::prepare(&config.corpora[0], &workspace, config.seed, &mut report).unwrap();
        commits.push(prepared.commits.clone());
        assert_eq!(prepared.commits.len(), 6);
        // A local source can have uncommitted edits: a pinned benchmark clone must
        // ignore them and must never checkout or reset that user's source tree.
        let tracked = prepared.root.join("file_00000.rs");
        std::fs::write(&tracked, "UNCOMMITTED_SOURCE_CHANGE").unwrap();
        let clone = workspace.join("pinned");
        corpus::clone_at(&prepared.root, &clone, &prepared.commits[0]).unwrap();
        assert!(
            !std::fs::read_to_string(clone.join("file_00000.rs"))
                .unwrap()
                .contains("UNCOMMITTED")
        );
        assert_eq!(
            std::fs::read_to_string(tracked).unwrap(),
            "UNCOMMITTED_SOURCE_CHANGE"
        );
    }
    assert_eq!(commits[0], commits[1]);
}

#[test]
fn merging_workers_preserves_partial_results_and_duplicate_samples() {
    // One corpus can fail after emitting samples while its sibling completes.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".cache/2026-09-22");
    std::fs::create_dir_all(&root).unwrap();
    let temporary = tempfile::tempdir_in(root).unwrap();
    let mut merged = report::Report::create(&temporary.path().join("merged")).unwrap();
    for name in ["completed", "failed"] {
        let path = temporary.path().join(name);
        let mut worker = report::Report::create(&path).unwrap();
        for _ in 0..2 {
            worker
                .sample(report::Sample {
                    corpus: name.into(),
                    scenario: "search".into(),
                    case: "absent".into(),
                    variant: "current".into(),
                    round: 0,
                    metric: "latency".into(),
                    value: 1.0,
                    unit: "ms".into(),
                })
                .unwrap();
        }
        worker.event(serde_json::json!({"corpus": name})).unwrap();
        drop(worker);
        merged.merge(&path).unwrap();
    }
    let samples = std::fs::read_to_string(merged.path.join("samples.jsonl")).unwrap();
    assert_eq!(samples.lines().count(), 4);
    assert_eq!(
        samples
            .lines()
            .filter(|line| line.contains("failed"))
            .count(),
        2
    );
    let events = std::fs::read_to_string(merged.path.join("events.jsonl")).unwrap();
    assert_eq!(events.lines().count(), 2);
}
