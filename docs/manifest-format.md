# Manifest 二进制格式

当前活动快照写入 `manifest.bin`，Git 树缓存写入 `manifests/<tree>.bin`。
它们使用同一编码：8 字节格式头 `CDRGMF01`，随后是 bincode 2 的 Serde
编码，采用小端序和定长整数。字段顺序由 [manifest.rs](../src/manifest.rs)
定义；变更字段顺序或类型时必须升级格式头。解码限制为 1 GiB，并拒绝未知
格式头、截断数据和尾随数据。

这是 manifest 的存储编码变更，索引逻辑版本仍为 4，lookup/postings 段格式
及刷新规则不变。写入先生成临时文件，刷新缓冲区后替换目标；成功发布二进制
文件后删除对应旧 JSON，避免留下过期的活动快照。

兼容读取旧 `manifest.json` 和 `manifests/<tree>.json`。只有对应 `.bin`
不存在时才尝试旧文件；存在但损坏的二进制文件不会回退到旧快照。正常搜索
保留已有的索引加载失败后重建行为，`--no-refresh` 则报告错误。

读取旧索引本身不会改写文件。要立即获得二进制加载收益，重新运行
`coderg index /path/to/repository`；否则在下一次需要写入 manifest 时升级。
升级后旧版 coderg 无法直接读取二进制 manifest，需要重建索引。

需要检查内容时，可以导出 JSON，索引本身不会被改写：

```sh
coderg stats /path/to/repository --json
```

`stats --json` 输出完整 manifest 记录，包含文档、文件元数据及段信息。
默认 `stats` 仍输出原有摘要。现有 Rust bench 和三个使用 manifest 的
Python bench 已适配新旧格式；历史实验目录内保存的脚本快照不变。

独立加载 benchmark 在同一组记录上比较 JSON 与二进制，每轮交替先后顺序，
分别记录读取、解析及合计耗时，反序列化对象的释放在计时外：

```sh
cargo bench --bench manifest_load -- \
  --path /path/to/index/manifest.bin --iterations 100 --warmup 10 \
  --output /tmp/manifest-load.json
```

也支持把旧 `manifest.json` 作为输入。测试使用热文件缓存，结果不包含进程
启动、索引段映射和工作区刷新。

[vLLM 实测](../benches/results/vllm-binary-manifest-2026-09-09.md)：读取加解析
中位数从 2.70 ms 降至 0.33 ms，单条 `AsyncMock` 查询的 no-refresh 进程耗时
从 6.30 ms 降至 3.51 ms。

[Chromium 实测](../benches/results/chromium-binary-manifest-2026-09-09.md)：同一组
50.6 万条记录的 manifest 从 207.88 MiB 缩至 110.23 MiB，读取加解析从
205.08 ms 降至 30.76 ms。新版 `MAX_FILE_SIZE` no-refresh 搜索为 59.96 ms。
