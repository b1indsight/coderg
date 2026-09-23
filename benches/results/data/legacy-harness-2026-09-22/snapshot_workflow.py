#!/usr/bin/env python3
"""Compare commit snapshot refreshes on deterministic edits to real source files."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import re
import tempfile

from snapshot_search import execute


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ("root", "baseline", "binary", "workspace", "output"):
        parser.add_argument(f"--{option}", type=Path, required=True)
    parser.add_argument("--extension", choices=[".py", ".rs"], required=True)
    parser.add_argument("--query", required=True)
    parser.add_argument("--iterations", type=int, default=5)
    parser.add_argument("--two-branches", action="store_true",
                        help="Alternate edits and commits on two divergent branches, then revisit both tips.")
    parser.add_argument("--history-compaction", action="store_true",
                        help="Allow cached segment locations to change during history maintenance; commits must still preserve references.")
    parser.add_argument("--expect-promotion-reuse", action="store_true",
                        help="Require commit_B to preserve the already indexed segment references.")
    args = parser.parse_args()
    source = args.root.resolve()
    revision = execute(["git", "-C", str(source), "rev-parse", "HEAD"])[0].stdout.decode().strip()
    binaries = {"baseline": args.baseline.resolve(), "snapshots": args.binary.resolve()}
    rows, selections = [], []
    verified = 0
    queries = [args.query, "SNAPSHOT_BENCH_COMMITTED", "SNAPSHOT_BENCH_DIRTY"]
    if args.two_branches:
        queries = [args.query] + [f"SNAPSHOT_BRANCH_{branch}{n}" for n in (1, 2) for branch in ("A", "B")]
    args.workspace.mkdir(parents=True, exist_ok=True)
    for iteration in range(args.iterations):
        with tempfile.TemporaryDirectory(prefix="real-workflow-", dir=args.workspace.resolve()) as temporary:
            temporary = Path(temporary)
            root = temporary / "repo"
            execute(["git", "clone", "--quiet", "--shared", "--no-checkout", str(source), str(root)])

            def git(*options):
                return execute(["git", "-C", str(root), *options])[0].stdout.decode().strip()

            git("checkout", "--quiet", "--detach", revision)
            for key, value in [("user.name", "Snapshot benchmark"), ("user.email", "benchmark@example.invalid"),
                               ("commit.gpgsign", "false"), ("core.hooksPath", "/dev/null")]:
                git("config", key, value)
            indexes = {name: temporary / name for name in binaries}
            segment_views = {}
            order = list(binaries) if iteration % 2 == 0 else list(reversed(binaries))

            def invoke(name, command, query=None, no_refresh=False):
                options = [str(binaries[name]), command]
                if query is not None:
                    options += ["-Fl", query]
                options += [str(root), "--index-dir", str(indexes[name])]
                if no_refresh:
                    options.append("--no-refresh")
                return execute(options, search=command == "search")

            def step(label, build=False, reuse_from=None):
                nonlocal verified
                expected = {}
                for query in queries:
                    result, _ = execute(["rg", "--hidden", "-g", "!.git", "-Fl", "--", query, "."], cwd=root, search=True)
                    expected[query] = sorted(p.removeprefix("./") for p in result.stdout.decode().splitlines())
                for name in order:
                    result, elapsed = invoke(name, "index" if build else "search", None if build else args.query)
                    diagnostic = result.stderr.decode().strip()
                    if name == "snapshots" and (reuse_from or label in {"rollback_A", "revisit_B", "discard_dirty"}):
                        promoted = label.startswith("commit_") and diagnostic == "coderg: advanced index to the current Git tree"
                        if "0 extracted" not in diagnostic and not promoted:
                            raise AssertionError(f"{label}: expected cached reuse, got {diagnostic}")
                    for query in queries:
                        result, _ = invoke(name, "search", query, no_refresh=True)
                        if sorted(result.stdout.decode().splitlines()) != expected[query]:
                            raise AssertionError(f"{name}/{label}/{query}: differs from rg")
                        verified += 1
                    stats = json.loads(execute([str(binaries[name]), "stats", str(root), "--json", "--index-dir", str(indexes[name])])[0].stdout)
                    previous_view = reuse_from or {"commit_B": "large_edit", "rollback_A": "build_A",
                                     "revisit_B": "commit_B", "discard_dirty": "commit_B"}.get(label)
                    check_refs = not args.history_compaction or label.startswith("commit_")
                    if check_refs and (args.expect_promotion_reuse or args.two_branches) and name == "snapshots" and previous_view:
                        if stats["segments"] != segment_views[(name, previous_view)]:
                            raise AssertionError(f"{label} rewrote already indexed segments")
                    segment_views[(name, label)] = stats["segments"]
                    match = re.search(r"snapshot: (\d+) extracted, (\d+) reused", diagnostic)
                    rows.append({"iteration": iteration, "step": label, "variant": name,
                                 "elapsed_ms": elapsed, "diagnostic": diagnostic,
                                 "extracted": int(match[1]) if match else None,
                                 "reused": int(match[2]) if match else None,
                                 "segments": len(stats["segments"]),
                                 "segment_refs": stats["segments"],
                                 "disk_bytes": sum(p.stat().st_size for p in indexes[name].rglob("*") if p.is_file())})
                    print(f"{source.name} {iteration} {label} {name}: {elapsed:.2f} ms", flush=True)

            step("build_A", build=True)
            stats = json.loads(execute([str(binaries["baseline"]), "stats", str(root), "--json", "--index-dir", str(indexes["baseline"])])[0].stdout)
            tracked = set(git("ls-files").splitlines())
            candidates = [d["path"] for d in stats["documents"]
                          if d["active"] and d["searchable"] and d["path"] in tracked
                          and d["path"].endswith(args.extension) and 0 < d["len"] <= 1024 * 1024
                          and not (root / d["path"]).is_symlink()]
            candidates.sort(key=lambda path: hashlib.sha256(path.encode()).digest())
            chosen = candidates[:max(1, len(candidates) // 2)]
            if not chosen:
                raise AssertionError("No source files selected")
            selections.append({"iteration": iteration, "eligible": len(candidates), "changed": len(chosen),
                               "source_bytes": sum((root / p).stat().st_size for p in chosen), "paths": chosen})
            prefix = "#" if args.extension == ".py" else "//"

            def append(marker):
                for path in chosen:
                    with (root / path).open("ab") as stream:
                        stream.write(("\n" + "".join(f"{prefix} {marker} {i:04d} benchmark refresh source change\n" for i in range(32))).encode())

            if args.two_branches:
                git("branch", "bench-A", revision)
                git("branch", "bench-B", revision)
                previous = {"A": "build_A", "B": "build_A"}
                for n in (1, 2):
                    for branch in ("A", "B"):
                        git("checkout", "--quiet", f"bench-{branch}")
                        # First checkout changes no indexed content or tree.
                        if (branch, n) != ("A", 1):
                            step(f"switch_{branch}{n}", reuse_from=previous[branch])
                        append(f"SNAPSHOT_BRANCH_{branch}{n}")
                        edit = f"edit_{branch}{n}"
                        step(edit)
                        git("add", "-u")
                        git("commit", "--quiet", "-m", f"Benchmark {branch}{n}")
                        label = f"commit_{branch}{n}"
                        step(label, reuse_from=edit)
                        previous[branch] = label
                for branch in ("A", "B"):
                    git("checkout", "--quiet", f"bench-{branch}")
                    step(f"revisit_{branch}2", reuse_from=previous[branch])
                continue

            append(queries[1])
            step("large_edit")
            git("add", "-u")
            git("commit", "--quiet", "-m", "Benchmark large source edit")
            commit_b = git("rev-parse", "HEAD")
            step("commit_B")
            git("checkout", "--quiet", "--detach", revision)
            step("rollback_A")
            git("checkout", "--quiet", "--detach", commit_b)
            step("revisit_B")
            append(queries[2])
            step("dirty_B")
            git("reset", "--quiet", "--hard", commit_b)
            step("discard_dirty")
    report = {"source": str(source), "commit": revision, "platform": platform.platform(),
              "iterations": args.iterations, "two_branches": args.two_branches,
              "history_compaction": args.history_compaction,
              "verified_queries": verified, "rows": rows, "searches": [],
              "selections": selections, "binaries": {name: {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
                                                         for name, path in binaries.items()},
              "notes": ["Real committed corpus; append 32 comment lines to deterministic half of eligible source files.",
                        "Each iteration uses fresh disposable clone and indexes; original source untouched.",
                        "Timings include CLI startup, refresh and one search; exclude edits, Git commands and rg verification.",
                        "All build samples retained; warm OS cache; variant order alternates by iteration."]}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
