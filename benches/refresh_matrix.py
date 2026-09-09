#!/usr/bin/env python3
"""Compare two coderg builds and rg across frozen corpora and query families."""

from manifest_helpers import read_manifest

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
import sys
import time

sys.dont_write_bytecode = True
from regex_suite import QUERIES as VLLM_QUERIES

MODES = ("baseline", "optimized", "baseline_no_refresh", "optimized_no_refresh", "rg")


def queries_for(kind):
    if kind == "vllm":
        queries = list(VLLM_QUERIES)
        extra = [
            ("asyncmock", r"\basyncmock\b"),
            ("cache", r"\b(kv_cache|block_size)\b"),
            ("class_attention", r"class[ \t]+\w*Attention\b"),
            ("def_forward", r"def[ \t]+forward\b"),
            ("config", r"\bmax_(model_len|num_seqs)\b"),
            ("exceptions", r"\b(ValueError|RuntimeError|NotImplementedError)\b"),
            ("tensor_creation", r"\b(torch|numpy)\.(empty|zeros|ones)\b"),
            ("async_def", r"async[ \t]+def[ \t]+\w+"),
        ]
        queries += [("wide_icase_" + name, "ignore case", pattern, ["-i"]) for name, pattern in extra]
        return queries
    if kind == "synthetic":
        return [
            ("rare_files", "selective / files", "CODERG_BENCHMARK_NEEDLE", ["-F", "-l"]),
            ("rare_lines", "selective / lines", "CODERG_BENCHMARK_NEEDLE", ["-F"]),
            ("common_files", "common / files", "ordinary_value", ["-F", "-l"]),
            ("common_count", "common / count", "ordinary_value", ["-F", "-c"]),
            ("alternation", "alternation", r"\b(module_7|module_23)\b", ["-l"]),
            ("ignore_case", "ignore case", "coderg_benchmark_needle", ["-F", "-i", "-l"]),
            ("short_files", "scan fallback", "fn", ["-F", "-l"]),
            ("absent", "no matches", "CODERG_WIDE_ABSENT_92be7", ["-F", "-l"]),
        ]
    common = [
        ("word_return", "common word", r"\breturn\b", []),
        ("errors", "alternation", r"\b(Error|Result|Exception|ValueError)\b", []),
        ("todo", "ignore case", r"\b(TODO|FIXME)\b", ["-i"]),
        ("short", "scan fallback", "if", ["-F", "-l"]),
        ("digits", "scan fallback", r"\b[0-9]{2,4}\b", ["-c"]),
        ("absent", "no matches", "CODERG_WIDE_ABSENT_92be7", ["-F"]),
    ]
    if kind == "rust":
        return common + [
            ("literal", "fixed string", "Transcription", ["-F"]),
            ("icase_literal", "ignore case", "transcription", ["-i", "-F"]),
            ("function", "function definitions", r"\bfn[ \t]+[a-z_][a-z_0-9]*", []),
            ("public", "optional group", r"\bpub(?:\([^\n)]*\))?[ \t]+(?:struct|enum|fn)\b", []),
            ("impl", "anchor", r"^impl(?:<[^\n>]+>)?[ \t]+", []),
            ("use", "imports", r"^use[ \t]+(?:std|crate)::", []),
            ("derive", "escaped punctuation", r"#\[derive\(", []),
            ("unwrap", "method call", r"\.(?:unwrap|expect)\(", []),
            ("async", "whitespace", r"\basync[ \t]+fn[ \t]+\w+", []),
            ("common_files", "common / files", "self", ["-F", "-l"]),
        ]
    if kind == "python":
        return common + [
            ("literal", "fixed string", "review_batch", ["-F"]),
            ("icase_literal", "ignore case", "review_batch", ["-i", "-F"]),
            ("function", "function definitions", r"\bdef[ \t]+[a-z_][a-z_0-9]*", []),
            ("class", "class definitions", r"^class[ \t]+\w+", []),
            ("import", "imports", r"^from[ \t]+[\w.]+[ \t]+import\b", []),
            ("decorator", "escaped punctuation", r"^[ \t]*@[A-Za-z_][\w.]*", []),
            ("async", "whitespace", r"\basync[ \t]+def[ \t]+\w+", []),
            ("db", "alternation", r"\b(session|transaction|connection)\b", ["-i"]),
            ("test", "test definitions", r"\bdef[ \t]+test_\w+", []),
            ("common_files", "common / files", "self", ["-F", "-l"]),
        ]
    raise ValueError(kind)


def digest(path):
    with path.open("rb") as file:
        return hashlib.file_digest(file, "sha256").hexdigest()


def summary(samples):
    ordered = sorted(samples)
    return dict(samples=samples, min=ordered[0], median=statistics.median(ordered),
                p95=ordered[math.ceil(len(ordered) * .95) - 1], mean=statistics.mean(ordered))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("plan", "baseline", "binary", "output"):
        parser.add_argument("--" + name, required=True, type=Path)
    parser.add_argument("--iterations", type=int, default=31)
    parser.add_argument("--warmup", type=int, default=3)
    parser.add_argument("--build-iterations", type=int, default=5)
    parser.add_argument("--rss-runs", type=int, default=3)
    parser.add_argument("--seed", type=int, default=20260908)
    args = parser.parse_args()
    if min(args.iterations, args.build_iterations) < 1 or min(args.warmup, args.rss_runs) < 0:
        parser.error("invalid sample counts")
    if args.rss_runs and platform.system() != "Darwin":
        parser.error("RSS measurement currently requires macOS")
    repo = Path(__file__).resolve().parent.parent
    plan = json.loads(args.plan.read_text())
    binaries = {"baseline": args.baseline.resolve(), "optimized": args.binary.resolve()}
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    rg = shutil.which("rg")
    if not rg:
        parser.error("rg is required")
    env = dict(os.environ)
    env.pop("RIPGREP_CONFIG_PATH", None)
    env["GIT_OPTIONAL_LOCKS"] = "0"
    rng = random.Random(args.seed)

    def run(command, root, *, capture=False, rss_log=None):
        cmd = list(map(str, command))
        if rss_log is not None:
            cmd = ["/usr/bin/time", "-l", *cmd]
        started = time.perf_counter_ns()
        proc = subprocess.run(cmd, cwd=root, env=env, stderr=subprocess.PIPE,
                              stdout=subprocess.PIPE if capture else subprocess.DEVNULL)
        elapsed = (time.perf_counter_ns() - started) / 1e6
        if proc.returncode not in (0, 1):
            raise RuntimeError((cmd, proc.returncode, proc.stderr.decode(errors="replace")))
        rss = None
        if rss_log is not None:
            rss_log.write_bytes(proc.stderr)
            match = re.search(rb"(\d+)\s+maximum resident set size", proc.stderr)
            if not match:
                raise RuntimeError(proc.stderr.decode(errors="replace"))
            rss = int(match[1])
        return elapsed, rss, proc

    def git_state(root):
        def git(*cmd):
            _, _, proc = run(["git", *cmd], root, capture=True)
            assert proc.returncode == 0
            return proc.stdout.decode().strip()
        return dict(commit=git("rev-parse", "HEAD"), status=git("status", "--porcelain", "--untracked-files=all"))

    source_hashes = {str(p.relative_to(repo)): digest(p) for p in
                     [repo / "Cargo.toml", repo / "Cargo.lock", *sorted((repo / "src").glob("*.rs"))]}
    binary_hashes = {name: digest(path) for name, path in binaries.items()}
    metadata = dict(recorded_at=datetime.now(timezone.utc).isoformat(), plan=plan,
                    platform=platform.platform(), logical_cpus=os.cpu_count(),
                    binaries={name: dict(path=str(path), sha256=binary_hashes[name]) for name, path in binaries.items()},
                    source_sha256=source_hashes, harness_sha256=digest(Path(__file__)),
                    environment={key: env.get(key) for key in ("RAYON_NUM_THREADS", "LANG", "LC_ALL")},
                    iterations=args.iterations, warmup=args.warmup, build_iterations=args.build_iterations,
                    rss_runs=args.rss_runs, seed=args.seed)
    for key, command in [("rustc", ["rustc", "--version"]), ("rg", [rg, "--version"])]:
        metadata[key] = run(command, repo, capture=True)[2].stdout.decode().strip()
    if platform.system() == "Darwin":
        metadata["cpu"] = run(["sysctl", "-n", "machdep.cpu.brand_string"], repo, capture=True)[2].stdout.decode().strip()
        metadata["memory_bytes"] = int(run(["sysctl", "-n", "hw.memsize"], repo, capture=True)[2].stdout)
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")

    for dataset in plan["datasets"]:
        name, root = dataset["id"], Path(dataset["root"])
        if output.is_relative_to(root):
            raise ValueError("output must be outside the corpus")
        directory = output / name
        directory.mkdir()
        logs = directory / "rss"
        logs.mkdir()
        before = git_state(root)
        assert not before["status"], before
        result = dict(dataset=dataset, before=before, builds={}, queries=[])

        def save():
            (directory / "results.json").write_text(json.dumps(result, indent=2) + "\n")

        indexes = {version: directory / (version + "-index") for version in binaries}
        for version, binary in binaries.items():
            elapsed, _, proc = run([binary, "index", root, "--index-dir", indexes[version]], root)
            assert proc.returncode == 0
            manifest = read_manifest(binary, root, indexes[version])
            docs = [d for d in manifest["documents"] if d["active"] and d["searchable"]]
            segment_hashes = sorted(digest(p) for p in (indexes[version] / "segments").iterdir())
            result["builds"][version] = dict(initial_ms=elapsed, files=len(docs),
                source_bytes=sum(d["len"] for d in docs),
                index_bytes=sum(p.stat().st_size for p in indexes[version].rglob("*") if p.is_file()),
                segment_sha256=segment_hashes, times_ms=[], rss_bytes=[])
        assert result["builds"]["baseline"]["segment_sha256"] == result["builds"]["optimized"]["segment_sha256"]
        for kind, count in (("timing", args.build_iterations), ("rss", args.rss_runs)):
            for repeat in range(count):
                order = list(binaries)
                rng.shuffle(order)
                for version in order:
                    temporary_index = directory / "build-scratch"
                    rss_log = logs / f"build-{version}-{repeat}.txt" if kind == "rss" else None
                    elapsed, rss, proc = run([binaries[version], "index", root, "--index-dir", temporary_index], root, rss_log=rss_log)
                    assert proc.returncode == 0
                    result["builds"][version]["rss_bytes" if kind == "rss" else "times_ms"].append(rss if kind == "rss" else elapsed)
                    shutil.rmtree(temporary_index)
        for data in result["builds"].values():
            data["timing"] = summary(data.pop("times_ms"))
            data["rss"] = summary(data.pop("rss_bytes")) if args.rss_runs else None
        print(f"{name}: build comparison complete ({result['builds']['baseline']['files']} files)", flush=True)
        index = indexes["baseline"]
        index_hashes = {str(p.relative_to(index)): digest(p) for p in index.rglob("*") if p.is_file()}
        commands = {}
        for query_id, category, pattern, flags in queries_for(dataset["kind"]):
            commands[query_id] = {}
            for mode in MODES:
                if mode == "rg":
                    cmd = [rg, "--no-config", "-n", "-H", "--hidden", "--glob", "!.git/**", "--glob", "!.coderg-index/**",
                           "--color", "never", "--no-heading", *flags, pattern, "."]
                else:
                    version = mode.removesuffix("_no_refresh")
                    cmd = [str(binaries[version]), "search", *flags, pattern, ".", "--index-dir", str(index)]
                    if mode.endswith("_no_refresh"):
                        cmd += ["--no-refresh"]
                commands[query_id][mode] = cmd
            outputs, parity = {}, {}
            for mode in MODES:
                _, _, proc = run(commands[query_id][mode], root, capture=True)
                assert not proc.stderr, proc.stderr
                lines = sorted(line.removeprefix(b"./") for line in proc.stdout.splitlines())
                outputs[mode] = (proc.returncode, lines)
                parity[mode] = dict(exit_code=proc.returncode, lines=len(lines),
                                    sha256=hashlib.sha256(b"\n".join(lines)).hexdigest())
            assert all(outputs[mode] == outputs["baseline"] for mode in MODES[:-1]), (name, query_id)
            actual, expected = Counter(outputs["baseline"][1]), Counter(outputs["rg"][1])
            equal = outputs["baseline"] == outputs["rg"]
            result["queries"].append(dict(id=query_id, category=category, pattern=pattern, flags=flags,
                parity=parity, rg_compatible=equal, extra_lines=sum((actual - expected).values()),
                missing_lines=sum((expected - actual).values()), timings_ms={mode: [] for mode in MODES},
                rss_bytes={mode: [] for mode in MODES}))
        result["commands"] = commands
        save()
        print(f"{name}: baseline parity {len(result['queries'])}/{len(result['queries'])}; rg parity "
              f"{sum(q['rg_compatible'] for q in result['queries'])}/{len(result['queries'])}", flush=True)
        jobs = [(q, mode) for q in result["queries"] for mode in MODES]
        for repeat in range(args.warmup + args.iterations):
            rng.shuffle(jobs)
            for query, mode in jobs:
                elapsed, _, proc = run(commands[query["id"]][mode], root)
                assert not proc.stderr, proc.stderr
                assert proc.returncode == query["parity"][mode]["exit_code"]
                if repeat >= args.warmup:
                    query["timings_ms"][mode].append(elapsed)
            save()
            if repeat >= args.warmup and ((repeat - args.warmup + 1) % 5 == 0 or repeat + 1 == args.warmup + args.iterations):
                print(f"{name}: timed {repeat - args.warmup + 1}/{args.iterations} rounds", flush=True)
        for repeat in range(args.rss_runs):
            rng.shuffle(jobs)
            for query, mode in jobs:
                _, rss, proc = run(commands[query["id"]][mode], root,
                                   rss_log=logs / f"{query['id']}-{mode}-{repeat}.txt")
                assert proc.returncode == query["parity"][mode]["exit_code"]
                assert b"coderg:" not in proc.stderr
                query["rss_bytes"][mode].append(rss)
        for query in result["queries"]:
            query["timings_ms"] = {mode: summary(values) for mode, values in query["timings_ms"].items()}
            query["rss_bytes"] = {mode: summary(values) if values else None for mode, values in query["rss_bytes"].items()}
        result["after"] = git_state(root)
        assert result["after"] == before
        assert index_hashes == {str(p.relative_to(index)): digest(p) for p in index.rglob("*") if p.is_file()}
        assert binary_hashes == {version: digest(path) for version, path in binaries.items()}
        assert all(digest(repo / path) == value for path, value in source_hashes.items())
        result["validation"] = dict(corpus_unchanged=True, index_unchanged=True, binaries_unchanged=True, source_unchanged=True)
        save()
        # Retain measurements and checksums; generated indexes are disposable.
        for path in indexes.values():
            shutil.rmtree(path)
        print(f"{name}: complete", flush=True)


if __name__ == "__main__":
    main()
