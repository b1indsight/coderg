#!/usr/bin/env python3
"""Measure unchanged search latency with 2..10 Rayon workers, serial CLI calls."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import subprocess
import tempfile
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("root", "binary", "workspace", "output"):
        parser.add_argument(f"--{name}", type=Path, required=True)
    parser.add_argument("--rounds", type=int, default=5)
    parser.add_argument("--samples", type=int, default=10)
    args = parser.parse_args()
    binary = args.binary.resolve()
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    queries = [("ABSENT_SNAPSHOT_TOKEN", True), ("SamplingParams", True),
               ("import ", True), ("input_ids|hidden_states|positions", False)]
    rows, verified = [], 0
    rng = random.Random(20260920)

    def run(command, cwd=None, threads=4, search=False):
        start = time.perf_counter_ns()
        result = subprocess.run(command, cwd=cwd, capture_output=True,
                                env={**os.environ, "RAYON_NUM_THREADS": str(threads)})
        elapsed = (time.perf_counter_ns() - start) / 1e6
        if result.returncode not in ([0, 1] if search else [0]):
            raise RuntimeError(result.stderr.decode(errors="replace"))
        return result, elapsed

    revision = run(["git", "-C", str(args.root.resolve()), "rev-parse", "HEAD"])[0].stdout.decode().strip()
    args.workspace.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="threads-", dir=args.workspace.resolve()) as temp:
        root, index = Path(temp) / "repo", Path(temp) / "index"
        run(["git", "clone", "--quiet", "--shared", "--no-checkout", str(args.root.resolve()), str(root)])
        run(["git", "-C", str(root), "checkout", "--quiet", "--detach", revision])
        run([str(binary), "index", str(root), "--index-dir", str(index)])
        manifest_hash = hashlib.sha256((index / "manifest.bin").read_bytes()).hexdigest()
        stats = json.loads(run([str(binary), "stats", str(root), "--index-dir", str(index), "--json"])[0].stdout)
        expected = {}
        for query, fixed in queries:
            result, _ = run(["rg", "--hidden", "-g", "!.git", "-Fl" if fixed else "-l", "--", query, "."], cwd=root, search=True)
            expected[query] = sorted(p.removeprefix("./") for p in result.stdout.decode().splitlines())
        configurations = [(t, q, fixed, nr) for t in range(2, 11) for q, fixed in queries for nr in (False, True)]
        for round_id in range(args.rounds):
            for sample in range(-2, args.samples):
                order = list(configurations)
                rng.shuffle(order)
                for threads, query, fixed, no_refresh in order:
                    command = [str(binary), "search", "-Fl" if fixed else "-l", query,
                               str(root), "--index-dir", str(index)]
                    if no_refresh:
                        command.append("--no-refresh")
                    result, elapsed = run(command, threads=threads, search=True)
                    assert not result.stderr, result.stderr
                    assert sorted(result.stdout.decode().splitlines()) == expected[query], (threads, query)
                    verified += 1
                    if sample >= 0:
                        rows.append(dict(round=round_id, sample=sample, threads=threads,
                                         query=query, fixed=fixed, no_refresh=no_refresh, elapsed_ms=elapsed))
            print(f"completed round {round_id + 1}/{args.rounds}", flush=True)
        assert hashlib.sha256((index / "manifest.bin").read_bytes()).hexdigest() == manifest_hash
    assert hashlib.sha256(binary.read_bytes()).hexdigest() == digest
    report = dict(commit=revision, binary=str(binary), binary_sha256=digest,
                  platform=platform.platform(), rounds=args.rounds, samples=args.samples,
                  searchable_files=sum(d["active"] and d["searchable"] for d in stats["documents"]),
                  match_counts={q: len(v) for q, v in expected.items()}, verified=verified, rows=rows,
                  notes=["Hot caches, one initial index shared by all thread configurations; no source changes.",
                         "Two warmup sweeps per round, seeded randomized configuration order; serial processes.",
                         "Includes process startup and output; excludes build, git and correctness checks.",
                         "RAYON_NUM_THREADS affects traversal, sorting and matching, not only refresh."])
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
