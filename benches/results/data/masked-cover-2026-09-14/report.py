from pathlib import Path
import gzip
import hashlib
import io
import json
import shutil
import sys
import tarfile

sys.dont_write_bytecode = True
repo = Path.cwd()
work = repo / '.cache/2026-09-10/masked-cover-profile'
data = repo / 'benches/results/data/masked-cover-2026-09-14'
data.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(repo / 'benches'))
from masked_cover_profile import summarize, MODES

rows = {}
for corpus, count in [('vllm',34), ('chromium',14)]:
    folder = data / corpus
    folder.mkdir(exist_ok=True)
    rows[corpus] = json.loads((work / corpus / 'summary.json').read_text())
    assert len(rows[corpus]) == count
    meta = json.loads((work / corpus / 'metadata.json').read_text())
    assert 'finished_utc' in meta
    for path, digest in meta['source_sha256'].items():
        assert hashlib.sha256((repo / path).read_bytes()).hexdigest() == digest, path
    for p in (work / corpus).glob('*.json'):
        if p.name in ['summary.json','metadata.json','commands.json']:
            shutil.copy2(p,folder / p.name)
        else:
            (folder / (p.name+'.gz')).write_bytes(gzip.compress(p.read_bytes(),mtime=0))
    for row in rows[corpus]:
        raw = json.loads((work / corpus / (row['id']+'.json')).read_text())
        assert raw['production_baseline_equivalent'] and raw['expected_counts_verified']
        for s in raw['query_samples']:
            assert s['stats']['posting_reads'] == len(set(s['stats']['loaded_keys'])) <= 128
        for s in raw['summaries']:
            assert s['candidates'] == raw['matched_files'] + s['false_positive_files']
            b = raw['summaries'][0]['candidates']
            assert s['candidates'] == b+s['added_vs_baseline']-s['removed_vs_baseline']

followups = {}
(data / 'followup').mkdir(exist_ok=True)
for p in (work / 'followup').glob('*.json'):
    shutil.copy2(p,data / 'followup' / p.name)
    if p.name != 'metadata.json':
        raw=json.loads(p.read_text())
        assert raw['production_baseline_equivalent'] and raw['expected_counts_verified']
        followups[p.stem]=summarize(raw)

aggregate = {}
for corpus, queries in rows.items():
    modes = {m:{k:sum(q['summary'][m][k] for q in queries) for k in
        ['posting_reads','decoded_ids','candidates','false_positive_files','query_ms']} for m in MODES}
    b,n=modes['Baseline'],modes['MaskedCover']
    aggregate[corpus]=dict(modes=modes,queries=len(queries),
        fewer_posting_queries=sum(q['summary']['MaskedCover']['posting_reads']<q['summary']['Baseline']['posting_reads'] for q in queries),
        more_posting_queries=sum(q['summary']['MaskedCover']['posting_reads']>q['summary']['Baseline']['posting_reads'] for q in queries),
        more_candidate_queries=sum(q['summary']['MaskedCover']['candidates']>q['summary']['Baseline']['candidates'] for q in queries),
        change_pct={k:100*(n[k]/b[k]-1) for k in b})
(data / 'aggregate.json').write_text(json.dumps(aggregate,indent=2)+'\n')

lines=['# Masked 4-gram covering 实现与对照 — 2026-09-14','',
    '**将 masked trigram 当作四字节覆盖单元，确实减少了 posting 查询；仅按覆盖长度贪心选择，会在部分查询中显著增加误报，不能直接替换默认 covering。**', '',
    '本轮在 `experiment/nextmask-false-positive-profile` 上新增独立实验实现，复用生产 gram/literal 提取器；产品 `src/` 未修改。', '',
    '## 实现', '',
    '- `Span` 分开保存 `key_len` 和 `cover_len`。`trigram + nextMask` 的 key 长度为 3、覆盖长度为 4；长度至少为 4 的真实 sparse gram 保持原长度。',
    '- 枚举当前可索引的 sparse grams；每个起点选择覆盖最远的项，同长优先真实长 gram。沿用覆盖边界重叠两个字节、保留最长 anchor 的贪心框架。没有预先读取额外 postings 来估算选择性。',
    '- 每个查询条件保留 key 与可选的四字节内容。正则必要组取 AND、literal 分支取 OR、分支内条件取 AND；mask 在该条件的 posting 上执行，而非查询结束后的独立过滤。',
    '- 原始 posting 缓存按物理 u32 hash 保存；mask 后的列表按完整条件保存。不同第四字节或碰撞到同一 hash 的不同 trigram 不会混用过滤结果；原始 posting 可复用。',
    '- 全局预算仍为 128 次不同物理 key 查询。预算不足时跳过整个尚未完整读取的 OR anchor 组，或放弃后续 AND 精化条件，以保留合法匹配。', '',
    '例如 `test` 原来查询 `tes ∩ est`，新 covering 可以只查询 `tes + nextMask(t)`。`rocm` 同样可由两个 posting 变为 `roc + nextMask(m)`，但后者的选择性在本轮很差。', '',
    '## 三组对照和测量口径', '',
    '| 模式 | covering | mask 应用位置 |', '|---|---|---|',
    '| Baseline | 原 covering | 无 mask |',
    '| OldCoverMask | 原 covering | 在各自分支的 posting 上过滤 |',
    '| MaskedCover | 四字节有效覆盖 + 长 sparse gram | 在各自分支的 posting 上过滤 |', '',
    '`OldCoverMask` 是本轮的集成对照，与上一轮的候选集后置 mask 不同：它遵守实际 posting 访问和全局预算，仅在条件被访问时使用其 mask。三组共用本轮通用执行器；每个查询额外检查 Baseline 的候选 ID 和实际 key 访问顺序都与生产执行器完全一致。', '',
    '沿用上一轮固定提交及新建 v5 索引：vLLM 6,835 个文件；Chromium 461,549 个文件。平台 Apple M5 / 10 逻辑 CPU / 24 GiB RAM。索引 manifest 哈希与上一轮一致。', '',
    '- 48 项查询均测倒排阶段：预热 2 轮、测量 9 轮，三种模式轮换先后顺序。每轮重新调用索引接口并解码 postings；磁盘索引对象常驻，计时不含索引加载、正则编译、计划生成和源码扫描。',
    '- `posting_reads` 是不同 key 的实际索引查询次数，包括未命中的 key；它不是物理磁盘 I/O 次数。`decoded_ids` 统计索引接口返回的文件 ID 数；本轮使用无旧版本叠加的新建索引。',
    '- 每项查询均扫描三组候选的并集一次，核对完整的逐文件匹配行数与上一轮经过 rg 验证的结果映射相等，并检查每种候选集包含全部真实命中。新增候选也实际扫描，未把验证范围限制为旧候选集。',
    '- 预先选定 11 项查询额外重复读取＋匹配计时：预热 2 轮、测量 3 轮；发现候选退化后，对 vLLM 的 4 项查询补测 5 轮。补测不计入 48 项查询的总体汇总。',
    '- 模式内匹配使用生产 `search -c --no-refresh` 的逐行计数逻辑；扫描阶段耗时为并行墙钟时间。匹配/读取 worker 指标为线程累计墙钟时间，不能与并行墙钟混用。',
    '- 表中每项耗时分别取中位数，总体合计为各查询中位数之和。查询阶段与扫描阶段分开测量，不能把合计当成直接测得的完整 CLI 延迟。少量百分比差异可能来自噪声。', '',
    '**mask 仍为预先准备的常驻元数据，没有实现新的落盘格式。** 与上一轮不同，本轮为相关 trigram 的完整 postings 准备掩码，包含旧候选集之外的文件。准备成本、覆盖文件数及字节数均记录，但不计入查询。没有测全量 mask 索引的构建、刷新、体积和加载成本。', '',
    '## 全集结果', '',
    '| 语料 | posting 查询 Baseline → New | 返回 ID 数 Baseline → New | 候选 Baseline → New | 查询阶段合计 ms Baseline → New |',
    '|---|---:|---:|---:|---:|']
for corpus,a in aggregate.items():
    b,n=a['modes']['Baseline'],a['modes']['MaskedCover']; c=a['change_pct']
    lines.append(f"| {corpus} | {b['posting_reads']:,} → {n['posting_reads']:,} ({c['posting_reads']:+.1f}%) | {b['decoded_ids']:,} → {n['decoded_ids']:,} ({c['decoded_ids']:+.1f}%) | {b['candidates']:,} → {n['candidates']:,} ({c['candidates']:+.1f}%) | {b['query_ms']:.3f} → {n['query_ms']:.3f} ({c['query_ms']:+.1f}%) |")
lines += ['', '48 项中有 27 项减少了实际 posting 查询。vLLM 有 1 项反而增加（`icase_env`：31 → 32）；全局预算、分支提前结束和 anchor 的变化会影响实际访问，不能只统计计划中的 gram 数。', '',
    'vLLM 的倒排阶段总体基本持平，候选反而增加；Chromium 的倒排阶段有所改善。不能据此得出整个搜索统一加速的结论。', '',
    '## 关键例子', '',
    '- vLLM `test`：posting 2 → 1，倒排阶段约 0.474 → 0.194 ms；新候选 3,341 个，仍多于保留两个 posting 加 mask 的 3,006 个。扫描阶段分别约 29.57 / 25.85 / 28.53 ms（Baseline / OldCoverMask / New）。少读一次 posting 节省的时间，未超过相对 OldCoverMask 多扫文件的成本。',
    '- vLLM `rocm`：posting 2 → 1，候选 561 → 2,360，误报 1 → 1,800；补测扫描阶段约 4.74 → 19.99 ms。查询阶段也没有改善。',
    '- 在本轮 next-byte 哈希中，`m` 与 `e` 都映射到 bit 0。因此仅含 `process` 的文件也可以通过 `roc + nextMask(m)`：它实际提供的是 `roc` 后接 `e`。原来的 `ocm` posting 能排除这一类文件。这个例子解释了为何省掉精化 posting 会退化，不能将所有 1,800 个误报都归为这一个字符串。',
    '- Chromium 类声明：posting 4 → 3，但返回 ID 数 446,912 → 535,317；倒排阶段约 34.77 → 54.77 ms。列表数量减少，列表长度反而增大。',
    '- Chromium `test`：posting 2 → 1，返回 ID 数 504,010 → 231,255；倒排阶段约 39.78 → 14.63 ms。这是覆盖替换有效的例子，但新误报 8,277 个仍多于 OldCoverMask 的 4,989 个。', '',
    '## 每项倒排查询对照', '',
    '每格按 Baseline / OldCoverMask / MaskedCover 排列。详细加载 key 列表、mask 检查次数、候选增减方向和原始重复计时均在 JSON 中。']
for corpus,queries in rows.items():
    lines += ['',f'### {corpus}','', '| 查询 | 实际 posting 次数 | 返回 ID 数 | 候选文件 | 倒排阶段 ms |', '|---|---:|---:|---:|---:|']
    for q in sorted(queries,key=lambda r:r['id']):
        s=q['summary']; cols=[' / '.join(f"{s[m][k]:,}" for m in MODES) for k in ['posting_reads','decoded_ids','candidates']]
        times=' / '.join(f"{s[m]['query_ms']:.3f}" for m in MODES)
        lines.append(f"| `{q['id']}` | {' | '.join(cols)} | {times} |")
lines += ['', '## 重复扫描计时', '', '单位 ms，仅含读取文件和逐行精确匹配；补测项用 * 标记。', '',
    '| 语料 / 查询 | Baseline | OldCoverMask | MaskedCover |', '|---|---:|---:|---:|']
for corpus,queries in rows.items():
    for q in sorted(queries,key=lambda r:r['id']):
        s=q['summary']; star=''
        if corpus=='vllm' and q['id'] in followups: s=followups[q['id']];star=' *'
        if s['Baseline']['scan'] is None: continue
        times=' | '.join(f"{s[m]['scan']['scan_wall_ms']:.3f}" for m in MODES)
        lines.append(f"| {corpus} / `{q['id']}`{star} | {times} |")
lines += ['', '## 判断与后续约束', '',
    '可以把四字节 mask 单元纳入覆盖候选，但不宜只根据覆盖长度决定替换。下一步至少需要便宜的 posting 长度/选择性估计：弱 masked gram 应保留第二个精化 gram，或在候选过多时补读它。这样可能放弃部分 posting 节省，换取稳定的文件扫描成本。本轮保留原始贪心结果，没有针对 `rocm` 等查询加特判。', '',
    '当前执行器为一个条件过滤整条 posting 后缓存结果。后续还可以在已有小候选集上按需检查 mask，降低 mask 检查与中间分配的开销。完整落盘方案仍需单独测量。', '',
    '## 验证与复现', '',
    '- 新增 7 项测试通过，覆盖四字节替换、2,000 个随机 literal 的覆盖完整性、同长度优先真实 sparse gram、OR 预算、共享 key 的不同 mask、u32 key 碰撞、EOF/重叠/位碰撞，以及大小写/Unicode/可选分支的匹配保留。',
    '- 48 项查询的生产 Baseline key 访问顺序及候选等价检查、三模式命中保留、候选并集逐文件结果检查均通过；4 项补测同样通过。',
    '- [覆盖与执行器](../support/masked_cover.rs)、[Rust 测量程序](../masked_cover_profile.rs)、[Python 驱动](../masked_cover_profile.py)、[测试](../../tests/masked_cover.rs)。',
    '- 构建：`cargo build --release --locked --bench masked_cover_profile`。运行驱动的 `--help` 可查看语料与结果路径参数；完整实测命令保存在各数据目录的 commands.json，补测命令在 followup/metadata.json。',
    '- [汇总数据](data/masked-cover-2026-09-14/aggregate.json)、[vLLM](data/masked-cover-2026-09-14/vllm/summary.json)、[Chromium](data/masked-cover-2026-09-14/chromium/summary.json)、[补测信息](data/masked-cover-2026-09-14/followup/metadata.json)。逐查询 JSON 以 gzip 归档。',
    '- 参考匹配映射来自[上一轮证据](nextmask-false-positive-2026-09-14.md)，通过逐查询哈希关联。当前源代码快照、归档验证结果及生成报告脚本也保存在本轮数据目录；大语料和索引留在 `.cache`。','']
report=repo / 'benches/results/masked-cover-2026-09-14.md'
report.write_text('\n'.join(lines))

meta=json.loads((work / 'vllm/metadata.json').read_text())
buffer=io.BytesIO()
with tarfile.open(fileobj=buffer,mode='w') as tar:
    for path in sorted(meta['source_sha256']):
        payload=(repo/path).read_bytes(); info=tarfile.TarInfo(path);info.size=len(payload);info.mode=0o644
        tar.addfile(info,io.BytesIO(payload))
snapshot=gzip.compress(buffer.getvalue(),mtime=0)
(data/'source.tar.gz').write_bytes(snapshot)
validation=dict(queries=48,adaptive_queries=4,new_tests_passed=7,source_hashes='passed',
    baseline_equivalence='passed',match_preservation='passed',query_counters='passed',
    source_archive_sha256=hashlib.sha256(snapshot).hexdigest())
(data/'validation.json').write_text(json.dumps(validation,indent=2)+'\n')
for name in ['report.py','followup.py']:
    if work/name != data/name: shutil.copy2(work/name,data/name)
print(json.dumps(aggregate,indent=2))
