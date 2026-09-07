"""Remeasure the two many-file cases whose 31-sample p95 had large spikes."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import random
import statistics
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--work", required=True, type=Path)
args = parser.parse_args()
work = args.work
meta = json.loads((work / "results/metadata.json").read_text())
original = json.loads((work / "results/many_small_16384/results.json").read_text())
root = Path(original["dataset"]["root"])
output = work / "tail-confirmation"
output.mkdir()
index = output / "index"
env = dict(os.environ)
env.pop("RIPGREP_CONFIG_PATH", None)
env["GIT_OPTIONAL_LOCKS"] = "0"
subprocess.run([meta["binaries"]["baseline"]["path"], "index", str(root), "--index-dir", str(index)],
               env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=True)
jobs = [("short_files", ""), ("common_count", "_no_refresh")]
commands, samples, expected = {}, {}, {}
for query, suffix in jobs:
    record = next(q for q in original["queries"] if q["id"] == query)
    for version in ("baseline", "optimized"):
        mode = version + suffix
        command = original["commands"][query][mode].copy()
        command[command.index("--index-dir") + 1] = str(index)
        key = query + "/" + mode
        commands[key] = command
        samples[key] = []
        proc = subprocess.run(command, cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        lines = b"\n".join(sorted(line.removeprefix(b"./") for line in proc.stdout.splitlines()))
        assert not proc.stderr
        assert proc.returncode == record["parity"][mode]["exit_code"]
        assert hashlib.sha256(lines).hexdigest() == record["parity"][mode]["sha256"]
        expected[key] = proc.returncode
rng = random.Random(613920)
for repeat in range(206):
    order = list(commands)
    rng.shuffle(order)
    for key in order:
        start = time.perf_counter_ns()
        proc = subprocess.run(commands[key], cwd=root, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
        elapsed = (time.perf_counter_ns() - start) / 1e6
        assert not proc.stderr and proc.returncode == expected[key]
        if repeat >= 5:
            samples[key].append(elapsed)
    if repeat >= 5 and (repeat - 4) % 40 == 0:
        print(f"Tail confirmation: {repeat - 4}/201 rounds", flush=True)
result = dict(iterations=201, warmup=5, seed=613920, commands=commands,
              root=str(root), binaries=meta["binaries"],
              harness_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              measurement="timing only, sandboxed; independent samples are not pooled with the matrix",
              results={key: dict(samples_ms=values, median_ms=statistics.median(values),
                       p95_ms=sorted(values)[math.ceil(.95 * len(values)) - 1],
                       max_ms=max(values), above_300_ms=sum(v > 300 for v in values))
                       for key, values in samples.items()})
(output / "results.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({key:{k:v for k,v in row.items() if k != 'samples_ms'} for key,row in result['results'].items()}, indent=2))
