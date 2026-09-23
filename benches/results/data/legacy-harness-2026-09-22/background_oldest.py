#!/usr/bin/env python3
"""Time one A -> B baseline advancement alone and alongside foreground searches."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tempfile
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("root", "binary", "workspace", "output"):
        parser.add_argument(f"--{name}", type=Path, required=True)
    parser.add_argument("--extension", required=True)
    parser.add_argument("--query", required=True)
    parser.add_argument("--broad-query", required=True)
    parser.add_argument("--rounds", type=int, default=5)
    args = parser.parse_args()
    binary = str(args.binary.resolve())
    env = {**os.environ, "RAYON_NUM_THREADS": "4"}
    jobs, searches, cases = [], [], []
    verified = 0

    def run(command, cwd=None, search=False):
        start = time.perf_counter_ns()
        result = subprocess.run(command, cwd=cwd, env=env, capture_output=True)
        elapsed = (time.perf_counter_ns() - start) / 1e6
        if result.returncode not in ([0, 1] if search else [0]):
            raise RuntimeError(result.stderr.decode(errors="replace"))
        return result, elapsed

    revision = run(["git", "-C", str(args.root.resolve()), "rev-parse", "HEAD"])[0].stdout.decode().strip()
    args.workspace.mkdir(parents=True, exist_ok=True)
    for fraction in (0.01, 0.5):
        for round_id in range(args.rounds):
            with tempfile.TemporaryDirectory(prefix="oldest-", dir=args.workspace.resolve()) as temp:
                temp = Path(temp)
                root, index = temp / "repo", temp / "prepared"
                run(["git", "clone", "--quiet", "--shared", "--no-checkout", str(args.root.resolve()), str(root)])

                def git(*options):
                    return run(["git", "-C", str(root), *options])[0].stdout.decode().strip()

                git("checkout", "--quiet", "--detach", revision)
                for key, value in [("user.name", "Benchmark"), ("user.email", "bench@example.invalid"),
                                   ("commit.gpgsign", "false"), ("core.hooksPath", "/dev/null")]:
                    git("config", key, value)
                run([binary, "index", str(root), "--index-dir", str(index)])
                stats = json.loads(run([binary, "stats", str(root), "--index-dir", str(index), "--json"])[0].stdout)
                tracked = set(git("ls-files").splitlines())
                candidates = sorted((d["path"] for d in stats["documents"] if d["active"] and d["searchable"]
                                     and d["path"] in tracked and d["path"].endswith(args.extension)
                                     and 0 < d["len"] <= 1024 * 1024 and not (root / d["path"]).is_symlink()),
                                    key=lambda p: hashlib.sha256(p.encode()).digest())
                selected = candidates[:max(1, int(len(candidates) * fraction))]
                assert selected
                prefix = "#" if args.extension == ".py" else "//"
                commits = []
                for version in ("B", "C", "D"):
                    paths = selected if version == "B" else selected[:1]
                    for path in paths:
                        with (root / path).open("ab") as file:
                            file.write(("\n" + "".join(f"{prefix} BACKGROUND_{version}_MARKER {i:04d} index change\n" for i in range(32))).encode())
                    run([binary, "search", "-Fl", args.query, str(root), "--index-dir", str(index)], search=True)
                    git("add", "-u")
                    git("commit", "--quiet", "-m", version)
                    commits.append(git("rev-parse", "HEAD"))
                    run([binary, "search", "-Fl", args.query, str(root), "--index-dir", str(index)], search=True)
                patterns = ["ABSENT_SNAPSHOT_TOKEN", args.query, args.broad_query,
                            "BACKGROUND_B_MARKER", "BACKGROUND_C_MARKER", "BACKGROUND_D_MARKER"]

                def expected_results():
                    expected = {}
                    for q in patterns:
                        result, _ = run(["rg", "--hidden", "-g", "!.git", "-Fl", "--", q, "."], cwd=root, search=True)
                        expected[q] = sorted(p.removeprefix("./") for p in result.stdout.decode().splitlines())
                    return expected

                expected = expected_results()

                def search(q, idx, phase=None, sample=0, no_refresh=False):
                    nonlocal verified
                    command = [binary, "search", "-Fl", q, str(root), "--index-dir", str(idx)]
                    if no_refresh:
                        command.append("--no-refresh")
                    result, ms = run(command, search=True)
                    assert sorted(result.stdout.decode().splitlines()) == expected[q], (phase, q)
                    verified += 1
                    if phase:
                        assert not result.stderr, result.stderr
                        searches.append(dict(fraction=fraction, round=round_id, phase=phase,
                                             query=q, sample=sample, no_refresh=no_refresh, elapsed_ms=ms))
                    return result.stderr.decode()

                cases.append(dict(fraction=fraction, round=round_id, changed_files=len(selected), commits=commits))
                configurations = [(q, False) for q in patterns[:3]] + [(patterns[0], True)]
                for sample in range(12):
                    for q, nr in configurations:
                        search(q, index, "idle" if sample >= 2 else None, sample, nr)
                for mode in (["alone", "concurrent"] if round_id % 2 == 0 else ["concurrent", "alone"]):
                    idx = temp / mode
                    shutil.copytree(index, idx)
                    # Checkout validation in the previous mode changes mtimes.
                    # Synchronize the copied view before timing maintenance.
                    search(args.query, idx)
                    started = time.perf_counter_ns()
                    proc = subprocess.Popen([binary, "compact-history", str(root), "--index-dir", str(idx)],
                                            env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                    sample = 0
                    if mode == "concurrent":
                        while proc.poll() is None:
                            q, nr = configurations[(sample + round_id) % len(configurations)]
                            search(q, idx, mode, sample, nr)
                            sample += 1
                    stdout, stderr = proc.communicate()
                    elapsed = (time.perf_counter_ns() - started) / 1e6
                    assert proc.returncode == 0, stderr
                    summary = json.loads(stdout)
                    assert summary["published"], summary
                    jobs.append(dict(fraction=fraction, round=round_id, mode=mode, wall_ms=elapsed, **summary))
                    for q in patterns:
                        search(q, idx, "after_" + mode)
                    for rev in commits:
                        git("checkout", "--quiet", "--detach", rev)
                        expected = expected_results()
                        assert "0 extracted" in search(args.query, idx), "successor must remain cached"
                        for q in patterns:
                            search(q, idx, no_refresh=True)
                    git("checkout", "--quiet", "--detach", commits[-1])
                    expected = expected_results()
                print(f"{args.root.name} fraction={fraction} round={round_id + 1} complete", flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(dict(commit=revision, platform=platform.platform(), binary=binary,
        binary_sha256=hashlib.sha256(Path(binary).read_bytes()).hexdigest(), threads=4, rounds=args.rounds,
        verified=verified, jobs=jobs, searches=searches, cases=cases), indent=2) + "\n")


if __name__ == "__main__":
    main()
