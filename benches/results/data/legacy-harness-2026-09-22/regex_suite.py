#!/usr/bin/env python3
"""Compare full-line regex search against rg on an existing, clean Git corpus."""

import argparse
from collections import Counter
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import random
import re
import shutil
import statistics
import subprocess
import time


# Same patterns and flags go to both executables; no multiline/PCRE-only syntax.
QUERIES = [
    ("literal_parallel", "fixed string", "get_tensor_model_parallel_world_size", ["-F"]),
    ("word_sampling", "word boundary", r"\bSamplingParams\b", []),
    ("word_asyncmock", "word boundary", r"\bAsyncMock\b", []),
    ("def_forward", "whitespace", r"def\s+forward\b", []),
    ("async_def", "whitespace", r"async\s+def\s+\w+", []),
    ("class_attention", "anchor / repetition", r"^class[ \t]+\w*Attention\b", []),
    ("python_import", "anchor / optional group", r"^from[ \t]+vllm(?:\.[\w.]+)?[ \t]+import\b", []),
    ("decorator", "anchor / character class", r"^[ \t]*@(?:torch|pytest)\.[A-Za-z_]+", []),
    ("logger", "group / alternation", r"logger\.(warning|error)\(", []),
    ("exceptions", "group / alternation", r"\b(ValueError|RuntimeError|NotImplementedError)\b", []),
    ("cache", "group / alternation", r"\b(kv_cache|block_size)\b", []),
    ("tensor_creation", "two alternative groups", r"\b(torch|numpy)\.(empty|zeros|ones)\b", []),
    ("env_vars", "character class / repetition", r"\b(VLLM|CUDA)_[A-Z0-9_]+\b", []),
    ("cuda_call", "escaped punctuation", r"\btorch\.cuda\.[a-z_]+\(", []),
    ("config_assignment", "optional whitespace", r"\bmax_(model_len|num_seqs)[ \t]*=", []),
    ("raise_error", "wildcard", r"raise[ \t]+\w*Error\(", []),
    ("todo_comment", "anchor / wildcard", r"^[ \t]*#.*\b(TODO|FIXME)\b", []),
    ("return_none", "both anchors", r"^[ \t]*return[ \t]+None[ \t]*$", []),
    ("quoted_device", "quote character class", r"[\"'](?:cuda|cpu|rocm)[\"']", []),
    ("icase_sampling", "ignore case", r"\bSamplingParams\b", ["-i"]),
    ("icase_platform", "ignore case / alternation", r"\b(cuda|rocm)\b", ["-i"]),
    ("icase_todo", "ignore case / alternation", r"\b(TODO|FIXME)\b", ["-i"]),
    ("icase_env", "ignore case / character class", r"\b(VLLM|CUDA)_[A-Z0-9_]+\b", ["-i"]),
    ("icase_logger", "ignore case / escaped punctuation", r"logger\.(warning|error)\(", ["-i"]),
    ("icase_literal", "ignore case / fixed string", "get_tensor_model_parallel_world_size", ["-i", "-F"]),
    ("digits", "scan fallback / bounded repetition", r"\b[0-9]{2,4}\b", []),
    ("hex_number", "hexadecimal / short prefix", r"\b0x[0-9A-Fa-f]+\b", []),
    ("short_word", "scan fallback / short literal", r"\bif\b", []),
    ("blank_lines", "scan fallback / empty match", r"^[ \t]*$", []),
    ("absent_regex", "no matches", r"\bCODERG_BENCH_ABSENT_[0-9]{8}\b", []),
]
MODES = ("coderg_default", "coderg_no_refresh", "rg")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def summary(samples):
    ordered = sorted(samples)
    return dict(min_ms=ordered[0], median_ms=statistics.median(ordered),
                p95_ms=ordered[math.ceil(len(ordered) * .95) - 1],
                mean_ms=statistics.mean(ordered), samples_ms=samples)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--binary", type=Path, default=Path("target/release/coderg"))
    parser.add_argument("--index-dir", required=True, type=Path, help="Must not exist; outside corpus")
    parser.add_argument("--output", required=True, type=Path, help="Must not exist; outside corpus")
    parser.add_argument("--iterations", type=int, default=31)
    parser.add_argument("--warmup", type=int, default=3)
    parser.add_argument("--rss-runs", type=int, default=0, help="macOS /usr/bin/time -l samples")
    parser.add_argument("--seed", type=int, default=20260907)
    args = parser.parse_args()
    if args.iterations < 1 or args.warmup < 0 or args.rss_runs < 0:
        parser.error("iterations must be positive; warmup and RSS runs must be nonnegative")
    if args.rss_runs and platform.system() != "Darwin":
        parser.error("--rss-runs currently requires macOS")
    root, binary = args.root.resolve(), args.binary.resolve()
    index, output = args.index_dir.resolve(), args.output.resolve()
    if index.is_relative_to(root) or output.is_relative_to(root):
        parser.error("index and output must be outside the corpus")
    if index.exists() or output.exists():
        parser.error("index and output must be new directories")
    rg = shutil.which("rg")
    if rg is None or not binary.is_file():
        parser.error("build coderg and install rg first")
    env = os.environ.copy()
    for key in ("RIPGREP_CONFIG_PATH", "CODERG_BENCH_VARIANTS", "CODERG_BENCH_KEYS", "CODERG_BENCH_TRACE"):
        env.pop(key, None)

    def run(command, *, capture=False, cwd=root, allowed=(0, 1)):
        started = time.perf_counter_ns()
        result = subprocess.run(list(map(str, command)), cwd=cwd, env=env,
                                stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                                stderr=subprocess.PIPE)
        elapsed = (time.perf_counter_ns() - started) / 1e6
        if result.returncode not in allowed:
            raise RuntimeError(f"{command}: {result.returncode}: {result.stderr.decode(errors='replace')}")
        return elapsed, result

    def text(command, cwd=root):
        return run(command, capture=True, cwd=cwd, allowed=(0,))[1].stdout.decode().strip()

    def corpus_state():
        return {"commit": text(["git", "rev-parse", "HEAD"]),
                "status": text(["git", "status", "--porcelain", "--untracked-files=all"])}

    before = corpus_state()
    if before["status"]:
        parser.error("corpus must be a clean Git checkout")
    output.mkdir(parents=True)
    repo = Path(__file__).resolve().parent.parent
    source_paths = [repo / "Cargo.toml", repo / "Cargo.lock", *sorted((repo / "src").glob("*.rs"))]
    source_hashes = {str(p.relative_to(repo)): digest(p) for p in source_paths}
    diff = run(["git", "diff", "HEAD", "--", "Cargo.toml", "Cargo.lock", "src"],
               capture=True, cwd=repo, allowed=(0,))[1].stdout
    (output / "source.diff").write_bytes(diff)
    metadata = dict(recorded_at=datetime.now(timezone.utc).isoformat(), root=str(root),
                    binary=str(binary), binary_sha256=digest(binary), rg=rg,
                    coderg_commit=text(["git", "rev-parse", "HEAD"], cwd=repo),
                    source_sha256=source_hashes, source_diff_sha256=digest(output / "source.diff"),
                    harness_sha256=digest(Path(__file__)), corpus_before=before,
                    platform=platform.platform(), logical_cpus=os.cpu_count(),
                    rustc=text(["rustc", "--version"], cwd=repo), rg_version=text([rg, "--version"]),
                    iterations=args.iterations, warmup=args.warmup, rss_runs=args.rss_runs,
                    seed=args.seed, index_dir=str(index),
                    environment={k: env.get(k) for k in ("RAYON_NUM_THREADS", "LC_ALL", "LANG")})
    if platform.system() == "Darwin":
        metadata["cpu"] = text(["sysctl", "-n", "machdep.cpu.brand_string"])
        metadata["memory_bytes"] = int(text(["sysctl", "-n", "hw.memsize"]))
    build_command = [str(binary), "index", ".", "--index-dir", str(index)]
    build_ms, built = run(build_command, capture=True, allowed=(0,))
    metadata["index_build"] = dict(command=build_command, elapsed_ms=build_ms,
                                   stdout=built.stdout.decode(), stderr=built.stderr.decode())
    metadata["index_stats"] = text([binary, "stats", ".", "--index-dir", index])
    print(f"Index built in {build_ms / 1000:.3f}s\n{metadata['index_stats']}", flush=True)
    index_hashes = {str(p.relative_to(index)): digest(p) for p in sorted(index.rglob("*")) if p.is_file()}
    results = dict(metadata=metadata, index_sha256=index_hashes, queries=[])

    def save():
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")

    def normalized(command):
        _, result = run(command, capture=True)
        lines = sorted(line.removeprefix(b"./") for line in result.stdout.splitlines())
        return lines, result.returncode

    commands = {}
    for query_id, category, pattern, flags in QUERIES:
        common = [str(binary), "search", *flags, pattern, ".", "--index-dir", str(index)]
        commands[query_id] = {"coderg_default": common, "coderg_no_refresh": [*common, "--no-refresh"],
                              "rg": [rg, "--no-config", *flags, "-n", "-H", "--hidden", "--glob", "!.git/**",
                                     "--glob", "!.coderg-index/**", "--color", "never", "--no-heading", pattern, "."]}
        expected, expected_exit = normalized(commands[query_id]["rg"])
        expected_counter = Counter(expected)
        parity = {}
        for mode in MODES[:2]:
            actual, actual_exit = normalized(commands[query_id][mode])
            extra = Counter(actual) - expected_counter
            missing = expected_counter - Counter(actual)
            parity[mode] = dict(equal=actual == expected and actual_exit == expected_exit,
                                exit_code=actual_exit, lines=len(actual),
                                sha256=hashlib.sha256(b"\n".join(actual)).hexdigest(),
                                extra_lines=sum(extra.values()), missing_lines=sum(missing.values()),
                                extra_examples=[x.decode(errors="replace") for x in sorted(extra)[:5]],
                                missing_examples=[x.decode(errors="replace") for x in sorted(missing)[:5]])
        query = dict(id=query_id, category=category, pattern=pattern, flags=flags,
                     rg=dict(lines=len(expected), files=len({line.split(b":", 1)[0] for line in expected}),
                             exit_code=expected_exit, sha256=hashlib.sha256(b"\n".join(expected)).hexdigest()),
                     parity=parity, eligible=all(p["equal"] for p in parity.values()),
                     samples={mode: [] for mode in MODES})
        results["queries"].append(query)
        print(f"Parity {query_id}: {'PASS' if query['eligible'] else 'FAIL'} ({len(expected)} rg lines)", flush=True)
        save()
    (output / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")

    # A fresh shuffle per round distributes query and executable order effects.
    rng = random.Random(args.seed)
    eligible = [q for q in results["queries"] if q["eligible"]]
    jobs = [(query, mode) for query in eligible for mode in MODES]
    for round_index in range(args.warmup + args.iterations):
        rng.shuffle(jobs)
        for query, mode in jobs:
            elapsed, result = run(commands[query["id"]][mode])
            if result.returncode != query["rg"]["exit_code"]:
                raise RuntimeError(f"exit code changed during timing: {query['id']} {mode}")
            if result.stderr:
                raise RuntimeError(f"unexpected timing stderr: {result.stderr.decode(errors='replace')}")
            if round_index >= args.warmup:
                query["samples"][mode].append(elapsed)
        completed = round_index + 1 - args.warmup
        print(f"{'Warmup' if completed <= 0 else 'Timed'} round {round_index + 1}/{args.warmup + args.iterations}", flush=True)
        save()
    for query in eligible:
        query["timings"] = {mode: summary(samples) for mode, samples in query.pop("samples").items()}
        query["speedup"] = {mode: query["timings"]["rg"]["median_ms"] / query["timings"][mode]["median_ms"]
                            for mode in MODES[:2]}
        if args.rss_runs:
            query["rss_bytes"] = {}
            for mode in MODES:
                samples = []
                for repeat in range(args.rss_runs):
                    _, result = run(["/usr/bin/time", "-l", *commands[query["id"]][mode]])
                    (output / f"{query['id']}-{mode}-rss-{repeat + 1}.txt").write_bytes(result.stderr)
                    match = re.search(rb"(\d+)\s+maximum resident set size", result.stderr)
                    if match is None:
                        raise RuntimeError("cannot parse macOS peak RSS")
                    samples.append(int(match.group(1)))
                query["rss_bytes"][mode] = dict(median=statistics.median(samples), samples=samples)
    after = corpus_state()
    results["metadata"]["corpus_after"] = after
    results["metadata"]["corpus_unchanged"] = before == after
    results["metadata"]["binary_unchanged"] = digest(binary) == metadata["binary_sha256"]
    results["metadata"]["source_unchanged"] = all(digest(repo / p) == h for p, h in source_hashes.items())
    after_index = {str(p.relative_to(index)): digest(p) for p in sorted(index.rglob("*")) if p.is_file()}
    results["metadata"]["index_unchanged"] = after_index == index_hashes
    for mode in MODES:
        results.setdefault("aggregate", {})[mode] = dict(
            mean_query_median_ms=statistics.mean(q["timings"][mode]["median_ms"] for q in eligible),
            wins_vs_rg=sum(q["speedup"][mode] > 1 for q in eligible) if mode != "rg" else None)
    save()
    assert all(results["metadata"][k] for k in ("corpus_unchanged", "binary_unchanged", "source_unchanged", "index_unchanged")), "inputs changed"
    print(json.dumps(results["aggregate"], indent=2), flush=True)
    print(f"Passed {len(eligible)}/{len(QUERIES)} queries. Evidence: {output}", flush=True)


if __name__ == "__main__":
    main()
