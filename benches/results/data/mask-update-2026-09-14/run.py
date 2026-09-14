import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

repo = Path.cwd()
base_work = repo / '.cache/2026-09-10/mask-update-profile'
batch_mode = os.environ.get('MASK_UPDATE_BATCH') == '1'
work = base_work / 'batch' if batch_mode else base_work
work.mkdir(parents=True, exist_ok=True)
source = repo / '.cache/2026-09-09/vllm-real-qjEMUc/vllm-history'
root = work / 'vllm'
cli = repo / 'target/release/coderg'
end = '569adb5a9780f9c02d22a6b29826acf711512356'
variants = {'baseline': (base_work / 'baseline-probe', 0), 'compute': (base_work / 'mask-probe', 1), 'persist': (base_work / ('batch-probe' if batch_mode else 'mask-probe'), 4 if batch_mode else 3)}
commands = []
def run(args, cwd=root):
    command = [str(x) for x in args]
    start = time.perf_counter()
    p = subprocess.run(command, cwd=cwd, capture_output=True)
    elapsed = (time.perf_counter() - start) * 1000
    assert p.returncode == 0, (command, p.stderr.decode())
    commands.append(command)
    return p.stdout, elapsed

def git(*args, cwd=root):
    return run(['git', *args], cwd)[0]

commits = git('rev-list', '--first-parent', '--max-count=31', end, cwd=source).decode().splitlines()[::-1]
assert len(commits) == 31
assert not root.exists(), 'use a fresh run directory'
git('clone', '--shared', '--no-checkout', '--quiet', '--', source, root, cwd=repo)
git('config', 'core.hooksPath', '/dev/null')

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def doc_stats(index):
    raw, _ = run([cli, 'stats', '.', '--index-dir', index, '--json'])
    value = json.loads(raw)
    return value, {d['path']: (i, d) for i, d in enumerate(value['documents']) if d['active'] and d['searchable']}

def mask_bit(byte):
    mask = (1 << 64) - 1
    value = (((0xcbf29ce484222325 ^ byte) * 0x1000000001b3) & mask) ^ 1
    value = ((value ^ (value >> 30)) * 0xbf58476d1ce4e5b9) & mask
    value = ((value ^ (value >> 27)) * 0x94d049bb133111eb) & mask
    return 1 << ((value ^ (value >> 31)) & 7)

bits = [mask_bit(i) for i in range(256)]
oracle_cache = {}
def expected_mask(path):
    data = path.read_bytes()
    key = hashlib.sha256(data).hexdigest()
    if key not in oracle_cache:
        masks = {}
        for pos in range(len(data) - 2):
            trigram = data[pos:pos+3]
            masks[trigram] = masks.get(trigram, 0) | (bits[data[pos+3]] if pos+3 < len(data) else 0)
        oracle_cache[key] = b''.join(trigram + bytes([masks[trigram]]) for trigram in sorted(masks, key=lambda x: int.from_bytes(x, 'little')))
    return oracle_cache[key]

report = dict(layout='batch' if batch_mode else 'per-document', commits=commits, variants={k: {'binary_sha256':sha(v[0]), 'mode':v[1]} for k,v in variants.items()}, rounds=[], verified_masks=0, verified_searches=0, methodology='30 consecutive first-parent commits; 3 full measured replays, rotating variant order; 3 warmup commits; fresh initial indexes per replay; refresh API timed without search')
query = r'def pending|def finish|def consume|def relay|_touch|def apply|decision.action|WAIT|wake'
batch_cache = {}
def verify(indexes, paths):
    snapshots = {}
    for mode, index in indexes.items():
        _, docs = doc_stats(index)
        snapshots[mode] = docs
    assert {p: d[1]['len'] for p,d in snapshots['baseline'].items()} == {p:d[1]['len'] for p,d in snapshots['persist'].items()}
    if batch_mode:
        seen, records = batch_cache.setdefault(str(indexes['persist']), (set(), {}))
        for file in sorted((indexes['persist'] / 'nextmask-batches').glob('*.mask')):
            if file.name in seen: continue
            data = file.read_bytes()
            cursor = 0
            while cursor < len(data):
                doc_id = int.from_bytes(data[cursor:cursor+4], 'little')
                size = int.from_bytes(data[cursor+4:cursor+8], 'little')
                assert size % 4 == 0 and cursor+8+size <= len(data)
                records[doc_id] = data[cursor+8:cursor+8+size]
                cursor += 8+size
            seen.add(file.name)
    for path in paths:
        if path not in snapshots['persist']:
            continue
        doc_id = snapshots['persist'][path][0]
        actual = records[doc_id] if batch_mode else (indexes['persist'] / 'nextmask-docs' / f'{doc_id:010}.mask').read_bytes()
        assert actual == expected_mask(root / path), path
        report['verified_masks'] += 1
    results = []
    for mode, index in indexes.items():
        raw, _ = run([cli, 'search', '-c', query, '.', '--index-dir', index, '--no-refresh'])
        results.append(sorted(raw.decode().splitlines()))
    assert all(r == results[0] for r in results)
    report['verified_searches'] += len(results)

for round_number in (-1, 0, 1, 2):
    git('checkout', '--quiet', '--detach', commits[0])
    indexes = {k: work / f'round-{round_number}-{k}' for k in variants}
    builds = {}
    for mode in ('baseline', 'persist'):
        binary, option = variants[mode]
        raw, wall = run([binary, 'build', root, indexes[mode], option])
        builds[mode] = dict(json.loads(raw), process_ms=wall)
    shutil.copytree(indexes['baseline'], indexes['compute'])
    initial_sizes = {k: sum(p.stat().st_size for p in index.rglob('*') if p.is_file()) for k,index in indexes.items()}
    print('round', round_number, 'built', {k:round(v['total_ms'],1) for k,v in builds.items()}, flush=True)
    record = dict(round=round_number, builds=builds, initial_sizes=initial_sizes, steps=[])
    report['rounds'].append(record)
    targets = commits[1:4] if round_number == -1 else commits[1:]
    for step, commit in enumerate(targets, 1):
        previous = commits[step-1]
        changes = git('diff', '--name-only', '--no-renames', '-z', previous, commit).decode().strip('\0').split('\0')
        numstat = git('diff', '--numstat', '--no-renames', previous, commit).decode()
        subject = git('show', '-s', '--format=%s', commit).decode().strip()
        git('checkout', '--quiet', '--detach', commit)
        changed_bytes = sum((root/p).stat().st_size for p in changes if (root/p).is_file())
        row = dict(step=step, commit=commit, subject=subject, changed_files=len(changes), changed_source_bytes=changed_bytes, paths=changes, numstat=numstat, measurements={})
        order = list(variants)
        offset = (step + round_number) % len(order)
        for mode in order[offset:] + order[:offset]:
            binary, option = variants[mode]
            raw, wall = run([binary, 'refresh', root, indexes[mode], option])
            row['measurements'][mode] = dict(json.loads(raw), process_ms=wall)
        outcomes = [m['outcome'] for m in row['measurements'].values()]
        assert len(set(outcomes)) == 1, outcomes
        verify(indexes, changes)
        record['steps'].append(row)
        (work / 'results.json').write_text(json.dumps(report, indent=2))
        if step % 5 == 0 or step == len(targets):
            print('round', round_number, 'step', step, 'files', len(changes), 'refresh_ms', {k:round(v['refresh_ms'],2) for k,v in row['measurements'].items()}, flush=True)
    record['final_sizes'] = {k: sum(p.stat().st_size for p in index.rglob('*') if p.is_file()) for k,index in indexes.items()}
    (work / 'results.json').write_text(json.dumps(report, indent=2))
(work / 'commands.json').write_text(json.dumps(commands, indent=2))
print('DONE', report['verified_masks'], 'masks', report['verified_searches'], 'searches', flush=True)
