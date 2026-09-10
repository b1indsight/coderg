import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import re
import shutil
import statistics
import subprocess
import time

parser=argparse.ArgumentParser()
parser.add_argument('--iterations',type=int,default=31)
parser.add_argument('--warmup',type=int,default=3)
args=parser.parse_args()
assert args.iterations>0 and args.warmup>=0
work=Path(__file__).resolve().parent
repo=work.parents[2]
root=Path('/private/tmp/coderg-vllm-bench.7cZ1iI/vllm')
previous=repo/'.cache/2026-09-10/chromium-letter-pairs'
versions=['hash','english','chromium_all','chromium_pairs']
bins={v:previous/v for v in versions}
indexes={v:previous/'results/vllm'/(v+'-index') for v in versions}
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
env.pop('RIPGREP_CONFIG_PATH',None)
rg=shutil.which('rg')
assert rg

def sha(p):
    with Path(p).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def call(cmd,capture=False):
    started=time.perf_counter_ns()
    p=subprocess.run(list(map(str,cmd)),cwd=root,env=env,stdout=subprocess.PIPE if capture else subprocess.DEVNULL,stderr=subprocess.PIPE)
    elapsed=(time.perf_counter_ns()-started)/1e6
    assert p.returncode in (0,1),(cmd,p.returncode,p.stderr)
    assert not p.stderr,(cmd,p.stderr)
    return p,elapsed
def git_state():
    return {k:call(['git',*a],True)[0].stdout.decode().strip() for k,a in [
        ('commit',['rev-parse','HEAD']),('status',['status','--porcelain','--untracked-files=all'])]}
def save(path,obj):path.write_text(json.dumps(obj,indent=2)+'\n')

# Select before measuring: the earlier anchor plus eleven seeded identifiers
# from distinct files. Only public, ASCII snake_case Python declarations with
# 4--8 all-alphabetic components and at most 64 characters are eligible.
source=call([rg,'-n','--no-heading','--color','never','--glob','*.py',r'^\s*(?:async\s+)?def [a-z][a-z0-9_]+\(',root/'vllm'],True)[0].stdout.decode()
eligible={}
for line in source.splitlines():
    path,num,text=line.split(':',2)
    m=re.search(r'\bdef ([a-z][a-z0-9_]+)\(',text)
    if not m:continue
    name=m.group(1);words=name.split('_')
    if 4<=len(words)<=8 and all(w.isalpha() for w in words) and len(name)<=64:
        item=dict(identifier=name,path=str(Path(path).relative_to(root)),line=int(num),words=words)
        # Pick a deterministic declaration when a name occurs in many files.
        if name not in eligible or (item['path'],item['line'])<(eligible[name]['path'],eligible[name]['line']):eligible[name]=item
anchor='get_tensor_model_parallel_world_size'
chosen=[eligible[anchor]];used={chosen[0]['path']}
selection_rng=random.Random(20260910)
pool=[eligible[n] for n in sorted(eligible) if n!=anchor];selection_rng.shuffle(pool)
for item in pool:
    if item['path'] in used:continue
    chosen.append(item);used.add(item['path'])
    if len(chosen)==12:break
assert len(chosen)==12
queries=[]
word_parents={}
for item in chosen:
    name=item['identifier'];words=list(dict.fromkeys(item['words']))
    queries.append(dict(id='identifier:'+name,family='identifier',pattern=name,flags=['-F'],parents=[name],words=words))
    for word in words:word_parents.setdefault(word,[]).append(name)
    for family,terms in [('or_two',[words[0],words[-1]]),('or_all',words)]:
        queries.append(dict(id=family+':'+name,family=family,pattern='(?:'+'|'.join(terms)+')',flags=[],parents=[name],words=terms))
for word,parents in sorted(word_parents.items()):
    queries.append(dict(id='word:'+word,family='word',pattern=word,flags=['-F'],parents=parents,words=[word]))
assert len(queries)==85 and len(word_parents)==49
assert len({(q['pattern'],tuple(q['flags'])) for q in queries})==len(queries)
for q in queries:q['has_short_word']=any(len(w)<3 for w in q['words'])
plan=dict(root=str(root),selection_seed=20260910,eligible_identifiers=len(eligible),identifiers=chosen,queries=queries,
    rules='Case-sensitive; full name uses -F substring semantics; identifier words split on underscore; word queries deduplicated; OR-two uses first and last distinct word; OR-all uses all distinct words; no word boundaries; no timing-informed selection')
assert not (work/'results.json').exists()
save(work/'plan.json',plan)
before=git_state();assert not before['status'],before
binary_hashes={v:sha(p) for v,p in bins.items()}
source_hashes={str(p.relative_to(repo)):sha(p) for p in [repo/'Cargo.toml',repo/'Cargo.lock',*sorted((repo/'src').glob('*.rs'))]}
index_hashes={str(p):sha(p) for index in indexes.values() for p in index.rglob('*') if p.is_file()}
sizes={}
for v in versions:
    p,_=call([bins[v],'stats',root,'--index-dir',indexes[v],'--json'],True)
    manifest=json.loads(p.stdout)
    assert manifest['version']==dict(hash=4,english=5,chromium_all=6,chromium_pairs=8)[v]
    sizes[v]=sum(p.stat().st_size for p in indexes[v].rglob('*') if p.is_file())
meta=dict(platform=platform.platform(),logical_cpus=os.cpu_count(),git_before=before,
    binary_sha256=binary_hashes,source_sha256=source_hashes,index_sha256=index_hashes,index_bytes=sizes,
    previous_environment=json.loads((previous/'results/metadata.json').read_text()),
    environment={k:env.get(k) for k in ['RAYON_NUM_THREADS','LANG','LC_ALL']},
    seed=20260910,iterations=args.iterations,warmup=args.warmup,harness_sha256=sha(__file__),
    protocol='Reuse four immutable indexes; full-output parity against rg outside timing; randomized four-way no-refresh timings; no builds or RSS measurements in this run')
result=dict(metadata=meta,queries=[])
print('Selected',len(chosen),'identifiers,',len(word_parents),'words,',len(queries),'queries',flush=True)
for i,q in enumerate(queries):
    commands={v:[str(bins[v]),'search',*q['flags'],q['pattern'],'.','--index-dir',str(indexes[v]),'--no-refresh'] for v in versions}
    commands['rg']=[rg,'--no-config','-n','-H','--hidden','--glob','!.git/**','--glob','!.coderg-index/**','--color','never','--no-heading',*q['flags'],q['pattern'],'.']
    expected=None;parity={}
    for v in ['rg',*versions]:
        p,_=call(commands[v],True)
        lines=sorted(line.removeprefix(b'./') for line in p.stdout.splitlines())
        observed=(p.returncode,lines)
        if expected is None:expected=observed
        assert observed==expected,(q['id'],v)
        parity[v]=dict(exit_code=p.returncode,lines=len(lines),sha256=hashlib.sha256(b'\n'.join(lines)).hexdigest())
    assert parity['rg']['lines']>0,q['id']
    result['queries'].append(dict(**q,commands=commands,parity=parity,samples_ms={v:[] for v in versions}))
    save(work/'results.json',result)
    if (i+1)%10==0 or i+1==len(queries):print('Parity',i+1,'/',len(queries),flush=True)
jobs=[(q,v) for q in result['queries'] for v in versions]
rng=random.Random(20260910)
for repeat in range(args.warmup+args.iterations):
    rng.shuffle(jobs)
    for q,v in jobs:
        p,elapsed=call(q['commands'][v])
        assert p.returncode==q['parity'][v]['exit_code']
        if repeat>=args.warmup:q['samples_ms'][v].append(elapsed)
    save(work/'results.json',result)
    if repeat>=args.warmup and ((repeat-args.warmup+1)%5==0 or repeat+1==args.warmup+args.iterations):
        print('Timed',repeat-args.warmup+1,'/',args.iterations,flush=True)
after=git_state();assert before==after
assert binary_hashes=={v:sha(p) for v,p in bins.items()}
assert index_hashes=={str(p):sha(p) for index in indexes.values() for p in index.rglob('*') if p.is_file()}
assert all(sha(repo/p)==h for p,h in source_hashes.items())
result['validation']=dict(all_outputs_match_rg=True,source_unchanged=True,indexes_unchanged=True,binaries_unchanged=True,git_after=after)
save(work/'results.json',result)
print('Complete',flush=True)
