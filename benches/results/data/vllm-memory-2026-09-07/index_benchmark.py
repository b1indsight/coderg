#!/usr/bin/env python3
"""Measure fresh index build latency, peak RSS, and file sizes on macOS."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import statistics
import subprocess
import tempfile
import time


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root, binary = args.root.resolve(), args.binary.resolve()
    output = args.output.resolve()
    if output.exists() or output.is_relative_to(root):
        parser.error("output must be new and outside the corpus")

    def text(command):
        return subprocess.check_output(command, cwd=root, text=True).strip()

    def corpus_state():
        return dict(commit=text(["git", "rev-parse", "HEAD"]),
                    status=text(["git", "status", "--porcelain", "--untracked-files=all"]))

    before = corpus_state()
    if before["status"]:
        parser.error("corpus must be clean")
    output.mkdir(parents=True)
    results = dict(recorded_at=datetime.now(timezone.utc).isoformat(),
                   root=str(root), binary=str(binary), binary_sha256=digest(binary),
                   harness_sha256=digest(Path(__file__)), corpus_before=before,
                   warmup_runs=1, timing_runs=5, rss_runs=3, runs=[])

    def save():
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")

    for kind, count in (("warmup", 1), ("timing", 5), ("rss", 3)):
        for repeat in range(1, count + 1):
            with tempfile.TemporaryDirectory(prefix="coderg-index-resources-") as temp:
                index = Path(temp) / "index"
                command = [str(binary), "index", ".", "--index-dir", str(index)]
                if kind == "rss":
                    command = ["/usr/bin/time", "-l", *command]
                started = time.perf_counter_ns()
                proc = subprocess.run(command, cwd=root, stdout=subprocess.PIPE,
                                      stderr=subprocess.PIPE, check=True)
                elapsed_ms = (time.perf_counter_ns() - started) / 1e6
                stem = f"{kind}-{repeat}"
                (output / f"{stem}.stderr.txt").write_bytes(proc.stderr)
                stats = text([str(binary), "stats", ".", "--index-dir", str(index)])
                files = {str(p.relative_to(index)): dict(bytes=p.stat().st_size,
                         allocated_bytes=p.stat().st_blocks * 512, sha256=digest(p))
                         for p in sorted(index.rglob("*")) if p.is_file()}
                run = dict(kind=kind, repeat=repeat, command=command, elapsed_ms=elapsed_ms,
                           stdout=proc.stdout.decode(), stats=stats, files=files,
                           total_file_bytes=sum(f["bytes"] for f in files.values()),
                           allocated_file_bytes=sum(f["allocated_bytes"] for f in files.values()))
                if kind == "rss":
                    match = re.search(rb"(\d+)\s+maximum resident set size", proc.stderr)
                    if match is None:
                        raise RuntimeError("macOS peak RSS missing")
                    run["rss_bytes"] = int(match.group(1))
                results["runs"].append(run)
                save()
                print(f"{stem}: {elapsed_ms / 1000:.3f}s, "
                      f"{run['total_file_bytes'] / 1048576:.2f} MiB index" +
                      (f", {run['rss_bytes'] / 1048576:.2f} MiB RSS" if kind == "rss" else ""),
                      flush=True)

    times = [r["elapsed_ms"] for r in results["runs"] if r["kind"] == "timing"]
    rss = [r["rss_bytes"] for r in results["runs"] if r["kind"] == "rss"]
    results["summary"] = dict(build_median_ms=statistics.median(times),
                              build_min_ms=min(times), build_max_ms=max(times),
                              build_mean_ms=statistics.mean(times),
                              rss_median_bytes=statistics.median(rss), rss_max_bytes=max(rss),
                              index_file_bytes=results["runs"][-1]["total_file_bytes"],
                              allocated_file_bytes=results["runs"][-1]["allocated_file_bytes"])
    results["corpus_after"] = corpus_state()
    results["corpus_unchanged"] = before == results["corpus_after"]
    results["binary_unchanged"] = digest(binary) == results["binary_sha256"]
    save()
    assert results["corpus_unchanged"] and results["binary_unchanged"], "inputs changed"
    print(json.dumps(results["summary"], indent=2), flush=True)


if __name__ == "__main__":
    main()
