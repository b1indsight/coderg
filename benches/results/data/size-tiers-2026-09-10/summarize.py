import json,statistics,math,collections,csv,hashlib,subprocess,tarfile,shutil,re
from pathlib import Path
p=Path(__file__).resolve().parent
names={"small":"viberwhisper","medium":"whisper.cpp","large":"vLLM"}
phase=lambda m: "rebuild" if "rebuilt index" in m["diagnostic"] else "base_merge" if "base + middle" in m["diagnostic"] else "middle_merge" if "compacted" in m["diagnostic"] else "incremental" if "incrementally indexed" in m["diagnostic"] else "metadata"
stat=lambda a:dict(count=len(a),median_ms=statistics.median(a),p95_ms=sorted(a)[math.ceil(.95*len(a))-1],min_ms=min(a),max_ms=max(a))
histories={tier:json.loads((p/(tier+".json")).read_text()) for tier in names}
queries=json.loads((p/"queries.json").read_text())
summary={"repositories":{},"verified_history_comparisons":sum(d["verified_comparisons"] for d in histories.values())}
for tier,d in histories.items():
    item=dict(name=names[tier],source=d["source"],base_commit=d["base_commit"],end_commit=d["end_commit"],variants={},maintenance=[],rollbacks=[])
    for v in ["main","current"]:
        measurements=[next(m for m in u["measurements"] if m["version"]==v) for u in d["updates"]]
        groups=collections.defaultdict(list)
        for m in measurements:groups[phase(m)].append(m["wall_ms"])
        first=next(m for m in d["builds"] if m["version"]==v);last=measurements[-1]
        item["variants"][v]=dict(phases={k:stat(a) for k,a in groups.items()},base_initial_bytes=first["base_bytes"],base_final_bytes=last["base_bytes"],middle_final_bytes=last["middle_bytes"],middle_peak_bytes=max(m["middle_bytes"] for m in measurements),peak_segments=max(m["segments"] for m in measurements),final_segments=last["segments"])
    common=[{m["version"]:m for m in u["measurements"]} for u in d["updates"] if all(phase(m)=="incremental" for m in u["measurements"])]
    item["matched_incremental"]={v:stat([m[v]["wall_ms"] for m in common]) for v in ["main","current"]}
    for sequence in ["updates","rollbacks"]:
        for u in d[sequence]:
            m=next(m for m in u["measurements"] if m["version"]=="current")
            if sequence=="updates" and phase(m) not in ["rebuild","base_merge"]:continue
            event=dict(step=u["step"],commit=u["commit"],subject=u["subject"],phase=phase(m),wall_ms=m["wall_ms"],middle_bytes_before=m["middle_bytes_before"],middle_bytes=m["middle_bytes"],base_bytes=m["base_bytes"])
            if phase(m)=="base_merge":
                event["trigger_bytes"]=int(re.search(r"\((\d+) input bytes",m["diagnostic"]).group(1))-m["base_bytes_before"]
            if phase(m)=="rebuild":
                seg=Path(d["workspace"])/"index-current"/"segments"
                delta=max(int(x.stem) for x in seg.glob("*.lookup") if int(x.stem)<m["base_id"])
                event["new_delta_bytes"]=sum((seg/f"{delta:020}.{ext}").stat().st_size for ext in ["lookup","postings"])
                event["trigger_bytes"]=m["middle_bytes_before"]+event["new_delta_bytes"]
            item["maintenance" if sequence=="updates" else "rollbacks"].append(event)
    summary["repositories"][tier]=item
(p/"summary.json").write_text(json.dumps(summary,indent=2)+"\n")
with (p/"per-commit.csv").open("w") as f:
    w=csv.writer(f);w.writerow(["tier","repository","sequence","step","commit","version","phase","wall_ms","base_bytes","middle_bytes","segments","rg_wall_ms"])
    for tier,d in histories.items():
        for sequence in ["updates","rollbacks"]:
            for u in d[sequence]:
                for m in u["measurements"]:w.writerow([tier,names[tier],sequence,u["step"],u["commit"],m["version"],phase(m),m["wall_ms"],m["base_bytes"],m["middle_bytes"],m["segments"],u["rg"]["wall_ms"]])
for tier,item in summary["repositories"].items():
    print(tier)
    for version,data in item["variants"].items():
        print(version,"B",round(data["base_final_bytes"]/2**20,3),"M final/peak",round(data["middle_final_bytes"]/2**20,3),round(data["middle_peak_bytes"]/2**20,3),"max segments",data["peak_segments"])
        print({kind:(a["count"],round(a["median_ms"],3),round(a["p95_ms"],3)) for kind,a in data["phases"].items()})
    print("maintenance",[(m["step"],m["commit"][:8],round(m["wall_ms"],3)) for m in item["maintenance"]])
    print("paired",{v:(a["count"],round(a["median_ms"],3)) for v,a in item["matched_incremental"].items()})
print("verified",summary["verified_history_comparisons"])
