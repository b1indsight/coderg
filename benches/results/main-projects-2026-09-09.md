# 主分支整合后的真实项目 benchmark — 2026-09-09

本轮将构建预算、gram 提取与 4 路归并、查询覆盖与分支执行，以及对应文档和实验记录整合到本地 `main`，代码提交为 **`a4cfe2d7f1b56cce2e4a3cd544d61e45fdd124cd`**。工作区原本已在 `main`，因此直接提交已有改动；仓库没有配置 Git 远端。

随后对已有的 vLLM、viberwhisper 和 agentflow 冻结快照重新进行同轮前后对照。**vLLM 构建峰值 RSS 中位数减少 59.2%，构建耗时增加 5.0%；两个小项目的构建耗时分别减少 11.9% / 3.6%。默认搜索整组表现接近原主分支，vLLM 均值小幅减少 0.8%，两个小项目基本持平。** 新旧索引段和全部 70 项查询输出一致。

## 版本、语料与方法

基线为提交前的主分支 **`74387541f37851bd8df570b9663a507912178909`**，从 Git 导出原始源码后独立构建。候选为上述整合提交，默认使用 256 MiB 构建工作缓冲预算、32 KiB 输入块和最多 4 个最终归并任务。双方都使用 release 配置；为强制 Cargo 重新编译，分别传入仅用于区分构建的 cfg 标记，具体命令及二进制 SHA-256 已归档。

基线包含此前已提交的内存数据结构优化和刷新优化，候选还包含本次提交的查询逻辑改进。因此本报告衡量的是两个主分支版本的整体差异，不能把所有变化归因于最终归并一项。

| 项目 | 可搜索文件 | 源码 MiB | 冻结快照提交 |
|---|---:|---:|---|
| vLLM | 6,835 | 82.38 | `569adb5a9780f9c02d22a6b29826acf711512356` |
| viberwhisper | 123 | 1.30 | `ab1f117d922fe06da6fc2f1f72605ba227ba4233` |
| agentflow | 114 | 1.38 | `40efd5bcc760bc1aa05029515d2eb46d3dff3c30` |

这三份语料沿用已有项目 benchmark 快照；本轮没有更新其源码或使用运行中的项目工作区。机器为 Apple M5、10 个逻辑 CPU、24 GiB RAM、macOS arm64。Rust 和 rg 的版本、语料绝对路径与 Git 状态见[运行元数据](data/main-projects-2026-09-09/results/metadata.json)。未设置 `RAYON_NUM_THREADS`。

- 完整构建：每版本、每项目先做 1 次初始化构建，再随机交错计时 7 次；另外独立测量 3 次 `/usr/bin/time -l` 峰值 RSS。每次构建使用新索引目录。
- 查询：每项查询对旧版默认、旧版 `--no-refresh`、新版默认、新版 `--no-refresh` 和 rg 五种模式，随机交错预热 3 轮、计时 31 轮，另测 3 轮 RSS。随机种子为 `20260909`。
- 使用热文件缓存，计时包含独立进程启动及命令执行；查询输出校验在计时外进行。编译、测试和其他 benchmark 不与测量并发。
- 默认搜索包含刷新检查，`--no-refresh` 排除该检查。新旧版本共用基线生成、且已验证与新版字节相同的索引，避免索引内容差异影响查询比较。

## 完整构建与峰值内存

耗时取 7 次中位数；RSS 为各次进程峰值再取 3 次中位数。

| 项目 | 旧主分支 ms | 当前主分支 ms | 耗时变化 | 旧 RSS MiB | 当前 RSS MiB | RSS 降幅 |
|---|---:|---:|---:|---:|---:|---:|
| vLLM | 762.05 | 799.95 | +5.0% | 521.91 | 213.12 | **59.2%** |
| viberwhisper | 26.09 | 23.00 | −11.9% | 23.83 | 21.73 | 8.8% |
| agentflow | 30.82 | 29.71 | −3.6% | 24.83 | 21.20 | 14.6% |

当前版本本轮构建 RSS 观测最大值分别为 213.16 / 21.80 / 21.39 MiB。256 MiB 参数控制主要构建缓冲区，不是进程 RSS 硬限制；具体边界见[预算设计](../../docs/index-build-memory-budget.md)。

这次 vLLM 基线重新测得 762 ms，当前版约 800 ms。此前[构建演进文档](../../docs/index-build-evolution.md)列出的 845 → 792 ms 是不同历史轮次的累计汇总，且当时的预算实验基线已含未提交的查询改动。本轮使用干净旧主分支，同轮数据应优先用于评价这两个主分支版本，保留“内存大幅下降、构建仍有约 38 ms 额外成本”的结论。

## 查询延迟

下表只纳入与 rg 输出一致的查询：vLLM 36 项、其余各 16 项。每一格为**各查询耗时中位数的等权平均**，不是某一条查询的耗时，也不是所有原始样本混合后的中位数。

| 项目 | 查询数 | 默认：旧 → 新 ms | 变化 | no-refresh：旧 → 新 ms | 变化 | rg ms |
|---|---:|---:|---:|---:|---:|---:|
| vLLM | 36 | 43.79 → 43.43 | −0.81% | 28.56 → 28.02 | −1.88% | 72.03 |
| viberwhisper | 16 | 6.28 → 6.29 | +0.19% | 4.85 → 4.87 | +0.52% | 7.44 |
| agentflow | 16 | 6.46 → 6.50 | +0.60% | 5.23 → 5.23 | 约 0% | 8.52 |

默认模式整体差值的 1,000 次 bootstrap 95% 区间分别为 `[-1.28%, -0.43%]`、`[-0.57%, +0.80%]`、`[-0.26%, +1.56%]`。两个小项目的区间覆盖零，不能据此声称稳定加速或回退；vLLM 的收益也只有约 0.35 ms 的均值差。区间描述本轮样本波动，不覆盖不同时间、机器或语料的系统差异。

新版默认模式分别在 33/36、16/16、15/16 项查询上快于 rg，分别在 16/36、6/16、4/16 项上快于旧主分支；这些按中位数计数，不是统计显著性计数。整组均值改善不代表每一项都更快。

各查询 p95 的等权平均，旧 → 新默认搜索为 vLLM 46.34 → 45.72 ms、viberwhisper 6.91 → 6.88 ms、agentflow 7.05 → 7.07 ms。单项原始样本、p95 和查询峰值 RSS 均保留在[汇总](data/main-projects-2026-09-09/summary.json)及[逐查询表](data/main-projects-2026-09-09/per-query.csv)。

## 单项收益与回退

以下列出部分默认搜索结果，展示整组均值背后的差别；完整查询模式和 flags 在各项目原始 JSON 中。

| 项目 / 查询 ID | 旧 ms | 新 ms | 变化 |
|---|---:|---:|---:|
| vLLM / `wide_icase_config` | 41.23 | 35.36 | −14.2% |
| vLLM / `cache` | 40.88 | 36.75 | −10.1% |
| vLLM / `wide_icase_tensor_creation` | 43.83 | 40.23 | −8.2% |
| vLLM / `wide_icase_cache` | 55.39 | 51.38 | −7.2% |
| viberwhisper / `async` | 6.27 | 5.86 | −6.5% |
| viberwhisper / `icase_literal` | 6.59 | 6.90 | +4.7% |
| vLLM / `config_assignment` | 29.42 | 30.77 | +4.6% |
| vLLM / `icase_literal` | 31.55 | 32.80 | +4.0% |
| vLLM / `icase_platform` | 42.63 | 44.28 | +3.9% |

上述回退的本轮 bootstrap 区间不覆盖零，绝对增量约 0.31–1.65 ms，应保留为后续查询优化的参考。逐查询区间未做多重比较校正，本轮也没有对这些模式单独插桩，因此不把差值直接归因于某个函数。

## 正确性与复现

整合前 `cargo test --offline` 共 52 项通过，`cargo fmt --check`、`cargo clippy --all-targets --offline -- -D warnings` 和 diff 检查通过。本轮 benchmark 完成 66 次构建；三个项目初始生成的新旧索引段 SHA-256 均相同。

70 项查询的新旧默认及 no-refresh 完整输出按行规范化后均一致；68 项与 rg 一致。vLLM 的 `async_def` 比 rg 多 4 行，`blank_lines` 多 6,440 行，均是基线已有的跨行或空行计数差异，新版没有增加差异。这两项继续保留在原始结果中，排除出与 rg 的性能汇总。

所有项目的 Git 提交和干净状态、查询使用的索引文件哈希、两版二进制哈希及当前源码哈希在测量结束后均通过一致性检查。临时构建与查询索引由脚本清理，项目快照没有改动。

实际执行命令及提交/二进制校验记录见[provenance.json](data/main-projects-2026-09-09/provenance.json)。复现使用仓库中的脚本：

```sh
python3 benches/refresh_matrix.py \
  --plan PLAN_JSON --baseline OLD_BINARY --binary NEW_BINARY \
  --output NEW_OUTPUT_DIR --iterations 31 --warmup 3 \
  --build-iterations 7 --rss-runs 3 --seed 20260909
```

[语料计划](data/main-projects-2026-09-09/plan.json)、[vLLM 原始结果](data/main-projects-2026-09-09/results/vllm/results.json)、[viberwhisper 原始结果](data/main-projects-2026-09-09/results/viberwhisper/results.json)、[agentflow 原始结果](data/main-projects-2026-09-09/results/agentflow/results.json)及 [RSS 日志包](data/main-projects-2026-09-09/rss-logs.tar.gz)已归档。同目录保留执行脚本快照和 `analyze.py`；在该目录运行后者可重新生成汇总和逐查询 CSV。
