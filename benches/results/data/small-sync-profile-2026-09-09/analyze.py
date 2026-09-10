#!/usr/bin/env python3
"""Summarize direct sync timings and paired real-history diagnostic controls."""

import json
import math
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent


def distribution(values):
    ordered = sorted(values)
    return {"median": statistics.median(ordered),
            "p95": ordered[math.ceil(len(ordered) * .95) - 1],
            "max": max(ordered), "sum": sum(ordered)}


def sync_times(measurement):
    return [float(line.split()[2]) for line in measurement["diagnostic"].splitlines()
            if line.startswith("PROBE ")]


def main():
    result = {"builds": {}, "history": {}}
    builds = json.loads((HERE / "builds.json").read_text())
    for name, dataset in builds["datasets"].items():
        result["builds"][name] = {
            variant: {"wall_ms": distribution([s["wall_ms"] for s in values]),
                      "sync_ms": distribution([
                          sum(sum(group) for group in s["sync_ms"].values()) for s in values])}
            for variant, values in dataset["samples"].items()
        }
        history = json.loads((HERE / f"{name}-history.json").read_text())
        summary = {"commits": len(history["updates"]),
                   "verified_comparisons": history["verified_comparisons"], "variants": {}}
        for variant in [v["name"] for v in history["variants"]]:
            measurements = [next(m for m in step["measurements"] if m["version"] == variant)
                            for step in history["updates"]]
            groups = {}
            for count in sorted({len(sync_times(m)) for m in measurements}):
                selected = [m for m in measurements if len(sync_times(m)) == count]
                groups[str(count)] = {
                    "updates": len(selected),
                    "wall_ms": distribution([m["wall_ms"] for m in selected]),
                    "sync_ms": distribution([sum(sync_times(m)) for m in selected]),
                }
            summary["variants"][variant] = {
                "wall_ms": distribution([m["wall_ms"] for m in measurements]),
                "sync_ms": distribution([sum(sync_times(m)) for m in measurements]),
                "sync_fraction_of_total": sum(sum(sync_times(m)) for m in measurements)
                                          / sum(m["wall_ms"] for m in measurements),
                "sync_call_groups": groups,
                "rollbacks_ms": [next(m["wall_ms"] for m in s["measurements"]
                                      if m["version"] == variant) for s in history["rollbacks"]],
            }
        for step in history["updates"] + history["rollbacks"]:
            current = next(m for m in step["measurements"] if m["version"] == "current")
            for variant in ["profile", "no-sync"]:
                other = next(m for m in step["measurements"] if m["version"] == variant)
                for key in ["base_bytes", "middle_bytes", "segments"]:
                    assert current[key] == other[key], (name, step["step"], variant, key)
                diagnostic = "\n".join(line for line in other["diagnostic"].splitlines()
                                       if line.startswith("coderg:"))
                assert current["diagnostic"] == diagnostic, (name, step["step"], variant)
        summary["identical_maintenance_except_sync"] = True
        result["history"][name] = summary
    (HERE / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    for name, data in result["history"].items():
        print(name, data["commits"], "commits", data["verified_comparisons"], "rg comparisons")
        for variant, values in data["variants"].items():
            print(variant, "wall ms", values["wall_ms"],
                  "sync median ms", values["sync_ms"]["median"],
                  "sync fraction", values["sync_fraction_of_total"])
        print("profile sync call groups", data["variants"]["profile"]["sync_call_groups"])


if __name__ == "__main__":
    main()
