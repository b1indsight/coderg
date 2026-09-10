"""Run after extracting the pinned Chromium archive. No checkout mutations."""
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = Path('/private/tmp/coderg-chromium-3986304/src')
INDEX = ROOT.parent / 'index'
BIN = HERE.parents[3] / 'target/release/coderg'
ENV = dict(os.environ, LC_ALL='C', RIPGREP_CONFIG_PATH='')
COMMANDS = {
    'coderg_default': [str(BIN), 'search', '-F', 'MAX_FILE_SIZE', '.', '--index-dir', str(INDEX)],
    'coderg_no_refresh': [str(BIN), 'search', '-F', 'MAX_FILE_SIZE', '.', '--index-dir', str(INDEX), '--no-refresh'],
    'rg_hidden': [shutil.which('rg'), '--no-config', '--hidden', '-g', '!.git', '-n', '--no-heading', '--color', 'never', '-F', 'MAX_FILE_SIZE', '.'],
    'rg_default': [shutil.which('rg'), '--no-config', '-n', '--no-heading', '--color', 'never', '-F', 'MAX_FILE_SIZE', '.'],
}

def run(command, capture=False):
    start = time.perf_counter()
    result = subprocess.run(command, cwd=ROOT, env=ENV, stdout=subprocess.PIPE if capture else subprocess.DEVNULL, stderr=subprocess.PIPE)
    if result.returncode != 0:
        raise RuntimeError((command, result.returncode, result.stderr.decode()))
    return (time.perf_counter() - start) * 1000, result

def normalized(output):
    return sorted(line.removeprefix(b'./') for line in output.splitlines())

if __name__ == '__main__':
    data = {'commit': '398630472335c10b9ca610a4d1b7888a040f702a', 'root': str(ROOT),
            'platform': platform.platform(), 'cpus': os.cpu_count(), 'commands': COMMANDS,
            'binary_sha256': hashlib.sha256(BIN.read_bytes()).hexdigest(),
            'rg_version': subprocess.check_output([shutil.which('rg'), '--version']).decode(),
            'warmup': 3, 'iterations': 15, 'seed': 20260909}
    if INDEX.exists():
        raise RuntimeError('Index already exists; use a fresh index for build timing')
    print('Building index', flush=True)
    data['build_ms'], build = run([str(BIN), 'index', '.', '--index-dir', str(INDEX)])
    data['build_stderr'] = build.stderr.decode()
    data['stats'] = run([str(BIN), 'stats', '.', '--index-dir', str(INDEX)], True)[1].stdout.decode()
    print(data['stats'], flush=True)
    outputs = {}
    for name, command in COMMANDS.items():
        outputs[name] = normalized(run(command, True)[1].stdout)
        (HERE / (name + '.txt')).write_bytes(b'\n'.join(outputs[name]) + b'\n')
    data['match_lines'] = {name: len(lines) for name, lines in outputs.items()}
    data['matches_equal'] = {name: lines == outputs['coderg_default'] for name, lines in outputs.items()}
    if not data['matches_equal']['rg_hidden'] or not data['matches_equal']['coderg_no_refresh']:
        (HERE / 'results.json').write_text(json.dumps(data, indent=2))
        raise RuntimeError('Search output mismatch; inspect output files')
    data['match_files'] = len({line.split(b':', 1)[0] for line in outputs['coderg_default']})
    print('Output verified: ' + str(data['match_lines']), flush=True)
    rng = random.Random(data['seed'])
    samples = {name: [] for name in COMMANDS}
    for round_num in range(-data['warmup'], data['iterations']):
        order = list(COMMANDS)
        rng.shuffle(order)
        for name in order:
            elapsed, _ = run(COMMANDS[name])
            if round_num >= 0:
                samples[name].append(elapsed)
        print('Round ' + str(round_num + 1), flush=True)
    data['samples_ms'] = samples
    data['summary_ms'] = {name: {'median': statistics.median(values), 'min': min(values), 'max': max(values)} for name, values in samples.items()}
    for name, command in COMMANDS.items():
        assert normalized(run(command, True)[1].stdout) == outputs[name]
    (HERE / 'results.json').write_text(json.dumps(data, indent=2) + '\n')
    print(json.dumps(data['summary_ms'], indent=2), flush=True)
