# 固定字面量优先提取：2026-09-08 benchmark

版本说明：本报告对应第一遍禁止字符类展开的中间实现。当前已进一步改为小字符类与显式分支共用组合预算，结果见[后续实验](small-variant-extraction-2026-09-08.md)。

装饰器查询的默认延迟下降约 8.6%，key 数从 122 降到 3；但引号内设备名和张量创建查询存在明显回退。原有 102 项查询的中位数总和基本持平：默认 +0.02%，no-refresh +0.20%，没有显示整体加速。

本轮基线是上一版多片段覆盖实现。新版本先尝试固定字面量，缺少可用固定条件时才展开字符类，并在选 gram 前删除被其他字面量组包含的条件。gram 生成、权重、索引格式、刷新、postings 执行和最终正则验证保持一致。

## 方法

基线二进制在重新构建前保存，SHA-256 与[上一轮 metadata](data/query-covering-2026-09-08/metadata.json)的 optimized 二进制一致。基线源码也逐文件对照上一轮诊断记录校验；本次另存了[基线 query.rs](data/fixed-literal-extraction-2026-09-08/before-query.rs)和[源码校验表](data/fixed-literal-extraction-2026-09-08/before-source-sha256.json)。这里的基线已经包含多片段覆盖，不是最初的 `7438754` 单 gram 版本。

环境为 Apple M5、24 GiB、macOS 26.6.2、Rust 1.94.0。复用 3 个真实仓库、4 组合成文件规模语料，以及上一轮共享最长片段的定向语料，共 110 项查询。主计时使用 release 构建、热缓存、新 CLI 进程、默认线程配置；每项预热 3 轮、计时 21 轮，随机交错运行修正前后默认搜索、两者的 `--no-refresh` 和 rg。输出校验放在计时外，同一语料共用基线构建的索引。

诊断在临时插桩源码中进行，每项预热 1 轮、随机交错采样 11 轮，使用单基础段索引。完整搜索延迟使用未插桩二进制；诊断时间不能代替完整搜索延迟。计数和全部诊断样本见[metrics.json](data/fixed-literal-extraction-2026-09-08/metrics.json)。

## 正确性与整组结果

- `cargo test` 的 28 个单元测试、3 个 CLI 测试、1 个 Git 增量测试全部通过。
- `cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`、`git diff --check` 通过。
- 110/110 项新旧完整输出一致；108 项与 rg 一致，另外两项保留基线已知的语义差异。
- 8 组语料的新旧构建生成相同的索引段字节；所有搜索后的语料、索引、二进制及源码校验均通过。

以下为每组各查询中位数之和，单位 ms，不代表一次搜索耗时。原有 102 项查询的默认搜索和 no-refresh 均有 48 项中位数下降；这不是统计显著性计数。原始数据包含所有采样，汇总入口为[per-query.csv](data/fixed-literal-extraction-2026-09-08/per-query.csv)。

| 语料 | 查询数 | 默认：修正前 → 修正后 | 变化 | no-refresh：修正前 → 修正后 | 变化 |
|---|---:|---:|---:|---:|---:|
| vLLM | 38 | 1801.59 → 1809.04 | +0.41% | 1224.32 → 1228.09 | +0.31% |
| viberwhisper | 16 | 107.12 → 108.01 | +0.82% | 82.98 → 83.72 | +0.89% |
| agentflow | 16 | 110.09 → 111.20 | +1.01% | 90.17 → 91.01 | +0.93% |
| 256 小文件 | 8 | 57.66 → 57.71 | +0.08% | 46.71 → 46.98 | +0.58% |
| 4096 常规文件 | 8 | 248.39 → 248.36 | −0.01% | 183.62 → 184.00 | +0.20% |
| 16384 小文件 | 8 | 858.98 → 849.70 | −1.08% | 697.21 → 695.52 | −0.24% |
| 64 大文件 | 8 | 83.29 → 83.60 | +0.37% | 75.48 → 75.94 | +0.61% |
| 原有矩阵合计 | 102 | 3267.12 → 3267.62 | +0.02% | 2400.49 → 2405.27 | +0.20% |
| 额外共享片段语料 | 8 | 73.79 → 73.59 | −0.28% | 59.65 → 58.60 | −1.77% |

固定字符串路径未改变，共享片段语料在这次比较中主要用于确认已有覆盖能力和结果保持一致；上一轮相对单 gram 基线的显著收益不应归入这次提取修正。

## 代表性查询的完整延迟

以下为 vLLM 查询的中位数，单位 ms，变化为新版本相对基线。正数表示耗时增加。完整样本、p95、最小值和机器信息保存在[原始计时数据](data/fixed-literal-extraction-2026-09-08/vllm/results.json)及[metadata](data/fixed-literal-extraction-2026-09-08/metadata.json)中。

| 查询 | 默认：修正前 → 修正后 | 变化 | no-refresh：修正前 → 修正后 | 变化 |
|---|---:|---:|---:|---:|
| decorator | 41.75 → 38.17 | −8.6% | 26.24 → 22.55 | −14.1% |
| cuda_call | 27.13 → 27.63 | +1.8% | 12.47 → 12.12 | −2.8% |
| env_vars | 37.03 → 36.14 | −2.4% | 21.52 → 20.99 | −2.5% |
| def_forward | 34.53 → 34.17 | −1.0% | 19.00 → 18.96 | −0.2% |
| tensor_creation | 39.90 → 44.03 | +10.4% | 24.29 → 28.38 | +16.8% |
| quoted_device | 32.65 → 42.69 | +30.8% | 17.44 → 27.09 | +55.3% |

装饰器默认 p95 从 43.50 降到 38.90 ms，no-refresh p95 从 28.18 降到 23.62 ms；rg 同项默认中位数为 74.19 ms。`quoted_device` 默认 p95 从 34.36 增到 44.49 ms。小幅变化仍可能受单机计时波动影响，本报告没有把每个中位数差异都当作统计显著结果。

## 候选与查表成本

110 项诊断查询的新旧输出全部一致。候选数有 82 项不变、26 项增加、2 项减少。固定字面量优先和字面量组去重会放宽部分过滤，不能延用上一轮“候选数均未增加”的结论。

下表为 vLLM 查询，箭头为修正前 → 修正后。过滤阶段时间为 11 次插桩采样的中位数。

| 查询 | 不同 key 数 | 候选文件 | 解码后的 ID 数 | 过滤阶段（ms） |
|---|---:|---:|---:|---:|
| 装饰器 decorator | 122 → 3 | 1,348 → 1,348 | 60,326 → 4,372 | 3.885 → 0.230 |
| CUDA 调用 cuda_call | 31 → 4 | 306 → 306 | 15,968 → 12,395 | 0.876 → 0.504 |
| 环境变量 env_vars | 80 → 5 | 1,218 → 1,219 | 13,885 → 8,823 | 0.942 → 0.445 |
| forward 定义 def_forward | 76 → 2 | 1,075 → 1,173 | 15,099 → 6,001 | 1.176 → 0.366 |
| 张量创建 tensor_creation | 20 → 14 | 1,363 → 2,096 | 30,592 → 24,719 | 2.511 → 1.620 |
| 引号内设备名 quoted_device | 18 → 5 | 677 → 2,264 | 9,836 → 6,983 | 0.544 → 0.356 |

装饰器查询为 `^[ \t]*@(?:torch|pytest)\.[A-Za-z_]+`。回归测试确认其固定字面量仅为 `@torch.` 与 `@pytest.`；覆盖选键后实际读取 3 个不同 key。候选字节仍为 19,287,511；规划耗时从 106.67 降到 26.38 µs。过滤总时间下降约 94.1%，主要节省来自停止处理无效的字符类变体 postings。

另外两类查询说明了当前策略的代价：

- `quoted_device` 是 `[\"'](?:cuda|cpu|rocm)[\"']`。第一遍停止展开两种引号，保留设备名字面量，候选字节从 20,671,769 增到 47,810,798。少量引号分支原本有较好的过滤效果。
- `tensor_creation` 是 `\b(torch|numpy)\.(empty|zeros|ones)\b`，不含待展开的字符类。这里的差异来自删除字面量层面被包含的条件及相应的选键变化：字面量组之间的包含关系，不保证经过覆盖预算和分支保守近似后的过滤强度也相同。候选字节从 38,449,672 增到 53,635,685。

`decoded_ids` 是 `DiskIndex::postings` 返回的有效文档 ID 数，不是压缩字节读取量。集合与缓存计时包含预算检查和缓存管理；全部字段定义沿用[覆盖设计文档](../../docs/query-covering.md)。

## 复现

语料路径及固定提交见[plan.json](data/fixed-literal-extraction-2026-09-08/plan.json)。诊断脚本新增 `--baseline-source`，支持与尚未提交的冻结源码快照比较。

```sh
python3 benches/query_cover_metrics.py \
  --baseline-source /private/tmp/coderg-fixed-literals-wshmv6t2/before-source \
  --plan /private/tmp/coderg-fixed-literals-wshmv6t2/plan.json \
  --output /private/tmp/coderg-fixed-literals-wshmv6t2/diagnostics --iterations 11

python3 benches/refresh_matrix.py \
  --plan /private/tmp/coderg-fixed-literals-wshmv6t2/plan.json \
  --baseline /private/tmp/coderg-fixed-literals-wshmv6t2/before \
  --binary target/release/coderg \
  --output /private/tmp/coderg-fixed-literals-wshmv6t2/matrix \
  --iterations 21 --warmup 3 --build-iterations 1 --rss-runs 0
```
