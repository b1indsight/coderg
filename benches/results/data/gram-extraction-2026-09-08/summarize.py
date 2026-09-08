"""Summarize all three variants without mixing RSS and timing runs."""
import argparse
import json
from pathlib import Path
import statistics

p = argparse.ArgumentParser()
p.add_argument("input", type=Path)
p.add_argument("output", type=Path)
a = p.parse_args()
raw = json.loads(a.input.read_text())
runs = raw["runs"]

def stats(values):
    return {"median": statistics.median(values), "min": min(values), "max": max(values)}

result = {"builds_with_index_parity": len(runs), "search_checks": sum(len(r.get("search_parity", [])) for r in runs), "corpora": {}}
for corpus in raw["corpora"]:
    variants = {}
    for version in ("before", "hash_only", "budget_256"):
        rows = [r for r in runs if r["corpus"] == corpus and r["version"] == version]
        timing = [r for r in rows if r["kind"] == "timing"]
        rss = [r for r in rows if r["kind"] == "rss"]
        variants[version] = {
            "timing_samples": len(timing),
            "elapsed_ms": stats([r["elapsed_ms"] for r in timing]),
            "rss_samples": len(rss),
            "rss_mib": stats([r["rss_bytes"] / 2**20 for r in rss]),
            "spilled_runs": sorted({r["spilled_runs"] for r in rows}),
        }
    for version in ("hash_only", "budget_256"):
        for baseline in ("before", "hash_only"):
            if version == baseline:
                continue
            before = {r["repeat"]: r["elapsed_ms"] for r in runs if r["corpus"] == corpus and r["version"] == baseline and r["kind"] == "timing"}
            after = {r["repeat"]: r["elapsed_ms"] for r in runs if r["corpus"] == corpus and r["version"] == version and r["kind"] == "timing"}
            variants[version][f"versus_{baseline}"] = {
                "median_change_percent": 100 * (variants[version]["elapsed_ms"]["median"] / variants[baseline]["elapsed_ms"]["median"] - 1),
                "paired_change_percent": stats([100 * (after[i] / before[i] - 1) for i in before]),
            }
    result["corpora"][corpus] = variants
a.output.write_text(json.dumps(result, indent=2) + "\n")
for name, variants in result["corpora"].items():
    print(name, {v: {"ms": round(r["elapsed_ms"]["median"], 2), "rss_mib": round(r["rss_mib"]["median"], 2)} for v, r in variants.items()})
    print("combined change %", round(variants["budget_256"]["versus_before"]["median_change_percent"], 2), "; vs hash-only %", round(variants["budget_256"]["versus_hash_only"]["median_change_percent"], 2))
