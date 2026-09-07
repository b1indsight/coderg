import datetime
import hashlib
import itertools
import json
from pathlib import Path
import random
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, '/private/tmp/coderg-vllm-metrics.41MLD8')
from benchmark_helpers import ROOT, REPO, run, text_command, time_stats, summarize, commands, MIB

OUT = Path(__file__).resolve().parent
BASE = Path('/private/tmp/coderg-vllm-metrics.41MLD8')
INDEX = BASE / 'index-3'
OLD = BASE / 'coderg-before-case-index'
NEW = REPO / 'target/release/coderg'
ITERATIONS = 30
WARMUP = 3
MEMORY_RUNS = 3
previous = json.loads((BASE / 'results.json').read_text())
assert hashlib.sha256(OLD.read_bytes()).hexdigest() == previous['metadata']['coderg_binary_sha256']
assert text_command(['git', 'rev-parse', 'HEAD']) == previous['metadata']['vllm_commit']
assert not text_command(['git', 'status', '--porcelain=v1'])
index_before = {str(p.relative_to(INDEX)): {'bytes': p.stat().st_size,
                'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for p in INDEX.rglob('*') if p.is_file()}
metadata = {'recorded_at': datetime.datetime.now().astimezone().isoformat(),
            'vllm_commit': previous['metadata']['vllm_commit'], 'corpus': previous['corpus'],
            'old_binary': str(OLD), 'old_binary_sha256': hashlib.sha256(OLD.read_bytes()).hexdigest(),
            'new_binary': str(NEW), 'new_binary_sha256': hashlib.sha256(NEW.read_bytes()).hexdigest(),
            'iterations': ITERATIONS, 'warmup': WARMUP, 'memory_runs': MEMORY_RUNS}
(OUT / 'change.diff').write_bytes(run(['git', 'diff', '--binary'], cwd=REPO).stdout)
queries = [
    ('忽略大小写：类型名', r'\basyncmock\b', ['-i']),
    ('忽略大小写：平台名', r'\b(cuda|rocm)\b', ['-i']),
    ('忽略大小写：类型引用', r'\bSamplingParams\b', ['-i']),
    ('忽略大小写：日志调用', r'logger\.(warning|error)\(', ['-i']),
    ('忽略大小写：长固定字符串', 'get_tensor_model_parallel_world_size', ['-i', '-F']),
    ('大小写敏感：长固定字符串', 'get_tensor_model_parallel_world_size', ['-F']),
    ('大小写敏感：类型引用', r'\bSamplingParams\b', []),
    ('大小写敏感：函数定义', r'def[ \t]+forward\b', []),
]
rng = random.Random(20260906)
rows = []


def verify(variants):
    with tempfile.TemporaryFile() as expected:
        result = subprocess.run([*variants['rg'], '--sort', 'path'], cwd=ROOT,
                                stdout=expected, stderr=subprocess.PIPE)
        assert result.returncode in (0, 1), result.stderr
        for name, argv in variants.items():
            if name == 'rg':
                continue
            with tempfile.TemporaryFile() as actual:
                result = subprocess.run(argv, cwd=ROOT, stdout=actual, stderr=subprocess.PIPE)
                assert result.returncode in (0, 1), (name, result.stderr)
                actual.seek(0)
                expected.seek(0)
                count = 0
                for left, right in itertools.zip_longest(actual, expected, fillvalue=b''):
                    count += 1
                    assert left.removeprefix(b'./') == right.removeprefix(b'./'), (name, count, left[:200], right[:200])
        return count


for qi, (label, pattern, flags) in enumerate(queries):
    variants = {}
    for version, binary in [('old', OLD), ('new', NEW)]:
        variants[f'{version}_default'] = list(map(str, [binary, 'search', *flags, pattern, '.', '--index-dir', INDEX]))
        variants[f'{version}_no_refresh'] = [*variants[f'{version}_default'], '--no-refresh']
    variants['rg'] = ['rg', '--hidden', '--glob', '!.git/**', '-n', '-H', '--color', 'never',
                      '--no-heading', *flags, pattern, '.']
    print(f'{qi + 1}/{len(queries)} {label}: {pattern}', flush=True)
    matching_lines = verify(variants)
    for _ in range(WARMUP):
        for args in variants.values():
            run(args, allowed=(0, 1), capture=False)
    samples = {name: [] for name in variants}
    for _ in range(ITERATIONS):
        names = list(variants)
        rng.shuffle(names)
        for name in names:
            started = time.perf_counter_ns()
            run(variants[name], allowed=(0, 1), capture=False)
            samples[name].append((time.perf_counter_ns() - started) / 1_000_000)
    memory = {name: [] for name in ['old_default', 'new_default', 'rg']}
    for n in range(MEMORY_RUNS):
        for name in memory:
            log = OUT / f'query-{qi}-{name}-{n}.time.txt'
            run(['/usr/bin/time', '-l', '-o', log, *variants[name]], allowed=(0, 1), capture=False)
            memory[name].append(time_stats(log))
    row = {'label': label, 'pattern': pattern, 'flags': flags, 'matching_lines': matching_lines,
           'verification': 'all four coderg modes and rg have identical output',
           'timing': {name: summarize(values) for name, values in samples.items()}, 'memory': memory}
    rows.append(row)
    (OUT / f'query-{qi}.json').write_text(json.dumps(row, indent=2, ensure_ascii=False) + '\n')
    print(json.dumps({'median_ms': {name: row['timing'][name]['median_ms'] for name in variants},
                      'peak_rss_mib': {name: max(s['peak_rss_bytes'] for s in memory[name]) / MIB for name in memory}}), flush=True)

index_after = {str(p.relative_to(INDEX)): {'bytes': p.stat().st_size,
               'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for p in INDEX.rglob('*') if p.is_file()}
assert index_after == index_before, 'The existing index must be reused without rewriting it'
assert not text_command(['git', 'status', '--porcelain=v1'])
assert metadata['new_binary_sha256'] == hashlib.sha256(NEW.read_bytes()).hexdigest()
report = {'metadata': metadata, 'index_unchanged': True, 'index_files': index_after, 'queries': rows}
(OUT / 'results.json').write_text(json.dumps(report, indent=2, ensure_ascii=False) + '\n')
(OUT / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
lines = ['# 有限大小写变体索引查询：vLLM 前后对照', '',
         f"时间：{metadata['recorded_at']}。Apple M5 / 24 GiB；vLLM `{metadata['vllm_commit'][:12]}`；6,835 个文本文件，82.38 MiB。", '',
         '- 旧版本为修改前保留的 release binary，新版本为当前工作区 release binary；完整补丁保存在 change.diff。',
         '- 复用上一轮 66.40 MiB 索引，逐文件 SHA-256 校验确认测试前后完全不变。',
         '- 使用默认匹配行输出；rg 使用 -n -H 对齐输出格式。每个模式预热 3 次，随机交错计时 30 次，包含进程启动、索引加载、格式化及写 stdout；输出直接丢弃。',
         '- 四种 coderg 模式均与 rg 的完整输出逐行校验一致；仅校验时为 rg 添加 --sort path，计时使用默认并行遍历。',
         '- 未清空系统文件缓存。峰值 RSS 单独采集 3 次，取最大值；内存测量不计入延迟分布。', '',
         '| 查询 | 旧默认 ms | 新默认 ms | 旧 --no-refresh ms | 新 --no-refresh ms | rg ms | 默认提速 |',
         '|---|---:|---:|---:|---:|---:|---:|']
for row in rows:
    t = row['timing']
    pattern = (' '.join(row['flags']) + ' ' if row['flags'] else '') + row['pattern'].replace('|', '\\|')
    values = [f"{t[name]['median_ms']:.2f}" for name in ('old_default', 'new_default', 'old_no_refresh', 'new_no_refresh', 'rg')]
    lines.append(f"| `{pattern}` | " + ' | '.join(values) + f" | {t['old_default']['median_ms'] / t['new_default']['median_ms']:.2f}× |")
lines += ['', '## 默认搜索峰值 RSS', '', '| 查询 | 旧 MiB | 新 MiB | rg MiB |', '|---|---:|---:|---:|']
for row in rows:
    values = [f"{max(s['peak_rss_bytes'] for s in row['memory'][name]) / MIB:.2f}" for name in ('old_default', 'new_default', 'rg')]
    lines.append(f"| {row['label']} | " + ' | '.join(values) + ' |')
lines += ['', 'results.json 包含所有计时样本和 p95；commands.json 包含计时命令；*.time.txt 保存内存原始日志；benchmark.py 为复现脚本。', '']
(OUT / 'report.md').write_text('\n'.join(lines))
print(f'Completed: {OUT / "report.md"}', flush=True)
