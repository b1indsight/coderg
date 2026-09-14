"""Run from the repository root to validate archived benchmark evidence."""
from pathlib import Path
import gzip
import hashlib
import json
import re
import shutil

root = Path.cwd()
data = root / 'benches/results/data/nextmask-false-positive-2026-09-14'
checked = 0
for corpus, count in [('vllm', 34), ('chromium', 14)]:
    rows = json.loads((data / corpus / 'summary.json').read_text())
    metadata = json.loads((data / corpus / 'metadata.json').read_text())
    assert len(rows) == count
    for path, expected in metadata['source_sha256'].items():
        assert hashlib.sha256((root / path).read_bytes()).hexdigest() == expected, path
    for row in rows:
        report = json.loads(gzip.decompress((data / corpus / (row['id'] + '.json.gz')).read_bytes()))
        matches = report['matches']
        expected_hash = hashlib.sha256(json.dumps(matches, sort_keys=True).encode()).hexdigest()
        assert all(p['sha256'] == expected_hash for p in row['parity'].values())
        assert len(report['samples']) == metadata['iterations'] * 2
        for sample in report['samples']:
            assert sample['matched_files'] == len(matches)
            assert sample['matched_lines'] == sum(matches.values())
            assert sample['candidates'] == sample['matched_files'] + sample['false_positive_files']
            assert 0 <= sample['false_positive_match_worker_ms'] <= sample['match_worker_ms']
            assert 0 <= sample['false_positive_read_worker_ms'] <= sample['read_worker_ms']
            assert 0 <= sample['scan_wall_ms'] <= sample['total_ms']
        checked += 1
report_path = root / 'benches/results/nextmask-false-positive-2026-09-14.md'
for line in report_path.read_text().splitlines():
    if line.startswith('- ['):
        for link in re.findall(r'\]\(([^)]+)\)', line):
            assert (report_path.parent / link).exists(), link
result = dict(queries=checked, source_hashes='passed', raw_samples='passed',
              parity_hashes='passed', report_links='passed')
(data / 'validation.json').write_text(json.dumps(result, indent=2) + '\n')
if Path(__file__).resolve() != data / 'verify.py':
    shutil.copy2(__file__, data / 'verify.py')
print(json.dumps(result))
