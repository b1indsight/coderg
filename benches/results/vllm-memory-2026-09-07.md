# 建索引内存优化：vLLM 复测 — 2026-09-07

全量建索引峰值 RSS 中位数从 **2082.70 MiB 降到 517.70 MiB**，减少 **75.1%**；耗时从 **2.478 秒降到 0.743 秒**，约 **3.33×** 速度。整个索引目录仍为 **66.40 MiB**，lookup 和 postings 的字节内容与原版一致。默认搜索汇总耗时仍为 **47.75 ms**。

## 实现

原版同时保留全语料的逐文件哈希集合、近 800 万键的 `HashMap<Vec<u32>>` 及各键独立分配的文档列表，还会生成与文件长度成比例的字节对权重数组。此次改动：

1. 每个文件去重后立即转成紧凑的 `Vec<u32>`，释放集合桶；汇总时逐文件取走键并释放临时数组。
2. 用连续的 8 字节 `(gram, document_id)` 记录代替全局 postings 哈希表，预分配总记录数，使用 Rayon 并行原地排序、去重。编码器按同键的连续切片直接写入原有版本 4 格式。
3. 字节对权重按需查共享表或计算，移除每文件约 `8 × 文件字节数` 的临时权重数组。

全量构建和增量 delta 构建共用新的记录汇总与编码路径。现有索引可直接继续使用。构建仍需保存所有 gram—文件关联记录，内存随关联数量增长；本次没有引入磁盘外排或固定内存上限。

## 环境与方法

- 基线 commit：`18c9cc78c332c38d95d70dfade3ca50d9455f23c`；优化版为该提交加[源码差异](data/vllm-memory-2026-09-07/source.diff)。均使用 `cargo build --release --locked`。
- Apple M5，10 个逻辑 CPU，24 GiB RAM，macOS arm64；Rust 1.94.0，rg 15.2.0。
- vLLM commit：`569adb5a9780f9c02d22a6b29826acf711512356`，6,835 个文本文件、86,384,180 bytes（82.38 MiB）。
- 基线采用紧接本次修改前的[实测结果](vllm-performance-2026-09-07.md)。两次构建改进分别用同一脚本、同一语料测量。各版本 1 次预热、5 次独立计时、3 次独立 `/usr/bin/time -l` RSS 测量；每次新建索引。
- 使用热文件系统缓存，没有强制清理系统缓存；新建索引不代表冷启动。内存是进程峰值 RSS，包含驻留的 mmap 页。计时与内存测量分开执行，没有并发运行其他 benchmark。
- 搜索完整复跑 30 个模式，每项、每模式 3 次预热 + 31 次随机交错计时，并另测 3 次 RSS。计时输出包含文件名、行号和匹配行，重定向到 `/dev/null`。

## 建索引结果

| 版本 | 耗时中位 s | 耗时范围 s | RSS 中位 MiB | RSS 最大 MiB | 目录文件 MiB |
|---|---:|---:|---:|---:|---:|
| 原版 | 2.478 | 2.453–2.497 | 2082.70 | 2116.98 | 66.40 |
| 紧凑记录与直接编码 | 0.754 | 0.737–0.778 | 594.86 | 597.72 | 66.40 |
| 最终版：再移除权重数组 | 0.743 | 0.710–0.773 | 517.70 | 523.56 | 66.40 |

最终版 5 次耗时样本（秒）：0.710、0.773、0.724、0.743、0.744。
最终版 3 次峰值 RSS（MiB）：517.70、517.17、523.56。

索引目录全部文件合计 69,627,944 bytes（66.40 MiB）；`coderg stats` 的活跃索引为 66,987,931 bytes（63.88 MiB），差额来自 Git 树缓存清单。文件已分配磁盘块合计为 69,636,096 bytes（66.41 MiB），不含目录元数据。

## 搜索回归

表中耗时为 28 个可比查询各自中位数的等权平均；RSS 平均为各查询峰值 RSS 中位数的等权平均。内存最大值取所有样本的最大值。

| 模式 | 修改前 ms | 修改后 ms | 修改后 RSS 平均 MiB | 修改后 RSS 最大 MiB |
|---|---:|---:|---:|---:|
| coderg 默认 | 47.75 | 47.75 | 23.08 | 61.62 |
| coderg `--no-refresh` | 31.30 | 31.07 | 20.21 | 60.00 |
| rg | 71.57 | 71.55 | 16.59 | 26.75 |

搜索汇总耗时基本持平；默认模式在 25/28 项中快于 rg，`--no-refresh` 为 26/28。全部逐项 min/median/p95/mean、原始样本及查询命令保存在下方证据中。

## 正确性与验证

- `cargo test --locked`：18 个单元测试、2 个 CLI 集成测试、1 个 Git 增量集成测试全部通过；`cargo fmt --check`、`git diff --check` 通过。
- 扩展编码往返测试，覆盖无序输入、重复记录、跨 128 键块、`u32::MAX` 键和文档 ID；增加空段测试。gram 选择与独立的完整扫描参考实现比较，覆盖 4,096 字节查表阈值两侧和长输入。
- 最终版 9 次资源测量构建加 1 次搜索构建的 lookup/postings SHA-256 均与原版相同；归一化自动生成的 segment 标识及文件名后，manifest 与原版完全一致。
- 30 类查询在默认和 `--no-refresh` 两种模式下的完整输出、行数、退出码均与原版一致。28 类与 rg 相同；`async\s+def\s+\w+` 的跨行匹配和 `^[ \t]*$` 的空行计数仍是原有两项差异，继续排除出对比统计。
- 搜索前后源码、二进制、全部索引文件 SHA-256、语料 commit 和工作区状态均保持一致；资源测量前后语料和二进制同样不变。

## 原始数据与复现

- [汇总](data/vllm-memory-2026-09-07/summary.json)、[最终建索引原始数据](data/vllm-memory-2026-09-07/optimized-on-demand-index/results.json)、[中间版本数据](data/vllm-memory-2026-09-07/optimized-index/results.json)
- [搜索样本与环境](data/vllm-memory-2026-09-07/search/results.json)、[查询命令](data/vllm-memory-2026-09-07/search/commands.json)、[252 份 RSS 日志](data/vllm-memory-2026-09-07/search/rss-logs.tar.gz)
- [段文件字节一致性](data/vllm-memory-2026-09-07/segment-parity.json)、[输出与 manifest 一致性](data/vllm-memory-2026-09-07/output-parity.json)
- [最终源码差异](data/vllm-memory-2026-09-07/source.diff)、[中间版本差异](data/vllm-memory-2026-09-07/flat-postings.diff)、[归档文件 SHA-256](data/vllm-memory-2026-09-07/manifest.json)

使用相同语料 checkout；索引和输出目录须为新路径且位于语料之外。峰值 RSS 测量依赖 macOS。

```sh
cargo build --release --locked
python3 benches/results/data/vllm-memory-2026-09-07/index_benchmark.py \
  --root /path/to/vllm --binary target/release/coderg \
  --output /tmp/coderg-memory-index-results
python3 benches/regex_suite.py \
  --root /path/to/vllm --binary target/release/coderg \
  --index-dir /tmp/coderg-memory-search-index \
  --output /tmp/coderg-memory-search-results \
  --iterations 31 --warmup 3 --rss-runs 3
```

归档的 `search/harness.py` 保存此次脚本的原始字节；如使用归档版本，应将其放回 `benches/regex_suite.py` 的位置，以正确定位源码。
