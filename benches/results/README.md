# Benchmark 结果索引

截至 2026-09-10，主分支采用固定英文字母频率权重 v5，维护策略沿用 `413d14a` 的分代更新实现。
规则以[最终设计](../../docs/generational-index-refresh.md)为准：B < 32 MiB 追加增量、
累计 8 MiB 重建；大库采用 25% 分代合并。不同报告的“current”指各自记录的二进制。

## 当前实现的直接证据

默认权重的初测见[简单字母频率权重](letter-frequency-2026-09-10.md)：
70 项查询结果一致，整组搜索基本持平，索引体积增加 7.8%–16.8%。
最新结果与采用英文表的取舍见[权重设计](../../docs/letter-frequency-weights.md#最新-benchmark-与默认方案)。
后续[Chromium 全部字母 / 首字母分布实验](chromium-letter-distributions-2026-09-10.md)
覆盖四种权重及四份语料，共 76 项查询一致；两份 Chromium 表均未显示相对英文表的稳定整体优势。
再后续的[Chromium 实际字母对实验](chromium-letter-pairs-2026-09-10.md)
使用 676 种有向字母对频率：相对 Chromium 单字母乘积索引缩小 2.23%–6.08%，
76 项查询一致，整体搜索未显示稳定改善，个别查询收益和回退明显。
[查询收益占比与类型分布](pair-query-benefits-2026-09-10.md) 对同一批样本进一步分析：
相对原始哈希 35/76 项中位数更快，但仅 2/76 项达到“至少快 5% 且逐项区间支持”的筛选条件。
另有 [vLLM 标识符、单词与 OR 扩展实验](identifier-word-or-2026-09-10.md)：
85 项区分大小写查询，每项 31 轮；实际字母对相对原始哈希有 9 项明显收益、2 项明显回退，
四类平均耗时仅降低 0.52%–1.45%，相对英文表未显示整体优势。

| 报告 | 覆盖范围 | 使用方式 |
|---|---|---|
| [大、中、小三档](size-tiers-2026-09-10.md) | viberwhisper、whisper.cpp、vLLM，各 100 次真实提交及 4 次历史切换 | 最新维护与查询基线；1464 项历史结果对照 |
| [8 MiB 初测](small-8m-2026-09-10.md) | 小库与原版/逐次直接合并版对照；大库 12 次真实提交 | 同一实现的补充；不能代替三档报告里的大合并样本 |

主表只使用中位数、P95 或单次事件，普通增量、M 合并、B+M 合并和源码重建分开。
B/M 的前进终点、峰值和独立查询状态分别标注；小库累计增量与大库清理后的 M 不能混为源码 diff。
对照版本维护时点不同，普通更新性能按相同提交配对；额外回退不混入前进样本。

真实历史由 [history_updates](../history_updates.rs) 重放；`commit_updates` 及生成语料
`compare_rg` 创建模拟更新。没有 Git 历史的 Chromium 源码压缩包只用于构建/查询剖析。
rg 与 coderg 对齐隐藏文件范围，索引放在源码外；rg 独立查询基线与逐提交循环计时分开，
避免[已观察到的顺序偏差](data/maintenance-rg-2026-09-09/rg-order-diagnosis/README.md)。

## 维护策略的历史试验

| 报告 | 状态 |
|---|---|
| [vLLM 早期两代更新](vllm-generational-updates-2026-09-09.md) | 模拟提交，仅保留初步观察 |
| [500 次真实 vLLM 更新](vllm-real-commits-2026-09-09.md) | 25%/50% 阈值选择依据，最终采用 25% |
| [小库全分代 25%](small-auto25-2026-09-09.md) | 已替代，小库现在不采用分代合并 |
| [同步开销定位](small-sync-profile-2026-09-09.md) | 诊断版不直接等于当前发布策略 |
| [早期规模估计](generation-scale-2026-09-09.md) | 保留旧平均值和外推，最新三档已补中间规模 |
| [维护事件与 rg](maintenance-rg-2026-09-09.md) | 旧版分组结果及 rg 顺序诊断 |
| [小库单段](small-single-2026-09-09.md) | 已取消逐次合并和无变化迁移 |
| [小库直接合并](small-direct-2026-09-10.md) | 已移除直接归并路径，保留小库缓存发布语义 |

## 分支相关的定位记录

| 报告 | 当前解释 |
|---|---|
| [vLLM 批量元数据](vllm-bulk-metadata-2026-09-09.md) | 原型变慢，未保留 |
| [vLLM 查询剖析](vllm-no-refresh-profile-2026-09-09.md)、[小库查询剖析](small-no-refresh-profile-2026-09-09.md) | 对应当时版本的热点定位 |
| [Chromium 搜索](chromium-max-file-size-2026-09-09.md)、[阶段剖析](chromium-search-profile-2026-09-09.md) | JSON 基线，未修改产品实现 |
| [vLLM 二进制 manifest](vllm-binary-manifest-2026-09-09.md)、[Chromium 二进制 manifest](chromium-binary-manifest-2026-09-09.md) | 已在 main 基线实现，不是本分支新增格式优化 |

更早的构建和查询算法实验见[已实现的优化总览](../../docs/implemented-optimizations.md)
与[构建演进](../../docs/index-build-evolution.md)。原始 JSON、CSV、源码归档和脚本按原报告
保留，不把旧样本改写为最终版本的测量结果。复测必须记录实际二进制 SHA、提交范围、
查询/输出模式、索引状态和计时顺序。
