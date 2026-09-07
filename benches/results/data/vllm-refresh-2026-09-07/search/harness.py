#!/usr/bin/env python3
"""Compare repeated fresh searches on one unchanged, already-indexed Git tree."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import re
import statistics
import subprocess
import sys
import time

parser = argparse.ArgumentParser(description=__doc__)
for name in ("repo", "root", "baseline", "binary", "index", "output"):
    parser.add_argument("--" + name, type=Path, required=True)
parser.add_argument("--iterations", type=int, default=31)
parser.add_argument("--warmup", type=int, default=3)
args = parser.parse_args()
repo, root, index = args.repo.resolve(), args.root.resolve(), args.index.resolve()
args.output.mkdir(parents=True, exist_ok=False)
env = dict(os.environ)
env.pop("RIPGREP_CONFIG_PATH", None)
env.pop("CODERG_REFRESH_PROFILE", None)
binaries = {"baseline": args.baseline.resolve(), "optimized": args.binary.resolve()}
sys.path.insert(0, str(repo / "benches"))
from regex_suite import QUERIES

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def git(*command):
    return subprocess.check_output(["git", "-C", str(root), *command]).decode().strip()

def index_hashes():
    return {str(p.relative_to(index)): sha(p) for p in sorted(index.rglob("*")) if p.is_file()}

before = dict(commit=git("rev-parse", "HEAD"), status=git("status", "--porcelain", "--untracked-files=all"))
assert not before["status"], before
binary_hashes = {name: sha(path) for name, path in binaries.items()}
sources = {str(p.relative_to(repo)): sha(p) for p in sorted((repo / "src").glob("*.rs"))}
initial_index = index_hashes()
queries = {q[0]: q for q in QUERIES}
selected = ["absent_regex", "literal_parallel", "word_sampling", "icase_sampling"]
cases = [(name, mode, query) for name in binaries for mode in ("default", "no_refresh") for query in selected]

def command(case):
    name, mode, query = case
    _, _, pattern, flags = queries[query]
    result = [str(binaries[name]), "search", *flags, pattern, str(root), "--index-dir", str(index)]
    if mode == "no_refresh":
        result += ["--no-refresh"]
    return result

def run(case, capture=False, rss=False):
    cmd = command(case)
    if rss:
        cmd = ["/usr/bin/time", "-l", *cmd]
    started = time.perf_counter_ns()
    proc = subprocess.run(cmd, stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                          stderr=subprocess.PIPE, env=env, cwd=root)
    elapsed = (time.perf_counter_ns() - started) / 1e6
    assert proc.returncode in (0, 1), (case, proc.stderr.decode())
    assert b"coderg:" not in proc.stderr, (case, proc.stderr.decode())
    if rss:
        match = re.search(rb"(\d+)\s+maximum resident set size", proc.stderr)
        assert match, proc.stderr
        return int(match[1]) / 1048576
    return (proc.returncode, proc.stdout) if capture else elapsed

parity = {}
for query in queries:
    expected = run(("baseline", "no_refresh", query), capture=True)
    for name, mode in (("baseline", "default"), ("optimized", "default"), ("optimized", "no_refresh")):
        assert run((name, mode, query), capture=True) == expected, (name, mode, query)
    parity[query] = dict(exit_code=expected[0], output_sha256=hashlib.sha256(expected[1]).hexdigest())

rng = random.Random(20260907)
timings = {case: [] for case in cases}
rounds = []
for iteration in range(args.warmup + args.iterations):
    order = cases.copy()
    rng.shuffle(order)
    round_values = {}
    for case in order:
        elapsed = run(case)
        if iteration >= args.warmup:
            timings[case].append(elapsed)
            round_values["/".join(case)] = elapsed
    if round_values:
        rounds.append(round_values)

rss = {case: [] for case in cases}
if platform.system() == "Darwin":
    for _ in range(3):
        order = cases.copy()
        rng.shuffle(order)
        for case in order:
            rss[case].append(run(case, rss=True))

after = dict(commit=git("rev-parse", "HEAD"), status=git("status", "--porcelain", "--untracked-files=all"))
assert before == after
assert initial_index == index_hashes()
assert binary_hashes == {name: sha(path) for name, path in binaries.items()}
assert all(sha(repo / path) == digest for path, digest in sources.items())
summary = {"/".join(case): dict(median_ms=statistics.median(values),
           p95_ms=sorted(values)[int(0.95 * len(values))], samples_ms=values,
           rss_samples_mib=rss[case], rss_median_mib=statistics.median(rss[case]) if rss[case] else None)
           for case, values in timings.items()}
result = dict(recorded_at=datetime.now(timezone.utc).isoformat(), platform=platform.platform(),
              logical_cpus=os.cpu_count(), rayon_num_threads=env.get("RAYON_NUM_THREADS"),
              root=str(root), corpus=before, index=str(index), index_hashes=initial_index,
              binaries={name: dict(path=str(path), sha256=binary_hashes[name]) for name, path in binaries.items()},
              source_sha256=sources, harness_sha256=sha(Path(__file__)), seed=20260907,
              iterations=args.iterations, warmup=args.warmup, parity=parity,
              commands={"/".join(case): command(case) for case in cases},
              rounds=rounds, summary=summary,
              validation=dict(corpus_unchanged=True, index_unchanged=True,
                              binaries_unchanged=True, source_unchanged=True))
(args.output / "results.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({case: {k: v for k, v in row.items() if "samples" not in k} for case, row in summary.items()}, indent=2))
