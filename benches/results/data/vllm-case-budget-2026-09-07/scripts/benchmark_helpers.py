import datetime
import hashlib
import json
import math
import os
from pathlib import Path
import random
import re
import statistics
import subprocess
import time

OUT = Path(__file__).resolve().parent
REPO = Path('/Users/b1indsight/personal_work/coderg')
ROOT = Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
BIN = REPO / 'target/release/coderg'
HARNESS = REPO / 'target/release/deps/compare_rg-fc1ea6204002ec67'
ITERATIONS = 50
WARMUP = 3
MEMORY_RUNS = 3
MIB = 1024 ** 2
GIB = 1024 ** 3
commands = []


def run(args, *, cwd=ROOT, env=None, allowed=(0,), capture=True):
    argv = list(map(str, args))
    commands.append({'argv': argv, 'cwd': str(cwd)})
    result = subprocess.run(argv, cwd=cwd, env=env, stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                            stderr=subprocess.PIPE)
    if result.returncode not in allowed:
        raise RuntimeError(f'{argv} exited {result.returncode}: {result.stderr.decode(errors="replace")}')
    return result


def text_command(args, cwd=ROOT):
    return run(args, cwd=cwd).stdout.decode().strip()


def time_stats(path):
    raw = path.read_text()
    timing = re.search(r'([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys', raw)
    if timing is None:
        raise ValueError(f'No timing in {path}')
    result = dict(zip(('real_s', 'user_s', 'sys_s'), map(float, timing.groups())))
    for label, key in [('maximum resident set size', 'peak_rss_bytes'),
                       ('peak memory footprint', 'peak_footprint_bytes'),
                       ('page faults', 'page_faults'), ('swaps', 'swaps')]:
        value = re.search(r'^\s*(\d+)\s+' + label + r'\s*$', raw, re.M)
        if value:
            result[key] = int(value.group(1))
    if 'peak_rss_bytes' not in result:
        raise ValueError(f'No peak RSS in {path}')
    return result


def timed_resource(args, label, allowed=(0,)):
    log = OUT / f'{label}.time.txt'
    result = run(['/usr/bin/time', '-l', '-o', log, *args], allowed=allowed)
    (OUT / f'{label}.stdout.txt').write_bytes(result.stdout)
    (OUT / f'{label}.stderr.txt').write_bytes(result.stderr)
    return time_stats(log)


def summarize(samples):
    ordered = sorted(samples)
    return {'min_ms': ordered[0], 'median_ms': ordered[math.ceil(len(ordered) * .5) - 1],
            'p95_ms': ordered[math.ceil(len(ordered) * .95) - 1],
            'mean_ms': statistics.mean(ordered), 'samples_ms': samples}

