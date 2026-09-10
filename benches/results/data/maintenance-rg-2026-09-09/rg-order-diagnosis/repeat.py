import json
import pathlib
import statistics
import subprocess
import time

project = next(parent for parent in pathlib.Path(__file__).resolve().parents
               if (parent / 'Cargo.toml').is_file() and (parent / 'benches').is_dir())
report = json.loads((project / 'benches/results/data/maintenance-rg-2026-09-09/vllm-history.json').read_text())
root = pathlib.Path(report['workspace']) / 'source'
rg = ['rg', '--no-config', '--hidden', '-g', '!.git', '-F', '-l', 'SamplingParams', '.']
rows = []
expected = None

def run(command, cwd, wrapped):
    if wrapped:
        command = ['/usr/bin/time', '-l'] + command
    start = time.perf_counter()
    output = subprocess.run(command, cwd=cwd, capture_output=True)
    elapsed = (time.perf_counter() - start) * 1000
    assert output.returncode == 0, output.stderr.decode()
    return output, elapsed

for round_no in range(10):
    for wrapped in [False, True]:
        for preceding in ['none', 'main', 'current', 'no-sync', 'current-no-refresh']:
            if preceding != 'none':
                name = 'current' if preceding == 'current-no-refresh' else preceding
                variant = next(v for v in report['variants'] if v['name'] == name)
                command = [variant['binary'], 'search', '-F', '-l', 'SamplingParams', str(root), '--index-dir', variant['index']]
                if preceding == 'current-no-refresh':
                    command.append('--no-refresh')
                run(command, project, wrapped)
            output, elapsed = run(rg, root, wrapped)
            files = sorted(output.stdout.splitlines())
            if expected is None:
                expected = files
            assert files == expected
            rows.append(dict(round=round_no, wrapped=wrapped, preceding=preceding, wall_ms=elapsed, time_stderr=output.stderr.decode(), file_count=len(files)))

destination = pathlib.Path(__file__).with_name('repeat.json')
destination.write_text(json.dumps(dict(commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root).decode().strip(), rows=rows), indent=2) + '\n')
for wrapped in [False, True]:
    for preceding in ['none', 'main', 'current', 'no-sync', 'current-no-refresh']:
        samples = [r['wall_ms'] for r in rows if r['wrapped'] == wrapped and r['preceding'] == preceding]
        print(wrapped, preceding, 'median', round(statistics.median(samples), 2), 'range', round(min(samples), 2), round(max(samples), 2), flush=True)
print(destination)
