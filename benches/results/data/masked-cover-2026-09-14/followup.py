from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import subprocess

repo = Path.cwd()
work = repo / '.cache/2026-09-10/masked-cover-profile'
previous = repo / '.cache/2026-09-10/nextmask-profile'
metadata = json.loads((work / 'vllm/metadata.json').read_text())
profiler = next((repo / 'target/release/deps').glob('masked_cover_profile-*'))
if profiler.suffix == '.d':
    profiler = next(p for p in (repo / 'target/release/deps').glob('masked_cover_profile-*') if p.suffix != '.d')
assert hashlib.sha256(profiler.read_bytes()).hexdigest() == metadata['profiler_sha256']
rows = {r['id']:r for r in json.loads((work / 'vllm/summary.json').read_text())}
out = work / 'followup'
out.mkdir(exist_ok=False)
commands = []
for name in ['literal_rocm', 'icase_platform', 'config_assignment', 'quoted_device']:
    row = rows[name]
    cmd = [str(profiler), '--root', str(previous / 'vllm'), '--index-dir', str(previous / 'vllm-index'),
           '--pattern', row['pattern'], *row['flags'], '--expected', str(work / 'vllm/expected' / (name+'.json')),
           '--output', str(out / (name+'.json')), '--iterations', '9', '--scan-iterations', '5']
    print('followup', name, flush=True)
    subprocess.run(cmd, check=True)
    commands.append(cmd)
(out / 'metadata.json').write_text(json.dumps(dict(
    selection='Adaptive follow-up for candidate regressions observed in the vLLM sweep; excluded from sweep aggregates.',
    completed_utc=datetime.now(timezone.utc).isoformat(),profiler_sha256=metadata['profiler_sha256'],
    query_iterations=9,scan_iterations=5,warmup=2,commands=commands),indent=2)+'\n')
