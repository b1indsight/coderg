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
import sys
import time

sys.dont_write_bytecode = True
from refresh_matrix import queries_for

work = Path(__file__).resolve().parent
repo = work.parents[2]
out = work / 'results'
out.mkdir(exist_ok=False)
versions = ['hash', 'english', 'chromium_all', 'chromium_pairs']
bins = {v: work/v for v in versions}
rng = random.Random(20260910)
env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
env.pop('RIPGREP_CONFIG_PATH', None)
rg = shutil.which('rg')
assert rg

def sha(path):
    with Path(path).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def run(cmd, root, capture=False, rss=False):
    cmd = list(map(str, cmd))
    if rss:
        cmd = ['/usr/bin/time', '-l', *cmd]
    start = time.perf_counter_ns()
    p = subprocess.run(cmd, cwd=root, env=env, stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                       stderr=subprocess.PIPE)
    ms = (time.perf_counter_ns()-start)/1e6
    assert p.returncode in (0, 1), (cmd, p.returncode, p.stderr)
    memory = None
    if rss:
        m = re.search(rb'(\d+)\s+maximum resident set size', p.stderr)
        assert m, p.stderr
        memory = int(m[1])
    return p, ms, memory

def dump(path, obj):
    path.write_text(json.dumps(obj, indent=2)+'\n')

def git_state(root):
    result = {}
    for key, args in [('commit',['rev-parse','HEAD']), ('status',['status','--porcelain','--untracked-files=all'])]:
        p, _, _ = run(['git', *args], root, capture=True)
        assert p.returncode == 0
        result[key] = p.stdout.decode().strip()
    assert not result['status'], result
    return result

source_hashes = {str(p.relative_to(repo)):sha(p) for p in [repo/'Cargo.toml', repo/'Cargo.lock', *sorted((repo/'src').glob('*.rs'))]}
binary_hashes = {v:sha(p) for v,p in bins.items()}
assert len(set(binary_hashes.values())) == 4
meta = dict(platform=platform.platform(), cpu=run(['sysctl','-n','machdep.cpu.brand_string'],repo,True)[0].stdout.decode().strip(),
    memory_bytes=int(run(['sysctl','-n','hw.memsize'],repo,True)[0].stdout),
    binary_sha256=binary_hashes, source_sha256=source_hashes, counts_sha256=sha(work/'counts.json'),
    tables=json.loads((work/'tables.json').read_text()),
    harness_sha256=sha(__file__), seed=20260910, logical_cpus=os.cpu_count(),
    environment={k:env.get(k) for k in ['RAYON_NUM_THREADS','LANG','LC_ALL']},
    rg_version=run([rg,'--version'],repo,True)[0].stdout.decode().strip(),
    rustc_version=run(['rustc','--version'],repo,True)[0].stdout.decode().strip())
dump(out/'metadata.json',meta)
plan=json.loads((repo/'.cache/2026-09-10/letter-frequency-bench/plan.json').read_text())['datasets']
plan += [dict(id='chromium', kind='chromium',root='/private/tmp/coderg-chromium-3986304/src',
              commit='398630472335c10b9ca610a4d1b7888a040f702a')]
chromium_queries = [
    ('max_file_size','fixed string','MAX_FILE_SIZE',['-F']),
    ('weak_ptr_factory','fixed string','WeakPtrFactory',['-F']),
    ('icase_weak_ptr_factory','ignore case','WeakPtrFactory',['-i','-F']),
    ('bind_once','fixed string','base::BindOnce',['-F']),
    ('histogram','alternation',r'\bUmaHistogram(Boolean|Enumeration)\b',[]),
    ('absent','no matches','CODERG_DISTRIBUTION_ABSENT_92be7',['-F']),
]
for dataset in plan:
    name=dataset['id'];root=Path(dataset['root']);large=name=='chromium'
    directory=out/name;directory.mkdir()
    result=dict(dataset=dataset,builds={},queries=[],default_probe={},query_iterations=15,
        query_warmup=2,build_iterations=1 if large else 5,build_rss_runs=1 if large else 3)
    if not large: result['git_before']=git_state(root)
    indexes={v:directory/(v+'-index') for v in versions}
    order=list(versions);rng.shuffle(order)
    for v in order:
        reused = large and v != 'chromium_pairs'
        if reused:
            previous = repo/'.cache/2026-09-10/chromium-letter-distributions/results/chromium'
            indexes[v] = previous/(v+'-index')
            reference = json.loads((previous/'results.json').read_text())['builds'][v]
            elapsed = reference['initial_ms']
            memory = reference['rss_bytes'][0]
            print(name, 'reuse unchanged control index', v, flush=True)
        else:
            print(name, 'initial build', v, flush=True)
            p,elapsed,memory=run([bins[v],'index',root,'--index-dir',indexes[v]],root,rss=large)
            assert p.returncode==0
        p,_,_=run([bins[v],'stats',root,'--index-dir',indexes[v],'--json'],root,capture=True)
        m=json.loads(p.stdout)
        assert m['version']==dict(hash=4,english=5,chromium_all=6,chromium_pairs=8)[v]
        docs=[d for d in m['documents'] if d['active'] and d['searchable']]
        b=dict(build_reused=reused,initial_ms=elapsed,files=len(docs),source_bytes=sum(d['len'] for d in docs),
            ngrams=sum(s['ngrams'] for s in m['segments']),
            index_bytes=sum(p.stat().st_size for p in indexes[v].rglob('*') if p.is_file()),
            samples_ms=[elapsed] if large else [],rss_bytes=[memory] if large else [],rss_logs=[])
        result['builds'][v]=b
        del docs,m,p
        dump(directory/'results.json',result)
    assert len({(b['files'],b['source_bytes']) for b in result['builds'].values()})==1
    if large:
        counts=json.loads((work/'counts.json').read_text())['counts']
        assert b['files']==counts['files'] and b['source_bytes']==counts['source_bytes']
    else:
        for kind,count in [('time',5),('rss',3)]:
            for repeat in range(count):
                rng.shuffle(order)
                for v in order:
                    scratch=directory/'build-scratch'
                    p,elapsed,memory=run([bins[v],'index',root,'--index-dir',scratch],root,rss=kind=='rss')
                    assert p.returncode==0
                    if kind=='rss':
                        result['builds'][v]['rss_bytes'].append(memory)
                        result['builds'][v]['rss_logs'].append(p.stderr.decode())
                    else: result['builds'][v]['samples_ms'].append(elapsed)
                    shutil.rmtree(scratch)
    print(name,'builds complete', {v:round(b['index_bytes']/2**20,2) for v,b in result['builds'].items()},flush=True)
    snapshot={str(p):sha(p) for index in indexes.values() for p in index.rglob('*') if p.is_file()}
    queries=chromium_queries if large else queries_for(dataset['kind'])
    modes=versions if large else [v+s for v in versions for s in ['', '_no_refresh']]+['rg']
    for query_id,category,pattern,flags in queries:
        commands={}
        for mode in modes:
            if mode=='rg':continue
            v=mode.removesuffix('_no_refresh')
            commands[mode]=[bins[v],'search',*flags,pattern,'.','--index-dir',indexes[v]]
            if large or mode.endswith('_no_refresh'):commands[mode]+=['--no-refresh']
        rg_cmd=[rg,'--no-config','-n','-H','--hidden','--glob','!.git/**','--glob','!.coderg-index/**',
                '--color','never','--no-heading',*flags,pattern,'.']
        commands['rg']=rg_cmd
        parity={};expected=None
        for mode in ['rg',*[m for m in commands if m!='rg']]:
            p,_,_=run(commands[mode],root,capture=True)
            assert not p.stderr, p.stderr
            lines=sorted(line.removeprefix(b'./') for line in p.stdout.splitlines())
            actual=(p.returncode,lines)
            if expected is None:expected=actual
            assert actual==expected,(name,query_id,mode)
            parity[mode]=dict(exit_code=p.returncode,lines=len(lines),sha256=hashlib.sha256(b'\n'.join(lines)).hexdigest())
        result['queries'].append(dict(id=query_id,category=category,pattern=pattern,flags=flags,
            parity=parity,commands={m:list(map(str,c)) for m,c in commands.items()},samples_ms={m:[] for m in modes}))
        dump(directory/'results.json',result)
    print(name,'parity',len(queries),'/',len(queries),'including rg',flush=True)
    jobs=[(q,m) for q in result['queries'] for m in modes]
    for repeat in range(17):
        rng.shuffle(jobs)
        for q,mode in jobs:
            p,elapsed,_=run(q['commands'][mode],root)
            assert not p.stderr and p.returncode==q['parity'][mode]['exit_code'],(name,q['id'],mode,p.stderr)
            if repeat>=2:q['samples_ms'][mode].append(elapsed)
        dump(directory/'results.json',result)
        if repeat>=2 and (repeat-1)%5==0:print(name,'timed',repeat-1,'/ 15',flush=True)
    if large:
        # Separately expose the refresh-dominated default cost on one query.
        q=result['queries'][0]
        result['default_probe']={v:[] for v in versions}
        for repeat in range(6):
            rng.shuffle(order)
            for v in order:
                cmd=[s for s in q['commands'][v] if s!='--no-refresh']
                p,elapsed,_=run(cmd,root,capture=True)
                lines=sorted(line.removeprefix(b'./') for line in p.stdout.splitlines())
                assert not p.stderr and p.returncode==q['parity'][v]['exit_code']
                assert hashlib.sha256(b'\n'.join(lines)).hexdigest()==q['parity'][v]['sha256']
                if repeat:result['default_probe'][v].append(elapsed)
    else:
        result['git_after']=git_state(root)
        assert result['git_after']==result['git_before']
    assert snapshot=={str(p):sha(p) for index in indexes.values() for p in index.rglob('*') if p.is_file()}
    assert binary_hashes=={v:sha(p) for v,p in bins.items()}
    assert all(sha(repo/p)==digest for p,digest in source_hashes.items())
    result['validation']=dict(parity=True,indexes_unchanged=True,binaries_unchanged=True,source_unchanged=True)
    dump(directory/'results.json',result)
    print(name,'complete',flush=True)
