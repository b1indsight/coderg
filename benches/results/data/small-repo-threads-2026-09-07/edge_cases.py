import json
import pathlib
import subprocess

work = pathlib.Path(__file__).parent
root = work / 'edge-tree'
root.mkdir()
(root / 'nested').mkdir()
(root / 'keep.txt').write_text('needle\n')
(root / '.ignore').write_text('*.skip\n')
(root / 'hidden.skip').write_text('needle\n')
indexes = {mode: work / f'edge-{mode}-index' for mode in ('before', 'after')}
for mode, index in indexes.items():
    subprocess.run([str(work / mode), 'index', str(root), '--index-dir', str(index)],
                   capture_output=True, check=True)

results = []
def check(name, expected_files):
    outputs = {}
    for mode, index in indexes.items():
        command = [str(work / mode), 'search', '-l', 'needle', str(root), '--index-dir', str(index)]
        proc = subprocess.run(command, capture_output=True, check=True)
        files = sorted(proc.stdout.decode().splitlines())
        assert len(files) == expected_files, (name, mode, len(files))
        outputs[mode] = {'files': files, 'stderr': proc.stderr.decode()}
    assert outputs['before'] == outputs['after'], name
    results.append({'stage': name, 'matching_files': expected_files,
                    'outputs_equal': True, 'stderr': outputs['after']['stderr']})

check('clean_small', 1)
for number in range(600):
    (root / f'nested/{number:04}.txt').write_text('needle\n')
check('grow_past_threshold', 601)
check('clean_large', 601)
for number in range(599):
    (root / f'nested/{number:04}.txt').unlink()
check('shrink_below_threshold', 2)
check('clean_small_again', 2)
(root / '.ignore').write_text('[unterminated\n')
check('invalid_ignore_rule', 3)
(root / '.ignore').write_text('*.skip\n')
check('restore_ignore_rule', 2)
(work / 'edge-cases.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps(results, indent=2))
