"""Freeze local Git sources and generate deterministic file-count controls."""
from pathlib import Path
import json
import subprocess

work = Path(__file__).resolve().parent
corpora = work / "corpora"
corpora.mkdir()
datasets = []


def git(root, *args):
    return subprocess.check_output(["git", "-c", "core.hooksPath=/dev/null",
                                   "-c", "commit.gpgsign=false", "-C", str(root), *args],
                                  stderr=subprocess.PIPE).decode().strip()


vllm = Path("/private/tmp/coderg-vllm-bench.7cZ1iI/vllm")
assert not git(vllm, "status", "--porcelain", "--untracked-files=all")
datasets.append(dict(id="vllm", kind="vllm", root=str(vllm), commit=git(vllm, "rev-parse", "HEAD")))
for name, kind in [("viberwhisper", "rust"), ("agentflow", "python")]:
    source = Path("/Users/b1indsight/personal_work") / name
    root = corpora / name
    subprocess.run(["git", "-c", "core.hooksPath=/dev/null", "clone", "--quiet", "--local",
                    "--no-hardlinks", str(source), str(root)], check=True)
    assert not git(root, "status", "--porcelain", "--untracked-files=all")
    datasets.append(dict(id=name, kind=kind, root=str(root), source=str(source),
                         commit=git(root, "rev-parse", "HEAD")))

for name, files, kib in [("small_256", 256, 16), ("regular_4096", 4096, 16),
                          ("many_small_16384", 16384, 4), ("few_large_64", 64, 1024)]:
    root = corpora / name
    root.mkdir()
    for file_id in range(files):
        directory = root / f"module_{file_id // 64:04}"
        directory.mkdir(exist_ok=True)
        target = kib * 1024
        data = bytearray()
        line = 0
        while len(data) < target:
            marker = "CODERG_BENCHMARK_NEEDLE" if file_id % 97 == 0 and line == 3 else "ordinary_value"
            data.extend((f'pub fn generated_{file_id}_{line}(input: usize) -> usize {{ let {marker} = '
                         f'"module_{file_id % 41}"; input.wrapping_mul(31).wrapping_add({line}) }}\n').encode())
            line += 1
        (directory / f"generated_{file_id:06}.rs").write_bytes(data[:target - 1] + b"\n")
    git(root, "init", "--quiet")
    git(root, "config", "user.name", "Coderg Benchmark")
    git(root, "config", "user.email", "benchmark@example.invalid")
    git(root, "add", ".")
    git(root, "commit", "--quiet", "-m", "Deterministic source corpus")
    datasets.append(dict(id=name, kind="synthetic", root=str(root), files=files,
                         kib_per_file=kib, source_bytes=files * kib * 1024,
                         commit=git(root, "rev-parse", "HEAD")))
    print(f"Generated {name}: {files} files, {files*kib/1024:.0f} MiB", flush=True)

(work / "plan.json").write_text(json.dumps(dict(datasets=datasets), indent=2) + "\n")
print(work / "plan.json")
