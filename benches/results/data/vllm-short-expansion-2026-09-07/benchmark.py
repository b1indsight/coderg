import argparse
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import random
import shutil
import statistics
import subprocess
import time

OUT = Path(__file__).resolve().parent
REPO = Path('/Users/b1indsight/personal_work/coderg')
ROOT = Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
INDEX = Path('/private/tmp/coderg-vllm-regex-suite-2026-09-07-index')
BASELINE = REPO / 'target/release/coderg'
PROTOTYPE = REPO / 'target/release/coderg-short-probe'
RG = shutil.which('rg')
QUERIES = [dict(id=word, pattern=rf'\b{word}\b', flags=[]) for word in
           ('if', 'in', 'is', 'as', 'id', 'os', 'io', 'kv', 'qx', 'QZ', 'def')]
QUERIES += [dict(id='fixed_' + word, pattern=word, flags=['-F']) for word in ('if', 'qx')]
MODES = [(implementation, refresh) for implementation in ('baseline', 'all', 'boundary')
         for refresh in ('default', 'no_refresh')] + [('rg', 'default')]
SEED = 20260907


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def command(query, implementation, refresh):
    env = os.environ.copy()
    for key in ('RIPGREP_CONFIG_PATH', 'CODERG_SHORT_PROBE', 'CODERG_SHORT_TRACE',
                'CODERG_BENCH_VARIANTS', 'CODERG_BENCH_KEYS', 'CODERG_BENCH_TRACE'):
        env.pop(key, None)
    if implementation == 'rg':
        argv = [RG, '--no-config', '-n', '-H', '--hidden', '--glob', '!.git/**',
                '--glob', '!.coderg-index/**', '--color', 'never', '--no-heading',
                *query['flags'], query['pattern'], '.']
    else:
        binary = BASELINE if implementation == 'baseline' else PROTOTYPE
        argv = [str(binary), 'search', *query['flags'], query['pattern'], '.', '--index-dir', str(INDEX)]
        if refresh == 'no_refresh':
            argv.append('--no-refresh')
        if implementation != 'baseline':
            env['CODERG_SHORT_PROBE'] = implementation
    return argv, env


def run(argv, env, capture=False):
    started = time.perf_counter_ns()
    result = subprocess.run(argv, cwd=ROOT, env=env, stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                            stderr=subprocess.PIPE)
    ms = (time.perf_counter_ns() - started) / 1e6
    if result.returncode not in (0, 1):
        raise RuntimeError((argv, result.returncode, result.stderr.decode(errors='replace')))
    return ms, result


def state():
    return dict(baseline_sha256=sha(BASELINE), prototype_sha256=sha(PROTOTYPE),
                source_sha256={str(p.relative_to(REPO)):sha(p) for p in sorted((REPO/'src').glob('*.rs'))},
                index_sha256={str(p.relative_to(INDEX)):sha(p) for p in sorted(INDEX.rglob('*')) if p.is_file()},
                vllm_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode().strip(),
                vllm_status=subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all'], cwd=ROOT).decode())


def summarize(samples):
    s = sorted(samples)
    return dict(median_ms=statistics.median(s), min_ms=s[0], p95_ms=s[math.ceil(.95 * len(s))-1],
                mean_ms=statistics.mean(s), samples_ms=samples)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('phase', choices=['check', 'bench'])
    parser.add_argument('--iterations', type=int, default=31)
    parser.add_argument('--warmup', type=int, default=3)
    args = parser.parse_args()
    if args.phase == 'check':
        data = dict(metadata=state(), queries=[])
        data['metadata'].update(recorded_at=datetime.now(timezone.utc).isoformat(), seed=SEED,
                                root=str(ROOT), index_dir=str(INDEX), harness_sha256=sha(Path(__file__)))
        assert not data['metadata']['vllm_status']
        for query in QUERIES:
            argv, env = command(query, 'rg', 'default')
            _, result = run(argv, env, True)
            expected = sorted(line.removeprefix(b'./') for line in result.stdout.splitlines())
            entry = dict(**query, matching_lines=len(expected), matching_files=len({line.split(b':', 1)[0] for line in expected}),
                         output_sha256=hashlib.sha256(b'\n'.join(expected)).hexdigest(), exit_code=result.returncode, modes={})
            for implementation, refresh in MODES:
                mode = implementation + '_' + refresh
                argv, env = command(query, implementation, refresh)
                _, result = run(argv, env, True)
                actual = sorted(line.removeprefix(b'./') for line in result.stdout.splitlines())
                assert actual == expected and result.returncode == entry['exit_code'], (query['id'], mode)
                assert not result.stderr, result.stderr
                entry['modes'][mode] = dict(command=argv, env={k:v for k,v in env.items() if k == 'CODERG_SHORT_PROBE'}, parity=True)
                if implementation != 'rg' and refresh == 'no_refresh':
                    if implementation == 'baseline':
                        argv[0] = str(PROTOTYPE)
                    env['CODERG_SHORT_TRACE'] = '1'
                    _, traced = run(argv, env)
                    entry['modes'][mode]['trace'] = json.loads(traced.stderr)
            data['queries'].append(entry)
            print(query['id'], 'PASS', entry['matching_files'], 'matching files;',
                  {mode:value['trace'] for mode,value in entry['modes'].items() if 'trace' in value}, flush=True)
        (OUT / 'checks.json').write_text(json.dumps(data, indent=2) + '\n')
        return
    data = json.loads((OUT / 'checks.json').read_text())
    assert all(data['metadata'][k] == v for k,v in state().items())
    data['metadata'].update(iterations=args.iterations, warmup=args.warmup)
    for query in data['queries']:
        for mode in query['modes'].values():
            mode['samples_ms'] = []
    jobs = [(query, implementation, refresh) for query in data['queries'] for implementation, refresh in MODES]
    rng = random.Random(SEED)
    for round_index in range(args.warmup + args.iterations):
        rng.shuffle(jobs)
        for query, implementation, refresh in jobs:
            argv, env = command(query, implementation, refresh)
            elapsed, result = run(argv, env)
            assert result.returncode == query['exit_code'] and not result.stderr, (query['id'], result.stderr)
            if round_index >= args.warmup:
                query['modes'][implementation + '_' + refresh]['samples_ms'].append(elapsed)
        print(f'Round {round_index + 1}/{args.warmup + args.iterations}', flush=True)
        (OUT / 'results.json').write_text(json.dumps(data, indent=2) + '\n')
    for query in data['queries']:
        for mode in query['modes'].values():
            mode['timing'] = summarize(mode.pop('samples_ms'))
    after = state()
    data['metadata']['inputs_unchanged'] = all(data['metadata'][k] == v for k,v in after.items())
    (OUT / 'results.json').write_text(json.dumps(data, indent=2) + '\n')
    assert data['metadata']['inputs_unchanged']
    print('Complete; inputs unchanged.', flush=True)


if __name__ == '__main__':
    main()
