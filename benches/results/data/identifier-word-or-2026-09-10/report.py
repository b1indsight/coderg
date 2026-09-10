import json
from pathlib import Path
import shutil

work = Path(__file__).resolve().parent
repo = work.parents[2]
s = json.loads((work / 'summary.json').read_text())
plan = json.loads((work / 'plan.json').read_text())
data = json.loads((work / 'results.json').read_text())
names = dict(identifier='完整标识符', word='组成单词', or_two='首尾两词 OR', or_all='全部组成词 OR')
versions = ['hash', 'english', 'chromium_all', 'chromium_pairs']
bases = versions[:-1]
labels = dict(hash='原始哈希', english='英文单字母', chromium_all='Chromium 单字母', chromium_pairs='Chromium 实际字母对')
out = ['# vLLM 标识符、单词与 OR 查询权重实验（2026-09-10）', '',
    '比较同一份 vLLM 源码上的四种权重。所有查询区分大小写，完整标识符和单词用 `-F` 子串搜索；OR 用正则，不加词边界。', '',
    '本轮结论：相对原始哈希，四类平均耗时降低 0.52%–1.45%，明显收益集中在 7 个组成词和 2 个两词 OR；12 个完整标识符没有明显收益，且有 1 个明显回退。相对英文表，实际字母对四类平均耗时均略高（0.16%–0.78%），只有 `chunks` 达到明显收益条件，`stage` 和 `stage|metadata` 明显回退。因此，这批扩展查询不支持实际字母对有普遍搜索优势；更明确的优势仍是相对两种单字母表索引更小。', '',
    '## 方法与范围', '',
    f"- vLLM 提交：`{data['metadata']['git_before']['commit']}`，工作树干净。",
    f"- 从 {plan['eligible_identifiers']} 个符合条件的公开 snake_case 函数名中选 12 个：保留前一轮的 `get_tensor_model_parallel_world_size`，其余按固定种子 `20260910` 抽样，来源文件不重复。名称包含 4–8 个纯字母组成词，长度不超过 64。抽样在计时前完成。",
    '- 12 个完整标识符、49 个去重组成词、12 个首尾两词 OR、12 个全部组成词 OR，共 85 项。多个标识符是分别查询；本轮没有测试完整标识符之间的 OR，也没有测试 AND。',
    '- 每项 3 轮预热、31 轮计时，四版本每轮随机交错；计时包括进程启动和搜索，标准输出丢弃。所有查询的完整输出与 `rg` 对齐，匹配行数包含在 CSV 中。',
    '- 复用前一轮四份不可变索引，`--no-refresh`；这轮没有重建、测构建时间或 RSS。源码、二进制和索引的前后哈希一致。',
    '- 字母对表来自 Chromium，而被搜索语料是 vLLM，因此本轮不是“使用目标仓库自身统计”的实验。',
    '- 下表耗时为每项查询 31 次耗时的中位数，再在类别内算术平均；变化为实际字母对相对基线，负数表示更快。',
    '- “明显收益/回退”：中位数变化至少 5%，且 5000 次独立重采样的逐项中位数比值 95% bootstrap 区间不跨 0。未做多重比较校正、独立时段重复；这是本轮筛选信号，不能称为稳定收益。', '',
    '## 分类别耗时', '',
    '| 类型 | 项数 | 原始哈希 ms | 英文 ms | Chromium 单字母 ms | 实际字母对 ms | 对哈希 | 对英文 | 对 Chromium 单字母 |',
    '|---|---:|---:|---:|---:|---:|---:|---:|---:|']
for f, g in s['groups'].items():
    out.append('| ' + ' | '.join([names[f], str(g['n']), *[f"{g['mean_median_ms'][v]:.3f}" for v in versions],
        *[f"{g['comparisons'][b]['change_pct']:+.2f}%" for b in bases]]) + ' |')
out += ['', '## 收益占比与回退', '', '| 类型 | 基线 | 中位数更快 | 明显收益 | 明显回退 |', '|---|---|---:|---:|---:|']
for f, g in s['groups'].items():
    for b in bases:
        c = g['comparisons'][b]
        out.append(f"| {names[f]} | {labels[b]} | {c['faster']}/{g['n']} | {c['clear_gains']}/{g['n']} | {c['clear_regressions']}/{g['n']} |")
out += ['', '## 短词拆分', '',
    '`dp`、`op`、`or`、`to`、`up` 只有两个字符，低于最短 gram 长度 3；对应单词查询及包含这种独立 OR 分支的查询不能使用这些字面量组筛选。完整标识符内部含短词不影响整串的 gram 提取。短词组的计时变化不能直接解释为权重改善了筛选。', '',
    '| 类型 | 词长 | 项数 | 哈希 ms | 实际字母对 ms | 对哈希 | 对英文 | 对 Chromium 单字母 |', '|---|---|---:|---:|---:|---:|---:|---:|']
for key, g in s['splits'].items():
    f, suffix = key.rsplit('_', 1)
    out.append('| ' + ' | '.join([names[f], '含两字符词' if suffix == 'short' else '全部至少三字符', str(g['n']),
        f"{g['mean_median_ms']['hash']:.3f}", f"{g['mean_median_ms']['chromium_pairs']:.3f}",
        *[f"{g['comparisons'][b]['change_pct']:+.2f}%" for b in bases]]) + ' |')
out += ['', '## 完整标识符逐项', '', '| 查询 | 匹配行 | 哈希 ms | 英文 ms | Chromium 单字母 ms | 实际字母对 ms | 对哈希 |', '|---|---:|---:|---:|---:|---:|---:|']
for r in s['queries']:
    if r['family'] == 'identifier':
        out.append('| ' + ' | '.join([f"`{r['pattern']}`", str(r['matches']), *[f"{r['median_ms'][v]:.3f}" for v in versions],
            f"{r['comparisons']['hash']['change_pct']:+.2f}%"]) + ' |')
out += ['', '## 单词与 OR 极值例子', '',
    '每类列出相对原始哈希的两个最大收益和两个最大回退；属于计时后的描述性极值，不代表总体。完整逐项数据另见 CSV。', '',
    '| 类型 | 查询 | 哈希 ms | 实际字母对 ms | 对哈希 | 95% 区间 | 明显变化 |', '|---|---|---:|---:|---:|---|---|']
for family in ['word', 'or_two', 'or_all']:
    group = sorted([r for r in s['queries'] if r['family'] == family], key=lambda r: r['comparisons']['hash']['change_pct'])
    for r in group[:2] + group[-2:]:
        c = r['comparisons']['hash']
        pattern = r['pattern'].replace('|', '\\|')
        out.append(f"| {names[family]} | `{pattern}` | {r['median_ms']['hash']:.3f} | {r['median_ms']['chromium_pairs']:.3f} | {c['change_pct']:+.2f}% | [{c['ci95'][0]:+.2f}%, {c['ci95'][1]:+.2f}%] | {'收益' if c['clear_gain'] else '回退' if c['clear_regression'] else '否'} |")
out += ['', '## 相对单字母表的明显变化', '', '| 查询 | 基线 | 基线 ms | 实际字母对 ms | 变化 | 95% 区间 |', '|---|---|---:|---:|---:|---|']
for b in ['english', 'chromium_all']:
    for r in s['queries']:
        c = r['comparisons'][b]
        if not (c['clear_gain'] or c['clear_regression']):
            continue
        pattern = r['pattern'].replace('|', '\\|')
        out.append(f"| `{pattern}` | {labels[b]} | {r['median_ms'][b]:.3f} | {r['median_ms']['chromium_pairs']:.3f} | {c['change_pct']:+.2f}% | [{c['ci95'][0]:+.2f}%, {c['ci95'][1]:+.2f}%] |")
out += ['', '## 索引体积', '', '| 权重 | 索引 MiB | 相对原始哈希 |', '|---|---:|---:|']
for v in versions:
    out.append(f"| {labels[v]} | {s['index_bytes'][v] / 2**20:.3f} | {100 * (s['index_bytes'][v] / s['index_bytes']['hash'] - 1):+.2f}% |")
out += ['', '## 原始数据与复现', '',
    '数据目录：[identifier-word-or-2026-09-10](data/identifier-word-or-2026-09-10/)。包含 `plan.json`（源码位置和抽样规则）、`results.json`（完整命令、31 次样本、结果摘要、环境及哈希）、`summary.json`、`per-query.csv` 和三个脚本。', '',
    '原始运行位置为 `.cache/2026-09-10/identifier-word-or/`，脚本依赖前一轮缓存中的四份二进制与索引；归档脚本用于审计，不是独立可移植的基准套件。运行命令：', '',
    '```sh', 'python3 .cache/2026-09-10/identifier-word-or/benchmark.py --iterations 31 --warmup 3',
    'python3 .cache/2026-09-10/identifier-word-or/analyze.py', 'python3 .cache/2026-09-10/identifier-word-or/report.py', '```', '',
    '完整输出对齐在计时外进行；为防止覆盖已有数据，benchmark 要求运行目录内不存在 `results.json`。', '']
dest = repo / 'benches/results/data/identifier-word-or-2026-09-10'
dest.mkdir(parents=True, exist_ok=True)
for name in ['plan.json', 'results.json', 'summary.json', 'per-query.csv', 'benchmark.py', 'analyze.py', 'report.py']:
    shutil.copy2(work / name, dest / name)
(repo / 'benches/results/identifier-word-or-2026-09-10.md').write_text('\n'.join(out))
