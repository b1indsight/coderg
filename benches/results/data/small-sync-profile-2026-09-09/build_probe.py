#!/usr/bin/env python3
"""Interleave builds with sync timing and a scratch-only skip-sync control."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import statistics
import subprocess
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", required=True, type=Path)
    parser.add_argument("--variant", action="append", required=True)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    here = Path(__file__).resolve().parent
    plan = json.loads((here.parent / "small-auto25-2026-09-09/plan.json").read_text())
    variants = {name: Path(binary).resolve() for name, binary in
                (value.split("=", 1) for value in args.variant)}
    workspace = args.workspace.resolve()
    workspace.mkdir(parents=True, exist_ok=True)
    assert not args.output.exists()
    env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
    rng = random.Random(20260909)
    report = {"variants": {name: str(path) for name, path in variants.items()},
              "binary_sha256": {name: hashlib.sha256(path.read_bytes()).hexdigest()
                                for name, path in variants.items()},
              "iterations": 9, "warmup": 2, "datasets": {}}
    for dataset in plan["datasets"]:
        name, root = dataset["id"], Path(dataset["root"])
        samples = {variant: [] for variant in variants}
        reference = None
        for iteration in range(-2, 9):
            order = list(variants)
            rng.shuffle(order)
            for variant in order:
                index = workspace / f"build-{name}-{variant}-{iteration}"
                assert not index.exists()
                command = [str(variants[variant]), "index", str(root), "--index-dir", str(index)]
                started = time.perf_counter_ns()
                result = subprocess.run(command, env=env, capture_output=True, check=True)
                wall_ms = (time.perf_counter_ns() - started) / 1e6
                sync = {}
                for line in result.stderr.decode().splitlines():
                    if line.startswith("PROBE "):
                        _, label, elapsed = line.split()
                        sync.setdefault(label, []).append(float(elapsed))
                hashes = sorted(hashlib.sha256(p.read_bytes()).hexdigest()
                                for p in (index / "segments").iterdir())
                if reference is None:
                    reference = hashes
                assert reference == hashes, (name, variant, iteration)
                if iteration >= 0:
                    samples[variant].append({"wall_ms": wall_ms, "sync_ms": sync,
                                             "command": command})
        report["datasets"][name] = {"root": str(root), "commit": dataset["commit"],
                                     "segment_hashes": reference, "samples": samples}
        for variant, values in samples.items():
            print(name, variant, "wall median ms",
                  round(statistics.median(v["wall_ms"] for v in values), 3),
                  "sync median ms", round(statistics.median(
                      sum(sum(group) for group in v["sync_ms"].values()) for v in values), 3),
                  flush=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
