"""Check real-index candidate expansion against unchanged matching semantics."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

import benchmark as bench

fixture = Path(tempfile.mkdtemp(prefix='coderg-short-edges-', dir='/private/tmp'))
root = fixture / 'corpus'
root.mkdir()
index = fixture / 'index'
documents = [b'', b'i', b'if', b'qx', b'xx', b'if\n', b'qx\n']
for literal in (b'if', b'qx'):
    for adjacent in range(1, 256):
        documents.append(bytes([adjacent]) + literal)
        documents.append(literal + bytes([adjacent]))
    for context in (' ', '\t', '\n', '(', ')', '.', '。', 'é', '中', '\u2003'):
        documents.append(context.encode() + literal + context.encode())
for number, document in enumerate(documents):
    (root / f'{number:04}.txt').write_bytes(document)
subprocess.run([str(bench.BASELINE), 'index', str(root), '--index-dir', str(index)],
               check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
bench.ROOT, bench.INDEX = root, index
records = []
for query in (q for q in bench.QUERIES if q['id'] in ('if', 'qx', 'fixed_if', 'fixed_qx')):
    argv, env = bench.command(query, 'baseline', 'no_refresh')
    _, expected = bench.run(argv, env, True)
    for implementation in ('all', 'boundary'):
        argv, env = bench.command(query, implementation, 'no_refresh')
        _, actual = bench.run(argv, env, True)
        assert actual.returncode == expected.returncode and actual.stdout == expected.stdout
        assert not actual.stderr
        records.append(dict(query=query['id'], implementation=implementation,
                            output_sha256=hashlib.sha256(actual.stdout).hexdigest(), passed=True))
result = dict(fixture=str(fixture), documents=len(documents), checks=records,
              coverage='BOF, EOF, exact two-byte file, empty/one-byte file, bytes 1..255 on either side, Unicode contexts')
(bench.OUT / 'edge-checks.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
