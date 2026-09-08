"""Hold input bytes constant while changing file-level extraction parallelism."""
import argparse
import hashlib
import json
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("source", type=Path)
p.add_argument("output", type=Path)
a = p.parse_args()
files = sorted(a.source.rglob("*.rs"))
assert len(files) == 64
assert all(f.stat().st_size == 1024 * 1024 for f in files)
a.output.mkdir(parents=True, exist_ok=False)
manifest = {"source": str(a.source), "source_files": [], "groups": {}}
whole = hashlib.sha256()
for f in files:
    data = f.read_bytes()
    whole.update(data)
    manifest["source_files"].append({"path": str(f.relative_to(a.source)), "bytes": len(data), "lines": data.count(b"\n"), "sha256": hashlib.sha256(data).hexdigest()})
manifest["concatenated_sha256"] = whole.hexdigest()
for count in (1, 8):
    name = f"grouped_{count}"
    directory = a.output / name
    directory.mkdir()
    entries = []
    per_file = 64 // count
    grouped_hash = hashlib.sha256()
    for i in range(count):
        data = b"".join(f.read_bytes() for f in files[i * per_file:(i + 1) * per_file])
        path = directory / f"generated_{i:03}.rs"
        path.write_bytes(data)
        grouped_hash.update(data)
        entries.append({"path": path.name, "bytes": len(data), "lines": data.count(b"\n"), "sha256": hashlib.sha256(data).hexdigest()})
    assert grouped_hash.hexdigest() == whole.hexdigest()
    manifest["groups"][name] = {"path": str(directory), "files": entries}
(a.output / "provenance.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({k: v["path"] for k, v in manifest["groups"].items()}))
