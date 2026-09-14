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
raw = json.loads((work / 'results.json').read_text())
rounds = [r for r in raw['rounds'] if r['round'] >= 0]
assert len(rounds) == 3 and all(len(r['steps']) == 30 for r in rounds)
rows = []
for i in range(30):
    steps = [r['steps'][i] for r in rounds]
    assert len({s['commit'] for s in steps}) == 1
    row = {k: steps[0][k] for k in ('step','commit','subject','changed_files','changed_source_bytes')}
    for mode in ('baseline', 'compute', 'persist'):
        row[mode] = {key: st.median(s['measurements'][mode][key] for s in steps) for key in ('load_ms','refresh_ms','total_ms','process_ms')}
        row[mode]['mask_written_bytes'] = st.median(s['measurements'][mode]['mask']['written_bytes'] for s in steps)
        row[mode]['mask_compute_worker_ms'] = st.median(s['measurements'][mode]['mask']['compute_worker_ms'] for s in steps)
        row[mode]['mask_write_worker_ms'] = st.median(s['measurements'][mode]['mask']['write_worker_ms'] for s in steps)
        row[mode]['mask_files'] = st.median(s['measurements'][mode]['mask']['files'] for s in steps)
    row['delta_ms'] = row['persist']['refresh_ms'] - row['baseline']['refresh_ms']
    row['delta_pct'] = 100 * row['delta_ms'] / row['baseline']['refresh_ms']
    row['compactions'] = [s['measurements']['baseline']['compaction'] for s in steps]
    rows.append(row)

def summarize(subset):
    result = {'commits':len(subset)}
    for mode in ('baseline','compute','persist'):
        vals = sorted(r[mode]['refresh_ms'] for r in subset)
        result[mode] = {'median_ms':st.median(vals),'p95_ms':vals[math.ceil(.95*len(vals))-1], 'sum_ms':sum(vals)}
    result['median_delta_ms'] = st.median(r['delta_ms'] for r in subset)
    result['median_delta_pct'] = st.median(r['delta_pct'] for r in subset)
    result['aggregate_delta_pct'] = 100*(result['persist']['sum_ms']/result['baseline']['sum_ms']-1)
    return result

summary = {'overall':summarize(rows), 'by_changed_files':{}, 'commits':rows}
for name, pred in [('1 file',lambda n:n==1), ('2–5 files',lambda n:2<=n<=5), ('6–10 files',lambda n:6<=n<=10), ('11+ files',lambda n:n>=11)]:
    subset = [r for r in rows if pred(r['changed_files'])]
    if subset: summary['by_changed_files'][name] = summarize(subset)
for name in ('results.json','commands.json'):
    (data/(name+'.gz')).write_bytes(gzip.compress((work/name).read_bytes(),mtime=0))
for name in ('run.py','prepare.py','report.py'):
    shutil.copyfile(work/name, data/name)
with tarfile.open(data/'source.tar.gz','w:gz') as archive:
    for p in sorted((work/'source').rglob('*')):
        if p.is_file(): archive.add(p,arcname=str(p.relative_to(work/'source')))
metadata = {'platform':platform.platform(), 'main_commit':subprocess.check_output(['git','rev-parse','main'],text=True).strip(), 'binaries':raw['variants'], 'source_sha256':{str(p.relative_to(work/'source')):hashlib.sha256(p.read_bytes()).hexdigest() for p in (work/'source/src').glob('*.rs')}}
for name in ('index.rs','ngram.rs','build.rs','segment.rs','compaction.rs','manifest.rs','git_state.rs'):
    assert subprocess.check_output(['git','show','main:src/'+name]) == (work/'source/src'/name).read_bytes()
(data/'metadata.json').write_text(json.dumps(metadata,indent=2))
(data/'summary.json').write_text(json.dumps(summary,indent=2))

overall = summary['overall']
lines = ['# nextMask：真实 commit 增量更新成本 — 2026-09-14','', '**本轮测的是同步落盘的 per-document sidecar 原型，不能当作最终 posting 内嵌 1-byte mask 格式的成本。** 主分支生产源码未修改。', '', '## 实现与口径', '', '- Baseline 使用与本地 main 完全一致的索引模块；Compute 在原文件读取循环中增加全 trigram mask 累计、排序和序列化，不写 mask；Persist 再把结果写入每文件 sidecar，临时文件写入后 sync_all、原子 rename、目录 sync_all。', '- 每条 sidecar 记录为精确 trigram 3 字节＋mask 1 字节，按 trigram 排序。单个文件版本内 OR 累计，修改后整体替换；EOF trigram 的 mask 为 0。文件内容只读取一次，跨 32 KiB 块沿用现有 23 字节重叠。', '- sidecar 与生产 postings 分开保存：现有 segment 合并照常进行，但不重写 sidecar。失效/删除文件的 sidecar 不用于验证，旧文件可能保留；没有实现垃圾回收、与 manifest 的跨文件崩溃一致性，也没有接入搜索读取。实际更新过的全部可搜索文件均用独立 Python 全文件扫描核对 mask。', '- 这不是最终存储格式：每条记录 4 字节且每文件单独同步，会增加文件系统开销；最终与 posting 共存可以减少这些开销，但也会增加 segment 合并流量。不能把这里的百分比视为最终方案预测或严格上界。', '- vLLM 最近 30 个连续 first-parent 真实提交，未按性能结果挑选。基线 '+raw['commits'][0]+' → '+raw['commits'][-1]+'。先回放 3 个提交预热，再做 3 次完整独立回放，每轮新建索引，逐步轮换三个版本的执行顺序。', '- refresh_ms 直接计时 index::refresh，包含文件状态检查、修改文件提取、posting 写入、自动合并和 manifest 发布；不含 Git checkout、初始建索引、索引加载、搜索或验证。JSON 另存 load_ms、load＋refresh 的 total_ms 及完整 probe 进程时间。热文件缓存，未强制清缓存。', '- 每个 commit 先取 3 轮中位数；汇总中的 median/p95 是这 30 个 commit 的分布。差值与百分比按同一个 commit 配对计算。mask worker 时间为各线程累计，不能当作并行墙钟时间。写入字节是 sidecar payload，不是设备物理写入量。', '', '## 更新结果', '', '| 模式 | 单 commit 中位数 ms | p95 ms | 30 个 commit 中位数合计 ms |', '|---|---:|---:|---:|']
for mode,label in [('baseline','主分支'),('compute','计算＋序列化 mask'),('persist','再同步落盘 mask')]:
    s=overall[mode]
    lines.append(f"| {label} | {s['median_ms']:.2f} | {s['p95_ms']:.2f} | {s['sum_ms']:.2f} |")
lines += ['', f"Persist 相对 Baseline，配对增加中位数 **{overall['median_delta_ms']:.2f} ms**，配对增幅中位数 **{overall['median_delta_pct']:.1f}%**；全 30 步合计增加 **{overall['aggregate_delta_pct']:.1f}%**。", '', '| Git 改动文件数 | commit 数 | 主分支中位数 ms | Persist 中位数 ms | 配对增加中位数 ms | 配对增幅中位数 |', '|---|---:|---:|---:|---:|---:|']
for label,s in summary['by_changed_files'].items():
    lines.append(f"| {label} | {s['commits']} | {s['baseline']['median_ms']:.2f} | {s['persist']['median_ms']:.2f} | {s['median_delta_ms']:.2f} | {s['median_delta_pct']:.1f}% |")
lines += ['', f"Git 改动文件数中位数 {st.median(r['changed_files'] for r in rows):g}，范围 {min(r['changed_files'] for r in rows)}–{max(r['changed_files'] for r in rows)}；Git 改动路径包含忽略文件，因此另记录实际 mask 更新文件数。每 commit 新写 mask payload 中位数 {st.median(r['persist']['mask_written_bytes'] for r in rows)/1024:.1f} KiB。", '', '## 初建与空间（独立于增量更新时间）', '']
for mode in ('baseline','persist'):
    ms=st.median(r['builds'][mode]['total_ms'] for r in rounds)
    size=st.median(r['initial_sizes'][mode] for r in rounds)/1024**2
    lines.append(f'- {mode}：初建中位数 {ms/1000:.3f} s，初始索引逻辑文件大小 {size:.2f} MiB。')
lines += ['', '## 每个 commit', '', '| commit | 改动文件 | 修改后文件总 KiB | Baseline ms | Compute ms | Persist ms | 增幅 |', '|---|---:|---:|---:|---:|---:|---:|']
for r in rows:
    lines.append(f"| {r['commit'][:8]} | {r['changed_files']} | {r['changed_source_bytes']/1024:.1f} | {r['baseline']['refresh_ms']:.2f} | {r['compute']['refresh_ms']:.2f} | {r['persist']['refresh_ms']:.2f} | {r['delta_pct']:+.1f}% |")
lines += ['', '## 验证与复现', '', f"- 独立全文件扫描核对 {raw['verified_masks']} 次已更新文件的完整 mask 序列，全部通过；{raw['verified_searches']} 次原搜索路径输出对照通过。后者只验证 postings 未受影响，不代表 sidecar 已接入查询。", '- Rust 的 mask 替换、EOF 和跨块覆盖测试通过。Baseline 索引模块逐文件核对本地 main 相同。', '', '[原始计时与汇总](data/mask-update-2026-09-14/summary.json) · [完整实验源码](data/mask-update-2026-09-14/source.tar.gz) · [回放脚本](data/mask-update-2026-09-14/run.py)', '']
(repo/'benches/results/mask-update-2026-09-14.md').write_text('\n'.join(lines))
print(json.dumps({k:v for k,v in summary.items() if k!='commits'},indent=2))
