import importlib.util
import json
from pathlib import Path
import statistics
import subprocess
import time

repo = Path.cwd()
out = repo / '.cache/2026-09-10/search-or-profile'
prior = repo / '.cache/2026-09-10/nextmask-profile'
binary = repo / 'target/release/coderg'
profiler = repo / 'target/release/deps/masked_cover_profile-a853751d9aeeb5b2'
pattern = r'def pending|def finish|def consume|def relay|_touch|def apply|decision.action|WAIT|wake'
spec = importlib.util.spec_from_file_location('profile_driver', repo / 'benches/masked_cover_profile.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

def run(command, root):
    start = time.perf_counter()
    p = subprocess.run([str(x) for x in command], cwd=root, capture_output=True)
    assert p.returncode in (0, 1), p.stderr.decode()
    assert not p.stderr, p.stderr.decode()
    return p.stdout, (time.perf_counter() - start) * 1000

def counts(raw):
    result = {}
    for line in raw.decode().splitlines():
        path, count = line.rsplit(':', 1)
        result[path.removeprefix('./')] = int(count)
    return result

results = {}
for suite in ('vllm', 'chromium'):
    root, index = prior / suite, prior / (suite + '-index')
    paths_file = prior / (suite + '-results/indexed-files.json')
    if paths_file.exists():
        paths = json.loads(paths_file.read_text())
    else:
        raw, _ = run([binary, 'stats', '.', '--index-dir', index, '--json'], root)
        paths = [d['path'] for d in json.loads(raw)['documents'] if d['active'] and d['searchable']]
    expected = {}
    for offset in range(0, len(paths), 1000):
        raw, _ = run(['rg', '--no-config', '--encoding', 'none', '--text', '--no-ignore', '-c', '-H', '--color', 'never', '--no-heading', '--', pattern, *paths[offset:offset+1000]], root)
        expected.update(counts(raw))
    expected_file = out / (suite + '-expected.json')
    expected_file.write_text(json.dumps(expected))
    print(suite, 'reference', len(expected), 'files', sum(expected.values()), 'lines', flush=True)
    report_file = out / (suite + '.json')
    command = [profiler, '--root', root, '--index-dir', index, '--pattern', pattern, '--expected', expected_file, '--output', report_file, '--iterations', '9', '--scan-iterations', '5', '--warmup', '2']
    run(command, repo)
    report = json.loads(report_file.read_text())
    assert report['production_baseline_equivalent'] and report['expected_counts_verified']
    summary = module.summarize(report)
    cli_command = [binary, 'search', '-c', pattern, '.', '--index-dir', index, '--no-refresh']
    cli_samples = []
    for i in range(7):
        raw, elapsed = run(cli_command, root)
        assert counts(raw) == expected
        if i >= 2:
            cli_samples.append(elapsed)
    results[suite] = dict(pattern=pattern, searchable_files=len(paths), matched_files=len(expected), matched_lines=sum(expected.values()), summary=summary, cli_samples_ms=cli_samples, cli_median_ms=statistics.median(cli_samples), profiler_command=[str(x) for x in command], cli_command=[str(x) for x in cli_command])
    (out / 'summary.json').write_text(json.dumps(results, indent=2))
    print(suite, json.dumps(results[suite]), flush=True)
