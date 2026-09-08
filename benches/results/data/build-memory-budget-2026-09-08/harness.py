#!/usr/bin/env python3
"""Compare fresh index builds, segment bytes, and peak RSS on macOS."""
import argparse
import hashlib
import json
import platform
import random
import re
import statistics
import subprocess
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def git_state(root):
    def git(*args):
        result = subprocess.run(["git", "-C", str(root), *args], capture_output=True)
        return result.stdout.decode().strip() if result.returncode == 0 else None
    return {"commit": git("rev-parse", "HEAD"), "status": git("status", "--porcelain", "--untracked-files=all")}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", required=True, type=Path)
    parser.add_argument("--after", required=True, type=Path)
    parser.add_argument("--corpus", action="append", required=True, help="name=/absolute/path")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--iterations", type=int, default=5)
    parser.add_argument("--warmup", type=int, default=1)
    parser.add_argument("--rss-runs", type=int, default=3)
    parser.add_argument("--budgets", type=int, nargs="+", default=[256])
    args = parser.parse_args()
    corpora = [(name, Path(path).resolve()) for name, path in (value.split("=", 1) for value in args.corpus)]
    output = args.output.resolve()
    if output.exists() or any(output.is_relative_to(root) for _, root in corpora):
        parser.error("output must be new and outside every corpus")
    output.mkdir(parents=True)
    versions = {"before": (args.before.resolve(), [])}
    versions.update({f"budget_{mib}": (args.after.resolve(), ["--build-memory-mib", str(mib)]) for mib in args.budgets})
    result = {
        "recorded_at": datetime.now(timezone.utc).isoformat(),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "harness_sha256": digest(Path(__file__)),
        "versions": {name: {"binary": str(binary), "sha256": digest(binary), "args": flags} for name, (binary, flags) in versions.items()},
        "method": {"warmup": args.warmup, "timing": args.iterations, "rss": args.rss_runs, "seed": 20260908, "cache": "warm filesystem cache; fresh index each run"},
        "corpora": {name: {"root": str(root), "before": git_state(root)} for name, root in corpora},
        "runs": [],
    }
    rng = random.Random(20260908)
    reference = {}
    # These queries exercise short fallback, long chunk-spanning literals,
    # alternation, and case folding. Complete outputs are compared to before.
    queries = [["-F", "def"], ["-F", "unique_boundary_marker_0123456789"], ["-i", "AsyncMock|cuda"], [r"@(?:torch|pytest)\.\w+"], ["-F", "x"]]

    def save():
        (output / "results.json").write_text(json.dumps(result, indent=2) + "\n")

    for corpus, root in corpora:
        for kind, count in [("warmup", args.warmup), ("timing", args.iterations), ("rss", args.rss_runs)]:
            for repeat in range(count):
                order = list(versions)
                rng.shuffle(order)
                for version in order:
                    binary, flags = versions[version]
                    stem = f"{corpus}-{version}-{kind}-{repeat + 1}"
                    with tempfile.TemporaryDirectory(prefix="index-", dir=output) as temporary:
                        index = Path(temporary) / "index"
                        command = [str(binary), "index", str(root), "--index-dir", str(index), *flags]
                        if kind == "rss":
                            command = ["/usr/bin/time", "-l", *command]
                        started = time.perf_counter_ns()
                        proc = subprocess.run(command, capture_output=True)
                        elapsed_ms = (time.perf_counter_ns() - started) / 1e6
                        stderr = proc.stderr.decode(errors="replace")
                        (output / f"{stem}.stderr.txt").write_text(stderr)
                        if proc.returncode:
                            raise RuntimeError(f"{stem}: {stderr}")
                        manifest = json.loads((index / "manifest.json").read_bytes())
                        segments = {
                            field: {"bytes": (index / manifest["segments"][0][field]).stat().st_size, "sha256": digest(index / manifest["segments"][0][field])}
                            for field in ["lookup", "postings"]
                        }
                        documents = [(doc["path"], doc["len"], doc["searchable"], doc["active"]) for doc in manifest["documents"]]
                        fingerprint = {"segments": segments, "documents": hashlib.sha256(json.dumps(documents).encode()).hexdigest()}
                        if corpus not in reference:
                            reference[corpus] = fingerprint
                        assert fingerprint == reference[corpus], f"index parity failed: {stem}"
                        assert not list(index.glob(".build-*")) and not list((index / "segments").glob(".write-*")), stem
                        run = {"corpus": corpus, "version": version, "kind": kind, "repeat": repeat + 1, "command": command, "elapsed_ms": elapsed_ms,
                               "segments": segments, "documents_sha256": fingerprint["documents"], "searchable_files": sum(doc[2] for doc in documents),
                               "source_bytes": sum(doc[1] for doc in documents if doc[2]), "index_bytes": sum(path.stat().st_size for path in index.rglob("*") if path.is_file())}
                        spill = re.search(r"spilled (\d+) build runs \((\d+) temporary bytes written, (\d+) peak temporary bytes\)", stderr)
                        run["spilled_runs"] = int(spill[1]) if spill else 0
                        run["temporary_bytes_written"] = int(spill[2]) if spill else 0
                        run["peak_temporary_bytes"] = int(spill[3]) if spill else 0
                        if kind == "rss":
                            match = re.search(r"(\d+)\s+maximum resident set size", stderr)
                            if match is None:
                                raise RuntimeError(f"missing peak RSS: {stem}")
                            run["rss_bytes"] = int(match[1])
                        # Validate search once per version/corpus, outside measured time.
                        if not any(item["corpus"] == corpus and item["version"] == version for item in result["runs"]):
                            outputs = []
                            for query in queries:
                                search = subprocess.run([str(binary), "search", *query, str(root), "--index-dir", str(index), "--no-refresh"], capture_output=True)
                                assert search.returncode in (0, 1), search.stderr
                                outputs.append({"args": query, "exit_code": search.returncode, "bytes": len(search.stdout), "sha256": hashlib.sha256(search.stdout).hexdigest()})
                            prior = next((item["search_parity"] for item in result["runs"] if item["corpus"] == corpus and "search_parity" in item), outputs)
                            assert outputs == prior, f"search parity failed: {stem}"
                            run["search_parity"] = outputs
                        result["runs"].append(run)
                        save()
                        print(f"{stem}: {elapsed_ms:.1f} ms" + (f", {run['rss_bytes'] / 2**20:.2f} MiB RSS" if "rss_bytes" in run else "") + f", {run['spilled_runs']} spills", flush=True)
        state = git_state(root)
        assert state == result["corpora"][corpus]["before"], f"corpus changed: {corpus}"
        result["corpora"][corpus]["after"] = state
    result["summary"] = {}
    for corpus, _ in corpora:
        result["summary"][corpus] = {}
        for version in versions:
            runs = [run for run in result["runs"] if run["corpus"] == corpus and run["version"] == version]
            timing = [run["elapsed_ms"] for run in runs if run["kind"] == "timing"]
            rss = [run["rss_bytes"] / 2**20 for run in runs if run["kind"] == "rss"]
            result["summary"][corpus][version] = {
                "median_ms": statistics.median(timing) if timing else None,
                "median_rss_mib": statistics.median(rss) if rss else None,
                "max_rss_mib": max(rss) if rss else None,
                "spilled_runs": [run["spilled_runs"] for run in runs],
                "temporary_bytes_written": [run["temporary_bytes_written"] for run in runs],
            }
    save()
    print(json.dumps(result["summary"], indent=2))


if __name__ == "__main__":
    main()
