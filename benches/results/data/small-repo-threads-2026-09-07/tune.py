import hashlib
import json
import math
import os
import pathlib
import random
import statistics
import subprocess

work = pathlib.Path(__file__).parent
binary = work / 'tune-target/release/coderg'
roots = json.loads((work / 'tune-plan.json').read_text())
jobs = [(name, mode) for name in roots for mode in ('serial', 'parallel1', 'parallel')]
samples = {name: {mode: [] for mode in ('serial', 'parallel1', 'parallel')} for name in roots}
counts = {}

def run(name, mode):
    env = dict(os.environ, CODERG_BENCH_WALK_MODE=mode)
    env.pop('RAYON_NUM_THREADS', None)
    if mode == 'parallel1':
        env['RAYON_NUM_THREADS'] = '1'
    p = subprocess.run([str(binary), roots[name]], env=env, capture_output=True, check=True)
    assert not p.stderr, p.stderr
    data = json.loads(p.stdout)
    if name in counts:
        assert counts[name] == data['files']
    counts[name] = data['files']
    return data['ms']

for _ in range(5):
    for job in jobs:
        run(*job)
rng = random.Random(20260909)
for _ in range(51):
    order = jobs.copy()
    rng.shuffle(order)
    for name, mode in order:
        samples[name][mode].append(run(name, mode))

summary = {name: {'files': counts[name], **{
    mode: {'median': statistics.median(values), 'p95': sorted(values)[math.ceil(len(values)*.95)-1]}
    for mode, values in modes.items()}} for name, modes in samples.items()}
result = {'method': 'direct collect_files wall-clock timing, new process, hot filesystem cache, 5 warmups and 51 randomized rounds, serial versus one-worker parallel versus default parallel',
          'roots': roots, 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
          'samples': samples, 'summary': summary}
(work / 'tuning.json').write_text(json.dumps(result, indent=2) + '\n')
for name, data in sorted(summary.items(), key=lambda pair: pair[1]['files']):
    print(name, json.dumps(data), flush=True)
