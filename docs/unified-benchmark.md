# 统一 Rust benchmark

当前入口是 `cargo bench`，使用方式、语料配置和测量口径见 [benchmark 指南](../benches/README.md)。

- 默认测试 viberwhisper、agentflow、whisper.cpp、vLLM、Chromium，固定语料版本；Chromium 使用模拟提交。
- 仅测当前 Cargo 构建的默认实现，保留 rg 性能对照和完整输出校验；不扫描线程或内存预算。
- 覆盖构建、日常查询、工作区变更、分支切换、历史回放和生产 manifest 加载。
- 查询预热 2 次、计时 16 次；构建和工作流 3 轮；历史与独立 RSS 采样各 2 轮。
- 默认每批最多两个仓库并发，仓库内顺序执行；`--jobs 1` 用于独占测量。并发数写入报告，资源争用会影响结果。
- 语料副本和索引互相隔离；校验失败保留已采样记录，整轮标记为未完成，其他仓库继续执行。
- 结果默认写入 `target/benchmarks/`，不提交生成产物。旧脚本、历史结果与实验代码已删除，需要追溯时查看 Git 历史。

配置文件：[完整语料](../benches/full.json)、[快速验证](../benches/smoke.json)、[本机路径示例](../benches/sources.example.json)。
