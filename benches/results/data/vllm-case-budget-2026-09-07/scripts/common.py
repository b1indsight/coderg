import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics
import subprocess
import tempfile
import time

OUT = Path(__file__).resolve().parent
ROOT = Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
INDEX = Path('/private/tmp/coderg-vllm-metrics.41MLD8/index-3')
QUERIES = [
    dict(id='asyncmock', split='train', pattern=r'\basyncmock\b', flags=['-i']),
    dict(id='platforms', split='train', pattern=r'\b(cuda|rocm)\b', flags=['-i']),
    dict(id='sampling', split='train', pattern=r'\bSamplingParams\b', flags=['-i']),
    dict(id='logger', split='train', pattern=r'logger\.(warning|error)\(', flags=['-i']),
    dict(id='parallel', split='train', pattern='get_tensor_model_parallel_world_size', flags=['-i', '-F']),
    dict(id='cache', split='train', pattern=r'\b(kv_cache|block_size)\b', flags=['-i']),
    dict(id='todos', split='train', pattern=r'\b(TODO|FIXME)\b', flags=['-i']),
    dict(id='envvars', split='train', pattern=r'\b(VLLM|CUDA)_[A-Z0-9_]+\b', flags=['-i']),
    dict(id='attention', split='holdout', pattern=r'class[ \t]+\w*Attention\b', flags=['-i']),
    dict(id='forward', split='holdout', pattern=r'def[ \t]+forward\b', flags=['-i']),
    dict(id='maxconfig', split='holdout', pattern=r'\bmax_(model_len|num_seqs)\b', flags=['-i']),
    dict(id='exceptions', split='holdout', pattern=r'\b(ValueError|RuntimeError|NotImplementedError)\b', flags=['-i']),
    dict(id='tensors', split='holdout', pattern=r'\b(torch|numpy)\.(empty|zeros|ones)\b', flags=['-i']),
    dict(id='asyncdef', split='holdout', pattern=r'async[ \t]+def[ \t]+\w+', flags=['-i']),
    dict(id='sensitive_parallel', split='control', pattern='get_tensor_model_parallel_world_size', flags=['-F']),
    dict(id='sensitive_sampling', split='control', pattern=r'\bSamplingParams\b', flags=[]),
    dict(id='sensitive_forward', split='control', pattern=r'def[ \t]+forward\b', flags=[]),
]

def config_name(v, k):
    return f'v{v}-k{k}'

def argv(query, binary=None, mode='norefresh'):
    if binary is None:
        return ['rg', *query['flags'], '-n', '-H', '--hidden', '--glob', '!.git/**', '--color', 'never', '--no-heading', query['pattern'], '.']
    command = [str(binary), 'search', *query['flags'], query['pattern'], '.', '--index-dir', str(INDEX)]
    if mode == 'norefresh':
        command.append('--no-refresh')
    return command

def child_env(v=None, k=None, trace=False):
    env = os.environ.copy()
    for key in ['CODERG_BENCH_VARIANTS', 'CODERG_BENCH_KEYS', 'CODERG_BENCH_TRACE']:
        env.pop(key, None)
    if v is not None:
        env['CODERG_BENCH_VARIANTS'] = str(v)
        env['CODERG_BENCH_KEYS'] = str(k)
    if trace:
        env['CODERG_BENCH_TRACE'] = '1'
    return env

def execute(command, env=None, stdout=subprocess.DEVNULL):
    start = time.perf_counter_ns()
    result = subprocess.run(command, cwd=ROOT, env=env, stdout=stdout, stderr=subprocess.PIPE)
    elapsed = (time.perf_counter_ns() - start) / 1e6
    if result.returncode not in (0, 1):
        raise RuntimeError((command, result.returncode, result.stderr.decode(errors='replace')))
    return elapsed, result.stderr

def output_hash(command, env=None):
    with tempfile.TemporaryFile() as stream:
        _, stderr = execute(command, env=env, stdout=stream)
        stream.seek(0)
        digest = hashlib.sha256()
        lines = size = 0
        for line in stream:
            line = line.removeprefix(b'./')
            digest.update(line)
            lines += 1
            size += len(line)
    trace = {}
    for line in stderr.splitlines():
        if line.startswith(b'{'):
            trace.update(json.loads(line))
    return dict(sha256=digest.hexdigest(), lines=lines, bytes=size), trace

def expectations():
    path = OUT / 'expected.json'
    if path.exists():
        return json.loads(path.read_text())
    expected = {q['id']: output_hash([*argv(q), '--sort', 'path'])[0] for q in QUERIES}
    save(path, expected)
    return expected

def summarize(samples):
    return dict(median_ms=statistics.median(samples),
                mean_ms=statistics.mean(samples),
                p95_ms=sorted(samples)[math.ceil(len(samples) * .95) - 1],
                min_ms=min(samples), samples_ms=samples)

def save(path, obj):
    path.write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n')

def index_snapshot():
    return {str(p.relative_to(INDEX)): dict(bytes=p.stat().st_size, sha256=hashlib.sha256(p.read_bytes()).hexdigest())
            for p in INDEX.rglob('*') if p.is_file()}

def memory(command, label):
    log = OUT / f'{label}.time.txt'
    execute(['/usr/bin/time', '-l', '-o', str(log), *command])
    raw = log.read_text()
    rss = re.search(r'^\s*(\d+)\s+maximum resident set size\s*$', raw, re.M)
    if rss is None:
        raise RuntimeError(f'Missing RSS: {log}')
    footprint = re.search(r'^\s*(\d+)\s+peak memory footprint\s*$', raw, re.M)
    return dict(peak_rss_bytes=int(rss.group(1)), peak_footprint_bytes=int(footprint.group(1)) if footprint else None)
