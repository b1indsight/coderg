# 刷新路径优化：vLLM 对比 — 2026-09-07

在同一份已建索引、未变化的 vLLM 工作区上，四项查询的默认搜索耗时中位数平均值从 **29.98 ms 降到 25.30 ms，减少 15.6%**。单文件修改后的首次搜索从 **45.95 ms 降到 38.19 ms**；缓存回退后的首次搜索从 **79.87 ms 降到 67.93 ms**。

30 项正则查询的完整输出与基线一致；7 种 Git 状态转换的刷新诊断、完整输出和索引段校验全部通过。默认搜索的峰值 RSS 中位数平均值增加 **2.80 MiB**。

## 改动

- 遍历线程各自收集文件快照，退出时获取一次锁发布整批结果，替代每个文件争用结果锁。所有批次的错误仍会传播，发生错误时不发布不完整快照。
- 汇总时按文件总数预分配空间，用 Rayon 并行排序；保留原有 `Path` 比较规则、文件编号和快照内容。
- manifest 通过 `fs::read` 和 `serde_json::from_slice` 解析，减少 reader adapter 的逐字节处理开销。临时 JSON 缓冲区在解析完成时释放。

源码见 [index.rs](../../src/index.rs)，相对于基线的实现和测试差异保存在 [source.diff](data/vllm-refresh-2026-09-07/source.diff)。这些改动同时影响默认刷新、首次构建、Git tree 缓存加载，以及无需刷新的 manifest 加载。

刷新仍完整遍历目录并读取文件元数据，时间复杂度仍随文件数量增长。新鲜度仍依赖路径、长度和 mtime；同长度、同 mtime 的内容替换仍有原有局限。

## 环境与方法

- 分支：`perf/refresh-path`；基线 commit：`403fd12bfc7c612181773ae5fa7a49e87bb045bf`。优化版为该基线加归档的源码差异；二进制和源码 SHA-256 均保存在结果中。
- Apple M5，10 个逻辑 CPU，24 GiB RAM，macOS 26.6.2 arm64；Rust 1.94.0；`cargo build --release --locked`。
- vLLM commit：`569adb5a9780f9c02d22a6b29826acf711512356`，6,835 个文本文件、82.38 MiB 文本。
- 热文件系统缓存。初始建索引在计时之外；已有索引由基线构建，两个版本共用这一索引，测试前后所有索引文件的 SHA-256 相同。
- 搜索输出文件名、行号和完整匹配行；计时输出重定向至 `/dev/null`。
- 四项计时查询分别预热 3 次、计时 31 次；每轮随机交错两个版本、默认和 `--no-refresh` 四种组合，不并发运行。另测 3 次峰值 RSS，内存采样不混入耗时。
- 30 项正则查询在计时之外比较退出码和完整 stdout；默认和 `--no-refresh` 均与基线逐字节一致。这里验证的是对基线的兼容性，保留既有的两处 rg 语义差异。
- 语料 Git 状态、索引文件、二进制和源码在测量前后均检查不变。

## 未变化工作区的连续搜索

耗时为各查询的 31 次计时中位数，单位 ms。

| 查询 | 基线默认 | 优化默认 | 降幅 | 基线 no-refresh | 优化 no-refresh |
|---|---:|---:|---:|---:|---:|
| 不存在的正则标记 | 27.18 | 22.97 | 15.5% | 10.60 | 7.77 |
| 固定字符串 `get_tensor_model_parallel_world_size` | 28.85 | 24.19 | 16.1% | 12.64 | 9.67 |
| `\bSamplingParams\b` | 30.86 | 25.95 | 15.9% | 14.43 | 11.46 |
| `-i '\bSamplingParams\b'` | 33.02 | 28.08 | 15.0% | 16.97 | 14.14 |
| 中位数等权平均 | **29.98** | **25.30** | **15.6%** | **13.66** | **10.76** |

`--no-refresh` 的收益来自共用的 manifest 加载路径。默认与 no-refresh 的汇总耗时差从 16.32 ms 降到 14.54 ms；这个差值包含路径间的调度和测量差异，不能当作单独计时的刷新阶段。三项改动没有分别做正式消融，所以不把默认搜索的全部收益归因于文件遍历。

峰值 RSS 为四项查询各自峰值 RSS 中位数的等权平均，单位 MiB：

| 模式 | 基线 | 优化版 | 增量 |
|---|---:|---:|---:|
| 默认 | 17.48 | 20.28 | +2.80 |
| no-refresh | 14.04 | 16.62 | +2.59 |

该索引的 manifest 为 2,640,013 bytes（2.52 MiB）。切片解析需要额外保存这份 JSON 缓冲区；批次汇总也有临时存储。峰值 RSS 包含短期分配和驻留 mmap 页，不能理解为每次搜索后持续增加的常驻内存。索引格式保持版本 4，已有索引内容没有变化。

## 修改、提交与回退后的首次搜索

使用现有 [vllm_git_transitions.py](data/legacy-harness-2026-09-22/vllm_git_transitions.py)，在临时本地克隆中运行完整流程：预热 1 轮，计时 11 轮，另测 RSS 3 轮。每轮分别为两个版本建立索引；每个状态随机交错两个版本，计时包括刷新、更新、manifest 发布/重载和最终搜索。原始 vLLM 工作区保持不变。

| 阶段 | 基线 ms | 优化 ms | 降幅 | 基线 RSS MiB | 优化 RSS MiB |
|---|---:|---:|---:|---:|---:|
| 干净工作区 | 31.27 | 26.60 | 14.9% | 15.31 | 17.95 |
| 未提交的单文件修改 | 45.95 | 38.19 | 16.9% | 19.36 | 22.42 |
| 提交已索引内容 | 75.63 | 68.16 | 9.9% | 22.09 | 24.48 |
| 直接提交 32 个文件的修改 | 87.70 | 79.48 | 9.4% | 28.20 | 28.36 |
| 同树空提交 | 60.85 | 54.79 | 10.0% | 22.03 | 24.38 |
| 回退到缓存树 | 79.87 | 67.93 | 14.9% | 22.17 | 24.50 |
| 回退后的再次搜索 | 30.72 | 26.77 | 12.8% | 15.52 | 18.27 |

每轮 7 个阶段的完整输出与 rg 一致。已有 overlay 的提交、同树空提交和缓存回退没有新增段；对应状态下两个版本的 lookup/postings 字节内容一致。每个阶段的全部样本、刷新诊断和索引文件哈希均在原始结果中。

## 验证与复现

`cargo test --locked` 通过 18 个单元测试、3 个 CLI 测试和 1 个 Git 增量测试；`cargo fmt --check`、`git diff --check`、`cargo clippy --all-targets --locked -- -D warnings` 全部通过。新增 CLI 测试覆盖 32 个目录中的新增、删除、重命名、内容修改、隐藏目录和 ignore 规则变更，并确认再次搜索不重写 manifest。

以下命令中的 `BASELINE` 是基线 release 二进制路径，`SOURCE` 是干净的 vLLM 路径；`INDEX`、`SEARCH_RESULTS` 和 `GIT_RESULTS` 使用语料外的新目录：

```sh
cargo build --release --locked
"$BASELINE" index "$SOURCE" --index-dir "$INDEX"
python3 -B benches/results/data/vllm-refresh-2026-09-07/search/harness.py \
  --repo . --root "$SOURCE" --baseline "$BASELINE" \
  --binary target/release/coderg --index "$INDEX" --output "$SEARCH_RESULTS"
python3 benches/vllm_git_transitions.py \
  --root "$SOURCE" --baseline "$BASELINE" --binary target/release/coderg \
  --output "$GIT_RESULTS" --iterations 11 --warmup 1 --rss-runs 3
```

搜索结果的 31 样本 p95 和 Git 阶段的 11 样本统计只描述本次单机、单仓库热缓存测量，不代表冷缓存、其他平台或更大仓库的表现。

- [搜索结果、样本、命令与 30 项输出哈希](data/vllm-refresh-2026-09-07/search/results.json)
- [Git 转换结果、样本、索引段校验](data/vllm-refresh-2026-09-07/git-transitions/results.json)
- [Git 转换峰值 RSS 日志](data/vllm-refresh-2026-09-07/git-transitions/rss-logs.tar.gz)
- [归档清单及 SHA-256](data/vllm-refresh-2026-09-07/manifest.json)
