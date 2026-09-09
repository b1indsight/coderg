# vLLM 二进制 manifest — 2026-09-09

将活动 manifest 和 Git 树缓存从 pretty JSON 改为带 `CDRGMF01` 格式头的
bincode 2 小端定长编码，保留 JSON 读取兼容。文件遍历、刷新规则和索引段格式不变。

## 独立加载

Rust `manifest_load` bench 使用 `/private/tmp/coderg-vllm-v4-index/manifest.json`
中的同一组记录，分别生成 JSON 和二进制文件，先验证解码后的全部字段一致。
预热 10 轮、测量 100 轮，每轮交替先后顺序；使用热缓存，计时不包含进程启动
和解码对象的释放。文件大小采用十进制 MB。

| 项目 | JSON | 二进制 |
|---|---:|---:|
| 文件大小 | 2,640,013 B | 1,231,923 B |
| 读取中位数 | 0.073 ms | 0.040 ms |
| 解码中位数 | 2.630 ms | 0.286 ms |
| 读取＋解码中位数 | 2.703 ms | 0.327 ms |

加载耗时减少约 87.9%，文件缩小约 53.3%。各阶段中位数独立计算。

```sh
cargo bench --bench manifest_load -- \
  --path /private/tmp/coderg-vllm-v4-index/manifest.json \
  --iterations 100 --warmup 10 --output /tmp/manifest-load.json
```

[完整加载样本](vllm-manifest-load-2026-09-09.json)

## 完整搜索

vLLM 快照 `/private/tmp/coderg-vllm-bench.7cZ1iI/vllm`，6,835 个可搜索文件、
82.38 MiB。Rust `compare_rg`，固定字符串 `AsyncMock`，逐行输出、预热 3 次、
测量 30 次。两版均新建独立索引，搜索输出与 rg 的 87 行一致。
新旧版分别运行，未随机交错；这些是该单条查询的热缓存进程耗时。

| 中位数 | JSON 旧版 | 二进制新版 | 变化 |
|---|---:|---:|---:|
| coderg no-refresh | 6.300 ms | 3.514 ms | −44.2% |
| coderg 默认刷新 | 21.387 ms | 18.991 ms | −11.2% |
| rg | 61.947 ms | 62.221 ms | +0.4% |

```sh
cargo bench --bench compare_rg -- \
  --root /private/tmp/coderg-vllm-bench.7cZ1iI/vllm \
  --query AsyncMock --lines --iterations 30 --warmup 3 \
  --output /tmp/manifest-search.json
```

旧版通过 `CODERG_BIN=/private/tmp/coderg-before-manifest-binary-20260909` 选择
修改前保留的 release 二进制。
[旧版样本](vllm-json-manifest-search-2026-09-09.json) ·
[新版样本](vllm-binary-manifest-search-2026-09-09.json)

验证：60 项 Rust 测试通过，覆盖 JSON/二进制全部字段一致、u128 纳秒时间、
未知版本/截断/尾随数据、旧索引不改写读取、更新迁移、旧 Git 树缓存切换以及
既有搜索和构建回归。Python bench 的 manifest 读取适配已分别通过新旧二进制
烟测，格式及 diff 检查通过。
