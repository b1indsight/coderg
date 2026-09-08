"""Summarize the archived before/after release benchmark."""
import argparse
import json
import statistics
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("input", type=Path)
parser.add_argument("output", type=Path)
args = parser.parse_args()
data = json.loads(args.input.read_text())
summary = {"builds_with_byte_parity": len(data["runs"]), "corpora": {}}
summary["queries_with_output_parity"] = sum(
    len(run.get("search_parity", [])) for run in data["runs"]
)
for corpus in data["corpora"]:
    versions = {}
    for version, label in [("before", "before"), ("budget_256", "after")]:
        rows = [r for r in data["runs"] if r["corpus"] == corpus and r["version"] == version]
        timing = [r["elapsed_ms"] for r in rows if r["kind"] == "timing"]
        rss = [r["rss_bytes"] / 2**20 for r in rows if r["kind"] == "rss"]
        versions[label] = {
            "timing_samples": len(timing),
            "median_ms": statistics.median(timing),
            "min_ms": min(timing),
            "max_ms": max(timing),
            "median_rss_mib": statistics.median(rss),
            "max_rss_mib": max(rss),
            "spilled_runs": sorted({r["spilled_runs"] for r in rows}),
            "temporary_bytes_written": sorted({r["temporary_bytes_written"] for r in rows}),
            "source_bytes": rows[0]["source_bytes"],
            "searchable_files": rows[0]["searchable_files"],
            "median_temporary_mib": statistics.median(r["temporary_bytes_written"] for r in rows) / 2**20,
            "median_peak_temporary_mib": statistics.median(r["peak_temporary_bytes"] for r in rows) / 2**20,
            "index_bytes": rows[0]["index_bytes"],
        }
    before, after = versions["before"], versions["after"]
    versions["latency_change_pct"] = (after["median_ms"] / before["median_ms"] - 1) * 100
    versions["rss_change_mib"] = after["median_rss_mib"] - before["median_rss_mib"]
    summary["corpora"][corpus] = versions
args.output.write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
