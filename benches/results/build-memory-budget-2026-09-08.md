# 256 MiB 构建工作内存预算 — 2026-09-08

默认构建预算设为 **256 MiB** 后，vLLM 全量构建峰值 RSS 中位数从 **512.84 MiB 降到 208.08 MiB**，四倍语料从 **1,735.53 MiB 降到 213.67 MiB**。对应构建耗时从 **0.845 秒增至 1.313 秒**、从 **2.741 秒增至 5.335 秒**。这是用分块提取和磁盘外排降低内存的取舍，没有获得构建加速。

72 次新建索引的 lookup、postings 字节及文档元数据全部一致，40 次完整搜索输出与退出码校验一致。索引保持版本 4。配置口径和实现见[构建预算设计](../../docs/index-build-memory-budget.md)。

## 方法与语料

- 同一台 Apple M5、10 个逻辑 CPU、24 GiB RAM、macOS arm64；Rust `1.94.0 (4a4ef493e 2026-03-02)`。两版都使用 `cargo build --release --locked`。
- 基线为 `74387541f37851bd8df570b9663a507912178909` 加本轮开始前已有的查询覆盖工作区改动。新版本在同一基线上加入构建预算；不是将未提交的查询优化和内存改动混在前后差值里。双方源码差异及二进制 SHA-256 已归档。
- 每组语料、每版 1 次预热、5 次独立计时、另测 3 次 `/usr/bin/time -l` 峰值 RSS；每轮随机交错版本顺序，种子 `20260908`。所有计时和 RSS 样本均在沙箱外执行；不混入前期单次试跑。
- 使用热文件系统缓存，每次创建全新索引；不与其他 benchmark 并发。计时包含文件检查、内容读取、构建和落盘，不包含之后的哈希与搜索校验。RSS 是子进程峰值，不是最终索引体积，也不是工作缓冲区分配量。

| 语料 | 可搜索文件 | 源码 MiB | 来源 |
|---|---:|---:|---|
| vLLM | 6,835 | 82.38 | commit `569adb5a9780f9c02d22a6b29826acf711512356` |
| vLLM ×4 | 27,340 | 329.53 | 将基线 manifest 中每个可搜索文件复制到 `copy_0` 至 `copy_3`，保留文件元数据；新语料不包含 Git 仓库 |
| viberwhisper | 123 | 1.30 | 既有 bench 快照，commit `ab1f117d922fe06da6fc2f1f72605ba227ba4233` |
| few_large_64 | 64 | 64.00 | 既有刷新矩阵合成语料，64 个 1 MiB 文件 |

四倍语料用于观察关联数量增长后的构建工作集，是重复内容的规模实验，不代表语言或内容分布更广的新仓库。真实语料的 Git 状态在测量前后相同；全部路径、提交及生成说明见原始数据。

## 耗时与峰值 RSS

耗时取 5 次中位数，RSS 取 3 次峰值中位数。

| 语料 | 原版 ms | 256 MiB ms | 耗时变化 | 原版 RSS MiB | 256 MiB RSS MiB | RSS 降幅 |
|---|---:|---:|---:|---:|---:|---:|
| vLLM | 845.10 | 1,312.78 | +55.3% | 512.84 | 208.08 | 59.4% |
| vLLM ×4 | 2,741.30 | 5,334.50 | +94.6% | 1,735.53 | 213.67 | 87.7% |
| viberwhisper | 28.99 | 28.78 | −0.7% | 23.97 | 19.81 | 17.3% |
| few_large_64 | 168.27 | 176.97 | +5.2% | 159.77 | 68.58 | 57.1% |

256 MiB 版的三次峰值 RSS 样本（MiB）：

| 语料 | 样本 1 | 样本 2 | 样本 3 |
|---|---:|---:|---:|
| vLLM | 207.80 | 208.08 | 208.23 |
| vLLM ×4 | 213.42 | 214.30 | 213.67 |
| viberwhisper | 19.95 | 19.81 | 19.75 |
| few_large_64 | 68.58 | 68.31 | 68.58 |

在本轮四组语料中，256 MiB 版最高 RSS 为 **214.30 MiB**。文件数和源码量增加四倍时，主要构建内存没有相应倍增；仍有快照、manifest 等随文件数增长的开销。该观察不能升级为“任意仓库进程 RSS 永远小于 256 MiB”的保证。

viberwhisper 没有触发外排，耗时基本持平。few_large_64 同样没有外排，分块路径的整体耗时仍增加约 5.2%；本轮没有分别消融线程数、块去重和编码实现，因此不对这些因素作单独性能归因。没有重新测量搜索延迟。

## 临时磁盘与最终索引

| 语料 | 初始 run 数 | 累计 run 写入 MiB | run 文件峰值 MiB | 最终索引目录 MiB |
|---|---:|---:|---:|---:|
| vLLM | 2 | 541.50–541.95 | 541.50–541.95 | 66.40 |
| vLLM ×4 | 8 | 2,167.03–2,167.78 | 2,167.03–2,167.78 | 212.58 |
| viberwhisper | 0 | 0 | 0 | 1.87 |
| few_large_64 | 0 | 0 | 0 | 4.34 |

写入量包括初始有序 run 和归并输出，不包含最终索引文件。峰值是临时 run 的逻辑文件长度之和；本轮都只需一次归并，在删除输入前输入和输出同时存在，因此峰值与累计 run 写入量相同。实际文件系统空间还包括分配块取整、目录元数据和段编码临时文件。并行生产的到达顺序会轻微改变跨 run 去重分布，所以不同重复的临时字节数略有变化，最终段字节完全一致。

每次成功构建后，脚本检查本次 `.build-*`、`.write-*` 目录均已清理。读取失败、段编码失败和旧 manifest 保留由测试覆盖；没有在真实机器上注入磁盘耗尽、断电或强制终止。

## 正确性与工程检查

- **72 次构建**：四组语料 × 两版 × 九轮，段 SHA-256 及文档路径、长度、可搜索/有效状态一致；最终索引字节数一致。
- **40 次搜索校验**：每组、每版五个查询，覆盖固定字面量、短串回退、分支和大小写；比较完整标准输出与退出码。分块边界的实际命中另由 CLI 测试覆盖。
- **47 项 Rust 测试**通过：41 个单元测试、2 个构建预算 CLI 测试、3 个既有 CLI 测试、1 个 Git 增量测试。多于 32 个 run 的多轮归并、跨 run 重复记录、长 posting 流式编码和异常清理均有覆盖。
- `cargo clippy --all-targets --locked -- -D warnings` 和格式检查通过。

## 复现与原始数据

准备上表相同语料和前后两个 release 二进制；四倍语料按[生成说明](data/build-memory-budget-2026-09-08/vllm_x4.json)复制基线 manifest 中的可搜索文件。输出目录必须是语料外的新路径。

```sh
python3 benches/build_memory.py \
  --before /path/to/before --after /path/to/after \
  --corpus vllm=/path/to/vllm \
  --corpus vllm_x4=/path/to/vllm_x4 \
  --corpus viberwhisper=/path/to/viberwhisper \
  --corpus few_large_64=/path/to/few_large_64 \
  --output /tmp/coderg-build-memory-results \
  --iterations 5 --warmup 1 --rss-runs 3 --budgets 256
```

- [汇总](data/build-memory-budget-2026-09-08/summary.json)、[全部样本、命令和校验摘要](data/build-memory-budget-2026-09-08/results.json)、[stderr 与 RSS 原始日志](data/build-memory-budget-2026-09-08/stderr-logs.tar.gz)
- [基线源码差异](data/build-memory-budget-2026-09-08/source-before.diff)、[最终源码差异](data/build-memory-budget-2026-09-08/source-after.diff)、[执行脚本快照](data/build-memory-budget-2026-09-08/harness.py)
- [源码与归档文件 SHA-256、HEAD 和 Rust 版本](data/build-memory-budget-2026-09-08/manifest.json)
