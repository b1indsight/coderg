# 分支覆盖与安全停止查表：2026-09-08 benchmark

原有 102 项查询的中位数总和：默认搜索下降 5.11%，no-refresh 下降 6.89%。vLLM 的 `tensor_creation` 默认延迟下降 15.4%，`quoted_device` 下降 4.1%，`wide_icase_class_attention` 下降 15.3%；`icase_literal` 增加 3.3%。

本轮保留每个字面量内部的 gram 求交关系，避免把不同 OR 分支的片段按序号混合；根据已缓存的 postings 和当前候选跳过无效查表。同时把整体字面量组合预算从 16 恢复为 128，单个字符类的初始展开上限仍为 16。

基线是上一轮 [16 / 16 小规模组合提取](small-variant-extraction-2026-09-08.md)，不是最初的单 gram 版本。基线二进制和源码在修改前对照上一轮 metadata 的 SHA-256 校验，保留了[基线 query.rs](data/branch-covering-2026-09-08/before-query.rs)、[基线 search.rs](data/branch-covering-2026-09-08/before-search.rs)和[源码校验表](data/branch-covering-2026-09-08/before-source-sha256.json)。

## 实现与正确性

- 查询计划是组间 AND、组内分支 OR、分支内 gram AND。选键后按实际 key 包含关系去掉 OR 中冗余的较严分支。
- 先为各组读取所有分支的首个 gram，建立完整初筛，再在当前候选内细化。候选已被其他分支接纳时不重复验证；分支清空时跳过后续键，整组已接纳全部当前候选时跳过其余分支。
- 分支优先级按新增 key 数、首个 posting 的文件数排序；分支内优先处理已缓存且文件数更少的 posting。未知 key 保持原覆盖顺序，未为估算成本额外读表。
- 实际不同 key 仍最多 128 个。初筛放不下所有分支时跳过整组；细化时放弃尚未读取的 AND 项，保留该分支已有候选。超预算不能删除 OR 分支。
- `limit_class` 首轮为 16，`limit_total` 两遍均为 128。小字符类可以组成超过 16 种字面量；装饰器的 53 字符后缀仍不展开。

33 个单元测试、3 个 CLI 测试、1 个 Git 增量测试通过；clippy、格式和 diff 检查通过。新增测试穷举四个 key 的 16 种出现组合，检查 `(A ∩ B) ∪ (C ∩ D)`，并验证安全提前停止和细化预算耗尽。索引格式、gram 生成与覆盖片段规则、权重和刷新逻辑沿用基线，现有索引可直接使用。

完整 benchmark 的 110/110 项新旧输出一致，108 项与 rg 一致；`async_def` 与 `blank_lines` 保留基线已知的语义差异。8 组语料的新旧构建生成相同的索引段字节；各组结束后的语料、索引、二进制和源码校验全部通过。

## 整组结果

以下是每组各查询中位数之和，单位 ms，不代表一次搜索耗时。原有 102 项中，默认搜索有 56 项中位数下降，no-refresh 有 60 项；这不是统计显著性计数。全部查询见[per-query.csv](data/branch-covering-2026-09-08/per-query.csv)。

| 语料 | 查询数 | 默认：优化前 → 优化后 | 变化 | no-refresh：优化前 → 优化后 | 变化 |
|---|---:|---:|---:|---:|---:|
| vLLM | 38 | 1815.31 → 1773.97 | −2.28% | 1228.52 → 1191.90 | −2.98% |
| viberwhisper | 16 | 106.96 → 107.78 | +0.77% | 83.87 → 84.06 | +0.22% |
| agentflow | 16 | 110.51 → 110.13 | −0.34% | 89.42 → 88.83 | −0.66% |
| 256 小文件 | 8 | 54.59 → 53.58 | −1.85% | 44.06 → 43.11 | −2.15% |
| 4096 常规文件 | 8 | 248.28 → 227.97 | −8.18% | 183.99 → 163.39 | −11.20% |
| 16384 小文件 | 8 | 857.62 → 754.67 | −12.00% | 702.10 → 596.73 | −15.01% |
| 64 大文件 | 8 | 81.23 → 79.02 | −2.72% | 73.27 → 71.58 | −2.31% |
| 原有矩阵合计 | 102 | 3274.51 → 3107.12 | −5.11% | 2405.24 → 2239.60 | −6.89% |
| 额外共享片段语料 | 8 | 70.78 → 70.44 | −0.49% | 57.63 → 57.02 | −1.05% |

收益主要来自部分多分支正则和合成规模语料。固定字符串的 gram 选择不变，共享最长片段语料主要用于确认原有覆盖能力；小仓库与小幅计时差异需要结合单机波动解读。

## 代表性查询的完整延迟

以下为 vLLM 查询中位数，单位 ms。正数表示耗时增加；完整样本和 p95 见[vLLM 原始结果](data/branch-covering-2026-09-08/vllm/results.json)。

| 查询 | 默认：优化前 → 优化后 | 变化 | no-refresh：优化前 → 优化后 | 变化 |
|---|---:|---:|---:|---:|
| decorator | 38.21 → 38.00 | −0.5% | 22.84 → 22.75 | −0.4% |
| tensor_creation | 43.37 → 36.67 | −15.4% | 28.64 → 21.44 | −25.1% |
| quoted_device | 32.09 → 30.77 | −4.1% | 17.20 → 15.56 | −9.6% |
| icase_literal | 32.31 → 33.39 | +3.3% | 17.51 → 18.02 | +2.9% |
| wide_icase_asyncmock | 24.98 → 25.18 | +0.8% | 10.12 → 10.24 | +1.2% |
| wide_icase_class_attention | 41.70 → 35.34 | −15.3% | 26.06 → 19.77 | −24.2% |
| wide_icase_config | 43.78 → 35.60 | −18.7% | 28.37 → 20.74 | −26.9% |

`tensor_creation` 默认 p95 从 45.62 降到 38.92 ms，no-refresh p95 从 30.16 降到 22.19 ms。装饰器基本持平，`icase_literal` 仍有小幅退化；不能把全部中位数差异都视为统计显著变化。

## 诊断结果

临时插桩程序使用同一份单基础段索引，每项预热 1 轮、随机交错采样 11 轮。110 项查询新旧输出全部一致；25 项候选减少、84 项不变、1 项增加。原始计数和时间样本见[metrics.json](data/branch-covering-2026-09-08/metrics.json)。

以下为 vLLM 查询，时间是插桩阶段中位数，不能代替完整 CLI 延迟。

| 查询 | 不同 key | 候选文件 | 候选字节 | 解码后 ID 数 | 过滤阶段（ms） |
|---|---:|---:|---:|---:|---:|
| decorator | 3 → 3 | 1,348 → 1,348 | 19,287,511 → 19,287,511 | 4,372 → 4,372 | 0.232 → 0.263 |
| tensor_creation | 14 → 14 | 2,096 → 1,211 | 53,635,685 → 26,029,399 | 24,719 → 24,719 | 1.610 → 1.052 |
| quoted_device | 14 → 14 | 677 → 667 | 20,671,769 → 13,025,959 | 4,340 → 4,340 | 0.279 → 0.292 |
| wide_icase_class_attention | 78 → 122 | 1,851 → 942 | 52,019,683 → 27,757,912 | 50,299 → 42,465 | 2.652 → 2.872 |
| wide_icase_config | 10 → 44 | 2,378 → 1,022 | 54,725,910 → 30,222,167 | 6,055 → 21,803 | 0.327 → 1.682 |
| wide_icase_asyncmock | 71 → 121 | 25 → 25 | 612,206 → 612,206 | 9,041 → 7,436 | 0.630 → 0.554 |
| icase_literal | 128 → 128 | 390 → 554 | 23,450,702 → 26,906,016 | 72,918 → 44,498 | 3.457 → 3.086 |

`tensor_creation` 和 `quoted_device` 在 key 数及解码后 ID 总数不变的情况下减少了候选，说明分支求交关系会影响过滤强度。`decoded_ids` 是 `DiskIndex::postings` 返回的 ID 数，不是读取的压缩字节数。

本轮同时恢复了组合预算并改变计划与执行顺序，大小写查询的变化不能全部归因于分支逻辑。只有 `icase_literal` 相对 16 / 16 基线候选增加，且它用满了 128 个 key；下方对照将组合预算影响与分支处理分开。当前优化也未消除字面量层面包含剔除带来的选键取舍，没有对未知 key 做解码前的全局成本预测。

## 区分预算恢复与分支优化

额外构建仅恢复 `limit_total(128)` 的对照：从冻结基线中只修改这一行，其余源文件均不变，见[对照 query.rs](data/branch-covering-2026-09-08/budget-only-query.rs)。在同一份 vLLM 索引上与最终版本比较 38 项查询，预热 1 轮、记录 1 轮诊断；此处只解释确定性的 key、候选和 ID 计数，不用单轮耗时推断性能。38 项输出全部一致，分支优化使 19 项候选减少、19 项不变、0 项增加。[对照数据](data/branch-covering-2026-09-08/budget-control.json)

| 查询 | 16 / 16 基线候选 | 仅恢复组合预算候选 | 最终候选 | 仅恢复预算 → 最终的 key 数 |
|---|---:|---:|---:|---:|
| tensor_creation | 2,096 | 2,096 | 1,211 | 14 → 14 |
| quoted_device | 677 | 677 | 667 | 14 → 14 |
| wide_icase_class_attention | 1,851 | 1,105 | 942 | 128 → 122 |
| wide_icase_config | 2,378 | 1,951 | 1,022 | 55 → 44 |
| wide_icase_asyncmock | 25 | 60 | 25 | 128 → 121 |
| icase_literal | 390 | 574 | 554 | 128 → 128 |

`icase_literal` 的候选增加主要发生在恢复组合预算时；分支优化将该对照的 574 个候选进一步减到 554，但没有恢复到 16 / 16 版本的 390。这说明更大的展开预算也不保证有限 key 预算下的最优选键。

```sh
python3 benches/query_cover_metrics.py \
  --baseline-source /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/budget-only-source \
  --plan /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/vllm-plan.json \
  --output /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/budget-control --iterations 1
```

## 方法与复现

语料沿用[plan.json](data/branch-covering-2026-09-08/plan.json)：3 个真实仓库、4 组合成规模语料及 1 组共享最长片段语料，共 110 项查询。完整 CLI 计时采用 release、热缓存、新进程、默认线程策略，每项 3 次预热和 21 次随机交错计时；输出在计时外校验，新旧版本共用基线索引，另测两者 no-refresh 和 rg。机器信息及源码、二进制 SHA-256 见[metadata](data/branch-covering-2026-09-08/metadata.json)；证据文件的校验表见[manifest](data/branch-covering-2026-09-08/manifest.json)。

```sh
python3 benches/query_cover_metrics.py \
  --baseline-source /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/before-source \
  --plan /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/plan.json \
  --output /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/diagnostics --iterations 11

python3 benches/refresh_matrix.py \
  --plan /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/plan.json \
  --baseline /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/before \
  --binary target/release/coderg \
  --output /var/folders/2z/493g7vw10fs5kz1h1b9txy6c0000gn/T/coderg-branch-cover-di5qftwj/matrix \
  --iterations 21 --warmup 3 --build-iterations 1 --rss-runs 0
```
