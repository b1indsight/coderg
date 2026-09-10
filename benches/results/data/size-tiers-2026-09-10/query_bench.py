import json,subprocess,time,statistics,math
from pathlib import Path
p=Path(__file__).resolve().parent
rows=[]
for tier,queries in [("small",["whisper","viberwhisper"]),("medium",["whisper_context"]),("large",["SamplingParams"])]:
    h=json.loads((p/(tier+".json")).read_text());root=Path(h["workspace"])/"source"
    for query in queries:
        reference=None
        for mode in ["default","no-refresh"]:
            commands={v["name"]:[v["binary"],"search","-F","-l",query,str(root),"--index-dir",v["index"]]+(["--no-refresh"] if mode=="no-refresh" else []) for v in h["variants"]}
            if mode=="default":commands["rg"]=["rg","--no-config","--hidden","-g","!.git","-F","-l",query,"."]
            for version,cmd in commands.items():
                samples=[]
                for i in range(11):
                    started=time.perf_counter_ns();r=subprocess.run(cmd,cwd=root,stdout=subprocess.PIPE,stderr=subprocess.PIPE);elapsed=(time.perf_counter_ns()-started)/1e6
                    assert r.returncode in (0,1),(cmd,r.stderr)
                    files=sorted(line.removeprefix("./") for line in r.stdout.decode().splitlines())
                    if reference is None:reference=files
                    assert files==reference,(tier,query,mode,version)
                    if i:samples.append(elapsed)
                row=dict(tier=tier,query=query,mode=mode,version=version,files=len(files),samples_ms=samples,median_ms=statistics.median(samples),p95_ms=sorted(samples)[math.ceil(.95*len(samples))-1],command=cmd,commit=h["end_commit"])
                if version!="rg":
                    v=next(v for v in h["variants"] if v["name"]==version)
                    m=json.loads(subprocess.check_output([v["binary"],"stats",str(root),"--index-dir",v["index"],"--json"]))
                    size=lambda seg:sum((Path(v["index"])/seg[k]).stat().st_size for k in ["lookup","postings"])
                    row.update(base_bytes=size(m["segments"][0]),middle_bytes=sum(size(s) for s in m["segments"][1:]),segments=len(m["segments"]),indexed_files=sum(d["active"] and d["searchable"] for d in m["documents"]),source_bytes=sum(d["len"] for d in m["documents"] if d["active"] and d["searchable"]))
                rows.append(row);print(tier,repr(query),mode,version,round(row["median_ms"],3),flush=True)
(p/"queries.json").write_text(json.dumps(rows,indent=2)+"\n")
