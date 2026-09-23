#!/usr/bin/env python3
"""Summarize paired repetitions without treating RAM-only time as end-to-end."""
import argparse
import json
from pathlib import Path
from statistics import median


def summarize(path):
    report = json.loads(path.read_text())
    groups = {}
    for row in report["rows"]:
        groups.setdefault(row["step"], []).append(row)
    steps = {}
    for name, rows in groups.items():
        timings = {}
        for variant, field in [
            ("scan", "input_scan_ms"),
            ("snapshot_update", "snapshot_ms"),
            ("incremental_update", "incremental_ms"),
            ("production_total", "production_ms"),
            ("snapshot_total", "snapshot_ms"),
            ("incremental_total", "incremental_ms"),
        ]:
            values = [
                row[field] + (row["input_scan_ms"] if variant in
                              ("snapshot_total", "incremental_total") else 0)
                for row in rows
            ]
            timings[variant] = {
                "median_ms": median(values), "min_ms": min(values),
                "max_ms": max(values),
            }
        steps[name] = {
            "samples": len(rows), "timings": timings,
            "snapshot_extracted_files": [r["snapshot_work"]["extracted_files"] for r in rows],
            "incremental_extracted_files": [r["incremental_work"]["extracted_files"] for r in rows],
        }
    return {
        "input": str(path), "files": report["files"], "steps": steps,
        "verified_queries": sum(r["verified_queries"] for r in report["rows"]),
        "final_snapshot_retained": report["rows"][-1]["snapshot_retained"],
        "final_incremental_retained": report["rows"][-1]["incremental_retained"],
        "initial_production_posting_bytes": report["rows"][0]["production_posting_bytes"],
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inputs", nargs="+", type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    args.output.write_text(json.dumps([summarize(p) for p in args.inputs], indent=2) + "\n")
