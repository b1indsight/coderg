# 两个真实仓库并发验证 — 2026-09-23

使用 main `4cad6faa38e4ff8563040e709e0b57d5bdeaabfc`，默认线程与预算，`--jobs 2`。viberwhisper 与 agentflow 同时执行全部六个场景，状态 **complete**。

运行耗时 130.11 秒（不含 Cargo 编译），记录 15,554 个样本、9,680 次成功 rg 输出比较。

本轮使用调整后的 full 采样参数：预热 2 次、查询计时 16 次、构建/工作流 3 轮、历史与 RSS 各 2 轮。真实历史更新数和查询集未缩减：viberwhisper 100 次，agentflow 56 次。仅验证这两库，未重新运行其余三库。

| 仓库 | 初始查询相对 rg 加速比 | 构建中位数 ms | 正确性比较 |
|---|---:|---:|---:|
| viberwhisper | 1.19× | 57.26 | 5,544 |
| agentflow | 1.23× | 64.75 | 4,136 |

加速比为同轮各查询 `rg 中位数 / coderg 中位数` 的几何平均。两库共享 CPU、磁盘和缓存；不可与先前单库独占测量直接比较，也不能据此推算五库长测加速比例。需要独占测量时使用 `--jobs 1`。

8 项 harness 回归测试、格式检查及 all-targets clippy（`-D warnings`）通过。新增回归覆盖合并不同 worker 的记录时保留重复样本和失败前结果。

[逐场景报告](data/two-real-parallel-2026-09-23/report.md)、[运行配置与来源](data/two-real-parallel-2026-09-23/run.json)、[统计 JSON](data/two-real-parallel-2026-09-23/summary.json)。原始 [samples](data/two-real-parallel-2026-09-23/samples.jsonl.gz) 与 [events](data/two-real-parallel-2026-09-23/events.jsonl.gz) 以 gzip 提交，解压到同一目录后可用 `--render` 再生成报告。
