import hashlib
import json
from pathlib import Path
import shutil
import subprocess

OUT = Path(__file__).resolve().parent
REPO = Path('/Users/b1indsight/personal_work/coderg')
SNAPSHOT = OUT / 'original'
TRIAL = OUT / 'trial'
TARGET = OUT / 'target'

SNAPSHOT.mkdir()
for name in ['Cargo.toml', 'Cargo.lock']:
    shutil.copy2(REPO / name, SNAPSHOT / name)
for name in ['src', 'benches']:
    shutil.copytree(REPO / name, SNAPSHOT / name)
shutil.copytree(SNAPSHOT, TRIAL)
shutil.copytree(REPO / 'target/release', TARGET / 'release')
(OUT / 'binaries').mkdir()
shutil.copy2(REPO / 'target/release/coderg', OUT / 'binaries/coderg-v64-k256')
(OUT / 'before.diff').write_bytes(subprocess.check_output(['git', 'diff', '--binary'], cwd=REPO))
(OUT / 'source-sha256.json').write_text(json.dumps({
    str(p.relative_to(SNAPSHOT)): hashlib.sha256(p.read_bytes()).hexdigest()
    for p in SNAPSHOT.rglob('*') if p.is_file()
}, indent=2) + '\n')

query = (TRIAL / 'src/query.rs').read_text()
query = query.replace('pub const MAX_LITERAL_VARIANTS: usize = 64;\n', '')
query = query.replace('pub const MAX_INDEX_LOOKUPS: usize = 256;\n', '')
query = query.replace('MAX_LITERAL_VARIANTS', 'literal_variant_limit()')
query = query.replace('MAX_INDEX_LOOKUPS', 'index_lookup_limit()')
query = query.replace('use crate::ngram;', '''use crate::ngram;

pub fn literal_variant_limit() -> usize {
    static VALUE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *VALUE.get_or_init(|| std::env::var("CODERG_BENCH_VARIANTS").unwrap().parse().unwrap())
}

pub fn index_lookup_limit() -> usize {
    static VALUE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *VALUE.get_or_init(|| std::env::var("CODERG_BENCH_KEYS").unwrap().parse().unwrap())
}''')
(TRIAL / 'src/query.rs').write_text(query)

search = (TRIAL / 'src/search.rs').read_text()
search = search.replace('query::MAX_LITERAL_VARIANTS', 'query::literal_variant_limit()')
search = search.replace('query::MAX_INDEX_LOOKUPS', 'query::index_lookup_limit()')
search = search.replace('    let strategies = if options.fixed_strings', '    let planning_start = std::time::Instant::now();\n    let strategies = if options.fixed_strings', 1)
search = search.replace('    let candidates = choose_candidates', '    let planning_us = planning_start.elapsed().as_micros();\n    let postings_start = std::time::Instant::now();\n    let candidates = choose_candidates', 1)
search = search.replace('    let root = &disk_index.manifest.root;', '''    if std::env::var_os("CODERG_BENCH_TRACE").is_some() {
        eprintln!("{}", serde_json::json!({
            "candidate_files": candidates.len(),
            "candidate_bytes": candidates.iter().map(|&id| disk_index.manifest.documents[id as usize].len).sum::<u64>(),
            "planning_us": planning_us,
            "postings_us": postings_start.elapsed().as_micros(),
        }));
    }
    let root = &disk_index.manifest.root;''', 1)
search = search.replace('    let mut cache = ngram::GramHashMap::default();', '    let mut skipped = 0usize;\n    let mut cache = ngram::GramHashMap::default();', 1)
search = search.replace('        if cache.len() + new_hashes.len() > query::index_lookup_limit() {', '        if cache.len() + new_hashes.len() > query::index_lookup_limit() {\n            skipped += 1;', 1)
search = search.replace('    Ok(candidates)\n}', '''    if std::env::var_os("CODERG_BENCH_TRACE").is_some() {
        eprintln!("{}", serde_json::json!({
            "keys_loaded": cache.len(), "strategies": strategies.len(),
            "groups_skipped": skipped, "full_scan": candidates.is_none(),
        }));
    }
    Ok(candidates)
}''', 1)
(TRIAL / 'src/search.rs').write_text(search)
subprocess.run(['cargo', 'build', '--release', '--bin', 'coderg', '--locked', '--offline', '--target-dir', str(TARGET)], cwd=TRIAL, check=True)
shutil.copy2(TARGET / 'release/coderg', OUT / 'binaries/coderg-harness')
print('Prepared sweep harness and archived the real 64/256 binary.', flush=True)
