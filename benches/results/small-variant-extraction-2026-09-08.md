# 小规模组合提取：2026-09-08 实验

版本说明：本报告对应 16 / 16 的中间实现。后续已恢复 128 的整体组合预算，并保留分支内部的覆盖关系，见[分支覆盖实验](branch-covering-2026-09-08.md)。

`quoted_device` 默认延迟下降 25.0%，装饰器基本持平；但 `wide_icase_class_attention` 默认延迟增加 18.3%。原有 102 项查询的中位数总和基本持平：默认 −0.69%，no-refresh +0.17%。这轮修复了少量引号选项的过滤退化，没有显示整体加速。

本轮把字符类和显式字面量分支按同一个组合预算处理。第一遍 `limit_class` 和 `limit_total` 均设为 16；没有完整可用条件时，再以 128 / 128 重试。实际索引查询仍最多读取 128 个不同 key。16 是初始试验值，用于容纳 12 种带引号设备名组合，同时避免装饰器的 106 种名称前缀组合；尚未通过多档预算实验确定最优值。

基线是上一轮“第一遍禁止字符类展开”的实现，不是最初的单 gram 版本。保留了[基线 query.rs](data/small-variant-extraction-2026-09-08/before-query.rs)和[源码校验表](data/small-variant-extraction-2026-09-08/before-source-sha256.json)。基线二进制 SHA-256 在开始前对照上一轮 metadata 校验通过。

## 规则与正确性

- `["'](?:cuda|cpu|rocm)["']` 的 2 × 3 × 2 共 12 种字面量组合全部保留，包括正则允许的混合引号。
- `^[ \t]*@(?:torch|pytest)\.[A-Za-z_]+` 仍保留 `@torch.` / `@pytest.` 两个字面量，名称字符类有 53 种取值，不进入初始展开。
- 类与字面量分支的组合也受同一个上限约束。测试验证 4 × 4 = 16 时可保留组合，5 × 4 = 20 时回退为较短的必要前缀，所有可选分支的匹配仍保留。
- 缺少可用初始条件时继续使用 128 的受限回退；短字面量、可选分支和 Unicode 大小写匹配的安全回退测试保持有效。

30 个单元测试、3 个 CLI 测试、1 个 Git 增量测试通过；clippy、格式和 diff 检查通过。没有改变索引格式、权重、gram 生成、刷新或 postings 执行逻辑；字面量组去重与覆盖的保守分支近似也沿用上一版。

完整 benchmark 的 110/110 项新旧输出一致；108 项与 rg 一致，`async_def` 和 `blank_lines` 保留基线已知的语义差异。8 组语料的新旧构建生成相同的索引段字节；每组结束后的语料、索引、二进制和源码校验均通过。

## 完整搜索延迟

以下为 vLLM 查询的中位数，单位 ms，变化为新版本相对上一轮固定字面量优先实现。正数表示耗时增加。原始样本及 p95 见[vLLM 计时数据](data/small-variant-extraction-2026-09-08/vllm/results.json)。

| 查询 | 默认：修正前 → 修正后 | 变化 | no-refresh：修正前 → 修正后 | 变化 |
|---|---:|---:|---:|---:|
| quoted_device | 42.84 → 32.13 | −25.0% | 27.16 → 17.74 | −34.7% |
| decorator | 38.16 → 38.34 | +0.5% | 22.86 → 22.69 | −0.7% |
| tensor_creation | 44.40 → 43.90 | −1.1% | 28.52 → 28.48 | −0.2% |
| icase_literal | 32.56 → 33.32 | +2.3% | 17.07 → 18.00 | +5.5% |
| wide_icase_asyncmock | 25.94 → 25.76 | −0.7% | 10.47 → 10.08 | −3.7% |
| wide_icase_class_attention | 34.88 → 41.27 | +18.3% | 20.32 → 26.26 | +29.2% |
| wide_icase_config | 42.38 → 42.87 | +1.2% | 26.49 → 28.88 | +9.0% |

`quoted_device` 默认 p95 从 45.10 降到 35.31 ms，no-refresh p95 从 28.94 降到 19.48 ms；rg 中位数为 69.83 ms。`wide_icase_class_attention` 默认 p95 从 37.04 增到 42.92 ms，退化也出现在 no-refresh 中。装饰器的计划和候选保持相同，小幅耗时变化不作为改善或退化的证据。`tensor_creation` 的计划也未变，上一轮字面量组去重造成的候选增加仍存在。

## 整组结果

以下为每组各查询中位数之和，单位 ms，不代表一次搜索耗时。原有 102 项中，默认搜索有 58 项中位数下降，no-refresh 有 62 项；这不是统计显著性计数。全部查询汇总见[per-query.csv](data/small-variant-extraction-2026-09-08/per-query.csv)。

| 语料 | 查询数 | 默认：修正前 → 修正后 | 变化 | no-refresh：修正前 → 修正后 | 变化 |
|---|---:|---:|---:|---:|---:|
| vLLM | 38 | 1826.83 → 1815.38 | −0.63% | 1240.58 → 1250.56 | +0.80% |
| viberwhisper | 16 | 106.03 → 104.81 | −1.15% | 82.57 → 81.06 | −1.82% |
| agentflow | 16 | 110.06 → 109.71 | −0.31% | 90.36 → 89.34 | −1.13% |
| 256 小文件 | 8 | 54.99 → 54.60 | −0.72% | 44.34 → 44.02 | −0.71% |
| 4096 常规文件 | 8 | 249.60 → 249.34 | −0.10% | 184.88 → 184.64 | −0.13% |
| 16384 小文件 | 8 | 862.67 → 853.73 | −1.04% | 699.21 → 696.48 | −0.39% |
| 64 大文件 | 8 | 87.54 → 87.35 | −0.22% | 79.54 → 79.40 | −0.18% |
| 原有矩阵合计 | 102 | 3297.72 → 3274.92 | −0.69% | 2421.48 → 2425.50 | +0.17% |
| 额外共享片段语料 | 8 | 75.05 → 74.35 | −0.93% | 60.83 → 60.77 | −0.11% |

固定字符串路径未变，共享片段语料在本轮主要验证已有覆盖能力。整体小幅波动不能证明普遍加速；当前取舍集中在少量正则的展开和选键上。

## 诊断结果

临时插桩程序在同一份单基础段索引上比较修正前后，每项预热 1 轮、随机交错采样 11 轮。110 项查询输出全部一致；85 项候选不变、19 项减少、6 项增加。原始数据见[metrics.json](data/small-variant-extraction-2026-09-08/metrics.json)。

以下为 vLLM 查询；时间是诊断阶段中位数，不能代替未插桩 CLI 的完整耗时。

| 查询 | 不同 key | 候选文件 | 候选字节 | 过滤阶段（ms） |
|---|---:|---:|---:|---:|
| quoted_device | 5 → 14 | 2,264 → 677 | 47,810,798 → 20,671,769 | 0.348 → 0.280 |
| decorator | 3 → 3 | 1,348 → 1,348 | 19,287,511 → 19,287,511 | 0.231 → 0.229 |
| class_attention | 6 → 8 | 637 → 623 | 20,996,163 → 20,852,657 | 0.821 → 0.990 |
| wide_icase_asyncmock | 128 → 71 | 60 → 25 | 2,491,629 → 612,206 | 0.410 → 0.633 |
| wide_icase_class_attention | 128 → 78 | 1,105 → 1,851 | 31,471,050 → 52,019,683 | 2.095 → 2.676 |
| wide_icase_config | 55 → 10 | 1,951 → 2,378 | 50,290,685 → 54,725,910 | 1.426 → 0.335 |

引号查询的解码后 ID 数从 6,983 降到 4,340。虽然 key 增加，但新片段更有选择性，解码规模和候选读取量同时下降。装饰器仍只读取 3 个 key，未重新引入大范围后缀展开。

初始预算同时作用于大小写折叠后的分支组合，因此也会改变这些查询的片段长度和预算分配。首轮已得到可用条件时，不会再以 128 重试；这解释了部分忽略大小写查询候选增加的可能路径。当前预算只限制组合数，不使用语料频率或实际 posting 长度预测收益。

## 方法与复现

语料及固定提交沿用[plan.json](data/small-variant-extraction-2026-09-08/plan.json)：3 个真实仓库、4 组合成规模语料和 1 组共享最长片段语料，共 110 项查询。环境为 Apple M5、24 GiB、macOS 26.6.2、Rust 1.94.0，详情及源码与二进制 SHA-256 见[metadata](data/small-variant-extraction-2026-09-08/metadata.json)。完整计时采用 release、热缓存、新 CLI 进程、默认线程策略，每项 3 次预热和 21 次随机交错计时；输出在计时外校验，修正前后默认搜索与 no-refresh 共用基线索引，另测 rg。证据文件校验表见[manifest.json](data/small-variant-extraction-2026-09-08/manifest.json)。

```sh
python3 benches/query_cover_metrics.py \
  --baseline-source /private/tmp/coderg-small-variants-c_r0i1kz/before-source \
  --plan /private/tmp/coderg-small-variants-c_r0i1kz/plan.json \
  --output /private/tmp/coderg-small-variants-c_r0i1kz/diagnostics --iterations 11

python3 benches/refresh_matrix.py \
  --plan /private/tmp/coderg-small-variants-c_r0i1kz/plan.json \
  --baseline /private/tmp/coderg-small-variants-c_r0i1kz/before \
  --binary target/release/coderg \
  --output /private/tmp/coderg-small-variants-c_r0i1kz/matrix \
  --iterations 21 --warmup 3 --build-iterations 1 --rss-runs 0
```
