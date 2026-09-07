"""Independently remeasure the three largest observed regressions and a control."""
from pathlib import Path
import hashlib
import json
import os
import random
import statistics
import subprocess
import time

work = Path(__file__).resolve().parent
meta = json.loads((work / "results/metadata.json").read_text())
cases = [("few_large_64", "absent"), ("agentflow", "word_return"),
         ("viberwhisper", "impl"), ("vllm", "word_sampling")]
env = dict(os.environ)
env.pop("RIPGREP_CONFIG_PATH", None)
env["GIT_OPTIONAL_LOCKS"] = "0"
output = work / "confirmation"
output.mkdir()
commands, expected = {}, {}
for dataset, query_id in cases:
    result = json.loads((work / "results" / dataset / "results.json").read_text())
    root = Path(result["dataset"]["root"])
    index = output / (dataset + "-index")
    subprocess.run([meta["binaries"]["baseline"]["path"], "index", str(root), "--index-dir", str(index)],
                   check=True, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, env=env)
    query = next(q for q in result["queries"] if q["id"] == query_id)
    expected[(dataset, query_id)] = query["parity"]["baseline"]
    for version in ("baseline", "optimized"):
        command = result["commands"][query_id][version].copy()
        command[command.index("--index-dir") + 1] = str(index)
        commands[(dataset, query_id, version)] = (root, command)
        proc = subprocess.run(command, cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        normalized = b"\n".join(sorted(line.removeprefix(b"./") for line in proc.stdout.splitlines()))
        assert not proc.stderr
        assert proc.returncode == expected[(dataset, query_id)]["exit_code"]
        assert hashlib.sha256(normalized).hexdigest() == expected[(dataset, query_id)]["sha256"]

rng = random.Random(129031)
samples = {case: [] for case in commands}
for repeat in range(106):
    jobs = list(commands)
    rng.shuffle(jobs)
    for case in jobs:
        root, command = commands[case]
        start = time.perf_counter_ns()
        proc = subprocess.run(command, cwd=root, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
        elapsed = (time.perf_counter_ns() - start) / 1e6
        assert not proc.stderr
        assert proc.returncode == expected[case[:2]]["exit_code"]
        if repeat >= 5:
            samples[case].append(elapsed)
rows = []
for dataset, query_id in cases:
    baseline = samples[(dataset, query_id, "baseline")]
    optimized = samples[(dataset, query_id, "optimized")]
    changes = []
    for _ in range(2000):
        indexes = [rng.randrange(101) for _ in range(101)]
        changes.append(100 * (statistics.median([optimized[i] for i in indexes]) /
                              statistics.median([baseline[i] for i in indexes]) - 1))
    changes.sort()
    row = dict(dataset=dataset, query=query_id, baseline_ms=statistics.median(baseline),
               optimized_ms=statistics.median(optimized),
               change_pct=100 * (statistics.median(optimized) / statistics.median(baseline) - 1),
               bootstrap_95=[changes[49], changes[1949]],
               baseline_samples_ms=baseline, optimized_samples_ms=optimized)
    rows.append(row)
    print({k: v for k, v in row.items() if 'samples' not in k})
(output / "results.json").write_text(json.dumps(dict(
    selected="three largest positive median changes in the matrix, plus word_sampling control",
    iterations=101, warmup=5, seed=129031, bootstrap_repeats=2000,
    measurement="timing only, sandboxed execution; independent samples are not pooled with the matrix",
    binaries=meta["binaries"], harness_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    commands={"/".join(case):dict(root=str(root), command=cmd) for case,(root,cmd) in commands.items()},
    results=rows), indent=2) + "\n")
