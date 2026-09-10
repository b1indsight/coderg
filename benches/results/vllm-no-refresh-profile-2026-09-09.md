# vLLM no-refresh 快速验证

> 文档状态（2026-09-10）：历史启动/查询剖析，测量对应下文记录的版本，不等同于当前 release。最终三档维护和查询对照见[最新实测](size-tiers-2026-09-10.md)。

复用 [小仓库测试](small-no-refresh-profile-2026-09-09.md) 的方法和 Chromium 插桩二进制，在既有 vLLM 快照 `/private/tmp/coderg-vllm-bench.7cZ1iI/vllm` 上新建独立临时索引。查询 `^class`，预热 5 轮，随机交替测量 release、插桩版和 rg 各 31 次。输出重定向到 `/dev/null`，计时包含独立进程启动；未清空文件缓存。计时外验证三种模式结果完全一致。

manifest 为 2,640,013 字节（2.52 MiB）。候选 2770 个文件，最终命中 2334 个文件、7601 行。

| 阶段 | 中位数 ms |
|---|---:|
| release no-refresh 完整进程 | 30.750 |
| 插桩 no-refresh 完整进程 | 31.005 |
| rg 完整进程 | 77.505 |
| manifest 读取 | 0.208 |
| JSON 反序列化 | 2.977 |
| 索引加载合计 | 3.249 |
| 候选筛选 | 0.455 |
| 读取候选文件并匹配 | 20.095 |
| 结果排序及输出 | 2.908 |
| 释放索引及结果 | 0.290 |

读取与解析 manifest 共约 3.18 ms，占插桩总耗时约 10.3%，有一定成本，但不是此次查询的主要瓶颈。读取与匹配 2770 个候选文件占约 65%。release no-refresh 比 rg 约快 2.52 倍。

这只代表 `^class` 这项命中较广的查询。选择性更高的查询匹配成本会变小，此时索引加载占比可能更高；本次未测量该情形。每项独立取中位数，分项不能精确相加得到总值。

[测量脚本](data/chromium-max-file-size-2026-09-09/vllm-profile.py) · [完整样本及命令](data/chromium-max-file-size-2026-09-09/vllm-profile-results.json)
