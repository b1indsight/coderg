# rg 执行顺序复核

原始 100 步报告为上级目录的 `vllm-history.json`；`summary.json` 按
`(3 - step) % 4 + 1` 计算 rg 在四者轮换中的位置，并匹配相同提交。
这里只报告中位数、p95 和范围，不使用平均耗时。

`replay.json` 使用原 bench 和原三个二进制，参数与原测试的差异是：

```text
--end d87a440f88e28e5b37f9b1e22ce214d0426d5352
--commits 8 --iterations 1 --warmup 0 --query SamplingParams
```

workspace/output 指向 `.cache/2026-09-09/rg-order-diagnosis`。
每个版本从新窗口起点建索引；这会改变索引维护事件，因此这次实验仅用来
对照相同源码提交下的 rg 耗时。通过 75 次结果集合和退出码对照。

`repeat.py` / `repeat.json` 是在原工作区最终快照 `569adb5a` 上的重复对照，
每种条件 10 次：直接执行或由 `/usr/bin/time -l` 包装；前置操作分别为空、
main/current/no-sync 的普通查询、current 的 `--no-refresh` 查询。
每次 rg 输出的文件集合必须一致。脚本依赖原报告所记录的缓存工作区和
二进制仍存在，运行会覆盖同目录 `repeat.json`。

现有证据确认完整 bench 的执行顺序影响结果，尚未定位具体前置操作或
系统机制。不能将慢的一组作为统一基线，也不能将固定快照重复查询直接
替换提交后首次查询的测量。
