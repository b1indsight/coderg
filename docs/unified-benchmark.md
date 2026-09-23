# 统一 benchmark 方案

状态：已实现统一 Rust 入口，使用方法见 [benchmark 指南](../benches/README.md)。审查及迁移日期：2026-09-22。

下文保留现状审查与设计依据；当前实际覆盖、统计口径和限制以 benchmark 指南为准。源码注入式诊断已归档，未迁移为稳定产品 API。

## 现有流程与结果

迁移前，`benches/` 顶层有 4 个 Rust benchmark 和 13 个 Python 脚本，另有
`results/data/` 中与历史实验绑定的驱动和汇总脚本。当时 Cargo 注册了 4 个独立
target，其中 manifest/history/commit benchmark 需要额外参数，不能直接把
`cargo bench` 当作完整套件入口。

值得保留的测量与回归场景：

| 来源 | 观察 | 新套件需要覆盖的内容 |
|---|---|---|
| [9/7 刷新矩阵](../benches/results/refresh-matrix-2026-09-07.md) | 7 组语料、102 个查询组合；不同文件数和大小分布收益不同 | 多档合成语料、真实语料、查询与输出模式矩阵 |
| [9/8 构建预算](../benches/results/build-memory-budget-2026-09-08.md) | vLLM RSS 512.84→208.08 MiB，构建 0.845→1.313 s | 同时报告构建延迟、RSS、外排与索引空间 |
| [9/10 三档历史](../benches/results/size-tiers-2026-09-10.md) | 小中大库的重建/合并行为不同 | 分离普通更新、重建、合并事件 |
| [9/18 完整测试](../benches/results/full-bench-final-2026-09-18.md) | vLLM 提交提升 47.20→813.71 ms，5 轮均复现 | 已索引脏内容提交后的段引用复用断言 |
| [9/19 复用修复](../benches/results/full-bench-reuse-2026-09-19.md) | 大改提交回退消除，但维护后搜索仍比 main 慢 | 初始与长期维护后搜索分别测量 |
| [9/19 版本化 base/overlay](../benches/results/versioned-base-overlay-2026-09-19.md) | 小库平均维护 +22.74%，峰值 193.12 ms；大库活动段增至 100 | 逐提交耗时、最大值、段数、磁盘增长曲线 |
| [9/19 双分支](../benches/results/two-branches-2026-09-19.md) | 分支切换明显受益，但当时只覆盖 5 个 tree | 分支交替、长历史回访、内容隔离与缓存复用 |
| [9/20 线程扫描](../benches/results/thread-scaling-2026-09-20.md) | 本次 vLLM 测量 4 线程最佳 | 线程配置作为实验维度，不能泛化最佳值 |
| [9/20 后台维护](../benches/results/background-oldest-2026-09-20.md) | vLLM 前台查询受争用影响约 14%～21% | 无维护/维护并发配对、维护阶段与前台尾延迟 |

这些数字来自历史报告，本次未重新运行或逐一审计全部原始样本。不同日期、二进制、
语料和计时方法的数字不能合成一次统一的性能比较。

现有实现的主要不一致：

- `compare_rg` 只测一个固定字符串；初始构建和状态转换各计时一次。
- 最新完整测试主要覆盖 `-Fl`，更早的正则和完整行矩阵没有整合进去。
- 部分脚本捕获 stdout，另一些重定向到空设备；历史 Rust harness 在 macOS
  使用 `/usr/bin/time -l` 包装计时，测得的延迟含包装开销。
- 有的输出比较用 `BTreeSet`，会丢弃重复结果；完整行正确性应保留重复项。
- 一次性驱动固定日期、缓存路径、旧二进制，甚至修改源码字符串注入诊断。
  这些不适合作为长期维护的 benchmark API。

## 入口与框架

建议保留一个 `[[bench]] name = "suite", harness = false`，设置
`autobenches = false` 并关闭产品 binary 的 libtest bench，确保无参数
`cargo bench` 能运行默认套件。入口使用 Rust，复用现有 clap/serde/tempfile。

首版选自定义 harness。理由是主要测量对象为外部 CLI、Git 状态转换、
RSS 和索引占用，需要严格控制每轮状态和查询顺序。

- [Cargo 官方文档](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
  明确支持 `harness = false`；遵循 cargo bench 不要求使用 nightly `#[bench]`。
- [Criterion](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html)
  支持 batched setup 和 `iter_custom`，可用于外部进程；适合后续需要自动统计分析的
  manifest 解码、候选计算等内部微基准。它不能替代本项目的状态编排与资源采集。
- [Divan](https://docs.rs/divan/latest/divan/) 提供参数化函数 benchmark、采样统计和
  cargo bench 集成；本任务的主要复杂度仍在进程与状态管理，引入它不会消除这些代码。

统一入口不等于单个巨大文件：按配置、执行器、语料、场景、报告拆分 Rust 模块；
禁止通过新入口继续调用旧 Python 脚本。

已实现的命令接口：

```sh
cargo bench
cargo bench --bench suite -- --config benches/full.json
cargo bench --bench suite -- --sources benches/sources.local.json --output target/benchmarks/run-001
cargo bench --bench suite -- --list
```

默认跑五个真实仓库的完整长测：viberwhisper、agentflow、whisper.cpp、vLLM、Chromium。
只比较当前构建的默认搜索与 rg，不提供旧版或 no-refresh 对比。确定性合成语料作为补充与 smoke，
需要显式选择，不替代缺失的真实语料。启动前检查全部语料，缺失时失败，不静默跳过。
配置里声明语料路径与固定 commit、查询集、场景和采样次数。线程与内存预算始终使用当前实现自身默认值，不提供覆盖或扫描。
不自动下载 Chromium/vLLM，不暗中使用机器特定的绝对路径。

本机已有 Chromium `398630472335c10b9ca610a4d1b7888a040f702a` 的源码归档，
没有 `.git`；按用户选择，在独立副本中初始化 Git 并生成 100 次确定性模拟提交，
用于历史回放、检查点搜索与回访测试。报告标记 `history_kind = synthetic`，
其结果不能当作 Chromium 的真实开发历史。
归档声明的提交号不能独立证明内容身份，还需要文件清单及内容摘要。
其他四库启动前校验固定提交与所需 first-parent 历史长度。

已核对的语料约束：

| 语料 | 固定提交 | 已检查的 first-parent 提交数 | 默认历史更新数 |
|---|---|---:|---:|
| viberwhisper | `bc73d1c54089dad53a3f5585ed5210b188542817` | 134 | 100 |
| agentflow | `911ea1ee5ae444c466e568edf2402b6fd15d8aea` | 57 | 56 |
| whisper.cpp | `c44b60b8053bbf2a5c1e014f11323fb3f2485177` | 201 | 100 |
| vLLM | `569adb5a9780f9c02d22a6b29826acf711512356` | 501 | 100 |
| Chromium | `398630472335c10b9ca610a4d1b7888a040f702a`（归档声明） | 无真实 Git 历史 | 100（模拟） |

agentflow 改用 2026-09-22 检查时本地最新 HEAD，工作区干净且不是浅克隆；
共有 57 个 first-parent 提交，可回放 56 次更新。该 SHA 固定在配置中，不能在每轮
采样时重新解析 HEAD。以后更新语料版本时显式更新 SHA 和回放长度。
56 次更新是显式配置，不能静默截断“100 次”请求；检查点为 0/14/28/42/56，
对应 0/25/50/75/100% 进度，并记录实际 commit。
旧 `/private/tmp/coderg-chromium-3986304/src` 已只剩空目录，不能使用。
候选源码树在 `.cache/2026-09-10/nextmask-profile/chromium`，另有同目录下 1.4 GB
`chromium.tar.gz`；正式运行前还须校验完整性和语料摘要，而非只检测目录存在。

Chromium 模拟历史的可复现合同：

- 在隔离副本初始化基线提交，不修改原始归档；生成提交与 Git 操作均在计时之外。
- 固定生成器版本、随机种子、候选文件排序、编辑内容、提交身份与时间。
  保存源语料摘要、每步操作清单、变更文件数/字节数及生成后的 commit/tree ID。
- 100 次更新混合局部修改、增删重命名和定期批量修改，同时保留反复编辑同一文件
  与修改新文件两类负载。操作分布和每步变更规模由配置固定，不只追加同一个标记。
- 每轮、每个 coderg 版本复用同一份已生成历史，从独立索引开始回放；
  检查点为 0/25/50/75/100，每步与 rg 对照，另外验证回滚与重访。
- 提交前已索引的内容提升、脏修改丢弃及双分支切换仍由 workflow/branches 场景
  单独覆盖，避免把生成历史时未建立索引的直接提交误当作提交提升测试。

## 覆盖矩阵

| 场景 | 必须测量/验证 | 默认 / 完整 |
|---|---|---|
| build | 每轮全新索引；时间、RSS、磁盘体积 | 默认使用实现自身线程设置和内存预算，不扫描预算 |
| search | literal、regex、OR、大小写、短词、无匹配、高命中；文件列表、行、计数；当前默认搜索与 rg | 默认完整语料查询集 |
| workflow | 增删改重命名、未跟踪/忽略文件、提交提升、回滚、重访、丢弃修改 | 默认 |
| branches | A1→B1→A2→B2；分支特有标记、提交复用 | 默认 |
| history | 固定 first-parent 提交列表；每步更新；检查点搜索与历史回访 | 默认 2 轮，更新数见语料表 |
| manifest | 使用实际产物调用生产 View::open；旧格式回退完整读取，记录 load 总耗时 | 默认；不转换格式，不含 CLI、读锁、段文件加载、新鲜度检查与析构 |
| diagnostics | 候选文件/字节、索引段、提取/复用量、阶段耗时 | 有稳定诊断接口时启用 |

默认语料包括 viberwhisper、agentflow、whisper.cpp、vLLM、Chromium。
补充语料为同为 64 MiB 的多小文件/少大文件合成对照。
不对所有维度直接做笛卡尔积；每个 preset 显式枚举需要回答的问题。

## 采样与正确性合同

1. 编译、版本检查、语料准备、预热和校验不计入测量。所有产品构建在采样前完成。
2. CLI 延迟包含启动、工作、输出写入空设备和退出；每个样本单独启动进程。
   状态转换不计 Git checkout/commit 与编辑时间。默认热文件系统缓存，明确标注；
   不把新进程或新索引叫作冷缓存。
3. 搜索每格预热 2 次、计时 16 次；构建/受控工作流 3 轮独立状态；历史和 RSS 各 2 轮。
   快速验证配置可减少样本，但报告标记为 smoke，不作为性能结论。
4. 同轮引擎/查询顺序按固定种子交错；依赖状态的工作流保持步骤顺序。
   默认每批最多两个仓库并发（`--jobs 2`），仓库内部场景串行。报告记录并发数，耗时包含仓库间资源争用；`--jobs 1` 恢复独占测量。每库输出隔离，失败记录保留，其他库继续执行。不在测量中编译或运行测试。
5. RSS 独立重复采集，不把包装器耗时计入 CLI 延迟。macOS/Linux 统一到 bytes；
   不支持的平台标注 unavailable，不能填 0。状态型 RSS 采样重新建立相同前置状态。
6. 正确性先于结论：退出码与完整输出对照 rg，保留重复行；只规范化明确等价的路径前缀
   和顺序。不支持/不同语义的选项显式标记，不能悄悄剔除失败样本。
7. 每次变更通过当前默认搜索验证新增/删除/分支特有标记，并与 rg 比较。
   提交提升验证完整段引用复用；长历史合并后不要求物理段路径保持不变。
8. 所有修改在独立克隆，固定到解析后的 commit；索引和输出在语料外。
   不触碰源工作区的 HEAD/索引/文件；缺少提交或依赖在采样前失败。
9. 两版各用自己的索引。旧版不支持某场景应显式列出能力缺失，不能伪造可比结果。

## 统一结果

一个独占输出目录，禁止覆盖已有运行：

```text
run.json         # schema_version、配置、源码/二进制 SHA256、语料 commit、环境、工具版本
samples.jsonl    # 每样本 scenario/corpus/variant/query/state/round/order/metric/value/unit
events.jsonl     # 状态转换、诊断、正确性检查、错误、跳过原因
summary.json     # 各同质场景统计与同轮配对比较
report.md        # 从上述数据生成，可单独重新渲染
```

元数据还包括 OS/arch/CPU/内存、线程、预算、缓存策略、精确命令、dirty 源码快照摘要、
Git/Rust/rg 版本、开始结束时间及二进制运行前后哈希。文件路径不等于版本身份。

延迟报告 N、median、mean、nearest-rank p95、max；状态转换展示逐步曲线和事件分类。
历史提交是异质负载，p95 只是描述性统计，不冒充置信区间；重复历史实验以完整轮次
为独立单位。比较同时给绝对差与百分比，避免小延迟百分比夸大。不提供混合所有
场景的单一总分。失败时落盘已采样数据并标记 incomplete，不生成成功结论。

## 迁移与验收

顶层脚本逐项归宿：

| 旧文件 | 新归宿 |
|---|---|
| `compare_rg.rs`、`regex_suite.py`、`snapshot_search.py` | search 场景，共用查询配置与正确性校验 |
| `refresh_matrix.py` | 语料/查询矩阵配置；采样交给共享执行器 |
| `build_memory.py` | build 场景与独立资源采样 |
| `thread_scaling.py` | 历史归档；当前套件不扫描线程 |
| `commit_updates.rs`、`commit_snapshots.py`、`snapshot_workflow.py` | workflow/branches 场景 |
| `history_updates.rs`、`vllm_git_transitions.py` | history 场景，去掉 vLLM 专用默认值 |
| `background_oldest.py` | 历史归档；主分支无对应后台推进流程，不纳入当前套件 |
| `manifest_load.rs` | manifest 场景 |
| `manifest_helpers.py` | 共享 stats/manifest 读取模块 |
| `commit_snapshot_summary.py`、`commit_snapshots_summary.py` | 统一报告生成 |
| `query_cover_metrics.py` | 旧源码注入方式归档；迁移到稳定诊断接口，不计入普通 CLI 延迟 |

1. 实现共享执行器、配置、报告与五库默认套件；验证无参数 cargo bench 的完整运行及缺失语料错误。
2. 移植真实语料、历史、分支、RSS；建立旧脚本到场景的覆盖表。线程和预算扫描已取消，统一遵循当前实现默认值。
3. 将正则等查询数据从脚本抽为配置；汇总只消费统一 schema。
4. 旧脚本中针对源码字符串的临时注入不直接迁移；诊断需要产品提供稳定接口后接入。
   历史实验源码、补丁、结果视为归档，不能声称在当前实现上仍可运行。
5. 完成覆盖及并跑验证后移除顶层旧脚本和旧 Cargo targets，更新 README 与文档命令。
   `results/` 历史数据保留；旧报告中的脚本引用移到归档快照，避免破坏证据链。
   已有未提交脚本也先归档后替换。
6. 验证范围：配置/统计/退出码/输出规范化的确定性测试；真实 CLI 小语料 smoke；
   小型真实 Git 历史回放；macOS RSS 实测；Linux RSS 在 Linux 环境验证。
   最后完整长测用于覆盖验收，不以未经复测的历史数字宣称新套件性能。
