import csv
import json
import pathlib
import random
import statistics

work = pathlib.Path(__file__).parent
directory = work / 'matrix'
metadata = json.loads((directory / 'metadata.json').read_text())
groups = []
rows = []
rng = random.Random(20260910)

def change(before, after):
    return (after / before - 1) * 100

for dataset in metadata['plan']['datasets']:
    data = json.loads((directory / dataset['id'] / 'results.json').read_text())
    comparable = [q for q in data['queries'] if q['rg_compatible']]
    modes = {}
    for mode in ('baseline', 'optimized', 'baseline_no_refresh', 'optimized_no_refresh', 'rg'):
        modes[mode] = {stat: statistics.mean(q['timings_ms'][mode][stat] for q in comparable)
                       for stat in ('median', 'p95')}
    changes = []
    for _ in range(1000):
        indices = rng.choices(range(metadata['iterations']), k=metadata['iterations'])
        aggregate = []
        for mode in ('baseline', 'optimized'):
            aggregate.append(statistics.mean(statistics.median(q['timings_ms'][mode]['samples'][i]
                             for i in indices) for q in comparable))
        changes.append(change(*aggregate))
    changes.sort()
    groups.append({'id': dataset['id'], 'files': data['builds']['baseline']['files'],
                   'snapshot_files': data['builds']['baseline'].get('snapshot_files'),
                   'queries': len(data['queries']), 'rg_compatible_queries': len(comparable),
                   'modes': modes, 'change_pct': change(modes['baseline']['median'], modes['optimized']['median']),
                   'bootstrap_95': [changes[25], changes[974]],
                   'faster_than_rg': sum(q['timings_ms']['optimized']['median'] < q['timings_ms']['rg']['median']
                                         for q in comparable)})
    for q in data['queries']:
        rows.append({'dataset': dataset['id'], 'query': q['id'], 'rg_compatible': q['rg_compatible'],
                     **{mode + '_ms': q['timings_ms'][mode]['median'] for mode in modes},
                     'change_pct': change(q['timings_ms']['baseline']['median'], q['timings_ms']['optimized']['median']),
                     'baseline_p95_ms': q['timings_ms']['baseline']['p95'],
                     'optimized_p95_ms': q['timings_ms']['optimized']['p95']})

(work / 'summary.json').write_text(json.dumps(groups, indent=2) + '\n')
with (work / 'per-query.csv').open('w') as file:
    writer = csv.DictWriter(file, fieldnames=list(rows[0]))
    writer.writeheader()
    writer.writerows(rows)
for g in groups:
    print(json.dumps(g), flush=True)
print('largest increases:', sorted(rows, key=lambda row: row['change_pct'], reverse=True)[:5])
