#!/usr/bin/env python3
"""Create the shared-longest-gram fixture and its refresh_matrix plan."""

import json
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1]).resolve()
root.mkdir(parents=True, exist_ok=False)
query = b"CODERG_BENCHMARK_NEEDLE"
# The fixed-weight baseline selects this 19-byte prefix. All files contain it,
# but only file_0007.txt contains the complete query.
anchor = b"CODERG_BENCHMARK_NE"
for i in range(512):
    content = anchor + b"\nmodule_" + str(i).encode() + b"\n"
    content += b"ordinary_value = 17;\n" * 1600
    if i == 7:
        content += query + b"\n"
    (root / f"file_{i:04}.txt").write_bytes(content)
for command in (
    ["git", "init", "--quiet"],
    ["git", "add", "."],
    ["git", "-c", "user.name=coderg benchmark", "-c", "user.email=benchmark@example.invalid",
     "commit", "--quiet", "-m", "Shared anchor fixture"],
):
    subprocess.run(command, cwd=root, check=True)
commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
print(json.dumps({"datasets": [{"id": "shared_anchor", "kind": "synthetic", "root": str(root), "commit": commit}]}, indent=2))
