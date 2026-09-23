"""Read manifest records from old JSON and current binary coderg indexes."""

import json
import subprocess


def manifest_path(index):
    binary = index / "manifest.bin"
    return binary if binary.exists() else index / "manifest.json"


def read_manifest(binary, root, index):
    path = manifest_path(index)
    if path.suffix == ".json":
        return json.loads(path.read_bytes())
    return json.loads(subprocess.check_output([
        str(binary), "stats", str(root), "--index-dir", str(index), "--json",
    ]))
