# Chromium 二进制 manifest — 2026-09-09

二进制 manifest 将 Chromium 的读取加解析耗时降低约 **85.0%**。
完整 `MAX_FILE_SIZE` 搜索中，no-refresh 进程耗时减少 **74.7%**，
默认搜索减少 **10.3%**；全目录刷新仍是默认搜索的主要成本。

## 语料与方法

- Chromium 官方主仓库冻结快照 `398630472335c10b9ca610a4d1b7888a040f702a`，
  路径 `/private/tmp/coderg-chromium-3986304/src`。未同步 DEPS 依赖，无 Git 元数据。
- manifest 包含 506,050 条文件状态及文档记录，其中 461,882 个文件可搜索，
  文本源码约 2.83 GiB。macOS arm64；[二进制哈希及运行元数据](chromium-manifest-run-metadata-2026-09-09.json)。
- 加载测试使用 Rust `manifest_load`，从已有 JSON 中读取同一组记录，生成两种
  编码并验证全部字段一致。预热 10 轮、测量 100 轮，每轮交替先后顺序。
- 搜索测试使用 Rust `compare_rg`，固定字符串 `MAX_FILE_SIZE`、逐行输出，
  预热 3 次、测量 15 次。每版本新建独立临时索引，分别测量 default、
  no-refresh 和 `rg --hidden --no-config`。新版和旧版顺序运行，没有随机交错。
- 使用热缓存，计时期间没有并发编译或其他由本任务启动的 benchmark。
  加载测试不计进程启动及对象释放；完整搜索包含进程启动和退出。

## Manifest 加载

| 项目 | JSON | 二进制 |
|---|---:|---:|
| 文件大小 | 217,973,589 B（207.88 MiB） | 115,585,731 B（110.23 MiB） |
| 读取中位数 | 13.356 ms | 6.840 ms |
| 解析中位数 | 191.533 ms | 23.863 ms |
| 读取＋解析中位数 | 205.076 ms | 30.765 ms |

各阶段中位数独立计算。文件缩小约 47.0%，读取加解析约快 6.7 倍。
[完整加载样本](chromium-manifest-load-2026-09-09.json)

## 完整搜索

| 中位数 | JSON 旧版 | 二进制新版 | 变化 |
|---|---:|---:|---:|
| coderg no-refresh | 237.222 ms | 59.958 ms | −74.7% |
| coderg 默认刷新 | 2073.775 ms | 1860.621 ms | −10.3% |
| rg | 8126.280 ms | 8047.440 ms | −1.0% |

两版均返回 114 行，与 rg 逐行一致。新版 no-refresh 的 p95 为 60.429 ms，
默认搜索 p95 为 1900.242 ms。新旧版初次建索引分别为 63.91 / 64.89 秒，
构建只测一次，不将该差异视为稳定收益。

新版默认与 no-refresh 仍相差约 1.80 秒。该差额与保留全目录元数据检查一致，
不是本轮新测的分阶段 refresh 耗时。这里的结果只代表这一条固定字符串查询，
且无 Git 元数据的快照不能完全代表正常 Git checkout。

[旧版搜索样本](chromium-json-manifest-search-2026-09-09.json) ·
[新版搜索样本](chromium-binary-manifest-search-2026-09-09.json)

## 复现

```sh
cargo bench --bench manifest_load -- \
  --path /private/tmp/coderg-chromium-3986304/index/manifest.json \
  --iterations 100 --warmup 10 --output /tmp/chromium-manifest-load.json

cargo bench --bench compare_rg -- \
  --root /private/tmp/coderg-chromium-3986304/src \
  --query MAX_FILE_SIZE --lines --iterations 15 --warmup 3 \
  --output /tmp/chromium-manifest-search.json
```

旧版搜索使用同一条命令，加上
`CODERG_BIN=/private/tmp/coderg-before-manifest-binary-20260909` 环境变量。
每次选择新的输出文件名。
