"""Summarize medians without adding overlapping parent/worker timings."""
import argparse
import json
from pathlib import Path
import statistics

p = argparse.ArgumentParser()
p.add_argument("inputs", nargs="+", type=Path)
p.add_argument("--output", type=Path, required=True)
a = p.parse_args()
runs = [run for path in a.inputs for run in json.loads(path.read_text())["runs"]]

def stats(values):
    return {"median": statistics.median(values), "min": min(values), "max": max(values)}

result = {"builds_with_byte_parity": len(runs), "corpora": {}}
for corpus in sorted({r["corpus"] for r in runs}):
    rows = [r for r in runs if r["corpus"] == corpus and not r["warmup"]]
    entry = {"source": rows[0]["source"], "files": rows[0]["files"], "source_bytes": rows[0]["source_bytes"], "versions": {}}
    for version in ("control", "probe"):
        selected = [r for r in rows if r["version"] == version]
        v = {"n": len(selected)}
        for key in ("wall_ms", "user_cpu_ms", "system_cpu_ms"):
            v[key] = stats([r[key] for r in selected])
        v["average_cpu_cores"] = stats([(r["user_cpu_ms"] + r["system_cpu_ms"]) / r["wall_ms"] for r in selected])
        if version == "probe":
            v["phases"] = {}
            for key, value in selected[0]["profile"].items():
                if isinstance(value, dict):
                    v["phases"][key] = {"ms": stats([r["profile"][key]["ms"] for r in selected]), "calls": stats([r["profile"][key]["calls"] for r in selected])}
                else:
                    v["phases"][key] = stats([r["profile"][key] for r in selected])
            v["phase_share_of_build"] = {k: stats([r["profile"][k]["ms"] / r["profile"]["total"]["ms"] for r in selected]) for k in ("pipeline", "receive_wait", "final_write", "sort", "dedup", "merge", "encode")}
            v["collector_wait_share_of_pipeline"] = stats([r["profile"]["receive_wait"]["ms"] / r["profile"]["pipeline"]["ms"] for r in selected])
            v["duplicate_fraction_at_sort"] = stats([1 - r["profile"]["sort_output"] / r["profile"]["sort_input"] for r in selected])
        entry["versions"][version] = v
    entry["probe_wall_delta_percent"] = 100 * (entry["versions"]["probe"]["wall_ms"]["median"] / entry["versions"]["control"]["wall_ms"]["median"] - 1)
    result["corpora"][corpus] = entry
a.output.write_text(json.dumps(result, indent=2) + "\n")
for name, row in result["corpora"].items():
    p = row["versions"]["probe"]["phases"]
    print(name, {v: round(row["versions"][v]["wall_ms"]["median"], 2) for v in ("control", "probe")})
    print({k: round(v["ms"]["median"], 2) if "ms" in v else v["median"] for k, v in p.items()})
