"""Instrument a disposable source copy, never the product source."""
from pathlib import Path

ROOT = Path('/private/tmp/coderg-chromium-perf-20260909/src')

def edit(file, old, new):
    path = ROOT / file
    text = path.read_text()
    assert text.count(old) == 1, (file, old, text.count(old))
    path.write_text(text.replace(old, new))

def mark(label):
    return f'\neprintln!("PERF {label} {{}}", perf_mark.elapsed().as_secs_f64()*1000.0); perf_mark = std::time::Instant::now();\n'

edit('search.rs', ') -> Result<bool> {\n    let regex_pattern', ') -> Result<bool> {\n    let mut perf_mark = std::time::Instant::now();\n    let regex_pattern')
edit('search.rs', '    let mut disk_index = match', mark('regex_compile') + '    let mut disk_index = match')
edit('search.rs', '    if !options.no_refresh {', mark('load_total') + '    if !options.no_refresh {')
edit('search.rs', '    let strategies = if options.fixed_strings', mark('refresh') + '    let strategies = if options.fixed_strings')
edit('search.rs', '    let candidates = choose_candidates', mark('query_plan') + '    let candidates = choose_candidates')
edit('search.rs', '    let root = &disk_index.manifest.root;', mark('candidates') + '    eprintln!("PERF candidate_count {}", candidates.len());\n    let root = &disk_index.manifest.root;')
edit('search.rs', '    let mut results: Vec<FileResult>', mark('read_and_match') + '    let mut results: Vec<FileResult>')
edit('search.rs', '    Ok(!results.is_empty())', mark('sort_and_output') + '    let found = !results.is_empty();\n    drop(results); drop(candidates); drop(disk_index);\n    eprintln!("PERF drop_index_and_results {}", perf_mark.elapsed().as_secs_f64()*1000.0);\n    Ok(found)')
edit('index.rs', '    Ok(serde_json::from_slice(&fs::read(path)?)?)', '''    let start = std::time::Instant::now();
    let bytes = fs::read(path)?;
    let read_ms = start.elapsed().as_secs_f64()*1000.0;
    let start = std::time::Instant::now();
    let manifest = serde_json::from_slice(&bytes)?;
    let parse_ms = start.elapsed().as_secs_f64()*1000.0;
    let start = std::time::Instant::now();
    drop(bytes);
    eprintln!("PERF manifest_buffer_drop {}", start.elapsed().as_secs_f64()*1000.0);
    eprintln!("PERF manifest_read {}", read_ms);
    eprintln!("PERF manifest_parse {}", parse_ms);
    Ok(manifest)''')
edit('index.rs', '    let segments = manifest\n', '    let start = std::time::Instant::now();\n    let segments = manifest\n')
edit('index.rs', '    Ok(DiskIndex { manifest, segments })', '    eprintln!("PERF segment_load {}", start.elapsed().as_secs_f64()*1000.0);\n    Ok(DiskIndex { manifest, segments })')
edit('index.rs', '    let mut builder = WalkBuilder::new(root);', '    let mut perf_mark = std::time::Instant::now();\n    let mut builder = WalkBuilder::new(root);')
edit('index.rs', '    let batches = batches\n', mark('refresh_walk_metadata') + '    let batches = batches\n')
edit('index.rs', '    files.par_sort_unstable_by', mark('refresh_merge') + '    files.par_sort_unstable_by')
edit('index.rs', '    Ok(files)\n}\n\nfn file_state', '    eprintln!("PERF refresh_sort {}", perf_mark.elapsed().as_secs_f64()*1000.0);\n    eprintln!("PERF snapshot_files {}", files.len());\n    Ok(files)\n}\n\nfn file_state')
edit('main.rs', '    if let Err(error) = run() {\n', '    let repeats = std::env::var("CODERG_PERF_REPEAT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);\n    for _ in 0..repeats { if let Err(error) = run() {\n')
edit('main.rs', '\n}\n\nfn run() -> Result<()> {', '\n    }\n}\n\nfn run() -> Result<()> {')
