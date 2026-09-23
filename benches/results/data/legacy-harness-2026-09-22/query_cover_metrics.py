#!/usr/bin/env python3
"""Measure query filtering in temporary instrumented builds, outside CLI timings."""

import argparse
import hashlib
import json
from pathlib import Path
import random
import shutil
import statistics
import subprocess
import sys

sys.dont_write_bytecode = True
from refresh_matrix import queries_for


FILTER = r'''
fn choose_candidates(
    index: &index::DiskIndex,
    strategies: &[Vec<ngram::GramHash>],
) -> Result<Vec<u32>> {
    let started = std::time::Instant::now();
    let mut keys = 0usize;
    let mut decoded_ids = 0usize;
    let mut load_ns = 0u128;
    let result = intersect_postings(strategies, |hash| {
        let load_started = std::time::Instant::now();
        let postings = index.postings(hash)?;
        load_ns += load_started.elapsed().as_nanos();
        keys += 1;
        decoded_ids += postings.len();
        Ok(postings)
    })?;
    let filter_ns = started.elapsed().as_nanos();
    let candidates: Vec<u32> = match result {
        Some(ids) => ids.into_iter().collect(),
        None => index.all_document_ids(),
    };
    let bytes: u64 = candidates.iter().map(|&id| index.manifest.documents[id as usize].len).sum();
    eprintln!("COVER_METRICS {}", serde_json::json!({
        "candidates": candidates.len(), "candidate_bytes": bytes,
        "keys": keys, "decoded_ids": decoded_ids,
        "postings_ns": load_ns, "filter_ns": filter_ns,
        "set_and_cache_ns": filter_ns - load_ns,
    }));
    Ok(candidates)
}

'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    baseline = parser.add_mutually_exclusive_group()
    baseline.add_argument("--baseline-ref", default="7438754")
    baseline.add_argument("--baseline-source", type=Path, help="Frozen source tree to compare with the working tree")
    parser.add_argument("--plan", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--iterations", type=int, default=11)
    args = parser.parse_args()
    if args.iterations < 1:
        parser.error("iterations must be positive")
    repo = Path(__file__).resolve().parent.parent
    output = args.output.resolve()
    plan = json.loads(args.plan.read_text())
    if any(output.is_relative_to(Path(item["root"]).resolve()) for item in plan["datasets"]):
        parser.error("output must be outside every corpus")
    output.mkdir(parents=True, exist_ok=False)
    binaries = {}
    sources = {}
    for mode in ("baseline", "optimized"):
        checkout = output / mode
        (checkout / "src").mkdir(parents=True)
        sources[mode] = {}
        for relative in ["Cargo.toml", "Cargo.lock", *[str(p.relative_to(repo)) for p in sorted((repo / "src").glob("*.rs"))]]:
            if mode == "baseline" and args.baseline_source:
                content = (args.baseline_source / relative).read_bytes()
            elif mode == "baseline":
                content = subprocess.check_output(["git", "show", f"{args.baseline_ref}:{relative}"], cwd=repo)
            else:
                content = (repo / relative).read_bytes()
            sources[mode][relative] = hashlib.sha256(content).hexdigest()
            (checkout / relative).write_bytes(content)
        # The executable uses only src/; exclude the unrelated benchmark target.
        manifest = checkout / "Cargo.toml"
        text = manifest.read_text()
        start = text.index("[[bench]]")
        end = text.index("[profile.release]", start)
        manifest.write_text(text[:start] + text[end:])
        search = checkout / "src/search.rs"
        text = search.read_text()
        start, end = text.index("fn choose_candidates("), text.index("fn intersect_postings(")
        instrumented_filter = FILTER
        if "strategies: &[query::GramGroup]" in text[start:end]:
            instrumented_filter = FILTER.replace(
                "strategies: &[Vec<ngram::GramHash>]", "strategies: &[query::GramGroup]"
            )
        text = text[:start] + instrumented_filter + text[end:]
        marker = "    let strategies = if options.fixed_strings"
        assert text.count(marker) == 1
        text = text.replace(marker, "    let planning_started = std::time::Instant::now();\n" + marker)
        marker = "    let candidates = choose_candidates(&disk_index, &strategies)?;"
        assert text.count(marker) == 1
        text = text.replace(marker, '    eprintln!("COVER_PLAN_NS {}", planning_started.elapsed().as_nanos());\n' + marker)
        search.write_text(text)
        subprocess.run(["cargo", "build", "--release", "--offline", "--manifest-path", str(manifest),
                        "--target-dir", str(output / "target")], check=True)
        binaries[mode] = output / (mode + "-metrics")
        shutil.copy2(output / "target/release/coderg", binaries[mode])

    results = {"baseline_ref": None if args.baseline_source else args.baseline_ref,
               "baseline_source": str(args.baseline_source.resolve()) if args.baseline_source else None,
               "source_sha256": sources,
               "iterations": args.iterations, "seed": 20260908, "datasets": []}
    rng = random.Random(results["seed"])
    for dataset in plan["datasets"]:
        root = Path(dataset["root"])
        index = output / (dataset["id"] + "-index")
        subprocess.run([str(binaries["baseline"]), "index", str(root), "--index-dir", str(index)], check=True)
        entry = {"id": dataset["id"], "queries": []}
        for query_id, category, pattern, flags in queries_for(dataset["kind"]):
            query = {"id": query_id, "pattern": pattern, "category": category, "modes": {}}
            samples = {mode: [] for mode in binaries}
            fingerprints = set()
            for repeat in range(args.iterations + 1):
                modes = list(binaries)
                rng.shuffle(modes)
                for mode in modes:
                    proc = subprocess.run([str(binaries[mode]), "search", *flags, "--no-refresh", pattern,
                                           str(root), "--index-dir", str(index)], capture_output=True, check=False)
                    assert proc.returncode in (0, 1), proc.stderr
                    fingerprints.add((proc.returncode, hashlib.sha256(proc.stdout).hexdigest()))
                    lines = proc.stderr.decode().splitlines()
                    assert len(lines) == 2, lines
                    metrics = json.loads(next(line.removeprefix("COVER_METRICS ") for line in lines if line.startswith("COVER_METRICS ")))
                    metrics["planning_ns"] = int(next(line.removeprefix("COVER_PLAN_NS ") for line in lines if line.startswith("COVER_PLAN_NS ")))
                    if repeat:
                        samples[mode].append(metrics)
            assert len(fingerprints) == 1, (dataset["id"], query_id)
            for mode, values in samples.items():
                query["modes"][mode] = {"samples": values, "median": {
                    key: statistics.median(sample[key] for sample in values) for key in values[0]}}
            entry["queries"].append(query)
        results["datasets"].append(entry)
        (output / "metrics.json").write_text(json.dumps(results, indent=2) + "\n")
        print(f"{dataset['id']}: {len(entry['queries'])} queries measured", flush=True)


if __name__ == "__main__":
    main()
