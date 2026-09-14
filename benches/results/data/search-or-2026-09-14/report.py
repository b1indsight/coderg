import gzip
import hashlib
import json
from pathlib import Path
import shutil
import statistics

root = Path.cwd()
work = root / '.cache/2026-09-10/search-or-profile'
data = root / 'benches/results/data/search-or-2026-09-14'
data.mkdir(parents=True, exist_ok=True)
results = json.loads((work / 'summary.json').read_text())
for name in ('summary.json', 'run.py', 'report.py'):
    shutil.copyfile(work / name, data / name)
for suite in results:
    for name in (suite + '.json', suite + '-expected.json'):
        (data / (name + '.gz')).write_bytes(gzip.compress((work / name).read_bytes(), mtime=0))
sources = ['target/release/coderg', 'target/release/deps/masked_cover_profile-a853751d9aeeb5b2', 'benches/masked_cover_profile.rs', 'benches/support/masked_cover.rs', 'benches/masked_cover_profile.py']
(data / 'sha256.json').write_text(json.dumps({p: hashlib.sha256((root / p).read_bytes()).hexdigest() for p in sources}, indent=2))
lines = ['# OR 正则搜索耗时 — 2026-09-14', '', '```regex', next(iter(results.values()))['pattern'], '```', '', '区分大小写；`decision.action` 中的点保留正则语义。沿用上一轮 vLLM、Chromium 固定语料及索引。以完整逐文件匹配行数核对 rg（索引可搜索文件范围、`--encoding none --text`），三个实验模式与生产 CLI 均通过；实验 Baseline 的候选及 key 访问顺序与生产执行器一致。', '', '预热 2 轮；倒排阶段 9 轮、读取＋逐行正则匹配 5 轮，均取中位数，三种模式轮换顺序。`search -c --no-refresh` CLI 另测 5 轮完整进程时间。没有提前停止或输出条数上限。mask 为预先准备的常驻元数据，不含构建、加载和落盘成本。以下倒排与扫描独立测量，不能将两列之和称为直接测得的 CLI 耗时。', '', '| 语料 / 模式 | posting 次数 | 候选 | 误报 | 倒排 ms | 读取＋匹配 ms | 正则占 worker 工作时间 | 误报占正则工作时间 |', '|---|---:|---:|---:|---:|---:|---:|---:|']
for suite, result in results.items():
    raw = json.loads((work / (suite + '.json')).read_text())
    for mode, row in result['summary'].items():
        scans = [s for s in raw['scan_samples'] if s['mode'] == mode]
        match_pct = statistics.median(100*s['match_worker_ms']/(s['match_worker_ms']+s['read_worker_ms']) for s in scans)
        fp_pct = statistics.median(100*s['false_positive_match_worker_ms']/s['match_worker_ms'] for s in scans)
        lines.append(f"| {suite} / {mode} | {row['posting_reads']} | {row['candidates']:,} | {row['false_positive_files']:,} | {row['query_ms']:.3f} | {row['scan']['scan_wall_ms']:.3f} | {match_pct:.1f}% | {fp_pct:.1f}% |")
lines += ['', 'worker 指标累加各线程耗时；百分比不能解释为并行墙钟时间占比。Baseline 为原 covering；OldCoverMask 保留原 covering 并加 mask；MaskedCover 允许 masked trigram 覆盖四字节并省略部分 posting。', '']
for suite, result in results.items():
    lines.append(f"- {suite}：{result['searchable_files']:,} 个可搜索文件，命中 {result['matched_files']:,} 个文件、{result['matched_lines']:,} 行；原生产 CLI 完整耗时中位数 **{result['cli_median_ms']:.2f} ms**。")
lines += ['', '本查询新 covering 在两套语料均减少 30% 的 posting 查询，倒排和扫描阶段均比 Baseline 快；OldCoverMask 候选最少，但倒排阶段承担更多 mask 过滤工作。新 covering 的候选并非原候选的子集：vLLM 新增 23、排除 178；Chromium 新增 926、排除 2,223。', '', '[原始数据、重复测量和复现脚本](data/search-or-2026-09-14/)']
(root / 'benches/results/search-or-2026-09-14.md').write_text('\n'.join(lines) + '\n')
print('\n'.join(lines))
