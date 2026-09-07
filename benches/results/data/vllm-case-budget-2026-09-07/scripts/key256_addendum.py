import argparse
import datetime
import hashlib
import json
from pathlib import Path
import random
import shutil
import statistics
import subprocess
from common import QUERIES, ROOT, INDEX, argv, child_env, execute, output_hash, expectations, index_snapshot, save, summarize

BASE = Path(__file__).resolve().parent
OUT = BASE / 'key256-addendum'
REPO = Path('/Users/b1indsight/personal_work/coderg')
TARGET = BASE / 'target'
TRIAL = OUT / 'trial'
ITERATIONS = 101
WARMUP = 5
SELECTED = [q for q in QUERIES if q['split'] != 'control']
CONFIGS = ['v128-k128', 'v128-k256']
parser = argparse.ArgumentParser()
parser.add_argument('phase', choices=['build', 'bench'])
args = parser.parse_args()
OUT.mkdir(exist_ok=True)

if args.phase == 'build':
    TRIAL.mkdir()
    (OUT / 'binaries').mkdir()
    for name in ['Cargo.toml', 'Cargo.lock']:
        shutil.copy2(REPO / name, TRIAL / name)
    for name in ['src', 'benches']:
        shutil.copytree(REPO / name, TRIAL / name)
    previous = json.loads((BASE / 'final-validation.json').read_text())
    current_binary = REPO / 'target/release/coderg'
    assert hashlib.sha256(current_binary.read_bytes()).hexdigest() == previous['sha256']
    shutil.copy2(current_binary, OUT / 'binaries/coderg-v128-k128')
    query_path = TRIAL / 'src/query.rs'
    query = query_path.read_text()
    assert 'pub const MAX_LITERAL_VARIANTS: usize = 128;' in query
    assert 'pub const MAX_INDEX_LOOKUPS: usize = 128;' in query
    query_path.write_text(query.replace('pub const MAX_INDEX_LOOKUPS: usize = 128;', 'pub const MAX_INDEX_LOOKUPS: usize = 256;', 1))
    subprocess.run(['cargo', 'build', '--release', '--bin', 'coderg', '--locked', '--offline', '--target-dir', str(TARGET)], cwd=TRIAL, check=True)
    shutil.copy2(TARGET / 'release/coderg', OUT / 'binaries/coderg-v128-k256')
    print('Prepared isolated 128/256 binary and archived the current 128/128 release.', flush=True)
    raise SystemExit(0)

assert not subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=ROOT).strip()
before = json.loads((BASE / 'index-before.json').read_text())
assert index_snapshot() == before
expected = expectations()
rng = random.Random(2026090704)
protocol = dict(recorded_at=datetime.datetime.now().astimezone().isoformat(),
    iterations=ITERATIONS, warmup=WARMUP, queries=SELECTED, configs=CONFIGS,
    seed=2026090704, mode='Warm filesystem caches; default matching-line output to /dev/null',
    metric='Mean of per-query medians and per-query nearest-rank p95; also report equal-weight mixed-workload p95 separately',
    p95='Sample at rank ceil(0.95 * n), 1-based; 96th of 101 samples',
    binaries={name: hashlib.sha256((OUT / f'binaries/coderg-{name}').read_bytes()).hexdigest() for name in CONFIGS})
save(OUT / 'protocol.json', protocol)
query_order = SELECTED.copy()
rng.shuffle(query_order)
rows = []
for qi, query in enumerate(query_order):
    path = OUT / f'query-{query["id"]}.json'
    if path.exists():
        rows.append(json.loads(path.read_text()))
        continue
    print(f'[{qi + 1}/{len(query_order)}] {query["id"]}', flush=True)
    variants = {}
    traces = {}
    for name in CONFIGS:
        binary = OUT / f'binaries/coderg-{name}'
        for mode in ['default', 'norefresh']:
            variant = f'{name}/{mode}'
            variants[variant] = argv(query, binary, mode)
            actual, _ = output_hash(variants[variant])
            assert actual == expected[query['id']], (variant, query['id'], actual)
        key_limit = 128 if name == 'v128-k128' else 256
        actual, trace = output_hash(argv(query, BASE / 'binaries/coderg-harness'), env=child_env(128, key_limit, trace=True))
        assert actual == expected[query['id']]
        assert trace['keys_loaded'] <= key_limit
        traces[name] = trace
    samples = {variant: [] for variant in variants}
    order = list(variants)
    for _ in range(WARMUP):
        rng.shuffle(order)
        for variant in order:
            execute(variants[variant])
    for iteration in range(ITERATIONS):
        rng.shuffle(order)
        for variant in order:
            elapsed, _ = execute(variants[variant])
            samples[variant].append(elapsed)
    row = dict(query=query, trace=traces, variants={name: summarize(values) for name, values in samples.items()}, output_verified=True)
    save(path, row)
    rows.append(row)
    print('  ' + ', '.join(f'{name}: p50 {values["median_ms"]:.2f}, p95 {values["p95_ms"]:.2f} ms' for name, values in row['variants'].items()), flush=True)

assert index_snapshot() == before
save(OUT / 'results.json', dict(protocol=protocol, queries=rows, index_unchanged=True))
for mode in ['default', 'norefresh']:
    print(mode, flush=True)
    for config in CONFIGS:
        name = f'{config}/{mode}'
        median = statistics.mean(row['variants'][name]['median_ms'] for row in rows)
        p95 = statistics.mean(row['variants'][name]['p95_ms'] for row in rows)
        pooled = sorted(value for row in rows for value in row['variants'][name]['samples_ms'])
        pooled_p95 = pooled[__import__('math').ceil(len(pooled) * .95) - 1]
        print(f'  {config}: median mean {median:.4f} ms, p95 mean {p95:.4f} ms; pooled p95 {pooled_p95:.4f} ms', flush=True)
