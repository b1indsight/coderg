import csv
import json
from pathlib import Path
import random
import statistics as st

work=Path(__file__).resolve().parent
versions=['hash','english','chromium_all','chromium_initial']
summary={};rows=[]
rng=random.Random(20260910)
def pct(a,b):return (b/a-1)*100
def ci(queries,old,new):
    boot=[]
    for _ in range(1000):
        means=[st.mean(st.median(rng.choices(q['samples_ms'][m],k=len(q['samples_ms'][m]))) for q in queries) for m in [old,new]]
        boot.append(pct(*means))
    boot.sort()
    return [boot[25],boot[974]]
for name in ['vllm','viberwhisper','agentflow','chromium']:
    r=json.loads((work/'results'/name/'results.json').read_text())
    assert all(r['validation'].values())
    builds={v:dict(build_ms=st.median(b['samples_ms']),rss_mib=st.median(b['rss_bytes'])/2**20,
        index_mib=b['index_bytes']/2**20,ngrams=b['ngrams'],files=b['files'],source_mib=b['source_bytes']/2**20)
        for v,b in r['builds'].items()}
    qs=r['queries'];modes=list(qs[0]['samples_ms'])
    assert all(all(len(q['samples_ms'][m])==15 for m in modes) for q in qs)
    means={m:st.mean(st.median(q['samples_ms'][m]) for q in qs) for m in modes}
    result=dict(builds=builds,queries=len(qs),mean_query_medians_ms=means,comparisons={})
    suffix='' if name=='chromium' else '_no_refresh'
    for new in ['chromium_all','chromium_initial']:
        for old in ['hash','english']:
            a=old+suffix;b=new+suffix
            result['comparisons'][new+'_vs_'+old]=dict(no_refresh_change_pct=pct(means[a],means[b]),
                bootstrap_95_pct=ci(qs,a,b),index_change_pct=pct(builds[old]['index_mib'],builds[new]['index_mib']),
                build_change_pct=pct(builds[old]['build_ms'],builds[new]['build_ms']),
                faster_queries=sum(st.median(q['samples_ms'][b])<st.median(q['samples_ms'][a]) for q in qs))
    if r['default_probe']:result['default_probe_ms']={v:st.median(a) for v,a in r['default_probe'].items()}
    for q in qs:
        row=dict(project=name,id=q['id'],pattern=q['pattern'],flags=' '.join(q['flags']))
        for v in versions:row[v]=st.median(q['samples_ms'][v+suffix])
        for v in versions[2:]:
            row[v+'_vs_english_pct']=pct(row['english'],row[v])
            row[v+'_vs_hash_pct']=pct(row['hash'],row[v])
        rows.append(row)
    summary[name]=result
(work/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
with (work/'per-query.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
tables=json.loads((work/'tables.json').read_text())
english=[8167,1492,2782,4253,12702,2228,2015,6094,6996,153,772,4025,2406,6749,7507,1929,95,5987,6327,9056,2758,978,2360,150,1974,74]
with (work/'frequencies.csv').open('w') as f:
    writer=csv.writer(f)
    writer.writerow(['letter','english_pct','chromium_all_pct','chromium_initial_pct','all_count','initial_count','all_weight_frequency','initial_weight_frequency'])
    for i in range(26):
        a=tables['chromium_all'];b=tables['chromium_initial']
        writer.writerow([chr(i+97),english[i]/1000,a['percent'][i],b['percent'][i],a['counts'][i],b['counts'][i],a['frequencies'][i],b['frequencies'][i]])
for n,s in summary.items():
    print(n,'mean no-refresh medians',s['mean_query_medians_ms'])
    print('builds',s['builds'])
    print('comparisons',s['comparisons'])
    if 'default_probe_ms' in s:print('default probe',s['default_probe_ms'])
    for v in versions[2:]:
        key=v+'_vs_english_pct'
        subset=sorted([r for r in rows if r['project']==n],key=lambda r:r[key])
        print(v,'gains/regressions',[(r['id'],round(r[key],2)) for r in subset[:3]+subset[-3:]])
