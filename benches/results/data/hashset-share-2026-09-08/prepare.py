"""Add a repeat-build driver to an isolated source copy, leaving hot code intact."""
import argparse
import difflib
import hashlib
import json
from pathlib import Path
import shutil

p = argparse.ArgumentParser()
p.add_argument("repo", type=Path)
p.add_argument("source", type=Path)
p.add_argument("output", type=Path)
a = p.parse_args()
a.source.mkdir(parents=True, exist_ok=False)
for name in ("src", "tests"):
    shutil.copytree(a.repo / name, a.source / name)
for name in ("Cargo.toml", "Cargo.lock"):
    shutil.copy2(a.repo / name, a.source / name)
(a.source / "benches").mkdir()
shutil.copy2(a.repo / "benches/compare_rg.rs", a.source / "benches/compare_rg.rs")
path = a.source / "src/main.rs"
before = path.read_text()
marker = "            let summary = index::build(&path, index_dir.as_deref(), build.build_memory_mib)?;"
assert before.count(marker) == 1
after = before.replace(marker, marker + '''
            let repeat: usize = std::env::var("CODERG_PROFILE_REPEAT").ok()
                .and_then(|value| value.parse().ok()).unwrap_or(1);
            if repeat > 1 { eprintln!("PROFILE_PID {}", std::process::id()); }
            for _ in 1..repeat {
                let temporary = tempfile::tempdir()?;
                index::build(&path, Some(temporary.path()), build.build_memory_mib)?;
            }''')
path.write_text(after)
(a.output / "driver.patch").write_text("".join(difflib.unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), fromfile="a/src/main.rs", tofile="b/src/main.rs")))
paths = sorted((a.repo / "src").glob("*.rs")) + [a.repo / "Cargo.toml", a.repo / "Cargo.lock"]
(a.output / "source-sha256.json").write_text(json.dumps({str(p.relative_to(a.repo)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, indent=2) + "\n")
print(a.source)
