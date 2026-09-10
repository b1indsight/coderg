import csv
import hashlib
import json
from pathlib import Path
import random
import statistics as st

repo=Path(__file__).resolve().parents[3]
work=Path(__file__).resolve().parent
data=repo/'benches/results/data/chromium-letter-pairs-2026-09-10'
BASELINES=['hash','english','chromium_all']
FAMILIES=['no_matches','scan_fallback','ignore_case','fixed_string','alternation','word_boundary','structural_regex']

def family(q):
    # Mutually exclusive; this precedence is fixed before computing outcomes.
    if q['category']=='no matches':return 'no_matches'
    if 'scan fallback' in q['category'] or q['id']=='hex_number':return 'scan_fallback'
    if '-i' in q['flags']:return 'ignore_case'
    if '-F' in q['flags']:return 'fixed_string'
    if '|' in q['pattern']:return 'alternation'
    if q['category'] in ['word boundary','common word']:return 'word_boundary'
    return 'structural_regex'

def ci(old,new,key):
    # Independent percentile bootstrap of the difference in medians, 5000 draws.
    seed=int.from_bytes(hashlib.sha256(key.encode()).digest()[:8],'little')
    rng=random.Random(seed)
    values=sorted((st.median(rng.choices(new,k=len(new)))/st.median(rng.choices(old,k=len(old)))-1)*100 for _ in range(5000))
    return values[125],values[4874]

rows=[]
for project in ['chromium','vllm','viberwhisper','agentflow']:
    r=json.loads((data/f'{project}.json').read_text())
    suffix='' if project=='chromium' else '_no_refresh'
    for q in r['queries']:
        new=q['samples_ms']['chromium_pairs'+suffix]
        for base in BASELINES:
            old=q['samples_ms'][base+suffix]
            before,after=st.median(old),st.median(new)
            change=(after/before-1)*100
            lo,hi=ci(old,new,f'{project}/{q["id"]}/{base}/20260910')
            rows.append(dict(project=project,id=q['id'],family=family(q),category=q['category'],
                pattern=q['pattern'],flags=' '.join(q['flags']),baseline=base,
                old_ms=before,new_ms=after,change_pct=change,saved_ms=before-after,
                ci_low_pct=lo,ci_high_pct=hi,
                median_faster=change<0,at_least_5pct_faster=change<=-5,
                interval_supports_gain=hi<0,
                clear_gain=change<=-5 and hi<0,
                clear_regression=change>=5 and lo>0,
                interval_inconclusive=lo<=0<=hi))

def summarize(subset):
    n=len(subset)
    result=dict(n=n)
    for key in ['median_faster','at_least_5pct_faster','interval_supports_gain','clear_gain','clear_regression','interval_inconclusive']:
        result[key]=sum(r[key] for r in subset)
        result[key+'_pct']=100*result[key]/n if n else None
    return result

summary={}
for base in BASELINES:
    subset=[r for r in rows if r['baseline']==base]
    overall=summarize(subset)
    groups={f:summarize([r for r in subset if r['family']==f]) for f in FAMILIES}
    for f,g in groups.items():
        g['share_of_clear_gains_pct']=100*g['clear_gain']/overall['clear_gain'] if overall['clear_gain'] else None
    summary[base]=dict(overall=overall,projects={p:summarize([r for r in subset if r['project']==p]) for p in ['chromium','vllm','viberwhisper','agentflow']},families=groups)
output=dict(input_sha256={name:hashlib.sha256((data/f'{name}.json').read_bytes()).hexdigest() for name in ['chromium','vllm','viberwhisper','agentflow']},definitions=dict(unit='query case, not internal gram',mode='no-refresh',bootstrap_draws=5000,
    threshold_pct=5,multiple_testing_adjustment=False,clear_gain='median change <= -5% and bootstrap 95% upper bound < 0',
    clear_regression='median change >= +5% and bootstrap 95% lower bound > 0',
    family_precedence=FAMILIES),summary=summary,rows=rows)
(work/'analysis.json').write_text(json.dumps(output,indent=2)+'\n')
with (work/'per-query.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
for base,s in summary.items():
    print(base,'overall',s['overall'])
    for p,v in s['projects'].items():print(' project',p,v)
    for fam,v in s['families'].items():print(' family',fam,v)
    print(' clear gains',[(r['project'],r['id'],round(r['change_pct'],2)) for r in rows if r['baseline']==base and r['clear_gain']])
    print(' clear regressions',[(r['project'],r['id'],round(r['change_pct'],2)) for r in rows if r['baseline']==base and r['clear_regression']])
print('CHROMIUM')
for r in rows:
    if r['project']=='chromium':print({k:r[k] for k in ['id','baseline','change_pct','ci_low_pct','ci_high_pct','clear_gain','clear_regression']})
