"""Run the temporary instrumented profiling build against the existing index."""
import json
import os
from pathlib import Path
import random
import statistics
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = Path('/private/tmp/coderg-chromium-3986304/src')
BIN = HERE.parents[3] / 'target/profiling/coderg'
COMMAND = [str(BIN), 'search', '-F', 'MAX_FILE_SIZE', '.', '--index-dir', str(ROOT.parent / 'index')]
ENV = dict(os.environ, LC_ALL='C')
EXPECTED = sorted((HERE / 'coderg_default.txt').read_bytes().splitlines())
samples = {'no_refresh': [], 'default': []}
randomizer = random.Random(20260909)
for round_number in range(-2, 15):
    modes = list(samples)
    randomizer.shuffle(modes)
    for mode in modes:
        start = time.perf_counter()
        result = subprocess.run(COMMAND + (['--no-refresh'] if mode == 'no_refresh' else []), cwd=ROOT, env=ENV, capture_output=True, check=True)
        total_ms = (time.perf_counter() - start) * 1000
        assert sorted(result.stdout.splitlines()) == EXPECTED
        timings = {'process_total': total_ms}
        for line in result.stderr.decode().splitlines():
            if line.startswith('PERF '):
                _, name, value = line.split()
                timings[name] = float(value)
        if round_number >= 0:
            samples[mode].append(timings)
    print(f'Round {round_number + 1}', flush=True)
summary = {mode: {key: statistics.median(s[key] for s in values) for key in values[0]} for mode, values in samples.items()}
(HERE / 'profile-results.json').write_text(json.dumps({'samples': samples, 'median': summary}, indent=2) + '\n')
print(json.dumps(summary, indent=2))
