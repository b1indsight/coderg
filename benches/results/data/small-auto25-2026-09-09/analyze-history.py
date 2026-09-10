#!/usr/bin/env python3
"""Summarize real first-parent histories; Python standard library only."""

import csv
import json
import math
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
MIB = 1024 * 1024


def distribution(values):
    values = sorted(values)
    return {
        "median": statistics.median(values),
        "p95": values[math.ceil(len(values) * 0.95) - 1],
        "max": max(values),
        "sum": sum(values),
    }


def summarize(report):
    result = {
        "base_commit": report["base_commit"],
        "end_commit": report["end_commit"],
        "commits": len(report["updates"]),
        "verified_comparisons": report["verified_comparisons"],
        "changes": {
            key: distribution([step[key] for step in report["updates"]])
            for key in ["changed_files", "added_lines", "deleted_lines", "target_source_bytes"]
        },
        "variants": {},
    }
    for variant in report["variants"]:
        name = variant["name"]
        measurements = [
            (step, next(m for m in step["measurements"] if m["version"] == name))
            for step in report["updates"]
        ]
        full = [
            {"step": step["step"], "commit": step["commit"], **measurement}
            for step, measurement in measurements
            if "base + middle" in measurement["diagnostic"]
        ]
        queries = {}
        for query in sorted({q["query"] for q in report["queries"]}):
            for no_refresh in [False, True]:
                rows = [
                    q for q in report["queries"]
                    if q["version"] == name and q["query"] == query
                    and q["no_refresh"] == no_refresh
                ]
                queries[f"{query}|{'no-refresh' if no_refresh else 'default'}"] = {
                    "checkpoint_medians_ms": {
                        str(q["step"]): statistics.median(q["samples_ms"]) for q in rows
                    },
                    "all_samples_ms": distribution([v for q in rows for v in q["samples_ms"]]),
                }
        result["variants"][name] = {
            "update_ms": distribution([m["wall_ms"] for _, m in measurements]),
            "without_base_merge_ms": distribution([
                m["wall_ms"] for _, m in measurements
                if "base + middle" not in m["diagnostic"]
            ]),
            "full_merges": full,
            "middle_merges": sum(
                "coderg: compacted" in m["diagnostic"] and "base + middle" not in m["diagnostic"]
                for _, m in measurements
            ),
            "max_middle_mib": max(m["middle_bytes"] for _, m in measurements) / MIB,
            "max_update_rss_mib": max(m["peak_rss_bytes"] or 0 for _, m in measurements) / MIB,
            "final": measurements[-1][1],
            "queries": queries,
            "rollbacks": [
                {"from": step["from_commit"], "to": step["commit"],
                 "changed_files": step["changed_files"],
                 **next(m for m in step["measurements"] if m["version"] == name)}
                for step in report["rollbacks"]
            ],
        }
    return result


def main():
    summaries = {}
    with (HERE / "history-updates.csv").open("w", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow([
            "experiment", "step", "commit", "changed_files", "target_source_bytes",
            "variant", "wall_ms", "base_mib", "middle_mib", "segments", "base_merged",
        ])
        for filename in ["viberwhisper-history.json", "agentflow-history.json"]:
            report = json.loads((HERE / filename).read_text())
            summaries[filename] = summarize(report)
            for step in report["updates"]:
                for m in step["measurements"]:
                    writer.writerow([
                        filename, step["step"], step["commit"], step["changed_files"],
                        step["target_source_bytes"], m["version"], m["wall_ms"],
                        m["base_bytes"] / MIB, m["middle_bytes"] / MIB, m["segments"],
                        "base + middle" in m["diagnostic"],
                    ])
    (HERE / "history-summary.json").write_text(json.dumps(summaries, indent=2) + "\n")
    for filename, summary in summaries.items():
        print(filename, summary["commits"], "commits", summary["verified_comparisons"], "rg checks")
        print("changes", summary["changes"])
        for name, variant in summary["variants"].items():
            final = variant["final"]
            print(name, "latency_ms", variant["update_ms"], "full steps",
                  [m["step"] for m in variant["full_merges"]],
                  "final B/M MiB", final["base_bytes"] / MIB, final["middle_bytes"] / MIB,
                  "max M MiB", variant["max_middle_mib"],
                  "disk MiB", final["retained_bytes"] / MIB,
                  "peak RSS MiB", variant["max_update_rss_mib"])


if __name__ == "__main__":
    main()
