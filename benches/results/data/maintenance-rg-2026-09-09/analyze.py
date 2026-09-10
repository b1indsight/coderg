#!/usr/bin/env python3
"""Group maintenance events and pair each group with rg on identical commits."""

import csv
import json
import math
from pathlib import Path
import re
import statistics

HERE = Path(__file__).resolve().parent
MIB = 1048576
MERGE = re.compile(r"compacted (\d+) (base \+ middle|middle) segments "
                   r"\((\d+) input bytes, (\d+) output bytes\)")


def stats(values):
    values = sorted(values)
    return {"count": len(values), "median": statistics.median(values),
            "p95": values[math.ceil(len(values) * .95) - 1],
            "min": values[0], "max": values[-1]}


def phase(measurement):
    diagnostic = measurement["diagnostic"]
    for fragment, name in [("base + middle", "base_merge"), ("compacted", "middle_merge"),
                           ("rebuilt index", "rebuild"), ("incrementally indexed", "incremental"),
                           ("advanced index", "metadata"), ("cached", "cached_tree")]:
        if fragment in diagnostic:
            return name
    if not diagnostic:
        return "unchanged"
    raise ValueError(diagnostic)


def main():
    summaries, flat = {}, []
    for repository in ["viberwhisper", "agentflow", "vllm"]:
        report = json.loads((HERE / f"{repository}-history.json").read_text())
        records = []
        for step in report["updates"]:
            versions = {m["version"]: m for m in step["measurements"]}
            current = versions["current"]
            control = versions["no-sync"]
            for key in ["base_bytes_before", "middle_bytes_before", "base_bytes",
                        "middle_bytes", "segments", "diagnostic"]:
                assert current[key] == control[key], (repository, step["step"], key)
            merge = MERGE.search(current["diagnostic"])
            merge_middle = None
            if merge:
                input_bytes, output_bytes = int(merge[3]), int(merge[4])
                merge_middle = (input_bytes - current["base_bytes_before"]
                                if merge[2] == "base + middle" else
                                current["middle_bytes"] + input_bytes - output_bytes)
                assert merge_middle >= 0
                if merge[2] == "base + middle":
                    assert current["middle_bytes"] == 0 and current["base_bytes"] == output_bytes
                else:
                    assert current["base_bytes"] == current["base_bytes_before"]
            row = {"repository": repository, "step": step["step"], "commit": step["commit"],
                   "current_phase": phase(current), "main_phase": phase(versions["main"]),
                   "current_ms": current["wall_ms"], "no_sync_ms": control["wall_ms"],
                   "main_ms": versions["main"]["wall_ms"], "rg_ms": step["rg"]["wall_ms"],
                   "middle_before_update_mib": current["middle_bytes_before"] / MIB,
                   "middle_entering_merge_mib": merge_middle / MIB if merge_middle is not None else None,
                   "middle_after_update_mib": current["middle_bytes"] / MIB,
                   "base_after_mib": current["base_bytes"] / MIB,
                   "main_middle_after_mib": versions["main"]["middle_bytes"] / MIB}
            records.append(row)
            flat.append(row)
        summary = {"query": report["update_query"], "commits": len(records),
                   "base_commit": report["base_commit"], "end_commit": report["end_commit"],
                   "verified_comparisons": report["verified_comparisons"],
                   "current_groups": {}, "main_groups": {},
                   "final_middle_mib": records[-1]["middle_after_update_mib"],
                   "max_middle_after_update_mib": max(r["middle_after_update_mib"] for r in records)}
        for category in ["incremental", "middle_merge", "base_merge", "metadata", "unchanged"]:
            selected = [r for r in records if r["current_phase"] == category]
            if not selected:
                continue
            grouped = {"steps": [r["step"] for r in selected],
                       **{key: stats([r[key] for r in selected]) for key in [
                           "current_ms", "no_sync_ms", "rg_ms", "middle_before_update_mib",
                           "middle_after_update_mib"]}}
            entering = [r["middle_entering_merge_mib"] for r in selected
                        if r["middle_entering_merge_mib"] is not None]
            if entering:
                grouped["middle_entering_merge_mib"] = stats(entering)
            summary["current_groups"][category] = grouped
        for category in ["incremental", "rebuild", "metadata", "cached_tree", "unchanged"]:
            selected = [r for r in records if r["main_phase"] == category]
            if selected:
                summary["main_groups"][category] = {
                    "steps": [r["step"] for r in selected],
                    **{key: stats([r[key] for r in selected])
                       for key in ["main_ms", "rg_ms", "main_middle_after_mib"]}}
        summary["full_merge_events"] = [r for r in records if r["current_phase"] == "base_merge"]
        summaries[repository] = summary
    (HERE / "summary.json").write_text(json.dumps(summaries, indent=2) + "\n")
    with (HERE / "per-commit.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(flat[0]))
        writer.writeheader()
        writer.writerows(flat)
    for repository, summary in summaries.items():
        print(repository, repr(summary["query"]), summary["commits"], "commits")
        for category, group in summary["current_groups"].items():
            print(category, len(group["steps"]),
                  "current/no-sync/rg median ms", [round(group[k]["median"], 2)
                                                   for k in ["current_ms", "no_sync_ms", "rg_ms"]],
                  "current p95", round(group["current_ms"]["p95"], 2),
                  "M after min/max", [round(group["middle_after_update_mib"][k], 3)
                                       for k in ["min", "max"]])
        for category, group in summary["main_groups"].items():
            print("main", category, len(group["steps"]), "median/p95/rg ms",
                  [round(group["main_ms"][k], 2) for k in ["median", "p95"]]
                  + [round(group["rg_ms"]["median"], 2)])


if __name__ == "__main__":
    main()
