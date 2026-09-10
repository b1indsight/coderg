import csv
import json
from pathlib import Path
import random
import statistics as st

work = Path(__file__).resolve().parent
root = work / 'results-full'
rng = random.Random(20260910)

def change(before, after):
    return 100 * (after / before - 1)

def bootstrap(queries, before, after):
    values = []
    for _ in range(1000):
        means = []
        for mode in (before, after):
            means.append(st.mean(st.median(rng.choices(q['timings_ms'][mode]['samples'],
                k=len(q['timings_ms'][mode]['samples']))) for q in queries))
        values.append(change(*means))
    values.sort()
    return [values[25], values[974]]

summary = {}
rows = []
for name in ['vllm', 'viberwhisper', 'agentflow']:
    r = json.loads((root / name / 'results.json').read_text())
    queries = r['queries']
    assert all(q['rg_compatible'] for q in queries)
    modes = list(queries[0]['timings_ms'])
    avg = {mode: st.mean(q['timings_ms'][mode]['median'] for q in queries) for mode in modes}
    builds = {}
    for version, b in r['builds'].items():
        builds[version] = dict(ms=b['timing']['median'], index_mib=b['index_bytes']/2**20,
            rss_mib=b['rss']['median']/2**20, ngrams=b['ngrams'],
            source_mib=b['source_bytes']/2**20, files=b['files'])
    delta = {key: change(builds['baseline'][key], builds['optimized'][key])
        for key in ['ms', 'index_mib', 'rss_mib', 'ngrams']}
    result = dict(builds=builds, build_change_pct=delta, query_count=len(queries),
        mean_query_medians_ms=avg, validation=r['validation'])
    for suffix in ['', '_no_refresh']:
        old, new = 'baseline'+suffix, 'optimized'+suffix
        result[new] = dict(change_pct=change(avg[old],avg[new]),
            bootstrap_95_pct=bootstrap(queries,old,new),
            faster_queries=sum(q['timings_ms'][new]['median'] < q['timings_ms'][old]['median'] for q in queries))
    for q in queries:
        med = {m:q['timings_ms'][m]['median'] for m in modes}
        rows.append(dict(project=name,id=q['id'],pattern=q['pattern'],flags=' '.join(q['flags']),
            **med,default_change_pct=change(med['baseline'],med['optimized']),
            no_refresh_change_pct=change(med['baseline_no_refresh'],med['optimized_no_refresh'])))
    summary[name] = result
(work/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
with (work/'per-query.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=list(rows[0]))
    writer.writeheader()
    writer.writerows(rows)
print(json.dumps(summary,indent=2))
for name in summary:
    subset=[r for r in rows if r['project']==name]
    print(name, 'largest no-refresh regressions and gains')
    for row in sorted(subset,key=lambda r:r['no_refresh_change_pct'],reverse=True)[:3] + sorted(subset,key=lambda r:r['no_refresh_change_pct'])[:3]:
        print(row['id'],*[round(row[k],3) for k in ['baseline_no_refresh','optimized_no_refresh','no_refresh_change_pct']])
