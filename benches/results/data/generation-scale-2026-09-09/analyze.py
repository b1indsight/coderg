#!/usr/bin/env python3
"""Compare actual histories by scale without mixing update means and medians."""

import csv
import json
import math
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent


def summarize(values):
    ordered = sorted(values)
    return {"mean_ms": statistics.mean(values), "median_ms": statistics.median(values),
            "p95_ms": ordered[math.ceil(len(ordered) * .95) - 1],
            "max_ms": max(values), "total_ms": sum(values)}


def main():
    result, rows = {}, []
    for name in ["viberwhisper", "agentflow", "vllm"]:
        history_path = (HERE / "vllm-history.json" if name == "vllm" else
                        HERE.parent / "small-sync-profile-2026-09-09" / f"{name}-history.json")
        history = json.loads(history_path.read_text())
        snapshot_path = (HERE.parent / "main-projects-2026-09-09/results/vllm/results.json"
                         if name == "vllm" else
                         HERE.parent / "small-auto25-2026-09-09/results" / name / "results.json")
        snapshot = json.loads(snapshot_path.read_text())["builds"]["baseline"]
        data = {"history_source": str(history_path.relative_to(HERE.parent)),
                "base_commit": history["base_commit"], "end_commit": history["end_commit"],
                "commits": len(history["updates"]), "terminal_files": snapshot["files"],
                "terminal_source_mib": snapshot["source_bytes"] / 1048576,
                "verified_comparisons": history["verified_comparisons"], "variants": {}}
        for variant in ["main", "current", "no-sync"]:
            measurements = [next(m for m in step["measurements"] if m["version"] == variant)
                            for step in history["updates"]]
            initial = next(m for m in history["builds"] if m["version"] == variant)
            previous = initial["base_id"]
            base_changes = []
            for step, measurement in zip(history["updates"], measurements):
                if previous != measurement["base_id"]:
                    base_changes.append({"step": step["step"], "commit": step["commit"],
                                         "wall_ms": measurement["wall_ms"],
                                         "diagnostic": measurement["diagnostic"]})
                    previous = measurement["base_id"]
            data["variants"][variant] = {
                **summarize([m["wall_ms"] for m in measurements]),
                "base_changes": base_changes,
                "initial_base_mib": initial["base_bytes"] / 1048576,
                "final_base_mib": measurements[-1]["base_bytes"] / 1048576,
                "final_middle_mib": measurements[-1]["middle_bytes"] / 1048576,
            }
        if name == "vllm":
            data["reference_base_mib"] = data["variants"]["main"]["final_base_mib"]
            data["reference_base_note"] = "Main last rebuilt at step 96; approximate terminal base size."
        else:
            data["reference_base_mib"] = snapshot["base_bytes"] / 1048576
            data["reference_base_note"] = "Fresh build of the terminal frozen snapshot."
        data["current_vs_main_mean_change_pct"] = 100 * (
            data["variants"]["current"]["mean_ms"] / data["variants"]["main"]["mean_ms"] - 1)
        result[name] = data
        rows.append({"repository": name, "files": data["terminal_files"],
                     "source_mib": data["terminal_source_mib"],
                     "reference_base_mib": data["reference_base_mib"],
                     "commits": data["commits"],
                     **{f"{v}_mean_ms": data["variants"][v]["mean_ms"]
                        for v in ["main", "current", "no-sync"]}})
    (HERE / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    with (HERE / "comparison.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    for row in rows:
        print({k: round(v, 2) if isinstance(v, float) else v for k, v in row.items()})


if __name__ == "__main__":
    main()
