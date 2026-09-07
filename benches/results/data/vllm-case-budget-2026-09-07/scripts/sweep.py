import datetime
import json
import random
import statistics
import subprocess
import sys
from common import *

VARIANTS = [4, 8, 16, 32, 64, 128]
KEYS = [8, 16, 32, 64, 128, 256, 512]
ITERATIONS = 7
WARMUP = 2
CONFIGS = [(v, k) for v in VARIANTS for k in KEYS]
PHASE = 'coarse'
if sys.argv[1:] == ['--fine']:
    PHASE = 'fine'
    CONFIGS = ([(v, k) for v in [24, 48, 96] for k in [64, 96, 128, 192, 256]]
               + [(v, k) for v in [32, 64, 128] for k in [96, 192]]
               + [(32, 64), (32, 128), (64, 128), (64, 256)])
    VARIANTS = sorted({v for v, k in CONFIGS})
    KEYS = sorted({k for v, k in CONFIGS})
elif sys.argv[1:]:
    raise SystemExit('Usage: sweep.py [--fine]')
TRAIN = [q for q in QUERIES if q['split'] == 'train']
HARNESS = OUT / 'binaries/coderg-harness'
rng = random.Random(2026090701)
assert not subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=ROOT).strip()
before = index_snapshot()
save(OUT / 'index-before.json', before)
save(OUT / f'{PHASE}-protocol.json', dict(
    recorded_at=datetime.datetime.now().astimezone().isoformat(),
    corpus_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
    configs=CONFIGS, variants=VARIANTS, keys=KEYS, queries=QUERIES, warmup=WARMUP, iterations=ITERATIONS,
    primary_objective='Equal-weight arithmetic mean of per-query median no-refresh latency on training queries',
    confirmation='Real constant binaries; default and no-refresh; heldout queries; three case-sensitive controls; peak RSS',
    coupled_cap='Key limit also caps the number of planned strategy groups, exactly as in current source',
    output='Matching filename:line:content; stdout to /dev/null; rg sorted only for correctness',
    harness_only_env=['CODERG_BENCH_VARIANTS', 'CODERG_BENCH_KEYS', 'CODERG_BENCH_TRACE'],
))
expected = expectations()
rows = []
for qi, query in enumerate(TRAIN):
    path = OUT / f'{PHASE}-{query["id"]}.json'
    if path.exists():
        rows.append(json.loads(path.read_text()))
        continue
    print(f'[{qi+1}/{len(TRAIN)}] verifying and warming {query["id"]}', flush=True)
    samples = {config_name(v, k): [] for v, k in CONFIGS}
    diagnostics = {}
    order = CONFIGS.copy()
    rng.shuffle(order)
    for v, k in order:
        name = config_name(v, k)
        actual, trace = output_hash(argv(query, HARNESS), env=child_env(v, k, trace=True))
        assert actual == expected[query['id']], (query, name, actual, expected[query['id']])
        assert trace['keys_loaded'] <= k, (name, trace)
        diagnostics[name] = trace
        for _ in range(WARMUP):
            execute(argv(query, HARNESS), env=child_env(v, k))
    for iteration in range(ITERATIONS):
        rng.shuffle(order)
        for v, k in order:
            elapsed, _ = execute(argv(query, HARNESS), env=child_env(v, k))
            samples[config_name(v, k)].append(elapsed)
    row = dict(query=query, expected=expected[query['id']],
               configs={name: dict(**summarize(values), **diagnostics[name]) for name, values in samples.items()})
    save(path, row)
    rows.append(row)
    leaders = sorted(row['configs'], key=lambda c: row['configs'][c]['median_ms'])[:3]
    print('  ' + ', '.join(f'{c}: {row["configs"][c]["median_ms"]:.2f} ms' for c in leaders), flush=True)

ranking = []
for v, k in CONFIGS:
    name = config_name(v, k)
    ranking.append(dict(config=name, variants=v, keys=k,
        avg_median_ms=statistics.mean(row['configs'][name]['median_ms'] for row in rows),
        avg_p95_ms=statistics.mean(row['configs'][name]['p95_ms'] for row in rows),
        avg_keys_loaded=statistics.mean(row['configs'][name]['keys_loaded'] for row in rows),
        avg_candidate_files=statistics.mean(row['configs'][name]['candidate_files'] for row in rows),
        full_scan_queries=sum(row['configs'][name]['full_scan'] for row in rows)))
ranking.sort(key=lambda row: row['avg_median_ms'])
assert index_snapshot() == before, 'Index files changed'
save(OUT / f'{PHASE}-results.json', dict(ranking=ranking, queries=rows))
print(json.dumps(ranking[:12], indent=2), flush=True)
