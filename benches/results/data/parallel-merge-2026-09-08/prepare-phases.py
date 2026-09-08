"""Instrument an isolated production-source copy; never edit product sources."""
import argparse
import shutil
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("repo", type=Path)
p.add_argument("dest", type=Path)
a = p.parse_args()
assert a.repo.resolve() != a.dest.resolve()
a.dest.mkdir(parents=True, exist_ok=True)
for name in ("src", "tests", "benches"):
    if name == "benches":
        (a.dest / name).mkdir(exist_ok=True)
        shutil.copy2(a.repo / name / "compare_rg.rs", a.dest / name)
    else:
        shutil.copytree(a.repo / name, a.dest / name, dirs_exist_ok=True)
for name in ("Cargo.toml", "Cargo.lock"):
    shutil.copy2(a.repo / name, a.dest / name)

names = "total collect_files git pipeline receive_wait extend file_open file_read gram_hash chunk_collect send_wait sort dedup spill spill_write merge count_keys encode final_write manifest partition parallel_merge_encode merge_encode_worker postings_join lookup_assemble".split()
counters = "chunks read_bytes emitted_records sort_input sort_output".split()
probe = '''use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
'''
for i, name in enumerate(names + counters):
    probe += f"pub const {name.upper()}: usize = {i};\n"
probe += f"static NS: [AtomicU64; {len(names)}] = [const {{ AtomicU64::new(0) }}; {len(names)}];\n"
probe += f"static COUNT: [AtomicU64; {len(names) + len(counters)}] = [const {{ AtomicU64::new(0) }}; {len(names) + len(counters)}];\n"
probe += '''pub struct Span(usize, Instant);
impl Span { pub fn new(id: usize) -> Self { Self(id, Instant::now()) } }
impl Drop for Span { fn drop(&mut self) {
    NS[self.0].fetch_add(self.1.elapsed().as_nanos() as u64, Ordering::Relaxed);
    COUNT[self.0].fetch_add(1, Ordering::Relaxed);
} }
pub fn timed<T>(id: usize, f: impl FnOnce() -> T) -> T { let _span = Span::new(id); f() }
pub fn add(id: usize, n: usize) { COUNT[id].fetch_add(n as u64, Ordering::Relaxed); }
pub fn report() {
    let names = NAMES;
    let mut values = serde_json::Map::new();
    for (i, name) in names.iter().enumerate() {
        let count = COUNT[i].swap(0, Ordering::Relaxed);
        if i < NS.len() {
            let ns = NS[i].swap(0, Ordering::Relaxed);
            values.insert((*name).to_owned(), serde_json::json!({"ms": ns as f64 / 1e6, "calls": count}));
        } else { values.insert((*name).to_owned(), serde_json::json!(count)); }
    }
    eprintln!("BUILD_PROFILE {}", serde_json::Value::Object(values));
}
'''
probe = probe.replace("NAMES", "[" + ", ".join(f'"{n}"' for n in names + counters) + "]")
(a.dest / "src/probe.rs").write_text(probe)

def edit(name, old, new, count=1):
    path = a.dest / "src" / name
    text = path.read_text()
    assert text.count(old) == count, (name, old, text.count(old), count)
    path.write_text(text.replace(old, new))

edit("main.rs", "mod build;", "mod probe;\nmod build;")
old = "            let summary = index::build(&path, index_dir.as_deref(), build.build_memory_mib)?;"
edit("main.rs", old, old + '''
            probe::report();
            let repeat: usize = std::env::var("CODERG_PROFILE_REPEAT").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
            for _ in 1..repeat {
                let temp = tempfile::tempdir()?;
                index::build(&path, Some(temp.path()), build.build_memory_mib)?;
                probe::report();
            }''')
edit("index.rs", ") -> Result<BuildSummary> {\n    let root = resolve_root(path)?;", ") -> Result<BuildSummary> {\n    let _total = crate::probe::Span::new(crate::probe::TOTAL);\n    let root = resolve_root(path)?;")
edit("index.rs", "let source_state = collect_files(&root, &index_dir, None)?;", "let source_state = crate::probe::timed(crate::probe::COLLECT_FILES, || collect_files(&root, &index_dir, None))?;")
# Other index refresh paths also inspect Git, but only build is exercised here.
edit("index.rs", "let repository_state = git_state::inspect(&root, &index_dir)?;", "let repository_state = crate::probe::timed(crate::probe::GIT, || git_state::inspect(&root, &index_dir))?;")
edit("index.rs", "    std::thread::scope(|scope| -> Result<()> {", "    let pipeline_timer = crate::probe::Span::new(crate::probe::PIPELINE);\n    std::thread::scope(|scope| -> Result<()> {")
edit("index.rs", '''                        sender
                            .send(Ok((position, hashes)))
                            .map_err(|_| anyhow::anyhow!("index build cancelled"))''', '''                        crate::probe::timed(crate::probe::SEND_WAIT, || sender
                            .send(Ok((position, hashes))))
                            .map_err(|_| anyhow::anyhow!("index build cancelled"))''')
edit("index.rs", "        for batch in receiver {", "        while let Ok(batch) = crate::probe::timed(crate::probe::RECEIVE_WAIT, || receiver.recv()) {")
edit("index.rs", "    let segment = postings.write(segment_id)?;", "    drop(pipeline_timer);\n    let segment = crate::probe::timed(crate::probe::FINAL_WRITE, || postings.write(segment_id))?;")
edit("index.rs", "    let mut file = File::open(path).with_context", "    let mut file = crate::probe::timed(crate::probe::FILE_OPEN, || File::open(path)).with_context")
edit("index.rs", "        let mut len = retained;\n        while", "        let mut len = retained;\n        let read_timer = crate::probe::Span::new(crate::probe::FILE_READ);\n        while")
edit("index.rs", "        if retained == 0 && is_binary", "        drop(read_timer);\n        crate::probe::add(crate::probe::READ_BYTES, len - retained);\n        if retained == 0 && is_binary")
edit("index.rs", "        emit(ngram::hashes_for_chunk(&bytes[..len]))?;", """        let hashes = crate::probe::timed(crate::probe::GRAM_HASH, || ngram::hashes_for_chunk(&bytes[..len]));
        crate::probe::add(crate::probe::CHUNKS, 1);
        crate::probe::add(crate::probe::EMITTED_RECORDS, hashes.len());
        emit(hashes)?;""")
edit("index.rs", "    write_manifest(&index_dir, &manifest)?;", "    let _manifest_timer = crate::probe::Span::new(crate::probe::MANIFEST);\n    write_manifest(&index_dir, &manifest)?;", 3)
# End of build only; the unused refresh guard simply times until that function returns.
edit("index.rs", "    Ok(BuildSummary {", "    drop(_manifest_timer);\n    Ok(BuildSummary {")

edit("build.rs", "    pub fn extend(&mut self, id: u32, hashes: &[ngram::GramHash]) -> Result<()> {", "    pub fn extend(&mut self, id: u32, hashes: &[ngram::GramHash]) -> Result<()> {\n        let _extend = crate::probe::Span::new(crate::probe::EXTEND);")
edit("build.rs", "    fn spill(&mut self) -> Result<()> {", "    fn spill(&mut self) -> Result<()> {\n        let _spill = crate::probe::Span::new(crate::probe::SPILL);")
edit("build.rs", "self.records.par_sort_unstable();", "crate::probe::add(crate::probe::SORT_INPUT, self.records.len());\n        crate::probe::timed(crate::probe::SORT, || self.records.par_sort_unstable());", 2)
edit("build.rs", "self.records.dedup();", "crate::probe::timed(crate::probe::DEDUP, || self.records.dedup());\n        crate::probe::add(crate::probe::SORT_OUTPUT, self.records.len());", 2)
edit("build.rs", "        let path = self.run_path()?;", "        let _write = crate::probe::Span::new(crate::probe::SPILL_WRITE);\n        let path = self.run_path()?;")
edit("build.rs", "let ngrams = self.records.chunk_by(|a, b| a.0 == b.0).count() as u64;", "let ngrams = crate::probe::timed(crate::probe::COUNT_KEYS, || self.records.chunk_by(|a, b| a.0 == b.0).count() as u64);")
edit("build.rs", "fn merge_runs(inputs: &[Run], output: PathBuf) -> Result<(Run, u64)> {", "fn merge_runs(inputs: &[Run], output: PathBuf) -> Result<(Run, u64)> {\n    let _merge = crate::probe::Span::new(crate::probe::MERGE);")
edit("segment.rs", "    const ENCODE_BYTES: usize = 64 * 1024;", "    let _encode = crate::probe::Span::new(crate::probe::ENCODE);\n    const ENCODE_BYTES: usize = 64 * 1024;")
# These phases only exist in the four-way implementation. Keep the script
# usable with the archived serial source for a paired phase comparison.
if "fn partition_runs(" in (a.dest / "src/build.rs").read_text():
    edit("build.rs", "fn partition_runs(runs: &[Run]) -> Result<Vec<Vec<RunRange<'_>>>> {", "fn partition_runs(runs: &[Run]) -> Result<Vec<Vec<RunRange<'_>>>> {\n    let _partition = crate::probe::Span::new(crate::probe::PARTITION);")
    edit("segment.rs", "    let parts = (0..partition_count)", "    let merge_timer = crate::probe::Span::new(crate::probe::PARALLEL_MERGE_ENCODE);\n    let parts = (0..partition_count)")
    edit("segment.rs", "    let ngrams = parts.iter()", "    drop(merge_timer);\n    let ngrams = parts.iter()")
    edit("segment.rs", ") -> Result<EncodedPart> {", ") -> Result<EncodedPart> {\n    let _worker = crate::probe::Span::new(crate::probe::MERGE_ENCODE_WORKER);")
    edit("segment.rs", "fn join_postings(path: &Path, parts: &[EncodedPart]) -> Result<u64> {", "fn join_postings(path: &Path, parts: &[EncodedPart]) -> Result<u64> {\n    let _join = crate::probe::Span::new(crate::probe::POSTINGS_JOIN);")
    edit("segment.rs", "    parts: &[EncodedPart],\n) -> Result<()> {", "    parts: &[EncodedPart],\n) -> Result<()> {\n    let _lookup = crate::probe::Span::new(crate::probe::LOOKUP_ASSEMBLE);")
print(a.dest)
