import hashlib
import html
import json
import pathlib
import shutil

WORK = pathlib.Path(__file__).parent
REPO = pathlib.Path('/Users/b1indsight/personal_work/coderg')
OUT = REPO / 'benches/results'
STEM = 'small-repo-profile-2026-09-07'
DATA = OUT / 'data' / STEM
DATA.mkdir(parents=True, exist_ok=True)
r = json.loads((WORK / 'results.json').read_text())
t = json.loads((WORK / 'threads.json').read_text())
phases = r['summary']['profile']['phases_ms']
manifest = json.loads((WORK / 'index/manifest.json').read_text())
searchable = sum(d['active'] and d['searchable'] for d in manifest['documents'])

for name in ('prepare.py', 'run.py', 'threads.py', 'render.py', 'results.json', 'threads.json',
             'trace.json', 'instrumentation.patch', 'source-sha256.json'):
    shutil.copy2(WORK / name, DATA / name)
shutil.copy2(WORK / 'source/src/probe.rs', DATA / 'probe.rs')

labels = {
    'main.run': '程序内部总耗时', 'cli.parse': 'CLI 参数解析', 'search.total': '搜索函数',
    'search.regex_compile': '正则编译', 'index.load': '索引加载', 'manifest.total': 'manifest 加载',
    'manifest.read': '读取 manifest', 'manifest.parse': '解析 manifest',
    'segment.load': '索引段打开 / mmap / 校验', 'segment.validate': '索引段校验',
    'refresh.total': '刷新检查', 'git.identity': 'Git 版本检查', 'git.discover': '打开 Git 仓库',
    'git.read_head': '读取 HEAD / commit', 'collect.total': '文件快照收集',
    'collect.setup': '遍历器准备', 'collect.rayon_init': 'Rayon 线程池初始化',
    'collect.walk_metadata': '目录遍历、忽略规则、元数据、线程同步',
    'collect.merge': '合并线程结果', 'collect.sort': '文件路径排序',
    'refresh.compare_snapshot': '比较文件快照', 'search.query_plan': '提取查询字面量',
    'search.postings': '查询倒排索引', 'search.read_and_match': '读取候选文件并匹配',
    'search.result_sort': '匹配结果排序', 'search.output': '输出结果',
}
selected = ['cli.parse', 'search.regex_compile', 'index.load', 'git.identity', 'collect.setup',
            'collect.walk_metadata', 'collect.merge', 'collect.sort', 'refresh.compare_snapshot',
            'search.query_plan', 'search.postings', 'search.read_and_match', 'search.result_sort', 'search.output']
table = '\n'.join(f'| {labels[n]} | {phases[n]["median"]:.4f} | {phases[n]["p95"]:.4f} |' for n in selected)
threads_table = '\n'.join(
    f'| {n} | {t["summary"][f"{n}/control"]["wall_ms"]:.3f} | '
    f'{t["summary"][f"{n}/profile"]["phases_ms"]["collect.walk_metadata"]:.3f} | '
    f'{t["summary"][f"{n}/profile"]["phases_ms"]["search.read_and_match"]:.3f} |'
    for n in (1, 2, 4, 10))

dependency = pathlib.Path('/Users/b1indsight/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ignore-0.4.33/src/walk.rs')
(DATA / 'dependency-observation.md').write_text(f'''# Dependency observation

The locked dependency is `ignore 0.4.33`. Its `WalkParallel` creates and joins worker threads on each run (walk.rs:1503–1533).
`Worker::get_work` waits for work or a quit message using the following code (walk.rs:1986–1987):

```rust
let dur = std::time::Duration::from_millis(1);
std::thread::sleep(dur);
```

Source SHA-256: `{hashlib.sha256(dependency.read_bytes()).hexdigest()}`.
These source observations and the thread-count experiment suggest scheduler / termination overhead.
The experiment did not separately time those sleeps or count their executions; it does not prove they explain the entire difference.
''')

report = f'''# 小仓库搜索分阶段计时 — 2026-09-07

对 viberwhisper 搜索 `^impl`。文件遍历阶段中位数 **{phases['collect.walk_metadata']['median']:.2f} ms**，完整文件快照收集 **{phases['collect.total']['median']:.2f} ms**，刷新检查共 **{phases['refresh.total']['median']:.2f} ms**。主要成本集中在遍历阶段；小仓库的索引加载仅 **{phases['index.load']['median']:.2f} ms**。

[交互式时间线与分项图]({STEM}.html) · [原始样本](data/{STEM}/results.json) · [Chrome trace](data/{STEM}/trace.json)

## 方法与范围

- 使用 `perf/refresh-path` 当前实现的临时源码副本，插入 `Instant` 计时；产品源码没有新增修改。release 构建沿用 Cargo.lock、thin LTO 与单 codegen unit。
- 语料固定在 `{r['before']['commit']}`；快照中 {len(manifest['source_state'])} 个文件，其中 {searchable} 个可检索文本文件，文本共约 1.30 MiB。
- 查询 `coderg search '^impl' ROOT --index-dir INDEX`：{dict(r['representative']['profile']['counts'])['candidate_files']} 个候选文件，{dict(r['representative']['profile']['counts'])['matched_files']} 个匹配文件，154 个匹配行。
- 比较无插桩版本、插桩版本、各自的 `--no-refresh` 与 rg。各预热 5 次、随机交错运行 51 次。热文件系统缓存，每次启动新进程；计时 stdout 指向 `/dev/null`，stderr 在进程结束时统一输出计时数据。
- 使用同一台 Apple M5 / 10 逻辑 CPU 的 Mac。本次为沙箱内计时，不与上一轮沙箱外矩阵的样本合并。没有测量冷缓存或首次二进制加载。
- 时间跨度由主线程记录。并行阶段包含等待线程完成的墙钟时间；它不是各线程 CPU 时间之和，也不是逐函数采样火焰图。
- 所有版本和 rg 的完整输出一致。测试前后语料提交、文件内容及索引文件校验和保持不变。

## 分项结果

单位 ms，下表每行分别取 51 次中位数和 p95，因此中位数不可直接相加得到精确总数。

| 阶段 | 中位数 | p95 |
|---|---:|---:|
{table}

`collect.walk_metadata` 包含目录枚举、忽略规则、文件元数据读取、遍历线程创建与同步，不能解释为纯 `stat` 系统调用耗时。排序、合并和快照比较相对很小。

程序内部 `main.run` 中位数 **{phases['main.run']['median']:.3f} ms**。外部完整进程计时如下：

| 模式 | 中位数 ms |
|---|---:|
| 无插桩，默认刷新 | {r['summary']['control']['wall_ms']['median']:.3f} |
| 插桩，默认刷新 | {r['summary']['profile']['wall_ms']['median']:.3f} |
| 无插桩，no-refresh | {r['summary']['control_no_refresh']['wall_ms']['median']:.3f} |
| 插桩，no-refresh | {r['summary']['profile_no_refresh']['wall_ms']['median']:.3f} |
| rg | {r['summary']['rg']['wall_ms']['median']:.3f} |

插桩进程外部耗时比对照约多 0.36 ms（默认）和 0.26 ms（no-refresh），包含计时、日志序列化/输出、二进制布局及调度差异。分项结果用于定位热点，不把插桩总耗时当作正式性能提升指标。

图中选取 `main.run` 恰好等于中位数的一次真实运行（第 {r['representative']['round'] + 1} 轮），内部耗时 {phases['main.run']['median']:.3f} ms，外部耗时 {r['representative']['wall_ms']:.3f} ms。两者差额包含进程创建、进入 main 之前的加载/初始化、退出清理、计时日志及父进程等待/测量开销；没有继续拆分，因此不能全部称为启动时间。

## 线程数诊断

另做独立的 51 轮随机交错实验，各预热 5 次，用 `RAYON_NUM_THREADS=1/2/4/10` 调整线程数。当前代码同时把这个值传给 ignore 遍历器，因此它同时影响遍历、排序和候选文件匹配。整次搜索使用无插桩二进制计时；分项使用插桩二进制。

| 线程数 | 完整进程 ms | 遍历阶段 ms | 读取候选并匹配 ms |
|---|---:|---:|---:|
{threads_table}

单线程相对 10 线程，遍历阶段从 2.779 ms 降到 0.726 ms，整次搜索从 7.081 ms 降到 5.485 ms，约减少 **22.5%**；单线程匹配变慢，说明遍历和匹配适合分别选择线程数。4 线程的匹配阶段在这组实验中更快，但不能据此确定所有查询的最优线程数。

锁定版本 `ignore 0.4.33` 在每次并行遍历时创建并等待工作线程，空闲线程等待任务/退出消息时有 1 ms 的休眠逻辑。[依赖源码观察](data/{STEM}/dependency-observation.md)。结合线程数实验，当前证据指向并行调度和收尾开销；本次没有单独记录这些休眠的执行次数或时长，尚不能把全部差额归因于这一处休眠。

本次只做诊断，未调整产品线程策略。

## 复现

数据目录保存了 `prepare.py`、`probe.rs` 和 `instrumentation.patch`。`prepare.py` 把产品源码复制到脚本所在目录的 `source/` 后插入计时，不修改原工作区。脚本中的 REPO/ROOT/CONTROL 路径需按本机环境调整。

```sh
python3 prepare.py
cargo build --manifest-path source/Cargo.toml --release --locked --offline --target-dir target
python3 run.py
python3 threads.py
```

首次准备时应使用一个新的可写目录。`run.py` 在脚本所在目录建立独立索引，保留全部原始样本；`threads.py` 复用这个索引。源码与二进制哈希见数据文件。
'''
(OUT / (STEM + '.md')).write_text(report)

payload = {'labels': labels, 'selected': selected, 'results': r, 'threads': t['summary']}
template = '''<!doctype html>
<html lang="zh-CN"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>小仓库搜索耗时</title>
<style>
body{font:15px/1.6 system-ui,sans-serif;background:#f4f6fa;color:#192337;margin:0;padding:28px}main{max-width:1150px;margin:auto}h1{font-size:28px;margin:0}h2{font-size:19px;margin-top:30px}p{color:#526079}section{background:white;padding:22px;border-radius:12px;margin:18px 0;box-shadow:0 2px 10px #19233708}.cards{display:flex;gap:14px;flex-wrap:wrap}.card{flex:1;min-width:170px;background:#edf2ff;border-radius:9px;padding:15px}.number{font-size:28px;color:#2845a0;font-weight:700}button,select{font:inherit;padding:7px 12px;border:1px solid #c7d1e2;border-radius:7px;background:white;color:#243854}button{cursor:pointer}svg{width:100%;height:auto}rect{cursor:pointer}text{font:12px system-ui;pointer-events:none}#detail{min-height:48px;background:#f4f6fa;padding:12px;border-radius:7px}table{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}th,td{padding:8px;border-bottom:1px solid #e6ebf3;text-align:right}th:first-child,td:first-child{text-align:left}.bar{height:12px;background:#6987e5;border-radius:3px;min-width:1px}.note{font-size:13px;color:#667189}a{color:#2845a0}.legend{display:flex;gap:18px;flex-wrap:wrap}strong{color:#192337}
</style><main>
<h1>小仓库搜索耗时</h1><p>viberwhisper · 123 个文本文件 / 129 个元数据条目 · 查询 <code>^impl</code> · 51 次热缓存搜索</p>
<div class="cards"><div class="card">无插桩完整进程<div class="number">7.10 ms</div></div><div class="card">文件遍历阶段<div class="number">2.81 ms</div></div><div class="card">Git 检查<div class="number">0.51 ms</div></div><div class="card">读取候选并匹配<div class="number">0.67 ms</div></div></div>
<section><h2>一次真实运行的时间线</h2><p>内部总耗时 4.405 ms。块宽表示墙钟耗时，子阶段放在下一层。点击色块查看并放大，悬停显示时间。</p>
<button id="reset">显示全部阶段</button><svg id="trace" viewBox="0 0 1080 410" role="img" aria-label="搜索阶段时间线"></svg><div id="detail">点击阶段查看详情。</div>
<p class="note">这是手工插入计时点生成的嵌套时间线，不是 CPU 采样火焰图。并行阶段显示主线程等待任务完成的时间，不展开工作线程调用栈。外部进程耗时 7.603 ms，额外部分没有细分。</p></section>
<section><h2>阶段分布</h2><label>统计量 <select id="stat"><option value="median">51 次中位数</option><option value="p95">51 次 p95</option></select></label><table><thead><tr><th>阶段</th><th>ms</th><th style="width:35%">相对最长阶段</th></tr></thead><tbody id="phases"></tbody></table><p class="note">各行独立统计，中位数不能精确相加。文件遍历阶段包含忽略规则、元数据读取和线程启动/同步。</p></section>
<section><h2>线程数诊断</h2><table><thead><tr><th>线程数</th><th>完整进程 ms</th><th>遍历 ms</th><th>读取并匹配 ms</th></tr></thead><tbody id="threads"></tbody></table><p>单线程使完整搜索耗时下降约 22.5%，同时候选匹配变慢。当前配置把遍历和匹配线程数绑定在一起。</p><p class="note">整次进程使用无插桩版本；分项使用插桩版本。独立 51 轮实验，不能与上方样本直接相加。产品线程策略未修改。</p></section>
<p><a href="small-repo-profile-2026-09-07.md">方法与完整报告</a> · <a href="data/small-repo-profile-2026-09-07/trace.json">Chrome trace JSON</a> · <a href="data/small-repo-profile-2026-09-07/results.json">原始样本</a></p>
</main><script>
const data=__DATA__;
const events=data.results.representative.profile.events.map(e=>({...e,end:e.start_ns+e.duration_ns}));
const whole=events.find(e=>e.name==='main.run');let focus=whole;
const ns='http://www.w3.org/2000/svg';
function element(tag,attrs,text){const n=document.createElementNS(ns,tag);Object.entries(attrs).forEach(([k,v])=>n.setAttribute(k,v));if(text)n.textContent=text;return n;}
function color(name){return name.startsWith('collect')?'#f2b777':name.startsWith('git')?'#c8b3ed':name.startsWith('refresh')?'#f2d8b8':name.startsWith('index')||name.startsWith('manifest')||name.startsWith('segment')?'#92cfca':'#a7bce9';}
function draw(){const svg=document.getElementById('trace');svg.replaceChildren();const filtered=events.filter(e=>e.start_ns>=focus.start_ns&&e.end<=focus.end).sort((a,b)=>a.start_ns-b.start_ns||b.duration_ns-a.duration_ns);let maxDepth=0;
for(let i=0;i<=5;i++){const x=10+i*212;svg.append(element('line',{x1:x,x2:x,y1:24,y2:390,stroke:'#e2e7f0'}));svg.append(element('text',{x:x+2,y:17},((focus.start_ns+focus.duration_ns*i/5)/1e6).toFixed(3)+' ms'));}
for(const e of filtered){let depth=filtered.filter(p=>p!==e&&p.start_ns<=e.start_ns&&p.end>=e.end&&(p.start_ns<e.start_ns||p.end>e.end)).length;maxDepth=Math.max(maxDepth,depth);const x=10+(e.start_ns-focus.start_ns)/focus.duration_ns*1060,w=Math.max(.8,e.duration_ns/focus.duration_ns*1060),y=33+depth*47;const group=element('g',{}),rect=element('rect',{x,y,width:w,height:38,rx:3,fill:color(e.name),stroke:'white'});rect.append(element('title',{},data.labels[e.name]+' · '+(e.duration_ns/1e6).toFixed(4)+' ms · '+e.name));rect.addEventListener('click',()=>{document.getElementById('detail').textContent=data.labels[e.name]+' ('+e.name+')：'+(e.duration_ns/1e6).toFixed(4)+' ms，占内部总耗时 '+(100*e.duration_ns/whole.duration_ns).toFixed(1)+'%。';focus=e;draw();});group.append(rect);if(w>75){const label=data.labels[e.name];group.append(element('text',{x:x+5,y:y+24,fill:'#203047'},label.slice(0,Math.max(2,Math.floor((w-12)/13)))))}svg.append(group)}svg.setAttribute('viewBox','0 0 1080 '+(85+maxDepth*47));}
document.getElementById('reset').onclick=()=>{focus=whole;draw()};draw();
function renderPhases(){const stat=document.getElementById('stat').value,p=data.results.summary.profile.phases_ms,max=Math.max(...data.selected.map(n=>p[n][stat]));document.getElementById('phases').innerHTML=data.selected.map(n=>'<tr><td>'+data.labels[n]+'</td><td>'+p[n][stat].toFixed(4)+'</td><td><div class="bar" style="width:'+(100*p[n][stat]/max)+'%"></div></td></tr>').join('');}document.getElementById('stat').onchange=renderPhases;renderPhases();
document.getElementById('threads').innerHTML=[1,2,4,10].map(n=>{const p=data.threads[n+'/profile'].phases_ms;return '<tr><td>'+n+'</td><td>'+data.threads[n+'/control'].wall_ms.toFixed(3)+'</td><td>'+p['collect.walk_metadata'].toFixed(3)+'</td><td>'+p['search.read_and_match'].toFixed(3)+'</td></tr>'}).join('');
</script></html>'''
# Only embed the representative trace and summary; full samples remain in the linked JSON.
payload['results'] = {k: r[k] for k in ('summary', 'representative')}
(OUT / (STEM + '.html')).write_text(template.replace('__DATA__', json.dumps(payload).replace('</', '<\\/')))

files = {str(p.relative_to(DATA)): {'bytes': p.stat().st_size,
          'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for p in DATA.iterdir() if p.is_file() and p.name != 'manifest.json'}
(DATA / 'manifest.json').write_text(json.dumps(files, indent=2) + '\n')
print(OUT / (STEM + '.md'))
print(OUT / (STEM + '.html'))
print('archived bytes:', sum(x['bytes'] for x in files.values()))
