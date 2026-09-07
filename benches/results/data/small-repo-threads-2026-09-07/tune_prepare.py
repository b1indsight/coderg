import json
import pathlib
import shutil
import subprocess

repo = pathlib.Path('/Users/b1indsight/personal_work/coderg')
work = pathlib.Path(__file__).parent
source = work / 'tune-source'
shutil.copytree(repo / 'src', source / 'src')
(source / 'benches').mkdir()
for name in ('Cargo.toml', 'Cargo.lock', 'benches/compare_rg.rs'):
    shutil.copy2(repo / name, source / name)
path = source / 'src/index.rs'
with path.open('a') as out:
    out.write('''
pub fn benchmark_collect(root: &Path, serial: bool) -> Result<()> {
    let started = std::time::Instant::now();
    let files = collect_files(root, &root.join(".coderg-index"), serial.then_some(0))?;
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    println!("{}", serde_json::json!({"files": files.len(), "ms": elapsed_ms}));
    Ok(())
}
''')
path = source / 'src/main.rs'
text = path.read_text()
text = text.replace('fn main() {', '''fn main() {
    if let Ok(mode) = std::env::var("CODERG_BENCH_WALK_MODE") {
        let root = std::env::args().nth(1).unwrap();
        index::benchmark_collect(std::path::Path::new(&root), mode == "serial").unwrap();
        return;
    }''', 1)
path.write_text(text)
(work / 'tune-target').mkdir()
shutil.copy2(repo / 'target/release/coderg', work / 'after')
subprocess.run(['cp', '-cR', str(repo / 'target/release'), str(work / 'tune-target/release')], check=True)

base = pathlib.Path('/private/tmp/coderg-refresh-wide-79u03ttz')
plan = json.loads((base / 'plan.json').read_text())
roots = {d['id']: d['root'] for d in plan['datasets']}
template = pathlib.Path(roots['regular_4096'])
files = sorted(template.glob('module_*/*.rs'))
for count in (512, 1024, 2048):
    root = work / f'synthetic_{count}'
    root.mkdir()
    subprocess.run(['git', 'init', '-q', str(root)], check=True)
    for original in files[:count]:
        target = root / original.relative_to(template)
        target.parent.mkdir(exist_ok=True)
        shutil.copy2(original, target)
    roots[f'synthetic_{count}'] = str(root)
(work / 'tune-plan.json').write_text(json.dumps(roots, indent=2) + '\n')
print(source)
