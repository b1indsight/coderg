"""Summarize the frozen matrix without mixing unlike query sets or samples."""
from pathlib import Path
import csv
import json
import random
import statistics

work = Path(__file__).resolve().parent
root = work / "results"
metadata = json.loads((root / "metadata.json").read_text())
rng = random.Random(90210)


def interval(rows, repeats=1000):
    count = len(rows[0][0])
    changes = []
    for _ in range(repeats):
        indexes = [rng.randrange(count) for _ in range(count)]
        baseline = statistics.mean(statistics.median([left[i] for i in indexes]) for left, _ in rows)
        optimized = statistics.mean(statistics.median([right[i] for i in indexes]) for _, right in rows)
        changes.append(100 * (optimized / baseline - 1))
    changes.sort()
    return [changes[24], changes[974]]


summary = dict(bootstrap_repeats=1000, bootstrap_seed=90210, datasets={}, regressions=[], queries=[])
flat = []
for dataset in metadata["plan"]["datasets"]:
    name = dataset["id"]
    result = json.loads((root / name / "results.json").read_text())
    assert all(result["validation"].values())
    queries = result["queries"]
    compatible = [q for q in queries if q["rg_compatible"]]
    data = dict(kind=dataset["kind"], queries=len(queries), rg_compatible=len(compatible),
                rg_differences=[q["id"] for q in queries if not q["rg_compatible"]],
                matching_queries=sum(q["parity"]["baseline"]["exit_code"] == 0 for q in queries),
                files=result["builds"]["baseline"]["files"],
                source_mib=result["builds"]["baseline"]["source_bytes"] / 1048576,
                index_mib=result["builds"]["baseline"]["index_bytes"] / 1048576,
                builds={version: dict(median_ms=b["timing"]["median"],
                        rss_median_mib=b["rss"]["median"] / 1048576,
                        rss_max_mib=max(b["rss"]["samples"]) / 1048576)
                        for version, b in result["builds"].items()},
                aggregate={})
    for scope, selected in [("all", queries), ("rg_compatible", compatible)]:
        modes = list(selected[0]["timings_ms"])
        rows = [(q["timings_ms"]["baseline"]["samples"], q["timings_ms"]["optimized"]["samples"]) for q in selected]
        averages = {mode: statistics.mean(q["timings_ms"][mode]["median"] for q in selected) for mode in modes}
        data["aggregate"][scope] = dict(
            medians_mean_ms=averages,
            p95_mean_ms={mode: statistics.mean(q["timings_ms"][mode]["p95"] for q in selected) for mode in modes},
            median_peak_rss_mean_mib={mode: statistics.mean(q["rss_bytes"][mode]["median"] / 1048576 for q in selected) for mode in modes},
            max_peak_rss_mib={mode: max(max(q["rss_bytes"][mode]["samples"]) for q in selected) / 1048576 for mode in modes},
            optimized_change_pct=100 * (averages["optimized"] / averages["baseline"] - 1),
            optimized_change_bootstrap_95=interval(rows),
            faster_than_baseline=sum(q["timings_ms"]["optimized"]["median"] < q["timings_ms"]["baseline"]["median"] for q in selected),
            faster_than_rg=sum(q["timings_ms"]["optimized"]["median"] < q["timings_ms"]["rg"]["median"] for q in selected),
            baseline_faster_than_rg=sum(q["timings_ms"]["baseline"]["median"] < q["timings_ms"]["rg"]["median"] for q in selected))
    for query in queries:
        t = query["timings_ms"]
        row = dict(dataset=name, query=query["id"], category=query["category"],
                   rg_compatible=query["rg_compatible"], matching_lines=query["parity"]["baseline"]["lines"],
                   **{mode + "_ms": value["median"] for mode, value in t.items()},
                   baseline_p95_ms=t["baseline"]["p95"], optimized_p95_ms=t["optimized"]["p95"],
                   change_pct=100 * (t["optimized"]["median"] / t["baseline"]["median"] - 1),
                   change_bootstrap_95=interval([(t["baseline"]["samples"], t["optimized"]["samples"])]),
                   **{mode + "_rss_mib": value["median"] / 1048576 for mode, value in query["rss_bytes"].items()})
        flat.append(row)
        if row["change_pct"] > 0:
            summary["regressions"].append(row)
    summary["datasets"][name] = data
summary["queries"] = flat
summary["regressions"].sort(key=lambda q: -q["change_pct"])
(work / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
with (work / "per-query.csv").open("w", newline="") as file:
    writer = csv.DictWriter(file, fieldnames=list(flat[0]))
    writer.writeheader()
    writer.writerows(flat)
for name, data in summary["datasets"].items():
    a = data["aggregate"]["rg_compatible"]
    medians = a["medians_mean_ms"]
    print(name, data["files"], round(data["source_mib"], 2),
          {mode: round(value, 3) for mode, value in medians.items()},
          round(a["optimized_change_pct"], 2), a["optimized_change_bootstrap_95"],
          f"wins baseline {a['faster_than_baseline']}/{data['rg_compatible']}, rg {a['faster_than_rg']}/{data['rg_compatible']}")
print('Regressions:',[(q['dataset'],q['query'],round(q['change_pct'],2),q['change_bootstrap_95']) for q in summary['regressions']])
