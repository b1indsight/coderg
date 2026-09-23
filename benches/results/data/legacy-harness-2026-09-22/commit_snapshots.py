#!/usr/bin/env python3
"""Compare real CLI binaries: build, unchanged refresh, edits and cached rollback."""
import argparse
import hashlib
import json
import platform
from pathlib import Path
import re
import subprocess
import tempfile
import time


def command(args, cwd=None):
    result = subprocess.run(args, cwd=cwd, capture_output=True)
    if result.returncode:
        raise RuntimeError(f"{args}: {result.stderr.decode(errors='replace')}")
    return result.stdout.decode().strip()


def git(root, *args):
    return command(["git", *args], root)


def source(file, size, version):
    content = [f"// SNAPSHOT_{version} file {file}\n"]
    salt = 0xCBF29CE484222325
    for byte in version.encode():
        salt = ((salt ^ byte) * 0x100000001B3) & ((1 << 64) - 1)
    state = (file + 1) ^ salt
    length = len(content[0])
    line = 0
    while length < size:
        state = (state * 6364136223846793005 + 1) & ((1 << 64) - 1)
        text = f"pub fn handler_{file}_{line}() -> u64 {{ let value_{state:x} = {line}; value_{state:x} }}\n"
        content.append(text)
        length += len(text)
        line += 1
    return "".join(content).encode()


def install(root, files):
    for path in root.iterdir():
        if path.is_file() and path.name not in files:
            path.unlink()
    for name, data in files.items():
        path = root / name
        if not path.exists() or path.read_bytes() != data:
            path.write_bytes(data)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", required=True, type=Path)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--files", type=int, default=1024)
    parser.add_argument("--bytes-per-file", type=int, default=8192)
    parser.add_argument("--iterations", type=int, default=5)
    parser.add_argument("--search-iterations", type=int, default=15)
    parser.add_argument("--search-checkpoints", nargs="+", default=["build_A"])
    parser.add_argument("--workspace", type=Path, default=Path(".cache/2026-09-18/commit-snapshot-production"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if min(args.files, args.iterations, args.search_iterations) < 1:
        parser.error("counts must be positive")
    args.workspace.mkdir(parents=True, exist_ok=True)
    binaries = {"baseline": args.baseline.resolve(), "snapshots": args.binary.resolve()}
    a = {f"file_{i:06}.rs": source(i, args.bytes_per_file, "A") for i in range(args.files)}
    b = dict(a)
    changed = max(1, args.files // 2)
    for i in range(changed):
        b[f"file_{i:06}.rs"] = source(i, args.bytes_per_file, "B")
    if args.files > 1:
        del b["file_000001.rs"]
    b["new.rs"] = source(args.files, args.bytes_per_file, "NEW")
    dirty = dict(b)
    for i in range(changed):
        if i != 1:
            dirty[f"file_{i:06}.rs"] = source(i, args.bytes_per_file, "DIRTY")
    del dirty["new.rs"]
    dirty["untracked.rs"] = source(args.files + 1, args.bytes_per_file, "DIRTY")
    rows, searches = [], []
    verified = 0
    queries = [f"SNAPSHOT_{v}" for v in ["A", "B", "NEW", "DIRTY", "STAGED", "UNSTAGED"]]
    queries += ["handler_0_", "handler_1_", "handler_", "ABSENT_SNAPSHOT_TOKEN"]

    for iteration in range(args.iterations):
        with tempfile.TemporaryDirectory(prefix="cli-", dir=args.workspace.resolve()) as temp:
            parent = Path(temp)
            root = parent / "repo"
            root.mkdir()
            git(root, "init", "-q")
            git(root, "config", "user.name", "Snapshot benchmark")
            git(root, "config", "user.email", "bench@example.invalid")
            git(root, "config", "core.autocrlf", "false")
            install(root, a)
            git(root, "add", "-A")
            git(root, "commit", "-qm", "A")
            ca = git(root, "rev-parse", "HEAD")
            indexes = {name: parent / name for name in binaries}
            order = list(binaries) if iteration % 2 == 0 else list(reversed(binaries))

            def invoke(name, operation, query=None, no_refresh=False):
                cmd = [str(binaries[name]), operation, "--index-dir", str(indexes[name])]
                if query is not None:
                    cmd += ["-Fl", query]
                if no_refresh:
                    cmd += ["--no-refresh"]
                cmd.append(str(root))
                start = time.perf_counter_ns()
                result = subprocess.run(cmd, capture_output=True)
                ms = (time.perf_counter_ns() - start) / 1e6
                if result.returncode not in ([0, 1] if operation == "search" else [0]):
                    raise RuntimeError(result.stderr.decode(errors="replace"))
                return result, ms

            def unchanged_searches(label, files):
                nonlocal verified
                # Interleave variants and modes instead of subtracting medians.
                for query in ["ABSENT_SNAPSHOT_TOKEN", "handler_0_", "handler_"]:
                    expected = sorted(p for p, data in files.items() if query.encode() in data)
                    for sample in range(args.search_iterations + 2):
                        for name in order if sample % 2 == 0 else reversed(order):
                            for no_refresh in [False, True]:
                                result, ms = invoke(name, "search", query, no_refresh)
                                if result.stderr:
                                    raise AssertionError(f"unchanged search refreshed: {result.stderr!r}")
                                if sorted(result.stdout.decode().splitlines()) != expected:
                                    raise AssertionError(f"{label}/{name}/{query} differs from scan")
                                verified += 1
                                if sample >= 2:
                                    searches.append({"iteration": iteration, "sample": sample - 2,
                                                     "checkpoint": label, "variant": name, "query": query,
                                                     "no_refresh": no_refresh, "elapsed_ms": ms})

            def step(label, files, build=False):
                nonlocal verified
                for name in order:
                    result, ms = invoke(name, "index" if build else "search",
                                        None if build else "handler_0_")
                    diagnostic = result.stderr.decode()
                    if name == "snapshots" and label in {"rollback_A", "revisit_B", "discard_dirty", "discard_partial"}:
                        if "0 extracted" not in diagnostic:
                            raise AssertionError(f"{label}: {diagnostic}")
                    for query in queries:
                        check, _ = invoke(name, "search", query, no_refresh=True)
                        expected = sorted(p for p, data in files.items() if query.encode() in data)
                        actual = sorted(check.stdout.decode().splitlines())
                        if actual != expected:
                            raise AssertionError(f"{name}/{label}/{query}: {actual[:5]} != {expected[:5]}")
                        verified += 1
                    stats = json.loads(command([str(binaries[name]), "stats", "--json", "--index-dir", str(indexes[name]), str(root)]))
                    extracted = re.search(r"snapshot: (\d+) extracted, (\d+) reused", diagnostic)
                    rows.append({"iteration": iteration, "step": label, "variant": name,
                                 "elapsed_ms": ms, "diagnostic": diagnostic.strip(),
                                 "extracted": int(extracted[1]) if extracted else None,
                                 "reused": int(extracted[2]) if extracted else None,
                                 "segments": len(stats["segments"]),
                                 "disk_bytes": sum(p.stat().st_size for p in indexes[name].rglob("*") if p.is_file())})
                    print(f"{iteration} {label} {name}: {ms:.2f} ms", flush=True)
                if label in args.search_checkpoints:
                    unchanged_searches(label, files)

            step("build_A", a, build=True)
            install(root, b)
            step("large_edit", b)
            git(root, "add", "-A")
            git(root, "commit", "-qm", "B")
            cb = git(root, "rev-parse", "HEAD")
            step("commit_B", b)
            git(root, "reset", "--hard", ca)
            step("rollback_A", a)
            git(root, "reset", "--hard", cb)
            step("revisit_B", b)
            install(root, dirty)
            step("dirty_B", dirty)
            git(root, "reset", "--hard", cb)
            (root / "untracked.rs").unlink()
            step("discard_dirty", b)
            partial = dict(b)
            partial["file_000000.rs"] = source(0, args.bytes_per_file, "STAGED")
            install(root, partial)
            git(root, "add", "file_000000.rs")
            unstaged = dict(partial)
            unstaged["file_000000.rs"] = source(0, args.bytes_per_file, "UNSTAGED")
            install(root, unstaged)
            step("partial_edit", unstaged)
            git(root, "commit", "-qm", "C: partially staged")
            step("partial_commit", unstaged)
            git(root, "restore", "file_000000.rs")
            step("discard_partial", partial)

    report = {"schema": 1, "platform": platform.platform(), "files": args.files,
              "bytes_per_file": args.bytes_per_file, "iterations": args.iterations,
              "search_iterations": args.search_iterations, "verified_queries": verified,
              "search_checkpoints": args.search_checkpoints,
              "binaries": {name: {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
                           for name, path in binaries.items()}, "rows": rows, "searches": searches,
              "notes": ["Real CLI processes; setup, Git mutations and correctness checks excluded from update timings.",
                        "Synthetic Rust-like files in disposable real Git repositories; OS caches warm.",
                        "Five independent indexes by default; variant order alternates. Search drops two warmups per query.",
                        "Update search is handler_0_; all query results verified against direct byte scans.",
                        "disk_bytes includes active segments, retained snapshots, manifests and other index files."]}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
