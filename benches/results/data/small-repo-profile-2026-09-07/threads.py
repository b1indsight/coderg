import json
import os
import pathlib
import random
import statistics
import subprocess
import time

work = pathlib.Path(__file__).parent
base = json.loads((work / 'results.json').read_text())
root = base['root']
commands = {mode: base['commands'][mode] for mode in ('control', 'profile')}
jobs = [(threads, mode) for threads in (1, 2, 4, 10) for mode in commands]
samples = {f'{threads}/{mode}': [] for threads, mode in jobs}

def run(threads, mode):
    env = dict(os.environ, LC_ALL='C', RAYON_NUM_THREADS=str(threads))
    start = time.perf_counter_ns()
    proc = subprocess.run(commands[mode], cwd=root, env=env, stdout=subprocess.DEVNULL,
                          stderr=subprocess.PIPE, check=True)
    result = {'wall_ms': (time.perf_counter_ns() - start) / 1e6}
    if mode == 'profile':
        profile = json.loads(proc.stderr.decode().removeprefix('CODERG_PROFILE '))
        result['phases_ms'] = {event['name']: event['duration_ns'] / 1e6
                              for event in profile['events']}
    else:
        assert not proc.stderr
    return result

for _ in range(5):
    for threads, mode in jobs:
        run(threads, mode)
rng = random.Random(20260908)
for round_id in range(51):
    order = jobs.copy()
    rng.shuffle(order)
    for threads, mode in order:
        value = run(threads, mode)
        value['round'] = round_id
        samples[f'{threads}/{mode}'].append(value)

summary = {}
for key, values in samples.items():
    summary[key] = {'wall_ms': statistics.median(v['wall_ms'] for v in values)}
    if 'phases_ms' in values[0]:
        summary[key]['phases_ms'] = {
            name: statistics.median(v['phases_ms'][name] for v in values)
            for name in values[0]['phases_ms']}
result = {'method': '51 randomized rounds, 5 warmups, same binaries/index/query as results.json; warm filesystem cache; new CLI per run',
          'samples': samples, 'summary': summary}
(work / 'threads.json').write_text(json.dumps(result, indent=2) + '\n')
for threads in (1, 2, 4, 10):
    phases = summary[f'{threads}/profile']['phases_ms']
    print(json.dumps({'threads': threads, 'control_wall_ms': summary[f'{threads}/control']['wall_ms'],
                      'profile_main_ms': phases['main.run'], 'walk_ms': phases['collect.walk_metadata'],
                      'collect_total_ms': phases['collect.total'],
                      'read_match_ms': phases['search.read_and_match']}))
