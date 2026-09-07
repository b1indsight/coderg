import hashlib
import json
import math
import os
import pathlib
import random
import statistics
import subprocess
import time

WORK = pathlib.Path(__file__).parent
ROOT = pathlib.Path('/private/tmp/coderg-refresh-wide-79u03ttz/corpora/viberwhisper')
CONTROL = pathlib.Path('/private/tmp/coderg-refresh-wide-79u03ttz/optimized')
PROFILE = WORK / 'target/release/coderg'
INDEX = WORK / 'index'
PATTERN = '^impl'
REPEATS = 51
WARMUP = 5
ENV = dict(os.environ)
ENV.pop('RAYON_NUM_THREADS', None)
ENV['LC_ALL'] = 'C'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, env=ENV).decode().strip()

def invoke(command, capture=False):
    start = time.perf_counter_ns()
    result = subprocess.run(command, cwd=ROOT, env=ENV,
                            stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                            stderr=subprocess.PIPE)
    elapsed = (time.perf_counter_ns() - start) / 1e6
    assert result.returncode == 0, (command, result.returncode, result.stderr.decode())
    profile = None
    for line in result.stderr.decode().splitlines():
        assert line.startswith('CODERG_PROFILE '), line
        profile = json.loads(line.removeprefix('CODERG_PROFILE '))
    return {'wall_ms': elapsed, 'profile': profile}, result.stdout

def commands():
    out = {}
    for name, binary in [('control', CONTROL), ('profile', PROFILE)]:
        out[name] = [str(binary), 'search', PATTERN, str(ROOT), '--index-dir', str(INDEX)]
        out[name + '_no_refresh'] = out[name] + ['--no-refresh']
    out['rg'] = ['rg', '--color', 'never', '--with-filename', '--line-number', PATTERN, '.']
    return out

def summary(values):
    return {'median': statistics.median(values), 'mean': statistics.mean(values),
            'min': min(values), 'max': max(values),
            'p95': sorted(values)[math.ceil(len(values) * .95) - 1]}

before = {'commit': git('rev-parse', 'HEAD'), 'status': git('status', '--porcelain'),
          'source_sha256': {str(p.relative_to(ROOT)): sha(p) for p in ROOT.rglob('*')
                            if p.is_file() and '.git' not in p.relative_to(ROOT).parts}}
assert not before['status']
build = subprocess.run([str(CONTROL), 'index', str(ROOT), '--index-dir', str(INDEX)],
                       cwd=ROOT, env=ENV, capture_output=True, check=True)
index_hashes = {str(p.relative_to(INDEX)): sha(p) for p in INDEX.rglob('*') if p.is_file()}
cmds = commands()
parity = {}
for mode, command in cmds.items():
    result, stdout = invoke(command, capture=True)
    normalized = sorted(line.removeprefix(b'./') for line in stdout.splitlines())
    parity[mode] = {'lines': len(normalized), 'sha256': hashlib.sha256(b'\n'.join(normalized)).hexdigest()}
    if mode == 'profile':
        print('initial profile:', json.dumps(result), flush=True)
assert len({p['sha256'] for p in parity.values()}) == 1, parity
for _ in range(WARMUP):
    for command in cmds.values():
        invoke(command)

rng = random.Random(20260907)
runs = {name: [] for name in cmds}
for round_id in range(REPEATS):
    order = list(cmds)
    rng.shuffle(order)
    for mode in order:
        result, _ = invoke(cmds[mode])
        result['round'] = round_id
        runs[mode].append(result)

after = {'commit': git('rev-parse', 'HEAD'), 'status': git('status', '--porcelain'),
         'source_sha256': {name: sha(ROOT / name) for name in before['source_sha256']}}
assert before == after
assert index_hashes == {str(p.relative_to(INDEX)): sha(p) for p in INDEX.rglob('*') if p.is_file()}

stats = {}
for mode, samples in runs.items():
    phases = {}
    for sample in samples:
        if sample['profile']:
            for event in sample['profile']['events']:
                phases.setdefault(event['name'], []).append(event['duration_ns'] / 1e6)
    stats[mode] = {'wall_ms': summary([s['wall_ms'] for s in samples]),
                   'phases_ms': {name: summary(values) for name, values in phases.items()}}

median_main = stats['profile']['phases_ms']['main.run']['median']
representative = min(runs['profile'], key=lambda sample:
                     abs(next(e['duration_ns'] / 1e6 for e in sample['profile']['events']
                              if e['name'] == 'main.run') - median_main))
result = {'query': PATTERN, 'root': str(ROOT), 'repeats': REPEATS, 'warmup': WARMUP,
          'cache': 'warm filesystem cache; new process for every search',
          'execution': 'workspace sandbox; timing only, no RSS sampling',
          'commands': cmds, 'binaries': {str(p): sha(p) for p in (CONTROL, PROFILE)},
          'system': subprocess.check_output(['sw_vers']).decode(),
          'cpu': 'Apple M5 (from the preceding benchmark environment record)',
          'before': before, 'after': after, 'index_sha256': index_hashes,
          'index_build_stderr': build.stderr.decode(), 'parity': parity,
          'runs': runs, 'summary': stats, 'representative': representative}
(WORK / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
trace = {'traceEvents': [{'name': e['name'], 'cat': 'wall-clock span', 'ph': 'X',
                         'ts': e['start_ns'] / 1e3, 'dur': e['duration_ns'] / 1e3,
                         'pid': 1, 'tid': 1}
                        for e in representative['profile']['events']]}
(WORK / 'trace.json').write_text(json.dumps(trace, indent=2) + '\n')
print(json.dumps({'summary': stats, 'counts': representative['profile']['counts'],
                  'representative': representative}, indent=2))
