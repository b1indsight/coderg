from pathlib import Path
import argparse
import gzip
import json
import shutil
import statistics

parser = argparse.ArgumentParser()
parser.add_argument('--repo', type=Path, default=Path.cwd())
parser.add_argument('--work', type=Path, default=Path('.cache/2026-09-10/nextmask-profile'))
args = parser.parse_args()
repo = args.repo.resolve()
work = args.work.resolve()
data = repo / 'benches/results/data/nextmask-false-positive-2026-09-14'
data.mkdir(parents=True, exist_ok=True)
shutil.copy2(work / 'provenance.json', data / 'provenance.json')
datasets = {}
for name in ['vllm', 'chromium']:
    source = work / (name + '-results')
    target = data / name
    target.mkdir(exist_ok=True)
    datasets[name] = json.loads((source / 'summary.json').read_text())
    assert len(datasets[name]) == {'vllm':34, 'chromium':14}[name]
    assert 'finished_utc' in json.loads((source / 'metadata.json').read_text())
    for p in source.glob('*.json'):
        if p.name in ['summary.json', 'metadata.json']:
            shutil.copy2(p, target / p.name)
        else:
            (target / (p.name + '.gz')).write_bytes(gzip.compress(p.read_bytes(), mtime=0))

def aggregate(rows):
    def sums(mode, key):
        return sum(r['summary'][mode][key] for r in rows)
    b = lambda k: sums('baseline', k)
    n = lambda k: sums('nextmask', k)
    return dict(queries=len(rows), candidates=b('candidates'), false_positives=b('false_positive_files'),
                false_positive_pct=100*b('false_positive_files')/b('candidates'),
                false_positive_match_worker_ms=b('false_positive_match_worker_ms'),
                false_positive_match_work_pct=100*b('false_positive_match_worker_ms')/b('match_worker_ms'),
                match_work_pct=100*b('match_worker_ms')/(b('read_worker_ms')+b('match_worker_ms')),
                scan_wall_ms=b('scan_wall_ms'), scan_phase_pct=100*b('scan_wall_ms')/b('total_ms'),
                removed_candidates=b('candidates')-n('candidates'),
                removed_candidates_pct=100*(b('candidates')-n('candidates'))/b('candidates'),
                removed_false_positives_pct=100*(b('candidates')-n('candidates'))/b('false_positive_files'),
                improved_queries=sum(r['summary']['nextmask']['candidates']<r['summary']['baseline']['candidates'] for r in rows),
                nextmask_scan_wall_ms=n('scan_wall_ms'),
                nextmask_scan_change_pct=100*(n('scan_wall_ms')/b('scan_wall_ms')-1))

aggregates = {name: {'all':aggregate(rows), 'indexed_only':aggregate([r for r in rows if not r['fallback']])}
              for name,rows in datasets.items()}
(data / 'aggregate.json').write_text(json.dumps(aggregates, indent=2)+'\n')

lines = ['# 候选文件误报与 nextMask 过滤实验 — 2026-09-14', '',
    '**当前索引的误报成本因查询而异；nextMask 对部分短片段有效，但无法消除大量由词边界、锚点和正则结构产生的误报。** 本实验测量现有搜索基线，并测试仅增强选中 trigram 的保守 nextMask 原型。它不是完整落盘索引的性能测试。', '',
    '分支：`experiment/nextmask-false-positive-profile`；基线 commit：`a517489ba3c14d58154944b46a8fceea1d444c64`。产品 `src/` 未修改，新增独立 Rust benchmark、Python 驱动和本报告。', '',
    '## 测量口径', '',
    '- **候选误报比例** = 最终没有匹配行的候选文件数 / 候选文件数。它不是统计学中以全部负例文件为分母的 FPR。零候选时比例记为 —。',
    '- **误报匹配耗时** = 无匹配候选文件上逐行正则扫描的工作线程累计墙钟时间；包括行切分，不包括读取文件、索引查询或结果文本构造。',
    '- **误报匹配占比** = 误报匹配耗时 / 全部候选文件的匹配耗时，分子分母采用相同的工作线程累计口径。',
    '- **候选扫描墙钟时间** = 并行读取全部候选文件并执行匹配的经过时间，包含 Rayon 调度。其占比的分母是编译正则、加载索引、生成查询、倒排筛选和扫描完成这几个阶段的经过时间。',
    '- 工作线程累计时间包含被调度出去的等待，**不是 CPU 时间，不能直接除以多线程阶段的墙钟时间，也不能直接理解为可节省的用户延迟**。',
    '- 使用 `search -c --no-refresh` 的逐行计数语义：所有行都扫描，不因首次命中提前停止。诊断阶段计时不含 CLI 启动、stdout 输出、释放索引和 refresh；另测未插桩 CLI 的完整进程耗时作为参照。',
    '- 每项预热 2 轮；vLLM 测量 9 轮，Chromium 5 轮。两种模式交替先后执行；各轮重新加载索引，复用进程内 Rayon 线程池，文件系统缓存为热缓存。正式计时没有并发构建或其他 benchmark。逐文件计时本身有少量扰动。',
    '- 表内各耗时分别取中位数；单项比例先逐轮计算再取中位数。汇总耗时按各查询中位数相加，汇总比例用这些合计值计算，因此不要求表内独立中位数精确相除。',
    '- 查询集合是人为选择的功能覆盖集合；汇总候选数统计的是“查询、候选文件”对，同一文件可以在不同查询中重复出现。它不代表真实用户查询分布。', '',
    '## 语料与验证', '',
    'Apple M5，10 个逻辑 CPU，24 GiB RAM。两份旧 `/private/tmp` 源码被清理后，本轮均在项目 `.cache/2026-09-10/nextmask-profile/` 下恢复并重建 v5 索引，结果按本轮实际搜索范围统计，不混用历史索引的文件数。会话始于 9 月 10 日，实际测量在 9 月 14 日。', '',
    '- vLLM：`569adb5a9780f9c02d22a6b29826acf711512356`，从本地保留的 Git 对象导出；6,835 个可搜索文件、86,384,180 字节。',
    '- Chromium：`398630472335c10b9ca610a4d1b7888a040f702a`，下载官方固定提交 archive；461,549 个可搜索文件、3,039,547,005 字节。',
    '- 全部 48 项查询：baseline / nextMask 的逐文件匹配行数在每个重复中一致，且与未插桩 coderg、同范围的 rg 参考结果逐文件一致。vLLM 使用 `rg --encoding none` 遍历；Chromium 使用 `rg --encoding none --text --no-ignore`，显式传入索引内 active + searchable 文件，每批 1,000 个路径，避免编码转码和二进制判定差异。完整文件清单及哈希随数据保存。验证比较完整映射，不仅比较总数。',
    '- 初次 Chromium `test` 校验发现默认 rg 比 coderg 多命中 32 个 UTF-16 文件；rg 自动按 BOM 转码，而 coderg 把含 NUL 的这些文件视为二进制。随后统一使用 `rg --encoding none` 按原始字节校验，并重验此前查询；未重跑已经有效的计时。初次差异、完整命令以及驱动更新前后的 metadata 均保留。',
    '- `if` 参考校验又发现 `chromium.ai`、`product_logo.ai` 两个文件的前 8 KiB 没有 NUL、后部存在 NUL：coderg 将其纳入索引，默认 rg 二进制检查会停止搜索。因此 Chromium 最终按上述文件清单 + `--text` 验证。这是测量输入范围的对齐，本轮没有更改产品的编码或二进制策略。',
    '- 现有 `cargo test --locked` 的 77 项测试通过；benchmark 自检覆盖重叠 trigram、文件结尾、掩码 OR 合并、大小写、Unicode 和带可选/不同长度分支的正则。`cargo fmt --all -- --check`、`git diff --check` 通过。', '',
    '## 全集汇总', '',
    '| 语料 | 查询数 | 候选对 | 误报候选对 | 候选误报比例 | 误报匹配累计 ms | 误报占全部匹配工作 | 匹配占读取+匹配工作 |',
    '|---|---:|---:|---:|---:|---:|---:|---:|']
for name in datasets:
    a = aggregates[name]['all']
    lines.append(f"| {name} | {a['queries']} | {a['candidates']:,} | {a['false_positives']:,} | {a['false_positive_pct']:.2f}% | {a['false_positive_match_worker_ms']:.2f} | {a['false_positive_match_work_pct']:.2f}% | {a['match_work_pct']:.2f}% |")
lines += ['', '读取文件的线程累计成本也很重要；nextMask 排除文件时可同时避免读取和匹配。下面单独列出候选扫描的墙钟时间，不能与上表的累计时间相加。', '',
    '| 语料 | 基线扫描合计 ms | 扫描占搜索阶段 | nextMask 扫描合计 ms | 扫描耗时变化 | 减少候选对 | 占原候选 | 消除原误报 | 候选减少的查询数 |',
    '|---|---:|---:|---:|---:|---:|---:|---:|---:|']
for name in datasets:
    a=aggregates[name]['all']
    lines.append(f"| {name} | {a['scan_wall_ms']:.2f} | {a['scan_phase_pct']:.2f}% | {a['nextmask_scan_wall_ms']:.2f} | {a['nextmask_scan_change_pct']:+.2f}% | {a['removed_candidates']:,} | {a['removed_candidates_pct']:.2f}% | {a['removed_false_positives_pct']:.2f}% | {a['improved_queries']}/{a['queries']} |")
lines += ['', '这些是 resident 元数据原型的扫描阶段对照。候选未变化的查询也会出现计时波动，不能据此声称算法加速或减速；没有把此表包装成完整 CLI 的 nextMask 加速比。排除全扫描回退查询后的汇总另见 `aggregate.json` 的 `indexed_only`。', '',
    '## 对优化方向的判断', '',
    '- Chromium 固定字符串 `test` 的误报从 24,425 个减少到 4,989 个，说明 nextMask 能有效排除部分 trigram 组合误报。但加上词边界成为 `\\btest\\b` 后，相同候选集中的误报为 92,464 个，过滤后仍有 73,028 个；这个 mask 条件不会检查完整单词边界。',
    '- vLLM 的整行 `return None` 正则候选误报比例约 79%，nextMask 只排除 29 个文件；`def\\s+forward\\b` 的误报候选有 665 个，当前原型一个也未排除。高候选误报比例并不自动意味着 nextMask 收益高。',
    '- Chromium 固定字符串 `test` 的正则匹配仅占“读取 + 匹配”线程累计时间约 2.6%。减少候选也会避免文件打开、读取和关闭；应同时考察这些成本，而不只关注正则引擎。',
    '- 建议优先把它当成选中 trigram 上的可选增强继续验证。完整实现是否值得默认启用，还需要索引体积、构建/刷新和元数据读取成本的数据。', '',
    '## nextMask 原型的边界', '',
    '每个 trigram 使用一个 u8，bit 位置为 `ngram::hash(&[next_byte]) & 7`；对文档内全部出现位置的后续字节按位 OR。原型采用独立的精确 24-bit trigram 命名空间，避免与现有 u32 sparse gram hash 混用。处理完整文件，保留重叠出现；文件结尾没有后续字节的出现不贡献位。', '',
    '查询约束来自现有 literal 提取器和 covering gram 选择：只有选中 gram 长度为 3、且同一 literal 中存在确定的第四个字节时才添加 mask 条件。不同必要组取 AND，literal 分支取 OR，分支内条件取 AND；如果任一 OR 分支没有可用 mask，整组不使用 mask 过滤。不会把 `abc.*d` 当成 `abcd`。', '',
    '这是在现有候选集之后附加的保守过滤。它没有把各 literal 分支的长 gram postings 与 mask 条件共同规划，且不模拟全局 lookup budget 下的磁盘调度；不能视为已经实现了完整的 nextMask 查询规划。', '',
    '**元数据预计算完全位于搜索计时之外，且只为本次查询涉及的 trigram、候选文件准备常驻掩码。** 各报告保留准备耗时、payload 字节数、饱和掩码数。payload 只统计掩码字节，不含 Vec 等容器开销，更不是完整索引大小；本轮未测完整索引体积、构建/刷新成本、额外磁盘读取或加载成本。过滤阶段只计常驻掩码判断。因此本实验支持判断过滤机会，不能证明完整实现会获得相同净收益。', '',
    '## 每项查询：当前误报与匹配成本', '',
    '“FP 匹配 ms”是工作线程累计值；“扫描 ms / 占比”是墙钟值；“CLI ms”来自未插桩 `-c --no-refresh` 独立进程。`fallback` 查询的候选就是全部可搜索文件。']

def f(value, digits=2):
    return '—' if value is None else f'{value:.{digits}f}'

for name, rows in datasets.items():
    lines += ['', f'### {name}', '',
        '| 查询 ID | 候选 | 误报 | 误报 % | FP 匹配 ms | FP 匹配占比 % | 扫描 ms | 扫描占比 % | CLI ms |',
        '|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|']
    for r in sorted(rows, key=lambda r:r['id']):
        b=r['summary']['baseline']
        label=r['id']+(' (fallback)' if r['fallback'] else '')
        lines.append(f"| `{label}` | {b['candidates']:,} | {b['false_positive_files']:,} | {f(b['false_positive_pct'])} | {f(b['false_positive_match_worker_ms'])} | {f(b['false_positive_match_work_pct'])} | {f(b['scan_wall_ms'])} | {f(b['scan_wall_pct'])} | {f(r['cli_median_ms'])} |")

lines += ['', '## 候选确实减少的 nextMask 对照', '',
          '| 语料 / 查询 | 候选前→后 | 误报前→后 | 扫描 ms 前→后 | 常驻 mask 过滤 ms |',
          '|---|---:|---:|---:|---:|']
for name, rows in datasets.items():
    for r in sorted(rows, key=lambda r:r['id']):
        b,n=r['summary']['baseline'],r['summary']['nextmask']
        if b['candidates'] == n['candidates']: continue
        lines.append(f"| {name} / `{r['id']}` | {b['candidates']:,} → {n['candidates']:,} | {b['false_positive_files']:,} → {n['false_positive_files']:,} | {b['scan_wall_ms']:.3f} → {n['scan_wall_ms']:.3f} | {n['mask_filter_ms']:.3f} |")
lines += ['', '## 查询定义', '', '| 语料 | 查询 ID | flags | pattern |', '|---|---|---|---|']
for name,rows in datasets.items():
    for r in sorted(rows,key=lambda r:r['id']):
        pattern=r['pattern'].replace('|', r'\|')
        flags=' '.join(r['flags']) or '—'
        lines.append(f"| {name} | `{r['id']}` | `{flags}` | `{pattern}` |")
lines += ['', '## 可复现证据', '',
    '- [Rust 测量程序](../../benches/candidate_profile.rs)、[Python 驱动](../../benches/candidate_profile.py)。先运行 `cargo build --release --locked --bench candidate_profile --bin coderg`，然后用 Python 驱动的 `--help` 查看路径参数。索引目录请使用绝对路径。',
    '- [语料来源、archive SHA-256、硬件信息](data/nextmask-false-positive-2026-09-14/provenance.json)、[全集及仅索引查询汇总](data/nextmask-false-positive-2026-09-14/aggregate.json)。',
    '- [vLLM 逐查询汇总](data/nextmask-false-positive-2026-09-14/vllm/summary.json)、[Chromium 逐查询汇总](data/nextmask-false-positive-2026-09-14/chromium/summary.json)。各目录同时保存 metadata.json（代码、二进制、manifest 哈希）、commands.json.gz（完整命令）、每项查询的 json.gz（每轮原始计时、mask 条件和逐文件匹配行数）。',
    '- 本轮生成报告脚本也随数据归档；大语料、索引、下载包留在被 Git 忽略的 `.cache`，没有加入结果文件。', '']
report = repo / 'benches/results/nextmask-false-positive-2026-09-14.md'
report.write_text('\n'.join(lines))
if Path(__file__).resolve() != data / 'report.py':
    shutil.copy2(__file__, data / 'report.py')
print(json.dumps(aggregates, indent=2))
