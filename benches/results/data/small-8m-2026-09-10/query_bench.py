import json, subprocess, time, statistics
from pathlib import Path
p=Path(__file__).resolve().parent
out=[]
for name, queries in [("viberwhisper", ["whisper", "viberwhisper"]), ("agentflow", ["def "]), ("vllm", ["SamplingParams"])]:
    history=json.loads((p/(name+".json")).read_text())
    root=Path(history["workspace"])/"source"
    for query in queries:
        commands={v["name"]:[v["binary"],"search","-F","-l",query,str(root),"--index-dir",v["index"]] for v in history["variants"]}
        commands["rg"]=["rg","--no-config","--hidden","-g","!.git","-F","-l",query,"."]
        reference=None
        for version,cmd in commands.items():
            samples=[]
            for i in range(11):
                started=time.perf_counter_ns()
                r=subprocess.run(cmd,cwd=root,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
                elapsed=(time.perf_counter_ns()-started)/1e6
                assert r.returncode in (0,1),r.stderr
                files=sorted(line.removeprefix("./") for line in r.stdout.decode().splitlines())
                if reference is None: reference=files
                assert files==reference,(name,query,version)
                if i: samples.append(elapsed)
            out.append(dict(repository=name,query=query,version=version,files=len(files),samples_ms=samples,median_ms=statistics.median(samples),command=cmd,commit=history["end_commit"],state="after four rollback/replay checks"))
            print(name,repr(query),version,round(statistics.median(samples),3),flush=True)
(p/"queries.json").write_text(json.dumps(out,indent=2)+"\n")
