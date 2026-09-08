"""Compare equivalent phase work, including legacy set-to-Vec conversion."""
import argparse
import json
from pathlib import Path
import statistics

p = argparse.ArgumentParser()
p.add_argument("input", type=Path)
p.add_argument("output", type=Path)
a = p.parse_args()
runs = json.loads(a.input.read_text())["runs"]
result = {"builds_with_byte_parity": len(runs), "corpora": {}}

def stats(values):
    return {"median_ms": statistics.median(values), "min_ms": min(values), "max_ms": max(values)}

for corpus in sorted({r["corpus"] for r in runs}):
    result["corpora"][corpus] = {}
    for version, label in (("control", "before"), ("probe", "after")):
        rows = [r for r in runs if r["corpus"] == corpus and r["version"] == version and not r["warmup"]]
        phases = {k: stats([r["profile"][k]["ms"] for r in rows]) for k, v in rows[0]["profile"].items() if isinstance(v, dict)}
        phases["gram_and_output"] = stats([r["profile"]["gram_hash"]["ms"] + r["profile"]["chunk_collect"]["ms"] for r in rows])
        result["corpora"][corpus][label] = {"n": len(rows), "wall_median_ms": statistics.median(r["wall_ms"] for r in rows), "phases": phases, "counters": {k: sorted({r["profile"][k] for r in rows}) for k, v in rows[0]["profile"].items() if not isinstance(v, dict)}}
a.output.write_text(json.dumps(result, indent=2) + "\n")
