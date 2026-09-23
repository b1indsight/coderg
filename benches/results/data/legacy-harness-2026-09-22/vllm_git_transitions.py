#!/usr/bin/env python3
"""Benchmark Git transitions in a disposable local clone of a clean vLLM tree."""

from manifest_helpers import read_manifest, manifest_path

import argparse
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
import tempfile
import time


PATTERN = r"\b(?:SamplingParams|CODERG_COMMIT_BENCH_(?:SINGLE|BATCH))\b"
PHASES = {
    "clean": "",
    "dirty_single": "incrementally indexed 1 changed files",
    "promote_commit": "advanced index to the current Git tree",
    "committed_batch": "incrementally indexed 32 changed files",
    "empty_commit": "advanced index to the current Git tree",
    "cached_rollback": "switched to cached Git tree index",
    "clean_after_rollback": "",
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def timing_summary(samples):
    ordered = sorted(samples)
    return dict(samples_ms=samples, median_ms=statistics.median(samples),
                min_ms=min(samples), max_ms=max(samples), mean_ms=statistics.mean(samples),
                p95_ms=ordered[math.ceil(len(ordered) * .95) - 1])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--iterations", type=int, default=11)
    parser.add_argument("--warmup", type=int, default=1)
    parser.add_argument("--rss-runs", type=int, default=3)
    args = parser.parse_args()
    source, output = args.root.resolve(), args.output.resolve()
    binaries = dict(baseline=args.baseline.resolve(), optimized=args.binary.resolve())
    if args.iterations < 1 or args.warmup < 0 or args.rss_runs < 0:
        parser.error("invalid sample counts")
    if args.rss_runs and platform.system() != "Darwin":
        parser.error("RSS measurement requires macOS")
    if output.exists() or output.is_relative_to(source):
        parser.error("output must be new and outside the corpus")
    rg = shutil.which("rg")
    if not rg or not all(p.is_file() for p in binaries.values()):
        parser.error("build both binaries and install rg first")
    env = os.environ.copy()
    env.pop("RIPGREP_CONFIG_PATH", None)
    env["GIT_TERMINAL_PROMPT"] = "0"

    def run(command, cwd, capture=True, allowed=(0,)):
        started = time.perf_counter_ns()
        proc = subprocess.run(list(map(str, command)), cwd=cwd, env=env,
                              stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                              stderr=subprocess.PIPE)
        elapsed = (time.perf_counter_ns() - started) / 1e6
        if proc.returncode not in allowed:
            raise RuntimeError(f"{command}: {proc.returncode}: {proc.stderr.decode(errors='replace')}")
        return elapsed, proc

    def text(command, cwd):
        return run(command, cwd)[1].stdout.decode().strip()

    def git(root, *args):
        return text(["git", "-c", "core.hooksPath=/dev/null", "-c", "commit.gpgsign=false", *args], root)

    def state(root):
        return dict(commit=git(root, "rev-parse", "HEAD"),
                    status=git(root, "status", "--porcelain", "--untracked-files=all"))

    before = state(source)
    if before["status"]:
        parser.error("source must be clean")
    output.mkdir(parents=True)
    repo = Path(__file__).resolve().parent.parent
    metadata = dict(recorded_at=datetime.now(timezone.utc).isoformat(), source=str(source),
                    source_before=before, coderg_commit=git(repo, "rev-parse", "HEAD"),
                    binaries={k: dict(path=str(p), sha256=sha(p)) for k, p in binaries.items()},
                    source_sha256={str(p.relative_to(repo)): sha(p) for p in
                                   [repo / "Cargo.toml", repo / "Cargo.lock", *sorted((repo / "src").glob("*.rs"))]},
                    harness_sha256=sha(Path(__file__)), pattern=PATTERN, phases=PHASES,
                    iterations=args.iterations, warmup=args.warmup, rss_runs=args.rss_runs,
                    platform=platform.platform(), logical_cpus=os.cpu_count(),
                    rg_version=text([rg, "--version"], repo),
                    environment={k: env.get(k) for k in ("RAYON_NUM_THREADS", "LC_ALL", "LANG")})
    if platform.system() == "Darwin":
        metadata.update(cpu=text(["sysctl", "-n", "machdep.cpu.brand_string"], repo),
                        memory_bytes=int(text(["sysctl", "-n", "hw.memsize"], repo)))
    results = dict(metadata=metadata, rounds=[])

    def save():
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")

    rng = random.Random(20260907)
    with tempfile.TemporaryDirectory(prefix="coderg-git-transitions-", dir="/private/tmp") as work:
        work = Path(work)
        root = work / "vllm"
        run(["git", "clone", "--quiet", "--no-hardlinks", str(source), str(root)], work)
        git(root, "config", "user.name", "Coderg Benchmark")
        git(root, "config", "user.email", "coderg-benchmark@example.invalid")
        git(root, "checkout", "--quiet", "--detach", before["commit"])
        single = Path("vllm/sampling_params.py")
        batch = [p.relative_to(root) for p in sorted((root / "vllm/config").glob("*.py"))]
        if not (root / single).is_file() or len(batch) < 32:
            raise RuntimeError("expected sampling_params.py and at least 32 config files")
        batch = batch[:32]
        metadata.update(clone=str(root), single_file=str(single), batch_files=list(map(str, batch)),
                        single_source_bytes=(root / single).stat().st_size,
                        batch_source_bytes=sum((root / p).stat().st_size for p in batch))
        rg_command = [rg, "--no-config", "-n", "-H", "--hidden", "--glob", "!.git/**",
                      "--glob", "!.coderg-index/**", "--color", "never", "--no-heading", PATTERN, "."]
        metadata["rg_command"] = rg_command
        kinds = ["warmup"] * args.warmup + ["timing"] * args.iterations + ["rss"] * args.rss_runs
        for number, kind in enumerate(kinds, 1):
            assert state(root) == before
            record = dict(number=number, kind=kind, builds={}, phases={})
            results["rounds"].append(record)
            with tempfile.TemporaryDirectory(prefix="indexes-", dir=work) as indexes_temp:
                indexes = {name: Path(indexes_temp) / name for name in binaries}
                fingerprints = {}

                def snapshot(index):
                    manifest = read_manifest(binaries[index.name], root, index)
                    segments = {}
                    for p in sorted((index / "segments").iterdir()):
                        stat = p.stat()
                        fingerprint = (stat.st_size, stat.st_mtime_ns, stat.st_ino)
                        if p in fingerprints:
                            assert fingerprints[p][0] == fingerprint, "immutable segment changed"
                        else:
                            fingerprints[p] = (fingerprint, sha(p))
                        segments[p.name] = dict(bytes=stat.st_size, sha256=fingerprints[p][1])
                    active = [manifest_path(index)]
                    for segment in manifest["segments"]:
                        active.extend([index / segment["lookup"], index / segment["postings"]])
                    return dict(active_segments=len(manifest["segments"]),
                                active_bytes=sum(p.stat().st_size for p in active),
                                directory_bytes=sum(p.stat().st_size for p in index.rglob("*") if p.is_file()),
                                cached_trees=len({p.stem for p in (index / "manifests").glob("*") if p.suffix in (".json", ".bin")}),
                                segment_files=segments)

                order = list(binaries)
                rng.shuffle(order)
                for name in order:
                    command = [binaries[name], "index", ".", "--index-dir", indexes[name]]
                    elapsed, proc = run(command, root, capture=False)
                    record["builds"][name] = dict(elapsed_ms=elapsed, stderr=proc.stderr.decode(),
                                                   index=snapshot(indexes[name]))
                previous = {name: record["builds"][name]["index"] for name in binaries}

                def phase(phase_name):
                    _, expected = run(rg_command, root, allowed=(0, 1))
                    expected_lines = sorted(line.removeprefix(b"./") for line in expected.stdout.splitlines())
                    entry = dict(commit=git(root, "rev-parse", "HEAD"), modes={})
                    record["phases"][phase_name] = entry
                    order = list(binaries)
                    rng.shuffle(order)
                    for name in order:
                        command = [str(binaries[name]), "search", PATTERN, ".", "--index-dir", str(indexes[name])]
                        measured = ["/usr/bin/time", "-l", *command] if kind == "rss" else command
                        elapsed, proc = run(measured, root, capture=False, allowed=(0, 1))
                        assert proc.returncode == expected.returncode
                        diagnostic = b"\n".join(line for line in proc.stderr.splitlines() if line.startswith(b"coderg:"))
                        if PHASES[phase_name]:
                            assert PHASES[phase_name] in diagnostic.decode(), diagnostic
                        else:
                            assert not diagnostic, diagnostic
                        if kind != "rss":
                            assert proc.stderr.strip() == diagnostic.strip()
                        _, checked = run([*command, "--no-refresh"], root, allowed=(0, 1))
                        actual = sorted(line.removeprefix(b"./") for line in checked.stdout.splitlines())
                        assert actual == expected_lines and checked.returncode == expected.returncode
                        current = snapshot(indexes[name])
                        old = previous[name]
                        added = sorted(set(current["segment_files"]) - set(old["segment_files"]))
                        incremental = phase_name in ("dirty_single", "committed_batch")
                        assert len(added) == (2 if incremental else 0), (phase_name, added)
                        expected_count = {"clean": 1, "dirty_single": 2, "promote_commit": 2,
                                          "committed_batch": 3, "empty_commit": 3,
                                          "cached_rollback": 1, "clean_after_rollback": 1}[phase_name]
                        assert current["active_segments"] == expected_count
                        data = dict(command=command, elapsed_ms=elapsed, diagnostic=diagnostic.decode(),
                                    parity=True, lines=len(actual),
                                    output_sha256=hashlib.sha256(b"\n".join(actual)).hexdigest(),
                                    index=current, new_segment_files=added)
                        if kind == "rss":
                            match = re.search(rb"(\d+)\s+maximum resident set size", proc.stderr)
                            if match is None:
                                raise RuntimeError("macOS peak RSS missing")
                            data["rss_bytes"] = int(match.group(1))
                            (output / f"rss-{number}-{phase_name}-{name}.txt").write_bytes(proc.stderr)
                        entry["modes"][name] = data
                        previous[name] = current
                    # Segment IDs differ, but both binaries must encode identical content.
                    signature = lambda mode: sorted((p["bytes"], p["sha256"]) for p in
                                                    entry["modes"][mode]["index"]["segment_files"].values())
                    assert signature("baseline") == signature("optimized")
                    save()

                phase("clean")
                with (root / single).open("ab") as file:
                    file.write(b"\n# CODERG_COMMIT_BENCH_SINGLE\n")
                phase("dirty_single")
                git(root, "add", "--", str(single))
                git(root, "commit", "--quiet", "-m", "benchmark single file")
                phase("promote_commit")
                for path in batch:
                    with (root / path).open("ab") as file:
                        file.write(b"\n# CODERG_COMMIT_BENCH_BATCH\n")
                git(root, "add", "--", *map(str, batch))
                git(root, "commit", "--quiet", "-m", "benchmark 32 files")
                phase("committed_batch")
                git(root, "commit", "--quiet", "--allow-empty", "-m", "benchmark same tree")
                phase("empty_commit")
                git(root, "checkout", "--quiet", "--detach", before["commit"])
                phase("cached_rollback")
                phase("clean_after_rollback")
            print(f"{kind} round {number}/{len(kinds)}: all 7 transitions and both binaries PASS", flush=True)

    results["summary"] = {}
    for phase_name in PHASES:
        results["summary"][phase_name] = {}
        for name in binaries:
            timings = [r["phases"][phase_name]["modes"][name]["elapsed_ms"] for r in results["rounds"] if r["kind"] == "timing"]
            rss = [r["phases"][phase_name]["modes"][name]["rss_bytes"] for r in results["rounds"] if r["kind"] == "rss"]
            results["summary"][phase_name][name] = dict(timing=timing_summary(timings),
                rss_samples_bytes=rss, rss_median_bytes=statistics.median(rss) if rss else None,
                rss_max_bytes=max(rss) if rss else None,
                final_index=results["rounds"][-1]["phases"][phase_name]["modes"][name]["index"])
    metadata["source_after"] = state(source)
    metadata["source_unchanged"] = before == metadata["source_after"]
    metadata["binaries_unchanged"] = all(sha(p) == metadata["binaries"][k]["sha256"] for k, p in binaries.items())
    metadata["code_unchanged"] = all(sha(repo / p) == h for p, h in metadata["source_sha256"].items())
    save()
    assert metadata["source_unchanged"] and metadata["binaries_unchanged"] and metadata["code_unchanged"]
    print(f"All checks passed. Results: {output}", flush=True)


if __name__ == "__main__":
    main()
