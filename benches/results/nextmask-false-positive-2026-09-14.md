# 候选文件误报与 nextMask 过滤实验 — 2026-09-14

**当前索引的误报成本因查询而异；nextMask 对部分短片段有效，但无法消除大量由词边界、锚点和正则结构产生的误报。** 本实验测量现有搜索基线，并测试仅增强选中 trigram 的保守 nextMask 原型。它不是完整落盘索引的性能测试。

分支：`experiment/nextmask-false-positive-profile`；基线 commit：`a517489ba3c14d58154944b46a8fceea1d444c64`。产品 `src/` 未修改，新增独立 Rust benchmark、Python 驱动和本报告。

## 测量口径

- **候选误报比例** = 最终没有匹配行的候选文件数 / 候选文件数。它不是统计学中以全部负例文件为分母的 FPR。零候选时比例记为 —。
- **误报匹配耗时** = 无匹配候选文件上逐行正则扫描的工作线程累计墙钟时间；包括行切分，不包括读取文件、索引查询或结果文本构造。
- **误报匹配占比** = 误报匹配耗时 / 全部候选文件的匹配耗时，分子分母采用相同的工作线程累计口径。
- **候选扫描墙钟时间** = 并行读取全部候选文件并执行匹配的经过时间，包含 Rayon 调度。其占比的分母是编译正则、加载索引、生成查询、倒排筛选和扫描完成这几个阶段的经过时间。
- 工作线程累计时间包含被调度出去的等待，**不是 CPU 时间，不能直接除以多线程阶段的墙钟时间，也不能直接理解为可节省的用户延迟**。
- 使用 `search -c --no-refresh` 的逐行计数语义：所有行都扫描，不因首次命中提前停止。诊断阶段计时不含 CLI 启动、stdout 输出、释放索引和 refresh；另测未插桩 CLI 的完整进程耗时作为参照。
- 每项预热 2 轮；vLLM 测量 9 轮，Chromium 5 轮。两种模式交替先后执行；各轮重新加载索引，复用进程内 Rayon 线程池，文件系统缓存为热缓存。正式计时没有并发构建或其他 benchmark。逐文件计时本身有少量扰动。
- 表内各耗时分别取中位数；单项比例先逐轮计算再取中位数。汇总耗时按各查询中位数相加，汇总比例用这些合计值计算，因此不要求表内独立中位数精确相除。
- 查询集合是人为选择的功能覆盖集合；汇总候选数统计的是“查询、候选文件”对，同一文件可以在不同查询中重复出现。它不代表真实用户查询分布。

## 语料与验证

Apple M5，10 个逻辑 CPU，24 GiB RAM。两份旧 `/private/tmp` 源码被清理后，本轮均在项目 `.cache/2026-09-10/nextmask-profile/` 下恢复并重建 v5 索引，结果按本轮实际搜索范围统计，不混用历史索引的文件数。会话始于 9 月 10 日，实际测量在 9 月 14 日。

- vLLM：`569adb5a9780f9c02d22a6b29826acf711512356`，从本地保留的 Git 对象导出；6,835 个可搜索文件、86,384,180 字节。
- Chromium：`398630472335c10b9ca610a4d1b7888a040f702a`，下载官方固定提交 archive；461,549 个可搜索文件、3,039,547,005 字节。
- 全部 48 项查询：baseline / nextMask 的逐文件匹配行数在每个重复中一致，且与未插桩 coderg、同范围的 rg 参考结果逐文件一致。vLLM 使用 `rg --encoding none` 遍历；Chromium 使用 `rg --encoding none --text --no-ignore`，显式传入索引内 active + searchable 文件，每批 1,000 个路径，避免编码转码和二进制判定差异。完整文件清单及哈希随数据保存。验证比较完整映射，不仅比较总数。
- 初次 Chromium `test` 校验发现默认 rg 比 coderg 多命中 32 个 UTF-16 文件；rg 自动按 BOM 转码，而 coderg 把含 NUL 的这些文件视为二进制。随后统一使用 `rg --encoding none` 按原始字节校验，并重验此前查询；未重跑已经有效的计时。初次差异、完整命令以及驱动更新前后的 metadata 均保留。
- `if` 参考校验又发现 `chromium.ai`、`product_logo.ai` 两个文件的前 8 KiB 没有 NUL、后部存在 NUL：coderg 将其纳入索引，默认 rg 二进制检查会停止搜索。因此 Chromium 最终按上述文件清单 + `--text` 验证。这是测量输入范围的对齐，本轮没有更改产品的编码或二进制策略。
- 现有 `cargo test --locked` 的 77 项测试通过；benchmark 自检覆盖重叠 trigram、文件结尾、掩码 OR 合并、大小写、Unicode 和带可选/不同长度分支的正则。`cargo fmt --all -- --check`、`git diff --check` 通过。

## 全集汇总

| 语料 | 查询数 | 候选对 | 误报候选对 | 候选误报比例 | 误报匹配累计 ms | 误报占全部匹配工作 | 匹配占读取+匹配工作 |
|---|---:|---:|---:|---:|---:|---:|---:|
| vllm | 34 | 56,741 | 15,561 | 27.42% | 195.95 | 20.13% | 20.16% |
| chromium | 14 | 1,243,225 | 465,368 | 37.43% | 2109.36 | 25.66% | 3.79% |

读取文件的线程累计成本也很重要；nextMask 排除文件时可同时避免读取和匹配。下面单独列出候选扫描的墙钟时间，不能与上表的累计时间相加。

| 语料 | 基线扫描合计 ms | 扫描占搜索阶段 | nextMask 扫描合计 ms | 扫描耗时变化 | 减少候选对 | 占原候选 | 消除原误报 | 候选减少的查询数 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| vllm | 497.83 | 92.34% | 489.82 | -1.61% | 1,089 | 1.92% | 7.00% | 13/34 |
| chromium | 21791.14 | 97.07% | 21408.59 | -1.76% | 39,033 | 3.14% | 8.39% | 5/14 |

这些是 resident 元数据原型的扫描阶段对照。候选未变化的查询也会出现计时波动，不能据此声称算法加速或减速；没有把此表包装成完整 CLI 的 nextMask 加速比。排除全扫描回退查询后的汇总另见 `aggregate.json` 的 `indexed_only`。

## 对优化方向的判断

- Chromium 固定字符串 `test` 的误报从 24,425 个减少到 4,989 个，说明 nextMask 能有效排除部分 trigram 组合误报。但加上词边界成为 `\btest\b` 后，相同候选集中的误报为 92,464 个，过滤后仍有 73,028 个；这个 mask 条件不会检查完整单词边界。
- vLLM 的整行 `return None` 正则候选误报比例约 79%，nextMask 只排除 29 个文件；`def\s+forward\b` 的误报候选有 665 个，当前原型一个也未排除。高候选误报比例并不自动意味着 nextMask 收益高。
- Chromium 固定字符串 `test` 的正则匹配仅占“读取 + 匹配”线程累计时间约 2.6%。减少候选也会避免文件打开、读取和关闭；应同时考察这些成本，而不只关注正则引擎。
- 建议优先把它当成选中 trigram 上的可选增强继续验证。完整实现是否值得默认启用，还需要索引体积、构建/刷新和元数据读取成本的数据。

## nextMask 原型的边界

每个 trigram 使用一个 u8，bit 位置为 `ngram::hash(&[next_byte]) & 7`；对文档内全部出现位置的后续字节按位 OR。原型采用独立的精确 24-bit trigram 命名空间，避免与现有 u32 sparse gram hash 混用。处理完整文件，保留重叠出现；文件结尾没有后续字节的出现不贡献位。

查询约束来自现有 literal 提取器和 covering gram 选择：只有选中 gram 长度为 3、且同一 literal 中存在确定的第四个字节时才添加 mask 条件。不同必要组取 AND，literal 分支取 OR，分支内条件取 AND；如果任一 OR 分支没有可用 mask，整组不使用 mask 过滤。不会把 `abc.*d` 当成 `abcd`。

这是在现有候选集之后附加的保守过滤。它没有把各 literal 分支的长 gram postings 与 mask 条件共同规划，且不模拟全局 lookup budget 下的磁盘调度；不能视为已经实现了完整的 nextMask 查询规划。

**元数据预计算完全位于搜索计时之外，且只为本次查询涉及的 trigram、候选文件准备常驻掩码。** 各报告保留准备耗时、payload 字节数、饱和掩码数。payload 只统计掩码字节，不含 Vec 等容器开销，更不是完整索引大小；本轮未测完整索引体积、构建/刷新成本、额外磁盘读取或加载成本。过滤阶段只计常驻掩码判断。因此本实验支持判断过滤机会，不能证明完整实现会获得相同净收益。

## 每项查询：当前误报与匹配成本

“FP 匹配 ms”是工作线程累计值；“扫描 ms / 占比”是墙钟值；“CLI ms”来自未插桩 `-c --no-refresh` 独立进程。`fallback` 查询的候选就是全部可搜索文件。

### vllm

| 查询 ID | 候选 | 误报 | 误报 % | FP 匹配 ms | FP 匹配占比 % | 扫描 ms | 扫描占比 % | CLI ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `absent_regex` | 0 | 0 | — | 0.00 | — | 0.00 | 0.04 | 4.00 |
| `async_def` | 749 | 416 | 55.54 | 10.80 | 68.34 | 6.76 | 86.52 | 12.36 |
| `blank_lines (fallback)` | 6,835 | 1,359 | 19.88 | 11.39 | 7.97 | 59.58 | 99.10 | 66.97 |
| `cache` | 1,020 | 348 | 34.12 | 8.02 | 27.38 | 9.15 | 89.38 | 14.95 |
| `class_attention` | 588 | 406 | 69.05 | 12.54 | 66.35 | 5.46 | 79.08 | 11.26 |
| `config_assignment` | 623 | 185 | 29.70 | 6.47 | 33.04 | 5.97 | 79.19 | 11.65 |
| `cuda_call` | 314 | 52 | 16.56 | 1.29 | 13.99 | 2.89 | 76.89 | 7.78 |
| `decorator` | 1,352 | 7 | 0.52 | 0.35 | 2.16 | 11.56 | 89.08 | 18.11 |
| `def_forward` | 1,173 | 665 | 56.69 | 16.64 | 56.63 | 10.30 | 90.72 | 15.81 |
| `digits (fallback)` | 6,835 | 2,188 | 32.01 | 11.34 | 12.48 | 59.66 | 98.97 | 66.47 |
| `env_vars` | 1,216 | 51 | 4.19 | 0.87 | 3.65 | 10.53 | 93.16 | 15.95 |
| `exceptions` | 1,613 | 5 | 0.31 | 0.08 | 0.20 | 14.36 | 91.30 | 20.85 |
| `hex_number` | 238 | 25 | 10.50 | 0.48 | 7.20 | 2.34 | 80.53 | 7.69 |
| `icase_env` | 2,769 | 367 | 13.25 | 5.63 | 10.22 | 23.82 | 91.28 | 31.58 |
| `icase_literal` | 240 | 5 | 2.08 | 0.17 | 2.20 | 2.32 | 61.74 | 8.13 |
| `icase_logger` | 469 | 142 | 30.28 | 5.19 | 35.58 | 4.37 | 75.35 | 10.28 |
| `icase_platform` | 1,978 | 400 | 20.22 | 8.13 | 18.85 | 17.10 | 91.98 | 23.93 |
| `icase_sampling` | 451 | 90 | 19.96 | 4.13 | 31.99 | 4.26 | 62.58 | 11.20 |
| `icase_todo` | 535 | 11 | 2.06 | 0.62 | 3.81 | 5.14 | 85.36 | 10.94 |
| `literal_cuda` | 1,477 | 0 | 0.00 | 0.00 | 0.00 | 12.76 | 95.03 | 18.32 |
| `literal_parallel` | 235 | 0 | 0.00 | 0.00 | 0.00 | 2.13 | 78.37 | 6.83 |
| `literal_rocm` | 561 | 1 | 0.18 | 0.00 | 0.03 | 5.02 | 89.76 | 9.88 |
| `literal_test` | 3,485 | 883 | 25.34 | 10.09 | 31.67 | 30.06 | 96.64 | 36.45 |
| `logger` | 385 | 58 | 15.06 | 1.57 | 16.38 | 3.50 | 80.62 | 8.76 |
| `python_import` | 3,752 | 197 | 5.25 | 2.44 | 3.69 | 32.24 | 87.44 | 43.55 |
| `quoted_device` | 668 | 17 | 2.54 | 0.49 | 3.78 | 5.88 | 86.86 | 10.97 |
| `raise_error` | 1,589 | 200 | 12.59 | 4.37 | 13.10 | 13.80 | 89.80 | 20.67 |
| `return_none` | 3,145 | 2,490 | 79.17 | 32.47 | 68.27 | 27.32 | 91.92 | 35.21 |
| `short_word (fallback)` | 6,835 | 2,583 | 37.79 | 8.23 | 13.84 | 60.23 | 99.15 | 66.58 |
| `tensor_creation` | 1,211 | 89 | 7.35 | 1.61 | 5.33 | 10.74 | 86.96 | 17.65 |
| `todo_comment` | 517 | 93 | 17.99 | 2.59 | 12.92 | 4.98 | 89.40 | 10.54 |
| `word_asyncmock` | 25 | 4 | 16.00 | 0.18 | 25.75 | 0.34 | 43.80 | 4.36 |
| `word_sampling` | 373 | 12 | 3.22 | 0.23 | 2.95 | 3.31 | 85.46 | 8.24 |
| `word_test` | 3,485 | 2,212 | 63.47 | 27.54 | 55.56 | 29.92 | 96.50 | 35.73 |

### chromium

| 查询 ID | 候选 | 误报 | 误报 % | FP 匹配 ms | FP 匹配占比 % | 扫描 ms | 扫描占比 % | CLI ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `absent` | 0 | 0 | — | 0.00 | — | 0.00 | 0.01 | 60.39 |
| `bind_once` | 14,127 | 93 | 0.66 | 1.90 | 1.36 | 125.62 | 76.22 | 194.80 |
| `check_macro` | 20,339 | 14,259 | 70.11 | 265.25 | 70.28 | 192.70 | 83.34 | 255.58 |
| `class_decl` | 108,923 | 31,809 | 29.20 | 335.05 | 35.77 | 2203.87 | 97.06 | 2175.71 |
| `histogram` | 2,768 | 0 | 0.00 | 0.00 | 0.00 | 24.43 | 36.78 | 91.08 |
| `icase_weak_ptr_factory` | 8,899 | 86 | 0.97 | 11.36 | 13.03 | 78.90 | 60.29 | 159.05 |
| `literal_test` | 213,055 | 24,425 | 11.46 | 211.75 | 18.61 | 4307.43 | 98.40 | 4537.12 |
| `literal_void` | 122,264 | 0 | 0.00 | 0.00 | 0.00 | 2650.82 | 98.55 | 2575.82 |
| `max_file_size` | 98 | 0 | 0.00 | 0.00 | 0.00 | 5.35 | 13.56 | 65.24 |
| `short_word (fallback)` | 461,549 | 296,583 | 64.26 | 630.62 | 32.91 | 6835.08 | 99.38 | 7055.87 |
| `std_move` | 36,655 | 232 | 0.63 | 8.33 | 1.49 | 545.82 | 89.44 | 838.99 |
| `todo_comment` | 32,680 | 5,417 | 16.58 | 113.67 | 13.35 | 361.76 | 90.96 | 432.50 |
| `weak_ptr_factory` | 8,813 | 0 | 0.00 | 0.00 | 0.00 | 79.83 | 71.03 | 139.31 |
| `word_test` | 213,055 | 92,464 | 43.40 | 531.43 | 39.06 | 4379.51 | 98.42 | 4515.46 |

## 候选确实减少的 nextMask 对照

| 语料 / 查询 | 候选前→后 | 误报前→后 | 扫描 ms 前→后 | 常驻 mask 过滤 ms |
|---|---:|---:|---:|---:|
| vllm / `async_def` | 749 → 743 | 416 → 410 | 6.763 → 6.692 | 0.002 |
| vllm / `class_attention` | 588 → 587 | 406 → 405 | 5.457 → 5.478 | 0.001 |
| vllm / `config_assignment` | 623 → 615 | 185 → 177 | 5.974 → 5.748 | 0.006 |
| vllm / `decorator` | 1,352 → 1,348 | 7 → 3 | 11.564 → 11.755 | 0.003 |
| vllm / `icase_logger` | 469 → 467 | 142 → 140 | 4.367 → 4.231 | 0.017 |
| vllm / `icase_sampling` | 451 → 395 | 90 → 34 | 4.259 → 3.773 | 0.010 |
| vllm / `icase_todo` | 535 → 534 | 11 → 10 | 5.139 → 5.154 | 0.006 |
| vllm / `literal_test` | 3,485 → 3,006 | 883 → 404 | 30.060 → 25.890 | 0.008 |
| vllm / `python_import` | 3,752 → 3,741 | 197 → 186 | 32.239 → 32.682 | 0.014 |
| vllm / `quoted_device` | 668 → 665 | 17 → 14 | 5.885 → 5.871 | 0.004 |
| vllm / `raise_error` | 1,589 → 1,579 | 200 → 190 | 13.801 → 13.776 | 0.005 |
| vllm / `return_none` | 3,145 → 3,116 | 2,490 → 2,461 | 27.323 → 27.183 | 0.012 |
| vllm / `word_test` | 3,485 → 3,006 | 2,212 → 1,733 | 29.922 → 25.698 | 0.009 |
| chromium / `class_decl` | 108,923 → 108,851 | 31,809 → 31,737 | 2203.875 → 2302.838 | 0.176 |
| chromium / `literal_test` | 213,055 → 193,619 | 24,425 → 4,989 | 4307.430 → 4086.469 | 0.430 |
| chromium / `std_move` | 36,655 → 36,588 | 232 → 165 | 545.821 → 435.709 | 0.124 |
| chromium / `todo_comment` | 32,680 → 32,658 | 5,417 → 5,395 | 361.761 → 353.625 | 0.071 |
| chromium / `word_test` | 213,055 → 193,619 | 92,464 → 73,028 | 4379.512 → 3987.866 | 0.481 |

## 查询定义

| 语料 | 查询 ID | flags | pattern |
|---|---|---|---|
| vllm | `absent_regex` | `—` | `\bCODERG_BENCH_ABSENT_[0-9]{8}\b` |
| vllm | `async_def` | `—` | `async\s+def\s+\w+` |
| vllm | `blank_lines` | `—` | `^[ \t]*$` |
| vllm | `cache` | `—` | `\b(kv_cache\|block_size)\b` |
| vllm | `class_attention` | `—` | `^class[ \t]+\w*Attention\b` |
| vllm | `config_assignment` | `—` | `\bmax_(model_len\|num_seqs)[ \t]*=` |
| vllm | `cuda_call` | `—` | `\btorch\.cuda\.[a-z_]+\(` |
| vllm | `decorator` | `—` | `^[ \t]*@(?:torch\|pytest)\.[A-Za-z_]+` |
| vllm | `def_forward` | `—` | `def\s+forward\b` |
| vllm | `digits` | `—` | `\b[0-9]{2,4}\b` |
| vllm | `env_vars` | `—` | `\b(VLLM\|CUDA)_[A-Z0-9_]+\b` |
| vllm | `exceptions` | `—` | `\b(ValueError\|RuntimeError\|NotImplementedError)\b` |
| vllm | `hex_number` | `—` | `\b0x[0-9A-Fa-f]+\b` |
| vllm | `icase_env` | `-i` | `\b(VLLM\|CUDA)_[A-Z0-9_]+\b` |
| vllm | `icase_literal` | `-i -F` | `get_tensor_model_parallel_world_size` |
| vllm | `icase_logger` | `-i` | `logger\.(warning\|error)\(` |
| vllm | `icase_platform` | `-i` | `\b(cuda\|rocm)\b` |
| vllm | `icase_sampling` | `-i` | `\bSamplingParams\b` |
| vllm | `icase_todo` | `-i` | `\b(TODO\|FIXME)\b` |
| vllm | `literal_cuda` | `-F` | `cuda` |
| vllm | `literal_parallel` | `-F` | `get_tensor_model_parallel_world_size` |
| vllm | `literal_rocm` | `-F` | `rocm` |
| vllm | `literal_test` | `-F` | `test` |
| vllm | `logger` | `—` | `logger\.(warning\|error)\(` |
| vllm | `python_import` | `—` | `^from[ \t]+vllm(?:\.[\w.]+)?[ \t]+import\b` |
| vllm | `quoted_device` | `—` | `[\"'](?:cuda\|cpu\|rocm)[\"']` |
| vllm | `raise_error` | `—` | `raise[ \t]+\w*Error\(` |
| vllm | `return_none` | `—` | `^[ \t]*return[ \t]+None[ \t]*$` |
| vllm | `short_word` | `—` | `\bif\b` |
| vllm | `tensor_creation` | `—` | `\b(torch\|numpy)\.(empty\|zeros\|ones)\b` |
| vllm | `todo_comment` | `—` | `^[ \t]*#.*\b(TODO\|FIXME)\b` |
| vllm | `word_asyncmock` | `—` | `\bAsyncMock\b` |
| vllm | `word_sampling` | `—` | `\bSamplingParams\b` |
| vllm | `word_test` | `—` | `\btest\b` |
| chromium | `absent` | `-F` | `CODERG_DISTRIBUTION_ABSENT_92be7` |
| chromium | `bind_once` | `-F` | `base::BindOnce` |
| chromium | `check_macro` | `—` | `\bCHECK_[A-Z]+\(` |
| chromium | `class_decl` | `—` | `^class[ \t]+\w+` |
| chromium | `histogram` | `—` | `\bUmaHistogram(Boolean\|Enumeration)\b` |
| chromium | `icase_weak_ptr_factory` | `-i -F` | `WeakPtrFactory` |
| chromium | `literal_test` | `-F` | `test` |
| chromium | `literal_void` | `-F` | `void` |
| chromium | `max_file_size` | `-F` | `MAX_FILE_SIZE` |
| chromium | `short_word` | `—` | `\bif\b` |
| chromium | `std_move` | `—` | `\bstd::(move\|forward)\(` |
| chromium | `todo_comment` | `—` | `^[ \t]*//.*\b(TODO\|FIXME)\b` |
| chromium | `weak_ptr_factory` | `-F` | `WeakPtrFactory` |
| chromium | `word_test` | `—` | `\btest\b` |

## 可复现证据

- Rust 测量程序 `benches/candidate_profile.rs` 和 Python 驱动 `benches/candidate_profile.py` 保留在 `experiment/nextmask-false-positive-profile` 分支，也可解包[完整实验源码](data/nextmask-false-positive-2026-09-14/experiment-source.tar.gz)。在实验分支或解包目录先运行 `cargo build --release --locked --bench candidate_profile --bin coderg`，然后用 Python 驱动的 `--help` 查看路径参数。索引目录请使用绝对路径；主分支仅归档报告及证据。
- [语料来源、archive SHA-256、硬件信息](data/nextmask-false-positive-2026-09-14/provenance.json)、[全集及仅索引查询汇总](data/nextmask-false-positive-2026-09-14/aggregate.json)。
- [vLLM 逐查询汇总](data/nextmask-false-positive-2026-09-14/vllm/summary.json)、[Chromium 逐查询汇总](data/nextmask-false-positive-2026-09-14/chromium/summary.json)。各目录同时保存 metadata.json（代码、二进制、manifest 哈希）、commands.json.gz（完整命令）、每项查询的 json.gz（每轮原始计时、mask 条件和逐文件匹配行数）。
- 本轮生成报告脚本也随数据归档；大语料、索引、下载包留在被 Git 忽略的 `.cache`，没有加入结果文件。
