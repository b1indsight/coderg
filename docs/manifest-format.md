# Manifest 二进制格式

字母频率权重更新将索引逻辑升为 v5，需要重建 v4 索引；
`CDRGMF01` 封装和段字节格式不变。下文 v4 兼容性描述属于更新前基线。
见[字母频率权重](letter-frequency-weights.md)。

截至 2026-09-10，当前活动快照写入 `manifest.bin`，编码为 8 字节格式头
`CDRGMF01` 加 bincode 2 的 Serde 编码，采用小端序和定长整数。
字段顺序由 [manifest.rs](../src/manifest.rs) 定义；变更字段顺序或类型时必须升级
格式头。解码限制为 1 GiB，并拒绝未知格式头、截断数据和尾随数据。

该二进制格式已在 main `0ab2294` 基线实现。分代更新分支没有再次改变 schema；
索引逻辑版本和 lookup/postings 段格式仍为 v4。当前的大小阈值、回退和维护规则
见[索引更新设计](generational-index-refresh.md)，不能从格式版本推断维护策略。

不再写入或切换 `manifests/<tree>.bin` / `.json` 历史树缓存，已有文件暂不回收。
活动旧 `manifest.json` 仍可读取：仅当 `manifest.bin` 不存在时尝试旧 JSON；
二进制存在但损坏时不会回退到过期 JSON。默认搜索在加载失败时尝试从源码重建，
`--no-refresh` 则报告错误；加载后读取 postings 的错误不保证自动修复。

读取本身不改写索引。显式 `index` 可立即重建，或等待下一次需要写入 manifest 时
升级。成功原子发布二进制后删除对应活动旧 JSON。旧到不支持二进制格式的 coderg
无法直接读取该快照；同为 `CDRGMF01`/v4 的 main 基线仍兼容当前编码。

构建、刷新和实际 compact 持有 OS `write.lock`。不可变段写完后，manifest 在
唯一临时文件中编码并通过 `NamedTempFile::persist` 原子替换。
小库增量、8 MiB 阈值重建和身份推进仅 flush 后替换，系统崩溃可能需要重建缓存；
小库重建跨过 32 MiB 时同步最终 B。大库更新、显式 index 和 compact 同步文件，
Unix 下也同步相关目录。不可变旧段暂留给已有读者，GC 尚未实现。

需要检查内容时，可以导出 JSON，索引本身不会被改写：

```sh
coderg stats /path/to/repository --json
```

`stats --json` 输出完整 manifest 记录，包含文档、文件元数据及段信息。
默认 `stats` 输出统计摘要，另含 B/M 大小、段数、`generational` 和维护提示；
小库 M 字段指累计增量。摘要与完整 manifest JSON 是不同接口。现有 Rust bench 和三个使用 manifest 的
Python bench 已适配新旧格式；历史实验目录内保存的脚本快照不变。

以下是二进制格式引入时的历史测量，不是分代更新分支的新增收益。
独立加载 benchmark 在同一组记录上比较 JSON 与二进制，每轮交替先后顺序，
分别记录读取、解析及合计耗时，反序列化对象的释放在计时外：

```sh
mkdir -p .cache/bench
cargo bench --bench manifest_load -- \
  --path /path/to/index/manifest.bin --iterations 100 --warmup 10 \
  --output .cache/bench/manifest-load.json
```

也支持把旧 `manifest.json` 作为输入。测试使用热文件缓存，结果不包含进程
启动、索引段映射和工作区刷新。

[vLLM 实测](../benches/results/vllm-binary-manifest-2026-09-09.md)：读取加解析
中位数从 2.70 ms 降至 0.33 ms，单条 `AsyncMock` 查询的 no-refresh 进程耗时
从 6.30 ms 降至 3.51 ms。

[Chromium 实测](../benches/results/chromium-binary-manifest-2026-09-09.md)：同一组
50.6 万条记录的 manifest 从 207.88 MiB 缩至 110.23 MiB，读取加解析从
205.08 ms 降至 30.76 ms。新版 `MAX_FILE_SIZE` no-refresh 搜索为 59.96 ms。
