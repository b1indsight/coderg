import csv
import hashlib
import json
from pathlib import Path
import random
import statistics as st

work = Path(__file__).resolve().parent
data = json.loads((work / 'results.json').read_text())
plan = json.loads((work / 'plan.json').read_text())
assert all(data['validation'][k] for k in ['all_outputs_match_rg', 'source_unchanged', 'indexes_unchanged', 'binaries_unchanged'])
versions = ['hash', 'english', 'chromium_all', 'chromium_pairs']
bases = versions[:-1]
rows = []
for q in data['queries']:
    samples = q['samples_ms']
    assert all(len(samples[v]) == 31 for v in versions)
    assert len({(p['exit_code'], p['lines'], p['sha256']) for p in q['parity'].values()}) == 1
    row = {k: q[k] for k in ['id', 'family', 'pattern', 'parents', 'has_short_word']}
    row['matches'] = q['parity']['rg']['lines']
    # Short components do not force a full identifier query to scan.
    row['short_branch'] = q['family'] != 'identifier' and q['has_short_word']
    row['median_ms'] = {v: st.median(samples[v]) for v in versions}
    row['p95_ms'] = {v: sorted(samples[v])[29] for v in versions}
    row['comparisons'] = {}
    for base in bases:
        seed = int.from_bytes(hashlib.sha256((q['id'] + base).encode()).digest()[:8], 'big')
        rng = random.Random(seed)
        changes = sorted(100 * (st.median(rng.choices(samples['chromium_pairs'], k=31)) /
                                st.median(rng.choices(samples[base], k=31)) - 1) for _ in range(5000))
        change = 100 * (row['median_ms']['chromium_pairs'] / row['median_ms'][base] - 1)
        lo, hi = changes[125], changes[4874]
        row['comparisons'][base] = dict(change_pct=change, ci95=[lo, hi],
            clear_gain=change <= -5 and hi < 0, clear_regression=change >= 5 and lo > 0)
    rows.append(row)

def summarize(group):
    means = {v: st.mean(r['median_ms'][v] for r in group) for v in versions}
    return dict(n=len(group), mean_median_ms=means, comparisons={b: dict(
        change_pct=100 * (means['chromium_pairs'] / means[b] - 1),
        faster=sum(r['comparisons'][b]['change_pct'] < 0 for r in group),
        clear_gains=sum(r['comparisons'][b]['clear_gain'] for r in group),
        clear_regressions=sum(r['comparisons'][b]['clear_regression'] for r in group)) for b in bases})

groups = {f: summarize([r for r in rows if r['family'] == f]) for f in ['identifier', 'word', 'or_two', 'or_all']}
splits = {}
for family in ['word', 'or_two', 'or_all']:
    for short in [False, True]:
        group = [r for r in rows if r['family'] == family and r['short_branch'] == short]
        if group:
            splits[family + ('_short' if short else '_3plus')] = summarize(group)
summary = dict(index_bytes=data['metadata']['index_bytes'], groups=groups, splits=splits, queries=rows)
(work / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
with (work / 'per-query.csv').open('w', newline='') as f:
    fields = ['id', 'family', 'pattern', 'matches', 'short_branch', *versions,
              *[b + suffix for b in bases for suffix in ['_change_pct', '_ci_lo', '_ci_hi', '_clear_gain', '_clear_regression']]]
    writer = csv.DictWriter(f, fieldnames=fields)
    writer.writeheader()
    for r in rows:
        record = {k: r[k] for k in fields[:5]}
        record.update(r['median_ms'])
        for b, c in r['comparisons'].items():
            record.update({b + '_change_pct': c['change_pct'], b + '_ci_lo': c['ci95'][0],
                           b + '_ci_hi': c['ci95'][1], b + '_clear_gain': c['clear_gain'], b + '_clear_regression': c['clear_regression']})
        writer.writerow(record)
print(json.dumps({k: v for k, v in summary.items() if k != 'queries'}, indent=2))
