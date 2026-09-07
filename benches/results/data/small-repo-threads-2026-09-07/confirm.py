import json
import os
import pathlib
import random
import statistics
import subprocess
import time

work = pathlib.Path(__file__).parent
cases = [('regular_4096', 'rare_files'), ('vllm', 'todo_comment'), ('many_small_16384', 'ignore_case')]
env = dict(os.environ)
env.pop('RAYON_NUM_THREADS', None)
env.pop('RIPGREP_CONFIG_PATH', None)
jobs = []
samples = {}

for dataset, query_id in cases:
    data = json.loads((work / 'matrix' / dataset / 'results.json').read_text())
    root = data['dataset']['root']
    index = work / f'confirm-{dataset}-index'
    subprocess.run([str(work / 'after'), 'index', root, '--index-dir', str(index)],
                   cwd=root, env=env, capture_output=True, check=True)
    commands = {}
    outputs = []
    for mode in ('baseline', 'optimized'):
        cmd = data['commands'][query_id][mode].copy()
        cmd[cmd.index('--index-dir') + 1] = str(index)
        commands[mode] = cmd
        proc = subprocess.run(cmd, cwd=root, env=env, capture_output=True, check=True)
        assert not proc.stderr
        outputs.append(proc.stdout)
        jobs.append((dataset, query_id, mode, root, cmd))
    assert outputs[0] == outputs[1]
    samples[dataset + '/' + query_id] = {mode: [] for mode in commands}

def run(root, cmd):
    started = time.perf_counter_ns()
    proc = subprocess.run(cmd, cwd=root, env=env, stdout=subprocess.DEVNULL,
                          stderr=subprocess.PIPE, check=True)
    elapsed = (time.perf_counter_ns() - started) / 1e6
    assert not proc.stderr
    return elapsed

for _ in range(5):
    for _, _, _, root, cmd in jobs:
        run(root, cmd)
rng = random.Random(20260911)
for _ in range(101):
    order = jobs.copy()
    rng.shuffle(order)
    for dataset, query_id, mode, root, cmd in order:
        samples[dataset + '/' + query_id][mode].append(run(root, cmd))

summary = {}
for key, modes in samples.items():
    before = statistics.median(modes['baseline'])
    after = statistics.median(modes['optimized'])
    changes = []
    for _ in range(2000):
        indices = rng.choices(range(101), k=101)
        b = statistics.median(modes['baseline'][i] for i in indices)
        a = statistics.median(modes['optimized'][i] for i in indices)
        changes.append((a / b - 1) * 100)
    changes.sort()
    summary[key] = {'before_ms': before, 'after_ms': after, 'change_pct': (after/before-1)*100,
                    'bootstrap_95': [changes[50], changes[1949]]}
(work / 'confirmation.json').write_text(json.dumps({'method': '5 warmups, 101 randomized rounds, same binaries and hot-cache fresh-process execution as main matrix; paired bootstrap 2000',
                                                    'samples': samples, 'summary': summary}, indent=2) + '\n')
print(json.dumps(summary, indent=2))
