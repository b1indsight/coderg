# vLLM 常见正则 benchmark — 2026-09-07

对当前工作区的 release 构建运行了 30 项查询，其中 28 项与 `rg` 的完整匹配行输出及退出码一致。默认模式在 25/28 项更快，`--no-refresh` 在 26/28 项更快。28 项查询中位数的等权平均为 **48.79 / 31.79 / 71.92 ms**（默认 / no-refresh / rg），对应总体比值 **1.47× / 2.26×**。两项输出存在差异的查询不参与计时或性能汇总。

环境：Apple M5，10 个逻辑 CPU，24 GiB RAM，macOS-26.6.2-arm64-arm-64bit-Mach-O；rustc 1.94.0 (4a4ef493e 2026-03-02)；ripgrep 15.2.0 (rev e89fff89ac)。运行开始于 `2026-09-07T07:26:04.961712+00:00`（UTC）。

vLLM commit：`569adb5a9780f9c02d22a6b29826acf711512356`；干净工作区，6,835 个文本文件，86,384,180 bytes（82.38 MiB）。coderg 基础 commit：`bdedff2affe7901c566ded4e2551d64c0a87ab1e`，另包含运行前已有的工作区修改；[源码差异](data/vllm-regex-suite-2026-09-07/source.diff)、源码逐文件 SHA-256 和二进制 SHA-256 均已归档。使用 `cargo build --release --locked` 构建。

新索引构建耗时 **3.495 秒**（单次测量），大小 **63.88 MiB**，一个 segment，7,980,942 个 n-grams。建索引时间单独记录，未计入搜索耗时；这不是冷缓存或索引构建时间分布测试。

测试使用正常的“路径:行号:匹配行”输出，计时期间重定向到 `/dev/null`，包含进程启动、查询处理、读取和输出生成成本。每项预热 3 次，随后测量 31 次；每轮随机打乱查询和三种执行模式的顺序，种子为 20260907。文件系统缓存为热缓存。`rg` 保留默认并行遍历，不加排序选项；一致性校验在进程结束后仅去掉路径开头的 `./` 并排序完整输出，保留重复行。

`rg` 启用隐藏文件搜索，排除 `.git/**` 和 `.coderg-index/**`，关闭用户配置、颜色及分组输出；正常 ignore 规则仍生效。两种 coderg 模式共用新索引；默认模式包含 Git/文件新鲜度检查，`--no-refresh` 跳过该检查。没有修改线程数。完整参数见 [commands.json](data/vllm-regex-suite-2026-09-07/commands.json)。

下表列出全部通过一致性校验的查询，单位为 ms，数值为中位数。匹配行数来自 `rg`，不是匹配子串的数量。

| 查询 | flags | 匹配行 | coderg 默认 | no-refresh | rg | 默认加速比 | no-refresh 加速比 |
|---|---|---:|---:|---:|---:|---:|---:|
| `get_tensor_model_parallel_world_size` | -F | 580 | 30.66 | 13.58 | 66.10 | 2.16× | 4.87× |
| `\bSamplingParams\b` | — | 1,348 | 32.25 | 15.36 | 67.50 | 2.09× | 4.39× |
| `\bAsyncMock\b` | — | 87 | 29.52 | 12.01 | 64.88 | 2.20× | 5.40× |
| `def\s+forward\b` | — | 1,733 | 38.35 | 21.06 | 68.36 | 1.78× | 3.25× |
| `^class[ \t]+\w*Attention\b` | — | 220 | 36.05 | 19.52 | 66.89 | 1.86× | 3.43× |
| `^from[ \t]+vllm(?:\.[\w.]+)?[ \t]+import\b` | — | 18,746 | 65.69 | 48.74 | 89.35 | 1.36× | 1.83× |
| `^[ \t]*@(?:torch\|pytest)\.[A-Za-z_]+` | — | 8,968 | 42.92 | 25.99 | 73.73 | 1.72× | 2.84× |
| `logger\.(warning\|error)\(` | — | 831 | 32.35 | 15.62 | 67.00 | 2.07× | 4.29× |
| `\b(ValueError\|RuntimeError\|NotImplementedError)\b` | — | 6,725 | 43.31 | 26.33 | 75.35 | 1.74× | 2.86× |
| `\b(kv_cache\|block_size)\b` | — | 8,041 | 45.44 | 28.67 | 69.21 | 1.52× | 2.41× |
| `\b(torch\|numpy)\.(empty\|zeros\|ones)\b` | — | 5,133 | 42.66 | 26.16 | 72.23 | 1.69× | 2.76× |
| `\b(VLLM\|CUDA)_[A-Z0-9_]+\b` | — | 6,332 | 40.78 | 23.77 | 72.56 | 1.78× | 3.05× |
| `\btorch\.cuda\.[a-z_]+\(` | — | 611 | 30.83 | 14.20 | 66.65 | 2.16× | 4.69× |
| `\bmax_(model_len\|num_seqs)[ \t]*=` | — | 1,547 | 34.49 | 17.52 | 67.73 | 1.96× | 3.87× |
| `raise[ \t]+\w*Error\(` | — | 5,675 | 41.52 | 24.84 | 74.61 | 1.80× | 3.00× |
| `^[ \t]*#.*\b(TODO\|FIXME)\b` | — | 726 | 104.63 | 88.25 | 67.52 | 0.65× | 0.77× |
| `^[ \t]*return[ \t]+None[ \t]*$` | — | 1,742 | 56.56 | 39.39 | 69.08 | 1.22× | 1.75× |
| `[\"'](?:cuda\|cpu\|rocm)[\"']` | — | 2,916 | 35.63 | 18.58 | 68.89 | 1.93× | 3.71× |
| `\bSamplingParams\b` | -i | 1,348 | 34.20 | 18.00 | 67.48 | 1.97× | 3.75× |
| `\b(cuda\|rocm)\b` | -i | 9,805 | 46.86 | 30.18 | 74.80 | 1.60× | 2.48× |
| `\b(TODO\|FIXME)\b` | -i | 890 | 35.09 | 18.90 | 68.67 | 1.96× | 3.63× |
| `\b(VLLM\|CUDA)_[A-Z0-9_]+\b` | -i | 20,751 | 57.87 | 40.79 | 80.12 | 1.38× | 1.96× |
| `logger\.(warning\|error)\(` | -i | 831 | 36.46 | 19.78 | 67.61 | 1.85× | 3.42× |
| `get_tensor_model_parallel_world_size` | -i -F | 580 | 35.71 | 19.33 | 66.61 | 1.87× | 3.45× |
| `\b[0-9]{2,4}\b` | — | 106,132 | 168.97 | 145.46 | 95.71 | 0.57× | 0.66× |
| `\b0x[0-9A-Fa-f]+\b` | — | 1,867 | 33.64 | 16.97 | 66.74 | 1.98× | 3.93× |
| `\bif\b` | — | 68,895 | 105.46 | 89.53 | 92.81 | 0.88× | 1.04× |
| `\bCODERG_BENCH_ABSENT_[0-9]{8}\b` | — | 0 | 28.23 | 11.55 | 65.68 | 2.33× | 5.69× |

加速比定义为 `rg 中位数 / coderg 中位数`，大于 1 表示 coderg 更快。上文总体比值是“每项中位数的等权平均之比”，不是对逐项加速比直接求平均。查询集合含 1 项固定字符串对照、17 项大小写敏感的常见正则、6 项忽略大小写查询、3 项数字/十六进制/短词查询、1 项无结果查询。它是人工选择的功能覆盖集合，不能代表真实用户查询频率。

词边界、日志调用、CUDA 调用等查询收益明显。默认模式较慢的三项为 TODO 注释、纯数字、短词 `if`；no-refresh 较慢的两项为 TODO 注释和纯数字。纯数字返回 106,132 行，默认模式约 169 ms，而 rg 约 96 ms。TODO 注释只返回 726 行仍较慢，说明结果数量不足以单独解释所有慢项；本次未采集候选文件数量或 CPU profile，不能据此把每个慢项都归因为全量扫描。

十六进制查询 `\b0x[0-9A-Fa-f]+\b` 只有两字符固定前缀，但字符类展开仍可能形成可用的索引片段。原始运行脚本中的该项 category 写为 “scan fallback / short prefix”，只是误标的设计标签，不是测得的执行路径；可复用脚本已改为 “hexadecimal / short prefix”，查询、参数和数据均未改变。

两项结果差异如下，默认与 no-refresh 的差异完全相同；它们均未进入上述计时。

| 正则 | rg 匹配行 | coderg 匹配行 | 多出行数 | 原因 |
|---|---:|---:|---:|---|
| `async\s+def\s+\w+` | 1,974 | 1,978 | 4 | `\s+` 跨越换行，匹配前一行标识符末尾的 `async` 与后面的 `def` |
| `^[ \t]*$` | 257,531 | 263,971 | 6,440 | 6,177 个末尾换行后的虚拟行，以及 263 个空文件的第 1 行 |

已对全部额外行做未计时复核，没有缺失行。跨行例子：`tests/distributed/test_sharded_rdt_producer.py:914` 的 `engine._publish_async` 后面隔着空行出现 `def _pack`，coderg 匹配到 `async\n\n        def _pack`，并输出匹配起点所在行。实现使用整文件 `regex.find_iter(bytes)`，而默认 rg 是按行搜索，因此这里存在搜索语义差异。

将异步函数模式改成 `async[ \t]+def[ \t]+\w+` 后，三种命令均返回 1,974 行，输出 SHA-256 完全一致。该替代模式只做了未计时一致性校验。全部额外行的 EOF 分类和跨行片段见 [parity-diagnostics.json](data/vllm-regex-suite-2026-09-07/parity-diagnostics.json)。本次保留实现，记录差异供后续修复。

每项另用 macOS `/usr/bin/time -l` 测量 3 次峰值 RSS，不计入上述耗时。下表 RSS 均值是各查询 RSS 中位数的等权平均；最大值是所有查询与重复中的最大观测值。mmap 索引的磁盘大小与进程峰值 RSS 是不同指标。

| 模式 | RSS 中位数均值 MiB | 观测最大 RSS MiB |
|---|---:|---:|
| coderg 默认 | 22.99 | 61.84 |
| coderg --no-refresh | 20.36 | 60.23 |
| rg | 16.69 | 27.83 |

逐项 p95 如下，31 个样本采用 nearest-rank 定义。样本较少，尾延迟适合观察，不作为稳定 SLA。

| 查询 ID | 默认 p95 ms | no-refresh p95 ms | rg p95 ms |
|---|---:|---:|---:|
| literal_parallel | 32.28 | 13.96 | 67.88 |
| word_sampling | 34.84 | 16.20 | 70.45 |
| word_asyncmock | 30.90 | 13.21 | 67.05 |
| def_forward | 40.12 | 22.79 | 70.36 |
| class_attention | 39.47 | 20.71 | 68.20 |
| python_import | 68.08 | 55.87 | 101.48 |
| decorator | 44.42 | 27.78 | 76.14 |
| logger | 34.83 | 16.57 | 68.17 |
| exceptions | 47.05 | 28.21 | 79.33 |
| cache | 49.46 | 35.95 | 71.94 |
| tensor_creation | 46.55 | 27.62 | 73.70 |
| env_vars | 58.99 | 24.93 | 74.17 |
| cuda_call | 32.50 | 14.94 | 70.46 |
| config_assignment | 36.93 | 19.19 | 69.49 |
| raise_error | 43.33 | 25.30 | 104.95 |
| todo_comment | 116.87 | 100.55 | 69.60 |
| return_none | 64.23 | 40.78 | 70.71 |
| quoted_device | 37.83 | 20.54 | 70.75 |
| icase_sampling | 37.63 | 19.31 | 68.95 |
| icase_platform | 50.10 | 33.07 | 76.67 |
| icase_todo | 47.79 | 20.17 | 72.67 |
| icase_env | 60.28 | 43.27 | 82.22 |
| icase_logger | 40.47 | 21.29 | 70.31 |
| icase_literal | 37.33 | 20.75 | 69.76 |
| digits | 183.46 | 182.40 | 96.97 |
| hex_number | 35.27 | 17.50 | 68.02 |
| short_word | 114.47 | 96.37 | 96.18 |
| absent_regex | 29.61 | 12.86 | 67.60 |

运行前后 vLLM commit 与工作区状态、coderg 源码与二进制 SHA-256、全部索引文件 SHA-256 均保持一致。原有源码修改未被改动；本次新增 benchmark 脚本、报告及证据文件。

可复现入口（使用相同 vLLM checkout；索引和输出目录必须是新的目录，且位于 vLLM 源码树之外）：

```sh
cargo build --release --locked
python3 benches/regex_suite.py \
  --root /path/to/vllm \
  --binary target/release/coderg \
  --index-dir /tmp/coderg-vllm-regex-repro-index \
  --output /tmp/coderg-vllm-regex-repro-results \
  --iterations 31 --warmup 3 --rss-runs 3
```

`--rss-runs` 依赖 macOS；其他平台可省略该参数。通用入口是 [benches/regex_suite.py](data/legacy-harness-2026-09-22/regex_suite.py)，[harness.py](data/vllm-regex-suite-2026-09-07/harness.py) 保存实际执行版本的原始字节，供核对 SHA-256。若需直接使用归档版本，应把它放回 `benches/regex_suite.py` 的位置，因为它根据脚本位置定位源码。

完整证据：[results.json](data/vllm-regex-suite-2026-09-07/results.json) 保存所有原始计时样本、min/median/p95/mean、输出校验、RSS 样本和环境；[commands.json](data/vllm-regex-suite-2026-09-07/commands.json) 保存搜索命令；[rss-logs.tar.gz](data/vllm-regex-suite-2026-09-07/rss-logs.tar.gz) 保存 252 份 `/usr/bin/time -l` 原始日志；[source.diff](data/vllm-regex-suite-2026-09-07/source.diff) 保存测量版本相对 HEAD 的源码差异。
