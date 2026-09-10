"""Reuse Chromium instrumentation on the vLLM repository snapshot."""
import json
import os
from pathlib import Path
import random
import shutil
import statistics
import subprocess
import tempfile
import time

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
TEMP = Path(tempfile.mkdtemp(prefix='coderg-small-profile-20260909-', dir='/private/tmp'))
ENV = dict(os.environ, LC_ALL='C', RIPGREP_CONFIG_PATH='')
rng = random.Random(20260909)
report = {}
for name, pattern in [('vllm', '^class')]:
    root = Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
    index = TEMP / name
    subprocess.run([str(REPO/'target/release/coderg'), 'index', str(root), '--index-dir', str(index)], env=ENV, check=True, capture_output=True)
    commands = {label: [str(REPO/f'target/{profile}/coderg'), 'search', pattern, '.', '--index-dir', str(index), '--no-refresh'] for label, profile in [('release', 'release'), ('instrumented', 'profiling')]}
    commands['rg'] = [shutil.which('rg'), '--no-config', '--hidden', '-g', '!.git', '-n', '--no-heading', '--color', 'never', pattern, '.']
    outputs = {}
    for label, command in commands.items():
        result = subprocess.run(command, cwd=root, env=ENV, capture_output=True, check=True)
        outputs[label] = sorted(line.removeprefix(b'./') for line in result.stdout.splitlines())
    assert outputs['release'] == outputs['instrumented'] == outputs['rg']
    samples = {label: [] for label in commands}
    for iteration in range(-5, 31):
        order = list(commands)
        rng.shuffle(order)
        for label in order:
            start = time.perf_counter_ns()
            result = subprocess.run(commands[label], cwd=root, env=ENV, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=True)
            record = {'process_total': (time.perf_counter_ns()-start)/1e6}
            for line in result.stderr.decode().splitlines():
                if line.startswith('PERF '):
                    _, key, value = line.split()
                    record[key] = float(value)
            if iteration >= 0:
                samples[label].append(record)
    report[name] = {'pattern': pattern, 'root': str(root), 'index': str(index), 'commands': commands, 'warmup': 5, 'iterations': 31, 'manifest_bytes': (index/'manifest.json').stat().st_size, 'match_lines': len(outputs['release']), 'match_files': len({line.split(b':',1)[0] for line in outputs['release']}), 'samples': samples, 'median': {label: {key: statistics.median(s[key] for s in values) for key in values[0]} for label, values in samples.items()}}
    print(name, json.dumps({key: value for key, value in report[name].items() if key not in ('samples', 'commands')}, indent=2), flush=True)
(HERE/'vllm-profile-results.json').write_text(json.dumps(report, indent=2)+'\n')
