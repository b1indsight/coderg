import argparse
import datetime
import json
import random
import shutil
import statistics
import subprocess
from common import *

ITERATIONS = 31
WARMUP = 3
MEMORY_RUNS = 3
parser = argparse.ArgumentParser()
parser.add_argument('phase', choices=['build', 'bench'])
args = parser.parse_args()
finalists = json.loads((OUT / 'finalists.json').read_text())
TARGET = OUT / 'target'
TRIAL = OUT / 'trial'
ORIGINAL = OUT / 'original'

if args.phase == 'build':
    for config in finalists:
        name = config_name(config['variants'], config['keys'])
        binary = OUT / f'binaries/coderg-{name}'
        if binary.exists():
            continue
        for p in (ORIGINAL / 'src').glob('*.rs'):
            shutil.copy2(p, TRIAL / 'src' / p.name)
        query = (TRIAL / 'src/query.rs').read_text()
        query = query.replace('pub const MAX_LITERAL_VARIANTS: usize = 64;', f'pub const MAX_LITERAL_VARIANTS: usize = {config["variants"]};')
        query = query.replace('pub const MAX_INDEX_LOOKUPS: usize = 256;', f'pub const MAX_INDEX_LOOKUPS: usize = {config["keys"]};')
        (TRIAL / 'src/query.rs').write_text(query)
        subprocess.run(['cargo', 'build', '--release', '--bin', 'coderg', '--locked', '--offline', '--target-dir', str(TARGET)], cwd=TRIAL, check=True)
        shutil.copy2(TARGET / 'release/coderg', binary)
        print(f'Built real constant binary: {name}', flush=True)
    raise SystemExit(0)

rng = random.Random(2026090702)
expected = expectations()
before = json.loads((OUT / 'index-before.json').read_text())
assert index_snapshot() == before
save(OUT / 'confirmation-protocol.json', dict(
    recorded_at=datetime.datetime.now().astimezone().isoformat(),
    finalists=finalists, queries=QUERIES,
    iterations=ITERATIONS, warmup=WARMUP, memory_runs=MEMORY_RUNS,
    binary_sha256={config_name(c['variants'], c['keys']): hashlib.sha256((OUT / f'binaries/coderg-{config_name(c["variants"], c["keys"])}').read_bytes()).hexdigest() for c in finalists},
))
rows = []
for qi, query in enumerate(QUERIES):
    path = OUT / f'confirm-{query["id"]}.json'
    if path.exists():
        rows.append(json.loads(path.read_text()))
        continue
    print(f'[{qi+1}/{len(QUERIES)}] {query["split"]}: {query["id"]}', flush=True)
    variants = {}
    traces = {}
    for config in finalists:
        name = config_name(config['variants'], config['keys'])
        binary = OUT / f'binaries/coderg-{name}'
        for mode in ['default', 'norefresh']:
            variant = f'{name}/{mode}'
            variants[variant] = argv(query, binary, mode)
            actual, _ = output_hash(variants[variant])
            assert actual == expected[query['id']], (query, variant, actual, expected[query['id']])
        actual, trace = output_hash(argv(query, OUT / 'binaries/coderg-harness'), child_env(config['variants'], config['keys'], trace=True))
        assert actual == expected[query['id']]
        traces[name] = trace
    variants['rg'] = argv(query)
    samples = {name: [] for name in variants}
    order = list(variants)
    for _ in range(WARMUP):
        rng.shuffle(order)
        for name in order:
            execute(variants[name])
    for iteration in range(ITERATIONS):
        rng.shuffle(order)
        for name in order:
            elapsed, _ = execute(variants[name])
            samples[name].append(elapsed)
    rss = {}
    order = [name for name in variants if name.endswith('/default') or name == 'rg']
    for repetition in range(MEMORY_RUNS):
        rng.shuffle(order)
        for name in order:
            label = f'rss-{query["id"]}-{name.replace("/", "-")}-{repetition}'
            rss.setdefault(name, []).append(memory(variants[name], label))
    row = dict(query=query, expected=expected[query['id']], trace=traces,
               variants={name: summarize(values) for name, values in samples.items()}, memory=rss)
    save(path, row)
    rows.append(row)
    print('  ' + ', '.join(f'{name}: {row["variants"][name]["median_ms"]:.2f} ms' for name in variants if name.endswith('/default')), flush=True)

assert index_snapshot() == before, 'Index files changed'
save(OUT / 'confirmation-results.json', dict(finalists=finalists, queries=rows, index_unchanged=True))
for group in ['train', 'holdout', 'control']:
    selected = [row for row in rows if row['query']['split'] == group]
    print(f'{group}:', flush=True)
    for name in rows[0]['variants']:
        avg = statistics.mean(row['variants'][name]['median_ms'] for row in selected)
        print(f'  {name}: {avg:.3f} ms', flush=True)
