#!/usr/bin/env python3
"""Summarize production snapshot CLI benchmarks, keeping query classes separate."""
import argparse
from collections import defaultdict
import json
from pathlib import Path
from statistics import median


def distribution(values):
    ordered = sorted(values)
    return {"count": len(values), "median_ms": median(values),
            "min_ms": ordered[0], "max_ms": ordered[-1],
            "p95_ms": ordered[min(len(ordered) - 1, (95 * len(ordered) + 99) // 100 - 1)]}


def summarize(path):
    data = json.loads(path.read_text())
    updates, queries = defaultdict(lambda: defaultdict(list)), defaultdict(lambda: defaultdict(list))
    rows = data.get("rows", [{**row, "step": "build_A"} for row in data.get("builds", [])])
    for row in rows:
        updates[row["step"]][row["variant"]].append(row)
    for row in data["searches"]:
        key = row["query"] + ("/no-refresh" if row["no_refresh"] else "/refresh")
        if row.get("checkpoint", "build_A") != "build_A":
            key = row["checkpoint"] + "/" + key
        queries[key][row["variant"]].append(row)

    def groups(source):
        result = {}
        for key, variants in source.items():
            item = {name: distribution([r["elapsed_ms"] for r in rows]) for name, rows in variants.items()}
            item["change_percent"] = (item["snapshots"]["median_ms"] / item["baseline"]["median_ms"] - 1) * 100
            paired = []
            for iteration in range(data["iterations"]):
                times = {name: median(r["elapsed_ms"] for r in rows if r["iteration"] == iteration)
                         for name, rows in variants.items()}
                paired.append((times["snapshots"] / times["baseline"] - 1) * 100)
            item["paired_iteration_change_percent"] = paired
            result[key] = item
        return result

    return {"input": str(path), "files": data.get("files", rows[0].get("searchable_files")),
            "source": data.get("source"), "commit": data.get("commit"),
            "verified_queries": data["verified_queries"], "updates": groups(updates), "searches": groups(queries),
            "disk_bytes": {step: {name: {"median": median(r["disk_bytes"] for r in rows),
                                        "max": max(r["disk_bytes"] for r in rows)}
                                  for name, rows in variants.items()} for step, variants in updates.items()}}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inputs", type=Path, nargs="+")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.output.write_text(json.dumps([summarize(p) for p in args.inputs], indent=2) + "\n")
