import ast
import hashlib
import json
import pathlib
import shutil

work = pathlib.Path(__file__).parent
repo = pathlib.Path('/Users/b1indsight/personal_work/coderg')
stem = 'small-repo-threads-2026-09-07'
out = repo / 'benches/results'
archive = out / 'data' / stem
archive.mkdir(parents=True, exist_ok=True)
summary = json.loads((work / 'summary.json').read_text())
tuning = json.loads((work / 'tuning.json').read_text())
confirmation = json.loads((work / 'confirmation.json').read_text())
metadata = json.loads((work / 'matrix/metadata.json').read_text())

for name in ('matrix.py', 'regex_suite.py', 'tune_prepare.py', 'tune.py', 'tune-plan.json',
             'tuning.json', 'tuning-initial.json', 'analyze.py', 'summary.json', 'per-query.csv',
             'edge_cases.py', 'edge-cases.json', 'confirm.py', 'confirmation.json', 'source.diff', 'report.py'):
    shutil.copy2(work / name, archive / name)
shutil.copy2(work / 'matrix/metadata.json', archive / 'metadata.json')
for dataset in metadata['plan']['datasets']:
    target = archive / dataset['id']
    target.mkdir(exist_ok=True)
    shutil.copy2(work / 'matrix' / dataset['id'] / 'results.json', target / 'results.json')

table = '\n'.join(
    f'| {g["id"]} | {tuning["summary"][g["id"]]["files"]:,} | '
    f'{g["modes"]["baseline"]["median"]:.2f} | {g["modes"]["optimized"]["median"]:.2f} | '
    f'{g["change_pct"]:+.1f}% | {g["modes"]["rg"]["median"]:.2f} | '
    f'{g["faster_than_rg"]}/{g["rg_compatible_queries"]} |'
    for g in summary)
intervals = '\n'.join(f'| {g["id"]} | {g["bootstrap_95"][0]:+.2f}% ～ {g["bootstrap_95"][1]:+.2f}% |'
                       for g in summary)
no_refresh = '\n'.join(f'| {g["id"]} | {g["modes"]["baseline_no_refresh"]["median"]:.2f} | '
                         f'{g["modes"]["optimized_no_refresh"]["median"]:.2f} |'
                         for g in summary)
tuning_table = '\n'.join(f'| {name} | {g["files"]:,} | {g["serial"]["median"]:.3f} | '
                           f'{g["parallel1"]["median"]:.3f} | {g["parallel"]["median"]:.3f} |'
                           for name, g in sorted(tuning['summary'].items(), key=lambda pair: pair[1]['files']))
confirm_table = '\n'.join(f'| {name} | {g["before_ms"]:.3f} | {g["after_ms"]:.3f} | '
                            f'{g["change_pct"]:+.2f}% | {g["bootstrap_95"][0]:+.2f}% ～ {g["bootstrap_95"][1]:+.2f}% |'
                            for name, g in confirmation['summary'].items())
stable_increases = [name for name, g in confirmation['summary'].items() if g['bootstrap_95'][0] > 0]
confirmation_conclusion = ('追加复测没有稳定复现初测的约 3% 回退，三项区间均覆盖零。'
                          if not stable_increases else '追加复测仍有正向耗时区间：' + '、'.join(stable_increases) + '；保留为已观测回退。')

text = f'''# 小仓库遍历线程策略 — 2026-09-07

根据上次文件快照选择串行或并行遍历后，viberwhisper 的默认搜索平均耗时从 **8.30 ms 降到 6.40 ms（−22.8%）**，agentflow 从 **8.56 ms 降到 6.42 ms（−25.0%）**。256 个文件的语料下降 **23.2%**，64 个大文件的语料下降 **17.4%**。三组大仓库的汇总变化区间都覆盖零。

## 实现

- 上次元数据快照不超过 **512 个文件**时，刷新通过 `WalkBuilder::build` 在调用线程收集和排序文件快照，避免并行 walker 创建工作线程和等待退出的开销。
- 更大的历史快照以及没有历史快照的首次构建使用原有并行遍历；候选文件匹配仍使用 Rayon。没有修改全局线程池配置。
- 常规刷新、Git 状态改变后的刷新和缓存 Git 树的快照重建都传入对应的历史文件数。计数包含二进制文件等元数据条目，不只是可检索文本。
- 文件数只作为执行方式的提示。即使从空目录突然增长到数百、数千文件，也完整遍历当前目录，不会按历史数量截断结果。下一次刷新使用更新后的快照大小。
- 两条路径共用隐藏文件、ignore、符号链接和索引目录排除配置，采用相同的路径比较规则，传播遍历错误。索引格式和查询逻辑没有改变。

实现见 [index.rs](../../src/index.rs)。[本轮源码差异](data/{stem}/source.diff)相对于前一轮刷新优化版生成，包含新增的回归测试。

## 阈值校准

直接在 `collect_files` 内部计时，每次启动新进程，热文件系统缓存，各预热 5 次、随机交错计时 51 次。比较串行迭代器、单工作线程并行 walker、默认并行 walker（这台机器 10 线程）。这个实验只运行文件收集，不包含 Git、索引加载或候选匹配。

| 语料 | 元数据文件数 | 串行 ms | 单工作线程 ms | 默认并行 ms |
|---|---:|---:|---:|---:|
{tuning_table}

串行和单工作线程的结果接近；串行路径避免了额外工作线程和批次锁。512 文件的合成语料串行仍有明显优势。虽然这组合成目录到 2,048 文件时串行也更快，但不同目录结构的收益会变化，因此保守采用 512 的阈值；4,096 文件及两个更大的语料继续并行。

前一轮[分阶段诊断](small-repo-profile-2026-09-07.md)观察到 `ignore 0.4.33` 的并行 walker 在等待任务/退出消息时使用 1 ms 休眠；本轮通过选择串行路径避免这类线程调度开销，没有修改依赖库。

## 完整搜索 benchmark

基线是前一轮刷新优化后的二进制，新版只增加小仓库遍历策略。各查询比较两版默认、两版 no-refresh 和 rg 五种模式，预热 3 次，随机交错计时 21 次，共 **10,710 次计时**。全部为热文件系统缓存、新 CLI 进程，stdout 重定向至 `/dev/null`，输出校验单独进行。

环境为同一台 Apple M5 / macOS；本轮在工作区沙箱内执行，不与先前沙箱外矩阵的样本合并。本轮没有重新采样 RSS；每版每语料的单次构建计时只用于运行检查，不据此判断构建速度。

基线 SHA-256：`{metadata['binaries']['baseline']['sha256']}`。新版 SHA-256：`{metadata['binaries']['optimized']['sha256']}`。

下表是各查询中位数的等权平均，单位 ms；只汇总与 rg 输出一致的查询。vLLM 为 36 项，两个小型真实仓库各 16 项，合成语料各 8 项。

| 语料 | 元数据文件数 | 优化前 | 优化后 | 变化 | rg | 新版快于 rg |
|---|---:|---:|---:|---:|---:|---:|
{table}

按轮次配对 bootstrap 1,000 次，汇总变化的 95% 区间如下。这些区间仅描述固定工作负载下的计时波动，未做多重比较修正。

| 语料 | 默认搜索变化区间 |
|---|---:|
{intervals}

no-refresh 不执行本次修改的遍历分支，其结果基本持平：

| 语料 | 优化前 no-refresh ms | 优化后 no-refresh ms |
|---|---:|---:|
{no_refresh}

## 大仓库单项复测

初测耗时增长最大的三项约为 +3%。对它们另预热 5 次，随机交错计时 101 次，使用同一对二进制和独立索引；下表给出中位数及配对 bootstrap 2,000 次的变化区间。

| 查询 | 优化前 ms | 优化后 ms | 变化 | 95% 区间 |
|---|---:|---:|---:|---:|
{confirm_table}

{confirmation_conclusion} [原始复测样本](data/{stem}/confirmation.json)。

## 正确性与验证

- **102 项查询**的两版默认/no-refresh 输出全部一致；其中 **100 项**也与 rg 一致。vLLM 的 `async_def` 和 `blank_lines` 保留既有语义差异，未纳入 rg 性能汇总。
- 七组语料的首次构建段文件哈希在两版之间一致；各语料的源码、Git 状态、共用索引和二进制在测试前后保持不变。
- 新增单元测试比较串行与并行的完整快照，覆盖错误的历史文件数提示、600 个新增文件、隐藏文件、ignore 规则、自定义索引目录、符号链接和错误传播。
- 另用两版二进制验证七个实际阶段：小仓库无变化、增加 600 文件越过阈值、大仓库无变化、删除文件回到阈值以内、再次无变化、无效 ignore 规则、恢复 ignore 规则。搜索输出和刷新诊断全部一致。[场景结果](data/{stem}/edge-cases.json)。
- `cargo test --locked`：19 个单元测试、3 个 CLI 集成测试、1 个 Git 集成测试通过。`cargo fmt --check`、Clippy `-D warnings` 和 `git diff --check` 通过。

## 数据与复现

[逐查询 CSV](data/{stem}/per-query.csv)、[汇总 JSON](data/{stem}/summary.json)、[环境与命令参数](data/{stem}/metadata.json)及各语料完整样本均已保存。[归档清单](data/{stem}/manifest.json)记录文件大小和 SHA-256。

`matrix.py` 复用广泛 benchmark 的查询和校验逻辑，只将仓库路径固定，并使用已有 CPU 环境记录，避免要求额外系统信息读取权限。`--rss-runs 0` 的命令参数保存在 metadata 中；原始语料的生成与固定提交见[上一轮报告](refresh-matrix-2026-09-07.md)。

`tune_prepare.py` 在独立目录复制源码并追加仅用于诊断的入口；`tune.py` 保存三种遍历方式的所有计时。`tuning-initial.json` 为最初只比较两种方式的探索样本，未合并进最终三方式统计。所有脚本中的工作区、语料和二进制路径需要按本机环境调整。
'''
(out / (stem + '.md')).write_text(text)
for path in archive.glob('*.py'):
    ast.parse(path.read_text(), filename=str(path))
files = {str(p.relative_to(archive)): {'bytes': p.stat().st_size,
         'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for p in archive.rglob('*')
         if p.is_file() and p.name != 'manifest.json'}
(archive / 'manifest.json').write_text(json.dumps(files, indent=2) + '\n')
print(out / (stem + '.md'))
print('archived bytes:', sum(v['bytes'] for v in files.values()))
