import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess

work=Path(__file__).resolve().parent
repo=work.parents[2]
counts=json.loads((work/'counts.json').read_text())
tables={}
for mode,version in [('all',6),('initial',7)]:
    raw=counts['counts'][mode]
    total=sum(raw)
    scaled=[n*100000//total for n in raw]
    remainders=[n*100000%total for n in raw]
    for i in sorted(range(26),key=lambda i:(-remainders[i],i))[:100000-sum(scaled)]:scaled[i]+=1
    assert sum(scaled)==100000 and min(scaled)>0 and max(scaled)<65536
    name='chromium_'+mode
    tables[name]=dict(counts=raw,total=total,frequencies=scaled,common_frequency=max(scaled),index_version=version,
        percent=[100*n/total for n in raw])
    src=work/(name+'-src');src.mkdir()
    for file in ['Cargo.toml','Cargo.lock']:shutil.copy2(repo/file,src/file)
    shutil.copytree(repo/'src',src/'src')
    (src/'benches').mkdir()
    for file in (repo/'benches').glob('*.rs'):shutil.copy2(file,src/'benches'/file.name)
    p=src/'src/ngram.rs';s=p.read_text()
    start=s.index('// English letter frequencies')
    end=s.index('\n#[derive(Default)]',start)
    s=s[:start]+f'// Chromium ASCII {mode} letter counts; fixed normalized frequencies per 100000.\n'+\
        '// Derived from counts.json; no document-context-dependent query weights.\n'+\
        'const LETTER_FREQUENCIES: [u32; 26] = ['+', '.join(map(str,scaled))+'];\n'+\
        f'const COMMON_FREQUENCY: u32 = {max(scaled)};\n'+s[end:]
    # The exhaustive ordering checks below apply to every prior. These two
    # fixed examples encode English-specific frequency ordering only.
    s=s.replace('        assert!(pair_weight(b"qz") > pair_weight(b"th"));\n','')
    s=s.replace('        assert!(pair_weight(b"th") > pair_weight(b"ee"));\n','')
    p.write_text(s)
    p=src/'src/index.rs';s=p.read_text().replace('const VERSION: u32 = 5;',f'const VERSION: u32 = {version};')
    p.write_text(s)
    (work/(name+'-ngram.rs')).write_bytes((src/'src/ngram.rs').read_bytes())
    print(name,scaled,'max',max(scaled))
(work/'tables.json').write_text(json.dumps(tables,indent=2)+'\n')

commands=[]
env=dict(__import__('os').environ,CARGO_TARGET_DIR=str(repo/'target'))
for name in tables:
    manifest=work/(name+'-src')/'Cargo.toml'
    cmd=['cargo','test','--locked','--offline','--manifest-path',str(manifest),'--bin','coderg']
    commands.append(cmd)
    subprocess.run(cmd,cwd=repo,env=env,check=True)
    cmd=['cargo','rustc','--release','--locked','--offline','--manifest-path',str(manifest),'--bin','coderg','--','--cfg','benchmark_'+name]
    commands.append(cmd)
    subprocess.run(cmd,cwd=repo,env=env,check=True)
    shutil.copy2(repo/'target/release/coderg',work/name)
(work/'build-commands.json').write_text(json.dumps(commands,indent=2)+'\n')
