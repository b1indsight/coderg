#!/usr/bin/env python3
"""Read a real repository through disposable local clones; never edit the source."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import tempfile
import time


def execute(args, cwd=None, search=False):
    start = time.perf_counter_ns()
    output = subprocess.run(args, cwd=cwd, capture_output=True)
    ms = (time.perf_counter_ns() - start) / 1e6
    if output.returncode not in ([0, 1] if search else [0]):
        raise RuntimeError(output.stderr.decode(errors="replace"))
    return output, ms


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--query", action="append", required=True)
    parser.add_argument("--iterations", type=int, default=5)
    parser.add_argument("--search-iterations", type=int, default=15)
    parser.add_argument("--bench-rg", action="store_true", help="Time rg alongside unchanged indexed searches.")
    parser.add_argument("--regex", action="store_true", help="Interpret queries as regular expressions in both tools.")
    parser.add_argument("--workspace", type=Path, default=Path(".cache/2026-09-18/commit-snapshot-production"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    search_flags = "-l" if args.regex else "-Fl"
    args.workspace.mkdir(parents=True, exist_ok=True)
    root = args.root.resolve()
    revision = execute(["git", "-C", str(root), "rev-parse", "HEAD"])[0].stdout.decode().strip()
    binaries = {"baseline": args.baseline.resolve(), "snapshots": args.binary.resolve()}
    builds, searches, rg_searches = [], [], []
    verified = 0
    with tempfile.TemporaryDirectory(prefix="real-search-", dir=args.workspace.resolve()) as temp:
        temp = Path(temp)
        clone = temp / "repo"
        execute(["git", "clone", "--quiet", "--shared", "--no-checkout", str(root), str(clone)])
        execute(["git", "-C", str(clone), "checkout", "--quiet", "--detach", revision])
        expected = {}
        for query in args.query:
            output, _ = execute(["rg", "--hidden", "-g", "!.git", search_flags, "--", query, "."], cwd=clone, search=True)
            expected[query] = sorted(p.removeprefix("./") for p in output.stdout.decode().splitlines())
        for iteration in range(args.iterations):
            order = list(binaries) if iteration % 2 == 0 else list(reversed(binaries))
            indexes = {name: temp / f"{iteration}-{name}" for name in binaries}
            for name in order:
                result, ms = execute([str(binaries[name]), "index", str(clone), "--index-dir", str(indexes[name])])
                stats = json.loads(execute([str(binaries[name]), "stats", str(clone), "--json", "--index-dir", str(indexes[name])])[0].stdout)
                builds.append({"iteration": iteration, "variant": name, "elapsed_ms": ms,
                               "searchable_files": sum(d["active"] and d["searchable"] for d in stats["documents"]),
                               "source_bytes": sum(d["len"] for d in stats["documents"] if d["active"] and d["searchable"]),
                               "disk_bytes": sum(p.stat().st_size for p in indexes[name].rglob("*") if p.is_file())})
                print(f"{root.name} {iteration} build {name}: {ms:.2f} ms", flush=True)
            for query in args.query:
                def measure_rg(sample):
                    nonlocal verified
                    result, ms = execute(["rg", "--hidden", "-g", "!.git", search_flags, "--", query, "."], cwd=clone, search=True)
                    if sorted(p.removeprefix("./") for p in result.stdout.decode().splitlines()) != expected[query]:
                        raise AssertionError(f"rg/{query}: unexpected results")
                    verified += 1
                    if sample >= 2:
                        rg_searches.append({"iteration": iteration, "sample": sample - 2,
                                            "query": query, "elapsed_ms": ms})

                for sample in range(args.search_iterations + 2):
                    if args.bench_rg and sample % 2 == 0:
                        measure_rg(sample)
                    for name in order if sample % 2 == 0 else reversed(order):
                        for no_refresh in [False, True]:
                            cmd = [str(binaries[name]), "search", search_flags, query, str(clone), "--index-dir", str(indexes[name])]
                            if no_refresh:
                                cmd.append("--no-refresh")
                            result, ms = execute(cmd, search=True)
                            if sorted(result.stdout.decode().splitlines()) != expected[query]:
                                raise AssertionError(f"{name}/{query} differs from rg")
                            if result.stderr:
                                raise AssertionError(f"unchanged search refreshed: {result.stderr!r}")
                            verified += 1
                            if sample >= 2:
                                searches.append({"iteration": iteration, "sample": sample - 2, "variant": name,
                                                 "query": query, "no_refresh": no_refresh, "elapsed_ms": ms})
                    if args.bench_rg and sample % 2 != 0:
                        measure_rg(sample)
    report = {"source": str(root), "commit": revision, "platform": platform.platform(),
              "iterations": args.iterations, "search_iterations": args.search_iterations,
              "verified_queries": verified, "builds": builds, "searches": searches,
              "rg_searches": rg_searches,
              "regex": args.regex,
              "match_counts": {query: len(paths) for query, paths in expected.items()},
              "rg_version": execute(["rg", "--version"])[0].stdout.decode().splitlines()[0] if args.bench_rg else None,
              "binaries": {name: {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
                           for name, path in binaries.items()},
              "notes": ["Only committed HEAD is cloned; original working directories are never modified.",
                        "Build timings include CLI startup and durable index publication; clone/checkout excluded.",
                        "Queries compared with rg; interleaved variants; two search warmups per query excluded.",
                        "OS caches warm; five builds per binary; all build samples retained."]}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
