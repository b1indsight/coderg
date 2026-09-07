import difflib
import hashlib
import json
import pathlib
import shutil

REPO = pathlib.Path('/Users/b1indsight/personal_work/coderg')
WORK = pathlib.Path(__file__).parent
SOURCE = WORK / 'source'
SOURCE.mkdir()
shutil.copytree(REPO / 'src', SOURCE / 'src')
for name in ('Cargo.toml', 'Cargo.lock'):
    shutil.copy2(REPO / name, SOURCE / name)
(SOURCE / 'benches').mkdir()
shutil.copy2(REPO / 'benches/compare_rg.rs', SOURCE / 'benches/compare_rg.rs')

hashes = {}
patches = []

def edit(name, transform):
    path = SOURCE / 'src' / name
    old = path.read_text()
    hashes[name] = hashlib.sha256(old.encode()).hexdigest()
    new = transform(old)
    path.write_text(new)
    patches.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True),
                                      fromfile='a/src/' + name, tofile='b/src/' + name))

def replace(text, old, new):
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new, 1)

def span(name, variable='phase'):
    return f'    let {variable} = crate::probe::Span::new("{name}");\n'

def main(text):
    text = 'mod probe;\n' + text
    text = replace(text, '    if let Err(error) = run() {',
                   '    let result = {\n' + span('main.run', '_main') +
                   '        run()\n    };\n    probe::emit();\n    if let Err(error) = result {')
    text = replace(text, '    match Cli::parse().command {',
                   span('cli.parse') + '    let cli = Cli::parse();\n    drop(phase);\n    match cli.command {')
    return text

def search(text):
    text = replace(text, '    let regex_pattern = if options.fixed_strings {',
                   span('search.total', '_search') + span('search.regex_compile') +
                   '    let regex_pattern = if options.fixed_strings {')
    text = replace(text, '    let mut disk_index = match index::load(path, requested_index_dir) {',
                   '    drop(phase);\n    let mut disk_index = match index::load(path, requested_index_dir) {')
    text = replace(text, '    let strategies = if options.fixed_strings && !options.ignore_case {',
                   span('search.query_plan') + '    let strategies = if options.fixed_strings && !options.ignore_case {')
    text = replace(text, '    let candidates = choose_candidates(&disk_index, &strategies)?;',
                   '    drop(phase);\n' + span('search.postings') +
                   '    let candidates = choose_candidates(&disk_index, &strategies)?;\n    drop(phase);\n'
                   '    crate::probe::count("candidate_files", candidates.len());\n' + span('search.read_and_match'))
    text = replace(text, '    let mut results: Vec<FileResult> = results.into_iter().flatten().collect();',
                   '    drop(phase);\n' + span('search.result_sort') +
                   '    let mut results: Vec<FileResult> = results.into_iter().flatten().collect();')
    text = replace(text, '    for result in &results {',
                   '    drop(phase);\n    crate::probe::count("matched_files", results.len());\n'
                   '    crate::probe::count("matching_lines", results.iter().map(|r| r.matches).sum());\n' +
                   span('search.output') + '    for result in &results {')
    text = replace(text, '    Ok(!results.is_empty())', '    drop(phase);\n    Ok(!results.is_empty())')
    return text

def index(text):
    text = replace(text, 'pub fn refresh(index: &DiskIndex, requested_index_dir: Option<&Path>) -> Result<RefreshOutcome> {',
                   'pub fn refresh(index: &DiskIndex, requested_index_dir: Option<&Path>) -> Result<RefreshOutcome> {\n' + span('refresh.total', '_refresh'))
    text = replace(text, '        if current_state == index.manifest.source_state {',
                   '        let unchanged = {\n' + span('refresh.compare_snapshot', '_compare') +
                   '            current_state == index.manifest.source_state\n        };\n        if unchanged {')
    text = replace(text, 'pub fn load(path: &Path, requested_index_dir: Option<&Path>) -> Result<DiskIndex> {',
                   'pub fn load(path: &Path, requested_index_dir: Option<&Path>) -> Result<DiskIndex> {\n' + span('index.load', '_load'))
    text = replace(text, '    Ok(DiskIndex { manifest, segments })',
                   '    crate::probe::count("indexed_files", manifest.documents.len());\n'
                   '    crate::probe::count("indexed_snapshot_files", manifest.source_state.len());\n'
                   '    crate::probe::count("index_segments", manifest.segments.len());\n'
                   '    Ok(DiskIndex { manifest, segments })')
    text = replace(text, '    Ok(serde_json::from_slice(&fs::read(path)?)?)',
                   span('manifest.total', '_manifest') + span('manifest.read') +
                   '    let bytes = fs::read(path)?;\n    drop(phase);\n' + span('manifest.parse') +
                   '    let manifest = serde_json::from_slice(&bytes)?;\n    drop(phase);\n    Ok(manifest)')
    text = replace(text, 'fn collect_files(root: &Path, index_dir: &Path) -> Result<Vec<FileState>> {',
                   'fn collect_files(root: &Path, index_dir: &Path) -> Result<Vec<FileState>> {\n' +
                   span('collect.total', '_collect') + span('collect.setup'))
    text = replace(text, '        .threads(rayon::current_num_threads())',
                   '        .threads({\n' + span('collect.rayon_init', '_rayon') +
                   '            rayon::current_num_threads()\n        })')
    text = replace(text, '    builder.build_parallel().run(|| {',
                   '    drop(phase);\n' + span('collect.walk_metadata') + '    builder.build_parallel().run(|| {')
    text = replace(text, '    let batches = batches\n',
                   '    drop(phase);\n' + span('collect.merge') + '    let batches = batches\n')
    text = replace(text, '    files.par_sort_unstable_by(|left, right| left.path.cmp(&right.path));',
                   '    drop(phase);\n' + span('collect.sort') +
                   '    files.par_sort_unstable_by(|left, right| left.path.cmp(&right.path));\n    drop(phase);\n'
                   '    crate::probe::count("walked_files", files.len());')
    return text

def git(text):
    for signature, label in (
        ('pub fn identity(root: &Path) -> Result<Option<GitIdentity>> {', 'git.identity'),
        ('fn discover(root: &Path) -> Result<Option<Repository>> {', 'git.discover'),
        ('fn read_identity(repository: &Repository) -> Result<GitIdentity> {', 'git.read_head'),
    ):
        text = replace(text, signature, signature + '\n' + span(label, '_git'))
    return text

def segment(text):
    signature = 'pub fn load(index_dir: &Path, meta: &SegmentMeta) -> Result<Segment> {'
    text = replace(text, signature, signature + '\n' + span('segment.load', '_segment'))
    text = replace(text, '    validate(&lookup, &postings)?;',
                   '    {\n' + span('segment.validate', '_validate') + '        validate(&lookup, &postings)?;\n    }')
    return text

edit('main.rs', main)
edit('search.rs', search)
edit('index.rs', index)
edit('git_state.rs', git)
edit('segment.rs', segment)
(WORK / 'instrumentation.patch').write_text(''.join(patches))
(WORK / 'source-sha256.json').write_text(json.dumps(hashes, indent=2) + '\n')
(SOURCE / 'src' / 'probe.rs').write_text('''use std::cell::RefCell;
use std::sync::OnceLock;
use std::time::Instant;

static ORIGIN: OnceLock<Instant> = OnceLock::new();
#[derive(serde::Serialize)]
struct Event { name: &'static str, start_ns: u64, duration_ns: u64 }
thread_local! {
    static EVENTS: RefCell<Vec<Event>> = RefCell::new(Vec::with_capacity(64));
    static COUNTS: RefCell<Vec<(&'static str, usize)>> = const { RefCell::new(Vec::new()) };
}
pub struct Span { name: &'static str, start: Instant }
impl Span {
    pub fn new(name: &'static str) -> Self {
        ORIGIN.get_or_init(Instant::now);
        Self { name, start: Instant::now() }
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        let end = Instant::now();
        EVENTS.with(|events| events.borrow_mut().push(Event {
            name: self.name, start_ns: self.start.duration_since(*ORIGIN.get().unwrap()).as_nanos() as u64,
            duration_ns: end.duration_since(self.start).as_nanos() as u64,
        }));
    }
}
pub fn count(name: &'static str, value: usize) {
    COUNTS.with(|counts| counts.borrow_mut().push((name, value)));
}
pub fn emit() {
    EVENTS.with(|events| COUNTS.with(|counts| {
        eprintln!("CODERG_PROFILE {}", serde_json::json!({"events": *events.borrow(), "counts": *counts.borrow()}));
    }));
}
''')
print(SOURCE)
