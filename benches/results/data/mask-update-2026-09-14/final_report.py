import gzip
import hashlib
import json
import math
from pathlib import Path
import platform
import shutil
import statistics as st
import subprocess
import tarfile

repo = Path.cwd()
work = repo / '.cache/2026-09-10/mask-update-profile'
data = repo / 'benches/results/data/mask-update-2026-09-14'
data.mkdir(parents=True, exist_ok=True)

def summarize(rows):
    result = {'commits':len(rows)}
    for mode in ('baseline','compute','persist'):
        vals = sorted(r[mode]['refresh_ms'] for r in rows)
        result[mode] = {'median_ms':st.median(vals),'p95_ms':vals[math.ceil(.95*len(vals))-1], 'sum_ms':sum(vals)}
    result['median_delta_ms'] = st.median(r['delta_ms'] for r in rows)
    result['median_delta_pct'] = st.median(r['delta_pct'] for r in rows)
    result['aggregate_delta_pct'] = 100*(result['persist']['sum_ms']/result['baseline']['sum_ms']-1)
    return result

summaries = {}
raws = {}
for layout, folder in [('per_document',work), ('batch',work/'batch')]:
    raw = json.loads((folder/'results.json').read_text())
    raws[layout] = raw
    rounds = [r for r in raw['rounds'] if r['round'] >= 0]
    assert len(rounds) == 3 and all(len(r['steps']) == 30 for r in rounds)
    rows = []
    for i in range(30):
        steps = [r['steps'][i] for r in rounds]
        assert len({s['commit'] for s in steps}) == 1
        row = {k: steps[0][k] for k in ('step','commit','subject','changed_files','changed_source_bytes')}
        for mode in ('baseline','compute','persist'):
            row[mode] = {key:st.median(s['measurements'][mode][key] for s in steps) for key in ('load_ms','refresh_ms','total_ms','process_ms')}
            row[mode]['mask'] = {key:st.median(s['measurements'][mode]['mask'][key] for s in steps) for key in ('files','written_bytes','compute_worker_ms','write_worker_ms')}
            row[mode]['compactions'] = [s['measurements'][mode]['compaction'] for s in steps]
        row['delta_ms'] = row['persist']['refresh_ms'] - row['baseline']['refresh_ms']
        row['delta_pct'] = 100*row['delta_ms']/row['baseline']['refresh_ms']
        rows.append(row)
    s = {'overall':summarize(rows),'commits':rows,'by_changed_files':{}}
    for name, pred in [('1 个文件',lambda n:n==1), ('2–5 个文件',lambda n:2<=n<=5), ('6–10 个文件',lambda n:6<=n<=10), ('11 个以上',lambda n:n>=11)]:
        subset = [r for r in rows if pred(r['changed_files'])]
        if subset: s['by_changed_files'][name] = summarize(subset)
    s['builds'] = {mode: {'median_ms':st.median(r['builds'][mode]['total_ms'] for r in rounds), 'initial_bytes':st.median(r['initial_sizes'][mode] for r in rounds), 'final_bytes':st.median(r['final_sizes'][mode] for r in rounds)} for mode in ('baseline','persist')}
    summaries[layout] = s
    for name in ('results.json','commands.json'):
        (data/(layout+'-'+name+'.gz')).write_bytes(gzip.compress((folder/name).read_bytes(),mtime=0))
assert raws['batch']['commits'] == raws['per_document']['commits']
for name in ('run.py','prepare.py','report.py','final_report.py'):
    shutil.copyfile(work/name,data/name)
shutil.copyfile(work/'source-per-document.tar.gz',data/'source-per-document.tar.gz')
with tarfile.open(data/'source.tar.gz','w:gz') as archive:
    for p in sorted((work/'source').rglob('*')):
        if p.is_file(): archive.add(p,arcname=str(p.relative_to(work/'source')))
metadata = {'platform':platform.platform(),'main_commit':subprocess.check_output(['git','rev-parse','main'],text=True).strip(),'variants':{layout:raw['variants'] for layout,raw in raws.items()},'source_sha256':{str(p.relative_to(work/'source')):hashlib.sha256(p.read_bytes()).hexdigest() for p in (work/'source/src').glob('*.rs')}}
for name in ('index.rs','ngram.rs','build.rs','segment.rs','compaction.rs','manifest.rs','git_state.rs'):
    assert subprocess.check_output(['git','show','main:src/'+name]) == (work/'source/src'/name).read_bytes()
(data/'metadata.json').write_text(json.dumps(metadata,indent=2))
(data/'summary.json').write_text(json.dumps(summaries,indent=2))

s = summaries['batch']
old = summaries['per_document']
overall = s['overall']
rows = s['commits']
lines = ['# nextMask：真实 commit 增量更新成本 — 2026-09-14','', f"**批量同步 sidecar 原型：每个 commit 配对增加中位数 {overall['median_delta_ms']:.2f} ms（{overall['median_delta_pct']:.1f}%）；30 步合计增加 {overall['aggregate_delta_pct']:.1f}%。** 这是实测原型成本，尚不是最终 posting 内嵌 mask 格式的成本。", '', '## 测量方法', '', '- 语料为 vLLM 固定历史的最近 30 个连续 first-parent 真实提交，未按测试结果挑选。范围 `'+raws['batch']['commits'][0]+'` → `'+raws['batch']['commits'][-1]+'`。', '- Baseline 的 index/ngram/build/segment/compaction/manifest/git_state 模块逐文件核对本地 main 相同。Compute 在同一文件读取循环中计算全部 trigram 的 nextMask，排序并序列化，不写 mask；Persist 在此基础上写入 sidecar 并同步。生产源码未改动。', '- 两种 Persist 分别做独立配对测试：先回放 3 个 commit 预热，再做 3 次完整回放，每轮新建索引，每个 commit 轮换 Baseline / Compute / Persist 顺序。每个 commit 取三轮中位数，再统计这 30 个 commit 的分布。p95 用 nearest-rank。', '- refresh_ms 直接计时 index::refresh，包含状态检查、修改文件提取、posting 写入、自动合并和 manifest 发布；不含 Git checkout、初次建索引、索引加载、搜索和验证。原始数据另存索引加载、加载＋刷新、完整 probe 进程时间。使用热文件缓存，未强制清缓存。', '', '## 两种持久化原型', '', '- 每条 mask 记录为精确 trigram 3 字节＋8-bit mask，共 4 字节；同一文件版本内 OR 累计，更新后以新版本替换，EOF trigram 的 mask 为 0。源文件只读取一次，沿用原读取块边界的 23 字节重叠。', '- **逐文件同步**：每个修改文件各写一个临时 sidecar、sync_all、rename、目录 sync_all。', '- **批量同步**：一次文件提取的结果收集为一个批次，文件头记录 doc ID 与 payload 长度，每文件多 8 字节；整个批次只做一次文件 sync_all、rename 和目录 sync_all。正常增量更新每 commit 一个批次；若触发全重建，重建提取另产生批次。', '- 两者的搜索接入、垃圾回收、与 manifest 的跨文件崩溃一致性尚未实现。验证读取各文档最新 mask，并依 manifest 排除失效文档；旧版本仍可能占空间。原 postings 的自动合并照常运行，但未测 mask 参与 segment 合并的成本。', '- **不能把本轮结果当作最终格式预测或严格上界**：sidecar 每条 4 字节，而最终对齐 posting 可只保存 mask 字节；最终格式会改变合并和读写成本。批量原型还临时缓存整批 mask，未接入已有内存预算管理。', '', '## 批量同步：单 commit 更新时间', '', '| 模式 | 中位数 ms | p95 ms | 30 个 commit 中位数合计 ms |', '|---|---:|---:|---:|']
for mode,label in [('baseline','主分支'),('compute','计算＋序列化 mask'),('persist','计算＋批量同步 mask')]:
    v=overall[mode]
    lines.append(f"| {label} | {v['median_ms']:.2f} | {v['p95_ms']:.2f} | {v['sum_ms']:.2f} |")
lines += ['', f"配对差值中位数 **+{overall['median_delta_ms']:.2f} ms**，配对增幅中位数 **+{overall['median_delta_pct']:.1f}%**。它们是逐个 commit 计算后取中位数，不能由上表两列总体中位数直接相减或相除得到。", '', '| Git 改动文件数 | commit 数 | 主分支中位数 ms | 批量 mask 中位数 ms | 配对增加中位数 ms | 配对增幅中位数 |', '|---|---:|---:|---:|---:|---:|']
for label,v in s['by_changed_files'].items():
    lines.append(f"| {label} | {v['commits']} | {v['baseline']['median_ms']:.2f} | {v['persist']['median_ms']:.2f} | {v['median_delta_ms']:.2f} | {v['median_delta_pct']:.1f}% |")
lines += ['', f"Git 改动文件数中位数 {st.median(r['changed_files'] for r in rows):g}，范围 {min(r['changed_files'] for r in rows)}–{max(r['changed_files'] for r in rows)}；每 commit 新写 mask payload（含批次中的文件头）中位数 {st.median(r['persist']['mask']['written_bytes'] for r in rows)/1024:.1f} KiB。Git 改动路径可能包含被忽略文件，实际 mask 提取文件数单独保存。", '', '## 为什么不能逐文件同步', '', f"另一次相同 30 个 commit 的配对测试中，逐文件同步的配对增幅中位数为 **{old['overall']['median_delta_pct']:.1f}%**，增加 **{old['overall']['median_delta_ms']:.2f} ms**；30 步合计增加 **{old['overall']['aggregate_delta_pct']:.1f}%**。对应 Baseline / Persist 总体中位数为 {old['overall']['baseline']['median_ms']:.2f} / {old['overall']['persist']['median_ms']:.2f} ms。", '', '这表明存储与同步粒度对更新延迟影响明显，不能把逐文件同步开销解释为 8-bit mask 的必然计算成本。两种持久化分轮运行，均与各自轮次的 Baseline 配对，原始样本全部保留。', '', '## 初建与空间（独立于增量更新）', '', '| 模式 | 初建中位数 s | 初始逻辑文件大小 MiB |', '|---|---:|---:|']
for name,group,mode in [('主分支（批量组）',s,'baseline'),('批量 mask',s,'persist'),('逐文件 mask',old,'persist')]:
    v=group['builds'][mode]
    lines.append(f"| {name} | {v['median_ms']/1000:.3f} | {v['initial_bytes']/1024**2:.2f} |")
lines += ['', '记录的是逻辑文件字节，不是磁盘实际分配量或设备写入量。空间增幅包含重复保存的 trigram key，不能等同于最终 1-byte 对齐格式。', '', '## 每个 commit（批量同步）', '', '| commit | 改动文件 | 修改后文件总 KiB | 主分支 ms | Compute ms | 批量 mask ms | 增幅 |', '|---|---:|---:|---:|---:|---:|---:|']
for r in rows:
    lines.append(f"| {r['commit'][:8]} | {r['changed_files']} | {r['changed_source_bytes']/1024:.1f} | {r['baseline']['refresh_ms']:.2f} | {r['compute']['refresh_ms']:.2f} | {r['persist']['refresh_ms']:.2f} | {r['delta_pct']:+.1f}% |")
lines += ['', '## 验证与复现', '', f"两组共 {sum(r['verified_masks'] for r in raws.values())} 次已更新文件 mask 序列与独立 Python 全文件扫描逐字节一致；{sum(r['verified_searches'] for r in raws.values())} 次原搜索路径输出对照通过（只验证 postings 未受影响，不代表 mask 已接入搜索）。Rust 测试覆盖旧 mask 替换、EOF、跨块合并以及批次序列化。", '', '从 source.tar.gz 构建 update_probe，不启用 feature 为 Baseline；启用 mask-experiment 后，模式 1 为 Compute、3 为逐文件同步、4 为批量同步。run.py 默认回放逐文件模式，设置 MASK_UPDATE_BATCH=1 回放批量模式。脚本中的本地语料路径需按环境调整。逐文件测量时的源码单独保存在 source-per-document.tar.gz。', '', '[汇总与逐项数据](data/mask-update-2026-09-14/summary.json) · [实验源码](data/mask-update-2026-09-14/source.tar.gz) · [回放脚本](data/mask-update-2026-09-14/run.py)', '']
(repo/'benches/results/mask-update-2026-09-14.md').write_text('\n'.join(lines))
print(json.dumps({k:{key:v for key,v in value.items() if key!='commits'} for k,value in summaries.items()},indent=2))
