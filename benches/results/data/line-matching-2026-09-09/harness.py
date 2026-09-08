import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess
import sys
import time

sys.dont_write_bytecode = True

parser = argparse.ArgumentParser()
parser.add_argument('--iterations', type=int, default=31)
parser.add_argument('--warmup', type=int, default=3)
parser.add_argument('--queries', default='todo_comment,digits,icase_todo,literal_parallel,word_sampling,async_def,blank_lines')
parser.add_argument('--output', required=True, type=Path)
args = parser.parse_args()
repo = Path('/Users/b1indsight/personal_work/coderg')
work = Path(__file__).parent
root = Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
spec = importlib.util.spec_from_file_location('suite', repo / 'benches/regex_suite.py')
suite = importlib.util.module_from_spec(spec)
spec.loader.exec_module(suite)
rg = shutil.which('rg')
commands = {}
parity = {}
for query, category, pattern, flags in suite.QUERIES:
    modes = {}
    for mode in ['before', 'after']:
        common = [str(work / mode), 'search', *flags, pattern, '.', '--index-dir', str(work / 'index')]
        modes[mode] = common
        modes[mode + '_no_refresh'] = [*common, '--no-refresh']
    modes['rg'] = [rg, '--no-config', *flags, '-n', '-H', '--hidden', '--glob', '!.git/**', '--glob', '!.coderg-index/**', '--color', 'never', '--no-heading', pattern, '.']
    commands[query] = modes
    outputs = {}
    for mode in ['rg', 'before_no_refresh', 'after_no_refresh', 'after']:
        p = subprocess.run(modes[mode], cwd=root, capture_output=True, check=False)
        if p.returncode not in (0, 1) or p.stderr:
            raise RuntimeError((query, mode, p.returncode, p.stderr))
        lines = sorted(line.removeprefix(b'./') for line in p.stdout.splitlines())
        outputs[mode] = (p.returncode, lines)
    parity[query] = {
        'pattern': pattern, 'flags': flags,
        'matching_lines': len(outputs['rg'][1]),
        'before_matches_rg': outputs['before_no_refresh'] == outputs['rg'],
        'after_matches_rg': outputs['after_no_refresh'] == outputs['rg'] == outputs['after'],
        'rg_sha256': hashlib.sha256(b'\n'.join(outputs['rg'][1])).hexdigest(),
    }
    if not parity[query]['after_matches_rg']:
        raise RuntimeError(('parity failed', query))
print('Parity:', len(parity), '/', len(parity), 'after matches rg; previous differences:', [q for q,p in parity.items() if not p['before_matches_rg']], flush=True)
selected = args.queries.split(',') if args.queries != 'all' else list(commands)
samples = {query: {mode: [] for mode in commands[query]} for query in selected}
jobs = [(query, mode) for query in selected for mode in commands[query]]
rng = random.Random(20260909)
for round_index in range(args.warmup + args.iterations):
    rng.shuffle(jobs)
    for query, mode in jobs:
        started = time.perf_counter_ns()
        p = subprocess.run(commands[query][mode], cwd=root, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=False)
        elapsed = (time.perf_counter_ns() - started) / 1e6
        if p.returncode not in (0, 1) or p.stderr:
            raise RuntimeError((query, mode, p.returncode, p.stderr))
        if round_index >= args.warmup:
            samples[query][mode].append(elapsed)
    if (round_index + 1) % 5 == 0:
        print('Round', round_index + 1, '/', args.warmup + args.iterations, flush=True)
medians = {q: {mode: statistics.median(values) for mode,values in modes.items()} for q,modes in samples.items()}
result = {'root': str(root), 'iterations': args.iterations, 'warmup': args.warmup, 'seed': 20260909, 'parity': parity, 'commands': commands, 'samples_ms': samples, 'medians_ms': medians, 'binary_sha256': {mode: hashlib.sha256((work / mode).read_bytes()).hexdigest() for mode in ('before','after')}}
result['environment'] = {'platform': platform.platform(), 'logical_cpus': os.cpu_count(), 'RAYON_NUM_THREADS': os.environ.get('RAYON_NUM_THREADS'), 'rg_version': subprocess.check_output([rg, '--version'], text=True), 'rustc': subprocess.check_output(['rustc', '--version'], text=True), 'corpus_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(), 'corpus_status': subprocess.check_output(['git', 'status', '--porcelain'], cwd=root, text=True), 'baseline_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=repo, text=True).strip()}
result['source_sha256'] = {str(p.relative_to(repo)): hashlib.sha256(p.read_bytes()).hexdigest() for p in [repo / 'Cargo.toml', repo / 'Cargo.lock', *sorted((repo / 'src').glob('*.rs'))]}
args.output.write_text(json.dumps(result, indent=2) + '\n')
for query, modes in medians.items():
    print(query, {mode: round(value, 3) for mode,value in modes.items()}, flush=True)
