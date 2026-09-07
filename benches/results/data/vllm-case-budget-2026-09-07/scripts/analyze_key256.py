import json
import math
from pathlib import Path
import random
import statistics
from common import save

BASE = Path(__file__).resolve().parent
OUT = BASE / 'key256-addendum'
data = json.loads((OUT / 'results.json').read_text())
rows = data['queries']
CONFIGS = ['v128-k128', 'v128-k256']
rng = random.Random(2026090705)

def p95(samples):
    return sorted(samples)[math.ceil(len(samples) * .95) - 1]

summary = {}
for mode in ['default', 'norefresh']:
    result = {}
    for config in CONFIGS:
        name = f'{config}/{mode}'
        result[config] = dict(
            avg_median_ms=statistics.mean(row['variants'][name]['median_ms'] for row in rows),
            avg_p95_ms=statistics.mean(row['variants'][name]['p95_ms'] for row in rows),
            mixed_workload_p95_ms=p95([s for row in rows for s in row['variants'][name]['samples_ms']]),
        )
    differences = {}
    for metric in ['avg_median_ms', 'avg_p95_ms', 'mixed_workload_p95_ms']:
        baseline, candidate = (result[c][metric] for c in CONFIGS)
        differences[metric] = dict(delta_ms=candidate - baseline, slower_pct=100 * (candidate - baseline) / baseline)
    draws = {'avg_median_ms': [], 'avg_p95_ms': []}
    for _ in range(2000):
        totals = {'avg_median_ms': [0., 0.], 'avg_p95_ms': [0., 0.]}
        for row in rows:
            arrays = [row['variants'][f'{c}/{mode}']['samples_ms'] for c in CONFIGS]
            indexes = [rng.randrange(len(arrays[0])) for _ in arrays[0]]
            for ci, array in enumerate(arrays):
                sample = sorted(array[i] for i in indexes)
                totals['avg_median_ms'][ci] += statistics.median(sample)
                totals['avg_p95_ms'][ci] += sample[math.ceil(len(sample) * .95) - 1]
        for metric, values in totals.items():
            draws[metric].append(100 * (values[1] - values[0]) / values[0])
    for metric, values in draws.items():
        values.sort()
        differences[metric]['slower_pct_bootstrap_95pct_ci'] = [values[49], values[1949]]
    result['difference_256_minus_128'] = differences
    summary[mode] = result

summary['diagnostics'] = {c: dict(
    avg_keys_loaded=statistics.mean(row['trace'][c]['keys_loaded'] for row in rows),
    avg_candidate_files=statistics.mean(row['trace'][c]['candidate_files'] for row in rows),
    max_keys_loaded=max(row['trace'][c]['keys_loaded'] for row in rows),
) for c in CONFIGS}
save(OUT / 'summary.json', summary)
print(json.dumps(summary, indent=2), flush=True)

lines = ['# 128/128 versus 128/256: p95 follow-up', '',
    '14 case-insensitive vLLM queries, 5 warmups and 101 randomized interleaved timings per configuration / mode / query. Query order was also randomized. Both are real release binaries; the only source change is MAX_INDEX_LOOKUPS from 128 to 256. Normal matching-line output goes to /dev/null; warm filesystem caches.', '',
    'All 56 complete output comparisons (14 queries × 2 configurations × 2 modes) matched the prior rg oracle. All index files retained the same SHA-256. Trace runs are separate from timed runs.', '',
    'The main metric retains the previous report\'s definition: equal-weight mean of per-query median or per-query p95. Per-query p95 uses the nearest-rank rule, rank 96 of 101. The mixed-workload p95 is also recorded separately, assigning every query equal frequency.', '',
    'Positive differences mean 128/256 is slower. Bootstrap intervals use 2,000 paired-round resamples within each query; they describe measurement noise for this fixed workload, not generalization to other workloads.', '',
    '| Mode | Metric | 128/128 ms | 128/256 ms | Difference ms | Slower % |',
    '|---|---|---:|---:|---:|---:|']
for mode in ['default', 'norefresh']:
    for metric in ['avg_median_ms', 'avg_p95_ms', 'mixed_workload_p95_ms']:
        r = summary[mode]
        d = r['difference_256_minus_128'][metric]
        lines.append(f'| {mode} | {metric} | {r[CONFIGS[0]][metric]:.3f} | {r[CONFIGS[1]][metric]:.3f} | {d["delta_ms"]:+.3f} | {d["slower_pct"]:+.2f}% |')
lines += ['', '95% intervals for relative slowdown:', '']
for mode in ['default', 'norefresh']:
    for metric in ['avg_median_ms', 'avg_p95_ms']:
        ci = summary[mode]['difference_256_minus_128'][metric]['slower_pct_bootstrap_95pct_ci']
        lines.append(f'- {mode}, {metric}: [{ci[0]:+.2f}%, {ci[1]:+.2f}%]')

for mode in ['default', 'norefresh']:
    lines += ['', f'## Per-query {mode}', '',
        '| Query | 128/128 p50 | 128/256 p50 | 128/128 p95 | 128/256 p95 | p95 difference ms |',
        '|---|---:|---:|---:|---:|---:|']
    for row in sorted(rows, key=lambda r: r['query']['id']):
        a, b = [row['variants'][f'{c}/{mode}'] for c in CONFIGS]
        lines.append(f'| {row["query"]["id"]} | {a["median_ms"]:.3f} | {b["median_ms"]:.3f} | {a["p95_ms"]:.3f} | {b["p95_ms"]:.3f} | {b["p95_ms"] - a["p95_ms"]:+.3f} |')
lines += ['', '## Index work', '', '| Query | 128/128 keys | 128/256 keys | 128/128 candidates | 128/256 candidates |', '|---|---:|---:|---:|---:|']
for row in sorted(rows, key=lambda r: r['query']['id']):
    a, b = [row['trace'][c] for c in CONFIGS]
    lines.append(f'| {row["query"]["id"]} | {a["keys_loaded"]} | {b["keys_loaded"]} | {a["candidate_files"]} | {b["candidate_files"]} |')
lines += ['', '## Artifacts', '', '- [Protocol and binary hashes](protocol.json)', '- [Raw timings and diagnostics](results.json)', '- [Summary and confidence intervals](summary.json)', '- [Build and measurement script](../key256_addendum.py)', '- [Analysis script](../analyze_key256.py)', '']
(OUT / 'report.md').write_text('\n'.join(lines))
